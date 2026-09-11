# S.P.A.R.K. Gate C2 — decay adjudication evidence (2026-09-11)

Writer evidence for `candidate/phase2-gate-c2-decay-adjudication-20260911`. The branch is
based directly on the controlling review commit
`6b167d84f21fb60f374a2ab45c62d5c6a040790a`. This directory cannot record its own commit's
hash.

Every check ran fresh on the code-and-test tree of checkpoint 1,
`244c54b66a9c5bdfe8dd49e28b05cc0f5c255637`. The final commit changes no code or test.
Environment: Linux x86_64, the workspace toolchain, `--offline` where noted. Builds are
storage-bounded, as in the controlling review (`CARGO_PROFILE_DEV_DEBUG=0`,
`CARGO_PROFILE_TEST_DEBUG=0`, `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=2`). These settings
change no source, assertion or optimization level. Captured `.txt` logs are
whitespace-normalized: trailing spaces and blank lines are removed. Command output is
otherwise unchanged.

## Scripts

| File | Purpose |
|---|---|
| `run_validation.py` | the 14 prescribed checks → `checks.json` and `<name>.txt` |
| `check_preservation.py` | the inherited preservation checks, plus three for this pass: comment-only changes to existing crate files, the adjudication introduced alone and unchanged since, and only additions under `engineering/` → `preservation.json` |
| `run_corpus.py` | runs a review corpus byte-unchanged in a disposable external crate. The runner exits 0 after capture; the cargo result in the log controls the disposition |
| `run_mutation_controls.py` | 10 compiled wrong implementations: the prior writer's 7 with byte-identical patches, plus 3 new ones → `mutation-controls.json`, `mutant-<id>.txt` |
| `run_reviewer_mutants.py` | the controlling review's 4 own mutants against this tree, with the review's probes read byte-unchanged from its evidence → `reviewer-mutations.json`, `reviewer-*.txt` |
| `run_surface_probes.py` | the review's 17 default-feature surface controls, byte-identical copy → `surface-probes.json`, `surface-*.txt` |
| `independent_probes_decision_adapted.rs` | the review's `independent_probes.rs` with exactly two expected tuples changed, `(100,LogicalTime(10))` → `(90,…)` and `(75,LogicalTime(40))` → `(65,…)`, the superseded S-1 values; nothing else differs |
| `capture_custody.py` | local/tracking/live pins for every controlling ref and this branch, plus worktree states → `custody.json` |
| `check_evidence.py` | asserts every total, disposition, source hash, the two-line adaptation, mutant kills, surfaces and preservation → `evidence-summary.json` |

## Results

| Evidence | Result |
|---|---|
| `workspace-tests` | **400 passed, 0 failed, 0 ignored** (10 in `phase2_decay_adjudication.rs`) |
| `fmt`, `clippy`, `strict`, `metadata`, both `whitespace-*` ranges | all exit 0 |
| five static targets | all exit 0 (static compile coverage only) |
| `preservation` | exit 0, no failures |
| `workload` | exit 0; see the report §10 |
| `original-corpus` | compile failure at the same revised surfaces (E0308, E0559, E0603, E0609) |
| `adapted-corpus` | 9 passed, 0 failed |
| `prior-revision-probes` | 13 passed, 1 failed (obsolete `LogicalTime(40)`) |
| `prior-revision-time-assertion-only` | 14 passed, 0 failed |
| `independent-probes` | 7 passed, 2 failed: the two `r2prime_frozen_q7_*` assertions of the superseded reading |
| `independent-probes-decision-adapted` | 9 passed, 0 failed |
| `mutation-controls.json` | all baselines pass; 10/10 mutants killed as claimed; both named prior tests survive their mutants |
| `reviewer-mutations.json` | 4/4 baselines pass; 4/4 killed |
| `surface-probes.json` | 17/17 |

## Correction during capture

The first `check_evidence.py` run failed on its own parser. It matched failing test names
as `test <name> ... FAILED`. Under `--nocapture`, libtest interleaves panic output with
that progress line, so the parser found no names. The corrected parser reads the final
`failures:` list instead. No log, source or result changed between the two runs; only the
checker line did. This was a defect in the checker, not in the candidate.

## Nonclaims

These are finite tests, source analysis and one-machine measurements. They are not formal
verification, a production benchmark, or executable Windows/Android parity. There is no
durable storage, crash recovery, durable mailbox, transport, service, multi-profile
runtime, definition migration, or G.A.M.E. integration. The tests are the writer's; the
independent review has not yet run.
