### 1. VERDICT

`REVISE_PHASE_1`

### 2. BLOCKERS

- **B-01 — Canonical envelope identity and finality are incomplete.** `CommandEnvelope` declares `effective_time`, `source_id`, `source_sequence`, and `command_kind`, but staging retains and compares only `(command_id, canonical_payload_hash)`, and the ordered stream digest/finalized record also hashes or retains only ordinal, command ID, and payload hash (`crates/spark-core/src/timeline.rs:82`, `:165`, `:188`, `:239`, `:338`). A bounded adversarial test changed all four omitted fields under the same ordinal/command/payload and received `AlreadyStagedIdempotent`, not poison/rejection. The implementation also does not validate envelope `source_id`, source-sequence uniqueness/monotonicity, command-ID uniqueness, or a full canonical-envelope hash. The existing old-epoch test accidentally demonstrates this: its helper puts the old sequencer in the envelope while the new sequencer submits it, and staging succeeds. Consequently two behaviorally distinct canonical streams can receive the same staged identity, fence digest, and finalized command representation. `reset_epoch` is additionally a public, unauthenticated mutation that accepts any epoch value and discards staged state without a sequencer/grant check (`timeline.rs:500`). The ordinal-window mechanics are sound in isolation, but canonical finality is not.

- **B-02 — Authority/write-class safety is not structurally enforced.** `StateStore` publicly exposes `declare_authority`, `observe_host_owned`, and `commit_derived` (`crates/spark-core/src/state.rs:158`, `:192`, `:225`). An external crate can fabricate a previously undeclared ID, declare it `Derived`, and successfully call the supposedly evaluator-only write path; the adversarial fixture did exactly that. The store records only authority, not a validated/profile-qualified `DefinitionSpec`, so it also accepts an arbitrary value variant and disallowed scope for a declared ID. There is no capability/token/type boundary distinguishing host ingress, evaluator commit, migration, or fixture access. A name/comment is not structural enforcement. The state store is also not profile-partitioned. Phase 2 would otherwise build effects on an API through which host-owned/derived/undeclared or cross-profile state can be fabricated.

- **B-03 — Scheduler ordering and occurrence identity depend on call/insertion order.** `Scheduler::schedule` assigns one global incrementing index at the moment the method is called and uses `(due_time, assigned_index)` as its complete order key (`crates/spark-core/src/scheduler.rs:47`, `:57`, `:88`). Reversing the submission order of the same two equal-time logical work items reversed their drain order in the adversarial fixture. This is the insertion-order shortcut the controlling contract forbids: worker interleaving, batch partition, or collection order can become canonical order, and the index is not a persisted semantic occurrence identity per trigger/rule and scope. Phase-2 deterministic waves and delayed work cannot safely build on this scheduler contract.

- **B-04 — Immutable definition identity is computed but not enforced, and failed validation mutates identity state.** `definition_fingerprint` includes kind, value type, authority/write class, and scopes (`crates/spark-profile/src/definition.rs:120`), but `validate` stores/checks only authority (`crates/spark-profile/src/validate.rs:47`). A reload changing `ValueType` under the same ID and authority was accepted. Validation mutates the live `AuthorityCatalog` as it walks; a later duplicate/no-scope error returns failure after earlier declarations remain installed, which the adversarial fixture showed can poison a corrected retry. The catalog is keyed only by `DefinitionId`, not profile plus ID. In addition, `DefinitionKind::Trigger` and `DefinitionKind::Custom("trigger")` canonicalize identically (`definition.rs:19`, `:30`) and produced the same fingerprint. These are direct violations of frozen identity, atomic activation, profile isolation, and unambiguous canonical encoding.

### 3. MAJORS

- **M-01 — Phase-1 requirement #13 is not implemented.** There is no `ConfigRevision`, logical config representation, or `config_revision_hash` in code. `manifest_content_hash` proves a reusable encoding technique but cannot bind content that does not exist in the manifest and cannot provide the separately required config identity. This is a defect requiring correction before Phase 2, not compositional satisfaction and not a non-blocking deferral.

- **M-02 — The deterministic replay evidence is not property-falsifying.** `run_scenario` discards every stage/fence result (`crates/spark-testkit/src/scenario.rs:61`, `:89`), while `canonical_state_digest` omits active sequencer, window width, all staged/poisoned slots, full finalized envelopes, fence bodies, and reset-record content (`crates/spark-core/src/timeline.rs:518`). An empty scenario and the same scenario with one successfully staged unfinalized command produced identical digests. The 50-repeat test therefore proves repeatability of an incomplete projection, not canonical replay equivalence. The reversed-delivery test compares frontier and truncated finalized records, not canonical digests; its two fences even use different fence IDs. Corpus items 1, 4, 18, and 19 need stronger semantic assertions.

- **M-03 — ID/scope/value bounds and canonical construction are incomplete.** `StableId` validates characters but does not require a namespace for `DefinitionId`; all custom scope taxonomies collapse to the single tag `ScopeKind::Custom`; and the documented directed-relationship example contains `->`, which `StableId` rejects (`crates/spark-core/src/scope.rs:15`, `:62`). `FixedPoint` is a public raw tuple and `from_integer` performs unchecked multiplication (`crates/spark-core/src/value.rs:24`, `:29`): `i64::MAX` panics in the debug review build and wraps in release. Timeline frontier/window arithmetic and scheduler occurrence increments are likewise unchecked. Definition value bounds are absent, and public categorical/custom strings are unbounded/unvalidated. These malformed or collision-prone values must be rejected before they enter canonical state/identity.

- **M-04 — Random addresses omit profile identity.** `RandomAddress` hashes root seed, numeric behavior epoch, rule/trigger ID, scope, and occurrence index, but no `ProfileId` or content identity (`crates/spark-core/src/random.rs:27`). The same private IDs and epoch number in two profiles therefore resolve to the same address. That conflicts with the frozen profile-qualified random-address/behavior-epoch context and weakens profile isolation even though call-order purity itself passes.

### 4. MINORS

- **m-01 — Dependency-direction enforcement is narrower than its claim.** The test deliberately parses only literal `[dependencies]` path entries containing `../` and ignores dev, build, renamed/workspace, and target-specific dependencies (`crates/spark-testkit/tests/workspace_dependency_direction.rs:1`, `:53`). `cargo metadata` shows the actual unused `spark-core` dev-dependency on `spark-testkit`, creating the test-only cycle `spark-core -> spark-testkit -> spark-profile -> spark-core`. This does not invert the normal production graph and is not independently a Phase-2 blocker, but the unused edge should be removed or justified and the mechanical check should inspect Cargo metadata across dependency kinds that policy intends to govern.

### 5. TEST / TOOL RESULTS

- `git status --short`: clean at review start and again after the bounded adversarial fixture was removed; no implementation file was changed.
- `git log --oneline --decorate -8`: HEAD `a962e1a`; implementation `336d4e3`; subsequent commits contain the writer report, phase status, and this review prompt, not code changes.
- `cargo fmt --check`: PASS, exit 0.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS, exit 0, zero warnings.
- `cargo test --workspace`: PASS, exit 0; 45 tests passed, 0 failed (25 `spark-core` unit, 10 timeline integration, 7 `spark-profile`, 1 `spark-testkit`, 2 dependency-direction; doc tests contain 0 tests). Repeated after fixture removal with the same result.
- Bounded independent adversarial fixture: FAIL as intended, 0 passed / 9 failed. It falsified full-envelope duplicate identity, evaluator-only visibility, type/scope write enforcement, fingerprint reload enforcement, validation atomicity, enum canonical separation, insertion-independent scheduling, staged-state replay hashing, and overflow-safe fixed-point construction. The fixture was removed after recording results.
- `cargo metadata --format-version 1`: PASS, exit 0. Workspace members are `spark-core`, `spark-profile`, and `spark-testkit`. Normal product direction is `spark-profile -> spark-core` and `spark-testkit -> spark-core + spark-profile`; `spark-core` has the noted dev-only reverse edge.
- Installed-target `cargo check --workspace --target ...`: PASS for `x86_64-pc-windows-gnu`, `x86_64-pc-windows-msvc`, `aarch64-linux-android`, `x86_64-linux-android`, and `armv7-linux-androideabi`.
- Final artifact handling: the repository review file was the only repository change, its Downloads copy was verified byte-identical, it was committed in an evidence-only commit, and the final working tree was clean.

### 6. TIMELINE / FINALITY RESULT

**FAIL overall.** The deterministic frontier-derived window, reserved ordinal slots, out-of-window no-eviction result, stale-token rejection, payload-hash duplicate idempotence, command-ID/payload collision poison, contiguous staged/unpoisoned fence range, wrong-digest/previous-hash atomic rejection, and partial-prefix tail preservation all work in the tested ordinary cases. No higher eligible ordinal crowds out the frontier.

Those narrow results do not establish canonical finality. Full envelope identity is discarded, the envelope source is not validated, source/command uniqueness rules are absent, there is no structured STAGED acknowledgement carrying the ADR-required identity fields, reset/handoff bypasses sequencer authorization and epoch constraints, arithmetic is unchecked, and the final digest omits observable staging state. Reversed/concurrent logical delivery has therefore not been shown to yield identical full canonical digests, and distinct canonical commands can be collapsed as duplicates.

### 7. AUTHORITY RESULT

**FAIL.** The three named write methods correctly reject use of the wrong named path for a catalog entry, and identical/different authority redeclaration behaves as tested. However, any external caller with `&mut StateStore` can create the catalog declaration and invoke host-ingress/evaluator-only methods directly. The catalog/store lacks the validated definition fingerprint, profile partition, type/bounds, and allowed scopes needed to reject fabricated or malformed state. Authority safety is presently convention plus method naming, not a structurally unbypassable API.

### 8. DETERMINISM / IDENTITY RESULT

**FAIL overall.** Positive evidence: canonical integers use explicit little-endian encoding, variable bytes are length-prefixed, manifests sort definitions, canonical stores use ordered collections, the logical clock rejects backward advancement, BLAKE3 derivation is stateless, and no wall clock/global RNG/floating-point canonical arithmetic was found.

Negative evidence is controlling: scheduler order is insertion-dependent; timeline hashes omit meaningful envelope fields; scenario hashes omit staged state and errors; fingerprints are not enforced atomically; built-in/custom definition kinds can collide; custom scopes are not uniquely representable; numeric overflow is unchecked; and random addresses omit profile identity.

The missing dedicated `ConfigRevision` / `config_revision_hash` is a **MAJOR Phase-1 defect requiring correction before Phase 2**. It is not satisfied compositionally by `manifest_content_hash`, because no separate canonical configuration content or identity is represented or hashed.

### 9. SCOPE / DEPENDENCY RESULT

**Scope discipline passes.** Inspection found no Phase-2 propagation/effect runtime, service transport, persistence backend, actor behavior, MCI/game adapter, dialogue/voice system, broad catalog, scripting engine, plugin platform, graph database, or distributed service implemented early.

**Normal dependency architecture passes with one minor test/dev-edge issue.** `spark-core` has only BLAKE3 as a direct normal dependency; `spark-profile` depends on core; testkit depends outward on both. BLAKE3 1.8.7 is permissively licensed (`CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception`), uses `std + pure`, and cross-target checks passed. Metadata still resolves BLAKE3's build script/`cc` package and platform-conditioned `cpufeatures` dependencies, but no target check required operator intervention or a target C compiler. The test-only core/testkit cycle and incomplete dependency checker are MINOR, not an architectural inversion in shipped code.

### 10. CROSS-PLATFORM RESULT

- **Linux execution/test:** proven locally for the complete workspace: fmt, strict clippy, and 45/45 repository tests pass on `x86_64-unknown-linux-gnu` with Rust/Cargo 1.98.0.
- **Windows target check:** proven locally for GNU and MSVC x86_64 targets; this is type/check compilation only.
- **Android target check:** proven locally for aarch64, x86_64, and armv7 targets; this is type/check compilation only.
- **Actual Windows execution determinism:** unproven.
- **Actual Android execution determinism:** unproven.

The absent Windows/Android execution gate is not by itself a Phase-1 blocker because the accepted brief defers that executable portability proof. It must not be described as passed later until canonical fixtures actually execute on those platforms.

### 11. PHASE-2 AUTHORIZATION

`NO`

Phase 2 must wait until the blocker-class canonical identity/finality, authority boundary, scheduler occurrence ordering, and immutable profile identity defects are corrected and independently re-tested; the major config/replay/value/profile-address issues must also be closed.
