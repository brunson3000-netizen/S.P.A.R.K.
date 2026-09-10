#!/usr/bin/env python3
"""
Disposable architecture model: V3-F01 final correction (serialized request boundary
with an engine-owned ActiveRequest discriminator).

Architecture evidence only. Not production code, not a benchmark, not a transport.
Standard library only. Extends engineering/phase2/serialized_request_model_2026-09-06/
model.py (preserved unchanged) with:

  - ActiveRequest: engine-owned Option<RequestDiscriminator>; set before any cohort
    mutation at request start; exact-resume match; typed non-mutating mismatch refusal;
    cleared atomically with F := h at completion; absent after a start-time refusal;
    committed in stable_boundary_digest; excluded from engine_state_digest;
    snapshot-carried; restore-validated.
  - ObligationStore alongside the scheduler, with engine-owned preflighted atomic
    cross-store extraction (V2-05): any mismatch refuses before either store mutates.
  - Executable-only cohort identity and pacing; conflicted slots extracted and reported
    without identity or budget cost; all-conflicted slices consumed at zero cost, also
    after the oversized exception, so PAUSED implies at least one deferred executable
    slice (V2-10).
  - Multi-wave cohorts with wave-0 versus later-wave rejection (V2-06).
  - Compare-and-take with a full slot fingerprint; non-least and stale fingerprints are
    typed no-ops.
  - Command finalization at request completion with ordinal assignment at the frontier;
    a finalization refusal still completes the horizon (F := h) and clears ActiveRequest.

Every check from the 2026-09-06 model is carried forward (S1..S6) and re-run here.
"""
import copy
import hashlib
import json
import sys


def H(obj):
    return hashlib.sha256(
        json.dumps(obj, sort_keys=True, separators=(",", ":")).encode()
    ).hexdigest()[:16]


# ---------------------------------------------------------------- requests / discriminator
class Advance:
    kind = "advance"

    def __init__(self, T):
        self.T = T

    def horizon(self):
        return self.T

    def label(self):
        return f"Advance({self.T})"

    def discriminator(self):
        return {"kind": "advance", "h": self.T, "id": H(["request_advance_v1", self.T])}


class Command:
    kind = "command"

    def __init__(self, e, payload, cmd_id=None, profile="P"):
        self.e, self.payload, self.profile = e, payload, profile
        self.cmd_id = cmd_id if cmd_id is not None else f"cmd:{e}:{payload}"

    def horizon(self):
        return self.e

    def label(self):
        return f"Command@{self.e}({self.payload})"

    def discriminator(self):
        # identity = canonical command request fields, WITHOUT the sequencer ordinal,
        # which is assigned at finalization as the timeline frontier.
        return {
            "kind": "command",
            "h": self.e,
            "id": H(["request_command_v1", self.profile, self.e, self.cmd_id, self.payload]),
        }


def canon_active(active):
    return "active_request.none" if active is None else [
        "active_request.some", active["kind"], active["h"], active["id"]]


class Mailbox:
    """Bounded FIFO. Head stays until the consumer reports completion or rejection."""

    def __init__(self, capacity):
        self.capacity, self.q = capacity, []

    def offer(self, r):
        if len(self.q) >= self.capacity:
            return "BACKPRESSURE_MAILBOX_FULL"
        self.q.append(r)
        return "ACCEPTED"

    def head(self):
        return self.q[0] if self.q else None

    def pop(self):
        return self.q.pop(0)


TERMINAL = ("COMPLETED", "COMPLETED_COMMAND_NOT_FINALIZED",
            "REFUSED_HORIZON_BEHIND_FRONTIER", "REFUSED_ACTIVE_REQUEST_MISMATCH")


# ---------------------------------------------------------------- engine
class Engine:
    def __init__(self, rules, budget):
        self.rules, self.B = rules, budget
        self.sched = {}            # (due, profile) -> {key: ("S", payload) | ("C", [payload hashes])}
        self.obl = {}              # key -> ("S", payload) | ("C", [payloads])   (obligation store)
        self.cells = {}            # name -> {"v": int, "updated_at": int}
        self.timeline = []         # finalized commands [(ordinal, e, cmd_id, payload)]
        self.cmd_ids = set()       # finalized command-id registry (uniqueness)
        self.frontier_ordinal = 1
        self.F = 0                 # horizon frontier: horizon of the last COMPLETED request
        self.active = None         # ActiveRequest: Option<RequestDiscriminator>
        self.trace = []            # canonical per-cohort records (partition-invariant)
        self.reports = []          # canonical per-slice reports incl. conflicts
        self.diag = []             # noncanonical pacing diagnostics per call
        self.boundary_log = []     # stable-boundary digests at pause/completion
        self.f_log = []            # NONCANONICAL negative control per pre-wave capture

    # --- digests
    def scheduler_digest(self):
        return H(sorted((k[0], k[1], sorted(v.items())) for k, v in self.sched.items() if v))

    def engine_state_digest(self):     # v1 s8 composition: excludes F and ActiveRequest
        return H({"sched": self.scheduler_digest(), "obl": sorted(self.obl.items()),
                  "cells": self.cells, "timeline": self.timeline})

    def stable_boundary_digest(self):  # request-boundary commitment
        return H(["stable_boundary_v1", self.engine_state_digest(), self.F, canon_active(self.active)])

    # --- scheduling (Phase-1 shape: total; idempotent; distinct payload poisons)
    def schedule(self, due, profile, key, payload):
        slice_ = self.sched.setdefault((due, profile), {})
        cur = slice_.get(key)
        if cur is None:
            slice_[key] = ("S", payload)
            self.obl[key] = ("S", payload)
            return "Scheduled"
        if cur[0] == "S":
            if cur[1] == payload:
                return "AlreadyScheduledIdempotent"
            slice_[key] = ("C", sorted({H(cur[1]), H(payload)}))
            self.obl[key] = ("C", sorted({cur[1], payload}))
            return "Conflicted"
        slice_[key] = ("C", sorted(set(cur[1]) | {H(payload)}))
        self.obl[key] = ("C", sorted(set(self.obl[key][1]) | {payload}))
        return "Conflicted"

    # --- X-1 / X-2 / X-3 (scheduler level)
    def least_due_slice(self, horizon):
        cands = [k for k, v in self.sched.items() if v and k[0] <= horizon]
        return min(cands) if cands else None

    def fingerprint(self, k):
        return H(["due_slice_fingerprint_v1", k[0], k[1], sorted(self.sched[k].items())])

    def executable_count(self, k):
        return sum(1 for s in self.sched[k].values() if s[0] == "S")

    def take_least_due_slice(self, horizon, expected):
        k = self.least_due_slice(horizon)
        if k is None:
            return "NoSliceDue", None
        if self.fingerprint(k) != expected:
            return "SliceChanged", self.fingerprint(k)
        return "OK", (k, self.sched.pop(k))

    # --- engine-owned cross-store extraction (V2-05): preflight, then infallible apply
    def extract_least_due_slice(self, horizon):
        k = self.least_due_slice(horizon)
        if k is None:
            return "NoSliceDue", None
        fp = self.fingerprint(k)
        for key, slot in self.sched[k].items():          # preflight: read-only
            rec = self.obl.get(key)
            if rec is None:
                return "ObligationRecordMissing", key
            if slot[0] == "S" and not (rec[0] == "S" and rec[1] == slot[1]):
                return "ObligationRecordMismatch", key
            if slot[0] == "C" and not (rec[0] == "C" and sorted(H(p) for p in rec[1]) == slot[1]):
                return "ObligationRecordMismatch", key
        status, taken = self.take_least_due_slice(horizon, fp)   # apply: X-3 first ...
        assert status == "OK", "unreachable under the exclusive engine borrow"
        (due, profile), slots = taken
        executable, conflicted = [], []
        for key in sorted(slots):
            rec = self.obl.pop(key)                               # ... then obligation removal
            if slots[key][0] == "S":
                executable.append((key, slots[key][1], rec[1]))
            else:
                conflicted.append((key, slots[key][1]))
        return "OK", {"due": due, "profile": profile, "executable": executable,
                      "conflicted": conflicted, "fingerprint": fp}

    # --- evaluation (now = cohort canonical time; never the horizon)
    def eval_cohort(self, kind, now, profile, items):
        prewave = self.engine_state_digest()   # captured after extraction, before evaluation
        self.f_log.append({"t": now, "kind": kind, "prewave": prewave,
                           "F_at_capture": self.F, "active_at_capture": canon_active(self.active),
                           "prewave_if_F_included": H([prewave, self.F]),
                           "prewave_if_active_included": H([prewave, canon_active(self.active)])})
        identity = H(["scheduled_cohort_v1", profile, now, len(items), sorted(k for k, _, _ in items)]) \
            if kind == "scheduled" else H(["command_cohort", now, items[0][0]])
        waves = self.rules(kind, now, [(k, p) for k, p, _ in items], self.cells)
        batches, outcome = [], "committed"
        for w, effects in enumerate(waves):
            if any(e["op"] == "reject" for e in effects):
                outcome = f"rejected_wave_{w}"
                break
            for eff in sorted(effects, key=lambda e: json.dumps(e, sort_keys=True)):
                if eff["op"] == "decay":
                    c = self.cells.setdefault(eff["cell"], {"v": 100, "updated_at": 0})
                    elapsed = now - c["updated_at"]
                    assert elapsed >= 0, f"BACKWARDS EVALUATION now={now} updated_at={c['updated_at']}"
                    c["v"] = max(0, c["v"] - elapsed * eff["rate"])
                    c["updated_at"] = now
                elif eff["op"] == "set":
                    self.cells[eff["cell"]] = {"v": eff["v"], "updated_at": now}
                elif eff["op"] == "enqueue":
                    assert eff["due"] >= now + 1, "delayed work must be strictly later"
                    self.schedule(eff["due"], profile, eff["key"], eff["payload"])
            batches.append(H({"prewave": prewave, "kind": kind, "now": now, "profile": profile,
                              "wave": w, "identity": identity, "effects": effects}))
        rec = {"kind": kind, "t": now, "p": profile, "prewave": prewave, "identity": identity,
               "batches": batches, "outcome": outcome, "post": self.engine_state_digest()}
        self.trace.append(rec)
        return rec

    def _deferred(self, h):
        due = [k for k, v in self.sched.items() if v and k[0] <= h and self.executable_count(k) > 0]
        return len(due), (min(due)[0] if due else None)

    # --- one processing call of the presented request
    def process(self, req):
        h, d = req.horizon(), req.discriminator()
        if self.active is not None:
            if self.active != d:
                return "REFUSED_ACTIVE_REQUEST_MISMATCH"   # typed, non-mutating
            # exact resume: h >= F holds by construction (F unchanged since start)
        else:
            if h < self.F:
                return "REFUSED_HORIZON_BEHIND_FRONTIER"   # non-canonical; nothing mutated
            self.active = d                                 # before any cohort mutation
        r, admitted, exception_fired = self.B, 0, False
        while True:
            k = self.least_due_slice(h)
            if k is None:
                break
            e = self.executable_count(k)
            if e > 0 and (exception_fired or not (e <= r or admitted == 0)):
                return self._pause(req, h)                  # defer, untouched
            if e > 0 and e > r and admitted == 0:
                exception_fired = True
            status, ext = self.extract_least_due_slice(h)
            assert status == "OK", status                   # conforming loop never mismatches
            if ext["conflicted"]:
                self.reports.append({"t": ext["due"], "p": ext["profile"],
                                     "conflicted": ext["conflicted"]})
            if e > 0:
                self.eval_cohort("scheduled", ext["due"], ext["profile"], ext["executable"])
                admitted += 1
                r = 0 if exception_fired else r - e
        result = "COMPLETED"
        if isinstance(req, Command):
            if not (1 <= r or admitted == 0):
                return self._pause(req, h, command_deferred=True)
            if req.cmd_id in self.cmd_ids:                  # Phase-1 identity uniqueness refusal
                result = "COMPLETED_COMMAND_NOT_FINALIZED"
            else:                                           # stage + fence(one ordinal) + execute
                ordinal = self.frontier_ordinal
                self.timeline.append((ordinal, req.e, req.cmd_id, req.payload))
                self.cmd_ids.add(req.cmd_id)
                self.frontier_ordinal += 1
                self.eval_cohort("command", req.e, req.profile, [(f"cmd#{ordinal}", req.payload, None)])
        self.F, self.active = h, None                       # atomic completion
        self.boundary_log.append({"event": result, "req": req.label(), "F": self.F,
                                  "active": canon_active(self.active),
                                  "cohorts": len(self.trace), "sbd": self.stable_boundary_digest()})
        return result

    def _pause(self, req, h, command_deferred=False):
        n, earliest = self._deferred(h)
        assert n >= 1 or command_deferred, "PAUSED requires a deferred executable slice or command"
        self.diag.append({"call": req.label(), "deferred_cohort_count": n,
                          "earliest_deferred_due_time": earliest, "command_deferred": command_deferred})
        self.boundary_log.append({"event": "PAUSED", "req": req.label(), "F": self.F,
                                  "active": canon_active(self.active),
                                  "cohorts": len(self.trace), "sbd": self.stable_boundary_digest()})
        return "PAUSED"

    # --- snapshot / restore (stores + F + ActiveRequest)
    def snapshot(self):
        return copy.deepcopy({"sched": self.sched, "obl": self.obl, "cells": self.cells,
                              "timeline": self.timeline, "cmd_ids": sorted(self.cmd_ids),
                              "frontier_ordinal": self.frontier_ordinal, "F": self.F,
                              "active": self.active, "sbd": self.stable_boundary_digest()})

    @classmethod
    def restore(cls, rules, budget, snap):
        e = cls(rules, budget)
        s = copy.deepcopy(snap)
        e.sched, e.obl, e.cells, e.timeline = s["sched"], s["obl"], s["cells"], s["timeline"]
        e.cmd_ids, e.frontier_ordinal, e.F, e.active = set(s["cmd_ids"]), s["frontier_ordinal"], s["F"], s["active"]
        if e.active is not None and e.active["h"] < e.F:
            return "RESTORE_REJECTED_ACTIVE_BEHIND_FRONTIER", None
        if e.stable_boundary_digest() != s["sbd"]:
            return "RESTORE_REJECTED_DIGEST_MISMATCH", None
        return "OK", e


# ---------------------------------------------------------------- consumer
def drive(engine, mailbox, log, max_calls=100):
    calls = 0
    while mailbox.head() is not None and calls < max_calls:
        r = mailbox.head()
        res = engine.process(r)
        calls += 1
        log.append((r.label(), res))
        if res in TERMINAL:
            mailbox.pop()
    return calls


# ---------------------------------------------------------------- rules (waves)
def rules(kind, now, items, cells):
    """Returns a list of waves; each wave is a list of effects. 'reject' marks a rejected wave."""
    w0, w1 = [], []
    for key, payload in items:
        if kind == "scheduled":
            if payload == "A":
                w0 += [{"op": "decay", "cell": "x", "rate": 1},
                       {"op": "enqueue", "due": now + 5, "key": "D", "payload": "D"}]
            elif payload == "B":
                w0 += [{"op": "decay", "cell": "x", "rate": 1}]
            elif payload == "D":
                w0 += [{"op": "set", "cell": "y", "v": now}]
            elif payload == "T2":            # wave 0 sets w; threshold emits wave 1
                w0 += [{"op": "set", "cell": "w", "v": 1}]
                w1 += [{"op": "set", "cell": "w2", "v": now}]
            elif payload == "T2bad":         # wave 0 commits; wave 1 has unequal RESULTs
                w0 += [{"op": "set", "cell": "w", "v": 1}]
                w1 += [{"op": "reject", "why": "unequal same-family RESULT"}]
            elif payload == "W0bad":         # wave 0 rejects
                w0 += [{"op": "reject", "why": "unequal same-family RESULT"}]
            elif payload == "N":             # no-effect cohort
                pass
        elif kind == "command":
            w0 += [{"op": "set", "cell": "z", "v": payload}, {"op": "decay", "cell": "x", "rate": 1}]
    return [w0] + ([w1] if w1 else [])


def fresh(budget):
    e = Engine(rules, budget)
    e.schedule(10, "P", "A", "A")
    e.schedule(20, "P", "B", "B")
    return e


def canonical(e):
    return {"trace": e.trace, "reports": e.reports, "final_engine": e.engine_state_digest(),
            "F": e.F, "active": canon_active(e.active), "sbd": e.stable_boundary_digest()}


# ---------------------------------------------------------------- scenarios
out = {}


def check(name, cond, detail=""):
    out[name] = ("PASS" if cond else "FAIL") + (f" - {detail}" if detail else "")
    print(f"[{out[name].split(' ')[0]}] {name}{(' - ' + detail) if detail else ''}")


# ===== carried forward from the 2026-09-06 model =====
# S1: two requests, second arrives while the first is paused (budget 1)
e1 = fresh(1); mb = Mailbox(4); log1 = []
mb.offer(Advance(20))
log1.append((mb.head().label(), e1.process(mb.head())))
paused = log1[-1][1] == "PAUSED"
acc = mb.offer(Command(21, "c1"))
head_still_first = mb.head().label() == "Advance(20)"
drive(e1, mb, log1)
order1 = [r for r, res in log1 if res == "COMPLETED"]
check("S1 second request waits while first paused", paused and acc == "ACCEPTED" and head_still_first
      and order1 == ["Advance(20)", "Command@21(c1)"], f"calls={log1}")


def run_budget(b, reqs=None):
    e = fresh(b); mb = Mailbox(8); log = []
    for r in (reqs or [Advance(20), Command(21, "c1")]):
        mb.offer(r)
    drive(e, mb, log)
    return e, log


eb1, lb1 = run_budget(1); eb2, lb2 = run_budget(2); eb3, lb3 = run_budget(3)
check("S2 budgets 1 and 2 give identical canonical results", canonical(eb1) == canonical(eb2),
      f"budget1 calls={len(lb1)} budget2 calls={len(lb2)}; cohort order={[(t['kind'], t['t']) for t in eb1.trace]}")
check("S2 pacing diagnostics differ (noncanonical, expected)", eb1.diag != eb2.diag)

# S3: one catch-up vs stepwise completed catch-ups
eL = fresh(3); mbL = Mailbox(4); mbL.offer(Advance(20)); mbL.offer(Command(21, "c1")); drive(eL, mbL, [])
eS = fresh(3); mbS = Mailbox(8)
for T in (10, 12, 15, 17, 20):
    mbS.offer(Advance(T))
mbS.offer(Command(21, "c1")); drive(eS, mbS, [])
check("S3 one catch-up == stepwise catch-ups (per-cohort digests, final state, F, ActiveRequest)",
      canonical(eL) == canonical(eS), "created D@15 consumed inside horizon in both")
check("S3b stable-boundary digests agree at equal completed horizon",
      eL.boundary_log[-1]["sbd"] == eS.boundary_log[-1]["sbd"])
check("S3c stable-boundary digests differ at unequal completed horizons (F committed)",
      eL.boundary_log[0]["sbd"] != eS.boundary_log[0]["sbd"])
pairs = list(zip(eL.f_log, eS.f_log))
excl_equal = all(a["prewave"] == b["prewave"] for a, b in pairs)
inclF_equal = all(a["prewave_if_F_included"] == b["prewave_if_F_included"] for a, b in pairs)
inclA_equal = all(a["prewave_if_active_included"] == b["prewave_if_active_included"] for a, b in pairs)
check("S3d F excluded from per-cohort digest keeps equality; F included would break it",
      excl_equal and not inclF_equal,
      f"F at capture (t, one-call, stepwise)={[(a['t'], a['F_at_capture'], b['F_at_capture']) for a, b in pairs if a['F_at_capture'] != b['F_at_capture']]}")
check("S3e ActiveRequest included in per-cohort digest would break equality (negative control)",
      excl_equal and not inclA_equal,
      f"active at capture differs at every cohort: {[(a['t'], a['active_at_capture'][2], b['active_at_capture'][2]) for a, b in pairs]}")

# S3f: pre-wave pin against an independently built reference (kills whole-prefix draining)
ref = Engine(rules, 3); ref.schedule(20, "P", "B", "B")
ref_cells = {}  # cohort A@10 is the first cohort: cells still initial
check("S3f cohort@10 pre-wave digest == reference engine holding exactly {B@20} (whole-prefix drain killed)",
      eL.trace[0]["prewave"] == ref.engine_state_digest() and eL.trace[0]["prewave"] != H({"sched": H([]), "obl": [], "cells": {}, "timeline": []}))

# S4: past-dated input handling
e4 = fresh(2); mb4 = Mailbox(4); log4 = []
mb4.offer(Advance(20)); mb4.offer(Command(15, "late")); mb4.offer(Command(20, "ok_equal")); mb4.offer(Command(25, "ok"))
drive(e4, mb4, log4)
res4 = {r: s for r, s in log4}
check("S4 past-dated command rejected non-canonically; equal-time and later accepted",
      res4["Command@15(late)"] == "REFUSED_HORIZON_BEHIND_FRONTIER" and res4["Command@20(ok_equal)"] == "COMPLETED"
      and res4["Command@25(ok)"] == "COMPLETED" and all(c[3] != "late" for c in e4.timeline), f"timeline={e4.timeline}")
eA = Engine(rules, 2); eA.schedule(10, "P", "N", "N"); mbA = Mailbox(2); mbA.offer(Advance(100)); drive(eA, mbA, [])
eB = Engine(rules, 2); eB.schedule(20, "P", "N", "N"); mbB = Mailbox(2); mbB.offer(Advance(100)); drive(eB, mbB, [])
same_state = eA.stable_boundary_digest() == eB.stable_boundary_digest() and eA.engine_state_digest() == eB.engine_state_digest()
dA, dB = eA.process(Command(15, "x")), eB.process(Command(15, "x"))
check("S4b equal digests -> identical decision on Command@15", same_state and dA == dB == "REFUSED_HORIZON_BEHIND_FRONTIER")

# S5: restart while paused: snapshot (stores, F, ActiveRequest) + consumer re-presents head
e5 = fresh(1); mb5 = Mailbox(4); log5 = []
mb5.offer(Advance(20)); mb5.offer(Command(21, "c1"))
log5.append((mb5.head().label(), e5.process(mb5.head())))
snap = e5.snapshot(); trace_before = copy.deepcopy(e5.trace); blog_before = copy.deepcopy(e5.boundary_log)
st, e5r = Engine.restore(rules, 1, snap); e5r.trace, e5r.boundary_log, e5r.reports = trace_before, blog_before, copy.deepcopy(e5.reports)
drive(e5r, mb5, log5)
check("S5 restart from (stores, F, ActiveRequest) with the exact head re-presented == uninterrupted run",
      st == "OK" and canonical(e5r) == canonical(eb1) and snap["active"] == Advance(20).discriminator())
check("S6 no backwards decay evaluation in any scenario", True, "assert elapsed >= 0 held in every cohort")

# ===== SB-01 and the ActiveRequest corrections =====
# S7: pause after processing through 15, substitute Command@12 -> typed no-op refusal
e7 = Engine(rules, 3)
for t, p in ((10, "A"), (15, "N"), (20, "B")):
    e7.schedule(t, "P", p, p)
r7 = e7.process(Advance(20))                                   # consumes 10 and 15 (N and created D), pauses before 20
before = (e7.stable_boundary_digest(), e7.engine_state_digest(), copy.deepcopy(e7.timeline), e7.F, e7.active)
sub = e7.process(Command(12, "sub"))
after = (e7.stable_boundary_digest(), e7.engine_state_digest(), copy.deepcopy(e7.timeline), e7.F, e7.active)
check("S7 SB-01: paused-request substitution Command@12 refused, byte-identical state",
      r7 == "PAUSED" and max(c["updated_at"] for c in e7.cells.values()) == 15
      and sub == "REFUSED_ACTIVE_REQUEST_MISMATCH" and before == after and e7.F == 0,
      f"cells updated_at max=15 while F=0; refusal mutated nothing")
sub2 = e7.process(Advance(25))
check("S7b higher-horizon Advance(25) also refused while Advance(20) is active",
      sub2 == "REFUSED_ACTIVE_REQUEST_MISMATCH" and e7.stable_boundary_digest() == before[0])
res7 = e7.process(Advance(20))
check("S7c exact head resumes and completes; ActiveRequest cleared, F=20",
      res7 == "COMPLETED" and e7.active is None and e7.F == 20)

# S8: restart matching
e8 = Engine(rules, 3)
for t, p in ((10, "A"), (15, "N"), (20, "B")):
    e8.schedule(t, "P", p, p)
e8.process(Advance(20)); snap8 = e8.snapshot()
st_a, e8a = Engine.restore(rules, 3, snap8); res8a = e8a.process(Advance(20))
e8u = Engine(rules, 3)
for t, p in ((10, "A"), (15, "N"), (20, "B")):
    e8u.schedule(t, "P", p, p)
e8u.process(Advance(20)); e8u.process(Advance(20))
check("S8 restart + exact request => byte-identical continuation",
      st_a == "OK" and res8a == "COMPLETED" and e8a.stable_boundary_digest() == e8u.stable_boundary_digest()
      and e8a.engine_state_digest() == e8u.engine_state_digest())
st_b, e8b = Engine.restore(rules, 3, snap8); sbd_b = e8b.stable_boundary_digest()
res8b = e8b.process(Command(20, "same_horizon_other"))
check("S8b restart + same-horizon different request (Command@20) => refused, byte-identical",
      st_b == "OK" and res8b == "REFUSED_ACTIVE_REQUEST_MISMATCH" and e8b.stable_boundary_digest() == sbd_b)
snap8c = copy.deepcopy(snap8); snap8c["F"] = 30; snap8c["active"] = {"kind": "advance", "h": 5, "id": "x"}
st_c, _ = Engine.restore(rules, 3, snap8c)
snap8d = copy.deepcopy(snap8); snap8d["active"] = None
st_d, _ = Engine.restore(rules, 3, snap8d)
check("S8c restore validation: active behind frontier rejected; tampered ActiveRequest fails the digest",
      st_c == "RESTORE_REJECTED_ACTIVE_BEHIND_FRONTIER" and st_d == "RESTORE_REJECTED_DIGEST_MISMATCH")

# S9: digest discrimination and equivalence
g1 = fresh(2); g2 = fresh(2); g2.active = Advance(20).discriminator()
g3 = fresh(2); g3.active = Command(20, "c").discriminator()
g4 = fresh(2); g4.active = Advance(21).discriminator()
g5 = fresh(2); g5.active = Command(20, "c2").discriminator()
esd_equal = len({g.engine_state_digest() for g in (g1, g2, g3, g4, g5)}) == 1
sbd_distinct = len({g.stable_boundary_digest() for g in (g1, g2, g3, g4, g5)}) == 5
check("S9 discrimination: None vs Some, kind, horizon, identity all change stable_boundary_digest; engine_state_digest unchanged",
      esd_equal and sbd_distinct)
q1 = Engine(rules, 1); q1.schedule(20, "P", "B", "B"); q1.schedule(10, "P", "A", "A"); q1.process(Advance(20))
q2 = fresh(1); q2.process(Advance(20))
eq_before = q1.stable_boundary_digest() == q2.stable_boundary_digest()
n1, n2 = q1.process(Command(12, "x")), q2.process(Command(12, "x"))
m1, m2 = q1.process(Advance(20)), q2.process(Advance(20))
check("S9b equivalence: equal stable_boundary_digest (permuted construction) => identical response to the same next requests",
      eq_before and n1 == n2 == "REFUSED_ACTIVE_REQUEST_MISMATCH" and m1 == m2 == "PAUSED"
      and q1.stable_boundary_digest() == q2.stable_boundary_digest())

# S10: cleanup after completion and after start-time refusal
e10 = fresh(4); e10.process(Advance(20))
c1 = e10.active is None and e10.F == 20
sbd10 = e10.stable_boundary_digest()
r10 = e10.process(Advance(5))
check("S10 ActiveRequest absent after COMPLETED and after HORIZON_BEHIND_FRONTIER refusal; refusal mutates neither F nor digest",
      c1 and r10 == "REFUSED_HORIZON_BEHIND_FRONTIER" and e10.active is None and e10.F == 20 and e10.stable_boundary_digest() == sbd10)
r10b = e10.process(Advance(20))
check("S10b equal-horizon Advance after completion is accepted and idempotent",
      r10b == "COMPLETED" and e10.stable_boundary_digest() == sbd10 and e10.active is None)

# S12: budgets 1,2,3 identical per-cohort; stable-boundary state equal only at equal progress points
same_trace = eb1.trace == eb2.trace == eb3.trace and eb1.reports == eb2.reports == eb3.reports
def progress_points(e):
    return {(b["cohorts"], b["active"] if isinstance(b["active"], str) else tuple(b["active"])): b["sbd"] for b in e.boundary_log}
pp = [progress_points(e) for e in (eb1, eb2, eb3)]
consistent = all(pp[i][k] == pp[j][k] for i in range(3) for j in range(3) for k in pp[i] if k in pp[j])
check("S12 budgets 1/2/3: per-cohort values equal by position; stable-boundary digests equal at equal progress points",
      same_trace and consistent and eb1.boundary_log[-1]["sbd"] == eb2.boundary_log[-1]["sbd"] == eb3.boundary_log[-1]["sbd"],
      f"calls={len(lb1)},{len(lb2)},{len(lb3)}")

# S13: cross-store preflight is atomic before either store mutates
e13 = fresh(2); e13.process(Advance(0))     # nothing due at 0: completes, F=0
del e13.obl["A"]                              # corrupt: missing record for a scheduled key
d_s, d_e = e13.scheduler_digest(), e13.engine_state_digest()
st13, key13 = e13.extract_least_due_slice(20)
ok13 = st13 == "ObligationRecordMissing" and key13 == "A" and e13.scheduler_digest() == d_s and e13.engine_state_digest() == d_e
e13.obl["A"] = ("S", "A-wrong")               # corrupt: mismatched record
d_s, d_e = e13.scheduler_digest(), e13.engine_state_digest()
st13b, _ = e13.extract_least_due_slice(20)
ok13b = st13b == "ObligationRecordMismatch" and e13.scheduler_digest() == d_s and e13.engine_state_digest() == d_e
e13.obl["A"] = ("S", "A")
st13c, ext13 = e13.extract_least_due_slice(20)
ok13c = st13c == "OK" and ext13["executable"][0][2] == "A" and "A" not in e13.obl and (10, "P") not in e13.sched
check("S13 V2-05: missing/mismatched obligation record refuses before either store mutates; repaired store extracts with the record",
      ok13 and ok13b and ok13c)

# S14: wave-0 versus later-wave rejection (V2-06)
e14a = Engine(rules, 4); e14a.schedule(10, "P", "W0bad", "W0bad"); e14a.process(Advance(10))
e14b = Engine(rules, 4); e14b.schedule(20, "P", "T2bad", "T2bad"); e14b.process(Advance(20))
e14c = Engine(rules, 4); e14c.schedule(30, "P", "T2", "T2"); e14c.process(Advance(30))
t0, t1, t2 = e14a.trace[0], e14b.trace[0], e14c.trace[0]
check("S14 wave-0 rejection: final state == post-extraction digest; later-wave rejection: earlier wave stays committed (no cohort rollback)",
      t0["outcome"] == "rejected_wave_0" and t0["post"] == t0["prewave"] and e14a.cells == {}
      and t1["outcome"] == "rejected_wave_1" and t1["post"] != t1["prewave"] and e14b.cells == {"w": {"v": 1, "updated_at": 20}}
      and t2["outcome"] == "committed" and e14c.cells["w2"]["v"] == 30
      and "W0bad" not in e14a.obl and "T2bad" not in e14b.obl and not e14a.sched.get((10, "P")) and not e14b.sched.get((20, "P")),
      "keys terminally consumed on every route")

# S15: conflicted slots: identity/pacing executable-only; V2-04 equal identity and equal prewave for {S} vs {S,X}
ea = Engine(rules, 2); ea.schedule(100, "P", "S", "B"); ea.schedule(101, "P", "T", "B")
eb = Engine(rules, 2); eb.schedule(100, "P", "S", "B"); eb.schedule(100, "P", "X", "p1"); eb.schedule(100, "P", "X", "p2"); eb.schedule(101, "P", "T", "B")
ra, rb = ea.process(Advance(101)), eb.process(Advance(101))
check("S15 V2-04: {S} vs {S, X_conflicted}: equal cohort identity, pre-wave digest, batches, final state; only the conflict report differs",
      ra == rb == "COMPLETED" and ea.trace == eb.trace and ea.engine_state_digest() == eb.engine_state_digest()
      and ea.reports == [] and eb.reports and eb.reports[0]["conflicted"][0][0] == "X")
ec = Engine(rules, 2); ec.schedule(100, "P", "S", "B"); ec.schedule(100, "P", "X", "p1"); ec.schedule(100, "P", "X", "p2"); ec.schedule(101, "P", "T", "B")
rc = ec.process(Advance(101))
ed = Engine(rules, 1); ed.schedule(100, "P", "S", "B"); ed.schedule(100, "P", "X", "p1"); ed.schedule(100, "P", "X", "p2"); ed.schedule(101, "P", "T", "B")
rd = ed.process(Advance(101))
check("S15b V2-03: budget 2 admits S then T (executable-only count); budget 1 admits S and defers T with count/diagnostics",
      rc == "COMPLETED" and len(ec.trace) == 2 and rd == "PAUSED" and len(ed.trace) == 1
      and ed.diag[-1]["deferred_cohort_count"] == 1 and ed.diag[-1]["earliest_deferred_due_time"] == 101)

# S16: all-conflicted slices cost nothing, are consumed after the oversized exception, PAUSED => deferred >= 1
e16 = Engine(rules, 2)
for k in ("a", "b", "c"):
    e16.schedule(100, "P", k, "B")            # oversized executable slice (3 > 2)
e16.schedule(101, "P", "x", "p1"); e16.schedule(101, "P", "x", "p2")   # all-conflicted
e16.schedule(102, "P", "y", "p1"); e16.schedule(102, "P", "y", "p2")   # all-conflicted
e16.schedule(103, "P", "T", "B")                                      # executable, deferred
r16 = e16.process(Advance(103))
check("S16 V2-10: oversized exception admits 100 whole; all-conflicted 101/102 consumed at zero cost after it; 103 deferred; PAUSED => deferred_cohort_count == 1",
      r16 == "PAUSED" and len(e16.trace) == 1 and [rp["t"] for rp in e16.reports] == [101, 102]
      and (101, "P") not in e16.sched and (102, "P") not in e16.sched and e16.diag[-1]["deferred_cohort_count"] == 1
      and e16.diag[-1]["earliest_deferred_due_time"] == 103)
e16b = Engine(rules, 2); e16b.schedule(100, "P", "x", "p1"); e16b.schedule(100, "P", "x", "p2")
for k in ("a", "b", "c"):
    e16b.schedule(101, "P", k, "B")
e16b.schedule(102, "P", "T", "B")
r16b = e16b.process(Advance(102))
check("S16b all-conflicted slice before an oversized slice does not disturb the exception premise",
      r16b == "PAUSED" and len(e16b.trace) == 1 and e16b.trace[0]["t"] == 101 and e16b.diag[-1]["deferred_cohort_count"] == 1)

# S17: command finalization refusal at completion still completes the horizon
e17 = fresh(4); mb17 = Mailbox(4); log17 = []
mb17.offer(Command(21, "c1", cmd_id="k1")); mb17.offer(Command(25, "dup", cmd_id="k1")); drive(e17, mb17, log17)
check("S17 duplicate command identity at finalization: COMPLETED_COMMAND_NOT_FINALIZED, F=25, ActiveRequest cleared, timeline unchanged",
      log17[-1][1] == "COMPLETED_COMMAND_NOT_FINALIZED" and e17.F == 25 and e17.active is None
      and len(e17.timeline) == 1 and e17.frontier_ordinal == 2)
r17 = e17.process(Command(24, "late", cmd_id="k9"))
check("S17b after the refused finalization, a request behind F=25 is still refused at start", r17 == "REFUSED_HORIZON_BEHIND_FRONTIER")

# S18: compare-and-take: non-least and stale fingerprints are typed no-ops
e18 = fresh(2); d18 = e18.engine_state_digest()
fp_nonleast = e18.fingerprint((20, "P"))
st18, obs = e18.take_least_due_slice(20, fp_nonleast)
fp_least = e18.fingerprint((10, "P")); e18.schedule(10, "P", "A", "A-other")     # same key, different payload -> conflicted
st18b, _ = e18.take_least_due_slice(20, fp_least)
check("S18 non-least fingerprint => SliceChanged with the live least fingerprint; stale fingerprint after Scheduled->Conflicted => no-op",
      st18 == "SliceChanged" and obs == fp_least and st18b == "SliceChanged" and e18.engine_state_digest() != d18
      and (10, "P") in e18.sched and e18.sched[(10, "P")]["A"][0] == "C")

# S19: command cohort weighs one unit; deferred command executes on the next call with identical result
e19a = fresh(1); mb19a = Mailbox(2); l19a = []; mb19a.offer(Command(21, "c1")); drive(e19a, mb19a, l19a)
e19b = fresh(9); mb19b = Mailbox(2); l19b = []; mb19b.offer(Command(21, "c1")); drive(e19b, mb19b, l19b)
check("S19 command deferred by budget executes on a later call; canonical result identical to the unbudgeted run",
      canonical(e19a) == canonical(e19b) and len(l19a) > len(l19b) and any(d.get("command_deferred") for d in e19a.diag))

json.dump({"checks": out, "trace_budget1": eb1.trace, "diag_budget1": eb1.diag, "diag_budget2": eb2.diag,
           "boundary_budget1": eb1.boundary_log, "boundary_budget2": eb2.boundary_log,
           "S7_refusal_state": before, "S9_sbd": [g.stable_boundary_digest() for g in (g1, g2, g3, g4, g5)],
           "S16_diag": e16.diag, "S17_log": log17},
          open(sys.argv[1] if len(sys.argv) > 1 else "results.json", "w"), indent=1, sort_keys=True, default=str)
print("ALL PASS" if all(v.startswith("PASS") for v in out.values()) else "SOME FAIL")
