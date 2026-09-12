//! The protocol's canonical first-proof fixture, as **test data**.
//!
//! `SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` §5 Gate C4 fixes the first
//! end-to-end proof as the causal sequence
//!
//! ```text
//! drought -> food/affordability -> household exposure -> actor choice
//!         -> hunting -> wildlife decline -> neighbouring-settlement feedback
//! ```
//!
//! Everything in this module is fixture data for exercising that sequence
//! across the contract seam. **The constants are test data, not new gameplay
//! laws**, and nothing here proposes a G.A.M.E. balance decision: G.A.M.E.
//! owns the world, its numbers and its rules.

use std::collections::BTreeSet;

use spark_core::authority::Authority;
use spark_core::clock::LogicalTime;
use spark_core::id::{CanonicalTag, CommandId, DefinitionId, ProfileId, SourceId};
use spark_core::scope::{ScopeId, ScopeKind};
use spark_core::timeline::{CommandKind, TimelineEpoch};
use spark_core::value::{CanonicalValue, FixedPoint, ValueConstraint};
use spark_engine::activation::{ActivatedProfile, ActivationRegistry};
use spark_engine::engine::{Engine, EngineGenesis, InitialWork, TimelineGenesis};
use spark_engine::profile::config::ConfigRevision;
use spark_engine::profile::definition::{DefinitionKind, DefinitionSpec};
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::profile::text::BoundedText;
use spark_engine::request::{CommandPayload, CommandRequest, Observation, Request};
use spark_engine::rules::{
    CmpOp, Condition, DeclaredBudgets, EmitOp, Expr, Input, RuleSetSpec, RuleSpec, ScopeRef,
    Trigger, Update,
};

use crate::{EntityMap, ExternalEntityRef};

pub const PROFILE: &str = "game-world";
pub const SEQUENCER: &str = "sequencer.engine";
pub const HOST_SOURCE: &str = "game.host";
/// The reserved `domain` tag that declares a definition an intent channel
/// (contract §6.2).
pub const INTENT_DOMAIN: &str = "intent";

const WIDE: i64 = 1_000_000_000;

pub fn profile_id() -> ProfileId {
    ProfileId::new(PROFILE).expect("a valid profile id")
}

pub fn def(id: &str) -> DefinitionId {
    DefinitionId::new(id).expect("a valid definition id")
}

fn tag(s: &str) -> CanonicalTag {
    CanonicalTag::new(s).expect("a valid canonical tag")
}

fn kind(s: &str) -> CommandKind {
    CommandKind::new(tag(s))
}

// ------------------------------------------------------------------ scopes

pub fn watershed() -> ScopeId {
    ScopeId::new(ScopeKind::Watershed, "upper-valley").expect("scope")
}
pub fn settlement() -> ScopeId {
    ScopeId::new(ScopeKind::Settlement, "riverbend").expect("scope")
}
pub fn neighbour() -> ScopeId {
    ScopeId::new(ScopeKind::Settlement, "stonebridge").expect("scope")
}
pub fn household() -> ScopeId {
    ScopeId::new(ScopeKind::Household, "miller").expect("scope")
}
pub fn actor() -> ScopeId {
    ScopeId::new(ScopeKind::Actor, "elin").expect("scope")
}
pub fn neighbour_actor() -> ScopeId {
    ScopeId::new(ScopeKind::Actor, "harro").expect("scope")
}
pub fn region() -> ScopeId {
    ScopeId::new(ScopeKind::Region, "north-wolds").expect("scope")
}

/// The host-side mapping of contract §4.1 for this fixture.
pub fn entity_map() -> EntityMap {
    let mut m = EntityMap::default();
    for (external, scope) in [
        ("game:watershed/1", watershed()),
        ("game:settlement/1", settlement()),
        ("game:settlement/2", neighbour()),
        ("game:household/1", household()),
        ("game:actor/1", actor()),
        ("game:actor/2", neighbour_actor()),
        ("game:region/1", region()),
    ] {
        m.bind(ExternalEntityRef(external.to_string()), scope)
            .expect("the fixture mapping is injective and stable");
    }
    m
}

// ------------------------------------------------------------- definitions

fn spec(
    id: &str,
    authority: Authority,
    scopes: &[ScopeKind],
    domain: Option<&str>,
) -> DefinitionSpec {
    DefinitionSpec {
        profile_id: profile_id(),
        id: def(id),
        kind: DefinitionKind::StateDefinition,
        domain: domain.map(tag),
        layer: None,
        value_constraint: ValueConstraint::int(-WIDE, WIDE).expect("constraint"),
        authority,
        valid_scopes: scopes.iter().cloned().collect::<BTreeSet<_>>(),
        enabled: true,
        version: 1,
        description: BoundedText::new("convergence first-proof fixture").expect("text"),
        behavioral_leverage: None,
    }
}

pub fn manifest() -> ProfileManifest {
    use ScopeKind::*;
    ProfileManifest::new(
        profile_id(),
        BoundedText::new("spark.game.first_proof").expect("text"),
        vec![
            // G.A.M.E.-owned physical/economic truth. Enters only by ingress.
            spec(
                "world.drought",
                Authority::HostOwned,
                &[Watershed, Settlement],
                None,
            ),
            spec(
                "world.food_supply",
                Authority::HostOwned,
                &[Settlement],
                None,
            ),
            spec("world.wildlife", Authority::HostOwned, &[Region], None),
            // S.P.A.R.K.-owned appraisal/pressure state.
            spec("econ.food_price", Authority::Derived, &[Settlement], None),
            spec("household.exposure", Authority::Derived, &[Household], None),
            spec("actor.stress", Authority::SparkOwned, &[Actor], None),
            spec(
                "neighbour.pressure",
                Authority::Derived,
                &[Settlement],
                None,
            ),
            // The advisory intent channel (contract §6.2).
            spec(
                "intent.hunt",
                Authority::SparkOwned,
                &[Actor],
                Some(INTENT_DOMAIN),
            ),
        ],
    )
}

pub fn intent_channels() -> Vec<DefinitionId> {
    vec![def("intent.hunt")]
}

// -------------------------------------------------------------- rule bodies

fn cell_at(definition: &str, scope: ScopeId) -> Input {
    Input::Cell {
        definition: def(definition),
        scope: ScopeRef::Fixed(scope),
        absent: 0,
    }
}

fn emit_at(sub: &str, target: &str, scope: ScopeId, update: Update) -> EmitOp {
    EmitOp {
        sub_id: tag(sub),
        target: def(target),
        scope: ScopeRef::Fixed(scope),
        update,
    }
}

fn emit_here(sub: &str, target: &str, update: Update) -> EmitOp {
    EmitOp {
        sub_id: tag(sub),
        target: def(target),
        scope: ScopeRef::Subject,
        update,
    }
}

fn w(v: i64) -> FixedPoint {
    FixedPoint::from_integer(v).expect("a representable weight")
}

/// The causal chain. Every constant is fixture test data.
pub fn rules() -> Vec<RuleSpec> {
    vec![
        // 1. drought + food supply -> affordability (price) at the settlement.
        //    price = 3*drought - 1*food_supply, floored at the engine's
        //    checked weighted-sum semantics.
        RuleSpec {
            rule_id: def("rule.affordability"),
            trigger: Trigger::Command(kind("cmd.world_tick")),
            conditions: vec![],
            emits: vec![emit_here(
                "price",
                "econ.food_price",
                Update::Assign(Expr::WeightedSum(vec![
                    (w(3), cell_at("world.drought", settlement())),
                    (w(-1), cell_at("world.food_supply", settlement())),
                ])),
            )],
            schedules: vec![],
            cooldown: None,
        },
        // 2. affordability -> household exposure.
        RuleSpec {
            rule_id: def("rule.exposure"),
            trigger: Trigger::Change {
                watched: def("econ.food_price"),
            },
            conditions: vec![],
            emits: vec![emit_at(
                "exposure",
                "household.exposure",
                household(),
                Update::Assign(Expr::Input(cell_at("econ.food_price", settlement()))),
            )],
            schedules: vec![],
            cooldown: None,
        },
        // 3. household exposure -> actor stress.
        RuleSpec {
            rule_id: def("rule.stress"),
            trigger: Trigger::Change {
                watched: def("household.exposure"),
            },
            conditions: vec![],
            emits: vec![emit_at(
                "stress",
                "actor.stress",
                actor(),
                Update::Assign(Expr::Input(cell_at("household.exposure", household()))),
            )],
            schedules: vec![],
            cooldown: None,
        },
        // 4. actor choice: above the fixture threshold, S.P.A.R.K. *advises*
        //    hunting. This is an intent, never an executable command.
        RuleSpec {
            rule_id: def("rule.choice"),
            trigger: Trigger::Change {
                watched: def("actor.stress"),
            },
            // Subject-scoped: the *changed* actor's own stress decides, and
            // the advice is emitted for that same actor. A fixed scope here
            // would let one actor's stress advise a different actor.
            conditions: vec![Condition::Compare {
                left: Expr::Input(Input::Cell {
                    definition: def("actor.stress"),
                    scope: ScopeRef::Subject,
                    absent: 0,
                }),
                op: CmpOp::Ge,
                right: Expr::Input(Input::Literal(40)),
            }],
            emits: vec![emit_here(
                "hunt",
                "intent.hunt",
                Update::Assign(Expr::Input(Input::Literal(1))),
            )],
            schedules: vec![],
            cooldown: None,
        },
        // 5. G.A.M.E.-confirmed wildlife decline -> neighbouring-settlement
        //    pressure. The wildlife value is host truth, re-entering only as a
        //    typed observation on `cmd.confirm`.
        RuleSpec {
            rule_id: def("rule.neighbour"),
            trigger: Trigger::Command(kind("cmd.confirm")),
            conditions: vec![],
            emits: vec![emit_at(
                "pressure",
                "neighbour.pressure",
                neighbour(),
                Update::Assign(Expr::WeightedSum(vec![
                    (w(-1), cell_at("world.wildlife", region())),
                    (w(1), Input::Literal(100)),
                ])),
            )],
            schedules: vec![],
            cooldown: None,
        },
        // 6. neighbouring-settlement feedback reaches its own actor.
        RuleSpec {
            rule_id: def("rule.neighbour_actor"),
            trigger: Trigger::Change {
                watched: def("neighbour.pressure"),
            },
            conditions: vec![],
            emits: vec![emit_at(
                "stress",
                "actor.stress",
                neighbour_actor(),
                Update::Assign(Expr::Input(cell_at("neighbour.pressure", neighbour()))),
            )],
            schedules: vec![],
            cooldown: None,
        },
    ]
}

pub fn budgets() -> DeclaredBudgets {
    DeclaredBudgets {
        max_due_per_cycle: 16,
        max_cohort_candidates: 256,
        max_effects_per_wave: 256,
        max_wave_depth: 8,
        max_enqueue_per_wave: 256,
        max_obligations: 4_096,
        max_scheduled_work: 4_096,
        max_fan_out: 16,
    }
}

/// Activate the fixture profile and build a fresh engine at genesis.
pub fn device_engine() -> (Engine, ActivatedProfile) {
    device_engine_paced(budgets().max_due_per_cycle, &[])
}

/// A heartbeat work rule, used only to create paced scheduled cohorts so the
/// pause/resume and `Busy` behaviours of contract §4.4 can be exercised.
fn heartbeat_rule() -> RuleSpec {
    RuleSpec {
        rule_id: def("rule.heartbeat"),
        trigger: Trigger::Work(spark_core::scheduler::WorkKind::new(tag("work.tick"))),
        conditions: vec![],
        emits: vec![emit_at(
            "beat",
            "actor.stress",
            actor(),
            Update::Add(Expr::Input(Input::Literal(1))),
        )],
        schedules: vec![],
        cooldown: None,
    }
}

/// Genesis with a chosen pacing budget and scheduled heartbeat work at the
/// given due times.
pub fn device_engine_paced(
    max_due_per_cycle: u32,
    due_times: &[u64],
) -> (Engine, ActivatedProfile) {
    let mut registry = ActivationRegistry::new();
    let m = manifest();
    let profile = registry
        .activate(&m)
        .expect("the fixture manifest activates");
    let mut all_rules = rules();
    if !due_times.is_empty() {
        all_rules.push(heartbeat_rule());
    }
    let mut b = budgets();
    b.max_due_per_cycle = max_due_per_cycle;
    let rule_set = registry
        .activate_rule_set(
            &profile,
            &RuleSetSpec {
                profile_id: profile_id(),
                budgets: b,
                rules: all_rules,
                baselines: vec![],
            },
        )
        .expect("the fixture rule set activates");
    let config = ConfigRevision::build(
        profile_id(),
        BoundedText::new("first_proof.config").expect("text"),
        std::iter::empty(),
    )
    .expect("config");
    let engine = Engine::genesis(EngineGenesis {
        profile: profile.clone(),
        rule_set,
        config,
        timeline: TimelineGenesis {
            timeline_epoch: TimelineEpoch(0),
            sequencer: SourceId::new(SEQUENCER).expect("sequencer"),
            window_width: 4,
        },
        start_time: LogicalTime(0),
        initial_work: due_times
            .iter()
            .map(|t| InitialWork {
                rule_id: def("rule.heartbeat"),
                scope: actor(),
                due_time: LogicalTime(*t),
                work_kind: spark_core::scheduler::WorkKind::new(tag("work.tick")),
            })
            .collect(),
    })
    .expect("genesis");
    (engine, profile)
}

// ---------------------------------------------------------------- requests

/// One host command carrying typed, scoped observations at a single subject.
pub fn host_command(
    id: &str,
    effective: u64,
    sequence: u64,
    command_kind: &str,
    subject: ScopeId,
    observations: &[(&str, i64)],
) -> Request {
    Request::Command(CommandRequest {
        command_id: CommandId::new(id).expect("command id"),
        profile_id: profile_id(),
        timeline_epoch: TimelineEpoch(0),
        effective_time: LogicalTime(effective),
        source_id: SourceId::new(HOST_SOURCE).expect("source"),
        source_sequence: sequence,
        command_kind: kind(command_kind),
        payload: CommandPayload::Host {
            subject,
            observations: observations
                .iter()
                .map(|(d, v)| Observation {
                    definition: def(d),
                    value: CanonicalValue::Int(*v),
                })
                .collect(),
            params: vec![],
        },
    })
}

pub fn advance(t: u64) -> Request {
    Request::Advance(LogicalTime(t))
}

// ------------------------------------------------------------------ replay

/// The exact genesis inputs of [`device_engine`], rebuilt from the same
/// declarations. Re-activating identical content in a fresh registry yields
/// byte-identical activation artifacts, which is what makes replay possible at
/// all (`spark-engine` crate documentation, content addressing).
pub fn genesis_inputs() -> EngineGenesis {
    let mut registry = ActivationRegistry::new();
    let m = manifest();
    let profile = registry
        .activate(&m)
        .expect("the fixture manifest activates");
    let rule_set = registry
        .activate_rule_set(
            &profile,
            &RuleSetSpec {
                profile_id: profile_id(),
                budgets: budgets(),
                rules: rules(),
                baselines: vec![],
            },
        )
        .expect("the fixture rule set activates");
    let config = ConfigRevision::build(
        profile_id(),
        BoundedText::new("first_proof.config").expect("text"),
        std::iter::empty(),
    )
    .expect("config");
    EngineGenesis {
        profile,
        rule_set,
        config,
        timeline: TimelineGenesis {
            timeline_epoch: TimelineEpoch(0),
            sequencer: SourceId::new(SEQUENCER).expect("sequencer"),
            window_width: 4,
        },
        start_time: LogicalTime(0),
        initial_work: vec![],
    }
}

/// The fixture's own command catalogue, by command id.
///
/// A `SemanticCommandEnvelope` stores only the canonical payload *hash*, so a
/// replaying party must already hold the payload. Contract §10.1 is explicit
/// that replay inputs are the finalized commands **with their payloads**; this
/// function is the fixture's stand-in for the host-side command log a real
/// adapter would keep.
pub fn replayable_command(command_id: &str) -> Option<CommandRequest> {
    let requests = [
        host_command(
            "cmd.tick.1",
            10,
            1,
            "cmd.world_tick",
            settlement(),
            &[("world.drought", 30), ("world.food_supply", 20)],
        ),
        host_command(
            "cmd.confirm.1",
            11,
            2,
            "cmd.confirm",
            region(),
            &[("world.wildlife", 90)],
        ),
    ];
    requests.into_iter().find_map(|r| match r {
        Request::Command(c) if c.command_id.as_str() == command_id => Some(c),
        _ => None,
    })
}
