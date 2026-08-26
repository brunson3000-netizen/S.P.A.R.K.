//! Phase-1 minimum test corpus items 1-10
//! (`engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md` §5), falsifying
//! the canonical timeline ingress against ADR-0003's reversed-delivery,
//! stale-token, idempotency/poisoning, and fence-validation invariants.

use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, Digest};
use spark_core::id::{CommandId, FenceId, ProfileId, SourceId};
use spark_core::timeline::{
    compute_ordered_stream_digest, CommandEnvelope, FenceError, Ordinal, SlotStatus, StageError,
    StageOutcome, TimelineEpoch, TimelineFence, TimelineIngress,
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

/// Builds an envelope for `ordinal` using `ingress`'s *current* admission
/// window (i.e. a freshly-valid token), tagging the command/payload
/// identity so distinct calls can be made to collide deliberately.
fn envelope_now(
    ingress: &TimelineIngress,
    ordinal: u64,
    command_tag: &str,
    payload_tag: &str,
) -> CommandEnvelope {
    let window = ingress.current_admission_window();
    CommandEnvelope {
        command_id: CommandId::new(command_tag).unwrap(),
        profile_id: window.profile_id.clone(),
        timeline_epoch: window.timeline_epoch,
        effective_time: LogicalTime(ordinal),
        source_id: sequencer(),
        source_sequence: ordinal,
        input_ordinal: Ordinal(ordinal),
        admission_window_token: window.token.clone(),
        command_kind: "test.command".to_string(),
        canonical_payload_hash: payload_hash(payload_tag),
    }
}

fn fresh_ingress(width: u32) -> TimelineIngress {
    TimelineIngress::new(profile(), epoch(), sequencer(), width)
}

fn fence_for(
    ingress: &TimelineIngress,
    start: u64,
    end: u64,
    fence_tag: &str,
    ordered: &[(Ordinal, CommandId, Digest)],
) -> TimelineFence {
    let window = ingress.current_admission_window();
    TimelineFence {
        profile_id: window.profile_id,
        timeline_epoch: window.timeline_epoch,
        fence_id: FenceId::new(fence_tag).unwrap(),
        start_ordinal: Ordinal(start),
        end_ordinal: Ordinal(end),
        previous_fence_hash: window.last_finalized_fence_hash,
        ordered_stream_digest: compute_ordered_stream_digest(ordered),
    }
}

/// Item 1: `n+1,n+2,n` versus `n,n+1,n+2` with width 2 produce identical
/// per-ordinal stage outcomes, and the same fence through `n+1` succeeds
/// identically in both delivery orders.
#[test]
fn reversed_delivery_within_capacity_two_window_is_order_independent() {
    // Delivery order: n+1, n+2, n.
    let mut reversed = fresh_ingress(2);
    let e_n1_first = envelope_now(&reversed, 1, "cmd.n1", "payload.n1");
    let out_n1_first = reversed
        .stage(&sequencer(), &e_n1_first)
        .expect("protocol-valid stage");
    let e_n2_first = envelope_now(&reversed, 2, "cmd.n2", "payload.n2");
    let out_n2_first = reversed
        .stage(&sequencer(), &e_n2_first)
        .expect("protocol-valid stage");
    let e_n_first = envelope_now(&reversed, 0, "cmd.n", "payload.n");
    let out_n_first = reversed
        .stage(&sequencer(), &e_n_first)
        .expect("protocol-valid stage");

    assert_eq!(out_n_first, StageOutcome::Staged);
    assert_eq!(out_n1_first, StageOutcome::Staged);
    assert_eq!(out_n2_first, StageOutcome::NotInAdmissionWindow);

    // Delivery order: n, n+1, n+2.
    let mut forward = fresh_ingress(2);
    let e_n_second = envelope_now(&forward, 0, "cmd.n", "payload.n");
    let out_n_second = forward
        .stage(&sequencer(), &e_n_second)
        .expect("protocol-valid stage");
    let e_n1_second = envelope_now(&forward, 1, "cmd.n1", "payload.n1");
    let out_n1_second = forward
        .stage(&sequencer(), &e_n1_second)
        .expect("protocol-valid stage");
    let e_n2_second = envelope_now(&forward, 2, "cmd.n2", "payload.n2");
    let out_n2_second = forward
        .stage(&sequencer(), &e_n2_second)
        .expect("protocol-valid stage");

    assert_eq!(out_n_second, out_n_first);
    assert_eq!(out_n1_second, out_n1_first);
    assert_eq!(out_n2_second, out_n2_first);

    // The same fence through n+1 must succeed identically in both.
    let ordered = vec![
        (
            Ordinal(0),
            e_n_first.command_id.clone(),
            e_n_first.canonical_payload_hash.clone(),
        ),
        (
            Ordinal(1),
            e_n1_first.command_id.clone(),
            e_n1_first.canonical_payload_hash.clone(),
        ),
    ];
    let fence_a = fence_for(&reversed, 0, 1, "fence.a", &ordered);
    let fence_b = fence_for(&forward, 0, 1, "fence.b", &ordered);

    reversed.submit_fence(&sequencer(), &fence_a).unwrap();
    forward.submit_fence(&sequencer(), &fence_b).unwrap();

    assert_eq!(reversed.frontier_ordinal(), forward.frontier_ordinal());
    assert_eq!(reversed.finalized_commands(), forward.finalized_commands());
}

/// Item 2: a stale (pre-frontier-advance) token for `n+2` is rejected
/// even though `n+2` is numerically inside the new window; an explicit
/// retry using the freshly issued token succeeds.
#[test]
fn stale_token_after_frontier_advance_is_rejected_then_succeeds_with_fresh_token() {
    let mut ingress = fresh_ingress(2);

    let e0 = envelope_now(&ingress, 0, "cmd.0", "payload.0");
    ingress.stage(&sequencer(), &e0).unwrap();
    let e1 = envelope_now(&ingress, 1, "cmd.1", "payload.1");
    ingress.stage(&sequencer(), &e1).unwrap();

    // Capture the (soon to be stale) window token before finalizing.
    let stale_window = ingress.current_admission_window();

    let ordered = vec![
        (
            Ordinal(0),
            e0.command_id.clone(),
            e0.canonical_payload_hash.clone(),
        ),
        (
            Ordinal(1),
            e1.command_id.clone(),
            e1.canonical_payload_hash.clone(),
        ),
    ];
    let fence = fence_for(&ingress, 0, 1, "fence.1", &ordered);
    ingress.submit_fence(&sequencer(), &fence).unwrap();
    assert_eq!(ingress.frontier_ordinal(), Ordinal(2));

    // Ordinal 2 is now numerically inside the window [2,3], but this
    // envelope carries the stale pre-finalization token.
    let stale_envelope = CommandEnvelope {
        command_id: CommandId::new("cmd.2").unwrap(),
        profile_id: profile(),
        timeline_epoch: epoch(),
        effective_time: LogicalTime(2),
        source_id: sequencer(),
        source_sequence: 2,
        input_ordinal: Ordinal(2),
        admission_window_token: stale_window.token.clone(),
        command_kind: "test.command".to_string(),
        canonical_payload_hash: payload_hash("payload.2"),
    };
    let result = ingress.stage(&sequencer(), &stale_envelope);
    assert_eq!(result, Err(StageError::StaleOrInvalidAdmissionWindowToken));
    assert_eq!(ingress.slot_status(Ordinal(2)), SlotStatus::Empty);

    // Retry with the freshly issued token succeeds.
    let fresh_envelope = envelope_now(&ingress, 2, "cmd.2", "payload.2");
    let retry_outcome = ingress.stage(&sequencer(), &fresh_envelope).unwrap();
    assert_eq!(retry_outcome, StageOutcome::Staged);
}

/// Item 3: an exact duplicate `(command_id, payload)` for an already
/// staged ordinal is idempotent.
#[test]
fn exact_duplicate_is_idempotent() {
    let mut ingress = fresh_ingress(2);
    let e = envelope_now(&ingress, 0, "cmd.a", "payload.a");
    assert_eq!(
        ingress.stage(&sequencer(), &e).unwrap(),
        StageOutcome::Staged
    );

    let e_dup = envelope_now(&ingress, 0, "cmd.a", "payload.a");
    assert_eq!(
        ingress.stage(&sequencer(), &e_dup).unwrap(),
        StageOutcome::AlreadyStagedIdempotent
    );
    assert_eq!(ingress.slot_status(Ordinal(0)), SlotStatus::Staged);
}

/// Item 4: a distinct payload for an already-staged ordinal poisons it,
/// identically regardless of which payload arrived first.
#[test]
fn distinct_payload_collision_poisons_identically_in_opposite_orders() {
    let mut order_a = fresh_ingress(2);
    let a1 = envelope_now(&order_a, 0, "cmd.a", "payload.a");
    let a2 = envelope_now(&order_a, 0, "cmd.b", "payload.b");
    order_a.stage(&sequencer(), &a1).unwrap();
    let second_outcome_a = order_a.stage(&sequencer(), &a2).unwrap();

    let mut order_b = fresh_ingress(2);
    let b1 = envelope_now(&order_b, 0, "cmd.b", "payload.b");
    let b2 = envelope_now(&order_b, 0, "cmd.a", "payload.a");
    order_b.stage(&sequencer(), &b1).unwrap();
    let second_outcome_b = order_b.stage(&sequencer(), &b2).unwrap();

    assert_eq!(second_outcome_a, StageOutcome::Poisoned);
    assert_eq!(second_outcome_b, StageOutcome::Poisoned);
    assert_eq!(order_a.slot_status(Ordinal(0)), SlotStatus::Poisoned);
    assert_eq!(order_b.slot_status(Ordinal(0)), SlotStatus::Poisoned);
}

/// Item 5: a missing ordinal in the fence range blocks finalization
/// atomically (nothing is promoted, frontier unchanged).
#[test]
fn missing_ordinal_blocks_fence_atomically() {
    let mut ingress = fresh_ingress(2);
    let e0 = envelope_now(&ingress, 0, "cmd.0", "payload.0");
    ingress.stage(&sequencer(), &e0).unwrap();
    // Ordinal 1 is deliberately left unstaged.

    let ordered = vec![(
        Ordinal(0),
        e0.command_id.clone(),
        e0.canonical_payload_hash.clone(),
    )];
    let fence = fence_for(&ingress, 0, 1, "fence.missing", &ordered);
    let result = ingress.submit_fence(&sequencer(), &fence);

    assert_eq!(result, Err(FenceError::RangeNotFullyStaged));
    assert_eq!(ingress.frontier_ordinal(), Ordinal(0));
    assert!(ingress.finalized_commands().is_empty());
}

/// Item 6: a fence whose claimed `ordered_stream_digest` does not match
/// the actual staged content is rejected.
#[test]
fn wrong_fence_digest_rejects() {
    let mut ingress = fresh_ingress(2);
    let e0 = envelope_now(&ingress, 0, "cmd.0", "payload.0");
    ingress.stage(&sequencer(), &e0).unwrap();
    let e1 = envelope_now(&ingress, 1, "cmd.1", "payload.1");
    ingress.stage(&sequencer(), &e1).unwrap();

    let window = ingress.current_admission_window();
    let bogus_fence = TimelineFence {
        profile_id: window.profile_id,
        timeline_epoch: window.timeline_epoch,
        fence_id: FenceId::new("fence.bogus").unwrap(),
        start_ordinal: Ordinal(0),
        end_ordinal: Ordinal(1),
        previous_fence_hash: window.last_finalized_fence_hash,
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
    let e0 = envelope_now(&ingress, 0, "cmd.0", "payload.0");
    ingress.stage(&sequencer(), &e0).unwrap();
    let e1 = envelope_now(&ingress, 1, "cmd.1", "payload.1");
    ingress.stage(&sequencer(), &e1).unwrap();

    let ordered = vec![
        (
            Ordinal(0),
            e0.command_id.clone(),
            e0.canonical_payload_hash.clone(),
        ),
        (
            Ordinal(1),
            e1.command_id.clone(),
            e1.canonical_payload_hash.clone(),
        ),
    ];
    let window = ingress.current_admission_window();
    let fence = TimelineFence {
        profile_id: window.profile_id,
        timeline_epoch: window.timeline_epoch,
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
    let e0 = envelope_now(&ingress, 0, "cmd.0", "payload.0");
    let result = ingress.stage(&impostor_sequencer(), &e0);
    assert_eq!(result, Err(StageError::NotActiveSequencer));

    ingress.stage(&sequencer(), &e0).unwrap();
    let e1 = envelope_now(&ingress, 1, "cmd.1", "payload.1");
    ingress.stage(&sequencer(), &e1).unwrap();

    let ordered = vec![
        (
            Ordinal(0),
            e0.command_id.clone(),
            e0.canonical_payload_hash.clone(),
        ),
        (
            Ordinal(1),
            e1.command_id.clone(),
            e1.canonical_payload_hash.clone(),
        ),
    ];
    let fence = fence_for(&ingress, 0, 1, "fence.impostor", &ordered);
    let fence_result = ingress.submit_fence(&impostor_sequencer(), &fence);
    assert_eq!(fence_result, Err(FenceError::NotActiveSequencer));
}

/// Item 9: after an explicit epoch handoff, the old epoch's sequencer can
/// no longer stage or fence.
#[test]
fn old_epoch_sequencer_rejects_after_handoff() {
    let mut ingress = fresh_ingress(2);
    let old_sequencer = sequencer();
    let new_sequencer = SourceId::new("sequencer.successor").unwrap();

    ingress.reset_epoch(TimelineEpoch(2), new_sequencer.clone());

    // An envelope built under the old epoch (old sequencer, old epoch
    // value) is rejected: the epoch no longer matches.
    let stale_epoch_envelope = CommandEnvelope {
        command_id: CommandId::new("cmd.stale_epoch").unwrap(),
        profile_id: profile(),
        timeline_epoch: TimelineEpoch(1),
        effective_time: LogicalTime(0),
        source_id: old_sequencer.clone(),
        source_sequence: 0,
        input_ordinal: Ordinal(0),
        admission_window_token: ingress.current_admission_window().token.clone(),
        command_kind: "test.command".to_string(),
        canonical_payload_hash: payload_hash("payload.stale"),
    };
    let result = ingress.stage(&old_sequencer, &stale_epoch_envelope);
    assert_eq!(result, Err(StageError::WrongTimelineEpoch));

    // The new sequencer, under the new epoch, works normally.
    let fresh = envelope_now(&ingress, 0, "cmd.fresh", "payload.fresh");
    assert_eq!(
        ingress.stage(&new_sequencer, &fresh).unwrap(),
        StageOutcome::Staged
    );
}

/// Item 10: a partial-prefix fence slides the window deterministically,
/// preserving already-staged higher ordinals that remain in the new
/// window and exposing newly reachable ordinals.
#[test]
fn partial_prefix_fence_slides_window_and_preserves_staged_tail() {
    let mut ingress = fresh_ingress(3); // window [0,2]

    let e0 = envelope_now(&ingress, 0, "cmd.0", "payload.0");
    ingress.stage(&sequencer(), &e0).unwrap();
    let e1 = envelope_now(&ingress, 1, "cmd.1", "payload.1");
    ingress.stage(&sequencer(), &e1).unwrap();
    let e2 = envelope_now(&ingress, 2, "cmd.2", "payload.2");
    ingress.stage(&sequencer(), &e2).unwrap();
    assert_eq!(ingress.slot_status(Ordinal(2)), SlotStatus::Staged);

    // Finalize only the prefix [0,1]; ordinal 2 remains staged and
    // un-finalized.
    let ordered = vec![
        (
            Ordinal(0),
            e0.command_id.clone(),
            e0.canonical_payload_hash.clone(),
        ),
        (
            Ordinal(1),
            e1.command_id.clone(),
            e1.canonical_payload_hash.clone(),
        ),
    ];
    let fence = fence_for(&ingress, 0, 1, "fence.prefix", &ordered);
    ingress.submit_fence(&sequencer(), &fence).unwrap();

    assert_eq!(ingress.frontier_ordinal(), Ordinal(2));
    // Window is now [2,4]; ordinal 2's earlier staging survived the slide.
    assert_eq!(ingress.slot_status(Ordinal(2)), SlotStatus::Staged);
    assert_eq!(ingress.current_admission_window().window_end, Ordinal(4));

    // Newly exposed ordinals 3 and 4 are stageable under the new window.
    let e3 = envelope_now(&ingress, 3, "cmd.3", "payload.3");
    assert_eq!(
        ingress.stage(&sequencer(), &e3).unwrap(),
        StageOutcome::Staged
    );

    // The retained ordinal-2 staging can still be finalized.
    let ordered_tail = vec![(
        Ordinal(2),
        e2.command_id.clone(),
        e2.canonical_payload_hash.clone(),
    )];
    let fence2 = fence_for(&ingress, 2, 2, "fence.tail", &ordered_tail);
    ingress.submit_fence(&sequencer(), &fence2).unwrap();
    assert_eq!(ingress.frontier_ordinal(), Ordinal(3));
}
