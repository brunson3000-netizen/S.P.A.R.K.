# S.P.A.R.K. final independent review evidence — 2026-09-10

Reviewed candidate: `74d044ddd2b59c3c2eff558beeeeb515390dbe0f`.
These are reviewer counterexamples and fresh validation logs, not correction code or
Phase-2 acceptance evidence. The Rust fixture uses the existing public Phase-1 API in a
temporary external Cargo package; no workspace crate, manifest, or inherited test changes.
The symbolic source S uses the valid canonical identifier `s`.

From the repository root, run:

```sh
python3 engineering/phase2/v3_f01_final_independent_review_evidence_2026-09-10/run_regressions.py
```

Requires Python 3, Cargo/Rust and the repository's cached Cargo dependencies (`--offline`).
The runner copies Cargo.lock into a disposable package, tests actual Phase-1 stage/fence,
requires the candidate no-mutation assertion to fail as a negative control, reruns all
34 candidate model checks, and probes mismatch-driven FIFO loss and history-only paused
reconstruction. It restores a snapshot and confirms exact-request continuation as a
positive control. A successful run means the review counterexamples were reproduced.

`regressions.txt` contains the successful final probe output. An initial fixture setup
attempt used uppercase S, which the canonical ID constructor rejected; changing only the
fixture to lowercase `s` made the intended counterexample executable. This setup error
was not a candidate finding. The Rust fixture was formatted after the successful run;
formatting did not change its behavior.

Test/static-target logs omit only terminal blank lines to satisfy `git diff --check`.

Other evidence:

- `model.txt`: fresh 34-check model stdout, byte-identical to candidate results.txt;
  generated JSON independently compared equal to the candidate's results.json.
- `workspace-tests.txt`: `cargo test --workspace`, 232 passed, zero failed/ignored.
- `clippy.txt`: `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- `strict.txt`: `cargo clippy -p spark-core -p spark-engine --lib -- -D warnings
  -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic
  -D clippy::indexing_slicing -D clippy::arithmetic_side_effects`.
- `static-targets.txt`: five `cargo check --workspace --all-targets --target TARGET`
  runs, each exit 0; compile coverage only.

Additional checks returned exit 0: `cargo fmt --check`,
`cargo metadata --format-version 1`, `git diff --check 772e38d 74d044d`.
The review's staged diff whitespace/scope checks passed before commit; newly authored
repository references exist and production Rust/manifests/tests and the production tip
are unchanged. Local, tracking and live branch hashes are checked after publication. The final chat receipt
reports the resulting commit without creating a self-referential document hash.

No actual Phase-2 acceptance runtime, durable crash-recovery backend, or Windows/Android
runtime parity was available to execute. The inherited Linux suite is fresh evidence;
prior reports are not substituted for it. The writer's model uses simplified JSON and
truncated SHA-256 rather than the production canonical encoding/hash, and lacks staged
slots/source sequencing; the Rust probe is necessary for the finalization claim.
