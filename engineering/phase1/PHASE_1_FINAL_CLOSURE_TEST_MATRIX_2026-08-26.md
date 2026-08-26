# S.P.A.R.K. Phase 1 — Final Closure Test Matrix (AT-H)

**Date:** 2026-08-26
**Companion to:** `PHASE_1_FABLE_FINAL_ADMISSION_ARCHITECTURE_2026-08-26.md` (controlling design)
**Discipline:** identical to AT-A…AT-G — every AT-H test is encoded against the **inherited tree first**, the red/green split is recorded verbatim as a baseline evidence file before any production change, and no existing test is weakened, deleted, or skipped. Every "X is impossible" claim is a digest/value equality or a compile-fail, never an error-was-returned check alone.

Expected red baseline against the current tree: **AT-H1, AT-H2, AT-H4 fail; AT-H5–AT-H9 pass** (AT-H3 is expressible only after the `test-support` seam lands, see below; AT-H10/H11 are gate-level). A different split than predicted must stop the writer and be reported, exactly as in the two prior test-first passes.

---

## AT-H — staged-identity order independence (the B-01 repair)

Location: new file `crates/spark-testkit/tests/final_closure.rs` unless noted. Helpers may mirror the overnight probe file recorded in `fable-overnight-baseline/FABLE_OVERNIGHT_PROBE_EVIDENCE_2026-08-26.txt`.

- **AT-H1 `contested_command_identity_is_arrival_order_independent`** — envelopes A (`cmd.a`) and B (`cmd.b`) contest ordinal 0 in both orders. Assert: (a) equal `canonical_state_digest` after the contest (already true — regression guard); (b) submitting the **same** later envelope reusing `cmd.a` with different semantics yields the **same disposition value** in both ingresses; (c) equal `canonical_state_digest` after the reuse; (d) the same holds for a reuse of `cmd.b`. This is probe P1 flipped from falsification to invariant.
- **AT-H2 `contested_source_sequence_is_arrival_order_independent`** — the P2 twin: contest with distinct `(source, seq)` pairs (5 and 6), then reuse pair 5 under a fresh command ID; same four assertions.
- **AT-H3 `staged_registries_are_a_derived_index_over_staged_slots`** — property test through the `test-support` fixture seam (or an in-crate unit test in `timeline.rs`): after every prefix of a scripted operation sequence containing stages, contests, poison pile-ons, fences, and an epoch reset, the two registries equal the index recomputed from the current `Staged` slots (§3.1 invariant, §3.2 lemma). If registry introspection is not exposed even under `test-support`, the writer may instead assert the invariant behaviorally: for each prefix, a probe ingress replaying only the surviving staged envelopes must answer every identity-reuse question identically. Either form is acceptable; the invariant statement is not negotiable.
- **AT-H4 `contested_identities_are_unclaimed_after_the_contest`** — after A/B contest ordinal 0 (either order): reuse of `cmd.a` at ordinal 1 and of `cmd.b` at ordinal 2 by semantically different envelopes are both `Acknowledged(NewlyStaged)`; digests remain equal across the two contest orders throughout. P5 flipped.
- **AT-H5 `poison_evidence_insertion_is_unconditional_and_unscreened`** — regression pin for the load-bearing P3 asymmetry (§4.3 of the architecture): an envelope whose command ID conflicts with a staged command at a *different* ordinal still poisons an occupied slot it targets, its semantic hash **is** recorded in that slot's evidence (`poison_evidence` contains it), and the same claim set in any order produces equal evidence sets and equal state digests. A writer change that screens the poison branch against the registries must turn this red.
- **AT-H6 `finalization_moves_identity_from_staged_to_finalized_registries`** — stage `cmd.0` at ordinal 0, fence it, then: (a) reuse of `cmd.0` (and its `(source, seq)`) by a different envelope is still rejected (permanent registries — unchanged behavior, existing tests timeline_admission.rs:500/:520 also still green); (b) via AT-H3's mechanism, the staged registries no longer contain the promoted entries (S1); (c) an epoch reset after finalization does not resurrect reusability of finalized identities.
- **AT-H7 `epoch_reset_convergence_is_preserved`** — P4 as a permanent guard: after opposite-order contests and an identical epoch reset, both ingresses have equal digests and identical responses to the same post-reset submissions.
- **AT-H8 `three_way_contest_with_reuse_converges_in_all_orders`** — contestants A, B, C on one ordinal in all six orders, **then** the same reuse probes (each contested command ID and source-sequence pair, plus one genuinely fresh identity): all six ingresses agree on state digest, every disposition, and post-reuse digest. Extends AT-H1/H4 past the pair case and catches any residual first/second-arrival asymmetry in the removal logic.
- **AT-H9 `hidden_evidence_guards_stay_green`** — the eight AT-G assertions and AT-F1/AT-F2 rerun untouched (no new code, just the explicit closure claim): full tracked evidence still committed, cap collapse still deliberate, poisoned-state digests still arrival-order-independent.

## Gate-level requirements

- **AT-H10 — no-weakening gate.** All 221 existing tests pass unmodified. The only permitted edit to an existing file's tests is *additive*. `cargo fmt --check`; `cargo clippy --workspace --all-targets [--all-features] -- -D warnings`; the explicit strict canonical lint gate (`unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `arithmetic_side_effects` denied) for `spark-core` and `spark-engine`; `cargo metadata` policy tests; all five Windows/Android static `cargo check` targets. Expected final count: 221 + AT-H additions, nothing removed.
- **AT-H11 `duplicate_id_manifest_hash_is_order_independent`** *(only if the operator bundles the m-02 fix)* — probe P6 flipped: two duplicate-ID manifests with the same definition multiset in different construction orders share one `manifest_content_hash`; plus a regression assertion that all existing manifest-hash tests (valid manifests) are unaffected. If m-02 is deferred, record P6 as accepted debt in the completion report instead.

## Explicitly unchanged tests (conformance checklist for the reviewer)

| Existing test | Why it must stay green unmodified |
|---|---|
| `reused_command_id_for_different_envelope_at_different_ordinal_conflicts` (timeline_admission.rs:500) | Contests against a *positively staged* envelope; registration-while-staged is preserved. |
| `reused_source_sequence_for_different_command_conflicts` (timeline_admission.rs:520) | Same, source-sequence side. |
| All poison-order/evidence tests (AT-F1, AT-G4, poison permutation/cap tests) | The repair never touches evidence insertion or its canonicalization. |
| Fence/finalization suite (24 timeline integration tests) | Finalized registries and history digest are untouched; only the redundant staged copies are cleaned (S1). |
| Scheduler suite in full | The scheduler is not modified at all. |

## Out of scope for this matrix

No Phase-2 evaluator/rule/effect tests; no persistence/service tests; no executable Windows/Android runs (static checks only, debt unchanged); no new public API to cover.
