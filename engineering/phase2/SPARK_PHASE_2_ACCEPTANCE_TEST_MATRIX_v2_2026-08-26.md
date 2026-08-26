# S.P.A.R.K. Phase 2 — Acceptance Test Matrix v2 (AT-I)

**Date:** 2026-08-26
**Companion to:** `SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v2_2026-08-26.md` (controlling
design; section references are to that document, "v1 §…" to the original freeze).
**Supersedes:** `SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_2026-08-26.md` (retained
unmodified as history). Tests are marked **UNCHANGED**, **REVISED v2**, or **NEW v2**.
Every Codex §9 mandated change is incorporated; the single modification (AT-I21
interference equivalence) is adjudicated in freeze v2 §2 C-11.

**Discipline (unchanged):** identical to AT-A…AT-H. Every AT-I test is encoded
red-first; baselines are recorded verbatim under `engineering/phase2/<pass>-baseline/`
before any production change ("inexpressible against inherited tree" is recorded, never
skipped). Every "X is impossible" claim is a digest/value equality or a compile-fail,
never an error-was-returned check alone. No existing test is weakened, deleted, or
skipped; the Phase-1 suite runs unmodified throughout. Locations as in v1 (testkit
integration files, in-crate unit/property tests, external compile-probe harness).

---

## Group 1 — Stable-snapshot isolation (freeze §5.1; v1 §Q2)

- **AT-I1 `evaluation_reads_only_the_pre_wave_snapshot`** (REVISED v2) — as v1, plus:
  overlapping **same-target** `AddDelta` candidates whose emitting rules also read the
  target must prove that every delta was computed from the one pre-wave value and the
  fold was applied exactly once — committed value equals the hand-computed
  `pre + Σdeltas`, never `((pre + d₁) + d₂)`-with-re-read or any partial-fold
  intermediate; engine digest equals the same transition applied to an untouched clone.
- **AT-I2 `mid_wave_state_is_unobservable`** (UNCHANGED) — compile-level and behavioral
  wave-chaining assertions as v1.

## Group 2 — Rule-order and insertion-permutation invariance (freeze §4; v1 §Q1)

- **AT-I3 `rule_declaration_order_cannot_change_canonical_state`** (REVISED v2) — as
  v1, with the permutation corpus extended to rule sets containing **equal and unequal
  additive contributions to one target** (`+10/+15` and `+10/+10`), same-family equal
  RESULT candidates, and a singleton TRANSFORM: identical `ruleset_content_hash`,
  engine digests, and wave reports across all permutations.
- **AT-I4 `work_insertion_permutation_invariance`** (UNCHANGED).

## Group 3 — Emission identity, deterministic ordering, idempotency (freeze §3, §5)

- **AT-I5 `effects_commit_in_frozen_total_order`** (REVISED v2) — asserts canonical
  **target-group and resolved-result** order against the wave report, digest-identical
  barrier replay, and additionally: renaming rule IDs or changing their lexical/sort
  order can never convert an incompatible same-target group into an accepted one, nor
  change any reduced result — sorting orders groups and reports, never causal meaning.
- **AT-I6a `exact_emission_duplicate_is_idempotent`** (NEW v2, replaces AT-I6) — the
  same emission identity and payload delivered twice within a wave applies once
  (`20 + 10 → 30`, one committed write, provenance counts one cause); a reused emission
  identity with a **different** payload rejects the wave atomically (pre-wave digest
  preserved) in every permutation.
- **AT-I6b `distinct_equal_contributions_are_not_deduplicated`** (NEW v2, replaces
  AT-I6) — two distinct rule/occurrence emission identities each contributing `+10`
  to `20` produce `40`; both sources are represented in provenance subject to the
  bounded-coverage contract (Group 14); digest equality across permutations.

## Group 4 — Same-target composition and atomic rejection (freeze §4, §5.3)

- **AT-I7a `simultaneous_additive_causes_fold_from_one_snapshot`** (NEW v2, replaces
  AT-I7) — `20 + 10 + 15 = 45` for every rule, candidate, and internal evaluation
  permutation; exactly one committed target write; batch digest identical across
  permutations.
- **AT-I7b `distinct_exclusive_values_reject_the_whole_wave_atomically`** (NEW v2) —
  same-family `ExclusiveResult(30)` and `ExclusiveResult(35)`: engine digest after
  rejection equals the pre-wave digest exactly (cells, obligations, ledgers, epoch
  registry, scheduler untouched); order-independent evidence; identical report value in
  all permutations.
- **AT-I7c `equal_exclusive_results_coalesce_within_family`** (NEW v2) — distinct
  emissions assigning the same value in the **same** RESULT family commit once with
  merged provenance, and are not mislabeled as delivery duplicates (provenance counts
  two causes). Counter-cases pinned in the same test: equal resolved values across
  **different** families reject, and two equal-valued TRANSFORM candidates (two
  `scale ×1.5`) reject — numeric coincidence never coalesces intent (freeze §2 C-2).
- **AT-I7d `incompatible_update_families_reject_atomically`** (NEW v2) —
  assignment+delta, undeclared add+scale, decay+shock, and multiple exclusive
  transforms each reject in every permutation: at rule-set activation where co-targeting
  is statically provable (atomic, no partial `ActivatedRuleSet`), else at runtime with
  pre-wave digest preserved.
- **AT-I7e `additive_fold_overflow_or_bounds_failure_is_atomic`** (NEW v2) — checked
  `i128` fold failure, checked application to the pre-wave value, final domain
  conversion, and target-bounds failure each reject **before any** cell, ledger,
  obligation, occurrence, cooldown, or queue mutation (preflight, freeze §7); boundary
  values per AT-I11's sweep.
- **AT-I8 `invalid_effect_rejects_the_whole_wave_atomically`** (REVISED v2) — as v1,
  extended with **late candidate-transition failures**: occurrence `u64` exhaustion
  (including mid-range in a multi-allocation, Group 7), obligation-store admission-cap
  exhaustion, scheduler queue admission-cap exhaustion, and
  scheduler↔ObligationStore invariant failure — every retained store and cell remains
  digest-unchanged in every permutation. (Obligation `WorkKey` collision is asserted as
  a legitimate committed poisoning outcome, *not* a rejection — Group 6.)
- **AT-I9 `rejected_wave_is_reproducible`** (UNCHANGED).
- **AT-I37 `operation_pair_classification_is_frozen`** (NEW v2) — table-driven over
  same-target pairs and their permutations:

  ```text
  add/add                              -> additive fold
  add/subtract                         -> additive fold (signed; checked negation edge)
  scale/scale                          -> reject (activation where provable, else runtime atomic)
  add/scale                            -> reject
  clamp/clamp                          -> reject (no intersection semantics)
  add/clamp                            -> reject
  weighted-result/weighted-result      -> equal: coalesce; unequal: conflict reject
  decay/add                            -> reject
  decay/decay                          -> activation reject (one reducer per target)
  aggregate-result/aggregate-result    -> activation reject (one authoritative reducer);
                                          dynamic co-emission: equal coalesce / unequal reject
  ```

  Every pair has exactly one expected class; no case may accept "whichever sorted
  effect runs first", and each rejection case is digest-atomic.

## Group 5 — Authority and bounds violations (UNCHANGED)

- **AT-I10 `rule_targeting_host_owned_is_rejected_at_activation`** (UNCHANGED).
- **AT-I11 `evaluator_writes_respect_declared_write_classes`** (UNCHANGED, applied to
  committed effects post-reduction).

## Group 6 — Delayed-obligation identity and collisions (freeze §8; v1 §5)

- **AT-I12 `obligation_payload_hash_is_the_record_hash`** (REVISED v2) — as v1, with
  the bidirectional scheduler↔ObligationStore invariant recomputed over the
  **claim-set-valued** store after every prefix of a scripted
  schedule/collide/drain/poison/reject sequence, including contested states holding
  multiple records under one work identity.
- **AT-I13 `materialized_mode_refuses_fingerprint_mismatch`** (REVISED v2) — as v1,
  plus a **multi-target** materialized effect list whose *second* target's definition
  fingerprint changes under a new epoch: execution refuses explicitly and atomically
  (nothing committed, refusal visible); with all fingerprints matching, execution is
  bit-identical to scheduling-time resolution.
- **AT-I14 `reevaluation_mode_requires_the_exact_rule_artifact`** (UNCHANGED).
- **AT-I15 `obligation_collisions_poison_order_independently`** (REVISED v2) — extended
  to **three or more** distinct obligation records under one `WorkKey` across **all
  insertion permutations**, at and beyond claim-set cap boundaries: equal scheduler and
  store digests per permutation, record-hash discrimination (differing tracked claims ⇒
  differing digests), poisoned drain reports the conflict, the **complete contested
  set** leaves the store with it, and post-drain reschedule under the next occurrence
  succeeds.

## Group 7 — Occurrence indexes (freeze §9; v1 §6)

- **AT-I16 `occurrence_ledger_is_committed_and_order_independent`** (REVISED v2) — as
  v1, plus **two or more simultaneous allocations for one ledger key** in one cohort
  under permuted candidate enumeration: allocation follows ascending complete emission
  identity, the payload↔occurrence mapping and final engine digest are identical across
  permutations, and mid-range exhaustion rejects atomically per AT-I8.
- **AT-I17 `per_gate_random_addresses_are_distinct_and_stable`** (UNCHANGED; sub-ID
  uniqueness validation now covers all emitting operations, freeze §3.2).

## Group 8 — Cohorts, cycle rejection, depth, backpressure (freeze §6, §11; v1 §Q4)

- **AT-I18 `zero_delay_cycles_are_rejected_at_activation`** (UNCHANGED).
- **AT-I19 `declared_fan_out_and_depth_bounds_are_enforced_statically`** (UNCHANGED).
- **AT-I20 `pacing_defers_whole_cohorts_and_preserves_semantics`** (REVISED v2) —
  the due-work set is constructed so a stable-prefix cut **would split co-target
  additive causes and a threshold input set** inside one equal-due-time cohort; with
  `max_due_per_cycle` forcing deferral: only whole cohorts defer, in ascending due-time
  order; the deferred cohort is untouched in the scheduler (digest-checked); over an
  identical canonical input history, deferred-then-completed execution is
  digest-identical to unbudgeted execution across **cells, scheduler, obligation store,
  occurrence ledger, cooldowns, returned reports, and threshold emissions** — including
  the opposing-contributions case where a partial fold would have falsely crossed a
  threshold and enqueued an obligation.
- **AT-I20b `semantic_cap_overflow_is_atomic_and_terminal`** (NEW v2) — a cohort
  exceeding `max_cohort_candidates`/`max_effects_per_wave`: rejects before any canonical
  mutation, then consumes the cohort's `WorkKey`s with a typed order-independent
  overload report (conflicted-drain shape); reschedule under the next occurrence
  succeeds; identical outcome on replay (caps are epoch-bound).
- **AT-I21 `depth_overflow_converts_to_strictly_later_work`** (REVISED v2) —
  propagation pending at `max_wave_depth` converts to scheduled work with
  `due_time ≥ now + 1`, visibly reported, never dropped, and **provably not executed
  within the same barrier** (single-drain rule: the barrier's drain set is pinned and
  the converted key is absent from it). In **isolation** (no interfering work), the
  converged state is digest-identical to the same rules under a larger depth budget.
  With **interfering due work** that changes the deferred rule's input, the test
  asserts determinism, continuation priority via `WorkKey` due-time ordering, and exact
  boundary identity — not cross-budget digest equality (adjudicated: depth budgets are
  epoch-bound declared behavior; freeze §2 C-11).

## Group 9 — Decay/recovery and aggregation (freeze §4.3; v1 §Q7/§7)

- **AT-I22 `decay_catch_up_is_chunk_invariant`** (REVISED v2) — as v1 (bit-equal
  chunk invariance, baseline convergence, exact clamping), plus: exact formula/rounding
  vectors pinned as expected values, and a scheduled decay evaluation **coincident with
  an independent `AddDelta` shock on the same target in one wave** asserts the frozen
  cross-family rejection — and, in companion fixtures, that the two sanctioned
  compositions (one rule body combining both; separate scheduled boundaries) produce
  their declared deterministic results.
- **AT-I23 `recovery_and_decay_respect_epoch_bound_rates`** (UNCHANGED).
- **AT-I24 `aggregation_is_permutation_and_path_invariant`** (REVISED v2) — as v1,
  plus: child contributions of **equal magnitude** remain distinct reducer inputs
  (never coincidence-deduplicated), full recomputation matches every permitted
  evaluation partition, and activation rejects a second aggregation rule targeting the
  same aggregate definition (one authoritative reducer).
- **AT-I25 `threshold_emission_uses_committed_state_after_complete_reduction`**
  (REVISED v2) — as v1, plus **multiple simultaneous same-target causes**: the crossing
  is computed exactly once from (previously committed value → fully reduced candidate
  result); no partial-fold false emission is possible — pinned with opposing
  contributions whose partial fold crosses the threshold but whose complete fold does
  not, and vice versa; crossing emissions appear only in wave N+1 or as strictly later
  scheduled work.

## Group 10 — Hidden-state / equal-digest equivalence (freeze §12; v1 §8)

For **each** of: EpochRegistry, **ObligationStore including its contested claim-set
state**, OccurrenceLedger, CooldownLedger, and any derived index/cache the
implementation introduces:

- **AT-I26 `<store>_discrimination`** (REVISED v2) — as v1; for the ObligationStore
  this explicitly includes states differing only in **tracked contested claims** under
  one work identity and only in coverage metadata retained in obligation records.
- **AT-I27 `<store>_equal_digest_same_next_input`** (REVISED v2) — as v1 (genuine
  counterexample attempts, mid-sequence interleavings, cap boundaries, reset/epoch
  crossings), extended across contested-claim permutations and claim-set cap edges.
- **AT-I28 `<derived index>_recomputation_invariant`** (UNCHANGED).
- **AT-I29 `wave_buffers_do_not_outlive_the_wave`** (REVISED v2) — as v1, with the
  transient set now explicitly including the candidate multiset, emission-identity
  canonicalization state, reduction groups, and the preflighted transition — after any
  committed or rejected wave, fresh reconstruction from committed inputs is
  digest-identical and next-barrier-identical.

## Group 11 — Global/dormant boundedness (UNCHANGED)

- **AT-I30 `work_scales_with_due_keys_not_scope_count`** (UNCHANGED).
- **AT-I31 `dormant_aggregates_update_only_at_their_cadence`** (UNCHANGED).

## Group 12 — Panic freedom and gate-level requirements (UNCHANGED discipline)

- **AT-I32 — strict lint gate extended** (UNCHANGED, applies to every new module
  including the candidate/reduction/preflight machinery; compile probes extend to any
  new must-not-compose surface, including forging emission identities or claim-set
  contents from outside the door).
- **AT-I33 — adversarial input totality** (REVISED v2) — the sweep additionally covers
  maximum candidate volumes at cohort caps, maximal same-target groups, `i64::MIN`
  subtraction normalization, and full-range fold boundaries; no panic anywhere; every
  failure typed.

## Group 13 — Phase-1 regression gates (UNCHANGED and mandatory)

- **AT-I34 — the complete inherited suite** (UNCHANGED; 232 tests + AT-I additions,
  nothing removed).
- **AT-I35 — Phase-1 digest stability** (UNCHANGED; freeze v2 changes no Phase-1
  canonical encoding — `StateCell`, manifests, configs, scheduler, timeline all
  bit-identical; the coverage block lives only in new Phase-2 records and presentation
  values).
- **AT-I36 — platform static checks** (UNCHANGED).

## Group 14 — Provenance coverage (freeze §10)

- **AT-I38 `provenance_cap_exposes_honest_coverage`** (NEW v2) — cause sets sharing the
  same retained smallest emission identities but differing beyond `MAX_SOURCE_REFS`:
  the committed value reflects **every** contribution (the fold never truncates —
  asserted by exact arithmetic), the coverage block reports
  `truncated`/retained/omitted exactly under `provenance_prune.v1`, `complete` appears
  iff nothing was omitted, `unknown` appears only for inherited prior-state provenance,
  and no hidden behavior divergence exists between capped and uncapped cause sets with
  equal retained prefixes (equal-digest/same-next-input probe).

## Explicitly unchanged tests (conformance checklist for the reviewer)

The v1 table stands: AT-H suite and timeline corpus, scheduler suite (AT-B, AT-G
scheduler half — `Scheduler` consumed, not modified; no batch API), activation ceremony
and compile probes, AT-G hidden-evidence guards, manifest/config hash pins.

## Out of scope

As v1: no persistence/service/protocol tests, no profile parsing/IO, no executable
Windows/Android runs, no Phase-4+ semantics, no MCI/game integration, no runtime-LLM
surface. Phase 3 is not authorized and nothing here pre-authorizes it.
