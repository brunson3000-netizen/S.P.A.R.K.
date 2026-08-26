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
//!    within finalized-and-staged history.
//! 4. **Finalized per-source sequence numbers strictly increase**; gaps
//!    are legal, regression is not.
//! 5. **All ordinal arithmetic is total.** Nothing here panics: window and
//!    frontier arithmetic returns a typed error, and a fence that cannot
//!    advance the frontier without overflow is rejected *before* anything
//!    is promoted.

use crate::clock::LogicalTime;
use crate::hash::{CanonicalEncoder, Digest};
use crate::id::{CanonicalTag, CommandId, FenceId, ProfileId, SourceId};
use std::collections::BTreeMap;
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum SlotState {
    // Boxed so `Poisoned`'s zero-sized variant doesn't force every slot
    // entry to reserve the full envelope size inline.
    Staged {
        envelope: Box<SemanticCommandEnvelope>,
        semantic_hash: Digest,
    },
    Poisoned,
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
    // In-flight (this epoch, unfinalized) identity registries; cleared on
    // epoch reset along with `slots`.
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
    pub fn new(
        profile_id: ProfileId,
        timeline_epoch: TimelineEpoch,
        active_sequencer: SourceId,
        window_width: u32,
    ) -> Result<Self, TimelineConfigError> {
        Self::resume_at_frontier(
            profile_id,
            timeline_epoch,
            active_sequencer,
            window_width,
            Ordinal(0),
        )
    }

    /// Creates an ingress whose finalized frontier already sits at
    /// `starting_frontier`, as a save continuation would
    /// (ADR-0006 "logical simulation time and last committed canonical
    /// input ordinal"). No persistence backend is implied — this is only
    /// the canonical constructor that makes a non-zero starting frontier
    /// expressible.
    ///
    /// A starting frontier near `u64::MAX` is accepted deliberately: the
    /// resulting ordinal-space exhaustion must be a *reportable* condition
    /// rather than an unreachable panic, and making it constructible is
    /// what allows that property to be tested at all.
    pub fn resume_at_frontier(
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
        self.frontier_ordinal
            .0
            .checked_add(u64::from(self.window_width) - 1)
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
            Some(SlotState::Poisoned) => Ok(StageDisposition::Poisoned(SlotPoisonRecord {
                profile_id: self.profile_id.clone(),
                timeline_epoch: self.timeline_epoch,
                input_ordinal: ordinal,
                rejected_semantic_hash: semantic_hash,
            })),
            Some(SlotState::Staged {
                semantic_hash: existing,
                ..
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
                    self.slots.insert(ordinal.0, SlotState::Poisoned);
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
            Some(SlotState::Poisoned) => SlotStatus::Poisoned,
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
                Some(SlotState::Poisoned) => {
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
            self.finalized_command_identity
                .insert(envelope.command_id.clone(), semantic_hash.clone());
            self.finalized_source_sequence_identity.insert(
                (envelope.source_id.clone(), envelope.source_sequence),
                semantic_hash.clone(),
            );
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
        // (within the new window) survives. If the new window cannot be
        // computed at all, the ordinal space is exhausted and no slot can
        // remain eligible, so retaining nothing is the correct behavior.
        let new_window_end = self.window_end().map(|o| o.0).unwrap_or(u64::MAX);
        self.slots
            .retain(|ord, _| *ord >= new_frontier && *ord <= new_window_end);

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
    /// admission credential under which that command was staged.
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
                SlotState::Poisoned => {
                    inner.push_str("poisoned");
                }
            }
            enc.push_block(&inner);
        }

        enc.finish()
    }
}
