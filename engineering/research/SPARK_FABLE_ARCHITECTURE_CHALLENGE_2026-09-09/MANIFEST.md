# Manifest — inspected baseline, isolated work location, inventory, checksums, commits

**RESEARCH / PROPOSED DESIGN — NOT ADOPTED.** 2026-09-09.

## Inspected baseline

| Item | Value |
|---|---|
| Production repository | `/home/chromikey/Projects/SPARK` |
| Production branch | `phase1-refoundation-v2` |
| Production HEAD at mission start and end | `47729bb719d8ae95ae00c4e195fa64d0e27bfdb7` |
| Production worktree | clean at start; **0 changed files at end** (verified with `git status`) |
| Baseline tests (production worktree) | `cargo test --workspace --all-features`: 232 passed, 0 failed |
| Controlling documents read | `engineering/PHASE_STATUS.md`, `engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md`, `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md` (§3, §11.3, §17–§19), Phase-2 freeze v3 (§3–§4), V3-F01 V2 independent review (§1, §4), serialized-request boundary addendum (whole), `serialized_request_model_2026-09-06/model.py` (head) |
| Source read | `crates/spark-core/src/{lib,scheduler,timeline,clock,hash,scope,id}.rs`, `crates/spark-engine/src/{lib,state}.rs`, crate manifests, workspace `Cargo.toml` |

## Isolated work location

| Item | Value |
|---|---|
| Worktree | `/home/chromikey/Projects/SPARK-fable-challenge-worktree` |
| Branch | `research/fable-architecture-challenge-2026-09-09` |
| Created from (parent) | `47729bb719d8ae95ae00c4e195fa64d0e27bfdb7` |
| Research commit 1 | `08f5469b239f9f239bed262040a00c31e8b9d5cc` — "Record Fable architecture challenge research" (parent `47729bb…`) |
| Research commit 2 | records this manifest; its hash is stated in the completion message and in the archive's `POST_COMMIT.txt` (a file cannot contain its own commit hash) |
| Push / merge | none; branch is local only |

## Changed-file inventory (research branch versus `47729bb`)

| Path | Change |
|---|---|
| `crates/spark-core/src/scheduler.rs` | +10 lines: research-only read-only accessor `Scheduler::next_due_time()`; also captured as `scheduler_accessor.patch` |
| `.gitignore` | +2 lines: ignore the example's `target/` |
| `engineering/research/SPARK_FABLE_ARCHITECTURE_CHALLENGE_2026-09-09/OPERATOR_SUMMARY.md` | new |
| `…/00_FIRST_PRINCIPLES_SKETCH.md` | new |
| `…/01_BASELINE_AND_CRITIQUE.md` | new |
| `…/02_FABLE_DESIGN.md` | new |
| `…/03_EXAMPLE_RESULTS.md` | new |
| `…/04_SELF_CHALLENGE.md` | new |
| `…/05_ROUTE_AND_PLAN.md` | new |
| `…/MANIFEST.md` | new (this file) |
| `…/scheduler_accessor.patch` | new |
| `…/brief_for_dev_compute.md` | new |
| `…/example/Cargo.toml`, `…/example/Cargo.lock`, `…/example/src/lib.rs`, `…/example/src/main.rs`, `…/example/results.txt` | new |
| `…/dev_compute_evidence/` (NOTE.md, orchestrate stdout/stderr, panel records) | new; gateway returned `blocked`, no worker output |
| `…/dev_compute_attempt1_failed/` | new; first attempt failed on a pre-existing out-dir |

No file under `engineering/phase0`, `engineering/phase1`, `engineering/phase2`,
`engineering/PHASE_STATUS.md`, or `engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` was
modified. No G.A.M.E. or S.W.A.R.M. file was touched.

## Verification performed in the research worktree

- `example/`: `cargo build` 0 warnings; `cargo clippy` 0 warnings; `cargo test` 2 passed;
  `cargo run` 13 PASS / 0 FAIL (`example/results.txt`).
- Workspace: `cargo test --workspace --all-features` with the accessor present — result
  recorded in `POST_COMMIT.txt`.

## Checksums

SHA-256 of every file in this directory (excluding `example/target/`) is listed in
`SHA256SUMS.txt`, generated immediately before the archive was created.
