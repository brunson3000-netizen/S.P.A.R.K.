# S.P.A.R.K. Phase 2 — V3-F01 Bounded Architecture Correction Candidate

**Date:** 2026-09-06
**Agent:** Claude Code (Opus), Gate C1 architecture writer
**Status:** **CANDIDATE — AWAITING INDEPENDENT REVIEW.** This document is a proposed
bounded correction. It is **not** accepted, **not** frozen, **not** canonical, and
authorizes **no** implementation. V3-F01 remains **open** until an independent Codex
reviewer accepts a correction and the operator records the freeze.
**Form:** tightly bounded additive correction to
`SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md`, with the explicit
supersession map in §11. No historical artifact is rewritten or deleted.
**Companion candidate:**
`SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_2026-09-06.md`
**Controlling finding addressed:**
`SPARK_PHASE_2_CODEX_FINAL_ARCHITECTURE_V3_CONFIRMATION_2026-08-26.md` §7 (V3-F01),
verdict `PHASE_2_ARCHITECTURE_V3_REVISE`.

**Scope of this pass (bounded):** exactly one defect — **V3-F01**, the inconsistency
between v3's cohort-granular pre-wave digest requirement and the inherited whole-prefix
`Scheduler::drain_due(now)` — plus the minimum consistency edits that correction forces.
Nothing else. No Phase-1 contract is reopened. No Phase-2 implementation is begun. No
production Rust was written by this pass.

**Nonclaims (restated at the front because they bound everything below):** this
correction does not claim that V3-F01 is closed, that Phase-2 architecture is frozen or
accepted, that the Phase-2 implementation writer is released, that Phase 3 is
authorized, that any cross-platform runtime parity exists, or that G.A.M.E. convergence
state `ARCHITECTURE_READY` has been reached.

---

## 1. Baseline verified before writing

| Item | Verified value |
|---|---|
| Repository | `/home/chromikey/Projects/SPARK` |
| Branch | `phase1-refoundation-v2` |
| Starting HEAD | `5fd556bfad958bda4439560cbfc5f4e537ce375a` |
| Worktree at entry | clean (`git status --porcelain` empty) |
| Phase 0 | CLOSED / PASS — not reopened |
| Phase 1 | CLOSED (`PHASE_1_CLOSED` at reviewed HEAD `20a1c66`) — not reopened |
| Phase 2 | architecture-only; `PHASE_2_ARCHITECTURE_V3_REVISE`; implementation not authorized |
| Phase 3 | NOT AUTHORIZED |
| Gate | C1 (`engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` §5) |

No material contradiction and no unexpected tracked change was found at entry. The
governing documents read, and the Phase-1 source seams inspected, are enumerated in the
architecture-writer report
(`SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_2026-09-06.md` §2).

---

## 2. The defect, reconciled against the controlling finding

### 2.1 What v3 requires

Three v3-frozen statements are jointly unsatisfiable against the inherited scheduler:

1. **v3 §3.2(d)** — every cohort after the admitted set is left "**scheduled and
   untouched** — already-canonical state, digest-checkable, not copied into any buffer".
2. **v3 §4.5** — `effect_batch_digest = H("effect_batch_v3" ‖ profile ‖ behavior_epoch ‖
   cohort identity ‖ **pre-wave engine digest** ‖ wave_index ‖ candidate-set digest ‖
   committed effect count ‖ each committed effect block)`, and the pre-wave engine digest
   of cohort `C` "is the committed state after every cohort strictly earlier in the
   cohort sequence".
3. **v1 §8** — `engine_state_digest = H("engine_state" ‖ state-store digest ‖
   **scheduler digest** ‖ timeline digest ‖ epoch-registry digest ‖ obligation-store
   digest ‖ occurrence-ledger digest ‖ cooldown-ledger digest)`.

Together: the scheduler's residency at the instant cohort `C`'s pre-wave digest is
captured is a **causal input to `C`'s batch digest**.

### 2.2 What the inherited surface does

`crates/spark-core/src/scheduler.rs:474–500` — `Scheduler::drain_due(now)` walks the
`BTreeMap` from the front and removes **every** slot with `due_time <= now`, scheduled
and conflicted alike, returning `DrainOutcome { due, conflicted }`. It has no
cohort granularity. It is the only removal API in Phase 1.

### 2.3 The counterexample, reproduced independently

Codex §7.1 is confirmed. Two scheduled slots for one profile, `C1` at `due_time = 100`
and `C2` at `due_time = 101`; no rule reads or mutates the scheduler; both cohorts inside
every semantic cap:

```text
Run U (one catch-up drain):  drain_due(101)
  at C1's pre-wave capture, C1 and C2 have both been removed
  scheduler digest component = canonical_state_digest(∅)

Run R (one due time per drain): drain_due(100), later drain_due(101)
  at C1's pre-wave capture, C1 removed, C2 still resident
  scheduler digest component = canonical_state_digest({C2})
```

`Scheduler::canonical_state_digest` (`scheduler.rs:545`) hashes the slot count and every
occupied slot, so the two values differ by the Phase-1 R1/R2 contract. The difference
propagates: scheduler digest → `engine_state_digest` → `effect_batch_v3` → `C1`'s
`effect_batch_digest`. Catch-up partition alone changed a canonical batch digest.

The same defect appears inside one due time: with cohorts `(t, P1)` and `(t, P2)`, v3
§3.2 may admit `(t, P1)` and defer `(t, P2)`, but `drain_due(t)` removes both.

### 2.4 Reconciliation verdict

The Codex finding is **accurate, complete, and correctly scoped**. This pass found no
inaccuracy in it and no additional defect in the V3-F01 area. Codex's §7.2 rejection of
the four apparent workarounds (buffer the tail; drain-and-reinsert; ignore because final
state converges; weaken only the oracle) is also confirmed, and this correction adopts
none of them.

The violated controlling invariant is ADR-0003 §15 — "quota/yield changes latency, not
semantics"; "scheduled occurrence identity derives from persisted logical occurrence
indexes, **never batching/worker/catch-up chunks**" — reinforced by R-038 (catch-up
equivalence) and R-044 (split-batch equivalence).

---

## 3. What the controlling record already settles (and this correction must not disturb)

| Controlling source | What it fixes | Bearing on this correction |
|---|---|---|
| Phase-1 `scheduler.rs:118–150` | `WorkKey` = `(due_time, profile_id, producer_definition_id, scope_id, occurrence_index, work_kind)`; derived `Ord` is that field order | The `(due_time, profile_id)` cohort is a **contiguous slice** of an existing total order. No new ordering is invented, and a cohort slice is addressable by a range scan. |
| Phase-1 `scheduler.rs:474–500` | `drain_due` removes the whole `due_time <= now` prefix, partitioning scheduled vs conflicted, each ascending | The prefix is exactly the concatenation, in cohort-sequence order, of the cohort slices. Cohort-granular extraction is a **refinement of the same removal**, not a different one. |
| Phase-1 `scheduler.rs:545` | `canonical_state_digest` hashes slot count + every occupied slot with per-state domain tags and full conflict claim sets | Residency is canonically observable; "byte-identical deferred slots" is a checkable claim, not a wish. |
| Phase-1 `Scheduler::schedule` | total; idempotent on equal payload; poisons on distinct payload; order-independent evidence | Untouched. This correction adds no insertion path. |
| v1 §8 | composed `engine_state_digest` including the scheduler digest; R1/R2 retained-store law | The reason cohort-granular extraction is required rather than cosmetic. |
| v1 Q6 (conclusion upheld by v2 §13) | no scheduler **batch-scheduling** API in Phase 2; AT-B7 contract preserved | Preserved verbatim. §9 forbids this correction from becoming one. |
| v2 §6.3 | over-cap cohorts reject atomically, then their `WorkKey`s are terminally consumed in conflicted-drain shape | Carried forward. Extraction is terminal for the cohort's keys on **both** routes. |
| v2 §11 | every wave-created obligation carries `due_time ≥ current logical time + 1`; "nothing scheduled during a barrier's execution is drained within that same barrier, by construction" | **Decisive.** Intra-boundary enqueues are structurally outside the boundary's due view, so a recomputed view can never admit them. See §6.3. |
| v2 §12 / v1 §8 | retained-store commitment law; transient buffers must not outlive the wave | The proposed view/selection/extraction values are transient and are held to this law (§8). |
| v3 §3.1–§3.2 | cohort = equal-`(profile_id, due_time)` slice; greedy whole-cohort prefix; oversized-earliest exception; deferral untouched | Selector **semantics** unchanged; only its consumption mechanics are made realizable (§6). |
| v3 §4.1–§4.4 | cohort identity, parent sets, emission identity | Unchanged. No causal primitive is added. |
| v3 §3.4 | `PacingDiagnostics` noncanonical, causally inert, in no digest | Unchanged and extended to the new surface (§9.4). |
| Budget §3.1–§3.7 | finite admission limits; quota changes no canonical ordering; deferrable work is resumed; deferred counts are telemetry-visible | Unchanged. Progress (v3 §3.3) is preserved verbatim by §7.3. |

---

## 4. Correction objective, stated as one property

> **P (the property to be delivered).** For a fixed canonical input history and a fixed
> activated artifact, the complete canonical state visible at the instant any cohort
> `C`'s pre-wave engine digest is captured is a function of `C`'s position in the global
> ascending `(due_time, profile_id)` cohort order — and of nothing else. It is
> independent of how drain calls partition the boundary, of the declared pacing budget,
> and of whether pacing is enabled.

Everything in §5–§8 exists to make **P** derivable rather than asserted.

---

## 5. Proposed additive scheduler-consumption surface (C-1 … C-3)

Three additive items on the inherited `Scheduler`. Names are normative for the companion
acceptance matrix's convenience; the **behavior** is what is proposed for freeze. Nothing
below changes `Scheduler::schedule`, `Scheduler::drain_due`, `WorkKey`, `WorkPayload`,
`DueWorkItem`, `SlotState`, `WorkKeyConflict`, `DrainOutcome`,
`ScheduleDisposition`, `WorkSlotStatus`, `Scheduler::canonical_state_digest`, or any
Phase-1 canonical encoding.

### C-1 — `DueCohortView` (read-only selection surface)

```text
Scheduler::due_cohort_view(&self, now: LogicalTime) -> DueCohortView<'_>
```

- **Borrowing, never owning.** The view borrows the scheduler **immutably** for its
  lifetime. It is not `'static`, not `Clone`-into-an-owned-plan, not storable in any
  struct that outlives the borrow. This is the structural guarantee that a due-work plan
  **cannot** be retained across a stable boundary; it is a type-level property, not a
  convention.
- **Content.** The cohorts with `due_time <= now`, enumerated in ascending
  `(due_time, profile_id)` order. Each entry exposes exactly: `due_time`, `profile_id`,
  `work_key_count` (all occupied slots in the slice — see D-4), the slice's `WorkKey`s in
  ascending order, and each key's `WorkSlotStatus`.
- **Excluded from the view.** Payloads, conflict evidence, and any mutable handle. The
  view is a *selection* surface, never an alternative read path for work content. Work
  content is obtained only by extraction (C-3), and only for the cohort actually admitted.
- **Non-mutating.** Calling it leaves `canonical_state_digest()` byte-identical.
- **Deterministic.** Two schedulers with equal `canonical_state_digest()` produce equal
  views for equal `now`.

### C-2 — `DueCohortSelection` (opaque, unforgeable selection token)

```text
DueCohortEntry<'_>::selection(&self) -> DueCohortSelection
```

- Owned, small, and **the only** thing that can name a cohort to C-3.
- Private fields: `due_time`, `profile_id`, `work_key_count`, and
  `cohort_key_digest = H("due_cohort_keys_v1" ‖ count ‖ ascending WorkKey identity
  digests)`.
- **Not constructible from outside.** No public constructor, no public field, no
  `From`/`Deserialize`/literal path — the same discipline `WorkKeyConflict` and
  `StageAcknowledgement` already apply ("evidence the scheduler issued, not a value a
  caller asserts"). A caller therefore cannot ask the scheduler to remove anything the
  scheduler did not itself just offer as a due cohort.
- Carries no payload, no conflict evidence, and no executable content.

### C-3 — `Scheduler::take_due_cohort` (atomic single-cohort extraction)

```text
Scheduler::take_due_cohort(&mut self, selection: DueCohortSelection)
    -> Result<DueCohortExtraction, DueCohortExtractionError>
```

**Success behavior.** Removes, in one indivisible step, exactly the occupied slots whose
`WorkKey` has `due_time == selection.due_time` and `profile_id == selection.profile_id`.
Nothing else is removed, inserted, reordered, or rewritten.

```text
DueCohortExtraction {
    due_time
    profile_id
    due:         Vec<DueWorkItem>       ascending WorkKey order
    conflicted:  Vec<WorkKeyConflict>   ascending WorkKey order
    work_key_count                       = due.len() + conflicted.len()
    cohort_key_digest                    (as C-2, recomputed from the removed slice)
}
```

The `due`/`conflicted` partition, their ordering, the `WorkKeyConflict` construction, and
the conflicted-slot semantics are **exactly** those of `DrainOutcome` — the same code path
semantics, narrowed to one cohort. See the drain-equivalence lemma (§7.4).

**Typed failure paths.** Each leaves the scheduler **byte-identical** (`canonical_state_
digest()` unchanged) and mutates nothing:

| Error | Condition |
|---|---|
| `CohortAbsent` | no occupied slot matches `(due_time, profile_id)` |
| `CohortMembershipChanged` | the live slice's recomputed `cohort_key_digest` ≠ the token's |
| `CohortNotDue` | `selection.due_time > now` for the boundary being processed |

All three are unreachable in the conforming cycle of §6 (the token is minted from the live
view, and nothing mutates the scheduler between S1 and S2). They are specified so the
contract is **fail-closed** rather than silently divergent, and so the oracle has a
determinate atomicity target. The operation has **no panic path**: it performs a bounded
range removal over an existing map, with no fallible re-lookup, matching the discipline
`drain_due` already documents.

---

## 6. Proposed canonical cohort processing sequence

### 6.1 The sequence (the digest-capture ordering rule)

For one drain cycle at boundary time `now`, with declared pacing budget
`B = max_due_per_cycle` (`B >= 1`, v3 §3.2(a) unchanged) and remaining budget `r`
initialised to `B` and `admitted = 0`:

```text
S0  recompute view := Scheduler::due_cohort_view(now)          [no mutation]
S1  if view is empty: the cycle ends.
    let C := the least cohort of view (ascending (due_time, profile_id)).
    admit C iff  C.work_key_count <= r            (v3 §3.2(b) normal admission)
             or  admitted == 0                    (v3 §3.2(c) progress exception)
    otherwise the cycle ends and every remaining cohort is deferred.
S2  extraction := Scheduler::take_due_cohort(C.selection())    [ATOMIC REMOVAL]
S3  pre_wave_engine_digest := engine_state_digest()            [CAPTURED AFTER S2]
    pre_wave_scheduler_digest := Scheduler::canonical_state_digest()
S4  cohort_identity := v3 §4.1 over `extraction`
S5  evaluate waves 0..n against the stable snapshot;
    commit atomically, or reject atomically (v2 §7 preflight, v2 §6.3 over-cap)
S6  STABLE BOUNDARY.
    admitted += 1
    r := (if the §3.2(c) exception fired) 0 else r - C.work_key_count
    drop view/selection/extraction; goto S0
```

**S2 strictly precedes S3, and S3 strictly precedes any evaluation of `C`.** This single
ordering rule is the correction: the cohort leaves canonical scheduler state
*immediately before* its own pre-wave digest, and every later or deferred cohort is still
resident and unmodified at that instant.

### 6.2 Why the selector's semantics are unchanged

For a static due set, S0–S1 iterated with remaining budget `r` admits exactly the unique
maximal ascending prefix of whole cohorts whose cumulative `WorkKey` count is `<= B` —
identical to v3 §3.2(b) — and when that prefix is empty it admits exactly the earliest
cohort whole and nothing else this cycle (`r := 0`) — identical to v3 §3.2(c). There is
no packing, no skipping, no reordering, and no lookahead: S1 only ever inspects the least
remaining cohort. v3 §3.2(d) deferral is unchanged, and is now *structurally* true rather
than merely required, because a deferred cohort is never touched by C-3.

v3 §3.3's progress derivation is preserved verbatim: every cycle with due work admits
`>= 1` cohort; an admitted cohort's keys are removed at S2 and are never returned to the
queue on either terminal route (§7.3); the number of cohorts ahead of any cohort strictly
decreases each cycle.

### 6.3 Why recomputing the view at S0 is safe (and required)

Recomputation is what makes the surface **retained-state-free**: no plan, tail buffer, or
continuation cursor survives S6. It is safe because the only mutations to the scheduler
during a boundary are (i) the extractions themselves and (ii) enqueues performed by
committing waves — and by **v2 §11** every such enqueue carries `due_time ≥ current
logical time + 1 > now`, and "nothing scheduled during a barrier's execution is drained
within that same barrier, by construction". Therefore:

> **Lemma V (view stability).** At every S0 of a boundary, the recomputed view is exactly
> the tail of the boundary-entry cohort sequence beginning at the first not-yet-extracted
> cohort. Recomputation can neither admit intra-boundary enqueues nor drop, reorder, or
> alter any boundary-entry cohort.

This holds under either reading of "boundary" (one drain call, or the family of paced
drain calls at one `now`), because the exclusion follows from `due_time > now` alone.

### 6.4 Command cohorts

Command-barrier rule evaluation forms its own cohort (v3 §3.1, unchanged) with the
finalized command barrier identity of v3 §4.2. It consumes no scheduler slot, so C-1…C-3
do not apply to it, and its pre-wave engine digest is captured at the same relative point
— immediately before its own evaluation, after every strictly-earlier cohort committed.

---

## 7. Derivations

Throughout: fix a canonical input history `H` and an activated artifact. A *partition* is
any legal sequence of drain calls with non-decreasing `now` (ADR-0003 monotonic logical
time) reaching the same final boundary, under any declared `B >= 1`.

### 7.1 Lemma A — cohorts are consumed in globally ascending order in every partition

S0's view enumerates ascending `(due_time, profile_id)` and S1 always selects its least
element. Within one cycle this is ascending by construction. Across cycles: every cohort
extracted in an earlier cycle is absent from every later view, and `now` is
non-decreasing, so any cohort visible at a later `now` but not at an earlier one has
strictly greater `due_time` than every cohort that was visible earlier and therefore
sorts after every already-extracted cohort. Deferred cohorts remain resident (C-3 never
touches them) and are therefore still the least elements of the next view. ∎

### 7.2 Theorem P — partition independence of the pre-wave state

By induction over the cohort sequence of Lemma A.

*Base.* Before the first cohort, the engine state is the state at boundary entry, which is
determined by `H` alone.

*Step.* Assume every cohort strictly before `C` was extracted, evaluated, and committed
from partition-independent state. Then at `C`'s S3:

- **Scheduler component.** Residency = (boundary-entry slots) ∖ (slices of every cohort
  up to and including `C`) ∪ (enqueues committed by cohorts strictly before `C`). By
  Lemma A the set of cohorts before `C` is partition-invariant; by Lemma V no other
  removal or insertion is possible; by the induction hypothesis the earlier commits — and
  hence their enqueues — are identical. `Scheduler::canonical_state_digest` is a pure
  function of that slot set, so the scheduler component is partition-invariant.
- **State store, epoch registry, obligation store, occurrence ledger, cooldown ledger.**
  Each is the committed result of the same earlier cohorts (induction hypothesis).
- **Timeline.** Not mutated by scheduled-cohort processing.

Therefore `pre_wave_engine_digest(C)` is partition-invariant. Because v3 §4.1 cohort
identity depends only on `(profile_id, due_time, the cohort's WorkKeys)`, v3 §7 CE-2 makes
wave indices cohort-scoped, and v3 §4.4 emission identity is a function of those plus
frozen artifact components, **every component of `effect_batch_v3` for `C` is
partition-invariant**, and so is `C`'s committed result. ∎

This closes the falsification in Codex §7.1. In the counterexample, both Run U and Run R
now capture `C1`'s pre-wave scheduler digest as `canonical_state_digest({C2})`, and `C2`'s
as `canonical_state_digest(∅)`.

### 7.3 Corollary — v3 §3.3 progress and v2 §6.3 disposition are preserved

`C`'s keys leave the queue at S2, before evaluation, on **every** route: commit,
atomic wave rejection (v2 §7 preflight, AT-I7b/AT-I8 class), and semantic-cap terminal
consumption (v2 §6.3). No route reinserts them. Cohorts ahead of any cohort strictly
decrease per cycle, exactly as v3 §3.3 derives. Budget §3.3's "safely deferrable work is
resumed in the same deterministic order" is satisfied because deferred cohorts are
untouched canonical slots ordered by `due_time` primacy.

### 7.4 Lemma D — drain equivalence (no Phase-1 semantic is altered)

Let `S` be any scheduler state and `now` any logical time. Let `R` be the sequence of
extractions produced by running §6.1 to exhaustion at `now` with `B` large enough that
the exception never fires, against a null evaluator that commits nothing and enqueues
nothing. Then:

1. `⋃ R.due` equals `drain_due(now).due` as a set, and their concatenation in
   cohort-sequence order equals `drain_due(now).due` **in order**;
2. likewise for `conflicted`;
3. each `WorkKeyConflict` value is identical, including retained evidence, omitted count,
   and truncation flag;
4. the scheduler's final `canonical_state_digest()` equals that after `drain_due(now)`.

This holds because the `due_time <= now` prefix of the `BTreeMap` is precisely the
ordered concatenation of the `(due_time, profile_id)` slices, and C-3 applies the same
per-slot disposition as `drain_due`. Lemma D is the conformance statement that the
proposed surface is a **refinement** of the inherited removal, not a new semantic. It is
an acceptance-test obligation (AT-I42(g)).

---

## 8. Transience and stable-boundary state

- **No retained canonical store is added.** C-1 borrows; C-2 is a token containing a
  coordinate and a digest; C-3 returns owned work that is the cohort's live evaluation
  input, exactly as `DrainOutcome` is today.
- **No transient due-work plan survives a stable boundary.** The view's immutable borrow
  ends at or before S2, and the extraction is consumed by S5. At S6 nothing derived from
  C-1…C-3 exists. This is falsified by AT-I29 reconstruction and AT-I42(f).
- **Failure atomicity of the extraction** is total: the three typed errors mutate nothing
  and leave the scheduler byte-identical.
- **Stable-boundary state after a rejected cohort is explicit.** An atomic wave rejection
  restores every canonical store to its **post-extraction** pre-wave snapshot — the state
  whose digest was captured at S3. The cohort's keys stay consumed; the rejection does not
  reinstate them. This is the same disposition v2 §6.3 already fixes for over-cap
  cohorts and is what v3 §3.3's progress derivation requires. **It is stated here because
  v3 left it implicit, and "the pre-wave digest is preserved" is otherwise ambiguous once
  removal moved inside the cohort step.**
- **No new causal primitive.** Cohort identity, ordering, canonical encoding, and every
  digest are v3/Phase-1 constructions consumed unchanged. The correction changes *when* a
  removal occurs, never *what* is removable or *how* it is identified.

---

## 9. Boundedness — what this surface may not become

These are proposed as binding prohibitions on the future implementation writer, each with
a compile-probe obligation in the companion matrix (AT-I42(h), AT-I32 extension).

1. **Not a batch-scheduling API.** No insertion, no multi-item `schedule`, no reschedule,
   no reinsert. v1 Q6's conclusion and the AT-B7 seam contract at `scheduler.rs:424–431`
   stand unweakened.
2. **Not an arbitrary-removal API.** C-3 accepts only a `DueCohortSelection` minted by a
   live view. There is no removal by `WorkKey`, by key list, by range, by predicate, by
   count, or by `due_time` alone. There is no cancellation path — ADR-0003 §11 forbids
   one, and nothing here creates one.
3. **Not a second read path for work content.** The view exposes no payload and no
   conflict evidence.
4. **Not causally readable telemetry.** Neither the view, the selection, nor the
   extraction may be read by rule evaluation other than as the cohort's own work input;
   `PacingDiagnostics` remains noncanonical, causally inert, and in no digest (v3 §3.4,
   unchanged).
5. **Not a replacement for `drain_due`.** `drain_due` and `schedule` keep their exact
   Phase-1 signatures, semantics, and tests. The Phase-2 evaluator consumes scheduled work
   **only** through C-1…C-3 and never calls `drain_due`; `drain_due` remains available to
   its existing Phase-1 callers and corpus.
6. **Not retained.** Enforced structurally by the borrow in C-1 and by AT-I29/AT-I42(f).

---

## 10. Decisions taken where v3 was ambiguous

Each of these had to be settled for the contract to be unambiguous. Each is flagged for
the independent reviewer.

| # | Ambiguity | Decision | Reasoning |
|---|---|---|---|
| **D-1** | When exactly is the cohort removed relative to its pre-wave digest? | Immediately before (S2 → S3), inside the cohort step. | The only ordering under which Theorem P holds. Codex §7.4(1) prescribes it. |
| **D-2** | Is the due-cohort view computed once per cycle or recomputed per cohort? | Recomputed at every S0. | Removes the last retained-state candidate. Provably equivalent to a fixed plan by Lemma V (v2 §11). A fixed plan would be a retained continuation buffer, which v3 §3.2(d) and §4.6 forbid. |
| **D-3** | Does a **conflicted** slot belong to the cohort and to its §4.1 identity? | **Yes** — the cohort is the whole `(due_time, profile_id)` slice; conflicted keys are extracted with it and their identity digests participate in `scheduled_cohort_identity`. | Any other reading either leaves conflicted keys resident (diverging from `drain_due` and polluting later pre-wave digests) or removes them outside any cohort (an unaccounted removal — precisely the arbitrary removal §9.2 forbids). Their reports go to the cohort's canonical semantic report in v2 §6.3's conflicted-drain shape. |
| **D-4** | Does the pacing budget count conflicted keys? | **Yes** — `work_key_count` is all occupied slots in the slice. | Keeps one number for budget, identity, and extraction. v1 Q4 phrased the budget over `DrainOutcome::due`; that phrasing predates cohort scoping. Pacing is semantics-neutral, so this cannot change any equality claim — but it must be unambiguous. |
| **D-5** | Does an **all-conflicted** cohort produce an effect batch? | No. It is extracted atomically, reports its cohort identity and conflict evidence, evaluates no wave, and emits no `effect_batch_digest`. It still consumes budget and advances progress. | There are no candidates, so `effect_batch_v3` has no defined candidate-set digest. Progress requires the keys to be consumed regardless. |
| **D-6** | How does the oracle observe the pre-wave digest at S3? | Every extracted cohort's **canonical semantic report** carries `cohort_identity`, `pre_wave_engine_digest`, and `pre_wave_scheduler_digest`. | Makes "scheduler-digest equality immediately before every corresponding cohort batch" a direct assertion instead of an inference, and covers D-5 cohorts that emit no batch. These are report fields only: in no causal identity, in no retained store, and equal across partitions by Theorem P. |
| **D-7** | Are the C-1…C-3 names binding? | Behavior is binding; names are normative for the matrix's convenience and may be changed only with a recorded equivalence. | Codex §7.4(1): "exact API name is not important". |

---

## 11. Supersession map against v3

This document **amends** the named v3 items and leaves every other v3, v2, and v1
section standing exactly as written. `SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md`
is **not** edited, moved, or deleted.

| v3 item | Disposition under this candidate |
|---|---|
| §3.1 cohort definition | **Unchanged**, clarified by D-3: the cohort is the whole `(profile_id, due_time)` slice, scheduled and conflicted slots alike. |
| §3.2(a) `max_due_per_cycle >= 1` | **Unchanged.** |
| §3.2(b)(c) selector | **Semantics unchanged**; restated operationally as §6.1 S0–S1 with a remaining budget. Provably identical for a static due set (§6.2). |
| §3.2(d) deferral | **Unchanged and strengthened** — now structurally guaranteed by C-3's single-cohort scope. |
| §3.3 progress | **Unchanged**; re-derived at §7.3. |
| §3.4 `PacingDiagnostics` | **Unchanged**; extended only by §9.4's restatement. |
| §3.5 semantic caps | **Unchanged.** |
| §4.1 scheduled cohort identity | **Unchanged formula**; its `WorkKey` set is settled by D-3 and computed from the C-3 extraction. |
| §4.2–§4.4 command identity, parent sets, emission identity | **Unchanged.** |
| §4.5 batch identity and partition independence | **Formula unchanged.** Its partition-independence claim is **superseded** by §7.2, which derives what §4.5 could only assert, given the extraction ordering of §6.1. |
| §4.6 transience | **Unchanged and extended** to C-1…C-3 by §8. |
| §4.7 invariance table | **Unchanged**; the "drain-call partition / catch-up chunking" row is now backed by §7.2. |
| §5 worked counterexamples | **Unchanged.** |
| §6 scope discipline | **Unchanged**; §12 restates conformance. |
| §7 consistency-edit table CE-1…CE-6 | **Unchanged**, extended by CE-7…CE-9 below. |
| §8 freeze summary | **Amended** by the delta table in §13. |
| §9 verdict `PHASE_2_ARCHITECTURE_FROZEN_V3` | **Not reinstated and not claimed.** The controlling independent verdict remains `PHASE_2_ARCHITECTURE_V3_REVISE`. This candidate does not itself change it. |

### New consistency edits

| # | Edit | Forced by |
|---|---|---|
| **CE-7** | Scheduled work is consumed through the additive cohort-granular surface C-1…C-3, one cohort at a time, extracted immediately before that cohort's pre-wave digest. The Phase-2 evaluator does not call `drain_due`; `drain_due` and `schedule` are unchanged. | V3-F01 §7.4(1)(2) |
| **CE-8** | Every extracted cohort's canonical semantic report carries `cohort_identity`, `pre_wave_engine_digest`, and `pre_wave_scheduler_digest` (report fields only; in no causal identity and no retained store). | V3-F01 §7.4(4) oracle observability; D-5, D-6 |
| **CE-9** | An atomic wave rejection restores canonical state to the **post-extraction** pre-wave snapshot; the rejected cohort's `WorkKey`s remain consumed. | V3-F01 "no ambiguity about failure atomicity or stable-boundary state"; v3 §3.3 |

No pinned digest value moves: no Phase-2 implementation exists, and no Phase-1 encoding,
tag, or digest input is touched. `effect_batch_v3`'s tag and component list are unchanged.

---

## 12. Scope discipline actually observed

- No new causal primitive; no new retained canonical store; no new addressable identity.
- No general scripting; the declarative operation vocabulary (blueprint §19.3) is untouched.
- Phase 1 is not reopened: `WorkKey`, `WorkPayload`, `DueWorkItem`, `WorkKeyConflict`,
  `SlotState`, `DrainOutcome`, `ScheduleDisposition`, `Scheduler::schedule`,
  `Scheduler::drain_due`, `Scheduler::canonical_state_digest`, the timeline, hashing, and
  the authority doors are consumed exactly as closed. Lemma D is the conformance proof.
- No production Rust and no test Rust was written or modified by this pass.
- Phase 2 is not implemented; the implementation writer is not released.
- Phase 3 is not authorized and is not pre-authorized.
- No cleared v2 or v3 decision is reopened; no historical artifact is edited or deleted.

---

## 13. Delta summary over v3

| Item | Disposition |
|---|---|
| Scheduler consumption | additive cohort-granular view + unforgeable selection token + atomic single-cohort extraction (C-1…C-3) |
| `schedule` / `drain_due` | unchanged in signature and semantics; evaluator uses neither for cohort consumption |
| Removal timing | atomic, immediately before the cohort's pre-wave engine digest (S2 → S3) |
| Later / deferred cohorts | resident and byte-identical at every earlier cohort's pre-wave capture |
| Conflicted slots | part of the cohort slice, extracted with it, in its identity and its budget count (D-3, D-4) |
| Retained state | none added; view borrows, selection is a coordinate + digest, extraction is transient |
| Continuation buffer | none; the view is recomputed at every S0 (Lemma V) |
| Failure atomicity | three typed extraction errors, each byte-identical no-ops; wave rejection restores the post-extraction snapshot (CE-9) |
| Partition independence | derived (Theorem P), not asserted |
| Phase-1 conformance | drain-equivalence lemma D, an acceptance obligation |
| Boundedness | six explicit prohibitions with compile-probe obligations (§9) |

---

## 14. Open items and residual risk for the independent reviewer

1. **D-3 / D-4 (conflicted slots in cohort identity and budget count)** are the most
   consequential judgement calls here. They are not stated in v3 and not prescribed by
   Codex §7.4. If the reviewer prefers executable-only cohort identity, the extraction
   contract must instead specify a separate, explicit, canonically ordered disposition of
   conflicted slots — which this pass judges strictly worse (§10 D-3), but which is a
   legitimate alternative the reviewer may impose.
2. **D-6 (report fields)** is a small additive reporting surface. It is proposed because
   the required oracle otherwise has no conforming observation point at S3.
3. **Lemma V** depends on reading v2 §11's "current logical time" during a boundary as the
   boundary's `now`. Under that reading intra-boundary enqueues are `> now` and the lemma
   is immediate. The alternative reading (per-cohort `due_time`) is not adopted; if the
   reviewer imposes it, D-2 must be revisited, because a recomputed view would then admit
   intra-boundary work and the cohort-membership of later due times would become
   partition-sensitive. This is called out rather than assumed.
4. **Lemma D is asserted architecturally and is an acceptance-test obligation
   (AT-I42(g)), not an executed result.** No Rust exists to run it against.
5. This pass performed **no** NIM panel and **no** independent adversarial review of its
   own output. Writer/reviewer separation is intact and is the reason this document
   claims nothing beyond "candidate".

---

## 15. Verdict of this pass

`V3_F01_CORRECTION_CANDIDATE` — submitted for independent review.

- V3-F01 status: **OPEN** (a candidate correction now exists).
- Phase-2 architecture: **NOT frozen**, **NOT accepted**.
- PHASE_2_IMPLEMENTATION_AUTHORIZATION: **NO**
- PHASE_3_AUTHORIZATION: **NO**
- Phase 1: **CLOSED and not reopened.**
