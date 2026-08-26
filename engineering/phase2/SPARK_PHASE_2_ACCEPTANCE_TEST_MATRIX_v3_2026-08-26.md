# S.P.A.R.K. Phase 2 — Acceptance Test Matrix v3 (AT-I)

**Date:** 2026-08-26
**Companion to:** `SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v3_2026-08-26.md` (controlling
design; unqualified section references are to that document, "v2 §…" / "v1 §…" to the
earlier freezes).
**Supersedes:** `SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v2_2026-08-26.md` (retained
unmodified as history), which superseded the v1 matrix (also retained). Tests are marked
**UNCHANGED v2**, **REVISED v3**, or **NEW v3**. Every change here traces to Codex
blocking finding **B01** or **B02** in
`SPARK_PHASE_2_CODEX_ARCHITECTURE_V2_REREVIEW_2026-08-26.md`, or to a consistency edit
enumerated in freeze v3 §7. No test cleared by the rereview is weakened.

**Discipline (unchanged from v1/v2):** identical to AT-A…AT-H. Every AT-I test is
encoded red-first; baselines are recorded verbatim under
`engineering/phase2/<pass>-baseline/` before any production change ("inexpressible
against inherited tree" is recorded, never skipped). Every "X is impossible" claim is a
digest/value equality or a compile-fail, never an error-was-returned check alone. No
existing test is weakened, deleted, or skipped; the Phase-1 suite (232 tests) runs
unmodified throughout. Locations as in v1 (testkit integration files, in-crate
unit/property tests, external compile-probe harness).

**Traceability of the v3 delta:**

| Codex requirement | Test |
|---|---|
| B01 oversized-first-cohort progress, no split | AT-I20c |
| B01 cohort size == budget / budget + 1 / thresholds | AT-I20c cases (a)–(c) |
| B01 multiple successive oversized cohorts | AT-I20c case (d) |
| B01 zero pacing budget rejection | AT-I20c case (e) |
| B01 co-target additive causes across the exception | AT-I20c case (f) |
| B01 no starvation / no livelock | AT-I20c case (g) |
| B01 canonical equivalence with unbudgeted execution | AT-I20c case (h), AT-I20 |
| B01 canonical vs noncanonical report separation | AT-I20d |
| B02 later-wave multi-parent identity replay stability | AT-I6c |
| B02 same numeric outcome, different parent sets ⇒ distinct identity | AT-I6c case (c) |
| B02 exact redelivery of a derived emission stays idempotent | AT-I6c case (d) |
| B02 arbitrary lexical parent selection impossible | AT-I6d |
| B02 pacing partition does not change cohort or later-wave identity | AT-I39 |
| B02 threshold parent-set digest covers every canonicalized contributing parent | AT-I25 |
| B02 reconstruction needs no hidden continuation/parent state | AT-I29 |
| CE-1 cohort is per-(profile, due_time) | AT-I40 |
| CE-2 wave index is cohort-scoped | AT-I41 |
| CE-3/CE-4 batch digest binds cohort identity; heartbeat retired | AT-I39, AT-I41 |

---

## Group 1 — Stable-snapshot isolation (freeze §4; v2 §5.1; v1 §Q2)

- **AT-I1 `evaluation_reads_only_the_pre_wave_snapshot`** (UNCHANGED v2) — overlapping
  same-target `AddDelta` candidates whose emitting rules also read the target prove every
  delta was computed from the one pre-wave value and the fold applied exactly once:
  committed value equals `pre + Σdeltas`, never `((pre + d₁) + d₂)`-with-re-read or any
  partial-fold intermediate; engine digest equals the same transition applied to an
  untouched clone.
- **AT-I2 `mid_wave_state_is_unobservable`** (UNCHANGED v2).

## Group 2 — Rule-order and insertion-permutation invariance (v2 §4; v1 §Q1)

- **AT-I3 `rule_declaration_order_cannot_change_canonical_state`** (UNCHANGED v2).
- **AT-I4 `work_insertion_permutation_invariance`** (REVISED v3) — as v2, extended to
  assert that the **scheduled cohort identity** (§4.1) is byte-identical across every
  `WorkKey` insertion permutation of the same cohort, and that it changes when any
  member `WorkKey` is added, removed, or altered in any semantic field.

## Group 3 — Emission identity, ordering, idempotency (freeze §4; v2 §3, §5)

- **AT-I5 `effects_commit_in_frozen_total_order`** (UNCHANGED v2) — canonical
  target-group and resolved-result order; digest-identical barrier replay; renaming rule
  IDs or changing their lexical/sort order can never convert an incompatible same-target
  group into an accepted one nor change any reduced result.
- **AT-I6a `exact_emission_duplicate_is_idempotent`** (UNCHANGED v2) — same identity and
  payload twice in a wave applies once (`20 + 10 → 30`, one committed write, provenance
  counts one cause); reused identity with a different payload rejects the wave atomically
  with the pre-wave digest preserved, in every permutation.
- **AT-I6b `distinct_equal_contributions_are_not_deduplicated`** (UNCHANGED v2) — two
  distinct emission identities each contributing `+10` to `20` produce `40`; both sources
  represented in provenance subject to the bounded-coverage contract (Group 14).
- **AT-I6c `later_wave_multi_parent_emission_identity_is_replay_stable`** (NEW v3 —
  Codex B02) — the fixture: pre-wave `stress = 20`; `WorkKey A → AddDelta(+10)` and
  `WorkKey B → AddDelta(+15)` in one cohort; fully reduced `45`; threshold `40` crosses
  and emits obligation `O` at wave 1.
  - **(a) Permutation stability.** Across the full cross product of `WorkKey` insertion
    permutations, rule declaration/ID permutations, candidate enumeration permutations,
    reduction-group enumeration permutations, and **parent enumeration permutations**,
    the following are byte-identical: `O`'s complete emission identity, its parent-set
    digest, the cohort identity, the wave-1 candidate-set digest, the wave-1
    `effect_batch_digest`, `O`'s obligation record hash, its occurrence mapping, the
    canonical wave report, and the final engine digest.
  - **(b) Parent-set completeness.** `O`'s parent-set digest equals the hand-computed
    `H("parents_emission_v1" ‖ 2 ‖ [e_A, e_B] ascending)` for the independently computed
    wave-0 emission identities `e_A`, `e_B`. Recomputing it from a set missing either
    parent yields a different digest — asserted as an explicit inequality, so a
    single-parent implementation fails here.
  - **(c) Same result, different causes.** A second fixture whose parents are `+20` and
    `+5` from different emissions also reduces to `45` and crosses the same threshold;
    its wave-1 emission identity, obligation record hash, and candidate-set digest are
    **different** from (a)'s, while the committed cell values are equal. Payload never
    entered identity; parent set did.
  - **(d) Derived redelivery.** Exact redelivery of one derived emission (same identity,
    same payload) inside the wave folds to one candidate and applies once; two distinct
    derived cause sets are never deduplicated; one derived identity carrying distinct
    payloads rejects the wave atomically.
  - **(e) Depth transitivity.** A wave-2 emission derived from `O` and a second wave-1
    emission has a parent-set digest over those two **emission identities**; perturbing a
    wave-0 parent changes the wave-2 identity, proving the chain is complete without any
    retained lineage store.
  - **(f) Empty parent set.** A forged wave-`N≥1` candidate with an empty parent set is
    rejected as a typed evaluator defect with the pre-wave digest preserved (freeze
    §4.3 rule 3) — never assigned a degenerate identity.
- **AT-I6d `no_parent_is_ever_selected`** (NEW v3 — Codex B02) — falsifies every
  arbitrary-selection implementation:
  - adding a **third contributing parent that sorts lexically below** both existing
    parents changes `O`'s identity (it is a different cause set) but does **not** make
    that parent the identity;
  - **renaming** an existing parent's rule ID so that it becomes the lexical minimum,
    while its emission identity content is otherwise unchanged, produces an identity
    change **iff** the rule fingerprint genuinely changed — never a change attributable
    to sort position alone; a control fixture that permutes only enumeration order (no
    content change) produces byte-identical results;
  - a compile-probe asserts that no `min`/`first`/`selected_parent`-shaped accessor
    exists on the parent-set type, and that parent sets cannot be constructed from
    outside the evaluator door (freeze §4.3 rule 1, §4.6).

## Group 4 — Same-target composition and atomic rejection (v2 §4, §5.3)

- **AT-I7a `simultaneous_additive_causes_fold_from_one_snapshot`** (UNCHANGED v2) —
  `20 + 10 + 15 = 45` in every permutation; one committed write; identical batch digest.
- **AT-I7b `distinct_exclusive_values_reject_the_whole_wave_atomically`** (UNCHANGED v2).
- **AT-I7c `equal_exclusive_results_coalesce_within_family`** (UNCHANGED v2) — including
  the counter-cases: equal values across different families reject, and two equal-valued
  TRANSFORM candidates reject.
- **AT-I7d `incompatible_update_families_reject_atomically`** (UNCHANGED v2).
- **AT-I7e `additive_fold_overflow_or_bounds_failure_is_atomic`** (UNCHANGED v2).
- **AT-I8 `invalid_effect_rejects_the_whole_wave_atomically`** (UNCHANGED v2) — including
  late candidate-transition failures: occurrence exhaustion (mid-range in a
  multi-allocation), obligation-store admission cap, scheduler queue admission cap, and
  scheduler↔ObligationStore invariant failure.
- **AT-I9 `rejected_wave_is_reproducible`** (UNCHANGED v2).
- **AT-I37 `operation_pair_classification_is_frozen`** (UNCHANGED v2) — the full
  table-driven pair matrix, each pair with exactly one expected class, every rejection
  digest-atomic.

## Group 5 — Authority and bounds violations (UNCHANGED v2)

- **AT-I10 `rule_targeting_host_owned_is_rejected_at_activation`** (UNCHANGED v2).
- **AT-I11 `evaluator_writes_respect_declared_write_classes`** (UNCHANGED v2).

## Group 6 — Delayed-obligation identity and collisions (v2 §8; v1 §5)

- **AT-I12 `obligation_payload_hash_is_the_record_hash`** (UNCHANGED v2).
- **AT-I13 `materialized_mode_refuses_fingerprint_mismatch`** (UNCHANGED v2).
- **AT-I14 `reevaluation_mode_requires_the_exact_rule_artifact`** (UNCHANGED v2).
- **AT-I15 `obligation_collisions_poison_order_independently`** (REVISED v3) — as v2
  (three or more distinct records under one `WorkKey`, all insertion permutations, cap
  boundaries, record-hash discrimination, complete contested set removed at the poisoned
  drain, reschedule under the next occurrence), extended so that at least one contested
  record originates from a **wave-1 multi-parent derived emission**: its record hash is
  a function of the complete parent-set-derived emission identity, so two contested
  records whose payload values coincide but whose parent sets differ remain
  discriminated.

## Group 7 — Occurrence indexes (freeze §7 CE-6; v2 §9; v1 §6)

- **AT-I16 `occurrence_ledger_is_committed_and_order_independent`** (REVISED v3) — as v2
  (two or more simultaneous allocations for one ledger key in one cohort under permuted
  candidate enumeration; allocation follows ascending complete emission identity;
  identical payload↔occurrence mapping and final engine digest across permutations;
  mid-range exhaustion rejects atomically per AT-I8), extended so that **at least two of
  the competing claims are wave-`N≥1` derived emissions with different parent sets**.
  This is the case v2 §9 could not decide, because its sort key was undefined at wave
  ≥ 1. The test additionally asserts the mapping is unchanged when the same history is
  executed under a pacing budget that splits the boundary into several drain calls.
- **AT-I17 `per_gate_random_addresses_are_distinct_and_stable`** (UNCHANGED v2).

## Group 8 — Cohorts, pacing, cycle rejection, depth, backpressure (freeze §3, §4; v2 §6, §11)

- **AT-I18 `zero_delay_cycles_are_rejected_at_activation`** (UNCHANGED v2).
- **AT-I19 `declared_fan_out_and_depth_bounds_are_enforced_statically`** (UNCHANGED v2).
- **AT-I20 `pacing_defers_whole_cohorts_and_preserves_semantics`** (REVISED v3 — Codex
  B01/B02) — as v2 (the due-work set is built so a stable-prefix cut *would* split
  co-target additive causes and a threshold input set inside one cohort; only whole
  cohorts defer, in ascending order; the deferred cohort is untouched in the scheduler,
  digest-checked), strengthened on three axes:
  - **at least two due-time cohorts**, chosen so the paced run requires strictly more
    drain calls than the unbudgeted run;
  - the paced/unbudgeted comparison now covers **complete wave-0 and later-wave emission
    identities**, canonical candidate-set digests, committed-effect blocks, per-cohort
    `effect_batch_digest`s, scheduled cohort identities, canonical semantic cohort/wave
    reports, cells, scheduler, obligation store, occurrence ledger, cooldowns, threshold
    emissions, and the final engine digest — not only final threshold outputs;
  - **noncanonical `PacingDiagnostics` are compared separately** and asserted to differ
    (see AT-I20d) rather than being folded into the equality claim.
  The opposing-contributions case is retained: a partial fold would have falsely crossed
  the threshold and enqueued an obligation; the complete fold does not.
- **AT-I20b `semantic_cap_overflow_is_atomic_and_terminal`** (UNCHANGED v2) — a cohort
  exceeding `max_cohort_candidates`/`max_effects_per_wave` rejects before any canonical
  mutation, then consumes the cohort's `WorkKey`s with a typed order-independent overload
  report (conflicted-drain shape); reschedule under the next occurrence succeeds;
  identical outcome on replay (caps are epoch-bound).
- **AT-I20c `oversized_first_cohort_makes_progress_without_split`** (NEW v3 — Codex
  B01) — the central B01 falsification. Base fixture: an earliest cohort of `N`
  `WorkKey`s at one `(profile, due_time)` whose candidate, effect, enqueue, queue, depth,
  and fan-out volumes are at or below **every** semantic cap, followed by at least one
  later cohort. Cases:
  - **(a) `N == max_due_per_cycle`.** Admitted by the normal prefix rule (§3.2(b));
    `pacing_overrun` is **false**; the later cohort defers.
  - **(b) `N == max_due_per_cycle + 1`.** The prefix is empty, so the exception
    (§3.2(c)) admits the whole cohort **exactly once**; `pacing_overrun` is **true** with
    `admitted_work_key_count == N` and the correct `overrun_cohort_identity`; **no later
    cohort is admitted**; every later cohort is untouched in the scheduler
    (digest-checked before and after).
  - **(c) Threshold behavior at the boundary.** A companion sub-case with
    `N == max_due_per_cycle - 1` (fits with room to spare, later cohort may also fit) and
    `N` far above the budget both hold; the admitted set is the frozen greedy prefix in
    every case and never a repacked or reordered selection.
  - **(d) Multiple successive oversized cohorts.** Three consecutive cohorts each larger
    than the budget: cycles 1, 2, 3 admit exactly cohort 1, then 2, then 3, each whole,
    each once, in ascending order, each with `pacing_overrun == true`; no cohort is ever
    admitted twice and none is skipped.
  - **(e) Zero budget.** A declared `max_due_per_cycle == 0` is **refused at the
    activation door**, atomically, leaving no partial `ActivatedRuleSet` and no epoch
    record; asserted as a value/digest equality against the pre-activation registry, not
    merely as an error return. A compile-probe asserts no runtime path can set the budget
    outside activation.
  - **(f) Co-target additive causes and threshold inputs.** The oversized cohort contains
    both equal (`+10/+10`) and unequal (`+10/+15`) co-target additive causes and an
    opposing-contribution threshold input set. The cohort is admitted **whole**: committed
    values equal the hand-computed `pre + Σdeltas`, exactly one committed write per
    target, and the threshold decision matches the complete fold. **No split is
    observable at any granularity** — asserted by pinning the admitted `WorkKey` set to
    the complete cohort and by the absence of any partial-fold intermediate.
  - **(g) No starvation, no livelock.** Driving the engine for a bounded number of cycles
    over a fixed due-work set, every cohort is admitted within finitely many cycles and
    the scheduler reaches empty. Explicitly falsified: an implementation that returns a
    deferral report without consuming the earliest cohort fails, because the test asserts
    **strictly positive `admitted_work_key_count` on every cycle for which due work
    exists**, and asserts the scheduler's due-key count strictly decreases across cycles.
  - **(h) Canonical equivalence with unbudgeted execution.** Over the identical canonical
    input history, the paced run and a run with pacing effectively unbounded are
    bit-identical across cells, scheduler, obligation store, occurrence ledger, cooldowns,
    epoch registry, canonical semantic cohort/wave reports, canonical candidate sets,
    committed-effect blocks, every scheduled cohort identity, every wave-0 and later-wave
    emission identity, every `effect_batch_digest`, and the final engine digest.
  - **(i) Semantic caps still bite.** A variant of the oversized cohort whose evaluation
    **does** exceed `max_cohort_candidates` is admitted structurally and then rejected
    atomically and consumed terminally per AT-I20b — proving the selector never
    pre-judges legality (freeze §2.1, B01-P3 modification) and that progress holds on
    both terminal routes.
- **AT-I20d `pacing_diagnostics_are_noncanonical_and_causally_inert`** (NEW v3 — Codex
  B01-X) — over the AT-I20/AT-I20c histories:
  - the paced run's `PacingDiagnostics` truthfully report deferral/overrun and the
    unbudgeted run's do not — asserted as an explicit **inequality**, so an
    implementation that suppresses honest telemetry to force report equality fails;
  - `PacingDiagnostics` appear in **no** canonical digest: engine digest,
    `effect_batch_digest`, every emission identity, every obligation record hash,
    occurrence allocation order, provenance blocks, and every retained-store digest are
    identical between the paced and unbudgeted runs;
  - diagnostics are **deterministic**: replaying the same history with the same budget and
    the same drain-call sequence reproduces them byte-identically;
  - a compile-probe asserts the evaluator cannot read them — no rule, effect, threshold,
    obligation, cooldown, or scheduler decision is given the value (freeze §3.4 rule 3).
- **AT-I21 `depth_overflow_converts_to_strictly_later_work`** (UNCHANGED v2) —
  propagation pending at `max_wave_depth` converts to scheduled work with
  `due_time ≥ now + 1`, visibly reported, never dropped, provably not executed within the
  same barrier (the barrier's drain set is pinned and the converted key is absent). In
  isolation the converged state is digest-identical under a larger depth budget; with
  interfering due work the assertions are determinism, continuation priority, and exact
  boundary identity — not cross-budget digest equality (adjudicated v2 §2 C-11, upheld).

## Group 9 — Decay/recovery, aggregation, thresholds (v2 §4.3, §5)

- **AT-I22 `decay_catch_up_is_chunk_invariant`** (UNCHANGED v2).
- **AT-I23 `recovery_and_decay_respect_epoch_bound_rates`** (UNCHANGED v2).
- **AT-I24 `aggregation_is_permutation_and_path_invariant`** (UNCHANGED v2).
- **AT-I25 `threshold_emission_uses_committed_state_after_complete_reduction`**
  (REVISED v3 — Codex B02) — as v2 (the crossing is computed exactly once from
  previously-committed value → fully-reduced candidate result; opposing contributions
  pin both the false-positive and false-negative partial-fold cases; crossing emissions
  appear only in wave N+1 or as strictly later scheduled work), strengthened with the
  parent-set coverage assertion:
  - the threshold emission's **parent-set digest covers every distinct canonicalized
    contributing parent** of the reduction it read — asserted by recomputing the digest
    from the independently enumerated canonical candidate set and comparing for equality,
    and by asserting inequality against every proper subset of that parent set;
  - an **exact duplicate parent appears exactly once** in the set (canonicalization
    precedes parent-set construction), so a redelivered contribution does not inflate the
    digest;
  - a **read target with no candidates in the wave contributes no parents**, and the
    emission is still fully discriminated because the wave's `effect_batch_digest`
    commits to the pre-wave engine digest (freeze §4.3 rule 2) — pinned by two fixtures
    with identical parent sets but different unchanged background state, which share the
    emission identity and differ in the batch digest;
  - a **multi-target** threshold/propagation operation reading two written groups has a
    parent-set digest equal to the union over both, in ascending order, with no per-target
    selection.

## Group 10 — Hidden-state / equal-digest equivalence (v2 §12; v1 §8)

For **each** of: EpochRegistry, ObligationStore (including its contested claim-set
state), OccurrenceLedger, CooldownLedger, and any derived index/cache the implementation
introduces:

- **AT-I26 `<store>_discrimination`** (UNCHANGED v2).
- **AT-I27 `<store>_equal_digest_same_next_input`** (UNCHANGED v2).
- **AT-I28 `<derived index>_recomputation_invariant`** (UNCHANGED v2).
- **AT-I29 `wave_buffers_do_not_outlive_the_wave`** (REVISED v3 — Codex B02) — as v2
  (the transient set includes the candidate multiset, emission-identity canonicalization
  state, reduction groups, and the preflighted transition; after any committed or
  rejected wave, fresh reconstruction from committed inputs is digest-identical and
  next-barrier-identical), extended with:
  - **parent sets are transient.** After a wave, no parent-set or lineage structure
    survives; a fresh engine reconstructed from committed inputs reproduces every
    subsequent emission identity, obligation record hash, and occurrence mapping exactly.
  - **reconstruction across a paced deferral.** At the stable boundary **between drain
    calls, after a cohort has been deferred and before that deferred cohort's waves are
    evaluated**, the engine is reconstructed from committed state alone; the deferred
    cohort then evaluates to byte-identical cohort identity, emission identities, batch
    digests, canonical reports, and next-barrier behavior. This proves **no uncommitted
    continuation, barrier-parent, drain-set, or pacing state is retained** (freeze §4.6).
  - **boundary discipline.** The reconstruction points asserted are the frozen stable
    boundaries — between drain calls, and after a cohort commits or rejects. Mid-cohort
    inter-wave points are **not** stable boundaries by construction (ADR-0003 §15) and are
    asserted unreachable by a compile-probe, so the test never claims reconstruction at a
    point the boundary model does not admit.

## Group 11 — Global/dormant boundedness (UNCHANGED v2)

- **AT-I30 `work_scales_with_due_keys_not_scope_count`** (UNCHANGED v2).
- **AT-I31 `dormant_aggregates_update_only_at_their_cadence`** (UNCHANGED v2).

## Group 12 — Panic freedom and gate-level requirements

- **AT-I32 — strict lint gate extended** (REVISED v3) — as v2, with the compile-probe set
  extended to the new constructions: emission contexts, cohort identities, parent sets,
  and `PacingDiagnostics` cannot be forged, mutated, or constructed from outside the
  evaluator door; no parent-selection accessor exists (AT-I6d); the evaluator is not given
  a `PacingDiagnostics` reference (AT-I20d).
- **AT-I33 — adversarial input totality** (REVISED v3) — as v2, with the sweep extended
  to: maximal cohort `WorkKey` counts far exceeding `max_due_per_cycle`, cohorts at
  exactly `max_due_per_cycle` and `max_due_per_cycle + 1`, maximal parent-set cardinality
  at `max_cohort_candidates`, and maximal wave depth with multi-parent chains at every
  level. No panic anywhere; every failure typed.

## Group 13 — Phase-1 regression gates (UNCHANGED v2 and mandatory)

- **AT-I34 — the complete inherited suite** (UNCHANGED v2; 232 tests + AT-I additions,
  nothing removed).
- **AT-I35 — Phase-1 digest stability** (UNCHANGED v2; freeze v3 changes no Phase-1
  canonical encoding — `StateCell`, manifests, configs, scheduler, `WorkKey`,
  `identity_digest`, and timeline all bit-identical; cohort identity and parent-set
  digests are new Phase-2 constructions that *consume* `WorkKey::identity_digest()`
  without altering it).
- **AT-I36 — platform static checks** (UNCHANGED v2).

## Group 14 — Provenance coverage (v2 §10)

- **AT-I38 `provenance_cap_exposes_honest_coverage`** (REVISED v3) — as v2 (the fold
  never truncates, exact coverage counts and status under `provenance_prune.v1`,
  `complete` iff nothing omitted, `unknown` only for inherited prior-state provenance,
  equal-digest/same-next-input probe), extended with the B02 separation: **provenance
  truncation never truncates the parent set used for identity**. Two cause sets sharing
  the same retained smallest emission identities but differing beyond `MAX_SOURCE_REFS`
  produce the same bounded presentation, different `omitted_source_count`, identical
  arithmetic — and **different later-wave emission identities**, because the parent-set
  digest covers every canonicalized parent regardless of the presentation cap.

## Group 15 — Cohort scoping and partition independence (NEW v3 — freeze §7 CE-1…CE-4)

- **AT-I39 `causal_identity_is_independent_of_drain_partition`** (NEW v3 — Codex B02) —
  one canonical input history executed under several partitions: (i) one drain call;
  (ii) a pacing budget forcing two drain calls; (iii) a pacing budget forcing one drain
  call per cohort; (iv) **catch-up** — ten due times evaluated in a single drain versus
  one due time evaluated ten times (R-038); and (v) a split-batch variant (R-044). Across
  every partition, all scheduled cohort identities, all wave-0 and later-wave emission
  identities, all `effect_batch_digest`s, all obligation record hashes, all occurrence
  mappings, all canonical semantic reports, and the final engine digest are
  **byte-identical**. Only `PacingDiagnostics` differ. A regression fixture pins the
  retired v1 heartbeat descriptor `H("heartbeat" ‖ logical time ‖ digest of drained
  WorkKeys)` as **absent from every canonical digest** — computed independently and
  asserted to have no influence, so an implementation that reintroduces it fails.
- **AT-I40 `cohort_is_scoped_to_profile_and_due_time`** (NEW v3 — freeze §7 CE-1) — a
  boundary draining work for two profiles at one `due_time` forms **two** cohorts with
  distinct identities, evaluated in ascending `(due_time, profile_id)` order. Asserted:
  the partition is contiguous in the frozen `WorkKey` total order; no co-target causal
  group is split (a compile-probe plus a runtime assertion confirm a cause in profile `P`
  cannot target a cell in profile `Q`); each cohort's identity depends only on its own
  members; and adding work for a third profile does not perturb the first two cohorts'
  identities, batch digests, or emission identities.
- **AT-I41 `wave_index_and_depth_budget_are_cohort_scoped`** (NEW v3 — freeze §7 CE-2) —
  two cohorts at one boundary each run their own wave chain from `wave_index = 0`, each
  with a full `max_wave_depth` budget. Asserted: the second cohort's wave-0 emission
  identities are what they would be if it were the only cohort at the boundary; a
  drain-scoped wave index or a drain-scoped shared depth budget is falsified by comparing
  the ten-days-once and one-day-ten-times executions (R-038) for byte equality; and the
  `effect_batch_v3` digest binds cohort identity and the cohort-scoped wave index (CE-3),
  pinned by recomputing the digest from its declared components.

---

## Explicitly unchanged tests (conformance checklist for the reviewer)

The v1 and v2 tables stand: AT-H suite and timeline corpus, scheduler suite (AT-B, AT-G
scheduler half — `Scheduler` consumed, not modified; no batch API), activation ceremony
and compile probes, AT-G hidden-evidence guards, manifest/config hash pins. Every AT-I
test the Codex rereview ruled SUFFICIENT is carried into v3 unweakened.

## Out of scope

As v1/v2: no persistence/service/protocol tests, no profile parsing/IO, no executable
Windows/Android runs, no Phase-4+ semantics, no MCI/game integration, no runtime-LLM
surface. Phase-2 implementation is not authorized by this matrix, and Phase 3 is not
authorized and is not pre-authorized here.
