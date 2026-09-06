# S.P.A.R.K. Phase 2 — Acceptance Test Oracle, V3-F01 Correction Candidate **V2** (AT-I)

**Date:** 2026-09-06
**Agent:** Claude Code (Opus 5), Gate C1 architecture correction writer (second pass)
**Status:** **CANDIDATE — AWAITING INDEPENDENT REVIEW.** Not accepted, not frozen, not
canonical, and authorizing no implementation.
**Companion to:**
`SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_V2_2026-09-06.md` (unqualified
"§" references are to that document; "adj §…" to
`SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md`; "v3 §…",
"v2 §…", "v1 §…" to the three Phase-2 freezes; "review §…" to
`SPARK_PHASE_2_CODEX_V3_F01_INDEPENDENT_REVIEW_2026-09-06.md`).
**Amends:** `SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v3_2026-08-26.md` (retained unmodified
as history) in exactly the entries named in §1.
**Supersedes (as a proposal):**
`SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_2026-09-06.md`, which is
preserved **unchanged** as rejected review history.

**Discipline (unchanged from v1/v2/v3).** Every AT-I test is encoded red-first; baselines
are recorded verbatim before any production change, and "inexpressible against the
inherited tree" is recorded, never skipped. Every "X is impossible" claim is a digest or
value equality, or a compile-fail — never an error-was-returned check alone. No existing
test is weakened, deleted, or skipped; the Phase-1 suite runs unmodified throughout.

**Falsification discipline added by this pass.** Every oracle below carries an explicit
**Kills:** line naming the concrete incorrect implementation it must reject. An oracle
that cannot name one is not evidence, and the independent review found several such rows
in the rejected matrix. Tests are specified here; **none has been executed**, no
production Rust exists to run them against, and this document claims no test result.

---

## 1. Exact delta

| Entry | Disposition |
|---|---|
| Matrix v3 "`Scheduler` consumed, not modified; no batch API" conformance row | **REPLACED** by §2 |
| **AT-I39** `causal_identity_is_independent_of_drain_partition` | **REVISED** — §3 |
| **AT-I40** `cohort_is_scoped_to_profile_and_due_time` | **REVISED** — §4 |
| **AT-I29** `wave_buffers_do_not_outlive_the_wave` | **EXTENDED** — §5 |
| **AT-I32** strict lint / compile-probe gate | **EXTENDED** — §6 |
| **AT-I35** Phase-1 digest stability | **EXTENDED** — §7 |
| **AT-I42** `least_due_compare_and_take_is_atomic_and_unprivileged` | **NEW (V2 definition, replacing the rejected AT-I42)** — §8 |
| **AT-I43** `command_barriers_are_placed_and_time_guarded` | **NEW** — §9 |
| **AT-I44** `every_rejection_class_is_terminal_and_cleans_its_stores` | **NEW** — §10 |
| **AT-I45** `pacing_counts_executable_work_only` | **NEW** — §11 |
| **AT-I46** `pre_wave_observation_is_test_only` | **NEW** — §12 |
| **AT-I20 / AT-I20b / AT-I20c / AT-I20d / AT-I21 / AT-I22** | **UNCHANGED in substance**, read under §3's clarified `now` (adj §9); AT-I21's "same barrier" means the same logical-time barrier |
| Every other AT-I entry | **UNCHANGED** |
| Rejected-matrix assertion **AT-I40(d)** (removing a conflicted key changes cohort identity) | **SUPERSEDED — its inverse is now asserted** (§4(e)) |
| Rejected-matrix assertions keyed to CE-8 report fields (AT-I39(a)(b) as canonical report reads) | **SUPERSEDED** — same equalities, observed through the `test-support` seam (§12) |
| Rejected-matrix **AT-I42(c)(d)** (all-conflicted consumes budget; three-error taxonomy) | **SUPERSEDED** by §8 and §11 |

---

## 2. Replacement conformance row

> **Scheduler suite (AT-B, AT-G scheduler half).** `Scheduler::schedule`,
> `Scheduler::drain_due`, `Scheduler::canonical_state_digest`, `WorkKey`, `WorkPayload`,
> `DueWorkItem`, `DrainOutcome`, `ScheduleDisposition`, `WorkKeyConflict`, and
> `WorkSlotStatus` are consumed **unmodified in signature and semantics**; the complete
> inherited scheduler corpus runs green and unedited. Phase 2 adds **exactly one**
> consumption surface: the least-due read handle, its non-authorizing slice fingerprint,
> the least-due compare-and-take extraction, and the noncanonical due-slice summary
> (§5 X-1 … X-4). **No batch-scheduling API lands** (v1 Q6; AT-B7 seam contract at
> `scheduler.rs:424–431`), **no arbitrary-removal API lands**, and **no cancellation path
> lands** (ADR-0003 §11). The surface is engine-internal by contract and appears on no
> host, service, or protocol surface. Conformance is proved by AT-I42 and AT-I32, not
> asserted.

---

## 3. AT-I39 — REVISED

**`causal_identity_is_independent_of_drain_partition`**

### (A) Work-producing catch-up — the mandatory review §5.1 counterexample

**Fixture.** One scheduled cohort `C @ due_time 100` for profile `P`, whose rule creates
delayed work `D` at delay 1, so `D.due_time = 101` under §4.3. Companion fixtures: delay 5
(`D @ 105`, outside a horizon of 101) and a delay that places `D` in a slice for a second
profile at 101.

**Partitions.** (i) one call `advance(101)`; (ii) `advance(100)` then `advance(101)`;
(iii) `advance(100)`, `advance(100)` (idempotent repeat), `advance(101)`; (iv) a pacing
budget forcing one cohort per call to the same final horizon.

**Assertions.** Across every partition, byte-identical: `D`'s complete `WorkKey`
(including `due_time`, which is the field the defect moved); the global cohort sequence;
each cohort's pre-wave scheduler digest and full pre-wave engine digest (observed per
§12); each `scheduled_cohort_identity`; every `effect_batch_digest`, recomputed
independently from its declared components rather than compared as an opaque value; every
emission identity and parent-set digest; obligation record hashes; occurrence mappings;
cooldown writes; canonical semantic reports; and the final engine digest. In the delay-5
fixture, `D @ 105` is resident and byte-identical at horizon 101 in every partition.

**Kills:**
- an implementation that evaluates cohorts with `now = T` (the horizon), which dates `D`
  at `102` under (i) and `101` under (ii);
- an implementation that pins a call's drain set at call entry, which leaves `D`
  unconsumed under (i) while (ii) consumes it;
- an implementation that iterates integer ticks rather than resident due times, which
  changes nothing canonically but is detected by the idle-span cost assertion in (C);
- the inherited whole-prefix `drain_due` consumption path, which fails the pre-wave
  scheduler-digest equality for every cohort that is not last in its call.

### (B) The original V3-F01 counterexample, pinned by hand

`C1 @ 100` and `C2 @ 101` for one profile, no rule reading or mutating the scheduler.
Asserted: `C1`'s pre-wave scheduler digest equals `canonical_state_digest({C2})` and
`C2`'s equals `canonical_state_digest(∅)` in **both** the single-call and per-due-time
partitions, with the reference digests built independently from a separately constructed
scheduler holding exactly the expected residual slots.

**Kills:** any whole-prefix removal; any capture of the digest *before* the extraction
(asserted as an explicit inequality against the pre-extraction digest); any
drain-and-reinsert scheme, whose reinsertion timing is observable here.

### (C) Expansion is over resident due times, not ticks

A fixture with cohorts at `due_time` 1 and 1 000 000 and nothing between. Asserted: one
call `advance(1_000_000)` produces exactly two barriers and two cohorts; the canonical
results equal the stepwise partition; and the number of extraction attempts is bounded by
the number of resident slices (asserted through the `test-support` observation seam, as a
cost property, not a canonical one).

**Kills:** a tick-iterating implementation, which performs ~10⁶ empty barriers and breaks
the Phase-0 budget's catch-up row.

### (D) Pacing: equal-prefix, and the explicit non-claim

Same history, two budgets. Asserted **by cohort-sequence position**: for every position
both runs have reached, the cohort, its members, its pre-wave engine digest, its identity,
its batch digests, and the engine state after it are byte-identical. Additionally asserted:
under precondition PE-C (segment drained to completion) the two runs' engine states and
canonical report sequences at segment end are byte-identical. Explicitly asserted **not**
to hold, as a negative control: equality of engine digests at equal *call index* when the
budgets differ — the test pins that they differ and that the difference is confined to
prefix length and `PacingDiagnostics`.

**Kills:** an implementation that carries remaining budget across calls (which would make
the prefix lengths agree and the negative control fail); an oracle that mistakes
equal-call-index equality for the real claim, which is the imprecision this pass was
directed to remove.

### (E) Retained partitions and the retired heartbeat

Matrix v3's partitions (one call; budget forcing two calls; one call per cohort; ten due
times in one call versus ten calls; the split-batch R-044 variant; a pacing cut between
two profiles at one due time) are retained unweakened, now with at least one
work-producing cohort in each. The retired v1 heartbeat descriptor
`H("heartbeat" ‖ logical time ‖ digest of drained WorkKeys)` is computed independently and
asserted absent from every canonical digest.

**Kills:** any reintroduction of a drain-call-scoped barrier component.

---

## 4. AT-I40 — REVISED

**`cohort_is_scoped_to_profile_and_due_time`**

Retained from matrix v3: two profiles at one `due_time` form two cohorts with distinct
identities in ascending `(due_time, profile_id)` order; the partition is contiguous in the
frozen `WorkKey` total order; no co-target causal group is split; adding a third profile
perturbs neither of the first two. Revised and extended:

- **(a) Pacing cut between profiles.** With `(t,P1)` and `(t,P2)` due and a budget
  admitting only `(t,P1)`: at `(t,P1)`'s pre-wave point, `(t,P2)` is resident and
  byte-identical; both cohorts' canonical values equal the unbudgeted run's, per
  cohort-sequence position. **Kills:** whole-prefix removal, which deletes `(t,P2)`.
- **(b) No cross-slice leakage.** Extracting `(t,P1)` removes no slot of `(t,P2)`,
  `(t',P1)`, or any other slice — asserted as key-set equality against a hand-enumerated
  residual set, not a count. **Kills:** range removal keyed on `due_time` alone.
- **(c) Ascending consumption under every partition**, including when a barrier's cohorts
  span calls. **Kills:** any implementation that resumes from a stored cursor rather than
  the live least slice.
- **(d) Created work joins the correct slice.** A cohort at `τ` creating work for a second
  profile at `τ+1` produces, at barrier `τ+1`, exactly the slices the unbudgeted single
  call produces, in the same order. **Kills:** an implementation that appends created work
  to a call-entry plan rather than re-deriving the least slice.
- **(e) Conflicted membership does not enter identity — the inverse of the rejected
  assertion.** For slices `A = {S}` and `B = {S, X_conflicted}` at the same `(t,P)`:
  `scheduled_cohort_identity(A) == scheduled_cohort_identity(B)`, and every emission
  identity of the two runs is equal; **and** the pre-wave engine digests, the
  `effect_batch_digest`s, and the canonical conflict reports **differ**, because `X`
  changes the scheduler digest and is reported. Both halves are asserted.
  **Kills:** the rejected candidate's D-3 (identity over the whole slice), which fails the
  first half; and any implementation that leaves `X` resident, which fails the second half
  by producing an unchanged pre-wave engine digest at the *next* cohort.

---

## 5. AT-I29 — EXTENDED

**`wave_buffers_do_not_outlive_the_wave`**

As matrix v3, extended with:

- **(a) No due-work plan at any stable boundary.** At every stable boundary — between
  calls, and after each cohort commits or rejects — an engine reconstructed from committed
  state alone reproduces the entire remaining cohort sequence: identities, pre-wave
  digests, emission identities, batch digests, obligation record hashes, occurrence
  mappings, reports, and the final engine digest. **Kills:** any retained plan, tail
  buffer, continuation cursor, or budget carry.
- **(b) Mid-cohort reconstruction.** Reconstruction at the post-extraction, pre-evaluation
  point reproduces that cohort's evaluation from committed state plus the extraction value
  alone. **Kills:** an implementation whose evaluation depends on the read handle
  outliving the extraction.
- **(c) Restart across a paced boundary.** Forcing an engine restart between two calls in
  the middle of a paced segment reproduces every subsequent canonical value.
  **Kills:** any in-memory-only pacing state.
- **(d) `Φ` is reconstructible (candidate §14 item 5).** The canonical time frontier used
  by SH-1/SH-2 is recomputed from committed state after a restart and drives identical
  command admission/refusal decisions. If it cannot be so recomputed, this test fails and
  the reviewer's attention is directed to R1: `Φ` would then be retained state requiring
  entry into the v1 §8 engine digest. **Kills:** an implementation holding `Φ` only in
  memory, and — deliberately — the candidate's own claim if that claim is wrong.

---

## 6. AT-I32 — EXTENDED (compile probes; each must fail to compile)

- storing a `LeastDueSlice<'_>` in any struct, ledger, or store that outlives its borrow;
- mutating a `LeastDueSlice<'_>` or reading a payload or conflict evidence from it;
- any scheduler method removing work by `WorkKey`, key list, range, predicate, count,
  coordinate, or `due_time` alone;
- any batch or multi-item `schedule`, reschedule, reinsert, or cancellation path;
- constructing an evaluation context from a horizon, or reading the horizon from inside
  rule/threshold/obligation/cooldown evaluation;
- passing `DueSliceSummary` or `PacingDiagnostics` into rule evaluation;
- reaching X-1 … X-4 from a host, service, or protocol surface.

**Deliberately absent, and stated as such:** there is **no** probe asserting that the
slice fingerprint cannot be constructed or retained. It is an ordinary owned value and
this pass makes no unforgeability claim about it (§5.5, §5.6). The rejected matrix's
probes for a `DueCohortSelection` constructor are withdrawn with the type.

**Kills:** any reintroduction of an owned removal capability, any arbitrary-removal
surface, and any horizon leak into evaluation.

---

## 7. AT-I35 — EXTENDED

Phase-1 stability pins: `Scheduler::schedule` and `Scheduler::drain_due` retain their
signatures and behavior (a `drain_due` call on any fixture returns the byte-identical
`DrainOutcome` and leaves the byte-identical `canonical_state_digest()` it does at Phase-1
closure); `WorkKey::identity_digest`, `Scheduler::canonical_state_digest`, `StateCell`,
manifests, configs, and timeline encodings remain bit-identical; the inherited scheduler
corpus runs unedited and green.

**Additional pin for this pass.** The slice fingerprint's per-slot block is asserted equal,
byte for byte, to the block `canonical_state_digest` builds for the same slot — computed
by an independent reference implementation in the test, over scheduled slots, conflicted
slots at and beyond `MAX_CONFLICT_EVIDENCE`, and conflicted slots at and beyond
`MAX_CONFLICT_TRACKED_CLAIMS` including the truncation flag.

**Kills:** a fingerprint that hashes only key identities (which would reopen review §5.3);
a fingerprint over the exposed evidence projection rather than the tracked claim set
(which would let two differently contested keys compare equal); any new encoding invented
for the fingerprint.

---

## 8. AT-I42 — NEW (V2 definition)

**`least_due_compare_and_take_is_atomic_and_unprivileged`**

- **(a) Atomic single-slice removal.** With slices `(100,P)`, `(101,P)`, `(101,Q)`:
  extraction removes exactly `(100,P)`; the residual `canonical_state_digest()` equals an
  independently built reference scheduler's; every remaining key's `slot_status`, payload,
  and `conflict_of` are unchanged. **Kills:** whole-prefix removal; range removal by
  `due_time`.
- **(b) Mixed slice.** A slice with scheduled and conflicted slots extracts both in one
  step; each returned `WorkKeyConflict` is value-identical to `drain_due`'s for the same
  history, including retained hashes, `omitted_distinct`, and `evidence_truncated`; a
  conflicted slot in a *later* slice is untouched. **Kills:** an implementation that
  leaves conflicted slots resident, detected at the next cohort's pre-wave digest.
- **(c) Non-least selection is refused.** A genuine fingerprint of a **non-least** slice
  (obtained through the `test-support` seam) is passed: the call returns
  `SliceChanged { observed }` where `observed` is the **live least** slice's fingerprint,
  and the scheduler is byte-identical. **Kills:** any API accepting a coordinate or token
  that names its own target — precisely the rejected candidate's C-2/C-3, which would
  succeed here and remove a non-least cohort (review §5.2).
- **(d) Replay is possible and unprivileged.** Extract `(100,P)` retaining its
  fingerprint. Three sub-cases, all asserted:
  1. re-create the identical `WorkKey` with an **identical** payload so the slice is again
     least and byte-identical, then replay: the take **succeeds**, and its effect is
     asserted equal to that of a take driven by a fresh `least_due_slice` observation in
     that same state — no privilege;
  2. re-create the identical `WorkKey` with a **different** payload: the slot is
     `Conflicted`, the fingerprint differs, and the replay is a typed byte-identical
     no-op;
  3. re-create a **different** key so some other slice is least: typed byte-identical
     no-op.
  **Kills:** the rejected candidate's claim that privacy prevents replay — sub-case 1
  would be reported there as impossible; and any implementation binding only key identity,
  which would let sub-case 2 remove newly scheduled work (review §5.2).
- **(e) Same-key state mutation between observation and extraction.** Observe the slice
  while key `K` is `Scheduled(A)`; end the borrow; call the public `schedule(K, B)` with
  `B ≠ A`, turning the slot `Conflicted`; then take. Asserted: typed `SliceChanged`, and
  the scheduler byte-identical — count and key set are unchanged, so only a
  state-and-content-bound comparison can detect it. **Kills:** key-only or count-only
  comparison (review §5.3).
- **(f) Meaningful, constructible not-due failure.** With the least resident due time
  `100`: a call at horizon `99` returns `NoSliceDue { least_resident_due_time: Some(100) }`
  and mutates nothing; on an empty scheduler it returns
  `NoSliceDue { least_resident_due_time: None }`. **Kills:** the rejected `CohortNotDue`,
  which had no `now` and no live-least comparison and was unconstructible through any
  conforming path (review §5.4).
- **(g) Every failure is a byte-identical no-op.** For each error, the
  `canonical_state_digest()`, occupied key set, every payload, and every conflict value
  equal the pre-call state — asserted as digest equality, never as an
  error-was-returned check. **Kills:** partial removal followed by an error.
- **(h) Scan-before-mutation.** A fixture in which the fingerprint mismatches on the
  *last* key of a large slice asserts that no earlier key was removed. **Kills:** a
  remove-then-validate implementation relying on a rollback that does not exist.
- **(i) Phase-1 refinement (Lemma D scope).** Over randomized static scheduler states and
  horizons, running the loop to exhaustion against a null evaluator yields `due` and
  `conflicted` concatenations equal in order to `drain_due(T)`'s, value-identical
  conflicts, and an identical final digest. Scope is explicitly recorded as static /
  null-evaluator; it is not cited for any work-producing claim. **Kills:** any per-slot
  disposition drift between the two paths.
- **(j) Determinism and permutation invariance.** Two schedulers with equal canonical
  digests built by permuted `schedule` orders yield equal handles, equal fingerprints,
  equal extractions, and equal residual digests for equal horizons.
- **(k) Totality and panic freedom.** Empty scheduler; horizon below and above every due
  time; single-slot slice; slices at `max_due_per_cycle` and `+1`; maximal slice sizes; a
  conflicted key at `MAX_CONFLICT_TRACKED_CLAIMS`; every error path. No panic; every
  failure typed.

---

## 9. AT-I43 — NEW

**`command_barriers_are_placed_and_time_guarded`**

- **(a) A command barrier drains no scheduled work.** With scheduled work resident at
  `due_time ≤ e`, executing a command at `effective_time = e` leaves every scheduled slot
  resident and byte-identical; only the following catch-up call consumes it.
  **Kills:** an implementation that attaches an implicit drain to the command barrier.
- **(b) Command placement is history, not a partition.** Two histories — *(one call to
  `T`, then command at `e`)* and *(stepwise calls to `T`, then command at `e`)* — are
  asserted **equal** under PE-A/PE-B/PE-C. A third history placing the command *between*
  the stepwise calls is asserted **deterministic and distinct**, and is explicitly not
  compared for equality. **Kills:** an oracle that treats command placement as a pacing
  partition and demands equality, and an implementation that reorders a command against
  scheduled cohorts by comparing `effective_time` with due times.
- **(c) Equal canonical times are ordered by explicit history position.** A command at `e`
  and a scheduled cohort at `due_time = e`, in both history orders, produce two
  deterministic and **distinct** canonical results; neither order is reachable implicitly.
  **Kills:** any implicit same-time ordering rule, which blueprint §28 item 2 and the
  Phase-0 "ambiguous same-time order rejected" correction forbid.
- **(d) Cohort-local evaluation time reaches the rule layer.** A decay/recovery rule and a
  cooldown check in a cohort at `due_time = τ` compute against `now = τ`, asserted with
  pinned closed-form vectors, across the one-call and stepwise partitions of AT-I39(A).
  **Kills:** an implementation passing the horizon `T` or the clock frontier as `now`,
  which changes decay outputs under partition (i) while leaving (ii) correct — a
  divergence no digest-only assertion elsewhere would localize.
- **(e) SH-1 refusal.** A finalized command with `effective_time < Φ` is refused: typed,
  atomic, nothing mutated, reported, no wave, no batch, no created work; identical on
  replay. **Kills:** an implementation that evaluates it, which is detected by the
  companion assertion that a decay operation in that command would otherwise compute a
  negative elapsed span and extrapolate backwards.
- **(f) SH-2 refusal.** A command at `e` executed while a scheduled slot with
  `due_time < e` is resident (reached by deferring work with a small budget and then
  commanding) is refused identically. **Kills:** an implementation that assumes the host
  drained first; the deferral makes the violation reachable without any malformed input.
- **(g) Conforming histories are unaffected.** A sweep of histories satisfying SH-1/SH-2
  produces no refusal, so the guard cannot silently suppress legitimate work.
- **(h) Defense-in-depth enqueue check.** A wave that would create work with
  `due_time ≤ Φ` rejects atomically; under SH-1/SH-2 the test must construct it through
  the `test-support` seam, and the test records that it is unreachable by conforming
  input. **Kills:** an implementation permitting scheduling into already-traversed time.
- **(i) Checked delayed-time arithmetic.** `due_time = canonical_time + delay` at
  `u64::MAX` rejects the wave atomically with a typed error and no mutation.
  **Kills:** wrapping or saturating arithmetic.

---

## 10. AT-I44 — NEW

**`every_rejection_class_is_terminal_and_cleans_its_stores`**

For **each** of the eight classes R-1 … R-8 of §8, one fixture, and for each:

- **(a) Post-state.** The extracted `WorkKey`s are absent from the scheduler; their
  `ObligationStore` records are absent; no cell, occurrence, cooldown, or enqueue of the
  rejected wave is applied; the engine digest equals the post-extraction digest exactly.
- **(b) Bidirectional invariant.** The v1 §8 scheduler↔`ObligationStore` invariant holds at
  the post-state and at every observable point, including immediately after the atomic
  extraction and after the rejection.
- **(c) Report.** The canonical semantic report carries the typed outcome with
  order-independent evidence; for R-5 it is the v2 §6.3 overload report in
  conflicted-drain shape.
- **(d) Retry under both identities.** The producer reschedules (i) under the **next**
  occurrence index and (ii) under the **same** `WorkKey`; both are accepted as new
  canonical input, produce fresh obligation records, and evaluate under ordinary rules.
  Asserted explicitly that the same-key retry is permitted, so the record's "typically
  under the next occurrence index" is not silently hardened into a restriction.
- **(e) Replay.** The same history replays to byte-identical results, caps being
  epoch-bound.
- **(f) No resurrection.** No route reinstates a scheduler slot or an obligation record for
  a consumed key; progress (v3 §3.3) is asserted by the strict decrease of cohorts ahead of
  a fixed later cohort.

**Kills:** an implementation that reinserts rejected work (livelock, and a partition-
dependent cohort sequence); one that removes obligation records at commit rather than at
extraction, which leaves orphan records after every rejection and fails (b); one that
attempts a rollback of the extraction, which fails (a); and the rejected candidate's CE-9,
which stated the post-extraction snapshot for wave rejection only and left R-5 … R-8
unspecified (review Q9).

---

## 11. AT-I45 — NEW

**`pacing_counts_executable_work_only`**

- **(a) The review §5.5 admission boundary.** With `max_due_per_cycle = 1`, slice
  `(100,P) = {S scheduled, X conflicted}` and slice `(101,P) = {T scheduled}`: the
  executable count of `(100,P)` is **1**, it admits normally, the oversized exception does
  **not** fire, remaining budget is **not** forced to zero, and `T` is **not** deferred.
  The full canonical result equals the inherited v1 Q4 budget-over-`DrainOutcome::due`
  computation. **Kills:** the rejected candidate's D-4, under which the count is 2, the
  exception fires, and `T` is deferred — an observable latency and diagnostic change that
  D-4 called "unchanged".
- **(b) All-conflicted slice.** A slice with only conflicted slots is extracted and
  reported, emits **no** cohort identity, **no** wave, and **no** `effect_batch_digest`,
  consumes **no** budget, does not increment the admitted-cohort count, and does not stop
  the admission loop; the next slice is then admitted in the same call with the full
  remaining budget. **Kills:** the rejected AT-I42(c), which required it to consume budget
  and report a cohort identity; and an implementation that stops the loop on it, which
  would defer executable work behind a slice that costs nothing.
- **(c) Oversized exception premise.** An all-conflicted slice preceding an oversized
  executable slice does not disturb the exception: the exception still fires on the
  oversized slice, admits it whole, sets remaining budget to zero, and admits nothing
  further that call. **Kills:** an implementation counting the conflicted slice as an
  admitted cohort, which would suppress the exception and starve the oversized cohort —
  reopening B01.
- **(d) Pacing remains semantics-neutral.** Over the (a)–(c) fixtures, paced and
  unbudgeted runs are compared per cohort-sequence position and, under PE-C, at segment
  end. Only `PacingDiagnostics` differ, and the diagnostics' `deferred_cohort_count` and
  `earliest_deferred_due_time` are asserted against an independently computed expectation.
  **Kills:** any budget rule that changes canonical results.

---

## 12. AT-I46 — NEW

**`pre_wave_observation_is_test_only`**

- **(a) No canonical report field.** The canonical semantic report's field set is pinned
  and asserted **equal to the v3 field set** — no `pre_wave_engine_digest`,
  `pre_wave_scheduler_digest`, or `cohort_identity` field is added.
  **Kills:** the rejected CE-8/D-6, which added all three (review F-06).
- **(b) The seam is compile-gated.** The existing
  `crates/spark-testkit/tests/workspace_dependency_direction.rs` hygiene test is asserted
  to still pass with the new observation seam present, proving no production dependency
  edge enables `test-support`. A build without the feature is asserted not to expose the
  observation points.
- **(c) The seam observes what AT-I39 needs.** With the feature enabled, the pre-wave
  engine and scheduler digests are observable at the post-extraction, pre-evaluation point
  for every cohort, which is what makes AT-I39(A)(B) and AT-I45(d) expressible.
- **(d) Causal inertness.** A compile probe asserts the observation values cannot reach
  rule evaluation.

**Kills:** any implementation that satisfies the oracle by widening a production surface,
and any oracle that can only observe the digest through a host-visible report.

---

## 13. Traceability

### 13.1 To the independent review's §7 required additions

| Review §7 item | Test |
|---|---|
| 1. one-call versus stepwise with an early cohort scheduling inside the final horizon | AT-I39(A) |
| 2. explicit canonical-barrier and command-barrier placements | AT-I39(A)(C), AT-I43(a)(b)(c) |
| 3. attempt to extract a non-least due cohort | AT-I42(c) |
| 4. replay after successful extraction and exact-key re-creation | AT-I42(d) |
| 5. same-key scheduled→conflicted between observation and extraction | AT-I42(e) |
| 6. constructible, meaningful not-due comparison against explicit `now` | AT-I42(f) |
| 7. post-extraction scheduler and `ObligationStore` state for every rejection class, with retry under same and next occurrence identities | AT-I44(a)(b)(d) |
| 8. inherited versus proposed pacing counts for scheduled, mixed, all-conflicted slices | AT-I45(a)(b)(c) |
| 9. structural proof that no view or selection/continuation capability is retainable at a stable boundary | AT-I29(a)(b)(c), AT-I32 |

### 13.2 To the independent review's §5 counterexamples

| Counterexample | Test |
|---|---|
| §5.1 delayed enqueue breaks the cohort sequence | AT-I39(A) |
| §5.2 token replay removes newly created work | AT-I42(d) |
| §5.2 non-least token use | AT-I42(c) |
| §5.3 key-only check misses a slot-state transition | AT-I42(e), AT-I35 |
| §5.4 `CohortNotDue` has no information source | AT-I42(f) |
| §5.5 conflicted-slot budget changes admission boundaries | AT-I45(a) |
| §5.6 borrow shape (failed attack; no obstacle) | AT-I29(b), AT-I32 |
| §5.7 static drain refinement (sound, narrow scope) | AT-I42(i) |

### 13.3 To the correction candidate's findings and edits

| Item | Test |
|---|---|
| F-01 time basis; work-producing partition equivalence | AT-I39(A)(B)(D), AT-I43(d) |
| F-02 no owned removal capability | AT-I42(c)(d), AT-I32 |
| F-03 explicit horizon; meaningful not-due | AT-I42(f), AT-I32 |
| F-04 executable-only identity and budget | AT-I40(e), AT-I45 |
| F-05 oracle completeness | this document's Kills lines throughout |
| F-06 no canonical report field | AT-I46 |
| F-07 borrow wording | AT-I29(b), AT-I32 |
| CE-7′ consumption through X-1 … X-3 | §2 row, AT-I42 |
| CE-8′ executable-only budget and identity | AT-I40(e), AT-I45 |
| CE-9′ terminal consumption and store cleanup | AT-I44 |
| CE-10′ command placement and SH-1/SH-2 | AT-I43 |
| CE-11′ test-only observation | AT-I46 |
| Candidate §14 item 5 (`Φ` reconstructible) | AT-I29(d) |

---

## 14. Out of scope (unchanged)

No persistence, service, or protocol tests; no profile parsing or IO; no executable
Windows/Android runs; no Phase-4+ semantics; no G.A.M.E. integration; no runtime-LLM
surface. **Phase-2 implementation is not authorized by this candidate; Phase 3 is not
authorized and is not pre-authorized here.** This is a proposed oracle awaiting
independent review, not an accepted one, and no test in it has been executed.
