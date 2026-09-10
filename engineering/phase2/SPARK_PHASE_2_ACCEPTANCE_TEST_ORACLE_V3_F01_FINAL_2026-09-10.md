# S.P.A.R.K. Phase 2 — Acceptance Test Oracle, V3-F01 Final Correction (AT-I)

**Date:** 2026-09-10
**Agent:** Claude Code (Fable 5.1), separated V3-F01 final correction writer (see the
candidate header and writer report §2 for the model-designation note).
**Status:** **CANDIDATE — AWAITING INDEPENDENT REVIEW.** Not accepted, not frozen, not
canonical, and authorizing no implementation.
**Companion to:**
`SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_FINAL_2026-09-10.md`
(unqualified "§" references are to that document; "v3 §…", "v2 §…", "v1 §…" to the three
Phase-2 freezes; "V2 review" to
`SPARK_PHASE_2_CODEX_V3_F01_V2_INDEPENDENT_REVIEW_2026-09-06.md`; "SB review" to
`SPARK_PHASE_2_CODEX_V3_F01_SERIALIZED_BOUNDARY_INDEPENDENT_REVIEW_2026-09-10.md`;
"model S…" to `engineering/phase2/v3_f01_final_correction_model_2026-09-10/`).
**Amends:** `SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v3_2026-08-26.md` (retained
unmodified as history) in exactly the entries named in §1.
**Supersedes (as a proposal), preserved unchanged as history:**
`SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_2026-09-06.md` and
`SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_V2_2026-09-06.md`.

**Discipline (unchanged from v1/v2/v3).** Every AT-I test is encoded red-first; baselines
are recorded verbatim under `engineering/phase2/<pass>-baseline/` before any production
change, and "inexpressible against the inherited tree" is recorded, never skipped. Every
"X is impossible" claim is a digest or value equality, or a compile-fail — never an
error-was-returned check alone. No existing test is weakened, deleted, or skipped; the
Phase-1 suite (232 tests at the last independently reviewed run) runs unmodified
throughout.

**Falsification discipline.** Every oracle carries a **Kills:** line naming the concrete
incorrect implementation it must reject and the observable that necessarily differs.
Where no structural proof exists, the claim is narrowed and says so. Tests are
specified here; **none has been executed**, no production Rust exists to run them
against, and **this pass writes no Rust and ran no Rust test**. The disposable Python
model is architecture evidence, not a test result.

---

## 1. Exact delta

| Entry | Disposition |
|---|---|
| Matrix v3 "`Scheduler` consumed, not modified; no batch API" conformance row | **REPLACED** by §2 |
| **AT-I39** `causal_identity_is_independent_of_drain_partition` | **REVISED** — §3 (V2 (C)/(D) Kills corrected per V2-09) |
| **AT-I40** `cohort_is_scoped_to_profile_and_due_time` | **REVISED** — §4 ((e) corrected per V2-04) |
| **AT-I29** `wave_buffers_do_not_outlive_the_wave` | **EXTENDED** — §5 ((d) `Φ` test **DELETED**; replaced by AT-I47) |
| **AT-I32** strict lint / compile-probe gate | **EXTENDED** — §6 (facade boundary per V2-08) |
| **AT-I35** Phase-1 digest stability | **EXTENDED** — §7 |
| **AT-I42** `least_due_compare_and_take_is_atomic_and_unprivileged` | **REVISED** — §8 ((d)(2) per V2-07; cross-store cases added per V2-05) |
| **AT-I43** `request_lifecycle_is_serialized_and_time_ordered` | **REWRITTEN** — §9 (was `command_barriers_are_placed_and_time_guarded`; SH-1/SH-2/`Φ` rows deleted) |
| **AT-I44** `every_rejection_class_is_terminal_and_cleans_its_stores` | **REVISED** — §10 (wave-index split per V2-06; R-8 rows deleted; mixed-slice case added) |
| **AT-I45** `pacing_counts_executable_work_only` | **REVISED** — §11 ((a) per V2-03; executable-only counts and post-exception residue per V2-10) |
| **AT-I46** `pre_wave_observation_is_test_only` | **REVISED** — §12 (three obligations separated) |
| **AT-I47** `request_boundary_state_is_committed_and_discriminated` | **NEW** — §13 (`F` and `ActiveRequest` AT-G tests; replaces AT-I29(d)) |
| **AT-I20 / AT-I20b / AT-I20c / AT-I20d / AT-I21 / AT-I22** | **UNCHANGED in substance**, read under the clarified `now` (candidate §4); AT-I21's "same barrier" is the same logical-time barrier; AT-I20's "drain call" is one `process` call |
| Every other AT-I entry (AT-I1 … AT-I19, AT-I23 … AT-I28, AT-I30, AT-I31, AT-I33, AT-I34, AT-I36 … AT-I38, AT-I41) | **UNCHANGED** |
| V2-matrix assertions AT-I40(e) second half (pre-wave digests differ for `{S}` vs `{S,X}`), AT-I45(a) (budget 1 admits `T`), AT-I42(d)(2) (fresh-key scheduling yields `Conflicted`), AT-I44(a) uniform post-extraction equality, AT-I29(d), AT-I43(e)(f)(g)(h), AT-I32 "reaching X-1 … X-4 from a host" as a Rust-privacy claim, AT-I39(C)(D) universal Kills, AT-I46(b) single-test claim | **SUPERSEDED** — each replaced below with the corrected assertion |

---

## 2. Replacement conformance row

> **Scheduler suite (AT-B, AT-G scheduler half).** `Scheduler::schedule`,
> `Scheduler::drain_due`, `Scheduler::canonical_state_digest`, `WorkKey`, `WorkPayload`,
> `DueWorkItem`, `DrainOutcome`, `ScheduleDisposition`, `WorkKeyConflict`, and
> `WorkSlotStatus` are consumed **unmodified in signature and semantics**; the complete
> inherited scheduler corpus runs green and unedited. Phase 2 adds **exactly one**
> scheduler consumption surface — X-1 … X-4 (§10 of the candidate) — and **one**
> engine-owned cross-store extraction over it (candidate §11). **No batch-scheduling
> API lands** (v1 Q6; AT-B7 seam), **no arbitrary-removal API lands**, **no cancellation
> path lands** (ADR-0003 §11), and **no `ActiveRequest` abandonment path lands**. X-1 …
> X-4 are `pub` on `spark-core` and carry no privacy claim; the enforced boundary is the
> `spark-engine` facade (AT-I32). Conformance is proved by AT-I42, AT-I32, and AT-I47,
> not asserted.

---

## 3. AT-I39 — REVISED

**`causal_identity_is_independent_of_drain_partition`**

### (A) Work-producing catch-up

**Fixture.** One scheduled cohort `C @ 100` for profile `P` whose rule creates delayed
work `D` at delay 1 (`D.due_time = 101`); companions at delay 5 (`D @ 105`, outside a
horizon of 101) and a delay placing `D` in a second profile's slice at 101.

**Partitions.** (i) `Advance(101)`; (ii) `Advance(100)`, `Advance(101)`; (iii)
`Advance(100)`, `Advance(100)`, `Advance(101)`; (iv) a budget forcing one cohort per
`process` call to the same final horizon.

**Assertions.** Across every partition, byte-identical: `D`'s complete `WorkKey`
(including `due_time`); the global cohort sequence; each cohort's pre-wave scheduler and
engine digests (observed per §12); each `scheduled_cohort_identity`; every
`effect_batch_digest`, recomputed independently from its declared components; every
emission identity and parent-set digest; obligation record hashes; occurrence mappings;
cooldown writes; canonical semantic reports; the final `engine_state_digest`; and, at
each completed horizon reached by both runs with no active request, the
`stable_boundary_digest`. In the delay-5 fixture `D @ 105` is resident and byte-identical
at horizon 101 in every partition.

**Kills:** evaluation with `now = h` (dates `D` at 102 under (i) and 101 under (ii));
a call-entry due-work plan (leaves `D` unconsumed under (i)); the inherited whole-prefix
`drain_due` path (fails the pre-wave scheduler-digest equality for every cohort not last
in its call). Model S3.

### (B) The original V3-F01 counterexample, pinned by hand

`C1 @ 100`, `C2 @ 101`, one profile, no scheduler-reading rule. Asserted: `C1`'s pre-wave
scheduler digest equals `canonical_state_digest` of an independently constructed
scheduler holding exactly `{C2}`, and `C2`'s equals that of an empty scheduler, in both
the single-call and per-due-time partitions; and `C1`'s pre-wave engine digest equals an
independently built reference engine's holding exactly `{C2}` and the initial stores.
Additionally asserted as an explicit **inequality**: the pre-wave digest differs from the
pre-*extraction* digest.

**Kills:** whole-prefix removal; capture before extraction; drain-and-reinsert (its
reinsertion timing is observable here). Model S3f.

### (C) Expansion is over resident due times, not ticks — narrowed per V2-09 item 7

Cohorts at `due_time` 1 and 1 000 000, nothing between. Asserted: `Advance(1_000_000)`
produces exactly two cohorts with results equal to the stepwise partition; and, through
the `test-support` seam, the **number of X-1 selections performed by the call equals the
number of slices consumed plus one** (the terminal `None`). This is a direct
loop-operation count, not an extraction count.

**Kills:** a tick-iterating implementation, whose X-1 selection count is ~10⁶. Stated
narrowly: this test bounds the selection loop only; it does not measure time.

### (D) Pacing: equal-progress, and the explicit non-claim — corrected per V2-09 item 8

Same request sequence, budgets 1, 2, and 3. Asserted by cohort-sequence position: the
cohort, its members, pre-wave engine digest, identity, batch digests, and the engine
state after it are byte-identical. Asserted at every pair of stable boundaries with
equal cohort count and equal `ActiveRequest`: equal `stable_boundary_digest`. Asserted at
completion: equal `stable_boundary_digest`. Negative control, pinned rather than
universally claimed: at call index 1, budget 1 has processed one cohort and budget 3 has
completed; their engine digests **differ**, and the difference is confined to prefix
length, `F`, `ActiveRequest`, and `PacingDiagnostics`.

**Kills:** the named concrete carry algorithm "`r_next := r_left + B`" (budget left
over at the end of one call is added to the next call's budget): with budget 2, a request
sequence `Advance(10)` (one resident cohort, so `r_left = 1`) then `Advance(30)` (three
resident cohorts), the carry gives the second call budget 3 and admits all three cohorts
(`COMPLETED`, `admitted_cohort_count = 3`), whereas the frozen one-budget-per-call rule
admits two and returns `PAUSED` with `admitted_cohort_count = 2` and
`deferred_cohort_count = 1` — observables that necessarily differ. Other carry shapes are
**not** claimed to be killed by this fixture. Model S12.

### (E) Retained partitions and the retired heartbeat

As matrix v3, each with at least one work-producing cohort; the retired v1 heartbeat
descriptor is computed independently and asserted absent from every canonical digest.
**Kills:** any drain-call-scoped barrier component.

---

## 4. AT-I40 — REVISED

**`cohort_is_scoped_to_profile_and_due_time`**

Retained from matrix v3 and the V2 matrix (a)–(d): pacing cut between profiles; no
cross-slice leakage (key-set equality against a hand-enumerated residual); ascending
consumption under every partition; created work joins the correct slice. Revised:

- **(e) Conflicted membership enters neither identity nor pre-wave state — corrected per
  V2-04.** For slices `A = {S}` and `B = {S, X_conflicted}` at one `(t, P)`, with `X`
  conflicted by two unequal claims while occupied: **equal** `scheduled_cohort_identity`,
  emission identities, pre-wave engine digests, `effect_batch_digest`s, obligation
  removals, and final `engine_state_digest`; **different** pre-*extraction* scheduler
  digests and canonical conflict reports (the `B` report carries `X`'s `WorkKeyConflict`;
  the `A` report carries none). Both halves are asserted.
  **Kills:** the first-pass D-3 (identity over the whole slice) and the V2 matrix's own
  AT-I40(e) second half (which would reject a conforming implementation); an
  implementation that leaves `X` resident, detected by the *next* cohort's pre-wave
  digest differing from the reference. Model S15.

---

## 5. AT-I29 — EXTENDED

**`wave_buffers_do_not_outlive_the_wave`**

As matrix v3 and V2 (a)–(c): no due-work plan at any stable boundary; mid-cohort
reconstruction from committed state plus the extraction value; restart across a paced
boundary reproduces every subsequent canonical value. **(d) is deleted**: `Φ` no longer
exists, and the request-boundary state it stood in for is tested as retained,
digest-committed state in AT-I47, not "discovered" by a test. (c) is read with AT-I47(d):
the restart carries `F` and `ActiveRequest`, and the exact request is re-presented.

**Kills:** any retained plan, tail buffer, continuation cursor, budget carry, or
in-memory-only pacing state.

---

## 6. AT-I32 — EXTENDED (compile probes; each must fail to compile) — corrected per V2-08

Probes over the **concrete engine/host facade**, not over `spark-core` privacy:

- obtaining `&Scheduler` or `&mut Scheduler` from any public `spark_engine` item; calling
  X-1 … X-4 through any public `spark_engine` path; calling `drain_due` on the engine's
  scheduler through any public `spark_engine` path;
- storing a `LeastDueSlice<'_>` in any struct, ledger, or store that outlives its borrow;
  mutating one; reading a payload or conflict evidence from it;
- any `spark_engine` method removing work by `WorkKey`, key list, range, predicate,
  count, coordinate, or `due_time` alone; any batch or multi-item `schedule`, reschedule,
  reinsert, or cancellation path;
- constructing an evaluation context from a horizon, from `F`, or from `ActiveRequest`,
  or reading any of them from inside rule/threshold/obligation/cooldown evaluation;
- passing `DueSliceSummary`, `PacingDiagnostics`, `F`, or `ActiveRequest` into rule
  evaluation;
- setting, clearing, or constructing `ActiveRequest` or `F` from outside `spark_engine`
  (no public constructor, field, setter, `From`, or deserialization path; snapshot
  restore is the only ingress and validates per candidate §6.5);
- any public `spark_engine` path that abandons, replaces, or cancels an active request.

**Stated honestly:** a crate that depends directly on `spark-core` **can** call X-1 …
X-4 and `drain_due` on a `Scheduler` it constructs itself. No probe claims otherwise; the
probes prove the engine's own scheduler and request-boundary state are unreachable
through the engine's public surface.

**Kills:** any reintroduction of an owned removal capability; any arbitrary-removal
surface; any horizon, `F`, or `ActiveRequest` leak into evaluation; any host-settable
request-boundary state.

---

## 7. AT-I35 — EXTENDED

As the V2 matrix §7: `schedule` and `drain_due` signature/behavior pins; Phase-1
encodings bit-identical; the slice fingerprint's per-slot block asserted byte-equal to the
block `canonical_state_digest` builds, over scheduled and conflicted slots at and beyond
`MAX_CONFLICT_EVIDENCE` and `MAX_CONFLICT_TRACKED_CLAIMS`. **Additional pin:**
`canonicalize(ActiveRequest)` and the `stable_boundary_digest` composition are pinned by
an independent reference implementation over `None`, `Some(Advance)`, and
`Some(Command)`; `engine_state_digest`'s component list is pinned equal to v1 §8's.

**Kills:** a key-only fingerprint; a fingerprint over the evidence projection; a new
encoding; any implementation that lets `F` or `ActiveRequest` into `engine_state_digest`.

---

## 8. AT-I42 — REVISED

**`least_due_compare_and_take_is_atomic_and_unprivileged`**

(a) atomic single-slice removal; (b) mixed slice; (c) non-least selection refused with
the live least fingerprint returned; (e) same-key `Scheduled → Conflicted` between
observation and extraction; (f) constructible `NoSliceDue`; (g) every failure a
byte-identical no-op; (h) scan-before-mutation; (i) Phase-1 refinement in its static /
null-evaluator scope; (j) determinism and permutation invariance; (k) totality — all as
the V2 matrix. Revised and added:

- **(d) Replay is possible and unprivileged — sub-case (2) corrected per V2-07.**
  Extract `(100, P)` retaining its fingerprint. (1) Re-create the identical `WorkKey`
  with an identical payload so the slice is again least and byte-identical: the take
  **succeeds** and equals a take from a fresh observation. (2) Re-create the identical
  key with a **different** payload: the slot is `Scheduled(new)` — Phase-1 frees a
  drained key — so the fingerprint differs and the replay is a typed byte-identical
  no-op; a fresh observation then extracts `Scheduled(new)`. (2′) Re-create the key with
  payload `B`, then schedule `C` under the same key so the slot is `Conflicted{B, C}`:
  the replay is a typed no-op and a fresh observation extracts the conflicted slot with
  the complete claim set. (3) Re-create a different key so another slice is least: typed
  no-op.
  **Kills:** the V2 matrix's own (d)(2) expectation of `Conflicted` (would reject
  conforming code); key-only comparison (would let (2) remove newly scheduled work).
  Model S18.
- **(l) Cross-store preflight, missing record (V2-05).** Remove a `Scheduled` key's
  obligation record through the `test-support` seam; call the engine extraction: typed
  `ObligationRecordMissing { key }`; scheduler digest, obligation-store digest, and
  `engine_state_digest` byte-identical; nothing evaluated; no report emitted.
- **(m) Cross-store preflight, mismatched record.** Replace the record with one whose
  hash differs from the slot's `canonical_payload_hash`: typed
  `ObligationRecordMismatch`; same byte-identity assertions. For a conflicted slot,
  replace its contested set with one missing a claim: typed `ObligationClaimSetMismatch`.
- **(n) Extracted records reach evaluation.** With a consistent store, the extraction's
  `executable` carries the full `MaterializedEffect` / `RuleReEvaluation` record for
  every key, asserted equal to the record that was in the store; the evaluator executes
  from that record and the store no longer contains it; the bidirectional invariant holds
  before, after, at the pre-wave capture, and after a rejection.
  **Kills (l–n):** scheduler-first removal followed by a failing record lookup (fails the
  byte-identity in (l)); removal of records at commit rather than extraction (leaves
  orphans after a rejection, fails (n)); an extraction returning payload hashes only
  (fails (n)). Model S13.

---

## 9. AT-I43 — REWRITTEN

**`request_lifecycle_is_serialized_and_time_ordered`**

- **(a) Lifecycle.** With `A@10`, `B@20` resident and budget 1: `process(Advance(20))`
  returns `PAUSED` with `F` unchanged and `ActiveRequest = Some(Advance, 20)`; a second
  `process(Advance(20))` returns `PAUSED` then `COMPLETED` with `F = 20` and
  `ActiveRequest = None`; a `Command@21` offered during the pause is processed only after.
  **Kills:** an implementation that starts a later request while one is paused.
  Model S1.
- **(b) Budget-independent order and result.** Budgets 1, 2, and 3 over `Advance(20)`,
  `Command@21`: identical cohort sequence `(10), (15), (20), cmd@21`, identical per-cohort
  values, identical final `engine_state_digest`, `F`, and `stable_boundary_digest`;
  only call counts and `PacingDiagnostics` differ. **Kills:** ALT-N (immediate command
  execution) and ALT-R (`Φ` refusal) by the command's identical position; any
  budget-dependent canonical result. Model S2, S12.
- **(c) Cohort-local evaluation time reaches the rule layer.** Decay/recovery and
  cooldown vectors in a cohort at `due_time = τ` compute against `now = τ` across the
  partitions of AT-I39(A), pinned by closed-form expected values.
  **Kills:** passing `h`, `F`, or the clock frontier as `now`. Model S3, S6.
- **(d) Start rule.** After `Advance(20)` completes: `Command@15` →
  `REFUSED_HORIZON_BEHIND_FRONTIER`, nothing staged, timeline digest and
  `stable_boundary_digest` unchanged, `ActiveRequest = None`; `Command@20` and
  `Command@25` → `COMPLETED` with the timeline containing exactly those two ordinals.
  **Kills:** an implementation that evaluates the past-dated command (the companion decay
  assertion would compute a negative elapsed span). Model S4.
- **(e) SB-01 substitution — the mandatory counterexample.** `F = 0`; work at 10, 15,
  and 20; budget 3; `process(Advance(20))` consumes 10 and 15 (cells `updated_at = 15`)
  and returns `PAUSED`. Present `Command@12`: `REFUSED_ACTIVE_REQUEST_MISMATCH` carrying
  `active = (Advance, 20)` and `presented = (Command, 12, id)`; `stable_boundary_digest`,
  `engine_state_digest`, timeline, `F`, and `ActiveRequest` byte-identical; no cohort
  evaluated. Then present `Advance(25)`: refused identically. Then present
  `Advance(20)`: `COMPLETED`, `F = 20`, `ActiveRequest = None`.
  **Kills:** the addendum's zero-active-state design (which accepts `Command@12` because
  `12 ≥ F`); any implementation binding only the horizon (would accept a `Command@20`
  substitution — see (f)). Model S7, S7b, S7c.
- **(f) Same-horizon different request.** While `Advance(20)` is paused, present
  `Command@20` and, separately, a second `Command@20` with a different payload: both
  refused with mismatch; state byte-identical. **Kills:** a discriminator over horizon
  alone, or over kind and horizon alone. Model S8b, S9.
- **(g) Equal-horizon advance after completion.** After `Advance(20)` completes, a
  further `Advance(20)` is `COMPLETED` with no cohort and byte-identical
  `stable_boundary_digest`. **Kills:** an implementation refusing `h = F`. Model S10b.
- **(h) Command finalization at completion.** `Command@21`: no ordinal is consumed
  before completion; at completion the envelope's `input_ordinal` equals the timeline
  frontier, exactly one positive `STAGED` acknowledgement and one one-ordinal fence occur,
  the command cohort evaluates at `now = 21` against the state after every slice `≤ 21`,
  and the timeline's `canonical_state_digest` shows no staged-unfinalized slot at any
  stable boundary. With budget exhausted before the command: `PAUSED` with the timeline
  unchanged; the next call finalizes and executes with a result equal to the unbudgeted
  run. **Kills:** staging at request start (a staged slot would be observable at the
  paused boundary); a pre-issued ordinal (a start-refused command would burn it, making
  the next fence's `start_ordinal ≠ frontier`). Model S19.
- **(i) Finalization refusal.** A command whose `command_id` was already finalized:
  `COMPLETED_COMMAND_NOT_FINALIZED { timeline_result }`, timeline unchanged, frontier
  unchanged, command cohort not evaluated, `F = effective_time`, `ActiveRequest = None`;
  a subsequent request below `F` is refused at start. **Kills:** an implementation that
  leaves `F` behind after consuming scheduled work through `effective_time` (a later
  `Command@(F_old + 1)` would evaluate backward against cells updated at
  `effective_time`). Model S17, S17b.
- **(j) Checked delayed-time arithmetic.** `due_time = canonical_time + delay` at
  `u64::MAX` rejects the wave atomically with a typed error and no mutation.
  **Kills:** wrapping or saturating arithmetic.
- **(k) Mailbox is not the mechanism.** Drive the engine directly, bypassing any consumer,
  with the sequence of (e): the refusals occur identically. **Kills:** an implementation
  whose safety depends on the consumer's discipline.

Deleted from the V2 AT-I43: (a) "a command barrier drains no scheduled work" (commands
now execute at completion after the drain), (b)/(c) history-position and same-time
ordering (a request's horizon orders it; a command at `e` follows every slice `≤ e`),
(e)–(h) SH-1/SH-2/`Φ`/defense check.

---

## 10. AT-I44 — REVISED

**`every_rejection_class_is_terminal_and_cleans_its_stores`**

For each of R-1 … R-7 (candidate §13), fixtures split by rejection point:

- **(a) Wave-0 rejection.** The extracted keys are absent from the scheduler; their
  obligation records absent; no cell, occurrence, cooldown, or enqueue applied; the
  `engine_state_digest` **equals the post-extraction (captured pre-wave) digest**.
- **(a′) Later-wave rejection.** A cohort whose wave 0 commits a cell update and whose
  threshold emission makes wave 1 contain an unequal same-family RESULT: wave 0's batch
  digest exists and its effects are committed; wave 1 applies nothing; the final
  `engine_state_digest` equals the digest **after wave 0's commit** and is asserted, as an
  explicit inequality, **different from** the post-extraction digest. The same fixture at
  wave 2 (two committed waves) pins the state after wave 1.
  **Kills:** whole-cohort rollback (fails the inequality); the V2 matrix's uniform (a).
  Model S14.
- **(b) Bidirectional invariant** at every observable point, including immediately after
  extraction, at the pre-wave capture, after each committed wave, and after the rejection.
- **(c) Report**, separated by class: conflict-only (R-6: a slice with only conflicted
  slots yields a report with the conflict section and no cohort outcome); R-7 obligation
  refusal (typed, nothing reinterpreted); R-5 semantic cap (the v2 §6.3 overload report
  in conflicted-drain shape). Each report's evidence is order-independent.
- **(c′) Mixed slice whose scheduled portion rejects.** `{S, X_conflicted}` with `S`'s
  wave 0 rejecting: the report is exactly `[conflict section: X] ‖ [cohort outcome:
  typed wave-0 rejection]` in that order; `X`'s claim set was removed as a complete set;
  the post-state equals the post-extraction digest. **Kills:** an implementation that
  orders the rejection before the conflicts, or that lets `X` into the candidate set.
- **(d) Retry under both identities** — next occurrence index and the same `WorkKey` —
  both accepted as new canonical input with fresh records.
- **(e) Replay** byte-identical, caps being epoch-bound.
- **(f) No resurrection**; progress by strict decrease of cohorts ahead of a fixed later
  cohort.
- **(g) Command cohort rejected at wave `n`.** The command is finalized (ordinal
  consumed, timeline digest advanced); waves `< n` committed; the typed outcome is in the
  command's canonical report; the request result is `COMPLETED`; no re-finalization
  path exists. **Kills:** an implementation that un-finalizes or re-queues a rejected
  command.

Deleted: every R-8 / command-refusal row, `EFFECTIVE_TIME_BEHIND_FRONTIER` fixtures, and
`X`-advancement assertions.

**Kills (overall):** reinsertion of rejected work (livelock, partition-dependent
sequence); record removal at commit rather than extraction (orphans after rejection);
any rollback of the extraction; the first-pass CE-9 and the V2 AT-I44(a) as written.

---

## 11. AT-I45 — REVISED

**`pacing_counts_executable_work_only`**

- **(a) Admission boundary — corrected per V2-03.** Slice `(100, P) = {S scheduled,
  X conflicted}` and `(101, P) = {T scheduled}`. With `max_due_per_cycle = 2`:
  executable-only counting admits `S` (count 1) then `T` (count 1) in one call —
  `COMPLETED`, two cohorts. With `max_due_per_cycle = 1`: `S` admits normally (no
  oversized exception, `pacing_overrun = false`), `T` is **deferred** with
  `deferred_cohort_count = 1` and `earliest_deferred_due_time = 101`, and the call returns
  `PAUSED`. **Kills:** conflict-inclusive counting, under which budget 2 treats
  `(100, P)` as size 2, consumes the budget, and defers `T` (differs at budget 2), and
  budget 1 fires the oversized exception with `pacing_overrun = true` (differs at
  budget 1). Model S15b.
- **(b) All-conflicted slice.** Extracted and reported; no cohort identity, wave, or
  batch; consumes no budget; does not increment `admitted_cohort_count`; the next slice is
  admitted in the same call with the full remaining budget. **Kills:** the first-pass
  AT-I42(c); an implementation that stops the loop on it.
- **(c) Oversized exception premise.** An all-conflicted slice preceding an oversized
  executable slice does not disturb the exception. Model S16b.
- **(c′) Residual conflicts after the oversized exception — per V2-10.** Oversized
  `(100, P)` (3 keys, budget 2), all-conflicted `(101, P)` and `(102, P)`, executable
  `(103, P)`: one call admits `(100, P)` whole, consumes `(101, P)` and `(102, P)` at
  zero cost with their reports, defers `(103, P)`, and returns `PAUSED` with
  `deferred_cohort_count = 1`, `earliest_deferred_due_time = 103`. **Kills:** an
  implementation stopping before `(101, P)` (returns `PAUSED` with residual conflicted
  slots and `deferred_cohort_count` computed over them, or 0); conflict-inclusive deferred
  counts. Model S16.
- **(d) Executable-only diagnostics.** `deferred_cohort_count` and
  `earliest_deferred_due_time` are asserted against an independently computed
  executable-only expectation for every fixture above; `PAUSED` implies
  `deferred_cohort_count ≥ 1` or a deferred command. **Kills:** any diagnostic counting
  conflicted-only slices.
- **(e) Pacing remains semantics-neutral.** Over (a)–(c′), paced and unbudgeted runs are
  compared per cohort-sequence position and at completion; only `PacingDiagnostics`
  differ.

---

## 12. AT-I46 — REVISED

**`pre_wave_observation_is_test_only`** — three separate obligations per V2 review §8
item 12:

- **(a) No canonical report field.** The canonical semantic report's field set is pinned
  equal to the v3 field set; no `pre_wave_*`, `cohort_identity`, `F`, or `ActiveRequest`
  field. **Kills:** CE-8/D-6.
- **(b) Feature absence.** `workspace_dependency_direction.rs` passes with the seam
  present: no production dependency edge enables `test-support`; a build without the
  feature does not expose the observation points (probe: the observation function is
  unresolvable).
- **(c) Report-schema stability.** The canonical report type's canonical encoding is
  byte-identical with and without the feature.
- **(d) Evaluator-type non-reachability.** A compile probe asserts the evaluator's input
  types cannot carry a `Digest` obtained from the observation seam (the seam returns a
  distinct newtype with no conversion into any evaluation-context type).
- **(e) The seam observes what AT-I39 needs** at the post-extraction, pre-evaluation
  point, plus `F` and `ActiveRequest` at every stable boundary for AT-I47.

**Kills:** satisfying the oracle by widening a production surface; claiming (d) from (b).

---

## 13. AT-I47 — NEW

**`request_boundary_state_is_committed_and_discriminated`** (the AT-G obligations for `F`
and `ActiveRequest`, required from their first implementation commit)

- **(a) Discrimination.** Five engines with equal `engine_state_digest` and equal `F`:
  `ActiveRequest = None`; `Some(Advance, 20)`; `Some(Command, 20, id₁)`;
  `Some(Advance, 21)`; `Some(Command, 20, id₂)`. Asserted: five pairwise-distinct
  `stable_boundary_digest`s, and equal `engine_state_digest`s. Two engines differing only
  in `F` (equal stores, `None`): distinct `stable_boundary_digest`, equal
  `engine_state_digest`. **Kills:** omission of `F` or of `ActiveRequest` (or of any of
  its three fields) from `stable_boundary_digest`; inclusion of either in
  `engine_state_digest`. Model S9.
- **(b) Equivalence — same next input.** Two engines reaching equal
  `stable_boundary_digest` by permuted construction orders: for each of the next requests
  `Command@12` (mismatch), `Advance(20)` (resume), `Advance(5)` after completion
  (behind frontier), and `Command@20`: identical result and identical post-request
  `stable_boundary_digest`. **Kills:** any hidden request-boundary state. Model S9b.
- **(c) Negative control — placement.** Recompute AT-I39(A)'s per-cohort pre-wave digest
  with `F` appended and, separately, with `canonicalize(ActiveRequest)` appended: the
  one-call and stepwise partitions **diverge** (at the cohorts at 15 and 20 for `F`; at
  every cohort for `ActiveRequest`), while the unappended digests agree. **Kills:** any
  proposal to move either value into the per-cohort digest. Model S3d, S3e.
- **(d) Snapshot and restore.** Snapshot at a paused boundary; restore; assert the
  restored `stable_boundary_digest` equals the recorded one; present the exact request:
  continuation byte-identical to the uninterrupted run; present a same-horizon different
  request: refused, byte-identical. Snapshot at a completed boundary: `ActiveRequest =
  None` restored. **Kills:** restart without `ActiveRequest` (accepts the substitute);
  restart without `F`. Model S5, S8, S8b.
- **(e) Restore validation.** A snapshot edited through the seam so `Some(d)` has
  `d.horizon < F`: `RESTORE_REJECTED_ACTIVE_BEHIND_FRONTIER`, nothing live. A snapshot
  whose `ActiveRequest` is replaced by `None` without recomputing the digest:
  `RESTORE_REJECTED_DIGEST_MISMATCH`. **Kills:** a restore that trusts `F` and
  `ActiveRequest` independently of the stores. Model S8c.
- **(f) Absence after completion and after start refusal.** After `COMPLETED`,
  `ActiveRequest = None` and `F = h`; after `REFUSED_HORIZON_BEHIND_FRONTIER`,
  `ActiveRequest = None`, `F` unchanged, `stable_boundary_digest` unchanged; after
  `REFUSED_ACTIVE_REQUEST_MISMATCH`, `ActiveRequest` still the original `Some`, `F`
  unchanged, digest unchanged; after `COMPLETED_COMMAND_NOT_FINALIZED`,
  `ActiveRequest = None` and `F = effective_time`. **Kills:** residue on any path.
  Model S10, S17.
- **(g) Set before any cohort mutation.** Through the seam, observe the boundary state
  immediately after the first extraction of a request's first call: `ActiveRequest` is
  already `Some(d)`. A forced panic-free abort injected between extraction and
  evaluation (test-only) leaves `Some(d)` persisted so the restart accepts only `d`.
  **Kills:** setting `ActiveRequest` at pause time only.
- **(h) No other transition.** Epoch activation, an ADR-0003 §11 epoch reset, and every
  public `spark_engine` operation other than `process` and restore leave `ActiveRequest`
  and `F` byte-identical; a compile probe (AT-I32) asserts no public setter.
  **Kills:** any abandonment or host-set path.

---

## 14. Traceability

### 14.1 To the SB review §6 oracle corrections

| SB review §6 item | Test |
|---|---|
| 1 pause through 15, substitute `Command@12`, typed no-op | AT-I43(e) |
| 2 pause, restart, exact request, byte-identical continuation | AT-I47(d), AT-I29(c) |
| 3 pause, restart, same-horizon different request, refusal | AT-I47(d), AT-I43(f) |
| 4 two budgets: per-cohort by position; stable-boundary at equal progress | AT-I39(D), AT-I43(b) |
| 5 `ActiveRequest` discrimination/equivalence in `stable_boundary_digest` | AT-I47(a)(b) |
| 6 absence after completion and after start refusal | AT-I47(f) |
| 7 negative control: `F` out of the per-cohort digest | AT-I47(c) |
| 8 preflighted atomic engine-plus-obligation-store extraction | AT-I42(l)(m)(n) |

### 14.2 To the V2 review findings

| Finding | Corrected in |
|---|---|
| V2-03 budget-one fixture | AT-I45(a) |
| V2-04 `{S}` vs `{S, X}` digests | AT-I40(e) |
| V2-05 cross-store extraction | AT-I42(l)(m)(n), AT-I44(b) |
| V2-06 rejection atomicity | AT-I44(a)(a′)(c)(c′)(g) |
| V2-07 replay sub-case | AT-I42(d)(2)(2′) |
| V2-08 visibility | AT-I32, §2 row |
| V2-09 non-discriminating Kills | AT-I39(C)(D), AT-I46(b)(d) |
| V2-10 diagnostics and loop | AT-I45(b)(c′)(d), AT-I44(c′) |
| V2 review §8 items 1–12 | 1 → AT-I45(a); 2 → AT-I40(e); 3 → AT-I47 (no `Φ`); 4 → AT-I42(d); 5 → AT-I44; 6 → AT-I32; 7 → AT-I39(C); 8 → AT-I39(D); 9 → AT-I45(c′)(d); 10 → AT-I44(c′); 11 → AT-I42(l)(m)(n); 12 → AT-I46 |

### 14.3 To the mission's required kills

| Wrong implementation the oracle must necessarily kill | Test |
|---|---|
| whole-prefix draining; pre-extraction digest capture | AT-I39(B) |
| horizon-time evaluation; call-entry due-work plans | AT-I39(A), AT-I43(c) |
| nonleast or stale-fingerprint extraction | AT-I42(c)(d)(e) |
| non-atomic scheduler/obligation-store extraction | AT-I42(l)(m)(n) |
| conflicted work entering executable identity or pacing | AT-I40(e), AT-I45(a) |
| whole-cohort rollback after a later-wave rejection | AT-I44(a′) |
| paused-request substitution, incl. a lower-horizon command after later work | AT-I43(e)(k) |
| restart with the exact request vs a same-horizon different request | AT-I47(d), AT-I43(f) |
| omission of `F` or `ActiveRequest` from `stable_boundary_digest` | AT-I47(a) |
| inclusion of either in per-cohort `engine_state_digest` | AT-I47(a)(c), AT-I35 |
| active-request residue after completion or start-time refusal | AT-I47(f) |
| budget-dependent canonical cohort or command results | AT-I43(b), AT-I39(D), AT-I45(e) |

### 14.4 To the model

Each **Model S…** reference above names the disposable check that demonstrates the
assertion at architecture level. The model is not a test result.

---

## 15. Out of scope (unchanged)

No persistence, service, or protocol tests; no mailbox/transport implementation; no
profile parsing or IO; no executable Windows/Android runs; no Phase-4+ semantics; no
G.A.M.E. integration; no runtime-LLM surface. **Phase-2 implementation is not authorized
by this oracle; Phase 3 is not authorized and is not pre-authorized here.** This is a
proposed oracle awaiting independent review, not an accepted one, and no test in it has
been executed.
