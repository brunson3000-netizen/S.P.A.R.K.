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
        let common: Vec<RuleSpec> = vec![
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

// ================================================================ behavior 1

/// **Behavior 1, the pinned discriminator.** Baseline 0, 100 at 0, rate 10 and
/// cadence 10. The additive `+5` at 15 settles the grid point at 10 first:
/// `(95, 15)`, and the evaluation at 20 gives `(85, 20)`. Explicitly
/// scheduling the decay evaluation at 10 before the addition must give the
/// same numbers, which is the whole point of settlement. Recovery mirrors it.
///
/// Kills the former lost-debt reading, which forfeits the point at 10 and
/// gives `(105, 15)` then `(95, 20)`.
#[test]
fn an_additive_write_settles_overdue_decay_before_applying_its_delta() {
    for sign in [1, -1] {
        // No evaluation precedes the additive write.
        let mut r = Rig::new(10, 10, 0, &Work::evals(&[20]));
        r.set(0, 100 * sign);
        r.shock(15, 5 * sign);
        r.want(95 * sign, 15);
        r.go(20);
        r.want(85 * sign, 20);
        r.replay();

        // The same history with the grid point evaluated explicitly first.
        let mut e = Rig::new(10, 10, 0, &Work::evals(&[10, 20]));
        e.set(0, 100 * sign);
        e.go(10);
        e.want(90 * sign, 10);
        e.shock(15, 5 * sign);
        e.want(95 * sign, 15);
        e.go(20);
        e.want(85 * sign, 20);
        e.replay();
    }
}

/// **The negative control for behavior 1.** The former implementation folded
/// the delta onto the raw committed value. This states the exact tuples that
/// reading produces, and asserts the engine does *not* produce them. A mutant
/// that restores the raw pre-wave base fails
/// `an_additive_write_settles_overdue_decay_before_applying_its_delta` on its
/// first assertion; this test states the superseded numbers explicitly so the
/// supersession is legible in the suite rather than only in the evidence.
#[test]
fn additive_settlement_kills_the_former_lost_debt_reading() {
    let mut r = Rig::new(10, 10, 0, &Work::evals(&[20]));
    r.set(0, 100);
    r.shock(15, 5);
    assert_ne!(r.cell(), Some((105, 15)), "superseded lost-debt reading");
    r.want(95, 15);
    r.go(20);
    assert_ne!(r.cell(), Some((95, 20)), "superseded lost-debt reading");
    r.want(85, 20);
}

/// **Behavior 2 against behavior 1.** The same history, one write apart: an
/// explicit replacement of 100 at 15 gives `(100, 15)` and `(90, 20)` — earlier
/// decay does not reduce the declared value — while the additive `+5` settles.
/// Neither re-phases the grid: both later evaluations land on the point at 20.
/// Kills settling a replacement, and kills re-phasing on either write.
#[test]
fn an_explicit_replacement_is_never_reduced_by_earlier_decay() {
    for sign in [1, -1] {
        let mut r = Rig::new(10, 10, 0, &Work::evals(&[20, 30]));
        r.set(0, 100 * sign);
        r.set(15, 100 * sign);
        r.want(100 * sign, 15);
        r.go(20);
        r.want(90 * sign, 20);
        r.go(30);
        r.want(80 * sign, 30);
        r.replay();

        // A replacement also supersedes the closing steps an additive write
        // would have settled.
        let mut s = Rig::new(10, 10, 0, &Work::evals(&[20]));
        s.set(0, 100 * sign);
        s.set(15, 40 * sign);
        s.want(40 * sign, 15);
        s.go(20);
        s.want(30 * sign, 20);
    }
}

/// **Behavior 1 at aligned, unaligned and not-yet-due write times.** From 100
/// at 0 with rate 10 and cadence 10:
/// - a `+5` at 9 has no overdue step, so it is `(105, 9)` and the evaluation
///   at 10 gives `(95, 10)`;
/// - a `+5` exactly on the grid point 20 settles both 10 and 20 — `(85, 20)`,
///   the endpoint at `now` included — and the evaluation at 30 gives
///   `(75, 30)`;
/// - a `+5` at 25 settles 10 and 20 and nothing else: `(85, 25)`.
///
/// Kills excluding the endpoint at `now` from settlement (90 at 20), settling
/// a step that has not ended (95 at 9) and double-charging the anchor.
#[test]
fn settlement_counts_exactly_the_grid_points_that_have_ended() {
    for sign in [1, -1] {
        let mut early = Rig::new(10, 10, 0, &Work::evals(&[10]));
        early.set(0, 100 * sign);
        early.shock(9, 5 * sign);
        early.want(105 * sign, 9);
        early.go(10);
        early.want(95 * sign, 10);
        early.replay();

        let mut aligned = Rig::new(10, 10, 0, &Work::evals(&[30]));
        aligned.set(0, 100 * sign);
        aligned.shock(20, 5 * sign);
        aligned.want(85 * sign, 20);
        aligned.go(30);
        aligned.want(75 * sign, 30);
        aligned.replay();

        let mut unaligned = Rig::new(10, 10, 0, &Work::default());
        unaligned.set(0, 100 * sign);
        unaligned.shock(25, 5 * sign);
        unaligned.want(85 * sign, 25);
        unaligned.replay();
    }
}

/// **Repeated additions settle once each, and co-firing additions settle once
/// in total.** From 100 at 0 with rate 10 and cadence 10: `+5` at 15 gives 95,
/// a second `+5` at 16 has nothing left to settle and gives 100, and a third
/// at 25 settles only the point at 20 and gives 95. Two additive rules firing
/// on one command at 25 reduce to one group: the base settles once (80) and
/// both deltas apply (`+5` and `+3`), giving `(88, 25)`.
///
/// Kills re-settling an already settled interval, and kills settling once per
/// contributing candidate (which would give 73).
#[test]
fn repeated_and_co_firing_additions_never_charge_a_grid_point_twice() {
    for sign in [1, -1] {
        let mut r = Rig::new(10, 10, 0, &Work::evals(&[30]));
        r.set(0, 100 * sign);
        r.shock(15, 5 * sign);
        r.want(95 * sign, 15);
        r.shock(16, 5 * sign);
        r.want(100 * sign, 16);
        r.shock(25, 5 * sign);
        r.want(95 * sign, 25);
        r.go(30);
        r.want(85 * sign, 30);
        r.replay();
    }
    let mut pair = Rig::new(10, 10, 0, &Work::default());
    pair.set(0, 100);
    pair.send(25, "cmd.pair", 5);
    pair.want(88, 25);
    pair.replay();
}

/// **Settlement saturates at the baseline and is inert at rate zero.**
/// Baseline 11, rate 4, cadence 6, starting seven above it at 1: the points 6,
/// 12 and 18 settle 18 → 14 → 11 → 11 and stop exactly at the baseline, so the
/// `+3` at 20 gives `(14, 20)`, not the unsettled 21. Recovery from seven below
/// mirrors it. At rate 0 the points exist but move nothing, so a `+5` at 20
/// over 70 is `(75, 20)`.
///
/// Kills overshooting the baseline during settlement (10, then 13 at 20) and
/// kills skipping settlement whenever it would not move the value.
#[test]
fn settlement_saturates_at_the_baseline_and_is_inert_at_rate_zero() {
    for sign in [1, -1] {
        let base = 11;
        let mut r = Rig::new(4, 6, base, &Work::evals(&[24]));
        r.set(1, base + 7 * sign);
        r.shock(20, 3 * sign);
        r.want(base + 3 * sign, 20);
        r.go(24);
        r.want(base, 24);
        r.replay();
    }
    let mut zero = Rig::new(0, 6, 0, &Work::default());
    zero.set(1, 70);
    zero.shock(20, 5);
    zero.want(75, 20);
    zero.replay();
}

// ================================================================ behaviors 3 and 4

/// **Behavior 3.** Rate 2, cadence 4, 100 at 0. The operation is removed at 13
/// and restored at 22. The old segment owns the points 4, 8 and 12; the
/// unfinished residual `(12, 13]` is discarded; nothing accrues while the
/// operation is absent; the restored grid starts at 22, so its first point is
/// 26. Evaluations at 25 and 26 give `(94, 25)` and `(92, 26)`.
///
/// Under behavior 1 an additive write inside the absent window still settles
/// the closing segment's whole steps: `+5` at 20 gives `(99, 20)`, and after
/// restoration the point at 26 gives `(97, 26)`. Kills accruing across the
/// absent interval (90 at 20), kills carrying the old origin into the restored
/// grid (a point at 24) and kills dropping the retained closing steps
/// (105 at 20).
#[test]
fn removal_suspends_accrual_and_restoration_starts_a_fresh_grid() {
    for sign in [1, -1] {
        let mut r = Rig::new(2, 4, 0, &Work::evals(&[25, 26]));
        r.set(0, 100 * sign);
        r.activate(13, false, 2, 4);
        r.restore();
        r.activate(22, true, 2, 4);
        r.go(25);
        r.want(94 * sign, 25);
        r.go(26);
        r.want(92 * sign, 26);
        r.replay();

        let mut w = Rig::new(2, 4, 0, &Work::evals(&[24, 26]));
        w.set(0, 100 * sign);
        w.activate(13, false, 2, 4);
        w.shock(20, 5 * sign);
        w.want(99 * sign, 20);
        w.activate(22, true, 2, 4);
        w.go(24);
        w.want(99 * sign, 24);
        w.go(26);
        w.want(97 * sign, 26);
        w.replay();
    }
}

/// **Behavior 4.** Two activations at one logical time are processed in
/// committed order. Rate 2 / cadence 4 with 100 at 0; at 13 the parameters
/// change to 7 / 5 and are immediately reversed to 2 / 4. The change closes the
/// old segment — the points 4, 8 and 12 give 94 — and the reversal starts a
/// fresh grid at 13 through an intermediate zero-length epoch, so the next
/// point is 17: `(94, 16)` then `(92, 17)`. Two activations that both leave the
/// parameters unchanged preserve the grid: `(92, 16)` at the original point 16.
///
/// An additive write at the same time as the activations settles the closing
/// segment at the old rate and is never charged for the new parameters before
/// their barrier: `+5` at 13 gives `(99, 13)`, and the fresh grid's point at 17
/// gives `(97, 17)`. Kills skipping the zero-length epoch, kills re-phasing on
/// an unchanged activation, and kills charging the new parameters pre-barrier.
#[test]
fn same_time_activations_are_processed_in_committed_order() {
    for sign in [1, -1] {
        for changed in [false, true] {
            let mut r = Rig::new(2, 4, 0, &Work::evals(&[16, 17]));
            r.set(0, 100 * sign);
            if changed {
                r.activate(13, true, 7, 5);
            } else {
                r.activate(13, true, 2, 4);
            }
            r.restore();
            r.activate(13, true, 2, 4);
            r.go(16);
            r.want(if changed { 94 } else { 92 } * sign, 16);
            r.go(17);
            r.want(92 * sign, 17);
            r.replay();
        }

        let mut w = Rig::new(2, 4, 0, &Work::evals(&[17]));
        w.set(0, 100 * sign);
        w.activate(13, true, 7, 5);
        w.activate(13, true, 2, 4);
        w.shock(13, 5 * sign);
        w.want(99 * sign, 13);
        w.go(17);
        w.want(97 * sign, 17);
        w.replay();
    }
}

/// **Behavior 4 at a parameter-changing barrier, with and without a scheduled
/// evaluation there.** Rate 2 / cadence 4 with 100 at 0, changing to 7 / 5 at
/// 12. Whether or not an evaluation is scheduled at 12, the additive `+5` at 12
/// sees 94 — the step ending exactly at the barrier is billed at the old rate —
/// and gives `(99, 12)`; the new grid's first point at 17 gives `(92, 17)`.
/// The two histories agree, so settlement charges the barrier endpoint exactly
/// once. Kills a second charge at the same endpoint (92 at 12) and kills
/// dropping the closing endpoint (101 at 12).
#[test]
fn settlement_bills_the_barrier_endpoint_once_at_the_old_rate() {
    for evals in [vec![17], vec![12, 17]] {
        let mut r = Rig::new(2, 4, 0, &Work::evals(&evals));
        r.set(0, 100);
        r.activate(12, true, 7, 5);
        r.shock(12, 5);
        r.want(99, 12);
        r.go(17);
        r.want(92, 17);
        r.replay();
    }
}

// ================================================================ behavior 5

/// **Behavior 5.** A decay evaluation of a cell that does not exist creates
/// neither cell nor effect, and it is not an error: the evaluation at 4 leaves
/// the cell absent, the assignment at 7 creates it, and the evaluation at 8
/// takes the point at 8 to give `(98, 8)`. An additive write to an absent cell
/// has nothing to settle and creates the cell from its delta alone: `+5` at 9
/// gives `(5, 9)`, and the point at 12 then gives `(3, 12)`.
///
/// Kills creating a cell from a decay evaluation, and kills settling a
/// non-existent value as though it were the baseline.
#[test]
fn a_decay_evaluation_never_creates_an_absent_cell() {
    let mut r = Rig::new(2, 4, 0, &Work::evals(&[4, 8]));
    let effects = r.go(4);
    assert_eq!(r.cell(), None);
    assert!(effects.iter().all(|e| e.definition != def(STRESS)));
    r.restore();
    r.set(7, 100);
    r.go(8);
    r.want(98, 8);
    r.replay();

    let mut fresh = Rig::new(2, 4, 0, &Work::evals(&[12]));
    fresh.go(4);
    assert_eq!(fresh.cell(), None);
    fresh.shock(9, 5);
    fresh.want(5, 9);
    fresh.go(12);
    fresh.want(3, 12);
    fresh.replay();
}

// ================================================================ boundaries

/// **A composed rule body keeps its declared stage order.** The body is `+5`
/// then decay on `state.mood`, rate 3 and cadence 6. From 4 at 5, the body at
/// 12 adds first — 9 — and then applies the points 6 and 12: `(3, 12)`.
/// Settling the body's starting value instead would clamp 4 to the baseline
/// before the addition and give 5, and would leave the declared decay stage
/// with nothing to do.
///
/// This is the boundary recorded in the implementation contract §5: the
/// sanctioned additive-plus-decay composition already accounts for the grid in
/// declared stage order, so an implicit settlement would charge the same
/// endpoints twice. The profile expresses settle-then-add by declaring the
/// decay stage first. Kills implicitly settling a composed body.
#[test]
fn a_composed_body_folds_its_declared_stages_in_order() {
    let mut r = Rig::new(
        10,
        10,
        0,
        &Work {
            bodies: vec![6, 12],
            ..Work::default()
        },
    );
    r.send(5, "cmd.mset", 4);
    assert_eq!(r.read(MOOD), Some((4, 5)));
    r.go(6);
    assert_eq!(r.read(MOOD), Some((6, 6)));
    r.go(12);
    assert_eq!(r.read(MOOD), Some((8, 12)));
    r.replay();

    let mut late = Rig::new(
        10,
        10,
        0,
        &Work {
            bodies: vec![12],
            ..Work::default()
        },
    );
    late.send(5, "cmd.mset", 4);
    late.go(12);
    assert_eq!(late.read(MOOD), Some((3, 12)));
    late.replay();
}

/// **`Scale` is neither an additive change nor an explicit replacement, and
/// keeps its existing semantics.** From 100 at 0 with rate 10 and cadence 10, a
/// halving at 15 resolves from the committed 100 and gives `(50, 15)`; the
/// point at 20 then gives `(40, 20)`. Settling first would give 45 and 35.
/// This is the second boundary of implementation contract §5, pinned so it is
/// visible rather than accidental.
#[test]
fn scale_keeps_its_existing_semantics() {
    let mut r = Rig::new(10, 10, 0, &Work::evals(&[20]));
    r.set(0, 100);
    r.send(15, "cmd.scale", 0);
    r.want(50, 15);
    r.go(20);
    r.want(40, 20);
    r.replay();
}

// ================================================================ reporting and atomicity

/// **A watcher observes one committed transition.** Settlement is folded into
/// the single committed effect of the additive write, so a falling-threshold
/// watcher at 97 compares the committed 100 with the committed 95 and fires.
/// Comparing against the settled intermediate 90 would make the transition a
/// rise and would not fire, which would be an unreported state transition.
/// The committed effect reported for the write carries the settled sum, and
/// reapplying the reported effects to an untouched clone reproduces the cell
/// exactly (AT-I1).
///
/// Kills comparing a watcher's threshold against the settled intermediate, and
/// kills reporting an effect whose value differs from what was committed.
#[test]
fn a_watcher_sees_one_committed_transition_and_the_reported_effect_matches() {
    let mut r = Rig::new(10, 10, 0, &Work::default());
    r.set(0, 100);
    assert_eq!(r.read(ALARM), None);
    let c = r.request(15, "cmd.shock", 5);
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

    // The same effects applied to an untouched clone give the same cell.
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

/// **A refused wave settles nothing.** Two refusals, each with a settlement
/// that would have applied had the wave committed:
/// - a coincident scheduled decay and scheduled `+5` on one target is the
///   retained atomic cross-family rejection (v2 §4.3); the cell keeps both its
///   value and its commit time;
/// - an additive write that leaves the declared bounds is refused; `state.bounded`
///   is 9 at 0 with rate 1 and cadence 4, so `+5` at 6 would settle the point at
///   4 to 8 and then exceed the maximum of 10. The cell must remain `(9, 0)`.
///
/// Kills committing a settlement before the refusal — which would leave
/// `(8, 6)` or `(9, 20)` behind — and kills routing settlement through the
/// forbidden family path.
#[test]
fn a_refused_wave_settles_nothing() {
    let mut mixed = Rig::new(
        10,
        10,
        0,
        &Work {
            evals: vec![20],
            kicks: vec![20],
            ..Work::default()
        },
    );
    mixed.set(0, 100);
    mixed.go(20);
    assert_eq!(mixed.cell(), Some((100, 0)), "nothing settled or committed");

    let mut bounded = Rig::new(10, 10, 0, &Work::default());
    bounded.send(0, "cmd.bset", 9);
    assert_eq!(bounded.read(BOUNDED), Some((9, 0)));
    let outcome = bounded.try_send(6, "cmd.bshock", 5);
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
    assert_eq!(bounded.read(BOUNDED), Some((9, 0)), "nothing settled");
}

/// **Replay, snapshot/restore and pacing reproduce settlement exactly.** One
/// history with unaligned additive writes, a replacement, a parameter change
/// and a removal/restoration runs under a tight and a generous pacing budget,
/// with a restore after every command. All three agree on the cell at every
/// stop and on both engine digests, and reconstruction from the completed
/// history reproduces them.
#[test]
fn replay_restore_and_pacing_reproduce_settlement() {
    let work = Work {
        evals: vec![9, 31, 44],
        kicks: vec![26],
        ..Work::default()
    };
    let mut seen: Vec<Vec<Option<(i64, u64)>>> = Vec::new();
    for (pacing, restore_each) in [(1, false), (50, false), (50, true)] {
        let mut r = Rig::build(3, 7, 0, &work, pacing);
        let mut stops = Vec::new();
        r.set(2, 100);
        if restore_each {
            r.restore();
        }
        stops.push(r.cell());
        r.go(9);
        stops.push(r.cell());
        r.shock(15, 5);
        if restore_each {
            r.restore();
        }
        stops.push(r.cell());
        r.activate(19, true, 5, 4);
        if restore_each {
            r.restore();
        }
        r.go(26);
        stops.push(r.cell());
        r.set(28, 60);
        if restore_each {
            r.restore();
        }
        r.go(31);
        stops.push(r.cell());
        r.activate(34, false, 5, 4);
        r.shock(38, 7);
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
