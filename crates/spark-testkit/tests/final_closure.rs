//! **Final closure corpus (AT-H).**
//!
//! `PHASE_1_FABLE_FINAL_ADMISSION_ARCHITECTURE_2026-08-26.md` states the
//! rule the Re-Foundation v2 tree satisfies everywhere except one place:
//!
//! > **R1 (state commitment).** Every retained internal state value that
//! > can change future canonical behavior participates in canonical state
//! > identity.
//! > **R2 (order independence).** Canonical state reached from the same
//! > claim set is identical in every arrival order.
//!
//! [`spark_core::timeline`]'s two in-flight identity registries —
//! command ID and `(source_id, source_sequence)` — violated both at once:
//! only the *first* arrival at a contested ordinal ever registered its
//! identity claims, and those claims were never withdrawn when the slot
//! poisoned. Two ingresses fed the same contesting envelopes in opposite
//! orders therefore reached **equal canonical state digests** and then
//! answered the very same later identity-reuse question differently.
//!
//! The repair makes the staged registries a *derived index over the
//! positively staged slots and nothing else*, so contested identities
//! become unclaimed after the contest, identically in every arrival
//! order. That is a structural fix: the digest already commits to every
//! staged envelope in full, so equal digests now imply equal registries
//! and therefore equal future admission behavior. No digest encoding, no
//! public signature, and no frozen contract changes.
//!
//! These assertions were written and run against the inherited tree
//! first. The recorded red baseline is
//! `engineering/phase1/final-closure-baseline/BASELINE_FINAL_CLOSURE_FAILURES.txt`.
//!
//! Two disciplines carry over from the AT-F/AT-G corpora:
//!
//! 1. **"Impossible" is proved by a value equality, never by an
//!    error-shape check.** Every order-independence test compares
//!    `canonical_state_digest` values *and* whole
//!    `Result<StageDisposition, StageError>` values, so a repair that
//!    merely changed which error variant fires would not satisfy it.
//! 2. **A structural claim is pinned from both sides.** AT-H5 exists to
//!    make the *deliberate* asymmetry — identity screening guards
//!    admission into positive staging only, while poison evidence is an
//!    unconditional set insert — fail loudly if a future writer
//!    "completes" the identity checks into the poison branch and thereby
//!    re-introduces arrival-order sensitivity inside the repair itself.
//!
//! AT-H3 and AT-H6's registry-content assertions require access to the
//! private registries and therefore live as in-crate unit tests in
//! `spark_core::timeline` (the matrix sanctions either form); everything
//! else is here, against the public surface only.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, Digest};
use spark_core::id::{CanonicalTag, CommandId, ProfileId, SourceId};
use spark_core::timeline::{
    AcknowledgedSlotState, CommandKind, Ordinal, SemanticCommandEnvelope, SlotStatus,
    StageDisposition, StageError, TimelineEpoch, TimelineIngress, MAX_POISON_EVIDENCE,
    MAX_POISON_TRACKED_CLAIMS,
};
use std::collections::BTreeSet;

fn profile() -> ProfileId {
    ProfileId::new("game-world").unwrap()
}

fn sequencer() -> SourceId {
    SourceId::new("sequencer.primary").unwrap()
}

fn epoch() -> TimelineEpoch {
    TimelineEpoch(1)
}

/// A semantic envelope whose command ID, ordinal, source sequence, and
/// payload are all nameable, so contests and identity reuses can be
/// constructed exactly.
fn env(
    command: &str,
    ordinal: u64,
    source_sequence: u64,
    payload: &str,
) -> SemanticCommandEnvelope {
    SemanticCommandEnvelope {
        command_id: CommandId::new(command).unwrap(),
        profile_id: profile(),
        timeline_epoch: epoch(),
        effective_time: LogicalTime(0),
        source_id: sequencer(),
        source_sequence,
        input_ordinal: Ordinal(ordinal),
        command_kind: CommandKind::new(CanonicalTag::new("test.command").unwrap()),
        canonical_payload_hash: hash_bytes(payload.as_bytes()),
    }
}

fn ingress(width: u32) -> TimelineIngress {
    TimelineIngress::new(profile(), epoch(), sequencer(), width).unwrap()
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

/// An ingress fed one arrival order of a claim set.
fn fed(width: u32, arrivals: &[SemanticCommandEnvelope]) -> TimelineIngress {
    let mut ingress = ingress(width);
    for envelope in arrivals {
        let _ = stage(&mut ingress, envelope);
    }
    ingress
}

fn assert_newly_staged(result: &Result<StageDisposition, StageError>) {
    match result {
        Ok(StageDisposition::Acknowledged(ack)) => {
            assert_eq!(
                ack.slot_state(),
                AcknowledgedSlotState::NewlyStaged,
                "expected a fresh positive staging acknowledgement"
            );
        }
        other => panic!("expected Acknowledged(NewlyStaged), got {other:?}"),
    }
}

/// Every arrival order of a claim set. Reversal alone can be satisfied by
/// an accidentally symmetric implementation; the three-contestant cases
/// below need the full permutation group to catch a repair that is
/// correct for first/last arrivals and wrong in the middle.
fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.is_empty() {
        return vec![Vec::new()];
    }
    let mut orders = Vec::new();
    for (index, item) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(index);
        for tail in permutations(&rest) {
            let mut order = vec![item.clone()];
            order.extend(tail);
            orders.push(order);
        }
    }
    orders
}

/// The exposed poison surface of one ordinal, as a caller sees it.
fn exposed_poison(
    ingress: &TimelineIngress,
    ordinal: u64,
) -> (BTreeSet<Digest>, u64, bool, SlotStatus) {
    let evidence = ingress
        .poison_evidence(Ordinal(ordinal))
        .expect("the ordinal under test is contested and therefore poisoned");
    (
        evidence.competing_semantic_hashes(),
        evidence.omitted_distinct(),
        evidence.evidence_truncated(),
        ingress.slot_status(Ordinal(ordinal)),
    )
}

// =====================================================================
// AT-H1 / AT-H2 — contested identity claims are arrival-order independent
// =====================================================================

/// **AT-H1** — probe P1 flipped from falsification to invariant.
///
/// Envelopes `cmd.a` and `cmd.b` contest ordinal 0 in both orders. The
/// contest itself already converged before the repair (assertion (a),
/// kept as a regression guard); what did **not** converge is what the two
/// ingresses do next. Reusing either contestant's command ID for a
/// semantically different envelope must produce the same disposition
/// *value* and leave the same canonical state digest, whichever
/// contestant happened to arrive first.
#[test]
fn contested_command_identity_is_arrival_order_independent() {
    let a = env("cmd.a", 0, 0, "a");
    let b = env("cmd.b", 0, 1, "b");

    let mut forward = fed(4, &[a.clone(), b.clone()]);
    let mut reversed = fed(4, &[b, a]);

    // (a) The contest converges — the closed B-01/AT-F1 property.
    assert_eq!(forward.slot_status(Ordinal(0)), SlotStatus::Poisoned);
    assert_eq!(reversed.slot_status(Ordinal(0)), SlotStatus::Poisoned);
    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest(),
        "the contest itself must already be arrival-order independent"
    );

    // (b)/(c) Reuse of the first-arrival contestant's command ID.
    let reuse_a = env("cmd.a", 1, 7, "different-a");
    let forward_reuse_a = stage(&mut forward, &reuse_a);
    let reversed_reuse_a = stage(&mut reversed, &reuse_a);
    assert_eq!(
        forward_reuse_a, reversed_reuse_a,
        "two ingresses holding equal canonical state must answer the same \
         identity-reuse submission identically"
    );
    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest(),
        "the reuse must leave both ingresses in equal canonical state"
    );

    // (d) ...and of the second-arrival contestant's command ID.
    let reuse_b = env("cmd.b", 2, 8, "different-b");
    let forward_reuse_b = stage(&mut forward, &reuse_b);
    let reversed_reuse_b = stage(&mut reversed, &reuse_b);
    assert_eq!(forward_reuse_b, reversed_reuse_b);
    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest()
    );
}

/// **AT-H2** — probe P2: the identical defect through the
/// `(source_id, source_sequence)` registry. A repair that treated only
/// the command-ID registry would pass AT-H1 and fail here.
#[test]
fn contested_source_sequence_is_arrival_order_independent() {
    let a = env("cmd.a", 0, 5, "a");
    let b = env("cmd.b", 0, 6, "b");

    let mut forward = fed(4, &[a.clone(), b.clone()]);
    let mut reversed = fed(4, &[b, a]);

    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest()
    );

    // Reuse of the first-arrival contestant's source-sequence pair, under
    // a command ID that has never been claimed.
    let reuse_five = env("cmd.c", 1, 5, "different-five");
    let forward_five = stage(&mut forward, &reuse_five);
    let reversed_five = stage(&mut reversed, &reuse_five);
    assert_eq!(forward_five, reversed_five);
    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest()
    );

    // ...and of the second-arrival contestant's pair.
    let reuse_six = env("cmd.d", 2, 6, "different-six");
    let forward_six = stage(&mut forward, &reuse_six);
    let reversed_six = stage(&mut reversed, &reuse_six);
    assert_eq!(forward_six, reversed_six);
    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest()
    );
}

// =====================================================================
// AT-H4 — the chosen semantics: contested identities become unclaimed
// =====================================================================

/// **AT-H4** — probe P5 flipped.
///
/// "No arrival picks a winner" (ADR-0003 §11) read coherently means
/// **neither** contestant's claim survives a contest, including its
/// identity reservation. After A and B contest ordinal 0, both `cmd.a`
/// and `cmd.b` — and both contestants' source-sequence pairs — are free
/// for a semantically different envelope to claim at another ordinal, in
/// either arrival order.
#[test]
fn contested_identities_are_unclaimed_after_the_contest() {
    let a = env("cmd.a", 0, 0, "a");
    let b = env("cmd.b", 0, 1, "b");

    for arrivals in [[a.clone(), b.clone()], [b.clone(), a.clone()]] {
        let mut ingress = fed(4, &arrivals);
        assert_eq!(ingress.slot_status(Ordinal(0)), SlotStatus::Poisoned);

        let reuse_a = env("cmd.a", 1, 7, "different-a");
        let reuse_b = env("cmd.b", 2, 8, "different-b");
        assert_newly_staged(&stage(&mut ingress, &reuse_a));
        assert_newly_staged(&stage(&mut ingress, &reuse_b));
        assert_eq!(ingress.slot_status(Ordinal(1)), SlotStatus::Staged);
        assert_eq!(ingress.slot_status(Ordinal(2)), SlotStatus::Staged);
    }

    // The two orders agree on canonical state at every step, not merely
    // on the final answer.
    let mut forward = fed(4, &[a.clone(), b.clone()]);
    let mut reversed = fed(4, &[b, a]);
    for reuse in [
        env("cmd.a", 1, 7, "different-a"),
        env("cmd.b", 2, 8, "different-b"),
    ] {
        assert_eq!(stage(&mut forward, &reuse), stage(&mut reversed, &reuse));
        assert_eq!(
            forward.canonical_state_digest(),
            reversed.canonical_state_digest()
        );
    }

    // ...and the same for the contestants' source-sequence pairs, which
    // the previous loop deliberately left untouched by using fresh ones.
    let mut forward = fed(6, &[env("cmd.a", 0, 0, "a"), env("cmd.b", 0, 1, "b")]);
    let mut reversed = fed(6, &[env("cmd.b", 0, 1, "b"), env("cmd.a", 0, 0, "a")]);
    for reuse in [
        env("cmd.reuse-seq-0", 3, 0, "seq-zero-again"),
        env("cmd.reuse-seq-1", 4, 1, "seq-one-again"),
    ] {
        let forward_result = stage(&mut forward, &reuse);
        assert_newly_staged(&forward_result);
        assert_eq!(forward_result, stage(&mut reversed, &reuse));
        assert_eq!(
            forward.canonical_state_digest(),
            reversed.canonical_state_digest()
        );
    }
}

// =====================================================================
// AT-H5 — the load-bearing asymmetry the repair must not "complete"
// =====================================================================

/// **AT-H5** — regression pin for probe P3 (architecture §4.3).
///
/// Identity screening guards **admission into positive staging** only. A
/// claim recorded as contest evidence is recorded by semantic hash,
/// unconditionally — evidence is a record of contest, not an admission.
/// Screening the poison branch against the identity registries would make
/// the evidence *set* a function of registry contents, i.e. of arrival
/// order, re-introducing the very defect this pass repairs.
///
/// This test therefore asserts the asymmetry from both sides and pins the
/// resulting evidence set exactly: a claimant whose command ID collides
/// with a *positively staged* envelope at another ordinal is
/// identity-rejected at an empty ordinal and yet is recorded, in full, as
/// evidence when it lands on an occupied one.
#[test]
fn poison_evidence_insertion_is_unconditional_and_unscreened() {
    let staged_elsewhere = env("cmd.elsewhere", 1, 1, "elsewhere");
    let occupant = env("cmd.occupant", 0, 0, "occupant");
    // Same command ID as the envelope positively staged at ordinal 1,
    // different semantics.
    let colliding = env("cmd.elsewhere", 0, 2, "colliding");

    // Side 1: the empty ordinal screens it.
    let mut screened = fed(4, std::slice::from_ref(&staged_elsewhere));
    let at_empty = stage(&mut screened, &env("cmd.elsewhere", 2, 3, "colliding-two"));
    assert_eq!(
        at_empty,
        Err(StageError::CommandIdentityConflict {
            command_id: CommandId::new("cmd.elsewhere").unwrap()
        }),
        "identity screening still guards admission into positive staging"
    );
    assert_eq!(screened.slot_status(Ordinal(2)), SlotStatus::Empty);

    // Side 2: the occupied ordinal records it, unconditionally.
    let mut contested = fed(4, &[staged_elsewhere.clone(), occupant.clone()]);
    let at_occupied = stage(&mut contested, &colliding);
    assert!(
        matches!(at_occupied, Ok(StageDisposition::Poisoned(_))),
        "a claim on an occupied slot is recorded as contest evidence, not \
         screened against the identity registries; got {at_occupied:?}"
    );
    let (hashes, omitted, truncated, status) = exposed_poison(&contested, 0);
    assert_eq!(status, SlotStatus::Poisoned);
    assert_eq!(omitted, 0);
    assert!(!truncated);
    assert!(
        hashes.contains(&colliding.semantic_hash()),
        "the colliding claimant's semantic hash must appear in the slot's \
         evidence: evidence is a record of contest, not an admission"
    );
    assert_eq!(
        hashes,
        BTreeSet::from([occupant.semantic_hash(), colliding.semantic_hash()])
    );

    // The unconditional insert is what keeps further claims on an already
    // poisoned slot commutative and idempotent, including claims whose
    // identities collide with positively staged envelopes elsewhere.
    let pile_on: Vec<SemanticCommandEnvelope> = vec![
        env("cmd.elsewhere", 0, 4, "pile-on-colliding-id"),
        env("cmd.pile-on", 0, 1, "pile-on-colliding-seq"),
        env("cmd.pile-on-fresh", 0, 9, "pile-on-fresh"),
    ];
    let expected: BTreeSet<Digest> = [occupant.semantic_hash(), colliding.semantic_hash()]
        .into_iter()
        .chain(pile_on.iter().map(|e| e.semantic_hash()))
        .collect();

    let mut digests: BTreeSet<Digest> = BTreeSet::new();
    for order in permutations(&pile_on) {
        let mut ingress = fed(
            4,
            &[
                staged_elsewhere.clone(),
                occupant.clone(),
                colliding.clone(),
            ],
        );
        for claim in &order {
            assert!(matches!(
                stage(&mut ingress, claim),
                Ok(StageDisposition::Poisoned(_))
            ));
        }
        assert_eq!(
            exposed_poison(&ingress, 0).0,
            expected,
            "every claim on a poisoned slot is evidence, whatever its \
             identity claims collide with"
        );
        digests.insert(ingress.canonical_state_digest());
    }
    assert_eq!(
        digests.len(),
        1,
        "claims on a poisoned slot must commute into one canonical state"
    );
}

// =====================================================================
// AT-H7 — epoch reset convergence stays a permanent guard
// =====================================================================

/// **AT-H7** — probe P4 as a permanent guard. An ADR-0003 §11 epoch reset
/// discards all unfinalized staged state, so two ingresses that contested
/// the same ordinal in opposite orders are indistinguishable afterwards,
/// including in their responses to identical post-reset submissions.
#[test]
fn epoch_reset_convergence_is_preserved() {
    let a = env("cmd.a", 0, 0, "a");
    let b = env("cmd.b", 0, 1, "b");

    let mut forward = fed(4, &[a.clone(), b.clone()]);
    let mut reversed = fed(4, &[b, a]);

    forward
        .reset_epoch(&sequencer(), TimelineEpoch(2), sequencer())
        .unwrap();
    reversed
        .reset_epoch(&sequencer(), TimelineEpoch(2), sequencer())
        .unwrap();
    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest()
    );

    for reuse in [
        env("cmd.a", 0, 7, "different-a"),
        env("cmd.b", 1, 8, "different-b"),
        env("cmd.fresh", 2, 9, "fresh"),
    ] {
        let mut reuse = reuse;
        reuse.timeline_epoch = TimelineEpoch(2);
        let forward_result = stage(&mut forward, &reuse);
        assert_newly_staged(&forward_result);
        assert_eq!(forward_result, stage(&mut reversed, &reuse));
        assert_eq!(
            forward.canonical_state_digest(),
            reversed.canonical_state_digest()
        );
    }
}

// =====================================================================
// AT-H8 — past the pair case
// =====================================================================

/// **AT-H8** — three contestants on one ordinal, in all six arrival
/// orders, then the full battery of reuse probes: each contested command
/// ID, each contested source-sequence pair, and one genuinely fresh
/// identity. All six ingresses must agree on the post-contest state
/// digest, on every disposition value, and on the digest after each
/// probe.
///
/// A repair that deregistered only the *staged* contestant on the first
/// poisoning transition but left a middle arrival's claims behind would
/// pass AT-H1 and fail here.
#[test]
fn three_way_contest_with_reuse_converges_in_all_orders() {
    let contestants = vec![
        env("cmd.a", 0, 0, "a"),
        env("cmd.b", 0, 1, "b"),
        env("cmd.c", 0, 2, "c"),
    ];

    // Each probe reuses exactly one contested identity facet at a fresh
    // ordinal, plus one probe that claims nothing contested at all.
    let probes = vec![
        env("cmd.a", 1, 11, "reuse-command-a"),
        env("cmd.b", 2, 12, "reuse-command-b"),
        env("cmd.c", 3, 13, "reuse-command-c"),
        env("cmd.probe-seq-0", 4, 0, "reuse-sequence-0"),
        env("cmd.probe-seq-1", 5, 1, "reuse-sequence-1"),
        env("cmd.probe-seq-2", 6, 2, "reuse-sequence-2"),
        env("cmd.probe-fresh", 7, 99, "fresh"),
    ];

    let mut ingresses: Vec<TimelineIngress> = Vec::new();
    let mut contest_digests: BTreeSet<Digest> = BTreeSet::new();
    for order in permutations(&contestants) {
        let ingress = fed(8, &order);
        assert_eq!(ingress.slot_status(Ordinal(0)), SlotStatus::Poisoned);
        contest_digests.insert(ingress.canonical_state_digest());
        ingresses.push(ingress);
    }
    assert_eq!(ingresses.len(), 6);
    assert_eq!(
        contest_digests.len(),
        1,
        "a three-way contest must converge in every arrival order"
    );

    for probe in &probes {
        let mut results: Vec<Result<StageDisposition, StageError>> = Vec::new();
        let mut digests: BTreeSet<Digest> = BTreeSet::new();
        for ingress in ingresses.iter_mut() {
            results.push(stage(ingress, probe));
            digests.insert(ingress.canonical_state_digest());
        }
        assert_newly_staged(&results[0]);
        assert!(
            results.windows(2).all(|pair| pair[0] == pair[1]),
            "arrival order decided the answer to probe {:?}: {results:?}",
            probe.command_id
        );
        assert_eq!(
            digests.len(),
            1,
            "arrival order decided canonical state after probe {:?}",
            probe.command_id
        );
    }
}

// =====================================================================
// AT-H9 — the closed hidden-evidence guards, restated
// =====================================================================

fn poison_pool(count: usize) -> Vec<SemanticCommandEnvelope> {
    let mut pool: Vec<SemanticCommandEnvelope> = (0..count)
        .map(|index| {
            let name = format!("cmd.{index:05}");
            env(&name, 0, 0, &name)
        })
        .collect();
    pool.sort_by_key(|envelope| envelope.semantic_hash());
    pool
}

fn poisoned_by(claims: &[SemanticCommandEnvelope]) -> TimelineIngress {
    let ingress = fed(2, claims);
    assert_eq!(ingress.slot_status(Ordinal(0)), SlotStatus::Poisoned);
    ingress
}

/// **AT-H9** — the closure claim that nothing the AT-F/AT-G passes closed
/// was traded away for this repair, restated as executable assertions:
/// poison evidence still commits to **every** tracked claim rather than
/// to its exposed projection, claims beyond the tracking cap are still
/// deliberately collapsed, and poisoned-state digests are still
/// arrival-order independent.
#[test]
fn hidden_evidence_guards_stay_green() {
    // Full tracked evidence is still committed (AT-G4): identical exposed
    // surfaces, different hidden claims, different state digests.
    let pool = poison_pool(MAX_POISON_EVIDENCE + 4);
    let mut left = pool[..MAX_POISON_EVIDENCE].to_vec();
    left.extend_from_slice(&pool[MAX_POISON_EVIDENCE..MAX_POISON_EVIDENCE + 1]);
    let mut right = pool[..MAX_POISON_EVIDENCE].to_vec();
    right.extend_from_slice(&pool[MAX_POISON_EVIDENCE + 1..MAX_POISON_EVIDENCE + 2]);

    let left_ingress = poisoned_by(&left);
    let right_ingress = poisoned_by(&right);
    assert_eq!(
        exposed_poison(&left_ingress, 0),
        exposed_poison(&right_ingress, 0)
    );
    assert_ne!(
        left_ingress.canonical_state_digest(),
        right_ingress.canonical_state_digest(),
        "hidden tracked poison claims must still be committed to canonical state"
    );

    // Cap collapse is still deliberate (AT-G6 companion).
    let capped = poison_pool(MAX_POISON_TRACKED_CLAIMS + 4);
    let base = &capped[..MAX_POISON_TRACKED_CLAIMS];
    let mut over_left = base.to_vec();
    over_left.extend_from_slice(&capped[MAX_POISON_TRACKED_CLAIMS..MAX_POISON_TRACKED_CLAIMS + 2]);
    let mut over_right = base.to_vec();
    over_right
        .extend_from_slice(&capped[MAX_POISON_TRACKED_CLAIMS + 2..MAX_POISON_TRACKED_CLAIMS + 4]);
    let over_left = poisoned_by(&over_left);
    let over_right = poisoned_by(&over_right);
    assert!(exposed_poison(&over_left, 0).2);
    assert_eq!(
        over_left.canonical_state_digest(),
        over_right.canonical_state_digest(),
        "claims dropped past the tracking cap are genuinely forgotten and \
         must stay collapsed"
    );

    // Poisoned-state digests are still arrival-order independent (AT-F1).
    let claims = &pool[..MAX_POISON_EVIDENCE];
    let forward = poisoned_by(claims);
    let reversed: Vec<SemanticCommandEnvelope> = claims.iter().rev().cloned().collect();
    let reversed = poisoned_by(&reversed);
    let interleaved: Vec<SemanticCommandEnvelope> = claims
        .iter()
        .step_by(3)
        .chain(claims.iter().skip(1).step_by(3))
        .chain(claims.iter().skip(2).step_by(3))
        .cloned()
        .collect();
    let interleaved = poisoned_by(&interleaved);
    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest()
    );
    assert_eq!(
        forward.canonical_state_digest(),
        interleaved.canonical_state_digest()
    );
}

// =====================================================================
// AT-H11 — m-02: the manifest content hash is a multiset function
// =====================================================================

mod manifest_content_hash {
    use spark_core::authority::Authority;
    use spark_core::hash::Digest;
    use spark_core::id::{DefinitionId, ProfileId};
    use spark_core::scope::ScopeKind;
    use spark_core::value::ValueConstraint;
    use spark_engine::profile::definition::{DefinitionKind, DefinitionSpec};
    use spark_engine::profile::manifest::ProfileManifest;
    use spark_engine::profile::text::BoundedText;
    use std::collections::BTreeSet;

    fn profile() -> ProfileId {
        ProfileId::new("game-world").unwrap()
    }

    fn definition(id: &str, description: &str) -> DefinitionSpec {
        DefinitionSpec {
            profile_id: profile(),
            id: DefinitionId::new(id).unwrap(),
            kind: DefinitionKind::Trait,
            domain: None,
            layer: None,
            value_constraint: ValueConstraint::boolean(),
            authority: Authority::SparkOwned,
            valid_scopes: BTreeSet::from([ScopeKind::Actor]),
            enabled: true,
            version: 1,
            description: BoundedText::new(description).unwrap(),
            behavioral_leverage: None,
        }
    }

    fn manifest(definitions: Vec<DefinitionSpec>) -> ProfileManifest {
        ProfileManifest::new(profile(), BoundedText::new("1.0.0").unwrap(), definitions)
    }

    /// **AT-H11** — probe P6 flipped.
    ///
    /// ADR-0004 requires the canonical logical representation used for
    /// content hashing to be deterministic and format-independent. Sorting
    /// definitions by ID alone left the hash construction-order sensitive
    /// whenever two *different* definitions shared one ID: such a manifest
    /// is invalid and can never activate, so no artifact carries the hash,
    /// but a canonical hash function that is order-sensitive on any input
    /// is a latent trap for later authoring/artifact tooling that hashes
    /// unvalidated manifests. The hash must be a multiset function on
    /// every input, valid or not.
    #[test]
    fn duplicate_id_manifest_hash_is_order_independent() {
        let first = definition("trait.curiosity", "first");
        let second = definition("trait.curiosity", "second");

        let forward = manifest(vec![first.clone(), second.clone()]);
        let reversed = manifest(vec![second, first]);

        assert_eq!(
            forward.manifest_content_hash(),
            reversed.manifest_content_hash(),
            "the manifest content hash must be a multiset function of the \
             definitions, including on invalid duplicate-ID input"
        );
    }

    /// **AT-H11 (regression companion)** — the m-02 fix must not move any
    /// *valid* manifest's content hash. The expected digest below was
    /// recorded from the inherited tree before the fix, so a change to the
    /// canonical encoding — as opposed to a change to the tie-break used
    /// for duplicate IDs — turns this red.
    #[test]
    fn valid_manifest_hashes_are_unchanged_by_the_duplicate_id_fix() {
        let curiosity = definition("trait.curiosity", "Curiosity trait.");
        let mut trust = definition("relationship.trust", "Trust relationship dimension.");
        trust.kind = DefinitionKind::Relationship;

        let forward = manifest(vec![curiosity.clone(), trust.clone()]);
        let reversed = manifest(vec![trust, curiosity]);

        assert_eq!(
            forward.manifest_content_hash(),
            reversed.manifest_content_hash(),
            "definition order never affected a valid manifest's hash"
        );
        assert_eq!(
            forward.manifest_content_hash(),
            Digest(INHERITED_VALID_MANIFEST_HASH),
            "the m-02 fix must not change the content hash of any manifest \
             that could actually activate"
        );
    }

    /// Recorded from the inherited tree at the AT-H red baseline.
    const INHERITED_VALID_MANIFEST_HASH: [u8; 32] = [
        0x9b, 0xdc, 0x65, 0xbf, 0xcf, 0x8d, 0x9f, 0xea, 0xa2, 0x3a, 0x7b, 0x35, 0x21, 0xaf, 0xaf,
        0xdf, 0x21, 0xa7, 0x7d, 0x24, 0x17, 0xfc, 0xcb, 0xd8, 0x8c, 0x78, 0xbd, 0xac, 0xc6, 0x15,
        0xc1, 0x2c,
    ];
}
