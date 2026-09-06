# S.P.A.R.K. Phase 2 — Acceptance Test Oracle, V3-F01 Correction Candidate (AT-I)

**Date:** 2026-09-06
**Agent:** Claude Code (Opus), Gate C1 architecture writer
**Status:** **CANDIDATE — AWAITING INDEPENDENT REVIEW.** Not accepted, not frozen, not
canonical, and authorizing no implementation.
**Companion to:**
`SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_2026-09-06.md` (unqualified
"§" references are to that document; "v3 §…" to
`SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md`; "v2 §…" / "v1 §…" to the
earlier freezes).
**Amends:** `SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v3_2026-08-26.md` (retained unmodified
as history) in exactly the entries named in §1. Every other AT-I entry in that matrix
stands as written and unweakened.
**Controlling finding addressed:**
`SPARK_PHASE_2_CODEX_FINAL_ARCHITECTURE_V3_CONFIRMATION_2026-08-26.md` §7.4(3)(4).

**Discipline (unchanged from v1/v2/v3):** every AT-I test is encoded red-first; baselines
recorded verbatim under `engineering/phase2/<pass>-baseline/` before any production
change ("inexpressible against inherited tree" is recorded, never skipped). Every "X is
impossible" claim is a digest/value equality or a compile-fail, never an
error-was-returned check alone. No existing test is weakened, deleted, or skipped; the
Phase-1 suite (232 tests) runs unmodified throughout.

---

## 1. Exact delta over acceptance matrix v3

| Matrix v3 entry | Disposition |
|---|---|
| "Explicitly unchanged tests" scheduler row — *"`Scheduler` consumed, not modified; no batch API"* | **REPLACED** by §2. |
| **AT-I39** `causal_identity_is_independent_of_drain_partition` | **STRENGTHENED** by §3. |
| **AT-I40** `cohort_is_scoped_to_profile_and_due_time` | **STRENGTHENED** by §4. |
| **AT-I29** `wave_buffers_do_not_outlive_the_wave` | **EXTENDED** by §5. |
| **AT-I32** strict lint / compile-probe gate | **EXTENDED** by §6. |
| **AT-I35** Phase-1 digest stability | **EXTENDED** by §7. |
| **AT-I42** `cohort_extraction_surface_is_atomic_and_bounded` | **NEW** — §8. |
| Traceability table | **EXTENDED** by §9. |
| Every other AT-I entry (AT-I1…AT-I28, AT-I30…AT-I34, AT-I36…AT-I38, AT-I41) | **UNCHANGED.** |
| "Out of scope" section | **UNCHANGED.** |

---

## 2. Replacement for the "Scheduler consumed, not modified" conformance row

> **Scheduler suite (AT-B, AT-G scheduler half).** `Scheduler::schedule`,
> `Scheduler::drain_due`, `Scheduler::canonical_state_digest`, `WorkKey`, `WorkPayload`,
> `DueWorkItem`, `DrainOutcome`, `ScheduleDisposition`, `WorkKeyConflict`, and
> `WorkSlotStatus` are consumed **unmodified in signature and semantics**; the complete
> inherited scheduler corpus runs green and unedited. Phase 2 adds **exactly one**
> additive consumption surface — the cohort-granular due-work view, its unforgeable
> selection token, and its atomic single-cohort extraction (§5 C-1…C-3). **No
> batch-scheduling API lands** (v1 Q6 conclusion and the AT-B7 seam contract at
> `scheduler.rs:424–431` stand unweakened), and **no arbitrary-removal API lands**: there
> is no removal by `WorkKey`, key list, range, predicate, count, or `due_time` alone, and
> no cancellation path. Conformance to this row is proved by **AT-I42**, not asserted.

---

## 3. AT-I39 — STRENGTHENED

**`causal_identity_is_independent_of_drain_partition`** (REVISED — V3-F01)

**Fixture precondition (new and mandatory).** **Every cohort the test compares is
pre-scheduled before either run begins.** No cohort is added mid-run. The fixture
contains, at minimum: at least ten distinct due times for one profile; at least one due
time carrying cohorts for two profiles `P1` and `P2`; at least one cohort containing both
scheduled and conflicted slots; and at least one cohort whose commit enqueues strictly
later work (`due_time ≥ now + 1`, v2 §11).

**Partitions compared** (as matrix v3, retained): (i) one drain call; (ii) a budget
forcing two drain calls; (iii) a budget forcing one drain call per cohort; (iv) catch-up —
ten due times in a single drain versus one due time evaluated ten times (R-038);
(v) a split-batch variant (R-044); and **(vi) NEW — a pacing cut between profiles at one
due time**: a budget that admits `(t, P1)` and defers `(t, P2)` to the next cycle, versus
an unbudgeted run that processes both in one cycle.

**Per-cohort assertions (new).** For each cohort `C`, at the instant defined by §6.1 S3 —
after `C`'s extraction and before any evaluation of `C` — the following are
**byte-identical across every partition (i)–(vi)**:

- **(a) scheduler digest.** `pre_wave_scheduler_digest` for `C` (CE-8). Explicitly pinned
  by hand for the Codex §7.1 counterexample: with `C1@100` and `C2@101` for one profile,
  `C1`'s value equals `canonical_state_digest({C2})` and `C2`'s equals
  `canonical_state_digest(∅)` in **both** the `drain(101)`-shaped catch-up run and the
  per-due-time run. An implementation that removes the whole due prefix fails here, and
  this sub-case alone is the direct falsification of V3-F01.
- **(b) full pre-wave engine digest.** `pre_wave_engine_digest` for `C` (CE-8) — the
  complete v1 §8 composition, not only its scheduler component.
- **(c) cohort identity.** `scheduled_cohort_identity` (v3 §4.1), including cohorts that
  contain conflicted slots (§10 D-3).
- **(d) effect-batch identity and digest.** Every wave's `effect_batch_digest` for `C`
  (v3 §4.5), recomputed independently from its declared components and compared for
  equality — not merely compared as opaque values.
- **(e) emission identities**, wave-0 and later-wave, and every parent-set digest.
- **(f) obligation record hashes, occurrence mappings, cooldown writes, canonical
  semantic reports**, and the final engine digest.

**Residency assertions (new).**

- **(g) Deferred-slot byte identity and continued residency.** At every cohort's S3, every
  cohort later in the global ascending `(due_time, profile_id)` order — deferred by
  pacing, later at this boundary, or not yet due — is still **present** in the scheduler
  (`slot_status` is `Scheduled` or `Conflicted`, never `Empty`) and **byte-identical**:
  asserted by pinning `canonical_state_digest()` of a reference scheduler holding exactly
  the expected residual slot set, and by per-key `slot_status` and `conflict_of`
  equality. A later cohort's payload, conflict evidence, omitted count, and truncation
  flag are all unchanged.
- **(h) Exactly-one-cohort removal.** Between consecutive S3 captures, the symmetric
  difference of the scheduler's occupied key set is **exactly** the extracted cohort's
  slice, plus only enqueues committed by that cohort's own waves.

**Retained assertions (carried from matrix v3, unweakened).** Only `PacingDiagnostics`
differ between partitions. A regression fixture pins the retired v1 heartbeat descriptor
`H("heartbeat" ‖ logical time ‖ digest of drained WorkKeys)` as absent from every
canonical digest, computed independently and asserted to have no influence.

**Failure the test must produce against the inherited surface.** Encoded red-first
against a `drain_due`-based consumption path, assertions (a), (b), and (d) fail for every
cohort that is not the last of its boundary. That recorded red baseline is the required
evidence that the oracle actually discriminates.

---

## 4. AT-I40 — STRENGTHENED

**`cohort_is_scoped_to_profile_and_due_time`** (REVISED — V3-F01)

As matrix v3 (two profiles at one `due_time` form two cohorts with distinct identities,
evaluated in ascending `(due_time, profile_id)` order; the partition is contiguous in the
frozen `WorkKey` total order; no co-target causal group is split — compile-probe plus
runtime assertion that a cause in profile `P` cannot target a cell in profile `Q`; each
cohort's identity depends only on its own members; adding a third profile perturbs
neither of the first two cohorts' identities, batch digests, or emission identities),
**with all cohorts pre-scheduled before every run compared**, strengthened by:

- **(a) Pacing cut between profiles at one due time.** With `(t, P1)` and `(t, P2)` both
  due and a budget admitting only `(t, P1)`: at `(t, P1)`'s S3, `(t, P2)` is still
  resident and byte-identical (§3(g)); `(t, P1)`'s `pre_wave_scheduler_digest`,
  `pre_wave_engine_digest`, cohort identity, and every `effect_batch_digest` are
  byte-identical to the unbudgeted run in which both cohorts are processed in one cycle;
  and `(t, P2)`'s values are likewise byte-identical across the two runs.
- **(b) No cross-profile leakage at the extraction boundary.** Extracting `(t, P1)`
  removes no slot of `(t, P2)`, of `(t', P1)` for any `t' ≠ t`, or of any other profile —
  asserted as a key-set equality against a hand-enumerated expected residual set, not as
  a count.
- **(c) Ascending consumption under every partition.** The observed cohort-consumption
  order equals the global ascending `(due_time, profile_id)` order in partitions (i)–(vi)
  of AT-I39, including when a boundary's cohorts are split across drain calls
  (Lemma A).
- **(d) Cohort membership includes conflicted slots.** A cohort containing both scheduled
  and conflicted slots has `work_key_count` equal to the whole slice and a
  `scheduled_cohort_identity` computed over the whole ascending `WorkKey` set (§10 D-3);
  removing the conflicted key from the fixture changes that identity — asserted as an
  explicit inequality.

---

## 5. AT-I29 — EXTENDED

**`wave_buffers_do_not_outlive_the_wave`** (REVISED — V3-F01)

As matrix v3 (transient candidate multiset, emission-identity canonicalization state,
reduction groups, preflighted transition, parent sets; reconstruction across a paced
deferral; boundary discipline with mid-cohort points asserted unreachable), extended
with:

- **(a) Absence of a transient due-work plan at every stable boundary.** At **every**
  stable boundary — between drain calls, and after each cohort commits or rejects — a
  fresh engine reconstructed from committed state alone reproduces the entire remaining
  cohort sequence: identical cohort identities, `pre_wave_scheduler_digest`s,
  `pre_wave_engine_digest`s, emission identities, `effect_batch_digest`s, obligation
  record hashes, occurrence mappings, canonical semantic reports, and final engine
  digest. **No view, selection token, extraction, cohort plan, tail buffer, remaining-
  budget carry, or continuation cursor survives a stable boundary.**
- **(b) Mid-cohort reconstruction at S3.** Reconstruction is additionally asserted at the
  S3 point of each cohort — after extraction, before evaluation — reproducing that
  cohort's evaluation exactly from committed state plus the extraction value alone.
- **(c) Structural non-retention of the view.** A compile-probe asserts the due-cohort
  view borrows the scheduler immutably, is not `'static`, and cannot be stored in any
  struct, ledger, or store that outlives the borrow (§5 C-1).

---

## 6. AT-I32 — EXTENDED (compile-probe set)

As matrix v3 (emission contexts, cohort identities, parent sets, and `PacingDiagnostics`
cannot be forged, mutated, or constructed outside the evaluator door; no parent-selection
accessor; the evaluator is not given a `PacingDiagnostics` reference), extended with
probes that **must fail to compile**:

- constructing a `DueCohortSelection` from outside — no public constructor, no public
  field, no literal, no `From`, no deserialization path;
- mutating a `DueCohortSelection` or a `DueCohortView`;
- storing a `DueCohortView` beyond its borrow;
- any scheduler method that removes work by `WorkKey`, by key list, by range, by
  predicate, by count, or by `due_time` alone;
- any batch or multi-item `schedule`, reschedule, reinsert, or cancellation path;
- reading a payload or conflict evidence from the view;
- passing a view, selection, extraction, or `PacingDiagnostics` into rule/threshold/
  obligation/cooldown/scheduler-decision evaluation as anything other than the cohort's
  own work input.

---

## 7. AT-I35 — EXTENDED (Phase-1 digest stability)

As matrix v3, extended with an explicit pin that the additive surface changes no Phase-1
contract:

- signature and behavior pins for `Scheduler::schedule` and `Scheduler::drain_due`
  (a `drain_due` call on any fixture returns the byte-identical `DrainOutcome` and leaves
  the byte-identical `canonical_state_digest()` it does at Phase-1 closure);
- `WorkKey::identity_digest`, `Scheduler::canonical_state_digest`, `StateCell`,
  manifests, configs, and timeline encodings remain bit-identical;
- the complete inherited scheduler corpus (AT-B, AT-G scheduler half) runs unedited and
  green.

---

## 8. AT-I42 — NEW

**`cohort_extraction_surface_is_atomic_and_bounded`** (NEW — V3-F01; §5, §6, §8, §9)

- **(a) Atomic removal of exactly one cohort — scheduled.** A boundary with cohorts
  `(100, P)`, `(101, P)`, `(101, Q)`: extracting `(100, P)` removes exactly that slice;
  the residual scheduler's `canonical_state_digest()` equals that of a reference scheduler
  built independently with only the remaining slots; every remaining key's `slot_status`,
  payload, and `conflict_of` are unchanged.
- **(b) Atomic removal of exactly one cohort — conflicted.** A cohort containing
  conflicted slots extracts them with the scheduled ones in one step; each returned
  `WorkKeyConflict` is value-identical to the one `drain_due` produces for the same
  history, including retained competing hashes, `omitted_distinct`, and
  `evidence_truncated`; a conflicted slot in a **later** cohort is untouched
  (digest-checked before and after).
- **(c) All-conflicted cohort.** A cohort whose every slot is conflicted is extracted,
  reports its cohort identity and complete order-independent conflict evidence, evaluates
  no wave, emits **no** `effect_batch_digest` (§10 D-5), consumes budget, and advances
  progress; the next cohort's `pre_wave_scheduler_digest` reflects the removal exactly.
- **(d) Typed failure paths are byte-identical no-ops.** For each of `CohortAbsent`,
  `CohortMembershipChanged`, and `CohortNotDue`: the typed error is returned **and** the
  scheduler's `canonical_state_digest()`, key set, every payload, and every conflict
  value are byte-identical to the pre-call state — asserted as a digest equality, never
  as an error-was-returned check alone. `CohortMembershipChanged` is exercised by
  scheduling an additional key into the slice between minting the token and calling the
  extraction.
- **(e) Digest-capture ordering.** The `pre_wave_scheduler_digest` recorded for a cohort
  equals the digest of the scheduler **after** that cohort's extraction and **before** any
  evaluation, and differs from the digest immediately before the extraction — asserted as
  an explicit inequality, so an implementation that captures the digest before removal
  fails.
- **(f) No transient plan and no continuation state.** Reconstruction at every stable
  boundary per AT-I29(a); additionally, forcing an engine restart between two drain cycles
  in the middle of a paced boundary reproduces every subsequent cohort identity, batch
  digest, and report byte-identically.
- **(g) Drain equivalence (Lemma D).** Over a corpus of randomized scheduler states and
  boundary times, running the cohort surface to exhaustion against a null evaluator
  produces: the concatenation of extracted `due` in cohort-sequence order equal to
  `drain_due(now).due` **in order**; likewise `conflicted`; value-identical
  `WorkKeyConflict`s; and an identical final `canonical_state_digest()`. This is the
  conformance proof that no Phase-1 semantic changed.
- **(h) Boundedness probes.** The AT-I32 compile-probe set of §6 is asserted from this
  test's fixture as well, so the prohibition travels with the surface rather than only
  with the lint gate.
- **(i) Determinism and permutation invariance.** Two schedulers with equal
  `canonical_state_digest()` built by permuted `schedule` call orders yield equal views,
  equal selection tokens, equal extraction values, and equal residual digests for the same
  `now`.
- **(j) Totality and panic freedom.** Adversarial sweep: empty scheduler; `now` below
  every due time; `now` above every due time; a single-slot cohort; a cohort at
  `max_due_per_cycle` and `max_due_per_cycle + 1`; maximal cohort `WorkKey` counts; a
  conflicted key at `MAX_CONFLICT_TRACKED_CLAIMS`; and every extraction error path. No
  panic anywhere; every failure typed.

---

## 9. Traceability additions

| V3-F01 requirement (Codex §7.4) | Test |
|---|---|
| Read-only canonical cohort view for the §3.2 selector | AT-I42(a)(i), AT-I29(c) |
| Atomic removal of exactly the admitted cohort immediately before its pre-wave digest | AT-I42(a)(b)(e), AT-I39(a)(h) |
| Later/deferred slots byte-identical and resident | AT-I39(g), AT-I40(a)(b), AT-I42(a)(b) |
| Scheduled and conflicted slots without a retained continuation buffer | AT-I42(b)(c)(f), AT-I29(a) |
| Every Phase-1 semantic preserved | AT-I42(g), AT-I35, AT-I34 |
| No new causal primitive or retained store | AT-I29(a)(c), AT-I42(f) |
| Cannot become a batch-scheduling or arbitrary-removal API | AT-I32 extension, AT-I42(h) |
| No ambiguity about failure atomicity or stable-boundary state | AT-I42(d), AT-I39(g), AT-I29(a)(b) |
| One-call catch-up vs one-due-time-per-call equality | AT-I39 partitions (i)/(iv) |
| Paced vs unbudgeted equality | AT-I39 partitions (ii)/(iii) vs (i); AT-I20c(h) unchanged |
| Pacing cut between profiles at one due time | AT-I39 partition (vi), AT-I40(a) |
| Scheduler-digest equality before every cohort batch | AT-I39(a) |
| Full pre-wave-engine-digest equality | AT-I39(b) |
| Effect-batch identity/digest equality | AT-I39(d) |
| Deferred-slot byte identity and residency | AT-I39(g) |
| No transient due-work plan at any stable boundary | AT-I29(a), AT-I42(f) |
| Atomicity for scheduled and conflicted cohorts and every typed failure | AT-I42(a)(b)(c)(d) |
| Conflicted slots in cohort identity and budget count (D-3, D-4) | AT-I40(d), AT-I42(b) |
| Report fields `pre_wave_*` (CE-8) | AT-I39(a)(b) |
| Post-extraction rejection snapshot (CE-9) | AT-I42(d), AT-I8/AT-I7b unchanged, read with CE-9 |

---

## 10. Out of scope (unchanged)

As v1/v2/v3: no persistence/service/protocol tests, no profile parsing/IO, no executable
Windows/Android runs, no Phase-4+ semantics, no MCI/G.A.M.E. integration, no runtime-LLM
surface. **Phase-2 implementation is not authorized by this candidate; Phase 3 is not
authorized and is not pre-authorized here.** This document is a proposed oracle awaiting
independent review, not an accepted one.
