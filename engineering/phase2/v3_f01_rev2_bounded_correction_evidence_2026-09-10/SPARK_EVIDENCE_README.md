# S.P.A.R.K. V3-F01 Revision-2 writer evidence — 2026-09-10

Writer evidence for
`engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_REV2_2026-09-10.md`,
based on review commit `5b19d7b5aa935b65fcfad1d8bda9a210d5a5684d`. It is architecture
evidence, not Phase-2 acceptance evidence and not crash-recovery evidence. No workspace
crate, manifest, test, or lockfile is changed.

## Contents

- `model.py` — disposable standard-library model (58 checks: 34 carried from the FINAL model,
  24 new). Run from this directory: `python3 model.py results.json > results.txt`.
  `results.txt` and `results.json` are its output; two runs were byte-identical.
- `rev2_finalization_probe.rs` and `run_rev2_probe.py` — the Revision-2 finalization
  preflight implemented over the **unchanged** Phase-1 `TimelineIngress` public API, built
  as a temporary external Cargo package that depends on `crates/spark-core` with its
  `test-support` feature (used only for the `u64::MAX` frontier states). Run from the
  repository root: `python3 engineering/phase2/v3_f01_rev2_bounded_correction_evidence_2026-09-10/run_rev2_probe.py`.
  It requires cached Cargo dependencies (`--offline`), runs the probe, and then requires the
  `--negative-control` run (the old stage-then-fence composition) to fail. `rev2-probe.txt`
  is its output, captured after the probe was formatted with `rustfmt`.
- `historical-red-probe-rerun.txt` — fresh output of the review's unchanged
  `v3_f01_final_independent_review_evidence_2026-09-10/run_regressions.py`, still confirming
  the FINAL-01/02/03 defects against the FINAL candidate. Preserved as historical red evidence.
- `workspace-tests.txt` (`cargo test --workspace --offline`: 232 passed, 0 failed, 0 ignored),
  `clippy.txt`, `strict.txt`, `static-targets.txt` (five `cargo check --target` runs,
  compile coverage only). `cargo fmt --all --check` and `cargo metadata --format-version 1`
  produced no output and exited 0. Logs have trailing whitespace and terminal blank lines
  trimmed to satisfy `git diff --check`; content is otherwise verbatim.

## Full-state comparison

The probe compares `format!("{t:?}")` of `TimelineIngress` — whose derived `Debug` renders
every private field, including the staged and finalized identity indexes and
`last_finalized_source_sequence` — together with `canonical_state_digest` and
`canonical_history_digest`. The model compares every field of its Phase-1 mirror
(`Timeline.full_state()`).

## Limits

The model's hashes are truncated SHA-256 over JSON, not the production canonical encoder,
and its poison evidence ignores the Phase-1 caps. Snapshots are deep copies. The probe is a
disposable fixture, not a Phase-2 engine. No Windows/Android runtime, durable mailbox, or
process-crash behavior was executed. Supplementary compute was not used.
