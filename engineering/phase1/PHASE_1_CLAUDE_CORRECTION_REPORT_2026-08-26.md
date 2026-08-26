# S.P.A.R.K. Phase 1 — Claude Correction Report

**Phase:** 1 — Rust Core Skeleton (bounded correction pass)
**Date:** 2026-08-26
**Writer:** Claude Code (Sonnet 5)
**Controlling architecture:** `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md` plus accepted Phase-0 ADRs/corrections
**Controlling correction brief:** `engineering/phase1/PHASE_1_CORRECTION_BRIEF_v0.1.md`
**Basis:** `engineering/phase1/PHASE_1_CODEX_INDEPENDENT_REVIEW_2026-08-26.md`
**Independent re-review:** Codex, after this correction pass

## 1. Correction verdict

**COMPLETE**, within the bounded scope the correction brief authorizes. All four blockers (B-01 through B-04), all four majors (M-01 through M-04), and the minor (m-01) are closed. No new causal primitive was introduced; no frozen Phase-0 architectural contract was changed. No Phase-2 scope (rule/effect runtime, service transport, persistence backend, actor behavior, MCI/game adapters, dialogue/voice, broad catalogs, scripting/plugin systems, runtime LLM) was implemented.

This correction pass does **not** authorize Phase 2. It is a candidate for independent Codex re-review.

## 2. Exact commit(s)

| Commit | Contents |
|---|---|
| `b744c95` | The correction: all B-01–B-04/M-01–M-04/m-01 code changes, adversarial tests, and dependency updates, on top of `336d4e3` (the original Phase-1 implementation). |
| *(this report's own commit — see `git log` on top of `b744c95`)* | This report, committed immediately after the correction commit, plus the identical `~/Downloads/` transfer copy noted outside the repository. |

`git log --oneline -5` at the time of writing:

```text
b744c95 Correct S.P.A.R.K. Phase 1 per independent Codex review (B-01..B-04, M-01..M-04, m-01)
bd1f858 Authorize bounded S.P.A.R.K. Phase 1 correction
080cf0b docs: record Phase 1 independent Codex review evidence
a962e1a Stage independent Codex review for S.P.A.R.K. Phase 1
cb8001d docs: mark Phase 1 writer complete pending independent review
```

The independent review artifact (`080cf0b`) was not rewritten; it remains evidence. `git status --short` is clean at the time of this correction commit (verified before starting: only the review-prompt/status docs from `bd1f858` were new, no code changes were pending).

## 3. Finding-by-finding disposition

### B-01 — Full canonical envelope identity and finality (CLOSED)

**Root defect:** staging/finality compared only `(command_id, canonical_payload_hash)`, so envelopes that differed in `effective_time`, `source_id`, `source_sequence`, or `command_kind` collapsed into the same identity; `command_id`/`(source_id, source_sequence)` reuse was unvalidated; `reset_epoch` was a public, unauthenticated, unbounded mutation.

**Correction** (`crates/spark-core/src/timeline.rs`):

- Added `semantic_envelope_hash(envelope: &CommandEnvelope) -> Digest`, the single function that now decides "same command" everywhere identity matters. It hashes `profile_id`, `timeline_epoch`, `effective_time`, `source_id`, `source_sequence`, `input_ordinal`, `command_id`, `command_kind`, and `canonical_payload_hash` — deliberately excluding `admission_window_token`, which is admission metadata, not behavior.
- `stage()` now compares the full semantic hash for same-ordinal idempotency/poisoning, and — for a *fresh* ordinal — checks two permanent, never-cleared registries (`finalized_command_identity`, `finalized_source_sequence_identity`) plus two per-epoch in-flight registries (`staged_command_identity`, `staged_source_sequence_identity`) to reject `command_id` or `(source_id, source_sequence)` reuse by a semantically different envelope (`StageError::CommandIdentityConflict` / `SourceSequenceConflict`).
- `FinalizedCommand` now retains the full `CommandEnvelope` (boxed at the `SlotState` level to keep the enum small; unboxed once finalized) plus its semantic hash, instead of only `(command_id, payload_hash)`.
- `compute_ordered_stream_digest` now hashes `(Ordinal, semantic_envelope_hash)` pairs, not `(Ordinal, CommandId, payload_hash)`, so a fence over a behaviorally distinct history can never produce the same digest.
- `submit_fence` now enforces, atomically before any mutation, that finalized `source_sequence` strictly increases per `source_id` (gaps legal, regression rejected: `FenceError::SourceSequenceNotIncreasing`), validated against a running in-fence map seeded from the permanent `last_finalized_source_sequence` baseline.
- `reset_epoch` now takes `requesting_sequencer: &SourceId` and returns `Result<EpochResetRecord, EpochResetError>`: only the currently active sequencer may request it (`NotActiveSequencer`), and `new_epoch` must be strictly greater than the current epoch (`EpochNotIncreasing`). `EpochResetRecord` now also carries `new_sequencer` for full evidence.
- `canonical_state_digest` was rewritten under M-02 (below) to include full envelope content, closing the identity-collapse the reviewer used to falsify the old digest.

**Falsification tests added** (`crates/spark-core/tests/timeline_admission.rs`): `changed_effective_time_is_not_idempotent`, `changed_source_id_is_not_idempotent`, `changed_source_sequence_is_not_idempotent`, `changed_command_kind_is_not_idempotent`, `reused_command_id_for_different_envelope_at_different_ordinal_conflicts`, `reused_source_sequence_for_different_command_conflicts`, `source_sequence_finalizing_in_descending_order_rejects`, `different_sources_may_legitimately_interleave`, `unauthorized_epoch_reset_rejects`, `arbitrary_epoch_rollback_or_reuse_rejects`, `finalized_history_digest_differs_for_behaviorally_distinct_envelopes`. All 10 original corpus items were also updated to the new API and continue to pass, including a corrected version of item 1 (see §6 below).

### B-02 — Structural authority/write-class and state-schema safety (CLOSED)

**Root defect:** `StateStore::declare_authority` was public, letting any external caller declare an arbitrary authority for an undeclared ID and then successfully call the "wrong" write path for it; the store had no type/bounds/scope validation and was not profile-qualified.

**Correction** (`crates/spark-core/src/state.rs`, `crates/spark-core/src/value.rs`, `crates/spark-profile/src/definition.rs`):

- Added `DefinitionSchema` (profile-qualified: `profile_id` + `definition_id`, plus `fingerprint`, `authority`, `value_constraint`, `valid_scopes`). `StateStore::new` is now the *only* way to build a store, consuming an already-validated, immutable schema table; there is no schema-mutation method afterward.
- `observe_host_owned`, `apply_spark_effect`, `commit_derived`, and their shared `validate_write`/`upsert` helpers are now `pub(crate)`, not `pub`. There is no public write path in Phase 1 at all — the "later higher-level host/evaluator facade" the brief anticipates is Phase 2/3 scope; Phase 1's own crate-internal tests exercise these paths directly, per the brief's explicit sanction ("tests may access internal paths from inside the crate").
- Every write now validates, in order: schema existence for `(profile_id, definition_id)`, authority match, value-type match, declared bounds (via the new `ValueConstraint`, see M-03), and scope-kind membership in `valid_scopes`.
- Cells are keyed by `(ProfileId, DefinitionId, ScopeId)`, so the same `DefinitionId` declared independently in two profiles cannot cross-read/write.
- `declare_authority` was removed entirely (see B-04 for why `AuthorityCatalog` itself was removed rather than kept as a second, overlapping enforcement point).

**Falsification tests added/kept** (`crates/spark-core/src/state.rs`): `undeclared_definition_cannot_be_written_through_any_path`, `wrong_value_type_rejects`, `out_of_bounds_value_rejects`, `disallowed_scope_rejects`, `wrong_profile_rejects`, `same_definition_id_in_two_profiles_does_not_cross_contaminate`, plus the two original authority-path tests updated to the new profile-qualified/schema-based constructor.

Note on "state registry cannot be mutated after activation": this is enforced structurally — `StateStore` has no method that can add, remove, or alter a schema entry post-construction — rather than by a runtime test, which is consistent with the brief's instruction not to add public surface merely to make something testable at runtime.

### B-03 — Scheduler semantic ordering and occurrence identity (CLOSED)

**Root defect:** `Scheduler::schedule` assigned `occurrence_index` from a global counter at call time, so insertion/worker-interleaving order became part of canonical identity.

**Correction** (`crates/spark-core/src/scheduler.rs`):

- `DueWorkItem` now carries `profile_id` and a `work_kind: String` domain-separated discriminator in addition to `due_time`, `producer_definition_id`, `scope_id`, and `occurrence_index`.
- `occurrence_index: OccurrenceIndex` is now supplied by the caller as part of the `DueWorkItem` the producer constructs; the scheduler never assigns it.
- `Scheduler::schedule` keys work by the caller-supplied `(due_time, occurrence_index)` slot: an exact-duplicate `DueWorkItem` is idempotent (`ScheduleOutcome::AlreadyScheduledIdempotent`); a different item claiming the same slot is a rejected conflict (`ConflictingDuplicateWorkKey`), mirroring the timeline's per-ordinal slot model.
- Added `OccurrenceIndex::checked_next()` for callers deriving the next index in a persisted sequence (e.g. resuming recurrence after a save), rejecting `u64` overflow instead of wrapping.

**Falsification tests added/kept**: `reversed_schedule_call_order_drains_identically` (A/B vs. B/A produces identical drain order), `exact_duplicate_schedule_is_idempotent`, `conflicting_duplicate_work_key_rejects`, `occurrence_index_overflow_rejects`, plus the original equal-due-time and drain-filtering tests updated to the new API.

### B-04 — Immutable full definition identity + atomic validation (CLOSED)

**Root defect:** the validator enforced only authority (not the full fingerprint), mutated the live catalog while walking a manifest (so a later error left earlier declarations installed), and `DefinitionKind::Trigger`/`Custom("trigger")` canonicalized identically, as did every `ScopeKind::Custom` variant.

**Correction** (`crates/spark-profile/src/identity.rs` [new], `crates/spark-profile/src/validate.rs`, `crates/spark-profile/src/definition.rs`, `crates/spark-core/src/scope.rs`):

- Removed `spark_core::authority::AuthorityCatalog` entirely (it was the single overlapping enforcement point the reviewer flagged) and replaced it with `spark_profile::identity::DefinitionIdentityRegistry`, keyed by `(ProfileId, DefinitionId)` and storing the *full* `definition_fingerprint()` (kind, value constraint, authority/write class, sorted valid scopes) rather than authority alone.
- `validate()` is now candidate-staged: every definition in a manifest is checked against a read-only view of the registry, and `DefinitionIdentityRegistry::commit` (crate-private) is called exactly once, atomically, only after the *entire* manifest has passed every check. A manifest that fails partway through leaves the registry byte-for-byte unchanged; a corrected retry then succeeds cleanly.
- `DefinitionKind::canonicalize` now encodes built-in variants as their literal tag and `Custom(tag)` as `"custom:" + tag`, so `Trigger` and `Custom("trigger")` fingerprint differently.
- `ScopeKind::Custom` now carries a validated `StableId` and canonicalizes as `"custom:" + id`, so two different custom scope kinds — and a custom kind vs. any built-in — never collide (this also lets `DefinitionSpec::valid_scopes` distinguish custom scopes for real, closing part of M-03).
- `validate()` also now checks that every definition's `profile_id` matches its manifest's `profile_id` (`ValidationError::ProfileMismatch`), and identity is profile-qualified end to end, so the same `DefinitionId` in two independent profiles cannot conflict.

**Falsification tests added/kept** (`crates/spark-profile/src/validate.rs`, `crates/spark-profile/src/definition.rs`): `changed_value_type_under_same_id_and_authority_is_rejected`, `changed_valid_scopes_is_rejected`, `failed_multi_definition_validation_leaves_registry_unchanged_and_retry_succeeds`, `same_definition_id_in_independent_profiles_does_not_conflict`, `builtin_trigger_fingerprint_differs_from_custom_trigger`, plus the original authority-reload and extensibility tests updated to the new registry.

### M-01 — `ConfigRevision` / `config_revision_hash` (CLOSED)

**Correction** (`crates/spark-profile/src/config.rs` [new]): added `ConfigEntry { key: DefinitionId, value: CanonicalValue }` and `ConfigRevision { profile_id, revision_label, entries }` with `config_revision_hash()` — sorted-by-key, order-independent, profile-qualified, deliberately excludes `revision_label` from the hash (mirroring `ProfileManifest::version_label`'s treatment) so a reused human label with different content still changes the hash. No Phase-3 mutation API was added, matching the brief's scope.

**Tests:** `changed_content_changes_hash`, `reused_label_with_different_content_hashes_differently`, `entry_order_does_not_affect_hash`, `different_profile_changes_hash`.

### M-02 — Replay/state digest must prove the property (CLOSED)

**Root defect:** `canonical_state_digest` omitted active sequencer, window width, and full staged-slot/finalized-envelope/fence-body content, so an empty scenario and a scenario with one successfully staged unfinalized command hashed identically; `run_scenario` discarded every stage/fence result.

**Correction** (`crates/spark-core/src/timeline.rs`, `crates/spark-testkit/src/scenario.rs`):

- `canonical_state_digest` now hashes: `profile_id`, `timeline_epoch`, `active_sequencer`, `window_width`, `frontier_ordinal`, `last_finalized_fence_hash`; every staged/poisoned slot (staged slots include the full envelope, not just its ordinal); every finalized command's full envelope; every finalized fence's full body (`profile_id`, `epoch`, `fence_id`, `start`, `end`, `previous_fence_hash`, `ordered_stream_digest`); and every epoch reset record.
- `run_scenario` now returns `ScenarioReplay { ingress_state_digest, transcript_digest }`. `transcript_digest` hashes every operation's actual outcome (which `StageOutcome`/`StageError` variant, which `FenceError` variant or `Ok`) in order, so a scenario whose operations succeed/fail differently is no longer silently indistinguishable from one that didn't.

**Falsification tests added** (`crates/spark-testkit/src/scenario.rs`): `empty_scenario_differs_from_one_staged_unfinalized_command`, `poisoned_staging_differs_from_clean_staging`, `different_finalized_envelope_changes_history_digest`, `differing_operation_outcome_changes_transcript_digest`, plus the strengthened 50-repeat replay-stability test (now asserting both digests) and a spark-core-level `finalized_history_digest_differs_for_behaviorally_distinct_envelopes` (also listed under B-01).

### M-03 — Canonical construction, bounds, overflow, IDs/scopes (CLOSED)

**Correction:**

- `crates/spark-core/src/id.rs`: added `MAX_STABLE_ID_LEN = 256` and `StableIdError::TooLong`; `StableId::new` now rejects IDs longer than the bound.
- `crates/spark-core/src/scope.rs`: `ScopeKind::Custom` now carries a real `StableId` (see B-04); the misleading `"actor.bron->player.main"` directed-relationship doc example (which `StableId` itself rejects, since `->` is an invalid character) was replaced with an explicit note that a relationship-pair scope type is Phase-4 scope and must use structured subject/target fields when it is added, not a single delimited string.
- `crates/spark-core/src/value.rs`: `FixedPoint`'s raw `i64` is now a private field (`from_raw`/`raw()` accessors); `from_integer` is now `fn(v: i64) -> Result<FixedPoint, FixedPointOverflow>` using `checked_mul`, rejecting overflow instead of panicking or wrapping. Added `ValueConstraint` (`Bool`, `Int{min,max}`, `Fixed{min,max}`, `Categorical{max_len}`, `Reference`) with `.accepts(value)` bounds checking, consumed by `StateStore` (B-02) and `DefinitionSpec`/`DefinitionSchema` (B-04).
- `crates/spark-core/src/timeline.rs`: `window_end()` uses `checked_add`/`checked_sub` (documented as an internal-invariant `.expect`, since `window_width >= 1` is a constructor invariant and overflow would require ~2⁶⁴ ordinals); the frontier advance in `submit_fence` (`fence.end_ordinal.0 + 1`) uses `checked_add` and returns the new `FenceError::OrdinalArithmeticOverflow` typed error rather than panicking/wrapping.
- `crates/spark-core/src/scheduler.rs`: `OccurrenceIndex::checked_next()` (see B-03).

**Tests added:** `rejects_id_longer_than_max_length` (id.rs), `fixed_point_integer_overflow_is_rejected_not_wrapped`, `int_constraint_rejects_out_of_bounds`, `constraint_rejects_wrong_type`, `categorical_constraint_bounds_length` (value.rs), `occurrence_index_overflow_rejects` (scheduler.rs, also listed under B-03), plus the B-02 out-of-bounds/wrong-type state tests.

### M-04 — Random-address profile/artifact context (CLOSED)

**Correction** (`crates/spark-core/src/random.rs`): `RandomAddress` now carries `profile_id: ProfileId` and `behavior_artifact_hash: Digest` (a deterministic digest of the active behavior artifact — e.g. `ProfileManifest::manifest_content_hash()` — the draw was made under), both hashed into the derivation alongside the existing fields. `address_at()`'s signature was updated to match.

**Tests added:** `same_ids_and_epoch_in_different_profiles_are_independent`, `same_profile_different_behavior_artifact_hash_is_independent`, alongside the original same-address/call-order-independence/fixed-fraction-bounds tests, all still passing.

### m-01 — Dependency-direction enforcement (CLOSED)

**Correction:**

- Removed the unused `spark-core` → `spark-testkit` dev-dependency from `crates/spark-core/Cargo.toml` (confirmed unused: `crates/spark-core/tests/timeline_admission.rs` never imported `spark_testkit`).
- Rewrote `crates/spark-testkit/tests/workspace_dependency_direction.rs` to invoke `cargo metadata --format-version 1` (the same command the completion gate runs) and parse its `resolve.nodes[].deps[]` graph — which reports every dependency kind (normal, dev, build, target-specific) — instead of hand-scanning only `[dependencies]` lines in each `Cargo.toml`. The rewritten test would have caught the now-removed dev-dependency immediately.
- Added `serde_json` as a **dev-only** dependency of `spark-testkit`, with `default-features = false, features = ["std"]` specifically to avoid pulling in `serde`'s derive-macro chain (`serde_derive`/`syn`/`quote`/`proc-macro2`), since the test only parses `serde_json::Value`. `serde_json` 1.0.151, `serde_core` 1.0.229, `itoa` 1.0.18, `memchr` 2.8.3, and `zmij` 1.0.23 (a small `dtolnay`-authored float-formatting crate `serde_json` depends on) are the resulting dev-only transitive dependencies — all MIT/Apache-2.0, mature, widely used, and do not appear in the shipped (non-dev) dependency graph of any crate.

## 4. Regression result

All 45 original Phase-1 tests remain valid; several were strengthened in place per the brief's instruction ("must either remain valid and pass or be replaced by stricter tests that prove the same intended invariant") rather than deleted:

- Corpus item 1 (`reversed_delivery_within_capacity_two_window_is_order_independent`) now additionally asserts `canonical_state_digest()` equality between the two delivery orders (using one shared `fence_id`, since a submitter-chosen label is not itself part of the logical outcome being compared).
- Corpus item 19 (`same_scenario_replayed_many_times_produces_identical_digest`) now asserts both `ingress_state_digest` and `transcript_digest` across 50 repeats.
- The authority tests in `state.rs` and `validate.rs` now exercise the profile-qualified schema/registry constructors.

No test was weakened, deleted, or special-cased to make code pass.

## 5. Total test count and results

**85 tests, 0 failures** (up from the original 45):

```text
spark-core     unit tests (lib):                    41 passed
spark-core     tests/timeline_admission.rs:          21 passed
spark-profile  unit tests (lib):                     16 passed
spark-testkit  unit tests (lib):                      5 passed
spark-testkit  tests/workspace_dependency_direction.rs: 2 passed
                                                      ---
                                                      85 passed, 0 failed
```

## 6. Determinism / authority / finality evidence

- **No wall clock, no ambient RNG, no `HashMap`/`HashSet` in canonical state**: still true after the correction; `grep` for `std::time`, `SystemTime`, `rand`, and `std::collections::HashMap` in `crates/*/src` returns nothing (the new dev-only `workspace_dependency_direction.rs` test uses `std::collections::HashMap` for a non-canonical, test-local bookkeeping map, which is outside the canonical kernel and not part of any hashed/replayed state).
- **Full-envelope identity**: `semantic_envelope_hash` is the single function deciding "same command" for idempotency, command-ID uniqueness, and source-sequence uniqueness alike; falsified directly by the four `changed_*_is_not_idempotent` tests (§3, B-01).
- **Authority is structurally unbypassable, not convention**: `StateStore` has zero public write methods; `cargo doc`/a `grep 'pub fn'` over `crates/spark-core/src/state.rs` shows only `new`, `schema_of`, and `get` as public — no `set`, `write`, or `declare_authority`.
- **Identity is atomic-or-nothing**: `failed_multi_definition_validation_leaves_registry_unchanged_and_retry_succeeds` proves a partially-valid manifest leaves the registry untouched and a corrected retry succeeds cleanly.
- **Finality fences are still all-or-nothing** (unchanged from the original evidence) and now additionally reject a per-source sequence regression atomically before any promotion (`source_sequence_finalizing_in_descending_order_rejects`).
- **Epoch handoff is authorized and monotonic**: `unauthorized_epoch_reset_rejects` and `arbitrary_epoch_rollback_or_reuse_rejects`.
- **Replay stability**: `same_scenario_replayed_many_times_produces_identical_digest` runs 51 times and asserts both digests are bit-identical every time; the digests themselves are now provably non-degenerate (`empty_scenario_differs_from_one_staged_unfinalized_command`, etc.).

## 7. Dependency / license changes

| Crate | Version | License | Kind | Why |
|---|---|---|---|---|
| `blake3` | 1.8.7 | CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception | normal (`spark-core`) | Unchanged from the original writer pass. |
| `serde_json` | 1.0.151 | MIT OR Apache-2.0 | **dev-only** (`spark-testkit`) | Robustly parses `cargo metadata` JSON for the m-01 dependency-direction test. `default-features = false, features = ["std"]` avoids the derive-macro chain. |
| `serde_core` | 1.0.229 | MIT OR Apache-2.0 | dev-only transitive (via `serde_json`) | — |
| `itoa` | 1.0.18 | MIT OR Apache-2.0 | dev-only transitive | — |
| `memchr` | 2.8.3 | Unlicense OR MIT | dev-only transitive | — |
| `zmij` | 1.0.23 | MIT | dev-only transitive (via `serde_json`) | Float-formatting; `dtolnay`-authored, same author as `syn`/`quote`/`serde`. |

None of these are normal (non-dev) dependencies of any shipped crate; `spark-core`'s only normal dependency remains `blake3` alone, verified by `spark_core_resolves_zero_dependencies_on_any_product_layer_crate` and by inspection of `cargo metadata`'s resolved graph. `Cargo.lock` is committed.

## 8. Windows / Android static checks

Re-run after the correction (via `rustup target add` + `cargo check --workspace --target <triple>`, no linking/execution — this machine cannot run Windows or Android binaries, unchanged from the original writer pass):

| Target | Result |
|---|---|
| `x86_64-pc-windows-gnu` | `cargo check` passes, 0 errors/warnings |
| `x86_64-pc-windows-msvc` | `cargo check` passes, 0 errors/warnings |
| `aarch64-linux-android` | `cargo check` passes, 0 errors/warnings |
| `x86_64-linux-android` | `cargo check` passes, 0 errors/warnings |
| `armv7-linux-androideabi` | `cargo check` passes, 0 errors/warnings |

Actual Windows/Android build/link/execution remains unexecuted and unclaimed, exactly as recorded in the original writer report and the independent review: the accepted brief defers that executable portability proof to a later gate, and no claim of Windows/Android execution success is made anywhere in this report.

## 9. Linux validation performed

```text
cargo fmt --check                                    → pass (0 diffs)
cargo clippy --workspace --all-targets -- -D warnings → pass (0 warnings)
cargo test --workspace                                → pass (85 passed, 0 failed)
cargo metadata --format-version 1                     → pass (exit 0)
```

One clippy finding was corrected during this pass (not a functional defect, but recorded per the "document non-obvious decisions" discipline): `clippy::large_enum_variant` on `timeline::SlotState` after `Staged` grew to hold a full `CommandEnvelope`. Fixed by boxing the envelope field (`Staged { envelope: Box<CommandEnvelope>, .. }`); `FinalizedCommand::envelope` remains unboxed since it isn't paired with a zero-sized sibling variant.

## 10. Deviations from the correction brief

None. Every finding in `PHASE_1_CORRECTION_BRIEF_v0.1.md` (B-01 through B-04, M-01 through M-04, m-01) was corrected as specified; no blocker required changing a frozen Phase-0 architectural contract or a new causal primitive, so no conflict is being reported under §14 ("Convergence rule").

One clarifying implementation choice, recorded for transparency: `AuthorityCatalog` (the Phase-1 writer pass's authority-only, non-profile-qualified identity mechanism) was removed rather than kept alongside the new profile-qualified `DefinitionIdentityRegistry`. Keeping both would have reintroduced exactly the kind of overlapping, inconsistent enforcement the independent review flagged as a structural risk; `Authority`/`WriteClass` (the two small fixed vocabularies `AuthorityCatalog` used to gate) remain in `spark-core::authority` unchanged, now consumed directly by `DefinitionSchema` and `DefinitionSpec` instead of through a duplicate catalog.

## 11. Known risks (carried forward / updated)

- `TimelineIngress` remains entirely in-memory with no persistence integration; Phase 3 will still need to bind `finalized_commands`/`finalized_fences`/`epoch_resets` to ADR-0006's content-addressed behavior-artifact and delayed-obligation model.
- `compute_fence_hash`, `semantic_envelope_hash`, and `compute_ordered_stream_digest` remain Phase-1-internal hash constructions, correct and deterministic for Phase 1's purposes but not yet frozen as a cross-version wire-protocol contract; `spark-protocol` (Phase 3) may need to re-derive them from a protocol-level canonical form.
- `DefinitionKind`/`ScopeKind` remain a small provisional baseline (now domain-separated against `Custom`), not the full blueprint taxonomy; extending them for later profile packs is expected profile-vocabulary work, not an engine change.
- Windows/Android execution (not just `cargo check`) remains unverified (§8); a genuine cross-compilation or CI-runner regression could still surface at build/link time even though static analysis is clean.
- `ConfigRevision` (M-01) has no consumer yet — it is a logical representation and content hash only, with no Phase-3 mutation API, matching the brief's explicitly bounded scope for this correction.

## 12. Phase 2 authorization

**`PHASE_2_AUTHORIZATION: NO`**

This is a bounded correction pass against the Phase-1 Rust core skeleton. It does not constitute or imply authorization to begin Phase 2 (rule/effect runtime), and no Phase 2 code was written. Independent Codex re-review of this correction is the next required step before any further phase may begin.
