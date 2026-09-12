# LUNA-C Bounded Collection-Only Continuation Campaign — 2026-09-12

## Role and authority

Independent adversarial tester, role LUNA-C (continuation). Collection only:
no file under `crates/` in any checkout was created, edited, or deleted by
this campaign. No corrective candidate was opened. No other checkout,
worktree, or branch was touched, reset, or deleted.

Authority: `engineering/phase2/SPARK_PHASE_2_GATE_C2_OPERATOR_NEXT_AUTHORIZED_STEP_2026-09-12.md`
plus the 2026-09-12 SPARK-GAME safe handoff mission section 1, authorizing a
bounded collection-only continuation of the original LUNA campaign
(2026-09-12T11:30:30Z–11:34:00Z, ~4 of its 30 allotted minutes) for its
unused window time. Continuation budget: 26 minutes of testing wall-clock,
hard limit.

## Actual UTC start/stop and elapsed time

- **Start:** 2026-09-12T12:25:36Z (first verification command run)
- **Stop:** 2026-09-12T12:32:50Z (evidence and cleanup complete)
- **Elapsed:** approximately 7.2 minutes of wall-clock testing/build/write
  time, well inside the 26-minute hard limit. No extension was taken or
  needed.

(Elapsed time is reported honestly as actually spent, not as the full
budget — most of the 26 minutes were unused, similar in spirit to the
original campaign's own under-use of its 30-minute window.)

## Target and verification

- Exact pinned commit under test: `67b877192cc78b75c6fbe60c69b5594dc10befe8`
  in `/home/chromikey/Projects/SPARK`.
- Verified against live GitHub with:
  `git -C /home/chromikey/Projects/SPARK ls-remote --heads origin`
- Observed: `67b877192cc78b75c6fbe60c69b5594dc10befe8` is present on
  `origin`, at the head of
  `refs/heads/candidate/phase2-gate-c2-w01-correction-20260912`. Full
  `ls-remote` output was captured in the session transcript; the relevant
  line was:
  ```
  67b877192cc78b75c6fbe60c69b5594dc10befe8	refs/heads/candidate/phase2-gate-c2-w01-correction-20260912
  ```
- The disposable worktree at `/tmp/luna-c-20260912/target` was created
  detached at that exact commit and its `HEAD` was re-verified via
  `git rev-parse HEAD` immediately before running tests: matched exactly.
- Note: the main checkout's own current branch (`review/v3-f01-final-20260910`,
  HEAD `5b19d7b...`) is unrelated to the pinned target and was never
  touched; only a new detached worktree was added and later removed.

## Disk discipline

- `df -h /` before starting: 9.9G free (92% used).
- `df -h /` re-checked before and after the build: remained ~9.8–9.9G free
  throughout; never dropped below the 3G stop threshold.
- Build used `CARGO_TARGET_DIR=/tmp/luna-c-20260912/cargo
  CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2`.
- `/tmp/luna-c-20260912/cargo` (the build cache) was removed at the end of
  the campaign, as instructed. `/tmp/luna-c-20260912/target` (the worktree)
  was removed with `git worktree remove --force` afterward, leaving only
  `evidence/` in place.

## What was exercised

A disposable external crate at `/tmp/luna-c-20260912/harness` (workspace of
its own, depending on `spark-core`, `spark-engine` with `test-support`, and
`spark-testkit` by path into the pinned worktree) with tests in
`harness/tests/luna_c.rs`, run through the public door only:
`Engine::genesis`, `Engine::process`, `Engine::snapshot`, `Engine::restore`,
`Engine::reconstruct_completed`. Three new, discriminating test cases,
none of which duplicate the existing 427-test workspace suite (not
re-run; it already passed on this exact tree per the mission brief):

1. **`random_walk_snapshot_restore_and_reconstruct_agree_seed_0xc0ffee1234`**
   — a generated, seeded (fixed seed `0xC0FFEE_1234`, xorshift64\* PRNG,
   recorded in the test source) 40-step mixed history: randomized
   time-advance increments, randomized command kind selection across three
   distinct command kinds/targets/scopes, and randomized signed payload
   values. At three randomly-chosen checkpoints mid-sequence, the engine is
   snapshotted and restored, and the restored engine's
   `(engine_state_digest, stable_boundary_digest)` pair is asserted equal
   to the live engine's before continuing the same sequence on the
   restored instance. At the end, the full command history is replayed
   independently through `Engine::reconstruct_completed` and its digest
   pair is asserted equal to the live-processed engine's. This is novel
   coverage against gap #1 in the original campaign (no generated sequence
   testing, no snapshot/restore round trips, no replay-equivalence check)
   — **result: passed, no divergence found.**

2. **`activate_epoch_is_refused_while_a_different_request_is_paused`** —
   forces a genuine `Outcome::Paused` by declaring `max_due_per_cycle = 1`
   against three scheduled work items spread across two due-time slices
   (two due at `t=8`, one at `t=10`), then, while the engine's `active`
   `RequestDiscriminator` is still the pending `Advance(10)`, presents a
   distinct `CommandPayload::ActivateEpoch` request at the same horizon.
   Asserts the engine returns `Outcome::RefusedActiveRequestMismatch`
   (never silently reorders the interloper ahead of the paused request),
   that the original paused request still completes normally afterward,
   and that the same epoch activation succeeds once no request is
   outstanding. This is novel coverage against gap #2 (epoch activation
   interleaved with a paused request) — **result: passed; engine correctly
   refuses the interloper and preserves FIFO discipline on the
   `ActiveRequest` boundary.**

3. **`max_effects_per_wave_breach_surfaces_typed_semantic_cap_report`** —
   declares `max_effects_per_wave = 5` and a rule that emits 20 distinct
   `Add` effects to 20 distinct `(definition, scope)` targets in a single
   wave (using distinct scopes so the effects cannot legitimately coalesce
   into one committed write, unlike same-target aggregation). Asserts the
   resulting `CohortReport` carries a typed
   `CohortOutcome::Rejected { wave: 0, rejection: WaveRejection::SemanticCap
   { cap: "max_effects_per_wave", observed: 20, bound: 5 } }`, not a panic,
   not a silent truncation, and not an untyped generic error. This is
   novel coverage against gap #5 (semantic cap overload paths) — **result:
   passed.**

An earlier draft of tests 2 and 3 initially failed for reasons traced to
my own test construction, not engine defects (see Triaged non-findings
below); both were corrected and the corrected versions pass.

## Findings

**Zero reproducible bugs found.** See `findings.md` for the index.

## Triaged non-findings (my own probe's wrong assumptions, corrected)

1. **Initial semantic-cap probe undercounted effects.** My first draft of
   test 3 emitted 20 `Add(1)` updates to the *same* `(state.stress, actor
   "a")` target. The engine correctly coalesces same-family `Add`
   operations to one target into a single committed effect (aggregation
   semantics, not a cap-evasion bug) — the wave report showed exactly one
   `CommittedEffect` with value 20, `CohortOutcome::Committed`, no
   rejection. This is expected same-target aggregation, not a defect;
   settled by inspecting `crates/spark-engine/src/engine.rs` wave
   commit logic and the `CommittedEffect`/`WaveReport` shapes in
   `crates/spark-engine/src/report.rs`. Fixed by targeting 20 distinct
   scopes so each effect is genuinely separate.
2. **Initial pacing-pause probe used one due-time slice.** My first draft
   of test 2 scheduled three work items all due at `t=10` with
   `max_due_per_cycle=1`. Reading `Engine::process`'s pacing loop
   (`crates/spark-engine/src/engine.rs` around the P0–A4 comments) showed
   that the *first* due-time slice is always admitted in full regardless
   of the budget (`admitted_executable == 0` bypass), with the budget
   exception only biting a *subsequent* slice in the same call. A single
   slice of three items therefore completed in one call instead of
   pausing. This is documented, intentional "at least one slice always
   admitted" pacing behavior, not a defect. Fixed by splitting the work
   across two due-time slices (`t=8` ×2, `t=10` ×1) so the second slice
   triggers the pause as designed.

Both are recorded here exactly per instructions: as triaged non-findings
with the source location that settles them, not as invented requirements.

## Coverage gaps that remain (explicitly not tested)

Given the bounded budget, the following items from the original campaign's
declared-gap list and this mission's attack list were **not** exercised
and remain open for a future bounded continuation:

- **Decay/recovery across an epoch boundary that changes rate or cadence**
  (mission item 3, the D-3…D-5 fixed-grid adjudication family). Not
  attempted: after reading `Engine::decay_walk`'s per-lineage-segment grid
  logic, I judged that constructing a correct hand-verified boundary
  vector (as the existing `phase2_decay_adjudication.rs` suite does, by
  hand, checked against a from-scratch reference model) was not safely
  achievable to a trustworthy standard inside the remaining bounded time
  without risking a false finding built on my own miscalculated
  expectation rather than an engine defect. This is an honest scope
  limitation, not a claim the engine is untested here — the existing suite
  already covers this family; my gap is that I added no *independent*
  vector for it this session.
- **`Update::Aggregate` and Derived-authority composed-body targets, and
  removal/restoration of composing bodies** (mission item 4). Identified
  `Update::Aggregate { source, weight }` in `crates/spark-engine/src/rules.rs`
  as the relevant primitive but did not build a test against it in the
  time available.
- **Timeline ordinal-window edges through the engine's command door
  specifically** (staging, window exhaustion, `reset_timeline_epoch`, and
  replay across a reset) (mission item 6). I exercised
  `CompletedHistory.resets` plumbing only implicitly (an empty vector in
  test 1); I did not exercise a real `reset_timeline_epoch` call, a
  post-reset command (which requires hand-building a `CommandRequest`
  with the new `TimelineEpoch` rather than the `command_request()` helper,
  which hardcodes `TimelineEpoch(0)`), or `reconstruct_completed` replay
  across a reset boundary.
- No exhaustive/property-based state-space search (still out of scope for
  a hand-written, bounded, deterministic-seed campaign — the mission asks
  for generated/randomized fixed-seed sequences, which test 1 provides,
  not exhaustive enumeration).
- No production mutation testing (out of scope per the mission's collection-
  only constraint; mutation testing on the compiled crate is a corrective/
  research activity, not collection).
- No fuzzing, no property-based shrinking, no multi-day soak.

## Commands and exit codes (see `full-run.txt` for the complete log)

```
$ git -C /home/chromikey/Projects/SPARK ls-remote --heads origin   # verified pin, see above
$ git -C /home/chromikey/Projects/SPARK worktree add --detach /tmp/luna-c-20260912/target 67b877192cc78b75c6fbe60c69b5594dc10befe8
$ CARGO_TARGET_DIR=/tmp/luna-c-20260912/cargo CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --offline -- --test-threads=1 --nocapture
running 3 tests
test activate_epoch_is_refused_while_a_different_request_is_paused ... ok
test max_effects_per_wave_breach_surfaces_typed_semantic_cap_report ... ok
test random_walk_snapshot_restore_and_reconstruct_agree_seed_0xc0ffee1234 ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
EXIT_CODE=0
```

## Honest summary

Negative result: three new, purposefully discriminating, public-door test
cases targeting previously-declared gaps (generated/seeded mixed-history
sequences with snapshot/restore and replay-equivalence checks, epoch
activation interleaved with a paused request under the `ActiveRequest`
boundary, and a semantic-cap overload path) all passed against the pinned
commit. Zero reproducible bugs found. Two of my own draft probes were
wrong on first attempt and are recorded above as triaged non-findings with
their settling source locations, not as defects. Substantial declared gaps
remain (decay-across-epoch-boundary independent vectors, `Aggregate`/
derived-authority composed bodies, and ordinal-window/reset-epoch replay
through the command door) and are listed above rather than glossed over.
