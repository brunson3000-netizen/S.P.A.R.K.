//! Returned presentation values (freeze v1 §3.5 chronicle boundary).
//!
//! Two structurally distinct channels (v3 §3.4): the **canonical semantic
//! report** ([`CohortReport`]), which is bit-identical between paced and
//! unbudgeted execution over one canonical input history, and the
//! **noncanonical** [`PacingDiagnostics`], which truthfully differs, appears in
//! no digest, and is never given to evaluation. No report field carries a
//! pre-wave digest, a `cohort_identity`, `F`, or `ActiveRequest` (FINAL
//! AT-I46(a), which controls over the earlier v3 §3.4 content list); cohort
//! identity remains bound into every emission identity and every
//! `effect_batch_digest` the report does carry, and is observed directly only
//! through `test-support`.

use crate::obligation::ObligationRecord;
use crate::rules::{ResultFamily, TransformFamily};
use crate::state::StateWriteError;
use spark_core::clock::LogicalTime;
use spark_core::hash::Digest;
use spark_core::id::DefinitionId;
use spark_core::scheduler::{WorkKey, WorkKeyConflict};
use spark_core::scope::ScopeId;
use spark_core::value::CanonicalValue;
use std::collections::BTreeSet;

/// Noncanonical, causally inert pacing telemetry: exactly the v3 §3.4 frozen
/// field list. A deferred command is not a separate field: a command request
/// that returns `Paused` with `deferred_cohort_count == 0` has its command
/// deferred (FINAL/Revision-2 A6), which is derivable from the frozen fields.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PacingDiagnostics {
    pub declared_max_due_per_cycle: u32,
    pub admitted_cohort_count: u64,
    pub admitted_work_key_count: u64,
    pub admitted_cohort_identities: Vec<Digest>,
    pub pacing_overrun: bool,
    pub overrun_cohort_identity: Option<Digest>,
    pub deferred_cohort_count: u64,
    pub earliest_deferred_due_time: Option<LogicalTime>,
}

/// ADR-0006 coverage status for bounded provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageStatus {
    Complete,
    Truncated,
    /// Reserved for inherited prior-state provenance; never used for a
    /// within-wave truncation.
    Unknown,
}

/// The deterministic pruning policy identity (v2 §10).
pub const PROVENANCE_PRUNING_POLICY: &str = "provenance_prune.v1";

/// The bounded provenance block (v2 §10). Truncation bounds explanation only:
/// the fold always sums every candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    /// The `MAX_SOURCE_REFS` smallest contributing emission identities,
    /// ascending.
    pub retained: Vec<Digest>,
    pub omitted_source_count: u64,
    pub coverage_status: CoverageStatus,
    pub pruning_policy: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WritePath {
    SparkEffect,
    CommitDerived,
}

impl WritePath {
    pub fn tag(&self) -> &'static str {
        match self {
            WritePath::SparkEffect => "spark_effect",
            WritePath::CommitDerived => "commit_derived",
        }
    }
}

/// One committed resolved effect: the single absolute value per target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommittedEffect {
    pub definition: DefinitionId,
    pub scope: ScopeId,
    pub write_path: WritePath,
    pub value: CanonicalValue,
    pub provenance: Provenance,
}

/// A conflicted key reported in Phase-1 conflicted-drain shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictEntry {
    pub key: WorkKey,
    pub competing_payload_hashes: BTreeSet<Digest>,
    pub omitted_distinct: u64,
    pub evidence_truncated: bool,
}

impl From<&WorkKeyConflict> for ConflictEntry {
    fn from(c: &WorkKeyConflict) -> Self {
        ConflictEntry {
            key: c.key().clone(),
            competing_payload_hashes: c.competing_payload_hashes().clone(),
            omitted_distinct: c.omitted_distinct(),
            evidence_truncated: c.evidence_truncated(),
        }
    }
}

/// Why a wave was rejected atomically (R-1 … R-5). Evidence is a function of
/// the canonical candidate set and pre-wave state, never of enumeration order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaveRejection {
    /// R-1: one emission identity carried distinct payloads.
    ContestedEmission { identity: Digest },
    /// R-1: unequal same-family RESULT values (every distinct value).
    ResultConflict {
        definition: DefinitionId,
        scope: ScopeId,
        family: ResultFamily,
        values: BTreeSet<i64>,
    },
    /// R-1: a cross-family group, or RESULT values of different families.
    FamilyMixture {
        definition: DefinitionId,
        scope: ScopeId,
    },
    /// R-1: two or more TRANSFORM candidates on one target.
    MultipleTransforms {
        definition: DefinitionId,
        scope: ScopeId,
        families: BTreeSet<TransformFamily>,
    },
    /// R-2: authority, type, bounds, or scope-invalid committed effect.
    InvalidEffect {
        definition: DefinitionId,
        scope: ScopeId,
        error: Box<StateWriteError>,
    },
    /// R-3: checked arithmetic failed (fold, domain conversion, occurrence
    /// exhaustion, delayed-time overflow, cooldown expiry).
    Arithmetic { what: &'static str },
    /// R-4: complete-transition preflight failure.
    Preflight { reason: PreflightFailure },
    /// R-5: semantic cap exceeded (the cohort's keys were consumed at
    /// extraction — the typed overload report).
    SemanticCap {
        cap: &'static str,
        observed: u64,
        bound: u32,
    },
    /// A derived (wave >= 1) emission with an empty parent set: an evaluator
    /// defect, never a degenerate identity (v3 §4.3 rule 3).
    EmptyParentSet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreflightFailure {
    ObligationAdmissionCap { occupied: u64, bound: u32 },
    SchedulerAdmissionCap { occupied: u64, bound: u32 },
    ForeignProfileWork,
    BidirectionalInvariant,
}

/// Obligation execution refusal (R-7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObligationRefusal {
    /// A `MaterializedEffect` target's current fingerprint differs (or the
    /// definition no longer exists).
    TargetFingerprintDrift { key: WorkKey, target: DefinitionId },
    /// A `RuleReEvaluation` record's exact originating artifact did not
    /// resolve in the retained activation lineage: no activated rule set with
    /// the recorded `ruleset_content_hash`, no rule with the recorded
    /// fingerprint inside it, or no epoch record equal to the creator artifact.
    RuleResolutionFailed { key: WorkKey, rule_id: DefinitionId },
    /// The originating rule resolved, but the current behavior epoch no
    /// longer carries a bit-identical rule under that ID: executing it would
    /// run superseded behavior, so it refuses explicitly (ADR-0006; no
    /// reinterpretation, no migration in Phase 2).
    RuleSuperseded { key: WorkKey, rule_id: DefinitionId },
}

/// The canonical report of one wave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaveReport {
    pub wave_index: u32,
    pub candidate_set_digest: Digest,
    /// Committed effects in canonical `(definition, scope, write path)` order.
    pub committed: Vec<CommittedEffect>,
    /// Obligations created, ascending by `WorkKey`, with record hashes.
    pub obligations: Vec<(WorkKey, Digest)>,
    pub cooldown_writes: Vec<(DefinitionId, ScopeId, LogicalTime)>,
    pub cooldown_removals: Vec<(DefinitionId, ScopeId)>,
    /// Depth-overflow conversions to strictly later work (v2 §11).
    pub depth_conversions: Vec<WorkKey>,
    pub effect_batch_digest: Digest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CohortKind {
    Scheduled,
    Command,
    /// An all-conflicted slice: extracted and reported, not a cohort.
    ConflictOnly,
}

/// How a cohort ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CohortOutcome {
    Committed,
    /// Wave `wave` rejected; waves before it stay committed (V2-06).
    Rejected {
        wave: u32,
        rejection: WaveRejection,
    },
    ObligationRefused(ObligationRefusal),
    /// A command's host observations were invalid; nothing applied.
    IngressRejected {
        definition: DefinitionId,
        error: StateWriteError,
    },
    EpochActivated {
        behavior_epoch: u64,
    },
    EpochActivationRejected {
        reason: &'static str,
    },
    ConflictOnly,
}

/// The canonical semantic report of one cohort (or all-conflicted slice).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CohortReport {
    pub kind: CohortKind,
    pub canonical_time: LogicalTime,
    /// The conflict section, ascending by `WorkKey`, reported before any
    /// cohort outcome (V2-10).
    pub conflicts: Vec<ConflictEntry>,
    /// Command ingress observations applied before wave 0.
    pub ingress: Vec<CommittedEffect>,
    pub waves: Vec<WaveReport>,
    pub outcome: CohortOutcome,
}

/// Obligation records removed by an extraction (for the canonical report and
/// the bidirectional-invariant tests).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovedClaims {
    pub key: WorkKey,
    pub records: Vec<ObligationRecord>,
}
