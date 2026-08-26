//! Phase-1 re-foundation **baseline** adversarial corpus.
//!
//! Every test in this file encodes one still-open counterexample from
//! `engineering/phase1/PHASE_1_REFOUNDATION_BRIEF_v0.1.md` against the
//! **inherited** (pre-re-foundation) implementation, from an external
//! crate — i.e. exactly the vantage point an integrator or a Phase-2
//! crate would have.
//!
//! These tests are expected to FAIL at this commit. That failure is the
//! recorded baseline required by the re-foundation mission's Step 2. The
//! file is superseded by the permanent adversarial corpus once the
//! affected foundation boundaries are re-founded.

use spark_core::authority::Authority;
use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, Digest};
use spark_core::id::{CommandId, DefinitionId, FenceId, ProfileId, SourceId};
use spark_core::scheduler::{DueWorkItem, OccurrenceIndex, Scheduler};
use spark_core::scope::{ScopeId, ScopeKind};
use spark_core::state::{DefinitionSchema, StateStore};
use spark_core::timeline::{
    compute_ordered_stream_digest, semantic_envelope_hash, CommandEnvelope, Ordinal, StageOutcome,
    TimelineEpoch, TimelineFence, TimelineIngress,
};
use spark_core::value::{CanonicalValue, ValueConstraint};
use spark_profile::config::{ConfigEntry, ConfigRevision};
use spark_profile::definition::{DefinitionKind, DefinitionSpec};
use std::collections::BTreeSet;

fn profile() -> ProfileId {
    ProfileId::new("game-world").unwrap()
}

fn sequencer() -> SourceId {
    SourceId::new("sequencer.primary").unwrap()
}

// ---------------------------------------------------------------------
// B-01 / M-02 — nonsemantic admission timing must not contaminate
// finalized canonical replay identity.
// ---------------------------------------------------------------------

fn envelope(ingress: &TimelineIngress, ordinal: u64) -> CommandEnvelope {
    let window = ingress.current_admission_window();
    CommandEnvelope {
        command_id: CommandId::new(format!("cmd.{ordinal}")).unwrap(),
        profile_id: window.profile_id.clone(),
        timeline_epoch: window.timeline_epoch,
        effective_time: LogicalTime(ordinal),
        source_id: sequencer(),
        source_sequence: ordinal,
        input_ordinal: Ordinal(ordinal),
        admission_window_token: window.token.clone(),
        command_kind: "test.command".to_string(),
        canonical_payload_hash: hash_bytes(format!("payload.{ordinal}").as_bytes()),
    }
}

fn fence_over(
    ingress: &TimelineIngress,
    start: u64,
    end: u64,
    tag: &str,
    ordered: &[CommandEnvelope],
) -> TimelineFence {
    let window = ingress.current_admission_window();
    let pairs: Vec<(Ordinal, Digest)> = ordered
        .iter()
        .map(|e| (e.input_ordinal, semantic_envelope_hash(e)))
        .collect();
    TimelineFence {
        profile_id: window.profile_id,
        timeline_epoch: window.timeline_epoch,
        fence_id: FenceId::new(tag).unwrap(),
        start_ordinal: Ordinal(start),
        end_ordinal: Ordinal(end),
        previous_fence_hash: window.last_finalized_fence_hash,
        ordered_stream_digest: compute_ordered_stream_digest(&pairs),
    }
}

/// Two runs finalize the *same* semantic commands behind the *same* fence
/// chain. They differ only in whether ordinal 1 was staged before the
/// frontier advanced (old, still-valid overlapping window token) or after
/// (new token). Canonical replay identity must be identical.
#[test]
fn baseline_admission_timing_must_not_change_canonical_state_digest() {
    // Run A: stage 0 and 1 under the genesis window token, fence [0,0],
    // then fence [1,1].
    let mut a = TimelineIngress::new(profile(), TimelineEpoch(1), sequencer(), 2);
    let a0 = envelope(&a, 0);
    let a1 = envelope(&a, 1);
    assert_eq!(a.stage(&sequencer(), &a0).unwrap(), StageOutcome::Staged);
    assert_eq!(a.stage(&sequencer(), &a1).unwrap(), StageOutcome::Staged);
    let fa = fence_over(&a, 0, 0, "fence.1", &[a0.clone()]);
    a.submit_fence(&sequencer(), &fa).unwrap();
    let fa2 = fence_over(&a, 1, 1, "fence.2", &[a1.clone()]);
    a.submit_fence(&sequencer(), &fa2).unwrap();

    // Run B: stage 0, fence [0,0], and only *then* stage ordinal 1 --
    // now under the new frontier token -- and fence [1,1].
    let mut b = TimelineIngress::new(profile(), TimelineEpoch(1), sequencer(), 2);
    let b0 = envelope(&b, 0);
    assert_eq!(b.stage(&sequencer(), &b0).unwrap(), StageOutcome::Staged);
    let fb = fence_over(&b, 0, 0, "fence.1", &[b0.clone()]);
    b.submit_fence(&sequencer(), &fb).unwrap();
    let b1 = envelope(&b, 1);
    assert_eq!(b.stage(&sequencer(), &b1).unwrap(), StageOutcome::Staged);
    let fb2 = fence_over(&b, 1, 1, "fence.2", &[b1.clone()]);
    b.submit_fence(&sequencer(), &fb2).unwrap();

    // Same semantic envelopes, same fence chain.
    assert_eq!(
        semantic_envelope_hash(&a1),
        semantic_envelope_hash(&b1),
        "semantic identity must already be token-free"
    );
    assert_eq!(a.finalized_commands().len(), b.finalized_commands().len());

    assert_eq!(
        a.canonical_state_digest(),
        b.canonical_state_digest(),
        "nonsemantic admission timing must not become canonical replay identity"
    );
}

/// The retained finalized envelope must not carry the ephemeral
/// admission-window token at all: two finalized histories that agree
/// semantically must be byte-identical in canonical form.
#[test]
fn baseline_finalized_history_must_not_retain_admission_token() {
    let mut a = TimelineIngress::new(profile(), TimelineEpoch(1), sequencer(), 4);
    let a0 = envelope(&a, 0);
    a.stage(&sequencer(), &a0).unwrap();
    let fa = fence_over(&a, 0, 0, "fence.1", &[a0.clone()]);
    a.submit_fence(&sequencer(), &fa).unwrap();

    // A second ingress with a *different* window width sees a different
    // admission-window token for the identical semantic command.
    let mut b = TimelineIngress::new(profile(), TimelineEpoch(1), sequencer(), 2);
    let b0 = envelope(&b, 0);
    b.stage(&sequencer(), &b0).unwrap();
    let fb = fence_over(&b, 0, 0, "fence.1", &[b0.clone()]);
    b.submit_fence(&sequencer(), &fb).unwrap();

    assert_eq!(semantic_envelope_hash(&a0), semantic_envelope_hash(&b0));
    assert_eq!(
        a.finalized_commands()[0].envelope, b.finalized_commands()[0].envelope,
        "finalized canonical history must contain semantic command data only"
    );
}

// ---------------------------------------------------------------------
// B-02 — the activated state schema must not be publicly forgeable.
// ---------------------------------------------------------------------

/// External code fabricates a `Derived` authority schema with a zero
/// fingerprint and installs it into a `StateStore`. This must be
/// structurally impossible, not merely discouraged.
#[test]
fn baseline_external_code_must_not_forge_and_activate_a_schema() {
    let forged = DefinitionSchema {
        profile_id: profile(),
        definition_id: DefinitionId::new("state.forged").unwrap(),
        fingerprint: Digest::ZERO,
        authority: Authority::Derived,
        value_constraint: ValueConstraint::Bool,
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
    };
    let store = StateStore::new([forged]);
    assert!(
        store
            .schema_of(&profile(), &DefinitionId::new("state.forged").unwrap())
            .is_none(),
        "a fabricated raw schema must never become activated authority"
    );
}

/// Two schema entries claiming the same `(profile, definition)` key must
/// be rejected, not silently resolved by last-writer-wins.
#[test]
fn baseline_duplicate_schema_keys_must_reject() {
    let id = DefinitionId::new("state.contested").unwrap();
    let first = DefinitionSchema {
        profile_id: profile(),
        definition_id: id.clone(),
        fingerprint: Digest::ZERO,
        authority: Authority::HostOwned,
        value_constraint: ValueConstraint::Bool,
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
    };
    let mut second = first.clone();
    second.authority = Authority::SparkOwned;

    let store = StateStore::new([first, second]);
    let installed = store.schema_of(&profile(), &id);
    assert!(
        installed.is_none(),
        "duplicate schema keys must reject rather than let insertion order pick the winner"
    );
}

/// A validated `DefinitionSpec` must not be convertible into activated
/// authority without going through the identity-registry validation
/// ceremony.
#[test]
fn baseline_definition_spec_must_not_self_activate() {
    let spec = DefinitionSpec {
        profile_id: profile(),
        id: DefinitionId::new("state.unvalidated").unwrap(),
        kind: DefinitionKind::StateDefinition,
        domain: None,
        layer: None,
        value_constraint: ValueConstraint::Bool,
        authority: Authority::HostOwned,
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
        enabled: true,
        version: 1,
        description: "never validated".to_string(),
        behavioral_leverage: None,
    };
    // `to_definition_schema` is public and performs no validation, so an
    // unvalidated spec becomes activated authority directly.
    let store = StateStore::new([spec.to_definition_schema()]);
    assert!(
        store
            .schema_of(&profile(), &DefinitionId::new("state.unvalidated").unwrap())
            .is_none(),
        "activation must require a validated profile/identity-registry result"
    );
}

// ---------------------------------------------------------------------
// B-03 — the scheduler's semantic work key must be complete.
// ---------------------------------------------------------------------

fn work(due: u64, producer: &str, scope: &str, occurrence: u64) -> DueWorkItem {
    DueWorkItem {
        due_time: LogicalTime(due),
        profile_id: profile(),
        producer_definition_id: DefinitionId::new(producer).unwrap(),
        scope_id: ScopeId::new(ScopeKind::Actor, scope).unwrap(),
        occurrence_index: OccurrenceIndex(occurrence),
        work_kind: "trigger.evaluate".to_string(),
    }
}

/// Two independent producers each legitimately schedule *their own*
/// occurrence 0 at the same due time. Both must be retained.
#[test]
fn baseline_independent_producers_may_share_an_occurrence_index() {
    let mut sched = Scheduler::new();
    sched.schedule(work(5, "trigger.a", "bron", 0)).unwrap();
    sched
        .schedule(work(5, "trigger.b", "bron", 0))
        .expect("independent producers must not collide on occurrence 0");
    assert_eq!(sched.len(), 2);
}

/// The same two independent items inserted in reversed order must drain
/// identically; first arrival must never select which logical work
/// survives.
#[test]
fn baseline_reversed_insertion_of_independent_producers_drains_identically() {
    let mut forward = Scheduler::new();
    forward.schedule(work(5, "trigger.a", "bron", 0)).unwrap();
    forward.schedule(work(5, "trigger.b", "bron", 0)).unwrap();

    let mut reversed = Scheduler::new();
    reversed.schedule(work(5, "trigger.b", "bron", 0)).unwrap();
    reversed.schedule(work(5, "trigger.a", "bron", 0)).unwrap();

    assert_eq!(
        forward.drain_due(LogicalTime(5)),
        reversed.drain_due(LogicalTime(5))
    );
}

/// Two scopes under one producer must likewise keep their own occurrence
/// sequences.
#[test]
fn baseline_independent_scopes_may_share_an_occurrence_index() {
    let mut sched = Scheduler::new();
    sched.schedule(work(5, "trigger.a", "bron", 0)).unwrap();
    sched
        .schedule(work(5, "trigger.a", "mira", 0))
        .expect("independent scopes must not collide on occurrence 0");
    assert_eq!(sched.len(), 2);
}

// ---------------------------------------------------------------------
// M-01 — config revisions must be validated, unique-key objects.
// ---------------------------------------------------------------------

fn config(entries: Vec<(&str, CanonicalValue)>) -> ConfigRevision {
    ConfigRevision {
        profile_id: profile(),
        revision_label: "1.0.0".to_string(),
        entries: entries
            .into_iter()
            .map(|(k, v)| ConfigEntry {
                key: DefinitionId::new(k).unwrap(),
                value: v,
            })
            .collect(),
    }
}

/// A config carrying two different values under the same key is
/// ambiguous and must be rejected at construction, not silently hashed.
#[test]
fn baseline_duplicate_config_keys_must_reject() {
    let forward = config(vec![
        ("trigger.a.base_rate", CanonicalValue::Int(1)),
        ("trigger.a.base_rate", CanonicalValue::Int(2)),
    ]);
    let reversed = config(vec![
        ("trigger.a.base_rate", CanonicalValue::Int(2)),
        ("trigger.a.base_rate", CanonicalValue::Int(1)),
    ]);
    // Neither should be constructible; the weakest observable form of
    // that requirement is that duplicate-key content is at least not
    // order-sensitive.
    assert_eq!(
        forward.config_revision_hash(),
        reversed.config_revision_hash(),
        "duplicate-key config must be rejected, never order-sensitively hashed"
    );
}

/// An oversized categorical config value must be rejected before the
/// revision becomes hashable/activatable.
#[test]
fn baseline_oversized_config_value_must_reject() {
    let huge = "x".repeat(100_000);
    let revision = config(vec![(
        "trigger.a.label",
        CanonicalValue::Categorical(huge.clone()),
    )]);
    assert!(
        revision.entries[0]
            .value
            .clone()
            .ne(&CanonicalValue::Categorical(huge)),
        "an unbounded canonical config value must not be constructible"
    );
}

// ---------------------------------------------------------------------
// M-03 — canonical construction and bounds.
// ---------------------------------------------------------------------

/// `CONTROLLING_BLUEPRINT_v0.2.md` §12.3 fixes definition IDs as
/// *namespaced*: `trait.curiosity`, never bare `curiosity`.
#[test]
fn baseline_definition_id_must_require_a_namespace() {
    assert!(
        DefinitionId::new("curiosity").is_err(),
        "a non-namespaced definition ID must be rejected"
    );
    assert!(DefinitionId::new("trait.curiosity").is_ok());
}

/// An incoherent numeric bound (`min > max`) can never accept any value
/// and must be rejected at construction.
#[test]
fn baseline_incoherent_value_bounds_must_reject() {
    let incoherent = ValueConstraint::Int { min: 10, max: 0 };
    assert!(
        !incoherent.accepts(&CanonicalValue::Int(5)),
        "sanity: an incoherent range accepts nothing"
    );
    // The real requirement: such a constraint must not be constructible
    // at all. Observably, a validated definition must refuse it.
    let spec = DefinitionSpec {
        profile_id: profile(),
        id: DefinitionId::new("state.incoherent").unwrap(),
        kind: DefinitionKind::StateDefinition,
        domain: None,
        layer: None,
        value_constraint: incoherent,
        authority: Authority::SparkOwned,
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
        enabled: true,
        version: 1,
        description: "incoherent bounds".to_string(),
        behavioral_leverage: None,
    };
    let manifest = spark_profile::manifest::ProfileManifest {
        profile_id: profile(),
        version_label: "1.0.0".to_string(),
        definitions: vec![spec],
    };
    let mut registry = spark_profile::identity::DefinitionIdentityRegistry::new();
    assert!(
        spark_profile::validate::validate(&manifest, &mut registry).is_err(),
        "a definition with min > max must fail validation"
    );
}

/// A custom definition-kind tag is canonical identity and must be a
/// bounded, validated identifier rather than an arbitrary `String`.
#[test]
fn baseline_oversized_custom_definition_kind_must_reject() {
    let oversized = "k".repeat(100_000);
    let spec = DefinitionSpec {
        profile_id: profile(),
        id: DefinitionId::new("state.oversized_kind").unwrap(),
        kind: DefinitionKind::Custom(oversized),
        domain: None,
        layer: None,
        value_constraint: ValueConstraint::Bool,
        authority: Authority::SparkOwned,
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
        enabled: true,
        version: 1,
        description: "oversized custom kind".to_string(),
        behavioral_leverage: None,
    };
    let manifest = spark_profile::manifest::ProfileManifest {
        profile_id: profile(),
        version_label: "1.0.0".to_string(),
        definitions: vec![spec],
    };
    let mut registry = spark_profile::identity::DefinitionIdentityRegistry::new();
    assert!(
        spark_profile::validate::validate(&manifest, &mut registry).is_err(),
        "an unbounded custom definition-kind string must fail validation"
    );
}

/// `command_kind` is canonical envelope identity and must likewise be a
/// bounded, validated canonical type rather than an unbounded `String`.
#[test]
fn baseline_oversized_command_kind_must_reject() {
    let mut ingress = TimelineIngress::new(profile(), TimelineEpoch(1), sequencer(), 2);
    let mut env = envelope(&ingress, 0);
    env.command_kind = "c".repeat(100_000);
    let outcome = ingress.stage(&sequencer(), &env);
    assert!(
        outcome.is_err(),
        "an unbounded canonical command kind must never be admitted"
    );
}

/// An oversized categorical canonical value must not be constructible at
/// all, since it becomes canonical state and canonical hash input.
#[test]
fn baseline_oversized_categorical_value_must_reject() {
    let huge = "v".repeat(100_000);
    let value = CanonicalValue::Categorical(huge);
    let permissive = ValueConstraint::Categorical { max_len: usize::MAX };
    assert!(
        !permissive.accepts(&value),
        "an unbounded categorical canonical value must be structurally impossible"
    );
}
