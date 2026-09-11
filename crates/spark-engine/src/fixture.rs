//! Sanctioned test-only seams over the crate-internal state-write paths.
//!
//! This module exists only under `#[cfg(test)]` or the `test-support`
//! cargo feature, which is off by default and enabled solely by
//! `spark-testkit`'s **dev**-dependency edge. It is therefore not part of
//! the production surface, and the workspace feature-hygiene test asserts
//! mechanically that no production dependency edge enables it
//! (`PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md` §3.6).
//!
//! It exists to replace the two failure modes the Phase-1 record shows:
//! "make it public for the tests" (which is what produced B-02 three
//! times) and "leave it untestable" (which is what let the schema
//! validation rules go unfalsified). Cross-crate tests still go through
//! the production door wherever possible — building a `ProfileManifest`
//! and calling [`crate::activation::ActivationRegistry::activate`] *is*
//! the production path and is the preferred fixture style. These seams
//! are only for the write paths, which have no Phase-1 production caller
//! at all because the host/evaluator facade is Phase 2/3 scope.
//!
//! The seams are thin forwarders. They add no capability the crate's own
//! evaluator would not have: every call still runs the full schema
//! validation (profile, authority, value type, declared bounds, scope
//! compatibility) inside [`crate::state::StateStore`].

use crate::state::{StateStore, StateWriteError};
use spark_core::clock::LogicalTime;
use spark_core::id::{DefinitionId, ProfileId};
use spark_core::scope::ScopeId;
use spark_core::value::CanonicalValue;

/// Test-only forwarder to the `host_owned` write path.
#[allow(clippy::too_many_arguments)]
pub fn observe_host_owned(
    store: &mut StateStore,
    profile_id: ProfileId,
    definition_id: DefinitionId,
    scope_id: ScopeId,
    value: CanonicalValue,
    at: LogicalTime,
    behavior_epoch: u64,
) -> Result<(), StateWriteError> {
    store.observe_host_owned(
        profile_id,
        definition_id,
        scope_id,
        value,
        at,
        behavior_epoch,
    )
}

/// Test-only forwarder to the `spark_owned` write path.
#[allow(clippy::too_many_arguments)]
pub fn apply_spark_effect(
    store: &mut StateStore,
    profile_id: ProfileId,
    definition_id: DefinitionId,
    scope_id: ScopeId,
    value: CanonicalValue,
    at: LogicalTime,
    behavior_epoch: u64,
) -> Result<(), StateWriteError> {
    store.apply_spark_effect(
        profile_id,
        definition_id,
        scope_id,
        value,
        at,
        behavior_epoch,
    )
}

/// Test-only forwarder to the `derived` write path.
#[allow(clippy::too_many_arguments)]
pub fn commit_derived(
    store: &mut StateStore,
    profile_id: ProfileId,
    definition_id: DefinitionId,
    scope_id: ScopeId,
    value: CanonicalValue,
    at: LogicalTime,
    behavior_epoch: u64,
) -> Result<(), StateWriteError> {
    store.commit_derived(
        profile_id,
        definition_id,
        scope_id,
        value,
        at,
        behavior_epoch,
    )
}

// ---------------------------------------------------------------- Phase-2 seams
//
// Test-only observation and fault-injection seams for the Phase-2 acceptance
// oracle. None of them exists without `test-support`; none reaches a canonical
// report; the observation values are a distinct newtype with no conversion into
// any evaluation-context type (AT-I46).

use crate::engine::{Engine, EngineSnapshot, ExtractionRefusal, Observation, PreflightRow};
use crate::obligation::{ObligationMode, ObligationRecord};
use crate::request::Request;
use spark_core::hash::Digest;
use spark_core::scheduler::{DueWorkItem, WorkKey};
use spark_core::timeline::SemanticCommandEnvelope;

/// The observation log (pre-wave digests, outer-loop selections, finalization
/// operation log).
pub fn observation(engine: &Engine) -> &Observation {
    &engine.observation
}

pub fn clear_observation(engine: &mut Engine) {
    engine.observation = Observation::default();
}

/// AT-I48(j): disable one preflight row (test builds only).
pub fn disable_preflight_row(engine: &mut Engine, row: Option<PreflightRow>) {
    engine.disabled_row = row;
}

/// The engine extraction called directly (AT-I42(l)(m)(n)). Returns the
/// executable records removed and the conflicted keys.
#[allow(clippy::result_large_err)]
pub fn extract_least_due_slice(
    engine: &mut Engine,
    horizon: LogicalTime,
) -> Result<(Vec<ObligationRecord>, Vec<WorkKey>), ExtractionRefusal> {
    let e = engine.extract_least_due_slice(horizon)?;
    Ok((
        e.executable.into_iter().map(|(_, r)| r).collect(),
        e.conflicted
            .into_iter()
            .map(|(c, _)| c.key().clone())
            .collect(),
    ))
}

/// Replace (or delete, with `None`) the obligation claim set under `key`.
pub fn tamper_obligations(
    engine: &mut Engine,
    key: &WorkKey,
    records: Option<Vec<ObligationRecord>>,
) {
    engine.obligations.tamper(key, records);
}

/// Insert one record into both stores exactly as a committed wave would
/// (commutative mirror of `Scheduler::schedule`), for collision fixtures.
pub fn schedule_record(engine: &mut Engine, record: ObligationRecord) {
    engine.scheduler.schedule(DueWorkItem {
        key: record.key.clone(),
        payload: record.payload(),
    });
    engine.obligations.insert(record);
}

/// A re-evaluation record under an arbitrary key (collision fixtures).
pub fn reevaluation_record(
    key: WorkKey,
    rule_id: DefinitionId,
    rule_fingerprint: Digest,
    ruleset_content_hash: Digest,
    creator_emission_identity: Digest,
    behavior_artifact_hash: Digest,
) -> ObligationRecord {
    ObligationRecord {
        key,
        creator_rule_id: rule_id.clone(),
        creator_rule_fingerprint: rule_fingerprint.clone(),
        creator_behavior_epoch: 1,
        creator_behavior_artifact_hash: behavior_artifact_hash,
        creator_emission_identity,
        mode: ObligationMode::RuleReEvaluation {
            rule_id,
            rule_fingerprint,
            ruleset_content_hash,
        },
    }
}

/// A record with an arbitrary mode (fingerprint-drift fixtures).
pub fn record_with_mode(template: &ObligationRecord, mode: ObligationMode) -> ObligationRecord {
    let mut r = template.clone();
    r.mode = mode;
    r
}

/// Stage an envelope directly into the engine's timeline, bypassing the
/// engine, to construct the unexpected (non-engine-reachable) staging states
/// of AT-I48(c) P-6. Returns whether the call returned `Ok`.
pub fn stage_raw(engine: &mut Engine, envelope: SemanticCommandEnvelope) -> bool {
    let Ok(window) = engine.timeline.current_admission_window() else {
        return false;
    };
    let grant = engine.grant.clone();
    engine
        .timeline
        .stage(&grant, &envelope.submit_with(window.ticket()))
        .is_ok()
}

/// Snapshot tampering for restore-validation tests.
pub fn snapshot_set_active(snapshot: &mut EngineSnapshot, request: Option<&Request>) {
    snapshot.active = request.map(Request::discriminator);
}

pub fn snapshot_set_frontier(snapshot: &mut EngineSnapshot, frontier: LogicalTime) {
    snapshot.frontier = frontier;
}

pub fn snapshot_stage_raw(
    snapshot: &mut EngineSnapshot,
    envelope: SemanticCommandEnvelope,
) -> bool {
    let Ok(window) = snapshot.timeline.current_admission_window() else {
        return false;
    };
    let grant = snapshot.grant.clone();
    snapshot
        .timeline
        .stage(&grant, &envelope.submit_with(window.ticket()))
        .is_ok()
}

/// Recompute and store the snapshot digest (to isolate one validation step).
pub fn snapshot_reseal(snapshot: &mut EngineSnapshot) {
    let esd = {
        let mut enc = spark_core::hash::CanonicalEncoder::new();
        enc.push_str("engine_state");
        enc.push_digest(&snapshot.store.canonical_state_digest());
        enc.push_digest(&snapshot.scheduler.canonical_state_digest());
        enc.push_digest(&snapshot.timeline.canonical_state_digest());
        enc.push_digest(&snapshot.epochs.canonical_digest());
        enc.push_digest(&snapshot.obligations.canonical_digest());
        enc.push_digest(&snapshot.occurrences.canonical_digest());
        enc.push_digest(&snapshot.cooldowns.canonical_digest());
        enc.finish()
    };
    snapshot.recorded_stable_boundary_digest =
        crate::engine::stable_boundary_digest_of(&esd, snapshot.frontier, snapshot.active.as_ref());
}

/// Delete one obligation claim set inside a snapshot (bidirectional-invariant
/// restore test).
pub fn snapshot_drop_obligation(snapshot: &mut EngineSnapshot, key: &WorkKey) {
    snapshot.obligations.tamper(key, None);
}

/// AT-I6a / AT-I6c(d): evaluate every wave seed twice — exactly
/// (`Some(false)`: identical identity and payload, a redelivery that must
/// fold) or with an altered parameter payload under the same identity
/// (`Some(true)`: a contested emission that must reject). `None` disables.
pub fn set_seed_duplication(engine: &mut Engine, mode: Option<bool>) {
    engine.seed_duplication = mode;
}

/// AT-I6c(f): forge an empty parent set for every derived (wave `N >= 1`)
/// seed, which the next wave must reject as a typed evaluator defect.
pub fn forge_empty_parents(engine: &mut Engine, forge: bool) {
    engine.forge_empty_parents = forge;
}

/// AT-I28: overwrite one lineage entry's activation time inside a snapshot
/// (the engine's derived per-epoch index), to prove restore validates it.
pub fn snapshot_set_activation_time(
    snapshot: &mut EngineSnapshot,
    epoch_index: usize,
    at: Option<LogicalTime>,
) {
    if let Some(entry) = snapshot.lineage.get_mut(epoch_index) {
        entry.activated_at = at;
    }
}

/// AT-I28: drop the newest lineage entry inside a snapshot.
pub fn snapshot_truncate_lineage(snapshot: &mut EngineSnapshot) {
    snapshot.lineage.pop();
}

/// AT-I26: set one cooldown entry directly (only the `CooldownLedger` moves).
pub fn set_cooldown(engine: &mut Engine, rule: DefinitionId, scope: ScopeId, expiry: LogicalTime) {
    engine.cooldowns.set(rule, scope, expiry);
}

/// AT-I26: append one well-chained epoch record re-binding the current
/// artifacts (only the `EpochRegistry` moves; the lineage is deliberately not
/// extended, so such an engine must never be restored).
pub fn append_epoch_record(engine: &mut Engine) {
    let Some(current) = engine.epochs.current().cloned() else {
        return;
    };
    let Ok((epoch, previous)) = engine.epochs.successor() else {
        return;
    };
    let record = crate::epoch::EpochRecord::new(
        engine.profile_id.clone(),
        epoch,
        current.manifest_content_hash().clone(),
        current.config_revision_hash().clone(),
        current.ruleset_content_hash().clone(),
        engine.store.activation_hash().clone(),
        previous,
        crate::epoch::ActivatingBarrier::Genesis,
    );
    let _ = engine.epochs.append(record);
}

/// AT-I1: apply already-resolved committed effects to an engine's store at
/// `at` exactly as a committed wave does (including baseline
/// materialization) — "the same effects applied to an untouched clone".
pub fn apply_committed_effects(
    engine: &mut Engine,
    effects: &[crate::report::CommittedEffect],
    at: LogicalTime,
) {
    let epoch = engine.behavior_epoch();
    for e in effects {
        let _ = match e.write_path {
            crate::report::WritePath::SparkEffect => engine.store.apply_spark_effect(
                engine.profile_id.clone(),
                e.definition.clone(),
                e.scope.clone(),
                e.value.clone(),
                at,
                epoch,
            ),
            crate::report::WritePath::CommitDerived => engine.store.commit_derived(
                engine.profile_id.clone(),
                e.definition.clone(),
                e.scope.clone(),
                e.value.clone(),
                at,
                epoch,
            ),
        };
        if let Some(b) = engine.rule_set.baseline(&e.definition) {
            engine
                .store
                .materialize_baseline(&e.definition, &e.scope, b);
        }
    }
}

/// The number of retained per-epoch artifact entries (one per epoch record).
pub fn lineage_len(engine: &Engine) -> usize {
    engine.lineage.len()
}

/// AT-I8: record the scheduler ↔ `ObligationStore` bidirectional invariant
/// after **every committed wave** (into [`Observation::committed_waves`]).
/// Off by default: the exhaustive check is `O(n log n)` per wave.
pub fn probe_wave_invariants(engine: &mut Engine, enabled: bool) {
    engine.probe_wave_invariants = enabled;
}

/// AT-I28: the engine's per-epoch artifact lineage equals the value
/// recomputed from committed state (the registry and finalized commands) —
/// the same check restore runs.
pub fn lineage_recomputes(engine: &Engine) -> bool {
    engine.lineage_is_valid()
}

/// AT-I28: the timeline's derived finalized-history indexes equal the values
/// recomputed from the finalized history — the same check restore runs.
pub fn timeline_indexes_recompute(engine: &Engine) -> bool {
    engine.timeline.derived_indexes_consistent()
}

/// AT-I28 fault injection into a live engine's timeline index.
pub fn inject_timeline_index_fault(
    engine: &mut Engine,
    fault: spark_core::timeline::DerivedIndexFault,
) {
    engine.timeline.inject_derived_index_fault(fault);
}

/// AT-I28 fault injection into a snapshot's timeline index.
pub fn snapshot_inject_timeline_index_fault(
    snapshot: &mut EngineSnapshot,
    fault: spark_core::timeline::DerivedIndexFault,
) {
    snapshot.timeline.inject_derived_index_fault(fault);
}

/// Set one occurrence-ledger sequence (AT-I8 exhaustion fixtures; `u64`
/// exhaustion is otherwise unreachable in a test's lifetime).
pub fn set_occurrence_next(
    engine: &mut Engine,
    key: crate::ledger::OccurrenceLedgerKey,
    next: u64,
) {
    engine.occurrences.set_next(key, next);
}

/// The complete timeline state: the derived `Debug` rendering exposes every
/// private field (slots, poison evidence, staged and finalized identity
/// indexes, last-finalized sequences, fences, epoch metadata, frontier).
pub fn timeline_debug(engine: &Engine) -> String {
    format!("{:?}", engine.timeline)
}

/// An isolated working copy of the engine's timeline (the AT-I48(e)
/// differential reference; never the production mechanism, D-6).
pub fn timeline_clone(engine: &Engine) -> spark_core::timeline::TimelineIngress {
    engine.timeline.clone()
}

/// Override the engine's sequencer grant (P-3 is unreachable otherwise).
pub fn set_grant(engine: &mut Engine, grant: spark_core::id::SourceId) {
    engine.grant = grant;
}

/// Replace the engine's timeline with one resumed at `frontier` (the
/// Phase-1 `test-support` constructor), to reach ordinal-space exhaustion.
pub fn resume_timeline_at(engine: &mut Engine, frontier: u64, window_width: u32) {
    if let Ok(t) = spark_core::timeline::TimelineIngress::resume_at_frontier(
        engine.profile_id.clone(),
        engine.timeline.timeline_epoch(),
        engine.grant.clone(),
        window_width,
        spark_core::timeline::Ordinal(frontier),
    ) {
        engine.timeline = t;
    }
}
