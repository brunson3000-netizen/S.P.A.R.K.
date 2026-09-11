# S.P.A.R.K. Gate C2 — Decay Adjudication Implementation Report

**Date:** 2026-09-11
**Writer:** Claude Code (Opus 5, `claude-opus-5`), the separated Gate C2 writer. The
independent reviewer is Codex and is not the writer.
**Mission:** `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_ADJUDICATION_WRITER_MISSION_2026-09-11.md`
**Decision implemented:** `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md`
**Branch:** `candidate/phase2-gate-c2-decay-adjudication-20260911`
**Verdict:** `GATE_C2_DECAY_ADJUDICATION_READY_FOR_INDEPENDENT_REVIEW`

Nothing is accepted or promoted. Production is unchanged. This pass used no Phase 3 work,
G.A.M.E. change, force-push, reset, discarded work or paid compute.

## 1. Summary

The Operator accepted fixed-grid decay as the controlling Phase-2 architecture. The
decision explicitly supersedes the conflicting v1 Q7 and v2 §4.3 duration language
(D-1 … D-7). That decision resolves the controlling review's blocker **C2R2-01**. It does
so by adjudication, not by the earlier writer's claim that the grid was already
frozen-conformant. The review rejected that claim, and the rejection stands as history.

1. **Record first.** Checkpoint 0 records the decision in a new immutable adjudication
   document, together with the mission, before any other change. The document identifies
   13 affected passages by path and line (S-1 … S-13), classes each as superseded,
   clarified or retained, amends AT-I22/AT-I23 into AT-I22′/AT-I23′, and lists four matters
   the decision does not address. No historical record was edited; this is mechanically
   checked.
2. **Audit: no behavioral correction was necessary.** The implementation already
   satisfies every rule (§3). The only source changes are comments. The `decay_walk`
   rationale previously called its representation "R-2′, flagged for review" and now
   cites the adjudication. A preservation check verifies that every changed line of
   every pre-existing crate file is a comment.
3. **Oracle and tests.** A new file, `phase2_decay_adjudication.rs`, adds 10 tests.
   Together they cover every vector class the mission names, each in both directions.
   Hand-computed values are cross-checked by a brute-force reference model.
   One test is a 40-timeline generated sweep with unaligned writes, which the earlier
   sweep lacked. Two earlier writer tests keep their values; their justifying comments now
   cite the decision instead of an unadjudicated representation argument (§5).
4. **Evidence.** 400 tests pass: 0 failed, 0 ignored. All 14 required checks pass.
   Ten compiled mutants are killed. Seven are the prior writer's mutants retargeted onto the
   new vectors. Three are the rejected residual and endpoint alternatives. The review's
   own four mutants remain killed, and 17/17 surface controls pass. The review's two
   `r2prime_frozen_q7_*` probes fail exactly at the superseded values; a copy with only
   those two literals changed passes 9/9.

## 2. Lineage and custody

| Item | Value |
|---|---|
| Worktree | `/home/chromikey/Projects/SPARK-gate-c2-adjudication` (fresh, isolated) |
| Base (direct parent of checkpoint 0) | `6b167d84f21fb60f374a2ab45c62d5c6a040790a`, the controlling review commit; verified local = tracking = live before branching |
| Reviewed candidate | `6968a4af917c9a32761be371c3e12ec12ba0f1bd` (ancestor) |
| Production | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` (ancestor; live `phase1-refoundation-v2`, unchanged) |
| Checkpoint 0 | `0c27ad89833d0c88162803ab41dec177424b49bb`: adjudication record and mission only, before any implementation step |
| Checkpoint 1 | `244c54b66a9c5bdfe8dd49e28b05cc0f5c255637`: comment-only source/test updates and the new vector file; this is the validated code-and-test tree |
| Checkpoint 2 | report, evidence and review mission. This is the final candidate commit, and its hash is in the receipt, since a commit cannot record its own hash |

Every pre-existing branch and worktree was left intact. `preservation.json` and
`custody.json` record them.

## 3. Audit of the implementation against the decision

| Rule | Implementation (checkpoint 1 source) | Verdict | Discriminating vector(s) |
|---|---|---|---|
| D-1 fixed grid | `decay_walk` counts grid points `origin + k·cadence` in `(from, end]` per epoch by checked quotient difference; origin comes from the activation lineage only | conforms | chunking over 64 partitions; generated sweep; every hand vector |
| D-2 no re-phasing by non-decay writes | the anchor is `updated_at`, but grid points do not depend on it. Single-op and rule-body decay share the walk | conforms | fresh assignment `(90, 10)` (Q7 duration reading: `(100, 10)`); reassignment 40 at 20 (duration: 50); scheduled and command post-shock `(65, 40)` (duration: 75) |
| D-3 changed activation closes, discards, restarts | `(Some(p), _) => Some((rate, cadence, begin))` when resolved `(rate, cadence)` differs; `end` of the closing epoch is the activation time, so the unfinished residual yields no point | conforms | residual `(14, 19]` discarded: 96 at 22, 91 at 23; carried-origin, old-rate-residual and absolute-grid mutants killed |
| D-3 (conditional) unchanged activation | the equal-parameter arm keeps the origin | conforms | 94 at 60; the reset-every-activation mutant is killed |
| D-4 endpoint ownership | the closing epoch's interval is `(from, activation]`; the new epoch counts from `begin` exclusive | conforms | points 7/14/21 old, 25 new: 89 at 25 via catch-up; the drop-endpoint mutant is killed |
| D-5 no pre-activation charge | a new segment's first point is `begin + cadence` | conforms | shortening at 53: 100 at 60/62, 95 at 63; lengthening at 55: 95 at 150, 90 at 155 |
| D-6 canonical commits | single-op decay of an existing cell always emits; `apply_wave` writes at `now` | conforms | `(2, 25)`, `(2, 26)` unmoved; `(0, 31)`, `(0, 40)` saturated; rate-0 segment `(70, 45)`, `(70, 60)`; no-step `(90, 19)` |
| D-7 retained corrections | the C2R-01/02/03 corrections are unchanged | conforms | all prior D/B/O/S tests pass unmodified in value; the prior 7 mutants are still killed by their prior tests |

The audit also checked these edge cases:
- scheduled work at an activation time evaluates under the old epoch;
- a command cohort after an activation at the same time evaluates under the new epoch, and
  finds no new point at the barrier;
- saturation clamps inside `move_toward`;
- restore validates the lineage the grid depends on.

None required a change.

## 4. Changed code

No executable line changed. The preservation checker verifies this: every added or removed
line of every modified pre-existing crate file is a `//` comment.

| File | Change |
|---|---|
| `crates/spark-engine/src/engine.rs` | comment-only: the `EpochArtifacts` rationale, the `decay_walk` rationale (now cites D-1 … D-7 and S-1/S-4/S-5, and flags the §5.1 operation-removal behavior as undecided), the canonical-commit comment (D-6), the rule-body comment (D-2), and the barrier-scoped config comment |
| `crates/spark-engine/src/rules.rs` | comment-only: `Update::Decay` doc ("per whole step of the operation's fixed `cadence` grid") |

## 5. Tests and oracle updates

**Added:** `crates/spark-testkit/tests/phase2_decay_adjudication.rs`, 10 tests. Its harness
drives:
- genesis, then commands for writes and activations;
- scheduled decay evaluations and scheduled shocks;
- stops at which the committed `(value, updated_at)` is read.

Each vector runs in its mirror image as well, so recovery is covered too. Every hand value
is also produced by `model`, a brute-force reference that enumerates grid points one by one
and shares no code with the engine.

| Test | Vector class | Exact vector (sign +) |
|---|---|---|
| `d2_fresh_assignment_keeps_the_fixed_grid` | fresh assignment, reassignment | assign 100 at 9 → 90 at 10, 90 at 19, 80 at 20; catch-up 80 at 20; reassign 50 at 17 → 40 at 20 |
| `d2_post_shock_decay_keeps_the_fixed_grid` | post-shock (scheduled and command) | 70@30, 75@31, **65@40**, 65@41, 55@50; command shock at 39 → 65@40 |
| `d3_d4_step_ending_at_activation_belongs_to_the_closing_segment` | activation boundary | 2/7 → 5/4 at 21: 94@21, 94@24, 89@25; catch-up 89@25 |
| `d3_residual_interval_is_discarded_at_a_parameter_change` | residual discard | 2/7 → 5/4 at 19: 96@19, 96@22, 91@23, 91@26, 86@27 |
| `d3_d5_shortened_cadence_starts_a_new_grid_at_the_barrier` | shortening | 1/100 → 5/10 at 53: 100@60, 95@63, 90@73; cadence-only 3/10 → 3/7 at 25: 94@30, 91@32 |
| `d3_d5_lengthened_cadence_starts_a_new_grid_at_the_barrier` | lengthening, unchanged control | 1/10 → 5/100 at 55: 95@150, 90@155; unchanged 94@60 |
| `d6_d7_saturation_and_unmoved_evaluations_commit_at_canonical_time` | saturation, unmoved, rate 0 | 12 at 5/10 → 4/3 at 25: 7, 2, (2,25), (2,26), (0,28), (0,31), (0,40); all 64 subsets → (0,40); rate-0 segment 70@45, 70@60, 60@70 |
| `d1_chunking_across_unaligned_writes_and_barriers` | chunking | composite (assign 9, 2/7 → 5/4 at 19, +5 at 29, → 3/6 at 36): 98, 98, 93, 88, 93, 88, 83, **80@45**; 64 partitions → (88,27), (80,45) |
| `d_replay_restore_and_pacing_reproduce_the_decision` | replay, restore, pacing | composite, both signs: pacing 1 = 50; reconstruction and restore after each of 4 commands digest-identical |
| `d_generated_timelines_with_writes_match_the_reference_model` | generated sweep | 40 timelines with a fresh unaligned assignment, reassignments, shocks of both signs, changed and unchanged activations, and evaluations before the cell exists |

**Assertions enforcing the rejected elapsed-since-write reading.** The repository tests
contain none that still enforce it. The prior writer pass had already changed the only one:
composition 2 of `phase2_oracle_completion.rs::at_i22_sanctioned_compositions_and_cross_family_rejection`,
from 75 at 40 to 65. That pass justified the change as a representation necessity, and the
review did not accept that justification. This pass keeps the value, which is now
controlling by D-2/S-6. Its comment is rewritten to state that the former value enforced
the superseded S-1/S-5 reading, which the Operator rejected.

Likewise `phase2_bounded_revision.rs::c2_05_epoch_bound_rates_compose_across_the_barrier`
keeps 95 at 60 for barrier 55. Its comment previously said "the residual interval the
review requires excluded"; the review never required that. It now cites D-3/D-5.

Both edits are comment-only. No value changed and no test was weakened. The negative
control in `c2_05_decay_preserves_elapsed_cadence_remainder_converted` ("measuring from
`updated_at` … yields 90") names the rejected reading as a wrong implementation, which
remains correct.

The review's `r2prime_frozen_q7_full_cadence_after_fresh_assignment` and
`r2prime_frozen_q7_full_cadence_after_separate_shock` are historical review evidence and stay
byte-unchanged (§9).

**Oracle rows after this pass** (**S** = meaningful finite executed support, not universal
proof):

| Entry | Review | Now | Evidence / remaining limit |
|---|---|---|---|
| AT-I22 → **AT-I22′** | R | S | (a) 64 partitions over unaligned writes and two barriers, plus the prior 32-subset and 40-timeline sweeps; (b) baseline convergence/saturation both signs, exact clamping; (c) coincident-shock rejection and both compositions at fixed-grid values (O, A); (d) fresh-assignment, post-shock and unmoved-commit vectors pinned. Engine digests are compared only between runs of one scheduled history, as before |
| AT-I23 → **AT-I23′** | S, policy qualified | S | (a)–(g): the policy qualification is removed by the decision; each clause has a named vector, both signs, and a killed mutant for the rejected residual/endpoint alternatives |
| every other row | as the review | unchanged | this pass changes no executable code; all other tests pass unmodified; AT-I13 stays P; AT-I39/AT-I40 stay per-profile |

## 6. Matters the decision does not address (adjudication §5)

These describe the implementation's existing behavior. **None is an Operator ruling.**
Each is flagged for the independent reviewer, and if necessary for the Operator.

1. **Removing and restoring the operation.** An epoch whose artifact lacks the operation
   integrates nothing and closes the segment. A later activation that restores it starts a
   new grid at its barrier, even if the parameters equal the earlier ones. Pinned by the
   earlier `c2r_02_zero_rate_and_absent_intervals` and the earlier generated sweep. The new
   sweep deliberately excludes it.
2. **Grid points before an unevaluated non-decay write are never applied.** A write is not
   a decay evaluation. For example, a cell at `(100, 0)` with no evaluation at 10, shocked
   +5 at 15, reads `(105, 15)`; decay at 20 then gives 95. An evaluation at 10 first gives
   85. The grid is not re-phased (D-2 holds). The shock simply reads the stored value. The
   superseded duration reading behaves identically in this respect. Chunk invariance
   (AT-I22′(a)) is claimed only across intervals without an intervening non-decay write.
   The reference model encodes this behavior and the generated sweep exercises it. No
   dedicated named vector pins it.
3. **Two activations at the same logical time.** They are processed in lineage order; a
   zero-length epoch still updates the segment state. So the grid restarts at that time
   if either activation changes the parameters relative to its predecessor. Not exercised
   by a test.
4. **Decay of an absent cell.** The evaluation emits nothing and creates no cell. The
   generated sweep evaluates before the fresh assignment, and the model agrees (no cell).

## 7. Negative controls

**Compiled mutants** (`run_mutation_controls.py`, `mutation-controls.json`, `mutant-<id>.txt`).
Each mutant is a disposable `git archive` of checkpoint 1 with one named wrong
implementation. Every listed test first passes on the unmutated archive. "Killed" means the
mutant compiles and the test fails by an assertion.

| Mutant (wrong implementation) | Killed by (A = new file) |
|---|---|
| C2-05-relative-steps: the **superseded elapsed-since-write count** | prior D sweep, prior B remainder; A fresh assignment, A post-shock, A shortening, A chunking, A generated sweep |
| C2R-01-backdated-commit | prior D reapplication; A fresh assignment, A saturation |
| C2R-02-absolute-grid (new parameters charge pre-barrier time) | prior D lengthened, prior D rate-only; A residual, A shortening, A lengthening |
| C2R-02-reset-every-activation | prior D unchanged; A lengthening (unchanged control) |
| C2R-03-skip-unmoved | prior D saturation, prior D every-evaluation; A fresh assignment, A saturation |
| **ADJ-carried-residual-origin** (new grid keeps the old phase) | A residual, A lengthening |
| **ADJ-residual-billed-at-old-rate** (residual billed as a whole old step) | A residual, A lengthening |
| **ADJ-drop-closing-endpoint** (step at the barrier leaves the closing segment) | A activation boundary |
| AT-I8-deferred-obligation-insert | prior D (the prior O test survives) |
| AT-I20b-tombstone-producer | prior D (the prior O test survives) |

Result: 21 distinct baseline tests pass on the unmutated archive. All 10 mutants compile.
Every one of the 28 listed kill runs fails by an assertion. Both prior O tests survive
their mutants (`baseline_all_pass: true`, `all_killed_as_claimed: true`).

The prior seven patches are byte-identical to the prior writer's; they still apply,
because no executable line changed. The **controlling review's own four mutants**
(`run_reviewer_mutants.py`, unchanged patches and byte-unchanged probes, retargeted only to
this tree) are also killed after four passing baselines.

## 8. Surfaces

The review's 17 default-feature external surface controls pass unchanged
(`surface-probes.json`). This pass adds no production surface.

## 9. The review corpora

| Corpus | Result on this candidate |
|---|---|
| first review's original corpus | fails to compile only at the three revised surfaces (E0308/E0559/E0603/E0609), unchanged |
| first review's adapted corpus | **9 passed, 0 failed** |
| second review's probes | **13 passed, 1 failed**: only the obsolete backdated `LogicalTime(40)` assertion, as before |
| the review's time-literal-only copy of those probes | **14 passed, 0 failed** |
| the controlling review's `independent_probes.rs` (byte-unchanged) | **7 passed, 2 failed**: exactly `r2prime_frozen_q7_full_cadence_after_fresh_assignment` (`(100, 10)`) and `r2prime_frozen_q7_full_cadence_after_separate_shock` (`(75, 40)`). Both encode the superseded S-1 reading (adjudication §3). The review's observation probe of `(90, 10)` and `(65, 40)` passes |
| `independent_probes_decision_adapted.rs` (exactly those two expected tuples changed to `(90, 10)` and `(65, 40)`; `check_evidence.py` verifies the two-line diff) | **9 passed, 0 failed** |

## 10. Validation (fresh; code-and-test tree of checkpoint 1 `244c54b`)

The runs used storage-bounded builds, as in the controlling review
(`CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`, `CARGO_INCREMENTAL=0`,
`CARGO_BUILD_JOBS=2`). These settings change no source, assertion or optimization level.
`checks.json` has the commands, exit codes and seconds.

| Check | Result |
|---|---|
| `cargo fmt --all --check` | exit 0 |
| `cargo test --workspace --all-features --no-fail-fast` | **400 passed, 0 failed, 0 ignored** (390 prior, including 232 inherited Phase-1, plus 10 new) |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| strict core/engine lint (unwrap, expect, panic, indexing, arithmetic denied) | exit 0 |
| `cargo metadata --format-version 1` | exit 0 |
| `git diff --check 6b167d8 HEAD` / `7e3a0aa HEAD` | both exit 0 |
| five Windows/Android `cargo check --workspace --all-targets` | all exit 0 (static only) |
| preservation | exit 0, no failures. Inherited tests, manifests and lockfile are byte-identical, as are the 17 inline Phase-1 modules and the 5 protected functions. `spark-core` is untouched by this pass. All 4 modified crate files are comment-only. The adjudication was introduced alone in checkpoint 0 and is unchanged since. Every `engineering/` change is an addition |
| release workload | exit 0. 1k / 4k / 16k finalized commands: history build 0.331 / 7.552 / 124.765 s; indexed preflight 2 383 / 2 368 / 2 406 ns (flat); scan reference 5 388 / 41 169 / 971 318 ns; whole finalization 0.77 / 3.72 / 14.15 ms. One machine, not a benchmark |
| corpora, mutants, reviewer mutants, surfaces | §7–§9 |

## 11. Limitations (honest scope)

1. The four matters of §6 are implementation behavior, not Operator decisions. Matter 3,
   same-time activations, is untested. Matter 2, forfeiting unevaluated points at a write,
   is exercised only by the model-agreement sweep.
2. The vectors are finite fixtures and a finite generated sweep, not a proof. The reference
   model encodes the decision as the writer reads it; the independent review should check
   that reading against §2 of the adjudication.
3. All prior scope limits stand:
   - AT-I39/AT-I40 are a per-profile composition, with no multi-profile runtime;
   - AT-I13 has a fixed manifest and no migration lifecycle;
   - AT-I21 depth overflow is reached only through the test-support seam;
   - AT-I33's "maximal" means the configured finite bound;
   - replay and restore are in memory only, with no durable storage, crash recovery,
     mailbox, transport, service or G.A.M.E. integration;
   - Windows/Android coverage is static only.
4. Performance: `decay_walk` walks the whole retained lineage per evaluation, which is
   bounded by activations. Whole finalization remains `O(history)` through the unchanged
   Phase-1 `submit_fence` (C2-09).
5. No independent review of this candidate exists yet.

Next action: the final independent Codex HIGH review under
`engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_ADJUDICATION_INDEPENDENT_REVIEW_MISSION_2026-09-11.md`.
