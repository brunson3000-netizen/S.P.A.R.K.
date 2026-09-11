#!/usr/bin/env python3
"""
Disposable architecture model: V3-F01 bounded correction, Revision 2.

Architecture evidence only. Not production code, not a benchmark, not a transport, not a
persistence backend, and not crash-recovery evidence (snapshots are deep copies).
Standard library only. Derived from
engineering/phase2/v3_f01_final_correction_model_2026-09-10/model.py (preserved unchanged),
with these Revision-2 changes:

  FINAL-01  A Phase-1-faithful timeline (slots, poison evidence, staged and finalized
            identity indexes, last finalized source sequence, fences, epoch resets,
            window/frontier arithmetic at u64 bounds) replaces the flat command list.
            Three finalization compositions are modelled:
              OLD  - the reviewed candidate's live stage-then-fence recipe (negative control);
              REV2 - complete read-only preflight, then the entailed live stage + fence;
              REF  - an isolated deep-copy working copy, used only as a differential oracle.
            Every refusal route compares the COMPLETE timeline state before and after.
  FINAL-02  REFUSED_ACTIVE_REQUEST_MISMATCH is not dequeue-eligible; the consumer pops only
            a head it presented and that received a result terminal for that request; a
            restore/mailbox disagreement halts the consumer fail-closed.
  FINAL-03  History-only replay is scoped to completed boundaries with explicit timeline
            metadata and payloads; paused recovery uses the committed snapshot plus the exact
            durable request.
  Oracle    Same-kind/same-horizon substitution; honest redundant-field discrimination with
            encoding pins; equal residual stores versus different removed sets; outer-loop
            selection counting; spelled-out successive lifecycle results.

Every check of the final-correction model (34) is carried and re-run here.
"""
import copy
import hashlib
import json
import sys

U64_MAX = 2 ** 64 - 1


def H(obj):
    return hashlib.sha256(
        json.dumps(obj, sort_keys=True, separators=(",", ":")).encode()
    ).hexdigest()[:16]


# ---------------------------------------------------------------- Phase-1 timeline mirror
def sem_hash(env):
    return H(["semantic_envelope_identity", env["profile"], env["epoch"], env["e"], env["source"],
              env["seq"], env["ordinal"], env["command_id"], env["kind"], env["payload_hash"]])


def seq_key(source, seq):
    return f"{source}#{seq}"


class Timeline:
    """Mirrors crates/spark-core/src/timeline.rs TimelineIngress: stage (lines 1226-1393),
    submit_fence (1435-1598), reset_epoch (1608-1638), digests (1656-1742). Poison evidence is
    an unbounded sorted set here (the Phase-1 caps are not exercised by this model)."""

    def __init__(self, profile="P", epoch=0, sequencer="seq", width=2, frontier=0):
        self.profile, self.epoch, self.sequencer, self.width = profile, epoch, sequencer, width
        self.frontier = frontier
        self.last_fence_hash = H(["timeline_genesis", profile, epoch, frontier])
        self.slots = {}          # ordinal -> ["staged", env, hash] | ["poisoned", [hashes]]
        self.staged_cmd, self.staged_seq = {}, {}
        self.fin_cmd, self.fin_seq, self.last_fin_seq = {}, {}, {}
        self.finalized, self.fences, self.resets = [], [], []

    def full_state(self):
        """Every field, private indexes included: the model analogue of the derived Debug."""
        return copy.deepcopy({k: v for k, v in self.__dict__.items()})

    @classmethod
    def from_state(cls, s):
        t = cls.__new__(cls)
        t.__dict__.update(copy.deepcopy(s))
        return t

    def window_end(self):
        end = self.frontier + (self.width - 1)
        return None if end > U64_MAX else end

    def token(self):
        return H(["admission_window_token", self.profile, self.epoch, self.frontier, self.width,
                  self.last_fence_hash])

    def current_window(self):
        we = self.window_end()
        return None if we is None else {"frontier": self.frontier, "end": we, "ticket": self.token()}

    def stage(self, sequencer, env, ticket):
        if env["profile"] != self.profile:
            return ("err", "WrongProfile")
        if env["epoch"] != self.epoch:
            return ("err", "WrongTimelineEpoch")
        if sequencer != self.sequencer:
            return ("err", "NotActiveSequencer")
        we = self.window_end()
        if we is None:
            return ("err", "OrdinalSpaceExhausted")
        o = env["ordinal"]
        if o < self.frontier or o > we:
            return ("not_in_admission_window",)
        if ticket != self.token():
            return ("err", "StaleOrInvalidAdmissionTicket")
        h = sem_hash(env)
        slot = self.slots.get(o)
        if slot is None:
            cid, sk = env["command_id"], seq_key(env["source"], env["seq"])
            ex = self.fin_cmd[cid] if cid in self.fin_cmd else self.staged_cmd.get(cid)
            if ex is not None and ex != h:
                return ("err", "CommandIdentityConflict")
            ex = self.fin_seq[sk] if sk in self.fin_seq else self.staged_seq.get(sk)
            if ex is not None and ex != h:
                return ("err", "SourceSequenceConflict")
            self.slots[o] = ["staged", copy.deepcopy(env), h]
            self.staged_cmd[cid] = h
            self.staged_seq[sk] = h
            return ("ack", "newly_staged", o, h)
        if slot[0] == "poisoned":
            slot[1] = sorted(set(slot[1]) | {h})             # evidence recorded: a mutation
            return ("poisoned",)
        if slot[2] == h:
            return ("ack", "already_staged_idempotent", o, h)
        displaced = slot[1]
        del self.staged_cmd[displaced["command_id"]]
        del self.staged_seq[seq_key(displaced["source"], displaced["seq"])]
        self.slots[o] = ["poisoned", sorted({slot[2], h})]
        return ("poisoned",)

    def submit_fence(self, sequencer, f):
        if f["profile"] != self.profile:
            return ("err", "WrongProfile")
        if f["epoch"] != self.epoch:
            return ("err", "WrongTimelineEpoch")
        if sequencer != self.sequencer:
            return ("err", "NotActiveSequencer")
        if f["start"] != self.frontier:
            return ("err", "StartNotAtFrontier")
        we = self.window_end()
        if we is None:
            return ("err", "OrdinalSpaceExhausted")
        if f["end"] < f["start"] or f["end"] > we:
            return ("err", "EndOutsideStageableHorizon")
        if f["prev"] != self.last_fence_hash:
            return ("err", "PreviousFenceHashMismatch")
        new_frontier = f["end"] + 1
        if new_frontier > U64_MAX:
            return ("err", "OrdinalSpaceExhausted")
        ordered = []
        for o in range(f["start"], f["end"] + 1):
            slot = self.slots.get(o)
            if slot is None:
                return ("err", "RangeNotFullyStaged")
            if slot[0] == "poisoned":
                return ("err", "RangeContainsPoisoned")
            ordered.append((o, slot[1], slot[2]))
        if H(["ordered_stream_digest", [[o, h] for o, _, h in ordered]]) != f["osd"]:
            return ("err", "DigestMismatch")
        running = {}
        for _, env, _ in ordered:
            prior = running.get(env["source"], self.last_fin_seq.get(env["source"]))
            if prior is not None and env["seq"] <= prior:
                return ("err", "SourceSequenceNotIncreasing", prior, env["seq"])
            running[env["source"]] = env["seq"]
        for o, env, h in ordered:                               # promote atomically
            sk = seq_key(env["source"], env["seq"])
            self.staged_cmd.pop(env["command_id"], None)
            self.staged_seq.pop(sk, None)
            self.fin_cmd[env["command_id"]] = h
            self.fin_seq[sk] = h
            self.last_fin_seq[env["source"]] = env["seq"]
            self.finalized.append([o, env, h])
        self.last_fence_hash = H(["timeline_fence", f])
        self.fences.append(copy.deepcopy(f))
        self.frontier = new_frontier
        self.slots = {o: s for o, s in self.slots.items() if o >= new_frontier}
        return ("ok", {"start": f["start"], "end": f["end"], "count": len(ordered), "frontier": new_frontier})

    def reset_epoch(self, requesting, new_epoch, new_sequencer):
        if requesting != self.sequencer:
            return ("err", "NotActiveSequencer")
        if new_epoch <= self.epoch:
            return ("err", "EpochNotIncreasing")
        rec = [len(self.resets), self.epoch, new_epoch, new_sequencer, self.frontier]
        self.slots, self.staged_cmd, self.staged_seq = {}, {}, {}
        self.epoch, self.sequencer = new_epoch, new_sequencer
        self.resets.append(rec)
        return ("ok", rec)

    def history_digest(self):
        return H(["timeline_finalized_history", self.profile, self.frontier, self.last_fence_hash,
                  self.finalized, self.fences, self.resets])

    def state_digest(self):
        return H(["timeline_ingress_state", self.history_digest(), self.epoch, self.sequencer, self.width,
                  sorted([o, s] for o, s in self.slots.items())])


def make_fence(t, env):
    o = env["ordinal"]
    return {"profile": t.profile, "epoch": t.epoch, "fence_id": f"fence.{o}", "start": o, "end": o,
            "prev": t.last_fence_hash, "osd": H(["ordered_stream_digest", [[o, sem_hash(env)]]])}


# ---------------------------------------------------------------- the three finalization compositions
def finalize_old(t, grant, req):
    """The reviewed candidate's recipe (FINAL candidate section 8.2-8.3): live stage, then live fence, no undo."""
    w = t.current_window()
    if w is None:
        return "OrdinalSpaceExhausted(window)"
    env = req.envelope(w["frontier"])
    d = t.stage(grant, env, w["ticket"])
    if d[0] != "ack":
        return f"stage:{d[1] if d[0] == 'err' else d[0]}"
    r = t.submit_fence(grant, make_fence(t, env))
    return None if r[0] == "ok" else f"fence:{r[1]}"


def preflight(t, grant, req):
    """REV2 step 1: complete, read-only. Returns (refusal, None) or (None, envelope)."""
    if req.profile != t.profile:
        return ("WrongProfile", None)
    if req.epoch != t.epoch:
        return ("WrongTimelineEpoch", None)
    if grant != t.sequencer:
        return ("NotActiveSequencer", None)
    w = t.current_window()
    if w is None:
        return ("OrdinalSpaceExhausted(window)", None)
    n = w["frontier"]
    if n + 1 > U64_MAX:
        return ("OrdinalSpaceExhausted(frontier)", None)
    if t.slots:                                   # clean-staging precondition (fail closed)
        return ("UnexpectedStagingState", None)
    assert not t.staged_cmd and not t.staged_seq  # derived-index invariant: no slot => empty
    env = req.envelope(n)
    h = sem_hash(env)
    if t.fin_cmd.get(env["command_id"], h) != h:
        return ("CommandIdentityConflict", None)
    if t.fin_seq.get(seq_key(env["source"], env["seq"]), h) != h:
        return ("SourceSequenceConflict", None)
    last = t.last_fin_seq.get(env["source"])
    if last is not None and env["seq"] <= last:
        return ("SourceSequenceNotIncreasing", None)
    return (None, env)


def finalize_rev2(t, grant, req, oplog=None):
    """REV2: preflight, then the entailed apply. The asserts are the entailment lemma."""
    refusal, env = preflight(t, grant, req)
    if refusal is not None:
        return refusal
    w = t.current_window()
    d = t.stage(grant, env, w["ticket"])
    assert d[:2] == ("ack", "newly_staged") and d[2] == env["ordinal"], d
    if oplog is not None:
        oplog.append(("positive_staged_ack", d[1], d[2]))
    r = t.submit_fence(grant, make_fence(t, env))
    assert r[0] == "ok" and (r[1]["start"], r[1]["end"], r[1]["count"]) == (env["ordinal"], env["ordinal"], 1), r
    if oplog is not None:
        oplog.append(("one_ordinal_fence", r[1]["start"], r[1]["frontier"]))
    return None


def finalize_ref(t, grant, req):
    """Differential reference: unchanged calls on an isolated working copy; returns (verdict, copy)."""
    work = Timeline.from_state(t.full_state())
    return finalize_old(work, grant, req), work


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

    def __init__(self, e, payload, cmd_id=None, profile="P", epoch=0, source="s", seq=None, ckind="probe.command"):
        self.e, self.payload, self.profile, self.epoch = e, payload, profile, epoch
        self.cmd_id = cmd_id if cmd_id is not None else f"cmd.{e}.{payload}"
        self.source, self.seq, self.ckind = source, (seq if seq is not None else e), ckind

    def horizon(self):
        return self.e

    def label(self):
        return f"Command@{self.e}({self.payload})"

    def fields(self):   # SemanticCommandEnvelope field order with input_ordinal omitted
        return [self.profile, self.epoch, self.e, self.source, self.seq, self.cmd_id, self.ckind, H(self.payload)]

    def discriminator(self):
        return {"kind": "command", "h": self.e, "id": H(["request_command_v1"] + self.fields())}

    def envelope(self, ordinal):
        return {"command_id": self.cmd_id, "profile": self.profile, "epoch": self.epoch, "e": self.e,
                "source": self.source, "seq": self.seq, "ordinal": ordinal, "kind": self.ckind,
                "payload_hash": H(self.payload)}


def canon_active(active):
    return "active_request.none" if active is None else [
        "active_request.some", active["kind"], active["h"], active["id"]]


class Mailbox:
    """Bounded FIFO. The head stays until the consumer receives a result terminal for it."""

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

    def labels(self):
        return [r.label() for r in self.q]


# Results terminal FOR THE PRESENTED REQUEST (Revision 2). Mismatch terminates nothing.
TERMINAL_FOR_PRESENTED = ("COMPLETED", "COMPLETED_COMMAND_NOT_FINALIZED", "REFUSED_HORIZON_BEHIND_FRONTIER")
# The reviewed candidate's rule, retained only as a negative control.
TERMINAL_V1 = TERMINAL_FOR_PRESENTED + ("REFUSED_ACTIVE_REQUEST_MISMATCH",)
HALT = "CONSUMER_HALTED_ACTIVE_REQUEST_NOT_AT_HEAD"


# ---------------------------------------------------------------- engine
class Engine:
    finalize_mode = "rev2"

    def __init__(self, rules, budget, width=2):
        self.rules, self.B = rules, budget
        self.sched = {}            # (due, profile) -> {key: ("S", payload) | ("C", [payload hashes])}
        self.obl = {}              # key -> ("S", payload) | ("C", [payloads])   (obligation store)
        self.cells = {}            # name -> {"v": int, "updated_at": int}
        self.timeline = Timeline(profile="P", sequencer="seq", width=width)
        self.grant = "seq"         # the engine's sequencer grant (D-1)
        self.F = 0
        self.active = None
        self.trace, self.reports, self.diag, self.boundary_log, self.f_log = [], [], [], [], []
        self.extractions = []      # removed sets per extraction (oracle precision P-c)
        self.sel_outer = 0         # X-1 selections at the outer-loop seam A2 only
        self.sel_total = 0         # every least-slice location, any seam
        self.last_refusal = None
        self.finalize_log = []

    # --- digests
    def scheduler_digest(self):
        return H(sorted((k[0], k[1], sorted(v.items())) for k, v in self.sched.items() if v))

    def engine_state_digest(self):     # v1 s8 composition: excludes F and ActiveRequest
        return H({"sched": self.scheduler_digest(), "obl": sorted(self.obl.items()),
                  "cells": self.cells, "timeline": self.timeline.state_digest()})

    def stable_boundary_digest(self, canon=None):
        return H(["stable_boundary_v1", self.engine_state_digest(), self.F, (canon or canon_active)(self.active)])

    # --- scheduling (Phase-1 shape)
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

    def least_due_slice(self, horizon, seam="internal"):
        self.sel_total += 1
        if seam == "outer":
            self.sel_outer += 1
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

    def extract_least_due_slice(self, horizon):
        k = self.least_due_slice(horizon)
        if k is None:
            return "NoSliceDue", None
        fp = self.fingerprint(k)
        for key, slot in self.sched[k].items():
            rec = self.obl.get(key)
            if rec is None:
                return "ObligationRecordMissing", key
            if slot[0] == "S" and not (rec[0] == "S" and rec[1] == slot[1]):
                return "ObligationRecordMismatch", key
            if slot[0] == "C" and not (rec[0] == "C" and sorted(H(p) for p in rec[1]) == slot[1]):
                return "ObligationRecordMismatch", key
        status, taken = self.take_least_due_slice(horizon, fp)
        assert status == "OK", "unreachable under the exclusive engine borrow"
        (due, profile), slots = taken
        executable, conflicted, removed = [], [], []
        for key in sorted(slots):
            rec = self.obl.pop(key)
            removed.append([key, rec[0], rec[1]])
            if slots[key][0] == "S":
                executable.append((key, slots[key][1], rec[1]))
            else:
                conflicted.append((key, slots[key][1]))
        self.extractions.append({"t": due, "p": profile, "removed": removed})
        return "OK", {"due": due, "profile": profile, "executable": executable,
                      "conflicted": conflicted, "fingerprint": fp}

    # --- evaluation (now = cohort canonical time)
    def eval_cohort(self, kind, now, profile, items):
        prewave = self.engine_state_digest()
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

    def _matches(self, active, d):
        return active == d

    def _finalize(self, req):
        if self.finalize_mode == "old":
            return finalize_old(self.timeline, self.grant, req)
        return finalize_rev2(self.timeline, self.grant, req, self.finalize_log)

    # --- one processing call of the presented request
    def process(self, req):
        h, d = req.horizon(), req.discriminator()
        if self.active is not None:
            if not self._matches(self.active, d):
                return "REFUSED_ACTIVE_REQUEST_MISMATCH"
        else:
            if h < self.F:
                return "REFUSED_HORIZON_BEHIND_FRONTIER"
            self.active = d
        r, admitted, exception_fired = self.B, 0, False
        while True:
            k = self.least_due_slice(h, seam="outer")
            if k is None:
                break
            e = self.executable_count(k)
            if e > 0 and (exception_fired or not (e <= r or admitted == 0)):
                return self._pause(req, h)
            if e > 0 and e > r and admitted == 0:
                exception_fired = True
            status, ext = self.extract_least_due_slice(h)
            assert status == "OK", status
            if ext["conflicted"]:
                self.reports.append({"t": ext["due"], "p": ext["profile"], "conflicted": ext["conflicted"]})
            if e > 0:
                self.eval_cohort("scheduled", ext["due"], ext["profile"], ext["executable"])
                admitted += 1
                r = 0 if exception_fired else r - e
        result = "COMPLETED"
        self.last_refusal = None
        if isinstance(req, Command):
            if not (1 <= r or admitted == 0):
                return self._pause(req, h, command_deferred=True)
            ordinal = self.timeline.frontier
            refusal = self._finalize(req)
            if refusal is not None:
                result, self.last_refusal = "COMPLETED_COMMAND_NOT_FINALIZED", refusal
            else:
                self.eval_cohort("command", req.e, req.profile, [(f"cmd#{ordinal}", req.payload, None)])
        self.F, self.active = h, None
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

    # --- snapshot / restore (stores + timeline + F + ActiveRequest)
    def snapshot(self):
        return copy.deepcopy({"sched": self.sched, "obl": self.obl, "cells": self.cells,
                              "timeline": self.timeline.full_state(), "grant": self.grant,
                              "F": self.F, "active": self.active, "sbd": self.stable_boundary_digest()})

    @classmethod
    def restore(cls, rules, budget, snap):
        e = cls(rules, budget)
        s = copy.deepcopy(snap)
        e.sched, e.obl, e.cells = s["sched"], s["obl"], s["cells"]
        e.timeline, e.grant = Timeline.from_state(s["timeline"]), s["grant"]
        e.F, e.active = s["F"], s["active"]
        if e.active is not None and e.active["h"] < e.F:
            return "RESTORE_REJECTED_ACTIVE_BEHIND_FRONTIER", None
        if e.timeline.slots:
            return "RESTORE_REJECTED_TIMELINE_STAGING_PRESENT", None
        if e.stable_boundary_digest() != s["sbd"]:
            return "RESTORE_REJECTED_DIGEST_MISMATCH", None
        return "OK", e


# ---------------------------------------------------------------- consumer (Revision 2)
def drive(engine, mailbox, log, max_calls=100, terminal=TERMINAL_FOR_PRESENTED):
    """Request-bound dequeue: present only the head; pop it only on a result terminal for it;
    halt fail-closed (no pop, no presentation) if the engine's ActiveRequest is not the head."""
    calls = 0
    while mailbox.head() is not None and calls < max_calls:
        r = mailbox.head()
        if terminal is TERMINAL_FOR_PRESENTED and engine.active is not None and engine.active != r.discriminator():
            log.append((r.label(), HALT))
            return calls
        res = engine.process(r)
        calls += 1
        log.append((r.label(), res))
        if res in terminal:
            mailbox.pop()
        elif res == "REFUSED_ACTIVE_REQUEST_MISMATCH":
            log.append((r.label(), HALT))
            return calls
    return calls


# ---------------------------------------------------------------- rules (waves)
def rules(kind, now, items, cells):
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
            elif payload == "T2":
                w0 += [{"op": "set", "cell": "w", "v": 1}]
                w1 += [{"op": "set", "cell": "w2", "v": now}]
            elif payload == "T2bad":
                w0 += [{"op": "set", "cell": "w", "v": 1}]
                w1 += [{"op": "reject", "why": "unequal same-family RESULT"}]
            elif payload == "W0bad":
                w0 += [{"op": "reject", "why": "unequal same-family RESULT"}]
            elif payload == "N":
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


out = {}


def check(name, cond, detail=""):
    out[name] = ("PASS" if cond else "FAIL") + (f" - {detail}" if detail else "")
    print(f"[{out[name].split(' ')[0]}] {name}{(' - ' + detail) if detail else ''}")


# ===================================================================== carried checks (34)
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
ref = Engine(rules, 3); ref.schedule(20, "P", "B", "B")
check("S3f cohort@10 pre-wave digest == reference engine holding exactly {B@20} (whole-prefix drain killed)",
      eL.trace[0]["prewave"] == ref.engine_state_digest() and eL.trace[0]["prewave"] != Engine(rules, 3).engine_state_digest())

e4 = fresh(2); mb4 = Mailbox(4); log4 = []
mb4.offer(Advance(20)); mb4.offer(Command(15, "late")); mb4.offer(Command(20, "ok_equal")); mb4.offer(Command(25, "ok"))
drive(e4, mb4, log4)
res4 = {r: s for r, s in log4}
fin4 = [(f[0], f[1]["e"], f[1]["command_id"]) for f in e4.timeline.finalized]
check("S4 past-dated command rejected non-canonically; equal-time and later accepted",
      res4["Command@15(late)"] == "REFUSED_HORIZON_BEHIND_FRONTIER" and res4["Command@20(ok_equal)"] == "COMPLETED"
      and res4["Command@25(ok)"] == "COMPLETED" and all("late" not in f[2] for f in fin4), f"timeline={fin4}")
eA = Engine(rules, 2); eA.schedule(10, "P", "N", "N"); mbA = Mailbox(2); mbA.offer(Advance(100)); drive(eA, mbA, [])
eB = Engine(rules, 2); eB.schedule(20, "P", "N", "N"); mbB = Mailbox(2); mbB.offer(Advance(100)); drive(eB, mbB, [])
same_state = eA.stable_boundary_digest() == eB.stable_boundary_digest() and eA.engine_state_digest() == eB.engine_state_digest()
dA, dB = eA.process(Command(15, "x")), eB.process(Command(15, "x"))
check("S4b equal digests -> identical decision on Command@15", same_state and dA == dB == "REFUSED_HORIZON_BEHIND_FRONTIER")

e5 = fresh(1); mb5 = Mailbox(4); log5 = []
mb5.offer(Advance(20)); mb5.offer(Command(21, "c1"))
log5.append((mb5.head().label(), e5.process(mb5.head())))
snap = e5.snapshot(); trace_before = copy.deepcopy(e5.trace); blog_before = copy.deepcopy(e5.boundary_log)
st, e5r = Engine.restore(rules, 1, snap); e5r.trace, e5r.boundary_log, e5r.reports = trace_before, blog_before, copy.deepcopy(e5.reports)
drive(e5r, mb5, log5)
check("S5 restart from (stores, F, ActiveRequest) with the exact head re-presented == uninterrupted run",
      st == "OK" and canonical(e5r) == canonical(eb1) and snap["active"] == Advance(20).discriminator())
check("S6 no backwards decay evaluation in any scenario", True, "assert elapsed >= 0 held in every cohort")

e7 = Engine(rules, 3)
for t, p in ((10, "A"), (15, "N"), (20, "B")):
    e7.schedule(t, "P", p, p)
r7 = e7.process(Advance(20))
before7 = (e7.stable_boundary_digest(), e7.engine_state_digest(), e7.timeline.full_state(), e7.F, e7.active)
sub = e7.process(Command(12, "sub"))
after7 = (e7.stable_boundary_digest(), e7.engine_state_digest(), e7.timeline.full_state(), e7.F, e7.active)
check("S7 SB-01: paused-request substitution Command@12 refused, byte-identical state",
      r7 == "PAUSED" and max(c["updated_at"] for c in e7.cells.values()) == 15
      and sub == "REFUSED_ACTIVE_REQUEST_MISMATCH" and before7 == after7 and e7.F == 0,
      "cells updated_at max=15 while F=0; refusal mutated nothing")
sub2 = e7.process(Advance(25))
check("S7b higher-horizon Advance(25) also refused while Advance(20) is active",
      sub2 == "REFUSED_ACTIVE_REQUEST_MISMATCH" and e7.stable_boundary_digest() == before7[0])
res7 = e7.process(Advance(20))
check("S7c exact head resumes and completes; ActiveRequest cleared, F=20",
      res7 == "COMPLETED" and e7.active is None and e7.F == 20)


def e8_base():
    e = Engine(rules, 3)
    for t, p in ((10, "A"), (15, "N"), (20, "B")):
        e.schedule(t, "P", p, p)
    return e


e8 = e8_base(); e8.process(Advance(20)); snap8 = e8.snapshot()
st_a, e8a = Engine.restore(rules, 3, snap8); res8a = e8a.process(Advance(20))
e8u = e8_base(); e8u.process(Advance(20)); e8u.process(Advance(20))
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

g1 = fresh(2); g2 = fresh(2); g2.active = Advance(20).discriminator()
g3 = fresh(2); g3.active = Command(20, "c").discriminator()
g4 = fresh(2); g4.active = Advance(21).discriminator()
g5 = fresh(2); g5.active = Command(20, "c2").discriminator()
G5 = (g1, g2, g3, g4, g5)
esd_equal = len({g.engine_state_digest() for g in G5}) == 1
sbd_distinct = len({g.stable_boundary_digest() for g in G5}) == 5
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

e10 = fresh(4); e10.process(Advance(20))
c1 = e10.active is None and e10.F == 20
sbd10 = e10.stable_boundary_digest()
r10 = e10.process(Advance(5))
check("S10 ActiveRequest absent after COMPLETED and after HORIZON_BEHIND_FRONTIER refusal; refusal mutates neither F nor digest",
      c1 and r10 == "REFUSED_HORIZON_BEHIND_FRONTIER" and e10.active is None and e10.F == 20 and e10.stable_boundary_digest() == sbd10)
r10b = e10.process(Advance(20))
check("S10b equal-horizon Advance after completion is accepted and idempotent",
      r10b == "COMPLETED" and e10.stable_boundary_digest() == sbd10 and e10.active is None)

same_trace = eb1.trace == eb2.trace == eb3.trace and eb1.reports == eb2.reports == eb3.reports


def progress_points(e):
    return {(b["cohorts"], b["active"] if isinstance(b["active"], str) else tuple(b["active"])): b["sbd"] for b in e.boundary_log}


pp = [progress_points(e) for e in (eb1, eb2, eb3)]
consistent = all(pp[i][k] == pp[j][k] for i in range(3) for j in range(3) for k in pp[i] if k in pp[j])
check("S12 budgets 1/2/3: per-cohort values equal by position; stable-boundary digests equal at equal progress points",
      same_trace and consistent and eb1.boundary_log[-1]["sbd"] == eb2.boundary_log[-1]["sbd"] == eb3.boundary_log[-1]["sbd"],
      f"calls={len(lb1)},{len(lb2)},{len(lb3)}")

e13 = fresh(2); e13.process(Advance(0))
del e13.obl["A"]
d_s, d_e = e13.scheduler_digest(), e13.engine_state_digest()
st13, key13 = e13.extract_least_due_slice(20)
ok13 = st13 == "ObligationRecordMissing" and key13 == "A" and e13.scheduler_digest() == d_s and e13.engine_state_digest() == d_e
e13.obl["A"] = ("S", "A-wrong")
d_s, d_e = e13.scheduler_digest(), e13.engine_state_digest()
st13b, _ = e13.extract_least_due_slice(20)
ok13b = st13b == "ObligationRecordMismatch" and e13.scheduler_digest() == d_s and e13.engine_state_digest() == d_e
e13.obl["A"] = ("S", "A")
st13c, ext13 = e13.extract_least_due_slice(20)
ok13c = st13c == "OK" and ext13["executable"][0][2] == "A" and "A" not in e13.obl and (10, "P") not in e13.sched
check("S13 V2-05: missing/mismatched obligation record refuses before either store mutates; repaired store extracts with the record",
      ok13 and ok13b and ok13c)

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


def conflict_pair(budget):
    a = Engine(rules, budget); a.schedule(100, "P", "S", "B"); a.schedule(101, "P", "T", "B")
    b = Engine(rules, budget); b.schedule(100, "P", "S", "B"); b.schedule(100, "P", "X", "p1"); b.schedule(100, "P", "X", "p2")
    b.schedule(101, "P", "T", "B")
    return a, b


ea, eb = conflict_pair(2)
ra, rb = ea.process(Advance(101)), eb.process(Advance(101))
check("S15 V2-04: {S} vs {S, X_conflicted}: equal cohort identity, pre-wave digest, batches, final state; only the conflict report differs",
      ra == rb == "COMPLETED" and ea.trace == eb.trace and ea.engine_state_digest() == eb.engine_state_digest()
      and ea.reports == [] and eb.reports and eb.reports[0]["conflicted"][0][0] == "X")
_, ec = conflict_pair(2); rc = ec.process(Advance(101))
_, ed = conflict_pair(1); rd = ed.process(Advance(101))
check("S15b V2-03: budget 2 admits S then T (executable-only count); budget 1 admits S and defers T with count/diagnostics",
      rc == "COMPLETED" and len(ec.trace) == 2 and rd == "PAUSED" and len(ed.trace) == 1
      and ed.diag[-1]["deferred_cohort_count"] == 1 and ed.diag[-1]["earliest_deferred_due_time"] == 101)

e16 = Engine(rules, 2)
for k in ("a", "b", "c"):
    e16.schedule(100, "P", k, "B")
e16.schedule(101, "P", "x", "p1"); e16.schedule(101, "P", "x", "p2")
e16.schedule(102, "P", "y", "p1"); e16.schedule(102, "P", "y", "p2")
e16.schedule(103, "P", "T", "B")
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

e17 = fresh(4); mb17 = Mailbox(4); log17 = []
mb17.offer(Command(21, "c1", cmd_id="k1")); mb17.offer(Command(25, "dup", cmd_id="k1")); drive(e17, mb17, log17)
check("S17 duplicate command identity at finalization: COMPLETED_COMMAND_NOT_FINALIZED, F=25, ActiveRequest cleared, timeline unchanged",
      log17[-1][1] == "COMPLETED_COMMAND_NOT_FINALIZED" and e17.last_refusal == "CommandIdentityConflict"
      and e17.F == 25 and e17.active is None and len(e17.timeline.finalized) == 1 and e17.timeline.frontier == 1)
r17 = e17.process(Command(24, "late", cmd_id="k9"))
check("S17b after the refused finalization, a request behind F=25 is still refused at start", r17 == "REFUSED_HORIZON_BEHIND_FRONTIER")

e18 = fresh(2); d18 = e18.engine_state_digest()
fp_nonleast = e18.fingerprint((20, "P"))
st18, obs = e18.take_least_due_slice(20, fp_nonleast)
fp_least = e18.fingerprint((10, "P")); e18.schedule(10, "P", "A", "A-other")
st18b, _ = e18.take_least_due_slice(20, fp_least)
check("S18 non-least fingerprint => SliceChanged with the live least fingerprint; stale fingerprint after Scheduled->Conflicted => no-op",
      st18 == "SliceChanged" and obs == fp_least and st18b == "SliceChanged" and e18.engine_state_digest() != d18
      and (10, "P") in e18.sched and e18.sched[(10, "P")]["A"][0] == "C")

e19a = fresh(1); mb19a = Mailbox(2); l19a = []; mb19a.offer(Command(21, "c1")); drive(e19a, mb19a, l19a)
e19b = fresh(9); mb19b = Mailbox(2); l19b = []; mb19b.offer(Command(21, "c1")); drive(e19b, mb19b, l19b)
check("S19 command deferred by budget executes on a later call; canonical result identical to the unbudgeted run",
      canonical(e19a) == canonical(e19b) and len(l19a) > len(l19b) and any(d.get("command_deferred") for d in e19a.diag))
CARRIED = len(out)

# ===================================================================== FINAL-01: atomic finalization
# F1: the review's executable regression at engine level: finalize S:10, empty frontier, distinct S:9.
def s10_engine(mode):
    e = Engine(rules, 4)
    e.finalize_mode = mode
    assert e.process(Command(20, "first", cmd_id="first", seq=10)) == "COMPLETED"
    return e


e_old = s10_engine("old")
tl_before_old = e_old.timeline.full_state(); esd_before_old = e_old.engine_state_digest()
r_old = e_old.process(Command(20, "distinct", cmd_id="distinct", seq=9))
old_staged = e_old.timeline.slots.get(1, [None])[0] == "staged"
check("F1 negative control: OLD stage-then-fence leaves a positively staged ordinal after SourceSequenceNotIncreasing",
      r_old == "COMPLETED_COMMAND_NOT_FINALIZED" and e_old.last_refusal == "fence:SourceSequenceNotIncreasing"
      and old_staged and e_old.timeline.full_state() != tl_before_old and e_old.engine_state_digest() != esd_before_old
      and e_old.timeline.frontier == 1 and len(e_old.timeline.finalized) == 1,
      "frontier and finalized history unchanged, but the staged slot and staged identity indexes survive (review FINAL-01)")

e_new = s10_engine("rev2")
tl_before = e_new.timeline.full_state(); esd_before = e_new.engine_state_digest()
r_new = e_new.process(Command(20, "distinct", cmd_id="distinct", seq=9))
check("F1b REV2: the same request is refused with the complete timeline and engine state byte-identical; F and ActiveRequest complete",
      r_new == "COMPLETED_COMMAND_NOT_FINALIZED" and e_new.last_refusal == "SourceSequenceNotIncreasing"
      and e_new.timeline.full_state() == tl_before and e_new.engine_state_digest() == esd_before
      and e_new.F == 20 and e_new.active is None)

# F2: every refusal route, complete timeline state compared for REV2 and OLD, REV2 verdict vs REF.
def tl_s10():
    t = Timeline(profile="P", sequencer="seq", width=2)
    assert finalize_rev2(t, "seq", Command(20, "first", cmd_id="first", seq=10)) is None
    return t


def tl_raw(stagings, width=2):
    t = Timeline(profile="P", sequencer="seq", width=width)
    for cmd, o in stagings:
        w = t.current_window()
        t.stage("seq", cmd.envelope(o), w["ticket"])
    return t


X = Command(20, "x", cmd_id="x", seq=3)
ROUTES = [
    # name, builder, request, grant, expected REV2 refusal, OLD mutates?, REF succeeds?
    ("source-sequence regression", tl_s10, Command(20, "d", cmd_id="d", seq=9), "seq", "SourceSequenceNotIncreasing", True, False),
    ("command-id reuse", tl_s10, Command(20, "d", cmd_id="first", seq=11), "seq", "CommandIdentityConflict", False, False),
    ("source-sequence pair reuse", tl_s10, Command(20, "d", cmd_id="d", seq=10), "seq", "SourceSequenceConflict", False, False),
    ("wrong timeline epoch", tl_s10, Command(20, "d", cmd_id="d", seq=11, epoch=1), "seq", "WrongTimelineEpoch", False, False),
    ("wrong profile", tl_s10, Command(20, "d", cmd_id="d", seq=11, profile="Q"), "seq", "WrongProfile", False, False),
    ("sequencer grant not active", tl_s10, Command(20, "d", cmd_id="d", seq=11), "other", "NotActiveSequencer", False, False),
    ("window arithmetic exhausted", lambda: Timeline(width=2, frontier=U64_MAX), X, "seq", "OrdinalSpaceExhausted(window)", False, False),
    ("frontier advance exhausted", lambda: Timeline(width=1, frontier=U64_MAX), X, "seq", "OrdinalSpaceExhausted(frontier)", True, False),
    ("poisoned frontier slot", lambda: tl_raw([(Command(20, "p1", cmd_id="p1", seq=1), 0), (Command(20, "p2", cmd_id="p2", seq=2), 0)]),
     X, "seq", "UnexpectedStagingState", True, False),
    ("different envelope staged at frontier", lambda: tl_raw([(Command(20, "p1", cmd_id="p1", seq=1), 0)]), X, "seq",
     "UnexpectedStagingState", True, False),
    ("identical envelope staged at frontier", lambda: tl_raw([(X, 0)]), X, "seq", "UnexpectedStagingState", True, True),
    ("staged tail above empty frontier", lambda: tl_raw([(Command(20, "t", cmd_id="t", seq=1), 1)]), X, "seq",
     "UnexpectedStagingState", True, True),
]
route_rows, all_rev2_identical, all_old_as_expected, differential_ok = [], True, True, True
for name, build, rq, grant, expect, old_mutates, ref_ok in ROUTES:
    t = build(); before = t.full_state(); digests = (t.state_digest(), t.history_digest())
    got = finalize_rev2(t, grant, rq)
    rev2_identical = got == expect and t.full_state() == before and (t.state_digest(), t.history_digest()) == digests
    live = build(); live_before = live.full_state()
    ref_verdict, _ = finalize_ref(live, grant, rq)
    ref_success = ref_verdict is None
    differential = live.full_state() == live_before and ref_success == ref_ok and (
        name in ("identical envelope staged at frontier", "staged tail above empty frontier") or not ref_success)
    o = build(); o_before = o.full_state()
    o_verdict = finalize_old(o, grant, rq)
    o_mutated = o.full_state() != o_before
    all_rev2_identical &= rev2_identical
    all_old_as_expected &= (o_mutated == old_mutates)
    differential_ok &= differential
    route_rows.append({"route": name, "rev2": got, "rev2_byte_identical": rev2_identical, "ref": ref_verdict,
                       "old": o_verdict, "old_mutated": o_mutated})
check("F2 every finalization refusal route: REV2 typed refusal with the COMPLETE timeline state byte-identical",
      all_rev2_identical, f"{len(ROUTES)} routes")
check("F2b negative control: OLD composition mutates the live timeline on exactly the expected routes",
      all_old_as_expected, f"mutated on {sum(r['old_mutated'] for r in route_rows)} of {len(ROUTES)}: "
      + ", ".join(r['route'] for r in route_rows if r['old_mutated']))
check("F2c differential: on clean-staging states REV2 refuses exactly where the isolated working-copy reference refuses; REF never touches live",
      differential_ok, "unclean states are refused by the fail-closed staging precondition even where Phase-1 would accept")

# F3: success path equals the working-copy reference byte-for-byte; ack precedes fence.
ta = tl_s10(); tb = tl_s10(); oplog = []
ok_a = finalize_rev2(ta, "seq", Command(25, "second", cmd_id="second", seq=11), oplog)
ref_v, ref_copy = finalize_ref(tb, "seq", Command(25, "second", cmd_id="second", seq=11))
check("F3 success: REV2 result state == working-copy reference; positive NewlyStaged ack precedes the one-ordinal fence",
      ok_a is None and ref_v is None and ta.full_state() == ref_copy.full_state() and ta.frontier == 2 and not ta.slots
      and [op[0] for op in oplog] == ["positive_staged_ack", "one_ordinal_fence"])

# F4: follow-on. OLD residue lets the next distinct command poison ordinal 1; REV2 finalizes it.
e_old.process(Command(21, "next", cmd_id="next", seq=11))
e_new.process(Command(21, "next", cmd_id="next", seq=11))
check("F4 follow-on: after the regression refusal REV2 finalizes the next distinct command at ordinal 1; OLD poisons ordinal 1",
      e_new.timeline.frontier == 2 and e_new.timeline.finalized[-1][1]["command_id"] == "next"
      and e_old.timeline.slots.get(1, [None])[0] == "poisoned" and e_old.timeline.frontier == 1,
      f"OLD second refusal={e_old.last_refusal}")

# F5: clean staging holds at every stable boundary of every REV2 scenario above; restore enforces it.
boundary_engines = (e1, eb1, eb2, eb3, eL, eS, e4, e5r, e7, e8a, e17, e19a, e19b, e_new)
snap_bad = e_new.snapshot(); snap_bad["timeline"]["slots"] = {2: ["staged", Command(30, "z").envelope(2), "h"]}
st_bad, _ = Engine.restore(rules, 4, snap_bad)
check("F5 clean-staging invariant: no timeline slot at any REV2 stable boundary; restore rejects a snapshot carrying staging",
      all(not e.timeline.slots for e in boundary_engines) and st_bad == "RESTORE_REJECTED_TIMELINE_STAGING_PRESENT")

# F6: D-2 preserved: refusal completes the horizon; earlier committed scheduled work is not rolled back.
e6 = fresh(4); assert e6.process(Command(20, "c", cmd_id="c", seq=10)) == "COMPLETED"
e6.schedule(22, "P", "B2", "B"); cells_pre = copy.deepcopy(e6.cells)
r6 = e6.process(Command(25, "c-again", cmd_id="c", seq=12))
check("F6 D-2: finalization refusal after scheduled work keeps that work committed, advances F, clears ActiveRequest",
      r6 == "COMPLETED_COMMAND_NOT_FINALIZED" and e6.trace[-1]["t"] == 22 and e6.trace[-1]["kind"] == "scheduled"
      and e6.cells != cells_pre and e6.F == 25 and e6.active is None and len(e6.timeline.finalized) == 1)

# ===================================================================== FINAL-02: request-bound FIFO
def fifo_setup():
    e = fresh(1); mb = Mailbox(4)
    mb.offer(Advance(20)); mb.offer(Command(21, "c1"))
    assert e.process(mb.head()) == "PAUSED"
    return e, mb


eq, mbq = fifo_setup()
q_before = (eq.snapshot(), mbq.labels(), mbq.head())
faulty = eq.process(Command(12, "substitute"))           # faulty presentation of a non-head request
dequeue_eligible = faulty in TERMINAL_FOR_PRESENTED
q_after = (eq.snapshot(), mbq.labels(), mbq.head())
logq = []
drive(eq, mbq, logq)
check("Q1 faulty presentation while the genuine active head is queued: mismatch is not dequeue-eligible; queue, order and engine unchanged",
      faulty == "REFUSED_ACTIVE_REQUEST_MISMATCH" and not dequeue_eligible and q_before == q_after)
check("Q1b the retained exact head then continues and the FIFO drains; canonical result equals the undisturbed run",
      [x[1] for x in logq] == ["PAUSED", "COMPLETED", "COMPLETED"] and canonical(eq) == canonical(eb1) and mbq.head() is None)
eq2, mbq2 = fifo_setup()
faulty2 = eq2.process(Command(12, "substitute"))
if faulty2 in TERMINAL_V1:
    mbq2.pop()
check("Q1c negative control: the reviewed candidate's terminal-pop rule loses the actual active head",
      mbq2.head() is not None and mbq2.head().label() == "Command@21(c1)" and eq2.active == Advance(20).discriminator())

eq3, mbq3 = fifo_setup()
snapq = eq3.snapshot()
st_q, eq3r = Engine.restore(rules, 1, snapq)
mb_wrong = Mailbox(4); mb_wrong.offer(Command(21, "c1"))     # durable head does not match ActiveRequest
pre = (eq3r.snapshot(), mb_wrong.labels()); logh = []
drive(eq3r, mb_wrong, logh)
check("Q2 restore/mailbox mismatch fails closed: consumer halts, presents nothing, pops nothing; engine unchanged",
      st_q == "OK" and logh == [("Command@21(c1)", HALT)] and (eq3r.snapshot(), mb_wrong.labels()) == pre)

# Q3: durability ordering. Completion persisted before the pop: redelivery is safe and byte-identical.
ea3 = fresh(4); ea3.process(Advance(20)); sa = ea3.stable_boundary_digest()
ra3 = ea3.process(Advance(20))
ec3 = fresh(4); ec3.process(Command(21, "c1")); sc = ec3.stable_boundary_digest(); tlc = ec3.timeline.full_state()
rc3 = ec3.process(Command(21, "c1"))
check("Q3 completion persisted before pop: a redelivered completed request is harmless and the stable boundary is byte-identical",
      ra3 == "COMPLETED" and ea3.stable_boundary_digest() == sa and rc3 == "COMPLETED_COMMAND_NOT_FINALIZED"
      and ec3.last_refusal == "CommandIdentityConflict" and ec3.timeline.full_state() == tlc and ec3.stable_boundary_digest() == sc)
ep3 = fresh(1); snap_paused = None
mbp = Mailbox(4); mbp.offer(Advance(20)); mbp.offer(Command(21, "c1"))
ep3.process(mbp.head()); snap_paused = ep3.snapshot()
mbp.pop()                                                    # pop persisted before the completion snapshot
st_p, ep3r = Engine.restore(rules, 1, snap_paused); logp = []
drive(ep3r, mbp, logp)
check("Q3b pop persisted before completion: restore detects the missing active request and halts fail-closed (not safe recovery)",
      st_p == "OK" and logp == [("Command@21(c1)", HALT)] and ep3r.active == Advance(20).discriminator())

# ===================================================================== FINAL-03: replay scope
pz = fresh(1); assert pz.process(Advance(20)) == "PAUSED"
rz = fresh(1); assert rz.process(Advance(pz.F)) == "COMPLETED"
check("R1 two reachable boundaries share finalized history and F but differ in paused progress",
      rz.timeline.full_state() == pz.timeline.full_state() and rz.F == pz.F == 0
      and rz.engine_state_digest() != pz.engine_state_digest() and rz.stable_boundary_digest() != pz.stable_boundary_digest(),
      "history + F cannot reconstruct the pause")
st_r, rr = Engine.restore(rules, 1, pz.snapshot())
sub_r = rr.process(Command(20, "different"))
while pz.process(Advance(20)) == "PAUSED":
    pass
while rr.process(Advance(20)) == "PAUSED":
    pass
check("R2 committed snapshot + exact request continues equivalently; a substitute is refused after restore",
      st_r == "OK" and sub_r == "REFUSED_ACTIVE_REQUEST_MISMATCH" and rr.snapshot() == pz.snapshot())


def history_run():
    e = fresh(1); mb = Mailbox(16)
    for r in (Advance(12), Command(20, "c1", cmd_id="c1", seq=5), Advance(21),
              Command(22, "dup", cmd_id="c1", seq=6), Command(24, "c2", cmd_id="c2", seq=7)):
        mb.offer(r)
    drive(e, mb, [])
    assert e.timeline.reset_epoch("seq", 1, "seq2")[0] == "ok"
    e.grant = "seq2"
    mb.offer(Command(30, "c3", cmd_id="c3", seq=8, epoch=1)); mb.offer(Advance(33))
    drive(e, mb, [])
    return e


orig = history_run()


def replay(history, width=2, apply_resets=True, budget=9):
    """History-only reconstruction at a COMPLETED boundary: genesis stores, timeline metadata
    (profile, genesis epoch/sequencer/frontier, width, reset records), each finalized command's
    full request INCLUDING its payload (a payload hash does not reconstruct a payload), and F."""
    e = fresh(budget)
    e.timeline = Timeline(profile="P", sequencer="seq", width=width)
    resets = list(history["resets"]) if apply_resets else []
    for ordinal, rq in history["commands"]:
        while resets and resets[0][4] == ordinal:
            rec = resets.pop(0)
            assert e.timeline.reset_epoch(e.grant, rec[2], rec[3])[0] == "ok"
            e.grant = rec[3]
        for r in (Advance(rq.e), rq):
            while (res := e.process(r)) == "PAUSED":
                pass
            assert res == "COMPLETED", res
    while resets:
        rec = resets.pop(0)
        e.timeline.reset_epoch(e.grant, rec[2], rec[3]); e.grant = rec[3]
    while e.process(Advance(history["F"])) == "PAUSED":
        pass
    return e


payloads = {"c1": Command(20, "c1", cmd_id="c1", seq=5), "c2": Command(24, "c2", cmd_id="c2", seq=7),
            "c3": Command(30, "c3", cmd_id="c3", seq=8, epoch=1)}
hist = {"commands": [(f[0], payloads[f[1]["command_id"]]) for f in orig.timeline.finalized],
        "resets": orig.timeline.resets, "F": orig.F}
rep = replay(hist)
check("R3 completed-boundary history replay (stores, timeline metadata, payloads, F) reproduces the stable boundary exactly",
      orig.active is None and rep.stable_boundary_digest() == orig.stable_boundary_digest()
      and rep.timeline.full_state() == orig.timeline.full_state(),
      "original ran budget 1 with pauses, a refused finalization and an epoch reset; replay ran budget 9")
rep_w = replay(hist, width=3)
rep_nr_state = None
try:
    replay(hist, apply_resets=False)
except AssertionError:
    rep_nr_state = "refused"     # the epoch-1 command cannot finalize without the reset record
check("R3b metadata is a precondition: a different window width gives equal finalized history but a different stable boundary; omitting the reset record cannot replay",
      rep_w.timeline.history_digest() == orig.timeline.history_digest()
      and rep_w.stable_boundary_digest() != orig.stable_boundary_digest() and rep_nr_state == "refused")

# ===================================================================== oracle precision
# P-a: same-kind same-horizon command substitution while a COMMAND is paused.
class KindHorizonMutant(Engine):
    def _matches(self, active, d):
        return (active["kind"], active["h"]) == (d["kind"], d["h"])


def paused_command(cls):
    e = cls(rules, 1); e.schedule(10, "P", "A", "A"); e.schedule(20, "P", "B", "B")
    assert e.process(Command(20, "p1", cmd_id="p1", seq=1)) == "PAUSED"
    return e


pc = paused_command(Engine); pc_before = pc.snapshot()
pa_res = pc.process(Command(20, "p2", cmd_id="p2", seq=2))
mut = paused_command(KindHorizonMutant)
mut_res = mut.process(Command(20, "p2", cmd_id="p2", seq=2))
km =KindHorizonMutant(rules, 1); km.schedule(10, "P", "A", "A"); km.schedule(20, "P", "B", "B"); km.process(Advance(20))
km_old = km.process(Command(20, "c"))
check("P-a paused Command, different same-kind same-horizon Command refused byte-identically; a kind+horizon mutant accepts it",
      pa_res == "REFUSED_ACTIVE_REQUEST_MISMATCH" and pc.snapshot() == pc_before and mut_res != "REFUSED_ACTIVE_REQUEST_MISMATCH"
      and km_old == "REFUSED_ACTIVE_REQUEST_MISMATCH",
      f"mutant result={mut_res}; the FINAL oracle's Command-while-Advance fixture does NOT kill the mutant (it refuses on kind)")


# P-b: redundant-field discrimination is honest; encoding pins are what kill kind/horizon omission.
def canon_no(field):
    idx = {"kind": 1, "h": 2, "id": 3}[field]
    return lambda a: canon_active(a) if a is None else [x for i, x in enumerate(canon_active(a)) if i != idx]


distinct = {f: len({g.stable_boundary_digest(canon_no(f)) for g in G5}) for f in ("kind", "h", "id")}
PIN = {"none": "active_request.none",
       "advance20": ["active_request.some", "advance", 20, H(["request_advance_v1", 20])]}
pin_ok = canon_active(None) == PIN["none"] and canon_active(Advance(20).discriminator()) == PIN["advance20"]
pin_kills = all(canon_no(f)(Advance(20).discriminator()) != PIN["advance20"] for f in ("kind", "h", "id"))
check("P-b five-engine discrimination does NOT kill omission of kind or horizon (identity binds them); it kills omission of identity; encoding pins kill all three",
      distinct["kind"] == 5 and distinct["h"] == 5 and distinct["id"] == 4 and pin_ok and pin_kills, f"distinct digests={distinct}")

# P-c: equal residual stores versus different removed sets.
pa_e, pb_e = conflict_pair(2)
pa_e.process(Advance(100)); pb_e.process(Advance(100))
check("P-c {S} vs {S, X}: resulting stores equal; removed sets differ by X's complete contested claim set",
      pa_e.obl == pb_e.obl and pa_e.engine_state_digest() == pb_e.engine_state_digest()
      and pa_e.extractions[0]["removed"] != pb_e.extractions[0]["removed"]
      and [r for r in pb_e.extractions[0]["removed"] if r[0] == "X"] == [["X", "C", ["p1", "p2"]]])

# P-d: outer-loop selection counting.
sp = Engine(rules, 9); sp.schedule(1, "P", "n1", "N"); sp.schedule(1_000_000, "P", "n2", "N")
assert sp.process(Advance(1_000_000)) == "COMPLETED"
ss = Engine(rules, 9); ss.schedule(1, "P", "n1", "N"); ss.schedule(1_000_000, "P", "n2", "N")
ss.process(Advance(1)); ss.process(Advance(1_000_000))
check("P-d AT-I39(C): outer-loop (A2) selections == slices consumed + 1; all least-slice locations counted together are not",
      len(sp.trace) == 2 and sp.sel_outer == 3 and sp.sel_total != 3 and sp.trace == ss.trace,
      f"outer={sp.sel_outer} total={sp.sel_total} (extraction preflight and X-3 also locate the least slice)")

# P-e: AT-I43(a) spelled out.
la = fresh(1); mba = Mailbox(4); mba.offer(Advance(20)); steps = []
res_a = la.process(mba.head()); steps.append((res_a, [t["t"] for t in la.trace], la.F, canon_active(la.active)))
mba.offer(Command(21, "c1"))
while mba.head() is not None:
    res_a = la.process(mba.head()); steps.append((res_a, [t["t"] for t in la.trace], la.F, canon_active(la.active)))
    if res_a in TERMINAL_FOR_PRESENTED:
        mba.pop()
adv = canon_active(Advance(20).discriminator())
check("P-e AT-I43(a): A@10's wave-0 rule creates D@15; successive calls return PAUSED, PAUSED, COMPLETED, then the Command COMPLETED",
      steps == [("PAUSED", [10], 0, adv), ("PAUSED", [10, 15], 0, adv), ("COMPLETED", [10, 15, 20], 20, "active_request.none"),
                ("COMPLETED", [10, 15, 20, 21], 21, "active_request.none")], f"steps={[(s[0], s[1]) for s in steps]}")

json.dump({"checks": out, "carried_checks": CARRIED, "finalization_routes": route_rows,
           "trace_budget1": eb1.trace, "diag_budget1": eb1.diag, "boundary_budget1": eb1.boundary_log,
           "S9_sbd": [g.stable_boundary_digest() for g in G5], "P_b_distinct": distinct,
           "selection_counts": {"outer": sp.sel_outer, "total": sp.sel_total}, "P_e_steps": steps},
          open(sys.argv[1] if len(sys.argv) > 1 else "results.json", "w"), indent=1, sort_keys=True, default=str)
print(f"{len(out)} checks ({CARRIED} carried, {len(out) - CARRIED} new)")
print("ALL PASS" if all(v.startswith("PASS") for v in out.values()) else "SOME FAIL")
