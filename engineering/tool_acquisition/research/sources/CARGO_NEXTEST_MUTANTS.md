# Source Study — cargo-nextest + cargo-mutants

Status: FIRST TRACE COMPLETE / BOUNDED EXPERIMENT REQUIRED
Study date: 2026-09-13

## Frozen sources

### cargo-nextest
- Upstream: `https://github.com/nextest-rs/nextest`
- Commit: `8527c325bf9f0e5dcdcb26fa80e3ed7dc7f3e35e`
- License: MIT OR Apache-2.0
- Rust edition: 2024; minimum Rust: 1.91
- Role: Rust test enumeration/execution/reporting/recording/partitioning.

### cargo-mutants
- Upstream: `https://github.com/sourcefrog/cargo-mutants`
- Commit: `fe82f1832778a591ab74248010fb40e699defafe`
- Version: 27.1.0
- License: MIT
- Rust edition: 2024; minimum Rust: 1.88
- Role: mutation/falsification testing by generating likely-valid wrong source variants and asking existing tests to catch them.

## Problem framing

PROBLEM: A passing Rust test suite says little about execution efficiency and does not prove tests would catch plausible implementation defects. SPARK needs deterministic validation machinery that can run ordinary tests quickly and periodically falsify the strength of those tests.

USER: builders, reviewers and automated qualification lanes.

INPUT: pinned Rust repository/worktree, canonical repository test policy, selected filters/profile, optional mutation scope/budget.

OUTPUT: per-test execution evidence; retries/timeouts/flakiness; machine reports; and for mutation runs, caught/missed/unviable/timeout mutant evidence with exact diffs/logs.

AUTHORITY: both tools execute project tests/builds and therefore execute repository code/build scripts/test side effects. cargo-mutants additionally generates and builds modified source copies. These are code-execution capabilities and belong inside a disposable/qualified execution boundary, never as read-only inspection tools.

TRUST: source/worktree, Cargo/toolchain/configuration, tests and their dependencies, nextest/cargo-mutants configuration, environment/credentials available to tests. A mutation-testing run assumes the unmutated baseline is valid.

STATE: nextest supports run recording/replay/rerun/JUnit and runtime scheduler state; cargo-mutants writes an incremental `mutants.out` evidence directory and scratch build copies. Neither output is canonical project acceptance until SPARK binds it to source/tool/config/environment identity.

FAILURE: nextest distinguishes test failure, execution/start failure, timeout, leaks, cancellation and flaky attempts. cargo-mutants distinguishes baseline failure, caught/missed mutant, unviable build/check, timeout and uncommon unattributed failure.

## Trace A — nextest list → isolated run → structured result

`cargo nextest run`
→ build test binaries with `cargo test --no-run`
→ query each test binary to enumerate tests
→ apply filters/partitions/configuration
→ execute each individual test in a **separate process**, in parallel
→ collect per-attempt output/status/timing
→ optional retries
→ aggregate per-test and run-level evidence
→ optional JUnit/record/replay/rerun artifacts.

This differs from `cargo test`, where each binary runs its own tests internally. Process-per-test gives nextest direct per-test visibility and strong process isolation between tests.

The event model explicitly distinguishes:
- test start/finish
- slow/timeout state
- failed attempt that will retry
- retry start
- setup-script events
- cancellation/kill/pause/resume
- execution failure separate from ordinary test assertion failure
- process leak states.

Machine evidence can include JUnit and recorded runs. Partitioning is deterministic relative to the selected test set and can happen after normal filters; build archives can be reused across workers.

## Trace B — nextest retry/timeout semantics

Retries are first-class and record every attempt. By default, if an initially failing test later passes it is labeled flaky and the **overall run succeeds**. `flaky-result = fail` changes that acceptance behavior.

SPARK implication:
- retries may be useful diagnostic evidence
- a first failure must remain visible
- assurance/acceptance profiles should fail or explicitly mark the run non-clean on any flaky result unless a project policy says otherwise.

Timeouts:
- default slow marker is 60s
- optional `terminate-after`
- Unix: process group receives SIGTERM then SIGKILL after grace period
- Windows: job-object termination kills the process tree immediately for timeout
- global timeout is available.

Nextest also permits timeout-as-pass for fuzz-like workloads. That is a workload-specific policy and must never silently apply to ordinary acceptance tests.

## Trace C — nextest limitations / canonical-test relationship

Important limitation at this pin: **doctests are not supported**. Repositories that rely on doctests need a separate `cargo test --doc` lane.

Custom harnesses can require adaptation because nextest has a thicker per-test interface than Cargo’s simple test-binary exit-code interface.

Therefore nextest is a high-value executor, not a universal replacement for a project’s canonical test policy. SPARK should preserve per-repository commands/gates and use nextest where compatibility is proven.

## Trace D — cargo-mutants discover → copy → baseline

cargo-mutants:
→ locate workspace/packages/targets with Cargo metadata
→ parse production source via `syn`
→ walk module/AST structure and generate likely-valid wrong replacements
→ exclude test branches / explicit skip regions
→ copy the whole source tree to a scratch directory by default
→ repair relative dependency paths as needed
→ run baseline build/tests in the copy
→ only if baseline passes, create/reuse one scratch build directory per parallel mutation worker.

This source-copy design is important: default mutation testing does not edit the canonical source tree. `--in-place` exists but crosses a substantially riskier boundary and should not be used by SPARK’s default lane.

## Trace E — mutant → build/test → outcome

For each selected mutant:
→ apply mutation in that worker’s copied source tree
→ build/check as configured
→ if build/check fails: `Unviable`
→ if build succeeds: run configured tests (`cargo test` or `cargo nextest run`)
→ if tests fail: `CaughtMutant`
→ if tests pass: `MissedMutant`
→ if build/test exceeds bound: `Timeout`
→ write diff/log/outcome evidence.

`LabOutcome` has explicit counters for caught/missed/timeout/unviable and gives baseline failure, timeout and missed-mutant conditions distinct exit codes.

Evidence directory includes:
- `mutants.json` generated before testing begins
- `outcomes.json`
- one diff per mutation
- per-scenario log
- caught/missed/timeout/unviable lists
- run lock/metadata.

This is highly suitable for machine qualification if SPARK copies the run artifacts into canonical evidence custody rather than relying on the mutable default `mutants.out` lifecycle.

## Trace F — baseline and timeout integrity

Baseline normally runs before mutations and is used to derive mutation timeout.

Default test timeout is 5× baseline test duration with a 20s floor. Explicit timeouts/multipliers are available; build timeout is optional.

Skipping baseline is supported, but upstream documentation correctly states the results are meaningless unless a separate known-good baseline has been established. Without a baseline, default test timeout falls back to 300s unless set explicitly.

SPARK implication: baseline identity must be bound to the same source/toolchain/environment/test-policy used for the mutation batch; “tests passed earlier somewhere” is insufficient if that evidence has been invalidated.

## Trace G — partition/shard semantics

### nextest
- partitions the selected test set across workers
- slice partitioning distributes individual tests evenly after filtering
- archives can separate build from distributed execution.

### cargo-mutants
- shards independently discover the same mutants and select a deterministic subset
- no runtime coordination between shards
- all shards must use identical arguments/source/diff/shard denominator
- baseline can be skipped per shard only if a separate valid baseline gate exists.

This maps naturally onto SPARK’s bounded worker model, but the coordinator must own source/config identity so independently running workers cannot silently disagree on the universe being partitioned.

## Trace H — pair integration

cargo-mutants natively supports `test_tool = "nextest"` / `--test-tool=nextest`.

For mutation testing, nextest fail-fast is recommended because only one failing test is required to prove a mutant was caught. Process-per-test can stop faster, although currently running straggler tests may still finish and some repositories can be slower under nextest.

Caution: nextest does not run doctests, so a mutant detectable only by doctests can appear missed under the nextest test tool. A complete falsification lane must run all project-required test categories.

## Boundary findings

### Test executor ↔ canonical project policy
The executor must not silently redefine “the tests.” Repository-specific commands, doctests, integration dependencies and setup scripts remain part of project acceptance.

### Retry ↔ acceptance
Retry is diagnostic/reliability machinery. It must not erase a first failure in an assurance gate.

### Mutation ↔ source truth
Scratch mutation is derived experimental source. Canonical source stays immutable. In-place mutation should be disabled unless a disposable checkout and separate authority explicitly permit it.

### Mutation ↔ external side effects
Copying source does **not** sandbox test behavior. A mutant can alter which existing destructive/network/API path a test exercises. Run mutation testing without production credentials and inside an isolated environment appropriate to the repository.

### Mutation outcome ↔ test quality
A missed mutant is evidence of a possible assertion gap or equivalent behavior, not automatic proof of a product bug. An unviable mutant is not test-strength evidence.

## Determinization candidates

- test enumeration/filtering/partitioning
- per-test isolated execution
- timeout/process-tree kill
- flaky-attempt recording
- machine result aggregation
- mutation enumeration/scope filters
- deterministic scratch-copy testing
- baseline binding
- mutation shard allocation
- outcome classification
- evidence packaging.

## Material extracted

IDEA: ordinary validation and test-strength falsification are two distinct lanes.

PATTERN: fast isolated test execution → bounded mutation challenge → structured evidence.

ALGORITHM: process-per-test scheduling/partitioning; AST mutation enumeration; baseline-derived timeout; deterministic shard selection.

CODE: both are strong candidates as external executables rather than libraries in SPARK’s Rust core. Invoke pinned binaries under a qualified RunSpec and ingest their structured evidence.

TEST/INVARIANT: mutation results are valid only against a passing, still-valid baseline; flaky/timeout/execution-error states remain distinct; scratch source is non-canonical.

## Primary disposition

EXPERIMENT_NOW

Likely operational disposition after qualification:
- cargo-nextest: REUSE_TOOL for compatible Rust repositories
- cargo-mutants: REUSE_TOOL as bounded periodic/targeted falsification, never every ordinary edit by default.

## Recommended default shape

Fast lane:
1. repository canonical quick tests
2. nextest profile where compatible
3. retries off or `flaky-result=fail` for acceptance evidence
4. hard per-test + global bounds
5. capture structured run evidence.

Falsification lane:
1. prove current baseline
2. select bounded package/file/diff mutant set
3. run in isolated disposable environment, default scratch-copy mode only
4. test with fail-fast nextest where compatible, plus any project-required doctest/other lane
5. preserve `mutants.json`, `outcomes.json`, diffs and logs
6. classify missed mutants for human/agent review rather than auto-editing code/tests blindly.
