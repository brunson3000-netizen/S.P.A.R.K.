# S.P.A.R.K. Gate C2 — decay-write resolution evidence (2026-09-12)

Writer evidence for `candidate/phase2-gate-c2-decay-write-resolution-20260912`. The branch
continues directly from the Operator mission checkpoint
`a3227eb423e75851b05b9992dfde6424a7d8932d`, whose parent is the controlling independent
review `e00f248e25f6d34f1041e219f74085649714c19d`. This directory cannot record its own
commit's hash; the publication receipt supplies it.

Every check ran fresh on the code-and-test tree of
`b1608ddc4470d73b9c9eaf5ce26c61697000be38` — the last commit that changes any crate file.
Later commits add documentation and this evidence only. Environment: Linux x86_64, the
workspace toolchain (cargo/rustc 1.98.0), `--offline` where noted. Builds are
storage-bounded exactly as in the controlling review (`CARGO_PROFILE_DEV_DEBUG=0`,
`CARGO_PROFILE_TEST_DEBUG=0`, `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=2`); these settings
change no source, assertion or optimization level. Captured `.txt` logs are
whitespace-normalized (trailing spaces and trailing blank lines removed); command output is
otherwise unchanged.

## Scripts

| File | Purpose |
|---|---|
| `run_validation.py` | the 15 prescribed checks → `checks.json` and `<name>.txt` |
| `check_preservation.py` | inherited preservation checks plus this pass's four: `spark-core` untouched, exactly four changed crate paths with a single replaced production line, the implementation contract recorded alone before implementation and unchanged since, and additions only under `engineering/` → `preservation.json` |
| `run_corpus.py` | runs a review corpus in a disposable external crate. The runner exits 0 after capture; the cargo result in the log controls the disposition |
| `run_mutation_controls.py` | 17 compiled wrong implementations: the preceding pass's 10 with byte-identical patches, plus 7 new ones for this resolution → `mutation-controls.json`, `mutant-<id>.txt` |
| `own_probes_resolution_adapted.rs` | the controlling review's `own_probes.rs` with exactly one expected-value literal changed (see below); nothing else differs |
| `capture_custody.py` | local/tracking/live pins for every controlling ref and this branch, plus worktree states → `custody.json` |
| `check_evidence.py` | asserts every total, disposition, source hash, the one-literal adaptation, mutant kills and preservation → `evidence-summary.json` |

## Results

| Evidence | Result |
|---|---|
| `workspace-tests` | **415 passed, 0 failed, 0 ignored** (15 in `phase2_decay_write_resolution.rs`, 10 in `phase2_decay_adjudication.rs`) |
| `fmt`, `clippy`, `strict`, `metadata`, all three `whitespace-*` ranges | all exit 0 |
| five static targets | all exit 0 (static compile coverage only) |
| `preservation` | exit 0, no failures; 17 inline Phase-1 test modules and all 627 pre-existing `engineering/` files intact |
| `workload` | exit 0; see the report §8 |
| `controlling-review-own-probes` | byte-unchanged: 8 passed, **1 expected failure** — `own_open_lost_prewrite_step`, the superseded observation |
| `controlling-review-own-probes-resolution-adapted` | 9 passed, 0 failed |
| `oracle-revision-review-probes-decision-adapted` | 9 passed, 0 failed |
| `mutation-controls.json` | 32 baseline tests pass; **17/17 mutants compiled and killed**, 45 assertion kills; both specified prior-test survivals hold |

## The one adapted literal

The controlling review's `own_open_lost_prewrite_step` encoded the behavior the Operator has
now decided against. It iterates `[(vec![20],95),(vec![10,20],85)]`: with no decay evaluation
before the `+5` at 15 it expected `(105, 15)` then `(95, 20)`, and with an evaluation at 10
first it expected `(95, 15)` then `(85, 20)`. Under the resolution both histories give the
same numbers, so the adapted copy changes the single literal `95` to `85` — one character
pair, one line, nothing else. The original stays byte-identical in the review's own evidence
directory as history. Its failure above is classified by the actual cargo result and the
failing assertion, never by the capture wrapper's exit code, and it does not count as a pass
against any superseded requirement.

## Nonclaims

These are finite tests, source analysis and one-machine measurements on a four-core Linux
host under contemporaneous build load. They are not formal proofs, not a production latency
guarantee, and not a runtime parity claim for Windows or Android — those five targets are
static compile checks only. Replay and snapshot/restore are in memory; there is no durable
persistence, crash recovery, mailbox, transport, service or G.A.M.E. integration. The
writer's own checks are not an independent review.
