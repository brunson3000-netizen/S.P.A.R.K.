# S.P.A.R.K. Phase 1 — Re-Foundation Implementation Report

**Date:** 2026-08-26
**Writer:** Claude Code — Opus — HIGH effort
**Controlling documents:** `CONTROLLING_BLUEPRINT_v0.2.md`, ADR-0001…ADR-0006, `PHASE_1_IMPLEMENTATION_BRIEF.md`, `PHASE_1_REFOUNDATION_BRIEF_v0.1.md`, `PHASE_1_CONVERGENCE_DECISION.md`
**Independent review evidence treated as authoritative:** `PHASE_1_CODEX_INDEPENDENT_REVIEW_2026-08-26.md`, `PHASE_1_CODEX_CORRECTION_REREVIEW_2026-08-26.md`

`PHASE_2_AUTHORIZATION: NO`

---

## 1. Branch and commits

**Branch:** `phase1-refoundation`, created from the canonical repository state at `1ac3e3b` ("Declare Phase 1 convergence stop and stage re-foundation").

No historical commit was rewritten. The failed implementation line, both Codex reviews, both Claude reports, and all Phase-0 evidence remain exactly as they were on `master`.

| Commit | Purpose |
| --- | --- |
| `6597b4a` | Step 2 — encode the adversarial failure corpus and record the baseline against the inherited implementation |
| `845e7ca` | Step 3/4 — re-found the affected foundation boundaries and port the full regression corpus |
| _(this report)_ | Step 6 — completion report |

Final working tree: clean.

---

## 2. Failing baseline before the rewrite

Fifteen counterexamples from `PHASE_1_REFOUNDATION_BRIEF_v0.1.md` were written as executable tests **from an external crate** (`spark-testkit`, i.e. an integrator's vantage point) and run against the inherited implementation at `1ac3e3b`, before any implementation was replaced.

```text
cargo test -p spark-testkit --test refoundation_adversarial_baseline
test result: FAILED. 0 passed; 15 failed
```

Raw output: `engineering/phase1/refoundation-baseline/BASELINE_ADVERSARIAL_FAILURES.txt`.

| # | Baseline test | Finding | Observed inherited behavior |
| --- | --- | --- | --- |
| 1 | `baseline_admission_timing_must_not_change_canonical_state_digest` | B-01 / M-02 | Same semantic commands, same fence chain, different digests: `f481d275…` vs `34075bbd…` |
| 2 | `baseline_finalized_history_must_not_retain_admission_token` | B-01 | Finalized `CommandEnvelope` retained `admission_window_token` `503b707a…` vs `977b9a9f…` |
| 3 | `baseline_external_code_must_not_forge_and_activate_a_schema` | B-02 | Fabricated `Derived` schema with `Digest::ZERO` installed and readable via `schema_of` |
| 4 | `baseline_duplicate_schema_keys_must_reject` | B-02 | `StateStore::new` silently resolved a duplicate key by last-writer-wins |
| 5 | `baseline_definition_spec_must_not_self_activate` | B-02 | `DefinitionSpec::to_definition_schema` promoted an unvalidated spec into activated authority |
| 6 | `baseline_independent_producers_may_share_an_occurrence_index` | B-03 | Two producers at occurrence 0 collided |
| 7 | `baseline_reversed_insertion_of_independent_producers_drains_identically` | B-03 | First arrival selected which logical work survived |
| 8 | `baseline_independent_scopes_may_share_an_occurrence_index` | B-03 | Two scopes at occurrence 0 collided |
| 9 | `baseline_duplicate_config_keys_must_reject` | M-01 | Duplicate-key config was hashable and order-sensitive |
| 10 | `baseline_oversized_config_value_must_reject` | M-01 | A 100 000-character categorical config value was constructible |
| 11 | `baseline_definition_id_must_require_a_namespace` | M-03 | `DefinitionId::new("curiosity")` accepted |
| 12 | `baseline_incoherent_value_bounds_must_reject` | M-03 | `Int { min: 10, max: 0 }` constructible and accepted by the validator |
| 13 | `baseline_oversized_custom_definition_kind_must_reject` | M-03 | 100 000-character `DefinitionKind::Custom` passed validation |
| 14 | `baseline_oversized_command_kind_must_reject` | M-03 | 100 000-character `command_kind` admitted into canonical staging |
| 15 | `baseline_oversized_categorical_value_must_reject` | M-03 | Unbounded `CanonicalValue::Categorical` constructible |

Six further requirements could not be expressed as a *runtime* assertion against the inherited API because the API surface they require did not exist. Their baseline is "does not compile: required API absent", which is a stronger failure, not a weaker one. Recorded in `engineering/phase1/refoundation-baseline/BASELINE_UNEXPRESSIBLE_FINDINGS.md`: structured `StageAcknowledgement`; structured finalization result; the structural (type-level) half of schema provenance and post-activation immutability; timeline window arithmetic panic-freedom and atomic overflow (unreachable from outside the crate because no constructor could place the frontier near `u64::MAX`); and the type-level claim that command/work kinds are bounded canonical types.

The baseline fixture was superseded by the permanent corpus in the same branch; its evidence is preserved in the evidence directory and in commit `6597b4a`.

---

## 3. Module and API replacement summary

### 3.1 Semantic vs admission data — `spark-core/src/timeline.rs`

The inherited design kept one `CommandEnvelope` holding both behavioral identity and `admission_window_token`, then tried to keep the token out of identity by excluding it from one hash function. That is a convention, and it failed twice: the token was still retained in every finalized envelope and still reached the ingress state digest.

The boundary is now a type boundary.

| Before | After |
| --- | --- |
| `CommandEnvelope` (semantic fields + `admission_window_token`) | `SemanticCommandEnvelope` (semantic fields only) |
| — | `AdmissionTicket` — ephemeral credential, **no `canonicalize` method**, never stored |
| — | `SubmittedCommand { semantic, admission }` — exists only for the duration of one `stage` call |
| `semantic_envelope_hash(&envelope)` free function | `SemanticCommandEnvelope::semantic_hash()` |
| `canonical_state_digest()` (mixed) | `canonical_history_digest()` (finalized semantic history) **and** `canonical_state_digest()` (complete observable state) |

Because no admission credential is retained anywhere in the module, admission timing cannot contaminate replay identity — not because a hash function omits it, but because there is nothing to omit.

### 3.2 Structured evidence — `spark-core/src/timeline.rs`

| Before | After |
| --- | --- |
| `stage -> Result<StageOutcome, StageError>` (bare enum) | `stage -> Result<StageDisposition, StageError>` |
| — | `StageDisposition::Acknowledged(StageAcknowledgement)` binding profile, epoch, ordinal, command ID, semantic-envelope hash, slot state, plus a derived `acknowledgement_hash` and a `covers(&envelope)` check |
| — | `StageDisposition::Poisoned(SlotPoisonRecord)` and `StageDisposition::NotInAdmissionWindow(AdmissionRetryAdvice)` |
| `submit_fence -> Result<(), FenceError>` | `submit_fence -> Result<FinalizationResult, FenceError>` identifying fence ID, finalized range, command count, previous/new fence hash, ordered-stream digest, new frontier, and canonical history digest |

`StageAcknowledgement` and `FinalizationResult` have private fields and no public constructor: they are evidence the ingress issued, not values a caller can assert.

### 3.3 Trusted schema activation — new `spark-core/src/activation.rs`, rewritten `spark-core/src/state.rs`

| Before | After |
| --- | --- |
| `state::DefinitionSchema` with all-public fields including `fingerprint` and `authority` | **Removed.** `activation::DefinitionDeclaration` is untrusted and has **no fingerprint field at all** |
| `StateStore::new(impl IntoIterator<Item = DefinitionSchema>)` | **Removed.** `StateStore::from_activated_schema(ActivatedSchema)` is the only constructor |
| — | `activation::ActivatedDefinition` / `ActivatedSchema` — private fields, no public constructor, shared-reference accessors only |
| `spark_profile::identity::DefinitionIdentityRegistry` (fingerprint supplied by caller) | `spark_core::activation::DefinitionIdentityRegistry::activate(...)` — computes the fingerprint itself, rejects duplicate keys, enforces immutable identity, commits atomically, and is the sole producer of `ActivatedSchema` |
| `DefinitionSpec::to_definition_schema()` (public, unvalidated) | **Removed.** `DefinitionSpec::to_declaration()` yields an inert declaration; `DefinitionSpec::definition_fingerprint()` delegates to the kernel so the profile layer cannot compute an identity that disagrees with the enforced one |
| `validate(&manifest, &mut registry) -> Result<(), Vec<ValidationError>>` | `-> Result<ValidatedManifest, Vec<ValidationError>>`; `ValidatedManifest` has private fields and carries the manifest content hash plus the activated schema |

The profile-qualified identity registry was relocated from `spark-profile` into `spark-core`. This is a deliberate boundary decision, not a convenience: immutable authority identity is a canonical kernel invariant (ADR-0002 "authority becomes a lifecycle invariant"; ADR-0004 "Stable IDs and immutable authority"), not a profile file-format concern — and locating it in the kernel is what allows the kernel to structurally refuse to build a `StateStore` from anything that did not pass through it. `spark-profile` still owns manifest types, manifest-level validation, and content hashing, and drives the ceremony. ADR-0001's dependency direction is unchanged and still mechanically enforced.

Also added: `StateStore` is now single-profile and rejects foreign-profile reads and writes explicitly (`StateWriteError::ForeignProfile`), strengthening ADR-0004 profile/trust-domain isolation.

### 3.4 Complete scheduler work key — `spark-core/src/scheduler.rs`

| Before | After |
| --- | --- |
| key `(due_time, occurrence_index)` | `WorkKey { due_time, profile_id, producer_definition_id, scope_id, occurrence_index, work_kind }` |
| `DueWorkItem` with inline fields and `work_kind: String` | `DueWorkItem { key: WorkKey, payload: WorkPayload }` with `work_kind: WorkKind(CanonicalTag)` |
| `ConflictingDuplicateWorkKey { due_time, occurrence_index }` | `ConflictingWorkPayload { key, installed_payload, rejected_payload }` (returned boxed as `ScheduleConflict`) |
| — | `WorkKey::identity_digest()` for binding a persisted delayed obligation to its exact logical identity (ADR-0006) |
| — | `Scheduler::canonical_state_digest()` and `Scheduler::contains(&key)` |

`WorkKey`'s field order is its `Ord` order, so drain order is a stable total order over semantic identity. Occurrence indexes are producer-, scope-, kind-, and profile-local, which is what makes them persistence-friendly.

### 3.5 Validated config — `spark-profile/src/config.rs`

| Before | After |
| --- | --- |
| public `entries: Vec<ConfigEntry>`, freely constructible | private `entries: BTreeMap<DefinitionId, CanonicalValue>` |
| no constructor | `ConfigRevision::build(...) -> Result<Self, ConfigRevisionError>` is the only constructor |
| duplicate keys hashed, order-sensitively | duplicate keys rejected, with the reported key set sorted so the same input is rejected identically in any order |
| `revision_label: String` | `revision_label: BoundedText` |

A unique-key representation is now the *only* representation: insertion order is not ignored, it is not retained.

### 3.6 Bounded canonical construction — `spark-core/src/id.rs`, `value.rs`; `spark-profile/src/text.rs`

| Before | After |
| --- | --- |
| `DefinitionId::new("curiosity")` accepted | `DefinitionId` requires ≥ 2 dot-separated segments per `CONTROLLING_BLUEPRINT_v0.2.md` §12.3; `StableIdError::TooFewSegments` |
| `command_kind: String`, `work_kind: String`, `DefinitionKind::Custom(String)` | all three are `CanonicalTag`-backed (`CommandKind`, `WorkKind`, `DefinitionKind::Custom(CanonicalTag)`), bounded at `MAX_CANONICAL_TAG_LEN = 64` |
| — | `CanonicalTag::from_static` — a `const fn` whose validation runs at compile time, so the engine's baseline vocabulary needs no fallible or panicking path at any use site |
| public-variant `ValueConstraint` enum | opaque `ValueConstraint` with validating constructors (`int`, `fixed`, `categorical`, `boolean`, `reference`); `min > max` is inexpressible |
| `CanonicalValue::Categorical(String)` | `CanonicalValue::Categorical(CategoricalValue)` — bounded at 256 chars, control characters rejected |
| `description: String`, `version_label: String` | `spark_profile::text::BoundedText` — bounded at 1024 chars, control characters rejected |
| `domain: Option<String>`, `layer: Option<String>` | `Option<CanonicalTag>` |

`ProfileId`, `SourceId`, `CommandId`, and `FenceId` deliberately keep the single-segment rule: the blueprint's namespacing contract governs the *definition* vocabulary, and imposing it elsewhere would be an invented constraint rather than an enforced one. Each newtype declares its own `MIN_SEGMENTS`, and a test asserts both the requirement and its scope.

### 3.7 Total ordinal arithmetic — `spark-core/src/timeline.rs`

| Before | After |
| --- | --- |
| `assert!(window_width >= 1)` in the constructor | `TimelineIngress::new(...) -> Result<Self, TimelineConfigError>` |
| `window_end()` panicked via `.expect("timeline window_end arithmetic overflow")` | returns `Result<Ordinal, TimelineOrdinalSpaceExhausted>`; `current_admission_window()` propagates it |
| `submit_fence` promoted commands/fence state, *then* checked `end_ordinal + 1` | the frontier advance is validated **before** anything is touched, so exhaustion rejects the fence atomically |
| overflow unreachable from outside the crate | `TimelineIngress::resume_at_frontier(...)` makes a non-zero starting frontier expressible, which is what makes both properties falsifiable |

### 3.8 Test harness — `spark-testkit/src/scenario.rs`

`ScenarioReplay` now carries three digests (`history_digest`, `ingress_state_digest`, `transcript_digest`), the transcript records structured acknowledgement/finalization evidence rather than a coarse enum tag, and `run_scenario` returns `Result<_, ScenarioError>` instead of panicking on malformed fixture input — a harness that panics on bad input is a harness that can mask a real regression behind an unrelated crash.

---

## 4. Final test results

```text
cargo test --workspace   ->  172 passed; 0 failed; 0 ignored
```

| Suite | Tests | Result |
| --- | --- | --- |
| `spark-core` unit | 70 | ok |
| `spark-core` integration `tests/timeline_admission.rs` | 24 | ok |
| `spark-profile` unit | 26 | ok |
| `spark-testkit` unit | 7 | ok |
| `spark-testkit` integration `tests/refoundation_adversarial.rs` | 27 | ok |
| `spark-testkit` integration `tests/workspace_dependency_direction.rs` | 2 | ok |
| `spark-core` doc-tests (10 of which are `compile_fail`) | 13 | ok |
| `spark-profile` doc-tests (all 3 `compile_fail`) | 3 | ok |
| `spark-testkit` doc-tests | 0 | ok |
| **Total** | **172** | **ok** |

Baseline for comparison: the inherited implementation had 85 tests.

The `compile_fail` doc-tests are the mechanically-verified half of the structural claims. They compile against the crate as an external consumer would, so a private field or a removed constructor is genuinely enforced by the compiler:

- `state::StateStore` — the raw-schema constructor does not exist
- `activation::ActivatedSchema`, `activation::ActivatedDefinition` — not constructible
- `timeline` module — `AdmissionTicket` has no canonical encoding
- `timeline::StageAcknowledgement` — not constructible
- `value::ValueConstraint` — the public-variant form is gone; `Int { min: 10, max: 0 }` is not nameable
- `value::CategoricalValue`, `id::CanonicalTag`, `profile::text::BoundedText` — unbounded values not constructible
- `id::CanonicalTag::from_static("Trigger")` — invalid literal is a **compile error**
- `config::ConfigRevision`, `validate::ValidatedManifest` — not constructible outside their validation paths

Every original Phase-1 corpus item (implementation brief §5 items 1–20) is still proved. `tests/timeline_admission.rs` was ported to the new API without weakening any assertion, and gained four tests (fence horizon, fence start-at-frontier, out-of-window non-disturbance, poisoned-slot non-finalizability). No architectural test was deleted or weakened for API convenience.

---

## 5. Disposition of every open prior finding

| Finding | Prior status | Disposition | Evidence |
| --- | --- | --- | --- |
| **B-01** canonical identity / finality | OPEN | **Addressed.** Admission data is a separate, never-stored type with no canonical encoding; structured `StageAcknowledgement` and `FinalizationResult` replace the bare enum and unit result | `admission_timing_does_not_change_canonical_identity`, `admission_credential_is_not_part_of_finalized_identity`, `stage_acknowledgement_binds_and_proves_its_envelope`, `finalization_returns_structured_evidence`, `distinct_semantic_fields_remain_canonical_identity`, `epoch_handoff_is_authenticated_monotonic_and_history_preserving`, plus the `AdmissionTicket` `compile_fail` doc-test |
| **B-02** authority / schema provenance | OPEN | **Addressed.** `DefinitionSchema` and `StateStore::new` removed; `ActivatedSchema` is unconstructible outside `DefinitionIdentityRegistry::activate`, which computes fingerprints itself | `state_store_authority_derives_only_from_a_validated_activation`, `duplicate_schema_keys_reject_and_commit_nothing`, `activated_identity_is_carried_from_validation_not_from_the_caller`, `post_activation_schema_mutation_is_unavailable`, `definition_spec_cannot_self_activate`, plus `ActivatedSchema`/`ActivatedDefinition`/`StateStore` `compile_fail` doc-tests |
| **B-03** scheduler semantic identity | OPEN | **Addressed.** `WorkKey` carries the complete semantic identity | `independent_producers_may_schedule_the_same_occurrence_index`, `reversed_insertion_drains_identically`, `same_key_is_idempotent_or_conflicts_deterministically`, `drain_order_is_independent_of_call_order`, `occurrence_identity_is_producer_and_scope_local` |
| **B-04** definition fingerprint / atomic validation | CLOSED | **Preserved after inspection**, and strengthened: the fingerprint is now computed by the kernel from validated fields rather than supplied, and atomic candidate validation moved with it | `every_immutable_identity_field_changes_the_fingerprint`, `failed_batch_leaves_registry_unchanged_and_retry_succeeds`, `builtin_and_custom_kinds_with_the_same_spelling_are_distinct`, `same_definition_id_in_independent_profiles_does_not_conflict`, and the ported `spark-profile` validation suite |
| **M-01** config revision validation | OPEN | **Addressed.** `ConfigRevision::build` is the only constructor; unique-key representation | `duplicate_config_keys_reject_in_any_order`, `oversized_config_value_is_rejected_before_hashability`, `config_identity_is_content_and_profile_never_order_or_label` |
| **M-02** replay equivalence | OPEN | **Addressed.** Replay identity is `canonical_history_digest`, which contains only finalized semantic history; the fuller `canonical_state_digest` still distinguishes staged/poisoned state | `admission_timing_does_not_change_canonical_identity`, `empty_scenario_differs_from_one_staged_unfinalized_command`, `poisoned_staging_differs_from_clean_staging`, `same_scenario_replayed_many_times_produces_identical_digest` (50 repeats) |
| **M-03** malformed canonical construction | OPEN | **Addressed.** Namespaced `DefinitionId`; bounded `CanonicalTag`/`CategoricalValue`/`BoundedText`; opaque coherent `ValueConstraint`; total ordinal arithmetic; atomic overflow handling | `definition_id_requires_a_namespace`, `incoherent_bounds_reject`, `custom_definition_kinds_are_bounded_and_validated`, `command_and_work_kinds_are_bounded_canonical_types`, `timeline_window_arithmetic_is_total`, `fence_finalization_is_atomic_when_the_frontier_would_overflow` |
| **M-04** profile/artifact-qualified random address | CLOSED | **Preserved after inspection.** Unchanged apart from removing an infallible-but-panicking slice conversion in `derive_u64` | `same_ids_and_epoch_in_different_profiles_are_independent`, `same_profile_different_behavior_artifact_hash_is_independent`, `call_order_does_not_affect_result_for_unrelated_addresses` |
| **m-01** dependency direction | CLOSED | **Preserved after inspection.** Resolved graph unchanged | `every_crate_resolved_dependencies_respect_adr_0001_direction`, `spark_core_resolves_zero_dependencies_on_any_product_layer_crate` |

### Closed-item regression results

| Closed item | Result |
| --- | --- |
| B-04 full immutable definition fingerprint + atomic candidate validation | PASS — no regression; fingerprint provenance strengthened |
| M-04 random address includes profile + behavior artifact | PASS — no regression |
| m-01 dependency direction | PASS — resolved graph is `spark-core -> blake3`; `spark-profile -> spark-core`; `spark-testkit -> spark-core + spark-profile` (+ dev-only `serde_json`). `spark-core` has no product-layer dependency |
| No unsafe canonical Rust | PASS — `#![forbid(unsafe_code)]` on all three crates; audit finds no `unsafe` |
| No Phase-2 scope | PASS — audit finds no propagation/effect runtime, transport, persistence backend, actor behavior, MCI/game adapter, dialogue/voice, broad catalogs, scripting/plugin runtime, or runtime LLM |
| Windows/Android claims remain static checks only | PASS — see §6; no execution is claimed |

---

## 6. Completion gate and cross-target checks

| Command | Result |
| --- | --- |
| `cargo fmt --check` | PASS, exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS, exit 0, zero warnings |
| `cargo test --workspace` | PASS, exit 0, 172 passed / 0 failed |
| `cargo metadata --format-version 1` | PASS, exit 0 |
| `git status --short` | clean |

### Installed-target static checks

Run without operator prompting, on the targets already installed in this toolchain:

| Target | `cargo check --workspace --target …` |
| --- | --- |
| `x86_64-pc-windows-gnu` | PASS, exit 0 |
| `x86_64-pc-windows-msvc` | PASS, exit 0 |
| `aarch64-linux-android` | PASS, exit 0 |
| `x86_64-linux-android` | PASS, exit 0 |
| `armv7-linux-androideabi` | PASS, exit 0 |

**These are compile checks only.** No Windows or Android binary was linked or executed, no fixture was replayed on either platform, and no claim of Windows or Android determinism, execution, or cross-platform replay equivalence is made. The executable portability gate remains outstanding, as Phase 0 and the implementation brief require.

### Determinism audit

No `unsafe`; no wall-clock API (`SystemTime`, `Instant`, `std::time`); no ambient or global RNG; no floating-point in canonical arithmetic; no `HashMap`/`HashSet` in any canonical path (the only occurrences are in the dependency-policy test's JSON parsing); all canonical iteration is over `BTreeMap`/`BTreeSet`/`Vec` in stable order.

No panic path remains in non-test library code: every `unwrap`/`expect`/`panic!`/`assert!` outside `#[cfg(test)]` was removed. The one remaining `assert!` is inside `CanonicalTag::from_static`, which is a `const fn` compile-time assertion — in a `const` context an invalid literal is a build failure. Its non-`const` caveat is documented on the item itself, and `CanonicalTag::new` is the validated path for any non-literal value. A test asserts that the `const` and runtime validators accept exactly the same language, so a constant can never encode a tag the runtime path would refuse.

---

## 7. Deviations and judgement calls

No conflict with a frozen Phase-0 contract was found, no new causal primitive was needed, no paid/cloud/credentialed resource was used, and no destructive action was taken. Three decisions are worth flagging explicitly for the independent reviewer:

1. **The definition-identity registry moved from `spark-profile` into `spark-core`.** Rationale in §3.3. This relocates an independently *closed* item (B-04), which the mission permits only after inspection; the item was inspected, its semantics are preserved, and its test coverage was ported and extended. ADR-0001's dependency direction is unaffected and still mechanically enforced. If the reviewer considers the registry a profile-layer concern, the alternative is to leave `ActivatedSchema` mintable by any caller, which is the defect B-02 describes.

2. **`DefinitionIdentityRegistry::activate` is a public method.** It has to be — it is the validation path. What changed is that it is now the *only* path: a caller cannot skip it, cannot assert a fingerprint, cannot install a duplicate key, and cannot mutate the result. The structural claim is "no `ActivatedSchema` exists that did not pass this ceremony", and that claim is enforced by private fields plus `compile_fail` doc-tests, not by documentation.

3. **`TimelineIngress::resume_at_frontier` is new API.** It exists because the ordinal-space-exhaustion properties (`min > max` of the timeline world) were literally untestable otherwise — no inherited constructor could place the frontier near `u64::MAX`, which is why the inherited `expect` panic and the non-atomic fence overflow were unreachable rather than absent. It is a pure canonical constructor within the authorized "canonical timeline ingress skeleton" scope and implies no persistence backend.

One item is a genuine, disclosed limitation rather than a deviation: **the Windows and Android results in §6 are static compile checks only.** Nothing in this report claims platform execution.

---

## 8. Phase-2 authorization

`PHASE_2_AUTHORIZATION: NO`

Phase 2 remains unauthorized. This re-foundation must receive independent Codex review before any propagation/effect runtime, service transport, persistence backend, actor behavior, adapter, dialogue/voice, catalog, scripting runtime, or runtime LLM work begins.

Per `PHASE_1_CONVERGENCE_DECISION.md`, if the independent re-review finds the same foundational defect classes again after this re-foundation, Phase 1 escalates to operator architecture/process review rather than another implementation attempt.
