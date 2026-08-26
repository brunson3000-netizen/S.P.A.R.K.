//! Re-Foundation **v2** step-1 baseline: the mandatory AT-A…AT-F
//! acceptance assertions from
//! `PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md` §10a and
//! `PHASE_1_REFOUNDATION_V2_IMPLEMENTATION_BRIEF.md` §9, expressed
//! against the **inherited** (v1 re-foundation) tree from an external
//! crate vantage point, before any v2 implementation change.
//!
//! Every test in this file states the property the v2 architecture
//! requires and is expected to **fail** here. The recorded failures are
//! the test-first evidence required by the brief §10; the permanent v2
//! corpus in `refoundation_v2_adversarial.rs` restates the same
//! assertions against the re-founded API.
//!
//! Requirements that cannot be expressed as a runtime assertion against
//! the inherited API at all (because the required API does not exist) are
//! recorded separately in
//! `engineering/phase1/refoundation-v2-baseline/BASELINE_V2_UNEXPRESSIBLE_FINDINGS.md`.

use spark_core::activation::{
    DefinitionDeclaration, DefinitionIdentityRegistry, DefinitionKindTag,
};
use spark_core::authority::Authority;
use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, Digest};
use spark_core::id::{CanonicalTag, DefinitionId, ProfileId, SourceId};
use spark_core::scheduler::{
    DueWorkItem, OccurrenceIndex, Scheduler, WorkKey, WorkKind, WorkPayload,
};
use spark_core::scope::{ScopeId, ScopeKind};
use spark_core::timeline::{
    CommandKind, Ordinal, SemanticCommandEnvelope, TimelineEpoch, TimelineIngress,
};
use spark_core::value::{FixedPoint, ValueConstraint};
use spark_profile::definition::{DefinitionKind, DefinitionSpec};
use spark_profile::text::BoundedText;
use std::collections::BTreeSet;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn profile() -> ProfileId {
    ProfileId::new("game-world").unwrap()
}

// ---------------------------------------------------------------------
// AT-A — trusted activation (B-02)
// ---------------------------------------------------------------------

/// AT-A3 (replaces the insufficient `definition_spec_cannot_self_activate`):
/// a **fully valid** `DefinitionSpec` must have no public path to an
/// activated artifact other than a `ProfileManifest` through the single
/// activation door.
#[test]
fn baseline_valid_definition_spec_must_not_self_activate() {
    let spec = DefinitionSpec {
        profile_id: profile(),
        id: DefinitionId::new("trait.curiosity").unwrap(),
        kind: DefinitionKind::Trait,
        domain: None,
        layer: None,
        value_constraint: ValueConstraint::fixed(
            FixedPoint::ZERO,
            FixedPoint::from_integer(1).unwrap(),
        )
        .unwrap(),
        authority: Authority::Derived,
        // A genuinely valid spec: non-empty scopes, coherent constraint.
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
        enabled: true,
        version: 1,
        description: BoundedText::new("valid spec").unwrap(),
        behavioral_leverage: None,
    };

    let mut registry = DefinitionIdentityRegistry::new();
    let self_activated = registry.activate(&profile(), &[spec.to_declaration()]);

    assert!(
        self_activated.is_err(),
        "a fully valid DefinitionSpec must not be able to self-activate through an intermediate \
         mint; activation must run the complete ceremony behind one door"
    );
}

/// AT-A1: caller-authored raw declarations must not reach a trusted mint.
#[test]
fn baseline_raw_declaration_must_not_reach_activation() {
    let declaration = DefinitionDeclaration {
        profile_id: profile(),
        definition_id: DefinitionId::new("state.forged.authority").unwrap(),
        kind: DefinitionKindTag::builtin(CanonicalTag::new("state_definition").unwrap()),
        authority: Authority::HostOwned,
        value_constraint: ValueConstraint::int(0, 10).unwrap(),
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
    };

    let mut registry = DefinitionIdentityRegistry::new();
    let activated = registry.activate(&profile(), &[declaration]);

    assert!(
        activated.is_err(),
        "an external caller must not be able to mint activated authority from a raw \
         caller-authored declaration"
    );
}

/// AT-A7 (Codex counterexample #4): an external caller must not be able
/// to assert the built-in/custom identity bit of a definition kind.
#[test]
fn baseline_builtin_kind_bit_must_not_be_caller_assertable() {
    let asserted = DefinitionKindTag::builtin(CanonicalTag::new("hydrological_pressure").unwrap());
    assert!(
        asserted.is_custom(),
        "external code must not be able to assert that a profile-invented tag is engine baseline \
         vocabulary; `DefinitionKindTag::builtin` must not be externally callable"
    );
}

// ---------------------------------------------------------------------
// AT-B — scheduler conflict (B-03)
// ---------------------------------------------------------------------

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

fn item(payload: &str) -> DueWorkItem {
    DueWorkItem {
        key: work_key(0),
        payload: WorkPayload::new(hash_bytes(payload.as_bytes())),
    }
}

/// AT-B1 (replaces the insufficient `payload_conflict_rejects_in_either_order`):
/// the canonical scheduler state after a same-key conflict must not
/// depend on arrival order.
#[test]
fn baseline_same_key_conflict_state_must_be_arrival_order_independent() {
    let a = item("payload.a");
    let b = item("payload.b");

    let mut forward = Scheduler::new();
    let _ = forward.schedule(a.clone());
    let _ = forward.schedule(b.clone());

    let mut reversed = Scheduler::new();
    let _ = reversed.schedule(b);
    let _ = reversed.schedule(a);

    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest(),
        "a same-key payload conflict must reach one canonical state in either arrival order; \
         first-arrival-wins makes canonical scheduler state an arrival-order artifact"
    );
}

/// AT-B3: after a conflict, neither competing payload may remain
/// executable.
#[test]
fn baseline_neither_payload_survives_a_conflict() {
    let a = item("payload.a");
    let b = item("payload.b");

    let mut sched = Scheduler::new();
    let _ = sched.schedule(a.clone());
    let _ = sched.schedule(b.clone());

    let due = sched.drain_due(LogicalTime(5));
    assert!(
        !due.contains(&a) && !due.contains(&b),
        "neither payload of a poisoned key may be drained as executable work; found {due:?}"
    );
}

/// AT-B2: a three-way conflict must converge to one canonical state in
/// all six arrival permutations.
#[test]
fn baseline_three_way_conflict_must_converge_in_all_orders() {
    let items = [item("payload.a"), item("payload.b"), item("payload.c")];
    let permutations = [
        [0usize, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    let mut digests: BTreeSet<Digest> = BTreeSet::new();
    for permutation in permutations {
        let mut sched = Scheduler::new();
        for index in permutation {
            let _ = sched.schedule(items[index].clone());
        }
        digests.insert(sched.canonical_state_digest());
    }

    assert_eq!(
        digests.len(),
        1,
        "all six arrival permutations of a three-way same-key conflict must converge to one \
         canonical scheduler state; found {} distinct states",
        digests.len()
    );
}

/// AT-B6: conflict evidence must be bounded *and* order-independent, so
/// a claim set larger than the evidence cap still converges.
#[test]
fn baseline_conflict_evidence_cap_must_be_order_independent() {
    let claims: Vec<DueWorkItem> = (0..20u32)
        .map(|i| item(&format!("payload.{i}")))
        .collect();

    let mut forward = Scheduler::new();
    for claim in &claims {
        let _ = forward.schedule(claim.clone());
    }
    let mut reversed = Scheduler::new();
    for claim in claims.iter().rev() {
        let _ = reversed.schedule(claim.clone());
    }

    assert_eq!(
        forward.canonical_state_digest(),
        reversed.canonical_state_digest(),
        "more distinct claims than the evidence cap must still produce identical canonical \
         conflict state in any arrival order"
    );
}

// ---------------------------------------------------------------------
// AT-C — panic-free canonical construction (M-03)
// ---------------------------------------------------------------------

/// AT-C1 (Codex-executed panic): an invalid `&'static str` reaching the
/// static tag constructor outside a const context must return `Err`,
/// never panic.
#[test]
fn baseline_invalid_static_tag_must_not_panic() {
    let outcome = catch_unwind(AssertUnwindSafe(|| CanonicalTag::from_static("INVALID TAG")));
    assert!(
        outcome.is_ok(),
        "a public canonical constructor must not panic on caller input; \
         `CanonicalTag::from_static` panics at runtime for an invalid static string"
    );
}

/// AT-C3 (Codex-executed panic): reversed clamp bounds must be a typed
/// error, never a panic.
#[test]
fn baseline_reversed_clamp_bounds_must_not_panic() {
    let min = FixedPoint::from_integer(1).unwrap();
    let max = FixedPoint::ZERO;
    let value = FixedPoint::from_raw(500_000);
    let outcome = catch_unwind(AssertUnwindSafe(|| value.clamp(min, max)));
    assert!(
        outcome.is_ok(),
        "a public canonical arithmetic operation must not panic on caller input; \
         `FixedPoint::clamp` panics for reversed bounds"
    );
}

// ---------------------------------------------------------------------
// AT-D — reconstruction restriction
// ---------------------------------------------------------------------

/// AT-D1: nonzero-frontier construction must be absent from the default
/// production surface.
#[test]
fn baseline_resume_at_frontier_must_be_absent_from_production_surface() {
    let injected = TimelineIngress::resume_at_frontier(
        profile(),
        TimelineEpoch(1),
        SourceId::new("sequencer.primary").unwrap(),
        2,
        Ordinal(1_000_000),
    );
    assert!(
        injected.is_err(),
        "an external caller must not be able to construct a canonical timeline at an arbitrary \
         nonzero frontier from the default production surface"
    );
}

// ---------------------------------------------------------------------
// AT-F — evidence/digest hardening
// ---------------------------------------------------------------------

fn envelope(ordinal: u64, command: &str, payload: &str) -> SemanticCommandEnvelope {
    SemanticCommandEnvelope {
        command_id: spark_core::id::CommandId::new(command).unwrap(),
        profile_id: profile(),
        timeline_epoch: TimelineEpoch(1),
        effective_time: LogicalTime(ordinal),
        source_id: SourceId::new("sequencer.primary").unwrap(),
        source_sequence: ordinal,
        input_ordinal: Ordinal(ordinal),
        command_kind: CommandKind::new(CanonicalTag::new("scenario.command").unwrap()),
        canonical_payload_hash: hash_bytes(payload.as_bytes()),
    }
}

fn poisoned_ingress_state(first: &str, second: &str) -> Digest {
    let sequencer = SourceId::new("sequencer.primary").unwrap();
    let mut ingress = TimelineIngress::new(profile(), TimelineEpoch(1), sequencer.clone(), 2)
        .expect("ingress config");
    for command in [first, second] {
        let ticket = ingress
            .current_admission_window()
            .expect("window")
            .ticket();
        let submission = envelope(0, command, command).submit_with(ticket);
        let _ = ingress.stage(&sequencer, &submission);
    }
    ingress.canonical_state_digest()
}

/// AT-F1 (Codex counterexample #12): poisoned slots caused by *different*
/// competing envelope pairs must remain distinguishable in canonical
/// ingress state.
#[test]
fn baseline_poison_evidence_must_distinguish_competing_pairs() {
    let pair_one = poisoned_ingress_state("cmd.a", "cmd.b");
    let pair_two = poisoned_ingress_state("cmd.c", "cmd.d");
    assert_ne!(
        pair_one, pair_two,
        "two poisoned slots caused by different competing envelope pairs must not share one \
         canonical ingress state digest; an evidence-free `Poisoned` unit variant erases the \
         difference"
    );
}

/// AT-F1 (companion): the *same* competing pair delivered in opposite
/// orders must produce identical canonical ingress state. This property
/// already holds in the inherited tree and must not regress.
#[test]
fn baseline_poison_evidence_is_arrival_order_independent() {
    assert_eq!(
        poisoned_ingress_state("cmd.a", "cmd.b"),
        poisoned_ingress_state("cmd.b", "cmd.a"),
        "the same competing pair must poison to one canonical state in either arrival order"
    );
}
