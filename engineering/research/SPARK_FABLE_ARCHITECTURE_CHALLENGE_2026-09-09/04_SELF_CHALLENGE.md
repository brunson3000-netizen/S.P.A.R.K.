# Bounded self-challenge: three weakest assumptions

**RESEARCH / PROPOSED DESIGN — NOT ADOPTED.** 2026-09-09. One pass, no recursion. Each
assumption gets an input sequence, expected versus actual behavior (executed in
`example/`, see `results.txt`), and either the smallest coherent correction or the decision
that remains.

## A1 — "The transaction unit is the equal-time slice, and a budget never splits it"

**Input.** Three items due at `t = 20` from producers `trigger.a`, `trigger.b`, `trigger.c`
writing `+10`, `+11`, `+12` to `pressure.drought`; `trigger.b` also schedules zero-delay
work (a rule defect). `Advance(20)` with budget 1.

**Expected under the design.** One cohort; the defect rejects the whole cohort; nothing
committed; all three keys consumed; budget 1 admits all three items in one call.

**Actual (check C8).** Exactly that: one record, `rejection = ZeroDelayFeedback`, `cells`
empty. The experiment exposes the two costs of the unit: blast radius (two innocent items
lost to one bad rule) and pausing granularity (an oversized slice — e.g. a daily tick for
every household — cannot be paused inside; v3 §3.2(c) accepts the same cost).

**Resolution.** Not a demonstrated failure of correctness; a product tradeoff. The smallest
coherent alternative is **per-`WorkKey` transactions in key order**: `run_cohort` becomes a
loop over items, each with its own overlay, identity (`WorkKey::identity_digest`), record,
and rejection; the budget counts items. Determinism is unchanged (key order is semantic, not
arrival). What changes: same-time effects on one cell commit sequentially instead of being
reduced together, so a threshold may fire between two same-time items; exact-duplicate
folding across items disappears (arguably correct for the game: two causes, two effects).
Check C9 executes this alternative (`TxnUnit::PerWorkKey`): budget, partition, and
snapshot invariance all still hold, and on the C8 fixture only `trigger.b` is rejected while
`+10` and `+12` commit. Item-granular *pausing* additionally needs a `Scheduler::take_first`
accessor; the example keeps pausing slice-granular.
**Operator decision D1:** time-slice unit (as frozen) or per-item unit. My recommendation:
per-item for the minimum version, because the game's failure isolation and frame budget
matter more than simultaneity reduction, and it removes the cohort-identity concept entirely.

## A2 — "The engine clock alone is the request frontier; past-dated inputs are rejected"

**Input.** The adapter flushes a frame as `Advance(100)` then offers an observation
generated during that frame at `t = 95`.

**Expected.** `Rejected::HorizonBehindClock { 95, 100 }`, nothing mutated.

**Actual (check C6, shape).** Rejected as expected. This is correct engine behavior and a
host foot-gun: a producer that flushes observations after the advance loses them.

**Resolution.** Two-part, no engine change needed for the first: (1) the adapter contract
states "offer every input with `effective_time ≤ T` before `Advance(T)`" — the mailbox
preserves that order; (2) for genuinely late observations the adapter re-dates:
`at = max(observed_at, clock)` with `observed_at` carried in the payload for rules that
care. Whether re-dating is allowed at all is **Operator decision D2** (strict reject vs.
adapter re-date). Note also that two inputs with equal `effective_time` execute in mailbox
order; that order is the sequencer's ordinal order and is therefore canonical, consistent
with ADR-0003.

## A3 — "A paused request retains nothing; partial progress is indistinguishable from 'not yet requested'"

**Input.** Fixture of design §11. `Advance(50)` with budget 1, three calls (cohorts 10, 15,
30 committed), then the mailbox is lost (non-durable host restart). The host offers
`Advance(12)` and `Input@12 { hunger += 1 }`.

**Expected under the addendum's rule (`F` unchanged by pause).** Both admitted (`12 ≥ 0`).

**Actual (check S1, `ClockPolicy::CompletionOnly`).** Both admitted; `exposure.hunger` is
rewritten with `updated_at = 12` after it was committed at 15. Time runs backwards on that
cell; the resulting history matches no uninterrupted run. **The assumption is false as
stated in the addendum.**

**Correction (smallest coherent).** The clock is *simulated-through time*: it advances to each
cohort's `due_time` as the cohort commits, and to `h` at completion. With
`ClockPolicy::PerCohort` the same input sequence yields `Rejected::HorizonBehindClock` for
both late requests and `clock = 30`. C1, C2, C7 still pass, so partition, budget, and
snapshot equivalence are unaffected. The design (§5) now carries this rule. No new state
is introduced; the existing `LogicalClock` simply advances more often.

## What remains undecided after this pass

| Id | Decision | Options | My recommendation |
|---|---|---|---|
| D1 | transaction unit | time slice / per-`WorkKey` | per-`WorkKey` |
| D2 | late-dated inputs | strict reject / adapter re-date | strict reject in engine; re-date allowed in adapter with `observed_at` payload |
| D3 | snapshot codec home and format | `spark-core` encode surfaces + `spark-host` codec | as stated; canonical encoder bytes, versioned header with activation hash |
