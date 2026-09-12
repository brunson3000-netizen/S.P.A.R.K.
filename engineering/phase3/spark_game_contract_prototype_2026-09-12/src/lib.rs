//! **Contained, reversible prototype of the S.P.A.R.K.–G.A.M.E. Integration
//! Contract V1.**
//!
//! Status: working/non-production material permitted by
//! `SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` §6 before Phase 3 is opened. It is
//! not the S.P.A.R.K. device, not a G.A.M.E. adapter, and not production code.
//! It exists to make the contract's logical surfaces executable and testable so
//! the contract can be reviewed against behavior rather than prose.
//!
//! Two properties are deliberate:
//!
//! * **No engine change.** Every canonical value used here is read from the
//!   public surface of `spark-core`/`spark-engine` at the pinned crate tree
//!   `7907f4d729104fd5dbfd4adad46e66cf09aa13dd`. Nothing under `crates/` is
//!   modified, because Gate C2 is not accepted.
//! * **No transport, no serialization.** The contract's logical semantics are
//!   defined over canonical content only, so this prototype carries Rust values
//!   in memory. Choosing a wire format or an embedded-versus-service form is
//!   explicitly reserved (contract §10.2) and nothing here presumes either.

#![forbid(unsafe_code)]

pub mod fake_host;
pub mod fixture;

use std::collections::BTreeMap;

use spark_core::clock::LogicalTime;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::DefinitionId;
use spark_core::scope::ScopeId;
use spark_core::value::CanonicalValue;
use spark_engine::activation::ActivatedProfile;
use spark_engine::engine::Engine;
use spark_engine::profile::definition::BehavioralLeverage;
use spark_engine::report::{CohortOutcome, CohortReport, Provenance};
use spark_engine::request::{FinalizationRefusal, Outcome, ProcessResult, Request};

// ===================================================================== §3.1

/// Contract §3.1. This prototype speaks `(1, 0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
}

pub const CONTRACT_VERSION: ProtocolVersion = ProtocolVersion { major: 1, minor: 0 };

/// Contract §3.1 minor negotiation: a higher host minor is accepted at the
/// device's own minor; a lower host minor holds the device down to the host's.
/// Written over two parameters rather than inline against the constant so the
/// rule stays meaningful when the device's minor is no longer zero.
pub fn negotiate_minor(host_minor: u16, device_minor: u16) -> u16 {
    host_minor.min(device_minor)
}

// ===================================================================== §3.3

/// Contract §3.3. A capability is a named, versioned surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Capability {
    ObserveV1,
    AdvanceV1,
    IntentV1,
    ReplayV1,
    SnapshotV1,
    EpochV1,
}

impl Capability {
    pub fn tag(self) -> &'static str {
        match self {
            Capability::ObserveV1 => "observe.v1",
            Capability::AdvanceV1 => "advance.v1",
            Capability::IntentV1 => "intent.v1",
            Capability::ReplayV1 => "replay.v1",
            Capability::SnapshotV1 => "snapshot.v1",
            Capability::EpochV1 => "epoch.v1",
        }
    }
}

/// Everything the device advertises when a session opens (contract §3).
///
/// `persist.v1` is intentionally absent and unrepresentable: the engine's
/// snapshot is an in-memory value with no persistence backend, so advertising
/// durability would be a false capability claim (contract §9.4).
#[derive(Debug, Clone)]
pub struct SessionDescriptor {
    pub negotiated_version: ProtocolVersion,
    pub manifest_content_hash: Digest,
    pub activation_hash: Digest,
    pub capabilities: Vec<Capability>,
    pub bounds: DeclaredBounds,
}

/// Contract §9.1. Every list in the seam is bounded and the bound is declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclaredBounds {
    pub max_observations_per_command: u32,
    pub max_intents_per_batch: u32,
}

impl Default for DeclaredBounds {
    fn default() -> Self {
        Self {
            max_observations_per_command: 64,
            max_intents_per_batch: 256,
        }
    }
}

// ===================================================================== §4.1

/// G.A.M.E.'s stable external entity reference. G.A.M.E. owns identity; this
/// is an opaque token to S.P.A.R.K.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExternalEntityRef(pub String);

/// G.A.M.E.'s stable external **affiliation** reference — a settlement, region,
/// faction or other grouping. Gate C3 enumerates "stable external entity **and
/// affiliation** references" as two surfaces, not one, so this is a distinct
/// type rather than an entity reference used loosely.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExternalAffiliationRef(pub String);

/// What an advisory intent is addressed to. An intent-channel effect committed
/// at an affiliation-kind scope addresses the affiliation, not an actor.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntentSubject {
    Entity(ExternalEntityRef),
    Affiliation(ExternalAffiliationRef),
}

/// The host-side mapping of contract §4.1, with both required properties
/// checked rather than assumed: **injective** and **stable**.
#[derive(Debug, Clone, Default)]
pub struct EntityMap {
    forward: BTreeMap<ExternalEntityRef, ScopeId>,
    reverse: BTreeMap<ScopeId, ExternalEntityRef>,
    affiliations: BTreeMap<ExternalAffiliationRef, ScopeId>,
    affiliations_reverse: BTreeMap<ScopeId, ExternalAffiliationRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingError {
    /// Two distinct external references would share one `ScopeId`. Accepting
    /// this silently merges two entities' causal state.
    NotInjective { existing: ExternalEntityRef },
    /// A reference's `ScopeId` would change. Committed cells, obligations and
    /// scheduler keys are addressed by it, so this is a new engine instance,
    /// never an edit.
    NotStable { existing: ScopeId },
}

impl EntityMap {
    pub fn bind(
        &mut self,
        external: ExternalEntityRef,
        scope: ScopeId,
    ) -> Result<(), MappingError> {
        if let Some(existing) = self.forward.get(&external) {
            if existing != &scope {
                return Err(MappingError::NotStable {
                    existing: existing.clone(),
                });
            }
            return Ok(());
        }
        if let Some(existing) = self.reverse.get(&scope) {
            return Err(MappingError::NotInjective {
                existing: existing.clone(),
            });
        }
        self.forward.insert(external.clone(), scope.clone());
        self.reverse.insert(scope, external);
        Ok(())
    }

    pub fn scope_of(&self, external: &ExternalEntityRef) -> Option<&ScopeId> {
        self.forward.get(external)
    }

    pub fn external_of(&self, scope: &ScopeId) -> Option<&ExternalEntityRef> {
        self.reverse.get(scope)
    }

    /// Affiliation binding, under the same two hard requirements as §4.1's
    /// entity binding: injective, and stable for the life of the engine.
    pub fn bind_affiliation(
        &mut self,
        external: ExternalAffiliationRef,
        scope: ScopeId,
    ) -> Result<(), AffiliationMappingError> {
        if let Some(existing) = self.affiliations.get(&external) {
            if existing != &scope {
                return Err(AffiliationMappingError::NotStable {
                    existing: existing.clone(),
                });
            }
            return Ok(());
        }
        if let Some(existing) = self.affiliations_reverse.get(&scope) {
            return Err(AffiliationMappingError::NotInjective {
                existing: existing.clone(),
            });
        }
        self.affiliations.insert(external.clone(), scope.clone());
        self.affiliations_reverse.insert(scope, external);
        Ok(())
    }

    pub fn affiliation_scope_of(&self, external: &ExternalAffiliationRef) -> Option<&ScopeId> {
        self.affiliations.get(external)
    }

    pub fn affiliation_of(&self, scope: &ScopeId) -> Option<&ExternalAffiliationRef> {
        self.affiliations_reverse.get(scope)
    }

    /// The subject an intent-channel effect at `scope` is addressed to:
    /// an affiliation if the scope is bound as one, otherwise an entity.
    pub fn subject_of(&self, scope: &ScopeId) -> Option<IntentSubject> {
        if let Some(a) = self.affiliation_of(scope) {
            return Some(IntentSubject::Affiliation(a.clone()));
        }
        self.external_of(scope)
            .map(|e| IntentSubject::Entity(e.clone()))
    }

    /// The canonical scope behind any intent subject.
    pub fn scope_of_subject(&self, subject: &IntentSubject) -> Option<&ScopeId> {
        match subject {
            IntentSubject::Entity(e) => self.scope_of(e),
            IntentSubject::Affiliation(a) => self.affiliation_scope_of(a),
        }
    }
}

/// The affiliation analogue of [`MappingError`], kept distinct so a diagnostic
/// never confuses an entity collision with an affiliation collision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AffiliationMappingError {
    NotInjective { existing: ExternalAffiliationRef },
    NotStable { existing: ScopeId },
}

// ===================================================================== §6

/// Contract §6.2. One advisory intent, projected from canonical report content.
/// It is never an executable G.A.M.E. command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorIntent {
    pub subject: IntentSubject,
    pub channel: DefinitionId,
    pub value: CanonicalValue,
    pub leverage: Option<BehavioralLeverage>,
    pub canonical_time: LogicalTime,
    pub provenance: Provenance,
}

/// Contract §6.2 / §7.2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntentBatch {
    /// The session's content-addressed profile activation identity (§3.2). It
    /// is the first component of the at-most-once key, so that two engines
    /// activated from different profile content can never share one.
    pub session_activation: Digest,
    pub correlation: Digest,
    pub horizon: LogicalTime,
    pub intents: Vec<BehaviorIntent>,
    pub batch_digest: Digest,
}

/// Contract §7.2. A function of canonical content alone, computed with the
/// engine's own encoder, so it is independent of any wire encoding.
pub fn batch_digest(
    correlation: &Digest,
    horizon: LogicalTime,
    intents: &[BehaviorIntent],
    map: &EntityMap,
) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("spark.intent_batch.v1");
    enc.push_digest(correlation);
    horizon.canonicalize(&mut enc);
    enc.push_u64(intents.len() as u64);
    for i in intents {
        let mut inner = CanonicalEncoder::new();
        // The canonical scope, not the host-side token: the digest must be a
        // function of S.P.A.R.K. canonical content.
        match map.scope_of_subject(&i.subject) {
            Some(s) => s.canonicalize(&mut inner),
            None => {
                inner.push_str("scope.unmapped");
            }
        }
        i.channel.canonicalize(&mut inner);
        i.value.canonicalize(&mut inner);
        i.canonical_time.canonicalize(&mut inner);
        enc.push_block(&inner);
    }
    enc.finish()
}

// ===================================================================== §8

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectionReason {
    IllegalAction,
    UnknownSubject,
    InsufficientResource,
    AuthorityRefused,
    UnsupportedChannel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeferReason {
    WorldBusy,
    AwaitingPrecondition,
    RateLimited,
}

/// Contract §8. What G.A.M.E. actually did with one advisory intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntentDisposition {
    /// The host executed it. `confirmations` are typed, scoped observations of
    /// `HostOwned` definitions only — the sole re-entry path (contract §8).
    Executed {
        confirmations: Vec<(DefinitionId, ScopeId, i64)>,
    },
    Rejected {
        reason: RejectionReason,
    },
    Deferred {
        reason: DeferReason,
        not_before: Option<LogicalTime>,
    },
}

/// Contract §8. Exactly one report answers one batch; `per_intent` has the
/// batch's length and order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutcomeReport {
    pub correlation: Digest,
    pub batch_digest: Digest,
    pub per_intent: Vec<IntentDisposition>,
}

// ===================================================================== §9.5

/// Contract §9.5. Every seam-level refusal is typed and fails closed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejected {
    UnsupportedProtocolVersion {
        supported_major: u16,
    },
    ProfileMismatch,
    StaleHorizon {
        frontier: LogicalTime,
        horizon: LogicalTime,
    },
    /// Contract §4.4: a *different* request while one is active.
    Busy {
        active_horizon: LogicalTime,
        active_identity: Digest,
    },
    MessageTooLarge {
        limit: u32,
        observed: u32,
    },
    /// Contract §9.5: sticky fail-stop. No stable boundary is published and the
    /// only exit is restoring a prior snapshot.
    FailStopped,
}

/// One cohort-level refusal, surfaced separately from the request-level
/// outcome (contract §9.5, "two-level reporting is part of the contract").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CohortRefusal {
    pub canonical_time: LogicalTime,
    pub outcome: CohortOutcome,
}

/// The device's answer to one presented request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceResponse {
    /// The request boundary completed. The batch is publishable; `refusals`
    /// carries every cohort that was refused inside that same boundary, which
    /// an adapter must read (contract §9.5).
    ///
    /// `capacity_exceeded` reports that the batch is larger than the host's
    /// declared `max_intents_per_batch`. The batch is still delivered whole:
    /// the work is already committed and silently dropping an intent would be
    /// far worse than an oversized message. The bound is a capacity hint the
    /// host must size up, not a licence to truncate (contract §9.1).
    Completed {
        batch: IntentBatch,
        refusals: Vec<CohortRefusal>,
        capacity_exceeded: bool,
    },
    /// The horizon completed but the command itself was not finalized (D-2).
    /// Cohorts committed inside that boundary are real work and are published;
    /// the typed engine refusal is carried alongside, never flattened away.
    CompletedWithRefusedCommand {
        batch: IntentBatch,
        refusals: Vec<CohortRefusal>,
        refusal: FinalizationRefusal,
    },
    /// Budget exhausted. Nothing is publishable; re-present the same request.
    Paused,
    Rejected(Rejected),
}

// ===================================================================== device

/// The prototype device façade: the contract's surfaces over a real `Engine`.
pub struct PrototypeDevice {
    engine: Engine,
    profile: ActivatedProfile,
    map: EntityMap,
    intent_channels: Vec<DefinitionId>,
    bounds: DeclaredBounds,
    negotiated: ProtocolVersion,
    /// Contract §6.2 / §4.3: canonical cohort reports accumulated across every
    /// `process` call of the **currently active request**.
    ///
    /// A `ProcessResult` carries only the cohorts of its own call. A paced
    /// request that pauses therefore spreads its committed cohorts over several
    /// results, and a device that projected only from the completing result
    /// would silently drop every intent committed before the last pause. The
    /// batch is published only at the completed boundary (§4.3), so the reports
    /// are held here until then.
    accumulated: Vec<CohortReport>,
    accumulated_for: Option<Digest>,
}

impl PrototypeDevice {
    /// Contract §3: open a session, negotiating version, profile and
    /// capabilities. Fails closed on an unsupported major.
    pub fn open(
        engine: Engine,
        profile: ActivatedProfile,
        map: EntityMap,
        intent_channels: Vec<DefinitionId>,
        host_version: ProtocolVersion,
        bounds: DeclaredBounds,
    ) -> Result<(Self, SessionDescriptor), Rejected> {
        if host_version.major != CONTRACT_VERSION.major {
            return Err(Rejected::UnsupportedProtocolVersion {
                supported_major: CONTRACT_VERSION.major,
            });
        }
        let negotiated = ProtocolVersion {
            major: CONTRACT_VERSION.major,
            minor: negotiate_minor(host_version.minor, CONTRACT_VERSION.minor),
        };
        let descriptor = SessionDescriptor {
            negotiated_version: negotiated,
            manifest_content_hash: profile.manifest_content_hash().clone(),
            activation_hash: profile.activation_hash().clone(),
            capabilities: vec![
                Capability::ObserveV1,
                Capability::AdvanceV1,
                Capability::IntentV1,
                Capability::ReplayV1,
                Capability::SnapshotV1,
                Capability::EpochV1,
            ],
            bounds,
        };
        Ok((
            Self {
                engine,
                profile,
                map,
                intent_channels,
                bounds,
                negotiated,
                accumulated: Vec::new(),
                accumulated_for: None,
            },
            descriptor,
        ))
    }

    pub fn negotiated_version(&self) -> ProtocolVersion {
        self.negotiated
    }

    pub fn bounds(&self) -> DeclaredBounds {
        self.bounds
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn profile(&self) -> &ActivatedProfile {
        &self.profile
    }

    pub fn map(&self) -> &EntityMap {
        &self.map
    }

    /// Contract §3.2: a message naming a different profile identity pair is
    /// refused before anything else happens.
    pub fn check_profile(
        &self,
        manifest_content_hash: &Digest,
        activation_hash: &Digest,
    ) -> Result<(), Rejected> {
        if manifest_content_hash != self.profile.manifest_content_hash()
            || activation_hash != self.profile.activation_hash()
        {
            return Err(Rejected::ProfileMismatch);
        }
        Ok(())
    }

    /// Present one request. This is the whole invocation surface: one
    /// outstanding request, pull results by re-presenting (contract §4.4).
    pub fn present(&mut self, request: &Request) -> DeviceResponse {
        if let Request::Command(c) = request {
            if let spark_engine::request::CommandPayload::Host { observations, .. } = &c.payload {
                let observed = observations.len() as u32;
                if observed > self.bounds.max_observations_per_command {
                    return DeviceResponse::Rejected(Rejected::MessageTooLarge {
                        limit: self.bounds.max_observations_per_command,
                        observed,
                    });
                }
            }
        }
        let result = self.engine.process(request);
        self.interpret(request, result)
    }

    /// Append this call's cohorts to the active request's accumulation,
    /// starting a fresh accumulation when a different request begins.
    fn accumulate(&mut self, correlation: &Digest, reports: &[CohortReport]) {
        if self.accumulated_for.as_ref() != Some(correlation) {
            self.accumulated.clear();
            self.accumulated_for = Some(correlation.clone());
        }
        self.accumulated.extend_from_slice(reports);
    }

    fn take_accumulated(&mut self) -> Vec<CohortReport> {
        self.accumulated_for = None;
        std::mem::take(&mut self.accumulated)
    }

    /// Discard the accumulation, but only if it belongs to this request. A
    /// `Busy` refusal is caused by an interloper and must never disturb the
    /// active request's accumulated work.
    fn discard_if_mine(&mut self, correlation: &Digest) {
        if self.accumulated_for.as_ref() == Some(correlation) {
            self.take_accumulated();
        }
    }

    fn interpret(&mut self, request: &Request, result: ProcessResult) -> DeviceResponse {
        let correlation = request.discriminator().identity().clone();
        match result.outcome() {
            Outcome::Paused => {
                self.accumulate(&correlation, result.reports());
                DeviceResponse::Paused
            }
            Outcome::RefusedHorizonBehindFrontier { frontier, horizon } => {
                let (frontier, horizon) = (*frontier, *horizon);
                self.discard_if_mine(&correlation);
                DeviceResponse::Rejected(Rejected::StaleHorizon { frontier, horizon })
            }
            Outcome::RefusedActiveRequestMismatch { active } => {
                DeviceResponse::Rejected(Rejected::Busy {
                    active_horizon: active.horizon(),
                    active_identity: active.identity().clone(),
                })
            }
            Outcome::FinalizationEntailmentViolated | Outcome::StoreInvariantViolated(_) => {
                // Sticky fail-stop: no stable boundary is published, so nothing
                // accumulated for this request may ever be exported.
                self.discard_if_mine(&correlation);
                DeviceResponse::Rejected(Rejected::FailStopped)
            }
            Outcome::CompletedCommandNotFinalized(refusal) => {
                let refusal = refusal.clone();
                // The horizon completed, so cohorts accumulated for this request
                // are real committed work and must still be published. Only the
                // command itself was refused, and that refusal is carried.
                self.accumulate(&correlation, result.reports());
                let reports = self.take_accumulated();
                let (intents, refusals) = self.project(&reports);
                let horizon = request.horizon();
                let digest = batch_digest(&correlation, horizon, &intents, &self.map);
                DeviceResponse::CompletedWithRefusedCommand {
                    batch: IntentBatch {
                        session_activation: self.profile.activation_hash().clone(),
                        correlation,
                        horizon,
                        intents,
                        batch_digest: digest,
                    },
                    refusals,
                    refusal,
                }
            }
            Outcome::Completed => {
                self.accumulate(&correlation, result.reports());
                let reports = self.take_accumulated();
                let (intents, refusals) = self.project(&reports);
                let horizon = request.horizon();
                let digest = batch_digest(&correlation, horizon, &intents, &self.map);
                let capacity_exceeded =
                    intents.len() as u64 > u64::from(self.bounds.max_intents_per_batch);
                DeviceResponse::Completed {
                    batch: IntentBatch {
                        session_activation: self.profile.activation_hash().clone(),
                        correlation,
                        horizon,
                        intents,
                        batch_digest: digest,
                    },
                    refusals,
                    capacity_exceeded,
                }
            }
        }
    }

    /// Contract §6.2. The projection adds no ordering of its own: it walks the
    /// engine's own canonical order — cohort sequence, then wave index, then
    /// the order the engine already fixes inside a wave.
    fn project(&self, reports: &[CohortReport]) -> (Vec<BehaviorIntent>, Vec<CohortRefusal>) {
        let mut intents = Vec::new();
        let mut refusals = Vec::new();
        for cohort in reports {
            if !matches!(
                cohort.outcome,
                CohortOutcome::Committed | CohortOutcome::EpochActivated { .. }
            ) {
                refusals.push(CohortRefusal {
                    canonical_time: cohort.canonical_time,
                    outcome: cohort.outcome.clone(),
                });
                // A refused cohort exports nothing (contract §6.2).
                continue;
            }
            for wave in &cohort.waves {
                for effect in &wave.committed {
                    if !self.intent_channels.contains(&effect.definition) {
                        continue;
                    }
                    let Some(subject) = self.map.subject_of(&effect.scope) else {
                        // An unmapped scope is never silently dropped into an
                        // anonymous intent; it is a host mapping defect and is
                        // reported as an unmappable refusal.
                        refusals.push(CohortRefusal {
                            canonical_time: cohort.canonical_time,
                            outcome: cohort.outcome.clone(),
                        });
                        continue;
                    };
                    intents.push(BehaviorIntent {
                        subject,
                        channel: effect.definition.clone(),
                        value: effect.value.clone(),
                        leverage: self.leverage_of(&effect.definition),
                        canonical_time: cohort.canonical_time,
                        provenance: effect.provenance.clone(),
                    });
                }
            }
        }
        (intents, refusals)
    }

    fn leverage_of(&self, _definition: &DefinitionId) -> Option<BehavioralLeverage> {
        // `behavioral_leverage` is declared metadata on the manifest input and
        // is not republished on the activated definition at the pinned tree, so
        // the prototype carries `None` rather than inventing a value. Contract
        // §6.2 marks the field advisory for exactly this reason.
        None
    }
}

// ===================================================================== §7.2

/// Contract §7.2: the host's at-most-once application ledger, keyed by
/// `(CorrelationId, batch_digest)`.
#[derive(Debug, Default, Clone)]
pub struct ApplicationLedger {
    applied: BTreeMap<(Digest, Digest, Digest), usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationDecision {
    /// First delivery of this batch identity: apply it now.
    Apply,
    /// Already applied. Acknowledge again; do not re-apply.
    AlreadyApplied,
}

impl ApplicationLedger {
    pub fn key_of(batch: &IntentBatch) -> (Digest, Digest, Digest) {
        (
            batch.session_activation.clone(),
            batch.correlation.clone(),
            batch.batch_digest.clone(),
        )
    }

    pub fn admit(&mut self, batch: &IntentBatch) -> ApplicationDecision {
        let key = Self::key_of(batch);
        match self.applied.get_mut(&key) {
            Some(seen) => {
                *seen = seen.saturating_add(1);
                ApplicationDecision::AlreadyApplied
            }
            None => {
                self.applied.insert(key, 1);
                ApplicationDecision::Apply
            }
        }
    }

    pub fn deliveries(&self, batch: &IntentBatch) -> usize {
        self.applied.get(&Self::key_of(batch)).copied().unwrap_or(0)
    }
}
