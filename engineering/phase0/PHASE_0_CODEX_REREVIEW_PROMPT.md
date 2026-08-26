# S.P.A.R.K. Phase 0 — Independent Re-Review Prompt

## Role

You are the independent architecture reviewer for the corrected S.P.A.R.K. Phase 0 package. The S.P.A.R.K. engineer authored the corrections; you did not.

**Effort level: HIGH.**

Do not implement Rust. Do not rewrite the artifacts. Review/falsify only.

## Evidence chain

Review:

1. `CONTROLLING_BLUEPRINT_v0.2.md`
2. `PHASE_0_CODEX_INDEPENDENT_REVIEW_2026-08-25.md` — the prior independent review
3. `PHASE_0_CORRECTION_REPORT_2026-08-25.md`
4. `ADR-0001-rust-workspace-and-dependency-direction.md`
5. `ADR-0002-authority-and-host-acknowledgement.md`
6. `ADR-0003-deterministic-clock-rng-and-commit-order.md`
7. `ADR-0004-profile-schema-validation-and-change-classes.md`
8. `ADR-0005-service-and-embedded-semantic-equivalence.md`
9. `ADR-0006-persistence-versioning-and-bounded-provenance.md`
10. `PHASE_0_PERFORMANCE_AND_SECURITY_BUDGET_v0.2.md`
11. `PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
12. `PHASE_0_GATE_REPORT.md`

## Primary objective

Determine whether the corrected Phase 0 is now sufficiently specified to authorize:

```text
Phase 1 — Rust core skeleton
```

Do not demand ordinary implementation details that the blueprint intentionally leaves provisional. Do identify any remaining ambiguity that would force Phase 1 to invent canonical semantics.

## Required blocker closure attacks

### B-01 — Canonical input transaction boundaries

Attempt to break:

- canonical command envelope;
- effective logical time;
- source identity/source sequence;
- total-order input ordinal;
- idempotency/duplicate conflict handling;
- same-time ordering;
- command barrier/snapshot closure;
- transport batch partition independence;
- concurrent service request independence;
- `advance_time` chunk/quota invariance;
- occurrence indexing;
- mid-wave profile/config activation.

A passing contract must make implicit arrival-order semantics impossible.

### B-02 — Immutable authority

Attempt to convert a persisted definition among:

```text
host_owned
spark_owned
derived
```

through:

- hot tune;
- structural reload;
- alias;
- migration;
- restore;
- cross-profile reference;
- MCI/inspection paths.

A passing contract must require a new ID + explicit migration + authority ADR + operator approval for authority/write-class change and must prevent fabricated host/derived truth.

### B-03 — Exact save/behavior-artifact binding

Attempt to break old-save continuation through:

- same version label/different content;
- missing old manifest/config;
- delayed obligation after profile replacement;
- obligation rule ID reused with changed rule content;
- failed migration;
- outcome replay substituted for mechanism replay;
- snapshot during propagation.

A passing contract must either continue under exact supported artifacts, perform an explicit atomic migration, or fail explicitly.

## Re-check prior majors/minors

Verify closure or acceptable Phase-0 disposition of M-01 through M-08 and N-01 through N-04 from the prior review. Do not reclassify a purely provisional numeric choice as a blocker unless it creates an unbounded or semantic hole.

## Primitive sufficiency

Reconfirm whether the frozen primitive set remains sufficient after the corrections. A new primitive recommendation requires a concrete blueprint-required counterexample that cannot be represented compositionally.

## Required output

Return exactly these sections:

### 1. VERDICT

Choose one:

- `PASS_PHASE_0`
- `REVISE_PHASE_0`
- `ESCALATE_TO_OPERATOR`

### 2. B-01 CLOSURE
`CLOSED` or `OPEN`, with concise evidence.

### 3. B-02 CLOSURE
`CLOSED` or `OPEN`, with concise evidence.

### 4. B-03 CLOSURE
`CLOSED` or `OPEN`, with concise evidence.

### 5. REMAINING BLOCKERS
List only actual Phase-1-blocking defects.

### 6. MAJORS / MINORS
List remaining non-blocking findings, if any.

### 7. PRIMITIVE-SUFFICIENCY RESULT
State whether the frozen primitive set remains sufficient.

### 8. PHASE-1 AUTHORIZATION
Return exactly `YES` or `NO`, then a short rationale.

## Artifact handling

Save the complete review as the canonical repository artifact:

```text
PHASE_0_CODEX_INDEPENDENT_REREVIEW_2026-08-25.md
```

in the current Phase-0 repository directory.

Also copy the same file to:

```text
~/Downloads/PHASE_0_CODEX_INDEPENDENT_REREVIEW_2026-08-25.md
```

The repository copy is canonical evidence. The Downloads copy is disposable transfer convenience.

If this is a Git repository and the working tree is otherwise clean, commit only the new re-review artifact with an appropriate review-evidence commit message. Do not modify the corrected Phase-0 source artifacts.

Then stop.
