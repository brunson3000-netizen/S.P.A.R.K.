# Gate C2 decay adjudication — independent review

**Reviewer:** Codex, independent of the candidate writer. **Completed:** 2026-09-12.
**Mission:** `SPARK_PHASE_2_GATE_C2_DECAY_ADJUDICATION_INDEPENDENT_REVIEW_MISSION_2026-09-11.md`.
**Exact candidate reviewed:** `544c5f6f99d8dabf9855f8ac68f5666286aa9741` on
`candidate/phase2-gate-c2-decay-adjudication-20260911`.
**Verdict:** `GATE_C2_DECAY_ADJUDICATION_ACCEPTABLE_FOR_OPERATOR_ACCEPTANCE`.

No decision-rule violation or new blocking defect was found. The explicit adjudication
resolves C2R2-01; fresh discriminators support closing C2-05 and the AT-I22 portions of
C2-07/C2R-04. AT-I22′ and AT-I23′ have meaningful executed support. This review does not
accept the candidate for the Operator, promote production, repair the candidate, or begin
Phase 3. The four expressly undecided matters remain unratified implementation behavior.

Review branch: `review/phase2-gate-c2-decay-adjudication-independent-20260911`.
Isolated worktree: `/tmp/spark-gate-c2-adjudication-review`.
Evidence directory (E): `gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/`.
The publication receipt identifies this report's containing review commit after normal push.

## 1. Receipt distinction, live custody and precedence

The initial fetch and live `git ls-remote --heads origin` agree on the full final candidate
hash above. Its immediate parent is `244c54b66a9c5bdfe8dd49e28b05cc0f5c255637`, the commit
named by the writer's validation receipt. These are different commits, not aliases.

| Lineage, oldest to newest | Role |
|---|---|
| `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` | Production and Operator acceptance freeze; retained ancestor |
| `6968a4af917c9a32761be371c3e12ec12ba0f1bd` | Prior published candidate |
| `6b167d84f21fb60f374a2ab45c62d5c6a040790a` | Controlling independent review; direct base |
| `0c27ad89833d0c88162803ab41dec177424b49bb` | Direct child of review; decision record plus writer mission only |
| `244c54b66a9c5bdfe8dd49e28b05cc0f5c255637` | Direct child of decision checkpoint; comments and ten new tests |
| `544c5f6f99d8dabf9855f8ac68f5666286aa9741` | Direct child of checkpoint 1; final report, evidence and review mission |

The last edge adds 75 files, all under `engineering/phase2/`; it changes no crate, test,
manifest or lockfile. The `crates` tree is `4b8c618615b3d6d8a38c9ca5538895757a6ac7b3`
at both checkpoints. Evidence scripts are part of the added evidence, so “documentation
and evidence only” does not mean that all added files are prose. The writer's 400-test
receipt on checkpoint 1 is accurate in scope but is not fresh validation of checkpoint 2.
This review reran every required check with HEAD pinned to **544c5f6**, including the
final evidence and both exact diff ranges. The mutation archives also pin 544c5f6.

The published branch tips for production, previous candidate and controlling review were
verified against live GitHub; the intermediate checkpoint hashes and parent links are
reachable through the fetched live candidate. `initial-custody.json` records every live
head, local/tracking ref and worktree. `independent-custody-audit.json` independently checks
the lineage, code-tree distinction, historical blobs and the writer's 75 checksum entries.

For decay cadence, residual and commit time, the Operator adjudication controls. Its
D-1–D-7 match the decision recorded in the checkpoint-0 writer mission. Other precedence
and accepted pins remain those of the controlling review: Operator freeze/Revision-2
review and architecture/oracle, FINAL, retained V2, freezes/matrices, ActiveRequest and
ADRs. Withdrawn Phi, SH-1/SH-2 and architectural R-8 remain withdrawn. This is conformance
to the newly adjudicated grid, not a reversal of the prior finding about the old Q7 text.

## 2. Decision record and preservation

**PASS.** Checkpoint 0 contains exactly the two new decision/mission documents and no
implementation changes. “Recorded alone” is satisfied as a document-only checkpoint with
its companion mission, not as a claim that there was only one file. The decision record's
blob has not changed since introduction. All **448** pre-existing `engineering/` files
at the controlling base remain byte-identical, including every historical review probe.

`supersession-source-lines.txt` captures the cited text at the exact base. All S-1–S-13
paths and line references identify the stated passages:

| Passage | Disposition in this review |
|---|---|
| S-1 | Correctly supersedes elapsed-since-write counting only; linear/floor/i128/baseline/bounds retained. |
| S-2 | Correctly limits evaluation-partition invariance to intervals with no intervening non-decay write and includes canonical commit time. |
| S-3 | Correctly connects hot-tunable parameters to changed-segment activation. |
| S-4 | Correctly retains no new encoding or cell machinery. |
| S-5 | Correctly replaces the conflicting v2 retention of the duration count; single decay rule per target and rounding remain. |
| S-6 | Correctly retains atomic mixed-family rejection and both sanctioned compositions; their numeric results follow the fixed grid. |
| S-7 | FINAL §4 canonical time remains binding and unchanged. |
| S-8 | FINAL oracle's old unchanged-substance statement for I22 is explicitly superseded. |
| S-9 | Same for I23 only; surrounding rows are retained. |
| S-10 | Correctly identifies original I22 and supplies the amended tuple contract. |
| S-11 | Correctly retains I23 nonretroactivity with explicit residual and endpoint policy. |
| S-12 | Correctly retains v2 rounding and composition/rejection requirements. |
| S-13 | Correctly supersedes old unchanged annotations for I22/I23. |

The record's detailed grid interpretation is consistent with the seven recorded rules.
Its four open matters are explicitly not new Operator rulings. In particular, the
`(updated_at, now]` read interval describes when an evaluation applies steps; it does not
make a preceding ordinary write an implicit decay evaluation.

Every changed line in each of the four modified pre-existing crate files is a comment.
The only added crate file is `phase2_decay_adjudication.rs`. No executable implementation
line changed in this candidate series. Preservation checks pass for the seven inherited
integration files, 17 inline Phase-1 test modules, all manifests/lockfile and five protected
functions. `spark-core` is unchanged by this pass; its retained earlier additive test-support
seams remain gated. `schedule`, `drain_due`, `stage`, `submit_fence`, and the accepted
`derived_indexes_consistent` implementation are byte-preserved against their proper bases.

FINAL §4 is preserved in the source and executed reapplication/commit-time probes. Grid
origin is derived from committed activation lineage and restore-validated artifacts, not
a new `StateCell` field, changed cell encoding, or backdated timestamp. Retained encoding
and digest fixtures and lineage/index-tampering tests pass. No new production surface is
introduced; all 17 external default-feature surface controls pass.

## 3. D-1–D-7 and retained findings

The source audit covers `engine.rs::decay_walk` (line 639), single decay evaluation
(line 926), rule-body decay (line 980), canonical commit application, activation ordering,
and `effects.rs::move_toward` (line 470). The walk counts endpoints with quotient
differences in each segment; the reference model enumerates endpoints. Independent
review probes use hand values and public requests, without either arithmetic implementation.

| Rule | Disposition | Independent executed discriminator |
|---|---|---|
| D-1 fixed grid | PASS | Rate 3/cadence 6, assignment 20 at 5: one evaluation at 12 or evaluations 6/7/11/12 both yield (14,12), both signs. Double-counting the anchor endpoint is killed. |
| D-2 no re-phasing | PASS | Unaligned command assignment and shock retain the grid. Rule body +5 then decay yields (22,6), (24,12) from 20@5 with 3/6. Replacing body anchor by now is killed. Writer scheduled-shock vectors and retained atomic rejection/compositions also pass. |
| D-3 change closes/discards/restarts; unchanged preserves | PASS | 2/4 → 7/5 at 13: (94,17), (87,18); residual after 12 is unpaid. Cadence-only 2/4 → 2/5 gives (94,16), (92,18). Unchanged 2/4 gives (92,16). Ignoring cadence-only change and keeping old origin are killed. |
| D-4 closing endpoint | PASS | 2/4 → 7/5 at 12: old points 4/8/12 yield 94; first new point 17 gives 87. Catch-up, scheduled work at barrier and a subsequent same-time shock all discriminate ownership. Excluding old endpoint is killed. |
| D-5 no pre-activation charge | PASS | 1/20 → 9/5 at 7 gives 100@11 and 91@12; 2/4 → 9/20 at 13 gives 94@32 and 85@33. Moving origin one tick before the barrier is killed. Both signs. |
| D-6 canonical commits | PASS for existing-cell decay | No-step, moved, saturated and zero-rate evaluations pin exact commit times; replay and existing reapplication/watcher/rejection controls pass. Next-tick commit mutant is killed. Absent-cell semantics remain separately undecided. |
| D-7 retained corrections | PASS | Baseline 11, starting 7 above/below, rate 4/cadence 6 reaches baseline without crossing it; later saturated evaluation commits at 19. Overshooting-baseline mutant is killed. All retained nonretroactivity/saturation controls pass. |

Nine own probes pass. Nine own mutant baselines pass and all nine mutants compile and
fail by assertions; eight target decision-rule errors and one targets skipping a zero-length
epoch. D4 and D6 reuse useful wrong-implementation shapes with new tests; this is not a
claim that every mutation patch is novel. The writer's ten mutants and controlling
review's four mutants also pass their positive baselines and are killed.

| Finding | Final disposition |
|---|---|
| C2R2-01 | CLOSED by explicit Operator architecture adjudication plus this exact-candidate conformance review. Prior rejection of “already frozen-conformant” remains valid history. |
| C2-05 | CLOSED under D-1–D-7 and AT-I22′. |
| C2-07 | CLOSED with retained scope limitations; revised AT-I22 is now supported. |
| C2R-04 | CLOSED for required bounded oracle support; I13 and previously disclosed scope debt remain. |
| C2R-01/02/03 | Prior canonical-time, nonretroactivity and saturation corrections preserved and independently discriminated. |

C2-01/02/03/04/06/08 remain closed. D-C2-5/7/11/13 remain revised and accepted;
D-C2-1/2/3/4/6/8/9/10/12/14/15 retain their accepted dispositions. R-1 and R-3–R-9
remain accepted, including enqueue-canonicalization R-8 (distinct from withdrawn
architectural R-8). C2-09 remains the disclosed finalization/history performance limitation.

## 4. Amended oracle and evidence quality

**AT-I22′: S (supported).** Every clause has executed support: evaluation partitions
without intervening writes, exact baseline/bounds/formula arithmetic, atomic coincident
shock rejection, both sanctioned compositions, 90@10 after assignment at 9, 65@40 after
shock at 31, both-sign unaligned-write partitions and canonical unmoved commits.

**AT-I23′: S (supported).** Both directions have shortening, lengthening, rate-only,
cadence-only, unchanged-parameter, residual-discard, old-endpoint and no-precharge vectors.
Scheduled work at the barrier runs before activation. Replay/restore/pacing controls
reproduce the decision; own barrier and open-matter probes add replay and restore coverage.

All ten candidate tests have correct hand values. Fresh assignment and post-shock directly
reject elapsed-since-write counting. Boundary catch-up (not only an evaluation exactly at
the barrier) detects dropped endpoints. Residual/shortening/lengthening distinguish carried
phase, absolute origin and residual billing. Saturation and zero-rate/no-step rows check
commit time as well as value. Recovery mirrors the finite hand vectors. Composite replay
and restore compare digests on identical histories. Chunking exercises 64 selected
partitions in each applicable fixture; the composite's two 32-subset groups vary each side
of the pinned pre-shock evaluation, not a 32×32 Cartesian product. The generated test has
40 timelines with assignments, reassignments, shocks, changed and unchanged activations.
It is a finite model-agreement sweep, not an exhaustive partition proof. Its generated
fresh times can be aligned for some randomly chosen cadences; the dedicated 9→10 test
supplies the mandatory unaligned discriminator.

The candidate `model` independently enumerates endpoints without quotient arithmetic or
calling `decay_walk`, then moves one step at a time. It shares the adjudicated segment
premise and encodes the existing lost-prewrite behavior; it cannot itself ratify that
open policy. No claim that every vector kills every alternative is needed: each class
has its relevant discriminator and the compiled controls demonstrate the stated gaps.
The ten writer mutants have 21 distinct passing baseline tests and 28 assertion-failure
kill runs; the two old oracle tests deliberately survive their respective mutants while
the stronger retained tests kill them.

The prior post-shock assertion changed from 75 to 65 in the preceding writer series;
this pass changes its rationale, not its value. The adjudication now explicitly authorizes
that numeric contract. Two `r2prime_frozen_q7_*` historical probes remain byte-identical
and fail only at the superseded fresh-write/post-shock expectations. The decision-adapted
copy differs by exactly the two expected-value literals, and passes 9/9. The single
obsolete backdated-time assertion in the second-review corpus likewise remains in its
original file; its already published one-literal copy passes 14/14.

Tuple invariance across different evaluation partitions is the amended I22′ contract.
The old I22 source also mentioned engine digests; the new entry explicitly supersedes its
substance and does not promise full-history/store digest equality for different scheduled
histories or watcher emissions. Digest equality here is tested for the same logical history
under replay, restore and pacing. This distinction does not weaken FINAL §4 or alter encoding.

## 5. Four undecided matters

These are observations, not new Operator decisions. No contradiction with D-1–D-7 was
found, so none is a blocking defect under the review mission's explicit scope rule.

| Matter | Executed observation and disposition |
|---|---|
| Remove/restore operation | PASS as reported. With 2/4, removal at 13 and restoration at 22, catch-up gives 94@25 and 92@26, both signs. Old points 4/8/12 are retained, absent interval contributes nothing, restoration starts at 22. Replay/restore pass. The earlier zero/absent vectors and generated sweep remain. |
| Lost grid point before an unevaluated write | PASS as reported. 100@0, +5 shock at 15, evaluation at 20 gives 95; inserting evaluation at 10 first gives 85. Both signs and replay pass. The writer's “duration reading behaves identically in this respect” means forfeiture of pre-write debt, not identical numeric results at 20: the duration reading would have no whole post-write step there. This is not re-phasing and is outside I22′(a)'s no-intervening-write condition. |
| Two activations at one time | PASS as reported; writer lacked a dedicated test. 2/4 → 7/5 → 2/4 at 13 gives 94@16, 92@17; two unchanged 2/4 activations give 92@16. Intermediate zero-length epoch still resets the segment. Both signs, restore between activations and replay pass; a skip-zero-length-epoch mutant is killed. |
| Absent cell | PASS for single-operation decay: no effect/cell at time 4, assignment at 7 followed by decay at 8 gives 98. Restore/replay pass. A multi-stage rule body can independently produce a value; the no-cell statement is not a prohibition on such other operations. |

An Operator decision is required to make these four behaviors normative guarantees or to
choose alternatives. This review does not require a new decision before the bounded Gate C2
acceptance decision, because the mission explicitly leaves them outside D-1–D-7. Acceptance
should acknowledge that boundary. The new dedicated probes live only in review evidence;
they do not silently repair or expand the candidate's checked-in regression suite.

## 6. Every oracle-row disposition

S = meaningful finite executed support with retained qualifications; P = partial lifecycle
coverage. The exact-candidate workspace suite reruns every inherited S row. This bounded
review rechecks the changed decay requirements and preserves the controlling review's
non-decay evidence and limits. Original unsplit I6/I7 are superseded by lettered rows.

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
| AT-I22′ | S | D-1–D-7 adjudication; all I22′ clauses supported as detailed in §4; two old Q7 expectations explicitly superseded. |
| AT-I23′ | S | Adjudicated endpoint/residual/grid policy, both-sign rate/cadence/unchanged controls, scheduled barrier ownership, replay/restore/pacing. |
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
| AT-I34 | S | All 232 inherited tests byte-preserved and included in 400 passing tests. |
| AT-I35 | S | None/Advance/Command golden encodings and independent boundary reference. |
| AT-I36 | S | All five Windows/Android static targets; no runtime parity claim. |
| AT-I37 | S | Discriminating static/runtime same-target family cases. |
| AT-I38 | S | Cause beyond presentation truncation affects complete identity. |
| AT-I39 | S, per-profile | O batch recomputation plus new composed-profile independence companion. |
| AT-I40 | S, per-profile | Distinct profile cohorts/own membership, third-profile independence and cross-profile refusals. |
| AT-I41 | S | Per-cohort effect_batch_v3 recomputation and cohort-local wave index. |
| AT-I42 | S | Core surface stale/nonleast/replay/cap/refinement/store mismatch fixtures. |
| AT-I43 | S | Request lifecycle/start/resume/substitution/time/finalization/overflow; decay numeric contract now governed by I22′. |
| AT-I44 | S | Mixed slice conflicts first, full removal and post-extraction digest. |
| AT-I45 | S | Budget-2 discriminator, zero-cost conflicts after oversized admission. |
| AT-I46 | S | Frozen report field pin, forbidden fields and feature-only observation seams. |
| AT-I47 | S | ActiveRequest/F distinctions, restore/resume and mismatch cleanup. |
| AT-I48 | S | Full-state refusal, 360 differentials, follow-on retention and sticky fail-stop. |
| AT-I49 | S | Wrong/non-head/active-head/dequeue durability-order controls; no real durable mailbox claim. |
| AT-I50 | S | Full command payload changes request identity under same envelope fields. |


## 7. Fresh validation on the exact final candidate

Every command below ran with HEAD at `544c5f6f99d8dabf9855f8ac68f5666286aa9741`.
No candidate repair or test-source substitution occurred. Review probes and mutants use
external disposable crates/archives. `checks.json` records exact commands, return codes,
durations and build settings; logs retain complete whitespace-normalized output.

| Required check | Result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo test --workspace --all-features --no-fail-fast` | 400 passed, 0 failed, 0 ignored |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| Strict core/engine all-feature library lint | PASS; warnings, unwrap, expect, panic, indexing/slicing and arithmetic-side-effects denied |
| `cargo metadata --format-version 1` | PASS |
| `git diff --check 6b167d8 HEAD` and `git diff --check 7e3a0aa HEAD` | Both PASS |
| Windows x86_64 GNU/MSVC `cargo check --workspace --all-targets` | Both PASS (static only) |
| Android aarch64/armv7/x86_64 same static checks | All PASS |
| Preservation including comment-only and immutability | PASS; no failures |
| Release offline `gate_c2_workload` | PASS |
| First original review corpus | Expected compile failure at exactly E0308/E0559/E0603/E0609 revised surfaces |
| First adapted review corpus | 9 passed, 0 failed |
| Second original review corpus | 13 passed, 1 expected obsolete timestamp failure |
| Second review's existing time-literal-only copy | 14 passed, 0 failed |
| Controlling original review corpus | 7 passed, 2 expected superseded Q7 assertion failures |
| Decision-adapted copy | 9 passed, 0 failed; exact two-literal change checked |
| Writer mutation controls | 21 positive baseline tests; 10/10 compiled mutants killed, 28 asserted kills; two specified prior-test survivals |
| Controlling review's four mutants | 4/4 positive baselines and 4/4 assertion kills |
| 17 default-feature surface controls | 17/17 expected compile outcomes and diagnostics |
| New independent probes | 9 passed, 0 failed, 0 ignored |
| New independent mutant controls | 9/9 positive runs and 9/9 compiled assertion kills |
| Both evidence checkers | PASS; exact pins, source hashes, adaptation scope and result counts verified |

Expected historical failures are classified by the actual cargo results and failing
assertions, never by the capture wrapper's zero exit. They do not count as passes against
the superseded requirement, and no original probe was edited to hide them.

| Finalized history | History build (s) | Indexed preflight (µs/call) | Scan control (µs/call) | Whole finalization (ms/call) |
|---:|---:|---:|---:|---:|
| 1,000 | 0.909 | 4.790 | 6.779 | 1.699 |
| 4,000 | 15.207 | 6.054 | 103.572 | 8.027 |
| 16,000 | 154.786 | 2.702 | 1,412.555 | 17.004 |

These measurements include contemporaneous local build load and are not a production
latency guarantee. Indexed preflight avoids the history scan, but whole finalization
remains O(history) through preserved Phase-1 code. Decay walks the retained activation
lineage, and pre-wave digests scan stores. All prior physical-scan limitations remain.

## 8. Open matters, publication and exact next action

**No remaining Gate C2 blocker was found within the adjudicated review scope.**
The four matters in §5 need explicit Operator decisions before being treated as normative
contracts. They are not silently accepted here. AT-I13 remains P: fixed-manifest support
without definition migration. AT-I39/40 remain per-profile composition, not a shared
multi-profile runtime or cross-profile transaction. AT-I21 depth overflow uses the test
seam; AT-I33 maximum is the configured finite bound. Replay/restore are in memory only;
there is no durable persistence, crash recovery, mailbox, transport, service or G.A.M.E.
integration. Windows/Android checks are static, and performance limits remain disclosed.

The candidate's own same-time activation coverage gap is honestly documented; this review
adds evidence for that behavior, not a source repair. Named lost-prewrite and same-time
fixtures could later be adopted by the separated writer if the Operator chooses to make
those behaviors normative. No such writer work is performed or authorized by this verdict.

Publish only this report and E on the named review branch with a normal commit and push.
Verify local HEAD = tracking ref = live GitHub review ref. Verify the candidate remains
544c5f6, production/acceptance remain 7e3a0aa, and every pre-existing branch and worktree
head remains as captured. The final publication receipt supplies the exact review commit
and synchronization result; this document cannot contain its own commit hash.

**Exact next action:** the Operator reviews this published report and its pinned
`544c5f6f99d8dabf9855f8ac68f5666286aa9741` candidate, then explicitly accepts or rejects
Gate C2 with AT-I13, the four unadjudicated behaviors and all retained limits acknowledged.
Any subsequent normative decision is recorded in a new decision document, preserving the
immutable adjudication. No production promotion or Phase-3 work follows automatically.
