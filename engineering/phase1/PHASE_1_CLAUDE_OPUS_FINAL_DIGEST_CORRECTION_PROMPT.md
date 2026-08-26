# S.P.A.R.K. Phase 1 — Claude Opus Final Digest Correction

Model: Opus
Effort: HIGH
Mode: bounded implementation correction only.

Work autonomously. Do not ask for routine permission.

Read:
1. engineering/phase1/PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md
2. engineering/phase1/PHASE_1_REFOUNDATION_V2_IMPLEMENTATION_REPORT_2026-08-26.md
3. engineering/phase1/PHASE_1_CODEX_REFOUNDATION_V2_INDEPENDENT_REVIEW_2026-08-26.md
4. engineering/phase1/PHASE_1_FINAL_DIGEST_CORRECTION_BRIEF_v0.1.md

Implement the brief exactly.

Test first: add the hidden-evidence digest-collision tests and demonstrate they fail against
the inherited implementation before changing production code.

Do not redesign architecture. Do not touch Phase 2. Do not weaken closed B-02/B-01/B-04,
M-01/M-02/M-03/M-04, dependency hygiene, reconstruction restriction, or Fable crate boundaries.

When complete, create:
engineering/phase1/PHASE_1_FINAL_DIGEST_CORRECTION_REPORT_2026-08-26.md
and identical:
~/Downloads/PHASE_1_FINAL_DIGEST_CORRECTION_REPORT_2026-08-26.md

Final operator output only:
PHASE_1_FINAL_DIGEST_CORRECTION_STATUS
commit hash(es)
baseline hidden-evidence test result
final test count/result
fmt
clippy
all-features clippy
strict lint
Windows static checks
Android static checks
canonical report path
Downloads report path
genuine deviation/blocker
PHASE_2_AUTHORIZATION: NO
