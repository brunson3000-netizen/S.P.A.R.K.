# S.P.A.R.K. Phase 1 — Fable Architecture / Process Review

## Mode

**ARCHITECTURE ONLY. NO IMPLEMENTATION.**

**Model:** Fable
**Effort:** HIGH

You are the independent architecture specialist for the S.P.A.R.K. Phase-1 Rust core skeleton.

Phase 0 is frozen and independently passed. Phase 1 has failed to converge after an original implementation, bounded correction, test-first re-foundation, and multiple independent Codex falsification reviews.

Per the convergence rule, do not write Rust and do not propose another ordinary patch pass.

Your job is to produce the exact Rust-level architecture for the remaining weak boundaries without changing the frozen causal grammar or Phase-0 product architecture.

## Autonomy

Work autonomously within this mission. You are pre-authorized to inspect repository files/history, run read-only Git/Cargo metadata/grep commands, create architecture specifications/test plans, create and commit only the final architecture-review artifact if the tree is otherwise clean, and copy the report to Downloads.

Do not ask the operator for routine permission.

Stop only if a required solution would change a frozen Phase-0 architecture, add a new causal runtime primitive, require paid/external resources, or materially expand project scope.

## Required reading

Read in order:

1. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md`
2. all accepted `engineering/phase0/ADR-000*.md`
3. `engineering/phase0/PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
4. `engineering/phase0/PHASE_0_CODEX_CLOSURE_REVIEW_2026-08-25.md`
5. `engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md`
6. `engineering/phase1/PHASE_1_CODEX_INDEPENDENT_REVIEW_2026-08-26.md`
7. `engineering/phase1/PHASE_1_CORRECTION_BRIEF_v0.1.md`
8. `engineering/phase1/PHASE_1_CODEX_CORRECTION_REREVIEW_2026-08-26.md`
9. `engineering/phase1/PHASE_1_CONVERGENCE_DECISION.md`
10. `engineering/phase1/PHASE_1_REFOUNDATION_BRIEF_v0.1.md`
11. `engineering/phase1/PHASE_1_REFOUNDATION_IMPLEMENTATION_REPORT_2026-08-26.md` if present
12. `engineering/phase1/PHASE_1_CODEX_REFOUNDATION_INDEPENDENT_REVIEW_2026-08-26.md`
13. relevant source modules only after understanding the architecture record

Repository behavior and independent falsification evidence outrank writer self-assessments.

## Central question

What exact Rust-level architecture should Phase 1 use so the remaining foundational boundaries are structurally correct and difficult to misuse?

Do not merely restate Codex findings. Produce a decisive design.

## Required architecture decisions

### A. Trusted activation / authority provenance

Design a non-bypassable activation lifecycle that ensures:

- external callers cannot mint trusted authority/schema from raw declarations;
- Core may own canonical fingerprint/schema invariants without giving every caller authority to activate them;
- profile parsing/validation remains above Core;
- activation is profile-qualified and atomic;
- StateStore construction cannot accept raw caller-authored authority/type/scope/fingerprint facts;
- future host/evaluator write facades can rely on the activation artifact;
- duplicate/divergent registry histories for the same canonical profile definition cannot silently exist;
- layering remains consistent with ADR-0001.

Decide:

1. Which crate/module owns activation?
2. Which types are public?
3. Which constructors are public/private/crate-private?
4. What opaque/unforgeable type crosses from validation to state activation?
5. How registry lifecycle/profile identity are anchored.
6. Whether a dedicated activation crate/module is justified.
7. How tests exercise internal mutation without opening production APIs.

Give concrete Rust API-shape pseudocode.

### B. Scheduler same-key conflict semantics

Choose one deterministic conflict model for two different payloads claiming the same full semantic WorkKey.

The result must guarantee:

- A then B and B then A yield identical scheduler canonical state;
- neither arrival gets privileged merely by being first;
- exact duplicate behavior is explicit;
- differing-payload conflict is visible/explainable;
- persistence/replay can represent the conflict state deterministically;
- Phase-2 delayed rules can build on it safely.

Give concrete WorkKey/slot/state API pseudocode and acceptance tests.

### C. Canonical panic-free construction policy

Define one uniform project-wide rule for canonical public construction/arithmetic.

Address:

- `CanonicalTag::from_static`;
- runtime vs const construction;
- `FixedPoint::clamp`;
- incoherent bounds;
- checked arithmetic;
- timeline frontier/window overflow;
- future canonical constructors.

Choose an approach such as fallible Result constructors, compile-time macros for literals plus fallible runtime constructors, validated newtypes with no panic-capable public operations, or a combination.

### D. `resume_at_frontier` / reconstruction authority

Codex recommends RESTRICT.

Define:

- whether nonzero-frontier construction exists in Phase 1;
- how test-only support remains non-production;
- future persistence constructor requirements such as snapshot identity, fence hash, finalized history/artifact provenance;
- how arbitrary canonical timeline injection is prevented.

### E. Closed-item preservation

Review and preserve where appropriate:

- semantic/admission separation;
- structured StageAcknowledgement/FinalizationResult;
- history/state/transcript digest model;
- immutable full definition fingerprint and atomic validation;
- ConfigRevision model;
- profile/artifact-qualified random addresses;
- dependency direction.

Identify any hidden coupling with your B-02/B-03/M-03 solution.

## Required deliverables

### 1. Recommended module/crate boundary
Provide the smallest clean Phase-1 crate/module map. Do not design Phase 2.

### 2. Public API surface
List every important public type/function that should exist after Phase 1, and dangerous types/functions that must not be public.

### 3. Trust-boundary diagram
Show:
```text
profile data
-> validation
-> activation authority
-> immutable activated schema
-> state runtime
```
and scheduler/finality equivalents.

### 4. Invariant table
For each major invariant:
```text
Invariant
Authoritative owner
Structural enforcement
Falsification/test
```

### 5. Acceptance-test corpus
Produce exact tests for the next implementation, including all current Codex counterexamples and any new ones.

### 6. Migration/reuse map
Classify current Phase-1 code/modules:
- REUSE AS-IS
- REUSE AFTER SMALL CHANGE
- REPLACE
- REMOVE/RESTRICT
- DEFER TO LATER PHASE

### 7. Implementation sequence
Give a bounded writer sequence, preferably:
```text
tests first
-> trusted activation
-> scheduler conflict model
-> canonical constructor policy
-> reconstruction restriction
-> closed-item integration
-> full regression
```

### 8. Writer model recommendation
Recommend the next implementation writer model and effort, while preserving independent review.

## Required verdict

Choose exactly one:

- `READY_FOR_IMPLEMENTATION_REFOUNDATION_V2`
- `ARCHITECTURE_CHANGE_REQUIRED`
- `OPERATOR_DECISION_REQUIRED`

If `OPERATOR_DECISION_REQUIRED`, ask only questions that materially affect product meaning/authority, not ordinary Rust design choices.

## Final report structure

### 1. VERDICT
### 2. ROOT-CAUSE ANALYSIS
### 3. TRUSTED ACTIVATION ARCHITECTURE
### 4. SCHEDULER CONFLICT ARCHITECTURE
### 5. PANIC-FREE CANONICAL API POLICY
### 6. RECONSTRUCTION / RESUME POLICY
### 7. CLOSED-ITEM PRESERVATION
### 8. MODULE / CRATE MAP
### 9. PUBLIC API CONTRACT
### 10. INVARIANT / TEST MATRIX
### 11. REUSE / REPLACE MAP
### 12. IMPLEMENTATION SEQUENCE
### 13. WRITER / REVIEWER RECOMMENDATION
### 14. OPERATOR DECISIONS REQUIRED
### 15. IMPLEMENTATION AUTHORIZATION RECOMMENDATION

## Artifact handling

Create canonical repository artifact:

`engineering/phase1/PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md`

Also copy the identical report to:

`~/Downloads/PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md`

Repository copy is canonical evidence. Downloads copy is disposable transfer convenience.

If Git is clean apart from the new report artifact, commit only that evidence file.

Then stop.
