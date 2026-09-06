# S.P.A.R.K. Phase 2 — V3-F01 Bounded Architecture Correction Candidate **V2**

**Date:** 2026-09-06
**Agent:** Claude Code (Opus 5), Gate C1 architecture correction writer (second pass)
**Status:** **CANDIDATE — AWAITING INDEPENDENT REVIEW.** Not accepted, not frozen, not
canonical, and authorizing no implementation. V3-F01 remains **OPEN**.
**Form:** additive bounded correction to
`SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md`, taking
`SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md` as its
**proposed** foundational basis — itself a candidate under the same review, not an
accepted ruling. No historical artifact is rewritten, moved, or deleted.
**Supersedes (as a proposal):** the rejected first-pass candidate
`SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_2026-09-06.md`, which is
preserved **unchanged** as rejected review history and is not repaired in place.
**Companion:**
`SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_V2_2026-09-06.md`
**Controlling findings addressed:** F-01 … F-07 of
`SPARK_PHASE_2_CODEX_V3_F01_INDEPENDENT_REVIEW_2026-09-06.md`, together with the
original V3-F01 finding in
`SPARK_PHASE_2_CODEX_FINAL_ARCHITECTURE_V3_CONFIRMATION_2026-08-26.md` §7.

**Nonclaims, stated first because they bound everything below.** This candidate does not
claim that V3-F01 is closed, that the foundational adjudication is accepted, that
Phase-2 architecture is frozen or accepted, that the Phase-2 implementation writer is
released, that Phase 3 is authorized, that Operator acceptance has occurred, that any
cross-platform runtime parity exists, or that G.A.M.E. convergence state
`ARCHITECTURE_READY` has been reached. No production Rust and no test was written by
this pass.

---

## 1. Baseline verified before writing

| Item | Verified value |
|---|---|
| Repository | `/home/chromikey/Projects/SPARK` |
| Branch | `phase1-refoundation-v2` |
| HEAD at entry (this pass's parent) | `379f8dc355125e6bca09801e4a66b0b35ec9720c` |
| **Foundational adjudication commit** | **`379f8dc355125e6bca09801e4a66b0b35ec9720c`** — parent `513c398…`; the adjudication report omitted its own hash, so it is pinned here |
| Independent review commit | `513c3982d5b9cd14f86ec07369662f3a178f1d95` |
| Rejected first-pass candidate commit | `03daa82032cecc6ed84407b440bc9eab06cbcd69` |
| Convergence baseline | `5fd556bfad958bda4439560cbfc5f4e537ce375a` |
| Lineage | `5fd556b` → `03daa82` → `513c398` → `379f8dc` (HEAD at entry) |
| Worktree at entry | clean (`git status --porcelain` empty) |
| Phase 0 | CLOSED / PASS — not reopened |
| Phase 1 | CLOSED (`PHASE_1_CLOSED` at reviewed HEAD `20a1c66`) — not reopened |
| Phase 2 | architecture-only; controlling verdict `PHASE_2_ARCHITECTURE_V3_REVISE` |
| Phase 3 | NOT AUTHORIZED |
| Gate | C1 (`engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` §5) |

Every path cited in this document was read in this pass. The Phase-1 seams inspected
directly are `crates/spark-core/src/clock.rs`, `crates/spark-core/src/scheduler.rs`
(`WorkKey`, `WorkPayload`, `DueWorkItem`, `SlotState`, `WorkSlotStatus`,
`ConflictEvidence`, `WorkKeyConflict`, `DrainOutcome`, `Scheduler::schedule`,
`Scheduler::drain_due`, `Scheduler::slot_status`, `Scheduler::conflict_of`,
`Scheduler::canonical_state_digest`), `crates/spark-core/src/timeline.rs`
(`SemanticCommandEnvelope`), and the `test-support` feature declarations in
`crates/spark-core/Cargo.toml`, `crates/spark-engine/Cargo.toml`, and
`crates/spark-testkit/Cargo.toml`.

---

## 2. Disposition of the seven independent-review findings

| ID | Finding | Disposition in this candidate |
|---|---|---|
| **F-01** | Theorem P assumes a partition-invariant cohort sequence that Lemma V does not prove; `now + 1` semantics make cohort membership partition-dependent | **RESOLVED** by adopting the adjudicated time basis (§4) and rebuilding the equivalence claims with explicit preconditions and a prefix induction that *derives* rather than assumes created keys (§10). Lemma V is withdrawn. |
| **F-02** | `DueCohortSelection` is an owned, retainable, replayable, key-only removal capability; any due entry can mint one | **RESOLVED** by deleting the token entirely and replacing it with a compare-and-take whose target is fixed to the live least due slice (§5). The comparison value is explicitly **non-authorizing** and its retainability is conceded, not denied. |
| **F-03** | `take_due_cohort` has no `now`/clock, so `CohortNotDue` is not implementable | **RESOLVED** by passing an explicit eligibility **horizon** and returning `NoSliceDue` with the live least resident due time (§5.4). The horizon is separated from evaluation time (§4.1, §5.5). |
| **F-04** | D-3/D-4 silently converted conflicted slots into cohort identity and pacing-budget units | **RESOLVED** by separating the **operational extraction slice** from the **executable cohort** (§6). Inherited `DrainOutcome::due` pacing and executable-only cohort identity are restored. D-3 and D-4 are withdrawn. |
| **F-05** | The oracle misses its own successful counterexamples and carries a false traceability row | **RESOLVED** in the companion matrix V2, which encodes every §5 counterexample and every §7 gap as a named test, each stating the incorrect implementation it must reject. |
| **F-06** | CE-8 expands a canonical host-visible report with internal state digests | **RESOLVED** by withdrawing CE-8 and observing the pre-wave digests through the **existing sanctioned `test-support` seam** enforced by the workspace feature-hygiene test (§9). No canonical report field is added. |
| **F-07** | §6/§8 borrow wording is internally imprecise | **RESOLVED** by an exact borrow-lifetime statement (§5.5) that claims only what non-lexical lifetimes actually provide, and by dropping the "structurally non-retainable" overclaim. |

---

## 3. What this correction does not disturb

Carried forward unchanged and unweakened: the equal-`(profile_id, due_time)` cohort (v3
§3.1); ascending cohort-sequence processing; `max_due_per_cycle` as pacing, semantics-
neutral, deferring whole cohorts (v2 §6.2, v3 §3.2); the oversized-earliest progress
exception (v3 §3.2(c)); deferral leaving cohorts resident and untouched (v3 §3.2(d));
progress (v3 §3.3); `PacingDiagnostics` as noncanonical and causally inert with its
frozen field list (v3 §3.4); semantic caps and over-cap disposition (v2 §6.3, v3 §3.5);
cohort/command/parent/emission/batch identities (v3 §4.1–§4.5); parent-set transience
(v3 §4.6); CE-1 … CE-6 (v3 §7); the complete-transition preflight (v2 §7); the
`ObligationStore` claim-set model and multi-target binding (v2 §8); occurrence
allocation (v2 §9); provenance (v2 §10); retained-store commitment and the composed
engine digest (v1 §8, v2 §12); no batch-scheduling API (v1 Q6); authority, determinism,
primitive-set, portability, chronicle, scripting, runtime-LLM, and Phase-3 boundaries
(v1 §3). Every Phase-1 semantic, encoding, conflict rule, and digest is consumed
unmodified.

---

## 4. Foundational time basis adopted (proposed, from the adjudication)

This section restates the adjudicated rule in the form this correction consumes. It is
adopted as a **proposed** basis; if the reviewer rejects the adjudication, §5–§10 fall
with it and this candidate must be re-opened rather than patched.

### 4.1 Horizon versus canonical time (the distinction the defect turned on)

- The **horizon** `T` is the `LogicalTime` argument of a host catch-up call. It is an
  inclusive **eligibility bound** on which work may be consumed. It is never the
  canonical time of anything evaluated.
- The **canonical time** of a scheduled cohort is its own `due_time`; of a command
  cohort, the finalized `effective_time`. This is what every inherited occurrence of
  "current logical time", `now`, and `at` denotes during that cohort's evaluation.
- The `LogicalClock` frontier is host-facing admission state. No canonical evaluation
  reads it.

### 4.2 Expansion

One catch-up call `advance(T)` consumes, in ascending `(due_time, profile_id)` order,
every resident slice with `due_time ≤ T`, **including slices that become resident because
earlier cohorts in the same call created them**, subject to pacing. It therefore expands
deterministically into one canonical logical-time barrier per **resident** due time
reached, and none at due times where nothing is resident. Iteration is over resident due
times, never over integer ticks, so an idle span costs no barriers.

A *canonical logical-time barrier* is a grouping label with no identity of its own; every
canonical value is per cohort. This is why a barrier may span several host calls under
pacing with no canonical consequence.

### 4.3 Delayed work and eligibility

Every obligation created or converted by cohort `C` carries
`due_time ≥ C.canonical_time + 1`, computed with checked `u64` addition; overflow rejects
the wave atomically (§8, class R-3). Created work with due time `δ` is eligible exactly
when the cohort sequence reaches `δ`: inside the same call when `δ ≤ T` and the budget
reaches it, otherwise resident and byte-identical for a later call. Because every created
key is strictly later than its creator, the membership of barrier `τ` is fixed when the
sequence first reaches `τ`, and nothing created at `τ` can join `τ`. That is the corrected
reading of v2 §11's second bullet.

### 4.4 Command-barrier placement

A finalized command executes at its position in canonical input history, as its own
cohort, with canonical time `effective_time` and identity v3 §4.2. A command barrier
evaluates only its own cohort and **does not drain scheduled work**. Command barriers
never interleave inside a catch-up call's expansion: a call is atomic with respect to
history ordering even when it expands into many barriers. Work created by a command
cohort is never consumed by a command barrier.

Equal canonical times between a command and a scheduled cohort are ordered by **explicit
history position** (call before command, or command before call), which is what blueprint
§28 item 2 and the Phase-0 "ambiguous same-time order rejected" correction require. No
implicit comparison of `effective_time` against due times ever reorders history.

### 4.5 Supported-history conditions on command effective time (adjudication U-1, resolved as a bounded guard)

The adjudication left open whether the engine should constrain `effective_time`, and
warned the writer not to assume host monotonicity in either direction. This pass does not
add an ingress admission rule and does not decide the ADR-0003 staging question. It
states the conditions explicitly and makes their violation **detected and fail-closed**
rather than silently assumed.

Let `Φ` be the **canonical time frontier**: the canonical time of the most recently
processed cohort (initially the engine's start time). Define:

- **SH-1.** A finalized command executed with `effective_time = e` satisfies `e ≥ Φ`.
- **SH-2.** At that execution, no resident scheduler slot has `due_time < e`.

**Why the conditions are needed, precisely.** Neither condition is required by the
equivalence proofs — §10's segment induction survives without them. They are required for
**semantic soundness of evaluation-time-dependent operations**. If a command evaluates at
`e < Φ`, then v1 Q7's closed-form decay/recovery computes elapsed logical time between a
cell's `updated_at` and `now = e` with `updated_at > e`, extrapolating a value *backwards*
under the frozen `i128` floor rule; v1 §6 cooldown expiry is consulted against a time
earlier than entries already written. Both are defined arithmetic and neither panics, so
the failure is silent semantic corruption, which is exactly the class the determinism
constitution exists to prevent. SH-2 additionally prevents a scheduled cohort at
`due_time < e` from being evaluated *after* the command, which would produce the same
backwards-time condition from the other side.

**Where the conditions come from.** They are supplied by an existing contract, not
invented here: Gate C3 of `engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` §5 already
lists **"monotonic logical execution point"** as a frozen item of the product integration
contract. SH-1 and SH-2 are the engine-side statement of that item. ADR-0003 §1 makes the
timeline sequencer the single authority that orders upstream sources before they become
canonical inputs, so a conforming deployment has exactly one place to satisfy them.

**How violation is handled.** A command whose execution would violate SH-1 or SH-2 is
**refused explicitly**: a typed, deterministic, atomic refusal before any canonical
mutation, reported in the canonical semantic report, with the command's cohort producing
no wave, no batch, and no created work. This is the shape v1 §5 already fixes for a
finalized obligation whose target fingerprints have drifted — "execution refuses
explicitly (typed error, obligation reported, nothing reinterpreted)" — and the class v1
Q4 already defines for structural violations detected at runtime. The refusal is a
function of canonical state and the finalized command alone, so it replays identically.

**What is explicitly not decided.** Whether the timeline should *additionally* reject
such a command at staging under ADR-0003 §5's command-validation item is left open. It is
an ADR-0003 question, it would change timeline finality semantics, and no Phase-2
correction may settle it. Both a staging rejection and this execution-door refusal can
coexist; adopting one does not preclude the other.

**Consequence when SH-1/SH-2 hold.** Canonical time is non-decreasing across the entire
global cohort sequence, no work is ever created into already-traversed logical time, and
`now ≥ updated_at` holds at every decay/cooldown evaluation. A defense-in-depth preflight
check additionally rejects, atomically, any wave that would create work with
`due_time ≤ Φ`; under SH-1/SH-2 it is unreachable, which is what makes it a defense.

---

## 5. The corrected scheduler-consumption surface (replaces C-1 … C-3)

Four additive items on the inherited `Scheduler`. Names are indicative; behavior is what
is proposed. Nothing here changes `Scheduler::schedule`, `Scheduler::drain_due`,
`WorkKey`, `WorkPayload`, `DueWorkItem`, `SlotState`, `WorkSlotStatus`,
`ScheduleDisposition`, `DrainOutcome`, `WorkKeyConflict`,
`Scheduler::canonical_state_digest`, or any Phase-1 canonical encoding.

**Visibility.** ADR-0001 fixes the dependency direction `spark-engine → spark-core`, so
the evaluator cannot reach a `pub(crate)` scheduler item. X-1 … X-4 are therefore `pub`
on `spark-core`'s `Scheduler` and are **engine-internal by contract**: they are never
re-exported on a host, service, or protocol surface, and the Phase-3 boundary must not
expose them. This candidate states that honestly rather than claiming a crate-private
surface it cannot have.

### X-1 — `least_due_slice` (read-only, least-only)

```text
Scheduler::least_due_slice(&self, horizon: LogicalTime) -> Option<LeastDueSlice<'_>>
```

Returns a borrowed handle to the **single least** resident `(due_time, profile_id)` slice
with `due_time ≤ horizon`, or `None`. It exposes exactly `due_time()`, `profile_id()`,
`executable_count()` (occupied slots in `Scheduled` state), `conflicted_count()`, the
slice's `WorkKey`s in ascending order with each key's `WorkSlotStatus`, and
`fingerprint()`. It exposes **no payload and no conflict evidence**; work content is
obtained only by extraction, and only for the slice actually admitted.

There is deliberately **no enumeration of later cohorts**. The v3 §3.2 selector inspects
only the least remaining cohort — it has no lookahead and no packing — so a multi-cohort
view was never needed, and removing it removes the "any due entry can mint a removal
token" surface that F-02 identified. Calling X-1 leaves `canonical_state_digest()`
byte-identical, and two schedulers with equal canonical digests return equal handles for
equal horizons.

### X-2 — `SliceFingerprint` (an owned, explicitly non-authorizing comparison value)

```text
LeastDueSlice<'_>::fingerprint(&self) -> SliceFingerprint

SliceFingerprint = H("due_slice_fingerprint_v1"
                     ‖ due_time ‖ profile_id ‖ occupied_slot_count
                     ‖ for each occupied slot of the slice, ascending by WorkKey:
                         block( WorkKey::canonicalize
                                ‖ "slot.scheduled"  ‖ WorkPayload::canonicalize
                                | "slot.conflicted" ‖ ConflictEvidence::canonicalize ))
```

The per-slot block is **byte-for-byte the block `Scheduler::canonical_state_digest`
already builds** (`scheduler.rs:545–565`), so the fingerprint is exactly the restriction
of the frozen canonical scheduler digest to one slice. No new encoding is invented. It
therefore commits to key identity, slot **state**, scheduled payload content, and the
**complete tracked** conflict claim set with its truncation flag — not the
`MAX_CONFLICT_EVIDENCE` presentation projection.

**It carries no authority.** It names no target: X-3 always operates on the live least
due slice regardless of what the fingerprint says. It is freely retainable, copyable, and
replayable, and this candidate makes no contrary claim. That is safe precisely because
possessing one grants nothing that a fresh call to X-1 would not grant (§5.6).

### X-3 — `take_least_due_slice` (compare-and-take; the only removal path)

```text
Scheduler::take_least_due_slice(
    &mut self,
    horizon: LogicalTime,
    expected: SliceFingerprint,
) -> Result<SliceExtraction, TakeSliceError>
```

**Target selection is fixed by the operation, never by the argument.** The operation
computes the live least resident slice with `due_time ≤ horizon`, recomputes its
fingerprint, and proceeds only if that value equals `expected`. There is no removal by
`WorkKey`, key list, range, predicate, count, `due_time`, or coordinate; there is no way
to name a non-least slice; there is no cancellation path (ADR-0003 §11).

```text
SliceExtraction {
    due_time
    profile_id
    due:        Vec<DueWorkItem>      ascending WorkKey order   (the executable cohort)
    conflicted: Vec<WorkKeyConflict>  ascending WorkKey order
    executable_count = due.len()
    fingerprint                       (recomputed from the removed slice)
}
```

The `due`/`conflicted` partition, its ordering, and every `WorkKeyConflict` value are
**exactly** those `drain_due` produces for the same slots — the same per-slot disposition
logic, narrowed to one slice (§10.5).

**Scan before mutation.** The implementation locates the slice, computes its fingerprint,
and compares, all before removing anything. There is no partial removal and no rollback
path, matching v1 Q3's "validate all, then apply all" discipline and `drain_due`'s
documented no-fallible-re-lookup property. The operation has no panic path.

### X-4 — `due_slice_summary` (noncanonical pacing telemetry only)

```text
Scheduler::due_slice_summary(&self, horizon: LogicalTime) -> DueSliceSummary
DueSliceSummary { slice_count, earliest_due_time: Option<LogicalTime> }
```

Read-only, non-mutating, computed once at the end of a catch-up call, and consumed
**only** to populate `PacingDiagnostics::deferred_cohort_count` and
`earliest_deferred_due_time`. It is subject to v3 §3.4 rule 3 in full: the evaluator is
never given it, and no rule, effect, threshold, obligation, cooldown, or scheduler
decision may read it.

### 5.4 Typed failures, each an atomic byte-identical no-op

| Error | Condition | Constructible by |
|---|---|---|
| `NoSliceDue { least_resident_due_time: Option<LogicalTime> }` | no resident slice has `due_time ≤ horizon` | passing a horizon below the least resident due time, or an empty scheduler — this is the **meaningful** not-due failure F-03 found unconstructible |
| `SliceChanged { observed: SliceFingerprint }` | the live least due slice's recomputed fingerprint ≠ `expected` | any membership change, any payload change, any `Scheduled`→`Conflicted` transition, or an `expected` naming a non-least slice |

Both leave `canonical_state_digest()`, the occupied key set, every payload, and every
conflict value byte-identical. `least_resident_due_time` and `observed` are returned so
the caller can re-observe; both are non-authorizing values.

### 5.5 Exact borrow and lifetime statement (repairs F-07)

```text
let expected = {
    let slice = scheduler.least_due_slice(horizon)?;   // immutable borrow begins
    admit_or_stop(slice.executable_count(), remaining_budget)?;
    slice.fingerprint()                                // owned value produced
};                                                     // borrow ends here
let extraction = scheduler.take_least_due_slice(horizon, expected)?;  // &mut begins
```

The claim is exactly this and no more: the immutable borrow of `LeastDueSlice<'_>` ends
before the `&mut self` call, because the handle's last use is inside the block that
produces the owned `SliceFingerprint`. Rust's non-lexical lifetimes make this ordinary
straight-line code; the independent review already disproved by compile probe that any
inherent borrow-checker obstacle exists. This candidate **withdraws** the first pass's
claim that borrowing structurally prevents retention of a due-work plan. It does not:
`SliceFingerprint` is owned and retainable. Non-retention of a *plan* is instead
guaranteed by there being no plan — X-1 exposes one slice, the loop recomputes from
canonical state after every stable boundary (§7), and nothing derived from X-1 … X-4
carries authority or survives as state (§10.6).

### 5.6 Why the three §5 counterexamples of the independent review are closed

- **Replay after extraction and exact re-creation (review §5.2).** Conceded and made
  harmless rather than denied. Replaying a stale fingerprint has exactly three outcomes.
  If the re-created slice is byte-identical *and* is currently the least due slice, the
  take succeeds — and removes precisely what a fresh `least_due_slice` call would have
  authorized in that same state, so no privilege is gained. If the re-created payload,
  membership, or slot state differs, the fingerprint differs and the call is a typed
  no-op. If some other slice is now least, the fingerprint does not match it and the call
  is a typed no-op. Authority lives in the operation's fixed target, not in the value.
- **Selecting a non-least due cohort (review §5.2 second half).** Structurally impossible:
  the argument cannot name a target. A fingerprint of a non-least slice yields
  `SliceChanged` with the live least slice's fingerprint returned.
- **Same-key `Scheduled`→`Conflicted` mutation between observation and extraction
  (review §5.3).** Closed by binding the comparison to slot state and content, not key
  identity alone. `schedule(K, payload B)` on a `Scheduled(A)` key changes that slot's
  block from `"slot.scheduled" ‖ h(A)` to `"slot.conflicted" ‖ {h(A), h(B)}`, so the
  fingerprint changes and the take is a typed no-op.

---

## 6. Operational extraction slice versus executable cohort (resolves F-04)

The first pass folded conflicted slots into cohort identity (D-3) and the pacing budget
(D-4). Both are withdrawn. The correct separation, which the independent review
prescribed and which restores inherited semantics:

| Concept | Membership | Consequences |
|---|---|---|
| **Operational extraction slice** | the whole `(due_time, profile_id)` slice: `Scheduled` **and** `Conflicted` slots | removed atomically in one step so no conflicted slot lingers to pollute a later cohort's pre-wave digest; each conflicted key reported in Phase-1 `DrainOutcome::conflicted` shape |
| **Executable cohort** | the `Scheduled` members only | supplies `scheduled_cohort_identity` (v3 §4.1), the wave input, the pacing count, and the batch digest |

**Cohort identity.** v3 §4.1's formula is unchanged; its input set is settled as the
executable keys only, so `work_key_count` is `executable_count` and the ascending
identity digests are those of the `Scheduled` keys.

**Pacing.** `max_due_per_cycle` counts **executable** keys, restoring v1 Q4's budget over
`DrainOutcome::due` exactly as the independent review requires. A conflicted slot consumes
no budget. This removes the review's §5.5 admission-boundary change: with
`max_due_per_cycle = 1`, a slice `(100,P)` holding one scheduled key `S` and one
conflicted key `X` has executable count 1, admits normally, does not trigger the
oversized exception, and does not defer `(101,P)`'s key `T`.

**Mixed slice.** Extracted whole; the executable members form the cohort and evaluate;
the conflicted members are reported and evaluate nothing.

**All-conflicted slice.** Extracted whole; reported in conflicted-drain shape; **no
cohort identity, no wave, no batch digest, no pre-wave capture**, and no budget consumed.
It is an operational consumption step, not an evaluation cohort. The first pass's D-5
disposition is retained in substance but corrected in two respects: it emits no cohort
identity, and it consumes no budget.

**Progress is preserved exactly (v3 §3.3).** Every loop iteration removes at least one
occupied slot. An all-conflicted slice has executable count `0 ≤ r` for every remaining
budget `r ≥ 0`, so it never stops the admission loop and never starves a later cohort;
and because it consumes no budget, the oversized-earliest exception's premise ("the full
budget was available") is unaffected when it fires on a following slice. Any cycle with a
resident executable slice therefore still admits at least one executable cohort.

**Accepted and stated consequence: cohort identity is not injective over slices.** Two
slices with identical `Scheduled` members but different `Conflicted` members produce the
**same** `scheduled_cohort_identity` and the same emission identities. This is correct:
the causes are genuinely identical, and a conflicted key produces no emission. The
difference is fully observable elsewhere — the conflicted keys change the scheduler
digest, hence the pre-wave engine digest, hence every `effect_batch_digest` of that
cohort, and they appear in the canonical conflict report. This is the exact inverse of the
rejected candidate's matrix assertion AT-I40(d), which required the identity to change;
that assertion is superseded and its inverse is now an oracle obligation.

---

## 7. The canonical processing loop

For one catch-up call `advance(T)` on engine state `S`, with epoch-bound
`B = max_due_per_cycle ≥ 1`:

```text
A0  if T < clock frontier: typed rejection, no canonical mutation; the call is not history.
A1  frontier := T;  r := B;  admitted_executable := 0;  exception_fired := false
A2  if exception_fired: goto A6                       (v3 §3.2(c): admit no other cohort)
    slice := Scheduler::least_due_slice(T)
    if none: goto A6
    e := slice.executable_count();  expected := slice.fingerprint()   [borrow ends]
A3  admit iff  e <= r                                  (v3 §3.2(b))
           or  admitted_executable == 0                (v3 §3.2(c); then exception_fired := true)
    otherwise goto A6                                  (v3 §3.2(d): defer, untouched)
A4  extraction := Scheduler::take_least_due_slice(T, expected)        [ATOMIC]
    remove the ObligationStore records of every extracted key         [same atomic step]
    report extraction.conflicted in Phase-1 conflicted-drain shape
    if extraction.executable_count > 0:
        pre-wave engine digest is now well defined                    [§9 observation]
        cohort_identity := v3 §4.1 over the executable keys
        evaluate waves 0..n with now = extraction.due_time;
        commit atomically, or take one of the rejection routes of §8
A5  STABLE BOUNDARY.
    if extraction.executable_count > 0: admitted_executable += 1;  r := exception_fired ? 0 : r - e
    drop the extraction; goto A2
A6  compute Scheduler::due_slice_summary(T); emit PacingDiagnostics; return.
```

**S2-before-S3 is preserved and is still the correction.** The slice leaves canonical
scheduler state, and its obligation records leave the `ObligationStore`, immediately
before the pre-wave engine digest is well defined, and every later or deferred slice is
resident and byte-identical at that instant.

**The extraction is one atomic Phase-2 step spanning two stores.** The scheduler removal
(X-3) and the `ObligationStore` record removal are not separately observable: no canonical
digest is captured, no evaluation runs, and no report is emitted between them. This is
what keeps v1 §8's scheduler↔`ObligationStore` bidirectional invariant true at every
observable point, including after a rejected wave (§8).

**Termination.** Each iteration removes at least one occupied slot; created work is
strictly later than its creator and bounded by `max_enqueue_per_wave`; due times consumed
are bounded by `T`.

**Command barriers** are not part of this loop. They occur between calls, at their history
position, under §4.4 and §4.5.

---

## 8. Rejection classes: terminal consumption, post-extraction state, store cleanup, retry

There is **no rollback mechanism and none is added.** v1 Q3's "atomicity without
rollback" is retained: a rejected wave validated before mutating, so it mutated nothing.
The only mutation that has occurred when a wave rejects is the §7 A4 extraction, and it is
**not undone**. "Restore to the post-extraction snapshot" therefore means "nothing was
applied", not "state was reverted" — the phrase the first pass left ambiguous (CE-9).

For every class below, uniformly: the slice's `WorkKey`s are **terminally consumed**; their
`ObligationStore` records were removed in the same atomic step as the scheduler removal
and are **not** reinstated; no cell, ledger, occurrence, cooldown, or scheduler enqueue of
the rejected wave is applied; the cohort's canonical semantic report carries the typed
outcome with order-independent evidence; progress is preserved because nothing returns to
the head of the queue.

| Class | Trigger | Canonical outcome | Retry |
|---|---|---|---|
| **R-1** Contested emission identity; unequal same-family RESULT; cross-family mixture; multiple TRANSFORMs without a declared reducer | v2 §5.3 | atomic wave rejection, typed order-independent evidence | producer may reschedule; normally under the next occurrence index, producing a **new** `WorkKey` and a new obligation record |
| **R-2** Authority, type, bounds, or scope-invalid effect | v1 Q3, v1 Q4 third bullet | atomic wave rejection; ill-formed-artifact signal | same |
| **R-3** Arithmetic failure: occurrence `u64` exhaustion, delayed-time `due_time` overflow, canonical arithmetic failure | v2 §7, §4.3 | atomic wave rejection | same |
| **R-4** Preflight failure of the complete candidate transition, incl. queue admission caps and the bidirectional invariant on the post-state | v2 §7 | atomic wave rejection before the first canonical mutation of the wave | same |
| **R-5** Semantic-cap overflow (`max_cohort_candidates`, `max_effects_per_wave`, `max_wave_depth` handling per v1 Q4, `max_enqueue_per_wave`) | v2 §6.3, v3 §3.5 | atomic rejection **plus** the typed overload report in conflicted-drain shape; the keys were **already** consumed at extraction, so v2 §6.3's "then consumed" is satisfied by the extraction and nothing is reinserted | same; identical on replay because caps are epoch-bound |
| **R-6** Conflicted slots | Phase-1 `schedule` poisoning | not a wave at all: reported in `DrainOutcome::conflicted` shape; contested `ObligationStore` claim sets for those keys removed as complete sets (v2 §8) | key is free; producer that resolved the ambiguity reschedules, normally under the next occurrence index |
| **R-7** Obligation execution refusal: `MaterializedEffect` target-fingerprint drift on any target, or `RuleReEvaluation` rule/ruleset resolution failure | v1 §5, v2 §8 | explicit typed refusal, nothing reinterpreted, no wave committed | same |
| **R-8** Supported-history violation SH-1 or SH-2 at a command barrier | §4.5 | typed atomic refusal of that command cohort; no wave, no batch, no created work; reported | host must satisfy the monotonic-logical-execution-point contract; the same command re-finalized in a conforming history executes normally |

**Same-key retry is permitted and is new canonical input.** Phase-1 frees a drained key,
so a producer may re-create the identical `WorkKey`. The record's guidance that reschedule
happens "typically under the next occurrence index" is guidance, not a restriction, and
this candidate does not turn it into one. Either retry produces a fresh obligation record
and is evaluated under the ordinary rules; neither is a resurrection of the rejected wave.

---

## 9. Observation without a canonical report surface (resolves F-06)

CE-8 is **withdrawn**. No canonical semantic report field is added, no report schema
changes, and the host gains no new observable it could feed back as canonical input.

The oracle instead observes the pre-wave engine digest and the pre-wave scheduler digest
through the **existing sanctioned test seam**: the `test-support` cargo feature already
declared on `spark-core` and `spark-engine`, dev-enabled only by `spark-testkit`, with
`crates/spark-testkit/tests/workspace_dependency_direction.rs` mechanically asserting that
no production dependency edge enables it. The observation points are exposed under
`#[cfg(any(test, feature = "test-support"))]`, exactly as the restricted nonzero-frontier
timeline constructor and the `spark_engine::fixture` write seams already are.

Two consequences worth stating. The seam is compile-gated out of every production build,
so it cannot be read by rule evaluation or by a host. And the hygiene test that protects
it already exists and already passes, so this correction adds an observation capability
without adding an enforcement obligation.

---

## 10. Partition equivalence: preconditions, claims, and explicit non-claims

Fix an activated artifact, one behavior epoch, and one declared cap set. A **catch-up
segment** is a maximal run of consecutive catch-up calls between finalized commands.

### 10.1 Preconditions, stated before any claim

- **PE-A (same history skeleton).** Compared runs have the same initial state, the same
  finalized command subsequence in the same order, and the same per-segment maximal
  horizon.
- **PE-B (supported history).** SH-1 and SH-2 hold at every command (§4.5). Runs
  containing a refused command are compared including that refusal.
- **PE-C (segment completion).** For **completed-horizon** equivalence only: each segment
  ends with no resident slice whose `due_time ≤` that segment's maximal horizon.
- **PE-D (same declared behavior).** Same epoch, same semantic caps, same rule set.
  Different budgets are permitted and are the subject of the pacing claim; different caps
  or epochs are different declared behavior (v2 C-11) and are never compared.

### 10.2 The cohort sequence as a function of history

Let `Run(S,T)` be the unbudgeted loop of §7 and `Run_B` the budgeted loop.

**Lemma 1 (least-slice invariance).** Within one call, the consumed slices are exactly the
successive live least resident slices with `due_time ≤ T`, and their due times are
non-decreasing. Created work is strictly later than its creator, so it can never sort
before the slice being processed; deferred and untouched slices keep their keys.

**Lemma 2 (prefix).** For `T1 ≤ T2`, the slices of `Run(S,T2)` with `due ≤ T1` form a
prefix `c_1 … c_j`, and `Run(S,T1)` processes exactly that prefix with the same state
after each. *Induction:* states before `c_i` agree; `c_i` is evaluated with
`now = c_i.due_time`, a property of the cohort and not of the horizon, so its evaluation,
its commit or rejection, and **every created `WorkKey`** agree; hence the resident sets
after `c_i` agree, and the next live least slice with `due ≤ T1` is the same one. Equality
of created keys is *derived here*, which is the induction premise F-01 found missing.

**Theorem P′ (catch-up composition).** For `T1 ≤ T2`,
`Run(Run(S,T1),T2) = Run(S,T2)` cohort by cohort, including each pre-wave engine digest,
cohort identity, wave index, candidate-set digest, committed effect block with its `at`
time, obligation record, occurrence allocation, cooldown write, canonical semantic report,
and the final engine digest.

**Corollary (R-038).** Iterating P′ over any non-decreasing horizon sequence ending at `T`
equals one call at `T`. "Ten days once ≡ one day ten times" now holds for **work-producing**
cohorts, not only pre-scheduled ones.

**Theorem P′-commands.** Under PE-A, PE-B, and PE-C, segment-by-segment composition gives
identical global cohort sequences and identical canonical values, because a command cohort
is a function of the finalized command, the state at its position, and
`now = effective_time`, all of which agree.

### 10.3 Pacing: equal-prefix, not equal-intermediate

**Theorem P′-pacing (equal-prefix form).** For two runs over the same history differing
only in `max_due_per_cycle`, index the global cohort sequence by **position**, not by call
number. Let `k` be the number of cohorts both runs have processed. Then for every
`i ≤ k`, cohort `i` is the same cohort with the same members, the same pre-wave engine
digest, the same identity, the same batch digests, and the same committed result; and the
engine states after cohort `i` are byte-identical.

**Completed-horizon form.** If both runs additionally satisfy PE-C for the segment, then
`k` is the whole segment and their engine states and canonical report sequences at segment
end are byte-identical.

**Explicit non-claim, required by the independent review's concern and stated here
because the first pass blurred it.** No claim is made that two runs are in equal states
after equal *numbers of host calls*. Different call counts supply different total pacing
budget, so at equal call index the runs have generally processed **different prefix
lengths** of the same sequence, and their engine digests differ. That difference is
correct, is pure latency, and is exactly what `PacingDiagnostics` reports. Equality is
claimed only per cohort-sequence position and, under PE-C, at segment end.

### 10.4 What is never claimed

Different command placement relative to catch-up calls (that is different canonical input
history, v2 §6.2); different epochs, caps, or rule sets; `PacingDiagnostics` values;
equal state at equal call index (§10.3); anything about histories violating SH-1/SH-2
other than that their refusals are deterministic; and any executable cross-platform
parity.

### 10.5 Phase-1 refinement (drain equivalence)

For a scheduler receiving no mutation, running §7 to exhaustion at horizon `T` against a
null evaluator yields the ordered concatenation of `due` equal to `drain_due(T).due` in
order, likewise `conflicted`, value-identical `WorkKeyConflict`s including retained
evidence, omitted count, and truncation flag, and an identical final
`canonical_state_digest()`. The independent review confirmed this lemma in its stated
scope; it is retained as a conformance obligation and is **not** used to prove anything
about work-producing runs.

### 10.6 No retained state is added

X-1 borrows; X-2 is an owned non-authorizing digest; X-3 returns the cohort's evaluation
input exactly as `DrainOutcome` does today; X-4 is a read-only count consumed by
noncanonical telemetry. At every stable boundary the only state is canonical: resident
slots, the stores, and the clock frontier. The loop recomputes from canonical state at
every A2. No plan, tail buffer, remaining-budget carry across calls, or continuation
cursor exists.

---

## 11. Boundedness — what this surface may not become

Each carries a compile-probe obligation in the companion matrix.

1. **Not a batch-scheduling API.** No insertion, multi-item `schedule`, reschedule, or
   reinsert. v1 Q6 and the AT-B7 seam contract at `scheduler.rs:424–431` stand.
2. **Not an arbitrary-removal API.** No removal by key, key list, range, predicate, count,
   coordinate, or `due_time`; the target is always the live least due slice; no
   cancellation path exists (ADR-0003 §11).
3. **Not a second read path for work content.** X-1 exposes no payload and no conflict
   evidence.
4. **Not causally readable telemetry.** X-4, `PacingDiagnostics`, and the horizon are
   never given to rule evaluation.
5. **Not a replacement for `drain_due`.** `drain_due` and `schedule` keep their exact
   Phase-1 signatures, semantics, and tests; the Phase-2 evaluator uses X-1 … X-3 and
   never calls `drain_due`.
6. **Not a host surface.** X-1 … X-4 are engine-internal by contract and are never
   re-exported on a host, service, or protocol surface.
7. **Not a horizon-driven evaluation context.** The evaluation context is constructible
   only from an extraction's `due_time` or a command's `effective_time`; the horizon is
   not reachable from it.

---

## 12. Supersession map

### 12.1 Against the rejected first-pass candidate (preserved unchanged as history)

| First-pass item | Disposition |
|---|---|
| C-1 `DueCohortView` (multi-cohort borrowed view) | **SUPERSEDED** by X-1, narrowed to the least slice only |
| C-2 `DueCohortSelection` (owned unforgeable token) | **WITHDRAWN.** Replaced by X-2, a non-authorizing comparison value |
| C-3 `take_due_cohort(selection)` | **SUPERSEDED** by X-3 compare-and-take with an explicit horizon |
| `CohortAbsent` / `CohortMembershipChanged` / `CohortNotDue` | **SUPERSEDED** by `NoSliceDue` / `SliceChanged`, both constructible and meaningful |
| §6.1 S0–S5 loop | **SUPERSEDED** by §7 A0–A6 (explicit horizon; `now = due_time`; conflicted slices handled) |
| §6.3 Lemma V | **WITHDRAWN — inconsistent** (it read `now` as the host call argument) |
| §7.1 Lemma A, §7.2 Theorem P | **SUPERSEDED** by §10.2 Lemma 1, Lemma 2, Theorem P′ |
| §7.4 Lemma D | **RETAINED** in its stated static/null-evaluator scope (§10.5) |
| D-1 (remove immediately before the pre-wave digest) | **RETAINED** |
| D-2 (recompute the view every cycle) | **RETAINED**, and now *required*: created work inside the horizon must be admitted at its barrier |
| D-3 (conflicted slots in cohort identity) | **WITHDRAWN** (§6) |
| D-4 (conflicted slots in the pacing budget) | **WITHDRAWN** (§6) |
| D-5 (all-conflicted cohort emits no batch) | **RETAINED in substance, CORRECTED**: also emits no cohort identity and consumes no budget |
| D-6 / CE-8 (pre-wave digests as canonical report fields) | **WITHDRAWN** (§9) |
| D-7 (names indicative, behavior binding) | **RETAINED** |
| CE-7 (consume through the additive surface; evaluator does not call `drain_due`) | **RETAINED**, restated over X-1 … X-3 |
| CE-9 (post-extraction state after rejection) | **RETAINED and completed** across all eight classes (§8) |
| §8 borrow/transience wording | **SUPERSEDED** by §5.5 and §10.6 |

### 12.2 Against v1 / v2 / v3 and the adjudication

| Source statement | Disposition |
|---|---|
| v1 Q3 effect `at` logical time | **CLARIFIED** — the cohort's canonical time |
| v1 Q3 heartbeat barrier identity | **RETAINED AS RETIRED** (v3 §4.5) |
| v1 Q3 "atomicity without rollback" | **RETAINED** and made explicit for extraction (§8) |
| v1 Q4 budget over `DrainOutcome::due` | **RETAINED** — restored against the first pass's D-4 |
| v1 Q4 structural-violation atomic rejection | **RETAINED**; reused for §4.5 and R-2 |
| v1 Q6 no batch-scheduling API | **RETAINED** |
| v1 Q7 decay `now`; AT-I22 chunk invariance | **CLARIFIED** — `now` is the cohort's canonical time; SH-1/SH-2 keep `now ≥ updated_at` |
| v1 §5 obligation refusal semantics | **RETAINED**; its shape reused for R-7 and R-8 |
| v1 §6 cooldown expiry on consult | **CLARIFIED** — consulted at the cohort's canonical time |
| v1 §8 engine digest and the scheduler↔`ObligationStore` bidirectional invariant | **RETAINED**; the invariant is preserved at every observable point by the single atomic extraction step (§7) |
| v2 §5.1 wave pipeline, §5.3 rejection classes | **RETAINED**; enumerated per class in §8 |
| v2 §6.1 cohort, §6.2 pacing, §6.3 over-cap | **RETAINED**; §6.3's "then consumed" satisfied by the extraction (R-5) |
| v2 §7 preflight | **RETAINED**; extended with the defense-in-depth `due_time ≤ Φ` check (§4.5) |
| v2 §8 `ObligationStore` claim sets and multi-target binding | **RETAINED**; record removal bound into the atomic extraction |
| v2 §11 first bullet (`≥ current logical time + 1`) | **CLARIFIED** — the creating cohort's canonical time |
| v2 §11 second bullet (one drain per canonical boundary) | **RETAINED** under the logical-time-barrier reading; **SUPERSEDED** under the host-call reading |
| v2 §12 transient-buffer row | **RETAINED**; extended to X-1 … X-4 |
| v3 §3.1 cohort; §3.2(a)(b)(c)(d); §3.3 progress; §3.4 diagnostics; §3.5 caps | **RETAINED**; §3.2(b)'s "`WorkKey` count" **CLARIFIED** as the executable count (§6) |
| v3 §4.1 cohort identity | **Formula RETAINED**; input set settled as executable-only (§6) |
| v3 §4.2–§4.4, §4.6, §4.7, §5, §7 CE-1 … CE-6 | **RETAINED** |
| v3 §4.5 partition-independence claim | **RETAINED**; its derivation **SUPPLIED** with explicit preconditions (§10) |
| v3 §9 verdict `PHASE_2_ARCHITECTURE_FROZEN_V3` | **NOT reinstated and NOT claimed** |
| Adjudication §4 rule A4, §5 loop, §6 commands, §7 eligibility, §8 proofs, §9 map | **ADOPTED as the proposed basis** |
| Adjudication §8 Theorem P′-pacing | **REFINED** into the equal-prefix and completed-horizon forms with an explicit non-claim (§10.3) |
| Adjudication §11 U-1 (`effective_time` monotonicity) | **RESOLVED as a bounded guard** (§4.5): supported-history conditions with fail-closed refusal; the ADR-0003 staging question left open |
| Adjudication §11 U-2 (clock frontier outside the engine digest) | **CARRIED FORWARD** unresolved (§14) |
| Adjudication §10 items 1–12 (downstream constraints) | **SATISFIED** — item-by-item mapping in the writer report |

### 12.3 New consistency edits (replacing the first pass's CE-7 … CE-9)

| # | Edit | Forced by |
|---|---|---|
| **CE-7′** | Scheduled work is consumed through the additive least-due compare-and-take surface X-1 … X-3, one slice at a time, extracted together with its `ObligationStore` records in one atomic step immediately before that cohort's pre-wave digest. The Phase-2 evaluator does not call `drain_due`. | V3-F01 §7.4(1)(2); F-02, F-03 |
| **CE-8′** | The pacing budget and `scheduled_cohort_identity` range over **executable** (`Scheduled`) keys only; conflicted keys are extracted with the operational slice, reported in Phase-1 conflicted shape, and are members of neither. | F-04 |
| **CE-9′** | For every rejection class, the extracted keys are terminally consumed, their obligation records stay removed, nothing is reinstated, and no rollback mechanism exists. | F-05, F-07 item 7; review Q9 |
| **CE-10′** | A finalized command executes at its history position with `now = effective_time`, drains no scheduled work, and is refused atomically if SH-1 or SH-2 fails. | Adjudication §6; F-01; mission item 3 |
| **CE-11′** | Pre-wave digest observation is a `test-support`-gated seam; no canonical report field is added. | F-06 |

No pinned digest value moves: no Phase-2 implementation exists and no Phase-1 encoding,
tag, or digest input is touched. `effect_batch_v3`'s tag and component list are unchanged,
and `SliceFingerprint` reuses the frozen per-slot block encoding rather than defining a
new one.

---

## 13. Delta summary

| Item | Disposition |
|---|---|
| Time basis | horizon expansion; `now` = cohort canonical time; eligibility at the created barrier |
| Command barriers | own cohort at history position; no scheduled drain; SH-1/SH-2 guarded |
| Consumption surface | least-due compare-and-take with explicit horizon; no selection token |
| Comparison value | non-authorizing; binds key identity, slot state, payload, full conflict claim set |
| Failure modes | two typed errors, both constructible, both atomic byte-identical no-ops |
| Conflicted slots | in the operational slice; not in identity; not in the budget |
| Cohort identity | v3 §4.1 formula, executable keys only |
| Removal timing | atomic, with obligation records, immediately before the pre-wave digest |
| Rejection classes | eight, each with terminal consumption, store cleanup, and retry stated |
| Observation | `test-support` seam; no canonical report field |
| Equivalence | preconditions PE-A…PE-D; completed-horizon and equal-prefix forms distinguished; equal-call-index equality explicitly disclaimed |
| Retained state | none added |
| Phase-1 | unmodified; refinement lemma retained in its static scope |

---

## 14. Open risks and residual items for the independent reviewer

1. **The foundational basis is itself a candidate.** §4 adopts the adjudication as a
   proposal. If the reviewer rejects horizon expansion or cohort-local evaluation time,
   this candidate does not survive as a patch and must be re-opened.
2. **SH-1/SH-2 enforcement point.** §4.5 places the guard at the command-execution door
   and leaves the ADR-0003 staging question open. A reviewer may prefer ingress rejection,
   both, or a different `Φ` definition. The consequence analysis (backwards decay and
   cooldown evaluation) is offered as the evidence, not as authority.
3. **Adjudication U-2 remains open.** Two engines with equal engine digests but different
   clock frontiers respond differently to a backward `advance`. Pre-existing Phase-1/v1 §8
   classification; not introduced or resolved here. Whether the frontier belongs in the
   engine digest is flagged, not decided.
4. **Cohort-identity non-injectivity** (§6) is a deliberate, stated consequence of
   restoring executable-only identity. A reviewer who wants slice-injective identity must
   say so explicitly; it would reopen v3 §4.1.
5. **`Φ` must be reconstructible.** §4.5 uses a canonical time frontier. If it is not
   derivable from committed state, it is retained state and must enter the v1 §8 digest
   under R1. This candidate's position is that it is derivable — it is the canonical time
   of the last processed cohort, which snapshots already capture as a complete stable
   boundary (ADR-0003 §15) — but the reviewer should test that claim directly; the matrix
   includes the obligation.
6. **Supplementary compute was attempted and failed.** One bounded invocation of the
   sanctioned MCI development gateway timed out before any request was submitted; no
   partial response exists beyond that record. This candidate rests on the frozen record,
   the independent review, and the adjudication.
7. **No adversarial review of this pass exists.** Writer/reviewer separation is intact and
   is the reason this document claims only "candidate".

---

## 15. Verdict of this pass

`V3_F01_CORRECTION_CANDIDATE_V2` — submitted for independent review.

- V3-F01 status: **OPEN**.
- Phase-2 architecture: **NOT frozen**, **NOT accepted**.
- PHASE_2_IMPLEMENTATION_AUTHORIZATION: **NO**
- PHASE_3_AUTHORIZATION: **NO**
- Phase 1: **CLOSED and not reopened.**
