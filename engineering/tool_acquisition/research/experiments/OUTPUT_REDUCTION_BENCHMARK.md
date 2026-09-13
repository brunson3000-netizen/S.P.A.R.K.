# Experiment — Output Reduction: Raw vs RTK vs Searchable Spillover

Status: READY_TO_RUN

## Hypothesis

A command-specific deterministic reducer can materially cut worker context without reducing debugging success, and a canonical raw artifact + derived search layer can recover the evidence the compact view omits.

## Fixed inputs

Use the same pinned repository and deterministic command fixtures for all variants.

Candidate commands:
- `git status`
- `git diff` on controlled fixture
- `cargo test` with pass + failure cases
- `cargo clippy` with multiple diagnostics
- `rg`/grep with many matches
- build output with repeated warnings
- structured JSON CLI output
- one output larger than normal compact limits.

## Variants

A. raw output directly to worker

B. RTK compact output only

C. RTK compact output + RTK recall as implemented

D. proposed SPARK shape:
- execute once
- immutable raw artifact with full digest
- RTK-like reducer
- searchable derived index
- worker receives compact result + artifact/retrieval/search references.

## Measurements

- raw bytes
- compact bytes
- estimated/actual tokenizer input tokens where available
- task/debugging success
- follow-up retrieval count
- missed load-bearing evidence
- time to useful diagnosis
- reducer latency
- reducer failure/fallback behavior
- raw-evidence recoverability
- exact byte fidelity of recovered evidence
- evidence retention/identity state.

## Adversarial cases

- parser sees a new/unexpected output format
- command returns non-UTF-8 bytes
- nonzero exit with huge output
- success output hides a line later needed for diagnosis
- raw output exceeds RTK recall cap
- two stored outputs with similar/short hash prefixes
- hook/rewrite disabled or fails
- compound command/pipeline that RTK deliberately leaves partly raw.

## Acceptance direction

A promoted reducer must:
1. preserve underlying operation status
2. never be canonical evidence owner
3. disclose every lossy/truncated view
4. provide a resolvable canonical artifact ID
5. bound fallback prompt output if parsing fails
6. preserve or improve task success against raw baseline
7. show meaningful context/token reduction.

RTK hook installation/rewrite is explicitly outside this experiment; invoke the reducer/wrapper explicitly inside a disposable test harness.
