# SONNET-C Bounded Collection-Only Continuation Campaign

## Role and authority

Role: SONNET-C (continuation), independent adversarial tester. Not the main
engineer, not a reviewer, not a repairer. COLLECTION ONLY: no file under
`crates/` in any checkout was edited, no corrective candidate was opened.

Authority: `engineering/phase2/SPARK_PHASE_2_GATE_C2_OPERATOR_NEXT_AUTHORIZED_STEP_2026-09-12.md`
plus the 2026-09-12 SPARK-GAME safe handoff mission, section 1, authorizing a
bounded collection-only continuation of the original SONNET campaign for its
unused window time.

## Target and verification

Pinned commit: `67b877192cc78b75c6fbe60c69b5594dc10befe8`.

Verified against live GitHub with:

```
git -C /home/chromikey/Projects/SPARK ls-remote --heads origin
```

Observed line:

```
67b877192cc78b75c6fbe60c69b5594dc10befe8	refs/heads/candidate/phase2-gate-c2-w01-correction-20260912
```

The pinned commit is exactly the head of
`candidate/phase2-gate-c2-w01-correction-20260912` on `origin` at verification
time. Local commit message at that hash: "Publish C2W-01 correction report,
evidence and review mission."

## Actual timing (UTC, self-recorded)

- Setup start (worktree creation): `2026-09-12T12:26:13Z`
- Final clean test run completed: `2026-09-12T12:36:29Z`
- **Elapsed testing wall-clock: ~10.3 minutes**, against a hard 19-minute
  continuation budget. I stopped short of the budget once I had exhausted the
  highest-priority open vectors and reached a clean, reproducible negative
  result; I did not use the remaining ~9 minutes to avoid stretching thin,
  low-value probes.

## Setup

```
mkdir -p /tmp/sonnet-c-20260912
git -C /home/chromikey/Projects/SPARK worktree add --detach /tmp/sonnet-c-20260912/target 67b877192cc78b75c6fbe60c69b5594dc10befe8
```

Disposable external crate at `/tmp/sonnet-c-20260912/harness` per the mission's
exact recipe (spark-core, spark-engine with `test-support`, spark-testkit, all
path deps into the worktree). Tests in
`/tmp/sonnet-c-20260912/harness/tests/sonnet_c.rs`.

Disk: `df -h /` showed 9.8-9.9G free throughout (root at ~92% used), never
dropped below the 3G stop threshold. Builds used
`CARGO_TARGET_DIR=/tmp/sonnet-c-20260912/cargo CARGO_INCREMENTAL=0
CARGO_BUILD_JOBS=2`.

## What I exercised

Given the original campaign's 30 cases already found zero reproducible bugs
and two of its five listed coverage gaps have since been closed (obligation-
drop restore mutation via `ObligationStore::keys()`, and a genuinely distinct
second profile), I did not re-probe those two and instead wrote 7 new,
narrowly-targeted cases against the remaining open vectors:

1. **SC-1** -- `sc1_snapshot_restore_mid_paused_command_then_resume`.
   A command paused mid-flight (budget forces a cohort boundary before the
   command's own horizon), snapshot taken while `ActiveRequest` is
   `Some(Paused-command)`, restored into an independently-activated profile,
   then: (a) restore succeeds since `horizon >= frontier`; (b) presenting a
   *different* request while still mismatched returns
   `RefusedActiveRequestMismatch` and leaves the engine byte-identical
   (`stable_boundary_digest` unchanged); (c) presenting the *same* request
   resumes and reaches a state digest-identical to an unbroken twin engine
   driven straight through with no snapshot/restore in between; (d) the
   engine that absorbed the failed mismatch attempt can still correctly
   resume afterward, proving the refused attempt left no residue.

2. **SC-2** -- `sc2_epoch_reset_mid_paused_command_then_resume_refuses_stale_epoch`.
   `reset_timeline_epoch` called directly while `ActiveRequest` is
   `Some(Paused-command)`. Confirms empirically the doc comment at
   `engine.rs` above `reset_timeline_epoch` ("changes neither `F` nor
   `ActiveRequest`"): both are unchanged immediately after the reset. Then
   resumes the now-stale-epoch command to completion: the engine correctly
   returns `CompletedCommandNotFinalized(FinalizationRefusal::WrongTimelineEpoch)`
   (P-2, D-2) rather than silently finalizing under a mismatched epoch, `F`
   still advances to the horizon, `ActiveRequest` clears, and the engine
   remains fully usable for a fresh command under the new epoch.

3. **SC-3** -- `sc3_store_invariant_violated_is_sticky_across_different_requests_and_blocks_snapshot`.
   A genuine scheduler/`ObligationStore` disagreement forced via
   `fixture::tamper_obligations(&mut e, &key, None)` (dropping a claim set the
   scheduler still expects). Confirms: the engine fail-stops with
   `StoreInvariantViolated`; the *identical* sticky outcome is returned for a
   repeat of the same request AND for a structurally different request
   (`Request::Command` after the triggering `Request::Advance`) -- i.e. the
   fail-stop check precedes all per-request logic regardless of request
   shape; `snapshot()` is refused with `SnapshotRefused::FailStopped`; and
   restoring an earlier, pre-tamper snapshot recovers a live, non-fail-stopped
   engine.

4. **SC-4** -- `sc4_snapshot_with_staged_residue_at_real_frontier_refuses_restore`.
   Staged an envelope directly into the *live* engine's timeline at the real
   frontier ordinal (`Engine::timeline_frontier_ordinal()`, per the campaign's
   own recorded observation about the `NotInAdmissionWindow` no-op trap) via
   `fixture::stage_raw`. Confirmed `staging_is_clean()` correctly flips to
   `false`, the engine snapshots the residue without itself refusing, and
   `Engine::restore` correctly fails closed with
   `RestoreError::TimelineStagingPresent` (Revision 2 §7 step 2b).

5. **SC-5** -- `sc5_max_enqueue_per_wave_semantic_cap_rejects_atomically_without_fail_stop`.
   Backpressure/boundedness: a command-triggered rule declaring two
   `ScheduleOp`s against a `max_enqueue_per_wave` budget of 1. Confirms the
   typed `WaveRejection::SemanticCap { cap: "max_enqueue_per_wave", observed:
   2, bound: 1 }` fires on wave 0, is reported per-cohort
   (`CohortOutcome::Rejected`) rather than as a fail-stop, that no
   obligation from the rejected wave partially lands
   (`e.obligations().is_empty()`), and that the engine remains fully usable
   for a later, unrelated command afterward.

6. **SC-6** -- `sc6_conflicting_work_key_extracts_and_reports_conflict_only_and_survives_restore`.
   A genuine duplicate/conflicting `WorkKey` staged via two calls to
   `fixture::schedule_record` under the identical key with distinct emission
   identities (hence distinct canonical payload hashes) -- the fixture's
   documented commutative mirror of `Scheduler::schedule`. Confirms the claim
   set reports `is_contested() == true`, extraction/cohort processing reports
   `CohortKind::ConflictOnly` / `CohortOutcome::ConflictOnly` with a non-empty
   conflict section, neither contested record's effect is arbitrarily
   applied, the engine is not fail-stopped by an ordinary conflict, and the
   contested state round-trips through snapshot/restore with an
   exactly-matching `stable_boundary_digest` and `is_contested()` preserved.

7. **SC-7** -- `sc7_reconstruct_completed_refuses_a_history_with_a_non_finalizing_command`.
   An adversarial `CompletedHistory` (one that could never be legitimately
   produced by a real engine, since only actually-finalized commands are
   recorded) containing one command stamped for a `TimelineEpoch` that will
   never match genesis's epoch. Confirms `Engine::reconstruct_completed`
   refuses closed with `ReplayError::StepRefused { step: "command 0" }`
   rather than silently accepting a non-finalizing replay step or reporting
   it as an unrelated `Diverged` digest mismatch.

All 7 tests pass against the pinned commit. Full captured output is in
`full-run.txt`; exit code 0.

## Findings

**Zero reproducible bugs found.** See `FINDINGS_INDEX.md`.

## Triaged non-findings

- **`reset_timeline_epoch` runs while `ActiveRequest` is `Some`.** Initially
  flagged as a candidate gap (an epoch reset landing mid-paused-command
  seemed like it could let a stale-epoch command silently finalize under new
  rules). Settled as **not a defect**: the doc comment directly above
  `reset_timeline_epoch` in `crates/spark-engine/src/engine.rs` explicitly
  states it "changes neither `F` nor `ActiveRequest`" -- this is declared,
  intentional behavior, not an oversight. SC-2 additionally confirms the
  *consequence* is handled correctly: the stale-epoch command refuses
  finalization (P-2 `WrongTimelineEpoch`) rather than silently applying.

- **`RefusedActiveRequestMismatch` takes priority over
  `RefusedHorizonBehindFrontier` when both conditions could apply.** In
  `Engine::process` (P0), the active-request-mismatch check runs
  unconditionally before the horizon-vs-frontier check, which only executes
  in the `None` (no active request) branch. So a mismatched request whose
  horizon happens to be behind the current frontier is reported as
  `RefusedActiveRequestMismatch`, never as `RefusedHorizonBehindFrontier`.
  I did not write a dedicated test for this because it follows directly and
  deterministically from the read code path (`engine.rs` lines ~1446-1474)
  and I found no controlling-document text requiring the opposite priority;
  recording it here as a settled reading rather than spending harness time
  reproducing what the source already makes unambiguous.

- **`fixture::snapshot_stage_raw` / `fixture::stage_raw` return
  `Result::is_ok()`, which is `true` for the retryable
  `StageDisposition::NotInAdmissionWindow`.** Pre-recorded by the mission
  brief; not re-reported as new. SC-4 explicitly worked around it by staging
  at `Engine::timeline_frontier_ordinal()`, the real frontier, and confirmed
  that staging then actually takes effect (`staging_is_clean()` flips).

## Coverage gaps that remain (explicitly not closed by this continuation)

- **Interrupted-request-boundary matrix across an epoch reset was only
  partly covered.** SC-2 covers reset-while-paused resumed with the *same*
  stale-epoch request. Not covered in this window: reset-while-paused
  resumed with a *different* request (interacts with the
  `RefusedActiveRequestMismatch`-priority note above, since a mismatched
  different request refuses on identity before epoch is ever checked), and
  reset-while-paused where the paused request is a bare `Advance` rather than
  a `Command` (no `finalize_command`/P-2 path applies to `Advance` at all, so
  the interesting question there is different: whether a stale-epoch
  `Advance`'s remaining cohorts evaluate under old or new rules -- not
  probed).
- **`reconstruct_completed` with refused commands / epoch activations mixed
  into the SAME history** (e.g., an `ActivateEpoch` command adjacent to a
  reset record, or several non-finalizing commands in sequence) was not
  probed beyond the single minimal case in SC-7.
- **Deep wave-chain depth (`max_wave_depth`) and `max_obligations` /
  `max_scheduled_work` store-pressure caps** were not exercised at all in
  this window (only `max_enqueue_per_wave` was, in SC-5). Very large
  same-time slices (`max_cohort_candidates`, `max_effects_per_wave`) were
  also not separately exercised.
- **`fail_stop` stickiness was only exercised for `StoreInvariantViolated`**
  (SC-3), not for `FinalizationEntailmentViolated` in this continuation
  window (reachable via `forge_empty_parents` or a contested-emission
  `set_seed_duplication(Some(true))` per the fixture doc comments). I note,
  without claiming this substitutes for adversarial testing, that the
  workspace's own regression suite (`phase2_finalization_and_replay.rs`,
  not the adversarial campaign) already contains direct assertions on
  `Outcome::FinalizationEntailmentViolated`; I prioritized the
  less-covered `StoreInvariantViolated` sticky path instead, given the time
  budget. Whether `FinalizationEntailmentViolated` stickiness holds under
  the SAME cross-request-type probe as SC-3 (a different request after the
  fail-stop) was not independently verified by me.
- I did **not** re-run the full existing workspace test suite (427 tests);
  per the mission brief this had already passed on this exact tree and was
  out of scope for this continuation.
- I did **not** attempt any fuzzing, property-based testing, concurrency/
  threading probes (the engine's public API as exercised here is entirely
  single-threaded/synchronous), or performance/timing measurements.

## Negative result

This continuation window found **zero reproducible bugs**. This is recorded
as a genuine, honest negative result on the seven new vectors above, not as
evidence of exhaustive coverage -- see "coverage gaps that remain" for what
was deliberately left unexercised given the 19-minute hard budget.
