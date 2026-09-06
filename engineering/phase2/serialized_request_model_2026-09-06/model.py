#!/usr/bin/env python3
"""
Disposable architecture model: serialized request boundary for S.P.A.R.K. Phase 2.

Architecture evidence only. Not production code, not a benchmark, not a transport.
Models: bounded FIFO mailbox -> one consumer -> engine; one active request until
completion; budget exhaustion pauses; horizon frontier F set only at completion;
per-cohort engine digest excludes F; stable-boundary digest includes F; commands
finalize+execute atomically at request completion after catch-up to effective_time;
past-dated requests rejected non-canonically at start; restart = (stores, F) snapshot
plus the consumer re-presenting the head request.
"""
import hashlib, json, copy, sys

def H(obj):
    return hashlib.sha256(json.dumps(obj, sort_keys=True, separators=(",", ":")).encode()).hexdigest()[:16]

# ---------------------------------------------------------------- requests / mailbox
class Advance:
    def __init__(self, T): self.T = T
    def horizon(self): return self.T
    def label(self): return f"Advance({self.T})"
class Command:
    def __init__(self, e, payload): self.e, self.payload = e, payload
    def horizon(self): return self.e
    def label(self): return f"Command@{self.e}({self.payload})"

class Mailbox:
    """Bounded FIFO. Head stays until the consumer reports completion or rejection."""
    def __init__(self, capacity): self.capacity, self.q = capacity, []
    def offer(self, r):
        if len(self.q) >= self.capacity: return "BACKPRESSURE_MAILBOX_FULL"
        self.q.append(r); return "ACCEPTED"
    def head(self): return self.q[0] if self.q else None
    def pop(self): return self.q.pop(0)

# ---------------------------------------------------------------- engine
class Engine:
    def __init__(self, rules, budget):
        self.rules, self.B = rules, budget
        self.sched = {}           # (due, profile) -> sorted list of (key, payload)
        self.cells = {}           # name -> {"v":int, "updated_at":int}
        self.timeline = []        # finalized commands [(ordinal, e, payload)]
        self.F = 0                # horizon frontier: horizon of last COMPLETED request
        self.trace = []           # canonical per-cohort records (partition-invariant)
        self.diag = []            # noncanonical pacing diagnostics per call
        self.boundary_log = []    # stable-boundary digests at request completion
        self.f_log = []           # NONCANONICAL: F observed at each pre-wave capture (negative control)

    # --- digests
    def engine_state_digest(self):   # v1 §8 composition: excludes F
        return H({"sched": sorted((k[0], k[1], v) for k, v in self.sched.items() if v),
                  "cells": self.cells, "timeline": self.timeline})
    def stable_boundary_digest(self):  # snapshot / equal-state commitment: includes F
        return H({"engine": self.engine_state_digest(), "F": self.F})

    # --- scheduler ops (compare-and-take shape, simplified)
    def schedule(self, due, profile, key, payload):
        self.sched.setdefault((due, profile), []).append((key, payload)); self.sched[(due, profile)].sort()
    def least_due_slice(self, horizon):
        cands = [k for k, v in self.sched.items() if v and k[0] <= horizon]
        return min(cands) if cands else None
    def take_least_due_slice(self, horizon, expected):
        k = self.least_due_slice(horizon)
        assert k == expected, "SliceChanged"
        return k, self.sched.pop(k)

    # --- evaluation (now = cohort canonical time; never the horizon)
    def eval_cohort(self, kind, now, profile, items):
        prewave = self.engine_state_digest()          # captured after extraction
        self.f_log.append({"t": now, "kind": kind, "prewave": prewave, "F_at_capture": self.F,
                           "prewave_if_F_included": H({"engine": prewave, "F": self.F})})
        effects = []
        for key, payload in items:
            for eff in self.rules(kind, now, payload, self.cells):
                effects.append(eff)
        for eff in sorted(effects, key=lambda e: json.dumps(e, sort_keys=True)):
            if eff["op"] == "decay":
                c = self.cells.setdefault(eff["cell"], {"v": 100, "updated_at": 0})
                elapsed = now - c["updated_at"]
                assert elapsed >= 0, f"BACKWARDS EVALUATION at now={now}, updated_at={c['updated_at']}"
                c["v"] = max(0, c["v"] - elapsed * eff["rate"]); c["updated_at"] = now
            elif eff["op"] == "set":
                self.cells[eff["cell"]] = {"v": eff["v"], "updated_at": now}
            elif eff["op"] == "enqueue":
                assert eff["due"] >= now + 1, "delayed work must be strictly later"
                self.schedule(eff["due"], profile, eff["key"], eff["payload"])
        batch = H({"prewave": prewave, "kind": kind, "now": now, "profile": profile,
                   "keys": [k for k, _ in items], "effects": effects})
        self.trace.append({"kind": kind, "t": now, "p": profile, "prewave": prewave, "batch": batch})

    # --- one processing call of the presented request
    def process(self, req):
        h = req.horizon()
        if h < self.F:
            return "REJECTED_HORIZON_BEHIND_FRONTIER"     # non-canonical; nothing mutated
        r, admitted = self.B, 0
        while True:
            k = self.least_due_slice(h)
            if k is None: break
            if not (1 <= r or admitted == 0):
                self.diag.append({"call": req.label(), "deferred_at": k[0]}); return "PAUSED"
            (due, profile), items = self.take_least_due_slice(h, k)
            self.eval_cohort("scheduled", due, profile, items); admitted += 1; r -= 1
        if isinstance(req, Command):
            if not (1 <= r or admitted == 0):
                self.diag.append({"call": req.label(), "deferred_at": "command"}); return "PAUSED"
            # finalize (stage+fence, one-command range) and execute atomically at completion
            ordinal = len(self.timeline) + 1
            self.timeline.append((ordinal, req.e, req.payload))
            self.eval_cohort("command", req.e, "P", [(f"cmd#{ordinal}", req.payload)])
        self.F = h                                        # only at completion
        self.boundary_log.append({"completed": req.label(), "F": self.F, "sbd": self.stable_boundary_digest()})
        return "COMPLETED"

    # --- snapshot / restart (stores + F only; no active-request state)
    def snapshot(self):
        return copy.deepcopy({"sched": self.sched, "cells": self.cells, "timeline": self.timeline, "F": self.F})
    @classmethod
    def restore(cls, rules, budget, snap):
        e = cls(rules, budget); e.sched, e.cells, e.timeline, e.F = copy.deepcopy(snap)["sched"], \
            copy.deepcopy(snap)["cells"], copy.deepcopy(snap)["timeline"], snap["F"]; return e

# ---------------------------------------------------------------- consumer
def drive(engine, mailbox, log, max_calls=100):
    """One consumer: present head until COMPLETED or REJECTED; then pop. Returns call count."""
    calls = 0
    while mailbox.head() is not None and calls < max_calls:
        r = mailbox.head(); res = engine.process(r); calls += 1
        log.append((r.label(), res))
        if res in ("COMPLETED", "REJECTED_HORIZON_BEHIND_FRONTIER"): mailbox.pop()
    return calls

# ---------------------------------------------------------------- rules
def rules(kind, now, payload, cells):
    """A@10 decays cell x and creates delayed work D at now+5 (inside a 20 horizon);
       B@20 decays x; D@15 sets y; commands set z and decay x."""
    if kind == "scheduled":
        if payload == "A": return [{"op": "decay", "cell": "x", "rate": 1}, {"op": "enqueue", "due": now + 5, "key": "D", "payload": "D"}]
        if payload == "B": return [{"op": "decay", "cell": "x", "rate": 1}]
        if payload == "D": return [{"op": "set", "cell": "y", "v": now}]
    if kind == "command":
        return [{"op": "set", "cell": "z", "v": payload}, {"op": "decay", "cell": "x", "rate": 1}]
    return []

def fresh(budget):
    e = Engine(rules, budget); e.schedule(10, "P", "A", "A"); e.schedule(20, "P", "B", "B"); return e

def canonical(e):
    return {"trace": e.trace, "final_engine": e.engine_state_digest(), "F": e.F, "sbd": e.stable_boundary_digest()}

# ---------------------------------------------------------------- scenarios
out = {}
def check(name, cond, detail=""):
    out[name] = ("PASS" if cond else "FAIL") + (f" — {detail}" if detail else "")
    print(f"[{out[name].split(' ')[0]}] {name}{(' — ' + detail) if detail else ''}")

# S1: two requests, second arrives while the first is paused (budget 1)
e1 = fresh(1); mb = Mailbox(4); log1 = []
mb.offer(Advance(20))
log1.append((mb.head().label(), e1.process(mb.head())))          # consumes A@10 only -> PAUSED
paused = log1[-1][1] == "PAUSED"
acc = mb.offer(Command(21, "c1"))                                # arrives while paused; waits behind head
head_still_first = mb.head().label() == "Advance(20)"
drive(e1, mb, log1)
order1 = [r for r, res in log1 if res == "COMPLETED"]
check("S1 second request waits while first paused", paused and acc == "ACCEPTED" and head_still_first
      and order1 == ["Advance(20)", "Command@21(c1)"], f"calls={log1}")

# S2: budgets 1 and 2 -> identical canonical trace and request order
def run_budget(b):
    e = fresh(b); mb = Mailbox(4); log = []
    mb.offer(Advance(20)); mb.offer(Command(21, "c1")); drive(e, mb, log); return e, log
eb1, lb1 = run_budget(1); eb2, lb2 = run_budget(2)
check("S2 budgets 1 and 2 give identical canonical results", canonical(eb1) == canonical(eb2),
      f"budget1 calls={len(lb1)} budget2 calls={len(lb2)}; cohort order={[ (t['kind'],t['t']) for t in eb1.trace]}")
check("S2 pacing diagnostics differ (noncanonical, expected)", eb1.diag != eb2.diag)

# S3: one large catch-up vs smaller completed catch-ups (R-038), work-producing cohort included
eL = fresh(3); mbL = Mailbox(4); mbL.offer(Advance(20)); mbL.offer(Command(21, "c1")); drive(eL, mbL, [])
eS = fresh(3); mbS = Mailbox(8)
for T in (10, 12, 15, 17, 20): mbS.offer(Advance(T))
mbS.offer(Command(21, "c1")); drive(eS, mbS, [])
check("S3 one catch-up == stepwise catch-ups (per-cohort digests, final state, F)", canonical(eL) == canonical(eS),
      f"created D@15 consumed inside horizon in both; prewave digests exclude F")
# S3b: stable-boundary digests differ mid-way (F differs) but agree at equal completed horizon
sbd_L_final, sbd_S_final = eL.boundary_log[-1]["sbd"], eS.boundary_log[-1]["sbd"]
check("S3b stable-boundary digests agree at equal completed horizon", sbd_L_final == sbd_S_final)
check("S3c stable-boundary digests differ at unequal completed horizons (F committed)",
      eL.boundary_log[0]["sbd"] != eS.boundary_log[0]["sbd"], "Advance(20) vs Advance(10) completion")

# S3d: negative control for the digest contradiction: including F in the per-cohort digest breaks S3
pairs = list(zip(eL.f_log, eS.f_log))
excl_equal = all(a["prewave"] == b["prewave"] for a, b in pairs)
incl_equal = all(a["prewave_if_F_included"] == b["prewave_if_F_included"] for a, b in pairs)
diverge = [(a["t"], a["F_at_capture"], b["F_at_capture"]) for a, b in pairs if a["F_at_capture"] != b["F_at_capture"]]
check("S3d F excluded from per-cohort digest keeps equality; F included would break it",
      excl_equal and not incl_equal, f"cohorts where F differs at capture (t, F_large, F_stepwise)={diverge}")

# S4: past-dated input handling
e4 = fresh(2); mb4 = Mailbox(4); log4 = []
mb4.offer(Advance(20)); mb4.offer(Command(15, "late")); mb4.offer(Command(20, "ok_equal")); mb4.offer(Command(25, "ok"))
drive(e4, mb4, log4)
res4 = {r: s for r, s in log4}
check("S4 past-dated command rejected non-canonically; equal-time and later accepted",
      res4["Command@15(late)"] == "REJECTED_HORIZON_BEHIND_FRONTIER" and res4["Command@20(ok_equal)"] == "COMPLETED"
      and res4["Command@25(ok)"] == "COMPLETED" and all(c[1] != "late" for c in [(o, p) for o, _, p in e4.timeline]),
      f"timeline={e4.timeline}")
# S4b: equal engine digests, different consumed histories, same next past-dated request -> same decision
eA = Engine(rules, 2); eA.schedule(10, "P", "N", "noop"); mbA = Mailbox(2); mbA.offer(Advance(100)); drive(eA, mbA, [])
eB = Engine(rules, 2); eB.schedule(20, "P", "N", "noop"); mbB = Mailbox(2); mbB.offer(Advance(100)); drive(eB, mbB, [])
same_state = eA.stable_boundary_digest() == eB.stable_boundary_digest() and eA.engine_state_digest() == eB.engine_state_digest()
dA, dB = eA.process(Command(15, "x")), eB.process(Command(15, "x"))
check("S4b review §5.3: equal digests -> identical decision on Command@15; no extra state needed",
      same_state and dA == dB == "REJECTED_HORIZON_BEHIND_FRONTIER", "the deciding value F=100 is equal and committed")

# S5: restart while paused: snapshot (stores, F) + consumer re-presents head
e5 = fresh(1); mb5 = Mailbox(4); log5 = []
mb5.offer(Advance(20)); mb5.offer(Command(21, "c1"))
log5.append((mb5.head().label(), e5.process(mb5.head())))         # PAUSED after A@10
snap = e5.snapshot(); trace_before = copy.deepcopy(e5.trace); blog_before = copy.deepcopy(e5.boundary_log)
e5r = Engine.restore(rules, 1, snap); e5r.trace, e5r.boundary_log = trace_before, blog_before
drive(e5r, mb5, log5)                                              # head re-presented by the durable mailbox
check("S5 restart from (stores, F) with head re-presented == uninterrupted run", canonical(e5r) == canonical(eb1),
      "no active-request state inside the engine; the mailbox owns the head")

# S6: monotone canonical time guard never trips (assertion inside eval_cohort) across all scenarios
check("S6 no backwards decay evaluation in any scenario", True, "assert elapsed >= 0 held in every cohort")

json.dump({"checks": out, "trace_budget1": eb1.trace, "trace_budget2": eb2.trace,
           "diag_budget1": eb1.diag, "diag_budget2": eb2.diag, "boundary_large": eL.boundary_log,
           "boundary_stepwise": eS.boundary_log, "S4_log": log4},
          open(sys.argv[1] if len(sys.argv) > 1 else "results.json", "w"), indent=1, sort_keys=True)
print("ALL PASS" if all(v.startswith("PASS") for v in out.values()) else "SOME FAIL")
