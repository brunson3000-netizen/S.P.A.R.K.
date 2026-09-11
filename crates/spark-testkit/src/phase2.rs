//! Phase-2 fixture builders shared by the Gate C2 acceptance suites.
//!
//! Every fixture goes through the production doors: a `ProfileManifest` through
//! `ActivationRegistry::activate`, a `RuleSetSpec` through
//! `ActivationRegistry::activate_rule_set`, and an engine through
//! `Engine::genesis`. Nothing here forges an activated artifact.

use spark_core::authority::Authority;
use spark_core::clock::LogicalTime;
use spark_core::hash::Digest;
use spark_core::id::{CanonicalTag, CommandId, DefinitionId, ProfileId, SourceId};
use spark_core::scheduler::WorkKind;
use spark_core::scope::{ScopeId, ScopeKind};
use spark_core::timeline::{CommandKind, TimelineEpoch};
use spark_core::value::{CanonicalValue, FixedPoint, ValueConstraint};
use spark_engine::activation::{ActivatedProfile, ActivationRegistry};
use spark_engine::engine::{Engine, EngineGenesis, InitialWork, TimelineGenesis};
use spark_engine::profile::config::{ConfigEntry, ConfigRevision};
use spark_engine::profile::definition::{DefinitionKind, DefinitionSpec};
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::profile::text::BoundedText;
use spark_engine::request::{
    CommandPayload, CommandRequest, Observation, Outcome, ProcessResult, Request,
};
use spark_engine::rules::{
    ActivatedRuleSet, BaselineDeclaration, CmpOp, Condition, DeclaredBudgets, EmitOp, Expr, Input,
    Param, RuleSetError, RuleSetSpec, RuleSpec, ScheduleMode, ScheduleOp, ScopeRef, Trigger,
    Update,
};
use std::collections::BTreeSet;

pub const PROFILE: &str = "game-world";
pub const SEQUENCER: &str = "sequencer.engine";
pub const SOURCE: &str = "host.input";

pub fn profile_id() -> ProfileId {
    ProfileId::new(PROFILE).unwrap()
}

pub fn def(id: &str) -> DefinitionId {
    DefinitionId::new(id).unwrap()
}

pub fn tag(s: &str) -> CanonicalTag {
    CanonicalTag::new(s).unwrap()
}

pub fn work(s: &str) -> WorkKind {
    WorkKind::new(tag(s))
}

pub fn kind(s: &str) -> CommandKind {
    CommandKind::new(tag(s))
}

pub fn actor(name: &str) -> ScopeId {
    ScopeId::new(ScopeKind::Actor, name).unwrap()
}

pub fn region(name: &str) -> ScopeId {
    ScopeId::new(ScopeKind::Region, name).unwrap()
}

/// An integer definition valid at actor and region scopes.
pub fn int_spec(id: &str, authority: Authority, min: i64, max: i64) -> DefinitionSpec {
    DefinitionSpec {
        profile_id: profile_id(),
        id: def(id),
        kind: DefinitionKind::StateDefinition,
        domain: None,
        layer: None,
        value_constraint: ValueConstraint::int(min, max).unwrap(),
        authority,
        valid_scopes: BTreeSet::from([ScopeKind::Actor, ScopeKind::Region]),
        enabled: true,
        version: 1,
        description: BoundedText::new("phase-2 fixture definition").unwrap(),
        behavioral_leverage: None,
    }
}

pub fn fixed_spec(id: &str, authority: Authority) -> DefinitionSpec {
    let mut s = int_spec(id, authority, 0, 0);
    s.value_constraint = ValueConstraint::fixed(
        FixedPoint::from_integer(-1_000_000).unwrap(),
        FixedPoint::from_integer(1_000_000).unwrap(),
    )
    .unwrap();
    s
}

const WIDE: i64 = 1_000_000_000_000;

/// The standard Phase-2 test manifest.
pub fn standard_manifest() -> ProfileManifest {
    ProfileManifest::new(
        profile_id(),
        BoundedText::new("phase2.fixture").unwrap(),
        vec![
            int_spec("state.stress", Authority::SparkOwned, -WIDE, WIDE),
            int_spec("state.mood", Authority::SparkOwned, -WIDE, WIDE),
            int_spec("state.energy", Authority::SparkOwned, -WIDE, WIDE),
            int_spec("state.pressure", Authority::Derived, -WIDE, WIDE),
            int_spec("state.total", Authority::Derived, -WIDE, WIDE),
            int_spec("state.food", Authority::HostOwned, -WIDE, WIDE),
            int_spec("state.bounded", Authority::SparkOwned, 0, 10),
            int_spec("state.alarm", Authority::SparkOwned, -WIDE, WIDE),
            int_spec("state.echo", Authority::SparkOwned, -WIDE, WIDE),
            int_spec("state.echo2", Authority::SparkOwned, -WIDE, WIDE),
            fixed_spec("state.intensity", Authority::SparkOwned),
        ],
    )
}

/// One activation registry plus the activated profile, through the door.
pub struct Fixture {
    pub registry: ActivationRegistry,
    pub profile: ActivatedProfile,
}

pub fn activate(manifest: &ProfileManifest) -> Fixture {
    let mut registry = ActivationRegistry::new();
    let profile = registry.activate(manifest).unwrap();
    Fixture { registry, profile }
}

pub fn standard() -> Fixture {
    activate(&standard_manifest())
}

/// Generous semantic caps with a chosen pacing budget.
pub fn budgets(max_due_per_cycle: u32) -> DeclaredBudgets {
    DeclaredBudgets {
        max_due_per_cycle,
        max_cohort_candidates: 1_000,
        max_effects_per_wave: 1_000,
        max_wave_depth: 8,
        max_enqueue_per_wave: 1_000,
        max_obligations: 100_000,
        max_scheduled_work: 100_000,
        max_fan_out: 32,
    }
}

impl Fixture {
    pub fn rule_set(
        &mut self,
        budgets: DeclaredBudgets,
        rules: Vec<RuleSpec>,
    ) -> Result<ActivatedRuleSet, Vec<RuleSetError>> {
        self.rule_set_with(budgets, rules, vec![])
    }

    /// A rule set that also declares baseline semantics (v1 Q7).
    pub fn rule_set_with(
        &mut self,
        budgets: DeclaredBudgets,
        rules: Vec<RuleSpec>,
        baselines: Vec<BaselineDeclaration>,
    ) -> Result<ActivatedRuleSet, Vec<RuleSetError>> {
        self.registry.activate_rule_set(
            &self.profile,
            &RuleSetSpec {
                profile_id: profile_id(),
                budgets,
                rules,
                baselines,
            },
        )
    }

    pub fn genesis(
        &mut self,
        budgets: DeclaredBudgets,
        rules: Vec<RuleSpec>,
        initial_work: Vec<InitialWork>,
    ) -> EngineGenesis {
        self.genesis_with(budgets, rules, vec![], initial_work)
    }

    /// Genesis with declared baselines.
    pub fn genesis_with(
        &mut self,
        budgets: DeclaredBudgets,
        rules: Vec<RuleSpec>,
        baselines: Vec<BaselineDeclaration>,
        initial_work: Vec<InitialWork>,
    ) -> EngineGenesis {
        let rule_set = self.rule_set_with(budgets, rules, baselines).unwrap();
        EngineGenesis {
            profile: self.profile.clone(),
            rule_set,
            config: config(vec![]),
            timeline: TimelineGenesis {
                timeline_epoch: TimelineEpoch(0),
                sequencer: SourceId::new(SEQUENCER).unwrap(),
                window_width: 2,
            },
            start_time: LogicalTime(0),
            initial_work,
        }
    }

    pub fn engine(
        &mut self,
        budgets: DeclaredBudgets,
        rules: Vec<RuleSpec>,
        initial_work: Vec<InitialWork>,
    ) -> Engine {
        Engine::genesis(self.genesis(budgets, rules, initial_work)).unwrap()
    }
}

pub fn config(entries: Vec<(&str, CanonicalValue)>) -> ConfigRevision {
    ConfigRevision::build(
        profile_id(),
        BoundedText::new("phase2.config").unwrap(),
        entries.into_iter().map(|(k, v)| ConfigEntry {
            key: def(k),
            value: v,
        }),
    )
    .unwrap()
}

// ---------------------------------------------------------------- rule builders

pub fn lit(v: i64) -> Expr {
    Expr::Input(Input::Literal(v))
}

pub fn cell(definition: &str) -> Input {
    Input::Cell {
        definition: def(definition),
        scope: ScopeRef::Subject,
        absent: 0,
    }
}

pub fn param(index: u8) -> Expr {
    Expr::Input(Input::Param { index })
}

/// A declared baseline (v1 Q7).
pub fn baseline(definition: &str, value: i64) -> BaselineDeclaration {
    BaselineDeclaration {
        definition: def(definition),
        value,
    }
}

/// A decay/recovery update with literal rate and cadence.
pub fn decay(rate: i64, cadence: i64) -> Update {
    Update::Decay {
        rate: Param::Literal(rate),
        cadence: Param::Literal(cadence),
    }
}

/// A decay/recovery update whose rate is a hot-tunable config key.
pub fn decay_config_rate(key: &str, cadence: i64) -> Update {
    Update::Decay {
        rate: Param::Config { key: def(key) },
        cadence: Param::Literal(cadence),
    }
}

pub fn emit(sub: &str, target: &str, update: Update) -> EmitOp {
    EmitOp {
        sub_id: tag(sub),
        target: def(target),
        scope: ScopeRef::Subject,
        update,
    }
}

pub fn emit_at(sub: &str, target: &str, scope: ScopeId, update: Update) -> EmitOp {
    EmitOp {
        sub_id: tag(sub),
        target: def(target),
        scope: ScopeRef::Fixed(scope),
        update,
    }
}

pub fn rule(id: &str, trigger: Trigger, emits: Vec<EmitOp>) -> RuleSpec {
    RuleSpec {
        rule_id: def(id),
        trigger,
        conditions: Vec::new(),
        emits,
        schedules: Vec::new(),
        cooldown: None,
    }
}

/// A rule evaluated by re-evaluation obligations of `work_kind`.
pub fn work_rule(id: &str, work_kind: &str, emits: Vec<EmitOp>) -> RuleSpec {
    rule(id, Trigger::Work(work(work_kind)), emits)
}

pub fn on_command(id: &str, command_kind: &str, emits: Vec<EmitOp>) -> RuleSpec {
    rule(id, Trigger::Command(kind(command_kind)), emits)
}

pub fn with_schedule(mut r: RuleSpec, s: ScheduleOp) -> RuleSpec {
    r.schedules.push(s);
    r
}

pub fn with_condition(mut r: RuleSpec, c: Condition) -> RuleSpec {
    r.conditions.push(c);
    r
}

pub fn reevaluate_after(sub: &str, delay: u64, work_kind: &str, target_rule: &str) -> ScheduleOp {
    ScheduleOp {
        sub_id: tag(sub),
        delay,
        work_kind: work(work_kind),
        scope: ScopeRef::Subject,
        mode: ScheduleMode::ReEvaluate {
            rule: def(target_rule),
        },
    }
}

pub fn compare(left: Expr, op: CmpOp, right: Expr) -> Condition {
    Condition::Compare { left, op, right }
}

/// Genesis re-evaluation work of `rule_id` at `scope`, due at `due`.
pub fn initial(rule_id: &str, scope: &ScopeId, due: u64, work_kind: &str) -> InitialWork {
    InitialWork {
        rule_id: def(rule_id),
        scope: scope.clone(),
        due_time: LogicalTime(due),
        work_kind: work(work_kind),
    }
}

// ---------------------------------------------------------------- requests

pub fn advance(t: u64) -> Request {
    Request::Advance(LogicalTime(t))
}

pub fn command_request(
    id: &str,
    effective: u64,
    sequence: u64,
    command_kind: &str,
    subject: ScopeId,
    observations: Vec<(&str, i64)>,
    params: Vec<i64>,
) -> CommandRequest {
    CommandRequest {
        command_id: CommandId::new(id).unwrap(),
        profile_id: profile_id(),
        timeline_epoch: TimelineEpoch(0),
        effective_time: LogicalTime(effective),
        source_id: SourceId::new(SOURCE).unwrap(),
        source_sequence: sequence,
        command_kind: kind(command_kind),
        payload: CommandPayload::Host {
            subject,
            observations: observations
                .into_iter()
                .map(|(d, v)| Observation {
                    definition: def(d),
                    value: CanonicalValue::Int(v),
                })
                .collect(),
            params,
        },
    }
}

pub fn command(
    id: &str,
    effective: u64,
    sequence: u64,
    command_kind: &str,
    subject: ScopeId,
) -> Request {
    Request::Command(command_request(
        id,
        effective,
        sequence,
        command_kind,
        subject,
        vec![],
        vec![],
    ))
}

/// Presents one request repeatedly while it pauses; returns every result.
pub fn drive(engine: &mut Engine, request: &Request) -> Vec<ProcessResult> {
    let mut out = Vec::new();
    for _ in 0..10_000 {
        let r = engine.process(request);
        let paused = matches!(r.outcome(), Outcome::Paused);
        out.push(r);
        if !paused {
            break;
        }
    }
    out
}

/// The numeric value of a cell, if present.
pub fn value(engine: &Engine, definition: &str, scope: &ScopeId) -> Option<i64> {
    engine
        .state()
        .get(&profile_id(), &def(definition), scope)
        .and_then(|c| match &c.value {
            CanonicalValue::Int(i) => Some(*i),
            CanonicalValue::Fixed(f) => Some(f.raw()),
            _ => None,
        })
}

/// All canonical cohort reports across a sequence of results.
pub fn reports(results: &[ProcessResult]) -> Vec<spark_engine::report::CohortReport> {
    results.iter().flat_map(|r| r.reports().to_vec()).collect()
}

pub fn digest_pair(engine: &Engine) -> (Digest, Digest) {
    (
        engine.engine_state_digest(),
        engine.stable_boundary_digest(),
    )
}
