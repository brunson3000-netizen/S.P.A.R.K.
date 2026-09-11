# S.P.A.R.K. Gate C2 — second bounded-correction evidence (2026-09-11)

Writer evidence for `candidate/phase2-gate-c2-decay-oracle-revision-20260911`. The branch
is based directly on the controlling revision review commit
`3a0b51463e1874ad8b02dd3a3261933fcd2e22f4`, the preceding evidence commit; this directory
cannot record its own commit's hash.

Every check ran fresh in this session on the code-and-test tree of checkpoint 1,
`5ae506adf13ddf1d610595c59202c52b7ddee81a`. The final documentation commit changes no code
or test. Environment: Linux x86_64, the workspace toolchain, `--offline` where noted.

Captured `.txt` logs are whitespace-normalized (trailing spaces and trailing blank lines
removed) so the committed range passes `git diff --check`; command output is otherwise
unchanged.

## Scripts

| File | Purpose |
|---|---|
| `run_validation.py` | reruns every prescribed check; writes `checks.json` (command, exit code, seconds) and one `<name>.txt` per check, each headed by the tree hash |
| `run_corpus.py` | runs a review corpus byte-unchanged (sha256 in the log header) in a disposable external crate against a chosen crate root; `--test-support` only where that corpus requires the fixture seams |
| `run_mutation_controls.py` | seven compiled wrong implementations, each in a disposable `git archive HEAD` copy; records kills (assertion failures in named tests) and survivals (named prior tests) → `mutation-controls.json`, `mutant-<id>.txt` |
| `check_preservation.py` | inherited tests, manifests, lockfile, 17 inline Phase-1 test modules and protected functions byte-identical; `spark-core` purely additive against the review and confined to the two `test-support`-gated items; lineage; worktrees → `preservation.json` |

## Results

| Evidence | Result |
|---|---|
| `revision_probes_pre_correction.txt` | second review's probes against the unchanged candidate code (before any change): **11 passed, 3 failed** — the review's three C2R contract failures, reproduced |
| `revision_probes_post_correction.txt` | the same probes against this candidate: **13 passed, 1 failed**. All three C2R contract probes pass. The failure is the reviewer's positive control `c2_05_recovery_and_unsaturated_remainders_positive`: its value (±45) matches, but it also asserts the backdated `LogicalTime(40)` that C2R-01 rejects (the canonical time is 42) |
| `original_corpus_post_correction.txt` | first review's original corpus, unchanged: fails to compile only at the three surfaces the first revision removed (external refusal construction, the report's cohort-identity field, the decay `toward` shape) |
| `adapted_corpus_post_correction.txt` | first review's adapted corpus (A1–A4): **9 passed, 0 failed** |
| `mutation-controls.json` | baseline 12/12 pass; **7/7 mutants killed** by assertion failures; both named prior tests survive their mutants (the gaps the review identified) |
| `fmt` | exit 0 |
| `workspace-tests` | **390 passed, 0 failed, 0 ignored** |
| `clippy` (all targets, all features, `-D warnings`) | exit 0 |
| `strict` (core/engine library: unwrap, expect, panic, indexing, arithmetic denied) | exit 0 |
| `metadata` | exit 0 |
| `whitespace-candidate-range`, `whitespace-production-range` | exit 0 |
| five static targets (`x86_64-pc-windows-gnu`, `x86_64-pc-windows-msvc`, `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`) | all exit 0 — static compile coverage only |
| `preservation` | **first run exit 1, a checker defect** (`preservation-first-run-checker-defect.txt`). The new fifth protected-function row compared `derived_indexes_consistent` with production, where it does not exist (it was added by accepted Gate C2 checkpoint `aad4633`). The corrected check compares it with the review commit and the four Phase-1 functions with production: exit 0, no failures (`preservation.txt`). No source changed between the runs |
| `workload` | exit 0 (one release run, one developer machine): 1k / 4k / 16k finalized commands → history build 0.398 / 7.251 / 137.480 s; indexed P-9 preflight 2 361 / 2 431 / 2 405 ns per call (flat); forbidden-scan reference 5 848 / 38 494 / 1 206 013 ns; whole successful finalization 1.10 / 4.04 / 14.91 ms (linear in history through the unchanged Phase-1 `submit_fence`, C2-09). Not a benchmark |

## Nonclaims

These are finite tests, source analysis and one-machine measurements. They are not formal
verification, a production benchmark, or executable Windows/Android parity. There is no
durable storage, crash recovery, durable mailbox, transport, service, multi-profile
runtime, definition migration, or G.A.M.E. integration. The tests are the writer's; the
independent review has not yet run.
