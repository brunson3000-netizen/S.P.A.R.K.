# S.P.A.R.K. Gate C1 — V3-F01 Architecture-Writer Report

**Date:** 2026-09-06
**Agent:** Claude Code (Opus), Gate C1 bounded architecture writer
**Role boundary:** writer only. This pass did **not** review its own work, did **not**
run an independent adversarial panel, and claims **no** acceptance.
**Repository:** `/home/chromikey/Projects/SPARK`
**Branch:** `phase1-refoundation-v2`
**Starting HEAD:** `5fd556bfad958bda4439560cbfc5f4e537ce375a`
**Gate:** C1 (`engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` §5)

---

## 1. Mission and outcome

**Objective.** Close V3-F01 — the cohort-extraction / pre-wave-digest inconsistency —
sufficiently for independent Codex acceptance and Phase-2 architecture freeze.

**Outcome.** `V3_F01_CORRECTION_CANDIDATE_READY`. Two candidate artifacts and this report
exist. V3-F01 is **not** closed by this pass.

---

## 2. Files read

### 2.1 Governing and controlling records

| File | Purpose |
|---|---|
| `engineering/PHASE_STATUS.md` | current phase/authority state, durable operating constraints |
| `engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` | Gate C1 authority, convergence states, role assignment |
| `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md` | Phase-0 baseline (targeted: §17, §18.1, §19.3–§19.5, §28, §32) |
| `engineering/phase0/ADR-0003-deterministic-clock-rng-and-commit-order.md` | §14 command barrier, §15 logical time / catch-up / snapshots, §11 no cancellation |
| `engineering/phase0/PHASE_0_PERFORMANCE_AND_SECURITY_BUDGET_v0.2.md` | §3.1–§3.7 backpressure, deferral resumption, telemetry visibility |
| `engineering/phase1/SPARK_PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_2026-08-26.md` | Phase-1 closure, scheduler retained-evidence ruling, digest pins |
| `engineering/phase2/PHASE_2_FABLE_READINESS_BLUEPRINT_2026-08-26.md` | inherited seams, R1/R2 law, trust boundary, AT-B7 batch-API contract |
| `engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_2026-08-26.md` | v1 §8 engine digest, Q4 budgets, Q6 no-batch-API |
| `engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v2_2026-08-26.md` | v2 §6.1–§6.3 cohorts/pacing/over-cap, §11 delayed-work boundary, §12–§13 |
| `engineering/phase2/SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md` | the v3 baseline this candidate amends (read in full) |
| `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v3_2026-08-26.md` | the v3 oracle this candidate amends (read in full) |
| `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_2026-08-26.md`, `…_v2_…` | conformance-row lineage for the scheduler entry |
| `engineering/phase2/SPARK_PHASE_2_CODEX_FINAL_ARCHITECTURE_V3_CONFIRMATION_2026-08-26.md` | the controlling finding (read in full) |

### 2.2 Phase-1 implementation seams inspected

| Seam | Location | What was confirmed |
|---|---|---|
| `WorkKey` | `crates/spark-core/src/scheduler.rs:118–150` | field set and derived `Ord` = `(due_time, profile_id, producer_definition_id, scope_id, occurrence_index, work_kind)`; `identity_digest()` = `H("work_key_identity" ‖ canonical encoding)` |
| `Scheduler::schedule` | `scheduler.rs:442–472` | total; idempotent on equal payload; poisons on distinct payload; order-independent `ConflictEvidence` |
| `Scheduler::drain_due` | `scheduler.rs:474–500` | removes **every** slot with `due_time <= now`, walking the map front; partitions scheduled vs conflicted, each ascending; no panic path |
| `Scheduler::canonical_state_digest` | `scheduler.rs:545–565` | hashes slot count plus every occupied slot with `slot.scheduled` / `slot.conflicted` domain tags and the **complete** tracked claim set |
| Conflict machinery | `scheduler.rs:196–338` | `WorkKeyConflict` private fields; `BoundedClaimSet` at `MAX_CONFLICT_EVIDENCE = 16`, `MAX_CONFLICT_TRACKED_CLAIMS = 256` |
| AT-B7 batch-API seam | `scheduler.rs:424–431` | contract text preserved: any future batch API must be atomic and order-independent; not authorization |
| Engine digest composition | absent in tree | no aggregate `engine_state_digest` exists yet; the only `canonical_state_digest` implementations are `state.rs:324`, `timeline.rs:1712`, `scheduler.rs:545`. `engine_state_digest` is a Phase-2 architecture construct (v1 §8) with no implementation, confirming that no pinned digest value can move |

All line citations were re-verified against the working tree at the starting HEAD. One
precision correction was made relative to inherited v3 citations: v3 §1.1 cites
`scheduler.rs:474–490` for `drain_due`; the function's doc comment plus body actually
spans `474–500`. This is a partial-range imprecision pointing at the same function, not a
contradiction; the candidate uses the full range.

---

## 3. Defect reconciliation

The Codex V3-F01 finding is **confirmed as accurate, complete, and correctly scoped**.

**Chain of consequence, verified against source and text:**

1. `Scheduler::canonical_state_digest` is a pure function of the occupied slot set
   (`scheduler.rs:545`). Residency is therefore canonically observable.
2. v1 §8 composes the scheduler digest into `engine_state_digest`.
3. v3 §4.5 composes the full pre-wave engine digest into `effect_batch_v3`.
4. v3 §4.5 requires cohort `C`'s pre-wave engine digest to be "the committed state after
   every cohort strictly earlier in the cohort sequence"; v3 §3.2(d) requires later
   cohorts to remain "scheduled and untouched … not copied into any buffer".
5. `drain_due(now)` removes the whole `due_time <= now` prefix in one step, so under any
   catch-up or multi-cohort boundary the later cohorts are gone before the earlier
   cohort's capture.
6. Therefore `C`'s `effect_batch_digest` varies with drain partition, falsifying R-038
   catch-up equivalence, R-044 split-batch equivalence, v3 §3.4's paced/unbudgeted
   canonical-report equality, and AT-I20(h)/AT-I39 as written.

The same contradiction appears within one due time when the v3 §3.2 selector cuts between
profiles `(t, P1)` and `(t, P2)`.

**Codex §7.2's rejection of the four apparent workarounds is confirmed** and none is
adopted: buffering the tail contradicts §3.2(d) and makes the catch-up digest depend on
how much was due; drain-and-reinsert leaves reinsertion timing unspecified and is not
"untouched"; final-state convergence does not rescue per-cohort batch digests, which are
controlling semantics; and weakening only the oracle conceals the defect.

**No additional defect was found** in the V3-F01 area, and no inaccuracy was found in the
Codex finding. Three genuine *ambiguities* were found that the correction had to settle;
they are recorded as decisions D-3, D-4, and D-5 (§4 below).

---

## 4. Decisions made

**Form.** A **tightly bounded additive correction with an explicit supersession map**,
not a self-contained v4. Reason: v3 is 576 lines, of which the correction touches the
consumption mechanics of one selector, one derivation in §4.5, and three new consistency
edits. A v4 rewrite would put 576 lines of settled adjudication at risk of silent drift
for no gain, and the operating constraints forbid casually regenerating an equivalent
canonical artifact. The supersession map (candidate §11) names every v3 item's
disposition, so exactly one implementation contract results.

**Substantive decisions** (candidate §5–§10):

| # | Decision |
|---|---|
| C-1 | An additive **read-only `DueCohortView`** that *borrows* the scheduler immutably, enumerates due cohorts in ascending `(due_time, profile_id)`, and exposes only coordinates, key counts, ascending `WorkKey`s, and slot statuses — never payloads or conflict evidence. The borrow is the structural reason a due-work plan cannot be retained. |
| C-2 | An **unforgeable `DueCohortSelection`** token, mintable only from a live view entry, with private fields `(due_time, profile_id, work_key_count, cohort_key_digest)`. This is the guard that stops the surface becoming an arbitrary-removal API, in the same house style as `WorkKeyConflict`. |
| C-3 | `Scheduler::take_due_cohort(selection)` — **atomic removal of exactly one `(due_time, profile_id)` slice**, returning `DrainOutcome`-shaped `due`/`conflicted` vectors in ascending order, with three typed byte-identical-no-op failure paths (`CohortAbsent`, `CohortMembershipChanged`, `CohortNotDue`) and no panic path. |
| D-1 | The removal happens at **S2, immediately before the S3 pre-wave digest capture**, inside the cohort step. This is the correction. |
| D-2 | The view is **recomputed at every S0**, not computed once per cycle. This removes the last retained-state candidate; Lemma V proves it equivalent to a fixed plan, using v2 §11. |
| D-3 | **Conflicted slots belong to the cohort** and to its v3 §4.1 identity. Any other reading either strands conflicted keys in the scheduler (diverging from `drain_due` and polluting later pre-wave digests) or removes them outside any cohort — an unaccounted removal. |
| D-4 | The pacing budget counts **all** occupied slots in the slice, so one `work_key_count` serves budget, identity, and extraction. v1 Q4 phrased the budget over `DrainOutcome::due`; that phrasing predates cohort scoping. Pacing is semantics-neutral, so no equality claim moves. |
| D-5 | An **all-conflicted cohort** is extracted and reported, evaluates no wave, and emits no `effect_batch_digest`; it still consumes budget and advances progress. |
| D-6 | Every extracted cohort's canonical semantic report carries `cohort_identity`, `pre_wave_engine_digest`, and `pre_wave_scheduler_digest` (CE-8) so the required oracle has a conforming observation point at S3, including for D-5 cohorts. Report fields only: in no causal identity and no retained store. |
| D-7 | Names are normative for the matrix's convenience; **behavior** is what is proposed for freeze (Codex §7.4(1): "exact API name is not important"). |
| CE-9 | An atomic wave rejection restores canonical state to the **post-extraction** pre-wave snapshot; the rejected cohort's keys stay consumed. Stated because v3 left it implicit and "the pre-wave digest is preserved" became ambiguous once removal moved inside the cohort step. |

**Derivations supplied** rather than asserted: Lemma A (globally ascending cohort
consumption in every partition), Lemma V (view stability under recomputation), **Theorem
P** (partition independence of the pre-wave state, by induction — the closure of V3-F01),
Corollary on v3 §3.3 progress, and **Lemma D** (drain equivalence: the cohort surface is a
refinement of `drain_due`, not a new semantic).

**Deliberately not done:** no v4 rewrite; no change to `schedule`/`drain_due`; no batch or
cancellation API; no new causal primitive; no new retained store; no change to v3 §4.1–
§4.4 identity constructions; no reopening of any cleared v1/v2/v3 decision; no Rust.

---

## 5. Exact files changed

| File | Change |
|---|---|
| `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_2026-09-06.md` | **NEW** — the bounded architecture correction candidate |
| `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_2026-09-06.md` | **NEW** — the corrected acceptance-test-oracle candidate |
| `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_2026-09-06.md` | **NEW** — this report |
| `engineering/PHASE_STATUS.md` | **MODIFIED** — one additive subsection under V3-F01 recording that a candidate exists and awaits independent review. V3-F01 is **not** marked closed; no other line changed. |

Nothing else in the repository was created, modified, moved, renamed, or deleted. No
historical architecture or review artifact was touched.

---

## 6. Validation performed

| Check | Result |
|---|---|
| Branch | `phase1-refoundation-v2` — matches expected |
| Starting HEAD | `5fd556bfad958bda4439560cbfc5f4e537ce375a` — matches expected |
| Worktree at entry | clean; no unexpected tracked change |
| Exact changed-file inspection | 3 new files + 1 additive `PHASE_STATUS.md` subsection; verified by `git status --porcelain` and `git diff` |
| `git diff --check` | **PASS** (no whitespace errors, no conflict markers) |
| Reference validation | every `.md` path cited in both candidates resolves in the tree |
| Source-path validation | every `crates/…` path and every `scheduler.rs:<lines>` citation re-verified against the working tree; one inherited partial range corrected to `474–500` |
| Contradiction audit vs the Codex finding | no contradiction; every §7.4 requirement mapped to a candidate section and a test (candidate matrix §9) |
| No Rust production or test file changed | **CONFIRMED** — `git status` shows no `crates/**` entry; no `.rs` file was written |
| No Phase-1 semantic contract altered | **CONFIRMED** — the correction is purely additive; Lemma D and AT-I35/AT-I42(g) are its conformance obligations |
| Phase 1 not reopened | **CONFIRMED** |
| Expensive unrelated suites | **NOT RUN**, per mission scope (architecture-only) |
| Final worktree accounting | see §8 |

**Checks deliberately not performed and why:** `cargo fmt`, `cargo clippy`, and
`cargo test` were not run. No Rust changed, so they would only reconfirm the Phase-1
closure evidence already recorded, and the mission forbids expensive unrelated suites. The
inherited 232-test evidence stands as recorded in the Phase-1 closure lineage.

---

## 7. Residual risks and open findings

1. **D-3 / D-4 are the most consequential judgement calls.** Neither is stated in v3 nor
   prescribed by Codex §7.4. If the reviewer prefers executable-only cohort identity, the
   extraction contract must instead specify a separate, explicit, canonically ordered
   disposition for conflicted slots. This pass judges that strictly worse, but it is a
   legitimate alternative the reviewer may impose.
2. **Lemma V depends on a reading of v2 §11.** "Current logical time" during a boundary is
   read as the boundary's `now`, which makes every intra-boundary enqueue `> now` and the
   lemma immediate. Under the alternative reading (per-cohort `due_time`), D-2 must be
   revisited: a recomputed view would admit intra-boundary work and later cohorts'
   membership would become partition-sensitive. Called out rather than assumed.
3. **CE-8 adds a small reporting surface.** It is proposed only because the required
   oracle otherwise has no conforming observation point at S3, and because a D-5 cohort
   emits no batch digest to compare. A reviewer may prefer to derive the value instead of
   reporting it.
4. **Lemma D is an architectural assertion and an acceptance-test obligation, not an
   executed result.** No Rust exists to run it against.
5. **The candidate has had no adversarial review of any kind.** No NIM panel was run; the
   completed benchmark study was not rerun, per the durable operating constraint.
6. **The oracle's red-first baseline has not been recorded**, because no implementation
   pass is authorized. AT-I39's required failure against a `drain_due`-based path is
   specified but unproven.
7. **Unchanged pre-existing debts** carry forward untouched: Windows/Android are static
   compile checks only; executable cross-platform fixture replay and digest parity remain
   unproven.

---

## 8. Final worktree accounting

At the end of the pass, before the commit:

```text
tracked, modified:   engineering/PHASE_STATUS.md
untracked, new:      engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_2026-09-06.md
                     engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_2026-09-06.md
                     engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_2026-09-06.md
```

No other tracked file is modified, and no other untracked file is introduced. After the
single bounded commit the worktree is clean. A disposable identical copy of this report is
placed in `~/Downloads` as a transfer copy only; the repository copy is canonical.

---

## 9. Explicit nonclaims

This pass does **not** claim that:

- V3-F01 is closed, corrected-and-accepted, or resolved;
- Phase-2 architecture is frozen, canonical, or accepted;
- the candidate artifacts are canonical or supersede v3 in effect (they propose a
  supersession that only an independent reviewer and the operator can enact);
- the Phase-2 implementation writer is released;
- Phase 3 is authorized or pre-authorized;
- any Rust was written, changed, verified, or authorized;
- any test was executed, or any red baseline recorded;
- convergence state `ARCHITECTURE_READY` has been reached, or that G.A.M.E. may treat
  anything here as a production dependency;
- cross-platform runtime or digest parity exists;
- Phase 1 was reopened, or that any Phase-0/Phase-1 contract changed.

The controlling independent verdict remains `PHASE_2_ARCHITECTURE_V3_REVISE`.

---

## 10. Recommendation to the independent reviewer

**Recommendation: review the candidate for acceptance, adversarially, against the
falsification list below rather than against this report's own reasoning.**

The reviewer should treat the following as the load-bearing claims to attack:

1. **Theorem P** (candidate §7.2) — does the induction actually hold for *every* legal
   partition, including non-decreasing-`now` sequences that interleave command barriers
   with scheduled cohorts, and including a boundary whose earliest cohort is admitted by
   the §3.2(c) exception?
2. **Lemma V** (candidate §6.3) and residual risk 2 — is the v2 §11 reading correct?
3. **Lemma A** (candidate §7.1) — is globally ascending consumption really forced when a
   deferred cohort and a newly-due later cohort coexist?
4. **Lemma D** (candidate §7.4) — is the cohort surface genuinely a refinement of
   `drain_due` for every scheduler state, including all-conflicted prefixes and
   truncated-evidence keys?
5. **D-3 and D-4** — is including conflicted slots in cohort identity and in the budget
   count correct, or should the reviewer impose the alternative?
6. **Boundedness (candidate §9)** — can the proposed surface be bent into a batch-
   scheduling, arbitrary-removal, cancellation, or alternative-read path? The
   `DueCohortSelection` guard and the borrowing view are the specific defenses to attack.
7. **Oracle sufficiency (candidate matrix §3, §4, §8)** — does AT-I39(a) actually fail
   against a `drain_due`-based implementation, and do AT-I42(d)(e) actually discriminate a
   digest captured before removal?

**Artifacts the independent Codex reviewer must review:**

1. `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_2026-09-06.md`
2. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_2026-09-06.md`
3. `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_2026-09-06.md` (this report)
4. `engineering/PHASE_STATUS.md` (the additive candidate-pointer subsection only)

**Reconciled against, unmodified:**
`SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md`,
`SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v3_2026-08-26.md`,
`SPARK_PHASE_2_CODEX_FINAL_ARCHITECTURE_V3_CONFIRMATION_2026-08-26.md`, the v1/v2 Phase-2
artifacts, the Phase-0 blueprint/ADRs/budget, the Phase-1 closure lineage, and the
Phase-1 scheduler source seams listed in §2.2.

---

## 11. Verdict

`V3_F01_CORRECTION_CANDIDATE_READY`

- V3-F01: **OPEN** — a candidate correction exists and awaits independent review.
- Phase 1: **CLOSED**, not reopened.
- Phase 2: architecture-only; **not frozen**, **not accepted**; implementation writer
  **not released**.
- Phase 3: **NOT AUTHORIZED.**
- Production Rust changed: **none.**
