//! Phase-1 minimum test corpus items 1-10
//! (`engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md` §5), plus the
//! full-semantic-envelope-identity, source-sequence, and
//! epoch-authorization falsification tests.
//!
//! Ported to the re-founded ingress API
//! (`PHASE_1_REFOUNDATION_BRIEF_v0.1.md`) without weakening any assertion:
//! every property the inherited corpus proved is still proved here, now
//! against structured [`StageDisposition`]/[`FinalizationResult`]
//! evidence and an admission-free [`SemanticCommandEnvelope`].

use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, Digest};
use spark_core::id::{CanonicalTag, CommandId, FenceId, ProfileId, SourceId};
use spark_core::timeline::{
    compute_ordered_stream_digest, AcknowledgedSlotState, AdmissionTicket, CommandKind,
    EpochResetError, FenceError, Ordinal, SemanticCommandEnvelope, SlotStatus, StageDisposition,
    StageError, TimelineEpoch, TimelineFence, TimelineIngress,
};

fn profile() -> ProfileId {
    ProfileId::new("game-world").unwrap()
}

fn epoch() -> TimelineEpoch {
    TimelineEpoch(1)
}

fn sequencer() -> SourceId {
    SourceId::new("sequencer.primary").unwrap()
}

fn impostor_sequencer() -> SourceId {
    SourceId::new("sequencer.impostor").unwrap()
}

fn payload_hash(tag: &str) -> Digest {
    hash_bytes(tag.as_bytes())
}

fn command_kind(tag: &str) -> CommandKind {
    CommandKind::new(CanonicalTag::new(tag).unwrap())
}

fn fresh_ingress(width: u32) -> TimelineIngress {
    TimelineIngress::new(profile(), epoch(), sequencer(), width).unwrap()
}

/// Builds a semantic envelope for `ordinal`, tagging the command/payload
/// identity so distinct calls can be made to collide deliberately.
///
/// Unlike the inherited helper, this needs no `&TimelineIngress`: a
/// semantic envelope carries no admission credential, so it does not
/// depend on any window state at all.
fn envelope(ordinal: u64, command_tag: &str, payload_tag: &str) -> SemanticCommandEnvelope {
    SemanticCommandEnvelope {
        command_id: CommandId::new(command_tag).unwrap(),
        profile_id: profile(),
        timeline_epoch: epoch(),
        effective_time: LogicalTime(ordinal),
        source_id: sequencer(),
        source_sequence: ordinal,
        input_ordinal: Ordinal(ordinal),
        command_kind: command_kind("test.command"),
        canonical_payload_hash: payload_hash(payload_tag),
    }
}

fn ticket(ingress: &TimelineIngress) -> AdmissionTicket {
    ingress.current_admission_window().unwrap().ticket()
}

/// Stages `envelope` under the ingress's current admission window.
fn stage_now(
    ingress: &mut TimelineIngress,
    submitter: &SourceId,
    envelope: &SemanticCommandEnvelope,
) -> Result<StageDisposition, StageError> {
    let ticket = ticket(ingress);
    ingress.stage(submitter, &envelope.clone().submit_with(ticket))
}

fn assert_newly_staged(disposition: &StageDisposition) {
    let ack = disposition
        .acknowledgement()
        .unwrap_or_else(|| panic!("expected a positive acknowledgement, got {disposition:?}"));
    assert_eq!(ack.slot_state(), AcknowledgedSlotState::NewlyStaged);
}

fn fence_for(
    ingress: &TimelineIngress,
    start: u64,
    end: u64,
    fence_tag: &str,
    ordered_envelopes: &[SemanticCommandEnvelope],
) -> TimelineFence {
    let ordered: Vec<(Ordinal, Digest)> = ordered_envelopes
        .iter()
        .map(|e| (e.input_ordinal, e.semantic_hash()))
        .collect();
    TimelineFence {
        profile_id: ingress.profile_id().clone(),
        timeline_epoch: ingress.timeline_epoch(),
        fence_id: FenceId::new(fence_tag).unwrap(),
        start_ordinal: Ordinal(start),
        end_ordinal: Ordinal(end),
        previous_fence_hash: ingress.last_finalized_fence_hash().clone(),
        ordered_stream_digest: compute_ordered_stream_digest(&ordered),
    }
}

/// Item 1: `n+1,n+2,n` versus `n,n+1,n+2` with width 2 produce identical
/// per-ordinal stage dispositions, and the same fence through `n+1`
/// succeeds identically in both delivery orders.
#[test]
fn reversed_delivery_within_capacity_two_window_is_order_independent() {
    let e_n = envelope(0, "cmd.n", "payload.n");
    let e_n1 = envelope(1, "cmd.n1", "payload.n1");
    let e_n2 = envelope(2, "cmd.n2", "payload.n2");

    // Delivery order: n+1, n+2, n.
    let mut reversed = fresh_ingress(2);
    let out_n1_first = stage_now(&mut reversed, &sequencer(), &e_n1).expect("protocol-valid stage");
    let out_n2_first = stage_now(&mut reversed, &sequencer(), &e_n2).expect("protocol-valid stage");
    let out_n_first = stage_now(&mut reversed, &sequencer(), &e_n).expect("protocol-valid stage");

    assert_newly_staged(&out_n_first);
    assert_newly_staged(&out_n1_first);
    assert!(matches!(
        out_n2_first,
        StageDisposition::NotInAdmissionWindow(_)
    ));

    // Delivery order: n, n+1, n+2.
    let mut forward = fresh_ingress(2);
    let out_n_second = stage_now(&mut forward, &sequencer(), &e_n).expect("protocol-valid stage");
    let out_n1_second = stage_now(&mut forward, &sequencer(), &e_n1).expect("protocol-valid stage");
    let out_n2_second = stage_now(&mut forward, &sequencer(), &e_n2).expect("protocol-valid stage");

    // The structured evidence itself is identical, not merely its coarse
    // tag: same acknowledgement bindings, same retry advice.
    assert_eq!(out_n_second, out_n_first);
    assert_eq!(out_n1_second, out_n1_first);
    assert_eq!(out_n2_second, out_n2_first);

    // The same fence through n+1 must succeed identically in both,
    // producing byte-identical canonical state.
    let fence_a = fence_for(
        &reversed,
        0,
        1,
        "fence.shared",
        &[e_n.clone(), e_n1.clone()],
    );
    let fence_b = fence_for(&forward, 0, 1, "fence.shared", &[e_n, e_n1]);

    let result_a = reversed.submit_fence(&sequencer(), &fence_a).unwrap();
    let result_b = forward.submit_fence(&sequencer(), &fence_b).unwrap();

    assert_eq!(result_a, result_b);
    assert_eq!(reversed.frontier_ordinal(), forward.frontier_ordinal());
    assert_eq!(reversed.finalized_commands(), forward.finalized_commands());
    assert_eq!(
        reversed.canonical_history_digest(),
        forward.canonical_history_digest()
    );
    assert_eq!(
        reversed.canonical_state_digest(),
        forward.canonical_state_digest()
    );
}

/// Item 2: a stale (pre-frontier-advance) admission ticket for `n+2` is
/// rejected even though `n+2` is numerically inside the new window; an
/// explicit retry using the freshly issued ticket succeeds.
#[test]
fn stale_ticket_after_frontier_advance_is_rejected_then_succeeds_with_fresh_ticket() {
    let mut ingress = fresh_ingress(2);

    let e0 = envelope(0, "cmd.0", "payload.0");
    let e1 = envelope(1, "cmd.1", "payload.1");
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();
    stage_now(&mut ingress, &sequencer(), &e1).unwrap();

    // Capture the (soon to be stale) admission ticket before finalizing.
    let stale_ticket = ticket(&ingress);

    let fence = fence_for(&ingress, 0, 1, "fence.1", &[e0, e1]);
    ingress.submit_fence(&sequencer(), &fence).unwrap();
    assert_eq!(ingress.frontier_ordinal(), Ordinal(2));

    // Ordinal 2 is now numerically inside the window [2,3], but this
    // submission carries the stale pre-finalization credential.
    let e2 = envelope(2, "cmd.2", "payload.2");
    let result = ingress.stage(&sequencer(), &e2.clone().submit_with(stale_ticket));
    assert_eq!(result, Err(StageError::StaleOrInvalidAdmissionTicket));
    assert_eq!(ingress.slot_status(Ordinal(2)), SlotStatus::Empty);

    // Retry with the freshly issued ticket succeeds -- and, because the
    // credential is not part of identity, the retried command is the same
    // command, not a behaviorally different one.
    let retry = stage_now(&mut ingress, &sequencer(), &e2).unwrap();
    assert_newly_staged(&retry);
    assert!(retry.acknowledgement().unwrap().covers(&e2));
}

/// Item 3: an exact duplicate (byte-identical semantic envelope) for an
/// already staged ordinal is idempotent.
#[test]
fn exact_duplicate_is_idempotent() {
    let mut ingress = fresh_ingress(2);
    let e = envelope(0, "cmd.a", "payload.a");
    assert_newly_staged(&stage_now(&mut ingress, &sequencer(), &e).unwrap());

    let duplicate = stage_now(&mut ingress, &sequencer(), &e).unwrap();
    assert_eq!(
        duplicate.acknowledgement().unwrap().slot_state(),
        AcknowledgedSlotState::AlreadyStagedIdempotent
    );
    assert_eq!(ingress.slot_status(Ordinal(0)), SlotStatus::Staged);
}

/// Item 4: a distinct payload for an already-staged ordinal poisons it,
/// identically regardless of which payload arrived first.
#[test]
fn distinct_payload_collision_poisons_identically_in_opposite_orders() {
    let a = envelope(0, "cmd.a", "payload.a");
    let b = envelope(0, "cmd.b", "payload.b");

    let mut order_a = fresh_ingress(2);
    stage_now(&mut order_a, &sequencer(), &a).unwrap();
    let second_a = stage_now(&mut order_a, &sequencer(), &b).unwrap();

    let mut order_b = fresh_ingress(2);
    stage_now(&mut order_b, &sequencer(), &b).unwrap();
    let second_b = stage_now(&mut order_b, &sequencer(), &a).unwrap();

    assert!(matches!(second_a, StageDisposition::Poisoned(_)));
    assert!(matches!(second_b, StageDisposition::Poisoned(_)));
    assert_eq!(order_a.slot_status(Ordinal(0)), SlotStatus::Poisoned);
    assert_eq!(order_b.slot_status(Ordinal(0)), SlotStatus::Poisoned);
    // Neither arrival order was allowed to pick a winner: neither
    // envelope is finalizable from a poisoned slot.
    let fence_a = fence_for(&order_a, 0, 0, "fence.poisoned", std::slice::from_ref(&a));
    assert!(matches!(
        order_a.submit_fence(&sequencer(), &fence_a),
        Err(FenceError::RangeContainsPoisoned { .. })
    ));
}

/// Item 5: a missing ordinal in the fence range blocks finalization
/// atomically (nothing is promoted, frontier unchanged).
#[test]
fn missing_ordinal_blocks_fence_atomically() {
    let mut ingress = fresh_ingress(2);
    let e0 = envelope(0, "cmd.0", "payload.0");
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();
    // Ordinal 1 is deliberately left unstaged.

    let fence = fence_for(&ingress, 0, 1, "fence.missing", std::slice::from_ref(&e0));
    let result = ingress.submit_fence(&sequencer(), &fence);

    assert_eq!(
        result,
        Err(FenceError::RangeNotFullyStaged {
            ordinal: Ordinal(1)
        })
    );
    assert_eq!(ingress.frontier_ordinal(), Ordinal(0));
    assert!(ingress.finalized_commands().is_empty());
}

/// Item 6: a fence whose claimed `ordered_stream_digest` does not match
/// the actual staged content is rejected.
#[test]
fn wrong_fence_digest_rejects() {
    let mut ingress = fresh_ingress(2);
    stage_now(
        &mut ingress,
        &sequencer(),
        &envelope(0, "cmd.0", "payload.0"),
    )
    .unwrap();
    stage_now(
        &mut ingress,
        &sequencer(),
        &envelope(1, "cmd.1", "payload.1"),
    )
    .unwrap();

    let bogus_fence = TimelineFence {
        profile_id: profile(),
        timeline_epoch: epoch(),
        fence_id: FenceId::new("fence.bogus").unwrap(),
        start_ordinal: Ordinal(0),
        end_ordinal: Ordinal(1),
        previous_fence_hash: ingress.last_finalized_fence_hash().clone(),
        ordered_stream_digest: hash_bytes(b"not the real digest"),
    };
    let result = ingress.submit_fence(&sequencer(), &bogus_fence);
    assert_eq!(result, Err(FenceError::DigestMismatch));
    assert_eq!(ingress.frontier_ordinal(), Ordinal(0));
}

/// Item 7: a fence whose `previous_fence_hash` does not match the
/// finalized chain is rejected.
#[test]
fn wrong_previous_fence_hash_rejects() {
    let mut ingress = fresh_ingress(2);
    let e0 = envelope(0, "cmd.0", "payload.0");
    let e1 = envelope(1, "cmd.1", "payload.1");
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();
    stage_now(&mut ingress, &sequencer(), &e1).unwrap();

    let ordered: Vec<(Ordinal, Digest)> = [&e0, &e1]
        .iter()
        .map(|e| (e.input_ordinal, e.semantic_hash()))
        .collect();
    let fence = TimelineFence {
        profile_id: profile(),
        timeline_epoch: epoch(),
        fence_id: FenceId::new("fence.wrong_prev").unwrap(),
        start_ordinal: Ordinal(0),
        end_ordinal: Ordinal(1),
        previous_fence_hash: hash_bytes(b"not the genesis hash"),
        ordered_stream_digest: compute_ordered_stream_digest(&ordered),
    };
    let result = ingress.submit_fence(&sequencer(), &fence);
    assert_eq!(result, Err(FenceError::PreviousFenceHashMismatch));
}

/// Item 8: neither staging nor fencing may be performed by any sequencer
/// identity other than the granted active one.
#[test]
fn non_sequencer_stage_and_fence_are_rejected() {
    let mut ingress = fresh_ingress(2);
    let e0 = envelope(0, "cmd.0", "payload.0");
    let result = stage_now(&mut ingress, &impostor_sequencer(), &e0);
    assert_eq!(result, Err(StageError::NotActiveSequencer));

    stage_now(&mut ingress, &sequencer(), &e0).unwrap();
    let e1 = envelope(1, "cmd.1", "payload.1");
    stage_now(&mut ingress, &sequencer(), &e1).unwrap();

    let fence = fence_for(&ingress, 0, 1, "fence.impostor", &[e0, e1]);
    let fence_result = ingress.submit_fence(&impostor_sequencer(), &fence);
    assert_eq!(fence_result, Err(FenceError::NotActiveSequencer));
}

/// Item 9: after an explicit, authorized epoch handoff, the old epoch's
/// sequencer can no longer stage or fence.
#[test]
fn old_epoch_sequencer_rejects_after_handoff() {
    let mut ingress = fresh_ingress(2);
    let old_sequencer = sequencer();
    let new_sequencer = SourceId::new("sequencer.successor").unwrap();

    ingress
        .reset_epoch(&old_sequencer, TimelineEpoch(2), new_sequencer.clone())
        .unwrap();

    // An envelope built under the old epoch is rejected: the epoch no
    // longer matches, whatever credential accompanies it.
    let stale_epoch_envelope = envelope(0, "cmd.stale_epoch", "payload.stale");
    let result = stage_now(&mut ingress, &old_sequencer, &stale_epoch_envelope);
    assert_eq!(result, Err(StageError::WrongTimelineEpoch));

    // The old sequencer is also no longer the active one.
    let mut current_epoch_envelope = envelope(0, "cmd.fresh", "payload.fresh");
    current_epoch_envelope.timeline_epoch = TimelineEpoch(2);
    assert_eq!(
        stage_now(&mut ingress, &old_sequencer, &current_epoch_envelope),
        Err(StageError::NotActiveSequencer)
    );

    // The new sequencer, under the new epoch, works normally.
    let fresh = stage_now(&mut ingress, &new_sequencer, &current_epoch_envelope).unwrap();
    assert_newly_staged(&fresh);
}

/// Item 10: a partial-prefix fence slides the window deterministically,
/// preserving already-staged higher ordinals that remain in the new
/// window and exposing newly reachable ordinals.
#[test]
fn partial_prefix_fence_slides_window_and_preserves_staged_tail() {
    let mut ingress = fresh_ingress(3); // window [0,2]

    let e0 = envelope(0, "cmd.0", "payload.0");
    let e1 = envelope(1, "cmd.1", "payload.1");
    let e2 = envelope(2, "cmd.2", "payload.2");
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();
    stage_now(&mut ingress, &sequencer(), &e1).unwrap();
    stage_now(&mut ingress, &sequencer(), &e2).unwrap();
    assert_eq!(ingress.slot_status(Ordinal(2)), SlotStatus::Staged);

    // Finalize only the prefix [0,1]; ordinal 2 remains staged and
    // un-finalized.
    let fence = fence_for(&ingress, 0, 1, "fence.prefix", &[e0, e1]);
    let result = ingress.submit_fence(&sequencer(), &fence).unwrap();
    assert_eq!(result.new_frontier_ordinal(), Ordinal(2));
    assert_eq!(result.finalized_command_count(), 2);

    assert_eq!(ingress.frontier_ordinal(), Ordinal(2));
    // Window is now [2,4]; ordinal 2's earlier staging survived the slide.
    assert_eq!(ingress.slot_status(Ordinal(2)), SlotStatus::Staged);
    assert_eq!(
        ingress.current_admission_window().unwrap().window_end,
        Ordinal(4)
    );

    // Newly exposed ordinal 3 is stageable under the new window.
    let e3 = envelope(3, "cmd.3", "payload.3");
    assert_newly_staged(&stage_now(&mut ingress, &sequencer(), &e3).unwrap());

    // The retained ordinal-2 staging can still be finalized.
    let fence2 = fence_for(&ingress, 2, 2, "fence.tail", std::slice::from_ref(&e2));
    ingress.submit_fence(&sequencer(), &fence2).unwrap();
    assert_eq!(ingress.frontier_ordinal(), Ordinal(3));
}

// ---------------------------------------------------------------------
// Full semantic-envelope identity falsification.
// ---------------------------------------------------------------------

/// A same-ordinal restage that changes `effective_time` (holding
/// command/payload fixed) must poison, not idempotently match.
#[test]
fn changed_effective_time_is_not_idempotent() {
    let mut ingress = fresh_ingress(2);
    let e0 = envelope(0, "cmd.a", "payload.a");
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();

    let mut retimed = e0.clone();
    retimed.effective_time = LogicalTime(999);
    assert!(matches!(
        stage_now(&mut ingress, &sequencer(), &retimed).unwrap(),
        StageDisposition::Poisoned(_)
    ));
}

/// A same-ordinal restage that changes `source_id` must poison.
#[test]
fn changed_source_id_is_not_idempotent() {
    let mut ingress = fresh_ingress(2);
    let e0 = envelope(0, "cmd.a", "payload.a");
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();

    let mut different_source = e0.clone();
    different_source.source_id = SourceId::new("source.beta").unwrap();
    assert!(matches!(
        stage_now(&mut ingress, &sequencer(), &different_source).unwrap(),
        StageDisposition::Poisoned(_)
    ));
}

/// A same-ordinal restage that changes `source_sequence` must poison.
#[test]
fn changed_source_sequence_is_not_idempotent() {
    let mut ingress = fresh_ingress(2);
    let e0 = envelope(0, "cmd.a", "payload.a");
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();

    let mut different_sequence = e0.clone();
    different_sequence.source_sequence = 999;
    assert!(matches!(
        stage_now(&mut ingress, &sequencer(), &different_sequence).unwrap(),
        StageDisposition::Poisoned(_)
    ));
}

/// A same-ordinal restage that changes `command_kind` must poison.
#[test]
fn changed_command_kind_is_not_idempotent() {
    let mut ingress = fresh_ingress(2);
    let e0 = envelope(0, "cmd.a", "payload.a");
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();

    let mut different_kind = e0.clone();
    different_kind.command_kind = command_kind("different.kind");
    assert!(matches!(
        stage_now(&mut ingress, &sequencer(), &different_kind).unwrap(),
        StageDisposition::Poisoned(_)
    ));
}

/// Reusing a `command_id` for a semantically different envelope at a
/// different (fresh) ordinal is a rejected conflict, not a fresh stage.
#[test]
fn reused_command_id_for_different_envelope_at_different_ordinal_conflicts() {
    let mut ingress = fresh_ingress(3);
    let e0 = envelope(0, "cmd.shared", "payload.first");
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();

    let e1 = envelope(1, "cmd.shared", "payload.second");
    let result = stage_now(&mut ingress, &sequencer(), &e1);
    assert_eq!(
        result,
        Err(StageError::CommandIdentityConflict {
            command_id: CommandId::new("cmd.shared").unwrap()
        })
    );
    // The conflicting attempt touched no slot.
    assert_eq!(ingress.slot_status(Ordinal(1)), SlotStatus::Empty);
}

/// Reusing `(source_id, source_sequence)` for a different command at a
/// different (fresh) ordinal is a rejected conflict.
#[test]
fn reused_source_sequence_for_different_command_conflicts() {
    let mut ingress = fresh_ingress(3);
    let mut e0 = envelope(0, "cmd.a", "payload.a");
    e0.source_id = SourceId::new("source.shared").unwrap();
    e0.source_sequence = 42;
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();

    let mut e1 = envelope(1, "cmd.b", "payload.b");
    e1.source_id = SourceId::new("source.shared").unwrap();
    e1.source_sequence = 42;
    let result = stage_now(&mut ingress, &sequencer(), &e1);
    assert_eq!(
        result,
        Err(StageError::SourceSequenceConflict {
            source_id: SourceId::new("source.shared").unwrap(),
            source_sequence: 42,
        })
    );
    assert_eq!(ingress.slot_status(Ordinal(1)), SlotStatus::Empty);
}

/// A fence whose finalized range would make one source's sequence go
/// backward (a later ordinal carrying an earlier sequence number for the
/// same source) is rejected atomically.
#[test]
fn source_sequence_finalizing_in_descending_order_rejects() {
    let mut ingress = fresh_ingress(2);
    let shared_source = SourceId::new("source.shared").unwrap();

    let mut e0 = envelope(0, "cmd.0", "payload.0");
    e0.source_id = shared_source.clone();
    e0.source_sequence = 5;
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();

    let mut e1 = envelope(1, "cmd.1", "payload.1");
    e1.source_id = shared_source.clone();
    e1.source_sequence = 3; // Regresses relative to ordinal 0's sequence 5.
    stage_now(&mut ingress, &sequencer(), &e1).unwrap();

    let fence = fence_for(&ingress, 0, 1, "fence.regress", &[e0, e1]);
    let result = ingress.submit_fence(&sequencer(), &fence);
    assert_eq!(
        result,
        Err(FenceError::SourceSequenceNotIncreasing {
            source_id: shared_source,
            previously_finalized: 5,
            attempted: 3,
        })
    );
    assert_eq!(ingress.frontier_ordinal(), Ordinal(0));
    assert!(ingress.finalized_commands().is_empty());
}

/// Two different sources may legitimately interleave arbitrary sequence
/// numbers; only same-source regression is rejected.
#[test]
fn different_sources_may_legitimately_interleave() {
    let mut ingress = fresh_ingress(2);

    let mut e0 = envelope(0, "cmd.0", "payload.0");
    e0.source_id = SourceId::new("source.a").unwrap();
    e0.source_sequence = 100;
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();

    let mut e1 = envelope(1, "cmd.1", "payload.1");
    e1.source_id = SourceId::new("source.b").unwrap();
    e1.source_sequence = 1;
    stage_now(&mut ingress, &sequencer(), &e1).unwrap();

    let fence = fence_for(&ingress, 0, 1, "fence.interleave", &[e0, e1]);
    assert!(ingress.submit_fence(&sequencer(), &fence).is_ok());
}

/// Only the currently active sequencer may request an epoch reset.
#[test]
fn unauthorized_epoch_reset_rejects() {
    let mut ingress = fresh_ingress(2);
    let result = ingress.reset_epoch(
        &impostor_sequencer(),
        TimelineEpoch(2),
        SourceId::new("sequencer.successor").unwrap(),
    );
    assert_eq!(result, Err(EpochResetError::NotActiveSequencer));
    // Original epoch/sequencer remain in force.
    let e0 = envelope(0, "cmd.0", "payload.0");
    assert_newly_staged(&stage_now(&mut ingress, &sequencer(), &e0).unwrap());
}

/// An epoch reset to the current epoch or an earlier one is rejected;
/// only a strictly later epoch is a valid transition.
#[test]
fn arbitrary_epoch_rollback_or_reuse_rejects() {
    let mut ingress = fresh_ingress(2);
    let successor = SourceId::new("sequencer.successor").unwrap();

    assert_eq!(
        ingress.reset_epoch(&sequencer(), TimelineEpoch(1), successor.clone()),
        Err(EpochResetError::EpochNotIncreasing {
            current: TimelineEpoch(1),
            attempted: TimelineEpoch(1),
        })
    );
    assert_eq!(
        ingress.reset_epoch(&sequencer(), TimelineEpoch(0), successor),
        Err(EpochResetError::EpochNotIncreasing {
            current: TimelineEpoch(1),
            attempted: TimelineEpoch(0),
        })
    );
}

/// The finalized history digest differs for behaviorally distinct
/// envelopes that vary only in `source_sequence`.
#[test]
fn finalized_history_digest_differs_for_behaviorally_distinct_envelopes() {
    let finalize = |sequence: u64, fence_tag: &str| {
        let mut ingress = fresh_ingress(2);
        let mut e0 = envelope(0, "cmd.0", "payload.0");
        e0.source_sequence = sequence;
        stage_now(&mut ingress, &sequencer(), &e0).unwrap();
        let fence = fence_for(&ingress, 0, 0, fence_tag, std::slice::from_ref(&e0));
        ingress.submit_fence(&sequencer(), &fence).unwrap();
        ingress
    };

    let a = finalize(1, "fence.a");
    let b = finalize(2, "fence.b");
    assert_ne!(a.canonical_history_digest(), b.canonical_history_digest());
    assert_ne!(a.canonical_state_digest(), b.canonical_state_digest());
}

/// A fence may not reach beyond the currently stageable horizon.
#[test]
fn fence_beyond_the_stageable_horizon_rejects() {
    let mut ingress = fresh_ingress(2);
    let e0 = envelope(0, "cmd.0", "payload.0");
    stage_now(&mut ingress, &sequencer(), &e0).unwrap();

    let fence = fence_for(&ingress, 0, 5, "fence.too_far", std::slice::from_ref(&e0));
    assert_eq!(
        ingress.submit_fence(&sequencer(), &fence),
        Err(FenceError::EndOutsideStageableHorizon)
    );
}

/// A fence must start exactly at the current frontier.
#[test]
fn fence_not_starting_at_the_frontier_rejects() {
    let mut ingress = fresh_ingress(3);
    let e1 = envelope(1, "cmd.1", "payload.1");
    stage_now(&mut ingress, &sequencer(), &e1).unwrap();

    let fence = fence_for(
        &ingress,
        1,
        1,
        "fence.skips_frontier",
        std::slice::from_ref(&e1),
    );
    assert_eq!(
        ingress.submit_fence(&sequencer(), &fence),
        Err(FenceError::StartNotAtFrontier)
    );
}

/// An out-of-window submission consumes no capacity and cannot evict,
/// poison, or otherwise disturb any other ordinal's slot (ADR-0003 §6).
#[test]
fn out_of_window_submission_disturbs_nothing() {
    let mut ingress = fresh_ingress(2);
    let e1 = envelope(1, "cmd.1", "payload.1");
    stage_now(&mut ingress, &sequencer(), &e1).unwrap();
    let before = ingress.canonical_state_digest();

    // Ordinals 2 and 7 are both outside the window [0,1].
    for ordinal in [2u64, 7] {
        let out_of_window = envelope(ordinal, "cmd.far", "payload.far");
        let disposition = stage_now(&mut ingress, &sequencer(), &out_of_window).unwrap();
        let advice = match disposition {
            StageDisposition::NotInAdmissionWindow(advice) => advice,
            other => panic!("expected retry advice, got {other:?}"),
        };
        assert_eq!(advice.attempted_ordinal(), Ordinal(ordinal));
        assert_eq!(advice.frontier_ordinal(), Ordinal(0));
        assert_eq!(advice.window_end(), Ordinal(1));
    }

    // The frontier slot stayed reserved and nothing else moved.
    assert_eq!(ingress.slot_status(Ordinal(0)), SlotStatus::Empty);
    assert_eq!(ingress.slot_status(Ordinal(1)), SlotStatus::Staged);
    assert_eq!(ingress.canonical_state_digest(), before);

    // ...and the reserved frontier slot is still usable.
    let e0 = envelope(0, "cmd.0", "payload.0");
    assert_newly_staged(&stage_now(&mut ingress, &sequencer(), &e0).unwrap());
}
