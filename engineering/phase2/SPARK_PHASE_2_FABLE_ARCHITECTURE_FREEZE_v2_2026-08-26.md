# S.P.A.R.K. Phase 2 — Fable Architecture Freeze v2

**Date:** 2026-08-26
**Agent:** Claude Code (Fable), Phase-2 bounded architecture-correction gate
**Status:** ARCHITECTURE FREEZE v2. This document supersedes
`SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_2026-08-26.md` (retained unmodified as
history) in exactly the sections it revises, after adjudicating the independent Codex
adversarial review (`SPARK_PHASE_2_CODEX_ARCHITECTURE_ADVERSARIAL_REVIEW_2026-08-26.md`,
verdict `PHASE_2_ARCHITECTURE_REVISE`). Every v1 section not revised here remains frozen
as written. This gate authorizes **no** production Rust, **no** Phase-3 work, and reopens
**no** Phase-0/Phase-1 contract. The Phase-2 writer pass is bound to this document and to
`SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v2_2026-08-26.md`.

---

## 1. Lineage verification (gate precondition)

- Branch: `phase1-refoundation-v2`; HEAD at gate entry:
  `905880ed138b79f40b94d7fbaefa643406905030` ("Record Phase 2 adversarial architecture
  review"), clean tree, a descendant of the reviewed freeze HEAD
  `87ba1c22b4b1634e5205fb14bb6806a886df3f0b` and of the Phase-1 closure lineage. No
  history is rewritten by this gate; the v1 freeze and v1 matrix are preserved verbatim.
- Phase 1 remains **CLOSED** (`PHASE_1_CLOSED`). No Phase-1 type, authority door,
  scheduler, timeline, hashing, or activation invariant is reopened; every Phase-1
  canonical encoding and pinned digest value remains bit-identical under this revision.
- The Codex review was treated as independent adversarial evidence, not sovereign truth:
  every citation it makes was re-verified against the controlling record before
  adjudication — ADR-0003 §15 ("quota/yield changes latency, not semantics"),
  performance/security budget §3 (finite queue admission, atomic pre-commit rejection,
  quota may not change accepted-command semantics), R-038 ("ten-days-once versus
  one-day-ten-times" catch-up equivalence), R-044 (split-batch equivalence), blueprint
  §12.5 (every definition declares its update semantics), §19.3–§19.5 (declarative
  operation vocabulary; recovery semantics for accumulating pressures), and ADR-0006's
  provenance coverage block (`complete | truncated | unknown`, retained/omitted counts,
  deterministic pruning-policy identity). All citations were confirmed accurate, and the
  Phase-1 seams the correction relies on were confirmed in source: `WorkKey` orders by
  `due_time` first (`scheduler.rs:122–129`), the AT-B7 batch-contract seam note stands
  (`scheduler.rs:424–431`), `BoundedClaimSet` exists as the shared contested-evidence
  discipline (`evidence.rs`), and `MAX_SOURCE_REFS = 8` (`state.rs:61`).

## 2. Adjudication of the Codex findings

Legend: **ACCEPTED** = adopted as stated (concretized where Codex deliberately left a
choice open); **MODIFIED** = adopted with a substantive change, with reasoning;
**REJECTED** = not adopted, with reasoning. Where Codex offered mutually exclusive
alternatives, the branch not taken is named.

| # | Codex finding | Ruling |
|---|---|---|
| C-1 | Primary defect (§3–§4, §10): Q3 equates `(target, resolved value)` with causal emission identity; deduplicates distinct equal causes, rejects distinct compatible additive causes, and lets work partition change semantics | **ACCEPTED** — confirmed against the controlling record (§1 above). The v1 Q3 conflict rule is withdrawn and replaced by §3–§5 of this document. |
| C-2 | Operation-by-operation same-target ruling (§5) | **ACCEPTED with one MODIFICATION** — the equal-value coalescence row of Codex §4.2 ("distinct exclusive assignments of the same value → compatible") is narrowed to *result-semantics candidates of the same declared family* (§4.3). Two transform-family candidates (e.g. two `scale ×1.5`) do **not** coalesce even when their snapshot-resolved values are equal: transform multiplicity is meaningful (the intent-level composition of two ×1.5 causes is ×2.25, not ×1.5), so coalescing on numeric coincidence would re-import the primary defect one level up. This modification harmonizes Codex's own §4.2 table with its §5 scale ruling ("makes such coexistence exclusive"). |
| C-3 | Correction shape (§6): candidate/committed distinction, emission identity, target-local closed reducer, cohort preflight, digest commitment to candidate resolution | **ACCEPTED** — with the specification that transforms are *resolved during evaluation against the snapshot* but retain their family and operation fingerprint until same-target resolution (§4.2), preserving v1's "commit re-executes nothing" while satisfying "retain operation intent until same-target resolution". |
| C-4 | §7.1 causal cohort that budgets may not split | **ACCEPTED** — with the open choice resolved: the cohort is the *equal-due-time slice* of drained due work per barrier (§6.1). The R-038 catch-up equivalence ("ten days once" ≡ "one day ten times") independently forces per-due-time wave slicing and rules out both alternatives Codex listed (single-item cohorts split co-target causes; whole-drain cohorts break catch-up equivalence). Over-cap disposition, which Codex left unspecified, is specified in §6.3 to preclude livelock. |
| C-5 | §7.2 complete preflight of every mutation class before any canonical mutation | **ACCEPTED** — §7. The v1 Q6 "batch atomicity is vacuous" argument is withdrawn as incomplete: `Scheduler::schedule` is total, but queue admission caps, `ObligationStore` invariants, and occurrence exhaustion are not, exactly as Codex showed. |
| C-6 | §7.3 obligation collision storage; multi-target fingerprint binding | **ACCEPTED** — §8. A single-value map keyed by `WorkKey::identity_digest()` cannot represent the contested state v1 itself promised to retain. |
| C-7 | §7.4 deterministic multi-allocation for one occurrence-ledger key | **ACCEPTED** — taking the *sort-by-complete-emission-identity* branch (§9). The alternative branch (structurally at most one allocation per ledger key per cohort) is **rejected**: legitimate waves can create several delayed obligations under one `(profile, producer, scope, kind)` sequence, and the restriction would push authors to fabricate artificial work kinds to evade it. |
| C-8 | §7.5 bounded provenance must carry the ADR-0006 coverage contract | **ACCEPTED** — §10, with the explicit addition that truncation bounds *explanation only*: the additive fold always sums every candidate delta regardless of the provenance cap; no arithmetic is ever truncated. |
| C-9 | §7.6 delayed-work boundary must not permit zero-time same-barrier recursion | **ACCEPTED** — §11: strictly later logical due time plus the single-drain-per-barrier rule. |
| C-10 | §8 table rulings: one authoritative reducer per aggregate target, threshold compares committed-old vs fully-reduced-new, one decay/recovery reducer per target/cadence | **ACCEPTED** — folded into the family contract (§4.3) and §5.4. |
| C-11 | §9 acceptance-test changes | **ACCEPTED with one MODIFICATION** — all mandated tests enter the v2 matrix. The AT-I21 interference extension is modified: with interfering due work present, the test asserts determinism, continuation priority, and strictly-later boundary identity — **not** digest equality against a larger depth budget. Depth budgets are epoch-bound declared behavior (v1 Q4, upheld), not runtime pacing; cross-budget equivalence under interference would compare two different declared behaviors and is not a coherent claim. Cross-budget digest equality is asserted only in the isolated (no-interference) case, where it is a pure determinism check. |

No finding is rejected outright. Within findings, two offered branches were rejected with
reasoning: at-most-one-occurrence-allocation-per-cohort (C-7) and clamp-intersection
semantics (Codex §5 named intersection as possible; §4.3 rejects it in favor of the
minimal singleton rule, as Codex itself recommended).

## 3. Corrected effect model (replaces v1 §Q3's effect/conflict rules)

The frozen invariant is unchanged: **one committed resulting value per target cell per
wave**, at most, applied atomically. What changes is what a candidate is. A candidate
cause is no longer forced into an absolute resolved value before same-target analysis;
operation intent is retained until same-target resolution.

The internal model distinguishes exactly five things (resolving the operator's question —
all five are distinguished):

1. **Additive delta candidate effects** — signed contributions whose meaning is "change
   the target by Δ"; multiplicity-significant; commutatively composable.
2. **Exclusive resolved-result candidates** — complete results whose meaning is "the
   target's value is V"; multiplicity-insignificant by meaning.
3. **Closed typed exclusive transforms** — relative but non-additive operations (scale,
   clamp-as-effect, decay/recovery catch-up); multiplicity-significant but with **no**
   frozen composition; at most one per target per wave.
4. **Stable causal emission identity** — the identity of one causal emission,
   independent of its numeric payload (§3.2).
5. **Committed resolved effects** — the single absolute `CanonicalValue` per target that
   the commit phase applies; commit re-executes nothing.

### 3.1 Types (frozen shape; names indicative, not source-binding)

```text
CandidateEffect
  target                (profile_id, definition_id, scope_id)
  write_path            spark_effect | commit_derived        (the only two; unchanged)
  emission_identity     (§3.2)
  update_intent         AddDelta { delta: signed fixed/i64 domain }
                      | ExclusiveResult { value: CanonicalValue, family: ResultFamily }
                      | ExclusiveTransform { family: TransformFamily,
                                             operation_fingerprint,
                                             snapshot_resolved: CanonicalValue }
  at, behavior_epoch
  provenance            (bounded, with coverage metadata — §10)

CommittedEffect
  target, write_path
  resolved CanonicalValue
  contributing emission identities (bounded) + coverage metadata
  at, behavior_epoch
```

This is an internal representation of the already-frozen
`PropagationRule → EffectBatch` primitive. No new causal primitive, no scripting, no new
public surface, no new write path.

### 3.2 Stable causal emission identity (frozen)

```text
emission_identity = H("emission" ‖ emission context
                                 ‖ rule fingerprint (within ruleset_content_hash lineage)
                                 ‖ effect-operation qualified sub-ID
                                 ‖ producer scope_id
                                 ‖ target (definition_id, scope_id)
                                 ‖ behavior_artifact_hash)

emission context  = barrier identity ‖ wave_index
                    ‖ (triggering WorkKey identity digest | "command" tag)
```

Every effect-emitting operation in a rule body carries a declared stable dotted sub-ID,
extending the v1 §6 probability-gate sub-ID discipline to all emitting operations;
uniqueness within the rule is validated at rule-set activation. Identity never includes
the numeric payload: two rules producing the same delta are not duplicates because their
numbers coincide, and one emission is not two causes because a value differs.

**Idempotency (frozen).** Within one wave, candidates canonicalize by emission identity
first: exact duplicates (same identity, same payload) fold to one candidate — an exact
replay/redelivery of the same causal emission applies once, never twice. One emission
identity with **distinct payloads** is a contested emission — a malformed artifact or
evaluator defect — and rejects the wave atomically. Across barriers, re-emission of the
same identity is structurally precluded by the Phase-1 scheduler (occurrence indexes are
consumed; a drained `WorkKey` slot is not redelivered); the within-wave rule is the
defense-in-depth layer and the definition mechanism replay verifies against.

## 4. The typed same-target composition contract (frozen; the smallest one)

### 4.1 Update families (closed enumeration)

| Family | Members (from the §19.3 vocabulary; no new primitive) | Multiplicity | Composition |
|---|---|---|---|
| **ADDITIVE** | `add`, `subtract` (normalized to a signed delta; checked negation at the `i64::MIN` edge), weighted-sum or piecewise-curve output *declared as* `emit_delta` | significant | commutative checked fold (§4.2) |
| **RESULT** | assignment-like complete results: weighted-sum / piecewise-curve / aggregation output declared as `emit_result` | insignificant by meaning | equal values coalesce within the same family; unequal values conflict |
| **TRANSFORM** | `scale`, `clamp` emitted as an effect, decay/recovery closed-form catch-up | significant, composition **not frozen** | at most one per target per wave; two or more reject — even when their resolved values are equal (C-2 modification) |

Every rule operation that emits an effect declares, and activation validates, which
family (and for ADDITIVE/RESULT-capable operations, which emission mode) it uses. The
family is part of the operation's typed form, never inferred from values.

### 4.2 Target-local resolution (frozen wave step)

For each wave, after emission-identity canonicalization (§3.2), candidates are grouped by
target in canonical `(definition_id, scope_id, write-path tag)` order and each group is
reduced by exactly this closed rule:

- **All ADDITIVE:** sum every distinct emission's delta in checked `i128`; apply the
  total once to the pre-wave snapshot value with one checked addition; then final
  type/bounds validation. Any checked failure is a typed error rejecting the wave
  atomically (§7) — never saturation. The fold is commutative and associative, so every
  rule/candidate/enumeration permutation yields the identical result from one stable
  snapshot.
- **All RESULT, same family:** all values equal → one committed effect with merged
  provenance (distinct causes, honestly represented — not mislabeled as delivery
  duplicates); any inequality → genuine conflict, atomic wave rejection with
  order-independent evidence.
- **Exactly one TRANSFORM:** commit its snapshot-resolved value (resolved during
  evaluation against the pre-wave snapshot; the resolver validates singleton-ness and
  family; commit re-executes nothing).
- **Any other mixture** — cross-family groups (assignment+delta, add+scale, decay+shock,
  RESULT values from different families even when equal), or ≥2 TRANSFORM candidates —
  **rejects**: at rule-set activation when co-targeting is statically provable, else as
  a typed runtime atomic wave rejection. No rule ID, declaration order, hash order,
  insertion order, or sort order ever resolves a non-commutative mixture.

Deliberately **not** given universal semantics (restating Codex §5, now binding):
`add+scale`, `scale+scale`, `clamp` mixtures and clamp-intersection, `decay+shock`,
`assignment+delta`, multiple weighted-sum results, multiple aggregate assignments from
distinct reducers. Authors express such combinations in exactly three sanctioned ways:
inside **one** rule's bounded declared operation list (the rule body is itself the
declared reducer — stage order is explicit, typed, and activation-validated), across
**separate scheduled boundaries** (strictly later due times, §11), or as **separate
component cells** combined by the single authoritative aggregation reducer (§5.4).

### 4.3 Family-specific rulings (binding on the writer)

- **Aggregation:** activation admits at most **one** aggregation rule per aggregate
  target definition per ruleset (the authoritative reducer); children are reducer inputs
  in stable BTree order, never competing absolute writes (v1 §7 upheld). A dynamically
  co-emitted second aggregate RESULT (e.g. via scope mapping the static analysis could
  not decide) falls under §4.2: equal → coalesce, unequal → conflict.
- **Decay/recovery:** activation admits at most one decay/recovery rule per target
  definition (TRANSFORM family); the v1 §Q7 closed-form catch-up formula, floor
  rounding, and chunk-invariance are upheld and remain the frozen formula. A
  simultaneous shock (`AddDelta`) coinciding with a decay evaluation on one target in
  one wave is a cross-family mixture: rejected, deterministically and visibly, unless
  the profile combines them in one rule body or separates them by a scheduled boundary.
- **Weighted sums / piecewise curves / probability-gated emissions:** must declare
  `emit_delta` or `emit_result`; separate complete weighted-sum results never
  auto-compose (equal/conflict only, within family).
- **Clamp:** stays where v1 put it — inside a single rule body or as the definition's
  final declared bounds validation. Multiple clamp *effects* on one target reject;
  intersection semantics are not frozen.

### 4.4 The mandated examples, resolved

Pre-wave `stress = 20`, all emissions distinct unless stated:

| Scenario | Result |
|---|---|
| `AddDelta(+10)` and `AddDelta(+15)` from distinct emissions | one committed write: `20 + (10 + 15) = 45`, in every permutation |
| `AddDelta(+10)` and `AddDelta(+10)` from **distinct** emissions | `20 + 20 = 40`; both causes in provenance |
| Exact replay of one `AddDelta(+10)` emission (same identity, same payload) | folds to one candidate: `30`; never applies twice |
| One emission identity carrying `+10` and `+15` payloads | contested emission → atomic wave rejection |
| `ExclusiveResult(30)` and `ExclusiveResult(35)`, same family | conflict → atomic wave rejection, order-independent evidence |
| `ExclusiveResult(30)` and `ExclusiveResult(30)`, same family, distinct emissions | one committed `30`, merged provenance |
| `ExclusiveResult(30)` + `AddDelta(+10)` | cross-family mixture → reject |
| two `scale ×1.5` transforms, equal resolved values | ≥2 TRANSFORM → reject (no coincidence coalescence) |

## 5. Wave pipeline, batch identity, and thresholds (revises v1 §Q3/§7 mechanics)

### 5.1 Frozen per-wave pipeline

1. **Evaluate** every rule of the cohort (§6) against the pre-wave stable snapshot only
   (v1 §Q2 borrow-split upheld), producing the candidate multiset plus
   obligation/cooldown/occurrence/enqueue claims.
2. **Canonicalize** candidates by emission identity (§3.2): idempotent fold or contested
   rejection.
3. **Reduce** per target by the §4.2 contract → at most one `CommittedEffect` per
   target.
4. **Threshold evaluation** — only after complete target reduction: each threshold
   operation compares the *previously committed* cell value against the target's *fully
   reduced* candidate result (never a partial fold). Crossing emissions are never
   injected into the wave being reduced: they become wave `N+1` candidates within the
   same barrier (subject to `max_wave_depth`) or scheduled work (§11). At most one
   emission per declared crossing identity per wave.
5. **Preflight** the complete candidate state transition (§7).
6. **Apply** atomically in canonical target order; allocate occurrences (§9); write
   cooldowns; insert obligation records and scheduler claims; return the wave report.

### 5.2 Batch identity (revised digest, pre-implementation so no pinned value moves)

```text
effect_batch_digest = H("effect_batch_v2" ‖ profile ‖ behavior_epoch ‖ barrier identity
                        ‖ pre-wave engine digest ‖ wave_index
                        ‖ canonical candidate-set digest
                        ‖ committed effect count ‖ each committed effect block)

canonical candidate-set digest = H over the canonicalized candidate multiset in
                                 ascending emission-identity order (identity ‖ intent tag
                                 ‖ payload/operation fingerprint)
```

The digest thereby distinguishes distinct causal emission sets even when they reduce to
the same committed values, as Codex §6 requires: outcome replay may apply the committed
resolved batch; mechanism replay must reproduce candidate identities and the reducer
result under the exact artifact. Wave chaining to barrier identity is unchanged from v1.

### 5.3 What atomic rejection is for (narrowed, per C-1)

Atomic wave rejection remains the response to: contested emission identities, unequal
same-family RESULT values, cross-family mixtures and multiple transforms not combined by
a declared reducer, authority/type/bounds/scope-invalid effects, arithmetic failures,
preflight failures (§7), and semantic-cap overflow (§6.3). It is **no longer** the
response to independent additive contributions — those compose. Rejection is a defect or
declared-bound signal, never the normal composition mechanism.

### 5.4 v1 §7 (aggregation/threshold) amendments

Aggregation remains a pure stable-order snapshot recomputation with no hidden
accumulator; the single-authoritative-reducer rule (§4.3) and the
threshold-after-complete-reduction rule (§5.1 step 4) are the only amendments.

## 6. Causal cohorts and budgets (revises v1 §Q4's deferral rules)

### 6.1 The cohort (frozen)

The atomic evaluation cohort is the **equal-due-time slice**: all due work items sharing
one logical `due_time` drained at one canonical boundary, together with the candidate
effects their evaluation emits at wave 0. Distinct due times within one drain are
successive cohorts in ascending time order, each evaluating against the previous
cohort's committed state. This is forced, not chosen: R-038's catch-up equivalence
("ten-days-once ≡ one-day-ten-times") requires per-due-time slicing, and same-time
co-target causes must reduce together (§4.2) — which rules out both single-item cohorts
and whole-drain cohorts.

- Same-cohort candidates are always reduced together; a budget may never split a cohort.
- Command-barrier rule evaluation forms its own cohort at that barrier (its emission
  context tag distinguishes it, §3.2).

### 6.2 Pacing versus semantic budgets (frozen distinction)

- **`max_due_per_cycle` is pacing.** It defers only **whole cohorts**, in ascending
  due-time order; a partially admitted cohort is never evaluated. Deferred cohorts
  remain scheduled untouched, and because `WorkKey` orders by `due_time` first, they are
  structurally drained before any newer work — continuation priority needs no extra
  state. Pacing is semantics-neutral over an identical canonical input history
  (ADR-0003 §15; budget §3.2): deferred-then-completed execution is digest-identical to
  unbudgeted execution, including thresholds, obligations, ledgers, and reports. New
  canonical input arriving between drains is new history, not a quota artifact; the
  equivalence claim and its tests (AT-I20 v2) are over identical input histories.
- **Cohort/wave caps are semantic.** `max_cohort_candidates` (bounding one cohort's
  candidate volume), `max_effects_per_wave`, `max_wave_depth`, `max_enqueue_per_wave`,
  and every queue admission cap are epoch-bound declared values (v1 Q4 upheld).
  Exceeding one is deterministic declared behavior, not pacing, so it does not violate
  §15 — but it must still be atomic and visible (§6.3).

### 6.3 Over-cap disposition (specifies what C-4 left open)

A cohort whose candidate transition exceeds a semantic cap **rejects atomically before
any canonical mutation** (§7): no cell, ledger, obligation, or queue moves. To preclude
livelock, the cohort's due `WorkKey`s are then consumed with a typed overload rejection
report in the same shape as a conflicted drain: the keys become free, the report carries
order-independent evidence, and producers may reschedule under the next occurrence —
the deterministic Phase-1 recovery pattern, reused. Because the caps are epoch-bound,
the outcome is identical on every replay of the same history under the same artifact.

## 7. Complete transition preflight (replaces v1 Q6's vacuity argument)

The v1 rule "validate all, then apply all; no rollback" is extended from cell writes to
the **entire candidate state transition**. Before the first canonical mutation of a
wave, preflight validates, against pre-wave state plus the candidate transition only:

- every committed effect (authority, type, bounds, scope — unchanged);
- every occurrence allocation (the full per-key ranges of §9; `checked_next` exhaustion
  detected here, not mid-apply);
- every cooldown write and deterministic expiry removal;
- every obligation record (store admission cap, record well-formedness, the §8
  claim-set invariants);
- every scheduler enqueue (finite declared queue admission budgets, budget §3.1/§3.5 —
  `Scheduler::schedule` being total does not make admission caps total);
- the scheduler↔ObligationStore bidirectional invariant on the post-state.

Any typed failure rejects the wave atomically with nothing mutated. An obligation
`WorkKey` collision is **not** a preflight failure: it is a legitimate deterministic
outcome that poisons the key order-independently (Phase-1 semantics unchanged) and is
part of the committed transition. **Binding rule (extends v1):** no validation step may
depend on state mutated earlier in the same commit; a validation that cannot complete
before the first mutation is an engine-semantic change requiring escalation.

## 8. ObligationStore: contested claims and multi-target binding (revises v1 §5)

- **Storage (frozen).** The store maps `WorkKey::identity_digest()` to a **bounded,
  order-independent claim set of content-addressed obligation records** — the
  `BoundedClaimSet<EXPOSED, TRACKED>` discipline reused, not a new mechanism. A second
  distinct record under one work identity is representable, retained, and removed as a
  complete contested set at the poisoned drain, exactly as v1 promised but could not
  represent in a single-value map. The store digest commits to all behavior-relevant
  tracked claims (R1) and is insertion-order independent (R2).
- **Multi-target binding (frozen).** `creator_definition_fingerprint` (singular) is
  replaced for `MaterializedEffect` by a **canonical ordered set of
  `(definition_id, definition fingerprint)` pairs covering every target in the frozen
  effect list** (encoded ascending; the record hash covers it). Execution refuses
  explicitly, atomically, if **any** target's current fingerprint mismatches — a
  second-target drift is caught exactly like a first-target drift. `RuleReEvaluation`
  is already bound by exact rule fingerprint + `ruleset_content_hash`; unchanged.
- Payload identity (`WorkPayload.canonical_payload_hash` = obligation record hash),
  explicit refusal semantics, and mode immutability are unchanged from v1 §5.

## 9. Occurrence allocation for one ledger key (specifies C-7)

When one cohort creates multiple obligations under one occurrence-ledger key
`(profile, producer definition, scope, work kind)`: allocation claims are sorted by
their **complete emission identity** in ascending canonical order, then a checked
consecutive range is allocated from the ledger's `next` in that order. Payload↔occurrence
mapping is therefore a deterministic function of the cohort's claim multiset —
independent of rule order, evaluator insertion order, and enumeration order. Exhaustion
anywhere in the range is a preflight failure (§7). Allocation remains a commit-phase
mutation of the digest-committed `OccurrenceLedger` (v1 §6 otherwise unchanged).

## 10. Bounded provenance with honest coverage (revises v1 §Q3 provenance)

Committed effects, wave reports, and obligation records carry the ADR-0006 coverage
block wherever provenance is bounded:

```text
coverage_status          complete | truncated | unknown
retained_source_count
omitted_source_count     (exact within a wave — the resolver counted every candidate)
pruning_policy identity  ("provenance_prune.v1": keep the MAX_SOURCE_REFS smallest
                          emission identities in ascending canonical order)
```

`unknown` is reserved for provenance inherited from prior committed state whose full
history was never retained; it is never used to paper over a within-wave truncation.
**Truncation bounds explanation only:** the §4.2 fold sums every candidate delta and the
conflict/coalescence analysis sees every candidate regardless of the cap; two cause sets
sharing the same retained smallest identities but differing beyond the cap produce the
same bounded presentation with differing `omitted_source_count` — and always the same
arithmetic. `StateCell.source_refs` and every Phase-1 canonical encoding are untouched;
the coverage block lives in Phase-2 records and returned presentation values only.

## 11. Delayed-work boundary (closes the zero-time loophole; revises v1 §Q4 depth rule)

- Every obligation created or converted by a wave — including depth-overflow
  conversion — carries `due_time ≥ current logical time + 1`: a **strictly later**
  logical due time, never "now".
- A canonical boundary drains due work **at most once**; nothing scheduled during a
  barrier's execution is drained within that same barrier, by construction.

Together these make zero-time same-barrier recursion structurally impossible: delayed
feedback always crosses a genuinely later scheduled boundary (§17, R-037). Depth-overflow
conversion remains visible, deterministic, and never dropped (v1 upheld); its
equivalence claims are as adjudicated in C-11.

## 12. Retained-store commitment (v1 §8 table, amended rows only)

| Store | Amendment |
|---|---|
| `ObligationStore` | now claim-set-valued per work identity (§8); digest commits to exposed **and** tracked contested claims; discrimination/equivalence tests must cover contested permutations and cap boundaries |
| wave evaluation buffers | now include the candidate multiset, reduction groups, and preflight transition — all transient; must not outlive the wave (committed or rejected); falsified by equal-digest/same-next-input tests |
| all other rows | unchanged from v1 §8, including the engine digest composition and the mandatory per-store AT-G-style falsification from first commit |

No new retained store is introduced by this revision; coverage metadata is carried in
records and presentation values already enumerated.

## 13. Unchanged v1 resolutions (explicit)

Q1 (closed typed operation enum, rule-set artifact, numeric model and floor rounding),
Q2 (borrow-split snapshot), Q5 (epoch record at its own barrier;
`behavior_artifact_hash` = epoch-record hash), Q6's *conclusion* (no batch API in
Phase 2; AT-B7 contract preserved — only its vacuity *argument* is withdrawn, §7),
Q7 (decay formula, chunk invariance, expiry deferred), §3 boundary restatement, §6
random-address discipline, §9 module placement, §10 deferred debts — all stand as
written in v1. Sub-ID declaration extends from probability gates to all emitting
operations (§3.2); this is additive validation at the rule-set door, not a Phase-1
change.

## 14. Freeze summary (delta over v1)

| Item | v2 resolution |
|---|---|
| Candidate identity | stable causal emission identity (§3.2), never `(target, value)` |
| Candidate intent | AddDelta / ExclusiveResult(family) / ExclusiveTransform(family, op fingerprint, snapshot-resolved) |
| Same-target contract | ADDITIVE commutative checked fold; same-family equal RESULT coalescence; singleton TRANSFORM; every mixture rejects; no implicit ordering semantics ever (§4) |
| Idempotency | exact identity+payload duplicates apply once; contested identity rejects (§3.2) |
| Thresholds | committed-old vs fully-reduced-new, post-reduction only; emissions go to wave N+1 or scheduled work (§5.1) |
| Batch digest | commits to canonical candidate set and committed effects (§5.2) |
| Cohort | equal-due-time slice; pacing defers whole cohorts only; semantic caps epoch-bound with atomic over-cap disposition (§6) |
| Preflight | complete transition incl. ledgers/obligations/queues before first mutation (§7) |
| ObligationStore | bounded claim set per work identity; multi-target fingerprint set binding (§8) |
| Occurrence multi-allocation | ascending emission-identity order, checked consecutive range (§9) |
| Provenance | ADR-0006 coverage block; truncation bounds explanation, never arithmetic (§10) |
| Delayed boundary | strictly later due time + single drain per barrier (§11) |

## 15. Verdict

**PHASE_2_ARCHITECTURE_FROZEN_V2.**

The Codex primary defect and all six supporting findings are adjudicated and closed
(9 ACCEPTED, 2 MODIFIED with reasoning, 0 rejected outright; rejected sub-branches
named). The corrected model composes distinct additive causes deterministically from one
stable snapshot (`20 → 45`, `20 → 40`), keeps exact emission replay idempotent
(`20 → 30`), freezes the smallest typed composition contract without inventing universal
ordering semantics, and preserves every Phase-0/Phase-1 boundary. No production Rust was
written.

PHASE_2_IMPLEMENTATION_AUTHORIZATION: NO (separate operator decision)
PHASE_3_AUTHORIZATION: NO
