# Third-agent substantive independent assessment

Exact candidate: `67b877192cc78b75c6fbe60c69b5594dc10befe8`.
Reviewer: `/root/independent_c2w01`, a separate agent from the correction writer and the
reviewer who raised C2W-01. The coordinator performed custody and standard validation;
this agent independently read the mission, Operator resolution, contract correction,
writer report, controlling review, changed implementation and relevant preserved paths,
and authored and ran the fresh probes and mutation controls named below. No candidate
source or test was repaired or edited. Probe crates and mutated archives are disposable
external directories. Only evidence is written into the review worktree.

**Substantive verdict: `GATE_C2_W01_CORRECTION_KEEP`.** C2W-01 is closed. This is a
bounded conformance review, not the subsequent adversarial campaign and not Gate C2
acceptance. The coordinator's completed exact-candidate validation/custody evidence must
accompany this assessment in the published review.

## Correctness judgment

The corrected selection is faithful to approved behavior 1. On each target/scope group,
all-additive stages still use AddDelta and the reducer's settled pre-value. A mixed group
containing Add or Subtract and no Decay starts with `settled(target, scope, now)`; its
stages fold in declared order. The final RuleBody transform carries a resolved result,
so the reducer does not apply settlement to it again. Transform-first, Subtract-only
additive stages, nonidentity transforms, several stages and derived targets all execute
this same path. There is no apparent special case for the original Add/identity Clamp
or Add/identity Scale vectors.

The presence of Decay is the right test for the accepted debt-consuming-stage boundary
within an admitted group. The stage fold dispatches every Decay to decay_walk with the
existing cell's updated_at. Admission permits at most one such operation per target in
a body and one decay rule per target (`rules.rs`, SecondDecayReducer validation and
`decay_operation`). A zero rate or absent cell may make the walk numerically inert,
but supplies no unpaid debt requiring a second walk. Settling before a declared walk
would reuse the same anchor and charge its endpoints twice; the independent explicit
stage-order vectors distinguish that error. Declared order continues to allow a later
explicit replacement to overwrite preceding calculations and a later declared Decay
to act at its declared position. No implicit replacement-debt policy is added.

Assign and Aggregate overwrite the accumulator wherever declared. The independent
Aggregate tests use committed mood=+/-40, a target initially +/-100, rate 10/cadence 10,
and a body at 15: Aggregate; Subtract(3*sign); Scale(1/2) gives 18 or -19, while
Scale(1/2); Subtract(3*sign); Aggregate gives +/-40. Later evaluation bills only the
point at 20. The same vectors pass on SparkOwned stress and Derived pressure. No
previous decay is charged against the declared replacement.

The transform-only extension is sound under the retained standalone boundary: neither
standalone nor a composition consisting entirely of Scale/Clamp declares an additive
event. It would be a different contract to settle every operation that reads current
state; the approved resolution and controlling review do not require that. Independent
binding Clamp, Scale, and Clamp-then-Scale vectors preserve their committed-input
results, while adding a Subtract to a nonidentity transform-first body settles first.
This boundary is an explicit retained qualification, not hidden deferral of additive debt.

## Independent executed probes

`own_third_probes.rs` is freshly authored, uses public engine requests and the standard
fixture builders, and passes **12/12** on the exact candidate. The final
`own_third_probes_v2.rs` preserves those tests and adds the nonbinding transform-only
discriminator below; it passes **13/13**. Exact commands, exit 0, source digests and
manifest are in `own_third_baseline.json` and `own_third_v2_baseline.json`; complete
output is in the corresponding `.log` files. Each test's concrete hand-valued assertions
are the oracle.

| Test | Discriminator |
|---|---|
| own_nonidentity_transform_first_subtract_derived | Starting +/-101, settle one step, half then subtract twice: +39/-40 at 15 and +29/-30 at 20, on both write authorities |
| own_multistage_no_double_charge | Subtract(3*sign) then double: +/-174 at 15, +/-342 at 16, +/-658 at 25, +/-648 at 30 |
| own_aggregate_replacement_stage_orders | Aggregate replacement in both stage positions, both signs and both write authorities |
| own_assign_order | Assign before/after nonidentity transform and Subtract on derived target |
| own_declared_decay_order | +/-4@5 with rate 3/cadence 6 gives +/-3 for addition then decay and +/-5 for decay then addition at 12 |
| own_transform_boundaries | Standalone half, standalone binding clamp, and binding clamp then half read committed value |
| own_nonbinding_transform_only (v2) | Half then wide clamp gives +/-50@15 and +/-40@20, discriminating erroneous implicit settlement |
| own_barrier_and_removed_lineage | Rate 4/5, barrier 10 to 7/4, body yields +/-95@10; removal 19, body 21 gives +/-84; restore 23, first endpoint 27 gives +/-77 |
| own_refusal_keeps_debt_and_time | Refused Subtract(-20)/half at 6 preserves bounded 9@0 and emits nothing; later valid composed +1 settles both points and gives 8@10 |
| own_watchers_both_directions_single_effect | Derived +/-100 to +/-95 crosses +/-97 in correct direction, emits one target effect and one alarm |
| own_pacing_restore_replay | Identical history at pacing 1,2,3,7,50, with/without restore, agrees at each stop and reconstructs both digests |
| own_same_time_reversal_and_unchanged | Change/reversal at 9 restarts grid while unchanged activation retains grid, with distinct values at 12 and 15 |
| own_absence_and_no_move_time | Single absent-cell decay creates no cell; distinct initializing composition works; zero-rate evaluation updates canonical time |

Atomicity is structural as well as tested: group_intent/settled operate on the immutable
EvalView, plan_wave validates the full reduced wave, and apply_wave is the only write
phase. A refusal cannot advance the store's value or updated_at through the changed
line. Canonical commits exclude already billed endpoints from successive writes. The
barrier and removal tests exercise closing-endpoint ownership and retained-lineage search.
The changed group selector introduces no StateCell field or encoder path. Pacing is
absent from EvalView, so the test budgets are representative evidence backed by that
structural exclusion, not an exhaustive enumeration of every integer budget. Retained
workspace scheduling, encoding, replay and restore tests remain part of the full review.

## Approved behaviors, oracle and prior findings

Behavior 1 is now supported for the changed composed path, in both directions and both
write authorities, including Subtract and nonidentity transform-first bodies. Behavior 2
is supported for both Assign and Aggregate orderings. Behavior 3 is supported for composed
writes within the absent window. Behavior 4 is supported for composed histories with
same-time activations. Behavior 5 is preserved, including distinct initialization.
No new blocking code finding was established.

Inherit every AT-I1 through AT-I50 disposition, subdivisions and exact S/P qualification
from e00f248 section 6 as inherited by e39690d section 5. Preserve supersession of original
I6/I7 and pre-adjudication I22/I23. The modified dependency is the mixed-body starting
accumulator; reducer, store, encoding, watcher, scheduling, activation and persistence
mechanisms are unchanged. Re-establish the affected numeric/decay commitments through
these fresh composed probes and the coordinator's full unchanged workspace/oracle tests:
I22'/I23' get mixed-body coverage; refusal/reduction get mixed-body coverage; I1 effects
and committed watcher transitions are checked in the single-effect vectors and retained
reapplication tests; replay/restore/pacing get new mixed-body histories. The former
behavior-1 qualification is removed because the previously failing composition path now
settles, rather than because it was renamed or deferred. All other retained findings and
limitations, including fixed-manifest I13, per-profile I39/40, configured finite caps,
in-memory replay/restore, static-only Windows/Android coverage and existing performance
limitations, stand. No unrelated settled decision is reopened.

## Nonblocking report accuracy qualifications

1. Writer report section 7 overstates the performance accounting when it describes one
   settlement walk per written target/scope per wave. A normal eligible mixed body calls
   settled in group_intent, and plan_wave then eagerly calls settled again for that
   canonical target. Multiple eligible body evaluations can also perform their own
   read-only calculations before canonicalization. This is repeated computation, not
   repeated committed charging: RuleBody consumes its resolved value, and reduce consults
   the eager settled map only for AddDelta. Correct the review's performance description
   accordingly; the inherited unbounded-lineage/performance limitation remains. No repair
   is required for this documentation qualification, and this assessment is the additive
   correction to the report's factual claim.

2. W01-settle-every-body is not literally equivalent by itself to the old unconditional
   RES-settle-composed-body mutation: it retains !declares_decay. It tests the no-decay
   transform-only partition. W01-settle-decay-body separately removes that partition's
   exemption and tests explicit-stage double charging. Together these retain the relevant
   behavioral discrimination of the old unconditional mutant; substitution is acceptable
   as complementary split coverage, not as a claim of identical patches or identical
   executable semantics. The coordinator's fresh runs establish compilation/assertion
   status; a compile failure must never count as a kill.

## Exact next action

After publication of independent KEEP, proceed to the separately recorded adversarial
campaign against `67b877192cc78b75c6fbe60c69b5594dc10befe8`. This assessment does not start
that campaign, accept Gate C2, promote production or authorize Phase 3.

## Final independent mutation results

`own_final_mutation_summary.json` records **14 compiled runs of 13 distinct wrong-
implementation patches, each in its own extracted candidate archive and target directory**.
Twelve runs fail by assertions (cargo exit 101), and two initial runs survive (cargo exit 0).
No compile failure is counted as a kill. Each `OWN-*.json` supplies the exact patch,
command, target directory, test source SHA-256, exit code and failing test names; its
matching `.log` retains complete output. The scripts `own_run_mutants.py`,
`own_run_extra_mutants.py` and `own_run_refined_control.py` reproduce the controls.

The assertion-killed wrong implementations omit Subtract from the additive predicate,
exempt transform-first bodies, settle after folding rather than before, charge the
starting debt twice, settle explicit-decay bodies, reduce Aggregate by old debt,
compare watchers to the settled intermediate, backdate derived commits, suppress
atomic refusal, search only the current epoch, and drop the closing barrier endpoint.
The transform-only over-settlement patch completes the twelve distinct killed patches
with the v2 refinement described below.

The two surviving runs are explicitly **not kills**:

- `OWN-settle-transform-only` survived v1 because the binding Clamp in its composed
  boundary probe mapped both settled and committed starting values to the same bound.
  That was a test-coverage gap. Preserve the original pass and add the nonbinding
  half-then-wide-clamp vector in `own_third_probes_v2.rs`. The same mutation, recorded
  as `OWN-settle-transform-only-v2`, compiles and fails that new assertion. The final
  candidate passes all 13 tests. This is evidence refinement, not a candidate repair.
- `OWN-lineage-first` survives because these fixtures preserve the same decay operation
  rule/sub-ID identity across epochs. Selecting oldest or latest supplies the same
  operation identity to the unchanged full-lineage walk. It is not claimed to be safe
  generally or killed. The directly relevant removed-window bug is independently
  discriminated by `OWN-current-epoch-only`, which fails `own_barrier_and_removed_lineage`.

The final assessment remains KEEP with the factual qualifications above. Full standard
validation is reported by the coordinator as completed: 15/15 gates, 427 workspace tests,
all required corpora and writer controls, on the exact candidate. Those results remain
separately attributed rather than relabelled as this agent's own test execution.
