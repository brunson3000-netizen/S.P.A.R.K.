# S.P.A.R.K. Gate C2 second correction — independent review

Date: 2026-09-11. Reviewer: Codex, independent of the correction writer.

**Verdict: `GATE_C2_DECAY_ORACLE_REVISION_BLOCKED_ON_OPERATOR_ARCHITECTURE_DECISION`.**

Candidate: `6968a4af917c9a32761be371c3e12ec12ba0f1bd`,
`candidate/phase2-gate-c2-decay-oracle-revision-20260911`.
Review branch: `review/phase2-gate-c2-decay-oracle-revision-independent-20260911`.
The publication receipt supplies the review commit hash and final synchronization;
this document cannot contain its own commit hash.

The candidate fixes the three demonstrated timestamp/retroactive-rate/saturation
counterexamples and substantially completes the missing oracle fixtures. Fresh workspace
validation passes **390 tests, zero failed or ignored**, including all 232 inherited tests.
Nevertheless, two new independent assertions fail: R-2′ charges a decay step before a
whole cadence has elapsed after a fresh assignment or separate shock. Its activation grid
is a coherent proposed policy, but it replaces a retained frozen formula. The writer's
claim that there is no architecture conflict is not accepted. No candidate repair,
production promotion, or Phase-3 work is part of this review.

## 1. Mission, custody, and controlling authority

Executed `SPARK_PHASE_2_GATE_C2_DECAY_ORACLE_REVISION_INDEPENDENT_REVIEW_MISSION_2026-09-11.md`
under the Operator's authorization for review, validation, evidence, branch creation,
commit, normal push, and live verification. No additional authorization was requested.
No paid compute, research-runtime merge, GAME change, force-push, reset, or discarded
pre-existing work occurred. Review probes and mutants ran in disposable external crates
and archives; candidate code and tests were never edited by this reviewer.

Fetched origin with pruning. Live GitHub refs matched the following local/tracking pins
before review; `custody.json` captures a fresh verification and all worktree states.

| Role | Exact hash |
|---|---|
| Second correction | `6968a4af917c9a32761be371c3e12ec12ba0f1bd` |
| Controlling revision review / direct base of correction series | `3a0b51463e1874ad8b02dd3a3261933fcd2e22f4` |
| Previously reviewed candidate | `5b7fcf50161a65dc83b4806b513f01c0a4943ea5` |
| Production `phase1-refoundation-v2` | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` |
| Original Gate C2 candidate | `053d1dc1131ec47be94b60513fad9ea8389cde0c` |
| Original Gate C2 review | `00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0` |
| Unmerged research | `03d36dd649641b28b232f4650b773413db634611` |

Fetched checkpoint 1, `5ae506adf13ddf1d610595c59202c52b7ddee81a`, directly from origin;
verified ancestry and byte-equal code/test/manifests/lockfile tree against the exact final
candidate. The series is `3a0b514` → `8844264` → `5ae506a` → `00221d8` → `6968a4a`.
Created the new isolated worktree `/tmp/spark-gate-c2-decay-oracle-review` directly at the
exact candidate. All twelve pre-existing worktrees and branches were preserved. The
original checkout remains on `review/v3-f01-final-20260910`. No applicable AGENTS.md,
CLAUDE.md, or CODEX.md was found in the repository or checked ancestor locations.

Precedence remains the Operator acceptance freeze and Revision-2 independent review/pins;
Revision-2 architecture/oracle; FINAL architecture/oracle; retained V2 sections;
freezes and matrices v1 → v2 → v3 with corrected Opus provenance; the ActiveRequest
decision; ADR-0001–0006. The current Phase-2 authorization supersedes historical lack of
implementation authorization, but does not itself amend a frozen semantic contract.
Phi, SH-1/SH-2, and architectural R-8 remain withdrawn.

Evidence directory (**E**):
`gate_c2_decay_oracle_revision_independent_review_evidence_2026-09-11/`.
**D** = `crates/spark-testkit/tests/phase2_decay_oracle_revision.rs`;
**B/O** = the bounded-revision/oracle-completion test files. Other inherited test-file
names below refer to `crates/spark-testkit/tests/phase2_*.rs`. Source line numbers refer
to this exact candidate, not the previous review tree.

## 2. Blocking finding C2R2-01: R-2′ changes the retained elapsed-time formula

**Severity: MAJOR architecture-conformance blocker. Continues C2-05 and AT-I22.**

`spark-engine/src/engine.rs:647–683` resolves parameter-segment origins, then computes
`floor((end-origin)/cadence) - floor((from-origin)/cadence)` where `from` includes the
cell's `updated_at`. This counts grid endpoints inside the interval. It does not count
whole elapsed cadence durations since an arbitrary non-decay write.

The independent external probe `r2prime_frozen_q7_full_cadence_after_fresh_assignment`
uses public commands/scheduled work, a fixed epoch, and no earlier decay:

1. Cadence 10, rate 10, baseline 0; assign 100 at canonical time 9.
2. Evaluate decay at canonical time 10.
3. `updated_at=9`, elapsed duration 1, hence zero whole cadence steps under Q7.
4. Candidate commits **(90, 10)**; frozen-duration expectation is **(100, 10)**.

The independent `r2prime_frozen_q7_full_cadence_after_separate_shock` also reconstructs
the contested sanctioned composition without copying its writer test: decay at 30 gives
70; a separate scheduled +5 shock at 31 gives (75,31); decay at 40 gives **(65,40)**,
although only nine time units have elapsed. Q7's duration reading gives **(75,40)**.
Both are executed assertion failures, not compiler errors. A separate observational
positive test pins the candidate's actual results, ruling out a mistaken setup.

Controlling text:

- v1 architecture **Q7, lines 261–267** freezes a closed-form function of current value,
  baseline, rate, and “whole elapsed cadence steps between `updated_at` and now.”
- v2 architecture **§4.3, lines 199–204** expressly retains that closed-form formula,
  rounding, and chunk invariance, including compositions separated by scheduled boundaries.
- FINAL architecture **§4, lines 122–125** makes every inherited `now`/`at` the cohort's
  canonical time. It fixes the timestamp, not the cadence origin.
- FINAL oracle **§1, lines 54–55** retains AT-I22 in substance under the clarified `now`
  and leaves AT-I23 unchanged. No accepted Revision-2 delta replaces the Q7 formula.

The fresh-assignment counterexample needs no arbitrary chunking interpretation, no
fractional-rate policy, no hot tuning, no saturation, and no timestamp backdating. It
shows that the proposed grid charges a whole step after less than a cadence of elapsed
cell time. “An endpoint lies between the two times” and “a whole cadence elapsed between
the two times” are different numeric contracts.

The writer establishes additivity **after choosing its grid**. It does not establish
that activation lineage is the uniquely sanctioned origin. An encoding constraint and
a need to preserve remainder do not authorize replacing an explicit formula. Nor does
this review prove that no alternative representation can satisfy every frozen constraint.
The writer mission's stop rule applies to an unresolved contract reconciliation: either
demonstrate a conforming representation or obtain an explicit architecture amendment.
Simply changing timestamps back, or reverting to `(now-updated_at)/cadence` on every
evaluation, would revive independently demonstrated failures and is not prescribed here.

### Each flagged R-2′ consequence

| Consequence | Independent disposition |
|---|---|
| Unbilled residual at a parameter-changing barrier | **Observed, explicit policy requiring inclusion in adjudication.** Own rate-2/cadence-7 → rate-5/cadence-4 barrier-20 fixture charges old steps 7/14, drops (14,20], charges no new step at 23, and first new step at 24; both signs pass. Whole-step-only arithmetic does not demand prorating. AT-I23 forbids retroactive use of the new rate, but does not uniquely choose this segment reset/loss policy. The statement that billing old-rate time later necessarily applies the old rate after its barrier is too broad: historical catch-up already integrates closed epochs. No separate fractional-charge violation is alleged. |
| Non-decay writes do not restart cadence | **Reject as frozen-conformant.** The two public-door counterexamples above directly discriminate the changed Q7 meaning. This is more than a storage representation choice. |
| Unmoved decay evaluations become committed writes | **Accept the local correction and its disclosed consequences.** Saturated, rate-zero, and no-step evaluations commit at canonical time; watchers fire, and same-wave shocks reject atomically. Own probes test all these cases. This follows ordinary write/wave semantics, closes the demonstrated timestamp partition defect, and does not itself authorize the grid. |
| A step ending exactly at activation belongs to the closing segment | **Accept.** Own barrier-21 fixture: old rate 2/cadence 7 owns steps 7,14,21; new rate 5/cadence 4 first owns 25. Catch-up without evaluation at 21 and split evaluation at 21 both reach (±89,25). A mutant dropping the closing endpoint is killed. Scheduled work at activation time runs before the activating command. |

**R-2′ overall: REJECT as an assertion of existing frozen conformance; explicit Operator
architecture decision required before accepting this policy.** Remainder preservation,
canonical commits, hot-tuning boundaries, and post-write phase must be resolved together.

## 3. C2R-01 through C2R-04 and retained dispositions

| Finding | Disposition on this exact candidate | Independent executed evidence |
|---|---|---|
| C2R-01 | **Demonstrated backdated-commit defect corrected; full reconciliation blocked by C2R2-01.** Per-effect time fields are removed and `apply_wave` writes at canonical `now`. | Prior canonical reapplication probe passes. Own recovery sweep at 1/6/7/11/23/60 reapplies reported effects to an untouched clone with exact digest equality; own next-tick-write mutant is killed. |
| C2R-02 | **Demonstrated pre-barrier new-rate charge corrected.** Segment policy is numerically consistent, but its policy status remains part of R-2′ adjudication. | Prior shortening probe now gives 95; D shortening/lengthening/rate-only/unchanged/zero/absent/replay controls pass. Own closing-endpoint/residual fixtures pass and endpoint-drop mutant is killed. |
| C2R-03 | **Demonstrated saturation timestamp defect corrected.** No independent unmoved-write defect found. Full AT-I22 conformance still depends on C2R2-01. | Prior saturated probe now gives (0,150) both ways. Own both-sign saturation partitions, watcher/shock controls, and zero-rate omission mutant discriminate the correction. |
| C2R-04 | **Non-decay gaps completed with the limits below; remains open through AT-I22 and overstated R-2′ conformance.** | D AT-I8/20b/21/28/33/39/40 pass; own per-wave/index probe passes and later-wave record-drop mutant is killed. AT-I23 has meaningful nonretroactivity support. Neither a green suite nor the new model closes C2R2-01. |

The previously accepted dispositions remain intact: C2-01/02/03/04/06/08 closed;
D-C2-5/7/11/13 revised and accepted; D-C2-1/2/4/6/8/9/10/12/14/15 preserved;
R-1 and R-3 through R-9 accepted. The latter R-8 is enqueue canonicalization, not the
withdrawn architectural R-8. Fresh suite and both prior corpora retain their discriminators.
C2-05 remains open on C2R2-01; C2-07 remains open on AT-I22/conformance evidence. C2-09's
performance limitation remains a disclosed limit, not an invented production guarantee.

## 4. Test adaptations, seams, reference model, and scope claims

| Adapted prior writer test | Disposition |
|---|---|
| B `c2_05_chunk_invariance_holds_for_every_evaluation_subset` | **Timestamp adaptation justified.** (90,10) becomes (90,15), then (80,20); the existing subset sweep remains. Does not independently authorize the segment policy. |
| B `c2_05_epoch_bound_rates_compose_across_the_barrier` | **Supported correction under the proposed segment policy.** Unaligned barrier 55 now gives 95 at 60 and 90 at 65; aligned-barrier rows remain. Avoiding new-rate pre-barrier charge is required; resetting the segment and dropping its residual is not uniquely entailed. Carry this expectation into the architecture decision rather than label it unconditionally frozen. |
| O `at_i22_sanctioned_compositions_and_cross_family_rejection` | **Not accepted as merely a necessary correction.** Changing 75 to 65 at time 40 removes the duration-based post-shock assertion. Added timestamp assertions do not compensate for replacing the retained numeric contract. This is the substantive weakening/redefinition identified by C2R2-01. |
| stores/semantics `at_i22_decay_catch_up_is_chunk_invariant` early case | **Unmoved-commit adaptation justified locally.** It now checks the ordinary write at 9 and the retained step at 10. Its chosen grid remains subject to R-2′; the no-candidate assertion was not a Phase-1 invariant. |

The additive core `DerivedIndexFault` enum and injector are gated by
`cfg(any(test, feature="test-support"))`; the engine's per-wave observation and fixtures
are likewise gated. No default-feature production surface was added. Independent external
surface controls pass **17/17**, including missing core fault type/method and missing wave
observation type, the prior report-field refusals, fixture privacy, and a positive public
matching control. Workspace dependency/feature hygiene remains unchanged and passes.
An enabled test-support feature intentionally exposes test seams; this is not a claim
that an explicit all-features build hides them.

D's model (`model`, lines 278 onward) enumerates step endpoints and applies one step at a
time; the engine uses checked quotient differences and closed-form movement. Thus it is
an independent **arithmetic implementation of R-2′**, rather than a call into engine
arithmetic. It copies the same parameter-segment premise, however, and cannot independently
establish that premise as the frozen specification. The forty generated timelines seed at
zero, then change parameters/evaluation times; they do not exercise a fresh unaligned
non-decay write or a later shock. They compare each generated partition to a single
catch-up, with pacing/replay/restore controls, not every possible partition of every
timeline. The separate saturation fixtures enumerate 32 subsets for each sign. None is
a universal proof. Those distinctions explain why the grid model and both new contract
failures can all be real.

The expanded oracle fixtures are otherwise meaningful:

- **AT-I8:** observations occur after `apply_wave`, including intermediate waves; counts
  2/3/3 and the bidirectional invariant are asserted. A deferred-insertion mutant survives
  the prior whole-cohort test but fails the new one. Own two-wave counts 1/2 also discriminate.
- **AT-I20b:** all three caps reject, compare post-extraction engine digest and the five
  non-consumption components, consume keys, replay, and retry the same producer/scope/kind
  at occurrence n+1. The producer is the WorkKey creator (`rule.arm`), whose obligation
  executes `rule.wide`; this is the frozen producer identity, not retry via an unrelated rule.
  The producer-tombstone mutant survives the old test and fails the new one.
- **AT-I21:** echo 5 versus 12 discriminates a shared snapshot with same-time interference;
  echo 20 versus 5 discriminates an intervening command. Overflow is still reached via the
  test-support depth seam because static validation prevents it. No public-door overflowing
  rule set or compiled AT-I21 mutant is claimed.
- **AT-I28:** ten prefixes recompute/restore, including pauses, refusals and resets. Three
  index faults are injected at the **nine prefixes with finalized commands**; genesis has
  positive empty-index checks and lineage faults, not three absent-key deletions. The report's
  “every prefix” fault wording needs that qualification. This does not leave an applicable
  nonempty-history corruption case untested in this script.
- **AT-I33:** waves 0–16 at configured maximum 16 use two parents per level and exact
  formulas; one extra level is refused statically and converted through the seam. The
  `u32::MAX` configured bound runs a finite chain; it is not billions of executed levels.
- **AT-I39/40:** a composition of separate per-profile engines compares reports, identities,
  obligations, batch and boundary digests with/without a third profile, reversed order,
  due-time interleaving, and pacing 1. This supplies the missing companion under accepted
  D-C2-2. It does not implement or validate a shared multi-profile engine/scheduler or
  cross-profile transaction. Foreign commands and foreign-only target definitions refuse.

## 5. Every oracle disposition

**S** = meaningful finite executed support, with retained scope qualifications; not universal
proof, acceptance, or a waiver. **P** = partial lifecycle coverage. **R** = demonstrated
frozen-contract failure. Inherited S rows were rerun in the fresh workspace suite and
reviewed against this bounded source diff; earlier review qualifications remain in force.

| Entry | Disposition | Evidence / limit |
|---|---|---|
| AT-I1 | S | Additive untouched-clone and D canonical decay reapplication; own recovery reapplication. |
| AT-I2 | S | Compile boundary and later-wave snapshot/retained-commit fixtures. |
| AT-I3 | S | O declaration/insertion permutations across reducer families. |
| AT-I4 | S | All six WorkKey fields affect identity; engine formula fixtures. |
| AT-I5 | S | Renamed/reordered declarations preserve results/rejection evidence. |
| AT-I6a | S | Exact folds and contested identities; enqueue redelivery controls. |
| AT-I6b | S | Distinct equal +10 causes retained, not coincidence-deduplicated. |
| AT-I6c | S | Full trigger/changed-read parent union, fixed/multiple scopes and depth; prior probe passes. |
| AT-I6d | S | Complete-set parent identity rather than minimum/representative. |
| AT-I7a | S | Full additive fold and construction-order controls. |
| AT-I7b | S | Unequal RESULT rejection with pre-wave digest equality. |
| AT-I7c | S | RESULT versus TRANSFORM multiplicity discriminated. |
| AT-I7d | S | Incompatible family table and atomic rejection. |
| AT-I7e | S | Checked three-term i128 overflow and atomicity. |
| AT-I8 | S | New genuine per-wave observations; two writer old/new controls and own mutant. |
| AT-I9 | S | Repeated unequal-result reports/digests. |
| AT-I10 | S | Host-owned target refusal and preserved lineage. |
| AT-I11 | S | Bounds/type/authority and atomic companion effects. |
| AT-I12 | S | Scheduler/store payload agreement across schedule/collision/drain. |
| AT-I13 | P | Accepted fixed-manifest scope; no definition-migration lifecycle. Retained debt, not newly completed. |
| AT-I14 | S | Origin artifact/config and current-rule compatibility controls; prior probe passes. |
| AT-I15 | S | Contested derived obligation records under one rule set. |
| AT-I16 | S | Ascending emission identity and pacing-independent claims. |
| AT-I17 | S | Qualified nested sub-ID collision fixtures. |
| AT-I18 | S | Direct/mutual/long change cycles rejected; delayed variants admitted. |
| AT-I19 | S | Scope breadth/materialized fan-out and exact/over-bound controls. |
| AT-I20 | S | Whole-cohort pacing and canonical equality on common histories. |
| AT-I20b | S | D three-cap atomic terminal refusal, replay, same-producer next-occurrence retry. |
| AT-I20c | S | Mixed oversized cohort, co-target, and threshold inputs. |
| AT-I20d | S | Evaluator cannot reach pacing diagnostics; external surfaces checked. |
| AT-I21 | S | New value-dependent same-time/intervening-command controls; depth-seam limitation retained. |
| AT-I22 | R | Grid chunk/saturation tests pass but independent Q7 fresh-write/shock assertions fail; C2R2-01. |
| AT-I23 | S, policy qualified | Shortening/lengthening/rate-only/zero/absent/replay and endpoint controls support nonretroactivity. Residual/reset policy still requires R-2′ adjudication. |
| AT-I24 | S | Widened aggregates, i128 bounds/cancellation and permutations; no cached aggregate path claimed. |
| AT-I25 | S | Fully reduced threshold crossing and complete changed-read parent union. |
| AT-I26 | S | Each retained store changes its component and engine digest. |
| AT-I27 | S | Cooldown/occurrence permutations, activation/reset and contested caps. |
| AT-I28 | S | D per-prefix recomputation/restore, nonempty-history index faults and lineage tampering; own injection. |
| AT-I29 | S | Stable-boundary snapshot/resume; inter-wave public entry forbidden. |
| AT-I30 | S | Dormant-scope logical-work invariance; not a bound on physical map scans. |
| AT-I31 | S | Aggregate cadence and promotion reading committed aggregate. |
| AT-I32 | S | Strict lint, candidate compile probes, own 17 default-feature surface controls. |
| AT-I33 | S | New maximal configured finite depth, complete two-parent formulas and typed overflow seam. |
| AT-I34 | S | All 232 inherited tests byte-preserved and included in 390 passing tests. |
| AT-I35 | S | None/Advance/Command golden encodings and independent boundary reference. |
| AT-I36 | S | All five Windows/Android static targets; no runtime parity claim. |
| AT-I37 | S | Discriminating static/runtime same-target family cases. |
| AT-I38 | S | Cause beyond presentation truncation affects complete identity. |
| AT-I39 | S, per-profile | O batch recomputation plus new composed-profile independence companion. |
| AT-I40 | S, per-profile | Distinct profile cohorts/own membership, third-profile independence and cross-profile refusals. |
| AT-I41 | S | Per-cohort effect_batch_v3 recomputation and cohort-local wave index. |
| AT-I42 | S | Core surface stale/nonleast/replay/cap/refinement/store mismatch fixtures. |
| AT-I43 | S | Request lifecycle/start/resume/substitution/time/finalization/overflow; Q7 numeric issue remains AT-I22. |
| AT-I44 | S | Mixed slice conflicts first, full removal and post-extraction digest. |
| AT-I45 | S | Budget-2 discriminator, zero-cost conflicts after oversized admission. |
| AT-I46 | S | Frozen report field pin, forbidden fields and feature-only observation seams. |
| AT-I47 | S | ActiveRequest/F distinctions, restore/resume and mismatch cleanup. |
| AT-I48 | S | Full-state refusal, 360 differentials, follow-on retention and sticky fail-stop. |
| AT-I49 | S | Wrong/non-head/active-head/dequeue durability-order controls; no real durable mailbox claim. |
| AT-I50 | S | Full command payload changes request identity under same envelope fields. |

Original unsplit AT-I6 and AT-I7 forms remain superseded by their lettered rows.
AT-I13 remains the sole P row; AT-I22 is R. No S label above overrides the R-2′ blocker.

## 6. Fresh validation and independent evidence

All required checks ran against the exact candidate in the isolated worktree. `checks.json`
contains commands, return codes, and durations; each log retains full normalized output.
The release workload's recorded measurements are in `workload.txt`.

| Check | Result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo test --workspace --all-features --no-fail-fast` | 390 passed, 0 failed, 0 ignored |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| Strict core/engine all-feature library lint | PASS; warnings, unwrap, expect, panic, indexing/slicing, arithmetic-side-effects denied |
| `cargo metadata --format-version 1` | PASS |
| Candidate-range `git diff --check 3a0b514 HEAD` | PASS on pinned candidate |
| Production-range `git diff --check 7e3a0aa HEAD` | PASS on pinned candidate |
| Windows x86_64 GNU / MSVC all-target static checks | Both PASS |
| Android aarch64 / armv7 / x86_64 all-target static checks | All PASS |
| Offline release `gate_c2_workload` | PASS; finite measurements below and in `workload.txt` / `checks.json` |
| Mechanical preservation | PASS: seven inherited integration files, 17 inline Phase-1 modules, manifests/lockfile and protected functions |
| First review's original byte-unchanged corpus | Expected compile failure at the same three revised surfaces: refusal construction, cohort report field, decay shape/Params |
| First review's adapted byte-unchanged corpus | 9/9 PASS |
| Second review's byte-unchanged runtime corpus | 13 pass, 1 fails only at obsolete `LogicalTime(40)` versus canonical 42 |
| Second review corpus with only that time literal changed | 14/14 PASS; both ±45 values, remainder equality and later baseline checks execute |
| Own independent runtime probes | 7 pass, 2 genuine Q7 contract assertions fail |
| Writer mutation controls rerun freshly | 12/12 baselines pass; all 7 compiled mutants killed; both named prior tests survive their mutants |
| Own compiled mutants | 4/4 baselines pass; all 4 killed by executed assertions, one per C2R finding |
| Own default-feature surface controls | 17/17 expectations and diagnostics satisfied |

| Finalized history | History build (s) | Indexed preflight (µs/call) | Scan control (µs/call) | Whole finalization (ms/call) |
|---:|---:|---:|---:|---:|
| 1,000 | 0.690 | 2.434 | 5.567 | 1.175 |
| 4,000 | 14.004 | 4.768 | 150.883 | 5.811 |
| 16,000 | 129.661 | 2.456 | 1,230.374 | 15.360 |

These are finite one-machine measurements under contemporaneous build load, not a
production latency guarantee. Indexed preflight avoids the history scan; successful
finalization still hashes history through unchanged Phase-1 code. Pre-wave digests scan
stores and decay traverses retained lineage. The 4k timing fluctuation is retained.

`run_corpus.py` is a capture wrapper that returns zero even when cargo returns 101;
the JSON line/log's cargo result controls the disposition. No wrapper success is counted
as semantic success. The prior review source is preserved; its only adapted copy is in
E and differs by exactly `LogicalTime(40)` → `LogicalTime(42)`. The unchanged corpus fails
on the negative-sign iteration first; the adapted execution is what also proves the
positive-sign iteration and subsequent assertions pass. This confirms report §9 precisely.

Writer mutants really compile and exercise their named wrong implementations: backdated
commits, relative elapsed steps, absolute grid, resetting every activation, skipping unmoved
decay, deferring obligation insertion, and tombstoning the rejected producer. Own independent
mutants instead advance commit time one tick, drop the closing activation endpoint, omit
rate-zero decay, and omit later-wave obligation records. Positive baselines precede all
kills. These support particular discriminators, not completeness of the architecture.

Initial concurrent default-debug builds exhausted temporary disk storage. Their captured
logs are preserved in `storage-exhaustion-attempt/`; the validation process was stopped
and only this review's generated target directory and disposable mutant archive were
removed. Every required check was restarted with `CARGO_PROFILE_DEV_DEBUG=0`,
`CARGO_PROFILE_TEST_DEBUG=0`, `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=2` (no source,
test, manifest, assertion, or optimization-level changes). A surface-runner diagnostic
matcher initially expected “could not find” instead of rustc's “cannot find type” for
two new negatives. Both were correctly hidden already; the reviewer matcher was corrected
and all 17 cases rerun. Those first outputs are retained separately. Neither infrastructure
failure nor matcher mistake is relabeled as candidate evidence.

The additive core diff is +39/−0 confined to the test-support enum/injector. `schedule`,
`drain_due`, `stage`, and `submit_fence` are byte-identical to production;
`derived_indexes_consistent` is byte-identical to the controlling review (it was introduced
after production). Candidate code/test tree equals checkpoint 1. No inherited test was
removed, ignored, or weakened. All review changes are this report and bounded evidence.

## 7. Blockers, publication, and exact next action

**Remaining Gate C2 blockers:** C2R2-01 / R-2′, keeping C2-05 and the AT-I22 portion of
C2-07/C2R-04 open. The old timestamp, shortened-cadence retroactivity, and saturation
counterexamples are corrected; they must not be reintroduced. AT-I13 is retained
fixed-manifest lifecycle debt, not newly completed work. Per-profile composition, finite
depth, test-support overflow, in-memory replay/restore, static-only platforms, and physical
scan/performance limits remain explicit. No durable persistence, crash-recovery service,
mailbox, transport, GAME integration, or Phase 3 is supplied.

Publish only this review/evidence on the named review branch with a normal commit/push;
verify local HEAD = tracking ref = live GitHub ref and recheck candidate, prior review,
production, research, and all pre-existing branch/worktree states. The final receipt
records the exact review commit and synchronization; publication itself is not acceptance.

**Exact next action:** present this pinned candidate and published review for an explicit
Operator architecture decision on the cadence origin after non-decay writes and the
residual/segment policy at parameter-changing barriers, while retaining canonical-time
commits and the corrected nonretroactivity/saturation behavior. If the grid is chosen,
amend Q7, retained v2 §4.3 and AT-I22/23 explicitly with discriminating fresh-write,
post-shock and barrier vectors; do not call that choice already frozen. If the duration
contract is retained, have the separated writer propose a conforming remainder/phase
representation within the encoding constraints or state the precise required amendment.
Then publish one new exact candidate for independent review. This reviewer makes no repair
or production advancement and does not begin Phase 3.
