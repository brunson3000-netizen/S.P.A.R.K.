# S.P.A.R.K. Phase 2 — V3-F01 Serialized Request Boundary (Addendum superseding the command-time candidate)

**Date:** 2026-09-06
**Agent:** Claude Code (Fable 5.1), Gate C1 foundational architecture adjudicator
**Status:** **CANDIDATE ADDENDUM — AWAITING A NEW V3-F01 WRITER PASS AND INDEPENDENT
REVIEW.** Not accepted, not frozen, not canonical; authorizes no implementation.
V3-F01 remains **OPEN**.
**Supersedes (as a proposal):** the command-time candidate at commit
`53a8432b9f0f3bad99c273d4ff7b52f4c63b5340`, specifically its ALT-U unified
pending-command mechanism, execution cursor `X`, and amendments A-1…A-3. That
document is preserved unchanged as history.
**Basis:** the Operator's serialized-request direction (bounded mailbox, one consumer,
one active request until completion, pause on budget exhaustion, in-order waiting,
backpressure, completion = the request's defined horizon, embedded ≡ standalone).
**Evidence:** `engineering/phase2/serialized_request_model_2026-09-06/` — a disposable
Python model (`model.py`, `results.txt`, `results.json`), architecture evidence only.

**Retained verbatim:** horizon expansion; scheduled evaluation at `now = due_time`;
live least-due recomputation; strictly later checked delayed work; scheduler
compare-and-take with full slot fingerprints; executable-only identity and pacing;
`test-support` digest observation; and the V2 review's bounded API/oracle corrections
V2-03…V2-10 as listed in the command-time candidate §10 (carried forward, not
redesigned).

---

## 1. Request lifecycle

A **request** is one external input to S.P.A.R.K.: `Advance{T}` or `Command{envelope}`.
Its **defined logical horizon** `h` is `T` or the envelope's `effective_time`.
Completion means every resident scheduled slice with `due_time ≤ h` has been consumed
and, for a command, the command has been finalized and executed. Timers with
`due_time > h` remain resident; completion never means exhausting them.

| Phase | Where | Canonical? | Definition |
|---|---|---|---|
| **Accept** | mailbox (host adapter, embedded or standalone) | no | The producer offers the request. If the bounded mailbox is full the producer receives backpressure (`MAILBOX_FULL`, or blocks, per transport). Nothing reaches the engine. Acceptance carries no ordering or admission authority over canonical state. |
| **Start** | consumer → engine `process(R)` | no | The consumer presents the mailbox **head**. The engine checks `h ≥ F` (§3). Failure returns `HORIZON_BEHIND_FRONTIER`: nothing mutated, nothing staged, not history; the consumer pops the request and reports it. Success makes `R` the active request for this call. |
| **Process** | engine | yes, per cohort | The retained loop consumes scheduled slices with `due_time ≤ h` in ascending `(due_time, profile_id)` order under the epoch-bound budget `B`, each at `now = due_time`. |
| **Pause** | engine → consumer | stable boundary | Budget exhausted with work `≤ h` remaining, or the command not yet executed: the call returns `PAUSED` with `PacingDiagnostics`. `F` is unchanged. The request stays at the head. **No later request may execute.** |
| **Resume** | consumer → engine | no | The consumer presents the **same** head again. The engine re-checks `h ≥ F` (passes, `F` unchanged) and continues from live canonical state; nothing carried over from the paused call is needed. |
| **Complete** | engine | yes | No resident slice has `due_time ≤ h`. For a command: the envelope is staged, acknowledged, fenced as a one-command range, and executed as its own cohort at `now = effective_time`, atomically within this call (ADR-0003 §13 convenience shape plus §14 barrier). The command cohort weighs one budget unit; if none remains it executes on the next call. Then `F := h` and the call returns `COMPLETED`. |
| **Next** | consumer | — | Only after `COMPLETED` or `HORIZON_BEHIND_FRONTIER` does the consumer pop the head and present the next request. |

Model evidence: S1 (second request accepted while the first is paused, waits, executes
after), S2 (budgets 1 and 2: identical canonical trace and request order; four calls
versus three), S4 (past-dated rejection, equal-time acceptance).

---

## 2. Minimum active-request state and ownership

**Inside the engine at any stable boundary: none.** The active request exists only as
the argument of the current `process` call. The engine's canonical state between calls
is the existing stores plus one scalar, `F` (§4).

**Ownership.** The consumer owns the active request by holding it at the mailbox head
until the engine returns `COMPLETED` or a rejection. Serialization — one request at a
time, later requests waiting in order — is the consumer's contract, exactly as ADR-0003
§7 makes STAGED-before-fence a sequencer protocol precondition rather than an engine
check. The engine's behavior is nevertheless total and deterministic for **any** call
sequence: every call is a horizon-driven step from live canonical state, so a
non-serializing consumer produces a different but fully replayable canonical history,
never a corrupted engine.

**Why nothing more is needed.** Partial progress of a paused `Advance` is time-ordered
work that any later request with an equal-or-greater horizon would consume identically;
it is canonically indistinguishable from "not yet requested". A paused `Command` has not
been staged, so it is not in history at all; losing it loses a host input at the host,
not engine consistency. This is what makes finalize-at-completion the right placement:
no finalized-but-unexecuted command ever exists at a stable boundary, so the V2 review's
V2-02 state question has an empty answer for commands.

**Replay.** Canonical history is the finalized command stream (timeline, already
digest-committed) plus `F`. Mechanism replay is: for each finalized command in ordinal
order, `Advance(effective_time)` to completion then finalize+execute; finally
`Advance(F)`. Intermediate advance horizons are not needed because partition
equivalence (§5) makes them irrelevant.

**Restart.** A snapshot at a stable boundary carries stores and `F`. If a request was
paused, the durable mailbox still holds it at the head; the consumer re-presents it and
the engine resumes from live state. Model evidence: S5 (snapshot after a pause, rebuild
the engine from stores and `F`, re-present the head: byte-identical to the uninterrupted
run). No message log, acknowledgement protocol, or delivery guarantee beyond
"head stays until completed" is introduced.

---

## 3. Command effective times between completed requests

Serialization prevents a command from being interleaved into another request's
catch-up. It does not prevent a producer from offering `Command@15` after `Advance(20)`
completed. That request would evaluate at `now = 15` against cells whose `updated_at`
may be 20, extrapolating closed-form decay backwards. The single rule that closes this:

> **A request may start only if `h ≥ F`.** Equal is permitted (a command at the current
> time). Otherwise `HORIZON_BEHIND_FRONTIER`, non-canonical, nothing staged.

Because `F` is the horizon of the last **completed** request and within a request the
loop is time-ordered, global canonical time is non-decreasing across the whole history
and `now ≥ updated_at` holds at every evaluation by construction. Model evidence: S4
(`Command@15` rejected after `Advance(20)`; `Command@20` and `Command@25` accepted; the
timeline never contains the past-dated command) and S6 (the model asserts non-negative
elapsed time in every cohort and never trips). Duplicate or replayed requests are already
handled by Phase-1 staging identity (`AlreadyStagedIdempotent`, `NOT_IN_ADMISSION_WINDOW`)
at completion; no new deduplication is added.

---

## 4. `F` and `X`, reassessed

| Item | Verdict | Reason |
|---|---|---|
| `X` execution cursor | **REMOVED** | Finalization is atomic with execution at request completion, so no finalized-pending command exists at any stable boundary. Nothing to point at. |
| `F` horizon frontier | **KEPT** — the only added state | Demonstrably required: review §5.3's engines have identical stores and must decide `Command@15` identically; the deciding value cannot be derived from stores (a no-effect cohort leaves no trace) and cannot be progress (pacing-causal). It is a host input: the last completed horizon. Model S4b. |
| `C` finalized time ceiling | **DROPPED as a separate concept** | Under serialization every finalized command's `effective_time` is `≤ F` at its own completion, so `C ≤ F` always and `max(F, C) = F`. |

`F` — owner: engine (the Phase-1 `LogicalClock`, unchanged); type: `LogicalTime`;
initialization: genesis time (`ZERO` by default); **updated only at request completion**
to `h`; unchanged by pause, rejection, restart, any cohort outcome, or any conflict
disposition; snapshot-carried; committed as in §5.

---

## 5. The digest contradiction, resolved exactly

The command-time candidate put `F` into `engine_state_digest` while claiming per-cohort
pre-wave and batch digests identical across `advance(10)` then `advance(20)` versus
`advance(20)`. Those cannot both hold: after `Advance(10)` completes, `F = 10`, so the
stepwise run's cohort at 15 would capture a different pre-wave digest than the one-call
run (`F = 0` throughout). Model S3d shows the divergence at exactly the cohorts at 15
and 20.

**Treatment.** Two digests with two jobs, neither weakened:

- `engine_state_digest` — the v1 §8 composition, **unchanged**, excluding `F`. It is the
  pre-wave engine digest bound into every `effect_batch_digest`. No evaluation reads
  `F`, so nothing evaluation-relevant is omitted. **Per-cohort claim, unchanged:**
  identical across every partition of advance calls and every budget, by cohort-sequence
  position (model S2, S3).
- `stable_boundary_digest = H("stable_boundary_v1" ‖ engine_state_digest ‖ F)` — the
  commitment for snapshots, restore validation, and the AT-G equal-state /
  same-next-input tests. **Request-boundary claim:** identical across partitions **at
  equal completed horizon** (model S3b), and deliberately different at unequal completed
  horizons (model S3c), because two engines that have completed to different horizons
  do respond differently to the same next past-dated request. That difference is the R1
  law working, not partition equivalence failing.

Partition equivalence is therefore stated as: per-cohort canonical values are invariant
under call partition and budget; engine state including `F` is invariant at equal
completed horizons. Nothing previously claimed is weakened; the boundary at which each
claim holds is now exact.

---

## 6. Amendments

**Disappear:** A-1 (`F` into `engine_state_digest`) — contradicted by §5; A-2
(`ExecutionCursor`) — no state; A-3 (ADR-0003 staging/fence effective-time addendum) —
the check moved to request start, which is a Phase-2 engine contract, and the timeline
is used unchanged one command at a time.

**Still needed, exact wording — one amendment to v1 §8 (proposed, not enacted):**

> Add to the retained-store table the row
> `HorizonFrontier F | canonical scalar | committed in stable_boundary_digest; AT-G
> discrimination/equivalence from first commit`.
> Add after the engine-digest definition: "`engine_state_digest` is the pre-wave engine
> digest and is unchanged. Snapshots, restore validation, and equal-state tests commit to
> `stable_boundary_digest = H("stable_boundary_v1" ‖ engine_state_digest ‖ F)`, where
> `F` is the horizon of the last completed request."

No ADR is amended. ADR-0003 §6 (retry, never eviction), §8 (fence), §13 (convenience
call), §14 (barrier), and §15 (monotonic host time) are used as written. Blueprint
§19.4 (bound work per cycle) is honored by pause. Budget §3 items 2–4 are honored:
pacing changes only the number of calls a request needs (model S2).

---

## 7. Consumer contract (the whole of what the host must do)

1. Offer requests to a bounded FIFO; on backpressure, wait or drop at the producer.
2. Present the head with `process(head)`; repeat while `PAUSED`.
3. On `COMPLETED` or `HORIZON_BEHIND_FRONTIER`, pop the head.
4. Never present a different request while the head is incomplete.

Embedded and standalone forms differ only in what the mailbox is made of; the engine
sees the same request sequence and produces the same canonical results (ADR-0005).

---

## 8. Supersession map (delta over the command-time candidate)

| Command-time candidate item | Disposition |
|---|---|
| ALT-U unified pending-command loop; `k` candidates from cursors; `(time, profile, kind)` tie rule | **WITHDRAWN** — commands never coexist with pending scheduled work as inputs; a command executes at its request's completion |
| `X` execution cursor; amendment A-2 | **REMOVED** |
| `C` finalized time ceiling; staging/fence preflight; amendment A-3 | **REMOVED** — subsumed by `h ≥ F` at request start |
| `F := T` at call entry; amendment A-1 | **SUPERSEDED** — `F := h` at completion only; committed in `stable_boundary_digest`, not in `engine_state_digest` |
| §5.3 loop for scheduled slices | **RETAINED** inside the active request (§1 Process) |
| §6 traces | **RE-DEMONSTRATED** by model S2 and S4b |
| §7 PE-A′ equivalence | **RESTATED** in §5 with the exact boundary for each claim |
| §10 downstream checklist V2-03…V2-10 | **CARRIED FORWARD** unchanged, except: delete the `X`-related rows; AT-I43 becomes the request-lifecycle oracle (start/pause/resume/complete/next, `h ≥ F`, budget-independent order), AT-I29(d) becomes AT-G tests of `F` under `stable_boundary_digest` |
| §11 item 1 (per-profile time) | **CLOSED for this contract**: one request stream, one horizon; the timeline is used one command at a time |
| §11 item 2 (stuck staged command) | **CLOSED**: nothing is staged before completion |

---

## 9. Model summary

`model.py` (disposable, ~200 lines, standard library only) runs eleven checks, all
passing: S1 second request waits while first is paused; S2 budgets 1 and 2 identical
(diagnostics differ); S3 one catch-up equals five completed catch-ups including a
work-producing cohort whose created work lands inside the horizon; S3b/S3c
stable-boundary digests agree at equal completed horizon and differ otherwise; S3d the
negative control for §5; S4 past-dated rejection; S4b review §5.3 with equal digests;
S5 restart from stores plus `F`; S6 no backwards evaluation. It models no transport,
no concurrency, and no performance.

---

## 10. Unresolved and nonclaims

Unresolved: whether the Phase-3 facade exposes `F` read-only to producers for
pre-validation (non-normative); mailbox capacity as a declared deployment value (budget
§3 item 1). Nonclaims: V3-F01 closure; acceptance of any candidate; enactment of the
v1 §8 amendment; freeze; implementation or Phase-3 authorization; any Phase-1 change;
any G.A.M.E. change; that the model is production or performance evidence.

**Verdict of this pass:** `SERIALIZED_REQUEST_BOUNDARY_ADDENDUM_READY`
