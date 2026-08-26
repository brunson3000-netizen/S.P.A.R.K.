# S.P.A.R.K. Phase 2 — Fable Architecture Freeze

**Date:** 2026-08-26
**Agent:** Claude Code (Fable, HIGH effort), Phase-2 architecture-only gate
**Status:** ARCHITECTURE FREEZE. This document freezes the Phase-2 rule/effect-runtime
architecture. It authorizes **no** production Rust, **no** Phase-3 work, and it reopens
**no** frozen Phase-0/Phase-1 contract. The Phase-2 writer pass is bound to this document
and to the companion acceptance matrix
(`SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_2026-08-26.md`) exactly as the Phase-1 writer was
bound to the Fable admission architecture and AT-H matrix.
**Mode honored:** no implementation file was changed by this gate; the only repository
changes are this document, the acceptance matrix, and the phase-status refresh.

---

## 1. Lineage verification (gate precondition)

- Branch: `phase1-refoundation-v2`; HEAD at gate entry:
  `f6665aadee89af2ec365441ab0a6983d00b493b6` ("Record independent Phase-1 final closure
  review"), a clean tree, exactly one commit ahead of the independently reviewed closure
  HEAD `20a1c66244646dddb926d08fb9beb78d5cd9be13` — verified by
  `git merge-base --is-ancestor`. No history was rewritten by this gate.
- Phase-1 closure: `SPARK_PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_2026-08-26.md` §1 returns
  **`PHASE_1_CLOSED`** with no MAJOR or BLOCKER finding remaining. Phase-0 closure stands
  (`PHASE_0_STATUS.md`, gate report, and the three correction cycles).
- Gate re-verification on this HEAD: `cargo fmt --check` pass; `cargo clippy --workspace
  --all-targets -- -D warnings` pass; `cargo test --workspace` **232 passed / 0 failed /
  0 ignored** — identical to the reviewed closure count.
- Operator authorization: the operator has directed this Phase-2 **architecture-only**
  pass on the closed Phase-1 lineage. Phase-2 *implementation* authorization remains a
  separate operator decision that this document does not grant.

## 2. Readiness-blueprint reconciliation against the actual tree

Every seam claimed by `PHASE_2_FABLE_READINESS_BLUEPRINT_2026-08-26.md` §2 was re-verified
by direct source inspection at this HEAD:

| Seam claimed | Verified against source | Result |
|---|---|---|
| Evaluator landing zone: three `pub(crate)` write paths, authority-hard-coded, schema-validating, no public caller | `state.rs:412/436/461` (`observe_host_owned`, `apply_spark_effect`, `commit_derived`), each delegating to `validate_write` (profile → declaration → authority → type → bounds → scope); `compile_fail` doc-probes pin non-constructibility | CONFIRMED |
| Activated schema as ground truth | `activation.rs`: `ActivatedProfile`/`ActivatedDefinition` private-field, door-minted, fingerprinted; `StateStore::from_activation` is the only store constructor | CONFIRMED |
| Deterministic work feed | `scheduler.rs`: complete semantic `WorkKey` (due_time, profile, producer, scope, occurrence, kind), order-independent conflict poisoning over `BoundedClaimSet`, digest commits to full tracked evidence, `DrainOutcome` separates due from conflicted | CONFIRMED |
| Canonical command ingress, claim-set-determined admission | `timeline.rs` post-closure: staged identity registries are derived indexes over `Staged` slots (B-01 repair), semantic/admission split, fences, epoch resets | CONFIRMED |
| Randomness fully qualified | `random.rs`: `RandomAddress` = seed + profile + epoch + `behavior_artifact_hash` + rule/trigger ID + scope + occurrence; stateless service, pure function | CONFIRMED |
| Panic-free template | strict clippy gate (deny `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `arithmetic_side_effects`) passes on `spark-core`/`spark-engine`; validated-domain constructors throughout | CONFIRMED |
| Batch-scheduling contract (AT-B7 seam) | `scheduler.rs:424–431` records the contract; no batch API exists | CONFIRMED — resolved by Q6 below |
| Bounded contested-state discipline | one shared `pub(crate)` `BoundedClaimSet<EXPOSED, TRACKED>` (`evidence.rs`) behind both consumers | CONFIRMED |
| `StateCell` decay/expiry fields absent by design | `state.rs:103–114`: value, `baseline`, created/updated, salience, bounded `source_refs`, `behavior_epoch` — no decay metadata | CONFIRMED — resolved by Q7 below |
| Config/manifest hashes exposed and paired; barrier machinery absent | `ProfileManifest::manifest_content_hash` (multiset function post-m-02), `ConfigRevision::config_revision_hash`; no epoch-activation barrier exists | CONFIRMED — resolved by Q5 below |

No discrepancy between the readiness blueprint and the tree was found. The blueprint's
file names for the landing zone (`rules.rs`, `effects.rs`, `evaluator.rs`) are the
*planned* Phase-2 modules inside `spark-engine`; they correctly do not exist yet.

## 3. Boundaries this freeze does not reopen (restated, binding)

1. **Authority (ADR-0002):** the three authority modes and implied write classes are
   untouched. Effect batches write `spark_owned` (via `apply_spark_effect`) and `derived`
   (via `commit_derived`) cells only; host-owned truth enters solely through ingress. No
   new write path, public or private, is added beside the existing three.
2. **Determinism (ADR-0003, blueprint §28):** wall clock, ambient RNG, unordered
   iteration, and floats remain prohibited in canonical code; stable total ordering and
   semantic random addresses remain the only ordering/randomness sources.
3. **Primitive set (blueprint §11.3):** Phase 2 realizes `EffectBatch` and
   `PropagationRule` from the frozen primitive table and adds **no new causal
   primitive**. Rule operations are profile *data* over the existing grammar.
4. **Portability (ADR-0001):** the declared Windows/Linux/Android envelope is unchanged;
   the executable Windows/Android debt carries forward exactly as recorded — Phase 2
   claims no new platform result and keeps the five static `cargo check` targets green.
5. **Chronicle boundary (blueprint §26.5, ADR-0006):** no event ledger. Wave/deferral
   reports are **returned presentation values**, mirroring `DrainOutcome`; canonical
   state retains only the §26.1 continuity list (which already names scheduler/delayed
   obligations and cooldowns/occurrence counters).
6. **Scripting (ADR-0004 security rule, §19.3):** the operation set is a closed typed
   enumeration; no general scripting language, eval, or dynamic module enters the tree.
7. **Runtime LLM (ADR-0001 invariant 1):** none.
8. **Phase-3 boundary (§32):** no profile parsing/IO, no protocol/service/transport, no
   persistence backend, no host facade, no inspection surface. Phase-2 tests keep
   building manifests, configs, and rule sets in code.

## 4. Resolution of the seven open architecture questions

### Q1 — Rule representation: **closed typed operation enum, activated as a content-addressed rule-set artifact**

**Decision.** A rule is data: `RuleSpec { rule_id, inputs, condition ops, effect ops,
scope mapping, delay/cooldown declarations }` whose operation body is a **closed,
non-recursive typed enum** with exactly the §19.3 vocabulary — threshold/eligibility,
add/subtract/scale/clamp, weighted sum, piecewise curve, probability gate, scope mapping,
delay/cooldown, decay/recovery, aggregation/threshold emission. Composition is a bounded
declared list of operations over named inputs; operations reference cell definitions and
config keys, never sub-expressions of arbitrary depth. Table-driven interpretation of
uninterpreted tags is **rejected**: it would reintroduce an indirection the validator
cannot exhaustively check without duplicating the enum, and it would weaken "effects are
data, not code" from a type-level property to a convention.

**Rule-set artifact.** Rules ship as a distinct content-addressed artifact
(`RuleSetSpec` → the same `ActivationRegistry` door → `ActivatedRuleSet` with a
door-computed `ruleset_content_hash` and per-rule fingerprints), referencing the activated
definition manifest by content hash. Validation at the door: every referenced definition
exists; every effect target's authority admits the evaluator write path (`spark_owned` or
`derived` only — a rule targeting `host_owned` is rejected at activation); value-type and
bounds compatibility; scope-mapping validity; the Q4 static cycle/fan-out analysis.
**Consequence deliberately accepted:** `ProfileManifest` and `ConfigRevision` canonical
encodings and hashes are untouched — no Phase-1 digest value moves, which extends the
closure pass's "no digest change" discipline into Phase 2.

**Numeric model (frozen).** All rule arithmetic is over `i64`/`FixedPoint`
(`FIXED_SCALE = 10^6`) with intermediates widened to `i128`. One rounding rule for every
operation: **floor division (toward negative infinity)**; no operation may choose its own
rounding. Weighted sums: Σ(wᵢ·xᵢ) in `i128`, one floor-scale at the end. Piecewise
curves: bounded breakpoint lists with strictly increasing x-coordinates (validated at
activation); linear interpolation computed in `i128` with the same floor rule; inputs
outside the declared domain clamp to the end segments. Probability gates: fire iff
`derive_fixed_fraction(address) < rate`, strict, with `rate` in `[0, FIXED_SCALE]`.
Overflow of any checked operation is a typed evaluation error that rejects the wave
atomically (§7) — never a silent saturation.

### Q2 — Snapshot mechanics: **borrow-split two-phase wave; no copies, no generation stamps**

**Decision.** The stable snapshot **is the store itself between commit waves**, made
stable by Rust's borrow rules rather than by copied or stamped state: a wave has an
*evaluation phase* that holds only `&` (shared) access to `StateStore`, the scheduler,
and the ledgers, and produces a candidate `EffectBatch`; then a *commit phase* that holds
`&mut` access and applies the validated batch. Nothing can write canonical state while
any evaluation read exists, by construction. Snapshot identity is the engine digest (§8)
at wave start and is bound into the batch identity (Q3).

Copy-on-write is **rejected** (a retained copy is hidden state that would itself need
digest commitment and equal-digest falsification); generation-stamped reads are
**rejected** (a generation counter is retained behavior-relevant state solving a problem
the borrow checker solves structurally). This is the same shape as the Phase-1
resolutions: don't audit the invariant, make the state incapable of violating it.

### Q3 — EffectBatch identity, total ordering, conflict, atomicity (frozen)

**Effect.** A typed declarative record: target `(profile_id, definition_id, scope_id)`,
write path (`spark_effect` | `commit_derived` — the only two), the **resolved**
`CanonicalValue` (evaluation resolves operations; commit re-executes nothing), `at`
logical time, `behavior_epoch`, and bounded provenance = the ascending smallest-first set
of producing rule IDs capped at `MAX_SOURCE_REFS`. Effect hash = `CanonicalEncoder`
digest over exactly those fields.

**Intra-batch conflict (frozen rule).** At most one write per target cell per batch.
Same target + same resolved value from multiple rules deduplicates into one effect with
merged provenance. Same target + **distinct** values is a *conflicting batch*: the whole
wave is **rejected atomically** — no cell written, no obligation enqueued, no ledger
moved — and reported with order-independent `BoundedClaimSet`-style evidence per
contested target. No rule order, declaration order, or iteration order ever picks a
winner; a conflicting batch means the rule set is ill-formed, and committing around it
would launder a profile defect into silent partial behavior. (The scheduler's per-key
poisoning is *not* the model here: scheduler claims arrive over time from outside; a
batch is the output of one deterministic evaluation of one snapshot, so the fence
precedent — "failure rejects atomically and promotes nothing", ADR-0003 §8 — governs.)

**Total ordering.** After dedup/conflict validation the batch has at most one effect per
target, so effects are sorted by the total key `(definition_id, scope_id, write-path
tag)` — unique within a batch — and committed in exactly that order. Rule evaluation
order therefore cannot influence committed state, which is what makes rule-order and
insertion-permutation invariance testable as digest equality.

**Atomicity without rollback.** `validate_write` is a function of the activated schema
and the effect alone — it never reads mutable cell contents (verified at
`state.rs:348–406`). The commit phase therefore validates the entire batch first and
only then applies writes, none of which can fail. **Binding rule:** no future validation
step may depend on current cell contents unless validate-all-then-apply-all remains
possible; a validation rule that cannot be completed before the first write is an
engine-semantic change requiring escalation.

**Batch identity and chaining.** `effect_batch_digest = H("effect_batch" ‖ profile ‖
behavior_epoch ‖ barrier identity ‖ pre-wave engine digest ‖ wave_index ‖ effect count ‖
each effect block)`, where barrier identity is either a finalized command's
`(timeline_epoch, input_ordinal, semantic hash, covering fence hash)` (ADR-0003 §14) or
a heartbeat drain's `("heartbeat" ‖ logical time ‖ digest of drained WorkKeys)`. Waves
within one barrier are numbered `wave_index = 0..max_wave_depth` and each binds the
previous wave's post-commit engine digest — a hash chain in the same shape as the fence
chain, giving mechanism replay an exact per-wave commitment without persisting any wave
log (the digests are recomputable; only the current engine digest is retained state).

### Q4 — Cycle/fan-out validation: **both**, with frozen division of labor

**Activation-time static analysis (structural, rejecting).** The rule-set door builds
the dependency graph (rule reads → rule writes, plus scope-mapping edges) and rejects at
activation: any cycle that does not contain at least one **declared delay edge** (a
delay/cooldown operation scheduling future work); any rule whose declared static fan-out
(distinct target definitions × declared scope mapping breadth) exceeds the declared
per-rule bound; any graph whose maximum zero-delay chain depth exceeds the declared
`max_wave_depth`. "No zero-delay recursive cycles" (§17) is thereby a property of every
activated artifact, not a runtime hope.

**Evaluation-time budgets (dynamic, deferring or rejecting).** Frozen behavior classes:

- **Work-volume overflow → visible deterministic deferral.** Per-cycle due-work budget:
  the evaluator processes the stable-`WorkKey`-order *prefix* of `DrainOutcome::due` up
  to the budget; the remainder is **left scheduled, untouched** (already canonical
  state), and the deferral is reported in the returned wave report. Deferred work is
  simply due at the next drain. No reordering, no dropping, no hidden queue.
- **Propagation-depth overflow → conversion to scheduled work.** If wave
  `max_wave_depth` still enqueues same-barrier propagation, that propagation is not run
  in this barrier and not dropped: it converts deterministically into scheduled work at
  the next due boundary (which is a scheduled boundary, satisfying §17), with a visible
  deferral report. Runtime depth overflow past a statically validated graph is also
  reported as a diagnostic, since static analysis should have precluded it.
- **Structural violation at runtime (defense in depth) → atomic wave rejection.** An
  effect targeting an undeclared definition, wrong authority, out-of-bounds value, or
  disallowed scope rejects the wave atomically (Q3) — these are ill-formed-artifact
  conditions, not load conditions.

All budgets (`max_due_per_cycle`, `max_effects_per_wave`, `max_wave_depth`,
`max_enqueue_per_wave`) are declared deployment/profile values validated at activation,
bound into the activation artifacts, and therefore epoch-committed; changing one is a
behavior-epoch change, never an ambient runtime knob.

### Q5 — Behavior-epoch activation: **an explicit epoch record minted only at a canonical barrier**

**Decision.** A behavior epoch is created only by an epoch-activation command — a
reserved `CommandKind` — finalized and executed at its own timeline barrier (ADR-0004:
activation at a declared canonical barrier or not at all). The epoch record binds:

```text
profile_id
behavior_epoch            (predecessor + 1, starting at 1)
manifest_content_hash
config_revision_hash
ruleset_content_hash
activation_hash           (definition-set activation)
previous_epoch_record_hash
activating barrier identity (timeline_epoch, input_ordinal, fence hash)
```

`behavior_artifact_hash` — the digest `RandomAddress` and every delayed obligation carry
— is **frozen as the canonical hash of that epoch record**. Two epochs sharing a numeric
value under different artifacts can never collide (closing the M-04 concern at the epoch
level), and the manifest/config/ruleset triple is bound as one identity instead of a
convention-paired tuple. Epoch records live in a canonical, digest-committed,
append-only-per-profile **EpochRegistry** (§8). Mid-wave activation is impossible by
construction: epoch commands are timeline commands and execute at their own barrier,
never inside another command's wave. The *service-side* reload/parse flow remains
Phase 3; Phase 2 implements the engine-side record, registry, and barrier binding only.

### Q6 — Scheduler batch API: **not needed; none will be built in Phase 2**

`Scheduler::schedule` is total (conflicts are legitimate outcomes, not failures), and
per-claim-set state is already commutative and idempotent. A wave's follow-up enqueues
therefore cannot partially fail, and the same enqueue set reaches one canonical scheduler
state in any order — batch atomicity is vacuous and batch order-independence already
holds. Phase 2 enqueues obligations item-by-item during the commit phase. The AT-B7 seam
contract recorded at `scheduler.rs:424–431` stays binding on any future phase that does
add a batch API; nothing here weakens it.

### Q7 — Decay/recovery: **rule-layer operations over existing cell fields; no encoding change**

**Decision.** Decay and recovery are declarative rule operations (Q1 enum members), not
new cell machinery:

- **Target/floor:** the existing `StateCell.baseline` field is the declared
  decay/recovery target. A decay/recovery rule references the target definition and its
  baseline; rules on definitions without a baseline semantics are rejected at rule-set
  activation.
- **Rate/cadence:** rule parameters, referencing `ConfigRevision` keys where hot-tunable
  (ADR-0004 change class 1).
- **Closed-form catch-up (frozen):** decay/recovery computes the new value as a closed-
  form function of (current value, baseline, rate, whole elapsed cadence steps between
  `updated_at` and now) in `i128` with the Q1 floor rule, clamped to declared bounds and
  the baseline — linear per-step; curved profiles compose the piecewise-curve operation.
  Catch-up over N missed steps in one evaluation is therefore bit-identical to N separate
  evaluations, so batching/chunking cannot alter outcomes (§28 item 2, §18.3
  deterministic catch-up).
- **Occurrence identity:** each decay evaluation is ordinary scheduled work under its
  `(producer = decay rule, scope, occurrence)` sequence; re-arming consumes
  `checked_next` like all recurrence.
- **Expiry/cell removal: deliberately deferred.** Phase 2 removes no cell; §12.2's
  expiry metadata remains unimplemented until the persistence/pruning phase can make
  removal policy-bounded and explainable. Recorded as accepted, visible debt.

`DefinitionSpec`, `StateCell`, `ProfileManifest`, and `ConfigRevision` canonical
encodings are therefore **unchanged** in Phase 2, and every Phase-1 pinned digest value
remains bit-identical.

## 5. Delayed obligations (frozen)

An obligation is scheduled through the existing `Scheduler` under its complete `WorkKey`,
and its full record lives in a canonical **ObligationStore** keyed by
`WorkKey::identity_digest()`:

```text
obligation_id            = WorkKey identity digest
due_time, occurrence     (as in the WorkKey)
creator_rule_id
creator_definition_fingerprint
creator_behavior_epoch
creator_behavior_artifact_hash   (binds manifest + config + ruleset via the epoch record)
execution_mode           = MaterializedEffect { frozen resolved effect list }
                         | RuleReEvaluation   { exact rule fingerprint + ruleset hash }
```

**Payload identity becomes real (readiness invariant 3):** the scheduled
`WorkPayload.canonical_payload_hash` is frozen as the canonical hash of the obligation
record. The scheduler/timeline commitment machinery is thereby commiting to real content,
and the invariant "every scheduled evaluator `WorkKey`'s payload hash equals the hash of
the obligation record it references" is mechanically falsifiable.

**Execution (ADR-0006, frozen):** `MaterializedEffect` applies its frozen effects
through normal batch validation against the *current* activated schema — if the target
definitions' fingerprints no longer match the creator's, execution **refuses
explicitly** (typed error, obligation reported, nothing reinterpreted).
`RuleReEvaluation` resolves the exact originating rule by fingerprint/ruleset hash in the
activation lineage — resolution failure refuses explicitly. No obligation may switch
modes or execute under a different rule merely because an ID matches. Obligation-vs-
obligation identity collisions (same `WorkKey`, different record hash) are already
handled: the scheduler poisons the key order-independently, and the poisoned drain
removes the obligation records with the conflict report.

## 6. Occurrence addressing and cooldowns (frozen)

- **OccurrenceLedger:** a canonical, digest-committed map `(profile, producer definition,
  scope, work kind) → next OccurrenceIndex`. Allocation happens only in the commit phase
  (part of the atomic wave), via `checked_next`; `u64` exhaustion is a typed error that
  rejects the wave. ADR-0006 already lists occurrence counters as persisted continuity
  state.
- **Random-draw identity:** every probability-gate operation inside a rule carries its
  own declared stable dotted sub-ID (uniqueness validated at rule-set activation), and
  `RandomAddress.rule_or_trigger_id` is that gate's fully qualified ID. Two gates in one
  rule at the same occurrence therefore have distinct addresses with **no change to
  `RandomAddress`**. Call order, batching, and wave composition cannot alter unrelated
  draws (already structural: the service is stateless).
- **CooldownLedger:** a canonical, digest-committed map `(rule, scope) → expiry
  LogicalTime`. Entries are written in the commit phase by cooldown operations; an
  expired entry is removed deterministically when next consulted during evaluation of
  that `(rule, scope)` — removal is a commit-phase mutation, visible in the digest.
  Cooldown checks read the ledger during evaluation like any snapshot read.

## 7. Aggregation, threshold emission, and dormant boundedness (frozen)

- **Aggregation is a pure recomputation over the snapshot.** An aggregation operation
  declares its input definition(s), the child→aggregate scope mapping, and weights; it
  enumerates matching cells in stable `(definition, scope)` BTree order, accumulates in
  `i128`, applies the Q1 floor rule once, and emits one derived-cell effect at the
  aggregate scope. **No incremental accumulator store exists in Phase 2** — any future
  accumulator is a retained store subject to §8 in full. Insertion-permutation invariance
  is therefore structural, and "aggregation equivalence" (full recompute vs any staged
  recompute path) is a digest-equality test.
- **Threshold emission** compares the newly computed aggregate against the previously
  committed cell value (committed state, not hidden memory) and the declared threshold;
  a crossing emits its declared effects/obligations in the same batch discipline.
  Emission direction (rising/falling/both) is declared per rule.
- **Dormant/global boundedness:** dormant scopes evolve only through scheduled aggregate
  work at declared cadences — work scales with due keys and changed state, never with
  total addressable scopes (§27.2). The dormant→active boundary inherits ADR-0006
  unchanged: promotion materializes from committed aggregates and rules and never
  fabricates unrecorded microhistory.

## 8. Retained-store commitment (the R1/R2 law, applied to Phase 2 from day one)

Phase 2 introduces exactly these retained stores. Nothing else may retain
behavior-relevant state:

| Store | Class | Commitment |
|---|---|---|
| `ActivatedRuleSet` | immutable activation artifact | content-addressed (`ruleset_content_hash`, per-rule fingerprints); lineage-recorded like manifests |
| `EpochRegistry` | canonical | digest-committed; append-only per profile; hash-chained records |
| `ObligationStore` | canonical | digest-committed; bidirectional invariant with scheduler slots (§5) |
| `OccurrenceLedger` | canonical | digest-committed |
| `CooldownLedger` | canonical | digest-committed |
| wave evaluation buffers | transient | must not outlive the wave; falsified by equal-digest/same-next-input tests |
| any compiled/predicate cache | derived index | must be an exact recomputable function of activation artifacts, with the AT-H3-style derived-index invariant test; never independently authoritative |

**Engine digest (frozen).** Phase 2 adds one composed
`engine_state_digest = H("engine_state" ‖ state-store digest ‖ scheduler digest ‖
timeline digest ‖ epoch-registry digest ‖ obligation-store digest ‖ occurrence-ledger
digest ‖ cooldown-ledger digest)`. Every retained value that can change future canonical
behavior participates (R1); per-claim-set/per-input-set state is arrival-order
independent (R2).

**Mandatory falsification (binding on the writer).** Every store in the table ships,
from its first commit, with AT-G-style tests: (a) *discrimination* — two states
differing only in that store's retained content have different digests; (b) *equivalence*
— two states with equal digests return identical whole result values **and** identical
post-input digests for the same next input, across permuted construction orders; (c) for
derived indexes, the AT-H3-style recomputation invariant after every scripted prefix.
The Phase-1 lesson stands: this defect class consumed three correction cycles and **must
not be re-learned empirically**.

## 9. Panic freedom and module placement

`rules.rs`, `effects.rs`, `evaluator.rs` land inside `spark-engine` behind the existing
trust boundary; the evaluator reaches state only through the three existing `pub(crate)`
paths and exposes **no** public constructor that accepts caller-authored schema or rule
facts (rule admission goes through the one door). The strict lint gate and
validated-domain-type template apply unchanged; every new canonical constructor is total
or returns a typed error; the `Ord::clamp` boundary note carries forward.

## 10. Explicitly deferred (unchanged debts, no new ones)

Windows/Android executable parity (static checks only); cell expiry/pruning (Q7);
service-side epoch reload flow, profile parsing/IO, persistence backend, host facade
(Phase 3); single-`ActivationRegistry` composition remains convention + content
addressing until Phase 3 makes it structural.

## 11. Freeze summary

| Frozen item | Resolution |
|---|---|
| Rule representation | closed typed enum ops; separate content-addressed rule-set artifact; i64/fixed with i128 intermediates and one floor-rounding rule (Q1) |
| Stable-snapshot evaluation | borrow-split two-phase wave; snapshot = store between waves; identity = pre-wave engine digest (Q2) |
| EffectBatch | resolved-value effects; ≤1 write per target; distinct-value conflict ⇒ atomic wave rejection with order-independent evidence; sort by (definition, scope, path); validate-all-then-apply-all; digest chained per wave to barrier identity (Q3) |
| Cycle/fan-out/depth/backpressure | static rejection at rule-set activation **and** runtime budgets: volume ⇒ stable-prefix deferral, depth ⇒ conversion to scheduled work, structural ⇒ atomic rejection; budgets epoch-bound (Q4) |
| Behavior epoch | epoch record minted at its own canonical barrier; `behavior_artifact_hash` = epoch-record hash binding manifest+config+ruleset (Q5) |
| Scheduler batch API | not built; AT-B7 contract preserved for the future (Q6) |
| Decay/recovery | rule-layer closed-form catch-up over `baseline`; no encoding changes; expiry deferred (Q7) |
| Delayed obligations | full ADR-0006 record in canonical ObligationStore; payload hash = record hash; explicit refusal on fingerprint mismatch; modes never switch (§5) |
| Occurrence addressing | canonical OccurrenceLedger; per-gate qualified random IDs; commit-phase allocation (§6) |
| Aggregation/threshold | pure snapshot recomputation, stable-order accumulation, no hidden accumulators; threshold state = committed cells (§7) |
| Retained-store law | seven-store enumeration, engine digest, mandatory AT-G-style falsification per store (§8) |

## 12. Verdict

**PHASE_2_ARCHITECTURE_FROZEN.**

All seven open questions are resolved; every mandated semantics is frozen; no frozen
Phase-0/Phase-1 boundary is reopened; no production Rust was written.

PHASE_2_IMPLEMENTATION_AUTHORIZATION: NO (separate operator decision)
PHASE_3_AUTHORIZATION: NO
