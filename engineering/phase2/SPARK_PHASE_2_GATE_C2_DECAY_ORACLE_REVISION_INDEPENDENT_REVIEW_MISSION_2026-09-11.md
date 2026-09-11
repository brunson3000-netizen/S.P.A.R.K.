# S.P.A.R.K. Gate C2 — Independent Review Mission for the Second Bounded Correction (Codex HIGH)

**Issued:** 2026-09-11, by the separated correction writer (Claude Code, Opus 5), under the
Operator's Phase-2 authorization. The reviewer is independent of the writer and must not
repair the candidate in the reviewer role.

## Exact scope

| Item | Value |
|---|---|
| Candidate branch | `candidate/phase2-gate-c2-decay-oracle-revision-20260911` |
| Candidate commit | the exact hash in the writer's publication receipt (a commit cannot record its own hash). Its code-and-test tree is checkpoint 1 `5ae506adf13ddf1d610595c59202c52b7ddee81a`; the final commit changes documentation and evidence only |
| Direct base | `3a0b51463e1874ad8b02dd3a3261933fcd2e22f4` (the controlling revision review) |
| Previously reviewed candidate | `5b7fcf50161a65dc83b4806b513f01c0a4943ea5` |
| Production baseline | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` (`phase1-refoundation-v2`) |
| Writer mission | `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_ORACLE_REVISION_MISSION_2026-09-11.md` |
| Writer report | `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_ORACLE_REVISION_REPORT_2026-09-11.md` |
| Writer evidence | `engineering/phase2/gate_c2_decay_oracle_revision_evidence_2026-09-11/` |
| Controlling findings | `engineering/phase2/SPARK_PHASE_2_CODEX_GATE_C2_REVISION_INDEPENDENT_REVIEW_2026-09-11.md` (C2R-01 … C2R-04) |

Verify every hash against live GitHub before starting. Work in a fresh isolated worktree
based exactly on the candidate commit. Preserve every existing branch and worktree.

## Authority

**Authorized:**
- review and fresh validation;
- disposable probes and mutants;
- a review document and bounded evidence;
- a normal commit and push on a new review branch (preferred
  `review/phase2-gate-c2-decay-oracle-revision-independent-20260911`);
- live-ref verification.

**Not authorized:** repairing the candidate, production promotion, Phase 3, G.A.M.E.
changes, merging research runtimes, force-push, reset, paid external compute.

## Precedence

As the controlling review §1: Operator acceptance freeze and Revision-2 independent
review/pins; Revision-2 architecture/oracle; FINAL architecture/oracle; retained V2
sections; freezes and matrices v1 → v2 → v3; the ActiveRequest decision; ADR-0001 …
ADR-0006. Withdrawn Phi, SH-1/SH-2 and architectural R-8 stay withdrawn.

## What to decide

For each finding, decide whether the correction is complete and frozen-conformant, with an
executed discriminator of your own.

| Finding | Writer's claimed correction (verify, do not assume) |
|---|---|
| C2R-01 | No per-effect commit time remains (`Intent::Transform.commit_at`, `Reduced.at`, `WavePlan.commit_times` removed); `apply_wave` writes at the cohort's canonical time; decay reapplication at canonical time reproduces the engine digest |
| C2R-02 | `decay_walk` counts grid points `origin + k·cadence` of the operation's parameter segment, where the origin is the activation of the first epoch with the same resolved `(rate, cadence)`; a changed parameter starts a new segment at its barrier; the closed segment's residual is not billed; an epoch without the operation closes the segment |
| C2R-03 | Every decay evaluation of an existing cell emits and commits at canonical time, whether moved, saturated, at rate 0, or with no whole step |
| C2R-04 | New `phase2_decay_oracle_revision.rs` (16 tests): AT-I8 per-committed-wave observation, AT-I20b rejected-producer retry, AT-I21 value-dependent interference, AT-I28 per-prefix recomputation and injected faults, AT-I33 maximal configured depth, AT-I39/AT-I40 composed per-profile oracle |

**Scrutinize especially the flagged representation R-2′ (report §4) and its stated
consequences:**
- the unbilled residual at a parameter-changing barrier;
- non-decay writes not re-phasing the cadence;
- unmoved decay evaluations being ordinary committed writes (change watchers fire; a
  coincident shock always rejects);
- a step at an activation time belonging to the closing segment.

Decide whether R-2′ satisfies v1 Q7, FINAL §4, AT-I22 and AT-I23 together, or whether one of
its consequences silently narrows a frozen requirement. If so, say which text controls and
whether this is an Operator architecture question.

**Also verify:**
- the four adapted prior writer tests (report §7) are corrections required by C2R-01 …
  C2R-03, not weakenings;
- the new `test-support` seams, including the additive `spark-core` `DerivedIndexFault`
  injector, add no production surface;
- the seven compiled mutants genuinely kill their named wrong implementations, and the two
  prior tests genuinely survive theirs;
- the reference model in `phase2_decay_oracle_revision.rs` is an independent specification
  (brute-force enumeration), not a restatement of the engine's arithmetic;
- the second review's probe `c2_05_recovery_and_unsaturated_remainders_positive` fails only
  on its `LogicalTime(40)` backdated-time assertion (report §9);
- the per-entry table (report §6) does not overstate coverage, in particular the
  per-profile scope of AT-I39/AT-I40, the AT-I21 depth-seam limitation, and the meaning of
  "maximal" in AT-I33;
- no inherited test, manifest, lockfile, inline Phase-1 test module or protected Phase-1
  function changed.

## Required fresh validation

- `cargo fmt --all --check`
- `cargo test --workspace --all-features --no-fail-fast`
- all-target, all-feature clippy with warnings denied
- the strict core/engine lint
- `cargo metadata`
- candidate-range and production-range `git diff --check`
- the five Windows/Android static target checks
- the release workload
- both prior reviews' corpora against the candidate
- the writer's mutation controls
- independent probes and mutants of your own for each finding

## Deliverables

- A review document with a verdict: `GATE_C2_DECAY_ORACLE_REVISION_ACCEPTABLE_FOR_OPERATOR_DECISION`,
  or a bounded-revision or blocking verdict.
- Per-finding and per-entry dispositions.
- Bounded evidence.
- A publication receipt with the review commit hash and local/tracking/live
  synchronization.

No acceptance or promotion follows from the review itself.
