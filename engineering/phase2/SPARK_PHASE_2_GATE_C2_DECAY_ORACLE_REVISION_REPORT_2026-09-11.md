# S.P.A.R.K. Gate C2 — Second Bounded-Correction Report (decay timing and oracle completion)

**Date:** 2026-09-11
**Writer:** Claude Code (Opus 5, `claude-opus-5`), the separated correction writer. The
independent reviewer is Codex and is not the writer.
**Mission:** `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_ORACLE_REVISION_MISSION_2026-09-11.md`
**Branch:** `candidate/phase2-gate-c2-decay-oracle-revision-20260911`
**Verdict:** `GATE_C2_DECAY_ORACLE_REVISION_READY_FOR_INDEPENDENT_REVIEW`

Nothing is accepted or promoted. Production is unchanged. Phase 3, G.A.M.E. changes,
force-push, reset, discarded work and paid compute were not used.

## 1. Summary

The controlling review (`3a0b514`) found that the prior revision's decay remainder backdated
committed cells (C2R-01), that shortening a hot-tuned cadence charged the new rate before its
barrier (C2R-02), that saturation broke the claimed commit-time chunk invariance (C2R-03),
and that several oracle rows were overstated or incomplete (C2R-04).

**No architecture conflict exists.** A representation satisfies the frozen remainder and
chunk contract (v1 Q7, AT-I22) and the frozen cohort-time contract (FINAL §4) together,
without any encoding change:

1. every decay/recovery evaluation of an existing cell commits at the cohort's canonical
   time — no per-effect commit time remains anywhere in the engine;
2. whole cadence steps are counted on the operation's **parameter-segment grid**: grid
   points `origin + k·cadence`, where the origin is the activation time of the first epoch
   of a maximal run of epochs carrying the operation with the same resolved `(rate,
   cadence)` (genesis: 0).

Step counts on a fixed grid are additive over every partition of `(updated_at, now]`, so
the remainder survives a canonical-time commit; a new segment's steps lie wholly after its
barrier, so no new rate or cadence touches pre-barrier time. The choice and its
consequences are flagged as **R-2′** (§4) for the reviewer.

C2R-04 is addressed by one new test file with 16 tests, including a reference-model sweep
over 40 generated parameter timelines, and by three additive test-support seams. Four
prior Gate C2 writer tests pinned consequences of the rejected representation. They are
adapted in place, each with its reason (§7).

The workspace passes **390 tests, 0 failed, 0 ignored** (the 232 inherited Phase-1 tests
unmodified). Seven compiled wrong implementations are each killed by a named new test (§8).
Every other gate check passes (§10).

## 2. Lineage and custody

| Item | Value |
|---|---|
| Worktree | `/home/chromikey/Projects/SPARK-gate-c2-rev2` (fresh, isolated) |
| Base (direct parent of checkpoint 0) | `3a0b51463e1874ad8b02dd3a3261933fcd2e22f4`, the controlling review commit, verified local = tracking = live before branching |
| Reviewed candidate | `5b7fcf50161a65dc83b4806b513f01c0a4943ea5` (ancestor) |
| Production | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` (ancestor; live `phase1-refoundation-v2`, unchanged) |
| Checkpoint 0 | `8844264` — mission recorded before any code change |
| Checkpoint 1 | `5ae506a` — C2R-01 … C2R-04 code, tests, seams, adaptations; pre-correction probe red |
| Checkpoint 2 | `00221d8` — report, evidence and review mission |
| Checkpoint 3 | evidence whitespace correction only: four empty-output logs lost a trailing blank line that `git diff --check` flags (see the evidence README). This is the final candidate commit; its hash is in the receipt, since a commit cannot record its own hash |

Every pre-existing branch and worktree was left intact (`preservation.json` lists the
worktrees). Before any production change, the second review's probes were run unchanged
against the unchanged candidate code: **11 passed, 3 failed** — exactly the review's three
contract failures (`revision_probes_pre_correction.txt`).

## 3. Disposition of C2R-01 … C2R-04

| Finding | Disposition | Correction | Executed evidence |
|---|---|---|---|
| **C2R-01** remainder changed frozen commit time | **Corrected** | `Intent::Transform.commit_at`, `Reduced.at` and `WavePlan.commit_times` removed; `apply_wave` writes every committed effect at `now` (the cohort's canonical time); the remainder lives in the segment grid (`decay_walk`) | `c2r_01_…` (every single-evaluation time 1…35: cell `(value, t)`; resolved effects reapplied to an untouched clone at canonical time reproduce the engine digest); the reviewer's probe now reads `(90,15)` for both engines; mutant **C2R-01-backdated-commit** killed |
| **C2R-02** new rate before barrier | **Corrected** | a parameter change starts a new segment whose first step ends one new cadence after the barrier; the closed segment's incomplete step is not billed at either rate; unchanged parameters keep the grid; an epoch without the operation integrates nothing and closes the segment | shortened (reviewer fixture: **95**, and 90 at 70), lengthened and residual interval (95 at 150, 90 at 155; both signs), rate-only at an unaligned barrier (−95 at 60, −90 at 65), unchanged parameters (94), zero-rate and absent intervals, replay and restore; 40-timeline model sweep; mutants **C2R-02-absolute-grid** and **C2R-02-reset-every-activation** killed; the pre-correction probe gave 70 |
| **C2R-03** saturation / chunk claim | **Corrected** | every decay evaluation of an existing numeric cell emits its candidate — moved, saturated, rate 0, or no whole step — so the committed cell after catching up to `T` is `(value(T), T)` under every partition | converted saturation probe `(0,150)` both ways; exhaustive 32 subsets into saturation for decay and recovery; rate-0 / no-step / at-baseline commits; sweep compares value **and** commit time, single catch-up versus chunked; pacing, replay and restore digest equality; mutant **C2R-03-skip-unmoved** killed |
| **C2R-04** oracle gaps | **Completed for every applicable case named** (§6), with the accepted scope interpretations kept honest | AT-I8 per-committed-wave observation seam; AT-I20b retry of the rejected producer; AT-I21 value-dependent interference; AT-I28 per-prefix recomputation and injected index faults; AT-I33 maximal configured depth; AT-I39/AT-I40 composed per-profile oracle | §6; mutants **AT-I8-deferred-obligation-insert** and **AT-I20b-tombstone-producer** killed by the new tests and survived by the prior tests the review criticized |

The accepted corrections and interpretations are preserved unchanged: C2-01, -02, -03, -04,
-06 and -08; D-C2-5, -7, -11 and -13; D-C2-1, -2, -4, -6, -8, -9, -10, -12, -14 and -15;
R-1 and R-3 … R-9. Their tests all pass unmodified; §7 lists the only adaptations.

## 4. The representation choice, flagged for review (R-2′, replacing rejected R-2)

**Frozen texts reconciled.**

- **v1 Q7:** "whole elapsed cadence steps between `updated_at` and now"; "catch-up over N
  missed steps in one evaluation is therefore bit-identical to N separate evaluations, so
  batching/chunking cannot alter outcomes".
- **FINAL §4:** every inherited `now`/`at` during a cohort's evaluation is its canonical
  time.
- **AT-I22:** chunk invariance and convergence exactly at the baseline.
- **AT-I23:** a hot-tuned rate applies only from its epoch's barrier.
- **v1 Q7 closing clause:** `StateCell` and the other encodings are unchanged.

**Why the grid.**

- With commits at canonical time, `updated_at` is always the last evaluation time.
- A count of `(now − updated_at) / cadence` is then not additive: the original lost
  remainder, 15/20 → 90 instead of 80.
- Chunk invariance for arbitrary evaluation times therefore requires the step boundaries to
  be independent of `updated_at` — a fixed grid.
- The grid's phase must come from committed state. `StateCell` has no phase field, and
  canonical-time commits overwrite `updated_at`. The only committed, restore-validated
  source of phase is the activation lineage (R-3, accepted). Hence the origin is an
  activation time.

**Which activation.** The one that starts the current parameter segment.

- Taking genesis for every segment (an absolute grid) lets a new-rate step contain
  pre-barrier time. Mutant **C2R-02-absolute-grid** is killed: 90 at 150 instead of 95.
- Taking every activation moves the grid under an activation that changed nothing. Mutant
  **C2R-02-reset-every-activation** is killed: 95 instead of 94.

**Consequences, stated for adjudication.**

1. **Residual interval.** At a parameter-changing barrier, the closed segment's incomplete
   last step is never billed, because Q7 integrates whole steps only. Billing it at the new
   rate is the retroactive charge C2R-02 forbids; billing it at the old rate would apply the
   old rate after the barrier. The unbilled residual is always less than one old cadence and
   independent of evaluation times.
2. **A non-decay write does not re-phase the cadence.** A shock at 31 between decay
   evaluations at 30 and 40 still lets the step ending at 40 elapse: 65 at 40, and 65 again
   at 41. Re-phasing would need a per-cell phase: either a backdated `updated_at` (C2R-01)
   or a new `StateCell` field (a frozen-encoding change). Both sanctioned compositions (v2
   §4.3) remain deterministic and declared.
3. **Every decay evaluation of an existing cell is a committed write,** including an
   unmoved one. A change watcher on the target therefore fires for it, exactly as for an
   equal-valued assignment (pinned by `c2r_03_every_evaluation_commits_at_its_canonical_time`).
   A coincident same-wave shock always rejects as a cross-family mixture, as v2 §4.3
   requires, rather than only when the decay happens to move the value.
4. A step ending exactly at an activation time belongs to the closing segment. Scheduled
   work due at that time runs before the activating command, and its evaluation sees only
   the old epoch.

If the Operator prefers different residual or re-phasing semantics, that is an explicit
architecture decision. A per-cell phase would need a frozen-encoding change, so it is not
within this writer's authority; nothing here silently amends a frozen requirement.

## 5. Changed code

| File | Change |
|---|---|
| `crates/spark-engine/src/engine.rs` | `decay_walk` rewritten to the segment grid (checked arithmetic, returns the value); single-operation decay always emits; rule-body decay stage uses the same walk; `commit_times` removed; `apply_wave` writes at `now`; AT-I8 seam (`CommittedWaveObservation`, `Observation::committed_waves`, `probe_wave_invariants`, all `test-support` only); `lineage_is_valid` made `pub(crate)` for the fixture |
| `crates/spark-engine/src/effects.rs` | `commit_at` and `Reduced.at` removed; the transform intent encoding is restored byte-for-byte to the original candidate's form (a Phase-2 encoding; no Phase-1 digest moves) |
| `crates/spark-engine/src/fixture.rs` | seams: `probe_wave_invariants`, `lineage_recomputes`, `timeline_indexes_recompute`, `inject_timeline_index_fault`, `snapshot_inject_timeline_index_fault` |
| `crates/spark-core/src/timeline.rs` | **additive, `test-support`-gated only:** `DerivedIndexFault` and `TimelineIngress::inject_derived_index_fault` (AT-I28 fault injection). No production surface; the protected `stage`, `submit_fence`, `derived_indexes_consistent`, `schedule` and `drain_due` bodies are byte-identical (checked) |

No production API was added to simplify a probe: every new item is `test-support`-gated,
and that feature is enabled only by `spark-testkit`'s dev-dependency edge (feature-hygiene
test unchanged and passing).

## 6. Oracle rows after this revision

**S** = meaningful finite executed support (not universal proof). **P** = applicable
coverage remains partial or an accepted scope interpretation has no full lifecycle
fixture. **D** = `phase2_decay_oracle_revision.rs`. Every row not listed keeps the review's
disposition and passes in the fresh 390-test run.

| Entry | Review | Now | Evidence / remaining limit |
|---|---|---|---|
| AT-I1 | R | S | additive untouched-clone (O) plus decay reapplication at canonical time for every time 1…35 (D `c2r_01_…`) |
| AT-I8 | P | S | D per-committed-wave observation: after waves 0, 1, 2 of one cohort the invariant holds and both stores hold exactly 2, 3, 3 items; the whole-cohort O test survives the deferred-insert mutant, the D test kills it; inherited late-failure fixtures unchanged |
| AT-I20b | P | S | D: effects, candidates and enqueue caps each reject with every retained component unchanged by the wave (engine digest = post-extraction pre-wave digest; five non-consumption components equal the pre-request values), keys consumed, replay identical, and **the rejected producer** rescheduled under occurrence `n+1` (same producer, scope, work kind) commits; tombstone mutant killed (the O retry-via-other-producer test survives it) |
| AT-I21 | P | S | D value-dependent: (a) same-time interfering due work shares the continuation's snapshot (echo 5, not 12); (b) an interfering command between deferral and continuation is read (echo 20, not the deferral-time 5); determinism, priority, exact identity; larger-depth run differs as adjudicated. Depth overflow is still reached only through the test-support depth seam, because sound static validation precludes it. The discriminating values are in-test controls; no compiled mutant for this row |
| AT-I22 | R | S | segment grid: remainder, 32-subset saturation sweeps both signs, 40-timeline model sweep (value **and** commit time, single versus chunked), pacing / replay / restore digests, rate-0 and no-step commits, sanctioned compositions (adapted §7) and cross-family rejection. Engine-digest equality is asserted only between runs of one scheduled history (different evaluation `WorkKey` histories legitimately differ in scheduler, ledger and provenance content) |
| AT-I23 | R | S | D shortening (95), lengthening and residual interval, rate-only unaligned, unchanged parameters, zero-rate, absent operation, replay and restore; B fixed-cadence cases (adapted §7) |
| AT-I28 | P | S | D: after each of 10 prefixes (command, paused boundary, resume, activation, refused finalization, timeline reset, activation after reset, command, advance) the timeline's finalized-history indexes and the epoch lineage equal their recomputation, the invariant holds, and restore is exact; at every prefix three injected timeline-index faults (P-7, P-8, P-9 indexes) are detected live and refused at restore (`DerivedIndexesInconsistent`), and every lineage-time fault is refused (`EpochLineageInvalid`) |
| AT-I33 | P | S | D maximal configured depth: a two-parent-per-level chain of exactly `max_wave_depth` = 16 runs waves 0…16, every level's identities equal the frozen formula over the complete prior-level parent set; one level more is rejected statically, and through the depth seam converts both seeds typed and strictly later; `max_wave_depth = u32::MAX` is total. "Maximal" is the configured bound on a finite fixture, not `u32::MAX` levels |
| AT-I39 | P | S (per-profile) | D composed oracle: per-profile reports, cohort identities, emission identities, batch digests, obligation records, engine and stable-boundary digests byte-identical with or without a third profile, under both composition orders, the ascending `(due_time, profile_id)` interleaving, and pacing 1. A composition of independent per-profile engines (D-C2-2), **not** a multi-profile runtime |
| AT-I40 | P | S (per-profile) | D: two profiles at one due time form two distinct cohorts, each exactly the formula over its own members; a third profile perturbs nothing; a rule targeting another profile's definition is rejected at the door; another profile's command is refused (`WrongProfile`). Same scope limit as AT-I39 |
| AT-I13 | P | P | unchanged accepted interpretation: fixed manifest, no definition-migration lifecycle implemented |

## 7. Tests: added, adapted, none weakened

**Added:** 16 tests in `phase2_decay_oracle_revision.rs`. No inherited Phase-1 test,
manifest, lockfile, inline Phase-1 test module or protected function changed (mechanically
checked).

**Adapted:** four prior Gate C2 writer tests. Each pinned a consequence of the
representation C2R-01 … C2R-03 reject; each is corrected in place with a comment naming the
reason and the old value, and strengthened.

| Test | Old assertion | New assertion | Reason |
|---|---|---|---|
| B `c2_05_chunk_invariance_holds_for_every_evaluation_subset` | one evaluation at 15 → `(90, 10)` | `(90, 15)`, then `(80, 20)` at 20 | C2R-01: the old line pinned the backdated commit; the follow-up proves the remainder survives the canonical-time commit. The 32-subset sweep is unchanged |
| B `c2_05_epoch_bound_rates_compose_across_the_barrier` | barrier 55: 90 at 60 | 95 at 60; new 90 at 65 | C2R-02: the old value billed `(50, 60]` at the new rate, half of it before the barrier. Barrier-50 rows unchanged (90) |
| O `at_i22_sanctioned_compositions_and_cross_family_rejection` (composition 2) | decay at 40 → 75, 41 → 65 | 40 → 65, 41 → 65; value **and** commit time checked at every step | the old values required a shock to re-phase the cadence, representable only by a backdated or new per-cell phase (§4 consequence 2). Compositions 1 and 3 unchanged |
| S `at_i22_decay_catch_up_is_chunk_invariant` (early case) | evaluation at 9 commits nothing | commits `(100, 9)`; the step at 10 is then still taken (90) | C2R-03: "no candidate when unmoved" made `updated_at` partition-dependent |

The unchanged `effects.rs` inline unit test lost only the `commit_at: None` field the prior
revision had added; it is now byte-identical to the original candidate's form.

## 8. Negative controls

**Compiled mutants** (`run_mutation_controls.py`; `mutation-controls.json`;
`mutant-<id>.txt`). Each is a disposable archive of checkpoint 1 with one named wrong
implementation. Each listed test first passes on the unmutated archive. "Killed" means the
mutant compiles and the test fails by an assertion.

| Mutant (wrong implementation) | Killed by | Prior test that survives it |
|---|---|---|
| C2R-01-backdated-commit (commit at last whole step) | D `c2r_01_…` | — |
| C2-05-relative-steps (steps from `updated_at`, commit at now: the original lost remainder) | D model sweep; B remainder test | — |
| C2R-02-absolute-grid (one grid from 0) | D lengthened/residual; D rate-only unaligned | — |
| C2R-02-reset-every-activation | D unchanged-parameters | — |
| C2R-03-skip-unmoved | D saturation sweep; D every-evaluation-commits | — |
| AT-I8-deferred-obligation-insert | D per-wave invariant | O `at_i8_invariant_holds_after_every_committed_wave` |
| AT-I20b-tombstone-producer | D producer retry | O `at_i20b_every_semantic_cap_…_retryable` |

Result on tree `5ae506adf13ddf1d610595c59202c52b7ddee81a`: all 12 listed tests pass on the
unmutated archive. Seven of seven mutants compile and are **killed by assertion failures**
in every named test. Both prior tests **survive** their mutant, which confirms the gaps the
review identified (`baseline_all_pass: true`, `all_killed_as_claimed: true`).

The carried-anchor rule of the rejected R-2 (steps billed on the new cadence from the old
anchor) is not re-mutated: it is the reviewed candidate itself, and the second review's
probes recorded its red directly (70 instead of 95).

**In-test discriminators** are named in each test's documentation, including AT-I21's
alternative values, the model's rejected readings, and AT-I28's injected faults.

## 9. The review corpora, run unchanged

| Corpus | Tree | Result |
|---|---|---|
| second review's `independent_revision_probes.rs` | unchanged candidate code (before any change) | **11 passed, 3 failed** — the three C2R contract assertions (`revision_probes_pre_correction.txt`) |
| same | this candidate | **13 passed, 1 failed** (`revision_probes_post_correction.txt`). All three C2R contract probes pass (95; `(90,15)` both ways; `(0,150)` both ways). The one failure is the reviewer's positive control `c2_05_recovery_and_unsaturated_remainders_positive`: its value component passes (±45), but it also asserts `LogicalTime(40)`, the backdated time C2R-01 rejects; the canonical time is 42. This is the probe encoding the prior representation, not a regression. |
| first review's original corpus | this candidate | fails to compile only at the three surfaces already corrected in the first revision (external refusal construction, the removed report field, the decay `toward` shape); unchanged from the prior revision |
| first review's adapted corpus (A1–A4) | this candidate | **9 passed, 0 failed** |

## 10. Validation (fresh; code-and-test tree of checkpoint 1 `5ae506a`)

`checks.json` records exact commands, exit codes and elapsed seconds; each `<name>.txt`
holds the full output.

| Check | Result |
|---|---|
| `cargo fmt --all --check` | exit 0 |
| `cargo test --workspace --all-features --no-fail-fast` | **390 passed, 0 failed, 0 ignored** (374 prior, including 232 inherited Phase-1, plus 16 new) |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| strict core/engine lint (unwrap, expect, panic, indexing, arithmetic denied) | exit 0 |
| `cargo metadata --format-version 1` | exit 0 |
| `git diff --check 3a0b514 HEAD` / `7e3a0aa HEAD` | both exit 0 |
| five Windows/Android `cargo check --workspace --all-targets` | all exit 0 (static only) |
| review corpora | §9 |
| mutation controls | 12/12 baseline pass; 7/7 killed as claimed (§8) |
| preservation | **first run exit 1 from a defect in the new checker** (its fifth protected-function row compared `derived_indexes_consistent`, added by accepted Gate C2 checkpoint `aad4633`, with production, where it does not exist). The corrected check compares it with the review commit (byte-identical) and the four Phase-1 functions with production. Result: exit 0, no failures, 17 inline modules identical, `spark-core` +39/−0 confined to the two `test-support`-gated items, branch starting directly at `3a0b514`. No source changed between the runs; both logs are retained |
| release workload | exit 0; indexed preflight 2.36 / 2.43 / 2.41 µs at 1k / 4k / 16k finalized commands; forbidden scan 5.8 µs → 1.21 ms; whole finalization 1.10 → 14.91 ms; 16k build 137.5 s |

## 11. Operator decision status

No architecture conflict required a stop: §4 shows a representation meeting both frozen
contracts without an encoding change. Two consequences deserve explicit Operator attention
after independent review: the unbilled residual at a parameter-changing barrier, and
non-decay writes not re-phasing the cadence (§4, 1–2). Each is fully determined, tested,
and reversible only through an explicit architecture decision; neither is raised as a
blocking question.

## 12. Limitations (honest scope)

1. AT-I39/AT-I40 evidence is a composition of independent per-profile engines under the
   accepted D-C2-2. There is **no multi-profile runtime**, no shared scheduler, and no
   cross-profile transaction.
2. AT-I13 stays an interpretation: a fixed manifest; **no definition-migration lifecycle**.
3. AT-I21 depth overflow remains reachable only through the test-support depth seam, and
   its discriminators are in-test values rather than compiled mutants.
4. AT-I33 "maximal" means the configured bound on a finite fixture (16 levels, plus a
   `u32::MAX` bound on a shorter chain).
5. Snapshot/restore and replay are in-memory; **no durable storage, process-crash recovery,
   durable mailbox, transport, service or G.A.M.E. integration**.
6. Windows/Android: static `cargo check` only; **no executable platform parity**.
7. Performance: whole finalization remains `O(history)` through the unchanged Phase-1
   `submit_fence`. Pre-wave digests re-hash stores and `cells_of` scans the cell map.
   `decay_walk` now walks the whole retained lineage per evaluation (bounded by
   activations). The workload is a one-machine measurement, not a benchmark (C2-09).
8. The per-wave invariant seam is `O(n log n)` per wave and exists only under
   `test-support`, off by default.
9. No independent review of this revision exists yet.

Next action: independent Codex HIGH review under
`engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_ORACLE_REVISION_INDEPENDENT_REVIEW_MISSION_2026-09-11.md`.
