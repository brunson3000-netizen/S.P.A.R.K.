//! Canonical timeline ingress: sequencer authority, deterministic
//! ordinal-credit admission window, one-slot-per-ordinal staging,
//! structured stage acknowledgements, contiguous digest-linked finality
//! fences, structured finalization evidence, and authorized epoch
//! handoff/reset.
//!
//! This module implements ADR-0003
//! (`engineering/phase0/ADR-0003-deterministic-clock-rng-and-commit-order.md`)
//! and is re-founded per `PHASE_1_REFOUNDATION_BRIEF_v0.1.md` §1 and §2.
//!
//! # The re-foundation: semantic data and admission data are different types
//!
//! The previous implementation kept one `CommandEnvelope` carrying both
//! the command's behavioral identity *and* its `admission_window_token`.
//! It then tried to keep the token out of canonical identity by excluding
//! it from one hash function. That failed independent review twice,
//! because the token was still retained in every finalized envelope and
//! still reached the ingress state digest: two runs that finalized the
//! *same* semantic commands behind the *same* fence chain produced
//! different canonical digests purely according to whether a tail ordinal
//! was staged before or after the frontier advanced. Nonsemantic
//! admission timing had become canonical replay identity.
//!
//! Excluding a field from one hash is a convention. The re-founded
//! boundary makes it a type:
//!
//! - [`SemanticCommandEnvelope`] is the command. It is the only thing
//!   staged, retained, finalized, hashed, or replayed.
//! - [`AdmissionTicket`] is the ephemeral credential proving the
//!   sequencer held a valid admission window when it submitted. It is
//!   consumed at the admission boundary and **never stored**. It has no
//!   `canonicalize` method at all, so it cannot reach a canonical hash
//!   even by mistake.
//! - [`SubmittedCommand`] pairs them for the duration of one `stage` call
//!   and no longer.
//!
//! ```compile_fail
//! use spark_core::hash::CanonicalEncoder;
//! use spark_core::timeline::AdmissionTicket;
//! fn contaminate(ticket: &AdmissionTicket, enc: &mut CanonicalEncoder) {
//!     // Admission credentials have no canonical encoding, by construction.
//!     ticket.canonicalize(enc);
//! }
//! ```
//!
//! # Structured evidence, not bare enums
//!
//! ADR-0003 §7 requires a positive staging acknowledgement that
//! *identifies* what it covers: profile, epoch, ordinal, command ID,
//! canonical payload/semantic hash, and slot state. The previous
//! implementation returned a bare `StageOutcome` enum and
//! `Result<(), FenceError>`, so a transport could not prove which envelope
//! a positive acknowledgement covered. [`StageAcknowledgement`] now binds
//! all of those fields plus a derived [`StageAcknowledgement::acknowledgement_hash`],
//! and [`FinalizationResult`] identifies the finalized fence, range,
//! resulting frontier, and canonical history digest.
//!
//! # Other invariants preserved from the ADR
//!
//! 1. **Capacity is allocated by ordinal position, not arrival.** At
//!    frontier `n` with window width `W`, every ordinal in `[n, n+W-1]`
//!    owns a distinct slot before any command for it arrives.
//! 2. **Identity is the full semantic envelope**, never a fragment: two
//!    envelopes agreeing on ordinal/command-ID/payload but differing in
//!    `effective_time`, `source_id`, `source_sequence`, or `command_kind`
//!    are distinct commands.
//! 3. **Command IDs and `(source_id, source_sequence)` pairs are unique**
//!    within finalized-and-staged history — and *only* within it. A claim
//!    is reserved exactly while its envelope is finalized or positively
//!    staged; an envelope swallowed by a contest is neither, so both
//!    contestants' identities become **unclaimed** after the contest, in
//!    every arrival order. That is the coherent reading of the frozen
//!    poison rule (ADR-0003 §11): if no arrival picks a winner, then
//!    neither contestant's claim is honored, including its identity
//!    reservation.
//!
//!    Screening for that uniqueness guards **admission into positive
//!    staging only**. A claim recorded as contest evidence is recorded by
//!    semantic hash, unconditionally and without consulting any identity
//!    registry: evidence is a record of contest, not an admission. The
//!    asymmetry is deliberate and load-bearing — screening the poison
//!    branch would make a poisoned slot's evidence *set* depend on
//!    registry contents, hence on arrival order, which is exactly the
//!    property the poison evidence exists to avoid.
//! 4. **Finalized per-source sequence numbers strictly increase**; gaps
//!    are legal, regression is not.
//! 5. **All ordinal arithmetic is total.** Nothing here panics: window and
//!    frontier arithmetic returns a typed error, and a fence that cannot
//!    advance the frontier without overflow is rejected *before* anything
//!    is promoted.

use crate::clock::LogicalTime;
use crate::evidence::BoundedClaimSet;
use crate::hash::{CanonicalEncoder, Digest};
use crate::id::{CanonicalTag, CommandId, FenceId, ProfileId, SourceId};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// A position in the canonical command sequence for one profile timeline
/// epoch. Ordinals are issued by the active sequencer, never inferred
/// from arrival order (ADR-0003 §2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ordinal(pub u64);

/// Identifies one grant of exclusive sequencing authority. A new epoch is
/// created only through explicit, authorized [`TimelineIngress::reset_epoch`]
/// handoff (ADR-0003 §11), and must be strictly greater than the epoch it
/// replaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimelineEpoch(pub u64);

/// The `command_kind` discriminator of a canonical envelope
/// (ADR-0003 §2). It is canonical identity, so it is a bounded validated
/// [`CanonicalTag`] rather than an unbounded `String`: an unbounded
/// canonical string is both a hashing/storage vector and a way for two
/// deployments to silently disagree about what is canonical.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandKind(CanonicalTag);

impl CommandKind {
    pub fn new(tag: CanonicalTag) -> Self {
        CommandKind(tag)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.0.canonicalize(enc);
    }
}

impl fmt::Display for CommandKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Rejects an ingress whose configuration is not usable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimelineConfigError {
    /// ADR-0003 §3 requires a finite window width of at least one, so the
    /// next frontier ordinal always has a reserved slot.
    WindowWidthMustBeAtLeastOne,
}

impl fmt::Display for TimelineConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimelineConfigError::WindowWidthMustBeAtLeastOne => {
                write!(f, "admission window width must be at least 1")
            }
        }
    }
}

impl std::error::Error for TimelineConfigError {}

/// The ordinal space for this timeline epoch is exhausted: the requested
/// arithmetic would exceed `u64`.
///
/// This is a normal, reportable terminal condition, not a panic. Recovery
/// is an explicit epoch handoff (ADR-0003 §11), never a wrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimelineOrdinalSpaceExhausted {
    pub frontier_ordinal: Ordinal,
    pub window_width: u32,
}

impl fmt::Display for TimelineOrdinalSpaceExhausted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "timeline ordinal space is exhausted at frontier {} with window width {}",
            self.frontier_ordinal.0, self.window_width
        )
    }
}

impl std::error::Error for TimelineOrdinalSpaceExhausted {}

/// The deterministic admission-window credential bound to one specific
/// `(profile, epoch, frontier, window_width, last_finalized_fence_hash)`
/// tuple (ADR-0003 §3).
///
/// The inner digest is private: a token is a credential the ingress
/// issues, not a value a caller composes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionWindowToken(Digest);

fn compute_window_token(
    profile_id: &ProfileId,
    epoch: TimelineEpoch,
    frontier_ordinal: Ordinal,
    window_width: u32,
    last_finalized_fence_hash: &Digest,
) -> AdmissionWindowToken {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("admission_window_token");
    profile_id.canonicalize(&mut enc);
    enc.push_u64(epoch.0);
    enc.push_u64(frontier_ordinal.0);
    enc.push_u32(window_width);
    enc.push_digest(last_finalized_fence_hash);
    AdmissionWindowToken(enc.finish())
}

/// A read-only snapshot of the current deterministic admission window
/// (ADR-0003 §3). A sequencer reads this to obtain the
/// [`AdmissionTicket`] it must present when staging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionWindow {
    pub profile_id: ProfileId,
    pub timeline_epoch: TimelineEpoch,
    pub frontier_ordinal: Ordinal,
    pub window_width: u32,
    pub window_end: Ordinal,
    pub last_finalized_fence_hash: Digest,
    token: AdmissionWindowToken,
}

impl AdmissionWindow {
    /// Mints the ephemeral admission credential for this window.
    ///
    /// The resulting [`AdmissionTicket`] is transport/admission metadata:
    /// it authorizes a submission, and then it is gone. It never becomes
    /// part of any command's identity or of canonical history.
    pub fn ticket(&self) -> AdmissionTicket {
        AdmissionTicket {
            token: self.token.clone(),
        }
    }

    /// Whether `ordinal` currently falls inside `[frontier, window_end]`.
    pub fn covers(&self, ordinal: Ordinal) -> bool {
        ordinal.0 >= self.frontier_ordinal.0 && ordinal.0 <= self.window_end.0
    }
}

/// The ephemeral credential proving the submitting sequencer held a valid
/// admission window (ADR-0003 §5).
///
/// This type deliberately has **no** `canonicalize` method and is never
/// stored by [`TimelineIngress`]. Admission timing is not behavior, so it
/// is structurally incapable of contaminating canonical identity, replay
/// digests, or finalized history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionTicket {
    token: AdmissionWindowToken,
}

/// One canonical, state-changing/evaluation command
/// (ADR-0003 §2), containing **only** semantic fields.
///
/// `command_kind`/payload semantics belong to Phase 2's rule runtime;
/// Phase 1 treats the payload as an opaque, already-hashed blob so the
/// ingress skeleton can be fully exercised without a rule language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCommandEnvelope {
    pub command_id: CommandId,
    pub profile_id: ProfileId,
    pub timeline_epoch: TimelineEpoch,
    pub effective_time: LogicalTime,
    pub source_id: SourceId,
    pub source_sequence: u64,
    pub input_ordinal: Ordinal,
    pub command_kind: CommandKind,
    pub canonical_payload_hash: Digest,
}

impl SemanticCommandEnvelope {
    /// The canonical encoding of this command. Because the type contains
    /// only semantic fields, the canonical encoding and the behavioral
    /// identity are the same thing — there is no "full" encoding that
    /// includes more.
    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.profile_id.canonicalize(enc);
        enc.push_u64(self.timeline_epoch.0);
        self.effective_time.canonicalize(enc);
        self.source_id.canonicalize(enc);
        enc.push_u64(self.source_sequence);
        enc.push_u64(self.input_ordinal.0);
        self.command_id.canonicalize(enc);
        self.command_kind.canonicalize(enc);
        enc.push_digest(&self.canonical_payload_hash);
    }

    /// The full behavioral identity of this command: the single value
    /// that decides "same command" everywhere identity matters — staging
    /// idempotency/collision, command-ID uniqueness, source-sequence
    /// uniqueness, and the fence's ordered stream digest.
    pub fn semantic_hash(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("semantic_envelope_identity");
        self.canonicalize(&mut enc);
        enc.finish()
    }

    /// Pairs this command with the admission credential under which the
    /// sequencer is submitting it.
    pub fn submit_with(self, admission: AdmissionTicket) -> SubmittedCommand {
        SubmittedCommand {
            semantic: self,
            admission,
        }
    }
}

/// One command plus the admission credential it is being submitted under.
///
/// The pairing lasts exactly as long as one [`TimelineIngress::stage`]
/// call: the ingress checks the credential, then keeps only
/// [`SubmittedCommand::semantic`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmittedCommand {
    pub semantic: SemanticCommandEnvelope,
    pub admission: AdmissionTicket,
}

/// Which positive slot state a [`StageAcknowledgement`] attests to
/// (ADR-0003 §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcknowledgedSlotState {
    /// The ordinal's slot was empty and is now occupied by this command.
    NewlyStaged,
    /// A semantically identical envelope was already staged for this
    /// ordinal; idempotent, no state change.
    AlreadyStagedIdempotent,
}

impl AcknowledgedSlotState {
    pub fn tag(&self) -> &'static str {
        match self {
            AcknowledgedSlotState::NewlyStaged => "newly_staged",
            AcknowledgedSlotState::AlreadyStagedIdempotent => "already_staged_idempotent",
        }
    }
}

/// The positive `STAGED` acknowledgement required by ADR-0003 §7 before a
/// sequencer may fence an ordinal range.
///
/// It binds every field the ADR enumerates — profile, timeline epoch,
/// input ordinal, command ID, the canonical semantic-envelope hash, and
/// the resulting slot state — plus a derived
/// [`acknowledgement_hash`](Self::acknowledgement_hash) over all of them.
/// A transport holding this value can prove exactly which envelope the
/// acknowledgement covers, which a bare enum could not.
///
/// Fields are private and there is no public constructor: an
/// acknowledgement is evidence the ingress issued, not a value a caller
/// can assert.
///
/// ```compile_fail
/// use spark_core::timeline::{AcknowledgedSlotState, StageAcknowledgement};
/// use spark_core::hash::Digest;
/// let forged = StageAcknowledgement {
///     slot_state: AcknowledgedSlotState::NewlyStaged,
///     semantic_envelope_hash: Digest::ZERO,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageAcknowledgement {
    profile_id: ProfileId,
    timeline_epoch: TimelineEpoch,
    input_ordinal: Ordinal,
    command_id: CommandId,
    semantic_envelope_hash: Digest,
    slot_state: AcknowledgedSlotState,
    acknowledgement_hash: Digest,
}

impl StageAcknowledgement {
    fn issue(
        profile_id: ProfileId,
        timeline_epoch: TimelineEpoch,
        input_ordinal: Ordinal,
        command_id: CommandId,
        semantic_envelope_hash: Digest,
        slot_state: AcknowledgedSlotState,
    ) -> Self {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("stage_acknowledgement");
        profile_id.canonicalize(&mut enc);
        enc.push_u64(timeline_epoch.0);
        enc.push_u64(input_ordinal.0);
        command_id.canonicalize(&mut enc);
        enc.push_digest(&semantic_envelope_hash);
        enc.push_str(slot_state.tag());
        let acknowledgement_hash = enc.finish();
        Self {
            profile_id,
            timeline_epoch,
            input_ordinal,
            command_id,
            semantic_envelope_hash,
            slot_state,
            acknowledgement_hash,
        }
    }

    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    pub fn timeline_epoch(&self) -> TimelineEpoch {
        self.timeline_epoch
    }

    pub fn input_ordinal(&self) -> Ordinal {
        self.input_ordinal
    }

    pub fn command_id(&self) -> &CommandId {
        &self.command_id
    }

    pub fn semantic_envelope_hash(&self) -> &Digest {
        &self.semantic_envelope_hash
    }

    pub fn slot_state(&self) -> AcknowledgedSlotState {
        self.slot_state
    }

    pub fn acknowledgement_hash(&self) -> &Digest {
        &self.acknowledgement_hash
    }

    /// Whether this acknowledgement actually covers `envelope`: same
    /// profile, epoch, ordinal, command ID, and semantic identity. This is
    /// the check a transport performs before treating a positive
    /// acknowledgement as satisfying ADR-0003 §7 for that command.
    pub fn covers(&self, envelope: &SemanticCommandEnvelope) -> bool {
        self.profile_id == envelope.profile_id
            && self.timeline_epoch == envelope.timeline_epoch
            && self.input_ordinal == envelope.input_ordinal
            && self.command_id == envelope.command_id
            && self.semantic_envelope_hash == envelope.semantic_hash()
    }
}

/// Evidence that an ordinal's slot is poisoned: two semantically
/// different envelopes claimed it, so no arrival order may pick a winner
/// (ADR-0003 §11).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotPoisonRecord {
    profile_id: ProfileId,
    timeline_epoch: TimelineEpoch,
    input_ordinal: Ordinal,
    rejected_semantic_hash: Digest,
}

impl SlotPoisonRecord {
    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    pub fn timeline_epoch(&self) -> TimelineEpoch {
        self.timeline_epoch
    }

    pub fn input_ordinal(&self) -> Ordinal {
        self.input_ordinal
    }

    /// The semantic identity of the envelope that was refused admission
    /// into the poisoned slot.
    pub fn rejected_semantic_hash(&self) -> &Digest {
        &self.rejected_semantic_hash
    }
}

/// The retryable out-of-window result (ADR-0003 §6). No slot was touched,
/// no state mutated, nothing evicted or poisoned. The advice carries the
/// current window so the sequencer knows what to retry against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionRetryAdvice {
    profile_id: ProfileId,
    timeline_epoch: TimelineEpoch,
    attempted_ordinal: Ordinal,
    frontier_ordinal: Ordinal,
    window_end: Ordinal,
}

impl AdmissionRetryAdvice {
    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    pub fn timeline_epoch(&self) -> TimelineEpoch {
        self.timeline_epoch
    }

    pub fn attempted_ordinal(&self) -> Ordinal {
        self.attempted_ordinal
    }

    pub fn frontier_ordinal(&self) -> Ordinal {
        self.frontier_ordinal
    }

    pub fn window_end(&self) -> Ordinal {
        self.window_end
    }
}

/// The logical result of one [`TimelineIngress::stage`] attempt
/// (ADR-0003 §4-§6). None of these are protocol violations;
/// [`StageDisposition::NotInAdmissionWindow`] in particular is an
/// expected, retryable outcome that consumes no capacity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageDisposition {
    /// A positive `STAGED` acknowledgement covering this exact envelope.
    Acknowledged(StageAcknowledgement),
    /// The ordinal is poisoned; it can only be recovered by an explicit
    /// epoch handoff.
    Poisoned(SlotPoisonRecord),
    /// The ordinal lies outside the current window. Retry after obtaining
    /// a later admission window.
    NotInAdmissionWindow(AdmissionRetryAdvice),
}

impl StageDisposition {
    /// Whether this disposition is a positive `STAGED` acknowledgement,
    /// the ADR-0003 §7 fence precondition.
    pub fn acknowledgement(&self) -> Option<&StageAcknowledgement> {
        match self {
            StageDisposition::Acknowledged(ack) => Some(ack),
            _ => None,
        }
    }

    /// A stable tag used for deterministic transcripts.
    pub fn tag(&self) -> &'static str {
        match self {
            StageDisposition::Acknowledged(ack) => ack.slot_state.tag(),
            StageDisposition::Poisoned(_) => "poisoned",
            StageDisposition::NotInAdmissionWindow(_) => "not_in_admission_window",
        }
    }
}

/// A protocol-level rejection of a `stage` attempt: the submission was
/// misauthorized or identity-conflicting for this ingress, as opposed to
/// a legitimate logical outcome like
/// [`StageDisposition::NotInAdmissionWindow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageError {
    WrongProfile,
    WrongTimelineEpoch,
    NotActiveSequencer,
    /// The ordinal is within the current window's numeric range, but the
    /// presented admission ticket does not match the current window
    /// (e.g. a stale ticket from before the frontier last advanced).
    StaleOrInvalidAdmissionTicket,
    /// `command_id` was already used (staged or finalized) by a
    /// semantically different envelope. Exact reuse (same semantic
    /// identity) is not an error.
    CommandIdentityConflict {
        command_id: CommandId,
    },
    /// `(source_id, source_sequence)` was already used (staged or
    /// finalized) by a semantically different envelope.
    SourceSequenceConflict {
        source_id: SourceId,
        source_sequence: u64,
    },
    /// The ordinal space for this epoch is exhausted; recovery is an
    /// explicit epoch handoff, never a wrap.
    OrdinalSpaceExhausted(TimelineOrdinalSpaceExhausted),
}

impl fmt::Display for StageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StageError::WrongProfile => write!(f, "envelope names a different profile"),
            StageError::WrongTimelineEpoch => write!(f, "envelope names a different timeline epoch"),
            StageError::NotActiveSequencer => {
                write!(f, "submitting source is not the active sequencer")
            }
            StageError::StaleOrInvalidAdmissionTicket => {
                write!(f, "admission ticket is stale or invalid for the current window")
            }
            StageError::CommandIdentityConflict { command_id } => write!(
                f,
                "command id '{command_id}' was already used by a semantically different envelope"
            ),
            StageError::SourceSequenceConflict {
                source_id,
                source_sequence,
            } => write!(
                f,
                "source '{source_id}' sequence {source_sequence} was already used by a semantically different envelope"
            ),
            StageError::OrdinalSpaceExhausted(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for StageError {}

/// One canonical timeline finality fence (ADR-0003 §8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineFence {
    pub profile_id: ProfileId,
    pub timeline_epoch: TimelineEpoch,
    pub fence_id: FenceId,
    pub start_ordinal: Ordinal,
    pub end_ordinal: Ordinal,
    pub previous_fence_hash: Digest,
    pub ordered_stream_digest: Digest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceError {
    WrongProfile,
    WrongTimelineEpoch,
    NotActiveSequencer,
    /// `start_ordinal` must equal the current frontier (ADR-0003 §8.2).
    StartNotAtFrontier,
    /// `end_ordinal` must be within the currently stageable horizon and
    /// not precede `start_ordinal` (ADR-0003 §8.3).
    EndOutsideStageableHorizon,
    /// Some ordinal in the range has no positive `STAGED` acknowledgement
    /// (ADR-0003 §7, §8.4).
    RangeNotFullyStaged {
        ordinal: Ordinal,
    },
    /// Some ordinal in the range is poisoned (ADR-0003 §8.4).
    RangeContainsPoisoned {
        ordinal: Ordinal,
    },
    /// `ordered_stream_digest` does not match the ordered canonical
    /// semantic envelope identities (ADR-0003 §8.5).
    DigestMismatch,
    /// `previous_fence_hash` does not match the finalized chain
    /// (ADR-0003 §8.6).
    PreviousFenceHashMismatch,
    /// Finalizing this range would make some source's finalized
    /// `source_sequence` go backward. Gaps are legal; regression is not.
    SourceSequenceNotIncreasing {
        source_id: SourceId,
        previously_finalized: u64,
        attempted: u64,
    },
    /// Advancing the frontier past `end_ordinal` would exceed the ordinal
    /// space. Checked **before** anything is promoted, so the fence is
    /// rejected atomically (ADR-0003 §8.8).
    OrdinalSpaceExhausted(TimelineOrdinalSpaceExhausted),
}

impl fmt::Display for FenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FenceError::WrongProfile => write!(f, "fence names a different profile"),
            FenceError::WrongTimelineEpoch => write!(f, "fence names a different timeline epoch"),
            FenceError::NotActiveSequencer => {
                write!(f, "submitting source is not the active sequencer")
            }
            FenceError::StartNotAtFrontier => {
                write!(f, "fence start ordinal is not the current frontier")
            }
            FenceError::EndOutsideStageableHorizon => {
                write!(f, "fence end ordinal is outside the stageable horizon")
            }
            FenceError::RangeNotFullyStaged { ordinal } => write!(
                f,
                "ordinal {} in the fence range has no positive staged acknowledgement",
                ordinal.0
            ),
            FenceError::RangeContainsPoisoned { ordinal } => {
                write!(f, "ordinal {} in the fence range is poisoned", ordinal.0)
            }
            FenceError::DigestMismatch => {
                write!(f, "fence ordered stream digest does not match the staged range")
            }
            FenceError::PreviousFenceHashMismatch => {
                write!(f, "fence previous-fence hash does not match the finalized chain")
            }
            FenceError::SourceSequenceNotIncreasing {
                source_id,
                previously_finalized,
                attempted,
            } => write!(
                f,
                "source '{source_id}' would finalize sequence {attempted} after {previously_finalized}"
            ),
            FenceError::OrdinalSpaceExhausted(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for FenceError {}

/// Structured evidence that one fence finalized one ordinal range
/// (ADR-0003 §8, re-foundation brief §2).
///
/// A bare `Ok(())` could not tell a transport, a persistence layer, or a
/// reviewer *what* had been finalized. This record identifies the fence,
/// the exact finalized range, the resulting frontier, the fence hash that
/// now anchors the chain, and the canonical history digest after
/// promotion. Fields are private: like [`StageAcknowledgement`], this is
/// evidence the ingress issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizationResult {
    profile_id: ProfileId,
    timeline_epoch: TimelineEpoch,
    fence_id: FenceId,
    start_ordinal: Ordinal,
    end_ordinal: Ordinal,
    finalized_command_count: u64,
    previous_fence_hash: Digest,
    fence_hash: Digest,
    ordered_stream_digest: Digest,
    new_frontier_ordinal: Ordinal,
    canonical_history_digest: Digest,
}

impl FinalizationResult {
    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    pub fn timeline_epoch(&self) -> TimelineEpoch {
        self.timeline_epoch
    }

    pub fn fence_id(&self) -> &FenceId {
        &self.fence_id
    }

    pub fn start_ordinal(&self) -> Ordinal {
        self.start_ordinal
    }

    pub fn end_ordinal(&self) -> Ordinal {
        self.end_ordinal
    }

    pub fn finalized_command_count(&self) -> u64 {
        self.finalized_command_count
    }

    pub fn previous_fence_hash(&self) -> &Digest {
        &self.previous_fence_hash
    }

    /// The hash of the fence just finalized; it becomes the
    /// `previous_fence_hash` the next fence must present.
    pub fn fence_hash(&self) -> &Digest {
        &self.fence_hash
    }

    pub fn ordered_stream_digest(&self) -> &Digest {
        &self.ordered_stream_digest
    }

    pub fn new_frontier_ordinal(&self) -> Ordinal {
        self.new_frontier_ordinal
    }

    /// The canonical finalized-history digest after this promotion.
    pub fn canonical_history_digest(&self) -> &Digest {
        &self.canonical_history_digest
    }
}

/// The maximum number of distinct competing semantic-envelope hashes a
/// poisoned slot retains as exposed evidence, and the cap on how many
/// distinct claims it tracks in order to report an exact omitted count.
///
/// These are the scheduler's conflict-evidence discipline
/// ([`crate::scheduler::MAX_CONFLICT_EVIDENCE`]) applied to the timeline:
/// retain the smallest hashes of the whole claim set, report how many
/// distinct claims were omitted, and flag saturation. All three are
/// functions of the claim set, never of arrival order. The two
/// poisoned-state representations cannot drift apart, because since the
/// closure pass they are one implementation ([`BoundedClaimSet`])
/// instantiated at these caps.
pub const MAX_POISON_EVIDENCE: usize = 16;
pub const MAX_POISON_TRACKED_CLAIMS: usize = 256;

/// Order-independent, bounded evidence about which envelopes contested one
/// ordinal's slot.
///
/// Codex counterexample #12: with an evidence-free `Poisoned` unit
/// variant, two slots poisoned by *entirely different* competing envelope
/// pairs produced the same canonical ingress state digest, erasing a real
/// difference from canonical state. Retaining a sorted set restores the
/// distinction without reintroducing arrival-order sensitivity, and
/// changes only the *state* representation: every closed B-01
/// hash/acknowledgement/fence property is untouched, and the per-call
/// [`SlotPoisonRecord`] keeps its per-call fields, because call *results*
/// may legitimately differ per call while canonical *state* may not.
///
/// The set itself is [`BoundedClaimSet`], the one implementation this
/// kernel has of a bounded order-independent contested-claim set; the
/// scheduler's conflict evidence is the same machinery at the same caps.
/// This type remains the public face of a poisoned timeline slot, with
/// its accessors, constants, and canonical encoding unchanged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotPoisonEvidence(BoundedClaimSet<MAX_POISON_EVIDENCE, MAX_POISON_TRACKED_CLAIMS>);

impl SlotPoisonEvidence {
    fn from_pair(a: Digest, b: Digest) -> Self {
        SlotPoisonEvidence(BoundedClaimSet::from_pair(a, b))
    }

    /// Idempotent, commutative insert retaining the smallest hashes, so
    /// the resulting value is a function of the claim set.
    fn insert(&mut self, hash: Digest) {
        self.0.insert(hash);
    }

    /// The retained competing semantic-envelope hashes, ascending.
    pub fn competing_semantic_hashes(&self) -> BTreeSet<Digest> {
        self.0.retained()
    }

    /// How many further distinct claims contested the slot without being
    /// retained as evidence.
    pub fn omitted_distinct(&self) -> u64 {
        self.0.omitted_distinct()
    }

    /// Whether the claim set exceeded [`MAX_POISON_TRACKED_CLAIMS`], in
    /// which case [`omitted_distinct`](Self::omitted_distinct) is a
    /// saturated lower bound.
    pub fn evidence_truncated(&self) -> bool {
        self.0.truncated()
    }

    /// Commits to **every** tracked competing hash, not to the exposed
    /// [`MAX_POISON_EVIDENCE`] projection — see
    /// [`BoundedClaimSet::canonicalize`] for why that distinction is the
    /// whole point of the type.
    ///
    /// This changes only the *state* representation. Finalized history is
    /// untouched — a poisoned slot can never be finalized, so no poison
    /// evidence has ever entered
    /// [`TimelineIngress::canonical_history_digest`].
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.0.canonicalize(enc);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SlotState {
    // Boxed so the `Poisoned` variant doesn't force every slot entry to
    // reserve the full envelope size inline.
    Staged {
        envelope: Box<SemanticCommandEnvelope>,
        semantic_hash: Digest,
    },
    Poisoned(SlotPoisonEvidence),
}

/// The externally observable status of one ordinal's staging slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotStatus {
    Empty,
    Staged,
    Poisoned,
}

/// One command promoted into canonical finalized history, in ascending
/// ordinal order, each behind its own stable command barrier
/// (ADR-0003 §14). It retains the full **semantic** envelope, which is
/// the whole command: there is no admission metadata left to retain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizedCommand {
    pub ordinal: Ordinal,
    pub envelope: SemanticCommandEnvelope,
    pub semantic_envelope_hash: Digest,
}

/// Evidence of an explicit, authorized sequencer epoch handoff/reset used
/// to recover from a poisoned/unrecoverable staging state (ADR-0003 §11).
/// The finalized frontier is frozen and unchanged across a reset; only
/// unfinalized staged state for the old epoch is discarded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpochResetRecord {
    pub reset_index: u64,
    pub old_epoch: TimelineEpoch,
    pub new_epoch: TimelineEpoch,
    pub new_sequencer: SourceId,
    pub frozen_finalized_frontier: Ordinal,
}

/// Rejects an unauthorized or non-monotonic epoch reset/handoff attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EpochResetError {
    /// Only the currently active sequencer may request a reset.
    NotActiveSequencer,
    /// `new_epoch` must be strictly greater than the current epoch;
    /// rollback and reuse of the current/an earlier epoch are rejected.
    EpochNotIncreasing {
        current: TimelineEpoch,
        attempted: TimelineEpoch,
    },
}

impl fmt::Display for EpochResetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EpochResetError::NotActiveSequencer => {
                write!(f, "only the active sequencer may request an epoch handoff")
            }
            EpochResetError::EpochNotIncreasing { current, attempted } => write!(
                f,
                "epoch handoff must strictly increase: current {}, attempted {}",
                current.0, attempted.0
            ),
        }
    }
}

impl std::error::Error for EpochResetError {}

/// The canonical timeline ingress state machine for one profile.
///
/// Holds exactly one active sequencer grant at a time. All mutation goes
/// through [`stage`](Self::stage), [`submit_fence`](Self::submit_fence),
/// and [`reset_epoch`](Self::reset_epoch); there is no other way to
/// affect the frontier or the finalized command log, so arrival order
/// structurally cannot choose canonical history.
#[derive(Debug, Clone)]
pub struct TimelineIngress {
    profile_id: ProfileId,
    timeline_epoch: TimelineEpoch,
    active_sequencer: SourceId,
    window_width: u32,
    frontier_ordinal: Ordinal,
    last_finalized_fence_hash: Digest,
    slots: BTreeMap<u64, SlotState>,
    // In-flight (this epoch, unfinalized) identity registries.
    //
    // These are **derived indexes over the positively staged slots and
    // nothing else**:
    //
    //     staged_command_identity
    //         == { env.command_id -> h(env) | Staged(env) in slots }
    //     staged_source_sequence_identity
    //         == { (env.source_id, env.source_sequence) -> h(env)
    //              | Staged(env) in slots }
    //
    // An envelope's identity claims are registered exactly while that
    // envelope occupies a `Staged` slot, and at no other time: registered
    // on staging into an empty slot, withdrawn when that slot poisons,
    // moved into the permanent registries at fence promotion, and cleared
    // wholesale with `slots` on epoch reset.
    //
    // The invariant is load-bearing, not tidiness. `canonical_state_digest`
    // commits to every staged envelope in full but does not (and must not)
    // hash these maps: hashing arrival-order-sensitive state would make
    // equal claim sets produce unequal digests and regress AT-F1. Keeping
    // them a pure function of the staged slots is what makes equal digests
    // imply equal future admission behavior *structurally* — the state is
    // incapable of violating the invariant rather than audited for it.
    // Before this repair they were neither committed nor derivable: a
    // contested ordinal left the first arrival's claims registered
    // forever, so arrival order silently decided whether a later command
    // reusing a contested identity was admitted.
    staged_command_identity: BTreeMap<CommandId, Digest>,
    staged_source_sequence_identity: BTreeMap<(SourceId, u64), Digest>,
    // Permanent registries covering finalized history; never cleared.
    finalized_command_identity: BTreeMap<CommandId, Digest>,
    finalized_source_sequence_identity: BTreeMap<(SourceId, u64), Digest>,
    last_finalized_source_sequence: BTreeMap<SourceId, u64>,
    finalized_commands: Vec<FinalizedCommand>,
    finalized_fences: Vec<TimelineFence>,
    epoch_resets: Vec<EpochResetRecord>,
}

fn genesis_fence_hash(
    profile_id: &ProfileId,
    epoch: TimelineEpoch,
    starting_frontier: Ordinal,
) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("timeline_genesis");
    profile_id.canonicalize(&mut enc);
    enc.push_u64(epoch.0);
    enc.push_u64(starting_frontier.0);
    enc.finish()
}

/// Computes the canonical `ordered_stream_digest` for a contiguous
/// ordinal range, so a fence's claimed digest can be independently
/// verified rather than trusted. `ordered` pairs each ordinal with its
/// envelope's [`SemanticCommandEnvelope::semantic_hash`], so two fences
/// over behaviorally distinct histories never collide.
pub fn compute_ordered_stream_digest(ordered: &[(Ordinal, Digest)]) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("ordered_stream_digest");
    enc.push_u64(ordered.len() as u64);
    for (ordinal, semantic_hash) in ordered {
        enc.push_u64(ordinal.0);
        enc.push_digest(semantic_hash);
    }
    enc.finish()
}

fn compute_fence_hash(fence: &TimelineFence) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("timeline_fence");
    fence.profile_id.canonicalize(&mut enc);
    enc.push_u64(fence.timeline_epoch.0);
    fence.fence_id.canonicalize(&mut enc);
    enc.push_u64(fence.start_ordinal.0);
    enc.push_u64(fence.end_ordinal.0);
    enc.push_digest(&fence.previous_fence_hash);
    enc.push_digest(&fence.ordered_stream_digest);
    enc.finish()
}

impl TimelineIngress {
    /// Creates a fresh ingress with an initial sequencer grant, starting
    /// at ordinal 0 with a deterministic genesis fence hash.
    ///
    /// This is the **only** production constructor. There is deliberately
    /// no public way to place the finalized frontier anywhere but the
    /// beginning: see [`TimelineIngress::resume_at_frontier`], which is
    /// feature-gated to test builds.
    pub fn new(
        profile_id: ProfileId,
        timeline_epoch: TimelineEpoch,
        active_sequencer: SourceId,
        window_width: u32,
    ) -> Result<Self, TimelineConfigError> {
        Self::at_frontier(
            profile_id,
            timeline_epoch,
            active_sequencer,
            window_width,
            Ordinal(0),
        )
    }

    /// Creates an ingress whose finalized frontier already sits at
    /// `starting_frontier`.
    ///
    /// # This is not a reconstruction constructor
    ///
    /// It exists **only** so that ordinal-space-exhaustion properties near
    /// `u64::MAX` are falsifiable at all, and it is gated behind
    /// `#[cfg(any(test, feature = "test-support"))]` so it does not exist
    /// on the production surface. The workspace feature-hygiene test
    /// asserts that no production dependency edge enables `test-support`.
    ///
    /// The restriction is not cosmetic. As a public API this function was
    /// an *authority* surface that [`TimelineIngress::new`] never exposed:
    /// any caller could name an arbitrary frontier and receive an ingress
    /// with empty finalized history plus a freshly **synthesized** genesis
    /// anchor derived from that arbitrary frontier — no prior fence hash,
    /// no finalized history, no snapshot identity, no artifact binding, no
    /// reconstruction evidence of any kind. That is arbitrary canonical
    /// timeline injection.
    ///
    /// # The Phase-3 constructor this does *not* implement
    ///
    /// A genuine resume constructor belongs to `spark-persistence` in
    /// Phase 3 and consumes *evidence*, not parameters. Its contract is
    /// specified now so the seam is unambiguous, and implementing it is
    /// explicitly out of Phase-1 scope:
    ///
    /// ```text
    /// pub struct TimelineResumeEvidence {
    ///     profile_id: ProfileId,
    ///     timeline_epoch: TimelineEpoch,
    ///     finalized_frontier: Ordinal,
    ///     last_finalized_fence_hash: Digest,  // real chain anchor, never synthesized
    ///     canonical_history_digest: Digest,   // re-verified against restored history
    ///     manifest_content_hash: Digest,      // ADR-0006 exact-artifact binding
    ///     config_revision_hash: Digest,
    ///     behavior_epoch: u64,
    ///     sequencer_grant: SourceId,
    ///     epoch_reset_chain: Vec<EpochResetRecord>,
    /// }
    /// ```
    ///
    /// That constructor validates internal consistency — the fence hash
    /// must match the restored history digest, and the artifact hashes
    /// must resolve within ADR-0006's compatibility envelope — and fails
    /// explicitly otherwise. Crucially it must never *invent* a genesis
    /// hash for a nonzero frontier, which is exactly what makes the
    /// function below unfit to be a production API.
    #[cfg(any(test, feature = "test-support"))]
    pub fn resume_at_frontier(
        profile_id: ProfileId,
        timeline_epoch: TimelineEpoch,
        active_sequencer: SourceId,
        window_width: u32,
        starting_frontier: Ordinal,
    ) -> Result<Self, TimelineConfigError> {
        Self::at_frontier(
            profile_id,
            timeline_epoch,
            active_sequencer,
            window_width,
            starting_frontier,
        )
    }

    fn at_frontier(
        profile_id: ProfileId,
        timeline_epoch: TimelineEpoch,
        active_sequencer: SourceId,
        window_width: u32,
        starting_frontier: Ordinal,
    ) -> Result<Self, TimelineConfigError> {
        if window_width < 1 {
            return Err(TimelineConfigError::WindowWidthMustBeAtLeastOne);
        }
        let last_finalized_fence_hash =
            genesis_fence_hash(&profile_id, timeline_epoch, starting_frontier);
        Ok(Self {
            profile_id,
            timeline_epoch,
            active_sequencer,
            window_width,
            frontier_ordinal: starting_frontier,
            last_finalized_fence_hash,
            slots: BTreeMap::new(),
            staged_command_identity: BTreeMap::new(),
            staged_source_sequence_identity: BTreeMap::new(),
            finalized_command_identity: BTreeMap::new(),
            finalized_source_sequence_identity: BTreeMap::new(),
            last_finalized_source_sequence: BTreeMap::new(),
            finalized_commands: Vec::new(),
            finalized_fences: Vec::new(),
            epoch_resets: Vec::new(),
        })
    }

    /// The last eligible ordinal of the current window. Total: an ordinal
    /// space that cannot accommodate the configured window is reported,
    /// never panicked on and never wrapped.
    fn window_end(&self) -> Result<Ordinal, TimelineOrdinalSpaceExhausted> {
        // `window_width >= 1` is validated at construction, so the
        // saturating subtraction is exact and its saturating branch is
        // unreachable; it is written this way so the function contains no
        // unchecked arithmetic at all.
        let span = u64::from(self.window_width).saturating_sub(1);
        self.frontier_ordinal
            .0
            .checked_add(span)
            .map(Ordinal)
            .ok_or(TimelineOrdinalSpaceExhausted {
                frontier_ordinal: self.frontier_ordinal,
                window_width: self.window_width,
            })
    }

    /// The current deterministic admission window. A sequencer reads this
    /// to obtain the [`AdmissionTicket`] it must present when staging.
    pub fn current_admission_window(
        &self,
    ) -> Result<AdmissionWindow, TimelineOrdinalSpaceExhausted> {
        let window_end = self.window_end()?;
        let token = compute_window_token(
            &self.profile_id,
            self.timeline_epoch,
            self.frontier_ordinal,
            self.window_width,
            &self.last_finalized_fence_hash,
        );
        Ok(AdmissionWindow {
            profile_id: self.profile_id.clone(),
            timeline_epoch: self.timeline_epoch,
            frontier_ordinal: self.frontier_ordinal,
            window_width: self.window_width,
            window_end,
            last_finalized_fence_hash: self.last_finalized_fence_hash.clone(),
            token,
        })
    }

    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    pub fn timeline_epoch(&self) -> TimelineEpoch {
        self.timeline_epoch
    }

    pub fn active_sequencer(&self) -> &SourceId {
        &self.active_sequencer
    }

    pub fn window_width(&self) -> u32 {
        self.window_width
    }

    pub fn frontier_ordinal(&self) -> Ordinal {
        self.frontier_ordinal
    }

    pub fn last_finalized_fence_hash(&self) -> &Digest {
        &self.last_finalized_fence_hash
    }

    pub fn finalized_commands(&self) -> &[FinalizedCommand] {
        &self.finalized_commands
    }

    pub fn finalized_fences(&self) -> &[TimelineFence] {
        &self.finalized_fences
    }

    pub fn epoch_resets(&self) -> &[EpochResetRecord] {
        &self.epoch_resets
    }

    /// Attempts to stage one submitted command.
    ///
    /// Key properties: (a) an out-of-window ordinal is reported via
    /// `Ok(NotInAdmissionWindow)` without ever touching `self.slots`, so
    /// it cannot evict, poison, or otherwise affect any other ordinal's
    /// slot; (b) duplicate/collision comparison uses the full semantic
    /// identity, not a fragment of the envelope; and (c) the presented
    /// [`AdmissionTicket`] is checked and then dropped — it is never
    /// stored, so admission timing cannot reach canonical history.
    pub fn stage(
        &mut self,
        submitting_sequencer: &SourceId,
        submission: &SubmittedCommand,
    ) -> Result<StageDisposition, StageError> {
        let envelope = &submission.semantic;

        if envelope.profile_id != self.profile_id {
            return Err(StageError::WrongProfile);
        }
        if envelope.timeline_epoch != self.timeline_epoch {
            return Err(StageError::WrongTimelineEpoch);
        }
        if submitting_sequencer != &self.active_sequencer {
            return Err(StageError::NotActiveSequencer);
        }

        // Ordinal-range membership is checked against the *current*
        // window before the ticket is checked at all: an ordinal outside
        // the current numeric range is never admitted regardless of what
        // credential it carries, and this check alone must never mutate
        // `self.slots` (ADR-0003 §5-§6).
        let window_end = self
            .window_end()
            .map_err(StageError::OrdinalSpaceExhausted)?;
        let ordinal = envelope.input_ordinal;
        if ordinal.0 < self.frontier_ordinal.0 || ordinal.0 > window_end.0 {
            return Ok(StageDisposition::NotInAdmissionWindow(
                AdmissionRetryAdvice {
                    profile_id: self.profile_id.clone(),
                    timeline_epoch: self.timeline_epoch,
                    attempted_ordinal: ordinal,
                    frontier_ordinal: self.frontier_ordinal,
                    window_end,
                },
            ));
        }

        let expected_token = compute_window_token(
            &self.profile_id,
            self.timeline_epoch,
            self.frontier_ordinal,
            self.window_width,
            &self.last_finalized_fence_hash,
        );
        if submission.admission.token != expected_token {
            return Err(StageError::StaleOrInvalidAdmissionTicket);
        }

        let semantic_hash = envelope.semantic_hash();

        match self.slots.get(&ordinal.0) {
            None => {
                // A fresh ordinal: reject reuse of a command ID or
                // source-sequence pair by a semantically different
                // envelope, checking both the permanent finalized
                // registries and this epoch's in-flight staging
                // registries.
                if let Some(existing) = self
                    .finalized_command_identity
                    .get(&envelope.command_id)
                    .or_else(|| self.staged_command_identity.get(&envelope.command_id))
                {
                    if *existing != semantic_hash {
                        return Err(StageError::CommandIdentityConflict {
                            command_id: envelope.command_id.clone(),
                        });
                    }
                }
                let seq_key = (envelope.source_id.clone(), envelope.source_sequence);
                if let Some(existing) = self
                    .finalized_source_sequence_identity
                    .get(&seq_key)
                    .or_else(|| self.staged_source_sequence_identity.get(&seq_key))
                {
                    if *existing != semantic_hash {
                        return Err(StageError::SourceSequenceConflict {
                            source_id: envelope.source_id.clone(),
                            source_sequence: envelope.source_sequence,
                        });
                    }
                }

                self.slots.insert(
                    ordinal.0,
                    SlotState::Staged {
                        envelope: Box::new(envelope.clone()),
                        semantic_hash: semantic_hash.clone(),
                    },
                );
                self.staged_command_identity
                    .insert(envelope.command_id.clone(), semantic_hash.clone());
                self.staged_source_sequence_identity
                    .insert(seq_key, semantic_hash.clone());
                Ok(StageDisposition::Acknowledged(StageAcknowledgement::issue(
                    self.profile_id.clone(),
                    self.timeline_epoch,
                    ordinal,
                    envelope.command_id.clone(),
                    semantic_hash,
                    AcknowledgedSlotState::NewlyStaged,
                )))
            }
            Some(SlotState::Poisoned(_)) => {
                // A further claim on an already-poisoned slot is still
                // refused, but it is *recorded*: the evidence set is what
                // makes two differently-contested slots distinguishable in
                // canonical state, and a set insert is idempotent and
                // commutative, so this cannot make state arrival-order
                // sensitive.
                if let Some(SlotState::Poisoned(evidence)) = self.slots.get_mut(&ordinal.0) {
                    evidence.insert(semantic_hash.clone());
                }
                Ok(StageDisposition::Poisoned(SlotPoisonRecord {
                    profile_id: self.profile_id.clone(),
                    timeline_epoch: self.timeline_epoch,
                    input_ordinal: ordinal,
                    rejected_semantic_hash: semantic_hash,
                }))
            }
            Some(SlotState::Staged {
                envelope: staged,
                semantic_hash: existing,
            }) => {
                if *existing == semantic_hash {
                    Ok(StageDisposition::Acknowledged(StageAcknowledgement::issue(
                        self.profile_id.clone(),
                        self.timeline_epoch,
                        ordinal,
                        envelope.command_id.clone(),
                        semantic_hash,
                        AcknowledgedSlotState::AlreadyStagedIdempotent,
                    )))
                } else {
                    // The slot poisons, so the envelope that held it stops
                    // being staged and can never be finalized in this
                    // epoch: its identity claims are withdrawn with it.
                    // The incoming claimant is *not* registered — it was
                    // never admitted to positive staging either.
                    //
                    // Safe-removal lemma (admission architecture §3.2):
                    // the entry under `staged.command_id` was created by
                    // this very envelope, because empty-slot screening
                    // admits at most one semantically *distinct* staged
                    // envelope per command ID, and a semantically
                    // identical envelope has an equal semantic hash —
                    // which covers `input_ordinal` — so it would be this
                    // same slot. The removal therefore cannot evict
                    // another staged envelope's registration. The same
                    // argument covers the source-sequence key.
                    let displaced_command_id = staged.command_id.clone();
                    let displaced_sequence_key = (staged.source_id.clone(), staged.source_sequence);
                    let evidence =
                        SlotPoisonEvidence::from_pair(existing.clone(), semantic_hash.clone());
                    self.staged_command_identity.remove(&displaced_command_id);
                    self.staged_source_sequence_identity
                        .remove(&displaced_sequence_key);
                    self.slots.insert(ordinal.0, SlotState::Poisoned(evidence));
                    Ok(StageDisposition::Poisoned(SlotPoisonRecord {
                        profile_id: self.profile_id.clone(),
                        timeline_epoch: self.timeline_epoch,
                        input_ordinal: ordinal,
                        rejected_semantic_hash: semantic_hash,
                    }))
                }
            }
        }
    }

    /// The externally observable status of one ordinal's staging slot.
    pub fn slot_status(&self, ordinal: Ordinal) -> SlotStatus {
        match self.slots.get(&ordinal.0) {
            None => SlotStatus::Empty,
            Some(SlotState::Staged { .. }) => SlotStatus::Staged,
            Some(SlotState::Poisoned(_)) => SlotStatus::Poisoned,
        }
    }

    /// The order-independent competing-envelope evidence for a poisoned
    /// ordinal, if that ordinal's slot is poisoned.
    ///
    /// Poisoning is a legitimate, deterministic outcome, and the
    /// blueprint's inspection posture requires it to be *explainable*
    /// rather than merely reported: a caller can see exactly which
    /// semantic identities contested the slot, in a form that does not
    /// depend on the order they arrived in.
    pub fn poison_evidence(&self, ordinal: Ordinal) -> Option<&SlotPoisonEvidence> {
        match self.slots.get(&ordinal.0) {
            Some(SlotState::Poisoned(evidence)) => Some(evidence),
            _ => None,
        }
    }

    /// Returns `true` if `ordinal` currently holds a positive `STAGED`
    /// acknowledgement (unpoisoned) — the protocol precondition a
    /// sequencer must confirm for every ordinal in a fence's range before
    /// submitting that fence (ADR-0003 §7).
    pub fn is_positively_staged(&self, ordinal: Ordinal) -> bool {
        self.slot_status(ordinal) == SlotStatus::Staged
    }

    /// Attempts to submit a finality fence.
    ///
    /// On success the covered range is promoted into finalized history in
    /// ascending ordinal order, the frontier advances to
    /// `end_ordinal + 1`, and already-staged higher ordinals that remain
    /// within the new window keep their slots (ADR-0003 §9). On failure,
    /// **nothing** is mutated — including in the ordinal-space-exhaustion
    /// case, which is checked before any promotion.
    pub fn submit_fence(
        &mut self,
        submitting_sequencer: &SourceId,
        fence: &TimelineFence,
    ) -> Result<FinalizationResult, FenceError> {
        if fence.profile_id != self.profile_id {
            return Err(FenceError::WrongProfile);
        }
        if fence.timeline_epoch != self.timeline_epoch {
            return Err(FenceError::WrongTimelineEpoch);
        }
        if submitting_sequencer != &self.active_sequencer {
            return Err(FenceError::NotActiveSequencer);
        }
        if fence.start_ordinal != self.frontier_ordinal {
            return Err(FenceError::StartNotAtFrontier);
        }
        let window_end = self
            .window_end()
            .map_err(FenceError::OrdinalSpaceExhausted)?;
        if fence.end_ordinal.0 < fence.start_ordinal.0 || fence.end_ordinal.0 > window_end.0 {
            return Err(FenceError::EndOutsideStageableHorizon);
        }
        if fence.previous_fence_hash != self.last_finalized_fence_hash {
            return Err(FenceError::PreviousFenceHashMismatch);
        }

        // The frontier advance is validated here, before any state is
        // touched, so an exhausted ordinal space rejects the fence
        // atomically instead of leaving commands promoted behind a
        // frontier that could not move (ADR-0003 §8.8).
        let new_frontier =
            fence
                .end_ordinal
                .0
                .checked_add(1)
                .ok_or(FenceError::OrdinalSpaceExhausted(
                    TimelineOrdinalSpaceExhausted {
                        frontier_ordinal: self.frontier_ordinal,
                        window_width: self.window_width,
                    },
                ))?;

        let mut ordered: Vec<(Ordinal, SemanticCommandEnvelope, Digest)> = Vec::new();
        for raw in fence.start_ordinal.0..=fence.end_ordinal.0 {
            match self.slots.get(&raw) {
                Some(SlotState::Staged {
                    envelope,
                    semantic_hash,
                }) => {
                    ordered.push((Ordinal(raw), (**envelope).clone(), semantic_hash.clone()));
                }
                Some(SlotState::Poisoned(_)) => {
                    return Err(FenceError::RangeContainsPoisoned {
                        ordinal: Ordinal(raw),
                    })
                }
                None => {
                    return Err(FenceError::RangeNotFullyStaged {
                        ordinal: Ordinal(raw),
                    })
                }
            }
        }

        let digest_input: Vec<(Ordinal, Digest)> = ordered
            .iter()
            .map(|(ord, _, hash)| (*ord, hash.clone()))
            .collect();
        let computed_digest = compute_ordered_stream_digest(&digest_input);
        if computed_digest != fence.ordered_stream_digest {
            return Err(FenceError::DigestMismatch);
        }

        // Validate per-source strictly-increasing finalized sequence
        // against the prior finalized baseline plus a running in-fence
        // view, before mutating any real state. A later command from the
        // same source within this fence must strictly exceed both any
        // previously finalized sequence for that source and any
        // earlier-ordinal command from that source within this same
        // fence; gaps are legal, regression is not.
        let mut running: BTreeMap<SourceId, u64> = BTreeMap::new();
        for (_, envelope, _) in &ordered {
            let prior = running.get(&envelope.source_id).copied().or_else(|| {
                self.last_finalized_source_sequence
                    .get(&envelope.source_id)
                    .copied()
            });
            if let Some(prior) = prior {
                if envelope.source_sequence <= prior {
                    return Err(FenceError::SourceSequenceNotIncreasing {
                        source_id: envelope.source_id.clone(),
                        previously_finalized: prior,
                        attempted: envelope.source_sequence,
                    });
                }
            }
            running.insert(envelope.source_id.clone(), envelope.source_sequence);
        }

        // All checks passed: promote atomically.
        let finalized_command_count = ordered.len() as u64;
        for (ordinal, envelope, semantic_hash) in ordered {
            // The identity claims *move* from the in-flight registries to
            // the permanent ones rather than being copied: the command is
            // no longer staged, so leaving a duplicate entry behind would
            // break the derived-index invariant on
            // `staged_command_identity` and leave two authoritative
            // records of one fact.
            let sequence_key = (envelope.source_id.clone(), envelope.source_sequence);
            self.staged_command_identity.remove(&envelope.command_id);
            self.staged_source_sequence_identity.remove(&sequence_key);
            self.finalized_command_identity
                .insert(envelope.command_id.clone(), semantic_hash.clone());
            self.finalized_source_sequence_identity
                .insert(sequence_key, semantic_hash.clone());
            self.last_finalized_source_sequence
                .insert(envelope.source_id.clone(), envelope.source_sequence);
            self.finalized_commands.push(FinalizedCommand {
                ordinal,
                envelope,
                semantic_envelope_hash: semantic_hash,
            });
        }
        let fence_hash = compute_fence_hash(fence);
        let previous_fence_hash =
            std::mem::replace(&mut self.last_finalized_fence_hash, fence_hash.clone());
        self.finalized_fences.push(fence.clone());
        self.frontier_ordinal = Ordinal(new_frontier);

        // Promoted ordinals are exactly `[old_frontier, new_frontier)`, so
        // requiring `ord >= new_frontier` already drops every promoted
        // slot; only an unfinalized staged tail above the new frontier
        // survives.
        //
        // An upper bound is not needed, because no slot can ever exceed
        // the *new* window end. Proof: `stage` only ever inserts at an
        // ordinal within the window open at that moment, the window is
        // `[frontier, frontier + width - 1]` for a fixed `width`, and the
        // frontier is monotonically nondecreasing — so every slot is at or
        // below the current window end at all times, and this promotion
        // strictly *raises* the frontier, hence the window end. The old
        // `*ord <= new_window_end` conjunct was therefore always true for
        // every slot that survived the lower bound. (It was also
        // ill-defined in the exhaustion case, where `window_end()` fails
        // and no upper bound is computable at all; retaining the staged
        // tail is correct there too, since nothing has been finalized past
        // it.)
        self.slots.retain(|ord, _| *ord >= new_frontier);

        Ok(FinalizationResult {
            profile_id: self.profile_id.clone(),
            timeline_epoch: self.timeline_epoch,
            fence_id: fence.fence_id.clone(),
            start_ordinal: fence.start_ordinal,
            end_ordinal: fence.end_ordinal,
            finalized_command_count,
            previous_fence_hash,
            fence_hash,
            ordered_stream_digest: fence.ordered_stream_digest.clone(),
            new_frontier_ordinal: self.frontier_ordinal,
            canonical_history_digest: self.canonical_history_digest(),
        })
    }

    /// Explicit, authorized sequencer epoch handoff/reset (ADR-0003 §11),
    /// the only supported recovery from a poisoned/unrecoverable staging
    /// state. Only the currently active sequencer may request it, and
    /// `new_epoch` must be strictly later than the current epoch. The
    /// finalized frontier, finalized history, and permanent identity
    /// registries are unchanged; all unfinalized staged state and this
    /// epoch's in-flight identity registries are discarded, and a new
    /// sequencer is granted for the new epoch.
    pub fn reset_epoch(
        &mut self,
        requesting_sequencer: &SourceId,
        new_epoch: TimelineEpoch,
        new_sequencer: SourceId,
    ) -> Result<EpochResetRecord, EpochResetError> {
        if requesting_sequencer != &self.active_sequencer {
            return Err(EpochResetError::NotActiveSequencer);
        }
        if new_epoch.0 <= self.timeline_epoch.0 {
            return Err(EpochResetError::EpochNotIncreasing {
                current: self.timeline_epoch,
                attempted: new_epoch,
            });
        }

        let record = EpochResetRecord {
            reset_index: self.epoch_resets.len() as u64,
            old_epoch: self.timeline_epoch,
            new_epoch,
            new_sequencer: new_sequencer.clone(),
            frozen_finalized_frontier: self.frontier_ordinal,
        };
        self.slots.clear();
        self.staged_command_identity.clear();
        self.staged_source_sequence_identity.clear();
        self.timeline_epoch = new_epoch;
        self.active_sequencer = new_sequencer;
        self.epoch_resets.push(record.clone());
        Ok(record)
    }

    /// The canonical digest of **finalized semantic history**: the
    /// profile, the finalized frontier and fence-chain anchor, every
    /// finalized command's full semantic envelope, every finalized
    /// fence's body, and every authorized epoch handoff.
    ///
    /// This is the value that must be equal whenever two runs finalized
    /// the same commands behind the same fence chain, regardless of when
    /// each command happened to be staged relative to a frontier advance.
    /// It contains no admission credential, because no admission
    /// credential is retained anywhere in this module.
    ///
    /// It deliberately excludes the *current* epoch, active sequencer,
    /// window width, and staged tail: those describe the live ingress
    /// configuration and in-flight state, not finalized history. Two runs
    /// that finalized identical history under different window widths
    /// agree here and differ in [`canonical_state_digest`](Self::canonical_state_digest).
    pub fn canonical_history_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("timeline_finalized_history");
        self.profile_id.canonicalize(&mut enc);
        enc.push_u64(self.frontier_ordinal.0);
        enc.push_digest(&self.last_finalized_fence_hash);

        enc.push_u64(self.finalized_commands.len() as u64);
        for fc in &self.finalized_commands {
            let mut inner = CanonicalEncoder::new();
            inner.push_u64(fc.ordinal.0);
            fc.envelope.canonicalize(&mut inner);
            inner.push_digest(&fc.semantic_envelope_hash);
            enc.push_block(&inner);
        }

        enc.push_u64(self.finalized_fences.len() as u64);
        for fence in &self.finalized_fences {
            let mut inner = CanonicalEncoder::new();
            fence.profile_id.canonicalize(&mut inner);
            inner.push_u64(fence.timeline_epoch.0);
            fence.fence_id.canonicalize(&mut inner);
            inner.push_u64(fence.start_ordinal.0);
            inner.push_u64(fence.end_ordinal.0);
            inner.push_digest(&fence.previous_fence_hash);
            inner.push_digest(&fence.ordered_stream_digest);
            enc.push_block(&inner);
        }

        enc.push_u64(self.epoch_resets.len() as u64);
        for reset in &self.epoch_resets {
            let mut inner = CanonicalEncoder::new();
            inner.push_u64(reset.reset_index);
            inner.push_u64(reset.old_epoch.0);
            inner.push_u64(reset.new_epoch.0);
            reset.new_sequencer.canonicalize(&mut inner);
            inner.push_u64(reset.frozen_finalized_frontier.0);
            enc.push_block(&inner);
        }

        enc.finish()
    }

    /// The canonical digest of the ingress's **complete** externally
    /// visible state: [`canonical_history_digest`](Self::canonical_history_digest)
    /// plus the live epoch/sequencer/window configuration and every
    /// staged or poisoned slot.
    ///
    /// Staged slots are hashed as their semantic envelopes, so an empty
    /// scenario and a scenario with one successfully staged unfinalized
    /// command cannot collide — while remaining independent of the
    /// admission credential under which that command was staged. Poisoned
    /// slots hash **every** competing hash they still track, not the
    /// [`MAX_POISON_EVIDENCE`] presentation projection, so two slots
    /// contested by different claim sets never collide even when their
    /// exposed evidence is identical.
    pub fn canonical_state_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("timeline_ingress_state");
        enc.push_digest(&self.canonical_history_digest());
        enc.push_u64(self.timeline_epoch.0);
        self.active_sequencer.canonicalize(&mut enc);
        enc.push_u32(self.window_width);

        enc.push_u64(self.slots.len() as u64);
        for (ordinal, slot) in &self.slots {
            let mut inner = CanonicalEncoder::new();
            inner.push_u64(*ordinal);
            match slot {
                SlotState::Staged {
                    envelope,
                    semantic_hash,
                } => {
                    inner.push_str("staged");
                    envelope.canonicalize(&mut inner);
                    inner.push_digest(semantic_hash);
                }
                SlotState::Poisoned(evidence) => {
                    inner.push_str("poisoned");
                    evidence.canonicalize(&mut inner);
                }
            }
            enc.push_block(&inner);
        }

        enc.finish()
    }
}

/// **AT-H3 / AT-H6** — the closure corpus assertions that are statements
/// about the ingress's *private* staged identity registries rather than
/// about its public surface, and so are expressed here rather than in
/// `spark-testkit`'s `final_closure.rs` (the final closure test matrix
/// sanctions either form).
///
/// The invariant under test is the whole of the B-01 repair:
///
/// ```text
/// staged_command_identity         == { env.command_id -> h(env)                | Staged(env) in slots }
/// staged_source_sequence_identity == { (env.source_id, env.source_sequence) -> h(env) | Staged(env) in slots }
/// ```
///
/// Everything else about the repair follows from it, which is why it is
/// asserted after *every prefix* of a scripted operation sequence rather
/// than only at the end.
#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
mod final_closure_registry_invariant {
    use super::*;
    use crate::hash::hash_bytes;

    fn profile() -> ProfileId {
        ProfileId::new("game-world").unwrap()
    }

    fn sequencer() -> SourceId {
        SourceId::new("sequencer.primary").unwrap()
    }

    fn env(
        command: &str,
        epoch: u64,
        ordinal: u64,
        source_sequence: u64,
        payload: &str,
    ) -> SemanticCommandEnvelope {
        SemanticCommandEnvelope {
            command_id: CommandId::new(command).unwrap(),
            profile_id: profile(),
            timeline_epoch: TimelineEpoch(epoch),
            effective_time: LogicalTime(0),
            source_id: sequencer(),
            source_sequence,
            input_ordinal: Ordinal(ordinal),
            command_kind: CommandKind::new(CanonicalTag::new("test.command").unwrap()),
            canonical_payload_hash: hash_bytes(payload.as_bytes()),
        }
    }

    fn stage(
        ingress: &mut TimelineIngress,
        envelope: &SemanticCommandEnvelope,
    ) -> Result<StageDisposition, StageError> {
        let ticket = ingress
            .current_admission_window()
            .expect("the admission window is open")
            .ticket();
        ingress.stage(&sequencer(), &envelope.clone().submit_with(ticket))
    }

    fn fence(
        ingress: &TimelineIngress,
        tag: &str,
        start: u64,
        end: u64,
        ordered: &[SemanticCommandEnvelope],
    ) -> TimelineFence {
        let pairs: Vec<(Ordinal, Digest)> = ordered
            .iter()
            .map(|e| (e.input_ordinal, e.semantic_hash()))
            .collect();
        TimelineFence {
            profile_id: ingress.profile_id().clone(),
            timeline_epoch: ingress.timeline_epoch(),
            fence_id: FenceId::new(tag).unwrap(),
            start_ordinal: Ordinal(start),
            end_ordinal: Ordinal(end),
            previous_fence_hash: ingress.last_finalized_fence_hash().clone(),
            ordered_stream_digest: compute_ordered_stream_digest(&pairs),
        }
    }

    /// The two registries recomputed from scratch as a pure index over
    /// the currently `Staged` slots — the definition the repair makes
    /// true.
    #[allow(clippy::type_complexity)]
    fn derived_index(
        ingress: &TimelineIngress,
    ) -> (
        BTreeMap<CommandId, Digest>,
        BTreeMap<(SourceId, u64), Digest>,
    ) {
        let mut commands = BTreeMap::new();
        let mut sequences = BTreeMap::new();
        for slot in ingress.slots.values() {
            if let SlotState::Staged {
                envelope,
                semantic_hash,
            } = slot
            {
                commands.insert(envelope.command_id.clone(), semantic_hash.clone());
                sequences.insert(
                    (envelope.source_id.clone(), envelope.source_sequence),
                    semantic_hash.clone(),
                );
            }
        }
        (commands, sequences)
    }

    fn assert_derived_index(ingress: &TimelineIngress, step: &str) {
        let (commands, sequences) = derived_index(ingress);
        assert_eq!(
            ingress.staged_command_identity, commands,
            "after `{step}`: the staged command-ID registry is not a derived \
             index over the positively staged slots"
        );
        assert_eq!(
            ingress.staged_source_sequence_identity, sequences,
            "after `{step}`: the staged source-sequence registry is not a \
             derived index over the positively staged slots"
        );
    }

    /// **AT-H3** — after every prefix of a scripted sequence containing
    /// stages, a contest, poison pile-ons, an identity-conflicting
    /// attempt, a fence promotion, and an epoch reset, both staged
    /// registries equal the index recomputed from the current `Staged`
    /// slots.
    ///
    /// Architecture §3.2's safe-removal lemma is what makes the poisoning
    /// removals sound: an entry under key `c` was created by the envelope
    /// that staged into this slot, and screening guarantees at most one
    /// semantically *distinct* staged envelope per key; a semantically
    /// identical envelope has an equal semantic hash, hence an equal
    /// `input_ordinal`, hence is this very slot. So the entry being
    /// removed can never belong to a different staged envelope.
    #[test]
    fn staged_registries_are_a_derived_index_over_staged_slots() {
        let mut ingress =
            TimelineIngress::new(profile(), TimelineEpoch(1), sequencer(), 4).unwrap();
        assert_derived_index(&ingress, "construction");

        let e0 = env("cmd.0", 1, 0, 0, "zero");
        stage(&mut ingress, &e0).unwrap();
        assert_derived_index(&ingress, "stage ordinal 0");

        let e1 = env("cmd.1", 1, 1, 1, "one");
        stage(&mut ingress, &e1).unwrap();
        assert_derived_index(&ingress, "stage ordinal 1");

        // Idempotent restage: no registry action at all.
        stage(&mut ingress, &e1).unwrap();
        assert_derived_index(&ingress, "idempotent restage of ordinal 1");

        // An identity-conflicting attempt at an empty ordinal leaves no
        // trace: no slot, no registration.
        let conflicting = env("cmd.1", 1, 2, 21, "conflicting");
        assert!(matches!(
            stage(&mut ingress, &conflicting),
            Err(StageError::CommandIdentityConflict { .. })
        ));
        assert_derived_index(&ingress, "rejected identity-conflicting stage");

        // The contest: ordinal 1 poisons, and the formerly staged
        // envelope's two claims must be withdrawn.
        let contestant = env("cmd.1-rival", 1, 1, 11, "rival");
        assert!(matches!(
            stage(&mut ingress, &contestant),
            Ok(StageDisposition::Poisoned(_))
        ));
        assert_derived_index(&ingress, "contest poisons ordinal 1");
        assert!(!ingress.staged_command_identity.contains_key(&e1.command_id));
        assert!(!ingress
            .staged_command_identity
            .contains_key(&contestant.command_id));

        // Pile-on claims on the poisoned slot are evidence only.
        for (index, payload) in ["pile-a", "pile-b"].iter().enumerate() {
            let pile = env(
                &format!("cmd.pile.{index}"),
                1,
                1,
                31 + index as u64,
                payload,
            );
            assert!(matches!(
                stage(&mut ingress, &pile),
                Ok(StageDisposition::Poisoned(_))
            ));
            assert_derived_index(&ingress, "pile-on claim on the poisoned slot");
        }

        // A contested identity is free again, at a fresh ordinal.
        let reclaim = env("cmd.1", 1, 2, 12, "reclaimed");
        stage(&mut ingress, &reclaim).unwrap();
        assert_derived_index(&ingress, "reuse of a contested command ID");

        // Fence promotion moves ordinal 0's claims into the finalized
        // registries; the staged registries must not keep duplicates.
        let promotion = fence(&ingress, "fence.0", 0, 0, std::slice::from_ref(&e0));
        ingress.submit_fence(&sequencer(), &promotion).unwrap();
        assert_derived_index(&ingress, "fence promotes ordinal 0");
        assert!(!ingress.staged_command_identity.contains_key(&e0.command_id));
        assert!(ingress
            .finalized_command_identity
            .contains_key(&e0.command_id));

        // A stage after the window slid.
        let e4 = env("cmd.4", 1, 4, 4, "four");
        stage(&mut ingress, &e4).unwrap();
        assert_derived_index(&ingress, "stage after the window slid");

        // Epoch reset clears slots and both staged registries together.
        ingress
            .reset_epoch(&sequencer(), TimelineEpoch(2), sequencer())
            .unwrap();
        assert_derived_index(&ingress, "epoch reset");
        assert!(ingress.staged_command_identity.is_empty());
        assert!(ingress.staged_source_sequence_identity.is_empty());

        let after_reset = env("cmd.after", 2, 1, 40, "after");
        stage(&mut ingress, &after_reset).unwrap();
        assert_derived_index(&ingress, "stage after the epoch reset");
    }

    /// **AT-H6** — fence promotion *moves* identity claims from the
    /// staged registries into the finalized ones (S1), and finalized
    /// identity remains permanently reserved across an epoch reset.
    #[test]
    fn finalization_moves_identity_from_staged_to_finalized_registries() {
        let mut ingress =
            TimelineIngress::new(profile(), TimelineEpoch(1), sequencer(), 4).unwrap();
        let e0 = env("cmd.0", 1, 0, 7, "zero");
        stage(&mut ingress, &e0).unwrap();
        assert!(ingress.staged_command_identity.contains_key(&e0.command_id));

        let promotion = fence(&ingress, "fence.0", 0, 0, std::slice::from_ref(&e0));
        ingress.submit_fence(&sequencer(), &promotion).unwrap();

        // (b) The staged registries no longer carry the promoted entries.
        assert!(!ingress.staged_command_identity.contains_key(&e0.command_id));
        assert!(!ingress
            .staged_source_sequence_identity
            .contains_key(&(e0.source_id.clone(), e0.source_sequence)));
        assert_derived_index(&ingress, "fence promotion");

        // (a) ...because the permanent registries took over: reuse of a
        // finalized command ID or source-sequence pair by a semantically
        // different envelope is still rejected.
        assert_eq!(
            stage(&mut ingress, &env("cmd.0", 1, 1, 8, "different")),
            Err(StageError::CommandIdentityConflict {
                command_id: CommandId::new("cmd.0").unwrap()
            })
        );
        assert_eq!(
            stage(&mut ingress, &env("cmd.other", 1, 1, 7, "different")),
            Err(StageError::SourceSequenceConflict {
                source_id: sequencer(),
                source_sequence: 7,
            })
        );

        // (c) An epoch reset voids unfinalized state only; finalized
        // identity stays reserved forever.
        ingress
            .reset_epoch(&sequencer(), TimelineEpoch(2), sequencer())
            .unwrap();
        assert_eq!(
            stage(&mut ingress, &env("cmd.0", 2, 1, 9, "different-again")),
            Err(StageError::CommandIdentityConflict {
                command_id: CommandId::new("cmd.0").unwrap()
            })
        );
        assert_eq!(
            stage(&mut ingress, &env("cmd.other", 2, 1, 7, "different-again")),
            Err(StageError::SourceSequenceConflict {
                source_id: sequencer(),
                source_sequence: 7,
            })
        );
    }
}
