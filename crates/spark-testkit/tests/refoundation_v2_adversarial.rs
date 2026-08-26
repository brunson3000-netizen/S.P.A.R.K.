//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! **Re-Foundation v2 mandatory acceptance corpus (AT-A … AT-F).**
//!
//! These are the assertions required by
//! `PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md` §10a and
//! `PHASE_1_REFOUNDATION_V2_IMPLEMENTATION_BRIEF.md` §9. Every one of them
//! was written and run against the **inherited** tree first; the recorded
//! result is
//! `engineering/phase1/refoundation-v2-baseline/BASELINE_V2_ADVERSARIAL_FAILURES.txt`
//! (eleven of twelve runtime-expressible assertions failed) plus
//! `BASELINE_V2_UNEXPRESSIBLE_FINDINGS.md` for the ones whose baseline was
//! "required API absent".
//!
//! Two disciplines are deliberate here, both of them corrections of how
//! the v1 corpus was written:
//!
//! 1. **Every "X is impossible" claim is proved by a genuine external
//!    compile failure or by a canonical-state-digest equality, never by an
//!    "an error was returned" check alone.** The compile-failure half
//!    lives in `external_compile_probes.rs`, which compiles real external
//!    consumer crates against the *default-feature* production surface.
//! 2. **No test proves its property by feeding an unrelated malformed
//!    input to a path that would have rejected it anyway.** The v1 test
//!    `definition_spec_cannot_self_activate` did exactly that — it used a
//!    spec with an empty scope set, so activation failed for a reason that
//!    had nothing to do with self-activation. AT-A3 below uses a fully
//!    valid spec.

use spark_core::authority::Authority;
use spark_core::clock::{LogicalClock, LogicalTime};
use spark_core::hash::{hash_bytes, CanonicalEncoder, Digest};
use spark_core::id::{
    CanonicalTag, CommandId, DefinitionId, FenceId, ProfileId, SourceId, StaticTagError,
};
use spark_core::random::{RandomAddress, RandomAddressService};
use spark_core::scheduler::{
    DueWorkItem, OccurrenceIndex, ScheduleDisposition, Scheduler, WorkKey, WorkKind, WorkPayload,
    WorkSlotStatus, MAX_CONFLICT_EVIDENCE, MAX_CONFLICT_TRACKED_CLAIMS,
};
use spark_core::scope::{ScopeId, ScopeKind};
use spark_core::timeline::{
    compute_ordered_stream_digest, CommandKind, Ordinal, SemanticCommandEnvelope, SlotStatus,
    StageDisposition, TimelineEpoch, TimelineFence, TimelineIngress,
};
use spark_core::value::{
    CanonicalValue, CategoricalValue, FixedPoint, FixedPointArithmeticError, FixedRange,
    IncoherentClampBounds, ValueConstraint,
};
use spark_engine::activation::{definition_fingerprint, ActivationRegistry, ValidationError};
use spark_engine::fixture;
use spark_engine::profile::config::{ConfigEntry, ConfigRevision, ConfigRevisionError};
use spark_engine::profile::definition::{DefinitionKind, DefinitionSpec};
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::profile::text::BoundedText;
use spark_engine::state::{StateStore, StateWriteError};
use std::collections::BTreeSet;

fn profile() -> ProfileId {
    ProfileId::new("game-world").unwrap()
}

fn sequencer() -> SourceId {
    SourceId::new("sequencer.primary").unwrap()
}

/// A **fully valid** definition spec: non-empty scopes, coherent
/// constraint, namespaced ID, bounded text. Nothing about it is
/// rejectable, which is the point — an activation refusal for this spec
/// could only mean the ceremony was bypassed, never that the input was
/// malformed.
fn valid_spec(id: &str, authority: Authority) -> DefinitionSpec {
    DefinitionSpec {
        profile_id: profile(),
        id: DefinitionId::new(id).unwrap(),
        kind: DefinitionKind::Trait,
        domain: None,
        layer: None,
        value_constraint: ValueConstraint::fixed(
            FixedPoint::ZERO,
            FixedPoint::from_integer(1).unwrap(),
        )
        .unwrap(),
        authority,
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
        enabled: true,
        version: 1,
        description: BoundedText::new("a completely valid definition").unwrap(),
        behavioral_leverage: None,
    }
}

fn manifest_of(profile_id: ProfileId, label: &str, specs: Vec<DefinitionSpec>) -> ProfileManifest {
    ProfileManifest::new(profile_id, BoundedText::new(label).unwrap(), specs)
}

// =====================================================================
// AT-A — trusted activation (B-02)
// =====================================================================

/// **AT-A3** — replaces the insufficient v1 `definition_spec_cannot_self_activate`.
///
/// A fully valid `DefinitionSpec` has exactly one route to an activated
/// artifact: a `ProfileManifest` through `ActivationRegistry::activate`.
/// This test proves the *positive* half — that the door path succeeds and
/// yields precisely the fingerprint the pure predictor computes — while
/// the negative half (no intermediate mint, no raw declaration type, no
/// `to_declaration`) is proved by external compile failure in
/// `external_compile_probes.rs`, because a removed API cannot be
/// falsified at run time.
#[test]
fn valid_spec_activates_only_through_the_door() {
    let s = valid_spec("trait.curiosity", Authority::Derived);
    let predicted = definition_fingerprint(&s);

    let mut registry = ActivationRegistry::new();
    let activated = registry
        .activate(&manifest_of(profile(), "1.0.0", vec![s.clone()]))
        .expect("a fully valid manifest activates");

    let definition = activated.definition(&s.id).unwrap();
    assert_eq!(definition.fingerprint(), &predicted);
    assert_eq!(definition.authority(), Authority::Derived);
    assert_ne!(definition.fingerprint(), &Digest::ZERO);

    // The spec's own predictor and the door agree, so a reviewer can
    // recompute an identity without being able to assert one.
    assert_eq!(s.definition_fingerprint(), predicted);
}

/// **AT-A4** — the store carries the full exact-artifact binding.
#[test]
fn store_carries_full_artifact_binding() {
    let manifest = manifest_of(
        profile(),
        "1.0.0",
        vec![valid_spec("trait.curiosity", Authority::SparkOwned)],
    );
    let mut registry = ActivationRegistry::new();
    let activated = registry.activate(&manifest).unwrap();
    let expected_activation = activated.activation_hash().clone();

    let store = activated.into_state_store();
    assert_eq!(store.activation_hash(), &expected_activation);
    assert_eq!(
        store.manifest_content_hash(),
        &manifest.manifest_content_hash()
    );
    assert_ne!(store.activation_hash(), &Digest::ZERO);
    assert_ne!(store.manifest_content_hash(), &Digest::ZERO);
    assert_ne!(store.canonical_state_digest(), Digest::ZERO);
}

/// **AT-A5** (Codex counterexample #3) — divergent activations are never
/// interchangeable, and identical ones are byte-identical.
///
/// Rust cannot stop a second `ActivationRegistry::new()`, so the invariant
/// is content addressing: two registries fed the *same* manifest agree on
/// every downstream hash (hence are harmlessly interchangeable), and two
/// registries fed *divergent* authority facts disagree on every downstream
/// hash including the store's canonical state digest — so divergence is
/// never silent.
#[test]
fn divergent_activations_are_never_interchangeable() {
    let host = manifest_of(
        profile(),
        "1.0.0",
        vec![valid_spec("state.contested", Authority::HostOwned)],
    );
    let spark = manifest_of(
        profile(),
        "1.0.0",
        vec![valid_spec("state.contested", Authority::SparkOwned)],
    );

    let mut registry_a = ActivationRegistry::new();
    let mut registry_b = ActivationRegistry::new();
    let mut registry_c = ActivationRegistry::new();

    // Both succeed locally: neither registry knows the other exists.
    let a = registry_a.activate(&host).unwrap();
    let b = registry_b.activate(&spark).unwrap();
    let c = registry_c.activate(&host).unwrap();

    // Identical manifests in independent registries are byte-identical.
    assert_eq!(a.activation_hash(), c.activation_hash());
    assert_eq!(a.manifest_content_hash(), c.manifest_content_hash());
    assert_eq!(registry_a.lineage_digest(), registry_c.lineage_digest());

    // Divergent authority facts differ on every downstream artifact.
    assert_ne!(a.activation_hash(), b.activation_hash());
    assert_ne!(a.manifest_content_hash(), b.manifest_content_hash());
    assert_ne!(registry_a.lineage_digest(), registry_b.lineage_digest());

    let store_a = a.into_state_store();
    let store_b = b.into_state_store();
    let store_c = c.into_state_store();
    assert_ne!(
        store_a.canonical_state_digest(),
        store_b.canonical_state_digest(),
        "a divergent activation must be visible on the state store's own digest"
    );
    assert_eq!(
        store_a.canonical_state_digest(),
        store_c.canonical_state_digest()
    );
}

/// **AT-A6** — the schema validation behind the crate-internal write paths
/// still rejects every violation class.
///
/// The *absence* of any public write path is proved by external compile
/// failure (`external_compile_probes.rs`); this test uses the sanctioned
/// `test-support` fixture seam to prove the rules those paths enforce are
/// all still live, so restricting the surface did not quietly disable the
/// checks.
#[test]
fn write_paths_still_enforce_every_validation_class() {
    let mut registry = ActivationRegistry::new();
    let mut host = valid_spec("state.host.value", Authority::HostOwned);
    host.value_constraint = ValueConstraint::int(0, 10).unwrap();
    let mut spark = valid_spec("state.spark.value", Authority::SparkOwned);
    spark.value_constraint = ValueConstraint::int(0, 10).unwrap();
    let mut derived = valid_spec("state.derived.value", Authority::Derived);
    derived.value_constraint = ValueConstraint::int(0, 10).unwrap();

    let mut store: StateStore = registry
        .activate(&manifest_of(profile(), "1.0.0", vec![host, spark, derived]))
        .unwrap()
        .into_state_store();

    let actor = ScopeId::new(ScopeKind::Actor, "bron").unwrap();
    let def = |id: &str| DefinitionId::new(id).unwrap();

    // Wrong authority for the write path.
    assert!(matches!(
        fixture::apply_spark_effect(
            &mut store,
            profile(),
            def("state.host.value"),
            actor.clone(),
            CanonicalValue::Int(1),
            LogicalTime(0),
            1,
        )
        .unwrap_err(),
        StateWriteError::WrongAuthorityForWritePath { .. }
    ));
    // Undeclared definition.
    assert!(matches!(
        fixture::apply_spark_effect(
            &mut store,
            profile(),
            def("state.never.declared"),
            actor.clone(),
            CanonicalValue::Int(1),
            LogicalTime(0),
            1,
        )
        .unwrap_err(),
        StateWriteError::UndeclaredDefinition { .. }
    ));
    // Wrong value type.
    assert!(matches!(
        fixture::apply_spark_effect(
            &mut store,
            profile(),
            def("state.spark.value"),
            actor.clone(),
            CanonicalValue::Bool(true),
            LogicalTime(0),
            1,
        )
        .unwrap_err(),
        StateWriteError::WrongValueType { .. }
    ));
    // Out of declared bounds.
    assert!(matches!(
        fixture::apply_spark_effect(
            &mut store,
            profile(),
            def("state.spark.value"),
            actor.clone(),
            CanonicalValue::Int(11),
            LogicalTime(0),
            1,
        )
        .unwrap_err(),
        StateWriteError::OutOfBounds { .. }
    ));
    // Disallowed scope kind.
    assert!(matches!(
        fixture::apply_spark_effect(
            &mut store,
            profile(),
            def("state.spark.value"),
            ScopeId::new(ScopeKind::Household, "bron").unwrap(),
            CanonicalValue::Int(1),
            LogicalTime(0),
            1,
        )
        .unwrap_err(),
        StateWriteError::DisallowedScope { .. }
    ));
    // Foreign profile.
    assert!(matches!(
        fixture::apply_spark_effect(
            &mut store,
            ProfileId::new("mci-social").unwrap(),
            def("state.spark.value"),
            actor.clone(),
            CanonicalValue::Int(1),
            LogicalTime(0),
            1,
        )
        .unwrap_err(),
        StateWriteError::ForeignProfile { .. }
    ));

    // Each authority's own path works, and only its own.
    assert!(fixture::observe_host_owned(
        &mut store,
        profile(),
        def("state.host.value"),
        actor.clone(),
        CanonicalValue::Int(3),
        LogicalTime(0),
        1,
    )
    .is_ok());
    assert!(fixture::apply_spark_effect(
        &mut store,
        profile(),
        def("state.spark.value"),
        actor.clone(),
        CanonicalValue::Int(4),
        LogicalTime(0),
        1,
    )
    .is_ok());
    assert!(fixture::commit_derived(
        &mut store,
        profile(),
        def("state.derived.value"),
        actor.clone(),
        CanonicalValue::Int(5),
        LogicalTime(0),
        1,
    )
    .is_ok());
    assert!(fixture::commit_derived(
        &mut store,
        profile(),
        def("state.host.value"),
        actor,
        CanonicalValue::Int(5),
        LogicalTime(0),
        1,
    )
    .is_err());
}

/// **AT-A7** (Codex counterexample #4) — a custom kind spelled exactly
/// like a baseline kind is a different identity, and the built-in bit is
/// not caller-assertable (the latter proved by compile failure).
#[test]
fn builtin_and_custom_kinds_stay_domain_separated() {
    let mut builtin = valid_spec("trigger.drought", Authority::SparkOwned);
    builtin.kind = DefinitionKind::Trigger;
    let mut custom = valid_spec("trigger.drought", Authority::SparkOwned);
    custom.kind = DefinitionKind::custom("trigger").unwrap();

    assert_ne!(
        definition_fingerprint(&builtin),
        definition_fingerprint(&custom)
    );

    // And the registry treats them as an identity conflict, not as the
    // same definition.
    let mut registry = ActivationRegistry::new();
    registry
        .activate(&manifest_of(profile(), "1.0.0", vec![builtin]))
        .unwrap();
    let errors = registry
        .activate(&manifest_of(profile(), "1.0.0", vec![custom]))
        .unwrap_err();
    assert!(matches!(
        errors[0],
        ValidationError::IdentityConflict { .. }
    ));

    // The activated artifact reports the bit; it does not accept it.
    let activated = ActivationRegistry::new()
        .activate(&manifest_of(
            profile(),
            "1.0.0",
            vec![{
                let mut s = valid_spec("trigger.custom", Authority::SparkOwned);
                s.kind = DefinitionKind::custom("weather_system").unwrap();
                s
            }],
        ))
        .unwrap();
    let definition = activated
        .definition(&DefinitionId::new("trigger.custom").unwrap())
        .unwrap();
    assert!(definition.kind().is_custom());
}

/// **AT-A8** (Codex counterexample #16) — the registry lineage is
/// append-only and deterministic, and repeated identical activations do
/// not grow divergence.
#[test]
fn registry_lineage_is_append_only_and_deterministic() {
    let m1 = manifest_of(
        profile(),
        "1.0.0",
        vec![valid_spec("trait.curiosity", Authority::SparkOwned)],
    );
    let m2 = manifest_of(
        profile(),
        "1.0.0",
        vec![valid_spec("trait.skepticism", Authority::SparkOwned)],
    );

    let run = |order: [&ProfileManifest; 2]| {
        let mut registry = ActivationRegistry::new();
        for manifest in order {
            registry.activate(manifest).unwrap();
        }
        registry
    };

    let forward = run([&m1, &m2]);
    let replay = run([&m1, &m2]);
    let reversed = run([&m2, &m1]);

    assert_eq!(forward.lineage_digest(), replay.lineage_digest());
    assert_ne!(forward.lineage_digest(), reversed.lineage_digest());
    assert_eq!(forward.lineage_len(), 2);

    // Repeating an identical activation is idempotent in the lineage.
    let mut repeated = run([&m1, &m2]);
    let before = repeated.lineage_digest();
    repeated.activate(&m1).unwrap();
    repeated.activate(&m2).unwrap();
    assert_eq!(before, repeated.lineage_digest());
    assert_eq!(repeated.lineage_len(), 2);
}

// =====================================================================
// AT-B — scheduler conflict (B-03)
// =====================================================================

fn work_key(occurrence: u64) -> WorkKey {
    WorkKey {
        due_time: LogicalTime(5),
        profile_id: profile(),
        producer_definition_id: DefinitionId::new("trigger.weather.drought").unwrap(),
        scope_id: ScopeId::new(ScopeKind::Settlement, "settlement.pontafique").unwrap(),
        occurrence_index: OccurrenceIndex(occurrence),
        work_kind: WorkKind::new(CanonicalTag::new("trigger.evaluate").unwrap()),
    }
}

fn claim(payload: &str) -> DueWorkItem {
    DueWorkItem {
        key: work_key(0),
        payload: WorkPayload::new(hash_bytes(payload.as_bytes())),
    }
}

/// **AT-B1** — replaces the insufficient v1
/// `payload_conflict_rejects_in_either_order`, which compared only error
/// shape and length and therefore could not see the winner-selection
/// defect at all. The defining assertions here are canonical-state-digest
/// equality and drained-work equality.
#[test]
fn same_key_conflict_state_is_arrival_order_independent() {
    let a = claim("payload.a");
    let b = claim("payload.b");
    let key = a.key.clone();

    let mut forward = Scheduler::new();
    forward.schedule(a.clone());
    forward.schedule(b.clone());

    let mut reversed = Scheduler::new();
    reversed.schedule(b.clone());
    reversed.schedule(a.clone());

    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest()
    );
    assert_eq!(forward.slot_status(&key), WorkSlotStatus::Conflicted);
    assert_eq!(reversed.slot_status(&key), WorkSlotStatus::Conflicted);
    assert_eq!(
        forward.conflict_of(&key).unwrap(),
        reversed.conflict_of(&key).unwrap()
    );
    assert_eq!(
        forward.drain_due(LogicalTime(5)),
        reversed.drain_due(LogicalTime(5))
    );
}

/// **AT-B3** — neither payload survives a conflict.
#[test]
fn neither_payload_survives_a_conflict() {
    let a = claim("payload.a");
    let b = claim("payload.b");
    let mut sched = Scheduler::new();
    sched.schedule(a.clone());
    sched.schedule(b.clone());

    let outcome = sched.drain_due(LogicalTime(5));
    assert!(outcome.due.is_empty());
    assert_eq!(outcome.conflicted.len(), 1);
    let report = &outcome.conflicted[0];
    assert_eq!(report.key(), &a.key);
    assert!(report
        .competing_payload_hashes()
        .contains(&a.payload.canonical_payload_hash));
    assert!(report
        .competing_payload_hashes()
        .contains(&b.payload.canonical_payload_hash));
}

/// **AT-B2** — a three-way conflict converges in all six permutations,
/// on both canonical state and drained output.
#[test]
fn three_way_conflict_converges_in_all_six_orders() {
    let claims = [claim("payload.a"), claim("payload.b"), claim("payload.c")];
    let permutations = [
        [0usize, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    let mut states: BTreeSet<Digest> = BTreeSet::new();
    let mut drains: BTreeSet<Digest> = BTreeSet::new();
    for permutation in permutations {
        let mut sched = Scheduler::new();
        for index in permutation {
            sched.schedule(claims[index].clone());
        }
        states.insert(sched.canonical_state_digest());

        let outcome = sched.drain_due(LogicalTime(5));
        assert!(outcome.due.is_empty());
        assert_eq!(outcome.conflicted.len(), 1);
        assert_eq!(outcome.conflicted[0].competing_payload_hashes().len(), 3);
        let mut enc = CanonicalEncoder::new();
        for hash in outcome.conflicted[0].competing_payload_hashes() {
            enc.push_digest(hash);
        }
        drains.insert(enc.finish());
    }
    assert_eq!(states.len(), 1);
    assert_eq!(drains.len(), 1);
}

/// **AT-B4** — an exact duplicate into a conflicted slot is idempotent.
#[test]
fn exact_duplicate_into_conflicted_slot_is_idempotent() {
    let a = claim("payload.a");
    let mut sched = Scheduler::new();
    sched.schedule(a.clone());
    sched.schedule(claim("payload.b"));
    let before = sched.canonical_state_digest();

    assert!(matches!(
        sched.schedule(a),
        ScheduleDisposition::Conflicted(_)
    ));
    assert_eq!(before, sched.canonical_state_digest());
}

/// **AT-B5** — conflict report, key release, and reschedule are
/// deterministic under full replay.
#[test]
fn conflict_report_and_requeue_are_deterministic() {
    fn run() -> (Digest, Digest, usize) {
        let mut sched = Scheduler::new();
        sched.schedule(claim("payload.a"));
        sched.schedule(claim("payload.b"));
        let outcome = sched.drain_due(LogicalTime(5));
        let after_drain = sched.canonical_state_digest();
        assert_eq!(
            sched.schedule(claim("payload.resolved")),
            ScheduleDisposition::Scheduled
        );
        (
            after_drain,
            sched.canonical_state_digest(),
            outcome.conflicted.len(),
        )
    }
    assert_eq!(run(), run());
    assert_eq!(run().2, 1);
}

/// **AT-B6** — the evidence cap is order independent: the retained set is
/// the smallest hashes of the whole claim set and the omitted count is
/// exact, for any arrival order.
#[test]
fn evidence_cap_is_order_independent() {
    let claims: Vec<DueWorkItem> = (0..(MAX_CONFLICT_EVIDENCE * 4))
        .map(|i| claim(&format!("payload.{i}")))
        .collect();

    let orders: Vec<Vec<&DueWorkItem>> = vec![
        claims.iter().collect(),
        claims.iter().rev().collect(),
        claims
            .iter()
            .step_by(3)
            .chain(claims.iter().skip(1).step_by(3))
            .chain(claims.iter().skip(2).step_by(3))
            .collect(),
    ];

    let mut digests: BTreeSet<Digest> = BTreeSet::new();
    for order in &orders {
        let mut sched = Scheduler::new();
        for c in order {
            sched.schedule((*c).clone());
        }
        digests.insert(sched.canonical_state_digest());

        let conflict = sched.conflict_of(&claims[0].key).unwrap();
        assert_eq!(
            conflict.competing_payload_hashes().len(),
            MAX_CONFLICT_EVIDENCE
        );
        assert_eq!(
            conflict.omitted_distinct() as usize,
            claims.len() - MAX_CONFLICT_EVIDENCE
        );
        assert!(!conflict.evidence_truncated());
        assert!(claims.len() <= MAX_CONFLICT_TRACKED_CLAIMS);
    }
    assert_eq!(digests.len(), 1);
}

/// Different conflict evidence must not collapse to one canonical state.
#[test]
fn different_conflict_evidence_is_digest_distinct() {
    let mut one = Scheduler::new();
    one.schedule(claim("payload.a"));
    one.schedule(claim("payload.b"));

    let mut two = Scheduler::new();
    two.schedule(claim("payload.c"));
    two.schedule(claim("payload.d"));

    assert_ne!(one.canonical_state_digest(), two.canonical_state_digest());
}

// =====================================================================
// AT-C — panic-free canonical construction (M-03)
// =====================================================================

/// **AT-C1** — the executed runtime panic is gone: an invalid
/// `&'static str` returns a typed error.
#[test]
fn try_from_static_is_total_at_runtime() {
    assert_eq!(
        CanonicalTag::try_from_static("INVALID TAG"),
        Err(StaticTagError::InvalidByte { byte: b'I' })
    );
    assert!(CanonicalTag::try_from_static("trigger").is_ok());
    // A `&'static str` manufactured at run time (the case that made the
    // old `const fn` panic) is reported, not panicked on.
    let leaked: &'static str = Box::leak("Not A Tag".to_string().into_boxed_str());
    assert!(CanonicalTag::try_from_static(leaked).is_err());
}

/// **AT-C2** — the literal macro produces the same value as the runtime
/// constructor. The invalid-literal half is a `compile_fail` doc-test on
/// `canonical_tag!` plus an external compile probe.
#[test]
fn canonical_tag_macro_agrees_with_the_runtime_constructor() {
    const TRIGGER: CanonicalTag = spark_core::canonical_tag!("trigger");
    assert_eq!(TRIGGER, CanonicalTag::new("trigger").unwrap());
    assert_eq!(TRIGGER, CanonicalTag::try_from_static("trigger").unwrap());
}

/// **AT-C3** — the executed reversed-clamp panic is gone.
#[test]
fn clamp_is_total_or_fallible_never_panicking() {
    let zero = FixedPoint::ZERO;
    let one = FixedPoint::from_integer(1).unwrap();
    let half = FixedPoint::from_raw(500_000);

    assert_eq!(
        half.checked_clamp(one, zero),
        Err(IncoherentClampBounds {
            min: 1_000_000,
            max: 0
        })
    );
    assert!(FixedRange::new(one, zero).is_err());

    let range = FixedRange::new(zero, one).unwrap();
    assert_eq!(FixedPoint::from_raw(-1).clamp_to(&range), zero);
    assert_eq!(FixedPoint::from_raw(i64::MAX).clamp_to(&range), one);
    assert_eq!(half.clamp_to(&range), half);
}

/// **AT-C4** (Codex counterexample #17) — at frontier `u64::MAX` with
/// width 1, repeated public operations in varied orders report the
/// exhaustion/rejection condition with **zero** state mutation and zero
/// panics.
#[test]
fn frontier_at_max_is_idempotently_reportable() {
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

    let history = ingress.canonical_history_digest();
    let state = ingress.canonical_state_digest();

    let fence = TimelineFence {
        profile_id: profile(),
        timeline_epoch: epoch,
        fence_id: FenceId::new("fence.last").unwrap(),
        start_ordinal: Ordinal(u64::MAX),
        end_ordinal: Ordinal(u64::MAX),
        previous_fence_hash: ingress.last_finalized_fence_hash().clone(),
        ordered_stream_digest: compute_ordered_stream_digest(&[(
            Ordinal(u64::MAX),
            command.semantic_hash(),
        )]),
    };

    // Repeat every public operation several times, in varied orders.
    for round in 0..3 {
        if round % 2 == 0 {
            assert!(ingress.submit_fence(&sequencer(), &fence).is_err());
            assert!(ingress.current_admission_window().is_ok());
        } else {
            assert!(ingress.current_admission_window().is_ok());
            assert!(ingress.submit_fence(&sequencer(), &fence).is_err());
        }
        assert_eq!(ingress.slot_status(Ordinal(u64::MAX)), SlotStatus::Staged);
        assert!(ingress.is_positively_staged(Ordinal(u64::MAX)));
        assert_eq!(ingress.frontier_ordinal(), Ordinal(u64::MAX));
        assert!(ingress.finalized_commands().is_empty());
        assert!(ingress.finalized_fences().is_empty());
        assert_eq!(ingress.canonical_history_digest(), history);
        assert_eq!(ingress.canonical_state_digest(), state);
    }

    // A width that cannot fit the remaining ordinal space reports rather
    // than wrapping, also idempotently.
    let exhausted = TimelineIngress::resume_at_frontier(
        profile(),
        epoch,
        sequencer(),
        4,
        Ordinal(u64::MAX - 1),
    )
    .unwrap();
    let before = exhausted.canonical_state_digest();
    for _ in 0..3 {
        assert!(exhausted.current_admission_window().is_err());
        assert_eq!(exhausted.canonical_state_digest(), before);
    }
}

/// **AT-C5** — checked fixed-point arithmetic reports overflow.
#[test]
fn checked_fixed_point_arithmetic() {
    assert_eq!(
        FixedPoint::from_raw(i64::MAX).checked_add(FixedPoint::from_raw(1)),
        Err(FixedPointArithmeticError::AdditionOverflow {
            lhs: i64::MAX,
            rhs: 1
        })
    );
    assert_eq!(
        FixedPoint::from_raw(i64::MIN).checked_sub(FixedPoint::from_raw(1)),
        Err(FixedPointArithmeticError::SubtractionOverflow {
            lhs: i64::MIN,
            rhs: 1
        })
    );
    assert!(FixedPoint::from_integer(i64::MAX).is_err());
    assert_eq!(OccurrenceIndex(u64::MAX).checked_next(), None);
}

/// **AT-C6** (Codex counterexample #14) — the canonical text contract is
/// deliberate byte distinction; no Unicode normalization enters canonical
/// hashing.
#[test]
fn unicode_is_byte_distinct() {
    let composed = CategoricalValue::new("caf\u{e9}").unwrap();
    let decomposed = CategoricalValue::new("cafe\u{301}").unwrap();
    assert_ne!(composed, decomposed);

    let hash_of = |value: CategoricalValue| {
        let mut enc = CanonicalEncoder::new();
        CanonicalValue::Categorical(value).canonicalize(&mut enc);
        enc.finish()
    };
    assert_ne!(hash_of(composed), hash_of(decomposed));

    // Profile text follows the same contract, and it reaches the manifest
    // content hash.
    let with_composed = manifest_of(profile(), "caf\u{e9}", vec![]);
    let with_decomposed = manifest_of(profile(), "cafe\u{301}", vec![]);
    // The label is deliberately not identity, so these agree...
    assert_eq!(
        with_composed.manifest_content_hash(),
        with_decomposed.manifest_content_hash()
    );
    // ...but a description, which *is* content, does not.
    let described = |text: &str| {
        let mut s = valid_spec("trait.curiosity", Authority::SparkOwned);
        s.description = BoundedText::new(text).unwrap();
        manifest_of(profile(), "1.0.0", vec![s]).manifest_content_hash()
    };
    assert_ne!(described("caf\u{e9}"), described("cafe\u{301}"));
}

// =====================================================================
// AT-D — reconstruction restriction
// =====================================================================

/// **AT-D3** (Codex counterexample #10) — an arbitrary test frontier
/// cannot masquerade as a validated continuation.
///
/// A nonzero-frontier ingress carries a *synthesized* genesis anchor that
/// matches no real fence chain. Its canonical history digest therefore
/// cannot equal any digest produced by an actual finalization sequence
/// over the same envelopes from frontier 0 — which is precisely why the
/// constructor is not a reconstruction API and is not on the production
/// surface at all (`external_compile_probes.rs` proves the latter).
#[test]
fn arbitrary_frontier_cannot_masquerade_as_a_validated_continuation() {
    let epoch = TimelineEpoch(1);

    // A genuine history: stage and finalize ordinal 0 from frontier 0.
    let mut genuine = TimelineIngress::new(profile(), epoch, sequencer(), 2).unwrap();
    let command = SemanticCommandEnvelope {
        command_id: CommandId::new("cmd.0").unwrap(),
        profile_id: profile(),
        timeline_epoch: epoch,
        effective_time: LogicalTime(0),
        source_id: sequencer(),
        source_sequence: 0,
        input_ordinal: Ordinal(0),
        command_kind: CommandKind::new(CanonicalTag::new("test.command").unwrap()),
        canonical_payload_hash: hash_bytes(b"payload.0"),
    };
    let ticket = genuine.current_admission_window().unwrap().ticket();
    genuine
        .stage(&sequencer(), &command.clone().submit_with(ticket))
        .unwrap();
    let fence = TimelineFence {
        profile_id: profile(),
        timeline_epoch: epoch,
        fence_id: FenceId::new("fence.1").unwrap(),
        start_ordinal: Ordinal(0),
        end_ordinal: Ordinal(0),
        previous_fence_hash: genuine.last_finalized_fence_hash().clone(),
        ordered_stream_digest: compute_ordered_stream_digest(&[(
            Ordinal(0),
            command.semantic_hash(),
        )]),
    };
    let finalized = genuine.submit_fence(&sequencer(), &fence).unwrap();
    let genuine_history = genuine.canonical_history_digest();
    assert_eq!(genuine.frontier_ordinal(), Ordinal(1));

    // Two synthesized continuations claiming to already be past that
    // point. Neither can produce the genuine history digest, and neither
    // carries the real fence-chain anchor.
    for frontier in [1u64, 1_000_000] {
        let injected = TimelineIngress::resume_at_frontier(
            profile(),
            epoch,
            sequencer(),
            2,
            Ordinal(frontier),
        )
        .unwrap();
        assert!(injected.finalized_commands().is_empty());
        assert!(injected.finalized_fences().is_empty());
        assert_ne!(
            injected.canonical_history_digest(),
            genuine_history,
            "a synthesized frontier must never reproduce a real finalized history digest"
        );
        assert_ne!(
            injected.last_finalized_fence_hash(),
            finalized.fence_hash(),
            "a synthesized genesis anchor must never equal a real fence hash"
        );
        assert_ne!(
            injected.last_finalized_fence_hash(),
            genuine.last_finalized_fence_hash()
        );
    }
}

// =====================================================================
// AT-E / AT-F — closed-item regression and evidence hardening
// =====================================================================

/// **AT-E2a** (Codex counterexample #15) — three or more duplicate config
/// keys with reordered values produce identical sorted diagnostics and no
/// partial revision.
#[test]
fn duplicate_config_diagnostics_are_deterministic_with_three_or_more() {
    let entry = |key: &str, value: CanonicalValue| ConfigEntry {
        key: DefinitionId::new(key).unwrap(),
        value,
    };
    let build = |entries: Vec<ConfigEntry>| {
        ConfigRevision::build(profile(), BoundedText::new("1.0.0").unwrap(), entries)
    };

    let forward = build(vec![
        entry("trigger.a.base_rate", CanonicalValue::Int(1)),
        entry("trigger.b.base_rate", CanonicalValue::Int(1)),
        entry("trigger.c.base_rate", CanonicalValue::Int(1)),
        entry("trigger.a.base_rate", CanonicalValue::Int(2)),
        entry("trigger.b.base_rate", CanonicalValue::Int(3)),
        entry("trigger.c.base_rate", CanonicalValue::Int(4)),
    ])
    .unwrap_err();
    let reversed = build(vec![
        entry("trigger.c.base_rate", CanonicalValue::Int(4)),
        entry("trigger.b.base_rate", CanonicalValue::Int(3)),
        entry("trigger.a.base_rate", CanonicalValue::Int(2)),
        entry("trigger.c.base_rate", CanonicalValue::Int(1)),
        entry("trigger.b.base_rate", CanonicalValue::Int(1)),
        entry("trigger.a.base_rate", CanonicalValue::Int(1)),
    ])
    .unwrap_err();
    let shuffled = build(vec![
        entry("trigger.b.base_rate", CanonicalValue::Int(1)),
        entry("trigger.a.base_rate", CanonicalValue::Int(2)),
        entry("trigger.c.base_rate", CanonicalValue::Int(4)),
        entry("trigger.b.base_rate", CanonicalValue::Int(3)),
        entry("trigger.c.base_rate", CanonicalValue::Int(1)),
        entry("trigger.a.base_rate", CanonicalValue::Int(1)),
    ])
    .unwrap_err();

    assert_eq!(forward, reversed);
    assert_eq!(forward, shuffled);
    let ConfigRevisionError::DuplicateKeys { keys } = &forward;
    assert_eq!(keys.len(), 3, "all three duplicate keys must be reported");
    let rendered: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
    assert_eq!(
        rendered,
        vec![
            "trigger.a.base_rate",
            "trigger.b.base_rate",
            "trigger.c.base_rate"
        ],
        "diagnostics must be sorted, not arrival-ordered"
    );
    // No partial revision escapes: `build` is the only constructor and it
    // returned `Err`, so nothing hashable was produced.
    assert!(build(vec![entry("trigger.a.base_rate", CanonicalValue::Int(1))]).is_ok());
}

/// **AT-F1** (Codex counterexample #12) — poisoned slots carry
/// order-independent bounded evidence: the same competing pair in either
/// order agrees, and *different* competing pairs disagree.
#[test]
fn poisoned_slot_evidence_is_order_independent_and_discriminating() {
    fn poisoned_state(first: &str, second: &str) -> Digest {
        let epoch = TimelineEpoch(1);
        let mut ingress = TimelineIngress::new(profile(), epoch, sequencer(), 2).unwrap();
        for command in [first, second] {
            let ticket = ingress.current_admission_window().unwrap().ticket();
            let envelope = SemanticCommandEnvelope {
                command_id: CommandId::new(command).unwrap(),
                profile_id: profile(),
                timeline_epoch: epoch,
                effective_time: LogicalTime(0),
                source_id: sequencer(),
                source_sequence: 0,
                input_ordinal: Ordinal(0),
                command_kind: CommandKind::new(CanonicalTag::new("test.command").unwrap()),
                canonical_payload_hash: hash_bytes(command.as_bytes()),
            };
            let _ = ingress.stage(&sequencer(), &envelope.submit_with(ticket));
        }
        assert_eq!(ingress.slot_status(Ordinal(0)), SlotStatus::Poisoned);
        ingress.canonical_state_digest()
    }

    // Same pair, opposite orders: identical canonical state.
    assert_eq!(
        poisoned_state("cmd.a", "cmd.b"),
        poisoned_state("cmd.b", "cmd.a")
    );
    // Different competing pairs: distinguishable canonical state.
    assert_ne!(
        poisoned_state("cmd.a", "cmd.b"),
        poisoned_state("cmd.c", "cmd.d")
    );
    // A three-way contest is distinguishable from a two-way one, and is
    // itself order-independent.
    fn poisoned_state_3(order: [&str; 3]) -> Digest {
        let epoch = TimelineEpoch(1);
        let mut ingress = TimelineIngress::new(profile(), epoch, sequencer(), 2).unwrap();
        for command in order {
            let ticket = ingress.current_admission_window().unwrap().ticket();
            let envelope = SemanticCommandEnvelope {
                command_id: CommandId::new(command).unwrap(),
                profile_id: profile(),
                timeline_epoch: epoch,
                effective_time: LogicalTime(0),
                source_id: sequencer(),
                source_sequence: 0,
                input_ordinal: Ordinal(0),
                command_kind: CommandKind::new(CanonicalTag::new("test.command").unwrap()),
                canonical_payload_hash: hash_bytes(command.as_bytes()),
            };
            let _ = ingress.stage(&sequencer(), &envelope.submit_with(ticket));
        }
        ingress.canonical_state_digest()
    }
    let three = poisoned_state_3(["cmd.a", "cmd.b", "cmd.c"]);
    assert_eq!(three, poisoned_state_3(["cmd.c", "cmd.a", "cmd.b"]));
    assert_eq!(three, poisoned_state_3(["cmd.b", "cmd.c", "cmd.a"]));
    assert_ne!(three, poisoned_state("cmd.a", "cmd.b"));
}

/// **AT-F3** (Codex counterexample #11) — a per-field mutation sweep:
/// `StageAcknowledgement::covers` must fail after mutating **any**
/// semantic field of the envelope it was issued for.
#[test]
fn stage_acknowledgement_covers_fails_after_any_field_mutation() {
    let epoch = TimelineEpoch(1);
    let mut ingress = TimelineIngress::new(profile(), epoch, sequencer(), 4).unwrap();
    let original = SemanticCommandEnvelope {
        command_id: CommandId::new("cmd.0").unwrap(),
        profile_id: profile(),
        timeline_epoch: epoch,
        effective_time: LogicalTime(3),
        source_id: sequencer(),
        source_sequence: 9,
        input_ordinal: Ordinal(0),
        command_kind: CommandKind::new(CanonicalTag::new("test.command").unwrap()),
        canonical_payload_hash: hash_bytes(b"payload.0"),
    };
    let ticket = ingress.current_admission_window().unwrap().ticket();
    let disposition = ingress
        .stage(&sequencer(), &original.clone().submit_with(ticket))
        .unwrap();
    let ack = disposition.acknowledgement().unwrap();
    assert!(ack.covers(&original));

    let mutations: Vec<(&str, SemanticCommandEnvelope)> = vec![
        ("command_id", {
            let mut e = original.clone();
            e.command_id = CommandId::new("cmd.other").unwrap();
            e
        }),
        ("profile_id", {
            let mut e = original.clone();
            e.profile_id = ProfileId::new("mci-social").unwrap();
            e
        }),
        ("timeline_epoch", {
            let mut e = original.clone();
            e.timeline_epoch = TimelineEpoch(2);
            e
        }),
        ("effective_time", {
            let mut e = original.clone();
            e.effective_time = LogicalTime(4);
            e
        }),
        ("source_id", {
            let mut e = original.clone();
            e.source_id = SourceId::new("sequencer.other").unwrap();
            e
        }),
        ("source_sequence", {
            let mut e = original.clone();
            e.source_sequence = 10;
            e
        }),
        ("input_ordinal", {
            let mut e = original.clone();
            e.input_ordinal = Ordinal(1);
            e
        }),
        ("command_kind", {
            let mut e = original.clone();
            e.command_kind = CommandKind::new(CanonicalTag::new("other.command").unwrap());
            e
        }),
        ("canonical_payload_hash", {
            let mut e = original.clone();
            e.canonical_payload_hash = hash_bytes(b"payload.other");
            e
        }),
    ];

    for (field, mutated) in mutations {
        assert!(
            !ack.covers(&mutated),
            "acknowledgement must not cover an envelope with a mutated {field}"
        );
        assert_ne!(
            original.semantic_hash(),
            mutated.semantic_hash(),
            "{field} must reach the semantic identity hash"
        );
    }
}

/// **AT-E5** — the original Phase-1 minimum corpus (implementation brief
/// §5, items 1–20) remains represented. Items 1–10 are proved in
/// `spark-core/tests/timeline_admission.rs`; items 11–13 in
/// `spark-engine`'s state/activation suites and
/// `write_paths_still_enforce_every_validation_class` above; item 14 in
/// `spark-engine`'s manifest suite; item 19 in the scenario harness; item
/// 20 in `workspace_dependency_direction.rs`. The remaining items — 15,
/// 16, 17 and 18 — are asserted here from the external vantage point so
/// the whole corpus is provably represented from outside the canonical
/// crates.
#[test]
fn original_minimum_corpus_items_15_to_18_remain_proved() {
    let service = RandomAddressService::new();
    let address = |occurrence: u64| RandomAddress {
        root_seed: 42,
        profile_id: profile(),
        behavior_epoch: 1,
        behavior_artifact_hash: hash_bytes(b"manifest.v1"),
        rule_or_trigger_id: DefinitionId::new("trigger.weather.drought").unwrap(),
        scope_id: ScopeId::new(ScopeKind::Region, "northwood").unwrap(),
        occurrence_index: occurrence,
    };

    // Item 15: the same seed/address inputs produce identical results.
    assert_eq!(
        service.derive_u64(&address(7)),
        service.derive_u64(&address(7))
    );

    // Item 16: unrelated addresses are isolated from call order.
    let forward_a = service.derive_u64(&address(1));
    let forward_b = service.derive_u64(&address(2));
    let reverse_b = service.derive_u64(&address(2));
    let reverse_a = service.derive_u64(&address(1));
    assert_eq!(forward_a, reverse_a);
    assert_eq!(forward_b, reverse_b);
    assert_ne!(forward_a, forward_b);

    // Item 17: the logical clock rejects backward advancement.
    let mut clock = LogicalClock::new(LogicalTime(5));
    assert!(clock.advance_to(LogicalTime(6)).is_ok());
    assert!(clock.advance_to(LogicalTime(5)).is_err());
    assert_eq!(clock.now(), LogicalTime(6));

    // Item 18: due work with equal time has a stable total order.
    let mut sched = Scheduler::new();
    for producer in ["trigger.c", "trigger.a", "trigger.b"] {
        sched.schedule(DueWorkItem {
            key: WorkKey {
                due_time: LogicalTime(5),
                profile_id: profile(),
                producer_definition_id: DefinitionId::new(producer).unwrap(),
                scope_id: ScopeId::new(ScopeKind::Actor, "bron").unwrap(),
                occurrence_index: OccurrenceIndex(0),
                work_kind: WorkKind::new(CanonicalTag::new("trigger.evaluate").unwrap()),
            },
            payload: WorkPayload::new(hash_bytes(b"payload")),
        });
    }
    let outcome = sched.drain_due(LogicalTime(5));
    let ordered: Vec<&str> = outcome
        .due
        .iter()
        .map(|w| w.key.producer_definition_id.as_str())
        .collect();
    assert_eq!(ordered, vec!["trigger.a", "trigger.b", "trigger.c"]);
}
