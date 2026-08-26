//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Phase-1 re-foundation permanent adversarial corpus (v1 line, ported to
//! the Re-Foundation v2 crate boundary).
//!
//! Every test here encodes one counterexample from
//! `engineering/phase1/PHASE_1_REFOUNDATION_BRIEF_v0.1.md` that survived
//! the bounded correction pass. They live in `spark-testkit` — a crate
//! separate from `spark-core` and `spark-engine` — deliberately: this is
//! the vantage point an integrator or a Phase-2 crate has, so anything
//! reachable from here is genuinely part of the public boundary.
//!
//! Every one of these failed against the pre-re-foundation implementation;
//! the recorded baseline is
//! `engineering/phase1/refoundation-baseline/BASELINE_ADVERSARIAL_FAILURES.txt`.
//! They are **ported, not rewritten**: no assertion here was weakened by
//! the v2 crate reshape. The two that independent review found materially
//! insufficient — `definition_spec_cannot_self_activate` (which used an
//! *invalid* spec, so it never exercised the bypass) and
//! `payload_conflict_rejects_in_either_order` (which compared only error
//! shape and length) — are replaced by strictly stronger assertions in
//! `refoundation_v2_adversarial.rs`, and the weak forms are gone.
//!
//! Requirements whose violation is a *type* error rather than a runtime
//! one — forging an `ActivatedProfile`, canonicalizing an
//! `AdmissionTicket`, building a `ValueConstraint` with `min > max`,
//! constructing a `StateStore` without an activation — are encoded as
//! `compile_fail` doc-tests on the affected types plus the external
//! compile probes in `external_compile_probes.rs`.

use spark_core::authority::Authority;
use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, Digest};
use spark_core::id::{
    CanonicalTag, CommandId, DefinitionId, FenceId, ProfileId, SourceId, StableIdError,
};
use spark_core::scheduler::{
    DueWorkItem, OccurrenceIndex, ScheduleDisposition, Scheduler, WorkKey, WorkKind, WorkPayload,
    WorkSlotStatus,
};
use spark_core::scope::{ScopeId, ScopeKind};
use spark_core::timeline::{
    compute_ordered_stream_digest, AcknowledgedSlotState, CommandKind, FenceError, Ordinal,
    SemanticCommandEnvelope, StageDisposition, TimelineEpoch, TimelineFence, TimelineIngress,
};
use spark_core::value::{
    CanonicalValue, CategoricalValue, ValueConstraint, ValueConstraintError,
    MAX_CATEGORICAL_VALUE_LEN,
};
use spark_engine::activation::{definition_fingerprint, ActivationRegistry, ValidationError};
use spark_engine::profile::config::{ConfigEntry, ConfigRevision, ConfigRevisionError};
use spark_engine::profile::definition::{DefinitionKind, DefinitionSpec};
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::profile::text::BoundedText;
use spark_engine::state::StateStore;
use std::collections::BTreeSet;

fn profile() -> ProfileId {
    ProfileId::new("game-world").unwrap()
}

fn sequencer() -> SourceId {
    SourceId::new("sequencer.primary").unwrap()
}

// =====================================================================
// Finality / admission
// =====================================================================

fn envelope(scenario_epoch: TimelineEpoch, ordinal: u64) -> SemanticCommandEnvelope {
    SemanticCommandEnvelope {
        command_id: CommandId::new(format!("cmd.{ordinal}")).unwrap(),
        profile_id: profile(),
        timeline_epoch: scenario_epoch,
        effective_time: LogicalTime(ordinal),
        source_id: sequencer(),
        source_sequence: ordinal,
        input_ordinal: Ordinal(ordinal),
        command_kind: CommandKind::new(CanonicalTag::new("test.command").unwrap()),
        canonical_payload_hash: hash_bytes(format!("payload.{ordinal}").as_bytes()),
    }
}

fn stage_now(
    ingress: &mut TimelineIngress,
    envelope: &SemanticCommandEnvelope,
) -> StageDisposition {
    let ticket = ingress.current_admission_window().unwrap().ticket();
    ingress
        .stage(&sequencer(), &envelope.clone().submit_with(ticket))
        .unwrap()
}

fn fence_over(
    ingress: &TimelineIngress,
    start: u64,
    end: u64,
    tag: &str,
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

/// Finality test 1: two runs finalize the *same* semantic commands behind
/// the *same* fence chain, differing only in whether ordinal 1 was staged
/// before the frontier advanced (under the old, still-overlapping window)
/// or after (under the new one). Canonical identity must be identical.
#[test]
fn admission_timing_does_not_change_canonical_identity() {
    let epoch = TimelineEpoch(1);

    // Run A: stage 0 and 1 up front, then fence [0,0] and [1,1].
    let mut a = TimelineIngress::new(profile(), epoch, sequencer(), 2).unwrap();
    let a0 = envelope(epoch, 0);
    let a1 = envelope(epoch, 1);
    stage_now(&mut a, &a0);
    stage_now(&mut a, &a1);
    a.submit_fence(
        &sequencer(),
        &fence_over(&a, 0, 0, "fence.1", std::slice::from_ref(&a0)),
    )
    .unwrap();
    a.submit_fence(
        &sequencer(),
        &fence_over(&a, 1, 1, "fence.2", std::slice::from_ref(&a1)),
    )
    .unwrap();

    // Run B: stage 0, fence [0,0], and only then stage ordinal 1 -- now
    // under the new frontier window -- and fence [1,1].
    let mut b = TimelineIngress::new(profile(), epoch, sequencer(), 2).unwrap();
    let b0 = envelope(epoch, 0);
    stage_now(&mut b, &b0);
    b.submit_fence(
        &sequencer(),
        &fence_over(&b, 0, 0, "fence.1", std::slice::from_ref(&b0)),
    )
    .unwrap();
    let b1 = envelope(epoch, 1);
    stage_now(&mut b, &b1);
    b.submit_fence(
        &sequencer(),
        &fence_over(&b, 1, 1, "fence.2", std::slice::from_ref(&b1)),
    )
    .unwrap();

    assert_eq!(a.finalized_commands(), b.finalized_commands());
    assert_eq!(a.canonical_history_digest(), b.canonical_history_digest());
    assert_eq!(a.canonical_state_digest(), b.canonical_state_digest());
}

/// Finality test 2: the admission credential is not part of semantic
/// finalized identity. Two ingresses configured with *different* window
/// widths issue different admission credentials for the identical semantic
/// command; the finalized history must be identical anyway.
#[test]
fn admission_credential_is_not_part_of_finalized_identity() {
    let epoch = TimelineEpoch(1);

    let mut narrow = TimelineIngress::new(profile(), epoch, sequencer(), 2).unwrap();
    let mut wide = TimelineIngress::new(profile(), epoch, sequencer(), 8).unwrap();

    // The two windows genuinely differ, so the credentials genuinely
    // differ.
    assert_ne!(
        narrow.current_admission_window().unwrap().ticket(),
        wide.current_admission_window().unwrap().ticket()
    );

    let command = envelope(epoch, 0);
    stage_now(&mut narrow, &command);
    stage_now(&mut wide, &command);
    narrow
        .submit_fence(
            &sequencer(),
            &fence_over(&narrow, 0, 0, "fence.1", std::slice::from_ref(&command)),
        )
        .unwrap();
    wide.submit_fence(
        &sequencer(),
        &fence_over(&wide, 0, 0, "fence.1", std::slice::from_ref(&command)),
    )
    .unwrap();

    assert_eq!(
        narrow.finalized_commands()[0].envelope,
        wide.finalized_commands()[0].envelope,
        "finalized canonical history must contain semantic command data only"
    );
    assert_eq!(
        narrow.canonical_history_digest(),
        wide.canonical_history_digest()
    );
}

/// Finality test 3: a structured `StageAcknowledgement` binds profile,
/// timeline epoch, ordinal, command ID, canonical semantic-envelope hash,
/// and slot state -- and can prove which envelope it covers.
#[test]
fn stage_acknowledgement_binds_and_proves_its_envelope() {
    let epoch = TimelineEpoch(1);
    let mut ingress = TimelineIngress::new(profile(), epoch, sequencer(), 2).unwrap();
    let command = envelope(epoch, 0);

    let ack = match stage_now(&mut ingress, &command) {
        StageDisposition::Acknowledged(ack) => ack,
        other => panic!("expected a positive acknowledgement, got {other:?}"),
    };

    assert_eq!(ack.profile_id(), &profile());
    assert_eq!(ack.timeline_epoch(), epoch);
    assert_eq!(ack.input_ordinal(), Ordinal(0));
    assert_eq!(ack.command_id(), &command.command_id);
    assert_eq!(ack.semantic_envelope_hash(), &command.semantic_hash());
    assert_eq!(ack.slot_state(), AcknowledgedSlotState::NewlyStaged);
    assert_ne!(ack.acknowledgement_hash(), &Digest::ZERO);

    assert!(ack.covers(&command));

    // It must not vouch for a different command...
    let other = envelope(epoch, 1);
    assert!(!ack.covers(&other));
    // ...nor for a same-ordinal command with different content.
    let mut altered = command.clone();
    altered.canonical_payload_hash = hash_bytes(b"different");
    assert!(!ack.covers(&altered));

    // A repeated identical submission is idempotent and says so.
    let repeat = stage_now(&mut ingress, &command);
    let repeat_ack = repeat.acknowledgement().unwrap();
    assert_eq!(
        repeat_ack.slot_state(),
        AcknowledgedSlotState::AlreadyStagedIdempotent
    );
    assert_eq!(
        repeat_ack.semantic_envelope_hash(),
        &command.semantic_hash()
    );
    assert_ne!(
        repeat_ack.acknowledgement_hash(),
        ack.acknowledgement_hash(),
        "the acknowledged slot state is bound, so the two evidences differ"
    );
}

/// Finality test 4: finalization returns structured evidence identifying
/// the finalized fence, range, and canonical result.
#[test]
fn finalization_returns_structured_evidence() {
    let epoch = TimelineEpoch(1);
    let mut ingress = TimelineIngress::new(profile(), epoch, sequencer(), 4).unwrap();
    let c0 = envelope(epoch, 0);
    let c1 = envelope(epoch, 1);
    stage_now(&mut ingress, &c0);
    stage_now(&mut ingress, &c1);

    let previous = ingress.last_finalized_fence_hash().clone();
    let fence = fence_over(&ingress, 0, 1, "fence.1", &[c0.clone(), c1.clone()]);
    let result = ingress.submit_fence(&sequencer(), &fence).unwrap();

    assert_eq!(result.profile_id(), &profile());
    assert_eq!(result.timeline_epoch(), epoch);
    assert_eq!(result.fence_id(), &fence.fence_id);
    assert_eq!(result.start_ordinal(), Ordinal(0));
    assert_eq!(result.end_ordinal(), Ordinal(1));
    assert_eq!(result.finalized_command_count(), 2);
    assert_eq!(result.previous_fence_hash(), &previous);
    assert_eq!(result.ordered_stream_digest(), &fence.ordered_stream_digest);
    assert_eq!(result.new_frontier_ordinal(), Ordinal(2));
    assert_eq!(
        result.canonical_history_digest(),
        &ingress.canonical_history_digest()
    );
    // The reported fence hash is what the next fence must chain onto.
    assert_eq!(result.fence_hash(), ingress.last_finalized_fence_hash());
}

/// Finality test 5: distinct semantic envelope fields remain part of
/// canonical identity and of the fence history.
#[test]
fn distinct_semantic_fields_remain_canonical_identity() {
    let epoch = TimelineEpoch(1);
    let base = envelope(epoch, 0);
    let baseline = base.semantic_hash();

    let mut other_time = base.clone();
    other_time.effective_time = LogicalTime(99);
    assert_ne!(other_time.semantic_hash(), baseline);

    let mut other_source = base.clone();
    other_source.source_id = SourceId::new("sequencer.other").unwrap();
    assert_ne!(other_source.semantic_hash(), baseline);

    let mut other_sequence = base.clone();
    other_sequence.source_sequence = 42;
    assert_ne!(other_sequence.semantic_hash(), baseline);

    let mut other_kind = base.clone();
    other_kind.command_kind = CommandKind::new(CanonicalTag::new("other.command").unwrap());
    assert_ne!(other_kind.semantic_hash(), baseline);

    let mut other_payload = base.clone();
    other_payload.canonical_payload_hash = hash_bytes(b"different");
    assert_ne!(other_payload.semantic_hash(), baseline);

    // ...and each produces a different finalized history digest.
    for variant in [
        other_time,
        other_source,
        other_sequence,
        other_kind,
        other_payload,
    ] {
        let mut a = TimelineIngress::new(profile(), epoch, sequencer(), 2).unwrap();
        let mut b = TimelineIngress::new(profile(), epoch, sequencer(), 2).unwrap();
        stage_now(&mut a, &base);
        stage_now(&mut b, &variant);
        a.submit_fence(
            &sequencer(),
            &fence_over(&a, 0, 0, "f", std::slice::from_ref(&base)),
        )
        .unwrap();
        b.submit_fence(
            &sequencer(),
            &fence_over(&b, 0, 0, "f", std::slice::from_ref(&variant)),
        )
        .unwrap();
        assert_ne!(a.canonical_history_digest(), b.canonical_history_digest());
    }
}

/// Finality test 6: epoch handoff stays authenticated and monotonic, and
/// preserves finalized history.
#[test]
fn epoch_handoff_is_authenticated_monotonic_and_history_preserving() {
    let epoch = TimelineEpoch(1);
    let mut ingress = TimelineIngress::new(profile(), epoch, sequencer(), 2).unwrap();
    let c0 = envelope(epoch, 0);
    stage_now(&mut ingress, &c0);
    ingress
        .submit_fence(&sequencer(), &fence_over(&ingress, 0, 0, "fence.1", &[c0]))
        .unwrap();
    let history_before = ingress.canonical_history_digest();

    // Unauthorized handoff is rejected.
    let impostor = SourceId::new("sequencer.impostor").unwrap();
    assert!(ingress
        .reset_epoch(&impostor, TimelineEpoch(2), impostor.clone())
        .is_err());
    // Non-monotonic handoff is rejected.
    assert!(ingress
        .reset_epoch(&sequencer(), TimelineEpoch(1), sequencer())
        .is_err());
    assert!(ingress
        .reset_epoch(&sequencer(), TimelineEpoch(0), sequencer())
        .is_err());

    let successor = SourceId::new("sequencer.successor").unwrap();
    let record = ingress
        .reset_epoch(&sequencer(), TimelineEpoch(2), successor.clone())
        .unwrap();
    assert_eq!(record.frozen_finalized_frontier, Ordinal(1));
    assert_eq!(ingress.frontier_ordinal(), Ordinal(1));
    assert_eq!(ingress.finalized_commands().len(), 1);
    assert_eq!(ingress.active_sequencer(), &successor);

    // Finalized history is unchanged except for the appended, authorized
    // reset record itself.
    assert_ne!(ingress.canonical_history_digest(), history_before);
    assert_eq!(ingress.epoch_resets().len(), 1);
}

// =====================================================================
// Schema provenance / authority
// =====================================================================

/// A definition spec offered to the activation door. Note the shape: a
/// caller may author these fields freely, but the *only* thing it can do
/// with them is offer a whole manifest to the one public door.
fn spec(id: &str, authority: Authority) -> DefinitionSpec {
    DefinitionSpec {
        profile_id: profile(),
        id: DefinitionId::new(id).unwrap(),
        kind: DefinitionKind::StateDefinition,
        domain: None,
        layer: None,
        value_constraint: ValueConstraint::boolean(),
        authority,
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
        enabled: true,
        version: 1,
        description: BoundedText::new("state definition").unwrap(),
        behavioral_leverage: None,
    }
}

fn manifest_of(specs: Vec<DefinitionSpec>) -> ProfileManifest {
    ProfileManifest::new(profile(), BoundedText::new("1.0.0").unwrap(), specs)
}

fn store_of(specs: Vec<DefinitionSpec>) -> StateStore {
    let mut registry = ActivationRegistry::new();
    registry
        .activate(&manifest_of(specs))
        .unwrap()
        .into_state_store()
}

/// Schema test 1/2/3: external code cannot install authority into a
/// `StateStore` except by passing through the complete activation
/// ceremony, and the identity it gets is the one the door computed.
///
/// The stronger, compile-time half of this requirement — that
/// `ActivatedProfile`, `ActivatedDefinition`, the raw declaration type,
/// and every `StateStore` constructor are not nameable at all from
/// outside — is encoded as `compile_fail` doc-tests on those items and as
/// external compile probes in `external_compile_probes.rs`.
#[test]
fn state_store_authority_derives_only_from_the_activation_door() {
    let declared = spec("state.forged", Authority::Derived);
    let store = store_of(vec![declared.clone()]);

    let activated = store
        .schema_of(&profile(), &DefinitionId::new("state.forged").unwrap())
        .expect("the one sanctioned path installs the definition");

    // The fingerprint is the door's, never a caller's: a caller has
    // nowhere to put one, and `Digest::ZERO` is not reachable.
    assert_eq!(activated.fingerprint(), &definition_fingerprint(&declared));
    assert_ne!(activated.fingerprint(), &Digest::ZERO);
    assert_ne!(store.activation_hash(), &Digest::ZERO);
    assert_ne!(store.manifest_content_hash(), &Digest::ZERO);
}

/// Schema test 4: duplicate definition keys reject, in either order, and
/// commit nothing.
#[test]
fn duplicate_schema_keys_reject_and_commit_nothing() {
    let host = spec("state.contested", Authority::HostOwned);
    let spark = spec("state.contested", Authority::SparkOwned);

    for pair in [
        vec![host.clone(), spark.clone()],
        vec![spark.clone(), host.clone()],
    ] {
        let mut registry = ActivationRegistry::new();
        let errors = registry.activate(&manifest_of(pair)).unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::DuplicateDefinitionId { .. }
        ));
        assert!(registry.is_empty());
        assert_eq!(registry.lineage_len(), 0);
    }
}

/// Schema test 5: fingerprint, profile, type, scope, and bounds are all
/// carried from the ceremony, and every one of them is part of the
/// identity the registry enforces on reload.
#[test]
fn activated_identity_is_carried_from_the_ceremony_not_from_the_caller() {
    let mut registry = ActivationRegistry::new();
    let original = spec("state.tracked", Authority::HostOwned);
    let activated_profile = registry
        .activate(&manifest_of(vec![original.clone()]))
        .unwrap();
    let activated = activated_profile
        .definition(&DefinitionId::new("state.tracked").unwrap())
        .unwrap();

    assert_eq!(activated.profile_id(), &profile());
    assert_eq!(activated.authority(), Authority::HostOwned);
    assert_eq!(activated.value_constraint(), &ValueConstraint::boolean());
    assert_eq!(
        activated.valid_scopes(),
        &BTreeSet::from([ScopeKind::Actor])
    );
    assert_eq!(activated.kind().tag().as_str(), "state_definition");
    assert!(!activated.kind().is_custom());

    // Any change to any of those is an identity change on reload.
    for mutated in [
        {
            let mut d = original.clone();
            d.authority = Authority::SparkOwned;
            d
        },
        {
            let mut d = original.clone();
            d.value_constraint = ValueConstraint::int(0, 1).unwrap();
            d
        },
        {
            let mut d = original.clone();
            d.valid_scopes = BTreeSet::from([ScopeKind::Household]);
            d
        },
        {
            let mut d = original.clone();
            d.kind = DefinitionKind::custom("state_definition").unwrap();
            d
        },
    ] {
        let errors = registry.activate(&manifest_of(vec![mutated])).unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::IdentityConflict { .. }
        ));
    }
}

/// Schema test 6: post-activation schema mutation is structurally
/// unavailable. A `StateStore` exposes only shared references to its
/// activated definitions, and its artifact hashes are stable across any
/// amount of state reading.
#[test]
fn post_activation_schema_mutation_is_unavailable() {
    let mut registry = ActivationRegistry::new();
    let activated = registry
        .activate(&manifest_of(vec![spec(
            "state.stable",
            Authority::SparkOwned,
        )]))
        .unwrap();
    let expected_activation = activated.activation_hash().clone();
    let expected_manifest = activated.manifest_content_hash().clone();
    let store = activated.into_state_store();

    assert_eq!(store.activation_hash(), &expected_activation);
    assert_eq!(store.manifest_content_hash(), &expected_manifest);
    let definitions: Vec<_> = store.definitions().collect();
    assert_eq!(definitions.len(), 1);
    // Reading the definitions cannot alter them: `definitions()` and
    // `schema_of()` both yield shared references, and there is no
    // `insert`/`remove`/`set_authority` anywhere on the type.
    assert_eq!(store.activation_hash(), &expected_activation);
    assert_eq!(
        store.canonical_state_digest(),
        store.canonical_state_digest()
    );
}

/// A manifest containing an invalid definition is rejected atomically.
/// (The far stronger property — that a *valid* spec still has no
/// self-activation path — is AT-A3 in `refoundation_v2_adversarial.rs`;
/// this test deliberately no longer claims to prove it.)
#[test]
fn invalid_definition_rejects_the_whole_manifest_atomically() {
    let mut invalid = spec("state.unvalidated", Authority::HostOwned);
    invalid.valid_scopes = BTreeSet::new();
    let good = spec("state.fine", Authority::HostOwned);

    let mut registry = ActivationRegistry::new();
    let errors = registry
        .activate(&manifest_of(vec![good.clone(), invalid]))
        .unwrap_err();
    assert!(matches!(errors[0], ValidationError::NoValidScope { .. }));
    assert!(registry.is_empty());
    assert!(registry.fingerprint_of(&profile(), &good.id).is_none());
}

// =====================================================================
// Scheduler
// =====================================================================

fn work_key(due: u64, producer: &str, scope: &str, occurrence: u64, kind: &str) -> WorkKey {
    WorkKey {
        due_time: LogicalTime(due),
        profile_id: profile(),
        producer_definition_id: DefinitionId::new(producer).unwrap(),
        scope_id: ScopeId::new(ScopeKind::Actor, scope).unwrap(),
        occurrence_index: OccurrenceIndex(occurrence),
        work_kind: WorkKind::new(CanonicalTag::new(kind).unwrap()),
    }
}

fn work(due: u64, producer: &str, scope: &str, occurrence: u64) -> DueWorkItem {
    DueWorkItem {
        key: work_key(due, producer, scope, occurrence, "trigger.evaluate"),
        payload: WorkPayload::new(hash_bytes(b"payload")),
    }
}

/// Scheduler test 1: two producers may both schedule occurrence 0 at the
/// same time without collision.
#[test]
fn independent_producers_may_schedule_the_same_occurrence_index() {
    let mut sched = Scheduler::new();
    assert_eq!(
        sched.schedule(work(5, "trigger.a", "bron", 0)),
        ScheduleDisposition::Scheduled
    );
    assert_eq!(
        sched.schedule(work(5, "trigger.b", "bron", 0)),
        ScheduleDisposition::Scheduled
    );
    assert_eq!(sched.len(), 2);
}

/// Scheduler test 2: reversed insertion of those two items drains
/// identically.
#[test]
fn reversed_insertion_drains_identically() {
    let mut forward = Scheduler::new();
    forward.schedule(work(5, "trigger.a", "bron", 0));
    forward.schedule(work(5, "trigger.b", "bron", 0));

    let mut reversed = Scheduler::new();
    reversed.schedule(work(5, "trigger.b", "bron", 0));
    reversed.schedule(work(5, "trigger.a", "bron", 0));

    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest()
    );
    assert_eq!(
        forward.drain_due(LogicalTime(5)),
        reversed.drain_due(LogicalTime(5))
    );
}

/// Scheduler test 3/4 (strengthened): the same semantic key with the same
/// payload is idempotent; with a *different* payload the key is poisoned,
/// and — unlike the first-arrival-wins rule this replaces — neither
/// payload survives.
#[test]
fn same_key_is_idempotent_or_poisons_deterministically() {
    let mut sched = Scheduler::new();
    let original = work(5, "trigger.a", "bron", 0);
    assert_eq!(
        sched.schedule(original.clone()),
        ScheduleDisposition::Scheduled
    );
    assert_eq!(
        sched.schedule(original.clone()),
        ScheduleDisposition::AlreadyScheduledIdempotent
    );
    assert_eq!(sched.len(), 1);
    assert_eq!(sched.slot_status(&original.key), WorkSlotStatus::Scheduled);

    let mut conflicting = original.clone();
    conflicting.payload = WorkPayload::new(hash_bytes(b"other"));
    assert!(matches!(
        sched.schedule(conflicting.clone()),
        ScheduleDisposition::Conflicted(_)
    ));
    assert_eq!(sched.slot_status(&original.key), WorkSlotStatus::Conflicted);

    let outcome = sched.drain_due(LogicalTime(5));
    assert!(
        outcome.due.is_empty(),
        "first arrival must not keep the key: {:?}",
        outcome.due
    );
    assert_eq!(outcome.conflicted.len(), 1);
    let evidence = outcome.conflicted[0].competing_payload_hashes();
    assert!(evidence.contains(&original.payload.canonical_payload_hash));
    assert!(evidence.contains(&conflicting.payload.canonical_payload_hash));
}

/// Scheduler test 5: ordering does not depend on call order.
#[test]
fn drain_order_is_independent_of_call_order() {
    let items = [
        work(5, "trigger.c", "bron", 0),
        work(5, "trigger.a", "mira", 3),
        work(5, "trigger.b", "bron", 1),
        work(3, "trigger.z", "bron", 0),
    ];

    let mut forward = Scheduler::new();
    for item in items.iter() {
        forward.schedule(item.clone());
    }
    let mut reversed = Scheduler::new();
    for item in items.iter().rev() {
        reversed.schedule(item.clone());
    }

    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest()
    );
    assert_eq!(
        forward.drain_due(LogicalTime(5)),
        reversed.drain_due(LogicalTime(5))
    );
}

/// Scheduler test 6: occurrence identity remains producer/scope-local and
/// persistence-friendly.
#[test]
fn occurrence_identity_is_producer_and_scope_local() {
    let mut sched = Scheduler::new();
    // Same producer, different scopes, both occurrence 0.
    sched.schedule(work(5, "trigger.a", "bron", 0));
    sched.schedule(work(5, "trigger.a", "mira", 0));
    // Same producer and scope, its own monotone sequence.
    sched.schedule(work(5, "trigger.a", "bron", 1));
    assert_eq!(sched.len(), 3);

    // The key digest is a stable identity a persisted obligation can bind
    // to, and it separates every one of those domains.
    let a_bron_0 = work_key(5, "trigger.a", "bron", 0, "trigger.evaluate");
    assert_eq!(
        a_bron_0.identity_digest(),
        work_key(5, "trigger.a", "bron", 0, "trigger.evaluate").identity_digest()
    );
    for other in [
        work_key(5, "trigger.a", "mira", 0, "trigger.evaluate"),
        work_key(5, "trigger.a", "bron", 1, "trigger.evaluate"),
        work_key(5, "trigger.b", "bron", 0, "trigger.evaluate"),
        work_key(6, "trigger.a", "bron", 0, "trigger.evaluate"),
        work_key(5, "trigger.a", "bron", 0, "state.decay"),
    ] {
        assert_ne!(a_bron_0.identity_digest(), other.identity_digest());
    }
}

// =====================================================================
// Config revision
// =====================================================================

fn entry(key: &str, value: CanonicalValue) -> ConfigEntry {
    ConfigEntry {
        key: DefinitionId::new(key).unwrap(),
        value,
    }
}

/// Config tests 1 and 4: duplicate keys reject, identically in any order.
#[test]
fn duplicate_config_keys_reject_in_any_order() {
    let forward = ConfigRevision::build(
        profile(),
        BoundedText::new("1.0.0").unwrap(),
        vec![
            entry("trigger.a.base_rate", CanonicalValue::Int(1)),
            entry("trigger.a.base_rate", CanonicalValue::Int(2)),
        ],
    )
    .unwrap_err();
    let reversed = ConfigRevision::build(
        profile(),
        BoundedText::new("1.0.0").unwrap(),
        vec![
            entry("trigger.a.base_rate", CanonicalValue::Int(2)),
            entry("trigger.a.base_rate", CanonicalValue::Int(1)),
        ],
    )
    .unwrap_err();

    assert_eq!(forward, reversed);
    assert!(matches!(forward, ConfigRevisionError::DuplicateKeys { .. }));
}

/// Config test 2: a malformed/oversized value is rejected before the
/// revision is hashable or activatable -- in fact before the value itself
/// exists.
#[test]
fn oversized_config_value_is_rejected_before_hashability() {
    assert!(CanonicalValue::categorical("x".repeat(MAX_CATEGORICAL_VALUE_LEN + 1)).is_err());
    assert!(CategoricalValue::new("x".repeat(MAX_CATEGORICAL_VALUE_LEN + 1)).is_err());
    assert!(BoundedText::new("x".repeat(100_000)).is_err());
}

/// Config tests 3, 5, and 6: unique-entry order is irrelevant, profile
/// context is identity, and the human label is not.
#[test]
fn config_identity_is_content_and_profile_never_order_or_label() {
    let build = |profile: ProfileId, label: &str, entries: Vec<ConfigEntry>| {
        ConfigRevision::build(profile, BoundedText::new(label).unwrap(), entries).unwrap()
    };

    let forward = build(
        profile(),
        "1.0.0",
        vec![
            entry("trigger.a", CanonicalValue::Bool(true)),
            entry("trigger.b", CanonicalValue::Int(3)),
        ],
    );
    let reversed = build(
        profile(),
        "9.9.9-relabelled",
        vec![
            entry("trigger.b", CanonicalValue::Int(3)),
            entry("trigger.a", CanonicalValue::Bool(true)),
        ],
    );
    assert_eq!(
        forward.config_revision_hash(),
        reversed.config_revision_hash()
    );

    let other_profile = build(
        ProfileId::new("mci-social").unwrap(),
        "1.0.0",
        vec![
            entry("trigger.a", CanonicalValue::Bool(true)),
            entry("trigger.b", CanonicalValue::Int(3)),
        ],
    );
    assert_ne!(
        forward.config_revision_hash(),
        other_profile.config_revision_hash()
    );

    let other_content = build(
        profile(),
        "1.0.0",
        vec![
            entry("trigger.a", CanonicalValue::Bool(false)),
            entry("trigger.b", CanonicalValue::Int(3)),
        ],
    );
    assert_ne!(
        forward.config_revision_hash(),
        other_content.config_revision_hash()
    );
}

// =====================================================================
// Canonical construction and bounds
// =====================================================================

/// Bounds test 1: a definition ID requires a namespace per the accepted ID
/// contract (`CONTROLLING_BLUEPRINT_v0.2.md` §12.3).
#[test]
fn definition_id_requires_a_namespace() {
    assert_eq!(
        DefinitionId::new("curiosity"),
        Err(StableIdError::TooFewSegments {
            found: 1,
            required: 2
        })
    );
    assert!(DefinitionId::new("trait.curiosity").is_ok());
    assert!(DefinitionId::new("trigger.weather.drought").is_ok());
}

/// Bounds test 2: incoherent bounds (`min > max`) reject.
#[test]
fn incoherent_bounds_reject() {
    assert_eq!(
        ValueConstraint::int(10, 0),
        Err(ValueConstraintError::IncoherentIntRange { min: 10, max: 0 })
    );
    assert!(ValueConstraint::fixed(
        spark_core::value::FixedPoint::from_integer(1).unwrap(),
        spark_core::value::FixedPoint::ZERO
    )
    .is_err());
    assert!(ValueConstraint::categorical(0).is_err());
    assert!(ValueConstraint::categorical(usize::MAX).is_err());
}

/// Bounds test 3: custom definition-kind strings are bounded and
/// validated.
#[test]
fn custom_definition_kinds_are_bounded_and_validated() {
    assert!(DefinitionKind::custom("k".repeat(100_000)).is_err());
    assert!(DefinitionKind::custom("").is_err());
    assert!(DefinitionKind::custom("Hydrological Pressure").is_err());
    assert!(DefinitionKind::custom("hydrological_pressure").is_ok());
}

/// Bounds test 4: command kind and scheduler work kind use bounded
/// canonical types rather than unbounded raw `String`.
#[test]
fn command_and_work_kinds_are_bounded_canonical_types() {
    assert!(CanonicalTag::new("c".repeat(100_000)).is_err());
    assert!(CanonicalTag::new("").is_err());

    let kind = CommandKind::new(CanonicalTag::new("test.command").unwrap());
    assert_eq!(kind.as_str(), "test.command");
    let work_kind = WorkKind::new(CanonicalTag::new("trigger.evaluate").unwrap());
    assert_eq!(work_kind.as_str(), "trigger.evaluate");
}

/// Bounds test 5: timeline window arithmetic cannot panic or wrap.
#[test]
fn timeline_window_arithmetic_is_total() {
    // A window that cannot fit in the ordinal space is reported, not
    // panicked on.
    let mut exhausted = TimelineIngress::resume_at_frontier(
        profile(),
        TimelineEpoch(1),
        sequencer(),
        4,
        Ordinal(u64::MAX - 1),
    )
    .unwrap();
    let err = exhausted.current_admission_window().unwrap_err();
    assert_eq!(err.frontier_ordinal, Ordinal(u64::MAX - 1));
    assert_eq!(err.window_width, 4);

    // Staging reports the same condition rather than wrapping.
    let command = SemanticCommandEnvelope {
        command_id: CommandId::new("cmd.tail").unwrap(),
        profile_id: profile(),
        timeline_epoch: TimelineEpoch(1),
        effective_time: LogicalTime(0),
        source_id: sequencer(),
        source_sequence: 0,
        input_ordinal: Ordinal(u64::MAX - 1),
        command_kind: CommandKind::new(CanonicalTag::new("test.command").unwrap()),
        canonical_payload_hash: hash_bytes(b"payload"),
    };
    // A ticket cannot even be minted for an exhausted window, so use a
    // fresh ingress's ticket to prove the ordinal check fires first.
    let donor = TimelineIngress::new(profile(), TimelineEpoch(1), sequencer(), 4).unwrap();
    let ticket = donor.current_admission_window().unwrap().ticket();
    assert!(matches!(
        exhausted.stage(&sequencer(), &command.submit_with(ticket)),
        Err(spark_core::timeline::StageError::OrdinalSpaceExhausted(_))
    ));

    // A width-1 window at the very last ordinal is still coherent.
    let last = TimelineIngress::resume_at_frontier(
        profile(),
        TimelineEpoch(1),
        sequencer(),
        1,
        Ordinal(u64::MAX),
    )
    .unwrap();
    let window = last.current_admission_window().unwrap();
    assert_eq!(window.frontier_ordinal, Ordinal(u64::MAX));
    assert_eq!(window.window_end, Ordinal(u64::MAX));
}

/// Bounds test 6: fence finalization is atomic if the frontier advance
/// would overflow -- nothing is promoted and no state changes.
#[test]
fn fence_finalization_is_atomic_when_the_frontier_would_overflow() {
    let epoch = TimelineEpoch(1);
    let mut ingress =
        TimelineIngress::resume_at_frontier(profile(), epoch, sequencer(), 1, Ordinal(u64::MAX))
            .unwrap();

    let command = SemanticCommandEnvelope {
        command_id: CommandId::new("cmd.last").unwrap(),
        profile_id: profile(),
        timeline_epoch: epoch,
        effective_time: LogicalTime(0),
        source_id: sequencer(),
        source_sequence: 0,
        input_ordinal: Ordinal(u64::MAX),
        command_kind: CommandKind::new(CanonicalTag::new("test.command").unwrap()),
        canonical_payload_hash: hash_bytes(b"payload"),
    };
    let ticket = ingress.current_admission_window().unwrap().ticket();
    assert!(matches!(
        ingress
            .stage(&sequencer(), &command.clone().submit_with(ticket))
            .unwrap(),
        StageDisposition::Acknowledged(_)
    ));

    let history_before = ingress.canonical_history_digest();
    let state_before = ingress.canonical_state_digest();

    let fence = fence_over(
        &ingress,
        u64::MAX,
        u64::MAX,
        "fence.last",
        std::slice::from_ref(&command),
    );
    let err = ingress.submit_fence(&sequencer(), &fence).unwrap_err();
    assert!(matches!(err, FenceError::OrdinalSpaceExhausted(_)));

    // Nothing was promoted: no finalized command, no fence, unchanged
    // frontier, unchanged digests, and the staged slot is untouched.
    assert!(ingress.finalized_commands().is_empty());
    assert!(ingress.finalized_fences().is_empty());
    assert_eq!(ingress.frontier_ordinal(), Ordinal(u64::MAX));
    assert_eq!(ingress.canonical_history_digest(), history_before);
    assert_eq!(ingress.canonical_state_digest(), state_before);
    assert!(ingress.is_positively_staged(Ordinal(u64::MAX)));
}

/// Bounds test 7: custom scope kinds remain distinct from each other and
/// from built-ins (independently closed; regression guard).
#[test]
fn custom_scope_kinds_remain_distinct() {
    let a = ScopeKind::Custom(spark_core::id::StableId::new("sensor_grid").unwrap());
    let b = ScopeKind::Custom(spark_core::id::StableId::new("logistics_hub").unwrap());
    let shadowing = ScopeKind::Custom(spark_core::id::StableId::new("actor").unwrap());
    assert_ne!(a, b);
    assert_ne!(a.canonical_tag(), b.canonical_tag());
    assert_ne!(ScopeKind::Actor, shadowing);
    assert_ne!(ScopeKind::Actor.canonical_tag(), shadowing.canonical_tag());
}

/// Bounds test 8: checked fixed-point/integer construction still rejects
/// overflow rather than wrapping (independently closed; regression
/// guard).
#[test]
fn checked_fixed_point_construction_still_rejects_overflow() {
    assert!(spark_core::value::FixedPoint::from_integer(i64::MAX).is_err());
    assert!(spark_core::value::FixedPoint::from_integer(2).is_ok());
    assert_eq!(OccurrenceIndex(u64::MAX).checked_next(), None);
}
