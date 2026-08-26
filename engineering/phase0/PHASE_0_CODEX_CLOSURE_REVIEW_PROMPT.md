# S.P.A.R.K. Phase 0 — B-01A Closure Review Prompt

## Role

You are the independent Phase-0 closure reviewer.

**Effort: HIGH.**

Do not implement Rust. Do not modify reviewed source artifacts.

## Review evidence

Inspect:

- `CONTROLLING_BLUEPRINT_v0.2.md`
- all prior `PHASE_0_CODEX_*REVIEW*.md` evidence
- `PHASE_0_THIRD_CORRECTION_REPORT_2026-08-25.md`
- `ADR-0003-deterministic-clock-rng-and-commit-order.md`
- `ADR-0005-service-and-embedded-semantic-equivalence.md`
- `ADR-0002-authority-and-host-acknowledgement.md`
- `ADR-0004-profile-schema-validation-and-change-classes.md`
- `ADR-0006-persistence-versioning-and-bounded-provenance.md`
- `PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
- `PHASE_0_PERFORMANCE_AND_SECURITY_BUDGET_v0.2.md`
- `PHASE_0_GATE_REPORT.md`

## Primary question

Does v0.4 close the sole remaining B-01A bounded-admission defect without reopening any prior blocker?

## Required attacks

Assume `window_width = 2`, frontier `n`.

Test:

1. `n+1, n+2, n` versus `n, n+1, n+2`.
2. All eligible higher slots occupied before frontier arrives.
3. `n+2` physically arrives after frontier advances but carries the old window token.
4. Retry `n+2` with the new window token.
5. Same-ordinal conflict in opposite arrival order.
6. Early fence attempt before all STAGED acknowledgements.
7. Partial-prefix fence, window slide, and retained higher staged ordinal.
8. Poisoned slot recovery/reset.
9. Service versus embedded versus concurrent-worker delivery.
10. Transport queue exhaustion must remain outside logical canonical staging.

A pass requires the same logical stage/fence/finalization result for the same sequencer protocol actions independent of command delivery order within declared resource bounds.

## Regression checks

Confirm:

- B-02 remains closed;
- B-03 remains closed;
- no previous major is reopened;
- the matrix table renders R-001 through R-108 under one header;
- no new causal primitive was introduced.

## Required output

Return exactly:

### 1. VERDICT
- `PASS_PHASE_0`
- `REVISE_PHASE_0`
- `ESCALATE_TO_OPERATOR`

### 2. B-01A CLOSURE
`CLOSED` or `OPEN`.

### 3. B-02 REGRESSION CHECK
`CLOSED` or `REOPENED`.

### 4. B-03 REGRESSION CHECK
`CLOSED` or `REOPENED`.

### 5. REMAINING BLOCKERS

### 6. MAJORS / MINORS

### 7. PRIMITIVE-SUFFICIENCY RESULT

### 8. PHASE-1 AUTHORIZATION
Exactly `YES` or `NO`, then concise rationale.

## Artifact handling

Save canonical evidence as:

```text
PHASE_0_CODEX_CLOSURE_REVIEW_2026-08-25.md
```

in the current Phase-0 repository directory.

Copy the identical file to:

```text
~/Downloads/PHASE_0_CODEX_CLOSURE_REVIEW_2026-08-25.md
```

Repository copy is canonical. Downloads is disposable transfer copy.

If Git is clean apart from the new review artifact, commit only that evidence file.

Then stop.
