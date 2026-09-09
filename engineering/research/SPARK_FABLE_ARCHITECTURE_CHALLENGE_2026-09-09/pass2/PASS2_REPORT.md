# Pass 2 — adversarial executable pass on the Fable design

**RESEARCH / PROPOSED DESIGN — NOT ADOPTED.** 2026-09-09, 05:45Z–05:56Z. Worktree
`/home/chromikey/Projects/SPARK-fable-challenge-worktree`, branch
`research/fable-architecture-challenge-2026-09-09`. Baseline at entry: `421c5f92031ac057acb6272176c1b81c12450d4a`,
clean. Production `/home/chromikey/Projects/SPARK` at `47729bb719d8ae95ae00c4e195fa64d0e27bfdb7`, unchanged.
Pass-1 evidence (`../example/`, `../0*.md`) preserved unchanged; this pass lives in `pass2/`.
Pass-2 content commit: `3e0d65ea90d2d096240eb241d91cd41591c9bd2e` (parent `421c5f9…`); the
manifest commit that follows is named in the completion message and `POST_COMMIT.txt`.

## Commands (reproduce)

```
cd engineering/research/SPARK_FABLE_ARCHITECTURE_CHALLENGE_2026-09-09/pass2/example
cargo run -q                 # 7 checks, debug   -> results_after_fixes.txt   (0 failed)
cargo run -q --release       # 7 checks, release -> results_after_fixes_release.txt (0 failed)
cargo test -q                # 2 unit tests (inherited)
git show 3e0d65e -- pass2/example/results_before_fixes.txt   # captured failing P3
```
Dependencies: the worktree's `crates/spark-core` with two research-only accessors
(`next_due_time`, `take_next`; see `../scheduler_accessor.patch` and `git diff 47729bb -- crates/spark-core`).
The archive includes the patched `scheduler.rs` so a reviewer can rebuild from `47729bb`.

## Regressions, initial failures, fixes, remaining failures

| Task | Regression | Initial result | Fix | Now |
|---|---|---|---|---|
| 1 brakes | P1: 3 same-time items, budget 1 → exactly 1 transaction, 2 resident; pause/clone/resume == uninterrupted (records, cells, boundary digest, both reports) | pass-1 per-item path could not do this (drained the slice) | `Scheduler::take_next()` (research-only): removes exactly the first slot, scheduled **or** conflicted, in `WorkKey` order; the loop checks budget **before** taking, so nothing is ever extracted and buffered | PASS |
| 1 divergence | P1b: same-time A(−20) then B(+20) on scarcity 25, threshold 20 | — | none (observation) | slice: net 0, no crossing, hunger 0; per-item: B crosses, hunger 20. One defective item: slice commits nothing, per-item commits the other two |
| 2 report | P2: request with 2 commits + 1 rejection through 2 pauses and an in-memory restore, then a second request | pass-1 `Outcome` had no report | `Outcome::Completed { report: CompletionReport { records, clock, boundary_digest } }` fed by an **engine-owned `undelivered` buffer** (delivery state, snapshot-carried), drained at completion | PASS: report 1 = exactly 3 records at t=20 in order, report 2 = 1 record at t=30, `undelivered` empty, concatenation == trace |
| 3 hashing | P3: same parent key at 10, same child key at 15, child payloads differ, advance to 10 | **FAIL** (captured in `results_before_fixes.txt`): batch digests identical for semantically different outcomes; engine/scheduler digests differed | batch digest now includes each scheduled item's payload hash; `CohortRecord::scheduled: Vec<(WorkKey, Digest)>`; conflicts and the rejection variant also hashed | PASS |
| 3 bodies | P3b: body store emptied; body content corrupted under a valid digest | pass-1 silently skipped a missing body (`if let Some`) — silent disappearance counted as success | `MissingBody` and `BodyDigestMismatch` typed rejections; verify `body.digest() == advertised` on read | PASS, nothing written |
| 4 collision | P4: transaction with a cell write + schedule into an occupied key with a different payload; then a slice containing only the poisoned key | undefined in pass 1 (key reported as "scheduled") | **committed poison**: writes stand, key becomes `Conflicted`, record lists it under `conflicts`; later the poisoned slot drains as a conflict-only record, never executed. Rollback rejected because it would reinstate arrival-order authority (ADR-0003 §11) | PASS; report matches `slot_status` |
| 4 arithmetic | P4b: delay `u64::MAX−5` at now 10; occurrence counter at `u64::MAX` | **panic in debug at commit** (`occurrences += n` overflow; would wrap in release) — a real debug/release divergence found by the check | `checked_add` for due time and for the post-commit counter, validated in `apply` so commit cannot fail; `TimeOverflow` / `OccurrenceOverflow` typed rejections, no partial writes | PASS in debug and release |
| 5 drill | not run | — | — | **deferred** (timebox); see below |

Remaining failures: none in the 7 checks. Known gaps: the restore in P1/P2 is an in-memory
clone, not a codec; no crash-recovery or exactly-once claim is made (a host crash between
`Completed` and consuming the report loses that report unless the host persists it; the
records remain in the engine's `trace`, which is append-only and unbounded here).

## What must persist where (from P2)

| Engine (snapshot) | Host |
|---|---|
| cells, scheduler slots, bodies, occurrence counters, finalized command log, clock | mailbox contents including the head being processed |
| `undelivered` records of the in-progress request (delivery state; distinct from causal state) | delivered reports, if at-least-once delivery to the game is required |

## Requirements comparison

Preserved: deterministic canonical results independent of budget and partition; `WorkKey`
semantic identity and Phase-1 poison rule; one active request until completion; later
requests wait; backpressure; completion = the request's horizon, not all timers; G.A.M.E.
owns world truth, Spark returns advisory records; measurements never enter digests.
Intentional changes (proposals): transaction unit per `WorkKey` with item-granular pausing;
clock advances per committed transaction; batch digest commits to scheduled payloads;
commit-time key collision = committed poison; missing/corrupt bodies are typed rejections.
Deferred: byte-level snapshot codec; bounded/acknowledged `trace`; `TimelineIngress` in
place of the `command_id` log; intents derivation; workload drill; multi-engine deployment.

## "One profile per engine" clarified

A profile is a vocabulary plus ruleset (e.g. `game-world`). One engine instance holds
**every scope** of that profile: all characters (`actor.*`), households, settlements,
regions, factions. Multiple characters are simply multiple actor scopes inside one engine;
shared scopes (a settlement's food scarcity) are cells at settlement scope that rules for
any actor read and that threshold rules can map down to households and actors, as the
example's chain does. "One profile per engine" only removes the case of two *vocabularies*
(e.g. `game-world` and `mci-agent-commons`) sharing one scheduler; those become two engines
with no cross-engine propagation, which the game does not require.

## Workload drill (task 5): deferred, with the caveat stated now

Transaction-count budgets bound the number of transactions per consumer step, not wall
time: one transaction may contain a deep wave chain or a wide cohort. A wall-clock bound
needs either a size-aware budget (count effects, not transactions) or the host limiting
steps per frame and accepting deferred work. Any timings would be synthetic and machine-
specific and were not measured in this pass.

## Revised recommendation and smallest next executable integration step

Recommendation unchanged: **focused refactor** on Phase 1, with D1 decided as per-`WorkKey`
transactions (now demonstrated with real brakes). Smallest next step: add `next_due_time`
and `take_next` to `spark-core` for real (with tests), then implement `spark_engine::request`
as in `pass2/example/src/lib.rs` `process()` plus a byte-level snapshot codec, and wire a
G.A.M.E. adapter stub that offers `Input`s before `Advance` and consumes `CompletionReport`s.
