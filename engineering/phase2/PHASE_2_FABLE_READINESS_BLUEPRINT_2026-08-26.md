# S.P.A.R.K. Phase 2 — Fable Readiness Blueprint

**Date:** 2026-08-26
**Status:** READINESS ASSESSMENT ONLY. **Phase 2 is NOT authorized**, no Phase-2 code exists, and this document authorizes none. It records what Phase 1 has prepared, what remains open, and what the Phase-2 architecture pass must decide — so that when the operator authorizes Phase 2, the entry is a decision, not an excavation.
**Preconditions before any Phase-2 work:** (1) the Phase-1 closure pass and its Codex review return `PHASE_1_CLOSED`; (2) explicit operator authorization of Phase 2.

---

## 1. What Phase 2 is (frozen scope, blueprint §32)

The rule and effect runtime: declarative operations (thresholds, add/subtract/scale/clamp, weighted sums, piecewise curves, probability gates, scope mapping, delays/cooldowns, decay/recovery, aggregation/threshold emission — §19.3, and **no general scripting language**); stable-snapshot evaluation; deterministic effect batches (collect → sort → validate → atomic commit waves, §19.4); delayed obligations; cycle/fan-out validation; no zero-delay recursive cycles (§17).

## 2. What Phase 1 has already prepared (verified in the tree)

| Seam | Where | State |
|---|---|---|
| Evaluator landing zone | `spark-engine` (`rules.rs`, `effects.rs`, `evaluator.rs` per the Fable §8 map) behind the same trust boundary; the three `pub(crate)` write paths (`observe_host_owned`, `apply_spark_effect`, `commit_derived`) exist, are authority-hard-coded, validate profile/type/bounds/scope, and have no public caller by construction | **READY** |
| Activated schema as the evaluator's ground truth | `ActivatedProfile`/`ActivatedDefinition` with door-computed fingerprints; content-addressed activation + manifest hashes on every store | **READY** |
| Deterministic work feed | `Scheduler` with complete semantic `WorkKey`, order-independent conflict poisoning, digest-complete state; `DrainOutcome` separates executable work from conflict reports | **READY** |
| Canonical command ingress | Timeline with semantic/admission type split, fences, epoch resets; after the closure pass, admission behavior is fully claim-set-determined | **READY after closure** |
| Randomness | `RandomAddress` fully qualified (seed, profile, epoch, behavior-artifact digest, producer, scope, occurrence); stateless service | **READY** |
| Panic-free discipline | validated-domain types + strict lint gate; the stated template for every new canonical constructor (Fable §5.3) applies to all rule/effect types | **READY (template)** |
| Batch-scheduling contract | scheduler docs record: batch conflict handling must be atomic and order-independent (AT-B7 seam) | **CONTRACT ONLY** |
| Bounded contested-state evidence | one discipline in two (post-S2: one) implementations, reusable if rule evaluation acquires contested outputs | **READY** |

## 3. Inherited invariants that bind every Phase-2 design (non-negotiable)

1. **R1/R2 from the closure pass:** every retained value that can change future canonical behavior participates in canonical state identity, and per-claim-set state is arrival-order-independent. Every new Phase-2 store (rule cache, obligation queue, aggregation accumulator, cooldown table) must ship with its digest commitment and an AT-G-style hidden-state discrimination test from day one — this class of defect consumed three Phase-1 correction cycles; Phase 2 must not re-learn it.
2. **One trust boundary:** rules/effects/evaluator live in `spark-engine`; the evaluator reaches state only through the existing `pub(crate)` paths; no new public write surface, no public evaluator constructor that accepts caller-authored schema facts.
3. **Effects are data, not code** (ADR-0004 security rule); `WorkPayload`/`CommandKind` opacity is replaced by typed declarative payloads whose canonical encoding becomes real — a payload's canonical hash must remain the identity the scheduler/timeline already commit to.
4. **ADR-0002 write classes** at the effect level: no effect batch may write host-owned state; derived cells only from the evaluator; host observations only through ingress.
5. **Deterministic waves:** stable snapshot in, sorted validated batch out, atomic commit, work bounded per cycle with visible deferral (§19.4–§19.5); no zero-delay cycles (feedback crosses a scheduled boundary).
6. **Delayed obligations** carry ADR-0006 binding (creator fingerprint, epoch, artifact hashes, explicit materialized-effect vs rule-re-evaluation mode) from their first implementation — the fields deliberately stay out of `WorkKey` and live in the obligation record.
7. **Panic-free canonical API policy** and the strict lint gate extend to every new module unchanged.

## 4. Open questions the Phase-2 architecture pass must decide (not now)

1. Rule representation: typed enum AST vs table-driven operations; how piecewise curves and weighted sums stay integer/fixed-point exact.
2. Snapshot mechanics: copy-on-write vs generation-stamped reads for "evaluate against a stable snapshot" inside one `StateStore`.
3. Effect-batch identity: canonical encoding and digest of an `EffectBatch`, and its relation to command barriers (ADR-0003 §14).
4. Cycle/fan-out validation placement: activation-time static analysis vs evaluation-time budgets vs both (blueprint wants both: reject illegal cycles, bound and report fan-out/depth).
5. Behavior-epoch activation: binding `ConfigRevision` + manifest into an explicit epoch at a canonical barrier (ADR-0004) — the hashes are exposed and paired in the testkit today; the barrier machinery is Phase-2/3 work.
6. Whether the scheduler batch API (AT-B7) is actually needed by the evaluator, and if so its atomic order-independent semantics.
7. Decay/recovery representation (per-cell metadata is already sketched in `StateCell`: expiry/decay fields are blueprint-listed but not yet present — adding them is a Phase-2 schema decision, and their absence today is deliberate).

## 5. Known debts Phase 2 inherits (do not lose)

- Windows/Android remain **static compile checks only**; executable fixture replay and cross-platform digest parity are unproven (ADR-0001 verification requires an executable Android path before the cross-platform acceptance criterion is claimed).
- Fully-qualified `Ord::clamp` remains a documented std boundary.
- m-02 (duplicate-ID manifest hash order sensitivity) if the operator deferred it out of the closure pass.
- The composition rule (one `ActivationRegistry` per deployment) is convention + content addressing until Phase 3 makes it structural.
- No profile parsing/IO exists (`spark-profile-io` is Phase 3); Phase-2 tests keep building manifests in code.

## 6. Recommended Phase-2 process

Same three-role separation that converged Phase 1: Fable architecture pass first (deciding §4's questions against this readiness base, producing the invariant/test matrix before any code), Opus writer strictly bound to it, Codex independent review with conformance + falsification. Test-first with recorded red baselines throughout. The convergence rule stands: a defect class surviving two writer passes escalates to architecture review, not a third loop.

## 7. Readiness verdict

**Phase-2 readiness: PREPARED, PENDING PHASE-1 CLOSURE.** No architectural unknown blocks Phase-2 planning; every entry seam is named, every binding invariant is stated, and the open questions are enumerated and bounded. Phase 2 remains unauthorized.

PHASE_2_IMPLEMENTATION_AUTHORIZATION: NO
