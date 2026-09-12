//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 decay-write resolution: the Operator's 2026-09-12 decision on the
//! four matters the fixed-grid adjudication
//! (`SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md` §5) left
//! open, recorded in `SPARK_GATE_C2_DECAY_WRITE_RESOLUTION_MISSION_2026-09-12.md`
//! and mapped onto the implementation in
//! `SPARK_PHASE_2_GATE_C2_DECAY_WRITE_IMPLEMENTATION_CONTRACT_2026-09-12.md`.
//!
//! 1. An **additive** change settles the applicable overdue decay or recovery
//!    against the existing value first, then applies its delta; it can no
//!    longer erase unapplied earlier grid steps.
//! 2. An **explicit replacement** sets the declared value, which earlier decay
//!    never reduces. Neither kind of write re-phases the fixed grid.
//! 3. **Removal** suspends accrual; **restoration** starts a fresh grid, while
//!    the closing segment's whole steps stay applicable and its unfinished
//!    residual is discarded.
//! 4. **Same-time activations** are processed in committed order; a change and
//!    its reversal still restart the grid, unchanged parameters preserve it.
//! 5. A single decay operation on an **absent cell** creates neither cell nor
//!    effect.
//!
//! Every vector is hand-valued and runs through the public door: commands for
//! writes and activations, scheduled work for decay evaluations, shocks and
//! composed bodies. Each test names the wrong implementation it kills. The
//! superseded reading — an additive write forfeiting the grid steps that ended
//! before it — is no longer asserted anywhere; it survives only as the
//! negative control in `additive_settlement_kills_the_former_lost_debt_reading`
//! and in the compiled mutants of this candidate's evidence.

use spark_core::clock::LogicalTime;
use spark_core::scope::ScopeId;
use spark_core::value::{CanonicalValue, FixedPoint, FIXED_SCALE};
use spark_engine::engine::{CompletedHistory, Engine, EngineGenesis};
use spark_engine::profile::config::ConfigRevision;
use spark_engine::report::{CohortOutcome, CommittedEffect, WaveRejection};
use spark_engine::request::{CommandPayload, CommandRequest, Outcome, Request};
use spark_engine::rules::{ActivatedRuleSet, Direction, EmitOp, Param, RuleSpec, Trigger, Update};
use spark_testkit::phase2::*;

const STRESS: &str = "state.stress";
const MOOD: &str = "state.mood";
const ALARM: &str = "state.alarm";
const BOUNDED: &str = "state.bounded";

fn a() -> ScopeId {
    actor("a")
}

fn tune(rate: i64, cadence: u64) -> ConfigRevision {
    config(vec![
        ("tune.rate", CanonicalValue::Int(rate)),
        ("tune.cadence", CanonicalValue::Int(cadence as i64)),
    ])
}

/// The hot-tunable decay/recovery operation on `state.stress`.
fn tuned_decay(sub: &str) -> EmitOp {
    emit(
        sub,
        STRESS,
        Update::Decay {
            rate: Param::Config {
                key: def("tune.rate"),
            },
            cadence: Param::Config {
                key: def("tune.cadence"),
            },
        },
    )
}

/// What a rig schedules at genesis. All scheduled work is declared up front,
/// so a window in which the decay operation is removed simply has no
/// evaluation due in it.
#[derive(Clone, Debug, Default)]
struct Work {
    /// Decay evaluations of `state.stress` (`rule.d`).
    evals: Vec<u64>,
    /// `+5` shocks on `state.stress` from scheduled work (`rule.kick`).
    kicks: Vec<u64>,
    /// Composed `+5` then decay bodies on `state.mood` (`rule.body`).
    bodies: Vec<u64>,
}

impl Work {
    fn evals(evals: &[u64]) -> Self {
        Work {
            evals: evals.to_vec(),
            ..Work::default()
        }
    }
}

/// One engine driven through the public door, with the rule set that carries
/// the `state.stress` decay operation and the one that does not.
struct Rig {
    engine: Engine,
    genesis: EngineGenesis,
    present: ActivatedRuleSet,
    absent: ActivatedRuleSet,
    seq: u64,
    history: Vec<CommandRequest>,
}

impl Rig {
    fn new(rate: i64, cadence: u64, baseline_value: i64, work: &Work) -> Self {
        Self::build(rate, cadence, baseline_value, work, 50)
    }

    fn build(rate: i64, cadence: u64, baseline_value: i64, work: &Work, pacing: u32) -> Self {
        let mut f = standard();
        // Rules present in both epochs. Only `rule.d` distinguishes them.
        let mut common: Vec<RuleSpec> = vec![
 on_command("rule.neutral", "cmd.neutral", vec![emit("a", STRESS, Update::Add(param(0))), emit("c", STRESS, Update::Clamp { min: -1000, max: 1000 })]),
 on_command("rule.clamp", "cmd.clamp", vec![emit("c", STRESS, Update::Clamp { min: -1000, max: 1000 })]),
 on_command("rule.identity", "cmd.identity", vec![emit("a", STRESS, Update::Add(param(0))), emit("s", STRESS, Update::Scale(FixedPoint::from_raw(FIXED_SCALE)))]),
            // Explicit replacement, and additive writes, on `state.stress`.
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
            work_rule(
                "rule.kick",
                "work.kick",
                vec![emit("k", STRESS, Update::Add(lit(5)))],
            ),
            // Two additive rules on one trigger and one target: a single
            // reduced group, so a single settlement.
            on_command(
                "rule.pair.a",
                "cmd.pair",
                vec![emit("p", STRESS, Update::Add(param(0)))],
            ),
            on_command(
                "rule.pair.b",
                "cmd.pair",
                vec![emit("q", STRESS, Update::Add(lit(3)))],
            ),
            // Neither additive nor an explicit replacement.
            on_command(
                "rule.scale",
                "cmd.scale",
                vec![emit(
                    "s",
                    STRESS,
                    Update::Scale(FixedPoint::from_raw(FIXED_SCALE / 2)),
                )],
            ),
            // A composed body on its own target: `+5`, then decay.
            on_command(
                "rule.mset",
                "cmd.mset",
                vec![emit("s", MOOD, Update::Assign(param(0)))],
            ),
            work_rule(
                "rule.body",
                "work.body",
                vec![
                    emit("a", MOOD, Update::Add(lit(5))),
                    emit("d", MOOD, decay(3, 6)),
                ],
            ),
            // A bounded target with its own decay operation, for refusal
            // atomicity.
            on_command(
                "rule.bset",
                "cmd.bset",
                vec![emit("s", BOUNDED, Update::Assign(param(0)))],
            ),
            on_command(
                "rule.bshock",
                "cmd.bshock",
                vec![emit("s", BOUNDED, Update::Add(param(0)))],
            ),
            work_rule("rule.bd", "work.bd", vec![emit("d", BOUNDED, decay(1, 4))]),
            // A falling-threshold watcher on `state.stress`.
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
        let mut delayed=on_command("rule.delay", "cmd.delay", vec![]);
        delayed.schedules.push(spark_engine::rules::ScheduleOp {sub_id:tag("later"),delay:15,work_kind:spark_testkit::phase2::work("work.later"),scope:spark_engine::rules::ScopeRef::Subject,mode:spark_engine::rules::ScheduleMode::Materialize {effects:vec![emit("a",STRESS,Update::Add(param(0)))]}});
        common.push(delayed);
        let baselines = vec![
            baseline(STRESS, baseline_value),
            baseline(MOOD, 0),
            baseline(BOUNDED, 0),
        ];
        let absent = f
            .rule_set_with(budgets(pacing), common.clone(), baselines.clone())
            .unwrap();
        let mut with_decay = common;
        with_decay.push(work_rule("rule.d", "work.d", vec![tuned_decay("d")]));
        let initial_work = work
            .evals
            .iter()
            .map(|t| initial("rule.d", &a(), *t, "work.d"))
            .chain(
                work.kicks
                    .iter()
                    .map(|t| initial("rule.kick", &a(), *t, "work.kick")),
            )
            .chain(
                work.bodies
                    .iter()
                    .map(|t| initial("rule.body", &a(), *t, "work.body")),
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

    /// Submits a command and asserts it committed.
    fn send(&mut self, at: u64, kind: &str, arg: i64) {
        let c = self.request(at, kind, arg);
        self.submit(c, CohortOutcome::Committed);
    }

    /// Submits a command and returns the last cohort outcome, whatever it is.
    fn try_send(&mut self, at: u64, kind: &str, arg: i64) -> CohortOutcome {
        let c = self.request(at, kind, arg);
        let results = drive(&mut self.engine, &Request::Command(c.clone()));
        assert_eq!(results.last().unwrap().outcome(), &Outcome::Completed);
        self.history.push(c);
        reports(&results).last().unwrap().outcome.clone()
    }

    fn submit(&mut self, c: CommandRequest, expected: CohortOutcome) {
        let results = drive(&mut self.engine, &Request::Command(c.clone()));
        assert_eq!(results.last().unwrap().outcome(), &Outcome::Completed);
        let last = reports(&results).last().unwrap().outcome.clone();
        assert_eq!(last, expected, "command at {}", c.effective_time.0);
        self.history.push(c);
    }

    fn set(&mut self, at: u64, v: i64) {
        self.send(at, "cmd.set", v);
    }

    fn shock(&mut self, at: u64, d: i64) {
        self.send(at, "cmd.shock", d);
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
        let results = drive(&mut self.engine, &Request::Command(c.clone()));
        assert_eq!(results.last().unwrap().outcome(), &Outcome::Completed);
        let last = reports(&results).last().unwrap().outcome.clone();
        assert!(
            matches!(last, CohortOutcome::EpochActivated { .. }),
            "activation at {at}: {last:?}"
        );
        self.history.push(c);
    }

    /// Advances to `at`, running everything due, and returns every committed
    /// effect of the cohorts it ran.
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

    /// Asserts `state.stress` is exactly `(value, commit time)`.
    fn want(&self, value: i64, at: u64) {
        assert_eq!(self.cell(), Some((value, at)));
    }

    fn restore(&mut self) {
        let snapshot = self.engine.snapshot().unwrap();
        self.engine = Engine::restore(snapshot, &self.genesis.profile).unwrap();
    }

    /// Reconstructs the same completed history from genesis and compares
    /// engine digests.
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
        assert_eq!(
            replayed
                .state()
                .get(&profile_id(), &def(STRESS), &a())
                .map(|c| (c.value.clone(), c.updated_at)),
            self.engine
                .state()
                .get(&profile_id(), &def(STRESS), &a())
                .map(|c| (c.value.clone(), c.updated_at)),
        );
    }
}



#[test] fn own_materialized_addition_settles() { for sign in [1,-1] {
let mut r=Rig::new(10,10,0,&Work::evals(&[20])); r.set(0,100*sign); r.send(0,"cmd.delay",5*sign); r.go(15); r.want(95*sign,15); r.go(20); r.want(85*sign,20); r.replay();
}}
