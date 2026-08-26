//! Canonical timeline ingress: sequencer authority, deterministic
//! ordinal-credit admission window, one-slot-per-ordinal staging,
//! contiguous digest-linked finality fences, and epoch handoff/reset.
//!
//! This module is a direct implementation of ADR-0003
//! (`engineering/phase0/ADR-0003-deterministic-clock-rng-and-commit-order.md`).
//! Its central property, restated from the ADR: **capacity is allocated
//! by ordinal position, not arrival.** At frontier `n` with admission
//! window width `W`, every ordinal in `[n, n+W-1]` owns a distinct
//! logical staging slot before any command for it has arrived. A higher
//! ordinal arriving first can never consume the frontier ordinal's slot,
//! and a command outside the tokenized window it was built against is
//! never opportunistically admitted merely because the frontier later
//! moved to include its numeric ordinal - the token, not just the
//! ordinal, must match the window under which the sequencer is currently
//! authorized to submit.

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
/// created only through explicit [`TimelineIngress::reset_epoch`]
/// handoff (ADR-0003 §11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimelineEpoch(pub u64);

/// The deterministic admission-window token bound to one specific
/// `(profile, epoch, frontier, window_width, last_finalized_fence_hash)`
/// tuple (ADR-0003 §3). An envelope's token must match the window it was
/// built against; a stale token from a since-superseded window is
/// rejected even if the envelope's ordinal now numerically falls inside
/// the current window.
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
/// (ADR-0003 §2). `command_kind`/payload semantics belong to Phase 2's
/// rule runtime; Phase 1 treats the payload as an opaque, already-hashed
/// blob so the ingress skeleton can be fully exercised without a rule
/// language.
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

/// The logical result of one `stage` attempt (ADR-0003 §4-§6). These are
/// not error conditions except where noted: `NotInAdmissionWindow` is an
/// expected, retryable outcome that consumes no capacity, not a protocol
/// violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageOutcome {
    /// The ordinal's slot was empty and is now occupied by this command.
    Staged,
    /// An identical `(command_id, canonical_payload_hash)` was already
    /// staged for this ordinal; idempotent, no state change.
    AlreadyStagedIdempotent,
    /// A conflicting payload was already staged (or previously
    /// poisoned) for this ordinal; the slot is now poisoned.
    Poisoned,
    /// `input_ordinal` is outside `[frontier_ordinal, window_end]` under
    /// the *current* window. No slot was touched. The sequencer may
    /// retry once it holds a token for a window that covers this
    /// ordinal.
    NotInAdmissionWindow,
}

/// A protocol-level rejection of a `stage` attempt: the envelope was
/// malformed/misauthorized for this ingress, as opposed to a legitimate
/// logical outcome like [`StageOutcome::NotInAdmissionWindow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageError {
    WrongProfile,
    WrongTimelineEpoch,
    NotActiveSequencer,
    /// The ordinal is within the current window's numeric range, but the
    /// envelope's token does not match the current window token (e.g. a
    /// stale token from before the frontier last advanced).
    StaleOrInvalidAdmissionWindowToken,
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
    /// envelopes/payloads (ADR-0003 §8.5).
    DigestMismatch,
    /// `previous_fence_hash` does not match the finalized chain
    /// (ADR-0003 §8.6).
    PreviousFenceHashMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SlotState {
    Staged {
        command_id: CommandId,
        canonical_payload_hash: Digest,
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
/// (ADR-0003 §14).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizedCommand {
    pub ordinal: Ordinal,
    pub command_id: CommandId,
    pub canonical_payload_hash: Digest,
}

/// Evidence of an explicit sequencer epoch handoff/reset used to recover
/// from a poisoned/unrecoverable staging state (ADR-0003 §11). The
/// finalized frontier is frozen and unchanged across a reset; only
/// unfinalized staged state for the old epoch is discarded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpochResetRecord {
    pub reset_index: u64,
    pub old_epoch: TimelineEpoch,
    pub new_epoch: TimelineEpoch,
    pub frozen_finalized_frontier: Ordinal,
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
/// independently verified rather than trusted.
pub fn compute_ordered_stream_digest(ordered: &[(Ordinal, CommandId, Digest)]) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("ordered_stream_digest");
    enc.push_u64(ordered.len() as u64);
    for (ordinal, command_id, payload_hash) in ordered {
        enc.push_u64(ordinal.0);
        command_id.canonicalize(&mut enc);
        enc.push_digest(payload_hash);
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
            finalized_commands: Vec::new(),
            finalized_fences: Vec::new(),
            epoch_resets: Vec::new(),
        }
    }

    fn window_end(&self) -> Ordinal {
        Ordinal(self.frontier_ordinal.0 + self.window_width as u64 - 1)
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
    /// [`StageError`] for the full decision table; the key property is
    /// that an out-of-window ordinal is reported via `Ok(NotInAdmissionWindow)`
    /// without ever touching `self.slots`, so it cannot evict, poison, or
    /// otherwise affect any other ordinal's slot.
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
        if ordinal.0 < self.frontier_ordinal.0 || ordinal.0 > self.window_end().0 {
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

        let outcome = match self.slots.get(&ordinal.0) {
            None => {
                self.slots.insert(
                    ordinal.0,
                    SlotState::Staged {
                        command_id: envelope.command_id.clone(),
                        canonical_payload_hash: envelope.canonical_payload_hash.clone(),
                    },
                );
                StageOutcome::Staged
            }
            Some(SlotState::Poisoned) => StageOutcome::Poisoned,
            Some(SlotState::Staged {
                command_id,
                canonical_payload_hash,
            }) => {
                if *command_id == envelope.command_id
                    && *canonical_payload_hash == envelope.canonical_payload_hash
                {
                    StageOutcome::AlreadyStagedIdempotent
                } else {
                    self.slots.insert(ordinal.0, SlotState::Poisoned);
                    StageOutcome::Poisoned
                }
            }
        };
        Ok(outcome)
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
        if fence.end_ordinal.0 < fence.start_ordinal.0 || fence.end_ordinal.0 > self.window_end().0
        {
            return Err(FenceError::EndOutsideStageableHorizon);
        }
        if fence.previous_fence_hash != self.last_finalized_fence_hash {
            return Err(FenceError::PreviousFenceHashMismatch);
        }

        let mut ordered: Vec<(Ordinal, CommandId, Digest)> = Vec::new();
        for raw in fence.start_ordinal.0..=fence.end_ordinal.0 {
            match self.slots.get(&raw) {
                Some(SlotState::Staged {
                    command_id,
                    canonical_payload_hash,
                }) => {
                    ordered.push((
                        Ordinal(raw),
                        command_id.clone(),
                        canonical_payload_hash.clone(),
                    ));
                }
                Some(SlotState::Poisoned) => return Err(FenceError::RangeContainsPoisoned),
                None => return Err(FenceError::RangeNotFullyStaged),
            }
        }

        let computed_digest = compute_ordered_stream_digest(&ordered);
        if computed_digest != fence.ordered_stream_digest {
            return Err(FenceError::DigestMismatch);
        }

        // All checks passed: promote atomically.
        for (ordinal, command_id, canonical_payload_hash) in ordered {
            self.finalized_commands.push(FinalizedCommand {
                ordinal,
                command_id,
                canonical_payload_hash,
            });
        }
        self.last_finalized_fence_hash = compute_fence_hash(fence);
        self.finalized_fences.push(fence.clone());
        self.frontier_ordinal = Ordinal(fence.end_ordinal.0 + 1);

        let new_window_end = self.window_end().0;
        let new_frontier = self.frontier_ordinal.0;
        self.slots
            .retain(|ord, _| *ord >= new_frontier && *ord <= new_window_end);

        Ok(())
    }

    /// Explicit sequencer epoch handoff/reset (ADR-0003 §11), the only
    /// supported recovery from a poisoned/unrecoverable staging state.
    /// The finalized frontier and finalized history are unchanged; all
    /// unfinalized staged state for the old epoch is discarded and a new
    /// sequencer is granted for the new epoch.
    pub fn reset_epoch(
        &mut self,
        new_epoch: TimelineEpoch,
        new_sequencer: SourceId,
    ) -> EpochResetRecord {
        let record = EpochResetRecord {
            reset_index: self.epoch_resets.len() as u64,
            old_epoch: self.timeline_epoch,
            new_epoch,
            frozen_finalized_frontier: self.frontier_ordinal,
        };
        self.slots.clear();
        self.timeline_epoch = new_epoch;
        self.active_sequencer = new_sequencer;
        self.epoch_resets.push(record.clone());
        record
    }

    /// A deterministic canonical hash of the ingress's externally visible
    /// state (frontier, last finalized fence hash, finalized command log,
    /// finalized fence count, epoch reset count). Used by
    /// `spark-testkit` to prove identical replay across repeated runs of
    /// the same scenario (Phase-1 test corpus item 19).
    pub fn canonical_state_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("timeline_ingress_state");
        self.profile_id.canonicalize(&mut enc);
        enc.push_u64(self.timeline_epoch.0);
        enc.push_u64(self.frontier_ordinal.0);
        enc.push_digest(&self.last_finalized_fence_hash);
        enc.push_u64(self.finalized_commands.len() as u64);
        for fc in &self.finalized_commands {
            enc.push_u64(fc.ordinal.0);
            fc.command_id.canonicalize(&mut enc);
            enc.push_digest(&fc.canonical_payload_hash);
        }
        enc.push_u64(self.finalized_fences.len() as u64);
        enc.push_u64(self.epoch_resets.len() as u64);
        enc.finish()
    }
}
