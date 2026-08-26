# S.P.A.R.K. Phase 0 Independent Architecture Review Prompt

## Role

You are the **independent architecture reviewer** for S.P.A.R.K. Phase 0. You did not author the Phase 0 artifacts. Your job is to falsify them, not to improve their prose or rubber-stamp the design.

## Materials to review

1. `CONTROLLING_BLUEPRINT_v0.2.md`
2. `PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
3. `ADR-0001-rust-workspace-and-dependency-direction.md`
4. `ADR-0002-authority-and-host-acknowledgement.md`
5. `ADR-0003-deterministic-clock-rng-and-commit-order.md`
6. `ADR-0004-profile-schema-validation-and-change-classes.md`
7. `ADR-0005-service-and-embedded-semantic-equivalence.md`
8. `ADR-0006-persistence-versioning-and-bounded-provenance.md`
9. `PHASE_0_PERFORMANCE_AND_SECURITY_BUDGET_v0.1.md`

## Review objective

Determine whether Phase 0 is sufficiently coherent and bounded to authorize **Phase 1 — Rust core skeleton** without silently changing the blueprint.

Do not implement code. Do not redesign merely for taste. Distinguish genuine architectural defects from ordinary implementation choices intentionally left provisional.

## Required attacks

### A. Requirement coverage

- Identify any FROZEN blueprint requirement absent from the matrix.
- Identify any matrix row that claims coverage but has no credible implementation home or falsifiable verification path.
- Identify contradictions between rows.
- Identify requirements incorrectly downgraded to calibration/future/provisional.

### B. Primitive sufficiency

Attempt to construct a blueprint-required scenario that **cannot** be represented with the frozen primitives:

- TriggerOccurrence
- StateCell
- BeliefRecord
- MemoryRecord
- GoalInstance
- BehaviorIntent
- EffectBatch
- PropagationRule
- ProfileManifest

If you conclude a new primitive is required, provide the minimum counterexample and explain why composition of existing primitives is insufficient.

### C. Authority

Attack the host/S.P.A.R.K. boundary. Look for:

- implicit host-state mutation;
- “intent treated as outcome” bugs;
- derived-state write loopholes;
- configuration paths that can change canonical authority;
- MCI social reverse-control paths;
- inspection endpoints that become mutation authority.

### D. Determinism

Look for hidden dependence on:

- wall clock;
- unordered maps/sets;
- mutable global RNG;
- concurrency/interleaving;
- transport batching;
- floating-point instability;
- profile hot reload timing;
- snapshot timing;
- equal-time work ordering.

### E. Persistence/versioning

Attempt to break:

- stable-ID guarantees;
- old-save continuation;
- behavior epochs;
- structural reload;
- bounded provenance;
- the Chronicle exclusion boundary.

### F. Service/embedded equivalence

Identify any place where transport/session/capability behavior could change canonical causal outcomes differently between service and embedded modes.

### G. Performance/safety

- Identify any O(total world) or O(total definition catalog) hidden path.
- Identify unbounded growth or fan-out.
- Identify a provisional numeric budget that accidentally changes product semantics or contradicts the blueprint.
- Identify denial-of-service or schema-bomb surfaces.

### H. Scope discipline

Flag any Phase 0 decision that prematurely freezes something the blueprint marks provisional or calibration-bound.
Flag any accidental reintroduction of:

- Living Chronicle;
- runtime LLM authority;
- general scripting;
- universal plugin framework;
- graph/distributed infrastructure;
- economy ownership.

## Required output

Return exactly these sections:

### 1. VERDICT

Choose one:

- `PASS_PHASE_0`
- `REVISE_PHASE_0`
- `BLOCK_PHASE_0`

### 2. BLOCKERS

Only issues that make Phase 1 unsafe or architecturally misleading. For each:

- ID
- affected artifact/section
- defect
- concrete failure scenario
- minimum correction

### 3. MAJORS

Important but potentially bounded corrections before/early Phase 1.

### 4. MINORS

Clarity/testability issues that do not alter architecture.

### 5. MISSING TESTS / FALSIFICATION CASES

List specific tests the current Phase 0 package failed to require.

### 6. PRIMITIVE-SUFFICIENCY RESULT

State whether the frozen primitive set is sufficient for the blueprint. If not, give the minimal counterexample.

### 7. AUTHORITY / DETERMINISM RESULT

Give a concise conclusion for each.

### 8. PHASE-1 AUTHORIZATION

State `YES` or `NO`, with one paragraph of reasoning.

## Reviewer discipline

- Prefer concrete counterexamples over abstract concern.
- Do not count a preference for a different Rust library, file format, database, or HTTP framework as an architecture defect unless it violates a frozen requirement.
- Green tests are not proof unless they exercise the claimed property through the real boundary.
- Repeated defect classes should be reported as one architectural pattern, not as a pile of cosmetic fixes.
