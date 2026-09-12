# S.P.A.R.K. Gate C2 — C2W-01 correction evidence (2026-09-12)

Writer evidence for `candidate/phase2-gate-c2-w01-correction-20260912`. The branch continues
directly from the controlling independent review
`e39690dc51b63fa09fe7c3f30ebd48aa2c16686f`, whose parent is the reviewed candidate
`cc3dc70182b428f7cbc953ae722f85b39d407dfa`. This directory cannot record its own commit's
hash; the publication receipt supplies it.

Every check ran fresh on the code-and-test tree of the last commit that changes any crate
file. Environment: Linux x86_64, cargo/rustc 1.98.0, `--offline` where noted. Builds are
storage-bounded as in every preceding pass (`CARGO_PROFILE_DEV_DEBUG=0`,
`CARGO_PROFILE_TEST_DEBUG=0`, `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=2`); these settings
change no source, assertion or optimization level. Captured `.txt` logs are
whitespace-normalized; command output is otherwise unchanged.

## Scripts

| File | Purpose |
|---|---|
| `run_validation.py` | the 15 prescribed checks → `checks.json` and `<name>.txt` |
| `check_preservation.py` | inherited preservation checks plus this pass's: `spark-core` untouched, exactly two changed crate paths with a single replaced production line, `decay_walk`/`baseline_of`/`settled`/`settlement_operation`/`effects::reduce` byte-identical, the contract correction recorded alone before implementation and unchanged since, and every historical record and published evidence directory intact → `preservation.json` |
| `run_corpus.py` | runs a corpus byte-unchanged in a disposable external crate. The runner exits 0 after capture; the cargo result in the log controls the disposition |
| `run_mutation_controls.py` | 20 compiled wrong implementations: 16 carried forward with byte-identical patches, plus 4 new C2W-01 controls → `mutation-controls.json`, `mutant-<id>.txt` |
| `capture_custody.py` | local/tracking/live pins for every controlling ref and this branch, plus worktree states → `custody.json` |
| `check_evidence.py` | asserts every total, disposition, source hash, mutant kill and preservation claim → `evidence-summary.json` |

## Results

| Evidence | Result |
|---|---|
| **C2W-01** | The controlling review's `own_probes.rs` runs **byte-unchanged and passes 9/9**. Its two blocking vectors — `own_composed_clamp_must_settle` and `own_composed_scale_must_settle_recovery` — now produce the required `95@15`/`85@20` and `-95@15`/`-85@20` |
| `workspace-tests` | **427 passed, 0 failed, 0 ignored** (12 new in `phase2_decay_write_composition.rs`, 15 in `phase2_decay_write_resolution.rs`, 10 in `phase2_decay_adjudication.rs`) |
| `fmt`, `clippy`, `strict`, `metadata`, all three whitespace ranges | all exit 0 |
| five static targets | all exit 0 (static compile coverage only) |
| `preservation` | exit 0, no failures; 17 inline Phase-1 modules and all **782** pre-existing `engineering/` files intact, including all four published evidence directories |
| `workload` | exit 0; see the report §7 |
| `reviewer-materialization-probe` | 1 passed, 0 failed |
| `historical-own-probes` | byte-unchanged: 8 passed, **1 expected failure** at `own_open_lost_prewrite_step`, the superseded observation — its disposition is unchanged from the preceding pass |
| `resolution-adapted`, `decision-adapted` | 9/9 each |
| `mutation-controls.json` | 42 baseline tests pass; **20/20 mutants compiled and killed**, 57 assertion kills; both specified prior-test survivals hold |

## One prior mutant replaced, not dropped

`RES-settle-composed-body` patched `let mut v = i128::from(current.unwrap_or(0));`, the exact
line this correction replaces. Its patch can no longer match, so re-running it unchanged
would produce a compile failure that must not be counted as a kill. Its wrong implementation
— settling every composed body — is carried forward as the distinct new mutant
`W01-settle-every-body`, which is killed by `transform_only_paths_are_not_additive_events`.
`check_evidence.py` asserts both that the old id is absent and that the new one is present.

## Nonclaims

These are finite tests, source analysis and one-machine measurements on a four-core Linux
host under contemporaneous build load. They are not formal proofs, not a production latency
guarantee, and not a runtime parity claim for Windows or Android — those five targets are
static compile checks only. Replay and snapshot/restore are in memory; there is no durable
persistence, crash recovery, mailbox, transport, service or G.A.M.E. integration. The
writer's own checks are not an independent review.
