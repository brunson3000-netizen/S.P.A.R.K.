# S.P.A.R.K. Phase 2 — V3-F01 Final Bounded Architecture Correction Candidate

**Date:** 2026-09-10
**Agent:** Claude Code (Fable 5.1), separated V3-F01 final correction writer. The mission
text named Claude Code Opus at high reasoning; the model actually used is recorded here
truthfully (see the writer report §2). The independent reviewer is Codex and is not the
writer.
**Status:** **CANDIDATE — AWAITING INDEPENDENT REVIEW.** Not accepted, not frozen, not
canonical; authorizes no implementation. V3-F01 remains **OPEN**.
**Form:** one consolidated bounded correction to
`SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md`, incorporating the accepted
foundation of the serialized-boundary independent review, the Operator's `ActiveRequest`
decision, and the corrected V2-03 … V2-10 downstream items.
**Supersedes (as a proposal), each preserved unchanged as history:**

| Superseded artifact | Commit |
|---|---|
| first-pass candidate `SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_2026-09-06.md` and matrix | `03daa82032cecc6ed84407b440bc9eab06cbcd69` |
| foundational-time adjudication `…_FOUNDATIONAL_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md` | `379f8dc355125e6bca09801e4a66b0b35ec9720c` (adopted, not superseded — see §3) |
| V2 candidate `…_ARCHITECTURE_CORRECTION_CANDIDATE_V2_2026-09-06.md` and matrix V2 | `983a01fd6807b8c627c97d2673a1f40c54cc059c` |
| command-time adjudication `…_COMMAND_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md` | `53a8432b9f0f3bad99c273d4ff7b52f4c63b5340` |
| serialized-request addendum `…_SERIALIZED_REQUEST_BOUNDARY_ADDENDUM_2026-09-06.md` | `30626abe4ec790e5f6a0287c3cc0b6d85470a893` |

**Controlling inputs (binding on this pass):**
`SPARK_PHASE_2_CODEX_V3_F01_SERIALIZED_BOUNDARY_INDEPENDENT_REVIEW_2026-09-10.md` (at
`bb1ccbb41f3a53948e3a87cd2b310e92ab1fa1d3`, verdict `V3_F01_BOUNDED_REVISION_REQUIRED`,
finding SB-01), `SPARK_PHASE_2_V3_F01_ACTIVE_REQUEST_OPERATOR_DECISION_2026-09-10.md`
(`ACTIVE_REQUEST_AUTHORIZED`), and the V2 independent review's V2-03 … V2-10.
**Companion oracle:**
`SPARK_PHASE_2_ACCEPTANCE_TEST_ORACLE_V3_F01_FINAL_2026-09-10.md`
**Evidence:** `engineering/phase2/v3_f01_final_correction_model_2026-09-10/` — a
disposable Python model (`model.py`, `results.txt`, `results.json`), architecture evidence
only: 34 checks, all passing, including every check of the 2026-09-06 model re-run.

**Nonclaims, first.** This candidate does not claim that V3-F01 is closed, that any
prior candidate is accepted, that Phase-2 architecture is frozen, that Phase-2 production
Rust or Phase 3 is authorized, that Operator acceptance of this text has occurred, that
any cross-platform parity exists, or that G.A.M.E. is changed. No Rust and no test was
written; no Rust test was run.

---

## 1. Baseline and lineage

| Item | Verified value |
|---|---|
| Repository | `/home/chromikey/Projects/SPARK` (worktree `/home/chromikey/Projects/SPARK-v3-f01-writer`) |
| Production branch | `phase1-refoundation-v2` at `e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8` (local and `origin` agree) |
| Required review branch / commit | `review/v3-f01-serialized-boundary-20260910`; `bb1ccbb41f3a53948e3a87cd2b310e92ab1fa1d3` — ancestor of this pass's parent |
| Parent of this pass | `772e38da0d130d7a1225ba2eb60a75b3999336fe` ("Record ActiveRequest decision and writer mission"), sole child of `bb1ccbb…`, equal to the remote review tip at entry |
| Candidate branch | `candidate/v3-f01-final-correction-20260910` |
| Lineage | `5fd556b` → `03daa82` → `513c398` → `379f8dc` → `983a01f` → `8d0dba9` → `53a8432` → `30626ab` → … → `e9e26e8` → `bb1ccbb` → `772e38d` |
| Worktree at entry | clean |
| Phase 0 / Phase 1 | CLOSED, not reopened |
| Phase 2 | architecture-only; controlling verdict `PHASE_2_ARCHITECTURE_V3_REVISE`; latest Gate C1 verdict `V3_F01_BOUNDED_REVISION_REQUIRED` |
| Phase 3 | NOT AUTHORIZED |

Every path cited below was read in this pass. Phase-1 seams inspected directly:
`crates/spark-core/src/clock.rs` (`LogicalTime`, `LogicalClock::advance_to`),
`crates/spark-core/src/scheduler.rs` (`WorkKey`, `WorkPayload`, `DueWorkItem`,
`SlotState`, `WorkSlotStatus`, `WorkKeyConflict`, `DrainOutcome`, `schedule`,
`drain_due`, `slot_status`, `conflict_of`, `canonical_state_digest`),
`crates/spark-core/src/timeline.rs` (`SemanticCommandEnvelope`, `AdmissionWindow`,
`TimelineIngress::stage` / `submit_fence` / `current_admission_window` /
`frontier_ordinal`), `crates/spark-core/src/hash.rs` (`CanonicalEncoder`),
`crates/spark-core/src/lib.rs`, `crates/spark-engine/src/lib.rs`,
`crates/spark-engine/src/state.rs`, all three crate manifests, and
`crates/spark-testkit/tests/workspace_dependency_direction.rs`.

---

## 2. What this candidate answers

The serialized-boundary review accepted the addendum's foundation (its §3, twelve items)
and left one blocker, **SB-01**: after a paced pause, `F` is unchanged and the engine
retained no active-request identity, so a substituted request with `h ≥ F` — the review's
`Command@12` after work through 15 was processed — would evaluate backward. The Operator
authorized the review's recommended mechanism: an engine-owned `ActiveRequest`
discriminator committed in the stable-boundary digest.

This candidate (a) defines `ActiveRequest` completely and makes exact-head resumption
mechanically binding at the engine boundary, so no prose consumer precondition is the only
enforcement; (b) consolidates the accepted foundation, the V2 scheduler surface, and the
corrected V2-03 … V2-10 items into one document; and (c) supplies the corrected oracle
in the companion document. Nothing accepted is reopened.

---

## 3. Accepted foundation, carried forward without reopening

Each item is restated normatively in the section named; the source is the serialized-
boundary review §3 unless stated.

| # | Foundation item | Where restated |
|---|---|---|
| 1 | One bounded FIFO consumer serializes external `Advance` and `Command` requests | §5, §17 |
| 2 | A request horizon is the advance target or the command effective time | §4 |
| 3 | Scheduled processing repeatedly selects the live least resident `(due_time, profile_id)` slice not beyond the active horizon | §9 A2 |
| 4 | A scheduled cohort evaluates with `now = due_time`, never the request horizon | §4, §9 A4 |
| 5 | Newly scheduled work is strictly later than its creator and is reconsidered from the live scheduler, never from a call-entry plan | §4, §9 |
| 6 | Compare-and-take uses a full slot fingerprint, removes only the live least slice, preserves inherited `schedule` and `drain_due` | §10 |
| 7 | Conflicted slots are part of the operational extraction but not executable identity or pacing cost | §12 |
| 8 | Per-cohort canonical values bind the unchanged `engine_state_digest`, excluding `F` (and now `ActiveRequest`) | §14 |
| 9 | Stable request-boundary persistence and equal-state tests bind `stable_boundary_digest` | §14 |
| 10 | `F` changes only at request completion; `h < F` is refused before mutation and never enters canonical history | §7 |
| 11 | A command is finalized and executed as one barrier only after scheduled work through its horizon completes; no finalized-but-unexecuted command exists at a stable boundary | §8 |
| 12 | `Phi`, execution cursor `X`, separate ceiling `C`, and amendments A-1 … A-3 remain withdrawn | §19 |
| — | The foundational-time adjudication's A4 rule (horizon expansion, per-barrier canonical time, eligibility at the created barrier, checked delayed-time arithmetic) — adopted by the V2 candidate and accepted by both later reviews | §4 |
| — | Executable-only `scheduled_cohort_identity` (v3 §4.1 formula over `Scheduled` keys); `test-support`-gated pre-wave observation; no canonical report field added | §12, §15 |

No new host, service, transport, cancellation, arbitrary-removal, or batch-scheduling
surface is introduced anywhere below.

---

## 4. Vocabulary (binding)

- **Request.** One external input: `Advance{T}` or `Command{request}` (§8 defines the
  command request's fields). Its **horizon** `h` is `T` or the command's `effective_time`.
- **Horizon.** An inclusive eligibility bound on which resident scheduled work may be
  consumed by the request. Never the canonical time of anything evaluated.
- **Canonical time of a cohort.** A scheduled cohort's `due_time`; a command cohort's
  `effective_time`. Every inherited use of `now`, `at`, and "current logical time" during
  that cohort's evaluation denotes this value. Delayed work created by a cohort carries
  `due_time ≥ canonical_time + 1` under checked `u64` addition; overflow rejects the wave
  atomically (R-3).
- **Slice.** All occupied scheduler slots sharing one `(due_time, profile_id)`; the
  **operational extraction unit**. Its **executable cohort** is its `Scheduled` members.
- **Stable boundary.** Any point at which the engine's canonical state is complete and no
  evaluation is in progress: between cohorts inside a call, and the state returned by
  every `process` call (`PAUSED`, `COMPLETED`, `COMPLETED_COMMAND_NOT_FINALIZED`, and both
  start-time refusals, which return the unchanged boundary). Mid-cohort inter-wave points
  are not stable boundaries (ADR-0003 §15).
- **`F` — horizon frontier.** The horizon of the last completed request (§7).
- **`ActiveRequest`.** The engine-owned discriminator of the request in progress or
  paused, `None` otherwise (§6).

---

## 5. Request lifecycle (revises addendum §1 in exactly the Start, Pause, Resume, and Complete rows)

| Phase | Where | Canonical? | Definition |
|---|---|---|---|
| **Accept** | mailbox (host adapter, embedded or standalone) | no | The producer offers the request to a bounded FIFO. If full, the producer receives backpressure (`MAILBOX_FULL`, or blocks, per transport). Nothing reaches the engine. Acceptance carries no ordering or admission authority over canonical state. |
| **Start** | consumer → `Engine::process(R)` | request-boundary | The engine computes the discriminator `d(R)` (§6.2). **If `ActiveRequest = Some(a)`:** `a = d(R)` → resume (below); `a ≠ d(R)` → `REFUSED_ACTIVE_REQUEST_MISMATCH { active: a, presented: d(R) }`, typed, non-mutating, not history. **If `ActiveRequest = None`:** `h < F` → `REFUSED_HORIZON_BEHIND_FRONTIER { F, h }`, typed, non-mutating, not history; otherwise `ActiveRequest := Some(d(R))` **before any cohort mutation**, and processing begins. |
| **Process** | engine | yes, per cohort | The loop of §9 consumes scheduled slices with `due_time ≤ h` in ascending `(due_time, profile_id)` order under the epoch-bound budget `B`, each executable cohort at `now = due_time`. |
| **Pause** | engine → consumer | stable boundary | Budget exhausted with an executable slice `≤ h` resident, or a command not yet executed for lack of budget: the call returns `PAUSED` with `PacingDiagnostics`. `F` unchanged. `ActiveRequest` unchanged (`Some(d(R))`). The request stays at the mailbox head. **No other request can start** — enforced by the Start row, not by convention. |
| **Resume** | consumer → engine | no | The consumer presents the head again. The discriminator matches; the engine continues from live canonical state. `h ≥ F` holds by construction because `F` changed only at completions and none has occurred since start. Resumption is idempotent: nothing carried over from the paused call is needed or read. |
| **Complete** | engine | yes | No resident slice has `due_time ≤ h`. For a command: the request is finalized (§8) and, on success, executed as its own cohort at `now = effective_time`, atomically within this call; the command cohort weighs one budget unit and, if none remains, the call returns `PAUSED` and the command executes on the next call. Then **atomically** `F := h` and `ActiveRequest := None`, and the call returns `COMPLETED` (or `COMPLETED_COMMAND_NOT_FINALIZED`, §8.3). |
| **Next** | consumer | — | Only after a terminal result (`COMPLETED`, `COMPLETED_COMMAND_NOT_FINALIZED`, `REFUSED_HORIZON_BEHIND_FRONTIER`, `REFUSED_ACTIVE_REQUEST_MISMATCH`) does the consumer pop the head and present the next request. |

Model evidence: S1, S2, S7, S7b, S7c, S10, S10b, S19.

---

## 6. `ActiveRequest` — complete definition

### 6.1 Owner, type, and placement

- **Owner:** the Phase-2 engine composition (`spark-engine`), alongside `F`. It is
  **request-boundary protocol state**: not a cohort input, not a store, not readable by
  rule evaluation.
- **Type:** `Option<RequestDiscriminator>`.
- **Class (v1 §8 table, amended per the Operator decision):** canonical request-boundary
  value; committed in `stable_boundary_digest`; excluded from `engine_state_digest`;
  snapshot-carried; AT-G discrimination/equivalence from its first implementation commit.

### 6.2 Fields and canonical encoding

```text
RequestDiscriminator {
    kind:      RequestKind          = Advance | Command          (canonical tag)
    horizon:   LogicalTime          = T | effective_time
    identity:  Digest
}

identity(Advance{T})        = H("request_advance_v1"  ‖ T)
identity(Command{request})  = H("request_command_v1"  ‖ CommandRequest::canonicalize)   (§8.1)

canonicalize(ActiveRequest):
    None    → push_str("active_request.none")
    Some(d) → push_str("active_request.some") ‖ push_str(kind tag)
              ‖ horizon.canonicalize ‖ push_digest(identity)
```

Encoding uses the Phase-1 `CanonicalEncoder` discipline (`push_str`, `push_u64`,
`push_digest`) with domain-separation tags; the `None`/`Some` tags make the two cases
non-colliding. The identity of an `Advance` is a function of `T` alone, so two advance
requests to the same horizon are the same request; the identity of a command is the
complete canonical command request, so two commands at one effective time with any
differing field are different requests. Equality of discriminators is field-wise
equality; because `kind` and `horizon` are also inside `identity`'s preimage they are
carried explicitly for read-only observation and typed refusal reporting, not for extra
discrimination.

### 6.3 Every transition

| Event | `ActiveRequest` | `F` | Stores |
|---|---|---|---|
| Engine genesis | `None` | genesis time (`ZERO` by default) | initial |
| Start, `ActiveRequest = None`, `h ≥ F` | `:= Some(d(R))` before any cohort mutation | unchanged | unchanged at this instant |
| Start, `ActiveRequest = None`, `h < F` | unchanged (`None`) | unchanged | unchanged — `REFUSED_HORIZON_BEHIND_FRONTIER` |
| Start, `ActiveRequest = Some(a)`, `a = d(R)` | unchanged (resume) | unchanged | continue from live state |
| Start, `ActiveRequest = Some(a)`, `a ≠ d(R)` | unchanged | unchanged | unchanged — `REFUSED_ACTIVE_REQUEST_MISMATCH` |
| Scheduled cohort committed / rejected (R-1 … R-7) / all-conflicted slice consumed | unchanged | unchanged | per cohort outcome |
| Pause (`PAUSED`) | unchanged (`Some`) | unchanged | as committed so far |
| Completion (`COMPLETED`) | `:= None` | `:= h` | as committed; atomic with the two assignments |
| Completion with command finalization refused (§8.3) | `:= None` | `:= h` | scheduled work committed; nothing staged/finalized |
| Snapshot | carried verbatim | carried | carried |
| Restore | restored verbatim after validation (§6.5) | restored | restored |
| Epoch reset (ADR-0003 §11), epoch activation, any host operation | **no path changes it** | unchanged | — |

There is **no other transition**. In particular there is no cancel, abandon, replace, or
host-set path: the only way `Some(d)` becomes `None` is completion of the request `d`.
This is the fail-closed property SB-01 requires, and it is the reason a substituted
request cannot evaluate backward: while `Some(d)` is present, only `d` can mutate cohort
state, and `d`'s horizon is the bound under which every already-processed cohort was
consumed.

### 6.4 Digest

```text
engine_state_digest    = v1 §8 composition, UNCHANGED; excludes F and ActiveRequest
stable_boundary_digest = H("stable_boundary_v1" ‖ engine_state_digest ‖ F ‖ canonicalize(ActiveRequest))
```

`stable_boundary_digest` is the commitment for snapshots, restore validation, and the
AT-G equal-state / same-next-input tests at request boundaries. Model S9: two engines
with equal `engine_state_digest` and equal `F` but `None` versus `Some`, or `Some`
values differing only in kind, only in horizon, or only in identity, have five distinct
`stable_boundary_digest`s. Model S9b: equal `stable_boundary_digest`s reached by permuted
construction respond identically (result and post-digest) to the same next requests.
Model S3e: including `ActiveRequest` in the per-cohort digest would break catch-up
equality at every cohort — the negative control that fixes its placement.

### 6.5 Snapshot and restore

A snapshot at a stable boundary carries the retained stores, `F`, `ActiveRequest`, and
the `stable_boundary_digest` computed at capture. Restore validates, in order, before
any state becomes live:

1. definition fingerprints and activation lineage as ADR-0002 §5 / ADR-0006 already
   require (unchanged);
2. `ActiveRequest = Some(d)` ⇒ `d.horizon ≥ F`, else typed
   `RESTORE_REJECTED_ACTIVE_BEHIND_FRONTIER` (a `Some` behind the frontier is not a
   reachable state: `F` is set only at completion, which clears `ActiveRequest`);
3. the recomputed `stable_boundary_digest` equals the recorded value, else typed
   `RESTORE_REJECTED_DIGEST_MISMATCH` (this binds `F` and `ActiveRequest` to the stores
   they were captured with, so neither can be edited independently).

After a restore with `Some(d)`, the engine accepts exactly the request `d` and refuses
every other request with `REFUSED_ACTIVE_REQUEST_MISMATCH`; the durable mailbox head
re-presents `d`. Model S5, S8, S8b, S8c.

### 6.6 Observation

The engine exposes `F` and `ActiveRequest` **read-only** to the consumer (non-normative
use: pre-validation of the mailbox head). Neither value is reachable from the evaluation
context; the compile-probe obligations of the oracle (AT-I32) cover both, exactly as they
cover the horizon and `PacingDiagnostics`. Neither is a canonical report field.

### 6.7 Why this is the whole mechanism

`ActiveRequest` adds one protocol-state item. It adds no causal primitive, no scheduler
operation, no pending-command list, no execution cursor, no transport, and no
persistence coupling to the host mailbox. Per-cohort partition equivalence is untouched
because `ActiveRequest` is identical for every partition of the same request and is
outside `engine_state_digest`. The review's alternative — an atomic engine-plus-mailbox
persistence object — is not adopted, for the review's stated reason (it couples Gate C1
to host persistence).

---

## 7. `F` — horizon frontier (carried; restated with the coupling)

Owner: engine, realized by the Phase-1 `LogicalClock` unchanged (`advance_to(h)` at
completion; its backward rejection is the same predicate as the start check, applied
without mutation at start). Type: `LogicalTime`. Initialization: genesis time. **Updated
only at request completion**, atomically with `ActiveRequest := None`, to `h` — for a
command, whether or not finalization succeeded (§8.3), because the scheduled horizon
through `h` has been consumed either way. Unchanged by pause, either start refusal,
restart, any cohort outcome, or any conflict disposition. Committed in
`stable_boundary_digest`; excluded from `engine_state_digest` (model S3d, the negative
control carried from the addendum).

The single rule that keeps global canonical time non-decreasing: **a request may start
only if `h ≥ F`** (equal permitted). With `ActiveRequest` binding the paused case, every
evaluation satisfies `now ≥ updated_at` by construction (model S6 asserts this in every
cohort of every scenario and never trips).

---

## 8. Command requests: identity, finalization at completion, refusal

### 8.1 The command request

```text
CommandRequest {
    command_id, profile_id, timeline_epoch, effective_time,
    source_id, source_sequence, command_kind, canonical_payload_hash
}
CommandRequest::canonicalize = the SemanticCommandEnvelope field order (timeline.rs)
                               with input_ordinal omitted
```

This is the Phase-1 `SemanticCommandEnvelope` minus the sequencer-issued `input_ordinal`.
**Decision D-1.** Under serialized processing the consumer is the active sequencer
(ADR-0003 §1) and requests are finalized one at a time in FIFO order, so a command's
ordinal is always the timeline frontier at its finalization. The engine therefore assigns
`input_ordinal := frontier_ordinal` at finalization, obtaining the current admission
window and staging, acknowledging, and fencing the single ordinal exactly as ADR-0003 §13
and ADR-0005 permit for the embedded convenience path. Consequences: a request refused at
start consumes no ordinal, no ordinal is ever burned, and the request's identity (§6.2)
is stable across pause, resume, and restart because it never depends on a value assigned
later. The alternative — pre-issued ordinals carried in the request — is rejected because
a start-time refusal would leave a gap that only an ADR-0003 §11 epoch reset can close.
The Phase-1 timeline code is unchanged.

### 8.2 Finalization at completion (foundation item 11, made exact)

When no resident slice has `due_time ≤ effective_time` and one budget unit remains (or
nothing was admitted this call):

```text
window   := timeline.current_admission_window()
envelope := SemanticCommandEnvelope { request fields, input_ordinal = window.frontier_ordinal }
stage(envelope.submit_with(window.ticket()))          → must return a positive STAGED acknowledgement
submit_fence(start = end = frontier_ordinal, ordered_stream_digest over the one envelope)
                                                       → must finalize
execute the command cohort at now = effective_time (identity v3 §4.2; waves per v2 §5.1)
F := effective_time;  ActiveRequest := None;  return COMPLETED
```

Stage, fence, and execution occur inside one call with no intervening stable boundary,
so no staged-unfinalized or finalized-unexecuted command is ever observable — including
in `engine_state_digest`'s timeline component. The command cohort's own wave outcome
(commit, or R-1 … R-7 rejection) is a cohort outcome inside `COMPLETED` and is reported
canonically; it does not change the request result.

### 8.3 Finalization refusal

If `stage` returns anything other than a positive `STAGED` acknowledgement (out-of-window,
identity reuse, source-sequence regression, epoch mismatch, poisoned slot, sequencer
authority) or `submit_fence` rejects, the timeline is unchanged (ADR-0003 §8 rule 8), the
command cohort is **not executed**, and the request completes as
`COMPLETED_COMMAND_NOT_FINALIZED { timeline_result }` with `F := effective_time` and
`ActiveRequest := None`. The scheduled work consumed through `effective_time` stays
committed: it is time-ordered canonical work that any later request with a horizon
`≥ effective_time` would have consumed identically. The host's remedy is the Phase-1
remedy for the reported timeline result; a re-presentation is a new request subject to
`h ≥ F`. **Decision D-2:** `F` advances on this path because the frontier tracks the
completed *horizon*, not the command's success; not advancing it would readmit a request
below cells already updated at `effective_time`. Model S17, S17b.

---

## 9. The canonical processing loop (one `process(R)` call)

```text
P0  d := discriminator(R);  h := horizon(R)
    if ActiveRequest = Some(a) and a ≠ d:   return REFUSED_ACTIVE_REQUEST_MISMATCH     [no mutation]
    if ActiveRequest = None and h < F:      return REFUSED_HORIZON_BEHIND_FRONTIER     [no mutation]
    if ActiveRequest = None:                ActiveRequest := Some(d)                   [before any cohort mutation]
P1  r := B;  admitted_executable := 0;  exception_fired := false
A2  s := least resident slice with due_time ≤ h                                        (X-1)
    if none: goto A6
    e := s.executable_count();  expected := s.fingerprint()                            [borrow ends]
A3  if e > 0:
        if exception_fired:                                     goto A7   (v3 §3.2(c): no other cohort)
        admit iff  e ≤ r  or  admitted_executable == 0          (v3 §3.2(b)(c))
        if not admitted:                                        goto A7   (v3 §3.2(d): defer, untouched)
        if e > r: exception_fired := true
    (e == 0: an all-conflicted slice is consumed unconditionally — it is not a cohort)
A4  extraction := Engine::extract_least_due_slice(h)                                    (§11; ATOMIC, both stores)
    report extraction.conflicted in Phase-1 conflicted-drain shape                      (§12.3 order)
    if e > 0:
        pre-wave engine digest is now well defined                                      (§15 observation)
        cohort_identity := v3 §4.1 over the executable keys
        evaluate waves 0..n with now = extraction.due_time; commit each validated wave;
        or take one of the rejection routes of §13
A5  STABLE BOUNDARY.  if e > 0: admitted_executable += 1;  r := exception_fired ? 0 : r − e
    drop the extraction; goto A2
A6  if R is a Command:
        if not (1 ≤ r or admitted_executable == 0):             goto A7   (command deferred; weighs one unit)
        finalize and execute per §8.2, or complete per §8.3
    F := h;  ActiveRequest := None   [atomic];  emit PacingDiagnostics;  return COMPLETED | COMPLETED_COMMAND_NOT_FINALIZED
A7  PAUSED: emit PacingDiagnostics (executable-only, §12.4); F and ActiveRequest unchanged; return PAUSED
```

Properties:

- **Time-ordered by construction.** Least-first selection, strictly-later created work,
  and the start rule make canonical time non-decreasing across the whole history.
- **Recomputed from live state at every A2.** No plan, cursor, tail buffer, or budget
  carry exists at any stable boundary.
- **`PAUSED ⇒ deferred_cohort_count ≥ 1` or the command is deferred.** Because
  all-conflicted slices are consumed unconditionally — also after the oversized exception
  — the least resident slice `≤ h` at A7 is executable (or the command awaits budget).
  **Decision D-3 (resolves V2-10 by changing the loop, not by narrowing the claim):** v3
  §3.2(c)'s "admit no other cohort this cycle" concerns cohorts; an all-conflicted slice is
  not a cohort (§12.2), costs no budget, and stopping before it would return `PAUSED` with
  nothing deferred. The narrowing alternative ("never stops progress before an oversized
  exception") is recorded and not chosen. Model S16, S16b.
- **Bounded per call.** At most `B` executable units plus one oversized exception plus
  zero-cost conflicted slices bounded by residency.
- **Termination.** Each iteration removes at least one occupied slot; created work is
  strictly later and bounded by `max_enqueue_per_wave`; due times are bounded by `h`.

---

## 10. Scheduler surface X-1 … X-4 (carried from the V2 candidate §5, with V2-08 corrected)

X-1 `least_due_slice(&self, horizon) -> Option<LeastDueSlice<'_>>` (exposes `due_time`,
`profile_id`, `executable_count`, `conflicted_count`, ascending `WorkKey`s with
`WorkSlotStatus`, `fingerprint`; no payload, no conflict evidence). X-2
`SliceFingerprint = H("due_slice_fingerprint_v1" ‖ due_time ‖ profile_id ‖
occupied_slot_count ‖ per-slot block)` where the per-slot block is byte-for-byte the block
`Scheduler::canonical_state_digest` builds (`scheduler.rs`, `slot.scheduled` /
`slot.conflicted` with the complete tracked claim set and truncation flag); owned,
retainable, **non-authorizing**. X-3 `take_least_due_slice(&mut self, horizon, expected)
-> Result<SliceExtraction, TakeSliceError>` with target fixed to the live least slice,
scan-before-mutation, typed byte-identical no-ops `NoSliceDue { least_resident_due_time }`
and `SliceChanged { observed }`. X-4 `due_slice_summary(&self, horizon)` read-only,
consumed only by `PacingDiagnostics` (§12.4). The V2 §5.4 failure table, §5.5 borrow
statement, and §5.6 closure of the replay / non-least / same-key TOCTOU counterexamples
are carried verbatim. Model S18.

**Visibility (V2-08 corrected).** X-1 … X-4 are `pub` items of the public
`spark_core::scheduler` module because ADR-0001's dependency direction leaves no other
placement, and `pub` is public to any crate that depends on `spark-core`. This candidate
makes **no Rust-privacy claim** about them. The enforced boundary is the **engine/host
facade**: the Phase-2 engine composition in `spark-engine` owns the `Scheduler` value
(no public accessor returns `&Scheduler` or `&mut Scheduler`), exposes only
`process`, read-only `F` / `ActiveRequest`, and the `test-support` observation seam, and
the oracle's compile probes target that concrete surface (AT-I32). `drain_due` remains
public and inherited; the Phase-2 evaluator does not call it, and X-3 is the only removal
path *the engine uses*, not the only one that exists on `Scheduler`.

---

## 11. Engine-owned atomic cross-store extraction (resolves V2-05)

```text
Engine::extract_least_due_slice(&mut self, horizon: LogicalTime)
    -> Result<CohortExtraction, ExtractionRefusal>

CohortExtraction {
    due_time, profile_id
    executable: Vec<(DueWorkItem, ObligationRecord)>      ascending WorkKey; the full record moves here
    conflicted: Vec<(WorkKeyConflict, ContestedClaimSet)> ascending WorkKey; the complete contested set
    fingerprint: SliceFingerprint
}
ExtractionRefusal = NoSliceDue { least_resident_due_time }
                  | ObligationRecordMissing  { key }
                  | ObligationRecordMismatch { key, expected_payload_hash, found }
                  | ObligationClaimSetMismatch { key, slot_claims, store_claims }
```

**Step 1 — preflight, read-only, under the engine's exclusive borrow.** Locate the live
least slice (X-1) and its fingerprint. For every slot of the slice: a `Scheduled` slot
must have exactly one obligation record under `WorkKey::identity_digest()` whose record
hash equals the slot's `canonical_payload_hash`; a `Conflicted` slot must have a contested
claim set whose record hashes equal the slot's complete tracked claim set (v2 §8). Any
failure returns the typed refusal with **neither store mutated** — both
`Scheduler::canonical_state_digest` and the obligation-store digest are byte-identical,
and nothing is evaluated.

**Step 2 — apply, infallible.** X-3 with the fingerprint just computed (it cannot fail:
the engine's `&mut self` excludes any interleaved mutation, and X-3's only failure modes
are themselves no-ops executed before any obligation removal), then remove every
extracted key's obligation record or contested set, moving the records into the
extraction. The evaluator receives the `MaterializedEffect` / `RuleReEvaluation` records
it must execute, which V2-05 found missing.

The v1 §8 scheduler ↔ `ObligationStore` bidirectional invariant therefore holds at every
observable point: before extraction, after it, at the pre-wave capture, and after any
rejection. The V2 candidate's "same atomic step" prose is replaced by this one operation.
Model S13.

---

## 12. Operational slice, executable cohort, conflicts, and diagnostics (resolves V2-04, V2-10)

### 12.1 Membership

| Concept | Membership | Consequences |
|---|---|---|
| Operational extraction slice | every occupied slot of the `(due_time, profile_id)` slice | extracted atomically (§11) so no conflicted slot lingers into a later pre-wave digest; conflicted keys reported |
| Executable cohort | the `Scheduled` members | `scheduled_cohort_identity` (v3 §4.1, executable keys only), wave input, pacing count, batch digests |

### 12.2 All-conflicted slice

Extracted whole; reported in conflicted-drain shape; no cohort identity, no wave, no
batch digest, no pre-wave capture; no budget consumed; not counted as an admitted cohort;
consumed unconditionally at A3, including after the oversized exception (D-3). Its
extraction removes its contested claim sets from the obligation store as complete sets
(v2 §8). Model S16.

### 12.3 Mixed slice and report order (V2-10)

Extracted whole. The canonical semantic report for the slice is ordered: (1) the conflict
section — every conflicted key in ascending `WorkKey` order with its `WorkKeyConflict`
value (retained hashes, `omitted_distinct`, `evidence_truncated`); (2) the executable
cohort's outcome — committed waves in wave order, or the typed rejection with
order-independent evidence (§13). Nothing in (1) enters (2): the conflicted keys are in
no identity, no candidate set, and no batch.

**V2-04 corrected.** For slices `A = {S}` and `B = {S, X_conflicted}` at one
`(t, P)`: extraction removes `S`, `X`, and their obligation records before the pre-wave
capture, so the two runs have **equal** cohort identity, emission identities, pre-wave
engine digests, `effect_batch_digest`s, and final state; **only** the conflict report
and the pre-*extraction* scheduler digest differ. The V2 candidate §6's last paragraph
and matrix AT-I40(e) asserted the opposite and are superseded. Model S15.

### 12.4 Executable-only diagnostics

`PacingDiagnostics` keeps its frozen v3 §3.4 field list; no field is added.
`deferred_cohort_count` counts resident slices with `due_time ≤ h` and
`executable_count > 0` at call end; `earliest_deferred_due_time` is the least such
`due_time`; `admitted_cohort_count` counts executable cohorts only. Residual conflicted
slots beyond the first deferred executable slice are not counted; they are consumed at
zero cost by the next call that reaches them. Model S15b, S16.

---

## 13. Rejection classes (resolves V2-06; R-8 deleted)

There is no rollback mechanism and none is added. For every class: the slice's keys are
terminally consumed at extraction (§11); their obligation records left the store in that
same operation and are not reinstated; nothing of the rejected wave is applied; the
canonical report carries the typed outcome with order-independent evidence; progress is
preserved. **What "nothing applied" means depends on the wave index:**

- **Wave-0 rejection.** No wave committed; the engine state equals the post-extraction
  state, whose digest is the captured pre-wave digest.
- **Later-wave rejection (wave `n ≥ 1`).** Waves `0 … n−1` were validated and committed
  each with its own batch digest and chained post-commit digest (v2 §5.1, v1 Q3); wave `n`
  applies nothing. The engine state equals the state after wave `n−1`'s commit — **not**
  the post-extraction state. There is no whole-cohort rollback; asserting equality with
  the post-extraction digest for a later-wave rejection is the false assertion V2-06
  identified. Model S14.

| Class | Trigger | Outcome | Retry |
|---|---|---|---|
| R-1 | contested emission identity; unequal same-family RESULT; cross-family mixture; multiple TRANSFORMs without a declared reducer (v2 §5.3) | atomic rejection of that wave | producer may reschedule, normally under the next occurrence index; same-key retry is permitted and is new canonical input |
| R-2 | authority, type, bounds, or scope-invalid effect | atomic rejection of that wave; ill-formed-artifact signal | same |
| R-3 | arithmetic failure: occurrence exhaustion, delayed-time overflow, canonical arithmetic | atomic rejection of that wave | same |
| R-4 | complete-transition preflight failure incl. queue admission caps and the bidirectional invariant on the post-state (v2 §7) | atomic rejection of that wave before its first mutation | same |
| R-5 | semantic-cap overflow (v2 §6.3, v3 §3.5) | atomic rejection plus the typed overload report in conflicted-drain shape; keys were already consumed at extraction | same; identical on replay |
| R-6 | conflicted slots | not a wave: reported per §12.3(1); contested claim sets removed as complete sets | key is free; reschedule normally under the next occurrence |
| R-7 | obligation execution refusal: `MaterializedEffect` target-fingerprint drift on any target; `RuleReEvaluation` resolution failure (v1 §5, v2 §8) | explicit typed refusal, nothing reinterpreted, no wave committed | same |
| ~~R-8~~ | ~~supported-history violation~~ | **DELETED** with SH-1/SH-2 and `Phi` (command-time adjudication); no canonical command refusal exists | — |

Command cohorts perform no extraction and have no `WorkKey`; classes R-1 … R-5 and R-7
apply to a command cohort's waves with the same wave-index rule. A command cohort
rejected at wave `n` leaves the command finalized in the timeline (ordinal consumed) with
the typed outcome in its canonical report; no re-finalization path exists (a new command
is new history).

---

## 14. The two digests, and the v1 §8 amendment

```text
engine_state_digest    = H("engine_state" ‖ state-store digest ‖ scheduler digest ‖ timeline digest
                           ‖ epoch-registry digest ‖ obligation-store digest
                           ‖ occurrence-ledger digest ‖ cooldown-ledger digest)          — v1 §8, UNCHANGED
stable_boundary_digest = H("stable_boundary_v1" ‖ engine_state_digest ‖ F ‖ canonicalize(ActiveRequest))
```

- `engine_state_digest` is the per-cohort pre-wave digest bound into every
  `effect_batch_digest` (v3 §4.5). No evaluation reads `F` or `ActiveRequest`, so
  nothing evaluation-relevant is omitted. **Claim:** identical across every partition of
  requests and every budget, by cohort-sequence position (model S2, S3, S12).
- `stable_boundary_digest` is the commitment for snapshots, restore validation, and the
  AT-G equal-state / same-next-input tests. **Claim:** identical across partitions at
  equal completed horizon with no active request (S3b); identical across budgets at equal
  progress points — same number of cohorts processed and same `ActiveRequest` (S12);
  deliberately different at unequal completed horizons (S3c) and between a paused and a
  completed engine at the same cohort position, because those engines respond differently
  to the same next request (S7 versus S10b). The differences are the R1 law working.

**Amendment to Phase-2 v1 §8, authorized by the Operator decision of 2026-09-10 and
written here as text subject to independent review:**

> Add to the retained-store table:
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

No ADR is amended. ADR-0003 §6, §8, §11, §13, §14, and §15 are used as written; the
correction mechanically enforces the serialized-head precondition and the monotonic-time
rule. Blueprint §19.4 is honored by pause; budget §3 items 2–4 by pacing neutrality.

---

## 15. Observation without a canonical report surface (carried)

CE-8 remains withdrawn. Pre-wave engine and scheduler digests are observed through the
existing `test-support` cargo feature (declared on `spark-core` and `spark-engine`,
dev-enabled only by `spark-testkit`, mechanically guarded by
`workspace_dependency_direction.rs`), exposed under
`#[cfg(any(test, feature = "test-support"))]`. Three separate obligations are stated
honestly (V2 review §8 item 12): the feature is absent from every production dependency
edge (the hygiene test); the canonical report schema is unchanged (a pinned field set);
and the observed values cannot reach evaluation (a compile probe over the evaluator's
input types, not the feature flag).

---

## 16. Equivalence: preconditions, claims, non-claims

Fix an activated artifact, one behavior epoch, one cap set (PE-D). Compared runs have the
same initial state and the same finalized command stream (PE-A); different budgets are
permitted; different caps, epochs, or rule sets are different declared behavior and are
never compared.

- **Lemma 1 / Lemma 2 / Theorem P′ / Corollary R-038** (V2 candidate §10.2, foundational
  adjudication §8): carried verbatim. Created keys are derived, not assumed.
- **Theorem P′-requests.** Two request sequences with the same finalized command stream
  and, between consecutive commands, non-decreasing advance horizons ending at the same
  value, produce identical global cohort sequences and per-cohort canonical values; at
  each completed request with no active request and equal `F`, their
  `stable_boundary_digest`s are equal. (Model S3, S3b.) Intermediate advance horizons are
  irrelevant by Theorem P′.
- **Theorem P′-pacing (equal-progress form).** Two runs over the same request sequence
  differing only in budget agree per cohort-sequence position on every canonical value,
  and their `stable_boundary_digest`s agree at every pair of boundaries with equal cohort
  count and equal `ActiveRequest`. (Model S2, S12, S19.)
- **Explicit non-claims.** Equal state at equal *call index* under different budgets;
  equal `PacingDiagnostics`; equality between a paused boundary and a completed boundary
  at the same cohort position (they differ in `ActiveRequest` and `F`, correctly); any
  histories with different command streams; any executable cross-platform parity.
- **Replay.** Canonical history is the finalized command stream (timeline) plus `F`.
  Mechanism replay: for each finalized command in ordinal order, `Advance(effective_time)`
  to completion then finalize and execute; finally `Advance(F)`. A paused request is not
  history; its partial progress is time-ordered work any later request with an
  equal-or-greater horizon consumes identically.

---

## 17. Consumer contract (the whole of what the host must do)

1. Offer requests to a bounded FIFO; on backpressure, wait or drop at the producer.
2. Present the head with `process(head)`; repeat while `PAUSED`.
3. On any terminal result, pop the head.
4. Never present a different request while the head is incomplete — **and if it does,
   the engine refuses it without mutation** (§5 Start). The contract is a liveness
   convenience, not the safety mechanism.
5. Keep the mailbox durable across restarts if paused requests must survive them; the
   engine's `ActiveRequest` will accept nothing else until the exact request completes.

Embedded and standalone forms differ only in what the mailbox is made of; the engine
sees the same request sequence and produces the same canonical results (ADR-0005).

---

## 18. Boundedness — what this surface may not become

1. Not a batch-scheduling API (v1 Q6; AT-B7 seam). 2. Not an arbitrary-removal API; no
cancellation path (ADR-0003 §11); no `ActiveRequest` abandonment path. 3. Not a second
read path for work content. 4. Not causally readable telemetry: X-4,
`PacingDiagnostics`, the horizon, `F`, and `ActiveRequest` are never given to
evaluation. 5. Not a replacement for `drain_due`. 6. Not a host surface for X-1 … X-4:
the host reaches only `process`, read-only `F` / `ActiveRequest`, and the frozen reports.
7. Not a horizon-driven evaluation context. 8. Not a transport, mailbox implementation,
or persistence coupling. Each carries a compile-probe obligation in the oracle.

---

## 19. Supersession map

### 19.1 Against the serialized-request addendum (`30626ab`)

| Addendum item | Disposition |
|---|---|
| §1 lifecycle rows Accept, Process, Next | **RETAINED** |
| §1 rows Start, Pause, Resume, Complete | **REVISED** (§5): `ActiveRequest` set/checked/cleared |
| §2 "inside the engine at any stable boundary: none"; "the engine's behavior is total … for any call sequence"; safe replayable history for a non-serializing consumer | **WITHDRAWN** — falsified by SB-01; replaced by §6 |
| §2 ownership: "serialization is the consumer's contract" | **NARROWED**: liveness is the consumer's; safety is the engine's (§17) |
| §2 replay and restart | **RETAINED**, restart now carries `ActiveRequest` (§6.5) |
| §3 `h ≥ F` start rule | **RETAINED** (§7), ordered after the mismatch check (§9 P0) |
| §4 `X` removed, `F` kept, `C` dropped | **RETAINED** |
| §5 two digests | **RETAINED**; `stable_boundary_digest` extended with `ActiveRequest` (§14) |
| §6 proposed v1 §8 amendment | **SUPERSEDED** by the Operator-authorized wording (§14) |
| §7 consumer contract | **REVISED** (§17): item 4 is no longer the only enforcement |
| §8 supersession of the command-time candidate | **RETAINED** in full |
| §9 model | **SUPERSEDED** by the 2026-09-10 model, which re-runs every prior check |
| §10 unresolved: `F` read-only to producers; mailbox capacity | **RETAINED** as residual (§22) |

### 19.2 Against the command-time candidate (`53a8432`) — via the addendum, all retained

ALT-U, `X`, `C`, A-1 … A-3: **WITHDRAWN** (unchanged disposition). §10 checklist V2-03 …
V2-10: **APPLIED** in §11–§13 and the oracle, with the `X` rows deleted, AT-I43 recast
as the request-lifecycle oracle, and AT-I29(d) recast as AT-G tests of `F` and
`ActiveRequest`. §11 items 1 (per-profile time) and 2 (stuck staged command): **CLOSED**
as the addendum stated; item 2 additionally by D-1 (no ordinal is issued before
finalization).

### 19.3 Against the V2 candidate (`983a01f`)

| V2 item | Disposition |
|---|---|
| §4.1–§4.3 time basis | **RETAINED** (§4) |
| §4.4 command placement "between calls, at history position"; §4.5 SH-1/SH-2, `Φ`, defense-in-depth enqueue check | **SUPERSEDED / WITHDRAWN** — commands execute at their request's completion (§8); no `Φ` |
| §5 X-1 … X-4, §5.4, §5.5, §5.6 | **RETAINED** (§10); visibility paragraph **CORRECTED** (V2-08) |
| §6 executable-only identity and pacing; all-conflicted no cohort | **RETAINED**; last paragraph ("pre-wave engine digest … differ") **WITHDRAWN** (V2-04, §12.3); mixed-slice order and post-exception behavior **SUPPLIED** (V2-10) |
| §7 loop A0–A6 | **SUPERSEDED** by §9 (request boundary; command at completion; conflicted consumption after exception) |
| §7 "extraction is one atomic step spanning two stores" (prose) | **SUPERSEDED** by the engine operation of §11 (V2-05) |
| §8 R-1 … R-7 | **RETAINED** with the wave-index rule (V2-06); R-8 **DELETED** |
| §9 observation | **RETAINED**, three obligations separated (§15) |
| §10 PE-A … PE-D, Lemma 1/2, P′, equal-prefix form, non-claims | **RETAINED**; PE-B **DELETED**; PE-C **NARROWED** to the state statement; P′-commands **REPLACED** by P′-requests (§16) |
| §11 boundedness | **RETAINED**, extended (§18) |
| §12.3 CE-7′ … CE-11′ | **RETAINED**, except CE-10′ **REPLACED** (§20) |

### 19.4 Against v1 / v2 / v3 and the adjudication

Every disposition in V2 §12.2 is carried; the V2 rows "v2 §7 preflight extended with the
`due_time ≤ Φ` check" and "Adjudication §11 U-1 resolved as a bounded guard" are
**SUPERSEDED**: the defense check is withdrawn with `Φ`, and U-1 is resolved by the start
rule plus `ActiveRequest`. Adjudication U-2 (frontier and R1) is **RESOLVED**: `F` is
committed in `stable_boundary_digest`, not in the engine digest; equal engine digests
with different `F` are different request-boundary states and are distinguished there.
v3 §9 `PHASE_2_ARCHITECTURE_FROZEN_V3` is **not reinstated and not claimed**.

---

## 20. Consistency edits (complete list; nothing outside it changed)

| # | Edit | Forced by |
|---|---|---|
| CE-7′ | Scheduled work is consumed through X-1 … X-3 via the engine-owned extraction of §11, one slice at a time, immediately before that cohort's pre-wave digest; the evaluator never calls `drain_due` | V3-F01; F-02, F-03; V2-05 |
| CE-8′ | Budget and `scheduled_cohort_identity` range over executable keys only; conflicted keys are extracted, reported, and are members of neither | F-04 |
| CE-9′ | Every rejection class consumes its keys terminally; obligation records stay removed; no rollback; later-wave rejection leaves earlier waves committed | V2-06 |
| CE-10″ | A command request is finalized (ordinal at the frontier) and executed atomically at its request's completion, after every scheduled slice `≤ effective_time`; finalization refusal completes the horizon without execution | foundation item 11; D-1, D-2 |
| CE-11′ | Pre-wave observation is `test-support`-gated; no canonical report field | F-06 |
| CE-12′ | `ActiveRequest` is engine-owned request-boundary state, set before any cohort mutation at start, exact-match resumed, cleared atomically with `F := h`, never otherwise changed | SB-01; Operator decision |
| CE-13′ | `stable_boundary_digest = H("stable_boundary_v1" ‖ engine_state_digest ‖ F ‖ canonicalize(ActiveRequest))` for snapshots, restore, and equal-state tests; `engine_state_digest` unchanged | Operator decision |
| CE-14′ | All-conflicted slices are consumed unconditionally, including after the oversized exception; diagnostics are executable-only; mixed-slice reports order conflicts before the cohort outcome | V2-10; D-3 |
| CE-15′ | X-1 … X-4 carry no privacy claim; the enforced boundary is the engine/host facade | V2-08 |

No pinned digest value moves: no Phase-2 implementation exists, no Phase-1 encoding,
tag, or digest input is touched, and `effect_batch_v3` is unchanged.

---

## 21. Model summary

`engineering/phase2/v3_f01_final_correction_model_2026-09-10/model.py` (standard library
only, ~640 lines) runs 34 checks, all passing (`results.txt`, `results.json`):

| Group | Checks | What they show |
|---|---|---|
| Carried from 2026-09-06 | S1, S2 (×2), S3, S3b, S3c, S3d, S4, S4b, S5, S6 | every prior claim still holds with `ActiveRequest` present |
| New negative control and pin | S3e, S3f | `ActiveRequest` in the per-cohort digest breaks catch-up equality; cohort@10 pre-wave equals an independently built reference holding exactly `{B@20}` |
| SB-01 | S7, S7b, S7c | `Command@12` after work through 15 refused byte-identically; `Advance(25)` likewise; exact head resumes |
| Restart | S8, S8b, S8c | exact request continues byte-identically; same-horizon different request refused; restore validation rejects a `Some` behind the frontier and a tampered `ActiveRequest` |
| Digests | S9, S9b | five-way discrimination with equal engine digests; equivalence of equal stable-boundary digests |
| Cleanup | S10, S10b | `None` after completion and after start refusal; equal-horizon advance idempotent |
| Pacing | S12, S19 | budgets 1/2/3 equal by position; stable-boundary equal at equal progress; deferred command identical |
| V2-05 | S13 | missing/mismatched record refuses before either store mutates |
| V2-06 | S14 | wave-0 versus later-wave rejection states |
| V2-03, V2-04 | S15, S15b | budget-2 discriminator; `{S}` versus `{S, X}` equal except the report |
| V2-10 | S16, S16b | zero-cost conflicted consumption after the exception; `PAUSED ⇒ deferred ≥ 1` |
| Finalization | S17, S17b | duplicate identity: `COMPLETED_COMMAND_NOT_FINALIZED`, `F` advanced, `ActiveRequest` cleared |
| Compare-and-take | S18 | non-least and stale fingerprints are typed no-ops |

It models no transport, no concurrency, no persistence backend, and no performance.

---

## 22. Decisions taken where the record was silent, and residual risks

**Decisions (each flagged for the reviewer):** D-1 ordinal assigned at finalization
(§8.1); D-2 `F` advances on finalization refusal (§8.3); D-3 all-conflicted slices
consumed after the oversized exception (§9); D-4 `Advance` identity is `H(T)` alone, so
equal-horizon advances are one request (§6.2); D-5 restore validation order and the two
typed restore failures (§6.5).

**Residual and unresolved:**

1. **No `ActiveRequest` abandonment path exists by design.** A host that loses a paused
   command's request (a non-durable mailbox) cannot present anything else until it
   reconstructs and completes that exact command; an `Advance` is reconstructible from the
   read-only discriminator, a command is not. If a deployment needs an abandonment path
   it is a separately authorized decision in the ADR-0003 §11 shape; this candidate
   states the fail-closed property and does not weaken it.
2. Whether Phase 3 exposes `F` / `ActiveRequest` read-only to producers, and mailbox
   capacity as a declared deployment value — carried from the addendum, non-normative.
3. Cohort-identity non-injectivity over slices (V2 §6) remains a stated consequence of
   executable-only identity.
4. No adversarial review of this pass exists; writer/reviewer separation is intact.
5. Supplementary compute was not used in this pass (writer report §7).

---

## 23. Verdict of this pass

`V3_F01_FINAL_CORRECTION_CANDIDATE` — submitted for independent review.

- V3-F01: **OPEN**. Phase-2 architecture: **NOT frozen, NOT accepted.**
- PHASE_2_IMPLEMENTATION_AUTHORIZATION: **NO**. PHASE_3_AUTHORIZATION: **NO**.
- Phase 1: **CLOSED and not reopened.** G.A.M.E.: **unchanged.**
