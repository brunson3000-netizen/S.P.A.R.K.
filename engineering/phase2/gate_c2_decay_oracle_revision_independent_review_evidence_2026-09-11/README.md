# Independent review evidence — exact second Gate C2 correction

Candidate `6968a4af917c9a32761be371c3e12ec12ba0f1bd`. Reviewer: Codex, separated
from the correction writer. See the adjacent
`SPARK_PHASE_2_CODEX_GATE_C2_DECAY_ORACLE_REVISION_INDEPENDENT_REVIEW_2026-09-11.md`.

The review verdict is
`GATE_C2_DECAY_ORACLE_REVISION_BLOCKED_ON_OPERATOR_ARCHITECTURE_DECISION`.
The candidate source/test tree was unchanged throughout review. Only this directory and
the review report are published. Disposable mutant code is never a candidate repair.

## Evidence and reproduction

Run from the review worktree. Scripts locate the root from this directory. They use
storage-bounded debug builds (`CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`,
`CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=2`); these preserve all assertions and normal
debug optimization level. Release workload optimization is unchanged.

- `run_validation.py`: all fourteen canonical checks, full `*.txt` logs, `checks.json`.
  `check_preservation.py` is the inspected writer checker, rerun independently; its
  source-preservation comparisons were cross-checked against the actual candidate diff.
- `run_mutation_controls.py`: inspected writer patches rerun against a disposable archive
  of the candidate (at run time HEAD was the pinned candidate). Twelve passing baselines,
  seven compiled mutants killed and two older tests surviving their respective mutant.
- `run_independent_mutants.py`: reviewer-authored independent patches and external tests,
  pinned explicitly to the candidate. Four positive baselines and four assertion kills.
- `independent_probes.rs`: nine reviewer-authored runtime probes. Seven pass; the two
  `r2prime_frozen_q7_*` assertions fail for actual contract differences. The corresponding
  observation probe passes and records the candidate's actual alternative values.
- `run_surface_probes.py`: 17 default-feature external controls, including three new
  test-seam negatives and the prior surface corpus; exact source and diagnostics retained.
- `run_corpus.py`: reads source byte-unchanged into a disposable external crate. Its wrapper
  exit is zero after capture; the printed JSON and log contain cargo's actual result.
  Use the tuple/diagnostic expectations below, never wrapper success, as the result.
- `prior_revision_time_assertion_only.rs`: copy of the second review's probes with exactly
  one literal changed, `LogicalTime(40)` → `LogicalTime(42)`. No other assertion changes.
- `capture_custody.py`: local/tracking/live pinned refs and all worktree/branch states.
- `check_evidence.py`: asserts the recorded totals, source hashes, mutant compilation and
  assertion failures, surface diagnostics, preservation and complete oracle-row coverage;
  writes `evidence-summary.json`. `SHA256SUMS` also covers the report.

Example external-probe command (substitute source/log as listed):

```sh
python3 engineering/phase2/gate_c2_decay_oracle_revision_independent_review_evidence_2026-09-11/run_corpus.py /tmp/spark-gate-c2-decay-oracle-review independent-probes engineering/phase2/gate_c2_decay_oracle_revision_independent_review_evidence_2026-09-11/independent_probes.rs --test-support
```

| Log | Source relative to `engineering/phase2/` | test-support | Cargo disposition |
|---|---|---|---|
| original-corpus | `gate_c2_independent_review_evidence_2026-09-11/independent_counterexamples.rs` | no | 101, same three revised API surfaces; no runtime claim |
| adapted-corpus | `gate_c2_revision_independent_review_evidence_2026-09-11/independent_counterexamples_adapted.rs` | yes | 0, 9 tests pass |
| prior-revision-probes | `gate_c2_revision_independent_review_evidence_2026-09-11/independent_revision_probes.rs` | yes | 101, 13 pass / one obsolete timestamp assertion fails |
| prior-revision-time-assertion-only | this directory's `prior_revision_time_assertion_only.rs` | yes | 0, 14 pass; both signs and all subsequent assertions execute |
| independent-probes | this directory's `independent_probes.rs` | yes | 101, 7 pass / 2 frozen-contract assertions fail |

## Corrections to the review execution

`storage-exhaustion-attempt/` retains outputs from the initial concurrent default-debug
attempt. Temporary disk exhaustion caused cargo write failures, some nested compile-probe
failures, and invalid mutant results. The validation runner was stopped. Only this
review's generated `target` and its disposable mutant directory were removed. All required
checks and corpus/mutation runs were repeated with bounded build outputs; final logs are
at this directory's top level. Missing first-attempt logs mean execution did not finish
capturing them; no success is inferred. This is not a candidate-source defect.

`surface-diagnostic-first-attempt/` retains the 15/17 matcher result and two diagnostics:
rustc correctly said `cannot find type`, while the reviewer initially expected `could not
find`. Only the expected diagnostic text was corrected; all 17 controls were rerun and
passed. The production seams were hidden in both attempts.

Logs are whitespace-normalized (trailing spaces/blank lines removed) for Git hygiene;
compiler/test messages and semantic results are otherwise retained. No paid external
compute or other agent authored the review. No Phase-3, production or research branch
was modified. The publication receipt records the final review hash separately to avoid
a self-referential commit hash.
