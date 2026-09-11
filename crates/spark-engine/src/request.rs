//! Serialized requests, the engine-owned `ActiveRequest` discriminator, and
//! request-bound results (V3-F01 FINAL §4–§8 as amended by Revision 2 §5).

use crate::profile::config::ConfigRevision;
use crate::report::{CohortReport, PacingDiagnostics};
use crate::rules::ActivatedRuleSet;
use spark_core::clock::LogicalTime;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{CommandId, DefinitionId, ProfileId, SourceId};
use spark_core::scope::ScopeId;
use spark_core::timeline::{CommandKind, Ordinal, SemanticCommandEnvelope, TimelineEpoch};
use spark_core::value::CanonicalValue;

/// One host observation carried by a command: host-owned truth enters only
/// through canonical ingress (ADR-0002).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observation {
    pub definition: DefinitionId,
    pub value: CanonicalValue,
}

/// The canonical payload of a command request. Its canonical hash is the
/// envelope's `canonical_payload_hash`; the payload itself travels with the
/// request because a hash does not reconstruct a payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandPayload {
    /// A host command: ingress observations at `subject`, then every rule
    /// triggered by the command kind evaluates at `subject` with `params`.
    Host {
        subject: ScopeId,
        observations: Vec<Observation>,
        params: Vec<i64>,
    },
    /// The reserved epoch-activation command (v1 Q5): activates a new
    /// `(rule set, config revision)` pair at its own barrier.
    ActivateEpoch {
        rule_set: ActivatedRuleSet,
        config: ConfigRevision,
    },
}

impl CommandPayload {
    pub fn canonical_hash(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        match self {
            CommandPayload::Host {
                subject,
                observations,
                params,
            } => {
                enc.push_str("command_payload.host_v1");
                subject.canonicalize(&mut enc);
                enc.push_u64(observations.len() as u64);
                for o in observations {
                    let mut inner = CanonicalEncoder::new();
                    o.definition.canonicalize(&mut inner);
                    o.value.canonicalize(&mut inner);
                    enc.push_block(&inner);
                }
                enc.push_u64(params.len() as u64);
                for p in params {
                    enc.push_i64(*p);
                }
            }
            CommandPayload::ActivateEpoch { rule_set, config } => {
                enc.push_str("command_payload.activate_epoch_v1");
                enc.push_digest(rule_set.content_hash());
                enc.push_digest(&config.config_revision_hash());
            }
        }
        enc.finish()
    }
}

/// A command request: the `SemanticCommandEnvelope` minus the
/// sequencer-issued `input_ordinal`, which the engine assigns as the frontier
/// at finalization (D-1), plus the payload itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandRequest {
    pub command_id: CommandId,
    pub profile_id: ProfileId,
    pub timeline_epoch: TimelineEpoch,
    pub effective_time: LogicalTime,
    pub source_id: SourceId,
    pub source_sequence: u64,
    pub command_kind: CommandKind,
    pub payload: CommandPayload,
}

impl CommandRequest {
    pub fn canonical_payload_hash(&self) -> Digest {
        self.payload.canonical_hash()
    }

    /// `CommandRequest::canonicalize`: the `SemanticCommandEnvelope` field order
    /// with `input_ordinal` omitted (FINAL §8.1).
    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.profile_id.canonicalize(enc);
        enc.push_u64(self.timeline_epoch.0);
        self.effective_time.canonicalize(enc);
        self.source_id.canonicalize(enc);
        enc.push_u64(self.source_sequence);
        self.command_id.canonicalize(enc);
        self.command_kind.canonicalize(enc);
        enc.push_digest(&self.canonical_payload_hash());
    }

    /// The envelope at a given ordinal (the frontier at finalization).
    pub fn envelope_at(&self, input_ordinal: Ordinal) -> SemanticCommandEnvelope {
        SemanticCommandEnvelope {
            command_id: self.command_id.clone(),
            profile_id: self.profile_id.clone(),
            timeline_epoch: self.timeline_epoch,
            effective_time: self.effective_time,
            source_id: self.source_id.clone(),
            source_sequence: self.source_sequence,
            input_ordinal,
            command_kind: self.command_kind.clone(),
            canonical_payload_hash: self.canonical_payload_hash(),
        }
    }
}

/// One external input to the serialized request boundary.
///
/// `Command` carries its payload by value (a hash does not reconstruct a
/// payload); requests are presented by reference, so the size difference
/// between variants is not a copying cost.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Advance(LogicalTime),
    Command(CommandRequest),
}

impl Request {
    /// The request horizon: the advance target or the command effective time.
    pub fn horizon(&self) -> LogicalTime {
        match self {
            Request::Advance(t) => *t,
            Request::Command(c) => c.effective_time,
        }
    }

    pub fn discriminator(&self) -> RequestDiscriminator {
        let mut enc = CanonicalEncoder::new();
        let kind = match self {
            Request::Advance(t) => {
                enc.push_str("request_advance_v1");
                t.canonicalize(&mut enc);
                RequestKind::Advance
            }
            Request::Command(c) => {
                enc.push_str("request_command_v1");
                c.canonicalize(&mut enc);
                RequestKind::Command
            }
        };
        RequestDiscriminator {
            kind,
            horizon: self.horizon(),
            identity: enc.finish(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestKind {
    Advance,
    Command,
}

impl RequestKind {
    pub fn tag(&self) -> &'static str {
        match self {
            RequestKind::Advance => "advance",
            RequestKind::Command => "command",
        }
    }
}

/// `RequestDiscriminator { kind, horizon, identity }` (FINAL §6.2). Private
/// fields; derived only from a [`Request`], so a host cannot construct one.
/// `kind` and `horizon` are redundant with `identity` and are carried for
/// observation and typed refusal reporting.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RequestDiscriminator {
    kind: RequestKind,
    horizon: LogicalTime,
    identity: Digest,
}

impl RequestDiscriminator {
    pub fn kind(&self) -> RequestKind {
        self.kind
    }

    pub fn horizon(&self) -> LogicalTime {
        self.horizon
    }

    pub fn identity(&self) -> &Digest {
        &self.identity
    }
}

/// `canonicalize(ActiveRequest)` (FINAL §6.2).
pub fn canonicalize_active_request(
    active: Option<&RequestDiscriminator>,
    enc: &mut CanonicalEncoder,
) {
    match active {
        None => {
            enc.push_str("active_request.none");
        }
        Some(d) => {
            enc.push_str("active_request.some");
            enc.push_str(d.kind.tag());
            d.horizon.canonicalize(enc);
            enc.push_digest(&d.identity);
        }
    }
}

/// One typed finalization refusal row (Revision-2 §4.4).
///
/// Every variant is `#[non_exhaustive]`, so a refusal is constructible only
/// inside `spark_engine` (Revision-2 oracle §10): an external crate can match
/// a refusal with `{ .. }` patterns and read its fields, but cannot build one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizationRefusal {
    /// P-1.
    #[non_exhaustive]
    WrongProfile,
    /// P-2.
    #[non_exhaustive]
    WrongTimelineEpoch,
    /// P-3 (unreachable under engine ownership; still checked).
    #[non_exhaustive]
    NotActiveSequencer,
    /// P-4.
    #[non_exhaustive]
    OrdinalSpaceExhaustedWindow,
    /// P-5.
    #[non_exhaustive]
    OrdinalSpaceExhaustedFrontierAdvance,
    /// P-6 (unreachable under I-CS; fail closed).
    #[non_exhaustive]
    UnexpectedStagingState { ordinal: Ordinal },
    /// P-7.
    #[non_exhaustive]
    CommandIdentityConflict { command_id: CommandId },
    /// P-8.
    #[non_exhaustive]
    SourceSequenceConflict {
        source_id: SourceId,
        source_sequence: u64,
    },
    /// P-9.
    #[non_exhaustive]
    SourceSequenceNotIncreasing {
        source_id: SourceId,
        previously_finalized: u64,
        attempted: u64,
    },
}

/// The request-level outcome of one `process` call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// Budget exhausted with an executable slice or the command still pending.
    Paused,
    Completed,
    /// Horizon completed; the command was not finalized (D-2).
    CompletedCommandNotFinalized(FinalizationRefusal),
    RefusedHorizonBehindFrontier {
        frontier: LogicalTime,
        horizon: LogicalTime,
    },
    RefusedActiveRequestMismatch {
        active: RequestDiscriminator,
    },
    /// Sticky fail-stop (D-8): an implementation defect, never an ordinary
    /// refusal. No stable boundary or snapshot is published; every later call
    /// returns this until the host restores the last committed snapshot.
    FinalizationEntailmentViolated,
    /// Sticky fail-stop (the same D-8 discipline, D-C2-11 revised): the
    /// engine-owned cross-store extraction found the scheduler and the
    /// `ObligationStore` in disagreement — an implementation or tampering
    /// defect, unreachable through the facade. The typed refusal is carried,
    /// never discarded; the extraction attempt mutated nothing, but
    /// `ActiveRequest` was set and any cohort committed earlier in this call
    /// stays committed. No stable boundary or snapshot is published; every
    /// later call returns this until the last committed snapshot is restored.
    /// Not dequeue-eligible.
    StoreInvariantViolated(crate::engine::ExtractionRefusal),
}

/// A request-bound result (Revision-2 §5.1): it always names the presented
/// request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessResult {
    pub(crate) presented: RequestDiscriminator,
    pub(crate) outcome: Outcome,
    pub(crate) reports: Vec<CohortReport>,
    pub(crate) diagnostics: Option<PacingDiagnostics>,
}

impl ProcessResult {
    pub fn presented(&self) -> &RequestDiscriminator {
        &self.presented
    }

    pub fn outcome(&self) -> &Outcome {
        &self.outcome
    }

    /// Canonical semantic cohort reports produced by this call, in cohort
    /// sequence order.
    pub fn reports(&self) -> &[CohortReport] {
        &self.reports
    }

    /// Noncanonical pacing telemetry (v3 §3.4); `None` for refusals.
    pub fn diagnostics(&self) -> Option<&PacingDiagnostics> {
        self.diagnostics.as_ref()
    }

    /// Request-bound dequeue eligibility (Revision-2 §5.2): true iff this
    /// result terminates exactly `head`. A mismatch terminates nothing.
    pub fn terminates(&self, head: &RequestDiscriminator) -> bool {
        &self.presented == head
            && matches!(
                self.outcome,
                Outcome::Completed
                    | Outcome::CompletedCommandNotFinalized(_)
                    | Outcome::RefusedHorizonBehindFrontier { .. }
            )
    }
}
