// Isolated reviewer probe of unchanged Phase-1 APIs, not production implementation.
use spark_core::clock::LogicalTime;
use spark_core::hash::hash_bytes;
use spark_core::id::CanonicalTag;
use spark_core::id::{CommandId, FenceId, ProfileId, SourceId};
use spark_core::timeline::*;

fn envelope(id: &str, ordinal: u64, sequence: u64) -> SemanticCommandEnvelope {
    SemanticCommandEnvelope {
        command_id: CommandId::new(id).unwrap(),
        profile_id: ProfileId::new("review").unwrap(),
        timeline_epoch: TimelineEpoch(0),
        effective_time: LogicalTime(20),
        source_id: SourceId::new("s").unwrap(),
        source_sequence: sequence,
        input_ordinal: Ordinal(ordinal),
        command_kind: CommandKind::new(CanonicalTag::new("review.command").unwrap()),
        canonical_payload_hash: hash_bytes(id.as_bytes()),
    }
}
fn fence(t: &TimelineIngress, e: &SemanticCommandEnvelope) -> TimelineFence {
    TimelineFence {
        profile_id: t.profile_id().clone(),
        timeline_epoch: t.timeline_epoch(),
        fence_id: FenceId::new(format!("fence.{}", e.input_ordinal.0)).unwrap(),
        start_ordinal: e.input_ordinal,
        end_ordinal: e.input_ordinal,
        previous_fence_hash: t.last_finalized_fence_hash().clone(),
        ordered_stream_digest: compute_ordered_stream_digest(&[(
            e.input_ordinal,
            e.semantic_hash(),
        )]),
    }
}
fn main() {
    let sequencer = SourceId::new("sequencer").unwrap();
    let mut t = TimelineIngress::new(
        ProfileId::new("review").unwrap(),
        TimelineEpoch(0),
        sequencer.clone(),
        2,
    )
    .unwrap();
    let first = envelope("first", 0, 10);
    let ticket = t.current_admission_window().unwrap().ticket();
    assert!(t
        .stage(&sequencer, &first.clone().submit_with(ticket))
        .unwrap()
        .acknowledgement()
        .is_some());
    t.submit_fence(&sequencer, &fence(&t, &first)).unwrap();
    assert_eq!(t.slot_status(Ordinal(1)), SlotStatus::Empty);
    let before = t.canonical_state_digest();
    let history = t.canonical_history_digest();
    let regressed = envelope("distinct", 1, 9);
    let ticket = t.current_admission_window().unwrap().ticket();
    let ack = t
        .stage(&sequencer, &regressed.clone().submit_with(ticket))
        .unwrap();
    assert_eq!(
        ack.acknowledgement().unwrap().slot_state(),
        AcknowledgedSlotState::NewlyStaged
    );
    let staged = t.canonical_state_digest();
    let staged_debug = format!("{t:?}");
    assert_ne!(before, staged);
    let err = t
        .submit_fence(&sequencer, &fence(&t, &regressed))
        .unwrap_err();
    assert_eq!(
        err,
        FenceError::SourceSequenceNotIncreasing {
            source_id: SourceId::new("s").unwrap(),
            previously_finalized: 10,
            attempted: 9,
        }
    );
    assert_eq!(staged, t.canonical_state_digest());
    assert_eq!(staged_debug, format!("{t:?}")); // all Debug-exposed private indexes unchanged by fence
    assert_ne!(before, t.canonical_state_digest());
    assert_eq!(history, t.canonical_history_digest());
    assert_eq!(t.frontier_ordinal(), Ordinal(1));
    assert!(t.is_positively_staged(Ordinal(1)));
    assert_eq!(t.finalized_commands().len(), 1);
    println!("CONFIRMED: S:10 finalized; distinct S:9 positively staged; fence rejects {err:?}; staged slot survives");
    println!(
        "before={before:?}\nstaged={staged:?}\nafter={:?}",
        t.canonical_state_digest()
    );
    if std::env::args().any(|a| a == "--assert-candidate") {
        assert_eq!(
            before,
            t.canonical_state_digest(),
            "candidate refusal atomicity is false"
        );
    }
}
