# Executable architectural example — what it is, how to run it, what it showed

**RESEARCH / PROPOSED DESIGN — NOT ADOPTED.** Not an engine, not an accepted implementation,
not a performance proof. Timing printed by the example is toy-model timing and is not Spark
throughput.

## Location and dependencies

`engineering/research/SPARK_FABLE_ARCHITECTURE_CHALLENGE_2026-09-09/example/` in the research
worktree. Standalone Cargo package (its own `[workspace]` table keeps it out of the production
workspace). Depends by path on the research worktree's `crates/spark-core`, which is the
Phase-1 kernel plus **one read-only accessor** `Scheduler::next_due_time()` (10 lines,
research-only). It reuses the real `Scheduler`, `WorkKey`, `DueWorkItem`, `LogicalClock`,
`LogicalTime`, `CanonicalEncoder`, `Digest`, `ProfileId`, `DefinitionId`, `ScopeId`.

## Run

```
cd engineering/research/SPARK_FABLE_ARCHITECTURE_CHALLENGE_2026-09-09/example
cargo test        # 2 unit tests: kernel-level extraction claim; budget-invariance smoke
cargo run -q      # 13 checks; exit 1 on any FAIL; prints the canonical trace and digest
```

Verified 2026-09-09 with cargo/rustc 1.98.0 on Linux: `cargo run` → 13 PASS, 0 FAIL, exit 0;
`cargo test` → 2 passed; `cargo build` → 0 warnings. Output preserved in `results.txt`.

## What each check demonstrates (mission §4 mapping)

| Check | Mission requirement | Result |
|---|---|---|
| C1 | identical canonical results under different processing budgets | budget 1 (4 calls), 3 (2 calls), unbudgeted (1 call): equal traces and boundary digests; diagnostics differ |
| C2 | equivalent large vs. smaller catch-ups | ten `Advance(+5)` = one `Advance(50)`, including decay and work created inside the horizon; equal at every cohort |
| C3 | a second request waits through the first request's pause | second request accepted while first paused, third gets `MailboxFull`, head unchanged, second executes after completion |
| C4 | delayed work within and beyond the active horizon | work created at 10 ran at 15 and 35 inside `Advance(50)`; work due 135 stayed resident and ran under `Advance(200)` |
| C5 | one relevant failure path | zero-delay feedback rejects the cohort atomically (+999 discarded), keys consumed, identical under budgets |
| C6 | rejection | past-dated input and redelivered `command_id` rejected without mutation; equal-time input accepted |
| C7 | restart boundary | snapshot at a pause (engine fields only) + durable mailbox head → identical to the uninterrupted run |
| W1 | deliberately incorrect variant | drain-whole-prefix (the V3-F01 shape) is detected: work created inside the horizon is skipped and pre-engine digests differ |
| W2 | deliberately incorrect variant | evaluate-at-horizon is detected: one-call vs. stepwise traces differ |
| C8 | self-challenge A1 | exposes the time-slice transaction unit's blast radius and pausing granularity |
| T1 | tuning measurements must not change canonical results | queue depth, calls waited, per-call wall time observed; canonical prefix unchanged |
| S1 | self-challenge A3 | clock-at-completion-only admits backwards writes after a lost-mailbox pause; per-cohort clock rejects them |
| C9 | self-challenge A1 / decision D1 | per-`WorkKey` transactions keep budget, partition, and digest invariance and reject only the defective item (drought = 10 + 12) |

## Restart and redelivery: what is exercised and what is deferred

Exercised: snapshot at a pause boundary and resume (C7); duplicate `command_id` redelivery
after completion (C6); lost-mailbox pause followed by lower-horizon requests (S1).
**Deferred:** a byte-level snapshot codec (the example clones in memory); redelivery of an
`Input` that was *paused* mid-request (the head is re-presented by a durable mailbox — the
example assumes the host persists the head; if it does not, the input is lost at the host,
never inside the engine); transport; concurrency; the Phase-1 `TimelineIngress` (the
example keeps a `command_id → digest` log in its place); and any performance claim.

## Operating conditions assumed

One profile per engine; one sequencer; integer time; all requests offered to one mailbox in
host order; the host persists the mailbox head if crash-resume of a paused request is
required; rules limited to add/decay/threshold/delayed-emission.
