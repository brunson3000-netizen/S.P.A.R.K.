# S.P.A.R.K. Phase 2 — Gate C2 Decay Architecture: Operator Adjudication

**Date:** 2026-09-11
**Decided by:** the Operator. **Recorded by:** the separated Gate C2 writer (Claude Code,
Opus 5, `claude-opus-5`). The recorder adds no semantics. Every rule in §2 is the
Operator's. §5 lists matters this record does **not** decide.
**Status:** immutable. This document is never edited after the commit that introduces it.
Any correction or later decision is recorded in a new document that cites this one.

## 1. What is being decided

| Item | Value |
|---|---|
| Controlling review | `engineering/phase2/SPARK_PHASE_2_CODEX_GATE_C2_DECAY_ORACLE_REVISION_INDEPENDENT_REVIEW_2026-09-11.md` |
| Review branch / commit | `review/phase2-gate-c2-decay-oracle-revision-independent-20260911` / `6b167d84f21fb60f374a2ab45c62d5c6a040790a` |
| Reviewed candidate | `6968a4af917c9a32761be371c3e12ec12ba0f1bd` (`candidate/phase2-gate-c2-decay-oracle-revision-20260911`) |
| Review verdict | `GATE_C2_DECAY_ORACLE_REVISION_BLOCKED_ON_OPERATOR_ARCHITECTURE_DECISION` |
| Question put to the Operator (review §7) | the cadence origin after non-decay writes, and the residual/segment policy at parameter-changing barriers, while keeping canonical-time commits and the corrected nonretroactivity and saturation behavior |

The review (finding **C2R2-01**) held that the candidate's parameter-segment grid (writer
representation **R-2′**) is a coherent policy but not the retained frozen v1 Q7 formula. It
demonstrated this with two public-door vectors: a fresh assignment at 9, then decay at 10;
and a shock at 31, then decay at 40. The review required an explicit Operator decision
before either reading could control. It did not require any particular choice.

## 2. The decision (verbatim substance, numbered for citation)

**Accept fixed-grid decay as the controlling Phase-2 architecture.** This explicitly
supersedes the conflicting Q7 and v2 duration language (§3).

- **D-1 Fixed grid.** Decay cadence follows a deterministic fixed grid.
- **D-2 Writes do not re-phase.** Ordinary non-decay writes do not restart or re-phase that
  grid.
- **D-3 Parameter-changing activation.** An activation that changes rate or cadence closes
  the old segment, discards its unfinished residual interval, and starts a new grid at the
  activation barrier.
- **D-4 Endpoint ownership.** A step ending exactly at activation belongs to the closing
  segment.
- **D-5 No pre-activation charge.** New parameters never charge time before their
  activation.
- **D-6 Canonical commits.** Every decay evaluation commits at the cohort's canonical time.
  This includes unmoved, saturated, zero-rate, and no-whole-step evaluations.
- **D-7 Retained corrections.** The previously corrected nonretroactivity and saturation
  behavior remains controlling.

Read together with the review's accepted findings, D-1 … D-7 are the rules below. None adds
a rule beyond the decision.

- The grid of one parameter segment has **origin** equal to the segment's activation
  barrier (genesis: logical time 0). Its whole steps end at `origin + k·cadence`,
  `k ≥ 1` (D-1, D-3).
- An evaluation at canonical time `now` of a cell last written at `updated_at` applies the
  grid steps ending in `(updated_at, now]`. Each step uses its own segment's rate, is
  linear, floor-exact in `i128`, and stops at the baseline. The unchanged v1 Q7 value
  function supplies these properties. The evaluation then commits `(value, now)` (D-1,
  D-2, D-6, D-7).
- The step ending exactly at a parameter-changing barrier is billed at the old rate. The
  interval after the old segment's last whole step, up to the barrier, is billed at no
  rate. The new segment's first step ends one new cadence after the barrier (D-3, D-4,
  D-5).
- An activation that leaves the resolved rate and cadence unchanged does not close a
  segment. D-3 is conditioned on a change (the review's accepted C2R-02 control:
  94, not 95).

## 3. Superseded and clarified frozen language

Historical records are **not rewritten**. The table identifies each affected passage at
the exact commit this record is based on. It states whether the passage is
**SUPERSEDED** (its conflicting meaning no longer controls), **CLARIFIED** (it still
controls, read as stated), or **RETAINED** (unchanged). Anything not listed keeps its
existing precedence.

| # | Source (path under `engineering/phase2/`, lines) | Text | Disposition |
|---|---|---|---|
| S-1 | `SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_2026-08-26.md` Q7, lines 261–264 | "closed-form function of (current value, baseline, rate, **whole elapsed cadence steps between `updated_at` and now**)" | **SUPERSEDED as to the step count only.** The step count is now the number of **fixed-grid steps ending in `(updated_at, now]`** of each parameter segment (§2). It is no longer whole cadence *durations* elapsed since `updated_at`. The rest of the bullet is **RETAINED**: a closed-form function of current value, baseline and rate; `i128`; the Q1 floor rule; clamping to declared bounds and the baseline; linear per step; curved profiles via the piecewise-curve operation. |
| S-2 | same, Q7 lines 265–267 | "Catch-up over N missed steps in one evaluation is therefore bit-identical to N separate evaluations, so batching/chunking cannot alter outcomes" | **CLARIFIED.** Still controlling, now derived from the fixed grid. It covers any partition of evaluations of one cell across an interval with no intervening non-decay write. Every partition yields the same value **and** commit time (D-6). "N missed steps" means N grid steps. |
| S-3 | same, Q7 lines 259–260 (rate/cadence as hot-tunable rule parameters) | "Rate/cadence: rule parameters, referencing `ConfigRevision` keys where hot-tunable" | **CLARIFIED** by D-3 … D-5. A hot-tuning activation that changes the resolved rate or cadence starts a new segment at its barrier. |
| S-4 | same, Q7 heading line 250 and Decision line 252–253 | "rule-layer operations over existing cell fields; **no encoding change**" | **RETAINED.** The grid origin derives from the committed, restore-validated activation lineage. No `StateCell` field or encoding is added. |
| S-5 | `SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v2_2026-08-26.md` §4.3, lines 199–201 | "the v1 §Q7 closed-form catch-up formula, floor rounding, and chunk-invariance are upheld and remain the frozen formula" | **SUPERSEDED to the extent** that it retains the S-1 elapsed-since-`updated_at` step count. "The v1 §Q7 formula" now means Q7 as amended by S-1 and S-2. Floor rounding and chunk invariance remain upheld. The one-decay-rule-per-target admission rule (line 199–200) is **RETAINED**. |
| S-6 | same, §4.3 lines 201–204 | coincident shock + decay in one wave is a cross-family rejection "unless the profile combines them in one rule body or separates them by a scheduled boundary" | **RETAINED**, with the declared results **CLARIFIED** by D-2. After a separate scheduled shock, the next grid step still ends on the unchanged grid (review vector: decay 30 → 70, shock 31 → 75, decay 40 → **65**). A rule body's non-decay stages likewise do not re-phase the grid. |
| S-7 | `SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_FINAL_2026-09-10.md` §4, lines 122–126 | "Every inherited use of `now`, `at`, and 'current logical time' during that cohort's evaluation denotes this value" | **RETAINED.** D-6 is its application to decay. |
| S-8 | `SPARK_PHASE_2_ACCEPTANCE_TEST_ORACLE_V3_F01_FINAL_2026-09-10.md` §1, line 54 | AT-I22 "UNCHANGED in substance, read under the clarified `now`" | **SUPERSEDED** as to AT-I22's substance, which is now the amended AT-I22′ (§4). The clarified `now` is **RETAINED**. |
| S-9 | same, §1, line 55 | AT-I23 (in the range "AT-I23 … AT-I28") "UNCHANGED" | **SUPERSEDED for AT-I23 only**, which is now AT-I23′ (§4). The other entries in that row are unaffected. |
| S-10 | `SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_2026-08-26.md` AT-I22, lines 139–142 | "N missed cadence steps applied as one catch-up evaluation vs N separate evaluations vs two uneven chunks: identical cell values …; convergence stops exactly at `baseline`; clamping … exact" | **CLARIFIED** into AT-I22′. "Cadence steps" are grid steps. "Identical cell values" includes the commit time. Baseline convergence and exact clamping are **RETAINED**. |
| S-11 | same, AT-I23, lines 143–145 | "a hot-tuned rate under a new behavior epoch applies only from that epoch's barrier; pre-barrier evaluations use the old rate" | **CLARIFIED** into AT-I23′ by D-3 … D-5. It is **RETAINED** in full as a nonretroactivity requirement. |
| S-12 | `SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v2_2026-08-26.md` AT-I22 (REVISED v2), lines 182–188 | exact formula/rounding vectors; coincident-shock rejection; "the two sanctioned compositions … produce their declared deterministic results" | **RETAINED**. The declared results are those of the fixed grid (S-6). |
| S-13 | same, AT-I23 line 189, and `SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v3_2026-08-26.md` AT-I22/AT-I23 lines 283–284 | "(UNCHANGED)" / "(UNCHANGED v2)" | **SUPERSEDED** by AT-I22′ and AT-I23′. |

The two review probes that encode the superseded S-1 reading are
`r2prime_frozen_q7_full_cadence_after_fresh_assignment`, expecting `(100, 10)`, and
`r2prime_frozen_q7_full_cadence_after_separate_shock`, expecting `(75, 40)`. Both are in
the review evidence's `independent_probes.rs`. They no longer state a controlling
requirement. The review's own observation probe
`r2prime_grid_behavior_is_observable_after_non_decay_write` pins `(90, 10)` and
`(65, 40)`. Under D-1 and D-2 those are the controlling values. The review evidence stays
byte-unchanged as a historical record.

## 4. Amended oracle entries

**AT-I22′ `decay_catch_up_is_chunk_invariant`** (supersedes AT-I22 v1/v2/v3 substance; keeps
the name):

(a) Chunk invariance across an interval with no non-decay write: any partition into
evaluations, one catch-up, or uneven chunks yields bit-identical `(value, commit time)`.
(b) Convergence stops exactly at the baseline, and clamping at declared bounds is exact.
The v2 formula/rounding vectors are retained.
(c) Coincident shock + decay in one wave is rejected atomically. The two sanctioned
compositions produce their fixed-grid results.
(d) The fixed-grid discriminators must be pinned by exact vectors:
  - a **fresh assignment** at 9, cadence 10, then decay at 10 → `(90, 10)`;
  - a separate **post-shock** decay at 40, after a shock at 31 → `(65, 40)`;
  - chunked evaluations after an unaligned write, both signs;
  - every unmoved evaluation (saturated, rate 0, no whole step) commits `(value, now)`.

**AT-I23′ `recovery_and_decay_respect_epoch_bound_rates`** (supersedes AT-I23 substance; keeps
the name). All vectors run in both directions (decay and recovery):

(a) No new rate or cadence charges time before its activation barrier.
(b) At a parameter-changing barrier, the step ending exactly at the barrier is billed at
the old rate (**activation-boundary** vector).
(c) The old segment's unfinished residual is billed at no rate (**residual-discard** vector).
(d) The new grid starts at the barrier (**shortening** and **lengthening** vectors).
(e) An activation leaving rate and cadence unchanged does not move the grid.
(f) Scheduled work due at the barrier time evaluates under the old epoch.
(g) Replay and snapshot/restore reproduce all of the above exactly.

## 5. Matters this record does not decide

The Operator's decision does not address the following. The writer's report states how the
existing implementation behaves for each and flags it for independent review. That
behavior is not an Operator ruling, and the report must not present it as one:

1. an activation that **removes** the operation, and a later activation that restores it;
2. grid steps that end **before** a non-decay write that no decay evaluation preceded. The
   write is not a decay evaluation, so such steps are never applied;
3. two activations at the **same** logical time;
4. a decay evaluation for a cell that does **not exist**.

## 6. Effect on the review's findings

C2R2-01 was a conformance objection: R-2′ did not match the retained Q7 formula. It is
resolved **by this decision**, not by any claim that R-2′ was already frozen-conformant.
The review's rejection of that claim stands as history. C2-05 and the AT-I22 portions of
C2-07 and C2R-04 can close only after an independent review of a candidate verified against
§2 and §4. Every other disposition in the controlling review, including the retained
limitations, is unchanged. This decision authorizes no production promotion and no Phase-3
work.
