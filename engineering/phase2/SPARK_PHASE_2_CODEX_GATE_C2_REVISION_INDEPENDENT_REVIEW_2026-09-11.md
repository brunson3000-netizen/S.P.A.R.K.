# S.P.A.R.K. Gate C2 bounded revision — independent review

Date: 2026-09-11. Reviewer: Codex, independent of the correction writer.

**Verdict: `GATE_C2_REVISION_BOUNDED_REVISION_REQUIRED`.**

Exact candidate: `5b7fcf50161a65dc83b4806b513f01c0a4943ea5`, branch
`candidate/phase2-gate-c2-bounded-revision-20260911`.
Review branch: `review/phase2-gate-c2-revision-independent-20260911`, based directly
on that candidate in `/tmp/spark-gate-c2-revision-review`.
The final publication receipt gives the review commit without embedding a self-reference.

The revision closes C2-01, C2-02, C2-03, C2-04, C2-06 and C2-08, and supplies meaningful
corrections for D-C2-7, D-C2-11 and D-C2-13. C2-05 and C2-07 remain open. All 374 candidate
workspace tests pass, but **three independent contract assertions fail** against unchanged
candidate sources. Decay backdates committed cells contrary to the controlling cohort-time
contract, shortening cadence can charge new rates before activation, and saturation breaks
the claimed value-and-commit-time chunk invariance. The oracle report also overstates some
new coverage and retains applicable uncompleted cases. This verdict does not accept the
candidate, promote production, or authorize Phase 3.

## 1. Authority, exact custody and precedence

Executed the canonical
`SPARK_PHASE_2_GATE_C2_REVISION_INDEPENDENT_REVIEW_MISSION_2026-09-11.md` under the
Operator's explicit authorization for routine review, validation, evidence, branch creation,
commit, normal push and live verification. No candidate repair, production promotion,
Phase-3 work, GAME change, research-runtime merge, reset, force-push or paid compute occurred.
Disposable mutations were wrong-implementation probes in a temporary archive, not repairs.

Fetched origin with pruning before review. Live GitHub, local and tracking refs agreed on:

| Ref / role | Exact hash |
|---|---|
| Corrected candidate | `5b7fcf50161a65dc83b4806b513f01c0a4943ea5` |
| Controlling prior review | `00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0` |
| Original candidate | `053d1dc1131ec47be94b60513fad9ea8389cde0c` |
| Production `phase1-refoundation-v2` | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` |
| Unmerged Fable research | `03d36dd649641b28b232f4650b773413db634611` |

The fetched candidate's chain is `00d647e` → `b12ac9a` → `99f8774` → `19930f3` →
`5b7fcf5`. The code/test/manifests/lockfile tree is identical to checkpoint
`19930f3c17a923c873b55cd5afb58692589d0681`; the final candidate commit changes only
engineering documentation/evidence. The review worktree started exactly at the pinned
candidate. All ten pre-existing worktrees and branches were preserved; custody capture
records their states. No applicable ancestor/repository AGENTS.md, CLAUDE.md or CODEX.md
was found. The initial checkout remains on `review/v3-f01-final-20260910`.

Applied precedence: Operator acceptance freeze and Revision-2 independent review/pins;
Revision-2 architecture/oracle; FINAL architecture/oracle; retained V2 sections;
freezes/matrices v1, v2, v3 with corrected Opus provenance; ActiveRequest decision;
ADR-0001–0006. The present Operator authorization supersedes historical Phase-2
nonauthorization statements. Phi, SH-1/SH-2 and the withdrawn architectural R-8 stay
withdrawn; the writer's new label **R-8 enqueue canonicalization is a different item**.

Evidence is in `gate_c2_revision_independent_review_evidence_2026-09-11/` (below, **E**).
Source anchors below are candidate line numbers. **B** means
`crates/spark-testkit/tests/phase2_bounded_revision.rs`; **O** means
`phase2_oracle_completion.rs`; **X** means `phase2_compile_probes.rs`.

## 2. Remaining findings

### C2R-01 — MAJOR: the remainder representation changes frozen commit time

**Continues C2-05; rejects R-2 as frozen conformance.**

`engine.rs:880–904` supplies `commit_at` from the last whole decay step;
`engine.rs:2015` chooses that instead of `now`; `engine.rs:2323–2340` writes the cell
at this earlier time. For value 100 at time 0, rate 10, cadence 10 and a cohort due at
15, the candidate commits value 90 with **updated_at=10**, while its report identifies
canonical_time=15.

FINAL architecture §4 is controlling: “Every inherited use of `now`, `at`, and
"current logical time" during that cohort's evaluation denotes this value” (the scheduled
cohort's due_time or command's effective_time). v1 Q3 and v2 §3.1 retain an effect's `at`
logical time. FINAL oracle §1 explicitly retains AT-I22 in substance under that clarified
`now`; it does not introduce a separate backdated effect timestamp. The earlier Q7
remainder obligation cannot silently replace this later time contract.

The independently executed
`c2_05_effect_reapplication_at_frozen_cohort_time_must_reproduce_state` starts with two
identical engines, runs the decay cohort on one, extracts the same work on the untouched
clone and applies the reported resolved effects at canonical time 15. Both values are 90;
timestamps are 10 versus 15 and engine digests differ. This extends the writer's AT-I1
untouched-clone method to the changed decay path. The existing O:260 test covers additive
writes, so it cannot detect this discrepancy. The private commit-time vector is not present
in the committed-effect report; a consumer following the frozen time convention cannot
reapply it exactly.

Required disposition: supply a remainder representation satisfying both frozen contracts,
or identify the exact architecture conflict for explicit adjudication before claiming
conformance. Merely restoring `commit_at=now` would revive the original lost-remainder
counterexample; this review prescribes no such partial repair.

### C2R-02 — MAJOR: shortened cadence applies the new rate before its barrier

**Continues C2-05 / AT-I23; independently fails even under R-2's proposed step-end rule.**

`engine.rs:612–655` carries the old interval's unconsumed `t` into the next epoch and
computes `(end-t)/new_cadence` without excluding step ends at or before that epoch's
activation. Public-door fixture: value 100 at 0; old rate 1/cadence 100; activate rate
5/cadence 10 at 50; first decay at 60. The result is **70**, counting six new-rate steps.
Under the writer's own rule that each step takes the epoch in effect at its end and a
step exactly at activation belongs to the old epoch, only the step ending at 60 may take
rate 5: expected **95**. The new rate was charged for ends 10, 20, 30, 40 and 50, when
it was not active. This is not a choice about fractional prorating.

`c2_05_shorter_cadence_must_not_bill_steps_ending_before_activation` executes this fixture
with a bit-identical rule across epochs and two validated config parameters. The failure
therefore does not depend on superseded-rule handling. Q7 explicitly allows hot-tunable
cadence as well as rate; AT-I23 requires the new rate to apply only from its barrier.
B:827/843 exercise rate changes at **fixed cadence 10**, leaving this defect alive.

Required correction: define and enforce epoch boundaries when cadence changes, including
shortening, lengthening and residual intervals, with recovery and replay controls.

### C2R-03 — MODERATE: saturation invalidates the generalized chunk claim

**Continues C2-05 / AT-I22 and the coverage portion of C2-07.**

The single-decay branch at `engine.rs:891–904` emits nothing when the value is already at
baseline. A one-shot evaluation at 150 from 100, baseline 0, cadence 10, rate 10 yields
`(value=0, updated_at=150)`. Evaluations at 50, 100 and 150 yield
`(value=0, updated_at=100)`. The independent
`c2_05_saturation_must_preserve_claimed_value_and_commit_time_chunk_invariance` asserts
both correct zero values before failing their cell-time equality.

The report and B:730 claim value **and commit-time** invariance over arbitrary evaluation
times, but its 32-subset sweep stops before saturation (100→60). Q7 and AT-I22 explicitly
include convergence at baseline and exact chunk equality. The claim is not established by
an unsaturated sweep. This probe does **not** compare full engine digests of histories with
different WorkKeys/occurrence histories, which would be an invalid equality claim; it
isolates the cell tuple the writer expressly claims invariant. Resolve the timing policy
together with C2R-01 and add saturated decay/recovery and zero-rate transitions.

### C2R-04 — MAJOR gate-evidence gap: C2-07 is improved, not complete

The forbidden report field and extra diagnostic field are removed; the new compile
boundaries work; the red-first correction is candid. Nevertheless, the writer mission
requires completing applicable P/R cases, and disclosure alone does not waive them.
Concrete remaining overstatements:

- **AT-I8:** O:849 is named/described as checking the invariant after every committed wave,
  but calls `bidirectional_invariant_holds` before and after the **whole two-wave cohort**,
  then after a later cohort. It never observes the intermediate committed wave. The
  touched-key full-commitment preflight test at O:777 is meaningful and kills the
  shape-only mutant; this does not turn the other test into per-wave instrumentation.
- **AT-I20b:** O:1000 correctly exercises effect/enqueue cap refusals and deterministic
  replay, but its “retry” runs `rule.small` via `rule.retry`, writing `state.echo`.
  It does not reschedule the rejected producer under its next occurrence as the frozen
  row requires. An implementation that permanently disables the overloaded producer
  could pass this test. Its atomicity check compares cell state, not all retained
  components for these new cap cases (other AT-I8 fixtures cover other failure paths).
- **AT-I21:** O:1141 orders the interfering key and continuation correctly and checks
  identities, but `rule.c` emits constant `+1` to echo without reading the changed stress
  value in its effect/condition. It is not a discriminator for reading a deferred input's
  old versus current value. The documented seam limitation is honest; a value-dependent
  interference fixture remains applicable.
- **AT-I22/23:** the claimed S statuses are defeated by C2R-01–03; O's two sanctioned
  composition tests pass but do not cover these cases.
- **AT-I28/33/39/40:** the report itself retains timeline-index injection, maximal-depth
  chain and cross-profile companion gaps. Those remain partial. Fixed-manifest AT-I13
  and per-profile architecture are accepted scope interpretations, not completed migration
  or multi-profile oracle evidence.

The per-entry table below distinguishes meaningful finite support from outstanding cases.
No historical checkpoint or inherited test should be rewritten to manufacture red-first
provenance, and no forbidden production API should be added merely to simplify a probe.

## 3. Disposition of every prior finding

| Prior ID | Independent disposition | Executed discriminator / limit |
|---|---|---|
| C2-01 | **Closed** | Trigger + changed condition-read union, deduplicated; B background/multi-target/depth tests and own fixed-scope repeated-read formula pass. Watched-only mutant is killed. |
| C2-02 | **Closed** | Distinct declared target/scope pairs plus delay edges; each materialized expansion separately bounded. Own 2/3 target scopes at bound 2 pass/refuse; definition-only mutant killed. |
| C2-03 | **Closed** | Nested operations claim the same qualified namespace as top-level effects, schedules and gates. Equal/unequal payload tests plus own cross-schedule collision pass; unclaimed-nested mutant killed. |
| C2-04 | **Closed** | Aggregate and composed intermediates stay checked i128 to their final conversion. MAX and MIN half-weight cases, overflow and staged cancellation pass; premature aggregate narrowing mutant killed. |
| C2-05 | **Open — C2R-01/02/03** | Baseline door and field targeting accepted; original remainder and fixed-cadence epoch-rate counterexamples now pass. New temporal/cadence/saturation assertions fail. |
| C2-06 | **Closed** | Kind/payload bound both ways; rejected activation finalizes normally without artifact/ingress changes. Own two-direction probe passes; payload-only activation mutant killed. |
| C2-07 | **Open — C2R-04** | Forbidden surfaces removed, claims substantially corrected, many missing tests supplied. Remaining applicability/coverage gaps and failed decay claims preclude closure. |
| C2-08 | **Closed** | All nine refusal variants independently fail external construction for non-exhaustiveness; external matching compiles. Candidate's unit/struct construction probes also pass. |
| C2-09 | **Note retained** | Fresh workload confirms bounded indexed refusal preflight and history-dependent successful finalization; no performance certification. |

## 4. Disposition of every prior implementation decision

| Decision | Disposition |
|---|---|
| D-C2-1 | **Preserved accepted:** pacing excluded from canonical artifact identity; canonical/pacing separation retained. |
| D-C2-2 | **Preserved accepted:** one engine per profile; cross-profile oracle remains partial, no merged runtime claim. |
| D-C2-3 | **Withdrawn, not rehabilitated:** old baseline fallback/aligned-only restriction replaced; C2-05 remains open under the replacement's defects. |
| D-C2-4 | **Preserved accepted:** explicit absent-cell arithmetic/crossing convention. |
| D-C2-5 | **Revised and accepted:** reserved activation binding now enforced; ordinary finalization unchanged. |
| D-C2-6 | **Preserved accepted:** fixed manifest and epoch-bound rule/config/seed; no migration proof. |
| D-C2-7 | **Revised compatibility account accepted as specified in R-4 below:** origin rule really resolves; changed current fingerprint refuses. Independent old-work/current-config and changed-rule controls pass. Decay arithmetic remains separately blocked. |
| D-C2-8 | **Preserved accepted mechanism:** one rule body is a declared reducer; widened arithmetic and both sanctioned composition fixtures pass. This does not waive decay timing. |
| D-C2-9 | **Preserved accepted subset:** conservative static mixture analysis plus runtime rejection, not complete alias analysis. |
| D-C2-10 | **Preserved accepted interpretation:** cohort-scoped depth and strictly-later conversion; remaining interference/depth corpus gaps disclosed. |
| D-C2-11 | **Revised and accepted:** typed sticky StoreInvariantViolated, no repeat-Paused loop; before-first and after-earlier-cohort fault probes, no dequeue, snapshot/reset refusal, restore recovery pass. |
| D-C2-12 | **Preserved accepted:** deterministic 64-bit parent-digest RNG projection; no injectivity claim. |
| D-C2-13 | **Revised and accepted:** materialized emissions use the creator rule fingerprint; same-epoch payload independence and independent formula after rule removal/epoch replacement pass. |
| D-C2-14 | **Preserved accepted:** isolated test-support seams; no default-feature evaluator/fixture exposure. Missing mandated fault cases remain obligations. |
| D-C2-15 | **Preserved accepted:** scoped size/argument lint allows; strict arithmetic/panic lint still passes. |

## 5. Disposition of all flagged interpretations

The mission calls these report §7 interpretations; they actually appear in writer report
§4. This review uses their R-1–R-9 identifiers without losing any item.

| Flag | Disposition and controlling text |
|---|---|
| R-1 baseline declaration/once-only materialization | **Accept.** Q7 requires the existing cell baseline and activation rejection without baseline semantics; it does not freeze a new declaration channel or epoch retargeting. The rule-set artifact commits validated declarations, engine writes populate the existing field once, and later declarations seed new cells without replacing existing targets. Existing encodings are preserved. B:928 plus independent recovery/door probes support it; not permission for migration or retargeting. |
| R-2 decay time/epoch integration | **Reject as frozen conformance.** FINAL §4 controls `at`; Q7/AT-I22 retain remainder/chunk obligations; AT-I23 bars retroactive new-rate use. C2R-01–03 show a time-contract replacement and two executed defects, not merely a preference about representation. |
| R-3 lineage outside engine digest | **Accept the representation.** v1 §8 immutable activation artifacts + ADR-0006 retention: hashes already enter EpochRegistry, activation times are bound through finalized command semantic hashes. Restore validates every retained entry and current artifacts. Independent multi-activation/reset snapshot controls and time-tamper refusals pass. This is not a new uncommitted behavior knob, nor completion of all timeline-derived-index AT-I28 cases. |
| R-4 exact-origin/current-epoch compatibility | **Accept this explicit Phase-2 account.** v1 §5 and AT-I14 require the exact rule fingerprint/ruleset to resolve in lineage and explicitly refuse a superseded same-ID rule; v2 §8 leaves that binding unchanged. Q5/Q7, ADR-0004 class-1 changes and AT-I23 put hot parameters at epoch barriers. Here originating artifacts, including config, remain retained and hash-validated; identical rule logic uses the hash-bound evaluation epoch's config, and missing origin or changed rule refuses. This is not permission to discard origin config, substitute unresolved artifacts, or run changed same-ID logic. The compatibility decision is explicit and tested, rather than the prior bare historical-hash membership check. C2R-02 separately defeats the implementation of decay integration. |
| R-5 new store fail-stop class | **Accept.** The accepted D-8 fail-stop discipline applies to a proven store invariant defect; direct extraction remains typed and read-only on refusal. Process honestly retains/reports prior committed cohorts and ActiveRequest changes. Independent and writer fault tests pass. |
| R-6 creator-rule fingerprint field | **Accept.** v1 §5 creator identity and v3 §4.4 require the true rule-fingerprint component. The new Phase-2 record field is hash-bound, not a Phase-1 encoding change. Independent post-activation formula verifies it; historical fence changes legitimately change later epoch hashes and thus emission identities. |
| R-7 report/diagnostic removal | **Accept.** Later FINAL AT-I46(a) controls over earlier v3 §3.4's cohort-identity list. `command_deferred` removal restores the frozen diagnostic fields; deferred-command status is derivable. Independent field-absence probes and candidate field pin pass. |
| R-8 enqueue canonicalization | **Accept.** v2 §3.2 idempotency extends to emitting operations. Exact enqueue claims fold; contested payload under one identity rejects atomically. Independent materialized-enqueue duplicate/contested controls and O derived redelivery pass. This label does not revive the withdrawn architectural R-8. |
| R-9 additive scheduler accessor | **Accept.** Read-only full-commitment lookup enables frozen AT-I8 invariant preflight; no host engine scheduler authority or protected-function/encoding change. The independent disposable shape-only mutant is killed by O:777. |

## 6. Oracle coverage and discrimination

**S** = meaningful finite executed support, not universal proof or an acceptance waiver.
**P** = applicable coverage remains partial or an accepted scope interpretation has no full
lifecycle fixture. **R** = demonstrated contract failure. Every entry below was evaluated
against the corrected tree and fresh tests; inherited supported entries retain the prior
review's caveats. Candidate test names/row identifiers and the independent probes in E
identify the evidence; none of the review probes was added to the candidate's test suite.

For inherited fixtures, C/D/W/Q/F/S/X name respectively core-surfaces, rule-set-door,
waves-and-pacing, request-boundary, finalization-and-replay, stores-and-semantics, and
compile-probes test files (`phase2_*.rs`). Prior line numbers are omitted because surface
adaptations shifted them.

| Entry | Review | Executed evidence / remaining requirement |
|---|---|---|
| AT-I1 | **R** | O additive untouched-clone test passes; independent decay reapplication at canonical time fails (C2R-01). |
| AT-I2 | **S** | X forbids run_waves; W distinguishes later-wave reads/retained commits. |
| AT-I3 | **S** | O: 24 declaration permutations × 2 insertion orders, RESULT coalescing + singleton TRANSFORM + additive |
| AT-I4 | **S** | O: each of six `WorkKey` fields moves cohort identity; engine-level formula |
| AT-I5 | **S** | O: renamed/reordered IDs — same rejection evidence, same results |
| AT-I6a | **S** | O: engine-level exact fold / contested rejection in every permutation (seam); obligation emissions fold (§5) |
| AT-I6b | **S** | W: distinct +10/+10 yields 40 with two retained causes. |
| AT-I6c | **S** | B union/background/multi-target and O redelivery/depth-2/empty-set; own fixed-scope repeated-read full-set formula passes; watched-only mutant killed. |
| AT-I6d | **S** | O renaming sub-case (identity = complete-set formula, never the minimum); probe |
| AT-I7a | **S** | W full additive fold and reversed construction equality. |
| AT-I7b | **S** | W unequal RESULT rejection and pre-wave digest equality. |
| AT-I7c | **S** | W distinguishes same-family equal results from equal-valued transforms/cross-family results. |
| AT-I7d | **S** | D/W static/runtime incompatible family table, atomic outcomes. |
| AT-I7e | **S** | O three-term `i128` overflow typed and atomic |
| AT-I8 | **P** | O full-commitment mismatch preflight rejects atomically, shape-only mutant killed; inherited late-failure cases pass. Claimed intermediate per-wave observation is absent (C2R-04). |
| AT-I9 | **S** | W repeated unequal-result report and digest equality. |
| AT-I10 | **S** | D rejects host-owned target with unchanged lineage; inherited authority tests retained. |
| AT-I11 | **S** | S bounds extremes and atomic companion effect; typed write-path checks retained. |
| AT-I12 | **S** | S slot/store payload agreement across schedule/collide/drain prefixes. |
| AT-I13 | **P** | Fixed-manifest scope accepted; multi-target schema-drift seam passes. No actual definition migration lifecycle is implemented. |
| AT-I14 | **S** | Exact originating rule/ruleset resolution and unknown creator refusal in B; current changed fingerprint refuses; own epoch/config controls pass under R-4. |
| AT-I15 | **S** | O contested records from wave-1 derived emissions under one rule set, both orders |
| AT-I16 | **S** | O derived competing claims, ascending identity, pacing split unchanged |
| AT-I17 | **S** | B nested sub-IDs |
| AT-I18 | **S** | D direct/mutual/long watched-edge cycles and delayed counterparts. |
| AT-I19 | **S** | B scope breadth/materialized expansion and inherited depth bounds; own exact/over-bound scope probes pass; definition-only fan-out mutant killed. |
| AT-I20 | **S** | W/Q whole-cohort pacing and canonical equality on tested histories. |
| AT-I20b | **P** | Effect/enqueue/candidate cap refusal and replay pass. New retry uses an unrelated producer, and the new effect/enqueue-cap fixtures do not compare all retained components (C2R-04). |
| AT-I20c | **S** | O (f) combined oversized cohort with co-target and threshold inputs |
| AT-I20d | **S** | X `EvalView` unreachable; `PacingDiagnostics` unforgeable; evaluator view has no diagnostics by construction |
| AT-I21 | **P** | Strict-later depth seam and exact interference ordering/identity pass; value-dependent deferred-input discriminator remains missing (C2R-04). |
| AT-I22 | **R** | Baseline and original remainder correction, unsaturated chunk sweep and sanctioned compositions pass; canonical-time reapplication and saturated cell-time equality fail (C2R-01/03). |
| AT-I23 | **R** | Original fixed-cadence rate-barrier case now gives 90; shortening hot cadence gives 70 instead of 95 (C2R-02). |
| AT-I24 | **S** | B/O widened aggregate and i128 boundary corpus; own negative MIN half-weight and cancellation pass; premature narrowing mutant killed. Finite corpus, no cached incremental aggregate path claimed. |
| AT-I25 | **S** | Full reduction/crossing plus changed condition-read union, unchanged background, multi-target and independent fixed-scope union; no representative parent selection. |
| AT-I26 | **S** | O each Phase-2 store alone moves exactly its component and the engine digest |
| AT-I27 | **S** | O permuted cooldown/occurrence construction through an epoch activation and a timeline reset; contested-claim permutations and caps as before |
| AT-I28 | **P** | Lineage time/truncation tampering and independent multiple-epoch/reset restore controls pass. No injected timeline-index corruption fixture; not every scripted prefix has a separate lineage recomputation observation. |
| AT-I29 | **S** | S snapshot/restore before each request/resume; F paused recovery; X prohibits inter-wave entry. |
| AT-I30 | **S** | O 300 dormant scopes leave active candidate set/effects/identities and outer selections equal. Supports the stated logical-work oracle, not physical map-scan cost. |
| AT-I31 | **S** | O aggregate moves only at its cadence; promotion reads the committed aggregate |
| AT-I32 | **S** | Strict lint; candidate 45 external probes; own 14 default-feature surface controls including all nine refusal variants. Every expected construction/privacy failure has its matching diagnostic. |
| AT-I33 | **P** | O exact 64-parent formula and cap-minus-one rejection; inherited extremes; multi-parent depth-2 chain. Maximal configured-depth chain remains absent. |
| AT-I34 | **S** | All 232 inherited tests preserved, unignored and included in 374 passing tests. |
| AT-I35 | **S** | O golden bytes for `None`/`Advance`/`Command`, independent boundary reference, three mutants |
| AT-I36 | **S** | Fresh five all-target static builds pass; no executable parity claim. |
| AT-I37 | **S** | D/W discriminating static/runtime pair tables for the enumerated families. |
| AT-I38 | **S** | O parent set covers a cause beyond the presentation cap |
| AT-I39 | **P** | O independently recomputed batch digests reject heartbeat substitution; inherited live-drain/pre-wave cases pass. Cross-profile companion remains unexecuted. |
| AT-I40 | **P** | Per-profile architecture accepted and inherited core multi-profile slice ordering passes; complete composed multi-profile engine oracle remains absent. |
| AT-I41 | **S** | O `effect_batch_v3` recomputed from components for two cohorts in one call; cohort-scoped wave index; drain-scoped mutant falsified |
| AT-I42 | **S** | C and W execute stale/nonleast/replay/cap/refinement/no-op and cross-store missing/mismatch/record transfer discriminators. |
| AT-I43 | **S** | Q lifecycle/start/substitution/resume/time/finalization/overflow cases meaningful; decay-rate correctness separately fails AT-I23. |
| AT-I44 | **S** | O (c′) mixed slice: conflicts first, complete removal, post-extraction digest |
| AT-I45 | **S** | W budget-2 discriminator and zero-cost conflicts after oversized admission. |
| AT-I46 | **S** | Corrected exhaustive field pin, independent forbidden-field/default-feature seam probes; report types have no feature-conditioned canonical fields. |
| AT-I47 | **S** | Q, F active/F discrimination, exact restore/resume, cleanup and mismatch behavior. |
| AT-I48 | **S** | F full-state refusal cases, 360 differentials, positive stage/fence, follow-on, refusal retention and sticky injected fail-stop. |
| AT-I49 | **S** | F wrong presentation, completed non-head, active/head disagreement and durability-order model. No real durable mailbox claim. |
| AT-I50 | **S** | O (d) a different payload under the same envelope fields diverges |

Superseded original AT-I6/AT-I7 forms remain superseded. No S label waives a P or R row.

## 7. Original corpus, converted tests, negative controls and provenance

The original counterexample source is byte-identical to the prior review. This review
freshly ran it in disposable external crates against both the prior candidate-code tree
at `00d647e` and the corrected candidate. Before correction it reproduces **3 passed,
7 failed**; after correction it fails compilation on exactly three changed surfaces:
external refusal construction, removed cohort report field, and the new decay shape
(`toward` removed; rate/cadence are Params). The multiple compiler errors for decay belong
to that one API surface, not additional unrelated failures.

The writer's adapted variant passes **9/9** freshly. Inspected its source diff: A1 removes
the construction-success counterexample (now required to fail compilation); A2 observes
cohort identity through test-support; A3 declares baseline zero and adapts rate/cadence
without changing fixture numeric values or expected results; A4 enables test-support only
for the adapted crate. These adaptations are justified and do not turn the original seven
failed semantic assertions into weaker assertions. They also do not establish the broader
new decay claims.

Six independently executed disposable mutants compile and fail the named candidate tests:
watched-only parent set; definition-only fan-out; unclaimed nested sub-ID; aggregate narrowed
before weighting; payload-only activation; shape-only enqueue preflight. Both new candidate
test files first passed unmodified in that disposable archive (25 + 29 tests). Thus these
are actual wrong-implementation kills, not compiler failures relabeled as negative controls.
The original corpus independently supplies the original remainder and retroactive-rate red
controls. New duplicate-enqueue, artifact resolution, fail-stop, arithmetic overflow,
provenance and raw-byte encoding controls are also executed in the fresh workspace suite
and independently checked where enumerated above. The missing cases in C2R-04 remain
missing despite these positive results.

The reviewer authored **14 additional runtime probes**: **11 pass, 3 contract assertions
fail**. Independent default-feature surface checks pass **14/14** (one matching positive
control, nine refusal-construction negatives, two forbidden report fields, private EvalView,
and feature-hidden fixture). These are additional to the candidate's 45 Phase-2 external
compile probes. All semantic probe sources and exact normalized output are retained in E.

The writer correctly retracts the original blanket red-first claim. Git history confirms
implementation preceded several original test slices. The pre-correction corpus is real red
evidence for its named defects, not evidence that every later oracle-completion test was
written red-first. Historical checkpoints and baseline evidence remain untouched.

## 8. Preservation and fresh validation

Mechanical preservation checks pass: seven inherited integration files, all 17 inherited
inline Phase-1 test modules, manifests and Cargo.lock are byte-identical to production;
`schedule`, `drain_due`, `stage`, `submit_fence` are byte-identical. The only spark-core
revision relative to the prior review is the additive 13-line read-only accessor. All 232
inherited tests remain included and pass. Gate C2 test adaptations were inspected: refusal
matching still checks every field, cohort observations move to the seam, decay declarations
preserve the original target values, and the changed-rule refusal becomes the specific
RuleSuperseded. No inherited Phase-1 test was weakened, ignored or removed.

| Fresh check | Result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo test --workspace --all-features --no-fail-fast` | **374 passed, 0 failed, 0 ignored** |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| strict core/engine all-feature library lint | PASS: warnings, unwrap, expect, panic, indexing/slicing, arithmetic-side-effects denied |
| `cargo metadata --format-version 1` | PASS; dependency-direction fixtures pass |
| `git diff --check 00d647e HEAD` and `7e3a0aa HEAD` on exact candidate | Both PASS |
| Windows x86_64 GNU and MSVC all-target static checks | Both PASS |
| Android aarch64, armv7, x86_64 all-target static checks | All three PASS |
| original corpus before correction | Expected cargo 101: 3 pass, 7 fail |
| original corpus on exact candidate | Expected cargo 101: three changed API surfaces only |
| adapted corpus on exact candidate | Cargo 0: 9 pass |
| independent runtime probes | Cargo 101: 11 pass, 3 genuine contract assertion failures |
| independent default-feature surface controls | 14/14 expectations and diagnostics satisfied |
| disposable compiled mutants | 6/6 killed by assertion failures |
| source/test/manifest/lineage preservation | PASS |
| offline release `gate_c2_workload` | PASS; measurements below |

`checks.json` records exact commands, elapsed times and runner exit codes. The corpus
capture runner returns 0 after capturing output even when cargo returns 101; its wrapper
status is **not** presented as semantic success. The individual logs and
`independent-results.json` distinguish compiler failures, expected mutant failures and the
three candidate contract failures. No executable Windows/Android parity is claimed.

| Finalized history | Build seconds | P-9 refusal / call | Scan control / call | Whole successful finalization / call |
|---:|---:|---:|---:|---:|
| 1,000 | 0.368 | 2.374 µs | 5.142 µs | 0.766 ms |
| 4,000 | 6.672 | 2.455 µs | 41.091 µs | 3.636 ms |
| 16,000 | 128.570 | 2.437 µs | 995.458 µs | 14.379 ms |

This preserves C2-09's note: indexed refusal preflight remains flat on this finite
one-machine workload; successful finalization still hashes history through unchanged
Phase-1 code. Pre-wave digests traverse stores, `cells_of` scans the cell map, and decay
now traverses retained epoch lineage. Report counts do not establish bounded physical
scan cost. No production performance guarantee, durable storage, crash recovery, durable
mailbox, transport, GAME integration, or Phase-3 implementation is claimed.

## 9. Publication, blockers and exact next action

Publish this document and bounded evidence in one normal commit on
`review/phase2-gate-c2-revision-independent-20260911`, push normally and verify local HEAD,
tracking ref and live GitHub review ref equality. Recheck candidate, original candidate,
prior review, production and research refs without moving them. The final receipt records
that exact review hash; no candidate source/test change is part of the review commit.

**Remaining Gate C2 blockers:** C2R-01–03 (C2-05), C2R-04 (C2-07), and the explicitly
partial applicable oracle cases above. Publication/validation availability is not a blocker.

**Exact next action:** return this pinned candidate and the published review to the
separated correction writer for another bounded Gate C2 revision under the existing
Phase-2 authorization. Preserve the accepted fixes, reconcile decay remainder with the
controlling cohort-time contract, prevent retroactive charging after cadence changes,
complete saturated/recovery controls and the applicable missing oracle cases, and publish
one new exact candidate for independent review. If the representation cannot satisfy the
frozen texts together, present that precise conflict for an explicit architecture decision;
do not silently amend their meaning. This reviewer makes no repair and does not promote
production or begin Phase 3.
