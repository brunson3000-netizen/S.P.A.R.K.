# S.P.A.R.K. Gate C1 — Independent V3-F01 Serialized-Boundary Review

**Date:** 2026-09-10

**Reviewer:** Codex, independent of the Fable addendum writer

**Repository:** `brunson3000-netizen/S.P.A.R.K.`

**Branch reviewed:** `phase1-refoundation-v2`

**Reviewed HEAD:** `e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8`

**Primary candidate:**
`SPARK_PHASE_2_V3_F01_SERIALIZED_REQUEST_BOUNDARY_ADDENDUM_2026-09-06.md`

**Verdict:** `V3_F01_BOUNDED_REVISION_REQUIRED`

## 1. Executive adjudication

The serialized-request addendum is the strongest current basis for closing V3-F01. It
correctly withdraws the pacing-causal `Phi`/`X` design, keeps command finalization and
execution together, evaluates scheduled cohorts at their own due times, and separates
the per-cohort `engine_state_digest` from a request-boundary commitment containing the
completed horizon `F`. The disposable model reproduces all eleven claimed scenarios.

It is not yet safe to accept or freeze. One concrete pause/resume counterexample defeats
the statement that the engine needs no active-request state and remains safe for any call
sequence. The next writer must either make exact-head resumption mechanically binding at
the engine boundary or explicitly define and prove an atomic engine-plus-mailbox state
boundary that makes substitution impossible. A prose consumer precondition alone does
not support the addendum's stronger safety and restart claims.

The proposed v1 section 8 amendment for `F` is technically sound, but Operator adoption
is still required. No Phase-2 implementation authority is released by this review.

## 2. Material reviewed

- the serialized-request addendum, report, executable model, and preserved outputs;
- the command-time adjudication and its downstream V2-03 through V2-10 checklist;
- the V3-F01 V2 correction candidate, matrix, writer report, and independent review;
- the foundational time adjudication and first V3-F01 review cycle;
- Phase-2 freezes v1 through v3 and their acceptance matrices;
- the final v3 confirmation that opened V3-F01;
- Phase-0 blueprint, ADR-0001, ADR-0002, ADR-0003, ADR-0005, and ADR-0006;
- the live Phase-1 `LogicalClock`, scheduler, timeline, engine state, manifests, and
  dependency-direction tests;
- the active S.P.A.R.K.-G.A.M.E. convergence protocol and September 9 handoff.

There is no repository-level `AGENTS.md`, `CLAUDE.md`, or `CODEX.md` at reviewed HEAD.

## 3. Accepted foundation

The following may be carried into the correction writer pass without reopening them:

1. One bounded FIFO consumer serializes external `Advance` and `Command` requests.
2. A request horizon is the advance target or command effective time.
3. Scheduled processing repeatedly selects the live least resident
   `(due_time, profile_id)` slice not beyond the active horizon.
4. A scheduled cohort evaluates with `now = due_time`, never the request horizon.
5. Newly scheduled work is strictly later than its creator and is reconsidered from the
   live scheduler rather than a retained call-entry plan.
6. The scheduler compare-and-take uses a full slot fingerprint, removes only the live
   least slice, and preserves inherited `schedule` and `drain_due` behavior.
7. Conflicted slots are part of the operational extraction but not executable identity
   or pacing cost.
8. Per-cohort canonical values bind the unchanged `engine_state_digest`, excluding `F`.
9. Stable request-boundary persistence and equal-state tests bind
   `stable_boundary_digest = H("stable_boundary_v1" || engine_state_digest || F)`.
10. `F` changes only when a request completes; a request with `h < F` is refused before
    mutation and never enters canonical history.
11. A command is finalized and executed as one command barrier only after scheduled work
    through its horizon completes. There is no finalized-but-unexecuted command at a
    stable boundary.
12. The command-time candidate's `Phi`, execution cursor `X`, time ceiling `C`, and
    amendments A-1 through A-3 remain withdrawn.

These decisions preserve pacing neutrality: budget changes call count and diagnostics,
not the canonical cohort sequence or per-cohort results.

## 4. Blocking finding

### SB-01 — paused-request substitution can cause backward evaluation

The addendum makes `Pause` a stable boundary, leaves `F` unchanged until completion, and
retains no active-request identity or horizon in the engine. It then says a
non-serializing consumer produces a different but safe replayable history.

Counterexample:

1. `F = 0`; the scheduler contains work at times 10, 15, and 20.
2. `Advance(20)` processes the 10 and 15 slices, then pauses for lack of budget.
3. Canonical cells may now have `updated_at = 15`, while `F` is still 0.
4. A faulty or mismatched consumer presents `Command@12` instead of resuming the head.
5. The specified start guard accepts it because `12 >= F`.
6. The command evaluates at 12 against state already updated at 15, violating
   ADR-0003's monotonic-time rule and the addendum's own `now >= updated_at` claim.

The same hole appears after restart if the engine snapshot and durable mailbox head are
not restored as one matched persistence unit. The engine cannot distinguish the correct
paused head from a replacement.

This does not invalidate horizon expansion, cohort-local evaluation, `F`, or the two-
digest design. It invalidates only the zero-active-state claim and the assertion of safe
total behavior for arbitrary call sequences.

## 5. Required bounded correction

The next writer must choose one complete, falsifiable mechanism; it must not mix partial
versions:

- **Recommended:** retain an engine-owned `ActiveRequest` discriminator only across a
  paused boundary. It minimally binds request kind, request identity/payload digest, and
  horizon. A different request while active receives a typed, non-mutating refusal. It is
  committed in `stable_boundary_digest`, excluded from the per-cohort digest, snapshot-
  carried, restored, and cleared atomically with `F := h` at completion. Starting a
  request sets it before any cohort mutation. Resuming the identical request is
  idempotent.
- **Alternative:** define the engine state and durable mailbox head as one atomic
  persistence object with a joint digest and prove that no API or recovery path can
  present a replacement head. This is larger and couples Gate C1 to host persistence, so
  it is not recommended.

The recommended correction adds one small protocol-state item; it does not add a causal
primitive, scheduler operation, pending-command list, execution cursor, or transport.
It preserves per-cohort partition equivalence because `ActiveRequest` is identical for
all partitions of the same request and stays outside `engine_state_digest`.

## 6. Oracle corrections required in the writer pass

Carry V2-03 through V2-10 from the command-time adjudication, adjusted by the serialized
addendum, and add:

1. pause after processing through time 15, substitute `Command@12`, and assert a typed
   no-op refusal with byte-identical state;
2. pause, restart from snapshot, re-present the exact request, and assert byte-identical
   continuation;
3. pause, restart, present a different request with the same horizon, and assert refusal;
4. two budgets processing the same request: compare per-cohort values by sequence
   position and compare stable-boundary state only at equal progress/completion points;
5. discrimination/equivalence tests for `ActiveRequest` in `stable_boundary_digest`;
6. prove that `ActiveRequest` is absent after completion and after a start-time
   `HORIZON_BEHIND_FRONTIER` refusal;
7. preserve the addendum model's negative control showing why `F` must stay out of the
   per-cohort digest;
8. prove the engine-plus-obligation-store extraction is preflighted and atomic before
   either store mutates, as required by V2-05.

The writer must also correct the old V2 matrix assertions already rejected by the prior
review: budget-one next-cohort admission, mixed-conflict pre-wave digest inequality,
fresh-key conflict creation, whole-cohort rollback after later-wave rejection, public
`spark-core` visibility claims, and non-discriminating loop-cost tests.

## 7. Operator decision

One frozen-architecture amendment is ready for decision, extended for the required
paused-request discriminator:

> Add to the Phase-2 v1 section 8 retained-store table:
> `HorizonFrontier F | canonical request-boundary scalar | committed in
> stable_boundary_digest; snapshot-carried; AT-G discrimination/equivalence from first
> commit` and `ActiveRequest | canonical request-boundary Option<RequestDiscriminator> |
> present only while paused/processing; committed in stable_boundary_digest;
> snapshot-carried; AT-G discrimination/equivalence from first commit`.
>
> Add after the engine-digest definition: `engine_state_digest` remains the per-cohort
> pre-wave digest and excludes request-boundary protocol state. Snapshots, restore
> validation, and equal-state tests commit to
> `stable_boundary_digest = H("stable_boundary_v1" || engine_state_digest || F ||
> canonicalize(ActiveRequest))`. A start-time refusal mutates neither value. Completion
> atomically clears `ActiveRequest` and sets `F` to the completed request horizon.

No ADR-0003 amendment is required: the correction mechanically enforces the addendum's
serialized-head precondition and its existing monotonic-time rule.

## 8. Verification performed

- Live GitHub and local reviewed branch both resolved to `e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8` at entry.
- Working tree was clean at entry.
- The serialized-request Python model reran locally: eleven checks passed.
- `git diff --check` passed before this review artifact was added.
- Rust/Cargo was not available in this execution environment, so the 232-test Rust
  baseline was not independently rerun here. The reviewed September 9 reconciliation
  record reports 232 passing workspace tests at the same HEAD. This review does not
  present that earlier run as fresh evidence.

## 9. Verdict and next gate

`V3_F01_BOUNDED_REVISION_REQUIRED`

V3-F01 remains open. Phase-2 architecture is not accepted or frozen; Phase-2 Rust and
Phase 3 remain unauthorized. The next permitted action is an independent architecture
writer pass incorporating the accepted foundation, SB-01, the corrected oracle, and the
Operator's disposition of the section 8 amendment. After that pass, an independent
reviewer must adjudicate the exact writer commit before implementation is released.
