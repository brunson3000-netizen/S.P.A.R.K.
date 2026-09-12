# Gate C2 C2W-01 correction — independent review

Date: 2026-09-12. Exact candidate: `67b877192cc78b75c6fbe60c69b5594dc10befe8`, on
`candidate/phase2-gate-c2-w01-correction-20260912`.

**Verdict: `GATE_C2_W01_CORRECTION_KEEP`. C2W-01 is closed.**
The correction passes the original reproductions and the third reviewer's broader probes.
No new blocking code defect was established. Two nonblocking report-accuracy qualifications
are recorded in §3. KEEP does not accept Gate C2; the next step is the separately recorded
adversarial campaign against this exact candidate.

Substantive reviewer: the separate Codex agent `/root/independent_c2w01`, independent of
the candidate writer and of the agent that raised C2W-01. The original finding author,
`/root`, coordinated custody, standard validation, prior-corpus/writer-mutant reruns and
publication. This satisfies the mission's third-agent requirement; coordinator checks are
not relabelled as a third independent judgment. The substantive review and new tests are
recorded separately in the evidence alongside this report.

Review branch: `review/phase2-gate-c2-w01-correction-independent-20260912`.
Evidence E: `gate_c2_w01_independent_evidence_2026-09-12/`.
The publication receipt supplies the full containing commit and local/tracking/live equality.
No candidate repair, Gate C2 acceptance, production promotion or Phase-3 work occurred.
The adversarial campaign is a separately authorized next step, not part of this review.

## 1. Authority, lineage and preservation

The Operator's 2026-09-12 resolution controls additive settlement and behaviors 2–5;
the immutable 2026-09-11 D-1–D-7/S-1–S-13 adjudication controls fixed grids, barriers,
canonical time and retained stage-order compositions. Otherwise inherit controlling review
`e39690dc51b63fa09fe7c3f30ebd48aa2c16686f` §1 precedence, including e00f248 §1. No settled
decision is reopened without a changed dependency. Withdrawn Phi, SH-1/SH-2 and
architectural R-8 remain withdrawn.

Live GitHub heads were captured before validation. The fetched candidate is the exact
requested hash. All listed ancestors are reachable from that live candidate:

| Commit | Verified role |
|---|---|
| `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` | Production/acceptance freeze |
| `cc3dc70182b428f7cbc953ae722f85b39d407dfa` | Previously reviewed candidate |
| `e39690dc51b63fa09fe7c3f30ebd48aa2c16686f` | Controlling review, direct base |
| `bda58aa9e43dc571deefa508ec12364dba0a4ad0` | First child; contract correction alone |
| `d1d9f8e55c33ecb5dddccc0e4799cd8f9aaccb40` | Production/test checkpoint |
| `67b877192cc78b75c6fbe60c69b5594dc10befe8` | Final report/mission/evidence publication |

**The final edge adds exactly 55 documentation/evidence files under engineering/phase2,
and changes no existing file.** Both checkpoint and final candidate have crate tree
`7907f4d729104fd5dbfd4adad46e66cf09aa13dd`. E/audit.py compares Git blob identities directly;
all **782** inherited engineering files are byte-identical. The contract correction is
byte-unchanged since its first, document-only commit. It explicitly withdraws the earlier
unapproved deferral and corrects the inaccurate double-charge commentary, while preserving
the original contract/report, adjudication, Operator resolution, review and evidence.

The rerun preservation checker passes: exactly two changed crate paths, one replaced
production line (+24/-1 in engine.rs), plus the new 12-test integration file. spark-core,
all manifests/lockfile, seven inherited integration files, 17 inline Phase-1 modules and
ten protected functions are byte-identical, including decay_walk, baseline_of, settled,
settlement_operation and effects::reduce.

Review used a fresh isolated clone/main worktree at
`/tmp/spark-c2w01-independent-review-20260912`, detached at the final candidate throughout
validation. Existing repository refs, worktrees and status were captured before testing and
compared again before publication. No reset or checkout of an existing worktree occurred.

## 2. C2W-01 and independent substantive assessment

The third reviewer's full reasoning, sources and vectors are in
[E/own_assessment.md](gate_c2_w01_independent_evidence_2026-09-12/own_assessment.md).
The changed selector in engine.rs settles mixed bodies containing Add **or Subtract** with
no Decay, before folding all stages in declared order. It is independent of stage position,
transform identity, target authority and the original two reproduction shapes. All-additive
bodies still use the already-correct AddDelta reduction path.

Within an admitted target group, testing for a declared Decay stage correctly identifies
the retained explicit debt-consuming path: every such stage dispatches to decay_walk with
the cell's existing updated_at, and admission limits decay operations to one per target.
Implicitly prepending another walk would double bill endpoints. The declared stage order
remains explicit, including replacement stages overwriting preceding calculations. Assign
and Aggregate supersede the settled accumulator wherever declared; old debt is not applied
afterward as an implicit reduction of a replacement.

Transform-only bodies consistently retain the accepted standalone Scale/Clamp boundary:
there is no additive event. This does not exempt a body that contains Add/Subtract. The
third reviewer found no new deferral of additive debt in the exercised admitted shapes.

| Approved behavior / boundary | Independent disposition |
|---|---|
| 1 Additive settlement | **PASS; C2W-01 closed.** Original 95@15→85@20 and recovery mirrors pass. Fresh nonidentity transform-first Subtract bodies from +/-101 give 39/-40 at 15 and 29/-30 at 20, on SparkOwned and Derived targets. Repeated multi-stage bodies give +/-174@15, +/-342@16, +/-658@25, +/-648@30. |
| 2 Explicit replacement | **PASS.** Aggregate committed +/-40 then Subtract and half yields 18/-19; moving Aggregate last yields +/-40. Assign orderings also pass on a Derived target. No implicit old-debt reduction follows a replacement. |
| 3 Removal/restoration | **PASS.** Own Derived-target history: barrier 10 changes 4/5 to 7/4; composed write gives +/-95@10, removal at 19 then body at 21 gives +/-84; restoration 23 gives +/-77 at its first endpoint 27. |
| 4 Same-time activations | **PASS.** Change/reversal at 9 restarts the grid; unchanged parameters preserve it, with differing composed-write results at 12 and evaluations at 15. |
| 5 Absent cell | **PASS.** Single absent-cell decay creates neither cell nor effect; a distinct composition initializes it, and later zero-rate evaluation commits at canonical time. |
| Explicit Decay bodies | **PASS.** From +/-4@5, rate 3/cadence 6 at 12, addition-then-decay yields +/-3 and decay-then-addition yields +/-5. The endpoints are charged once at the declared stage. |
| Standalone / transform-only Scale/Clamp | **PASS as retained boundary.** Binding Clamp, half Scale and Clamp-then-Scale retain committed-input results, in both directions. Adding a Subtract switches to settled input. |
| Refusal and watchers | **PASS.** Refused composed update keeps bounded 9@0 and emits nothing; later valid write bills both endpoints to give 8@10. Derived +/-100→+/-95 crosses +/-97 in the proper direction, with exactly one target effect and one alarm. |
| Replay/restore/pacing | **PASS for tested histories/budgets.** Pacing 1,2,3,7,50 with and without restore agrees at every stop and reconstructs digests; structural exclusion of pacing from EvalView supports inheritance beyond this finite set. |

Settlement remains read-only until the fully validated wave commits. The original timestamp
is untouched on refusal, while successful writes commit updated_at=now and exclude billed
endpoints from later walks. The changed code introduces no encoder, StateCell field,
backdated commit, watcher protocol or family-conflict exception.

## 3. Nonblocking report-accuracy findings

**C2W-RN01 — mutation substitution is split, not individually equivalent.** The writer's
16 carried patch tuples are byte-identical to their predecessors. `W01-settle-every-body`
retains `!declares_decay`, so it changes only non-decay bodies; it is not itself equivalent
to the removed unconditional `RES-settle-composed-body`. `W01-settle-decay-body` supplies
the explicit-decay double-charge error separately. Their combined tested coverage retains
the old error and extends it to transform-only bodies. This review records that precise
mapping rather than treating the label as proof. No mutation is counted as killed by
failure to patch or compile. The old literal patch would fail its match assertion after
the production-line replacement; that would not be a semantic kill.

**C2W-RN02 — two pure walks, not one.** Writer report §7 says composed settlement walks
lineage once per written target/scope per wave. For an ordinary mixed additive body,
group_intent now calls settled to obtain its starting value, and unchanged plan_wave
calls settled again for every canonical target. The RuleBody reducer consumes the already
resolved transform value; it does not apply the second calculation to that value. This is
repeated computation, not double billing or an unreported commit. No new scan class is
introduced, but the claimed walk count understates work. Record this limitation accurately;
it does not justify an unsolicited optimization/repair in this review.

## 4. Exact-final validation and evidence quality

Every candidate validation command ran with HEAD at
`67b877192cc78b75c6fbe60c69b5594dc10befe8`, not merely at d1d9f8e. External probes link to
that unchanged tree. Mutation archives are created from its exact HEAD, with one target
directory per mutant. E/checks.json and full logs retain commands, actual exits and timing;
metadata stdout is retained. Storage settings disable dev/test debug info and incremental
compilation and use two Cargo jobs; release optimization is unchanged.

| Fresh check | Result |
|---|---|
| fmt | PASS |
| Full workspace, all features, no-fail-fast | **427 passed; 0 failed; 0 ignored** |
| All-target/all-feature clippy, warnings denied | PASS |
| Strict core/engine all-feature library lint | PASS; warnings, unwrap, expect, panic, indexing/slicing and arithmetic-side-effects denied |
| cargo metadata | PASS; full stdout retained |
| Review/production/reviewed-candidate diff --check | All PASS |
| Windows x86_64 GNU/MSVC; Android aarch64/armv7/x86_64 | All five PASS, static only |
| Preservation | PASS, no failures |
| Release offline gate_c2_workload | PASS |
| Controlling review's own probes/materialization | 9/9 and 1/1 PASS, byte-unchanged |
| Historical adjudication corpus | 8 passes, exactly 1 superseded assertion failure; cargo 101 |
| Resolution-adapted / decision-adapted | 9/9 each PASS |
| Writer mutation rerun | 42 positive baselines; 20/20 compiled mutants killed; 57 assertion kills; 2 specified prior-test survivals |
| Third-reviewer probes and mutants | 13/13 final probes PASS; 14 compiled mutant runs of 13 patches, 12 assertion kills and 2 disclosed initial survivals |

All **15 required gate commands** passed. E/validation-summary.json checks the exact pins,
counts, preservation and isolated mutation builds. The third reviewer's two surviving runs are not counted as kills. A binding Clamp masked
transform-only over-settlement in v1; the added nonbinding v2 probe kills the same patch.
An oldest-lineage-identity lookup survives the stable-identity fixtures and remains an
explicit coverage limit; the mission-relevant current-epoch-only removal bug is killed.
Both source versions, baseline results and all mutation outcomes are retained. This finite
coverage does not claim correctness for every possible operation-identity history.

The writer's 16 retained patch tuples
were compared directly, rather than inferred from test results. Full baseline output was
added to the copied runner; its candidate patches were not changed.

| Finalized history | Build seconds | Indexed preflight µs | Scan control µs | Whole finalization ms |
|---|---:|---:|---:|---:|
| 1,000 | 0.516 | 2.680 | 6.495 | 1.618 |
| 4,000 | 12.675 | 2.642 | 49.393 | 6.734 |
| 16,000 | 257.470 | 2.498 | 1514.440 | 26.544 |

This run included contemporaneous build load. It is not a production latency guarantee;
O(history) finalization and pre-wave store scans remain, with the extra pure-walk accounting
qualification in §3.

The controlling review's own_probes.rs and own_materialized.rs were read byte-unchanged
into disposable crates; their source SHA-256 is in the logs. The original two C2W-01 failures
now pass: 95@15 then 85@20, and -95@15 then -85@20. The historical adjudication-review corpus
remains byte-unchanged and fails only `own_open_lost_prewrite_step`, with cargo exit 101.
That superseded observation is historical failure evidence, not a normative pass. Both
resolution-adapted and decision-adapted copies retain their prior passing dispositions.
Capture wrappers' exits do not decide test outcomes.

## 5. Oracle and retained dispositions

Inherit **every row of e00f248 §6, AT-I1–AT-I50 including lettered subdivisions**, with its
exact S/P qualifications, as carried by e39690d §5. Original unsplit I6/I7 and
pre-adjudication I22/I23 remain superseded. The third reviewer explicitly endorses this
inheritance: only the mixed-body starting accumulator changes; protected reduction,
store, scheduling, activation, encoding and watcher mechanisms remain byte-preserved.
The full workspace suite was rerun on the exact final candidate.

Re-established affected dependencies: I1 reported effects/reapplication; I7a/d and I11
reduction/refusal; I22′/I23′ settlement, fixed grids, explicit stages and barriers; I25
committed crossings; I28/29/43 replay/restore/request timing; I34 preservation; I36 static
targets. The former qualification that approved behavior 1 was incomplete is now removed
on executed evidence, not by renaming or deferral.

Retain C2-01–C2-08, C2R-01–C2R-04, C2R2-01, D-C2-1–D-C2-15, R-1 and R-3–R-9 as
previously dispositioned, and the AT-I8/20b/21/28/33/39/40 completions. Enqueue-canonicalization
R-8 remains distinct from withdrawn architectural R-8. C2-09 retains its history/finalization
performance limitation. AT-I13 stays P: fixed-manifest support without definition migration.
I39/40 remain per-profile, not shared-runtime or cross-profile transactions. I21 overflow
uses the test seam; I33 maximum is the configured finite bound. Replay/restore are in memory;
no durable persistence, crash recovery, mailbox, transport, service or G.A.M.E. integration
is claimed. Windows/Android checks are static, not runtime parity. No unrelated settled
finding is reopened.

## 6. Exact next action

**Proceed to the separately recorded adversarial campaign against
`67b877192cc78b75c6fbe60c69b5594dc10befe8`**, as specified in
`SPARK_PHASE_2_GATE_C2_OPERATOR_NEXT_AUTHORIZED_STEP_2026-09-12.md`.
Independent testers record reproducible bugs and evidence on their own branches; the main
engineer triages and repairs under writer discipline. This review does not begin that
campaign. Gate C2 acceptance remains the Operator's decision, informed by the campaign.
Do not promote production or begin Phase 3.
