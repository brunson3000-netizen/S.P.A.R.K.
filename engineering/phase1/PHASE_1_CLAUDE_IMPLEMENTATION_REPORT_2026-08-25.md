# S.P.A.R.K. Phase 1 — Claude Implementation Report

**Phase:** 1 — Rust Core Skeleton
**Date:** 2026-08-25
**Writer:** Claude Code (Sonnet 5)
**Controlling architecture:** `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md` plus accepted Phase-0 ADRs (0001-0006) and `PHASE_0_CODEX_CLOSURE_REVIEW_2026-08-25.md`
**Controlling brief:** `engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md`
**Independent reviewer:** Codex, after this writer pass

## 1. Implementation verdict

**COMPLETE**, within the scope the implementation brief authorizes. All 20 minimum test-corpus items pass, `cargo fmt --check` / `cargo clippy -D warnings` / `cargo test` all pass on Linux, and `cargo check` passes cleanly against two Windows targets and three Android ABIs. No Phase-2 scope (rule/effect runtime, service transport, persistence backend, actor behavioral richness, MCI/game integration, dialogue, voice, or catalog expansion) was implemented.

This writer pass does **not** authorize Phase 2. It is a candidate for independent Codex review.

## 2. Exact commit(s)

| Commit | Contents |
|---|---|
| `336d4e3b0fec1e5ef5db6edcf591254bfbf32702` | The Rust implementation: workspace, all three crates, all tests. |
| *(this report's own commit — see `git log` on top of `336d4e3`)* | This report, committed immediately after the implementation commit, plus the identical `~/Downloads/` transfer copy noted outside the repository. |

The working tree is clean after both commits. `git log --oneline -3` at the time of writing:

```text
336d4e3 Implement S.P.A.R.K. Phase 1 Rust core skeleton
8a5f7c7 Authorize and stage S.P.A.R.K. Phase 1 Rust core skeleton
79dd6cd docs: record Phase 0 closure review
```

## 3. Workspace / crate structure

```text
spark/                       (repository root)
  Cargo.toml                 workspace manifest (resolver "2", 3 members)
  Cargo.lock                 committed
  crates/
    spark-core/               canonical deterministic kernel
      src/
        id.rs                 StableId + typed newtypes (DefinitionId, ProfileId, SourceId, CommandId, FenceId)
        scope.rs               ScopeKind + ScopeId
        value.rs               CanonicalValue, FixedPoint (scale 1_000_000), ValueType
        hash.rs                Digest, CanonicalEncoder, hash_bytes (blake3-backed)
        authority.rs           Authority, WriteClass, AuthorityCatalog (immutable-identity enforcement)
        state.rs                StateCell, StateStore (3 narrow authority-safe write paths, no generic `set`)
        clock.rs                LogicalTime, LogicalClock (monotonic, rejects backward advancement)
        scheduler.rs            OccurrenceIndex, DueWorkItem, Scheduler (stable total order)
        random.rs               RandomAddress, RandomAddressService (pure-function derivation)
        timeline.rs             TimelineIngress: ADR-0003 sequencer/admission-window/staging/fence/epoch-reset
      tests/
        timeline_admission.rs   corpus items 1-10 (reversed delivery, stale token, idempotency/poisoning,
                                 fence validation, non-sequencer/old-epoch rejection, partial-prefix slide)
    spark-profile/            profile manifest logical types + validation + content hashing
      src/
        definition.rs           DefinitionSpec, DefinitionKind, BehavioralLeverage, definition_fingerprint()
        manifest.rs              ProfileManifest, manifest_content_hash()
        validate.rs              validate() against a shared AuthorityCatalog
    spark-testkit/            deterministic fixtures / scenario replay harness
      src/
        scenario.rs              Scenario, ScenarioOp, run_scenario() -> canonical state digest
      tests/
        workspace_dependency_direction.rs   corpus item 20 (ADR-0001 automated check)
```

Dependency direction (ADR-0001), verified both by inspection and by the automated test in
`crates/spark-testkit/tests/workspace_dependency_direction.rs`:

- `spark-core` → no normal dependency on any other S.P.A.R.K. crate (only `blake3`).
- `spark-profile` → `spark-core` only.
- `spark-testkit` → `spark-core`, `spark-profile`.
- `spark-core` has a **dev-only** dependency on `spark-testkit` (for its own integration tests to reuse fixture helpers in a later phase if useful); this is a Cargo dev-dependency cycle, which does not affect the crate's normal build/dependency graph and is explicitly excluded from the ADR-0001 check for that reason.

Not built in Phase 1 (explicitly out of scope per the brief): `spark-protocol`, `spark-service`, `spark-persistence`, `spark-inspection`, `spark-expression`, `spark-voice`, `profiles/`, `adapters/`. These remain named in the blueprint's recommended workspace shape (§8.3) for later phases.

## 4. Requirements implemented (brief §2, items 1-16)

| # | Item | Where |
|---|---|---|
| 1 | Workspace/crate boundaries | `Cargo.toml`, 3 crates above |
| 2 | Immutable namespaced IDs | `spark-core::id` |
| 3 | Scope identifiers/types | `spark-core::scope` |
| 4 | Canonical value types | `spark-core::value` |
| 5 | Authority model (`host_owned`/`spark_owned`/`derived` + immutable write class) | `spark-core::authority` |
| 6 | `StateCell` + authority-safe write API | `spark-core::state` |
| 7 | Monotonic logical clock | `spark-core::clock` |
| 8 | Due-work scheduler skeleton, stable ordering | `spark-core::scheduler` |
| 9 | Semantic random-address derivation | `spark-core::random` |
| 10 | Canonical timeline ingress (epoch, sequencer, admission window, slots, tokens, idempotent/poison, STAGED, fence validation, contiguous finalization, frontier, epoch-reset) | `spark-core::timeline` |
| 11 | Profile manifest logical types | `spark-profile::manifest`, `spark-profile::definition` |
| 12 | Profile validation skeleton | `spark-profile::validate` |
| 13 | Canonical manifest/config content hashing | `ProfileManifest::manifest_content_hash` |
| 14 | Definition fingerprint incl. authority/write-class identity | `DefinitionSpec::definition_fingerprint` |
| 15 | Deterministic test fixtures + replay hashes | `spark-testkit::scenario`, `TimelineIngress::canonical_state_digest` |
| 16 | CI-friendly commands/scripts/docs | §9 below (no extra script needed; corpus item 20 is a `cargo test`, not a shell script) |

Item 13's "config revision hash" is covered at the level Phase 1 needs it: `manifest_content_hash` already demonstrates the general content-hashing mechanism (deterministic, format-independent, order-independent). A dedicated `ConfigRevision` type for hot-tunable parameter patches was judged Phase-2/3 scope (it has no consumer until the rule runtime and config-mutation API exist) and was not added, to avoid speculative, untested surface area.

## 5. Tests added and results

**45 tests, 0 failures**, across 3 crates (unit tests inline via `#[cfg(test)]`, integration tests under `tests/`):

```text
spark-core     unit tests:        25 passed
spark-core     tests/timeline_admission.rs:  10 passed
spark-profile  unit tests:         7 passed
spark-testkit  unit tests:         1 passed
spark-testkit  tests/workspace_dependency_direction.rs:  2 passed
                                   ---
                                   45 passed, 0 failed
```

### Minimum test-corpus mapping (brief §5, all 20 items)

| # | Requirement | Test |
|---|---|---|
| 1 | `n+1,n+2,n` vs `n,n+1,n+2`, width 2, same fence result | `timeline_admission::reversed_delivery_within_capacity_two_window_is_order_independent` |
| 2 | Old-token delayed `n+2` rejects after frontier moves; new-token retry stages | `timeline_admission::stale_token_after_frontier_advance_is_rejected_then_succeeds_with_fresh_token` |
| 3 | Exact duplicate is idempotent | `timeline_admission::exact_duplicate_is_idempotent` |
| 4 | Distinct payload poisons identically in opposite orders | `timeline_admission::distinct_payload_collision_poisons_identically_in_opposite_orders` |
| 5 | Missing ordinal blocks fence atomically | `timeline_admission::missing_ordinal_blocks_fence_atomically` |
| 6 | Wrong fence digest rejects | `timeline_admission::wrong_fence_digest_rejects` |
| 7 | Wrong previous-fence hash rejects | `timeline_admission::wrong_previous_fence_hash_rejects` |
| 8 | Non-sequencer staging/fence rejects | `timeline_admission::non_sequencer_stage_and_fence_are_rejected` |
| 9 | Old-epoch sequencer rejects after handoff | `timeline_admission::old_epoch_sequencer_rejects_after_handoff` |
| 10 | Partial-prefix fence slides window deterministically | `timeline_admission::partial_prefix_fence_slides_window_and_preserves_staged_tail` |
| 11 | Host-owned state cannot be mutated by S.P.A.R.K.-owned write path | `state::tests::host_owned_cannot_be_mutated_by_spark_effect_write_path` |
| 12 | Derived state cannot be independently written | `state::tests::derived_cannot_be_written_by_client_facing_effect_path` |
| 13 | Authority/write-class change under same ID is invalid | `authority::tests::changing_authority_under_same_id_is_rejected` (spark-core) and `validate::tests::authority_change_across_structural_reload_is_rejected` (spark-profile, across a simulated reload) |
| 14 | Reused version label, different content ⇒ different hash | `manifest::tests::reused_version_label_with_different_content_hashes_differently` |
| 15 | Same seed/address ⇒ identical random result | `random::tests::same_address_produces_identical_result` |
| 16 | Unrelated random addresses isolated from call order | `random::tests::call_order_does_not_affect_result_for_unrelated_addresses`, `different_occurrence_index_is_independent` |
| 17 | Logical clock rejects backward advancement | `clock::tests::rejects_backward_advancement` |
| 18 | Equal-time due work has stable total order | `scheduler::tests::equal_due_time_drains_in_schedule_order` |
| 19 | Same fixture repeated many times ⇒ identical canonical hashes | `scenario::tests::same_scenario_replayed_many_times_produces_identical_digest` (50 repeated runs) |
| 20 | Workspace dependency policy has an automated check | `workspace_dependency_direction::*` (2 tests) |

### Extra tests added beyond the minimum corpus

- `hash::tests::length_prefixing_prevents_field_boundary_collision` — falsifies a specific canonical-encoding bug class (`"ab"+"c"` colliding with `"a"+"bc"`) that a naive concatenation-based hash would have.
- `manifest::tests::identical_content_hashes_identically_regardless_of_definition_order` — proves the manifest hash does not depend on `Vec`/iteration order, directly testing determinism constitution item 2.
- `state::tests::undeclared_definition_cannot_be_written_through_any_path` — proves an ID with no declared authority is unwritable by any of the three paths, not just the "wrong" one.
- `validate::tests::new_trait_and_supernatural_trigger_are_addable_as_pure_profile_data` — a first slice of the blueprint's §30.4 extensibility acceptance criterion (add `trait.curiosity` / `trigger.supernatural.blood_moon` without Rust changes), scoped to what the Phase-1 validator can demonstrate.
- `scope::tests::distinct_kinds_with_same_instance_id_are_distinct_scopes` — falsifies scope-key collision across kinds.

## 6. Determinism evidence

- **No wall clock, no OS time, no ambient RNG anywhere in `spark-core`/`spark-profile`/`spark-testkit`.** Verified by inspection; there is no `std::time`, `SystemTime`, or `rand` dependency in the workspace.
- **No `HashMap`/`HashSet` in canonical state.** `StateStore`, `AuthorityCatalog`, and `TimelineIngress` all use `BTreeMap`/`BTreeSet` or a scheduler ordered by an explicit total-order key; grep confirms no `std::collections::HashMap` appears in `spark-core/src` or `spark-profile/src`.
- **Canonical hashing is explicit and length-prefixed** (`CanonicalEncoder`), preventing field-boundary aliasing; `hash::tests::length_prefixing_prevents_field_boundary_collision` falsifies the naive-concatenation failure mode directly.
- **Scheduler stable ordering**: `OccurrenceIndex` is assigned monotonically at schedule time (not derived from a `HashMap` key, thread ID, or wall time), and the drain order key is `(due_time, occurrence_index)`, proven by `scheduler::tests::equal_due_time_drains_in_schedule_order`.
- **Random-address purity**: `RandomAddressService` holds no fields and no mutable state; every method is `&self` over an explicit `RandomAddress` argument, so isolation between addresses and independence from call order fall out of the type signature rather than needing runtime bookkeeping. Proven directly by `random::tests::call_order_does_not_affect_result_for_unrelated_addresses`.
- **Replay stability**: `scenario::tests::same_scenario_replayed_many_times_produces_identical_digest` runs one 6-operation scenario (staging, an out-of-window rejection, a partial fence, a window slide, and a second fence) 51 times from a fresh `TimelineIngress` each time and asserts every run's `canonical_state_digest()` is bit-identical.
- **Cross-run determinism was also checked directly**: running `cargo test --workspace` repeatedly (see §9) produced identical pass/fail results and, for the digest-bearing tests, byte-identical assertions each time — no test uses a seeded-but-varying source.

## 7. Authority evidence

- `spark-core::state::StateStore` exposes exactly three write methods — `observe_host_owned`, `apply_spark_effect`, `commit_derived` — each hard-checking one specific `Authority` variant against `AuthorityCatalog` before writing. **There is no generic `set`/`write` method taking an `Authority` parameter anywhere in the crate**; grep for `pub fn set` / `pub fn write` in `spark-core/src/state.rs` returns nothing beyond the three named methods plus the private `upsert` helper (not `pub`).
- `state::tests::host_owned_cannot_be_mutated_by_spark_effect_write_path` proves `apply_spark_effect` against a `HostOwned` definition returns `Err` and leaves no cell behind.
- `state::tests::derived_cannot_be_written_by_client_facing_effect_path` proves `apply_spark_effect` against a `Derived` definition returns `Err`, and only `commit_derived` (the evaluator-only path) succeeds.
- `AuthorityCatalog::declare` is idempotent for a repeated identical declaration but returns `AuthorityIdentityConflict` for any attempt to redeclare an existing ID with a different authority — proven at the catalog level (`authority::tests::changing_authority_under_same_id_is_rejected`) and again at the profile-validation level across a simulated structural reload (`validate::tests::authority_change_across_structural_reload_is_rejected`), so the invariant holds both for direct catalog use and for the profile-load path that will carry it in later phases.

## 8. Admission-window / finality evidence

All 10 timeline items in the minimum corpus pass (§5 table); the key structural properties, restated from ADR-0003 and directly exercised by tests:

- **Capacity is allocated by ordinal position, not arrival**: `TimelineIngress::stage` checks ordinal-range membership against the *current* window before ever touching `self.slots`, so an out-of-window submission cannot evict or poison anything (`reversed_delivery_within_capacity_two_window_is_order_independent`).
- **Token staleness is distinct from range staleness**: an ordinal that is numerically inside the new window but carries a token computed under the old window is rejected as `StaleOrInvalidAdmissionWindowToken` (a protocol error), not silently accepted or treated the same as `NotInAdmissionWindow` (a logical, capacity-preserving outcome) — `stale_token_after_frontier_advance_is_rejected_then_succeeds_with_fresh_token`.
- **Fencing is all-or-nothing**: a fence is validated fully (profile, epoch, sequencer, `start_ordinal == frontier`, `end_ordinal` within window, every ordinal positively staged and unpoisoned, digest match, previous-fence-hash match) before any mutation; any failure leaves `frontier_ordinal` and `finalized_commands` untouched (`missing_ordinal_blocks_fence_atomically`, `wrong_fence_digest_rejects`, `wrong_previous_fence_hash_rejects`).
- **Partial-prefix finalization slides the window and preserves still-staged higher ordinals** rather than discarding them (`partial_prefix_fence_slides_window_and_preserves_staged_tail`).
- **Epoch reset freezes the finalized frontier and discards only unfinalized staged state**, and the old sequencer/epoch can no longer stage afterward (`old_epoch_sequencer_rejects_after_handoff`).

## 9. Dependency / license summary

| Crate | Version | License | Why |
|---|---|---|---|
| `blake3` | 1.8.7 | `CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception` | Sole third-party dependency in the whole workspace. Used for every canonical content hash (`hash::hash_bytes`/`CanonicalEncoder::finish`) and for semantic random-address derivation (`random::RandomAddressService`). Chosen for: maturity and wide adoption, a 256-bit deterministic output suitable for content-addressed identity (manifest hashes, definition fingerprints, fence hashes), and a `pure` build mode (see below) that keeps cross-target checks free of native-toolchain requirements. |

**Dependency-direction discipline**: `spark-core`'s only normal dependency is `blake3`; `spark-profile` depends only on `spark-core`; `spark-testkit` depends only on `spark-core` and `spark-profile`. This is enforced by an automated `cargo test` (corpus item 20), not merely documented — see `crates/spark-testkit/tests/workspace_dependency_direction.rs`. No script beyond `cargo fmt --check` / `cargo clippy --workspace --all-targets -- -D warnings` / `cargo test --workspace` is required for CI; all three are ordinary `cargo` subcommands.

**Non-obvious dependency decision, recorded per the writer prompt's dependency-discipline requirement**: `blake3`'s default build enables an optional C/assembly SIMD backend via a `cc`-based build script. On `x86_64-unknown-linux-gnu` this built without incident, but a first `cargo check --target aarch64-linux-android` failed because no Android NDK `clang` is installed on this machine (`ToolNotFound: aarch64-linux-android-clang`). Rather than deferring the whole Android check, `spark-core`'s `Cargo.toml` was changed to `blake3 = { version = "1", default-features = false, features = ["std", "pure"] }`, which forces blake3's portable Rust implementation and removes the `cc`/native-toolchain requirement entirely. This is a strict improvement for the stated Android-from-inception constraint (blueprint §31 item 1, §8.1) and was re-verified: after the change, all of Linux `cargo test`, `x86_64-pc-windows-{gnu,msvc}`, and `{aarch64,x86_64,armv7}-linux-android*` cross-checks pass with zero native compiler involvement (see §11). No cryptographic-strength/performance requirement was traded away for Phase 1's purposes — nothing in Phase 1 hashes performance-sensitive volumes of data, and `blake3`'s pure-Rust path remains a mature, correct implementation of the same algorithm.

`Cargo.lock` is committed.

## 10. Linux validation performed

Run on this machine (`x86_64-unknown-linux-gnu`, `rustc 1.98.0`, `cargo 1.98.0`, installed via `rustup` during this session since no Rust toolchain was previously present):

```text
cargo fmt --check                                    → pass (0 diffs after one `cargo fmt` pass)
cargo clippy --workspace --all-targets -- -D warnings → pass (0 warnings)
cargo test --workspace                                → pass (45 passed, 0 failed)
```

## 11. Windows / Android checks performed or explicitly deferred

**Performed** (via `rustup target add` + `cargo check --workspace --target <triple>`, no linking/execution — this machine cannot run Windows or Android binaries):

| Target | Result |
|---|---|
| `x86_64-pc-windows-gnu` | `cargo check` passes, 0 errors/warnings |
| `x86_64-pc-windows-msvc` | `cargo check` passes, 0 errors/warnings |
| `aarch64-linux-android` | `cargo check` passes, 0 errors/warnings (after the `blake3` `pure`-feature change; previously blocked on a missing NDK `clang`) |
| `x86_64-linux-android` | `cargo check` passes, 0 errors/warnings |
| `armv7-linux-androideabi` | `cargo check` passes, 0 errors/warnings |

**Explicitly deferred** (not executed in this writer pass, per the brief's instruction not to interrupt the operator to install missing toolchains mid-pass):

- **Actual Windows build/link/test/execution.** `cargo check` proves the code is type/borrow-correct for the Windows target and pulls in no Windows-incompatible API, but does not prove it links or runs. No MinGW/MSVC linker or Windows CI runner was available/installed. Later portability gate commands:
  ```text
  rustup target add x86_64-pc-windows-gnu
  cargo build --workspace --target x86_64-pc-windows-gnu
  cargo test --workspace --target x86_64-pc-windows-gnu   # requires a Windows runner or Wine
  ```
- **Actual Android build/link/deterministic-fixture-replay execution**, which the brief explicitly says must not be claimed as passed until it actually runs. `cargo check` proves type/borrow correctness and, after the `pure` feature change, that the crate graph builds without a native toolchain up through the check stage — but linking/running still requires the Android NDK. Later portability gate commands:
  ```text
  # Install Android NDK (e.g. via Android Studio's SDK Manager, or
  # https://developer.android.com/ndk/downloads), then:
  cargo install cargo-ndk
  rustup target add aarch64-linux-android x86_64-linux-android armv7-linux-androideabi
  cargo ndk -t arm64-v8a -t x86_64 -t armeabi-v7a build --workspace
  # Deterministic fixture replay on-device/emulator:
  cargo ndk -t arm64-v8a test --workspace
  ```

No claim of Windows or Android determinism/execution success is made anywhere in this report; both are recorded as unexecuted, matching the brief's requirement.

## 12. Deviations from the implementation brief

1. **`spark-testkit` was scoped narrower than a full "scenario harness."** It provides a `Scenario`/`ScenarioOp`/`run_scenario` replay mechanism sufficient to prove item 19's replay-stability requirement, but does not yet include a Pontafique/Lindemar-style narrative fixture (that scenario needs the Phase-4/6 actor/rule/economy machinery this phase deliberately does not implement). This is a scope-matching decision, not an omission against anything the brief authorizes for Phase 1.
2. **`blake3`'s default features were narrowed to `["std", "pure"]`** (see §9) partway through validation, after the Android cross-check surfaced a native-toolchain dependency. This is a strict improvement against the brief's cross-platform posture requirement and is recorded here per the "document any nontrivial dependency choice" instruction.
3. **No dedicated `ConfigRevision`/hot-tune-patch type was added** in `spark-profile` (see §4 note on item 13). `manifest_content_hash` demonstrates the same deterministic content-hashing mechanism a future `config_revision_hash` would reuse; adding an unused type now would be speculative surface area with no Phase-1 consumer or test.
4. **No CLI/script wrapper was added for the corpus item 20 dependency check.** It is implemented as an ordinary `cargo test`, which is simpler and already CI-native; a wrapper shell script would only duplicate `cargo test -p spark-testkit`.

No test was weakened to make code pass; no Phase-2 scope (rule/effect evaluation, service transport, persistence, actor behavior, dialogue, MCI/game adapters) was implemented.

## 13. Known risks

- **`TimelineIngress` is entirely in-memory** with no persistence integration; Phase 3 will need to bind its `finalized_commands`/`finalized_fences`/`epoch_resets` to ADR-0006's content-addressed behavior-artifact and delayed-obligation model. Nothing here forecloses that, but it has not been exercised end-to-end.
- **`compute_fence_hash` and `compute_ordered_stream_digest` are Phase-1-internal hash constructions** (canonical field encodings chosen by this writer); they are not yet declared as part of any versioned external protocol. If `spark-protocol` (Phase 3) needs a different wire encoding, these internal functions may need to be re-derived from a protocol-level canonical form rather than assumed stable as-is — they are correct and deterministic for Phase 1's purposes but not yet frozen as a cross-version contract.
- **The `DefinitionKind`/`ScopeKind` enums are a small provisional baseline**, not the full taxonomy in blueprint Appendix A. Extending them for later profile packs is expected profile-vocabulary work per ADR-0004's taxonomy rule (`Custom(String)`/`Custom` variants already provide an escape hatch), not an engine change.
- **Windows/Android execution (not just `cargo check`) is unverified** (see §11); a genuine cross-compilation or CI-runner regression could still surface at build/link time even though static analysis is clean.
- **`blake3`'s `pure` feature trades some performance for zero native-toolchain dependency.** For Phase 1's data volumes this is immaterial; if a later phase's profiling shows hashing is hot, revisiting this feature flag (with a proper NDK-equipped CI Android target) is a reasonable, narrowly-scoped follow-up.

## 14. Files changed

25 files, 3567 insertions, in commit `336d4e3b0fec1e5ef5db6edcf591254bfbf32702` (plus `.gitignore`'s one-line `/target/` addition folded into the same commit): the workspace `Cargo.toml`/`Cargo.lock`, and the three crates' `Cargo.toml` + `src/*.rs` + `tests/*.rs` listed in full in §3. This report and its `~/Downloads/` copy are the only files added afterward.

## 15. Phase 2 authorization

**Phase 2 remains unauthorized pending independent review.** This writer pass implements and self-tests the Phase-1 Rust core skeleton only. It does not constitute or imply authorization to begin Phase 2 (rule/effect runtime), and no Phase 2 code was written. Independent Codex review of this implementation is the next required step before any further phase may begin.
