### 1. VERDICT

`REVISE_PHASE_1`

### 2. B-01 CLOSURE

`OPEN`

Full semantic-envelope hashing, command/source uniqueness, per-source finalized sequence ordering, and authorized monotonic epoch handoff are materially improved. Two foundational finality defects remain:

- Admission-window tokens are correctly excluded from `semantic_envelope_hash`, but the full token is retained in each finalized `CommandEnvelope` and included by `canonical_state_digest` (`crates/spark-core/src/timeline.rs:122-138`, `:763-806`). A bounded adversarial fixture finalized the same semantic envelopes behind the same fence chain, once with ordinal 1 staged before the frontier advanced and once afterward under the new token. The semantic hashes and finalized fences were identical, but the final ingress digests differed. Nonsemantic admission timing therefore still becomes canonical replay identity.
- `stage` still returns only the unqualified enum `StageOutcome::{Staged, AlreadyStagedIdempotent, Poisoned, NotInAdmissionWindow}`, and `submit_fence` returns `Result<(), FenceError>` (`timeline.rs:163-182`, `:482-486`, `:602-606`). There is no ADR/correction-brief-required structured STAGED acknowledgement binding profile, epoch, ordinal, command ID, payload/semantic hash, and slot state, nor a structured finalized result. A future transport cannot prove which envelope a positive acknowledgement covers from these result values alone.

### 3. B-02 CLOSURE

`OPEN`

The individual host/effect/evaluator write methods are no longer public, and their internal validation covers authority, type, value bounds, scope, and profile. However, the authority schema is still publicly forgeable: every field of `DefinitionSchema` is public and `StateStore::new` publicly accepts arbitrary raw schema entries without a validation capability, registry binding, duplicate rejection, or fingerprint verification (`crates/spark-core/src/state.rs:63-77`, `:236-251`). `DefinitionSpec::to_definition_schema` is also public and only documents—not enforces—that the definition must already have passed validation (`crates/spark-profile/src/definition.rs:162-174`).

A separate-crate adversarial fixture constructed an arbitrary `Derived` schema with `Digest::ZERO`; `StateStore::new` installed it and exposed it through `schema_of`. This is still a public authority-declaration path at activation. Making later write facades crate-internal will not make the schema provenance structurally trustworthy if external code can supply fabricated authority/type/scope/fingerprint metadata.

### 4. B-03 CLOSURE

`OPEN`

The scheduler no longer manufactures occurrence indexes from call order, but its actual key is only `(due_time, occurrence_index)` (`crates/spark-core/src/scheduler.rs:88-121`). The correction brief required the semantic key to include profile, producer/rule, scope, occurrence index, and work kind. Two independent producers legitimately using occurrence index 0 at the same due time collide; the first scheduled item remains installed while the second is rejected. Reversing insertion reverses which logical work survives.

A bounded adversarial fixture scheduled `trigger.a` and `trigger.b`, both at time 5 and each at its own semantic occurrence 0. The second schedule failed in either order, and first arrival selected the retained item. The existing reversed-order test avoids this defect by assigning globally distinct indexes 0 and 1. Scheduler identity therefore remains an implicit global counter contract and cannot safely support independent per-producer/per-scope occurrence sequences.

### 5. B-04 CLOSURE

`CLOSED`

The definition registry is profile-qualified and compares the full fingerprint. Definition kind, value constraint/type, authority/write class, and sorted valid scopes are committed only after whole-manifest validation succeeds. Failed validation leaves the registry unchanged; corrected retry succeeds. Built-in `Trigger` and `Custom("trigger")` are domain-separated, distinct custom scope identifiers remain distinct, and independent profiles do not contaminate each other's identity history. No fingerprint omission or collision was found in the accepted immutable-identity fields.

### 6. MAJOR CLOSURE

- M-01: `OPEN`. `ConfigRevision` exists and correctly distinguishes content/profile while ignoring human labels for identity in ordinary unique-key fixtures. It is not a validated config object: its public `Vec<ConfigEntry>` permits duplicate keys and arbitrary unbounded canonical values (`crates/spark-profile/src/config.rs:23-60`). Reversing two different values under the same key changes the hash because equal-key order is preserved, so malformed content is neither rejected nor insertion-order independent. No constructor/validator applies the promised canonical bounds discipline.
- M-02: `OPEN`. Empty/staged/poisoned/finalized/error transcript cases and 50 repeated runs are stronger and pass, but the ingress state digest canonically hashes admission tokens. The adversarial equivalent-history fixture described under B-01 produced different final digests solely from whether an already-eligible ordinal was staged before or after a frontier slide. Replay equivalence remains arrival/admission-timing-sensitive.
- M-03: `OPEN`. Length-limited `StableId`, custom-scope identity, private `FixedPoint` raw storage, checked integer conversion, state value constraints, and scheduler occurrence overflow are improvements. Surviving malformed-construction paths include: `DefinitionId::new("curiosity")` accepts a non-namespaced ID because every typed ID shares the generic single-segment-accepting constructor (`crates/spark-core/src/id.rs:62-89`, `:113-130`); the profile validator accepts `Int { min: 10, max: 0 }` and oversized `DefinitionKind::Custom(String)` because it validates neither constraint coherence nor bounded custom strings (`crates/spark-core/src/value.rs:171-229`, `crates/spark-profile/src/definition.rs:26-49`, `crates/spark-profile/src/validate.rs:75-130`); `command_kind` and scheduler `work_kind` are also unbounded public `String`s. Timeline overflow handling remains non-atomic: `window_end` can panic via `expect`, while `submit_fence` promotes commands/fence state before checking `end_ordinal + 1` (`timeline.rs:425-433`, `:680-704`).
- M-04: `CLOSED`. Random addresses include root seed, profile ID, behavior epoch, behavior-artifact digest, producer ID, scope, and occurrence index. Same semantic address repeats identically; changing profile or artifact changes the draw; call order is irrelevant.

### 7. MINOR CLOSURE

- m-01: `CLOSED`. The unused `spark-core -> spark-testkit` dev edge is removed. The metadata-based policy test inspects resolved workspace edges across dependency kinds. Current product direction is `spark-profile -> spark-core` and `spark-testkit -> spark-core + spark-profile`; `spark-core` has no product-layer dependency. The only new dependency is `serde_json` as a `spark-testkit` dev dependency, so it does not enter the shipped graph.

### 8. NEW FINDINGS

- **BLOCKER — Nonsemantic admission timing contaminates finalized replay state.** The same semantic command/fence history receives different `canonical_state_digest` values when a tail ordinal is staged under the old overlapping window token versus the new frontier token.
- **BLOCKER — The state schema is publicly forgeable.** Raw public `DefinitionSchema` construction plus public `StateStore::new` bypasses the definition identity registry and validation ceremony.
- **BLOCKER — Scheduler semantic key is incomplete.** `(due_time, occurrence_index)` conflates independent profile/producer/scope/work-kind occurrence domains and lets first arrival choose the retained item on collision.
- **MAJOR — Config revisions admit ambiguous duplicate keys.** Duplicate-key order affects `config_revision_hash`, and no validator rejects the ambiguity or bounds values.
- **MAJOR — Malformed canonical constructors remain accepted.** Non-namespaced definition IDs, incoherent min/max constraints, oversized custom definition kinds, and unbounded command/work kinds enter public canonical types; timeline overflow error handling is not fully panic-free or atomic.

### 9. TEST / TOOL RESULTS

- `git status --short`: clean at review start. Temporary adversarial fixtures were removed after evidence capture. Before adding this report, the tree was clean again.
- `git log --oneline --decorate -10`: HEAD at review start was `d1fbda0`; correction commit `b744c95`; later commits `5804145` and `d1fbda0` contain correction evidence/status and this re-review prompt, not subsequent implementation changes.
- `cargo fmt --check`: PASS, exit 0.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS, exit 0, zero warnings.
- `cargo test --workspace`: PASS, exit 0; 85 tests passed, 0 failed: 41 `spark-core` unit, 21 timeline integration, 16 `spark-profile`, 5 `spark-testkit`, and 2 dependency-policy tests. Doc tests contain 0 tests.
- Bounded independent adversarial fixtures: FAIL as intended, 0 passed / 7 failed. They falsified admission-token causal inertness, complete scheduler semantic identity, validated-only state schema construction, required definition namespacing, duplicate-key config order independence/rejection, invalid-bound rejection, and oversized custom-kind rejection. Both fixture files were removed afterward.
- `cargo metadata --format-version 1`: PASS, exit 0. Workspace members are `spark-core`, `spark-profile`, and `spark-testkit`; no `spark-core -> spark-testkit` edge remains. Normal shipped direction is valid; `serde_json` is dev-only in `spark-testkit`.
- Installed-target static checks: `cargo check --workspace --target` PASS for `x86_64-pc-windows-gnu`, `x86_64-pc-windows-msvc`, `aarch64-linux-android`, `x86_64-linux-android`, and `armv7-linux-androideabi`. These are compile checks only; no Windows or Android binary was linked/executed, and no execution claim is made.
- General audit: no unsafe Rust in canonical Phase-1 crates; no canonical wall clock, ambient/global RNG, unordered canonical state iteration, or floating-point canonical arithmetic found; no Phase-2 rule/effect runtime or other prohibited scope creep found.
- Final working-tree status: clean after committing only this canonical review artifact.

### 10. CONVERGENCE RESULT

The bounded correction did **not** converge. The same foundational defect classes survive: canonical identity/finality remains admission-timing-sensitive and lacks the required structured acknowledgement; authority/schema provenance remains bypassable through public raw construction; and scheduler semantics remain dependent on a globally shared occurrence-index slot that lets first arrival choose among independent work. B-04's immutable definition-identity/atomic-validation class did converge and is closed.

### 11. PHASE-2 AUTHORIZATION

`NO`

B-01, B-02, and B-03 remain open blockers, and M-01 through M-03 remain open majors. Phase 2 must not build rule/effect semantics on these ingress, authority-schema, scheduler, config, replay, and canonical-construction foundations.
