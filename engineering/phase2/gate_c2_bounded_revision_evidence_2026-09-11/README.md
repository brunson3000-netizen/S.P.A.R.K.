# S.P.A.R.K. Gate C2 — bounded-revision evidence (2026-09-11)

Writer evidence for `candidate/phase2-gate-c2-bounded-revision-20260911`, based on the
controlling independent review commit `00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0`
(the preceding evidence commit; this directory cannot record its own commit's hash).

Every check ran fresh in this session on the code-and-test tree of checkpoint 2
(`19930f3c17a923c873b55cd5afb58692589d0681`); the final documentation commit changes no
code or test. Environment: Linux x86_64, rustc/cargo 1.98.0, `--offline` where noted.

Every captured `.txt` log here is whitespace-normalized after capture (trailing spaces —
libtest prints `test name ... ` before a panic message — and trailing blank lines removed)
so the committed range passes `git diff --check`; command output is otherwise unchanged.

## Scripts

| File | Purpose |
|---|---|
| `run_validation.py` | reruns every prescribed check; writes `checks.json` (command, exit code, seconds) and one `<name>.txt` per check, each headed by the tree hash |
| `run_original_corpus.py` | runs the review's original corpus byte-unchanged (sha256 in the log header) in a disposable external crate against a chosen crate root; optionally the documented adapted variant with `--test-support`; logs are whitespace-normalized and say so |
| `check_preservation.py` | inherited tests, manifests, lockfile, 17 inline Phase-1 test modules and four protected functions byte-identical to production; spark-core additive relative to the review commit; lineage; worktrees → `preservation.json` |
| `independent_counterexamples_adapted.rs` | the corpus with adaptations A1–A4, each explained in its header |

## Results

| Check (`<name>.txt`) | Result |
|---|---|
| `fmt` | exit 0 |
| `workspace-tests` | **374 passed, 0 failed, 0 ignored** |
| `clippy` (all targets, all features, `-D warnings`) | exit 0 |
| `strict` (core/engine library, unwrap/expect/panic/indexing/arithmetic denied) | exit 0 |
| `metadata` | exit 0 |
| five static targets (`x86_64-pc-windows-gnu`, `x86_64-pc-windows-msvc`, `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`) | all exit 0 — static compile coverage only |
| `original_corpus_pre_correction.txt` | unchanged corpus against the unchanged review worktree: **3 passed, 7 failed** (reproduces the review) |
| `original_corpus_post_correction.txt` | unchanged corpus against this candidate: fails to compile only at `FinalizationRefusal::WrongProfile` construction, `report.cohort_identity`, and `Update::Decay { toward, … }` — the surfaces C2-08, C2-07 and C2-05 remove |
| `adapted_corpus_post_correction.txt` | adapted corpus: **9 passed, 0 failed** |
| `preservation` | no failures |
| `whitespace-*` | see below |
| `workload` | see below |

### Whitespace

The first validation pass's two `git diff --check` ranges failed **only** on raw corpus
logs committed in checkpoint 1 (libtest progress lines ending in a space, a trailing blank
line). The runner now normalizes captured output; the three corpus logs were regenerated
(the pre-correction run again against the untouched review worktree at `00d647e`), and
both ranges were re-checked on the exact staged final tree: `whitespace-final.txt`.

### Workload

One release run on one developer machine (`workload.txt`); nanoseconds per call, history
built through the public `Engine::process`.

| Finalized history | Build (s) | P-9 refusal path (preflight) | Forbidden scan (reference) | Whole successful finalization |
|---:|---:|---:|---:|---:|
| 1 000 | 0.44 | 3 539 | 10 659 | 1 206 826 |
| 4 000 | 9.95 | 2 508 | 43 577 | 4 027 551 |
| 16 000 | 129.86 | 2 510 | 1 141 488 | 14 371 205 |

The indexed preflight stays flat (~2.5 µs at 4k and 16k, including `process` overhead;
the 1k figure is higher, 3.5 µs, and does not grow with history) while the scan it replaces grows ~100×; whole finalization still grows
linearly with history through the unchanged Phase-1 `submit_fence` (C2-09). Not a
production benchmark.

## Nonclaims

Finite tests, source analysis and one-machine measurements; not formal verification, not a
production benchmark, not executable Windows/Android parity. No durable storage, crash
recovery, durable mailbox, transport, service, or G.A.M.E. integration. The tests are the
writer's; the independent review has not yet run.
