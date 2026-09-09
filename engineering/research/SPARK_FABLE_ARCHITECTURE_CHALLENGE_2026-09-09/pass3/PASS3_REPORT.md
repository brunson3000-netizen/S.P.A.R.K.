# Pass 3 — smallest complete flow-control and result-delivery design

**RESEARCH / PROPOSED DESIGN — NOT ADOPTED.** 2026-09-09 05:55Z–06:00Z. Per-WorkKey
transactions are used as a research setting, not an adopted decision. Code:
`pass2/example/src/bin/pass3.rs` (copy here). Run: `cargo run -q --bin pass3` in
`pass2/example` → 7 PASS / 0 FAIL (`pass3_results.txt`). Crash boundaries are an
**in-memory fault-injection model** (clone-and-drop of the device); they do not prove
process durability.

## Comparison and choice

| Option | Extra machinery it earns | Verdict |
|---|---|---|
| One request at a time, GAME pulls results | one active request, one sequence-numbered retained report, `ack(seq)` | **chosen**: sufficient for trickle and bursts because GAME already owns its events; backpressure is the `Busy` reply |
| Bounded request/result mailbox | queue storage, head-until-complete rule, mailbox persistence for crash-resume | earns nothing beyond option 1: a queue only moves GAME-owned events into Spark memory |
| Credit/ack mechanism | credit accounting on both sides | the ack alone (one retained report) is the whole credit; N>1 credits only add buffering |

**Chosen design (`Device`).** `offer(req)` → `Accepted | Busy | UnackedReport | Refused`.
`step(budget)` runs bounded work; output is bounded by `chunk` records: a request producing
more pauses at the chunk boundary (a delivery boundary, never causal) and continues after
`ack`. `poll()` returns the one retained report (same `seq` on redelivery); `ack(seq)`
releases it; a new request is refused until the report is acknowledged. GAME applies a
report exactly once by `seq`. States: **accepted** (active), **applied** (records
committed, in `undelivered`), **available** (retained chunk), **delivered** (polled),
**acknowledged** (`acked_through`). Owner of unfinished work: Spark (active request + resident
scheduler). Owner of undelivered results: Spark (retained chunk, snapshot-carried). Owner of
unoffered events: GAME.

## Boundaries demonstrated (B1–B6, EQ, MEM)

B1 faster producer → `Busy`, nothing queued or lost. B2/B3 GAME stops consuming while 12
records exceed chunk 5 → Spark pauses with state unchanged; chunks 1..3. B4 restart after
commit before delivery (modeled) → retained chunk survives, chunks equal the unchunked
reference, digest equal, 12 effects applied once. B5 lost ack → same `seq` redelivered,
GAME dedups, ack clears. B6 old `Advance(5)` and duplicate `Input` refused explicitly.
**Finding:** `Advance(T)` with `T == clock` is accepted as a legal empty advance (equal time
allowed); it completes with zero records and re-executes nothing. EQ: records and digests
identical across chunk 1/3/7/∞ and budgets 1/2/∞/4.

## Guarantees and limits

Delivery: **at-least-once delivery of each report, exactly-once application by `seq`**,
provided GAME persists `acked_through` (or tolerates re-applying an idempotent report).
Spark never re-executes committed work. Memory: at most one active request, one retained
chunk (≤ `chunk` records), the live world state (cells, resident scheduler slots, referenced
bodies, occurrence counters, clock), and finalized ids **only at the current clock time**
(older duplicates are refused by the time rule anyway, so older ids may be released; a very
late retry after release is refused as `HorizonBehindClock`, explicitly, never silently).
**Gaps found:** `engine.trace` is unbounded (test oracle; drop from the engine); consumed
bodies are never released (12 remain after the run; needs a reference sweep at commit or
snapshot). Enormous transaction: smallest practical protection is a declared, epoch-bound
`max_effects_per_transaction` cap → deterministic typed rejection, never partial, never
deadline-interrupted; a host deadline can only choose not to start the next transaction.

## Remaining decisions (gameplay / policy)

D1 transaction unit (still open); D2 late inputs; chunk size and host budget as deployment
values; whether GAME persists `acked_through` (exactly-once) or accepts idempotent re-apply.

## Smallest next implementation step

Move `Device` (offer/step/poll/ack, chunking, retained report) into `spark-host` over the
real `spark_engine::request`, add body release and trace removal, then a byte-level
snapshot of engine + retained chunk, and test B4 with a real process restart.
