# S.P.A.R.K. Phase 2 — Codex Independent Adversarial Architecture Review

**Date:** 2026-08-26  
**Review type:** bounded independent architecture review; no production implementation  
**Repository:** `/home/chromikey/Projects/SPARK`  
**Branch:** `phase1-refoundation-v2`  
**Review base HEAD:** `87ba1c22b4b1634e5205fb14bb6806a886df3f0b`  
**Primary subject:** same-target causal semantics in the frozen Phase-2 `EffectBatch`

## 1. Primary verdict

`PHASE_2_ARCHITECTURE_REVISE`

Phase-2 implementation must not begin from the current Q3 rule. The defect is local to
the Phase-2 rule/effect architecture and does not reopen Phase 1, but it is load-bearing
inside Phase 2 because it changes ordinary causal meaning, effect identity, backpressure
equivalence, delayed materialization, aggregation, threshold emission, and the AT-I
oracle.

Phase 3 remains unauthorized. This review changes no production Rust and proposes no new
causal primitive or general scripting facility.

## 2. Scope and controlling record

The review reconciled the frozen decision against the following controlling lineage, in
the blueprint's precedence order:

1. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md`, especially §§3.1–3.2,
   6.4–6.7, 11.2–11.3, 12.2–12.5, 17, 19.3–19.5, 26.1/26.5, 28, 29, 30.2,
   31, and 32;
2. Phase-0 ADRs, especially ADR-0002 (authority/write classes), ADR-0003 §14–15
   (stable command barriers, deterministic waves, quota/yield semantic neutrality),
   ADR-0004 (declared update semantics and change classes), and ADR-0006 (delayed
   obligations, replay modes, bounded provenance);
3. `engineering/phase0/PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`, especially R-026,
   R-037, R-038, R-043, R-044, R-055, R-086, and R-096;
4. `engineering/phase0/PHASE_0_PERFORMANCE_AND_SECURITY_BUDGET_v0.2.md` §3;
5. the closed Phase-1 architecture and closure record, particularly the R1/R2 law,
   semantic occurrence identity, conflict poisoning, authority door, and the final
   `PHASE_1_CLOSED` review;
6. `engineering/phase2/PHASE_2_FABLE_READINESS_BLUEPRINT_2026-08-26.md`;
7. `engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_2026-08-26.md`;
8. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_2026-08-26.md`.

The Rust tree was inspected only to verify the Phase-1 seams claimed by the readiness
and freeze documents. There is no Phase-2 production implementation to review.

## 3. Exact defect

The freeze evaluates each rule into an absolute, resolved `CanonicalValue`, then treats
the target/value pair as both causal identity and conflict identity:

- same target + same value is deduplicated;
- same target + different value rejects the whole wave.

That equivalence is false. Equal values do not prove duplicate causes, and unequal
values do not prove incompatible causes.

For a pre-wave `stress = 20`:

```text
cause A: add +10 stress
cause B: add +15 stress
```

each rule independently resolves against the required stable snapshot:

```text
A -> Set(stress, 30)
B -> Set(stress, 35)
```

Q3 sees two distinct absolute values and rejects the wave. The intended operation-level
meaning, however, is two independent additive contributions:

```text
20 + (10 + 15) = 45
```

The equal-magnitude variant is more revealing:

```text
cause A: add +10 stress
cause B: add +10 stress
```

Q3 deduplicates the two `30` values and commits `30`, even though two distinct causal
occurrences contributed a total of `+20` and the correct additive result is `40`.
Thus the current rule preserves or erases a cause according to numeric coincidence.

The architecture has discarded the information needed to tell these cases apart before
it performs conflict analysis. A resolved absolute value is sufficient for final commit
and outcome replay, but it is not sufficient as the candidate effect presented to a
same-target causal resolver.

## 4. Why the defect matters

### 4.1 It contradicts the controlling causal model

The blueprint defines S.P.A.R.K. as a causal-pressure engine that propagates state
changes through bounded declarative rules, requires accumulating state and stacked
pressure, and explicitly includes `add/subtract/scale/clamp`, weighted sums,
decay/recovery, aggregation, and threshold emission. The Pontafique/Lindemar reference
loop depends on many local causes accumulating into aggregate pressures.

The blueprint does not freeze a single-writer-per-cell model. It requires each
definition to declare update semantics. Requiring authors to discover every potentially
simultaneous cause and route it through an ad hoc, manually centralized rule would make
ordinary causal composition an authoring convention rather than a property of the
frozen grammar. It would also make adding a new profile-data cause capable of turning a
previously valid wave into rejection without changing authority, type, scope, or bounds.

Explicit aggregation remains the right model for a derived total computed from a known
input set. It is not an adequate universal substitute for independent additive events
targeting an accumulating S.P.A.R.K.-owned state.

### 4.2 It confuses duplicate, compatible, and conflicting claims

The Phase-1 scheduler precedent does not justify Q3. A `WorkKey` collision means two
different payloads claim one semantic occurrence slot; neither may win. Two effects on
one cell do not necessarily claim one occurrence slot. They can be two distinct causes
whose declared update algebra admits both.

The correct distinctions are:

| Case | Required treatment |
|---|---|
| Exact replay/delivery of the same causal emission identity and payload | idempotent duplicate; apply once |
| Same emission identity with different payload | malformed/contested emission; reject atomically |
| Distinct emission identities with additive deltas, including equal deltas | independently composable; fold all deltas |
| Distinct exclusive assignments of the same value | compatible result; one write with merged provenance |
| Distinct exclusive assignments of different values | genuine conflict; reject atomically |
| Different update families without a frozen cross-family rule | cannot safely coexist; reject or reject the rule set at activation |
| Non-commutative/order-sensitive transforms | require one explicitly declared reducer or a semantic barrier; never resolve by rule ID/order |

Idempotency must therefore be keyed by a stable causal emission identity, not by
`(target, resolved value)`. At minimum it must distinguish behavior artifact/rule,
producer scope, occurrence, effect sub-ID, and target. Two rules producing the same
delta are not duplicates merely because their numbers match.

### 4.3 It violates partition/backpressure promises

Suppose A and B are evaluated together. Q3 rejects. If `max_due_per_cycle` or a depth
deferral separates them into stable successive waves, A can commit `30` and B can later
commit `45`. Runtime quota partition has changed an already accepted command's causal
result.

That contradicts:

- ADR-0003 §15: quota/yield changes latency, not semantics;
- the Phase-0 performance/security budget §3: a runtime work quota may not change
  already accepted command semantics;
- R-038/R-044's quota-resume and split-batch equivalence;
- AT-I20 and AT-I21, which already require deferred and unbudgeted execution to converge.

The existing AT-I20 does not include co-targeting work or threshold side effects, so it
can pass while the governing invariant is false. Opposing additive contributions are an
especially strong falsifier: a partial wave can cross a threshold and enqueue an
irreversible obligation even when the full simultaneous fold would not cross it.

### 4.4 Whole-wave rejection is the wrong response to a valid cause set

Atomic rejection remains correct for an invalid effect, arithmetic overflow, authority
violation, impossible scope, contested emission identity, or genuinely incompatible
updates. It is not correct for ordinary independent additions.

A rejected finalized barrier does not merely delay the pressure: it loses the state
transition while canonical input history advances. Re-evaluating a clone may reproduce
the rejection report, as AT-I9 requests, but reproducibility does not restore the causal
result. Calling every such wave an ill-formed profile hides a semantic limitation behind
transaction safety.

## 5. Operation-by-operation ruling

The grammar needs an explicit same-target composition contract, but it does not need a
general expression language. The smallest safe v1 contract automatically composes only
the operation family with an unambiguous required meaning and makes every other
coexistence explicit.

| Operation | Same-target ruling before implementation |
|---|---|
| `add` | Emit an additive delta, not an absolute result. Distinct emissions sum in checked `i128`; add to the pre-wave value once. |
| `subtract` | Normalize to a signed additive delta and use the same fold. Checked negation is required at the numeric edge. |
| `scale` | A single scale inside one reducer is deterministic. Multiple scales or scale mixed with deltas are ordering/rounding-sensitive unless a specific reducer defines their joint meaning. The minimal rule makes such coexistence exclusive rather than inventing product, percentage-sum, or ID-order semantics. |
| `clamp` | A clamp inside one reducer or the definition's final bounds is deterministic. Multiple clamp operations could be intersected, but mixing clamp effects with other writers needs a frozen stage rule. The minimal rule keeps clamp in the single reducer/final validation path. |
| weighted sum | The weighted-sum operation is already a deterministic internal reducer: accumulate all declared terms in `i128`, round once. Its output must declare whether it is an exclusive derived value or one additive contribution. Separate complete weighted-sum assignments do not auto-compose. |
| decay/recovery | This is a baseline-seeking state transition, not automatically an independent additive event. Permit one declared decay/recovery reducer for a target/cadence. A simultaneous shock plus recovery must be combined by that reducer or separated by an explicit semantic boundary. The exact closed-form formula and rounding must be frozen. |
| aggregation | Use one authoritative derived reducer per aggregate target/epoch. Independent children are reducer inputs, not competing absolute writes. Multiple complete aggregate assignments require equality or conflict. |
| threshold emission | Compare the prior committed aggregate with the one reduced candidate result. Emit at most once per declared crossing identity; never observe partial same-target folds. |

This ruling deliberately does not choose a universal interpretation for `Add + Scale`,
two percentage scales, recovery plus shock, or assignment plus delta. Those meanings are
not algebraically interchangeable. Profile authors may express them inside one bounded
typed reducer where order/stages are declared, or represent separate causal components
and aggregate them. The engine must not silently derive meaning from rule declaration,
insertion, hash, or commit order.

## 6. Smallest safe architectural correction

Keep the invariant **one committed resulting value per target cell per wave**, but stop
requiring every candidate cause to be an already resolved absolute value.

Introduce an internal type distinction within the existing `EffectBatch` primitive:

```text
CandidateEffect
  target
  write_path
  emission_identity
  update_intent = AddDelta(value) | ExclusiveResult(value)
                  | ExclusiveTransform(closed_typed_operation)
  logical_time / behavior_epoch
  bounded provenance with coverage metadata

CommittedEffect
  target
  write_path
  resolved CanonicalValue
  contributing emission identities/provenance summary
  logical_time / behavior_epoch
```

This is not a new causal primitive. It is the minimum internal representation needed to
implement the already frozen `PropagationRule -> EffectBatch` primitive correctly.

For each wave:

1. evaluate every rule only against the pre-wave stable snapshot;
2. canonicalize candidates by `emission_identity`:
   exact identity+payload duplicates are idempotent, while one identity with distinct
   payloads rejects the wave;
3. group candidates by target in canonical target order;
4. resolve each group using a closed target-local rule:
   - all `AddDelta`: checked `i128` sum, then one checked addition to the pre-wave value,
     then final type/bounds validation;
   - all `ExclusiveResult`: all values must be equal, otherwise conflict;
   - one exclusive transform: execute it once against the snapshot;
   - any other mixture or multiple exclusive transforms: reject as incompatible unless
     one activated typed reducer already combined them into a single candidate;
5. produce at most one `CommittedEffect` per target;
6. validate all committed effects and all associated ledger/obligation/queue mutations;
7. apply the fully preflighted state transition atomically in canonical target order.

The example then deterministically becomes `45` in every rule/effect enumeration order.
Two distinct `+10` causes become `40`; an exact retry of the same `+10` emission remains
`30`.

Activation must reject statically provable incompatible co-target writers. Dynamic
condition/scope overlap that cannot be decided at activation remains a typed atomic wave
rejection. This preserves defensive atomicity without using rejection as the normal
composition mechanism.

The batch digest must commit to the canonical candidate-resolution result sufficiently
to distinguish distinct causal emission sets when that distinction appears in returned
explanation/provenance. Outcome replay may apply the committed resolved batch; mechanism
replay must reproduce candidate identities and the reducer result under the exact
artifact.

## 7. Required supporting clarifications before Opus

The primary correction exposes several nearby freeze gaps that must be closed in the
same architecture revision. None implicates Phase 1.

### 7.1 Define the causal cohort that a runtime budget may not split

Q4 currently selects an arbitrary stable-`WorkKey` prefix, while Q3 speaks of one wave
from one snapshot. The freeze must state whether one due work item, all equal-time due
items, or one trigger/propagation closure forms the atomic evaluation cohort.

Whatever cohort is chosen:

- same-cohort candidate effects must be reduced together;
- a work quota may defer only at cohort boundaries;
- the cohort must have a finite declared cap;
- exceeding a non-deferrable cohort cap rejects before partial canonical mutation;
- deferred depth continuation must retain canonical priority so unrelated work cannot
  interleave and change a result that AT-I21 claims is equivalent.

Stable ordering is necessary for deterministic processing; it is not permission to make
an otherwise simultaneous, non-commutative operation order meaningful.

### 7.2 Preflight obligation, scheduler, and ledger mutations

Q3 promises that rejected waves enqueue no obligation and move no ledger. Q6 calls
scheduler batch atomicity vacuous because `schedule` is total, then proposes item-by-item
enqueue during commit. That proof omits failure from the finite obligation-queue cap,
`ObligationStore` insertion/invariant checks, occurrence exhaustion, and any later
canonical validation.

All cell writes, occurrence allocations, cooldown changes, obligation records, and
scheduler claims generated by a wave must be validated as one candidate state transition
before the first mutation. No rollback mechanism is required if preflight is complete.

### 7.3 Make obligation collision storage representable

The freeze says `ObligationStore` is keyed by `WorkKey::identity_digest()`, but also says
two distinct records claiming that key are retained until the poisoned drain removes
both. A single-value map keyed only by the work identity cannot represent both records.
The architecture must require either a bounded claim set per work identity plus
content-addressed records, or an equivalent order-independent structure. The engine
digest must commit to all behavior-relevant tracked claims, following the Phase-1 R1/R2
lesson.

For `MaterializedEffect`, one singular `creator_definition_fingerprint` is insufficient
when the frozen effect list targets multiple definitions. Bind every target definition
fingerprint (canonical ordered set) or bind a content-addressed schema artifact that
proves the same fact.

### 7.4 Canonically allocate multiple occurrences for one ledger key

AT-I16 covers interleavings of independent ledger keys only. If one wave creates two
obligations under the same `(profile, producer, scope, work kind)` sequence, assigning
occurrences in evaluator insertion order violates rule-order independence and can swap
payload identities under scheduler keys.

Either structurally allow at most one allocation for a ledger key per cohort or sort
allocation claims by their complete semantic emission identity before allocating a
checked consecutive range. The exact rule must be in the freeze and digest tests.

### 7.5 Complete bounded provenance semantics

Q3 caps merged rule IDs at `MAX_SOURCE_REFS` but does not carry the ADR-0006 coverage
contract. When distinct causes exceed the cap, the effect/report must include complete/
truncated/unknown coverage, retained count, omitted count where knowable, and the frozen
pruning-policy identity. Otherwise different cause sets can collapse to the same
presentation without an honest truncation signal.

### 7.6 Pin the delayed-boundary rule

Static zero-delay cycle rejection is sound. Depth overflow and delayed feedback are
sound only if “next due boundary” cannot be drained again inside the same stable barrier
as a zero-time recursion loophole. The revised freeze must bind it to a strictly later
logical due time or a precisely identified later canonical barrier, and AT-I21 must
prove no same-barrier execution.

## 8. Review of the other frozen decisions

| Frozen area | Adversarial result | Required action |
|---|---|---|
| Stable-snapshot isolation | Sound borrow-split mechanism. The defect is loss of operation intent after the read, not snapshot instability. | Keep; candidate reducers read only the pre-wave snapshot. |
| Rule/declaration/insertion-order independence | Goal is sound; current Q3 achieves it by rejecting/erasing valid causes. | Preserve through commutative additive fold and exclusive-family rejection. |
| Deterministic effect ordering | Sound for ordering target groups and reports. Ordering must not resolve non-commutative same-target operations. | Sort groups/results, not causal meaning. |
| Conflicting-effect handling | Too broad. | Restrict conflict to contested emission IDs, distinct exclusive results, incompatible families, invalid effects, and arithmetic/type/scope/authority failures. |
| Atomic batch rejection | Sound and required. | Keep, with full state-transition preflight including ledgers/queues. |
| Authority enforcement | Sound and consistent with ADR-0002. | Keep existing activation door/write paths; no Phase-1 change. |
| Delayed-obligation identity | Artifact/mode distinction is sound; collision representation and multi-target fingerprint binding are incomplete. | Clarify per §7.3. |
| Occurrence indexing | Persisted per-semantic-key indexes are sound; same-key multi-allocation ordering is unspecified. | Clarify per §7.4. |
| Scheduler collisions | Phase-1 poisoning is sound for one `WorkKey`. It is not the model for composable cell contributions. | Keep scheduler semantics; make `ObligationStore` mirror contested claims. |
| Zero-delay cycle rejection | Sound static invariant. | Pin deferred work to a genuinely later boundary. |
| Delayed feedback | Sound when artifact-bound and later-boundary execution is explicit. | Extend AT-I21 with interfering work and boundary identity. |
| Fan-out/causal-depth limits | Static + dynamic division is sound in principle. | Define unsplittable bounded cohorts and preflight overflow. |
| Queue/backpressure | Current stable-prefix rule is underspecified against simultaneous effects and thresholds. | Defer only at causal-cohort boundaries; prove semantic equivalence. |
| Decay/recovery | Baseline/rate/epoch model is plausible; simultaneous writers and exact catch-up formula are underfrozen. | One reducer per target/cadence; pin formula, rounding, and interaction policy. |
| Aggregation | Pure stable-order full recomputation with no hidden accumulator is sound. | One authoritative reducer per aggregate target; compose children as inputs. |
| Threshold emission | Committed-state memory is sound. Partial folds can create false crossings. | Compare old committed value to one fully reduced candidate value. |
| Canonical/hidden-retained-state equivalence | The R1/R2 law and store inventory are sound. | Extend it to candidate-cohort/obligation collision state and provenance coverage. |

## 9. Exact acceptance-test changes required

The current AT-I matrix must be revised before it becomes an implementation oracle.
The following are mandatory changes, not optional coverage suggestions.

### 9.1 Replace the incorrect duplicate/conflict oracle

Replace AT-I6 with two tests:

1. **`exact_emission_duplicate_is_idempotent`** — the same emission identity and
   payload delivered twice applies once; a reused emission identity with a different
   payload rejects atomically.
2. **`distinct_equal_contributions_are_not_deduplicated`** — two distinct rule/
   occurrence identities each contributing `+10` to `20` produce `40`, with both
   sources represented subject to bounded provenance.

Replace AT-I7 with:

1. **`simultaneous_additive_causes_fold_from_one_snapshot`** — `20 + 10 + 15 = 45`
   for every rule, candidate, and internal evaluation permutation; exactly one committed
   target write.
2. **`distinct_exclusive_values_reject_the_whole_wave_atomically`** — two exclusive
   assignments of `30` and `35` reject with order-independent evidence and no state
   mutation.
3. **`equal_exclusive_values_coalesce_with_merged_provenance`** — distinct exclusive
   causes assigning the same value commit once without being mislabeled as delivery
   duplicates.
4. **`incompatible_update_families_reject_atomically`** — assignment+delta,
   undeclared add+scale, and multiple exclusive transforms reject in every permutation.
5. **`additive_fold_overflow_or_bounds_failure_is_atomic`** — checked `i128` fold,
   final conversion, and target bounds fail before any cell/ledger/queue mutation.

### 9.2 Strengthen snapshot, ordering, and threshold tests

- Extend AT-I1 so overlapping same-target add/sub candidates prove that all deltas read
  one pre-wave value and the fold is applied once.
- Change AT-I5 to assert canonical target-group/resolved-result order. It must also prove
  that changing rule IDs or their lexical order cannot change an incompatible operation
  into an accepted one.
- Extend AT-I3 across permutations containing equal and unequal additive contributions.
- Extend AT-I25 with multiple simultaneous causes: threshold crossing is computed once
  from old committed value to fully reduced candidate; no partial-fold false emission.

### 9.3 Pin every operation class

Add a table-driven test covering same-target pairs and permutations of:

```text
add/add
add/subtract
scale/scale
add/scale
clamp/clamp
add/clamp
weighted-result/weighted-result
decay/add
decay/decay
aggregate-result/aggregate-result
```

Each pair must have one explicit expected class: additive fold, equal exclusive
coalescence, activation rejection, or runtime atomic incompatibility. No test may accept
“whichever sorted effect runs first.”

### 9.4 Make deferral equivalence adversarial

- Extend AT-I20 so a stable-prefix cut would otherwise split co-target causes. Compare
  unbudgeted and deferred execution including cells, scheduler, obligations, occurrence
  ledger, returned reports, and threshold emissions. The test must reflect the revised
  causal-cohort boundary.
- Extend AT-I21 with unrelated due work capable of changing a deferred rule's input;
  prove continuation priority/boundary identity preserves the declared equivalence.
- Extend AT-I22 with a scheduled decay/recovery coincident with an independent shock and
  assert the declared exclusive/reducer policy, plus exact formula/rounding vectors.
- Extend AT-I24 so child contributions of equal magnitude remain distinct reducer inputs
  and full recomputation matches all permitted evaluation partitions.

### 9.5 Close obligation, occurrence, atomicity, and provenance gaps

- Extend AT-I8 with late candidate failures from occurrence `u64` exhaustion,
  obligation-cap exhaustion, obligation-record collision, and scheduler/store invariant
  failure; every retained store and cell must remain unchanged.
- Extend AT-I12/AT-I15 with three or more distinct obligation records under one
  `WorkKey`, all insertion permutations, cap boundaries, record-hash discrimination,
  and removal of the complete contested set on drain.
- Extend AT-I13 with a materialized multi-target effect whose second target fingerprint
  changes; execution must refuse atomically.
- Extend AT-I16 with two or more simultaneous allocations for the same ledger key and
  permuted candidate enumeration; payload-to-occurrence mapping and final digest must be
  identical.
- Add a provenance-cap test: cause sets sharing the same retained smallest rule IDs but
  differing beyond the cap must expose correct truncation/omitted metadata and must not
  create hidden behavior divergence.
- Extend AT-I26–I29 to any retained contested-obligation/cohort state introduced by the
  correction.

## 10. Classification and boundary ruling

**Exact primary defect:** Q3 equates `(target, resolved value)` with causal emission
identity and offers no operation-aware same-target reducer. It therefore deduplicates
distinct equal causes, rejects distinct unequal but compatible causes, and permits work
partition to change semantics.

**Severity:** BLOCKING for Phase-2 implementation.

**Foundational or local:** local to the Phase-2 architecture lineage, chiefly Q3 with
necessary Q4/§5/§6 test clarifications. It is an engine-semantic/transaction detail that
must be corrected before code, but it does not invalidate any closed Phase-1 type,
authority, scheduler, timeline, hashing, or activation invariant.

**Smallest safe correction:** retain one committed absolute value per target, add an
internal candidate-effect/update-intent distinction, automatically fold only additive
`add/subtract` deltas by stable causal emission identity, require a single explicit typed
reducer for ordering-sensitive/mixed operations, reject genuinely incompatible groups,
and preflight the whole bounded causal cohort before mutation.

**Rejected alternatives:**

- rejecting every unequal same-target result: causally wrong for independent additions;
- deduplicating every equal target/value pair: erases distinct causes;
- sorting rules/effects and applying sequentially: deterministic but makes arbitrary ID
  order causal and violates stable-snapshot semantics;
- universal implicit affine math: silently chooses disputed meanings for scale,
  rounding, clamp, recovery, and assignment;
- requiring every additive cause to be manually aggregated elsewhere: expressible but
  not a faithful or structurally safe default for the blueprint's accumulating
  S.P.A.R.K.-owned pressures;
- general scripting: unnecessary and prohibited.

## 11. Authorization result

Phase-2 architecture must be revised and independently rechecked before any Opus
production implementation authorization. Phase 1 stays closed. Phase 3 is not
authorized.

