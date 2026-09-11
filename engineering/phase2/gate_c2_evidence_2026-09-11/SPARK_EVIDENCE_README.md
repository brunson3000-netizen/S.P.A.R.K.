# S.P.A.R.K. Gate C2 — implementation evidence (2026-09-11)

Writer evidence for the Phase-2 implementation candidate on
`candidate/phase2-gate-c2-implementation-20260911`, based on production
`7e3a0aae069a8bf840e4dfb74221cdf2687b1db5`. Every file below was produced in this session
on the tree of checkpoint 5 (the later documentation commit changes no code or test).

Environment: Linux x86_64, 4 CPUs, rustc 1.98.0 (88d9e12ae 2026-08-18), cargo 1.98.0
(797e8a9bc 2026-08-05), `--offline` with the repository's cached dependencies.

## Contents

| File | Command | Result |
|---|---|---|
| `baseline/inherited-workspace-tests.txt` | `cargo test --workspace --all-features --offline` on the unmodified baseline | 232 passed, 0 failed, 0 ignored |
| `baseline/SPARK_BASELINE_README.md` | — | red-first record: every AT-I entry inexpressible against the inherited tree |
| `validation/workspace-tests.txt` | `cargo test --workspace --all-features --offline` | **320 passed, 0 failed, 0 ignored** (232 inherited + 88 new) |
| `validation/fmt.txt` | `cargo fmt --all --check` | exit 0 |
| `validation/clippy.txt` | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `validation/strict.txt` | `cargo clippy -p spark-core -p spark-engine --lib -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::indexing_slicing -D clippy::arithmetic_side_effects` | exit 0 |
| `validation/metadata.txt` | `cargo metadata --format-version 1` | exit 0 |
| `validation/static-targets.txt` | `cargo check --workspace --all-targets --target …` for `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`, `x86_64-pc-windows-gnu`, `x86_64-pc-windows-msvc` | all five exit 0 — static compile coverage only, no executable parity |
| `validation/workload.txt` | `cargo run --release --offline -p spark-testkit --example gate_c2_workload` | see below |

`git diff --check 7e3a0aa HEAD` passed over the committed range.

## Long-history workload (acceptance pin 2) — measured, not a benchmark

One run on one developer machine, release profile. Nanoseconds per call; the history is
built through the public `Engine::process`.

| Finalized history | Preflight only (P-9 refusal; all rows, PX-1 lookups) | Forbidden scan of P-7…P-9 (reference only) | Whole successful finalization | History build |
|---:|---:|---:|---:|---:|
| 1 000 | 2 060 | 6 077 | 846 777 | 0.35 s |
| 4 000 | 2 380 | 35 529 | 3 773 965 | 7.5 s |
| 16 000 | 2 434 | 1 135 395 | 14 572 049 | 123.4 s |

**Reading.** The production preflight is effectively flat across a 16× history increase
(≈2.1–2.4 µs, consistent with `BTreeMap` lookups), while the scan it replaces grows by more
than two orders of magnitude. **Whole finalization still grows linearly with history**: the
unchanged Phase-1 `TimelineIngress::submit_fence` builds a `FinalizationResult` whose
`canonical_history_digest()` re-hashes the entire finalized history on every fence, and the
build time is therefore quadratic. That cost is inherited, not introduced by Gate C2, and is
not attributable to the preflight; it is recorded as a limitation for the later performance
gate. Every cohort's pre-wave `engine_state_digest` likewise re-hashes the full timeline and
state store (the frozen v1 §8 composition). No production performance claim is made.

## Nonclaims

No durable storage, process-crash recovery, durable mailbox, transport, or executable
Windows/Android replay is implemented or tested; snapshots are in-memory values. The test
suites are Phase-2 acceptance tests against this implementation, not independent review.
