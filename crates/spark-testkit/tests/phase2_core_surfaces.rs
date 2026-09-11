//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 checkpoint 1: the frozen additive `spark-core` surfaces.
//!
//! - AT-I42 (FINAL oracle §8): the least-due compare-and-take surface X-1 … X-4,
//!   at scheduler level (the engine-level cross-store cases (l)(m)(n) live in the
//!   engine suites).
//! - AT-I35 (FINAL oracle §7): the slice fingerprint's per-slot block is
//!   byte-equal to the block `canonical_state_digest` builds, over scheduled and
//!   conflicted slots at and beyond `MAX_CONFLICT_EVIDENCE` and
//!   `MAX_CONFLICT_TRACKED_CLAIMS`; the scheduler digest bytes are unchanged.
//! - Acceptance pin 2 / PX-1: the three bounded read-only timeline lookups agree
//!   with finalized history, and the derived-index validation holds.

use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, CanonicalEncoder, Digest};
use spark_core::id::{CanonicalTag, CommandId, DefinitionId, FenceId, ProfileId, SourceId};
use spark_core::scheduler::{
    DueWorkItem, OccurrenceIndex, Scheduler, SlotCommitment, TakeSliceError, WorkKey, WorkKind,
    WorkPayload, WorkSlotStatus, MAX_CONFLICT_EVIDENCE, MAX_CONFLICT_TRACKED_CLAIMS,
};
use spark_core::scope::{ScopeId, ScopeKind};
use spark_core::timeline::{
    compute_ordered_stream_digest, CommandKind, Ordinal, SemanticCommandEnvelope, TimelineEpoch,
    TimelineFence, TimelineIngress,
};
use std::collections::BTreeSet;

fn key(due: u64, profile: &str, producer: &str, occurrence: u64) -> WorkKey {
    WorkKey {
        due_time: LogicalTime(due),
        profile_id: ProfileId::new(profile).unwrap(),
        producer_definition_id: DefinitionId::new(producer).unwrap(),
        scope_id: ScopeId::new(ScopeKind::Actor, "bron").unwrap(),
        occurrence_index: OccurrenceIndex(occurrence),
        work_kind: WorkKind::new(CanonicalTag::new("rule.evaluate").unwrap()),
    }
}

fn item(k: &WorkKey, payload: &str) -> DueWorkItem {
    DueWorkItem {
        key: k.clone(),
        payload: WorkPayload::new(hash_bytes(payload.as_bytes())),
    }
}

/// Independent reference encoder for one scheduled slot block, written from the
/// Phase-1 `canonical_state_digest` source, not from the Gate C2 helper.
fn reference_scheduled_block(k: &WorkKey, payload: &str) -> CanonicalEncoder {
    let mut inner = CanonicalEncoder::new();
    k.canonicalize(&mut inner);
    inner.push_str("slot.scheduled");
    inner.push_digest(&hash_bytes(payload.as_bytes()));
    inner
}

fn fixture() -> (Scheduler, WorkKey, WorkKey, WorkKey) {
    let a = key(100, "game-world", "rule.a", 0);
    let x = key(100, "game-world", "rule.x", 0);
    let later = key(101, "game-world", "rule.a", 1);
    let mut s = Scheduler::new();
    s.schedule(item(&later, "later"));
    s.schedule(item(&x, "p1"));
    s.schedule(item(&x, "p2")); // conflicted
    s.schedule(item(&a, "a"));
    (s, a, x, later)
}

/// AT-I42(a)(b), X-1: the handle names only the least slice; reading it leaves
/// the canonical digest unchanged.
#[test]
fn at_i42_x1_exposes_only_the_live_least_slice_and_is_read_only() {
    let (s, a, x, _later) = fixture();
    let before = s.canonical_state_digest();
    let slice = s.least_due_slice(LogicalTime(1_000)).unwrap();
    assert_eq!(slice.due_time(), LogicalTime(100));
    assert_eq!(slice.executable_count(), 1);
    assert_eq!(slice.conflicted_count(), 1);
    assert_eq!(slice.occupied_slot_count(), 2);
    assert_eq!(
        slice.keys(),
        vec![
            (a.clone(), WorkSlotStatus::Scheduled),
            (x.clone(), WorkSlotStatus::Conflicted)
        ]
    );
    drop(slice);
    assert_eq!(before, s.canonical_state_digest());
    assert!(s.least_due_slice(LogicalTime(99)).is_none());
}

/// AT-I35 pin: the per-slot block and the fingerprint are pinned against an
/// independent reference encoder, and the scheduler digest bytes still equal the
/// Phase-1 composition.
#[test]
fn at_i35_fingerprint_block_is_the_scheduler_digest_block() {
    let k0 = key(7, "game-world", "rule.a", 0);
    let k1 = key(7, "game-world", "rule.b", 0);
    let mut s = Scheduler::new();
    s.schedule(item(&k1, "one"));
    s.schedule(item(&k0, "zero"));

    let blocks = [
        reference_scheduled_block(&k0, "zero"),
        reference_scheduled_block(&k1, "one"),
    ];
    // Scheduler digest, recomputed from the Phase-1 composition.
    let mut digest = CanonicalEncoder::new();
    digest.push_str("scheduler_state").push_u64(2);
    for b in &blocks {
        digest.push_block(b);
    }
    assert_eq!(s.canonical_state_digest(), digest.finish());
    // Fingerprint, recomputed from the frozen formula over the same blocks.
    let mut fp = CanonicalEncoder::new();
    fp.push_str("due_slice_fingerprint_v1");
    LogicalTime(7).canonicalize(&mut fp);
    ProfileId::new("game-world").unwrap().canonicalize(&mut fp);
    fp.push_u64(2);
    for b in &blocks {
        fp.push_block(b);
    }
    let slice = s.least_due_slice(LogicalTime(7)).unwrap();
    assert_eq!(slice.fingerprint().as_digest(), &fp.finish());
    // Per-slot commitments are the digests of those blocks.
    let commitments = slice.slot_commitments();
    assert_eq!(commitments[0].2.as_digest(), &blocks[0].finish());
    assert_eq!(
        commitments[0].2,
        SlotCommitment::scheduled(&k0, &WorkPayload::new(hash_bytes(b"zero")))
    );
}

/// AT-I35 pin over conflicted slots at and beyond both evidence caps: the
/// commitment tracks the complete tracked claim set and truncation flag, not
/// the exposed evidence projection.
#[test]
fn at_i35_conflicted_commitment_covers_the_tracked_claim_set_at_caps() {
    for claims in [
        2usize,
        MAX_CONFLICT_EVIDENCE,
        MAX_CONFLICT_EVIDENCE + 1,
        MAX_CONFLICT_TRACKED_CLAIMS,
        MAX_CONFLICT_TRACKED_CLAIMS + 3,
    ] {
        let k = key(3, "game-world", "rule.c", 0);
        let mut s = Scheduler::new();
        let mut hashes: Vec<Digest> = Vec::new();
        for i in 0..claims {
            let p = format!("claim.{i}");
            hashes.push(hash_bytes(p.as_bytes()));
            s.schedule(item(&k, &p));
        }
        hashes.sort();
        let truncated = claims > MAX_CONFLICT_TRACKED_CLAIMS;
        let tracked: BTreeSet<Digest> = hashes
            .into_iter()
            .take(MAX_CONFLICT_TRACKED_CLAIMS)
            .collect();
        let slice = s.least_due_slice(LogicalTime(3)).unwrap();
        let (_, status, commitment) = slice.slot_commitments().remove(0);
        assert_eq!(status, WorkSlotStatus::Conflicted);
        assert_eq!(
            commitment,
            SlotCommitment::conflicted(&k, &tracked, truncated),
            "claims={claims}"
        );
        // Dropping one tracked claim changes the commitment.
        let mut fewer = tracked.clone();
        let first = fewer.iter().next().unwrap().clone();
        fewer.remove(&first);
        assert_ne!(
            commitment,
            SlotCommitment::conflicted(&k, &fewer, truncated)
        );
    }
}

/// AT-I42(a)(b): atomic single-slice removal of a mixed slice, in `drain_due`
/// shape; later work untouched.
#[test]
fn at_i42_take_removes_exactly_the_least_slice_in_drain_shape() {
    let (mut s, a, x, later) = fixture();
    let mut reference = s.clone();
    let expected = s.least_due_slice(LogicalTime(1_000)).unwrap().fingerprint();
    let taken = s
        .take_least_due_slice(LogicalTime(1_000), &expected)
        .unwrap();
    assert_eq!(taken.due_time, LogicalTime(100));
    assert_eq!(taken.fingerprint, expected);
    let drained = reference.drain_due(LogicalTime(100));
    assert_eq!(taken.due, drained.due);
    assert_eq!(taken.conflicted, drained.conflicted);
    assert_eq!(taken.due[0].key, a);
    assert_eq!(taken.conflicted[0].key(), &x);
    assert!(s.contains(&later));
    assert_eq!(s.len(), 1);
    assert_eq!(
        s.canonical_state_digest(),
        reference.canonical_state_digest()
    );
}

/// AT-I42(c)(g)(h): a non-least fingerprint is a byte-identical typed no-op that
/// returns the live least fingerprint.
#[test]
fn at_i42_c_non_least_fingerprint_is_refused_as_a_no_op() {
    let (mut s, _a, _x, _later) = fixture();
    let live = s.least_due_slice(LogicalTime(1_000)).unwrap().fingerprint();
    let mut only_later = s.clone();
    let first = only_later
        .least_due_slice(LogicalTime(1_000))
        .unwrap()
        .fingerprint();
    only_later
        .take_least_due_slice(LogicalTime(1_000), &first)
        .unwrap();
    let non_least = only_later
        .least_due_slice(LogicalTime(1_000))
        .unwrap()
        .fingerprint();
    let before = s.canonical_state_digest();
    let err = s
        .take_least_due_slice(LogicalTime(1_000), &non_least)
        .unwrap_err();
    assert_eq!(err, TakeSliceError::SliceChanged { observed: live });
    assert_eq!(before, s.canonical_state_digest());
    assert_eq!(s.len(), 3);
}

/// AT-I42(f)(g): `NoSliceDue` is constructible and carries the least resident
/// due time; empty scheduler reports `None`.
#[test]
fn at_i42_f_no_slice_due_is_constructible_and_a_no_op() {
    let (mut s, _a, _x, _later) = fixture();
    let fp = s.least_due_slice(LogicalTime(1_000)).unwrap().fingerprint();
    let before = s.canonical_state_digest();
    assert_eq!(
        s.take_least_due_slice(LogicalTime(99), &fp).unwrap_err(),
        TakeSliceError::NoSliceDue {
            least_resident_due_time: Some(LogicalTime(100))
        }
    );
    assert_eq!(before, s.canonical_state_digest());
    let mut empty = Scheduler::new();
    assert_eq!(
        empty.take_least_due_slice(LogicalTime(5), &fp).unwrap_err(),
        TakeSliceError::NoSliceDue {
            least_resident_due_time: None
        }
    );
}

/// AT-I42(d)(1)(2)(2')(3), corrected per V2-07: replay of a retained
/// fingerprint is possible and unprivileged.
#[test]
fn at_i42_d_replay_is_possible_and_unprivileged() {
    let k = key(100, "game-world", "rule.a", 0);
    let other = key(50, "game-world", "rule.b", 0);
    let setup = || {
        let mut s = Scheduler::new();
        s.schedule(item(&k, "A"));
        s
    };
    let mut s = setup();
    let retained = s.least_due_slice(LogicalTime(100)).unwrap().fingerprint();
    s.take_least_due_slice(LogicalTime(100), &retained).unwrap();

    // (1) identical re-creation: replay succeeds and equals a fresh take.
    let mut one = s.clone();
    one.schedule(item(&k, "A"));
    let mut fresh = one.clone();
    let replayed = one
        .take_least_due_slice(LogicalTime(100), &retained)
        .unwrap();
    let fresh_fp = fresh
        .least_due_slice(LogicalTime(100))
        .unwrap()
        .fingerprint();
    assert_eq!(
        replayed,
        fresh
            .take_least_due_slice(LogicalTime(100), &fresh_fp)
            .unwrap()
    );

    // (2) same key, different payload: Scheduled(new), fingerprint differs.
    let mut two = s.clone();
    two.schedule(item(&k, "B"));
    assert_eq!(two.slot_status(&k), WorkSlotStatus::Scheduled);
    let before = two.canonical_state_digest();
    assert!(matches!(
        two.take_least_due_slice(LogicalTime(100), &retained),
        Err(TakeSliceError::SliceChanged { .. })
    ));
    assert_eq!(before, two.canonical_state_digest());
    let fresh_two = two.least_due_slice(LogicalTime(100)).unwrap().fingerprint();
    let taken = two
        .take_least_due_slice(LogicalTime(100), &fresh_two)
        .unwrap();
    assert_eq!(
        taken.due[0].payload.canonical_payload_hash,
        hash_bytes(b"B")
    );

    // (2') re-created then conflicted: replay no-op; fresh take extracts the
    // conflicted slot with the complete claim set.
    let mut conflicted = s.clone();
    conflicted.schedule(item(&k, "B"));
    conflicted.schedule(item(&k, "C"));
    assert!(conflicted
        .take_least_due_slice(LogicalTime(100), &retained)
        .is_err());
    let fp = conflicted
        .least_due_slice(LogicalTime(100))
        .unwrap()
        .fingerprint();
    let taken = conflicted
        .take_least_due_slice(LogicalTime(100), &fp)
        .unwrap();
    assert!(taken.due.is_empty());
    assert_eq!(taken.conflicted[0].competing_payload_hashes().len(), 2);

    // (3) a different slice is now least: typed no-op.
    let mut three = setup();
    three.schedule(item(&other, "O"));
    assert!(matches!(
        three.take_least_due_slice(LogicalTime(100), &retained),
        Err(TakeSliceError::SliceChanged { .. })
    ));
}

/// AT-I42(e): same-key `Scheduled -> Conflicted` between observation and take.
#[test]
fn at_i42_e_scheduled_to_conflicted_between_observation_and_take_is_a_no_op() {
    let k = key(10, "game-world", "rule.a", 0);
    let mut s = Scheduler::new();
    s.schedule(item(&k, "A"));
    let observed = s.least_due_slice(LogicalTime(10)).unwrap().fingerprint();
    s.schedule(item(&k, "B"));
    let before = s.canonical_state_digest();
    assert!(matches!(
        s.take_least_due_slice(LogicalTime(10), &observed),
        Err(TakeSliceError::SliceChanged { .. })
    ));
    assert_eq!(before, s.canonical_state_digest());
    assert_eq!(s.slot_status(&k), WorkSlotStatus::Conflicted);
}

/// AT-I42(i), Phase-1 refinement in its static/null-evaluator scope: taking the
/// least slice to exhaustion equals one `drain_due`, value for value.
#[test]
fn at_i42_i_repeated_take_refines_drain_due() {
    let mut s = Scheduler::new();
    for (due, producer, occ, payload) in [
        (5u64, "rule.a", 0u64, "a"),
        (5, "rule.b", 0, "b"),
        (7, "rule.a", 1, "c"),
        (9, "rule.c", 0, "d"),
        (20, "rule.z", 0, "future"),
    ] {
        s.schedule(item(&key(due, "game-world", producer, occ), payload));
    }
    s.schedule(item(&key(7, "game-world", "rule.a", 1), "c2")); // conflicted
    let mut reference = s.clone();
    let drained = reference.drain_due(LogicalTime(9));
    let mut due = Vec::new();
    let mut conflicted = Vec::new();
    while let Some(slice) = s.least_due_slice(LogicalTime(9)) {
        let fp = slice.fingerprint();
        let t = s.take_least_due_slice(LogicalTime(9), &fp).unwrap();
        due.extend(t.due);
        conflicted.extend(t.conflicted);
    }
    assert_eq!(due, drained.due);
    assert_eq!(conflicted, drained.conflicted);
    assert_eq!(
        s.canonical_state_digest(),
        reference.canonical_state_digest()
    );
}

/// AT-I40 (scheduler half) and AT-I42(j): slices are per `(due_time, profile)`
/// in ascending `(due_time, profile_id)` order, and insertion permutations yield
/// identical fingerprints and extractions.
#[test]
fn at_i40_at_i42_j_slices_are_per_profile_and_permutation_invariant() {
    let items = [
        item(&key(10, "profile-b", "rule.a", 0), "b"),
        item(&key(10, "profile-a", "rule.a", 0), "a1"),
        item(&key(10, "profile-a", "rule.b", 0), "a2"),
        item(&key(11, "profile-a", "rule.a", 1), "later"),
    ];
    let mut forward = Scheduler::new();
    for i in &items {
        forward.schedule(i.clone());
    }
    let mut reversed = Scheduler::new();
    for i in items.iter().rev() {
        reversed.schedule(i.clone());
    }
    let mut order = Vec::new();
    while let Some(slice) = forward.least_due_slice(LogicalTime(20)) {
        let fp = slice.fingerprint();
        let other = reversed
            .least_due_slice(LogicalTime(20))
            .unwrap()
            .fingerprint();
        assert_eq!(fp, other);
        let a = forward.take_least_due_slice(LogicalTime(20), &fp).unwrap();
        let b = reversed.take_least_due_slice(LogicalTime(20), &fp).unwrap();
        assert_eq!(a, b);
        order.push((a.due_time.0, a.profile_id.as_str().to_string(), a.due.len()));
    }
    assert_eq!(
        order,
        vec![
            (10, "profile-a".to_string(), 2),
            (10, "profile-b".to_string(), 1),
            (11, "profile-a".to_string(), 1)
        ]
    );
}

/// AT-I42(k): totality at the `u64` edges.
#[test]
fn at_i42_k_surface_is_total_at_u64_edges() {
    let k = key(u64::MAX, "game-world", "rule.a", u64::MAX);
    let mut s = Scheduler::new();
    s.schedule(item(&k, "edge"));
    assert!(s.least_due_slice(LogicalTime(u64::MAX - 1)).is_none());
    let fp = s
        .least_due_slice(LogicalTime(u64::MAX))
        .unwrap()
        .fingerprint();
    let t = s.take_least_due_slice(LogicalTime(u64::MAX), &fp).unwrap();
    assert_eq!(t.due.len(), 1);
    assert!(s.is_empty());
    assert_eq!(
        s.due_slice_summary(LogicalTime(u64::MAX)),
        Default::default()
    );
}

/// X-4: the summary counts executable slices only.
#[test]
fn x4_summary_is_executable_only_telemetry() {
    let mut s = Scheduler::new();
    let c = key(5, "game-world", "rule.c", 0);
    s.schedule(item(&c, "p1"));
    s.schedule(item(&c, "p2")); // all-conflicted slice at 5
    s.schedule(item(&key(6, "game-world", "rule.a", 0), "a"));
    s.schedule(item(&key(9, "game-world", "rule.a", 1), "b"));
    let before = s.canonical_state_digest();
    let summary = s.due_slice_summary(LogicalTime(8));
    assert_eq!(summary.resident_slice_count, 2);
    assert_eq!(summary.executable_slice_count, 1);
    assert_eq!(summary.earliest_executable_due_time, Some(LogicalTime(6)));
    assert_eq!(before, s.canonical_state_digest());
}

fn sequencer() -> SourceId {
    SourceId::new("sequencer").unwrap()
}

fn finalize(t: &mut TimelineIngress, command: &str, source: &str, seq: u64) {
    let ordinal = t.frontier_ordinal();
    let e = SemanticCommandEnvelope {
        command_id: CommandId::new(command).unwrap(),
        profile_id: t.profile_id().clone(),
        timeline_epoch: t.timeline_epoch(),
        effective_time: LogicalTime(1),
        source_id: SourceId::new(source).unwrap(),
        source_sequence: seq,
        input_ordinal: ordinal,
        command_kind: CommandKind::new(CanonicalTag::new("probe.command").unwrap()),
        canonical_payload_hash: hash_bytes(command.as_bytes()),
    };
    let ticket = t.current_admission_window().unwrap().ticket();
    t.stage(&sequencer(), &e.clone().submit_with(ticket))
        .unwrap();
    let fence = TimelineFence {
        profile_id: t.profile_id().clone(),
        timeline_epoch: t.timeline_epoch(),
        fence_id: FenceId::new(format!("fence.{}", ordinal.0)).unwrap(),
        start_ordinal: ordinal,
        end_ordinal: ordinal,
        previous_fence_hash: t.last_finalized_fence_hash().clone(),
        ordered_stream_digest: compute_ordered_stream_digest(&[(ordinal, e.semantic_hash())]),
    };
    t.submit_fence(&sequencer(), &fence).unwrap();
}

/// Pin 2 / PX-1: the indexed lookups agree with finalized history (scanned here,
/// in the test only), including across an epoch reset; derived indexes validate.
#[test]
fn px1_lookups_agree_with_finalized_history_and_indexes_validate() {
    let mut t = TimelineIngress::new(
        ProfileId::new("game-world").unwrap(),
        TimelineEpoch(0),
        sequencer(),
        2,
    )
    .unwrap();
    for i in 0..40u64 {
        let source = if i % 3 == 0 { "source.a" } else { "source.b" };
        finalize(&mut t, &format!("cmd.{i}"), source, 10 + i);
        if i == 20 {
            t.reset_epoch(&sequencer(), TimelineEpoch(1), sequencer())
                .unwrap();
        }
    }
    assert!(t.derived_indexes_consistent());
    for fc in t.finalized_commands() {
        assert_eq!(
            t.finalized_command_claim(&fc.envelope.command_id),
            Some(&fc.semantic_envelope_hash)
        );
        assert_eq!(
            t.finalized_source_sequence_claim(&fc.envelope.source_id, fc.envelope.source_sequence),
            Some(&fc.semantic_envelope_hash)
        );
    }
    for source in ["source.a", "source.b"] {
        let sid = SourceId::new(source).unwrap();
        let scanned = t
            .finalized_commands()
            .iter()
            .rev()
            .find(|f| f.envelope.source_id == sid)
            .map(|f| f.envelope.source_sequence);
        assert_eq!(t.last_finalized_source_sequence(&sid), scanned);
    }
    assert_eq!(
        t.finalized_command_claim(&CommandId::new("never").unwrap()),
        None
    );
    assert_eq!(
        t.last_finalized_source_sequence(&SourceId::new("source.c").unwrap()),
        None
    );
    assert_eq!(t.frontier_ordinal(), Ordinal(40));
}
