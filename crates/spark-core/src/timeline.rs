//! Canonical timeline ingress: sequencer authority, deterministic
//! ordinal-credit admission window, one-slot-per-ordinal staging,
//! full-semantic-envelope identity, contiguous digest-linked finality
//! fences, and authorized epoch handoff/reset.
//!
//! This module is a direct implementation of ADR-0003
//! (`engineering/phase0/ADR-0003-deterministic-clock-rng-and-commit-order.md`),
//! corrected per `PHASE_1_CORRECTION_BRIEF_v0.1.md` §2 (B-01). Its central
//! properties, restated from the ADR and the correction brief:
//!
//! 1. **Capacity is allocated by ordinal position, not arrival.** At
//!    frontier `n` with admission window width `W`, every ordinal in
//!    `[n, n+W-1]` owns a distinct logical staging slot before any command
//!    for it has arrived. A higher ordinal arriving first can never
//!    consume the frontier ordinal's slot, and a command outside the
//!    tokenized window it was built against is never opportunistically
//!    admitted merely because the frontier later moved to include its
//!    numeric ordinal.
//! 2. **Identity is the full semantic envelope, not a fragment of it.**
//!    Two envelopes that agree on ordinal/command-ID/payload-hash but
//!    disagree on `effective_time`, `source_id`, `source_sequence`, or
//!    `command_kind` are behaviorally distinct and must never be treated
//!    as the same duplicate. [`semantic_envelope_hash`] is the single
//!    function that decides "same command" for staging idempotency,
//!    command-ID uniqueness, and source-sequence uniqueness alike.
//! 3. **Command IDs and `(source_id, source_sequence)` pairs are unique**
//!    within this timeline's finalized-and-staged history: reusing either
//!    for a different semantic envelope is a rejected conflict, not a
//!    silent overwrite or a fresh slot.
//! 4. **Finalized per-source sequence numbers strictly increase.** Gaps
//!    are legal (a source may skip sequence numbers); going backward for
//!    the same source is not.
//! 5. **Epoch handoff is an authorized, monotonic transition**, not a
//!    public unauthenticated mutation: only the currently active
//!    sequencer may request it, and the new epoch must be strictly later
//!    than the old one. The finalized frontier and finalized history are
//!    frozen and unchanged across a reset; only unfinalized staging (and
//!    this epoch's in-flight identity registries) for the old epoch is
//!    discarded.

use crate::clock::LogicalTime;
use crate::hash::{CanonicalEncoder, Digest};
use crate::id::{CommandId, FenceId, ProfileId, SourceId};
use std::collections::BTreeMap;

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

/// The deterministic admission-window token bound to one specific
/// `(profile, epoch, frontier, window_width, last_finalized_fence_hash)`
/// tuple (ADR-0003 §3). An envelope's token must match the window it was
/// built against; a stale token from a since-superseded window is
/// rejected even if the envelope's ordinal now numerically falls inside
/// the current window. The token is admission metadata only: it never
/// participates in [`semantic_envelope_hash`], so a retry that reuses a
/// newer token for an otherwise-identical envelope does not become a
/// behaviorally different command.
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
/// (ADR-0003 §3). Callers use this to construct envelopes with a valid
/// token; the token cannot be forged from ordinal/time alone because it
/// is a hash of the full window identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionWindow {
    pub profile_id: ProfileId,
    pub timeline_epoch: TimelineEpoch,
    pub frontier_ordinal: Ordinal,
    pub window_width: u32,
    pub window_end: Ordinal,
    pub last_finalized_fence_hash: Digest,
    pub token: AdmissionWindowToken,
}

/// One canonical, state-changing/evaluation command envelope
/// (ADR-0003 §2, correction brief B-01). `command_kind`/payload semantics
/// belong to Phase 2's rule runtime; Phase 1 treats the payload as an
/// opaque, already-hashed blob so the ingress skeleton can be fully
/// exercised without a rule language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandEnvelope {
    pub command_id: CommandId,
    pub profile_id: ProfileId,
    pub timeline_epoch: TimelineEpoch,
    pub effective_time: LogicalTime,
    pub source_id: SourceId,
    pub source_sequence: u64,
    pub input_ordinal: Ordinal,
    pub admission_window_token: AdmissionWindowToken,
    pub command_kind: String,
    pub canonical_payload_hash: Digest,
}

impl CommandEnvelope {
    /// Encodes every field, including the admission-window token, for use
    /// in evidence/state digests. This is distinct from
    /// [`semantic_envelope_hash`], which deliberately excludes the token
    /// because it is admission metadata, not behavioral identity.
    fn canonicalize_full(&self, enc: &mut CanonicalEncoder) {
        self.command_id.canonicalize(enc);
        self.profile_id.canonicalize(enc);
        enc.push_u64(self.timeline_epoch.0);
        self.effective_time.canonicalize(enc);
        self.source_id.canonicalize(enc);
        enc.push_u64(self.source_sequence);
        enc.push_u64(self.input_ordinal.0);
        enc.push_digest(&self.admission_window_token.0);
        enc.push_str(&self.command_kind);
        enc.push_digest(&self.canonical_payload_hash);
    }
}

/// The full behavioral identity of one command envelope: every field that
/// distinguishes one canonical command from another, **excluding** the
/// admission-window token (admission metadata, not behavior — see
/// [`AdmissionWindowToken`]'s doc comment). This is the single function
/// that decides "same command" everywhere identity matters: staging
/// idempotency/collision, command-ID uniqueness, and source-sequence
/// uniqueness (correction brief B-01 requirement 1-4).
pub fn semantic_envelope_hash(envelope: &CommandEnvelope) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("semantic_envelope_identity");
    envelope.profile_id.canonicalize(&mut enc);
    enc.push_u64(envelope.timeline_epoch.0);
    envelope.effective_time.canonicalize(&mut enc);
    envelope.source_id.canonicalize(&mut enc);
    enc.push_u64(envelope.source_sequence);
    enc.push_u64(envelope.input_ordinal.0);
    envelope.command_id.canonicalize(&mut enc);
    enc.push_str(&envelope.command_kind);
    enc.push_digest(&envelope.canonical_payload_hash);
    enc.finish()
}

/// The logical result of one `stage` attempt (ADR-0003 §4-§6). These are
/// not error conditions except where noted: `NotInAdmissionWindow` is an
/// expected, retryable outcome that consumes no capacity, not a protocol
/// violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageOutcome {
    /// The ordinal's slot was empty and is now occupied by this command.
    Staged,
    /// A semantically identical envelope was already staged for this
    /// ordinal; idempotent, no state change.
    AlreadyStagedIdempotent,
    /// A semantically conflicting envelope was already staged (or
    /// previously poisoned) for this ordinal; the slot is now poisoned.
    Poisoned,
    /// `input_ordinal` is outside `[frontier_ordinal, window_end]` under
    /// the *current* window. No slot was touched. The sequencer may
    /// retry once it holds a token for a window that covers this
    /// ordinal.
    NotInAdmissionWindow,
}

/// A protocol-level rejection of a `stage` attempt: the envelope was
/// malformed/misauthorized/identity-conflicting for this ingress, as
/// opposed to a legitimate logical outcome like
/// [`StageOutcome::NotInAdmissionWindow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageError {
    WrongProfile,
    WrongTimelineEpoch,
    NotActiveSequencer,
    /// The ordinal is within the current window's numeric range, but the
    /// envelope's token does not match the current window token (e.g. a
    /// stale token from before the frontier last advanced).
    StaleOrInvalidAdmissionWindowToken,
    /// `command_id` was already used (staged or finalized) by a
    /// semantically different envelope. Exact reuse (same semantic
    /// identity) is not an error; it reaches this path only when the
    /// content differs.
    CommandIdentityConflict {
        command_id: CommandId,
    },
    /// `(source_id, source_sequence)` was already used (staged or
    /// finalized) by a semantically different envelope.
    SourceSequenceConflict {
        source_id: SourceId,
        source_sequence: u64,
    },
    /// Internal ordinal/window arithmetic would overflow `u64`.
    OrdinalArithmeticOverflow,
}

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
    RangeNotFullyStaged,
    /// Some ordinal in the range is poisoned (ADR-0003 §8.4).
    RangeContainsPoisoned,
    /// `ordered_stream_digest` does not match the ordered canonical
    /// semantic envelope identities (ADR-0003 §8.5).
    DigestMismatch,
    /// `previous_fence_hash` does not match the finalized chain
    /// (ADR-0003 §8.6).
    PreviousFenceHashMismatch,
    /// Finalizing this range would make some source's finalized
    /// `source_sequence` go backward relative to that source's
    /// previously finalized sequence (correction brief B-01 requirement
    /// 5). Gaps are legal; regression is not.
    SourceSequenceNotIncreasing {
        source_id: SourceId,
        previously_finalized: u64,
        attempted: u64,
    },
    /// Internal ordinal arithmetic would overflow `u64`.
    OrdinalArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SlotState {
    // Boxed so `Poisoned`'s zero-sized variant doesn't force every slot
    // entry (most of which are transient/small) to reserve the full
    // `CommandEnvelope` inline size.
    Staged {
        envelope: Box<CommandEnvelope>,
        semantic_hash: Digest,
    },
    Poisoned,
}

/// The externally observable status of one ordinal's staging slot
/// (`Empty` is never distinguished from "out of window" by this type;
/// callers combine it with [`AdmissionWindow`] range membership when that
/// distinction matters).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotStatus {
    Empty,
    Staged,
    Poisoned,
}

/// One command promoted into canonical finalized history, in ascending
/// ordinal order, each behind its own stable command barrier
/// (ADR-0003 §14). Retains the full envelope so canonical history can be
/// reconstructed/hashed, not merely a fragment of it (correction brief
/// B-01 requirement 2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizedCommand {
    pub ordinal: Ordinal,
    pub envelope: CommandEnvelope,
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

/// Rejects an unauthorized or non-monotonic epoch reset/handoff attempt
/// (correction brief B-01 requirement 9).
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

/// The canonical timeline ingress state machine for one profile.
///
/// Holds exactly one active sequencer grant at a time. All mutation goes
/// through [`stage`](Self::stage), [`submit_fence`](Self::submit_fence),
/// and [`reset_epoch`](Self::reset_epoch); there is no other way to
/// affect `frontier_ordinal` or the finalized command log, so arrival
/// order structurally cannot choose canonical history.
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

fn genesis_fence_hash(profile_id: &ProfileId, epoch: TimelineEpoch) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("timeline_genesis");
    profile_id.canonicalize(&mut enc);
    enc.push_u64(epoch.0);
    enc.finish()
}

/// Computes the canonical `ordered_stream_digest` for a contiguous
/// ordinal range of staged commands, so a fence's claimed digest can be
/// independently verified rather than trusted. `ordered` pairs each
/// ordinal with its envelope's [`semantic_envelope_hash`] (not merely its
/// payload hash), so two fences over behaviorally distinct histories
/// never collide (correction brief B-01 requirement 3).
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
    /// Creates a fresh ingress with an initial sequencer grant. The
    /// finalized frontier starts at ordinal 0 with a deterministic
    /// genesis fence hash derived from `(profile_id, timeline_epoch)`.
    pub fn new(
        profile_id: ProfileId,
        timeline_epoch: TimelineEpoch,
        active_sequencer: SourceId,
        window_width: u32,
    ) -> Self {
        assert!(window_width >= 1, "admission window width must be >= 1");
        let last_finalized_fence_hash = genesis_fence_hash(&profile_id, timeline_epoch);
        Self {
            profile_id,
            timeline_epoch,
            active_sequencer,
            window_width,
            frontier_ordinal: Ordinal(0),
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
        }
    }

    /// Ordinal arithmetic here is guarded by `checked_add`/`checked_sub`
    /// rather than left to silently wrap: `window_width >= 1` is an
    /// established constructor invariant and `frontier_ordinal` only ever
    /// grows by finalized ranges, so overflow here would indicate an
    /// internal invariant violation rather than an attacker- or
    /// profile-data-controllable condition (correction brief M-03).
    fn window_end(&self) -> Ordinal {
        Ordinal(
            self.frontier_ordinal
                .0
                .checked_add(self.window_width as u64)
                .and_then(|v| v.checked_sub(1))
                .expect("timeline window_end arithmetic overflow"),
        )
    }

    /// The current deterministic admission window. Callers (a sequencer
    /// implementation, or a fixture) read this to construct envelopes
    /// with a valid token.
    pub fn current_admission_window(&self) -> AdmissionWindow {
        let window_end = self.window_end();
        let token = compute_window_token(
            &self.profile_id,
            self.timeline_epoch,
            self.frontier_ordinal,
            self.window_width,
            &self.last_finalized_fence_hash,
        );
        AdmissionWindow {
            profile_id: self.profile_id.clone(),
            timeline_epoch: self.timeline_epoch,
            frontier_ordinal: self.frontier_ordinal,
            window_width: self.window_width,
            window_end,
            last_finalized_fence_hash: self.last_finalized_fence_hash.clone(),
            token,
        }
    }

    pub fn frontier_ordinal(&self) -> Ordinal {
        self.frontier_ordinal
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

    /// Attempts to stage one command envelope. See [`StageOutcome`] and
    /// [`StageError`] for the full decision table; the key properties are
    /// that (a) an out-of-window ordinal is reported via
    /// `Ok(NotInAdmissionWindow)` without ever touching `self.slots`, so
    /// it cannot evict, poison, or otherwise affect any other ordinal's
    /// slot, and (b) duplicate/collision comparison uses the full
    /// [`semantic_envelope_hash`], not a fragment of the envelope.
    pub fn stage(
        &mut self,
        submitting_sequencer: &SourceId,
        envelope: &CommandEnvelope,
    ) -> Result<StageOutcome, StageError> {
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
        // window before the token is checked at all: an ordinal outside
        // the current numeric range is never admitted regardless of what
        // token it carries, and this check alone must never mutate
        // `self.slots` (ADR-0003 §5-§6).
        let ordinal = envelope.input_ordinal;
        let window_end = self.window_end();
        if ordinal.0 < self.frontier_ordinal.0 || ordinal.0 > window_end.0 {
            return Ok(StageOutcome::NotInAdmissionWindow);
        }

        let expected_token = compute_window_token(
            &self.profile_id,
            self.timeline_epoch,
            self.frontier_ordinal,
            self.window_width,
            &self.last_finalized_fence_hash,
        );
        if envelope.admission_window_token != expected_token {
            return Err(StageError::StaleOrInvalidAdmissionWindowToken);
        }

        let semantic_hash = semantic_envelope_hash(envelope);

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
                    .insert(seq_key, semantic_hash);
                Ok(StageOutcome::Staged)
            }
            Some(SlotState::Poisoned) => Ok(StageOutcome::Poisoned),
            Some(SlotState::Staged {
                semantic_hash: existing,
                ..
            }) => {
                if *existing == semantic_hash {
                    Ok(StageOutcome::AlreadyStagedIdempotent)
                } else {
                    self.slots.insert(ordinal.0, SlotState::Poisoned);
                    Ok(StageOutcome::Poisoned)
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
    /// acknowledgement (unpoisoned), the protocol precondition a
    /// sequencer must confirm for every ordinal in a fence's range before
    /// submitting that fence (ADR-0003 §7).
    pub fn is_positively_staged(&self, ordinal: Ordinal) -> bool {
        self.slot_status(ordinal) == SlotStatus::Staged
    }

    /// Attempts to submit a finality fence. See [`FenceError`] for the
    /// full validation table. On success, the covered range is promoted
    /// into finalized history in ascending ordinal order, the frontier
    /// advances to `end_ordinal + 1`, a new window token is implied for
    /// the new frontier, and already-staged higher ordinals that remain
    /// within the new window keep their slots (ADR-0003 §9). On failure,
    /// nothing is mutated.
    pub fn submit_fence(
        &mut self,
        submitting_sequencer: &SourceId,
        fence: &TimelineFence,
    ) -> Result<(), FenceError> {
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
        let window_end = self.window_end();
        if fence.end_ordinal.0 < fence.start_ordinal.0 || fence.end_ordinal.0 > window_end.0 {
            return Err(FenceError::EndOutsideStageableHorizon);
        }
        if fence.previous_fence_hash != self.last_finalized_fence_hash {
            return Err(FenceError::PreviousFenceHashMismatch);
        }

        let mut ordered: Vec<(Ordinal, CommandEnvelope, Digest)> = Vec::new();
        for raw in fence.start_ordinal.0..=fence.end_ordinal.0 {
            match self.slots.get(&raw) {
                Some(SlotState::Staged {
                    envelope,
                    semantic_hash,
                }) => {
                    ordered.push((Ordinal(raw), (**envelope).clone(), semantic_hash.clone()));
                }
                Some(SlotState::Poisoned) => return Err(FenceError::RangeContainsPoisoned),
                None => return Err(FenceError::RangeNotFullyStaged),
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
        // atomically, against the prior finalized baseline plus a running
        // in-fence view, before mutating any real state (correction brief
        // B-01 requirement 4). A later command from the same source
        // within this fence must strictly exceed both any previously
        // finalized sequence for that source and any earlier-ordinal
        // command from that source within this same fence; gaps are
        // legal, regression is not.
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
        self.last_finalized_fence_hash = compute_fence_hash(fence);
        self.finalized_fences.push(fence.clone());
        self.frontier_ordinal = Ordinal(
            fence
                .end_ordinal
                .0
                .checked_add(1)
                .ok_or(FenceError::OrdinalArithmeticOverflow)?,
        );

        // Promoted ordinals are exactly `[old_frontier, new_frontier)`,
        // so requiring `ord >= new_frontier` already drops every promoted
        // slot; only an unfinalized staged tail above the new frontier
        // (within the new window) survives. In-flight identity
        // registrations for promoted ordinals were already copied into
        // the permanent finalized registries above; leaving their
        // staging-registry entries in place is harmless (they still
        // agree with the finalized hash).
        let new_window_end = self.window_end().0;
        let new_frontier = self.frontier_ordinal.0;
        self.slots
            .retain(|ord, _| *ord >= new_frontier && *ord <= new_window_end);

        Ok(())
    }

    /// Explicit, authorized sequencer epoch handoff/reset (ADR-0003 §11,
    /// correction brief B-01 requirement 9), the only supported recovery
    /// from a poisoned/unrecoverable staging state. Only the currently
    /// active sequencer may request it, and `new_epoch` must be strictly
    /// later than the current epoch. The finalized frontier, finalized
    /// history, and permanent identity registries are unchanged; all
    /// unfinalized staged state and this epoch's in-flight identity
    /// registries are discarded, and a new sequencer is granted for the
    /// new epoch.
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

    /// A deterministic canonical hash of the ingress's complete externally
    /// visible state: profile/epoch/active-sequencer/window identity, the
    /// frontier and last finalized fence hash, every staged/poisoned slot
    /// (not only finalized ones), every finalized command's full
    /// envelope, every finalized fence's full body, and every epoch reset
    /// record. This is a strengthened replacement for the Phase-1 writer
    /// pass's digest, which the independent review found omitted enough
    /// state that an empty scenario and a scenario with one successfully
    /// staged unfinalized command hashed identically (correction brief
    /// M-02); this digest cannot exhibit that collision because staged
    /// slots are hashed directly.
    pub fn canonical_state_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("timeline_ingress_state");
        self.profile_id.canonicalize(&mut enc);
        enc.push_u64(self.timeline_epoch.0);
        self.active_sequencer.canonicalize(&mut enc);
        enc.push_u32(self.window_width);
        enc.push_u64(self.frontier_ordinal.0);
        enc.push_digest(&self.last_finalized_fence_hash);

        enc.push_u64(self.slots.len() as u64);
        for (ordinal, slot) in &self.slots {
            enc.push_u64(*ordinal);
            match slot {
                SlotState::Staged {
                    envelope,
                    semantic_hash,
                } => {
                    enc.push_str("staged");
                    envelope.canonicalize_full(&mut enc);
                    enc.push_digest(semantic_hash);
                }
                SlotState::Poisoned => {
                    enc.push_str("poisoned");
                }
            }
        }

        enc.push_u64(self.finalized_commands.len() as u64);
        for fc in &self.finalized_commands {
            enc.push_u64(fc.ordinal.0);
            fc.envelope.canonicalize_full(&mut enc);
            enc.push_digest(&fc.semantic_envelope_hash);
        }

        enc.push_u64(self.finalized_fences.len() as u64);
        for fence in &self.finalized_fences {
            fence.profile_id.canonicalize(&mut enc);
            enc.push_u64(fence.timeline_epoch.0);
            fence.fence_id.canonicalize(&mut enc);
            enc.push_u64(fence.start_ordinal.0);
            enc.push_u64(fence.end_ordinal.0);
            enc.push_digest(&fence.previous_fence_hash);
            enc.push_digest(&fence.ordered_stream_digest);
        }

        enc.push_u64(self.epoch_resets.len() as u64);
        for reset in &self.epoch_resets {
            enc.push_u64(reset.reset_index);
            enc.push_u64(reset.old_epoch.0);
            enc.push_u64(reset.new_epoch.0);
            reset.new_sequencer.canonicalize(&mut enc);
            enc.push_u64(reset.frozen_finalized_frontier.0);
        }

        enc.finish()
    }
}
