# Pattern Card — Fast Tests + Bounded Falsification

PATTERN: Separate Fast Validation From Test-Strength Challenge
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH for mechanics; repository-specific value requires experiment

## Source evidence
- cargo-nextest @ `8527c325bf9f0e5dcdcb26fa80e3ed7dc7f3e35e`
- cargo-mutants @ `fe82f1832778a591ab74248010fb40e699defafe`
- process-per-test runner, structured retry/timeout/outcome events
- scratch-copy mutation lab, explicit baseline, mutant outcomes/diffs/logs
- native `test_tool=nextest` integration.

## Problem

Ordinary tests need to run frequently and cheaply. Mutation testing asks a different, expensive question: would the suite notice plausible wrong behavior? Mixing these into one undifferentiated gate either slows development badly or makes falsification too weak.

## Mechanism

Maintain two deterministic lanes.

### Fast validation
- enumerate project-required tests
- run compatible Rust tests process-per-test
- bounded parallelism/timeouts
- record all attempt states
- fail acceptance on flaky results unless policy explicitly allows them
- preserve project-specific extra test categories (including doctests separately).

### Bounded falsification
- require a valid baseline bound to source/toolchain/environment
- generate mutations in a copied/disposable tree
- select only a bounded package/file/diff/sample/shard set
- run fail-fast tests
- classify caught/missed/unviable/timeout distinctly
- preserve exact mutation diff and run logs
- surface missed mutants as review evidence rather than automatically editing code/tests.

## Benefits

- high-frequency feedback stays cheap
- independent evidence about assertion strength
- deterministic fault injection without model-generated bugs
- parallel workers can receive disjoint mutation shards
- structured evidence supports later review and trend analysis.

## Risks / failure modes

- retry policy hides flaky failures
- mutation suite runs with production credentials/side effects
- baseline is stale/invalid
- nextest compatibility silently omits doctests/custom harness behavior
- too many mutants consume excessive compute
- equivalent/uninteresting mutants create review noise
- unviable mutants are miscounted as evidence of strong tests.

## Boundaries crossed

Repository source → scratch mutation source.
Test runner → repository code execution.
Baseline evidence → mutation result validity.
Mutation outcomes → reviewer/agent judgment.

## Determinization relevance

VERY HIGH. Selection, execution, timeouts, sharding and outcome classification should be deterministic; model judgment is useful after a missed mutant is identified.

## Likely architectural location

Validation/Falsification service under the development supervisor/tool fabric.

## Finding class

PATTERN + TOOL REUSE + TEST/INVARIANT

## Primary disposition

EXPERIMENT_NOW

## Required invariants

1. Fast test executor cannot redefine project canonical acceptance coverage silently.
2. First failure/flaky evidence is retained across retries.
3. Mutation run is invalid without a current passing baseline or explicitly linked prior valid baseline.
4. Canonical source is not mutated by default.
5. Mutation workers receive no production credentials unless explicitly required/authorized.
6. Missed, caught, unviable and timeout are distinct states.
7. Falsification compute has explicit mutant/time/parallelism budgets.
8. Every reported mutant binds source commit, exact diff, test command/config and tool versions.
