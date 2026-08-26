# S.P.A.R.K. Phase 1 — Final Digest Correction Brief

Status: bounded correction authorized. Phase 2 remains unauthorized.

## Defect
Codex reports convergence of the foundational architecture, with one remaining MAJOR:
canonical scheduler/timeline conflict-state digests commit only to the exposed 16-hash
presentation projection, while the implementation retains up to 256 behavior-relevant
claim hashes that can affect future reactions.

## Required rule
Every retained internal state value that can change future canonical behavior must
participate in canonical state identity.

Presentation truncation may remain 16 hashes. Canonical state hashing must include all
behavior-relevant tracked hashes retained internally.

Apply this to:
1. scheduler conflict evidence;
2. timeline poison evidence.

Do not change finalized semantic-history rules unless required.

## Mandatory tests
Add tests proving:
- two scheduler conflict states with identical exposed 16/count/flag but different hidden
  tracked hashes have different canonical digests;
- submitting the same next hash can evolve those two states differently, proving they were
  behaviorally distinct;
- permutations of the same 17/64/256/>256 claim sets converge to identical digest/state/drain;
- analogous hidden-evidence discrimination and permutation convergence for timeline poison
  state.

Preserve all previously closed items and Phase-1 Re-Foundation v2 architecture.

## Validation
Run:
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo metadata --format-version 1
and the existing strict canonical lint gate plus installed Windows/Android static checks.

## Report
Create:
engineering/phase1/PHASE_1_FINAL_DIGEST_CORRECTION_REPORT_2026-08-26.md

Copy identical transfer file to:
~/Downloads/PHASE_1_FINAL_DIGEST_CORRECTION_REPORT_2026-08-26.md

End report with:
PHASE_2_AUTHORIZATION: NO
