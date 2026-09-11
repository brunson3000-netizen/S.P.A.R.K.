//! The Phase-2 rule-set artifact (freeze v1 Q1, v2 §4, v3 §3.2(a)).
//!
//! A rule is **data**: a closed, non-recursive typed operation enum over named
//! inputs with exactly the blueprint §19.3 vocabulary — threshold/eligibility
//! comparisons, add/subtract/scale/clamp, weighted sum, piecewise curve,
//! probability gate, scope mapping, delay/cooldown, decay/recovery, and
//! aggregation/threshold emission. There is no scripting, no eval, and no
//! table-driven interpretation of uninterpreted tags.
//!
//! Rules ship as a separate content-addressed artifact: a [`RuleSetSpec`] goes
//! through the one door, [`crate::activation::ActivationRegistry::activate_rule_set`],
//! and comes out as an [`ActivatedRuleSet`] with a door-computed
//! `ruleset_content_hash` and per-rule fingerprints. `ProfileManifest` and
//! `ConfigRevision` encodings are untouched, so no Phase-1 digest moves.
//!
//! # Validation at the door (all rejecting, atomic, every error reported)
//!
//! - every referenced definition exists in the activated profile; every effect
//!   target is `spark_owned` or `derived` (a `host_owned` target is rejected —
//!   AT-I10) and numeric;
//! - every emitting operation — including every effect nested in a
//!   `Materialize` schedule — and every probability gate carries a stable
//!   dotted sub-ID unique within its rule and valid when qualified (v2 §3.2);
//! - clamp bounds coherent, curves strictly increasing, cadences, delays, and
//!   cooldowns at least one (delayed work is strictly later, v2 §11);
//! - declared budgets: `max_due_per_cycle >= 1` (v3 §3.2(a)); every semantic cap
//!   at least one; `max_wave_depth` may be zero;
//! - per-rule static fan-out — distinct target definitions × declared scope
//!   mapping breadth, plus each declared delay edge, and separately each
//!   materialized schedule's own expansion — within `max_fan_out`; no
//!   zero-delay cycle; the longest zero-delay propagation chain within
//!   `max_wave_depth` (v1 Q4);
//! - at most one aggregation rule and at most one decay/recovery rule (and one
//!   decay operation) per target definition (v2 §4.3); statically provable
//!   incompatible co-targeting between rules that share a trigger is rejected
//!   (v2 §4.2, AT-I37);
//! - a decay/recovery operation targets a definition whose baseline semantics
//!   the rule set declares (v1 Q7); its rate and cadence are rule parameters —
//!   a literal or a `ConfigRevision` key;
//! - no rule is triggered by the reserved epoch-activation command kind (v1 Q5).

use crate::activation::ActivatedProfile;
use spark_core::authority::Authority;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{CanonicalTag, DefinitionId, ProfileId};
use spark_core::scheduler::WorkKind;
use spark_core::scope::ScopeId;
use spark_core::timeline::CommandKind;
use spark_core::value::{FixedPoint, ValueType, FIXED_SCALE};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Epoch-bound declared budgets (v1 Q4, v2 §6.2, v3 §3.2). Changing one is a
/// behavior-epoch change, never an ambient runtime knob.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclaredBudgets {
    /// Pacing: executable `WorkKey`s admitted per `process` call. Must be `>= 1`.
    pub max_due_per_cycle: u32,
    /// Semantic cap: candidates in one wave of one cohort.
    pub max_cohort_candidates: u32,
    /// Semantic cap: committed effects in one wave.
    pub max_effects_per_wave: u32,
    /// Semantic cap: highest wave index a cohort may evaluate (cohort-scoped).
    pub max_wave_depth: u32,
    /// Semantic cap: obligations enqueued by one wave.
    pub max_enqueue_per_wave: u32,
    /// Queue admission cap: occupied obligation-store work identities.
    pub max_obligations: u32,
    /// Queue admission cap: occupied scheduler slots.
    pub max_scheduled_work: u32,
    /// Static per-rule fan-out bound.
    pub max_fan_out: u32,
}

impl DeclaredBudgets {
    /// The budgets that are **declared behavior**: every semantic cap and
    /// admission bound. `max_due_per_cycle` is deliberately absent. It is
    /// pacing, not semantics (v2 §6.2, v3 §3.2–§3.4): the frozen equivalence
    /// claims fix one activated artifact and permit different pacing budgets
    /// (FINAL §16 PE-D) while requiring bit-identical emission identities,
    /// batch digests, and engine digests between paced and unbudgeted runs. A
    /// pacing budget inside `ruleset_content_hash` would move
    /// `behavior_artifact_hash` and therefore every emission identity, making
    /// that claim false. It is still declared in the rule-set spec and
    /// validated at the door (`>= 1`, v3 §3.2(a)).
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str("declared_budgets_v1");
        for v in [
            self.max_cohort_candidates,
            self.max_effects_per_wave,
            self.max_wave_depth,
            self.max_enqueue_per_wave,
            self.max_obligations,
            self.max_scheduled_work,
            self.max_fan_out,
        ] {
            enc.push_u32(v);
        }
    }
}

/// Which scope an input or effect addresses.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScopeRef {
    /// The rule's subject scope: a command's declared subject, a scheduled
    /// work item's scope, or the changed target's scope for propagation.
    Subject,
    /// A fixed scope — the declared scope mapping (e.g. child → aggregate).
    Fixed(ScopeId),
}

impl ScopeRef {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            ScopeRef::Subject => {
                enc.push_str("scope.subject");
            }
            ScopeRef::Fixed(s) => {
                enc.push_str("scope.fixed");
                s.canonicalize(enc);
            }
        }
    }
}

/// A decay/recovery rule parameter (v1 Q7: "rule parameters, referencing
/// `ConfigRevision` keys where hot-tunable"): a literal, or a numeric config
/// key. A config parameter is read under the behavior epoch in effect over each
/// integrated interval, so a hot-tuned value applies only from its activating
/// barrier (AT-I23).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Param {
    Literal(i64),
    Config { key: DefinitionId },
}

impl Param {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            Param::Literal(v) => {
                enc.push_str("param.literal");
                enc.push_i64(*v);
            }
            Param::Config { key } => {
                enc.push_str("param.config");
                key.canonicalize(enc);
            }
        }
    }

    pub(crate) fn config_key(&self) -> Option<&DefinitionId> {
        match self {
            Param::Literal(_) => None,
            Param::Config { key } => Some(key),
        }
    }
}

/// A declared baseline for one definition (v1 Q7). The value is materialized
/// into the existing `StateCell.baseline` field of that definition's cells by
/// engine writes, and a decay/recovery operation moves toward the cell's
/// baseline. `DefinitionSpec`, `StateCell`, and every Phase-1 encoding are
/// unchanged; the declaration lives in the Phase-2 rule-set artifact and is
/// part of its content hash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineDeclaration {
    pub definition: DefinitionId,
    pub value: i64,
}

/// A named input read from the stable pre-wave snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    /// A committed cell's numeric value (`Int` value or `Fixed` raw); `absent`
    /// when the cell does not exist.
    Cell {
        definition: DefinitionId,
        scope: ScopeRef,
        absent: i64,
    },
    /// A numeric `ConfigRevision` entry of the current behavior epoch.
    Config {
        key: DefinitionId,
    },
    /// A numeric command parameter (command cohorts only; `0` elsewhere).
    Param {
        index: u8,
    },
    Literal(i64),
}

impl Input {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            Input::Cell {
                definition,
                scope,
                absent,
            } => {
                enc.push_str("input.cell");
                definition.canonicalize(enc);
                scope.canonicalize(enc);
                enc.push_i64(*absent);
            }
            Input::Config { key } => {
                enc.push_str("input.config");
                key.canonicalize(enc);
            }
            Input::Param { index } => {
                enc.push_str("input.param");
                enc.push_u32(u32::from(*index));
            }
            Input::Literal(v) => {
                enc.push_str("input.literal");
                enc.push_i64(*v);
            }
        }
    }
}

/// A bounded, non-recursive numeric expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Input(Input),
    /// `floor(Σ wᵢ·xᵢ / FIXED_SCALE)` in `i128` (v1 Q1 numeric model).
    WeightedSum(Vec<(FixedPoint, Input)>),
    /// Piecewise-linear curve over strictly increasing x; clamps to the end
    /// segments; floor rounding.
    Curve {
        input: Input,
        points: Vec<(i64, i64)>,
    },
}

impl Expr {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            Expr::Input(i) => {
                enc.push_str("expr.input");
                i.canonicalize(enc);
            }
            Expr::WeightedSum(terms) => {
                enc.push_str("expr.weighted_sum");
                enc.push_u64(terms.len() as u64);
                for (w, i) in terms {
                    enc.push_i64(w.raw());
                    i.canonicalize(enc);
                }
            }
            Expr::Curve { input, points } => {
                enc.push_str("expr.curve");
                input.canonicalize(enc);
                enc.push_u64(points.len() as u64);
                for (x, y) in points {
                    enc.push_i64(*x);
                    enc.push_i64(*y);
                }
            }
        }
    }

    fn inputs(&self) -> Vec<&Input> {
        match self {
            Expr::Input(i) => vec![i],
            Expr::WeightedSum(terms) => terms.iter().map(|(_, i)| i).collect(),
            Expr::Curve { input, .. } => vec![input],
        }
    }

    /// The RESULT family an `Assign` of this expression declares (v2 §4.1).
    pub(crate) fn result_family(&self) -> ResultFamily {
        match self {
            Expr::Input(_) => ResultFamily::Assignment,
            Expr::WeightedSum(_) => ResultFamily::WeightedSum,
            Expr::Curve { .. } => ResultFamily::Curve,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Lt,
    Le,
    Eq,
    Ne,
    Ge,
    Gt,
}

impl CmpOp {
    fn tag(&self) -> &'static str {
        match self {
            CmpOp::Lt => "lt",
            CmpOp::Le => "le",
            CmpOp::Eq => "eq",
            CmpOp::Ne => "ne",
            CmpOp::Ge => "ge",
            CmpOp::Gt => "gt",
        }
    }

    pub(crate) fn holds(&self, l: i64, r: i64) -> bool {
        match self {
            CmpOp::Lt => l < r,
            CmpOp::Le => l <= r,
            CmpOp::Eq => l == r,
            CmpOp::Ne => l != r,
            CmpOp::Ge => l >= r,
            CmpOp::Gt => l > r,
        }
    }
}

/// An eligibility condition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    Compare {
        left: Expr,
        op: CmpOp,
        right: Expr,
    },
    /// Probability gate: fires iff `derive_fixed_fraction(address) < rate`,
    /// with `rate` in `[0, FIXED_SCALE]` and the address's rule ID the gate's
    /// fully qualified `rule_id.sub_id`.
    Chance {
        sub_id: CanonicalTag,
        rate: FixedPoint,
    },
}

impl Condition {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            Condition::Compare { left, op, right } => {
                enc.push_str("condition.compare");
                left.canonicalize(enc);
                enc.push_str(op.tag());
                right.canonicalize(enc);
            }
            Condition::Chance { sub_id, rate } => {
                enc.push_str("condition.chance");
                sub_id.canonicalize(enc);
                enc.push_i64(rate.raw());
            }
        }
    }
}

/// The update an emitting operation applies to its target (v2 §4.1 families).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Update {
    /// ADDITIVE (`emit_delta`).
    Add(Expr),
    /// ADDITIVE, normalized to a signed delta with checked negation.
    Subtract(Expr),
    /// RESULT (`emit_result`); family from the expression's typed form.
    Assign(Expr),
    /// TRANSFORM: `floor(v · factor / FIXED_SCALE)`.
    Scale(FixedPoint),
    /// TRANSFORM: clamp into `[min, max]`.
    Clamp { min: i64, max: i64 },
    /// TRANSFORM: closed-form decay/recovery of the target cell toward its
    /// `StateCell.baseline` by `rate` per whole step of the operation's fixed
    /// `cadence` grid (v1 Q7 as amended by the Operator decay adjudication).
    /// The target definition's baseline semantics must be declared by the
    /// rule set; there is no substitute target.
    Decay { rate: Param, cadence: Param },
    /// RESULT (family `aggregate`): `floor(Σ over every cell of source ·
    /// weight / FIXED_SCALE)`, stable BTree order, no accumulator (v1 §7).
    Aggregate {
        source: DefinitionId,
        weight: FixedPoint,
    },
}

impl Update {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            Update::Add(e) => {
                enc.push_str("update.add");
                e.canonicalize(enc);
            }
            Update::Subtract(e) => {
                enc.push_str("update.subtract");
                e.canonicalize(enc);
            }
            Update::Assign(e) => {
                enc.push_str("update.assign");
                e.canonicalize(enc);
            }
            Update::Scale(f) => {
                enc.push_str("update.scale");
                enc.push_i64(f.raw());
            }
            Update::Clamp { min, max } => {
                enc.push_str("update.clamp");
                enc.push_i64(*min);
                enc.push_i64(*max);
            }
            Update::Decay { rate, cadence } => {
                enc.push_str("update.decay");
                rate.canonicalize(enc);
                cadence.canonicalize(enc);
            }
            Update::Aggregate { source, weight } => {
                enc.push_str("update.aggregate");
                source.canonicalize(enc);
                enc.push_i64(weight.raw());
            }
        }
    }

    fn inputs(&self) -> Vec<&Input> {
        match self {
            Update::Add(e) | Update::Subtract(e) | Update::Assign(e) => e.inputs(),
            Update::Decay { .. }
            | Update::Scale(_)
            | Update::Clamp { .. }
            | Update::Aggregate { .. } => Vec::new(),
        }
    }

    /// The static family of a single-operation contribution.
    pub(crate) fn family(&self) -> StaticFamily {
        match self {
            Update::Add(_) | Update::Subtract(_) => StaticFamily::Additive,
            Update::Assign(e) => StaticFamily::Result(e.result_family()),
            Update::Aggregate { .. } => StaticFamily::Result(ResultFamily::Aggregate),
            Update::Scale(_) => StaticFamily::Transform(TransformFamily::Scale),
            Update::Clamp { .. } => StaticFamily::Transform(TransformFamily::Clamp),
            Update::Decay { .. } => StaticFamily::Transform(TransformFamily::Decay),
        }
    }
}

/// RESULT families (v2 §4.1): equal values coalesce only within one family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResultFamily {
    Assignment,
    WeightedSum,
    Curve,
    Aggregate,
}

impl ResultFamily {
    pub fn tag(&self) -> &'static str {
        match self {
            ResultFamily::Assignment => "result.assignment",
            ResultFamily::WeightedSum => "result.weighted_sum",
            ResultFamily::Curve => "result.curve",
            ResultFamily::Aggregate => "result.aggregate",
        }
    }
}

/// TRANSFORM families (v2 §4.1): at most one per target per wave.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransformFamily {
    Scale,
    Clamp,
    Decay,
    /// Several operations of one rule on one target, composed in declared
    /// stage order: the rule body is the declared reducer (v2 §4.2).
    RuleBody,
}

impl TransformFamily {
    pub fn tag(&self) -> &'static str {
        match self {
            TransformFamily::Scale => "transform.scale",
            TransformFamily::Clamp => "transform.clamp",
            TransformFamily::Decay => "transform.decay",
            TransformFamily::RuleBody => "transform.rule_body",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StaticFamily {
    Additive,
    Result(ResultFamily),
    Transform(TransformFamily),
}

/// One effect-emitting operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitOp {
    pub sub_id: CanonicalTag,
    pub target: DefinitionId,
    pub scope: ScopeRef,
    pub update: Update,
}

impl EmitOp {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str("emit_op");
        self.sub_id.canonicalize(enc);
        self.target.canonicalize(enc);
        self.scope.canonicalize(enc);
        self.update.canonicalize(enc);
    }
}

/// How a delayed obligation executes (v1 §5; modes never switch).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleMode {
    /// Re-evaluate the exact named rule (bound by fingerprint and ruleset hash).
    ReEvaluate { rule: DefinitionId },
    /// Apply these effects, resolved now and frozen in the record. Only
    /// `Add`, `Subtract`, and `Assign` are admissible.
    Materialize { effects: Vec<EmitOp> },
}

/// A delayed-obligation operation: a declared delay edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleOp {
    pub sub_id: CanonicalTag,
    /// Strictly positive: `due_time = canonical_time + delay` (checked).
    pub delay: u64,
    pub work_kind: WorkKind,
    pub scope: ScopeRef,
    pub mode: ScheduleMode,
}

impl ScheduleOp {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str("schedule_op");
        self.sub_id.canonicalize(enc);
        enc.push_u64(self.delay);
        self.work_kind.canonicalize(enc);
        self.scope.canonicalize(enc);
        match &self.mode {
            ScheduleMode::ReEvaluate { rule } => {
                enc.push_str("mode.reevaluate");
                rule.canonicalize(enc);
            }
            ScheduleMode::Materialize { effects } => {
                enc.push_str("mode.materialize");
                enc.push_u64(effects.len() as u64);
                for e in effects {
                    e.canonicalize(enc);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Rising,
    Falling,
    Both,
}

impl Direction {
    fn tag(&self) -> &'static str {
        match self {
            Direction::Rising => "rising",
            Direction::Falling => "falling",
            Direction::Both => "both",
        }
    }

    /// Whether `old -> new` crosses `threshold` in this direction.
    pub(crate) fn crossed(&self, old: i64, new: i64, threshold: i64) -> bool {
        let rising = old < threshold && new >= threshold;
        let falling = old >= threshold && new < threshold;
        match self {
            Direction::Rising => rising,
            Direction::Falling => falling,
            Direction::Both => rising || falling,
        }
    }
}

/// What makes a rule evaluate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trigger {
    /// Wave 0 of a command cohort of this kind, at the command's subject.
    Command(CommandKind),
    /// A re-evaluation obligation naming this rule (scheduled work).
    Work(WorkKind),
    /// Zero-delay propagation: wave `N+1` for each scope whose `watched` cell
    /// committed an effect in wave `N`.
    Change { watched: DefinitionId },
    /// Threshold emission: wave `N+1` when `watched`'s previously committed
    /// value → fully reduced result crosses `threshold` (v2 §5.1 step 4).
    Crossing {
        watched: DefinitionId,
        threshold: i64,
        direction: Direction,
    },
}

impl Trigger {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            Trigger::Command(k) => {
                enc.push_str("trigger.command");
                k.canonicalize(enc);
            }
            Trigger::Work(k) => {
                enc.push_str("trigger.work");
                k.canonicalize(enc);
            }
            Trigger::Change { watched } => {
                enc.push_str("trigger.change");
                watched.canonicalize(enc);
            }
            Trigger::Crossing {
                watched,
                threshold,
                direction,
            } => {
                enc.push_str("trigger.crossing");
                watched.canonicalize(enc);
                enc.push_i64(*threshold);
                enc.push_str(direction.tag());
            }
        }
    }

    pub(crate) fn watched(&self) -> Option<&DefinitionId> {
        match self {
            Trigger::Change { watched } | Trigger::Crossing { watched, .. } => Some(watched),
            _ => None,
        }
    }
}

/// One rule, as authored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleSpec {
    pub rule_id: DefinitionId,
    pub trigger: Trigger,
    pub conditions: Vec<Condition>,
    pub emits: Vec<EmitOp>,
    pub schedules: Vec<ScheduleOp>,
    /// Cooldown after firing, in logical time units (`>= 1`), keyed by
    /// `(rule, subject scope)` in the canonical `CooldownLedger`.
    pub cooldown: Option<u64>,
}

impl RuleSpec {
    /// The rule's content fingerprint (within its ruleset lineage).
    pub fn fingerprint(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("rule_fingerprint_v1");
        self.rule_id.canonicalize(&mut enc);
        self.trigger.canonicalize(&mut enc);
        enc.push_u64(self.conditions.len() as u64);
        for c in &self.conditions {
            c.canonicalize(&mut enc);
        }
        enc.push_u64(self.emits.len() as u64);
        for e in &self.emits {
            e.canonicalize(&mut enc);
        }
        enc.push_u64(self.schedules.len() as u64);
        for s in &self.schedules {
            s.canonicalize(&mut enc);
        }
        match self.cooldown {
            Some(c) => {
                enc.push_bool(true);
                enc.push_u64(c);
            }
            None => {
                enc.push_bool(false);
            }
        }
        enc.finish()
    }

    fn all_inputs(&self) -> Vec<&Input> {
        let mut v: Vec<&Input> = Vec::new();
        for c in &self.conditions {
            if let Condition::Compare { left, right, .. } = c {
                v.extend(left.inputs());
                v.extend(right.inputs());
            }
        }
        for e in &self.emits {
            v.extend(e.update.inputs());
        }
        for s in &self.schedules {
            if let ScheduleMode::Materialize { effects } = &s.mode {
                for e in effects {
                    v.extend(e.update.inputs());
                }
            }
        }
        v
    }

    /// Every committed cell this rule reads **to become eligible** — its
    /// condition reads — as declared `(definition, scope mapping)` pairs.
    /// With the watched trigger group, these are the reduction groups whose
    /// candidates form a derived emission's parent set (v3 §4.3).
    pub(crate) fn condition_cell_reads(&self) -> Vec<(&DefinitionId, &ScopeRef)> {
        let mut reads = Vec::new();
        for c in &self.conditions {
            if let Condition::Compare { left, right, .. } = c {
                for i in left.inputs().into_iter().chain(right.inputs()) {
                    if let Input::Cell {
                        definition, scope, ..
                    } = i
                    {
                        reads.push((definition, scope));
                    }
                }
            }
        }
        reads
    }

    /// Groups of this rule's emitting operations by `(target, scope)`, in
    /// first-declaration order: each group yields at most one candidate.
    pub(crate) fn target_groups(&self) -> Vec<((DefinitionId, ScopeRef), Vec<&EmitOp>)> {
        let mut groups: Vec<((DefinitionId, ScopeRef), Vec<&EmitOp>)> = Vec::new();
        for e in &self.emits {
            let k = (e.target.clone(), e.scope.clone());
            if let Some((_, ops)) = groups.iter_mut().find(|(gk, _)| gk == &k) {
                ops.push(e);
            } else {
                groups.push((k, vec![e]));
            }
        }
        groups
    }
}

/// An authored rule set for one profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleSetSpec {
    pub profile_id: ProfileId,
    pub budgets: DeclaredBudgets,
    pub rules: Vec<RuleSpec>,
    /// Declared baseline semantics (v1 Q7), at most one per definition.
    pub baselines: Vec<BaselineDeclaration>,
}

/// One rule inside an activated rule set: the spec plus its door-computed
/// fingerprint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedRule {
    spec: RuleSpec,
    fingerprint: Digest,
}

impl ActivatedRule {
    pub fn spec(&self) -> &RuleSpec {
        &self.spec
    }

    pub fn rule_id(&self) -> &DefinitionId {
        &self.spec.rule_id
    }

    pub fn fingerprint(&self) -> &Digest {
        &self.fingerprint
    }
}

/// The unforgeable activated rule-set artifact. Private fields; minted only by
/// the activation door.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedRuleSet {
    profile_id: ProfileId,
    manifest_content_hash: Digest,
    activation_hash: Digest,
    budgets: DeclaredBudgets,
    rules: BTreeMap<DefinitionId, ActivatedRule>,
    baselines: BTreeMap<DefinitionId, i64>,
    content_hash: Digest,
}

impl ActivatedRuleSet {
    /// The declared baseline of `definition`, if the rule set gives it
    /// baseline semantics (v1 Q7).
    pub fn baseline(&self, definition: &DefinitionId) -> Option<i64> {
        self.baselines.get(definition).copied()
    }

    /// Config keys read as decay rates and as decay cadences; activation
    /// checks that an epoch's config gives each a valid value (rate `>= 0`,
    /// cadence `>= 1`).
    pub(crate) fn decay_parameter_keys(&self) -> (BTreeSet<DefinitionId>, BTreeSet<DefinitionId>) {
        let mut rates = BTreeSet::new();
        let mut cadences = BTreeSet::new();
        for r in self.rules.values() {
            for e in &r.spec.emits {
                if let Update::Decay { rate, cadence } = &e.update {
                    rates.extend(rate.config_key().cloned());
                    cadences.extend(cadence.config_key().cloned());
                }
            }
        }
        (rates, cadences)
    }

    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    pub fn manifest_content_hash(&self) -> &Digest {
        &self.manifest_content_hash
    }

    pub fn activation_hash(&self) -> &Digest {
        &self.activation_hash
    }

    pub fn budgets(&self) -> &DeclaredBudgets {
        &self.budgets
    }

    /// `ruleset_content_hash`: a multiset function of the rules (declaration
    /// order cannot change it — AT-I3).
    pub fn content_hash(&self) -> &Digest {
        &self.content_hash
    }

    pub fn rule(&self, rule_id: &DefinitionId) -> Option<&ActivatedRule> {
        self.rules.get(rule_id)
    }

    /// Every rule in ascending rule-ID order.
    pub fn rules(&self) -> impl Iterator<Item = &ActivatedRule> {
        self.rules.values()
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub(crate) fn rules_triggered_by_command(&self, kind: &CommandKind) -> Vec<&ActivatedRule> {
        self.rules
            .values()
            .filter(|r| matches!(&r.spec.trigger, Trigger::Command(k) if k == kind))
            .collect()
    }

    pub(crate) fn rules_watching(&self, definition: &DefinitionId) -> Vec<&ActivatedRule> {
        self.rules
            .values()
            .filter(|r| r.spec.trigger.watched() == Some(definition))
            .collect()
    }

    /// Config keys any rule reads; validated against the epoch's config.
    pub(crate) fn config_keys(&self) -> BTreeSet<DefinitionId> {
        let mut keys = BTreeSet::new();
        for r in self.rules.values() {
            for i in r.spec.all_inputs() {
                if let Input::Config { key } = i {
                    keys.insert(key.clone());
                }
            }
        }
        let (rates, cadences) = self.decay_parameter_keys();
        keys.extend(rates);
        keys.extend(cadences);
        keys
    }
}

/// Rejects a rule set at the activation door.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleSetError {
    ProfileMismatch {
        profile_id: ProfileId,
        rule_set_profile_id: ProfileId,
    },
    /// The activated profile was not minted by this registry's lineage.
    ProfileNotActivatedHere {
        profile_id: ProfileId,
    },
    DuplicateRuleId {
        rule_id: DefinitionId,
    },
    UnknownDefinition {
        rule_id: DefinitionId,
        definition_id: DefinitionId,
    },
    HostOwnedTarget {
        rule_id: DefinitionId,
        target: DefinitionId,
    },
    NonNumericDefinition {
        rule_id: DefinitionId,
        definition_id: DefinitionId,
    },
    DisallowedFixedScope {
        rule_id: DefinitionId,
        target: DefinitionId,
    },
    DuplicateSubId {
        rule_id: DefinitionId,
        sub_id: CanonicalTag,
    },
    QualifiedIdInvalid {
        rule_id: DefinitionId,
        sub_id: CanonicalTag,
    },
    IncoherentClamp {
        rule_id: DefinitionId,
    },
    InvalidCurve {
        rule_id: DefinitionId,
    },
    ZeroCadence {
        rule_id: DefinitionId,
    },
    ZeroDelay {
        rule_id: DefinitionId,
    },
    ZeroCooldown {
        rule_id: DefinitionId,
    },
    ChanceRateOutOfRange {
        rule_id: DefinitionId,
    },
    InvalidMaterializedEffect {
        rule_id: DefinitionId,
    },
    UnknownReEvaluationRule {
        rule_id: DefinitionId,
        named: DefinitionId,
    },
    /// v3 §3.2(a): `max_due_per_cycle` must be at least one.
    ZeroPacingBudget,
    ZeroSemanticCap {
        cap: &'static str,
    },
    FanOutExceeded {
        rule_id: DefinitionId,
        fan_out: u64,
        bound: u32,
    },
    /// A zero-delay propagation cycle (named, ascending rule IDs).
    ZeroDelayCycle {
        rules: Vec<DefinitionId>,
    },
    WaveDepthExceeded {
        depth: u64,
        bound: u32,
    },
    SecondAggregationReducer {
        target: DefinitionId,
    },
    SecondDecayReducer {
        target: DefinitionId,
    },
    /// Statically provable incompatible co-targeting between rules that share
    /// a trigger (v2 §4.2 "at rule-set activation when statically provable").
    StaticFamilyMixture {
        target: DefinitionId,
        rules: Vec<DefinitionId>,
    },
    /// A materialized schedule whose frozen effects expand beyond the
    /// per-rule fan-out bound when it executes (v1 Q4, C2-02).
    MaterializedFanOutExceeded {
        rule_id: DefinitionId,
        sub_id: CanonicalTag,
        fan_out: u64,
        bound: u32,
    },
    /// v1 Q5: only the reserved epoch-activation command activates an epoch;
    /// no rule may be triggered by that kind.
    ReservedCommandKind {
        rule_id: DefinitionId,
    },
    /// v1 Q7: a decay/recovery operation on a definition without declared
    /// baseline semantics.
    DecayWithoutBaseline {
        rule_id: DefinitionId,
        target: DefinitionId,
    },
    /// A negative literal decay rate.
    InvalidDecayParameter {
        rule_id: DefinitionId,
    },
    /// A baseline declaration for an unknown, non-numeric, host-owned, or
    /// out-of-bounds definition.
    BaselineInvalid {
        definition: DefinitionId,
    },
    DuplicateBaseline {
        definition: DefinitionId,
    },
}

impl fmt::Display for RuleSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rule set rejected at the activation door: {self:?}")
    }
}

impl std::error::Error for RuleSetError {}

fn is_numeric(t: ValueType) -> bool {
    matches!(t, ValueType::Int | ValueType::Fixed)
}

/// The rule-set door's validation. Pure: returns the minted artifact or every
/// error found. The registry commits lineage only on success.
pub(crate) fn validate_and_mint(
    profile: &ActivatedProfile,
    spec: &RuleSetSpec,
    check_depth: bool,
) -> Result<ActivatedRuleSet, Vec<RuleSetError>> {
    let mut errors: Vec<RuleSetError> = Vec::new();
    if &spec.profile_id != profile.profile_id() {
        errors.push(RuleSetError::ProfileMismatch {
            profile_id: profile.profile_id().clone(),
            rule_set_profile_id: spec.profile_id.clone(),
        });
    }
    let b = &spec.budgets;
    if b.max_due_per_cycle == 0 {
        errors.push(RuleSetError::ZeroPacingBudget);
    }
    for (cap, v) in [
        ("max_cohort_candidates", b.max_cohort_candidates),
        ("max_effects_per_wave", b.max_effects_per_wave),
        ("max_enqueue_per_wave", b.max_enqueue_per_wave),
        ("max_obligations", b.max_obligations),
        ("max_scheduled_work", b.max_scheduled_work),
        ("max_fan_out", b.max_fan_out),
    ] {
        if v == 0 {
            errors.push(RuleSetError::ZeroSemanticCap { cap });
        }
    }

    let mut baselines: BTreeMap<DefinitionId, i64> = BTreeMap::new();
    for decl in &spec.baselines {
        let valid = profile.definition(&decl.definition).is_some_and(|d| {
            d.authority() != Authority::HostOwned
                && is_numeric(d.value_constraint().value_type())
                && d.value_constraint().accepts(&numeric_value(
                    d.value_constraint().value_type(),
                    decl.value,
                ))
        });
        if !valid {
            errors.push(RuleSetError::BaselineInvalid {
                definition: decl.definition.clone(),
            });
        }
        if baselines
            .insert(decl.definition.clone(), decl.value)
            .is_some()
        {
            errors.push(RuleSetError::DuplicateBaseline {
                definition: decl.definition.clone(),
            });
        }
    }
    let mut rules: BTreeMap<DefinitionId, ActivatedRule> = BTreeMap::new();
    for rule in &spec.rules {
        if rules.contains_key(&rule.rule_id) {
            errors.push(RuleSetError::DuplicateRuleId {
                rule_id: rule.rule_id.clone(),
            });
            continue;
        }
        rules.insert(
            rule.rule_id.clone(),
            ActivatedRule {
                spec: rule.clone(),
                fingerprint: rule.fingerprint(),
            },
        );
    }
    for rule in rules.values() {
        validate_rule(profile, &rules, &baselines, &rule.spec, b, &mut errors);
    }
    validate_reducers(&rules, &mut errors);
    validate_static_mixtures(&rules, &mut errors);
    validate_graph(&rules, b, check_depth, &mut errors);

    if !errors.is_empty() {
        return Err(errors);
    }
    let content_hash = ruleset_content_hash(profile, b, &rules, &baselines);
    Ok(ActivatedRuleSet {
        profile_id: profile.profile_id().clone(),
        manifest_content_hash: profile.manifest_content_hash().clone(),
        activation_hash: profile.activation_hash().clone(),
        budgets: *b,
        rules,
        baselines,
        content_hash,
    })
}

fn numeric_value(t: ValueType, raw: i64) -> spark_core::value::CanonicalValue {
    match t {
        ValueType::Fixed => spark_core::value::CanonicalValue::Fixed(FixedPoint::from_raw(raw)),
        _ => spark_core::value::CanonicalValue::Int(raw),
    }
}

fn ruleset_content_hash(
    profile: &ActivatedProfile,
    budgets: &DeclaredBudgets,
    rules: &BTreeMap<DefinitionId, ActivatedRule>,
    baselines: &BTreeMap<DefinitionId, i64>,
) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("ruleset_content_v1");
    profile.profile_id().canonicalize(&mut enc);
    enc.push_digest(profile.manifest_content_hash());
    enc.push_digest(profile.activation_hash());
    budgets.canonicalize(&mut enc);
    enc.push_u64(rules.len() as u64);
    for r in rules.values() {
        let mut inner = CanonicalEncoder::new();
        r.spec.rule_id.canonicalize(&mut inner);
        inner.push_digest(&r.fingerprint);
        enc.push_block(&inner);
    }
    enc.push_str("baselines");
    enc.push_u64(baselines.len() as u64);
    for (d, v) in baselines {
        d.canonicalize(&mut enc);
        enc.push_i64(*v);
    }
    enc.finish()
}

fn check_definition(
    profile: &ActivatedProfile,
    rule_id: &DefinitionId,
    definition: &DefinitionId,
    numeric: bool,
    errors: &mut Vec<RuleSetError>,
) -> bool {
    match profile.definition(definition) {
        None => {
            errors.push(RuleSetError::UnknownDefinition {
                rule_id: rule_id.clone(),
                definition_id: definition.clone(),
            });
            false
        }
        Some(d) => {
            if numeric && !is_numeric(d.value_constraint().value_type()) {
                errors.push(RuleSetError::NonNumericDefinition {
                    rule_id: rule_id.clone(),
                    definition_id: definition.clone(),
                });
                return false;
            }
            true
        }
    }
}

fn check_input(
    profile: &ActivatedProfile,
    rule_id: &DefinitionId,
    input: &Input,
    errors: &mut Vec<RuleSetError>,
) {
    if let Input::Cell { definition, .. } = input {
        check_definition(profile, rule_id, definition, true, errors);
    }
}

fn check_expr(
    profile: &ActivatedProfile,
    rule_id: &DefinitionId,
    expr: &Expr,
    errors: &mut Vec<RuleSetError>,
) {
    for i in expr.inputs() {
        check_input(profile, rule_id, i, errors);
    }
    if let Expr::Curve { points, .. } = expr {
        let increasing = points.windows(2).all(|w| match w {
            [(x0, _), (x1, _)] => x0 < x1,
            _ => true,
        });
        if points.is_empty() || !increasing {
            errors.push(RuleSetError::InvalidCurve {
                rule_id: rule_id.clone(),
            });
        }
    }
}

fn check_emit(
    profile: &ActivatedProfile,
    baselines: &BTreeMap<DefinitionId, i64>,
    rule_id: &DefinitionId,
    e: &EmitOp,
    materialized: bool,
    errors: &mut Vec<RuleSetError>,
) {
    if check_definition(profile, rule_id, &e.target, true, errors) {
        if let Some(d) = profile.definition(&e.target) {
            if d.authority() == Authority::HostOwned {
                errors.push(RuleSetError::HostOwnedTarget {
                    rule_id: rule_id.clone(),
                    target: e.target.clone(),
                });
            }
            if let ScopeRef::Fixed(s) = &e.scope {
                if !d.valid_scopes().contains(s.kind()) {
                    errors.push(RuleSetError::DisallowedFixedScope {
                        rule_id: rule_id.clone(),
                        target: e.target.clone(),
                    });
                }
            }
        }
    }
    if materialized
        && !matches!(
            e.update,
            Update::Add(_) | Update::Subtract(_) | Update::Assign(_)
        )
    {
        errors.push(RuleSetError::InvalidMaterializedEffect {
            rule_id: rule_id.clone(),
        });
    }
    match &e.update {
        Update::Add(x) | Update::Subtract(x) | Update::Assign(x) => {
            check_expr(profile, rule_id, x, errors)
        }
        Update::Scale(_) => {}
        Update::Clamp { min, max } => {
            if min > max {
                errors.push(RuleSetError::IncoherentClamp {
                    rule_id: rule_id.clone(),
                });
            }
        }
        Update::Decay { rate, cadence } => {
            if matches!(rate, Param::Literal(r) if *r < 0) {
                errors.push(RuleSetError::InvalidDecayParameter {
                    rule_id: rule_id.clone(),
                });
            }
            if matches!(cadence, Param::Literal(c) if *c < 1) {
                errors.push(RuleSetError::ZeroCadence {
                    rule_id: rule_id.clone(),
                });
            }
            if !baselines.contains_key(&e.target) {
                errors.push(RuleSetError::DecayWithoutBaseline {
                    rule_id: rule_id.clone(),
                    target: e.target.clone(),
                });
            }
        }
        Update::Aggregate { source, .. } => {
            check_definition(profile, rule_id, source, true, errors);
        }
    }
}

fn validate_rule(
    profile: &ActivatedProfile,
    all: &BTreeMap<DefinitionId, ActivatedRule>,
    baselines: &BTreeMap<DefinitionId, i64>,
    rule: &RuleSpec,
    budgets: &DeclaredBudgets,
    errors: &mut Vec<RuleSetError>,
) {
    let rule_id = &rule.rule_id;
    if let Some(w) = rule.trigger.watched() {
        check_definition(profile, rule_id, w, true, errors);
    }
    if let Trigger::Command(k) = &rule.trigger {
        if k.as_str() == crate::engine::EPOCH_ACTIVATION_COMMAND_KIND.as_str() {
            errors.push(RuleSetError::ReservedCommandKind {
                rule_id: rule_id.clone(),
            });
        }
    }
    let mut sub_ids: BTreeSet<CanonicalTag> = BTreeSet::new();
    let mut claim = |sub: &CanonicalTag, errors: &mut Vec<RuleSetError>| {
        if !sub_ids.insert(sub.clone()) {
            errors.push(RuleSetError::DuplicateSubId {
                rule_id: rule_id.clone(),
                sub_id: sub.clone(),
            });
        }
        if DefinitionId::new(format!("{}.{}", rule_id.as_str(), sub.as_str())).is_err() {
            errors.push(RuleSetError::QualifiedIdInvalid {
                rule_id: rule_id.clone(),
                sub_id: sub.clone(),
            });
        }
    };
    for c in &rule.conditions {
        match c {
            Condition::Compare { left, right, .. } => {
                check_expr(profile, rule_id, left, errors);
                check_expr(profile, rule_id, right, errors);
            }
            Condition::Chance { sub_id, rate } => {
                claim(sub_id, errors);
                if rate.raw() < 0 || rate.raw() > FIXED_SCALE {
                    errors.push(RuleSetError::ChanceRateOutOfRange {
                        rule_id: rule_id.clone(),
                    });
                }
            }
        }
    }
    for e in &rule.emits {
        claim(&e.sub_id, errors);
        check_emit(profile, baselines, rule_id, e, false, errors);
    }
    for s in &rule.schedules {
        claim(&s.sub_id, errors);
        if s.delay == 0 {
            errors.push(RuleSetError::ZeroDelay {
                rule_id: rule_id.clone(),
            });
        }
        match &s.mode {
            ScheduleMode::ReEvaluate { rule: named } => {
                if !all.contains_key(named) {
                    errors.push(RuleSetError::UnknownReEvaluationRule {
                        rule_id: rule_id.clone(),
                        named: named.clone(),
                    });
                }
            }
            ScheduleMode::Materialize { effects } => {
                if effects.is_empty() {
                    errors.push(RuleSetError::InvalidMaterializedEffect {
                        rule_id: rule_id.clone(),
                    });
                }
                // Every nested emitting operation claims its sub-ID in the
                // rule's one namespace (v2 §3.2, AT-I17): at execution the
                // sub-ID is an emission-identity input, so two same-target
                // effects sharing one would collide (unequal payloads) or
                // fold into one (equal payloads).
                for e in effects {
                    claim(&e.sub_id, errors);
                    check_emit(profile, baselines, rule_id, e, true, errors);
                }
                // The materialized expansion executes as one later seed; its
                // declared scope breadth is bounded like the rule's own.
                let expansion: BTreeSet<(&DefinitionId, &ScopeRef)> =
                    effects.iter().map(|e| (&e.target, &e.scope)).collect();
                let expansion = expansion.len() as u64;
                if expansion > u64::from(budgets.max_fan_out) {
                    errors.push(RuleSetError::MaterializedFanOutExceeded {
                        rule_id: rule_id.clone(),
                        sub_id: s.sub_id.clone(),
                        fan_out: expansion,
                        bound: budgets.max_fan_out,
                    });
                }
            }
        }
    }
    if rule.cooldown == Some(0) {
        errors.push(RuleSetError::ZeroCooldown {
            rule_id: rule_id.clone(),
        });
    }
    // v1 Q4: distinct target definitions × declared scope-mapping breadth,
    // i.e. every distinct declared (target, scope mapping) pair, plus one per
    // declared delay edge.
    let targets: BTreeSet<(&DefinitionId, &ScopeRef)> =
        rule.emits.iter().map(|e| (&e.target, &e.scope)).collect();
    let fan_out = (targets.len() as u64).saturating_add(rule.schedules.len() as u64);
    if fan_out > u64::from(budgets.max_fan_out) {
        errors.push(RuleSetError::FanOutExceeded {
            rule_id: rule_id.clone(),
            fan_out,
            bound: budgets.max_fan_out,
        });
    }
    // One decay operation per target inside a rule body, too (v2 §4.3).
    let mut decayed: BTreeSet<&DefinitionId> = BTreeSet::new();
    for e in &rule.emits {
        if matches!(e.update, Update::Decay { .. }) && !decayed.insert(&e.target) {
            errors.push(RuleSetError::SecondDecayReducer {
                target: e.target.clone(),
            });
        }
    }
}

fn validate_reducers(
    rules: &BTreeMap<DefinitionId, ActivatedRule>,
    errors: &mut Vec<RuleSetError>,
) {
    let mut aggregation: BTreeMap<&DefinitionId, BTreeSet<&DefinitionId>> = BTreeMap::new();
    let mut decay: BTreeMap<&DefinitionId, BTreeSet<&DefinitionId>> = BTreeMap::new();
    for r in rules.values() {
        for e in &r.spec.emits {
            match e.update {
                Update::Aggregate { .. } => {
                    aggregation
                        .entry(&e.target)
                        .or_default()
                        .insert(&r.spec.rule_id);
                }
                Update::Decay { .. } => {
                    decay.entry(&e.target).or_default().insert(&r.spec.rule_id);
                }
                _ => {}
            }
        }
    }
    for (target, owners) in aggregation {
        if owners.len() > 1 {
            errors.push(RuleSetError::SecondAggregationReducer {
                target: target.clone(),
            });
        }
    }
    for (target, owners) in decay {
        if owners.len() > 1 {
            errors.push(RuleSetError::SecondDecayReducer {
                target: target.clone(),
            });
        }
    }
}

/// The static family of one rule's contribution to one `(target, scope)`.
fn group_family(ops: &[&EmitOp]) -> Option<StaticFamily> {
    match ops {
        [only] => Some(only.update.family()),
        [] => None,
        _ => {
            if ops
                .iter()
                .all(|o| o.update.family() == StaticFamily::Additive)
            {
                Some(StaticFamily::Additive)
            } else {
                Some(StaticFamily::Transform(TransformFamily::RuleBody))
            }
        }
    }
}

fn compatible(a: StaticFamily, b: StaticFamily) -> bool {
    match (a, b) {
        (StaticFamily::Additive, StaticFamily::Additive) => true,
        (StaticFamily::Result(x), StaticFamily::Result(y)) => x == y,
        _ => false,
    }
}

fn validate_static_mixtures(
    rules: &BTreeMap<DefinitionId, ActivatedRule>,
    errors: &mut Vec<RuleSetError>,
) {
    // Rules sharing an identical trigger certainly co-fire at one subject; a
    // target group they share is statically provable co-targeting.
    let list: Vec<&ActivatedRule> = rules.values().collect();
    let mut reported: BTreeSet<(DefinitionId, Vec<DefinitionId>)> = BTreeSet::new();
    for (i, a) in list.iter().enumerate() {
        for b in list.iter().skip(i.saturating_add(1)) {
            if a.spec.trigger != b.spec.trigger {
                continue;
            }
            for (ka, opsa) in a.spec.target_groups() {
                for (kb, opsb) in b.spec.target_groups() {
                    if ka != kb {
                        continue;
                    }
                    if let (Some(fa), Some(fb)) = (group_family(&opsa), group_family(&opsb)) {
                        if !compatible(fa, fb) {
                            let names = vec![a.spec.rule_id.clone(), b.spec.rule_id.clone()];
                            if reported.insert((ka.0.clone(), names.clone())) {
                                errors.push(RuleSetError::StaticFamilyMixture {
                                    target: ka.0.clone(),
                                    rules: names,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Zero-delay propagation graph: `a -> b` iff `b` watches a definition `a`
/// emits to. Delay edges (schedule operations) are not zero-delay edges.
fn zero_delay_edges(
    rules: &BTreeMap<DefinitionId, ActivatedRule>,
) -> BTreeMap<DefinitionId, BTreeSet<DefinitionId>> {
    let mut edges: BTreeMap<DefinitionId, BTreeSet<DefinitionId>> = BTreeMap::new();
    for a in rules.values() {
        let written: BTreeSet<&DefinitionId> = a.spec.emits.iter().map(|e| &e.target).collect();
        let entry = edges.entry(a.spec.rule_id.clone()).or_default();
        for b in rules.values() {
            if let Some(w) = b.spec.trigger.watched() {
                if written.contains(w) {
                    entry.insert(b.spec.rule_id.clone());
                }
            }
        }
    }
    edges
}

fn validate_graph(
    rules: &BTreeMap<DefinitionId, ActivatedRule>,
    budgets: &DeclaredBudgets,
    check_depth: bool,
    errors: &mut Vec<RuleSetError>,
) {
    let edges = zero_delay_edges(rules);
    // Iterative three-colour DFS; the first cycle found per start is named.
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Colour {
        White,
        Grey,
        Black,
    }
    let mut colour: BTreeMap<&DefinitionId, Colour> =
        edges.keys().map(|k| (k, Colour::White)).collect();
    let mut cycles: BTreeSet<Vec<DefinitionId>> = BTreeSet::new();
    for start in edges.keys() {
        if colour.get(start) != Some(&Colour::White) {
            continue;
        }
        let mut stack: Vec<(&DefinitionId, Vec<&DefinitionId>)> = Vec::new();
        let mut path: Vec<&DefinitionId> = Vec::new();
        colour.insert(start, Colour::Grey);
        path.push(start);
        stack.push((
            start,
            edges
                .get(start)
                .map(|s| s.iter().collect())
                .unwrap_or_default(),
        ));
        while let Some((node, pending)) = stack.last_mut() {
            let node = *node;
            match pending.pop() {
                Some(next) => match colour.get(next).copied() {
                    Some(Colour::White) => {
                        colour.insert(next, Colour::Grey);
                        path.push(next);
                        stack.push((
                            next,
                            edges
                                .get(next)
                                .map(|s| s.iter().collect())
                                .unwrap_or_default(),
                        ));
                    }
                    Some(Colour::Grey) => {
                        let from = path.iter().position(|p| *p == next).unwrap_or(0);
                        let mut members: Vec<DefinitionId> =
                            path.iter().skip(from).map(|p| (*p).clone()).collect();
                        members.sort();
                        cycles.insert(members);
                    }
                    _ => {}
                },
                None => {
                    colour.insert(node, Colour::Black);
                    path.pop();
                    stack.pop();
                }
            }
        }
    }
    let acyclic = cycles.is_empty();
    for rules in cycles {
        errors.push(RuleSetError::ZeroDelayCycle { rules });
    }
    if acyclic {
        // Longest path (in edges) by memoized depth over the DAG.
        let mut depth: BTreeMap<&DefinitionId, u64> = BTreeMap::new();
        let mut order: Vec<&DefinitionId> = edges.keys().collect();
        // Repeated relaxation bounded by node count keeps this panic-free and
        // allocation-light for the small graphs rule sets are.
        for _ in 0..order.len() {
            let mut changed = false;
            for node in &order {
                let d = edges
                    .get(*node)
                    .map(|succ| {
                        succ.iter()
                            .map(|s| depth.get(s).copied().unwrap_or(0).saturating_add(1))
                            .max()
                            .unwrap_or(0)
                    })
                    .unwrap_or(0);
                if depth.get(*node).copied() != Some(d) {
                    depth.insert(node, d);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        order.clear();
        let longest = depth.values().copied().max().unwrap_or(0);
        if check_depth && longest > u64::from(budgets.max_wave_depth) {
            errors.push(RuleSetError::WaveDepthExceeded {
                depth: longest,
                bound: budgets.max_wave_depth,
            });
        }
    }
}
