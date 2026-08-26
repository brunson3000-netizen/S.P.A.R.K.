# S.P.A.R.K. Phase 1 Implementation Brief

**Phase:** 1 — Rust Core Skeleton  
**Status:** AUTHORIZED  
**Controlling architecture:** `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md` plus accepted Phase-0 ADRs/corrections  
**Implementation writer:** Claude Code  
**Independent reviewer:** Codex after writer completion

## 1. Objective

Implement the smallest production-quality Rust skeleton that embodies the frozen Phase-0 contracts without beginning Phase 2 behavior/rule richness.

The implementation should make later development safer by making the dangerous invariants executable early:

- canonical timeline sequencing/finality;
- immutable authority/write class;
- deterministic logical clock/scheduler;
- semantic random addresses;
- typed/scoped state;
- profile manifest validation and content identity;
- stable identifiers;
- deterministic fixtures;
- workspace dependency discipline.

## 2. Authorized implementation scope

Implement:

1. Rust workspace and crate boundaries sufficient for Phase 1.
2. Common immutable namespaced IDs.
3. Scope identifiers/types.
4. Canonical value types needed by the skeleton.
5. Authority model:
   - `host_owned`
   - `spark_owned`
   - `derived`
   - immutable implied write class.
6. `StateCell` minimal representation and authority-safe write API.
7. Monotonic integer logical clock.
8. Due-work scheduler skeleton with stable ordering.
9. Semantic random-address derivation service.
10. Canonical timeline ingress skeleton:
    - timeline epoch;
    - one sequencer authority;
    - deterministic admission window;
    - one slot per eligible ordinal;
    - window tokens;
    - idempotent duplicate handling;
    - collision poisoning;
    - `STAGED` acknowledgement;
    - fence validation;
    - contiguous finalization;
    - finalized frontier;
    - epoch-reset/handoff representation.
11. Profile manifest logical types.
12. Profile validation skeleton.
13. Canonical manifest/config content hashing.
14. Definition fingerprint including immutable authority/write-class identity.
15. Deterministic test fixtures and replay hashes for Phase-1 behaviors.
16. CI-friendly commands/scripts/documentation for format, lint, and tests.

## 3. Explicitly NOT authorized

Do not implement:

- full Phase-2 propagation/effect rule runtime;
- general scripting;
- graph database;
- service HTTP/WebSocket transport;
- persistence database/backend;
- runtime LLM;
- actor psychology/behavior selection;
- dialogue/expression;
- MCI social integration;
- game adapter;
- TTS;
- broad trigger/factor catalogs;
- universal plugin system;
- distributed services;
- unsafe Rust in the canonical kernel.

Small placeholder interfaces/types are allowed only when needed to keep dependency direction clean.

## 4. Phase-0 invariants Phase 1 must embody

### Canonical timeline

Arrival order must have zero canonical authority.

At frontier `n` with admission-window width `W`, each eligible ordinal in `[n, n+W-1]` has its own logical slot.

Out-of-window commands return a retryable logical result and do not evict staged commands.

A fence cannot finalize until every ordinal in its range has positive staging acknowledgement, is unpoisoned, contiguous, and matches the fence digest/hash chain.

### Authority

Authority/write class is immutable identity for a persisted/activated definition ID.

The Phase-1 API must not contain a generic state mutation path that allows:
- S.P.A.R.K. to commit `host_owned` truth;
- clients to directly write `derived` truth.

### Determinism

Canonical results must not depend on:
- hash-map iteration;
- thread scheduling;
- wall clock;
- transport/session identity;
- mutable global RNG;
- staging delivery order.

Use stable total ordering and deterministic canonicalization.

### Content identity

Human version labels are not behavioral identity.

Manifest/config/definition identity must support exact content-addressed hashes/fingerprints.

## 5. Minimum Phase-1 test corpus

At minimum implement automated tests for:

1. `n+1,n+2,n` versus `n,n+1,n+2` with width 2:
   - `n` staged
   - `n+1` staged
   - `n+2` not in admission window
   - same fence result.
2. Old-token delayed `n+2` still rejects after frontier moves; retry with new token stages.
3. Same-ordinal exact duplicate is idempotent.
4. Same-ordinal distinct payload poisons identically in opposite delivery orders.
5. Missing ordinal blocks fence atomically.
6. Wrong fence digest rejects.
7. Wrong previous-fence hash rejects.
8. Non-sequencer staging/fence rejects.
9. Old-epoch sequencer rejects after handoff.
10. Partial-prefix fence slides window deterministically.
11. Host-owned state cannot be mutated by the S.P.A.R.K.-owned write path.
12. Derived state cannot be independently written.
13. Authority/write-class change under the same definition ID is invalid.
14. Manifest version label reused with different content produces different content hashes.
15. Same seed/address inputs produce identical random-address result.
16. Unrelated random addresses are isolated from call order.
17. Logical clock rejects backward advancement.
18. Due work with equal time has stable total order.
19. Same fixture repeated many times produces identical canonical hashes.
20. Workspace dependency policy has an automated or scriptable check.

## 6. Rust engineering constraints

- stable Rust toolchain;
- deny/avoid `unsafe` in canonical Phase-1 crates;
- prefer straightforward deterministic structures over clever concurrency;
- no kernel async requirement;
- no wall-clock APIs in canonical evaluation;
- use integer/fixed-point canonical representations;
- avoid unordered iteration in canonical hashing/commit ordering;
- serialization/canonical hashing must be explicit and stable;
- dependencies should be small, mature, and permissively licensed;
- commit `Cargo.lock`;
- no paid/cloud dependency.

## 7. Cross-platform posture

Required now:

- Linux build/test.
- Windows-compatible code path and CI/test definition where practical.
- Android-compatible canonical core constraints from inception.

If Windows/Android toolchains are not already installed locally, do not interrupt the operator merely to install them during this writer pass. Record the exact unexecuted validation and provide commands for the later portability gate.

Do not claim Android determinism passed until executable Android fixture replay actually runs.

## 8. Writer completion gate

The writer is complete only when:

- code exists in the canonical repository;
- `cargo fmt --check` passes;
- `cargo clippy` passes with the project-selected strictness;
- `cargo test` passes;
- Phase-1 deterministic/adversarial fixtures pass;
- no Phase-2 scope is smuggled in;
- Git working tree is clean after the writer commit;
- a complete implementation report is stored in the repository;
- an identical transfer copy of that report is written to `~/Downloads/`.

Writer completion does **not** authorize Phase 2.

The implementation must receive independent Codex review first.
