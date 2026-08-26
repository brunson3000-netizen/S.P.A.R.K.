//! **Final digest correction corpus (AT-G).**
//!
//! `PHASE_1_FINAL_DIGEST_CORRECTION_BRIEF_v0.1.md` states one rule:
//!
//! > Every retained internal state value that can change future canonical
//! > behavior must participate in canonical state identity.
//!
//! The Re-Foundation v2 tree satisfied that rule only for the *exposed*
//! projection of contested-slot evidence. Both bounded-evidence types in
//! the kernel — [`spark_core::scheduler`]'s conflict evidence and
//! [`spark_core::timeline`]'s poison evidence — retain up to 256 distinct
//! claim hashes internally but canonicalized only the 16 they expose,
//! plus the omitted count and the truncation flag. Two states could
//! therefore agree on every canonicalized field, hold *different* hidden
//! tracked claims, and then react differently to the very same next
//! claim: a false canonical-state commitment.
//!
//! These assertions were written and run against the inherited tree
//! first. The recorded red baseline is
//! `engineering/phase1/final-digest-correction-baseline/BASELINE_HIDDEN_EVIDENCE_FAILURES.txt`.
//!
//! Two disciplines carry over from the v2 corpus and matter especially
//! here:
//!
//! 1. **A hidden-state test must actually hide the difference.** Every
//!    discrimination test below first asserts that the two states are
//!    indistinguishable through the *entire exposed surface* — retained
//!    hash set, omitted count, truncation flag, slot status — so that a
//!    digest difference can only come from the hidden tracked claims and
//!    never from something a caller could already see.
//! 2. **A behavioral claim is proved by divergence, not by assertion.**
//!    Each discrimination test is paired with a test that submits *the
//!    same* next claim to both states and shows they evolve differently.
//!    That is what makes the hidden difference behavior-relevant rather
//!    than a representational detail, and therefore what obliges canonical
//!    state identity to commit to it.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, Digest};
use spark_core::id::{CanonicalTag, CommandId, DefinitionId, ProfileId, SourceId};
use spark_core::scheduler::{
    DrainOutcome, DueWorkItem, OccurrenceIndex, Scheduler, WorkKey, WorkKind, WorkPayload,
    WorkSlotStatus, MAX_CONFLICT_EVIDENCE, MAX_CONFLICT_TRACKED_CLAIMS,
};
use spark_core::scope::{ScopeId, ScopeKind};
use spark_core::timeline::{
    CommandKind, Ordinal, SemanticCommandEnvelope, SlotStatus, TimelineEpoch, TimelineIngress,
    MAX_POISON_EVIDENCE, MAX_POISON_TRACKED_CLAIMS,
};
use std::collections::BTreeSet;

fn profile() -> ProfileId {
    ProfileId::new("game-world").unwrap()
}

fn sequencer() -> SourceId {
    SourceId::new("sequencer.primary").unwrap()
}

/// The claim-set sizes the brief names: just past the exposed cap, well
/// past it, exactly at the tracking cap, and beyond the tracking cap.
const MANDATED_SET_SIZES: [usize; 4] = [17, 64, MAX_CONFLICT_TRACKED_CLAIMS, 300];

/// Three deterministic arrival orders over the same claim set. Reversal
/// alone can be satisfied by an accidentally symmetric implementation, so
/// the third order interleaves three strides and is neither the forward
/// order nor its reverse for any set larger than three.
fn arrival_orders<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let forward: Vec<T> = items.to_vec();
    let reversed: Vec<T> = items.iter().rev().cloned().collect();
    let interleaved: Vec<T> = items
        .iter()
        .step_by(3)
        .chain(items.iter().skip(1).step_by(3))
        .chain(items.iter().skip(2).step_by(3))
        .cloned()
        .collect();
    vec![forward, reversed, interleaved]
}

// =====================================================================
// AT-G1 … AT-G3 — scheduler conflict evidence
// =====================================================================

fn work_key() -> WorkKey {
    WorkKey {
        due_time: LogicalTime(5),
        profile_id: profile(),
        producer_definition_id: DefinitionId::new("trigger.weather.drought").unwrap(),
        scope_id: ScopeId::new(ScopeKind::Settlement, "settlement.pontafique").unwrap(),
        occurrence_index: OccurrenceIndex(0),
        work_kind: WorkKind::new(CanonicalTag::new("trigger.evaluate").unwrap()),
    }
}

/// A payload hash whose byte order is its numeric order, so a test can
/// state *exactly* which claims fall inside the smallest-16 exposed
/// window and which are retained only as hidden tracked evidence.
///
/// This is a legitimate construction, not a shortcut around the API:
/// `WorkPayload` is defined to carry an already-canonical opaque payload
/// hash, so any 32-byte digest is a payload a real producer could present.
fn payload_hash(n: u32) -> Digest {
    let mut bytes = [0u8; 32];
    bytes[0] = ((n >> 8) & 0xff) as u8;
    bytes[1] = (n & 0xff) as u8;
    Digest(bytes)
}

fn claim(n: u32) -> DueWorkItem {
    DueWorkItem {
        key: work_key(),
        payload: WorkPayload::new(payload_hash(n)),
    }
}

fn conflicted_scheduler(claims: &[u32]) -> Scheduler {
    let mut sched = Scheduler::new();
    for n in claims {
        sched.schedule(claim(*n));
    }
    sched
}

/// The whole exposed surface of a conflicted key, as a caller sees it.
fn exposed_conflict(sched: &Scheduler) -> (BTreeSet<Digest>, u64, bool, WorkSlotStatus) {
    let key = work_key();
    let conflict = sched
        .conflict_of(&key)
        .expect("the key under test is contested and therefore conflicted");
    (
        conflict.competing_payload_hashes().clone(),
        conflict.omitted_distinct(),
        conflict.evidence_truncated(),
        sched.slot_status(&key),
    )
}

/// The two claim sets used by AT-G1/AT-G2. They share the sixteen
/// smallest hashes — the entire exposed evidence window — and the same
/// number of tracked claims, and differ only in claims that are tracked
/// internally without ever being exposed.
fn hidden_evidence_pair() -> (Vec<u32>, Vec<u32>) {
    let common: Vec<u32> = (0..MAX_CONFLICT_EVIDENCE as u32).collect();
    let mut left = common.clone();
    left.extend([200, 201]);
    let mut right = common;
    right.extend([202, 203]);
    (left, right)
}

/// **AT-G1** — two conflict states that are identical across the entire
/// exposed surface but hold different hidden tracked claims must not
/// share a canonical state digest.
///
/// This is the defect Codex reproduced against Re-Foundation v2: the
/// digest committed to the sixteen-hash presentation projection, so
/// hidden tracked claims 17…256 were absent from canonical state identity
/// even though the implementation retained them.
#[test]
fn hidden_conflict_evidence_is_committed_to_canonical_state() {
    let (left_claims, right_claims) = hidden_evidence_pair();
    let left = conflicted_scheduler(&left_claims);
    let right = conflicted_scheduler(&right_claims);

    // The difference really is hidden: every exposed field agrees.
    assert_eq!(exposed_conflict(&left), exposed_conflict(&right));
    assert_eq!(exposed_conflict(&left).1, 2);
    assert!(!exposed_conflict(&left).2);

    // ...and the retained hidden claims really do differ.
    assert_ne!(left_claims, right_claims);

    assert_ne!(
        left.canonical_state_digest(),
        right.canonical_state_digest(),
        "conflict states with different retained tracked claims must not \
         share a canonical state digest"
    );
}

/// **AT-G2** — the hidden difference is behavior-relevant: the same next
/// claim evolves the two states differently, so a shared digest would
/// have been a false commitment rather than a harmless representation
/// detail.
#[test]
fn hidden_conflict_evidence_changes_the_reaction_to_the_next_claim() {
    let (left_claims, right_claims) = hidden_evidence_pair();
    let mut left = conflicted_scheduler(&left_claims);
    let mut right = conflicted_scheduler(&right_claims);

    let before_left = left.canonical_state_digest();
    let before_right = right.canonical_state_digest();

    // Claim 200 is already tracked by `left` and unknown to `right`.
    left.schedule(claim(200));
    right.schedule(claim(200));

    // The same input, two different outcomes: idempotent on the left,
    // a new distinct claim on the right.
    assert_eq!(exposed_conflict(&left).1, 2);
    assert_eq!(exposed_conflict(&right).1, 3);
    assert_ne!(
        left.canonical_state_digest(),
        right.canonical_state_digest()
    );
    assert_eq!(before_left, left.canonical_state_digest());
    assert_ne!(before_right, right.canonical_state_digest());

    // Their drains differ too, so the divergence reaches reported work.
    assert_ne!(
        left.clone().drain_due(LogicalTime(5)),
        right.clone().drain_due(LogicalTime(5))
    );

    // Therefore the two states were behaviorally distinct *before* that
    // claim arrived, and canonical state identity had to say so.
    assert_ne!(
        before_left, before_right,
        "states that react differently to the same next claim must not \
         have shared a canonical state digest"
    );
}

/// **AT-G3** — committing to the hidden tracked claims must not cost
/// arrival-order independence at any scale: every permutation of one
/// claim set converges on one digest, one conflict report, and one drain,
/// just past the exposed cap, well past it, exactly at the tracking cap,
/// and beyond it.
#[test]
fn conflict_permutations_converge_at_every_mandated_set_size() {
    for size in MANDATED_SET_SIZES {
        let claims: Vec<u32> = (0..size as u32).collect();

        let mut digests: BTreeSet<Digest> = BTreeSet::new();
        let mut exposed: BTreeSet<Vec<u8>> = BTreeSet::new();
        let mut drains: Vec<DrainOutcome> = Vec::new();

        for order in arrival_orders(&claims) {
            let mut sched = conflicted_scheduler(&order);
            digests.insert(sched.canonical_state_digest());

            let (hashes, omitted, truncated, status) = exposed_conflict(&sched);
            assert_eq!(status, WorkSlotStatus::Conflicted);
            assert_eq!(hashes.len(), MAX_CONFLICT_EVIDENCE);
            let tracked = size.min(MAX_CONFLICT_TRACKED_CLAIMS);
            assert_eq!(omitted as usize, tracked - MAX_CONFLICT_EVIDENCE);
            assert_eq!(truncated, size > MAX_CONFLICT_TRACKED_CLAIMS);
            // The exposed window is the sixteen smallest hashes of the
            // whole claim set, in every arrival order.
            assert_eq!(
                hashes,
                (0..MAX_CONFLICT_EVIDENCE as u32)
                    .map(payload_hash)
                    .collect::<BTreeSet<Digest>>()
            );

            let mut key = Vec::new();
            for hash in &hashes {
                key.extend_from_slice(hash.as_bytes());
            }
            key.extend_from_slice(&omitted.to_le_bytes());
            key.push(truncated as u8);
            exposed.insert(key);

            drains.push(sched.drain_due(LogicalTime(5)));
        }

        assert_eq!(digests.len(), 1, "set size {size} did not converge");
        assert_eq!(exposed.len(), 1, "set size {size} exposed differently");
        for drain in &drains {
            assert_eq!(drain, &drains[0], "set size {size} drained differently");
            assert!(drain.due.is_empty());
            assert_eq!(drain.conflicted.len(), 1);
        }
    }
}

/// **AT-G3 (companion)** — the two claim sets that differ *only* beyond
/// the tracking cap are genuinely collapsed, so the correction commits to
/// retained state without inventing a commitment to forgotten state.
#[test]
fn claims_forgotten_beyond_the_tracking_cap_stay_collapsed() {
    let base: Vec<u32> = (0..MAX_CONFLICT_TRACKED_CLAIMS as u32).collect();

    let mut left = base.clone();
    left.extend([60000, 60001]);
    let mut right = base;
    right.extend([60002, 60003]);

    let left = conflicted_scheduler(&left);
    let right = conflicted_scheduler(&right);

    assert!(exposed_conflict(&left).2);
    assert_eq!(
        left.canonical_state_digest(),
        right.canonical_state_digest(),
        "claims dropped past the tracking cap are not retained state and \
         must not be reconstructible from the digest"
    );
}

// =====================================================================
// AT-G4 … AT-G6 — timeline poison evidence
// =====================================================================

fn epoch() -> TimelineEpoch {
    TimelineEpoch(1)
}

fn envelope(index: usize) -> SemanticCommandEnvelope {
    let name = format!("cmd.{index:05}");
    SemanticCommandEnvelope {
        command_id: CommandId::new(&name).unwrap(),
        profile_id: profile(),
        timeline_epoch: epoch(),
        effective_time: LogicalTime(0),
        source_id: sequencer(),
        source_sequence: 0,
        input_ordinal: Ordinal(0),
        command_kind: CommandKind::new(CanonicalTag::new("test.command").unwrap()),
        canonical_payload_hash: hash_bytes(name.as_bytes()),
    }
}

/// Envelopes ordered by their semantic hash, so a test can state exactly
/// which competing identities fall inside the exposed poison window.
///
/// Unlike a scheduler payload hash, a semantic hash is *computed* from the
/// envelope and cannot be chosen, so the ordering is discovered here
/// rather than constructed.
fn envelope_pool(count: usize) -> Vec<SemanticCommandEnvelope> {
    let mut pool: Vec<SemanticCommandEnvelope> = (0..count).map(envelope).collect();
    pool.sort_by_key(|e| e.semantic_hash());
    pool
}

fn poisoned_ingress(envelopes: &[SemanticCommandEnvelope]) -> TimelineIngress {
    let mut ingress = TimelineIngress::new(profile(), epoch(), sequencer(), 2).unwrap();
    for envelope in envelopes {
        let ticket = ingress
            .current_admission_window()
            .expect("the window is open at frontier 0")
            .ticket();
        let _ = ingress.stage(&sequencer(), &envelope.clone().submit_with(ticket));
    }
    assert_eq!(ingress.slot_status(Ordinal(0)), SlotStatus::Poisoned);
    ingress
}

/// The whole exposed surface of a poisoned slot, as a caller sees it.
fn exposed_poison(ingress: &TimelineIngress) -> (BTreeSet<Digest>, u64, bool, SlotStatus) {
    let evidence = ingress
        .poison_evidence(Ordinal(0))
        .expect("the ordinal under test is contested and therefore poisoned");
    (
        evidence.competing_semantic_hashes(),
        evidence.omitted_distinct(),
        evidence.evidence_truncated(),
        ingress.slot_status(Ordinal(0)),
    )
}

/// The poison analogue of [`hidden_evidence_pair`]: same sixteen smallest
/// semantic hashes, same tracked count, different hidden claims.
fn hidden_poison_pair() -> (
    Vec<SemanticCommandEnvelope>,
    Vec<SemanticCommandEnvelope>,
    SemanticCommandEnvelope,
) {
    let pool = envelope_pool(MAX_POISON_EVIDENCE + 8);
    let common = &pool[..MAX_POISON_EVIDENCE];

    let mut left = common.to_vec();
    left.extend_from_slice(&pool[MAX_POISON_EVIDENCE..MAX_POISON_EVIDENCE + 2]);
    let mut right = common.to_vec();
    right.extend_from_slice(&pool[MAX_POISON_EVIDENCE + 2..MAX_POISON_EVIDENCE + 4]);

    // A claim already hidden-tracked by `left` and unknown to `right`.
    let shared_next = pool[MAX_POISON_EVIDENCE].clone();
    (left, right, shared_next)
}

/// **AT-G4** — the scheduler defect is structurally duplicated in the
/// timeline: two poisoned slots identical across the entire exposed
/// surface but holding different hidden tracked claims must not share a
/// canonical state digest.
#[test]
fn hidden_poison_evidence_is_committed_to_canonical_state() {
    let (left_envelopes, right_envelopes, _) = hidden_poison_pair();
    let left = poisoned_ingress(&left_envelopes);
    let right = poisoned_ingress(&right_envelopes);

    assert_eq!(exposed_poison(&left), exposed_poison(&right));
    assert_eq!(exposed_poison(&left).1, 2);
    assert!(!exposed_poison(&left).2);

    assert_ne!(
        left.canonical_state_digest(),
        right.canonical_state_digest(),
        "poisoned slots with different retained tracked claims must not \
         share a canonical state digest"
    );

    // The finalized-history rules are untouched by any of this: nothing
    // was finalized, so both agree there.
    assert_eq!(
        left.canonical_history_digest(),
        right.canonical_history_digest()
    );
}

/// **AT-G5** — the hidden poison difference is behavior-relevant under
/// the same next claim.
#[test]
fn hidden_poison_evidence_changes_the_reaction_to_the_next_claim() {
    let (left_envelopes, right_envelopes, shared_next) = hidden_poison_pair();
    let mut left = poisoned_ingress(&left_envelopes);
    let mut right = poisoned_ingress(&right_envelopes);

    let before_left = left.canonical_state_digest();
    let before_right = right.canonical_state_digest();

    for ingress in [&mut left, &mut right] {
        let ticket = ingress.current_admission_window().unwrap().ticket();
        let _ = ingress.stage(&sequencer(), &shared_next.clone().submit_with(ticket));
    }

    assert_eq!(exposed_poison(&left).1, 2);
    assert_eq!(exposed_poison(&right).1, 3);
    assert_eq!(before_left, left.canonical_state_digest());
    assert_ne!(before_right, right.canonical_state_digest());

    assert_ne!(
        before_left, before_right,
        "poisoned slots that react differently to the same next claim \
         must not have shared a canonical state digest"
    );
}

/// **AT-G6** — poison evidence stays arrival-order independent at every
/// mandated set size once the hidden claims are canonicalized.
#[test]
fn poison_permutations_converge_at_every_mandated_set_size() {
    let pool = envelope_pool(*MANDATED_SET_SIZES.iter().max().unwrap());
    let exposed_window: BTreeSet<Digest> = pool
        .iter()
        .take(MAX_POISON_EVIDENCE)
        .map(|e| e.semantic_hash())
        .collect();

    for size in MANDATED_SET_SIZES {
        let claims = &pool[..size];

        let mut digests: BTreeSet<Digest> = BTreeSet::new();
        for order in arrival_orders(claims) {
            let ingress = poisoned_ingress(&order);
            digests.insert(ingress.canonical_state_digest());

            let (hashes, omitted, truncated, status) = exposed_poison(&ingress);
            assert_eq!(status, SlotStatus::Poisoned);
            assert_eq!(hashes, exposed_window);
            let tracked = size.min(MAX_POISON_TRACKED_CLAIMS);
            assert_eq!(omitted as usize, tracked - MAX_POISON_EVIDENCE);
            assert_eq!(truncated, size > MAX_POISON_TRACKED_CLAIMS);
        }
        assert_eq!(digests.len(), 1, "poison set size {size} did not converge");
    }
}

/// **AT-G6 (companion)** — poison claims dropped past the tracking cap
/// are collapsed, exactly as the scheduler's are.
#[test]
fn poison_claims_forgotten_beyond_the_tracking_cap_stay_collapsed() {
    let pool = envelope_pool(MAX_POISON_TRACKED_CLAIMS + 4);
    let base = &pool[..MAX_POISON_TRACKED_CLAIMS];

    let mut left = base.to_vec();
    left.extend_from_slice(&pool[MAX_POISON_TRACKED_CLAIMS..MAX_POISON_TRACKED_CLAIMS + 2]);
    let mut right = base.to_vec();
    right.extend_from_slice(&pool[MAX_POISON_TRACKED_CLAIMS + 2..MAX_POISON_TRACKED_CLAIMS + 4]);

    let left = poisoned_ingress(&left);
    let right = poisoned_ingress(&right);

    assert!(exposed_poison(&left).2);
    assert_eq!(
        left.canonical_state_digest(),
        right.canonical_state_digest()
    );
}
