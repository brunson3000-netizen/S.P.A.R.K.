# Experiment — Rust Fast Validation + Bounded Mutation Falsification

Status: READY_TO_RUN
No installation/adoption authorized by this record.

## Hypotheses

H1: nextest can reduce wall-clock feedback while preserving the repository's required Rust test evidence for compatible test categories.

H2: a small, deterministic cargo-mutants sample can find assertion gaps not exposed by ordinary passing tests at acceptable compute cost.

## Candidate repository

Use a disposable Rust test fixture first; then a SPARK-owned Rust component only after the fixture validates the harness.

No production credentials, external write authority, deployment credentials or canonical source mutation.

## Phase A — executor equivalence

For one pinned source commit run:

A. canonical `cargo test` command(s)
B. equivalent nextest profile
C. repository-required doctest command separately if applicable.

Capture:
- exact commands/profile/config
- toolchain + nextest version
- selected/enumerated test IDs
- pass/fail/ignored counts
- first failure and retry attempts
- wall-clock and CPU proxy
- per-test duration
- timeout/leak/exec-failure states
- structured JUnit/record artifact.

Acceptance:
- no required test silently omitted
- any intentional unsupported category is a separate explicit gate
- repeat runs select the same tests under identical source/config
- flaky tests remain non-clean evidence.

## Phase B — bounded mutation sample

Precondition: Phase A baseline passes and remains valid.

Run cargo-mutants in default copied-source mode with:
- one package or one/two selected files
- list mutants first and preserve `mutants.json`
- explicit mutant-count/sample/shard budget
- explicit wall-clock/test timeout
- no `--in-place`
- fail-fast nextest when compatible
- separate doctest/other project-required tests if the mutation can be covered only there.

Preserve:
- baseline result
- source commit/digest
- toolchain versions
- `.cargo/mutants.toml` / nextest profile
- `mutants.json`
- `outcomes.json`
- exact mutation diffs
- logs for missed/timeout/unviable mutants
- count and runtime by outcome.

## Review

For each `MissedMutant`, classify:
1. likely meaningful assertion gap
2. behavior equivalent / non-observable mutant
3. test category omitted by execution configuration
4. environmental/fixture weakness
5. requires deeper review.

Do not let an agent automatically add a test solely to kill a mutant without confirming the mutant represents a required behavior.

## Metrics

- ordinary test wall-clock: cargo vs nextest
- tests/second
- flaky/timeout/execution-error count
- baseline determinism across repeated runs
- mutants tested per CPU/wall-clock budget
- caught/missed/unviable/timeout ratio
- meaningful missed-mutant yield
- reviewer time per meaningful finding
- incremental context/evidence volume delivered to reviewer.

## Promotion criteria

Promote nextest to a standard compatible-Rust execution option if it preserves required test coverage and meaningfully improves observability/speed.

Promote cargo-mutants to a periodic/on-demand falsification tool if a bounded sample produces meaningful test-strength findings without unacceptable compute/side-effect risk.

Neither promotion replaces repository-specific acceptance policy.
