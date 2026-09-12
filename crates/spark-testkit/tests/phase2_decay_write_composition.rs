//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 **C2W-01**: additive settlement across composed rule bodies.
//!
//! The controlling independent review
//! (`SPARK_PHASE_2_CODEX_GATE_C2_DECAY_WRITE_RESOLUTION_INDEPENDENT_REVIEW_2026-09-12.md`,
//! `e39690dc51b63fa09fe7c3f30ebd48aa2c16686f`) found that a body performing an
//! additive change and finishing with a transform stage forfeited its overdue
//! decay or recovery, violating approved behavior 1. The corrected rule, recorded
//! in `SPARK_PHASE_2_GATE_C2_W01_CONTRACT_CORRECTION_2026-09-12.md`, is:
//!
//! > A composed rule body that performs an additive change and declares no
//! > debt-consuming `Decay` stage is an additive event. Its **starting value** is
//! > settled to the cohort's canonical time; its declared stages then fold in
//! > declared order from that value.
//!
//! What that rule must *not* disturb is pinned here too: declared stage order in
//! bodies that do declare a decay stage, explicit replacements anywhere in a
//! body, the retained standalone `Scale`/`Clamp` boundary, single charging,
//! canonical commit times, atomic refusal, and committed-to-committed watchers.
//!
//! Every vector is hand-valued and runs through the public door. Recovery
//! vectors are written out separately rather than negated, because `Scale`
//! floors toward negative infinity and is not symmetric. Each test names the
//! wrong implementation it kills.

use spark_core::clock::LogicalTime;
use spark_core::scope::ScopeId;
use spark_core::value::{CanonicalValue, FixedPoint, FIXED_SCALE};
use spark_engine::engine::{CompletedHistory, Engine, EngineGenesis};
use spark_engine::profile::config::ConfigRevision;
use spark_engine::report::{CohortOutcome, CommittedEffect, WaveRejection};
use spark_engine::request::{CommandPayload, CommandRequest, Outcome, Request};
use spark_engine::rules::{ActivatedRuleSet, Direction, Param, RuleSpec, Trigger, Update};
use spark_testkit::phase2::*;

const STRESS: &str = "state.stress";
const MOOD: &str = "state.mood";
const ENERGY: &str = "state.energy";
const ALARM: &str = "state.alarm";
const BOUNDED: &str = "state.bounded";
/// A clamp wide enough never to bind on any vector here.
const WIDE: (i64, i64) = (-1000, 1000);

fn a() -> ScopeId {
    actor("a")
}

fn tune(rate: i64, cadence: u64) -> ConfigRevision {
    config(vec![
        ("tune.rate", CanonicalValue::Int(rate)),
        ("tune.cadence", CanonicalValue::Int(cadence as i64)),
    ])
}

fn fixed(numerator: i64, denominator: i64) -> FixedPoint {
    FixedPoint::from_raw(FIXED_SCALE / denominator * numerator)
}

/// Scheduled work declared at genesis.
#[derive(Clone, Debug, Default)]
struct Work {
    /// Decay evaluations of `state.stress` (`rule.d`).
    evals: Vec<u64>,
    /// Composed `+5` then decay bodies on `state.mood` (`rule.mbody`).
    mood_bodies: Vec<u64>,
    /// Composed decay then `+5` bodies on `state.energy` (`rule.ebody`).
    energy_bodies: Vec<u64>,
}

impl Work {
    fn evals(evals: &[u64]) -> Self {
        Work {
            evals: evals.to_vec(),
            ..Work::default()
        }
    }
}

struct Rig {
    engine: Engine,
    genesis: EngineGenesis,
    present: ActivatedRuleSet,
    absent: ActivatedRuleSet,
    seq: u64,
    history: Vec<CommandRequest>,
}

impl Rig {
    fn new(rate: i64, cadence: u64, work: &Work) -> Self {
        Self::build(rate, cadence, work, 50)
    }

    fn build(rate: i64, cadence: u64, work: &Work, pacing: u32) -> Self {
        let mut f = standard();
        let clamp = |(min, max): (i64, i64)| Update::Clamp { min, max };
        // Every rule below is present in both epochs; only `rule.d`, the
        // hot-tunable decay operation on `state.stress`, distinguishes them.
        let common: Vec<RuleSpec> = vec![
            on_command(
                "rule.set",
                "cmd.set",
                vec![emit("s", STRESS, Update::Assign(param(0)))],
            ),
            on_command(
                "rule.shock",
                "cmd.shock",
                vec![emit("s", STRESS, Update::Add(param(0)))],
            ),
            // Additive compositions: the C2W-01 path. Each performs an additive
            // change and declares no decay stage.
            on_command(
                "rule.add.clamp",
                "cmd.add.clamp",
                vec![
                    emit("a", STRESS, Update::Add(param(0))),
                    emit("c", STRESS, clamp(WIDE)),
                ],
            ),
            on_command(
                "rule.add.clamp.tight",
                "cmd.add.clamp.tight",
                vec![
                    emit("a", STRESS, Update::Add(param(0))),
                    emit("c", STRESS, clamp((-96, 96))),
                ],
            ),
            on_command(
                "rule.add.scale.identity",
                "cmd.add.scale.identity",
                vec![
                    emit("a", STRESS, Update::Add(param(0))),
                    emit("s", STRESS, Update::Scale(fixed(1, 1))),
                ],
            ),
            on_command(
                "rule.add.scale.half",
                "cmd.add.scale.half",
                vec![
                    emit("a", STRESS, Update::Add(param(0))),
                    emit("s", STRESS, Update::Scale(fixed(1, 2))),
                ],
            ),
            on_command(
                "rule.clamp.add",
                "cmd.clamp.add",
                vec![
                    emit("c", STRESS, clamp(WIDE)),
                    emit("a", STRESS, Update::Add(param(0))),
                ],
            ),
            // A body of transform stages alone: no additive event, no settlement.
            on_command(
                "rule.scale.clamp",
                "cmd.scale.clamp",
                vec![
                    emit("s", STRESS, Update::Scale(fixed(1, 2))),
                    emit("c", STRESS, clamp(WIDE)),
                ],
            ),
            // Explicit replacements inside a body, before and after the addition.
            on_command(
                "rule.assign.add",
                "cmd.assign.add",
                vec![
                    emit("s", STRESS, Update::Assign(param(0))),
                    emit("a", STRESS, Update::Add(lit(3))),
                ],
            ),
            on_command(
                "rule.add.assign",
                "cmd.add.assign",
                vec![
                    emit("a", STRESS, Update::Add(param(0))),
                    emit("s", STRESS, Update::Assign(lit(40))),
                ],
            ),
            // The retained standalone boundary.
            on_command(
                "rule.scale",
                "cmd.scale",
                vec![emit("s", STRESS, Update::Scale(fixed(1, 2)))],
            ),
            // Bodies that DO declare a decay stage, in both stage orders, each on
            // its own target (one decay rule per target, v2 §4.3).
            on_command(
                "rule.mset",
                "cmd.mset",
                vec![emit("s", MOOD, Update::Assign(param(0)))],
            ),
            work_rule(
                "rule.mbody",
                "work.mbody",
                vec![
                    emit("a", MOOD, Update::Add(lit(5))),
                    emit("d", MOOD, decay(3, 6)),
                ],
            ),
            on_command(
                "rule.eset",
                "cmd.eset",
                vec![emit("s", ENERGY, Update::Assign(param(0)))],
            ),
            work_rule(
                "rule.ebody",
                "work.ebody",
                vec![
                    emit("d", ENERGY, decay(3, 6)),
                    emit("a", ENERGY, Update::Add(lit(5))),
                ],
            ),
            // A bounded target with its own decay operation and an additive
            // composition, for refusal atomicity.
            on_command(
                "rule.bset",
                "cmd.bset",
                vec![emit("s", BOUNDED, Update::Assign(param(0)))],
            ),
            on_command(
                "rule.bbody",
                "cmd.bbody",
                vec![
                    emit("a", BOUNDED, Update::Add(param(0))),
                    emit("c", BOUNDED, clamp(WIDE)),
                ],
            ),
            work_rule("rule.bd", "work.bd", vec![emit("d", BOUNDED, decay(1, 4))]),
            rule(
                "rule.watch",
                Trigger::Crossing {
                    watched: def(STRESS),
                    threshold: 97,
                    direction: Direction::Falling,
                },
                vec![emit("w", ALARM, Update::Add(lit(1)))],
            ),
        ];
        let baselines = vec![
            baseline(STRESS, 0),
            baseline(MOOD, 0),
            baseline(ENERGY, 0),
            baseline(BOUNDED, 0),
        ];
        let absent = f
            .rule_set_with(budgets(pacing), common.clone(), baselines.clone())
            .unwrap();
        let mut with_decay = common;
        with_decay.push(work_rule(
            "rule.d",
            "work.d",
            vec![emit(
                "d",
                STRESS,
                Update::Decay {
                    rate: Param::Config {
                        key: def("tune.rate"),
                    },
                    cadence: Param::Config {
                        key: def("tune.cadence"),
                    },
                },
            )],
        ));
        let initial_work = work
            .evals
            .iter()
            .map(|t| initial("rule.d", &a(), *t, "work.d"))
            .chain(
                work.mood_bodies
                    .iter()
                    .map(|t| initial("rule.mbody", &a(), *t, "work.mbody")),
            )
            .chain(
                work.energy_bodies
                    .iter()
                    .map(|t| initial("rule.ebody", &a(), *t, "work.ebody")),
            )
            .collect();
        let mut g = f.genesis_with(budgets(pacing), with_decay, baselines, initial_work);
        g.config = tune(rate, cadence);
        Rig {
            engine: Engine::genesis(g.clone()).unwrap(),
            present: g.rule_set.clone(),
            genesis: g,
            absent,
            seq: 0,
            history: Vec::new(),
        }
    }

    fn request(&mut self, at: u64, kind: &str, arg: i64) -> CommandRequest {
        self.seq += 1;
        let id = format!("cmd.{}", self.seq);
        command_request(&id, at, self.seq, kind, a(), vec![], vec![arg])
    }

    fn dispatch(&mut self, c: CommandRequest) -> CohortOutcome {
        let results = drive(&mut self.engine, &Request::Command(c.clone()));
        assert_eq!(results.last().unwrap().outcome(), &Outcome::Completed);
        self.history.push(c);
        reports(&results).last().unwrap().outcome.clone()
    }

    fn send(&mut self, at: u64, kind: &str, arg: i64) {
        let c = self.request(at, kind, arg);
        let at = c.effective_time.0;
        assert_eq!(
            self.dispatch(c),
            CohortOutcome::Committed,
            "command at {at}"
        );
    }

    fn try_send(&mut self, at: u64, kind: &str, arg: i64) -> CohortOutcome {
        let c = self.request(at, kind, arg);
        self.dispatch(c)
    }

    fn set(&mut self, at: u64, v: i64) {
        self.send(at, "cmd.set", v);
    }

    /// Activates an epoch, with or without the `state.stress` decay operation.
    fn activate(&mut self, at: u64, present: bool, rate: i64, cadence: u64) {
        let mut c = self.request(at, "spark.epoch.activate", 0);
        c.payload = CommandPayload::ActivateEpoch {
            rule_set: if present {
                self.present.clone()
            } else {
                self.absent.clone()
            },
            config: tune(rate, cadence),
        };
        let outcome = self.dispatch(c);
        assert!(
            matches!(outcome, CohortOutcome::EpochActivated { .. }),
            "activation at {at}: {outcome:?}"
        );
    }

    /// Advances to `at` and returns every committed effect of the cohorts run.
    fn go(&mut self, at: u64) -> Vec<CommittedEffect> {
        let results = drive(&mut self.engine, &advance(at));
        assert_eq!(results.last().unwrap().outcome(), &Outcome::Completed);
        reports(&results)
            .iter()
            .flat_map(|r| r.waves.iter().flat_map(|w| w.committed.clone()))
            .collect()
    }

    fn read(&self, definition: &str) -> Option<(i64, u64)> {
        self.engine
            .state()
            .get(&profile_id(), &def(definition), &a())
            .map(|c| match c.value {
                CanonicalValue::Int(v) => (v, c.updated_at.0),
                ref other => panic!("numeric cell expected, got {other:?}"),
            })
    }

    fn cell(&self) -> Option<(i64, u64)> {
        self.read(STRESS)
    }

    fn want(&self, value: i64, at: u64) {
        assert_eq!(self.cell(), Some((value, at)));
    }

    fn restore(&mut self) {
        let snapshot = self.engine.snapshot().unwrap();
        self.engine = Engine::restore(snapshot, &self.genesis.profile).unwrap();
    }

    fn replay(&self) {
        let history = CompletedHistory {
            commands: self.history.clone(),
            resets: vec![],
            frontier: self.engine.frontier(),
            recorded_history_digest: self.engine.timeline_history_digest(),
            recorded_stable_boundary_digest: self.engine.stable_boundary_digest(),
        };
        let replayed = Engine::reconstruct_completed(self.genesis.clone(), &history).unwrap();
        assert_eq!(digest_pair(&replayed), digest_pair(&self.engine));
    }
}

// ================================================================ C2W-01

/// **The review's two failing vectors, and their mirror images.** Baseline 0,
/// rate 10 / cadence 10, `100@0`, no evaluation before the write. A body of
/// `Add(+5)` finished by an identity `Clamp` or an identity `Scale` is an
/// additive event: it settles the grid point at 10 first, so it is `(95, 15)`,
/// and the evaluation at 20 gives `(85, 20)`. Recovery from `-100@0` mirrors it.
/// A transform stage placed *first* changes nothing.
///
/// Kills C2W-01 — forfeiting the pre-write debt whenever a transform stage is
/// present — which gives `(105, 15)` and `(95, 20)`.
#[test]
fn an_additive_composition_settles_before_its_declared_stages() {
    for (kind, delta) in [
        ("cmd.add.clamp", 5),
        ("cmd.add.scale.identity", 5),
        ("cmd.clamp.add", 5),
    ] {
        for sign in [1, -1] {
            let mut r = Rig::new(10, 10, &Work::evals(&[20]));
            r.set(0, 100 * sign);
            r.send(15, kind, delta * sign);
            r.want(95 * sign, 15);
            r.go(20);
            r.want(85 * sign, 20);
            r.replay();
        }
    }
}

/// **Catch-up at the additive write equals an explicit pre-evaluation.** The
/// same composed history with the grid point at 10 evaluated explicitly first
/// yields the same tuples at 15 and at 20. This is the property that makes
/// settlement a catch-up rather than a new rule, and it is the composed-body
/// counterpart of the resolution's pinned discriminator.
///
/// Kills any implementation whose composed result depends on whether an
/// evaluation happened to precede the write.
#[test]
fn a_composed_catch_up_equals_an_explicit_pre_evaluation() {
    for sign in [1, -1] {
        let mut catch_up = Rig::new(10, 10, &Work::evals(&[20]));
        catch_up.set(0, 100 * sign);
        catch_up.send(15, "cmd.add.clamp", 5 * sign);
        catch_up.want(95 * sign, 15);
        catch_up.go(20);
        catch_up.want(85 * sign, 20);

        let mut explicit = Rig::new(10, 10, &Work::evals(&[10, 20]));
        explicit.set(0, 100 * sign);
        explicit.go(10);
        explicit.want(90 * sign, 10);
        explicit.send(15, "cmd.add.clamp", 5 * sign);
        explicit.want(95 * sign, 15);
        explicit.go(20);
        explicit.want(85 * sign, 20);
        explicit.replay();
    }
}

/// **Nonidentity transforms apply to the settled sum.** With `100@0`, rate 10 /
/// cadence 10, a body of `Add(+5)` then `Scale(0.5)` at 15 resolves
/// `floor((90 + 5) / 2) = 47`, not `floor(105 / 2) = 52`; the evaluation at 20
/// gives `(37, 20)`. Recovery is written out rather than negated, because the
/// single floor runs toward negative infinity: `floor((-90 - 5) / 2) = -48`,
/// and the evaluation at 20 recovers to `(-38, 20)`.
///
/// A tight `Clamp[-96, 96]` after `Add(+5)` shows the same thing from the other
/// side: the settled sum 95 does not reach the bound, so the clamp is inert and
/// the cell is `(95, 15)`. Without settlement the unsettled 105 would bind and
/// give exactly 96 — a value the correct implementation can never produce here.
///
/// Kills settling after the transforms, and kills forfeiting the debt when the
/// finishing stage is not an identity.
#[test]
fn nonidentity_transforms_resolve_from_the_settled_sum() {
    let mut decayed = Rig::new(10, 10, &Work::evals(&[20]));
    decayed.set(0, 100);
    decayed.send(15, "cmd.add.scale.half", 5);
    decayed.want(47, 15);
    decayed.go(20);
    decayed.want(37, 20);
    decayed.replay();

    let mut recovered = Rig::new(10, 10, &Work::evals(&[20]));
    recovered.set(0, -100);
    recovered.send(15, "cmd.add.scale.half", -5);
    recovered.want(-48, 15);
    recovered.go(20);
    recovered.want(-38, 20);
    recovered.replay();

    for sign in [1, -1] {
        let mut tight = Rig::new(10, 10, &Work::evals(&[20]));
        tight.set(0, 100 * sign);
        tight.send(15, "cmd.add.clamp.tight", 5 * sign);
        assert_ne!(tight.cell(), Some((96 * sign, 15)), "unsettled sum bound");
        tight.want(95 * sign, 15);
        tight.go(20);
        tight.want(85 * sign, 20);
    }
}

/// **A composed additive write charges each grid point exactly once.** From
/// `100@0`, rate 10 / cadence 10: the body at 15 settles the point at 10 and
/// gives 95; a second body at 16 has nothing left to settle and gives 100; a
/// third at 25 settles only the point at 20 and gives 95; the evaluation at 30
/// gives 85. The committed `updated_at` excludes every previously charged
/// endpoint from later walks.
///
/// Kills re-settling an already settled interval and kills charging an endpoint
/// twice across successive composed writes.
#[test]
fn successive_composed_writes_never_charge_a_grid_point_twice() {
    for sign in [1, -1] {
        let mut r = Rig::new(10, 10, &Work::evals(&[30]));
        r.set(0, 100 * sign);
        r.send(15, "cmd.add.clamp", 5 * sign);
        r.want(95 * sign, 15);
        r.send(16, "cmd.add.clamp", 5 * sign);
        r.want(100 * sign, 16);
        r.send(25, "cmd.add.clamp", 5 * sign);
        r.want(95 * sign, 25);
        r.go(30);
        r.want(85 * sign, 30);
        r.replay();
    }
}

// ================================================================ preserved boundaries

/// **A body that declares a decay stage is not settled, and keeps its declared
/// stage order.** On `state.mood` with rate 3 / cadence 6, `+5` then decay from
/// `20@5` evaluated at 12 gives `(19, 12)`: the addition first, then the points
/// 6 and 12. Prepending settlement would walk those two endpoints twice —
/// settling 20 to 14, adding 5, then walking again to 13 — which is exactly the
/// double charge the corrected contract rules out.
///
/// From the saturation-sensitive `4@5` the two orders separate: `+5` then decay
/// gives `(3, 12)`, while decay then `+5`, declared on `state.energy`, gives
/// `(5, 12)` because the decay stage saturates at the baseline before the
/// addition. Both are the profile's declared choice.
///
/// Kills implicitly settling a body that declares a decay stage, and kills
/// reordering declared stages.
#[test]
fn a_body_that_declares_a_decay_stage_keeps_its_order_and_charges_once() {
    let mut linear = Rig::new(
        10,
        10,
        &Work {
            mood_bodies: vec![12],
            ..Work::default()
        },
    );
    linear.send(5, "cmd.mset", 20);
    linear.go(12);
    assert_eq!(linear.read(MOOD), Some((19, 12)));
    assert_ne!(
        linear.read(MOOD),
        Some((13, 12)),
        "double-charged endpoints"
    );
    linear.replay();

    let mut add_then_decay = Rig::new(
        10,
        10,
        &Work {
            mood_bodies: vec![12],
            ..Work::default()
        },
    );
    add_then_decay.send(5, "cmd.mset", 4);
    add_then_decay.go(12);
    assert_eq!(add_then_decay.read(MOOD), Some((3, 12)));

    let mut decay_then_add = Rig::new(
        10,
        10,
        &Work {
            energy_bodies: vec![12],
            ..Work::default()
        },
    );
    decay_then_add.send(5, "cmd.eset", 4);
    decay_then_add.go(12);
    assert_eq!(decay_then_add.read(ENERGY), Some((5, 12)));
    decay_then_add.replay();
}

/// **Transform-only paths keep the accepted standalone boundary.** A standalone
/// halving at 15 resolves from the committed 100 and gives `(50, 15)`, then
/// `(40, 20)`. A body of `Scale(0.5)` then a wide `Clamp` — two transform stages
/// and no additive stage — is the same: `(50, 15)` then `(40, 20)`. Neither is
/// an additive change nor a declared replacement, so neither settles. Recovery
/// mirrors both.
///
/// Kills extending settlement to every path that reads the committed value,
/// which would give 45 and 35.
#[test]
fn transform_only_paths_are_not_additive_events() {
    for kind in ["cmd.scale", "cmd.scale.clamp"] {
        for sign in [1, -1] {
            let mut r = Rig::new(10, 10, &Work::evals(&[20]));
            r.set(0, 100 * sign);
            r.send(15, kind, 0);
            r.want(50 * sign, 15);
            r.go(20);
            r.want(40 * sign, 20);
            r.replay();
        }
    }
}

/// **An explicit replacement inside a body is never reduced by earlier decay,
/// in either stage order.** From `100@0`, rate 10 / cadence 10: `Assign(50)`
/// then `Add(3)` at 15 gives `(53, 15)` and then `(43, 20)`; `Add(5)` then
/// `Assign(40)` gives `(40, 15)` and then `(30, 20)`. Settlement applies to the
/// starting value only, so the declared value wins wherever it is declared.
///
/// Kills charging the debt against a declared replacement (47 and 53, or 37 and
/// 40 after the evaluation).
#[test]
fn a_declared_replacement_inside_a_body_wins_in_either_stage_order() {
    for sign in [1, -1] {
        let mut first = Rig::new(10, 10, &Work::evals(&[20]));
        first.set(0, 100 * sign);
        first.send(15, "cmd.assign.add", 50 * sign);
        first.want(50 * sign + 3, 15);
        first.go(20);
        first.want(50 * sign + 3 - 10 * sign, 20);
        first.replay();
    }
    let mut last = Rig::new(10, 10, &Work::evals(&[20]));
    last.set(0, 100);
    last.send(15, "cmd.add.assign", 5);
    last.want(40, 15);
    last.go(20);
    last.want(30, 20);
    last.replay();
}

// ================================================================ interactions

/// **Removal and restoration under a composed additive write.** Rate 2 /
/// cadence 4 with `100@0`; the decay operation is removed at 13 and restored at
/// 22. A composed `Add(+5)` at 20, inside the absent window, still settles the
/// closing segment's whole steps 4, 8 and 12 — the discarded residual `(12, 13]`
/// is billed at no rate and the absent interval contributes nothing — giving
/// `(99, 20)`. The restored grid starts at 22, so its first point is 26 and the
/// evaluation there gives `(97, 26)`. Recovery mirrors it.
///
/// Kills resolving the settlement operation from the current epoch only, which
/// leaves the write unsettled at 105, and kills accruing across the removed
/// interval.
#[test]
fn a_composed_write_settles_the_closing_segment_while_the_operation_is_removed() {
    for sign in [1, -1] {
        let mut r = Rig::new(2, 4, &Work::evals(&[24, 26]));
        r.set(0, 100 * sign);
        r.activate(13, false, 2, 4);
        r.send(20, "cmd.add.clamp", 5 * sign);
        r.want(99 * sign, 20);
        r.activate(22, true, 2, 4);
        r.go(24);
        r.want(99 * sign, 24);
        r.go(26);
        r.want(97 * sign, 26);
        r.replay();
    }
}

/// **A composed additive write at a parameter-changing barrier.** Rate 2 /
/// cadence 4 with `100@0`, changing to 7 / 5 at 12. The composed `Add(+5)` at 12
/// settles the old segment's points 4, 8 and 12 — the step ending exactly at the
/// barrier is billed at the old rate — giving `(99, 12)`, and the new grid's
/// first point at 17 gives `(92, 17)`. Running the same history with an
/// evaluation also scheduled at the barrier gives the same tuples, so the
/// barrier endpoint is charged exactly once. These are the numbers the simple
/// additive path already produces, so the composed path is not a second policy.
///
/// Kills charging the new parameters before their barrier, dropping the closing
/// endpoint, and charging the barrier endpoint twice.
#[test]
fn a_composed_write_at_a_barrier_bills_the_closing_endpoint_once() {
    for evals in [vec![17], vec![12, 17]] {
        for sign in [1, -1] {
            let mut r = Rig::new(2, 4, &Work::evals(&evals));
            r.set(0, 100 * sign);
            r.activate(12, true, 7, 5);
            r.send(12, "cmd.add.clamp", 5 * sign);
            r.want(99 * sign, 12);
            r.go(17);
            r.want(92 * sign, 17);
            r.replay();
        }
    }
}

/// **A refused composed wave settles nothing.** `state.bounded` is declared
/// `0..=10` with its own rate 1 / cadence 4 operation and is `9@0`. A composed
/// `Add(+5)` at 6 would settle the point at 4 to 8 and then leave the declared
/// bounds, so the wave is refused atomically: the cell keeps both its value and
/// its commit time, `(9, 0)`. A subsequent composed `+1` at 10 then settles the
/// points at 4 and 8 to 7 and commits `(8, 10)` — proof that the refusal charged
/// no endpoint and consumed no debt.
///
/// Kills committing a settlement before the refusal, and kills treating a
/// refused wave as having advanced `updated_at`.
#[test]
fn a_refused_composed_wave_settles_nothing() {
    let mut r = Rig::new(10, 10, &Work::default());
    r.send(0, "cmd.bset", 9);
    assert_eq!(r.read(BOUNDED), Some((9, 0)));
    let outcome = r.try_send(6, "cmd.bbody", 5);
    assert!(
        matches!(
            outcome,
            CohortOutcome::Rejected {
                rejection: WaveRejection::InvalidEffect { .. },
                ..
            }
        ),
        "{outcome:?}"
    );
    assert_eq!(r.read(BOUNDED), Some((9, 0)), "nothing settled");
    r.send(10, "cmd.bbody", 1);
    assert_eq!(r.read(BOUNDED), Some((8, 10)));
}

/// **A composed additive write is one committed transition.** From `100@0`,
/// rate 10 / cadence 10, the body at 15 commits 95 once. A falling watcher at 97
/// therefore compares the committed 100 with the committed 95 and fires;
/// comparing against the settled intermediate 90 would make the transition a
/// rise and would not fire, which would be an unreported state change. Exactly
/// one `state.stress` effect is reported, carrying the settled sum, and
/// reapplying the reported effects to an untouched clone reproduces the cell
/// (AT-I1).
///
/// Kills comparing a watcher's threshold against the settled intermediate, and
/// kills emitting a separate settlement effect.
#[test]
fn a_composed_write_is_one_committed_transition_a_watcher_can_see() {
    let mut r = Rig::new(10, 10, &Work::default());
    r.set(0, 100);
    assert_eq!(r.read(ALARM), None);
    let c = r.request(15, "cmd.add.clamp", 5);
    let results = drive(&mut r.engine, &Request::Command(c.clone()));
    assert_eq!(results.last().unwrap().outcome(), &Outcome::Completed);
    r.history.push(c);
    let effects: Vec<CommittedEffect> = reports(&results)
        .iter()
        .flat_map(|rep| rep.waves.iter().flat_map(|w| w.committed.clone()))
        .collect();
    r.want(95, 15);
    assert_eq!(r.read(ALARM), Some((1, 15)), "100 -> 95 crosses 97 falling");
    let stress: Vec<&CommittedEffect> = effects
        .iter()
        .filter(|e| e.definition == def(STRESS))
        .collect();
    assert_eq!(stress.len(), 1, "one committed transition, not two");
    assert_eq!(stress[0].value, CanonicalValue::Int(95));

    let mut clone = Engine::genesis(r.genesis.clone()).unwrap();
    spark_engine::fixture::apply_committed_effects(&mut clone, &effects, LogicalTime(15));
    assert_eq!(
        clone
            .state()
            .get(&profile_id(), &def(STRESS), &a())
            .map(|c| (c.value.clone(), c.updated_at)),
        Some((CanonicalValue::Int(95), LogicalTime(15))),
    );
    r.replay();
}

/// **Replay, snapshot/restore and pacing reproduce composed settlement.** One
/// history mixing composed additive writes, a nonidentity transform, a declared
/// replacement inside a body, a parameter change and a removal/restoration runs
/// under a tight and a generous pacing budget, and again with a restore after
/// every command. All three agree at every stop, and reconstruction from the
/// completed history reproduces both engine digests.
#[test]
fn replay_restore_and_pacing_reproduce_composed_settlement() {
    let work = Work {
        evals: vec![9, 31, 44],
        ..Work::default()
    };
    let mut seen: Vec<Vec<Option<(i64, u64)>>> = Vec::new();
    for (pacing, restore_each) in [(1, false), (50, false), (50, true)] {
        let mut r = Rig::build(3, 7, &work, pacing);
        let mut stops = Vec::new();
        r.set(2, 100);
        r.go(9);
        stops.push(r.cell());
        r.send(15, "cmd.add.clamp", 5);
        if restore_each {
            r.restore();
        }
        stops.push(r.cell());
        r.activate(19, true, 5, 4);
        r.send(26, "cmd.add.scale.half", 7);
        if restore_each {
            r.restore();
        }
        stops.push(r.cell());
        r.send(28, "cmd.assign.add", 60);
        r.go(31);
        stops.push(r.cell());
        r.activate(34, false, 5, 4);
        r.send(38, "cmd.clamp.add", 7);
        if restore_each {
            r.restore();
        }
        stops.push(r.cell());
        r.activate(40, true, 5, 4);
        r.go(44);
        stops.push(r.cell());
        r.replay();
        seen.push(stops);
    }
    assert_eq!(seen[0], seen[1]);
    assert_eq!(seen[1], seen[2]);
    assert!(seen[0].iter().all(|c| c.is_some()));
}
