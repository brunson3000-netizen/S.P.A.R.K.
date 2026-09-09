# Development route: retain-and-repair vs. focused refactor vs. rewrite, and the plan

**RESEARCH / PROPOSED DESIGN — NOT ADOPTED.** 2026-09-09.

## 1. Comparison

| | Retain and repair | **Focused refactor (recommended)** | Rewrite |
|---|---|---|---|
| Working code/tests preserved | all of Phase 1 (232 tests) | all of Phase 1 (232 tests) + one accessor | little; ids/hash/scheduler would be re-derived |
| Behavior to rebuild or reverify | none now; the whole Phase-2 evaluator later, after the V3-F01 candidate lineage is accepted and frozen | evaluator, request loop, rules artifact, snapshot codec, host crate — all new code either way | everything, including the Phase-1 invariants that took three review rounds to get right |
| Principal risk | process risk: the correction lineage is on its fifth pass and still returns `FOUNDATIONAL_REVIEW_REQUIRED`; each pass adds mechanism; acceptance oracles for nonexistent code keep failing review | scope discipline: the refactor must not quietly reopen Phase-1 invariants; needs a decision on D1/D2 before coding | re-litigating B-02/ADR-0003 class defects; losing the activation-boundary lesson; no concrete advantage identified |
| Relative effort / uncertainty | low effort per pass, unbounded number of passes; high uncertainty | medium effort, low uncertainty (the example already exercises the hard part on the real kernel) | high effort, high uncertainty |
| Time to first useful sandbox demonstration | unknown; blocked behind freeze + Phase-2 implementation authorization | shortest: loop + minimal rules + fixture in the first pass; adapter slice in the second | longest |

**Recommendation: focused refactor.** A rewrite has no concrete advantage: the kernel is
small, tested, and already provides what the loop needs. Retain-and-repair keeps paying for
the prose. The refactor keeps every line of Phase 1 and replaces the Phase-2 *method*
(specify digests, then review the specification) with *build the loop, then falsify it
with executable fixtures*.

## 2. The smallest real vertical slice

**Goal.** G.A.M.E. submits one observation and one advance and receives a meaningful,
digest-verifiable Spark result with an advisory intent.

**Scope.**
- `spark-engine::rules`: a validated artifact with exactly five operations — `AddDelta`,
  `Clamp`, linear `Decay` on read, `Threshold → emit`, `ScheduleWork(delay ≥ 1)`.
- `spark-engine::evaluate`: the overlay transaction and wave loop (design §6).
- `spark-engine::request`: `process(&Request, Budget) -> Outcome` (design §4–§5), clock
  advancing per cohort.
- `spark-core::Scheduler::next_due_time()` and an `iter()`/encode surface for snapshots.
- `spark-host`: `Mailbox`, `consumer_step`, `Snapshot` codec (canonical encoder bytes,
  versioned header with activation hash), telemetry struct.
- Intent derivation: a fixed table from `intent.*` cells to `BehaviorIntent { actor, kind,
  strength }` at completion.
- Fixture: the canonical chain `drought → food/affordability → household exposure → actor
  choice → hunting → wildlife decline → neighboring-settlement feedback` as data.
- Embedded form only (Rust library call); the standalone form is the same crate behind a
  transport later.

**Dependencies.** Operator decisions D1, D2, D3; nothing from G.A.M.E. beyond stable ids and
integer logical time (already reserved by the convergence protocol).

**Acceptance evidence (executable, replaces the prose matrix for this slice).**
1. Budget invariance and catch-up partition equivalence over the fixture (C1/C2 shape).
2. Pause/resume with a waiting request; backpressure (C3).
3. Delayed work inside and beyond the horizon (C4).
4. Typed rejection atomicity (C5); past-dated and duplicate rejection (C6).
5. Snapshot → bytes → restore → identical trace and boundary digest (C7, now byte-level).
6. Two wrong variants detected (W1/W2), kept in the test suite as mutation checks.
7. Lost-mailbox pause followed by lower-horizon request rejected (S1).
8. Linux executable digest parity between two processes; Windows/Android `cargo check` as
   before, executable parity when a runner exists.

**Experiments to measure practical gameplay delay (noncanonical, telemetry only).**
- Mailbox depth and calls-waited per request under a synthetic bubble: N active actors with
  per-minute cadence plus M dormant scopes with daily timers; vary N, M, budget.
- Per-call processing time versus budget size; find the budget that keeps the p99 call
  under a frame budget the game chooses.
- Catch-up cost after simulated offline periods (hours, days of logical time) as a function
  of resident timers, to test the "sparse near the bubble, timers elsewhere" hypothesis.
- Report all as diagnostics; assert canonical traces unchanged across every configuration.

## 3. Ordered implementation plan

| # | Step | Who decides |
|---|---|---|
| 0 | Operator decides D1–D5 | Operator |
| 1 | Record this research as a proposal; open a bounded "Phase-2 loop-first" brief that supersedes the V3-F01 candidate lineage as a *proposal* and keeps the freezes operative until independent acceptance | Operator |
| 2 | `spark-core`: `Scheduler::next_due_time`, `Scheduler::iter`/canonical encode; `StateStore::cells` enumeration; no semantic change; existing tests untouched | engineering |
| 3 | `spark-engine::rules` artifact + validation at the activation door (five ops) | engineering |
| 4 | `spark-engine::evaluate` + `request` (loop, overlay, waves, clock per cohort, records, digests) | engineering; D1 fixes the unit |
| 5 | Executable fixtures 1–7 above, including W1/W2 mutation checks, in `spark-testkit` | engineering |
| 6 | `spark-host`: mailbox, consumer, snapshot codec, telemetry | engineering; D3 fixes the codec |
| 7 | Independent review of steps 2–6 against the executable fixtures (Codex HIGH), one pass | Operator schedules |
| 8 | G.A.M.E. adapter thin slice (Gate C4 shape): ids, time, observations before advance, intents advisory, confirmed outcomes as inputs | G.A.M.E. side; D2 fixes late-input policy |
| 9 | Telemetry experiments above; tune budget and mailbox capacity from evidence | engineering |
| 10 | Deferred capabilities in order of game need: explanation (emission identity/parent sets), random-address gates, curves/aggregation, standalone transport, multi-engine deployment | later |

Ordinary engineering choices the implementing agent resolves without the Operator: internal
type names, overlay representation, encoder layout details, fixture values, test
organization, feature flags for test-support seams, error enum shapes, clippy gate scope for
the new modules (same deny set as Phase 1).
