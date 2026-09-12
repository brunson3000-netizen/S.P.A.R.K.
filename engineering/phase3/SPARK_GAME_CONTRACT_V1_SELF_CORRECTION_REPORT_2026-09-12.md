# Integration Contract V1 — self-found corrections, Revision 2

**Date:** 2026-09-12. **Author:** the local main engineer (Claude Code, Opus 5), who also
wrote the material being corrected. **This is a writer's self-correction record, not a
review.** Self-review never satisfies independence; the independent review of this work is
recorded separately.

**Base:** `d8ab37ed6489843380d27e381c716021a18b7fd9` on
`candidate/spark-game-safe-handoff-20260912`.

## Why these corrections exist

The handoff mission requires recovering **existing accepted G.A.M.E. integration decisions
before designing replacements**. The first revision of the contract was written from the
S.P.A.R.K. side and the shared protocol alone. Carrying out that recovery properly, against
G.A.M.E. `origin/main` at `a73fbac743adc1a00679ebc63b91027386718d71`, turned up three prior
sources that the first revision had not read — and one of them led directly to a real defect
in the prototype. All six corrections below are the author's own findings.

## SF-01 — BLOCKING DEFECT: a paced request's early intents were silently dropped

**What was wrong.** `PrototypeDevice::present` projected its intent batch from the
**completing** `ProcessResult`. But a `ProcessResult` carries only the cohort reports of its
own `process` call, and a paced request that pauses spreads its committed cohorts across
several calls. Every intent committed before the last pause was therefore lost, with no
error and no diagnostic — the worst possible failure shape for an advisory seam.

**Proved, not inferred.** A probe against the pinned crate tree drove a two-due-time request
with a pacing budget of 1: the engine produced cohort reports on **both** calls
(`per_call = [1, 1]`, total 2), while the device saw only the final call's 1.

**Fix.** The device accumulates cohort reports across every call of the active request and
projects once, at the completed boundary — which is also the only point at which §4.3 allows
the host to apply anything. An interleaved `Busy` refusal (a *different* request) must not
disturb the active request's accumulation; a sticky fail-stop discards it, since no stable
boundary is published; a completed-but-unfinalized command still publishes its committed
cohorts, with the typed refusal carried alongside.

**Regression evidence.** `tests/end_to_end.rs`:
- **E-12** — a paced request pauses, and both slices' intents arrive in the final batch, in
  canonical time order (5 then 6). This test fails on the pre-correction code.
- **E-6e** — after an interleaved `Busy` refusal, the resumed request still delivers both
  slices' intents.

**Contract change.** §6.2 now states the accumulation rule and why it exists.

## SF-02 — accepted G.A.M.E. decisions were not recovered or cited

`project_records/foundation/GAME_PRODUCT_DESIGN_FOUNDATION_V1.md` carries the status
**CANONICAL — OPERATOR-APPROVED PRODUCT/DESIGN DIRECTION**. Its §11.3 holds a **SETTLED
DIRECTION** on the authority split and a **REQUIRED CAPABILITY** list for the integration
seam, and §11.2 states G.A.M.E. integrates a specific S.P.A.R.K. artifact only under a later
bounded implementation mission. `BehaviorIntent` is G.A.M.E.'s own vocabulary there.

The first revision neither cited it nor demonstrated coverage of it. New contract §1.2.1
records it as the recovered accepted decision and maps every required capability to the
contract section that provides it. No capability was found uncovered — but that is now
*shown*, where before it was only true by luck.

## SF-03 — the provenance claim in §4.4 was true but incomplete

The first revision said a search of G.A.M.E. records "finds no G.A.M.E. record adopting an
outstanding-request/Busy/pull-results integration design". That is literally true and
materially misleading: `GAME_IMPLEMENTATION_READINESS_CLOSEOUT_20260909.md` — which G.A.M.E.
classifies as working research, non-operative — **names** that flow, together with
late-input, transaction-unit, retention and durable-snapshot choices, and states they "remain
proposals", adding a distinct **G.A.M.E.-side** proposal of `BehaviorIntent` parent/child
identity, staged child admission, durable outcome replay and frontier-based release.

Corrected in §1.2.2 and §4.4: the design is **named but expressly unadopted**, and the four
divergences from the G.A.M.E.-side proposal are tabulated with reasons rather than left for
a reviewer to discover.

## SF-04 — acknowledgment identity lacked an instance component

The prior S.P.A.R.K. research established that a persisted sequence number alone cannot prove
exactly-once application, and proposed `(spark_instance_id, report_seq, report_digest)`. The
first revision's `(CorrelationId, batch_digest)` adopted the conclusion but dropped the
instance component.

Corrected to `(activation_hash, CorrelationId, batch_digest)` in §7.2 and in the code: every
component is content-addressed, so no instance counter or issued sequence is needed. Two
genuinely interchangeable engines produce the identical key — which makes deduplication
correct, not a collision — and two divergent engines cannot be confused. **E-13** proves the
activation component actually separates keys and that a ledger does not deduplicate across it.

§7.2 also now states plainly what the key does **not** buy: at-most-once, not at-least-once.

## SF-05 — `max_intents_per_batch` was declared but unenforceable

Once SF-01 is fixed the batch is an accumulation, so its size is not statically bounded by a
single call. Refusing an oversized batch is impossible without discarding committed work, and
truncating one silently loses advisory output.

Corrected in §9.1: inbound bounds are refusals; the **outbound** batch bound is a **capacity
hint**. An oversized batch is delivered whole with `capacity_exceeded` set. Chunked pull of
one boundary's results — the prior research's approach — is recorded as a possible V2 surface
and deliberately excluded from the smallest V1.

## SF-06 — device-side retention divergence was unrecorded

The prior research design has the device retain an unacknowledged report and redeliver it,
which would close the §9.3 gap where a host that loses a batch cannot re-derive it. V1 has no
retention, because the pinned engine has no durable state to retain it in — a retained report
held only in memory is lost by exactly the failure it exists to survive.

Recorded in §9.3 as a divergence and a **candidate resolution of the §9.4 durability gap**,
to be designed with the persistent boundary rather than bolted on before it.

## What did not change

No file under `crates/` is touched. The disposition, the diagnostics and the campaign
evidence are unchanged; none of these corrections bears on them. No gate is accepted, no
reserved choice is frozen, and no G.A.M.E. file is modified.

## Validation after the corrections

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --offline --all-targets -- -D warnings` | PASS |
| `cargo test --offline` | **14 passed, 0 failed, 0 ignored** (was 12; E-12 and E-13 are new) |
| five Windows/Android `cargo check` targets | PASS — **COMPILE-ONLY** |

Logs in `spark_game_contract_prototype_2026-09-12/evidence/`.

## Standing limitation of this record

These corrections were found and made by the author of the material. They have not been
independently verified at the time of writing. Independent verification of repairs is
required before the corrected contract is relied upon.
