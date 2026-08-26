# S.P.A.R.K. Phase 1 — Claude Code Bounded Correction Mission

## Mode

**IMPLEMENTATION CORRECTION WRITER — AUTONOMOUS WITHIN SCOPE**

The independent Codex review found four blockers, four majors, and one minor in the Phase-1 implementation.

Your mission is to correct them against the already-frozen Phase-0 architecture.

Do not begin Phase 2.

## Autonomy / operator-babysitting rule

Work autonomously.

You are pre-authorized to:

- inspect all S.P.A.R.K. repository files and Git history;
- edit/refactor Phase-1 Rust code and tests;
- add/remove local free Rust dependencies when justified and permissively licensed;
- run Cargo/rustup checks that do not require paid/external credentials;
- create adversarial tests;
- run format/lint/test/metadata/cross-target checks;
- make coherent local Git commits;
- create the final correction report;
- copy the report to Downloads.

Do **not** repeatedly ask the operator for ordinary permission.

Stop only for:

- frozen architecture change;
- new causal primitive requirement;
- destructive action affecting unrelated operator data;
- paid/cloud/credentialed operation;
- unresolved conflicting repository work;
- material scope expansion beyond Phase 1.

## Read first

1. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md`
2. accepted Phase-0 ADRs
3. `engineering/phase0/PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
4. `engineering/phase0/PHASE_0_CODEX_CLOSURE_REVIEW_2026-08-25.md`
5. `engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md`
6. `engineering/phase1/PHASE_1_CLAUDE_IMPLEMENTATION_REPORT_2026-08-25.md`
7. `engineering/phase1/PHASE_1_CODEX_INDEPENDENT_REVIEW_2026-08-26.md`
8. `engineering/phase1/PHASE_1_CORRECTION_BRIEF_v0.1.md`

The independent review and correction brief override the original writer's self-assessment where they conflict.

## Mission

Implement **every correction in `PHASE_1_CORRECTION_BRIEF_v0.1.md`**.

Treat B-01 through B-04 and M-01 through M-04 as Phase-2 gate requirements, not optional cleanup.

Also close m-01 unless doing so would itself cause a larger architectural defect.

## Engineering posture

Prefer smaller, structurally safe APIs over public methods protected only by naming/comments.

Prefer:

- immutable activated schema;
- crate-private mutation seams;
- typed/validated canonical constructors;
- explicit semantic keys;
- pure/candidate validation before activation;
- checked arithmetic;
- profile-qualified identity;
- domain-separated canonical encoding.

Do not patch around failing adversarial tests with special cases.

## Independent-review awareness

Codex will re-run adversarial tests and may create new ones.

Do not optimize for the names of its current tests; fix the underlying property.

## Git discipline

Start by confirming:

```bash
git status --short
git log --oneline --decorate -8
```

The independent review artifact is evidence and must not be rewritten.

Keep commits coherent and the final tree clean.

## Completion gate

Before reporting COMPLETE:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo metadata --format-version 1
```

Rerun installed Windows/Android target checks.

The final report must be canonical in the repository and copied identically to Downloads.

## Required final response to operator

Return only:

- `PHASE_1_CORRECTION_STATUS: COMPLETE` or `BLOCKED`
- correction commit hash(es)
- blockers closed count
- majors closed count
- minor closed count
- fmt result
- clippy result
- test result and total count
- Windows check status
- Android check status
- canonical report path
- Downloads report path
- genuine deviation/blocker
- `PHASE_2_AUTHORIZATION: NO`

Then stop.
