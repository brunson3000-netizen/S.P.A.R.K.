# S.P.A.R.K. Phase 2 — Acceptance Test Matrix (AT-I)

**Date:** 2026-08-26
**Companion to:** `SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_2026-08-26.md` (controlling
design; section references below are to that document unless noted).
**Discipline:** identical to AT-A…AT-H. Every AT-I test is encoded **red-first**: tests
for a subsystem land with the first commit that makes them expressible, are run against
the tree *before* the behavior is implemented, and the red/green split is recorded
verbatim as a baseline evidence file under
`engineering/phase2/<pass>-baseline/` before any production change. Where a test cannot
compile before its types exist, the writer records that as the baseline ("inexpressible
against inherited tree") — never skips the recording. Every "X is impossible" claim is a
digest/value equality or a compile-fail, never an error-was-returned check alone. No
existing test is weakened, deleted, or skipped; the Phase-1 suite runs unmodified
throughout.

Locations: new integration files under `crates/spark-testkit/tests/` (`rule_runtime.rs`,
`effect_batch.rs`, `obligations.rs`, `epochs.rs`), in-crate unit/property tests in
`spark-engine` where private invariants require it (the AT-H3 pattern), and additions to
the external compile-probe harness for every new must-not-compose surface.

---

## Group 1 — Stable-snapshot isolation (freeze §Q2)

- **AT-I1 `evaluation_reads_only_the_pre_wave_snapshot`** — a barrier whose rules both
  read and write overlapping cells: every read that fed any effect must resolve to the
  pre-wave value. Assert by constructing rules whose output value differs depending on
  whether a same-wave write is visible; the committed values must equal the
  hand-computed pure function of the pre-wave snapshot; engine digest after the wave
  equals the digest of the same effects applied to an untouched clone.
- **AT-I2 `mid_wave_state_is_unobservable`** — compile-level: no public or
  `test-support` API exposes a store view during the commit phase; external compile
  probe asserts the evaluator entry point takes/returns by value or `&mut` such that a
  concurrent read cannot compile. Plus behavioral: wave-chained barriers (wave N+1 sees
  exactly wave N's committed state, `wave_index` binding per freeze §Q3).

## Group 2 — Rule-order and insertion-permutation invariance (freeze §Q1, §Q3)

- **AT-I3 `rule_declaration_order_cannot_change_canonical_state`** — one rule set
  declared in multiple permutations activates to the same `ruleset_content_hash`
  (multiset function, the m-02 lesson applied on day one) and produces identical engine
  digests and identical wave reports for the same inputs.
- **AT-I4 `work_insertion_permutation_invariance`** — the same logical claim set of due
  work scheduled in multiple arrival orders: equal scheduler digests before drain, equal
  `DrainOutcome`s, equal post-barrier engine digests (extends AT-B to the full
  evaluator path).

## Group 3 — Deterministic effect ordering (freeze §Q3)

- **AT-I5 `effects_commit_in_frozen_total_order`** — a wave producing effects on many
  `(definition, scope)` targets from rules evaluated in different internal orders across
  runs: the sorted commit order `(definition_id, scope_id, path tag)` is asserted
  directly against the wave report, and replaying the whole barrier is digest-identical.
- **AT-I6 `duplicate_effects_deduplicate_with_merged_provenance`** — two rules emitting
  the same target+value: exactly one committed write; provenance is the ascending
  bounded set; digest equality across rule-order permutations.

## Group 4 — Conflicting effects and atomic rejection (freeze §Q3)

- **AT-I7 `distinct_value_conflict_rejects_the_whole_wave_atomically`** — two rules
  writing different values to one target: engine digest after the rejected wave equals
  the pre-wave digest exactly (nothing committed — cells, obligations, ledgers, epoch
  registry all untouched); the report carries order-independent evidence; all rule-order
  permutations produce the identical report value.
- **AT-I8 `invalid_effect_rejects_the_whole_wave_atomically`** — a batch mixing valid
  effects with one authority/type/bounds/scope-invalid effect: pre-wave digest
  preserved; no partial commit in any permutation.
- **AT-I9 `rejected_wave_is_reproducible`** — re-evaluating the same barrier against the
  unchanged snapshot deterministically reproduces the same rejection report (no retained
  rejection state; the Chronicle boundary holds).

## Group 5 — Authority and bounds violations (ADR-0002; freeze §3.1)

- **AT-I10 `rule_targeting_host_owned_is_rejected_at_activation`** — rule-set activation
  fails atomically (no partial `ActivatedRuleSet`, lineage unchanged) for a rule whose
  effect targets a `host_owned` definition; compile probes keep proving no public write
  path exists around the door.
- **AT-I11 `evaluator_writes_respect_declared_write_classes`** — spark-owned targets
  commit via the spark-effect path, derived via commit-derived, cross-path attempts are
  unrepresentable in the effect type (compile-fail) or rejected (typed error) —
  whichever the implementation chooses, the test pins it as digest-unchanged rejection.
  Bounds: boundary values (min, max, min−1, max+1, `i64::MIN/MAX`, `FIXED_SCALE` edges)
  per AT-I8's atomicity.

## Group 6 — Delayed-obligation identity and scheduler collisions (freeze §5)

- **AT-I12 `obligation_payload_hash_is_the_record_hash`** — for every scheduled
  obligation, `WorkPayload.canonical_payload_hash` equals the canonical hash of the
  ObligationStore record; the AT-H3-style bidirectional invariant (scheduler slots ↔
  obligation records) is recomputed after every prefix of a scripted
  schedule/drain/poison/reject sequence.
- **AT-I13 `materialized_mode_refuses_fingerprint_mismatch`** — schedule a materialized
  obligation, activate a new behavior epoch whose target definition fingerprint differs,
  drain: explicit typed refusal, nothing committed, digest visibly records the outcome;
  the same obligation under an epoch with matching fingerprints executes the frozen
  effects bit-identically to their scheduling-time resolution.
- **AT-I14 `reevaluation_mode_requires_the_exact_rule_artifact`** — rule-re-evaluation
  obligations execute only when the exact `ruleset_content_hash`/rule fingerprint
  resolves in the lineage; a superseded rule with the same ID refuses explicitly; modes
  cannot switch (ADR-0006 verification 1/4).
- **AT-I15 `obligation_collisions_poison_order_independently`** — same `WorkKey`,
  distinct record hashes, all arrival permutations: equal scheduler digests, poisoned
  key, drain reports the conflict, both obligation records leave the store with it,
  post-drain reschedule under the next occurrence succeeds (extends AT-B5 through the
  ObligationStore).

## Group 7 — Occurrence indexes (freeze §6)

- **AT-I16 `occurrence_ledger_is_committed_and_order_independent`** — permuted
  interleavings of work for independent (producer, scope) pairs allocate identical
  per-pair sequences and identical engine digests; ledger discrimination/equivalence per
  Group 10; `u64` exhaustion rejects the wave atomically (AT-I8 discipline).
- **AT-I17 `per_gate_random_addresses_are_distinct_and_stable`** — two probability gates
  in one rule at one occurrence draw under distinct qualified IDs; permuting unrelated
  work does not move any draw; the same address always yields the same draw (extends
  M-04 into the rule layer).

## Group 8 — Cycle rejection, fan-out/depth, queue/backpressure (freeze §Q4)

- **AT-I18 `zero_delay_cycles_are_rejected_at_activation`** — direct (A→A), mutual
  (A→B→A), and long-loop zero-delay cycles all reject atomically with the cycle named;
  the same graphs with one declared delay edge activate successfully.
- **AT-I19 `declared_fan_out_and_depth_bounds_are_enforced_statically`** — a graph
  exceeding declared per-rule fan-out or `max_wave_depth` chain depth rejects at
  activation; the boundary case (exactly at the bound) activates.
- **AT-I20 `work_volume_overflow_defers_the_stable_prefix_remainder`** — more due work
  than `max_due_per_cycle`: exactly the stable-`WorkKey`-order prefix executes; the
  remainder is untouched in the scheduler (digest-checked); the deferral is reported;
  draining again completes it; the two-step total equals the unbudgeted single-step
  result digest.
- **AT-I21 `depth_overflow_converts_to_scheduled_work`** — propagation still pending at
  `max_wave_depth` becomes scheduled work at the next due boundary, visibly reported,
  never dropped and never run zero-delay; final converged state (after the follow-up
  boundary) is digest-identical to the same rules under a larger depth budget.

## Group 9 — Decay/recovery and aggregation (freeze §Q7, §7)

- **AT-I22 `decay_catch_up_is_chunk_invariant`** — N missed cadence steps applied as one
  catch-up evaluation vs N separate evaluations vs two uneven chunks: identical cell
  values and engine digests; convergence stops exactly at `baseline`; clamping at
  declared bounds is exact (no float, no drift — assert bit equality, not tolerance).
- **AT-I23 `recovery_and_decay_respect_epoch_bound_rates`** — a hot-tuned rate under a
  new behavior epoch applies only from that epoch's barrier; pre-barrier evaluations use
  the old rate (epoch binding, freeze §Q5).
- **AT-I24 `aggregation_is_permutation_and_path_invariant`** — the same child-cell
  multiset written in different orders, and full-recompute vs any staged evaluation
  path: identical aggregate cell value and digest; weighted-sum overflow at `i128`
  boundary rejects per AT-I8; floor rounding pinned by exact expected values.
- **AT-I25 `threshold_emission_uses_committed_state_only`** — crossing detection
  compares against the previously committed aggregate: a rising-then-falling sequence
  across barriers emits per declaration; re-running any barrier reproduces its emission
  exactly; no hidden last-value memory exists (equal-digest probe per Group 10).

## Group 10 — Hidden-state / equal-digest equivalence (freeze §8 — every retained store)

For **each** of: EpochRegistry, ObligationStore, OccurrenceLedger, CooldownLedger, any
derived index/cache the implementation introduces:

- **AT-I26 `<store>_discrimination`** — two states differing only in that store's
  retained content have different engine digests.
- **AT-I27 `<store>_equal_digest_same_next_input`** — states reached by permuted
  construction orders with equal digests return identical whole result values and
  identical post-input digests for the same later input (the AT-G falsification, applied
  from day one; the writer must attempt genuine counterexamples, not only happy pairs —
  include mid-sequence interleavings, cap boundaries, and reset/epoch crossings).
- **AT-I28 `<derived index>_recomputation_invariant`** — after every prefix of a
  scripted operation sequence, the index equals the value recomputed from committed
  state (the AT-H3 pattern).
- **AT-I29 `wave_buffers_do_not_outlive_the_wave`** — after any wave (committed or
  rejected), constructing the same engine state fresh from the committed inputs is
  digest-identical and behaves identically on the next barrier.

## Group 11 — Global/dormant boundedness (freeze §7; blueprint §27.2)

- **AT-I30 `work_scales_with_due_keys_not_scope_count`** — instantiate many dormant
  scopes with no due work: drain and barrier cost touch zero of them (assert via
  wave-report work counts, not timing); adding dormant scopes leaves the engine digest
  path of active work unchanged.
- **AT-I31 `dormant_aggregates_update_only_at_their_cadence`** — dormant-tier
  aggregation fires exactly at scheduled occurrences; promotion reads only committed
  aggregates (no synthesized microhistory — assert the promoted state is a function of
  committed values, ADR-0006 verification 8).

## Group 12 — Panic freedom and gate-level requirements

- **AT-I32 — strict lint gate extended.** The deny set (`unwrap_used`, `expect_used`,
  `panic`, `indexing_slicing`, `arithmetic_side_effects`) applies to every new module;
  `cargo fmt --check`; `cargo clippy --workspace --all-targets [--all-features]
  -- -D warnings`; metadata/dependency-direction policy tests; the external compile
  harness gains probes for every new must-not-compose surface (no public evaluator
  constructor accepting caller-authored schema/rule facts, no public write path, no
  ObligationStore/ledger forgery) each failing for the right reason.
- **AT-I33 — adversarial input totality.** Property-style sweeps of extreme values
  (`i64::MIN/MAX`, `u64::MAX` occurrences/times, maximum-length IDs, empty and
  maximum-size rule sets, budget boundaries) through activation, evaluation, and commit:
  no panic anywhere; every failure is a typed error; the process-level result is a
  passing test, not a caught unwind.

## Group 13 — Phase-1 regression gates (unchanged and mandatory)

- **AT-I34 — the complete inherited suite.** All 232 Phase-1 tests (AT-A…AT-H, corpus
  1–20, compile probes) pass unmodified; only additive edits to existing files are
  permitted. Expected count: 232 + AT-I additions, nothing removed.
- **AT-I35 — Phase-1 digest stability.** The pinned manifest/config hash values and the
  scheduler/timeline canonical encodings are bit-identical before and after every
  Phase-2 pass (freeze §Q1/§Q7 promise there is no encoding change; verified by the
  existing pinned-hash tests plus a pre/post canonical-value probe in a clean worktree,
  as in the closure pass).
- **AT-I36 — platform static checks.** All five Windows/Android `cargo check` targets
  (plain and `--all-targets`) stay green; executable parity remains explicitly
  out of scope and is restated as debt, not silently claimed.

## Explicitly unchanged tests (conformance checklist for the reviewer)

| Existing surface | Why it must stay green unmodified |
|---|---|
| AT-H suite and timeline admission/fence corpus | Phase 2 adds no ingress semantics; epoch-activation commands use the existing envelope/fence machinery |
| Scheduler suite (AT-B, AT-G scheduler half) | `Scheduler` is consumed, not modified; no batch API lands (freeze §Q6) |
| Activation ceremony and compile-probe suite | the rule-set door extends the registry by addition; the definition door is untouched |
| AT-G hidden-evidence guards | `BoundedClaimSet` policy constants and encodings unchanged |
| Manifest/config hash pins | encodings frozen; rule sets are a separate artifact |

## Out of scope for this matrix

No persistence/service/protocol tests; no profile parsing/IO; no executable
Windows/Android runs; no actor-behavior (Phase 4+) semantics; no MCI/game integration;
no runtime-LLM surface (none exists). Phase 3 is not authorized and nothing here
pre-authorizes it.
