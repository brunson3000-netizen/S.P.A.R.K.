//! The Phase-2 engine composition: one profile's canonical stores, the serialized
//! request boundary, deterministic cohort-granular scheduled-work processing, the
//! wave pipeline, atomic single-command finalization, and snapshot/restore/replay.
//!
//! Controlling architecture: the Phase-2 freezes v1 → v3 as amended by the
//! accepted V3-F01 FINAL candidate and Revision 2, with both acceptance pins.
//!
//! # The facade (FINAL §10, §18; Revision 2 §4.1)
//!
//! The engine owns its `Scheduler`, `TimelineIngress`, `StateStore`, and every
//! Phase-2 store by value. No public item returns `&Scheduler`,
//! `&mut Scheduler`, `&TimelineIngress`, `&mut TimelineIngress`, or `&mut` of any
//! store. The host reaches only [`Engine::process`], the timeline epoch-reset
//! operation, read-only digests and views, `F` and `ActiveRequest` read-only, and
//! snapshot/restore/reconstruction. There is no staging, fencing, arbitrary
//! removal, cancellation, or `ActiveRequest` abandonment path. Rule evaluation
//! receives an `EvalView` that structurally excludes the horizon, `F`,
//! `ActiveRequest`, and pacing state.

use crate::activation::ActivatedProfile;
use crate::effects::{
    candidate_set_digest, canonicalize_candidates, command_cohort_identity, curve, decay,
    emission_identity, provenance_of, reduce, scale, scheduled_cohort_identity, weighted_sum,
    Candidate, EmissionContext, Intent, ParentContext,
};
use crate::epoch::{ActivatingBarrier, EpochRecord, EpochRegistry};
use crate::ledger::{CooldownLedger, OccurrenceLedger, OccurrenceLedgerKey};
use crate::obligation::{
    ClaimSet, FrozenIntent, MaterializedEffect, ObligationMode, ObligationRecord, ObligationStore,
};
use crate::profile::config::ConfigRevision;
use crate::report::{
    CohortKind, CohortOutcome, CommittedEffect, ConflictEntry, CoverageStatus, ObligationRefusal,
    PacingDiagnostics, PreflightFailure, Provenance, WaveRejection, WaveReport, WritePath,
    PROVENANCE_PRUNING_POLICY,
};
use crate::request::{
    canonicalize_active_request, CommandPayload, CommandRequest, FinalizationRefusal, Outcome,
    ProcessResult, Request, RequestDiscriminator,
};
use crate::rules::{
    ActivatedRule, ActivatedRuleSet, Condition, Expr, Input, ScheduleMode, ScopeRef, Trigger,
    Update,
};
use crate::state::{StateStore, StateWriteError};
use spark_core::authority::Authority;
use spark_core::canonical_tag;
use spark_core::clock::LogicalTime;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{CanonicalTag, DefinitionId, FenceId, ProfileId, SourceId};
use spark_core::random::{RandomAddress, RandomAddressService};
use spark_core::scheduler::{
    DueWorkItem, OccurrenceIndex, Scheduler, WorkKey, WorkKeyConflict, WorkKind, WorkSlotStatus,
};
use spark_core::scope::ScopeId;
use spark_core::timeline::{
    compute_ordered_stream_digest, AcknowledgedSlotState, CommandKind, EpochResetError,
    EpochResetRecord, FinalizedCommand, Ordinal, SlotStatus, TimelineConfigError, TimelineEpoch,
    TimelineFence, TimelineIngress,
};
use spark_core::value::{CanonicalValue, FixedPoint, ValueType};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Work kind of a depth-overflow conversion (v1 Q4, v2 §11).
pub const DEPTH_OVERFLOW_WORK_KIND: CanonicalTag = canonical_tag!("spark.depth_overflow");
/// The reserved epoch-activation command kind (v1 Q5).
pub const EPOCH_ACTIVATION_COMMAND_KIND: CanonicalTag = canonical_tag!("spark.epoch.activate");
/// The config key carrying the random root seed, so the seed is committed through
/// the config revision hash in the epoch record (R1) rather than held as an
/// uncommitted engine field.
pub const ROOT_SEED_CONFIG_KEY: &str = "random.root_seed";

// ================================================================ genesis

/// Timeline genesis metadata (Revision 2 §6.1(c)).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineGenesis {
    pub timeline_epoch: TimelineEpoch,
    pub sequencer: SourceId,
    pub window_width: u32,
}

/// One re-evaluation obligation present in the genesis state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitialWork {
    pub rule_id: DefinitionId,
    pub scope: ScopeId,
    pub due_time: LogicalTime,
    pub work_kind: WorkKind,
}

/// Everything that fixes an engine's genesis state (Revision 2 §6.1(b)(c)).
#[derive(Debug, Clone)]
pub struct EngineGenesis {
    pub profile: ActivatedProfile,
    pub rule_set: ActivatedRuleSet,
    pub config: ConfigRevision,
    pub timeline: TimelineGenesis,
    pub start_time: LogicalTime,
    pub initial_work: Vec<InitialWork>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenesisError {
    RuleSetNotBoundToProfile,
    ConfigProfileMismatch,
    MissingConfigKey(DefinitionId),
    Timeline(TimelineConfigError),
    UnknownInitialRule(DefinitionId),
    InitialWorkExceedsAdmission,
    OccurrenceExhausted,
}

impl fmt::Display for GenesisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "engine genesis rejected: {self:?}")
    }
}

impl std::error::Error for GenesisError {}

// ================================================================ errors

/// The engine-owned cross-store extraction's typed refusals (FINAL §11). Every
/// refusal leaves both stores byte-identical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractionRefusal {
    NoSliceDue {
        least_resident_due_time: Option<LogicalTime>,
    },
    ObligationRecordMissing {
        key: WorkKey,
    },
    ObligationRecordMismatch {
        key: WorkKey,
    },
    ObligationClaimSetMismatch {
        key: WorkKey,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResetRefusal {
    FailStopped,
    Timeline(EpochResetError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotRefused {
    /// No snapshot is published from a fail-stopped instance (D-8).
    FailStopped,
}

/// Restore validation failures (FINAL §6.5 as amended by Revision 2 §7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreError {
    /// Step 1: definition fingerprints / activation lineage / artifact binding.
    ArtifactBindingMismatch,
    EpochChainInvalid,
    /// Step 2.
    ActiveBehindFrontier,
    /// Step 2b (Revision 2 §7).
    TimelineStagingPresent,
    /// Derived private indexes are validated, not trusted.
    DerivedIndexesInconsistent,
    BidirectionalInvariantBroken,
    /// Step 3.
    DigestMismatch,
}

/// History reconstruction failures (Revision 2 §6.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayError {
    Genesis(GenesisError),
    /// A replayed command did not complete finalized, or a reset was refused.
    StepRefused {
        step: String,
    },
    /// `REPLAY_DIVERGED`: recomputed history or stable-boundary digest differs.
    Diverged,
}

// ================================================================ snapshot / history

/// A committed stable-boundary snapshot: stores, timeline, `F`, `ActiveRequest`,
/// the current artifacts, and the recorded `stable_boundary_digest`. An
/// in-memory value — no persistence backend and no crash-recovery claim.
#[derive(Debug, Clone)]
pub struct EngineSnapshot {
    pub(crate) profile_id: ProfileId,
    pub(crate) store: StateStore,
    pub(crate) scheduler: Scheduler,
    pub(crate) timeline: TimelineIngress,
    pub(crate) grant: SourceId,
    pub(crate) epochs: EpochRegistry,
    pub(crate) obligations: ObligationStore,
    pub(crate) occurrences: OccurrenceLedger,
    pub(crate) cooldowns: CooldownLedger,
    pub(crate) rule_set: ActivatedRuleSet,
    pub(crate) config: ConfigRevision,
    pub(crate) frontier: LogicalTime,
    pub(crate) active: Option<RequestDiscriminator>,
    pub(crate) recorded_stable_boundary_digest: Digest,
}

impl EngineSnapshot {
    pub fn recorded_stable_boundary_digest(&self) -> &Digest {
        &self.recorded_stable_boundary_digest
    }

    pub fn frontier(&self) -> LogicalTime {
        self.frontier
    }

    pub fn active_request(&self) -> Option<&RequestDiscriminator> {
        self.active.as_ref()
    }
}

/// The declared inputs of completed-boundary reconstruction beyond genesis
/// (Revision 2 §6.1(c)–(e)): the finalized commands with their payloads in
/// ordinal order, every epoch-reset record, `F`, and the recorded digests.
#[derive(Debug, Clone)]
pub struct CompletedHistory {
    pub commands: Vec<CommandRequest>,
    pub resets: Vec<EpochResetRecord>,
    pub frontier: LogicalTime,
    pub recorded_history_digest: Digest,
    pub recorded_stable_boundary_digest: Digest,
}

// ================================================================ observation seam

/// An observed digest: a distinct newtype with no conversion into any
/// evaluation-context type (AT-I46(d)).
#[cfg(any(test, feature = "test-support"))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObservedDigest(pub(crate) Digest);

#[cfg(any(test, feature = "test-support"))]
impl ObservedDigest {
    pub fn value(&self) -> &Digest {
        &self.0
    }
}

/// One post-extraction, pre-evaluation observation point.
#[cfg(any(test, feature = "test-support"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrewaveObservation {
    pub cohort_identity: Digest,
    pub wave_index: u32,
    pub engine_digest: ObservedDigest,
    pub scheduler_digest: ObservedDigest,
    pub active_request_set: bool,
}

/// Finalization operation log entries (AT-I48(d)).
#[cfg(any(test, feature = "test-support"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizeOp {
    PositiveStagedAcknowledgement { ordinal: Ordinal },
    OneOrdinalFence { ordinal: Ordinal, fence_id: String },
}

/// Preflight rows, for the AT-I48(j) fault-injection seam.
#[cfg(any(test, feature = "test-support"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreflightRow {
    P5,
    P6,
    P7,
    P8,
    P9,
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Observation {
    pub prewave: Vec<PrewaveObservation>,
    /// X-1 selections at the outer-loop seam A2 only (AT-I39(C)).
    pub outer_selections: u64,
    pub finalize_ops: Vec<FinalizeOp>,
}

// ================================================================ the engine

/// One profile's Phase-2 engine.
#[derive(Debug, Clone)]
pub struct Engine {
    pub(crate) profile_id: ProfileId,
    pub(crate) store: StateStore,
    pub(crate) scheduler: Scheduler,
    pub(crate) timeline: TimelineIngress,
    pub(crate) grant: SourceId,
    pub(crate) epochs: EpochRegistry,
    pub(crate) obligations: ObligationStore,
    pub(crate) occurrences: OccurrenceLedger,
    pub(crate) cooldowns: CooldownLedger,
    pub(crate) rule_set: ActivatedRuleSet,
    pub(crate) config: ConfigRevision,
    pub(crate) frontier: LogicalTime,
    pub(crate) active: Option<RequestDiscriminator>,
    pub(crate) fail_stopped: bool,
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) observation: Observation,
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) disabled_row: Option<PreflightRow>,
}

/// A numeric view of a canonical value (`Int` value or `Fixed` raw).
fn numeric(v: &CanonicalValue) -> Option<i64> {
    match v {
        CanonicalValue::Int(i) => Some(*i),
        CanonicalValue::Fixed(f) => Some(f.raw()),
        _ => None,
    }
}

fn arithmetic(what: &'static str) -> WaveRejection {
    WaveRejection::Arithmetic { what }
}

fn advance_time(t: LogicalTime, by: u64, what: &'static str) -> Result<LogicalTime, WaveRejection> {
    t.0.checked_add(by).map(LogicalTime).ok_or(arithmetic(what))
}

/// The first eight bytes of a digest as a `u64` occurrence discriminator for
/// derived evaluations (probability-gate addresses at wave >= 1).
fn digest_u64(d: &Digest) -> u64 {
    let b = d.as_bytes();
    let mut out = [0u8; 8];
    for (o, v) in out.iter_mut().zip(b.iter()) {
        *o = *v;
    }
    u64::from_le_bytes(out)
}

fn write_path_of(authority: Authority) -> Option<WritePath> {
    match authority {
        Authority::SparkOwned => Some(WritePath::SparkEffect),
        Authority::Derived => Some(WritePath::CommitDerived),
        Authority::HostOwned => None,
    }
}

/// A seed of one wave: something that emits candidates at that wave.
#[derive(Debug, Clone)]
enum Seed {
    Rule {
        rule_id: DefinitionId,
        subject: ScopeId,
        parent: ParentContext,
        occurrence: u64,
        params: Vec<i64>,
    },
    Materialized {
        record: ObligationRecord,
    },
}

#[derive(Debug, Clone)]
struct EnqueueClaim {
    identity: Digest,
    ledger_key: OccurrenceLedgerKey,
    due_time: LogicalTime,
    creator_rule: DefinitionId,
    mode: ObligationMode,
}

#[derive(Debug, Default)]
struct Draft {
    candidates: Vec<Candidate>,
    enqueues: Vec<EnqueueClaim>,
    cooldown_writes: BTreeMap<(DefinitionId, ScopeId), LogicalTime>,
    cooldown_removals: BTreeSet<(DefinitionId, ScopeId)>,
}

/// The read-only evaluation view: the stable pre-wave snapshot. It holds no
/// horizon, no `F`, no `ActiveRequest`, and no pacing diagnostics, so rule
/// evaluation structurally cannot read them.
struct EvalView<'a> {
    profile_id: &'a ProfileId,
    store: &'a StateStore,
    cooldowns: &'a CooldownLedger,
    rule_set: &'a ActivatedRuleSet,
    config: &'a ConfigRevision,
    behavior_epoch: u64,
    artifact: Digest,
    root_seed: u64,
}

struct EvalCtx<'a> {
    cohort: &'a Digest,
    wave: u32,
    parent: Digest,
    now: LogicalTime,
    params: &'a [i64],
    occurrence: u64,
}

impl EvalView<'_> {
    fn cell(&self, definition: &DefinitionId, scope: &ScopeId) -> Option<i64> {
        self.store
            .get(self.profile_id, definition, scope)
            .and_then(|c| numeric(&c.value))
    }

    fn resolve(scope: &ScopeRef, subject: &ScopeId) -> ScopeId {
        match scope {
            ScopeRef::Subject => subject.clone(),
            ScopeRef::Fixed(s) => s.clone(),
        }
    }

    fn input(&self, input: &Input, subject: &ScopeId, ctx: &EvalCtx<'_>) -> i64 {
        match input {
            Input::Cell {
                definition,
                scope,
                absent,
            } => self
                .cell(definition, &Self::resolve(scope, subject))
                .unwrap_or(*absent),
            Input::Config { key } => self.config.get(key).and_then(numeric).unwrap_or(0),
            Input::Param { index } => ctx.params.get(usize::from(*index)).copied().unwrap_or(0),
            Input::Literal(v) => *v,
        }
    }

    fn expr(&self, e: &Expr, subject: &ScopeId, ctx: &EvalCtx<'_>) -> Result<i64, WaveRejection> {
        match e {
            Expr::Input(i) => Ok(self.input(i, subject, ctx)),
            Expr::WeightedSum(terms) => {
                let pairs: Vec<(i64, i64)> = terms
                    .iter()
                    .map(|(w, i)| (w.raw(), self.input(i, subject, ctx)))
                    .collect();
                weighted_sum(&pairs)
            }
            Expr::Curve { input, points } => curve(self.input(input, subject, ctx), points),
        }
    }

    fn aggregate(&self, source: &DefinitionId, weight: &FixedPoint) -> Result<i64, WaveRejection> {
        let values: Vec<i64> = self
            .store
            .cells_of(source)
            .filter_map(|c| numeric(&c.value))
            .collect();
        let mut acc: i128 = 0;
        for v in values {
            acc = acc
                .checked_add(i128::from(v))
                .ok_or(arithmetic("aggregate"))?;
        }
        let clamped = i64::try_from(acc).map_err(|_| arithmetic("aggregate"))?;
        scale(clamped, weight.raw())
    }

    /// Evaluates one rule at `subject` into `draft`. Pure over the view.
    fn evaluate_rule(
        &self,
        rule: &ActivatedRule,
        subject: &ScopeId,
        ctx: &EvalCtx<'_>,
        draft: &mut Draft,
    ) -> Result<(), WaveRejection> {
        let spec = rule.spec();
        if spec.cooldown.is_some() {
            if let Some(expiry) = self.cooldowns.expiry_of(&spec.rule_id, subject) {
                if expiry > ctx.now {
                    return Ok(());
                }
                // An expired entry consulted now is removed deterministically in
                // the commit phase (v1 §6).
                draft
                    .cooldown_removals
                    .insert((spec.rule_id.clone(), subject.clone()));
            }
        }
        for c in &spec.conditions {
            let holds = match c {
                Condition::Compare { left, op, right } => op.holds(
                    self.expr(left, subject, ctx)?,
                    self.expr(right, subject, ctx)?,
                ),
                Condition::Chance { sub_id, rate } => {
                    let gate =
                        DefinitionId::new(format!("{}.{}", spec.rule_id.as_str(), sub_id.as_str()))
                            .map_err(|_| arithmetic("chance_gate_id"))?;
                    let address = RandomAddress {
                        root_seed: self.root_seed,
                        profile_id: self.profile_id.clone(),
                        behavior_epoch: self.behavior_epoch,
                        behavior_artifact_hash: self.artifact.clone(),
                        rule_or_trigger_id: gate,
                        scope_id: subject.clone(),
                        occurrence_index: ctx.occurrence,
                    };
                    RandomAddressService::new().derive_fixed_fraction(&address) < rate.raw()
                }
            };
            if !holds {
                return Ok(());
            }
        }
        let emission_ctx = EmissionContext {
            cohort_identity: ctx.cohort,
            wave_index: ctx.wave,
            parent_context: &ctx.parent,
            rule_fingerprint: rule.fingerprint(),
            producer_scope: subject,
            behavior_artifact_hash: &self.artifact,
        };
        for ((target, scope_ref), ops) in spec.target_groups() {
            let scope = Self::resolve(&scope_ref, subject);
            let Some(definition) = self.store.schema_of(self.profile_id, &target) else {
                return Err(WaveRejection::InvalidEffect {
                    definition: target.clone(),
                    scope,
                    error: Box::new(StateWriteError::UndeclaredDefinition {
                        profile_id: self.profile_id.clone(),
                        definition_id: target.clone(),
                    }),
                });
            };
            let Some(write_path) = write_path_of(definition.authority()) else {
                return Err(WaveRejection::InvalidEffect {
                    definition: target.clone(),
                    scope,
                    error: Box::new(StateWriteError::WrongAuthorityForWritePath {
                        profile_id: self.profile_id.clone(),
                        definition_id: target.clone(),
                        declared: definition.authority(),
                        attempted_path: "evaluator",
                    }),
                });
            };
            let Some(intent) = self.group_intent(rule, &target, &scope, &ops, subject, ctx)? else {
                continue;
            };
            let sub_ids: Vec<&CanonicalTag> = ops.iter().map(|o| &o.sub_id).collect();
            let identity = emission_identity(&emission_ctx, &sub_ids, &target, &scope);
            draft.candidates.push(Candidate {
                definition: target,
                scope,
                write_path,
                identity,
                intent,
            });
        }
        for s in &spec.schedules {
            let scope = Self::resolve(&s.scope, subject);
            let due_time = advance_time(ctx.now, s.delay, "delayed_time")?;
            let mode = match &s.mode {
                ScheduleMode::ReEvaluate { rule: named } => {
                    let Some(target_rule) = self.rule_set.rule(named) else {
                        return Err(arithmetic("unknown_reevaluation_rule"));
                    };
                    ObligationMode::RuleReEvaluation {
                        rule_id: named.clone(),
                        rule_fingerprint: target_rule.fingerprint().clone(),
                        ruleset_content_hash: self.rule_set.content_hash().clone(),
                    }
                }
                ScheduleMode::Materialize { effects } => {
                    let mut frozen = Vec::new();
                    let mut fingerprints = BTreeMap::new();
                    for e in effects {
                        let target_scope = Self::resolve(&e.scope, subject);
                        let intent = match &e.update {
                            Update::Add(x) => FrozenIntent::AddDelta(self.expr(x, subject, ctx)?),
                            Update::Subtract(x) => FrozenIntent::AddDelta(
                                self.expr(x, subject, ctx)?
                                    .checked_neg()
                                    .ok_or(arithmetic("subtract_negation"))?,
                            ),
                            Update::Assign(x) => FrozenIntent::Result {
                                value: self.expr(x, subject, ctx)?,
                                family: x.result_family(),
                            },
                            _ => return Err(arithmetic("invalid_materialized_update")),
                        };
                        if let Some(d) = self.store.schema_of(self.profile_id, &e.target) {
                            fingerprints.insert(e.target.clone(), d.fingerprint().clone());
                        }
                        frozen.push(MaterializedEffect {
                            sub_id: e.sub_id.clone(),
                            target: e.target.clone(),
                            scope: target_scope,
                            intent,
                        });
                    }
                    ObligationMode::MaterializedEffect {
                        effects: frozen,
                        target_fingerprints: fingerprints,
                    }
                }
            };
            let identity = emission_identity(&emission_ctx, &[&s.sub_id], &spec.rule_id, &scope);
            draft.enqueues.push(EnqueueClaim {
                identity,
                ledger_key: OccurrenceLedgerKey {
                    profile_id: self.profile_id.clone(),
                    producer: spec.rule_id.clone(),
                    scope_id: scope,
                    work_kind: s.work_kind.clone(),
                },
                due_time,
                creator_rule: spec.rule_id.clone(),
                mode,
            });
        }
        if let Some(cooldown) = spec.cooldown {
            let expiry = advance_time(ctx.now, cooldown, "cooldown_expiry")?;
            draft
                .cooldown_writes
                .insert((spec.rule_id.clone(), subject.clone()), expiry);
        }
        Ok(())
    }

    /// The single intent of one rule's operation group on one target.
    fn group_intent(
        &self,
        rule: &ActivatedRule,
        target: &DefinitionId,
        scope: &ScopeId,
        ops: &[&crate::rules::EmitOp],
        subject: &ScopeId,
        ctx: &EvalCtx<'_>,
    ) -> Result<Option<Intent>, WaveRejection> {
        let current = self.cell(target, scope);
        let cell = self.store.get(self.profile_id, target, scope);
        let op_fp = |family_tag: &str| {
            let mut enc = CanonicalEncoder::new();
            enc.push_str("operation_fingerprint_v1");
            enc.push_digest(rule.fingerprint());
            enc.push_str(family_tag);
            for o in ops {
                o.sub_id.canonicalize(&mut enc);
            }
            enc.finish()
        };
        let all_additive = ops
            .iter()
            .all(|o| matches!(o.update, Update::Add(_) | Update::Subtract(_)));
        if all_additive {
            let mut total: i64 = 0;
            for o in ops {
                let v = match &o.update {
                    Update::Add(x) => self.expr(x, subject, ctx)?,
                    Update::Subtract(x) => self
                        .expr(x, subject, ctx)?
                        .checked_neg()
                        .ok_or(arithmetic("subtract_negation"))?,
                    _ => 0,
                };
                total = total.checked_add(v).ok_or(arithmetic("rule_body_delta"))?;
            }
            return Ok(Some(Intent::AddDelta(total)));
        }
        if let [only] = ops {
            return Ok(match &only.update {
                Update::Assign(x) => Some(Intent::Result {
                    value: self.expr(x, subject, ctx)?,
                    family: x.result_family(),
                }),
                Update::Aggregate { source, weight } => Some(Intent::Result {
                    value: self.aggregate(source, weight)?,
                    family: crate::rules::ResultFamily::Aggregate,
                }),
                Update::Scale(f) => Some(Intent::Transform {
                    family: crate::rules::TransformFamily::Scale,
                    op_fingerprint: op_fp("scale"),
                    resolved: scale(current.unwrap_or(0), f.raw())?,
                }),
                Update::Clamp { min, max } => Some(Intent::Transform {
                    family: crate::rules::TransformFamily::Clamp,
                    op_fingerprint: op_fp("clamp"),
                    resolved: current.unwrap_or(0).max(*min).min(*max),
                }),
                Update::Decay {
                    toward,
                    rate,
                    cadence,
                } => {
                    let Some(cell) = cell else {
                        return Ok(None);
                    };
                    let Some(value) = numeric(&cell.value) else {
                        return Ok(None);
                    };
                    let elapsed = ctx
                        .now
                        .0
                        .checked_sub(cell.updated_at.0)
                        .ok_or(arithmetic("decay_backwards"))?;
                    let target_value = match cell.baseline.as_ref().and_then(numeric) {
                        Some(b) => b,
                        None => self.expr(toward, subject, ctx)?,
                    };
                    decay(
                        value,
                        target_value,
                        self.expr(rate, subject, ctx)?,
                        elapsed,
                        *cadence,
                    )?
                    .map(|resolved| Intent::Transform {
                        family: crate::rules::TransformFamily::Decay,
                        op_fingerprint: op_fp("decay"),
                        resolved,
                    })
                }
                Update::Add(_) | Update::Subtract(_) => None,
            });
        }
        // Several operations of one rule on one target: the rule body is the
        // declared reducer, composed in declared stage order (v2 §4.2).
        let mut v = current.unwrap_or(0);
        for o in ops {
            v = match &o.update {
                Update::Add(x) => v
                    .checked_add(self.expr(x, subject, ctx)?)
                    .ok_or(arithmetic("rule_body"))?,
                Update::Subtract(x) => v
                    .checked_sub(self.expr(x, subject, ctx)?)
                    .ok_or(arithmetic("rule_body"))?,
                Update::Assign(x) => self.expr(x, subject, ctx)?,
                Update::Scale(f) => scale(v, f.raw())?,
                Update::Clamp { min, max } => v.max(*min).min(*max),
                Update::Aggregate { source, weight } => self.aggregate(source, weight)?,
                Update::Decay {
                    toward,
                    rate,
                    cadence,
                } => {
                    let updated_at = cell.map(|c| c.updated_at.0).unwrap_or(ctx.now.0);
                    let elapsed = ctx
                        .now
                        .0
                        .checked_sub(updated_at)
                        .ok_or(arithmetic("decay_backwards"))?;
                    let target_value =
                        match cell.and_then(|c| c.baseline.as_ref()).and_then(numeric) {
                            Some(b) => b,
                            None => self.expr(toward, subject, ctx)?,
                        };
                    decay(
                        v,
                        target_value,
                        self.expr(rate, subject, ctx)?,
                        elapsed,
                        *cadence,
                    )?
                    .unwrap_or(v)
                }
            };
        }
        Ok(Some(Intent::Transform {
            family: crate::rules::TransformFamily::RuleBody,
            op_fingerprint: op_fp("rule_body"),
            resolved: v,
        }))
    }
}

/// The finalized command barrier returned by finalization.
struct Barrier {
    timeline_epoch: u64,
    ordinal: Ordinal,
    semantic_hash: Digest,
    fence_hash: Digest,
}

enum FinalizeFailure {
    Refusal(FinalizationRefusal),
    EntailmentViolated,
}

pub(crate) struct CohortExtraction {
    pub(crate) due_time: LogicalTime,
    pub(crate) executable: Vec<(DueWorkItem, ObligationRecord)>,
    pub(crate) conflicted: Vec<(WorkKeyConflict, ClaimSet)>,
}

impl Engine {
    // ------------------------------------------------------------ genesis

    pub fn genesis(genesis: EngineGenesis) -> Result<Engine, GenesisError> {
        let EngineGenesis {
            profile,
            rule_set,
            config,
            timeline,
            start_time,
            initial_work,
        } = genesis;
        if rule_set.profile_id() != profile.profile_id()
            || rule_set.manifest_content_hash() != profile.manifest_content_hash()
            || rule_set.activation_hash() != profile.activation_hash()
        {
            return Err(GenesisError::RuleSetNotBoundToProfile);
        }
        if config.profile_id() != profile.profile_id() {
            return Err(GenesisError::ConfigProfileMismatch);
        }
        if let Some(missing) = rule_set
            .config_keys()
            .into_iter()
            .find(|k| config.get(k).is_none())
        {
            return Err(GenesisError::MissingConfigKey(missing));
        }
        let profile_id = profile.profile_id().clone();
        let tl = TimelineIngress::new(
            profile_id.clone(),
            timeline.timeline_epoch,
            timeline.sequencer.clone(),
            timeline.window_width,
        )
        .map_err(GenesisError::Timeline)?;
        let mut epochs = EpochRegistry::new(profile_id.clone());
        let record = EpochRecord::new(
            profile_id.clone(),
            1,
            profile.manifest_content_hash().clone(),
            config.config_revision_hash(),
            rule_set.content_hash().clone(),
            profile.activation_hash().clone(),
            Digest::ZERO,
            ActivatingBarrier::Genesis,
        );
        epochs
            .append(record)
            .map_err(|_| GenesisError::OccurrenceExhausted)?;
        let budgets = *rule_set.budgets();
        let mut engine = Engine {
            profile_id,
            store: profile.into_state_store(),
            scheduler: Scheduler::new(),
            timeline: tl,
            grant: timeline.sequencer,
            epochs,
            obligations: ObligationStore::default(),
            occurrences: OccurrenceLedger::default(),
            cooldowns: CooldownLedger::default(),
            rule_set,
            config,
            frontier: start_time,
            active: None,
            fail_stopped: false,
            #[cfg(any(test, feature = "test-support"))]
            observation: Observation::default(),
            #[cfg(any(test, feature = "test-support"))]
            disabled_row: None,
        };
        let mut sorted = initial_work;
        sorted.sort_by(|a, b| {
            (&a.rule_id, &a.scope, a.due_time, &a.work_kind).cmp(&(
                &b.rule_id,
                &b.scope,
                b.due_time,
                &b.work_kind,
            ))
        });
        if sorted.len() > budgets.max_obligations as usize
            || sorted.len() > budgets.max_scheduled_work as usize
        {
            return Err(GenesisError::InitialWorkExceedsAdmission);
        }
        for w in sorted {
            let Some(rule) = engine.rule_set.rule(&w.rule_id) else {
                return Err(GenesisError::UnknownInitialRule(w.rule_id));
            };
            let ledger_key = OccurrenceLedgerKey {
                profile_id: engine.profile_id.clone(),
                producer: w.rule_id.clone(),
                scope_id: w.scope.clone(),
                work_kind: w.work_kind.clone(),
            };
            let occurrence = engine.occurrences.next_for(&ledger_key);
            let next = occurrence
                .checked_add(1)
                .ok_or(GenesisError::OccurrenceExhausted)?;
            let key = WorkKey {
                due_time: w.due_time,
                profile_id: engine.profile_id.clone(),
                producer_definition_id: w.rule_id.clone(),
                scope_id: w.scope.clone(),
                occurrence_index: OccurrenceIndex(occurrence),
                work_kind: w.work_kind.clone(),
            };
            let mut enc = CanonicalEncoder::new();
            enc.push_str("genesis_work_v1");
            key.canonicalize(&mut enc);
            let record = ObligationRecord {
                key: key.clone(),
                creator_rule_id: w.rule_id.clone(),
                creator_behavior_epoch: 1,
                creator_behavior_artifact_hash: engine.artifact(),
                creator_emission_identity: enc.finish(),
                mode: ObligationMode::RuleReEvaluation {
                    rule_id: w.rule_id.clone(),
                    rule_fingerprint: rule.fingerprint().clone(),
                    ruleset_content_hash: engine.rule_set.content_hash().clone(),
                },
            };
            engine.occurrences.set_next(ledger_key, next);
            engine.scheduler.schedule(DueWorkItem {
                key,
                payload: record.payload(),
            });
            engine.obligations.insert(record);
        }
        Ok(engine)
    }

    // ------------------------------------------------------------ read-only facade

    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    /// Read-only canonical state (no public write path exists on `StateStore`).
    pub fn state(&self) -> &StateStore {
        &self.store
    }

    /// `F`, the horizon frontier (read-only).
    pub fn frontier(&self) -> LogicalTime {
        self.frontier
    }

    /// `ActiveRequest` (read-only; no public setter, constructor, or clearer).
    pub fn active_request(&self) -> Option<&RequestDiscriminator> {
        self.active.as_ref()
    }

    pub fn is_fail_stopped(&self) -> bool {
        self.fail_stopped
    }

    pub fn rule_set(&self) -> &ActivatedRuleSet {
        &self.rule_set
    }

    pub fn config(&self) -> &ConfigRevision {
        &self.config
    }

    pub fn epoch_registry(&self) -> &EpochRegistry {
        &self.epochs
    }

    pub fn obligations(&self) -> &ObligationStore {
        &self.obligations
    }

    pub fn occurrences(&self) -> &OccurrenceLedger {
        &self.occurrences
    }

    pub fn cooldowns(&self) -> &CooldownLedger {
        &self.cooldowns
    }

    pub fn scheduled_work_count(&self) -> usize {
        self.scheduler.len()
    }

    pub fn slot_status(&self, key: &WorkKey) -> WorkSlotStatus {
        self.scheduler.slot_status(key)
    }

    pub fn scheduler_digest(&self) -> Digest {
        self.scheduler.canonical_state_digest()
    }

    pub fn timeline_state_digest(&self) -> Digest {
        self.timeline.canonical_state_digest()
    }

    pub fn timeline_history_digest(&self) -> Digest {
        self.timeline.canonical_history_digest()
    }

    pub fn timeline_frontier_ordinal(&self) -> Ordinal {
        self.timeline.frontier_ordinal()
    }

    pub fn timeline_epoch(&self) -> TimelineEpoch {
        self.timeline.timeline_epoch()
    }

    pub fn timeline_window_width(&self) -> u32 {
        self.timeline.window_width()
    }

    pub fn timeline_slot_status(&self, ordinal: Ordinal) -> SlotStatus {
        self.timeline.slot_status(ordinal)
    }

    pub fn finalized_commands(&self) -> &[FinalizedCommand] {
        self.timeline.finalized_commands()
    }

    pub fn epoch_resets(&self) -> &[EpochResetRecord] {
        self.timeline.epoch_resets()
    }

    /// The clean-staging invariant I-CS (Revision 2 §4.3): no slot in the
    /// current window.
    pub fn staging_is_clean(&self) -> bool {
        let Ok(window) = self.timeline.current_admission_window() else {
            return true;
        };
        let mut o = window.frontier_ordinal.0;
        loop {
            if self.timeline.slot_status(Ordinal(o)) != SlotStatus::Empty {
                return false;
            }
            if o >= window.window_end.0 {
                return true;
            }
            match o.checked_add(1) {
                Some(n) => o = n,
                None => return true,
            }
        }
    }

    /// The current behavior epoch and its `behavior_artifact_hash`.
    pub fn behavior_epoch(&self) -> u64 {
        self.epochs
            .current()
            .map(|r| r.behavior_epoch())
            .unwrap_or(0)
    }

    fn artifact(&self) -> Digest {
        self.epochs
            .current()
            .map(|r| r.record_hash())
            .unwrap_or(Digest::ZERO)
    }

    fn root_seed(&self) -> u64 {
        DefinitionId::new(ROOT_SEED_CONFIG_KEY)
            .ok()
            .and_then(|k| self.config.get(&k).and_then(numeric))
            .map(|v| v as u64)
            .unwrap_or(0)
    }

    /// `engine_state_digest` — the frozen v1 §8 composition, unchanged; it
    /// excludes `F` and `ActiveRequest`.
    pub fn engine_state_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("engine_state");
        enc.push_digest(&self.store.canonical_state_digest());
        enc.push_digest(&self.scheduler.canonical_state_digest());
        enc.push_digest(&self.timeline.canonical_state_digest());
        enc.push_digest(&self.epochs.canonical_digest());
        enc.push_digest(&self.obligations.canonical_digest());
        enc.push_digest(&self.occurrences.canonical_digest());
        enc.push_digest(&self.cooldowns.canonical_digest());
        enc.finish()
    }

    /// `stable_boundary_digest = H("stable_boundary_v1" ‖ engine_state_digest ‖
    /// F ‖ canonicalize(ActiveRequest))`.
    pub fn stable_boundary_digest(&self) -> Digest {
        stable_boundary_digest_of(
            &self.engine_state_digest(),
            self.frontier,
            self.active.as_ref(),
        )
    }

    fn view(&self) -> EvalView<'_> {
        EvalView {
            profile_id: &self.profile_id,
            store: &self.store,
            cooldowns: &self.cooldowns,
            rule_set: &self.rule_set,
            config: &self.config,
            behavior_epoch: self.behavior_epoch(),
            artifact: self.artifact(),
            root_seed: self.root_seed(),
        }
    }

    // ------------------------------------------------------------ the request boundary

    /// Processes one presented request (FINAL §9 loop P0 … A7 with Revision 2
    /// §4.8's A6). Every result is request-bound.
    pub fn process(&mut self, request: &Request) -> ProcessResult {
        let presented = request.discriminator();
        let horizon = request.horizon();
        if self.fail_stopped {
            return ProcessResult {
                presented,
                outcome: Outcome::FinalizationEntailmentViolated,
                reports: Vec::new(),
                diagnostics: None,
            };
        }
        // P0.
        match &self.active {
            Some(active) if active != &presented => {
                return ProcessResult {
                    presented,
                    outcome: Outcome::RefusedActiveRequestMismatch {
                        active: active.clone(),
                    },
                    reports: Vec::new(),
                    diagnostics: None,
                };
            }
            Some(_) => {}
            None => {
                if horizon < self.frontier {
                    return ProcessResult {
                        presented,
                        outcome: Outcome::RefusedHorizonBehindFrontier {
                            frontier: self.frontier,
                            horizon,
                        },
                        reports: Vec::new(),
                        diagnostics: None,
                    };
                }
                // Set before any cohort mutation.
                self.active = Some(presented.clone());
            }
        }
        // P1.
        let budget = u64::from(self.rule_set.budgets().max_due_per_cycle);
        let mut remaining = budget;
        let mut admitted_executable: u64 = 0;
        let mut exception_fired = false;
        let mut reports = Vec::new();
        let mut diagnostics = PacingDiagnostics {
            declared_max_due_per_cycle: self.rule_set.budgets().max_due_per_cycle,
            ..PacingDiagnostics::default()
        };
        loop {
            // A2: the outer-loop selection seam.
            #[cfg(any(test, feature = "test-support"))]
            {
                self.observation.outer_selections =
                    self.observation.outer_selections.saturating_add(1);
            }
            let executable = match self.scheduler.least_due_slice(horizon) {
                None => break,
                Some(slice) => slice.executable_count() as u64,
            };
            // A3.
            if executable > 0 {
                if exception_fired {
                    return self.pause(presented, reports, diagnostics, horizon, false);
                }
                let admit = executable <= remaining || admitted_executable == 0;
                if !admit {
                    return self.pause(presented, reports, diagnostics, horizon, false);
                }
                if executable > remaining {
                    exception_fired = true;
                    diagnostics.pacing_overrun = true;
                }
            }
            // A4.
            let extraction = match self.extract_least_due_slice(horizon) {
                Ok(e) => e,
                Err(_) => {
                    // Unreachable through the facade (the bidirectional
                    // invariant holds at every stable boundary); reachable only
                    // through test-support tampering. Nothing was mutated; the
                    // call stops with the request still active.
                    return self.pause(presented, reports, diagnostics, horizon, false);
                }
            };
            let report = self.run_scheduled_cohort(extraction);
            // A5: stable boundary.
            if executable > 0 {
                admitted_executable = admitted_executable.saturating_add(1);
                diagnostics.admitted_cohort_count = admitted_executable;
                diagnostics.admitted_work_key_count = diagnostics
                    .admitted_work_key_count
                    .saturating_add(executable);
                if let Some(id) = &report.cohort_identity {
                    diagnostics.admitted_cohort_identities.push(id.clone());
                    if exception_fired && diagnostics.overrun_cohort_identity.is_none() {
                        diagnostics.overrun_cohort_identity = Some(id.clone());
                    }
                }
                remaining = if exception_fired {
                    0
                } else {
                    remaining.saturating_sub(executable)
                };
            }
            reports.push(report);
        }
        // A6.
        let mut outcome = Outcome::Completed;
        if let Request::Command(command) = request {
            if !(remaining >= 1 || admitted_executable == 0) {
                return self.pause(presented, reports, diagnostics, horizon, true);
            }
            match self.finalize_command(command) {
                Ok(barrier) => reports.push(self.run_command_cohort(command, barrier)),
                Err(FinalizeFailure::Refusal(refusal)) => {
                    outcome = Outcome::CompletedCommandNotFinalized(refusal);
                }
                Err(FinalizeFailure::EntailmentViolated) => {
                    self.fail_stopped = true;
                    return ProcessResult {
                        presented,
                        outcome: Outcome::FinalizationEntailmentViolated,
                        reports: Vec::new(),
                        diagnostics: None,
                    };
                }
            }
        }
        self.frontier = horizon;
        self.active = None;
        self.finish_diagnostics(&mut diagnostics, horizon);
        ProcessResult {
            presented,
            outcome,
            reports,
            diagnostics: Some(diagnostics),
        }
    }

    fn finish_diagnostics(&self, diagnostics: &mut PacingDiagnostics, horizon: LogicalTime) {
        let summary = self.scheduler.due_slice_summary(horizon);
        diagnostics.deferred_cohort_count = summary.executable_slice_count;
        diagnostics.earliest_deferred_due_time = summary.earliest_executable_due_time;
    }

    fn pause(
        &self,
        presented: RequestDiscriminator,
        reports: Vec<crate::report::CohortReport>,
        mut diagnostics: PacingDiagnostics,
        horizon: LogicalTime,
        command_deferred: bool,
    ) -> ProcessResult {
        self.finish_diagnostics(&mut diagnostics, horizon);
        diagnostics.command_deferred = command_deferred;
        ProcessResult {
            presented,
            outcome: Outcome::Paused,
            reports,
            diagnostics: Some(diagnostics),
        }
    }

    // ------------------------------------------------------------ extraction (FINAL §11)

    /// The engine-owned atomic cross-store extraction: preflight read-only,
    /// then an apply that cannot fail under the exclusive borrow.
    // Refusals are cold, unreachable-through-the-facade paths carrying the
    // complete offending `WorkKey` as typed evidence; boxing would only obscure it.
    #[allow(clippy::result_large_err)]
    pub(crate) fn extract_least_due_slice(
        &mut self,
        horizon: LogicalTime,
    ) -> Result<CohortExtraction, ExtractionRefusal> {
        let (fingerprint, commitments) = match self.scheduler.least_due_slice(horizon) {
            None => {
                return Err(ExtractionRefusal::NoSliceDue {
                    least_resident_due_time: None,
                })
            }
            Some(slice) => (slice.fingerprint(), slice.slot_commitments()),
        };
        // Step 1: preflight, read-only.
        for (key, status, commitment) in &commitments {
            match self.obligations.get(key) {
                None => {
                    return Err(ExtractionRefusal::ObligationRecordMissing { key: key.clone() })
                }
                Some(set) => {
                    if &set.expected_commitment() != commitment {
                        return Err(match status {
                            WorkSlotStatus::Conflicted => {
                                ExtractionRefusal::ObligationClaimSetMismatch { key: key.clone() }
                            }
                            _ => ExtractionRefusal::ObligationRecordMismatch { key: key.clone() },
                        });
                    }
                }
            }
        }
        // Step 2: apply (X-3 first; its failure modes are no-ops before any
        // obligation removal).
        let taken = self
            .scheduler
            .take_least_due_slice(horizon, &fingerprint)
            .map_err(|e| match e {
                spark_core::scheduler::TakeSliceError::NoSliceDue {
                    least_resident_due_time,
                } => ExtractionRefusal::NoSliceDue {
                    least_resident_due_time,
                },
                spark_core::scheduler::TakeSliceError::SliceChanged { .. } => {
                    ExtractionRefusal::NoSliceDue {
                        least_resident_due_time: None,
                    }
                }
            })?;
        let mut executable = Vec::new();
        for item in taken.due {
            if let Some(set) = self.obligations.remove(&item.key) {
                if let Some(record) = set.sole_record() {
                    executable.push((item, record.clone()));
                }
            }
        }
        let mut conflicted = Vec::new();
        for c in taken.conflicted {
            if let Some(set) = self.obligations.remove(c.key()) {
                conflicted.push((c, set));
            }
        }
        Ok(CohortExtraction {
            due_time: taken.due_time,
            executable,
            conflicted,
        })
    }

    // ------------------------------------------------------------ cohorts

    fn run_scheduled_cohort(
        &mut self,
        extraction: CohortExtraction,
    ) -> crate::report::CohortReport {
        let conflicts: Vec<ConflictEntry> = extraction
            .conflicted
            .iter()
            .map(|(c, _)| ConflictEntry::from(c))
            .collect();
        if extraction.executable.is_empty() {
            return crate::report::CohortReport {
                kind: CohortKind::ConflictOnly,
                cohort_identity: None,
                canonical_time: extraction.due_time,
                conflicts,
                ingress: Vec::new(),
                waves: Vec::new(),
                outcome: CohortOutcome::ConflictOnly,
            };
        }
        let keys: Vec<WorkKey> = extraction
            .executable
            .iter()
            .map(|(i, _)| i.key.clone())
            .collect();
        let cohort = scheduled_cohort_identity(&self.profile_id, extraction.due_time, &keys);
        // R-7: resolve every record before evaluating anything.
        let mut seeds = Vec::new();
        for (item, record) in &extraction.executable {
            match record.mode() {
                ObligationMode::MaterializedEffect {
                    target_fingerprints,
                    ..
                } => {
                    for (target, fp) in target_fingerprints {
                        let current = self
                            .store
                            .schema_of(&self.profile_id, target)
                            .map(|d| d.fingerprint().clone());
                        if current.as_ref() != Some(fp) {
                            return crate::report::CohortReport {
                                kind: CohortKind::Scheduled,
                                cohort_identity: Some(cohort),
                                canonical_time: extraction.due_time,
                                conflicts,
                                ingress: Vec::new(),
                                waves: Vec::new(),
                                outcome: CohortOutcome::ObligationRefused(
                                    ObligationRefusal::TargetFingerprintDrift {
                                        key: item.key.clone(),
                                        target: target.clone(),
                                    },
                                ),
                            };
                        }
                    }
                    seeds.push(Seed::Materialized {
                        record: record.clone(),
                    });
                }
                ObligationMode::RuleReEvaluation {
                    rule_id,
                    rule_fingerprint,
                    ruleset_content_hash,
                } => {
                    let resolves = self
                        .rule_set
                        .rule(rule_id)
                        .map(|r| r.fingerprint() == rule_fingerprint)
                        .unwrap_or(false)
                        && self
                            .epochs
                            .records()
                            .iter()
                            .any(|e| e.ruleset_content_hash() == ruleset_content_hash);
                    if !resolves {
                        return crate::report::CohortReport {
                            kind: CohortKind::Scheduled,
                            cohort_identity: Some(cohort),
                            canonical_time: extraction.due_time,
                            conflicts,
                            ingress: Vec::new(),
                            waves: Vec::new(),
                            outcome: CohortOutcome::ObligationRefused(
                                ObligationRefusal::RuleResolutionFailed {
                                    key: item.key.clone(),
                                    rule_id: rule_id.clone(),
                                },
                            ),
                        };
                    }
                    seeds.push(Seed::Rule {
                        rule_id: rule_id.clone(),
                        subject: item.key.scope_id.clone(),
                        parent: ParentContext::WorkKey(item.key.identity_digest()),
                        occurrence: item.key.occurrence_index.0,
                        params: Vec::new(),
                    });
                }
            }
        }
        let (waves, outcome) = self.run_waves(&cohort, extraction.due_time, seeds);
        crate::report::CohortReport {
            kind: CohortKind::Scheduled,
            cohort_identity: Some(cohort),
            canonical_time: extraction.due_time,
            conflicts,
            ingress: Vec::new(),
            waves,
            outcome,
        }
    }

    fn run_command_cohort(
        &mut self,
        command: &CommandRequest,
        barrier: Barrier,
    ) -> crate::report::CohortReport {
        let cohort = command_cohort_identity(
            barrier.timeline_epoch,
            barrier.ordinal.0,
            &barrier.semantic_hash,
            &barrier.fence_hash,
        );
        let now = command.effective_time;
        let mut report = crate::report::CohortReport {
            kind: CohortKind::Command,
            cohort_identity: Some(cohort.clone()),
            canonical_time: now,
            conflicts: Vec::new(),
            ingress: Vec::new(),
            waves: Vec::new(),
            outcome: CohortOutcome::Committed,
        };
        match &command.payload {
            CommandPayload::ActivateEpoch { rule_set, config } => {
                report.outcome = match self.activate_epoch(rule_set, config, &barrier) {
                    Ok(epoch) => CohortOutcome::EpochActivated {
                        behavior_epoch: epoch,
                    },
                    Err(reason) => CohortOutcome::EpochActivationRejected { reason },
                };
            }
            CommandPayload::Host {
                subject,
                observations,
                params,
            } => {
                // Direct canonical ingress (ADR-0003 §14): validate every
                // observation first, then apply all.
                for o in observations {
                    if let Err(error) =
                        self.store
                            .validate_observation(&o.definition, subject, &o.value)
                    {
                        report.outcome = CohortOutcome::IngressRejected {
                            definition: o.definition.clone(),
                            error,
                        };
                        return report;
                    }
                }
                let epoch = self.behavior_epoch();
                for o in observations {
                    if self
                        .store
                        .observe_host_owned(
                            self.profile_id.clone(),
                            o.definition.clone(),
                            subject.clone(),
                            o.value.clone(),
                            now,
                            epoch,
                        )
                        .is_ok()
                    {
                        report.ingress.push(CommittedEffect {
                            definition: o.definition.clone(),
                            scope: subject.clone(),
                            write_path: WritePath::SparkEffect,
                            value: o.value.clone(),
                            provenance: Provenance {
                                retained: Vec::new(),
                                omitted_source_count: 0,
                                coverage_status: CoverageStatus::Complete,
                                pruning_policy: PROVENANCE_PRUNING_POLICY,
                            },
                        });
                    }
                }
                let seeds: Vec<Seed> = self
                    .rule_set
                    .rules_triggered_by_command(&command.command_kind)
                    .into_iter()
                    .map(|r| Seed::Rule {
                        rule_id: r.rule_id().clone(),
                        subject: subject.clone(),
                        parent: ParentContext::Command(cohort.clone()),
                        occurrence: barrier.ordinal.0,
                        params: params.clone(),
                    })
                    .collect();
                if !seeds.is_empty() {
                    let (waves, outcome) = self.run_waves(&cohort, now, seeds);
                    report.waves = waves;
                    report.outcome = outcome;
                }
            }
        }
        report
    }

    fn activate_epoch(
        &mut self,
        rule_set: &ActivatedRuleSet,
        config: &ConfigRevision,
        barrier: &Barrier,
    ) -> Result<u64, &'static str> {
        if rule_set.profile_id() != &self.profile_id
            || rule_set.manifest_content_hash() != self.store.manifest_content_hash()
            || rule_set.activation_hash() != self.store.activation_hash()
        {
            return Err("rule_set_not_bound_to_profile");
        }
        if config.profile_id() != &self.profile_id {
            return Err("config_profile_mismatch");
        }
        if rule_set
            .config_keys()
            .iter()
            .any(|k| config.get(k).is_none())
        {
            return Err("missing_config_key");
        }
        let (epoch, previous) = self.epochs.successor().map_err(|_| "epoch_exhausted")?;
        let record = EpochRecord::new(
            self.profile_id.clone(),
            epoch,
            self.store.manifest_content_hash().clone(),
            config.config_revision_hash(),
            rule_set.content_hash().clone(),
            self.store.activation_hash().clone(),
            previous,
            ActivatingBarrier::Command {
                timeline_epoch: barrier.timeline_epoch,
                input_ordinal: barrier.ordinal.0,
                semantic_hash: barrier.semantic_hash.clone(),
                fence_hash: barrier.fence_hash.clone(),
            },
        );
        self.epochs.append(record).map_err(|_| "epoch_chain")?;
        self.rule_set = rule_set.clone();
        self.config = config.clone();
        Ok(epoch)
    }

    // ------------------------------------------------------------ the wave pipeline

    fn run_waves(
        &mut self,
        cohort: &Digest,
        now: LogicalTime,
        seeds: Vec<Seed>,
    ) -> (Vec<WaveReport>, CohortOutcome) {
        let budgets = *self.rule_set.budgets();
        let mut waves = Vec::new();
        let mut pending = seeds;
        let mut wave: u32 = 0;
        loop {
            let prewave = self.engine_state_digest();
            #[cfg(any(test, feature = "test-support"))]
            {
                self.observation.prewave.push(PrewaveObservation {
                    cohort_identity: cohort.clone(),
                    wave_index: wave,
                    engine_digest: ObservedDigest(prewave.clone()),
                    scheduler_digest: ObservedDigest(self.scheduler.canonical_state_digest()),
                    active_request_set: self.active.is_some(),
                });
            }
            match self.plan_wave(cohort, now, wave, &pending, &budgets) {
                Err(rejection) => {
                    return (waves, CohortOutcome::Rejected { wave, rejection });
                }
                Ok(plan) => {
                    let next = plan.next.clone();
                    let report = self.apply_wave(cohort, now, wave, &prewave, plan);
                    let converted = !report.depth_conversions.is_empty();
                    waves.push(report);
                    if next.is_empty() || converted {
                        return (waves, CohortOutcome::Committed);
                    }
                    pending = next;
                    wave = match wave.checked_add(1) {
                        Some(w) => w,
                        None => {
                            return (
                                waves,
                                CohortOutcome::Rejected {
                                    wave,
                                    rejection: arithmetic("wave_index"),
                                },
                            )
                        }
                    };
                }
            }
        }
    }

    /// Evaluate, canonicalize, reduce, derive wave `N+1`, and preflight the
    /// complete transition — read-only.
    fn plan_wave(
        &self,
        cohort: &Digest,
        now: LogicalTime,
        wave: u32,
        pending: &[Seed],
        budgets: &crate::rules::DeclaredBudgets,
    ) -> Result<WavePlan, WaveRejection> {
        let view = self.view();
        let mut draft = Draft::default();
        for seed in pending {
            match seed {
                Seed::Rule {
                    rule_id,
                    subject,
                    parent,
                    occurrence,
                    params,
                } => {
                    if let ParentContext::Emissions(p) = parent {
                        if p.is_empty() {
                            return Err(WaveRejection::EmptyParentSet);
                        }
                    }
                    let Some(rule) = self.rule_set.rule(rule_id) else {
                        continue;
                    };
                    let ctx = EvalCtx {
                        cohort,
                        wave,
                        parent: parent.digest(),
                        now,
                        params,
                        occurrence: *occurrence,
                    };
                    view.evaluate_rule(rule, subject, &ctx, &mut draft)?;
                }
                Seed::Materialized { record } => {
                    let parent = ParentContext::WorkKey(record.key().identity_digest()).digest();
                    let mut rfp = CanonicalEncoder::new();
                    rfp.push_str("materialized_obligation_v1");
                    rfp.push_digest(&record.record_hash());
                    let record_fp = rfp.finish();
                    let ctx = EmissionContext {
                        cohort_identity: cohort,
                        wave_index: wave,
                        parent_context: &parent,
                        rule_fingerprint: &record_fp,
                        producer_scope: &record.key().scope_id,
                        behavior_artifact_hash: &view.artifact,
                    };
                    if let ObligationMode::MaterializedEffect { effects, .. } = record.mode() {
                        for e in effects {
                            let Some(def) = self.store.schema_of(&self.profile_id, &e.target)
                            else {
                                continue;
                            };
                            let Some(write_path) = write_path_of(def.authority()) else {
                                continue;
                            };
                            draft.candidates.push(Candidate {
                                definition: e.target.clone(),
                                scope: e.scope.clone(),
                                write_path,
                                identity: emission_identity(
                                    &ctx,
                                    &[&e.sub_id],
                                    &e.target,
                                    &e.scope,
                                ),
                                intent: match &e.intent {
                                    FrozenIntent::AddDelta(d) => Intent::AddDelta(*d),
                                    FrozenIntent::Result { value, family } => Intent::Result {
                                        value: *value,
                                        family: *family,
                                    },
                                },
                            });
                        }
                    }
                }
            }
        }
        // R-5: candidate volume.
        let candidate_count = draft.candidates.len() as u64;
        if candidate_count > u64::from(budgets.max_cohort_candidates) {
            return Err(WaveRejection::SemanticCap {
                cap: "max_cohort_candidates",
                observed: candidate_count,
                bound: budgets.max_cohort_candidates,
            });
        }
        let canonical = canonicalize_candidates(draft.candidates)?;
        let set_digest = candidate_set_digest(&canonical);
        let reduced = reduce(&canonical, &|d, s| view.cell(d, s))?;
        // Committed effects, validated (authority, type, bounds, scope).
        let mut committed = Vec::new();
        for r in &reduced {
            let Some(def) = self.store.schema_of(&self.profile_id, &r.definition) else {
                return Err(WaveRejection::InvalidEffect {
                    definition: r.definition.clone(),
                    scope: r.scope.clone(),
                    error: Box::new(StateWriteError::UndeclaredDefinition {
                        profile_id: self.profile_id.clone(),
                        definition_id: r.definition.clone(),
                    }),
                });
            };
            let value = match def.value_constraint().value_type() {
                ValueType::Int => CanonicalValue::Int(r.raw),
                ValueType::Fixed => CanonicalValue::Fixed(FixedPoint::from_raw(r.raw)),
                other => {
                    return Err(WaveRejection::InvalidEffect {
                        definition: r.definition.clone(),
                        scope: r.scope.clone(),
                        error: Box::new(StateWriteError::WrongValueType {
                            profile_id: self.profile_id.clone(),
                            definition_id: r.definition.clone(),
                            expected: other,
                            actual: ValueType::Int,
                        }),
                    })
                }
            };
            self.store
                .validate_effect(&r.definition, &r.scope, &value, r.write_path)
                .map_err(|error| WaveRejection::InvalidEffect {
                    definition: r.definition.clone(),
                    scope: r.scope.clone(),
                    error: Box::new(error),
                })?;
            committed.push(CommittedEffect {
                definition: r.definition.clone(),
                scope: r.scope.clone(),
                write_path: r.write_path,
                value,
                provenance: provenance_of(&r.contributing),
            });
        }
        let committed_count = committed.len() as u64;
        if committed_count > u64::from(budgets.max_effects_per_wave) {
            return Err(WaveRejection::SemanticCap {
                cap: "max_effects_per_wave",
                observed: committed_count,
                bound: budgets.max_effects_per_wave,
            });
        }
        // Threshold / propagation, only after complete reduction: previously
        // committed value -> fully reduced result.
        let mut triggers: BTreeMap<(DefinitionId, ScopeId), BTreeSet<Digest>> = BTreeMap::new();
        for r in &reduced {
            for watcher in self.rule_set.rules_watching(&r.definition) {
                let fire = match &watcher.spec().trigger {
                    Trigger::Change { .. } => true,
                    Trigger::Crossing {
                        threshold,
                        direction,
                        ..
                    } => {
                        let old = view.cell(&r.definition, &r.scope).unwrap_or(0);
                        direction.crossed(old, r.raw, *threshold)
                    }
                    _ => false,
                };
                if fire {
                    triggers
                        .entry((watcher.rule_id().clone(), r.scope.clone()))
                        .or_default()
                        .extend(r.contributing.iter().cloned());
                }
            }
        }
        let next_wave = wave.checked_add(1).ok_or(arithmetic("wave_index"))?;
        let mut next: Vec<Seed> = Vec::new();
        let mut conversions: Vec<EnqueueClaim> = Vec::new();
        for ((rule_id, subject), parents) in triggers {
            let parent = ParentContext::Emissions(parents);
            let parent_digest = parent.digest();
            if wave >= budgets.max_wave_depth {
                // Depth overflow: convert to strictly later work, never dropped
                // and never run in this cohort (v2 §11).
                let Some(rule) = self.rule_set.rule(&rule_id) else {
                    continue;
                };
                let ctx = EmissionContext {
                    cohort_identity: cohort,
                    wave_index: next_wave,
                    parent_context: &parent_digest,
                    rule_fingerprint: rule.fingerprint(),
                    producer_scope: &subject,
                    behavior_artifact_hash: &view.artifact,
                };
                let tag = DEPTH_OVERFLOW_WORK_KIND;
                let identity = emission_identity(&ctx, &[&tag], &rule_id, &subject);
                conversions.push(EnqueueClaim {
                    identity,
                    ledger_key: OccurrenceLedgerKey {
                        profile_id: self.profile_id.clone(),
                        producer: rule_id.clone(),
                        scope_id: subject.clone(),
                        work_kind: WorkKind::new(DEPTH_OVERFLOW_WORK_KIND),
                    },
                    due_time: advance_time(now, 1, "delayed_time")?,
                    creator_rule: rule_id.clone(),
                    mode: ObligationMode::RuleReEvaluation {
                        rule_id: rule_id.clone(),
                        rule_fingerprint: rule.fingerprint().clone(),
                        ruleset_content_hash: self.rule_set.content_hash().clone(),
                    },
                });
            } else {
                next.push(Seed::Rule {
                    rule_id,
                    subject,
                    occurrence: digest_u64(&parent_digest),
                    parent,
                    params: Vec::new(),
                });
            }
        }
        let converted_keys_from = draft.enqueues.len();
        let mut enqueues = draft.enqueues;
        enqueues.extend(conversions);
        let enqueue_count = enqueues.len() as u64;
        if enqueue_count > u64::from(budgets.max_enqueue_per_wave) {
            return Err(WaveRejection::SemanticCap {
                cap: "max_enqueue_per_wave",
                observed: enqueue_count,
                bound: budgets.max_enqueue_per_wave,
            });
        }
        // Occurrence allocation (v2 §9): per ledger key, ascending complete
        // emission identity, checked consecutive range.
        let mut by_key: BTreeMap<OccurrenceLedgerKey, Vec<(usize, &EnqueueClaim)>> =
            BTreeMap::new();
        for (i, c) in enqueues.iter().enumerate() {
            by_key.entry(c.ledger_key.clone()).or_default().push((i, c));
        }
        let mut records: Vec<(usize, ObligationRecord)> = Vec::new();
        let mut ledger_updates: Vec<(OccurrenceLedgerKey, u64)> = Vec::new();
        for (key, mut claims) in by_key {
            claims.sort_by(|a, b| a.1.identity.cmp(&b.1.identity));
            let start = self.occurrences.next_for(&key);
            let mut occurrence = start;
            for (index, claim) in &claims {
                let work_key = WorkKey {
                    due_time: claim.due_time,
                    profile_id: key.profile_id.clone(),
                    producer_definition_id: key.producer.clone(),
                    scope_id: key.scope_id.clone(),
                    occurrence_index: OccurrenceIndex(occurrence),
                    work_kind: key.work_kind.clone(),
                };
                records.push((
                    *index,
                    ObligationRecord {
                        key: work_key,
                        creator_rule_id: claim.creator_rule.clone(),
                        creator_behavior_epoch: view.behavior_epoch,
                        creator_behavior_artifact_hash: view.artifact.clone(),
                        creator_emission_identity: claim.identity.clone(),
                        mode: claim.mode.clone(),
                    },
                ));
                occurrence = occurrence
                    .checked_add(1)
                    .ok_or(arithmetic("occurrence_exhausted"))?;
            }
            ledger_updates.push((key, occurrence));
        }
        records.sort_by(|a, b| a.1.key.cmp(&b.1.key));
        // R-4 preflight: bidirectional invariant on touched keys, admission caps.
        let mut fresh: BTreeSet<WorkKey> = BTreeSet::new();
        for (_, r) in &records {
            if r.key.profile_id != self.profile_id {
                return Err(WaveRejection::Preflight {
                    reason: PreflightFailure::ForeignProfileWork,
                });
            }
            let status = self.scheduler.slot_status(&r.key);
            let store = self.obligations.get(&r.key);
            let consistent = match (status, store) {
                (WorkSlotStatus::Empty, None) => true,
                (WorkSlotStatus::Scheduled, Some(s)) => !s.is_contested(),
                (WorkSlotStatus::Conflicted, Some(s)) => s.is_contested(),
                _ => false,
            };
            if !consistent {
                return Err(WaveRejection::Preflight {
                    reason: PreflightFailure::BidirectionalInvariant,
                });
            }
            if status == WorkSlotStatus::Empty {
                fresh.insert(r.key.clone());
            }
        }
        let occupied = (self.scheduler.len() as u64).saturating_add(fresh.len() as u64);
        if occupied > u64::from(budgets.max_scheduled_work) {
            return Err(WaveRejection::Preflight {
                reason: PreflightFailure::SchedulerAdmissionCap {
                    occupied,
                    bound: budgets.max_scheduled_work,
                },
            });
        }
        let stored = (self.obligations.len() as u64).saturating_add(fresh.len() as u64);
        if stored > u64::from(budgets.max_obligations) {
            return Err(WaveRejection::Preflight {
                reason: PreflightFailure::ObligationAdmissionCap {
                    occupied: stored,
                    bound: budgets.max_obligations,
                },
            });
        }
        let depth_conversions: Vec<WorkKey> = records
            .iter()
            .filter(|(i, _)| *i >= converted_keys_from)
            .map(|(_, r)| r.key.clone())
            .collect();
        Ok(WavePlan {
            candidate_set_digest: set_digest,
            committed,
            records: records.into_iter().map(|(_, r)| r).collect(),
            ledger_updates,
            cooldown_writes: draft.cooldown_writes,
            cooldown_removals: draft.cooldown_removals,
            depth_conversions,
            next: if !conversion_pending(&next) {
                next
            } else {
                Vec::new()
            },
        })
    }

    /// Applies a preflighted plan. Every step is a function of values already
    /// validated against the same schema, so no step can fail.
    fn apply_wave(
        &mut self,
        cohort: &Digest,
        now: LogicalTime,
        wave: u32,
        prewave: &Digest,
        plan: WavePlan,
    ) -> WaveReport {
        let epoch = self.behavior_epoch();
        for e in &plan.committed {
            let _ = match e.write_path {
                WritePath::SparkEffect => self.store.apply_spark_effect(
                    self.profile_id.clone(),
                    e.definition.clone(),
                    e.scope.clone(),
                    e.value.clone(),
                    now,
                    epoch,
                ),
                WritePath::CommitDerived => self.store.commit_derived(
                    self.profile_id.clone(),
                    e.definition.clone(),
                    e.scope.clone(),
                    e.value.clone(),
                    now,
                    epoch,
                ),
            };
        }
        for (rule, scope) in &plan.cooldown_removals {
            self.cooldowns.remove(rule, scope);
        }
        for ((rule, scope), expiry) in &plan.cooldown_writes {
            self.cooldowns.set(rule.clone(), scope.clone(), *expiry);
        }
        for (key, next) in &plan.ledger_updates {
            self.occurrences.set_next(key.clone(), *next);
        }
        let mut obligations = Vec::new();
        for r in &plan.records {
            obligations.push((r.key.clone(), r.record_hash()));
            self.scheduler.schedule(DueWorkItem {
                key: r.key.clone(),
                payload: r.payload(),
            });
            self.obligations.insert(r.clone());
        }
        let batch = effect_batch_digest(
            &self.profile_id,
            epoch,
            cohort,
            prewave,
            wave,
            &plan.candidate_set_digest,
            &plan.committed,
        );
        WaveReport {
            wave_index: wave,
            candidate_set_digest: plan.candidate_set_digest,
            committed: plan.committed,
            obligations,
            cooldown_writes: plan
                .cooldown_writes
                .into_iter()
                .map(|((r, s), t)| (r, s, t))
                .collect(),
            cooldown_removals: plan.cooldown_removals.into_iter().collect(),
            depth_conversions: plan.depth_conversions,
            effect_batch_digest: batch,
        }
    }

    // ------------------------------------------------------------ finalization (Revision 2 §4)

    #[cfg(any(test, feature = "test-support"))]
    fn row_enabled(&self, row: PreflightRow) -> bool {
        self.disabled_row != Some(row)
    }

    #[cfg(not(any(test, feature = "test-support")))]
    fn row_enabled(&self, _row: ()) -> bool {
        true
    }

    /// Complete read-only preflight P-1 … P-9, then the entailed apply.
    fn finalize_command(&mut self, command: &CommandRequest) -> Result<Barrier, FinalizeFailure> {
        #[cfg(any(test, feature = "test-support"))]
        macro_rules! row {
            ($r:ident) => {
                self.row_enabled(PreflightRow::$r)
            };
        }
        #[cfg(not(any(test, feature = "test-support")))]
        macro_rules! row {
            ($r:ident) => {
                self.row_enabled(())
            };
        }
        let refuse = |r| Err(FinalizeFailure::Refusal(r));
        // P-1.
        if &command.profile_id != self.timeline.profile_id() {
            return refuse(FinalizationRefusal::WrongProfile);
        }
        // P-2.
        if command.timeline_epoch != self.timeline.timeline_epoch() {
            return refuse(FinalizationRefusal::WrongTimelineEpoch);
        }
        // P-3.
        if &self.grant != self.timeline.active_sequencer() {
            return refuse(FinalizationRefusal::NotActiveSequencer);
        }
        // P-4.
        let Ok(window) = self.timeline.current_admission_window() else {
            return refuse(FinalizationRefusal::OrdinalSpaceExhaustedWindow);
        };
        let n = window.frontier_ordinal;
        // P-5.
        if row!(P5) && n.0.checked_add(1).is_none() {
            return refuse(FinalizationRefusal::OrdinalSpaceExhaustedFrontierAdvance);
        }
        // P-6: I-CS over the whole window (bounded by the window width).
        if row!(P6) {
            let mut o = n.0;
            loop {
                if self.timeline.slot_status(Ordinal(o)) != SlotStatus::Empty {
                    return refuse(FinalizationRefusal::UnexpectedStagingState {
                        ordinal: Ordinal(o),
                    });
                }
                if o >= window.window_end.0 {
                    break;
                }
                match o.checked_add(1) {
                    Some(next) => o = next,
                    None => break,
                }
            }
        }
        let envelope = command.envelope_at(n);
        let hash = envelope.semantic_hash();
        // P-7 (PX-1 bounded lookup).
        if row!(P7) {
            if let Some(existing) = self.timeline.finalized_command_claim(&command.command_id) {
                if existing != &hash {
                    return refuse(FinalizationRefusal::CommandIdentityConflict {
                        command_id: command.command_id.clone(),
                    });
                }
            }
        }
        // P-8.
        if row!(P8) {
            if let Some(existing) = self
                .timeline
                .finalized_source_sequence_claim(&command.source_id, command.source_sequence)
            {
                if existing != &hash {
                    return refuse(FinalizationRefusal::SourceSequenceConflict {
                        source_id: command.source_id.clone(),
                        source_sequence: command.source_sequence,
                    });
                }
            }
        }
        // P-9.
        if row!(P9) {
            if let Some(previous) = self
                .timeline
                .last_finalized_source_sequence(&command.source_id)
            {
                if command.source_sequence <= previous {
                    return refuse(FinalizationRefusal::SourceSequenceNotIncreasing {
                        source_id: command.source_id.clone(),
                        previously_finalized: previous,
                        attempted: command.source_sequence,
                    });
                }
            }
        }
        // A-1 … A-4: entailed apply. Any non-entailed result is a missing
        // preflight row: sticky fail-stop, never an ordinary refusal (D-8).
        let ticket = window.ticket();
        let staged = self
            .timeline
            .stage(&self.grant, &envelope.clone().submit_with(ticket));
        let ack_ok = match &staged {
            Ok(d) => match d.acknowledgement() {
                Some(ack) => {
                    ack.slot_state() == AcknowledgedSlotState::NewlyStaged && ack.covers(&envelope)
                }
                None => false,
            },
            Err(_) => false,
        };
        if !ack_ok {
            return Err(FinalizeFailure::EntailmentViolated);
        }
        #[cfg(any(test, feature = "test-support"))]
        self.observation
            .finalize_ops
            .push(FinalizeOp::PositiveStagedAcknowledgement { ordinal: n });
        let fence_name = format!("fence.{}", n.0);
        let Ok(fence_id) = FenceId::new(fence_name.clone()) else {
            return Err(FinalizeFailure::EntailmentViolated);
        };
        let fence = TimelineFence {
            profile_id: self.timeline.profile_id().clone(),
            timeline_epoch: self.timeline.timeline_epoch(),
            fence_id,
            start_ordinal: n,
            end_ordinal: n,
            previous_fence_hash: self.timeline.last_finalized_fence_hash().clone(),
            ordered_stream_digest: compute_ordered_stream_digest(&[(n, hash.clone())]),
        };
        match self.timeline.submit_fence(&self.grant, &fence) {
            Ok(result)
                if result.start_ordinal() == n
                    && result.end_ordinal() == n
                    && result.finalized_command_count() == 1 =>
            {
                #[cfg(any(test, feature = "test-support"))]
                self.observation
                    .finalize_ops
                    .push(FinalizeOp::OneOrdinalFence {
                        ordinal: n,
                        fence_id: fence_name,
                    });
                Ok(Barrier {
                    timeline_epoch: self.timeline.timeline_epoch().0,
                    ordinal: n,
                    semantic_hash: hash,
                    fence_hash: result.fence_hash().clone(),
                })
            }
            _ => Err(FinalizeFailure::EntailmentViolated),
        }
    }

    // ------------------------------------------------------------ host operations

    /// ADR-0003 §11 authorized epoch handoff/reset, at a stable boundary. It
    /// changes neither `F` nor `ActiveRequest`.
    pub fn reset_timeline_epoch(
        &mut self,
        new_epoch: TimelineEpoch,
        new_sequencer: SourceId,
    ) -> Result<EpochResetRecord, ResetRefusal> {
        if self.fail_stopped {
            return Err(ResetRefusal::FailStopped);
        }
        let record = self
            .timeline
            .reset_epoch(&self.grant, new_epoch, new_sequencer.clone())
            .map_err(ResetRefusal::Timeline)?;
        self.grant = new_sequencer;
        Ok(record)
    }

    /// A committed snapshot of the current stable boundary. Refused from a
    /// fail-stopped instance.
    pub fn snapshot(&self) -> Result<EngineSnapshot, SnapshotRefused> {
        if self.fail_stopped {
            return Err(SnapshotRefused::FailStopped);
        }
        Ok(EngineSnapshot {
            profile_id: self.profile_id.clone(),
            store: self.store.clone(),
            scheduler: self.scheduler.clone(),
            timeline: self.timeline.clone(),
            grant: self.grant.clone(),
            epochs: self.epochs.clone(),
            obligations: self.obligations.clone(),
            occurrences: self.occurrences.clone(),
            cooldowns: self.cooldowns.clone(),
            rule_set: self.rule_set.clone(),
            config: self.config.clone(),
            frontier: self.frontier,
            active: self.active.clone(),
            recorded_stable_boundary_digest: self.stable_boundary_digest(),
        })
    }

    /// Restore with validation, in order, before anything becomes live (FINAL
    /// §6.5 as amended by Revision 2 §7). Derived indexes are validated, not
    /// trusted.
    pub fn restore(
        snapshot: EngineSnapshot,
        profile: &ActivatedProfile,
    ) -> Result<Engine, RestoreError> {
        let s = snapshot;
        // Step 1: definition lineage and artifact binding.
        if s.profile_id != *profile.profile_id()
            || s.store.activation_hash() != profile.activation_hash()
            || s.store.manifest_content_hash() != profile.manifest_content_hash()
            || s.rule_set.activation_hash() != profile.activation_hash()
            || s.config.profile_id() != profile.profile_id()
        {
            return Err(RestoreError::ArtifactBindingMismatch);
        }
        if !s.epochs.chain_is_valid() {
            return Err(RestoreError::EpochChainInvalid);
        }
        match s.epochs.current() {
            Some(r)
                if r.ruleset_content_hash() == s.rule_set.content_hash()
                    && r.config_revision_hash() == &s.config.config_revision_hash() => {}
            _ => return Err(RestoreError::ArtifactBindingMismatch),
        }
        // Step 2.
        if let Some(a) = &s.active {
            if a.horizon() < s.frontier {
                return Err(RestoreError::ActiveBehindFrontier);
            }
        }
        let engine = Engine {
            profile_id: s.profile_id,
            store: s.store,
            scheduler: s.scheduler,
            timeline: s.timeline,
            grant: s.grant,
            epochs: s.epochs,
            obligations: s.obligations,
            occurrences: s.occurrences,
            cooldowns: s.cooldowns,
            rule_set: s.rule_set,
            config: s.config,
            frontier: s.frontier,
            active: s.active,
            fail_stopped: false,
            #[cfg(any(test, feature = "test-support"))]
            observation: Observation::default(),
            #[cfg(any(test, feature = "test-support"))]
            disabled_row: None,
        };
        // Step 2b.
        if !engine.staging_is_clean() {
            return Err(RestoreError::TimelineStagingPresent);
        }
        if !engine.timeline.derived_indexes_consistent()
            || engine.timeline.active_sequencer() != &engine.grant
        {
            return Err(RestoreError::DerivedIndexesInconsistent);
        }
        if !engine.bidirectional_invariant_holds() {
            return Err(RestoreError::BidirectionalInvariantBroken);
        }
        // Step 3.
        if engine.stable_boundary_digest() != s.recorded_stable_boundary_digest {
            return Err(RestoreError::DigestMismatch);
        }
        Ok(engine)
    }

    /// The v1 §8 scheduler ↔ `ObligationStore` bidirectional invariant, checked
    /// exhaustively (restore validation and tests; `O(n log n)`).
    pub fn bidirectional_invariant_holds(&self) -> bool {
        let mut probe = self.scheduler.clone();
        let mut seen: u64 = 0;
        loop {
            let (fp, commitments) = match probe.least_due_slice(LogicalTime(u64::MAX)) {
                None => break,
                Some(slice) => (slice.fingerprint(), slice.slot_commitments()),
            };
            for (key, _, commitment) in &commitments {
                match self.obligations.get(key) {
                    Some(set) if &set.expected_commitment() == commitment => {}
                    _ => return false,
                }
                seen = seen.saturating_add(1);
            }
            if probe
                .take_least_due_slice(LogicalTime(u64::MAX), &fp)
                .is_err()
            {
                return false;
            }
        }
        seen == self.obligations.len() as u64
    }

    /// History-only reconstruction of a **completed** boundary (Revision 2
    /// §6.1), under every declared input: genesis, timeline metadata and every
    /// epoch-reset record (same-frontier records applied in ascending
    /// `reset_index` — acceptance pin 1), each finalized command with its
    /// payload, and `F`. Verifies the recomputed digests.
    pub fn reconstruct_completed(
        genesis: EngineGenesis,
        history: &CompletedHistory,
    ) -> Result<Engine, ReplayError> {
        let mut engine = Engine::genesis(genesis).map_err(ReplayError::Genesis)?;
        let mut resets: Vec<EpochResetRecord> = history.resets.clone();
        resets.sort_by_key(|r| r.reset_index);
        let mut resets = resets.into_iter().peekable();
        let drive = |engine: &mut Engine, request: &Request| -> Result<Outcome, ReplayError> {
            loop {
                let result = engine.process(request);
                match result.outcome {
                    Outcome::Paused => continue,
                    other => return Ok(other),
                }
            }
        };
        for (index, command) in history.commands.iter().enumerate() {
            let ordinal = engine.timeline.frontier_ordinal();
            while let Some(r) = resets.peek() {
                if r.frozen_finalized_frontier != ordinal {
                    break;
                }
                let Some(r) = resets.next() else { break };
                engine
                    .reset_timeline_epoch(r.new_epoch, r.new_sequencer.clone())
                    .map_err(|_| ReplayError::StepRefused {
                        step: format!("reset {}", r.reset_index),
                    })?;
            }
            drive(&mut engine, &Request::Advance(command.effective_time))?;
            match drive(&mut engine, &Request::Command(command.clone()))? {
                Outcome::Completed => {}
                _ => {
                    return Err(ReplayError::StepRefused {
                        step: format!("command {index}"),
                    })
                }
            }
        }
        for r in resets {
            engine
                .reset_timeline_epoch(r.new_epoch, r.new_sequencer.clone())
                .map_err(|_| ReplayError::StepRefused {
                    step: format!("reset {}", r.reset_index),
                })?;
        }
        drive(&mut engine, &Request::Advance(history.frontier))?;
        if engine.timeline.canonical_history_digest() != history.recorded_history_digest
            || engine.stable_boundary_digest() != history.recorded_stable_boundary_digest
        {
            return Err(ReplayError::Diverged);
        }
        Ok(engine)
    }
}

fn conversion_pending(_next: &[Seed]) -> bool {
    false
}

#[derive(Debug)]
struct WavePlan {
    candidate_set_digest: Digest,
    committed: Vec<CommittedEffect>,
    records: Vec<ObligationRecord>,
    ledger_updates: Vec<(OccurrenceLedgerKey, u64)>,
    cooldown_writes: BTreeMap<(DefinitionId, ScopeId), LogicalTime>,
    cooldown_removals: BTreeSet<(DefinitionId, ScopeId)>,
    depth_conversions: Vec<WorkKey>,
    next: Vec<Seed>,
}

/// `stable_boundary_digest` composition, shared by the engine and snapshots.
pub fn stable_boundary_digest_of(
    engine_state_digest: &Digest,
    frontier: LogicalTime,
    active: Option<&RequestDiscriminator>,
) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("stable_boundary_v1");
    enc.push_digest(engine_state_digest);
    frontier.canonicalize(&mut enc);
    canonicalize_active_request(active, &mut enc);
    enc.finish()
}

/// `effect_batch_v3` (v3 §4.5).
pub fn effect_batch_digest(
    profile: &ProfileId,
    behavior_epoch: u64,
    cohort_identity: &Digest,
    prewave_engine_digest: &Digest,
    wave_index: u32,
    candidate_set_digest: &Digest,
    committed: &[CommittedEffect],
) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("effect_batch_v3");
    profile.canonicalize(&mut enc);
    enc.push_u64(behavior_epoch);
    enc.push_digest(cohort_identity);
    enc.push_digest(prewave_engine_digest);
    enc.push_u32(wave_index);
    enc.push_digest(candidate_set_digest);
    enc.push_u64(committed.len() as u64);
    for e in committed {
        let mut inner = CanonicalEncoder::new();
        e.definition.canonicalize(&mut inner);
        e.scope.canonicalize(&mut inner);
        inner.push_str(e.write_path.tag());
        e.value.canonicalize(&mut inner);
        inner.push_u64(e.provenance.retained.len() as u64);
        for p in &e.provenance.retained {
            inner.push_digest(p);
        }
        inner.push_u64(e.provenance.omitted_source_count);
        enc.push_block(&inner);
    }
    enc.finish()
}

// Silence the unused-import lint for kinds used only in type positions in some
// feature combinations.
#[allow(dead_code)]
fn _kind_anchor(_k: CommandKind) {}
