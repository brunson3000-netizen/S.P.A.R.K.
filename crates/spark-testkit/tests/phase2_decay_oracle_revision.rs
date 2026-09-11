//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 second bounded correction: the independent review's C2R-01 … C2R-04
//! (`SPARK_PHASE_2_CODEX_GATE_C2_REVISION_INDEPENDENT_REVIEW_2026-09-11.md`).
//!
//! - C2R-01/02/03: decay/recovery commits at the cohort's canonical time and
//!   counts whole cadence steps on the operation's parameter-segment grid.
//!   Named fixtures pin exact values; a reference model (brute-force grid
//!   enumeration, written independently of the engine's closed form) is swept
//!   over generated parameter timelines, evaluation partitions, pacing
//!   budgets, replay, and restore.
//! - C2R-04: AT-I8 (per-committed-wave invariant), AT-I20b (retry of the
//!   rejected producer under its next occurrence; all retained components),
//!   AT-I21 (value-dependent interference), AT-I28 (per-prefix recomputation
//!   and injected index faults), AT-I33 (maximal configured-depth multi-parent
//!   chain), AT-I39/AT-I40 (composed per-profile oracle under the accepted
//!   one-engine-per-profile architecture, D-C2-2 — not a multi-profile
//!   runtime).
//!
//! Each test names the wrong implementation it kills.

use spark_core::authority::Authority;
use spark_core::clock::LogicalTime;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{ProfileId, SourceId};
use spark_core::scheduler::{OccurrenceIndex, WorkKey};
use spark_core::scope::ScopeId;
use spark_core::timeline::{DerivedIndexFault, TimelineEpoch};
use spark_core::value::CanonicalValue;
use spark_engine::activation::ActivationRegistry;
use spark_engine::effects::scheduled_cohort_identity;
use spark_engine::engine::{
    CompletedHistory, Engine, EngineGenesis, RestoreError, TimelineGenesis,
};
use spark_engine::fixture;
use spark_engine::profile::config::{ConfigEntry, ConfigRevision};
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::profile::text::BoundedText;
use spark_engine::report::{CohortOutcome, CohortReport, WaveRejection};
use spark_engine::request::{
    CommandPayload, CommandRequest, FinalizationRefusal, Outcome, Request,
};
use spark_engine::rules::{
    ActivatedRuleSet, Direction, Expr, Param, RuleSetError, RuleSetSpec, RuleSpec, Trigger, Update,
};
use spark_testkit::phase2::*;
use std::collections::{BTreeMap, BTreeSet};

fn a() -> ScopeId {
    actor("a")
}

fn add(v: i64) -> Update {
    Update::Add(lit(v))
}

fn adder(id: &str, kind: &str, target: &str, v: i64) -> RuleSpec {
    work_rule(id, kind, vec![emit("x", target, add(v))])
}

fn run(e: &mut Engine, t: u64) -> Vec<CohortReport> {
    reports(&drive(e, &advance(t)))
}

fn stress(e: &Engine) -> (i64, LogicalTime) {
    let c = e
        .state()
        .get(&profile_id(), &def("state.stress"), &a())
        .unwrap();
    match c.value {
        CanonicalValue::Int(v) => (v, c.updated_at),
        _ => panic!("numeric"),
    }
}

fn components(e: &Engine) -> [Digest; 7] {
    [
        e.state().canonical_state_digest(),
        e.scheduler_digest(),
        e.timeline_state_digest(),
        e.epoch_registry().canonical_digest(),
        e.obligations().canonical_digest(),
        e.occurrences().canonical_digest(),
        e.cooldowns().canonical_digest(),
    ]
}

fn activation_command(
    id: &str,
    t: u64,
    seq: u64,
    rule_set: ActivatedRuleSet,
    cfg: ConfigRevision,
) -> CommandRequest {
    CommandRequest {
        payload: CommandPayload::ActivateEpoch {
            rule_set,
            config: cfg,
        },
        ..command_request(id, t, seq, "spark.epoch.activate", a(), vec![], vec![])
    }
}

// ================================================================ decay harness

/// A literal-parameter decay engine: `state.stress@a` = `initial` at 0,
/// baseline 0, evaluations at `times`.
fn literal_decay(times: &[u64], start: i64, rate: i64, cadence: i64) -> Engine {
    let mut f = standard();
    let seed = on_command(
        "rule.seed",
        "cmd.seed",
        vec![emit("s", "state.stress", Update::Assign(lit(start)))],
    );
    let d = work_rule(
        "rule.d",
        "work.d",
        vec![emit("d", "state.stress", decay(rate, cadence))],
    );
    let mut e = Engine::genesis(
        f.genesis_with(
            budgets(50),
            vec![seed, d],
            vec![baseline("state.stress", 0)],
            times
                .iter()
                .map(|t| initial("rule.d", &a(), *t, "work.d"))
                .collect(),
        ),
    )
    .unwrap();
    drive(&mut e, &command("cmd.seed", 0, 1, "cmd.seed", a()));
    e
}

fn tune(rate: i64, cadence: u64) -> ConfigRevision {
    config(vec![
        ("tune.rate", CanonicalValue::Int(rate)),
        ("tune.cadence", CanonicalValue::Int(cadence as i64)),
    ])
}

/// A parameter timeline: the genesis `(rate, cadence)`, then activations
/// `(time, parameters)` where `None` is an epoch whose artifact lacks the
/// decay operation. Both parameters are hot-tunable config keys.
#[derive(Clone, Debug)]
struct Scenario {
    initial: i64,
    genesis: (i64, u64),
    epochs: Vec<(u64, Option<(i64, u64)>)>,
    evals: Vec<u64>,
}

/// The parameters in effect for scheduled work at `t` (an activation at `t`
/// is a command, which runs after the scheduled work due at `t`).
fn in_effect(s: &Scenario, t: u64) -> Option<(i64, u64)> {
    let mut p = Some(s.genesis);
    for (at, q) in &s.epochs {
        if *at < t {
            p = *q;
        }
    }
    p
}

struct Run {
    engine: Engine,
    genesis: EngineGenesis,
    commands: Vec<CommandRequest>,
}

fn end_of(s: &Scenario, evals: &[u64]) -> u64 {
    let last_eval = evals.iter().copied().max().unwrap_or(0);
    let last_activation = s.epochs.iter().map(|(t, _)| *t).max().unwrap_or(0);
    last_eval.max(last_activation)
}

/// Runs a scenario through the public door: seed at 0, each activation as a
/// finalized reserved-kind command, then an advance to the end. With
/// `restore_after = Some(i)`, the engine is snapshotted and restored after the
/// `i`-th command and continues from the restored instance.
fn run_scenario(s: &Scenario, pacing: u32, evals: &[u64], restore_after: Option<usize>) -> Run {
    let mut f = standard();
    let seed = on_command(
        "rule.seed",
        "cmd.seed",
        vec![emit("s", "state.stress", Update::Assign(param(0)))],
    );
    let d = work_rule(
        "rule.d",
        "work.d",
        vec![emit(
            "d",
            "state.stress",
            Update::Decay {
                rate: Param::Config {
                    key: def("tune.rate"),
                },
                cadence: Param::Config {
                    key: def("tune.cadence"),
                },
            },
        )],
    );
    let mut g = f.genesis_with(
        budgets(pacing),
        vec![seed.clone(), d],
        vec![baseline("state.stress", 0)],
        evals
            .iter()
            .map(|t| initial("rule.d", &a(), *t, "work.d"))
            .collect(),
    );
    g.config = tune(s.genesis.0, s.genesis.1);
    let present = g.rule_set.clone();
    let absent = f
        .rule_set_with(
            budgets(pacing),
            vec![seed],
            vec![baseline("state.stress", 0)],
        )
        .unwrap();
    let genesis = g.clone();
    let mut e = Engine::genesis(g).unwrap();
    let mut commands = vec![command_request(
        "cmd.seed",
        0,
        1,
        "cmd.seed",
        a(),
        vec![],
        vec![s.initial],
    )];
    for (i, (t, p)) in s.epochs.iter().enumerate() {
        let (rs, cfg) = match p {
            Some((rate, cadence)) => (present.clone(), tune(*rate, *cadence)),
            None => (absent.clone(), tune(0, 1)),
        };
        commands.push(activation_command(
            &format!("cmd.epoch{i}"),
            *t,
            i as u64 + 2,
            rs,
            cfg,
        ));
    }
    for (i, c) in commands.iter().enumerate() {
        let r = drive(&mut e, &Request::Command(c.clone()));
        assert_eq!(r.last().unwrap().outcome(), &Outcome::Completed);
        if i > 0 {
            assert!(matches!(
                reports(&r).last().unwrap().outcome,
                CohortOutcome::EpochActivated { .. }
            ));
        }
        if restore_after == Some(i) {
            let snapshot = e.snapshot().unwrap();
            e = Engine::restore(snapshot, &genesis.profile).unwrap();
        }
    }
    drive(&mut e, &advance(end_of(s, evals)));
    Run {
        engine: e,
        genesis,
        commands,
    }
}

/// The reference model: every whole cadence step is enumerated explicitly
/// (no division) as a grid point `origin + k·cadence` of its parameter
/// segment; each evaluation applies the steps ending in `(last, t]` at their
/// segment's rate, linearly toward baseline 0, and commits at `t`.
fn model(s: &Scenario, evals: &[u64]) -> (i64, LogicalTime) {
    let end = end_of(s, evals);
    let mut epochs = vec![(0u64, Some(s.genesis))];
    epochs.extend(s.epochs.iter().cloned());
    let mut steps: Vec<(u64, i64)> = Vec::new();
    let mut segment: Option<((i64, u64), u64)> = None;
    for (i, (start, p)) in epochs.iter().enumerate() {
        let stop = epochs.get(i + 1).map_or(end, |n| n.0.min(end));
        segment = match (p, segment) {
            (None, _) => None,
            (Some(q), Some((prev, origin))) if *q == prev => Some((prev, origin)),
            (Some(q), _) => Some((*q, *start)),
        };
        let Some(((rate, cadence), origin)) = segment else {
            continue;
        };
        let mut t = origin + cadence;
        while t <= stop {
            if t > *start {
                steps.push((t, rate));
            }
            t += cadence;
        }
    }
    let toward = |v: i64, rate: i64| {
        if v > 0 {
            (v - rate).max(0)
        } else {
            (v + rate).min(0)
        }
    };
    let (mut v, mut last) = (s.initial, 0u64);
    for &e in evals {
        for &(t, rate) in &steps {
            if last < t && t <= e {
                v = toward(v, rate);
            }
        }
        last = e;
    }
    (v, LogicalTime(last))
}

struct Lcg(u64);

impl Lcg {
    fn below(&mut self, n: u64) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) % n
    }

    fn parameters(&mut self) -> (i64, u64) {
        (self.below(13) as i64, 1 + self.below(25))
    }
}

/// Deterministically generated parameter timelines: up to three activations,
/// each shortening or lengthening the cadence, changing only the rate,
/// repeating the parameters unchanged, or removing the operation; initial
/// values on both sides of the baseline (decay and recovery); evaluation
/// subsets at unaligned times, restricted to epochs that carry the operation.
fn scenarios() -> Vec<Scenario> {
    let mut g = Lcg(0x5EED_C2E2);
    let mut out = Vec::new();
    while out.len() < 40 {
        let initial = g.below(241) as i64 - 120;
        let genesis = g.parameters();
        let mut times = BTreeSet::new();
        let count = g.below(4) as usize;
        while times.len() < count {
            times.insert(5 + g.below(86));
        }
        let mut epochs = Vec::new();
        let mut previous = Some(genesis);
        for t in times {
            let p = match g.below(6) {
                0 => None,
                1 if previous.is_some() => previous,
                2 if previous.is_some() => previous.map(|(r, _)| (r, 1 + g.below(25))),
                3 if previous.is_some() => previous.map(|(_, c)| (g.below(13) as i64, c)),
                _ => Some(g.parameters()),
            };
            epochs.push((t, p));
            previous = p;
        }
        let mut s = Scenario {
            initial,
            genesis,
            epochs,
            evals: Vec::new(),
        };
        let mut evals = Vec::new();
        for t in 1..=100 {
            if (g.below(7) == 0 || t == 100) && in_effect(&s, t).is_some() {
                evals.push(t);
            }
        }
        if evals.is_empty() {
            continue;
        }
        s.evals = evals;
        out.push(s);
    }
    out
}

// ================================================================ C2R-01

/// C2R-01 converted and generalized: for every single-evaluation time, the
/// committed cell is `(value, canonical time)` and the reported resolved
/// effects, reapplied to an untouched clone at the frozen canonical time
/// (FINAL §4 `at`), reproduce the engine state exactly (AT-I1).
/// Kills: the backdated commit (`updated_at` = end of the last whole step,
/// e.g. `(90, 10)` at 15), whose digest differs from the reapplication.
#[test]
fn c2r_01_decay_reapplied_at_canonical_time_reproduces_the_state() {
    for t in 1..=35u64 {
        let mut e = literal_decay(&[t], 100, 10, 10);
        let mut clone = e.clone();
        let r = run(&mut e, t);
        assert_eq!(r[0].canonical_time, LogicalTime(t));
        assert_eq!(stress(&e), (100 - 10 * (t / 10) as i64, LogicalTime(t)));
        fixture::extract_least_due_slice(&mut clone, LogicalTime(t)).unwrap();
        fixture::apply_committed_effects(&mut clone, &r[0].waves[0].committed, LogicalTime(t));
        assert_eq!(
            e.engine_state_digest(),
            clone.engine_state_digest(),
            "t={t}"
        );
    }
}

// ================================================================ C2R-02

/// C2R-02 converted: value 100; rate 1 / cadence 100; rate 5 / cadence 10
/// activated at 50; decay at 60. Only the new segment's step ending at 60 may
/// use rate 5: 95. Kills: the carried partial step counted on the new cadence
/// from the old anchor (steps ending 10 … 60 → 70), and the current rate over
/// the whole span (40).
#[test]
fn c2r_02_shortened_cadence_bills_only_steps_after_the_barrier_converted() {
    let base = Scenario {
        initial: 100,
        genesis: (1, 100),
        epochs: vec![(50, Some((5, 10)))],
        evals: vec![],
    };
    for (evals, expected) in [
        (vec![60], (95, 60)),
        (vec![50, 60], (95, 60)),
        (vec![55, 60], (95, 60)),
        (vec![60, 70], (90, 70)),
        (vec![70], (90, 70)),
    ] {
        let s = Scenario {
            evals: evals.clone(),
            ..base.clone()
        };
        let r = run_scenario(&s, 50, &evals, None);
        assert_eq!(
            stress(&r.engine),
            (expected.0, LogicalTime(expected.1)),
            "{evals:?}"
        );
        assert_eq!(model(&s, &evals), stress(&r.engine));
    }
}

/// C2R-02 lengthening and the residual interval: rate 1 / cadence 10, then
/// rate 5 / cadence 100 activated at 55. The old segment bills its five whole
/// steps (95); its residual `(50, 55]` is never billed; the new segment's first
/// step ends at 155. Kills: an absolute grid from 0 (a new-rate step at 100,
/// covering pre-barrier time → 90 at 150) and the carried anchor (old anchor
/// 50 + 100 = 150 → 90 at 150). Recovery from −100 mirrors it exactly.
#[test]
fn c2r_02_lengthened_cadence_and_residual_interval() {
    for sign in [1i64, -1] {
        let base = Scenario {
            initial: 100 * sign,
            genesis: (1, 10),
            epochs: vec![(55, Some((5, 100)))],
            evals: vec![],
        };
        for (evals, expected) in [
            (vec![150], (95, 150)),
            (vec![50, 100, 150], (95, 150)),
            (vec![100], (95, 100)),
            (vec![150, 155], (90, 155)),
            (vec![155], (90, 155)),
            (vec![254, 255], (85, 255)),
        ] {
            let s = Scenario {
                evals: evals.clone(),
                ..base.clone()
            };
            let r = run_scenario(&s, 50, &evals, None);
            assert_eq!(
                stress(&r.engine),
                (expected.0 * sign, LogicalTime(expected.1)),
                "{sign} {evals:?}"
            );
        }
    }
}

/// C2R-02: a rate-only change at an unaligned barrier (recovery direction):
/// from −100 at rate 1 / cadence 10, rate 5 activated at 55: −95 at 60 (no
/// new-segment step yet), −90 at 65. Kills: billing the step `(50, 60]` at the
/// new rate (−90 at 60).
#[test]
fn c2r_02_rate_only_change_at_an_unaligned_barrier() {
    let base = Scenario {
        initial: -100,
        genesis: (1, 10),
        epochs: vec![(55, Some((5, 10)))],
        evals: vec![],
    };
    for (evals, expected) in [
        (vec![60], (-95, 60)),
        (vec![60, 65], (-90, 65)),
        (vec![65], (-90, 65)),
    ] {
        let s = Scenario {
            evals: evals.clone(),
            ..base.clone()
        };
        let r = run_scenario(&s, 50, &evals, None);
        assert_eq!(stress(&r.engine), (expected.0, LogicalTime(expected.1)));
    }
}

/// C2R-02: an activation that leaves the operation's resolved parameters
/// unchanged does not move the grid: six steps by 60 (94). Kills: restarting
/// the grid at every activation (95).
#[test]
fn c2r_02_unchanged_parameters_do_not_move_the_grid() {
    let s = Scenario {
        initial: 100,
        genesis: (1, 10),
        epochs: vec![(55, Some((1, 10)))],
        evals: vec![60],
    };
    let r = run_scenario(&s, 50, &[60], None);
    assert_eq!(r.engine.behavior_epoch(), 2);
    assert_eq!(stress(&r.engine), (94, LogicalTime(60)));
}

/// C2R-02/03 zero-rate transitions and an interval without the operation:
/// rate 10 → 0 at 30 → 10 at 60 (cadence 10): 70 by 30, nothing in `(30, 60]`,
/// the new segment's first step at 70 → 60. And rate 10, operation absent
/// from 30 to 45, restored at 45: the step at 30 belongs to the closing
/// segment, then 55 → 60. Every partition agrees in value and commit time.
#[test]
fn c2r_02_zero_rate_and_absent_intervals() {
    let zero = Scenario {
        initial: 100,
        genesis: (10, 10),
        epochs: vec![(30, Some((0, 10))), (60, Some((10, 10)))],
        evals: vec![],
    };
    let absent = Scenario {
        initial: 100,
        genesis: (10, 10),
        epochs: vec![(30, None), (45, Some((10, 10)))],
        evals: vec![],
    };
    for (base, partitions) in [
        (
            zero,
            vec![vec![75], vec![25, 45, 62, 75], vec![30, 60, 70, 75]],
        ),
        (absent, vec![vec![60], vec![20, 60], vec![30, 55, 60]]),
    ] {
        for evals in partitions {
            let s = Scenario {
                evals: evals.clone(),
                ..base.clone()
            };
            let r = run_scenario(&s, 50, &evals, None);
            assert_eq!(stress(&r.engine), (60, LogicalTime(*evals.last().unwrap())));
            assert_eq!(model(&s, &evals), stress(&r.engine));
        }
    }
}

/// C2R-02 replay and recovery: the reviewer's fixture reconstructed from
/// history, and snapshot/restore after the activation then continued, are
/// digest-identical to the uninterrupted run.
#[test]
fn c2r_02_barrier_semantics_survive_replay_and_restore() {
    let s = Scenario {
        initial: 100,
        genesis: (1, 100),
        epochs: vec![(50, Some((5, 10)))],
        evals: vec![45, 60, 75],
    };
    let r = run_scenario(&s, 50, &s.evals, None);
    // 45: no step on the cadence-100 grid; 60: the new segment's step at 60
    // (95); 75: its step at 70 (90).
    assert_eq!(stress(&r.engine), (90, LogicalTime(75)));
    assert_eq!(model(&s, &s.evals), stress(&r.engine));
    let restored = run_scenario(&s, 50, &s.evals, Some(1));
    assert_eq!(digest_pair(&restored.engine), digest_pair(&r.engine));
    let history = CompletedHistory {
        commands: r.commands.clone(),
        resets: vec![],
        frontier: r.engine.frontier(),
        recorded_history_digest: r.engine.timeline_history_digest(),
        recorded_stable_boundary_digest: r.engine.stable_boundary_digest(),
    };
    let replayed = Engine::reconstruct_completed(r.genesis.clone(), &history).unwrap();
    assert_eq!(digest_pair(&replayed), digest_pair(&r.engine));
}

// ================================================================ C2R-03

/// C2R-03 converted and extended: saturation at the baseline preserves value
/// **and** commit time under every evaluation subset (decay from 100 and
/// recovery from −100). Kills: emitting nothing when the value does not move
/// (`updated_at` = time of the last moving evaluation: `(0, 100)` vs
/// `(0, 150)`).
#[test]
fn c2r_03_saturation_preserves_value_and_commit_time_chunk_invariance() {
    let grid = [23u64, 50, 77, 100, 123];
    for initial in [100, -100] {
        let mut one = literal_decay(&[150], initial, 10, 10);
        run(&mut one, 150);
        assert_eq!(stress(&one), (0, LogicalTime(150)));
        for mask in 0u32..(1 << grid.len()) {
            let mut times: Vec<u64> = grid
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, t)| *t)
                .collect();
            times.push(150);
            let mut e = literal_decay(&times, initial, 10, 10);
            run(&mut e, 150);
            assert_eq!(stress(&e), stress(&one), "{initial} {times:?}");
        }
    }
}

/// C2R-03: every decay evaluation of an existing cell commits its value at the
/// canonical time — no whole step elapsed, rate zero, or at the baseline — and
/// such a commit is an ordinary write: a change watcher on the target fires
/// for it exactly as for an equal-valued assignment (documented consequence).
#[test]
fn c2r_03_every_evaluation_commits_at_its_canonical_time() {
    for (start, rate, t) in [(100, 10, 7), (100, 0, 25), (0, 10, 25)] {
        let mut f = standard();
        let seed = on_command(
            "rule.seed",
            "cmd.seed",
            vec![emit("s", "state.stress", Update::Assign(lit(start)))],
        );
        let d = work_rule(
            "rule.d",
            "work.d",
            vec![emit("d", "state.stress", decay(rate, 10))],
        );
        let watcher = rule(
            "rule.w",
            Trigger::Change {
                watched: def("state.stress"),
            },
            vec![emit("e", "state.echo", add(1))],
        );
        let mut e = Engine::genesis(f.genesis_with(
            budgets(50),
            vec![seed, d, watcher],
            vec![baseline("state.stress", 0)],
            vec![initial("rule.d", &a(), t, "work.d")],
        ))
        .unwrap();
        drive(&mut e, &command("cmd.seed", 0, 1, "cmd.seed", a()));
        let before_echo = value(&e, "state.echo", &a());
        let r = run(&mut e, t);
        assert_eq!(r[0].waves[0].committed.len(), 1);
        assert_eq!(stress(&e), (start, LogicalTime(t)));
        assert_eq!(
            value(&e, "state.echo", &a()),
            Some(before_echo.unwrap_or(0) + 1)
        );
    }
}

/// C2R-02/03 generalized: the reference model over 40 generated parameter
/// timelines. For each: the chunked evaluation history and the single
/// catch-up both equal the model's `(value, commit time)`; a pacing budget of
/// one gives identical engine and stable-boundary digests; history
/// reconstruction and snapshot/restore after the first activation are
/// digest-identical. (Engine digests are compared only between runs of one
/// scheduled history; histories with different evaluation `WorkKey`s differ
/// legitimately in scheduler, ledger and provenance content.)
#[test]
fn c2r_02_03_grid_matches_the_reference_model_under_every_partition() {
    let all = scenarios();
    let mut shapes = BTreeSet::new();
    for (i, s) in all.iter().enumerate() {
        shapes.insert(s.epochs.len());
        let expected = model(s, &s.evals);
        let chunked = run_scenario(s, 50, &s.evals, None);
        assert_eq!(stress(&chunked.engine), expected, "scenario {i}: {s:?}");
        let last = *s.evals.last().unwrap();
        let single = run_scenario(s, 50, &[last], None);
        assert_eq!(stress(&single.engine), expected, "scenario {i}: catch-up");
        let paced = run_scenario(s, 1, &s.evals, None);
        assert_eq!(
            digest_pair(&paced.engine),
            digest_pair(&chunked.engine),
            "scenario {i}: pacing"
        );
        let history = CompletedHistory {
            commands: chunked.commands.clone(),
            resets: vec![],
            frontier: chunked.engine.frontier(),
            recorded_history_digest: chunked.engine.timeline_history_digest(),
            recorded_stable_boundary_digest: chunked.engine.stable_boundary_digest(),
        };
        let replayed = Engine::reconstruct_completed(chunked.genesis.clone(), &history).unwrap();
        assert_eq!(digest_pair(&replayed), digest_pair(&chunked.engine));
        if !s.epochs.is_empty() {
            let restored = run_scenario(s, 50, &s.evals, Some(1));
            assert_eq!(
                digest_pair(&restored.engine),
                digest_pair(&chunked.engine),
                "scenario {i}: restore"
            );
        }
    }
    assert_eq!(shapes, BTreeSet::from([0, 1, 2, 3]), "all timeline shapes");
}

// ================================================================ AT-I8

/// AT-I8 per committed wave: a three-wave cohort whose waves 0 and 1 enqueue
/// obligations. After **each** committed wave — the intermediate points the
/// whole-cohort check cannot see — the scheduler ↔ ObligationStore invariant
/// holds and both stores hold exactly the expected work. Kills: deferring the
/// obligation-store insertion to the end of the cohort (the whole-cohort check
/// still passes).
#[test]
fn at_i8_invariant_holds_after_each_intermediate_committed_wave() {
    let rules = vec![
        with_schedule(
            adder("rule.a", "work.one", "state.stress", 10),
            reevaluate_after("f", 10, "work.f", "rule.f"),
        ),
        with_schedule(
            rule(
                "rule.w",
                Trigger::Change {
                    watched: def("state.stress"),
                },
                vec![emit("m", "state.mood", add(1))],
            ),
            reevaluate_after("g", 20, "work.g", "rule.g"),
        ),
        rule(
            "rule.x",
            Trigger::Change {
                watched: def("state.mood"),
            },
            vec![emit("e", "state.echo", add(1))],
        ),
        adder("rule.f", "work.f", "state.energy", 1),
        adder("rule.g", "work.g", "state.energy", 1),
        adder("rule.u", "work.u", "state.energy", 1),
    ];
    let mut f = standard();
    let mut e = f.engine(
        budgets(50),
        rules,
        vec![
            initial("rule.a", &a(), 100, "work.one"),
            initial("rule.u", &a(), 200, "work.u"),
        ],
    );
    fixture::probe_wave_invariants(&mut e, true);
    let r = run(&mut e, 100);
    assert_eq!(r[0].waves.len(), 3);
    let points = &fixture::observation(&e).committed_waves;
    assert_eq!(
        points
            .iter()
            .map(|p| (p.wave_index, p.obligation_count, p.scheduled_work_count))
            .collect::<Vec<_>>(),
        vec![(0, 2, 2), (1, 3, 3), (2, 3, 3)]
    );
    assert!(points.iter().all(|p| p.bidirectional_invariant));
    let cohort = &points[0].cohort_identity;
    assert!(points.iter().all(|p| &p.cohort_identity == cohort));
    run(&mut e, 200);
    assert!(fixture::observation(&e)
        .committed_waves
        .iter()
        .all(|p| p.bidirectional_invariant));
}

// ================================================================ AT-I20b

/// AT-I20b: a cohort over each semantic cap (effects, candidates, enqueues)
/// rejects with no retained component moved by the wave, consumes its keys
/// terminally, replays identically, and **the rejected producer** rescheduled
/// under its next occurrence (same producer, scope and work kind, occurrence
/// `n + 1`) succeeds. Kills: permanently disabling the overloaded producer
/// (tombstoning its occurrence sequence), reusing the rejected occurrence, and
/// leaving the keys resident.
#[test]
fn at_i20b_rejected_producer_retries_under_its_next_occurrence() {
    for cap in [
        "max_effects_per_wave",
        "max_cohort_candidates",
        "max_enqueue_per_wave",
    ] {
        let enqueue = cap == "max_enqueue_per_wave";
        let (wide, extra) = if enqueue {
            let mut w = work_rule("rule.wide", "work.wide", vec![]);
            w.schedules
                .push(reevaluate_after("s1", 5, "work.s1", "rule.sink"));
            w.schedules
                .push(reevaluate_after("s2", 5, "work.s2", "rule.sink"));
            let mut x = work_rule("rule.extra", "work.extra", vec![]);
            x.schedules
                .push(reevaluate_after("s3", 5, "work.s3", "rule.sink"));
            (w, x)
        } else {
            (
                work_rule(
                    "rule.wide",
                    "work.wide",
                    vec![
                        emit("s", "state.stress", add(1)),
                        emit("m", "state.mood", add(1)),
                    ],
                ),
                adder("rule.extra", "work.extra", "state.energy", 1),
            )
        };
        let arm = with_schedule(
            on_command("rule.arm", "cmd.arm", vec![]),
            reevaluate_after("w", 1, "work.wide", "rule.wide"),
        );
        let sink = adder("rule.sink", "work.s1", "state.echo", 1);
        let mut b = budgets(8);
        match cap {
            "max_effects_per_wave" => b.max_effects_per_wave = 2,
            "max_cohort_candidates" => b.max_cohort_candidates = 2,
            _ => b.max_enqueue_per_wave = 2,
        }
        let build = || {
            let mut f = standard();
            let mut e = f.engine(
                b,
                vec![arm.clone(), wide.clone(), extra.clone(), sink.clone()],
                vec![initial("rule.extra", &a(), 11, "work.extra")],
            );
            drive(&mut e, &command("cmd.arm", 10, 1, "cmd.arm", a()));
            e
        };
        let mut e = build();
        let rejected_key = e
            .obligations()
            .keys()
            .find(|k| k.producer_definition_id == def("rule.arm"))
            .unwrap()
            .clone();
        let before = components(&e);
        fixture::clear_observation(&mut e);
        let r = run(&mut e, 11);
        assert!(
            matches!(
                &r[0].outcome,
                CohortOutcome::Rejected {
                    wave: 0,
                    rejection: WaveRejection::SemanticCap { cap: c, observed: 3, bound: 2 }
                } if *c == cap
            ),
            "{cap}: {:?}",
            r[0].outcome
        );
        // Every retained component equals the post-extraction pre-wave state.
        assert_eq!(
            &e.engine_state_digest(),
            fixture::observation(&e).prewave[0].engine_digest.value(),
            "{cap}"
        );
        let after = components(&e);
        for i in [0usize, 2, 3, 5, 6] {
            assert_eq!(before[i], after[i], "{cap}: component {i}");
        }
        assert_eq!(e.scheduled_work_count(), 0, "{cap}: consumed terminally");
        assert_eq!(e.obligations().keys().count(), 0, "{cap}");
        let mut replay = build();
        assert_eq!(run(&mut replay, 11), r, "{cap}: replay");
        assert_eq!(digest_pair(&replay), digest_pair(&e));
        // Retry: the same producer, next occurrence.
        drive(&mut e, &command("cmd.arm.2", 12, 2, "cmd.arm", a()));
        let retry = e.obligations().keys().next().unwrap().clone();
        assert_eq!(
            retry,
            WorkKey {
                due_time: LogicalTime(13),
                occurrence_index: OccurrenceIndex(rejected_key.occurrence_index.0 + 1),
                ..rejected_key.clone()
            },
            "{cap}"
        );
        let r = run(&mut e, 13);
        assert_eq!(
            r[0].outcome,
            CohortOutcome::Committed,
            "{cap}: retry succeeds"
        );
        if enqueue {
            assert_eq!(e.obligations().keys().count(), 2);
        } else {
            assert_eq!(value(&e, "state.stress", &a()), Some(1));
            assert_eq!(value(&e, "state.mood", &a()), Some(1));
        }
    }
}

// ================================================================ AT-I21

fn interference_rules() -> Vec<RuleSpec> {
    vec![
        on_command(
            "rule.seed",
            "cmd.seed",
            vec![emit("m", "state.mood", Update::Assign(lit(5)))],
        ),
        on_command(
            "rule.shift",
            "cmd.shift",
            vec![emit("m", "state.mood", Update::Assign(lit(20)))],
        ),
        adder("rule.s", "work.one", "state.stress", 1),
        adder("rule.i", "work.i", "state.mood", 7),
        rule(
            "rule.c",
            Trigger::Change {
                watched: def("state.stress"),
            },
            vec![emit(
                "e",
                "state.echo",
                Update::Assign(Expr::Input(cell("state.mood"))),
            )],
        ),
        adder("rule.late", "work.late", "state.energy", 1),
    ]
}

/// The interference engine with `max_wave_depth = 0` (through the depth seam,
/// since sound validation precludes overflow) or a sufficient depth.
fn interference_engine(depth_zero: bool, init: Vec<(&str, u64, &str)>) -> Engine {
    let mut f = standard();
    let rules = interference_rules();
    let mut g = f.genesis(
        budgets(8),
        rules.clone(),
        init.into_iter()
            .map(|(r, t, k)| initial(r, &a(), t, k))
            .collect(),
    );
    if depth_zero {
        let mut shallow = budgets(8);
        shallow.max_wave_depth = 0;
        g.rule_set = f
            .registry
            .activate_rule_set_without_depth_bound(
                &f.profile,
                &RuleSetSpec {
                    profile_id: profile_id(),
                    budgets: shallow,
                    rules,
                    baselines: vec![],
                },
            )
            .unwrap();
    }
    let mut e = Engine::genesis(g).unwrap();
    drive(&mut e, &command("cmd.seed", 1, 1, "cmd.seed", a()));
    e
}

/// AT-I21 value-dependent interference. The deferred rule `rule.c` writes the
/// value of `state.mood` it reads, so its result discriminates which state the
/// continuation evaluates against.
///
/// (a) Interfering due work at the continuation's due time 101 (`mood += 7`)
/// shares the continuation's cohort and pre-wave snapshot: echo = 5, mood = 12.
/// Kills: evaluating the continuation after the same-time interference (12).
///
/// (b) An interfering command at 100 commits between deferral and
/// continuation (`mood := 20`): the continuation re-evaluates against current
/// committed state, echo = 20. Kills: capturing the input at deferral (5). The
/// larger-depth run evaluates `rule.c` inside cohort 100 (echo 5), so the
/// converged states legitimately differ (adjudicated v2 §2 C-11: no
/// cross-budget equality under interference). Both variants are deterministic
/// and keep exact priority and identity.
#[test]
fn at_i21_value_dependent_interference() {
    // (a)
    let build_a = || {
        interference_engine(
            true,
            vec![
                ("rule.s", 100, "work.one"),
                ("rule.i", 101, "work.i"),
                ("rule.late", 102, "work.late"),
            ],
        )
    };
    let mut e = build_a();
    let interfering = e
        .obligations()
        .keys()
        .find(|k| k.due_time == LogicalTime(101))
        .unwrap()
        .clone();
    fixture::clear_observation(&mut e);
    let r = run(&mut e, 102);
    assert_eq!(
        r.iter().map(|c| c.canonical_time.0).collect::<Vec<_>>(),
        vec![100, 101, 102]
    );
    let converted = r[0].waves[0].depth_conversions[0].clone();
    assert_eq!(converted.due_time, LogicalTime(101));
    let at_101 = fixture::observation(&e).prewave[1].cohort_identity.clone();
    assert_eq!(
        at_101,
        scheduled_cohort_identity(&profile_id(), LogicalTime(101), &[converted, interfering])
    );
    assert_eq!(
        value(&e, "state.echo", &a()),
        Some(5),
        "pre-wave snapshot read"
    );
    assert_eq!(value(&e, "state.mood", &a()), Some(12));
    let mut again = build_a();
    assert_eq!(run(&mut again, 102), r);
    assert_eq!(digest_pair(&again), digest_pair(&e));

    // (b)
    let build_b = |depth_zero: bool| {
        let mut e = interference_engine(
            depth_zero,
            vec![("rule.s", 100, "work.one"), ("rule.late", 102, "work.late")],
        );
        let first = reports(&drive(
            &mut e,
            &command("cmd.shift", 100, 2, "cmd.shift", a()),
        ));
        let rest = run(&mut e, 102);
        (e, first, rest)
    };
    let (e, first, rest) = build_b(true);
    assert_eq!(first.len(), 2, "cohort 100, then the command at 100");
    assert_eq!(first[0].waves[0].depth_conversions.len(), 1);
    assert_eq!(
        rest.iter().map(|c| c.canonical_time.0).collect::<Vec<_>>(),
        vec![101, 102]
    );
    assert_eq!(
        value(&e, "state.echo", &a()),
        Some(20),
        "current value read"
    );
    let (again, first2, rest2) = build_b(true);
    assert_eq!((first2, rest2), (first, rest));
    assert_eq!(digest_pair(&again), digest_pair(&e));
    let (deep, _, _) = build_b(false);
    assert_eq!(value(&deep, "state.echo", &a()), Some(5));
    assert_ne!(deep.engine_state_digest(), e.engine_state_digest());
}

// ================================================================ AT-I28

/// AT-I28 after every prefix of a scripted sequence — command, paused
/// boundary, resumed request, epoch activation, refused finalization, timeline
/// epoch reset, activation under the new timeline epoch, command, advance —
/// both derived indexes (the timeline's finalized-history indexes and the
/// engine's epoch lineage) equal their recomputation from committed state, the
/// bidirectional invariant holds, and snapshot/restore is exact. At every
/// prefix, each injected timeline-index fault and each lineage-time fault is
/// detected by recomputation and refused by restore. Kills: a validator that
/// trusts the stored index (the injections would pass).
#[test]
fn at_i28_derived_indexes_recompute_after_every_prefix_and_faults_are_detected() {
    let mut f = standard();
    let rules = vec![
        with_schedule(
            adder("rule.w", "work.one", "state.stress", 1),
            reevaluate_after("n", 10, "work.one", "rule.w"),
        ),
        adder("rule.v", "work.two", "state.mood", 1),
        on_command("rule.cmd", "cmd.x", vec![emit("c", "state.echo", add(1))]),
    ];
    let g = f.genesis(
        budgets(1),
        rules,
        vec![
            initial("rule.w", &a(), 5, "work.one"),
            initial("rule.v", &a(), 6, "work.two"),
        ],
    );
    let profile = g.profile.clone();
    let rs = g.rule_set.clone();
    let mut e = Engine::genesis(g).unwrap();
    let mut prefixes = 0;
    let mut check = |e: &Engine, step: &str| {
        prefixes += 1;
        assert!(fixture::timeline_indexes_recompute(e), "{step}");
        assert!(fixture::lineage_recomputes(e), "{step}");
        assert!(e.bidirectional_invariant_holds(), "{step}");
        let s = e.snapshot().unwrap();
        let restored = Engine::restore(s.clone(), &profile).unwrap();
        assert_eq!(
            restored.stable_boundary_digest(),
            e.stable_boundary_digest()
        );
        assert_eq!(fixture::lineage_len(&restored), fixture::lineage_len(e));
        if let Some(fc) = e.finalized_commands().last() {
            let faults = [
                DerivedIndexFault::DropFinalizedCommandIdentity(fc.envelope.command_id.clone()),
                DerivedIndexFault::DropFinalizedSourceSequence(
                    fc.envelope.source_id.clone(),
                    fc.envelope.source_sequence,
                ),
                DerivedIndexFault::SetLastFinalizedSourceSequence(
                    fc.envelope.source_id.clone(),
                    fc.envelope.source_sequence + 7,
                ),
            ];
            for fault in faults {
                let mut bad = s.clone();
                fixture::snapshot_inject_timeline_index_fault(&mut bad, fault.clone());
                assert_eq!(
                    Engine::restore(bad, &profile).unwrap_err(),
                    RestoreError::DerivedIndexesInconsistent,
                    "{step}: {fault:?}"
                );
                let mut live = e.clone();
                fixture::inject_timeline_index_fault(&mut live, fault);
                assert!(!fixture::timeline_indexes_recompute(&live), "{step}");
            }
        }
        for index in 0..fixture::lineage_len(e) {
            let mut bad = s.clone();
            fixture::snapshot_set_activation_time(&mut bad, index, Some(LogicalTime(999)));
            assert_eq!(
                Engine::restore(bad, &profile).unwrap_err(),
                RestoreError::EpochLineageInvalid,
                "{step}: lineage {index}"
            );
        }
    };
    check(&e, "genesis");
    drive(&mut e, &command("cmd.a", 1, 1, "cmd.x", a()));
    check(&e, "command");
    assert_eq!(e.process(&advance(20)).outcome(), &Outcome::Paused);
    check(&e, "paused boundary");
    drive(&mut e, &advance(20));
    check(&e, "resumed");
    let extra = config(vec![("tune.other", CanonicalValue::Int(3))]);
    drive(
        &mut e,
        &Request::Command(activation_command("cmd.epoch2", 25, 2, rs.clone(), extra)),
    );
    assert_eq!(e.behavior_epoch(), 2);
    check(&e, "activation");
    let refused = drive(&mut e, &command("cmd.b", 26, 1, "cmd.x", a()));
    assert!(matches!(
        refused.last().unwrap().outcome(),
        Outcome::CompletedCommandNotFinalized(_)
    ));
    check(&e, "refused finalization");
    e.reset_timeline_epoch(TimelineEpoch(1), SourceId::new("seq.two").unwrap())
        .unwrap();
    check(&e, "timeline reset");
    let mut c = activation_command("cmd.epoch3", 30, 3, rs, config(vec![]));
    c.timeline_epoch = TimelineEpoch(1);
    drive(&mut e, &Request::Command(c));
    assert_eq!(e.behavior_epoch(), 3);
    check(&e, "activation after reset");
    let mut c = command_request("cmd.c", 40, 4, "cmd.x", a(), vec![], vec![]);
    c.timeline_epoch = TimelineEpoch(1);
    drive(&mut e, &Request::Command(c));
    check(&e, "command after reset");
    drive(&mut e, &advance(60));
    check(&e, "advance");
    assert_eq!(prefixes, 10);
}

// ================================================================ AT-I33

fn chain_manifest(levels: usize) -> ProfileManifest {
    ProfileManifest::new(
        profile_id(),
        BoundedText::new("depth.chain").unwrap(),
        (0..=levels)
            .map(|k| int_spec(&format!("state.c{k}"), Authority::SparkOwned, -1_000, 1_000))
            .collect(),
    )
}

/// Wave-0 writers `w0a`, `w0b` to `c0`; at each level `k`, two watchers of
/// `c(k−1)` write `ck`, so every derived seed has exactly two parents.
fn chain_rules(levels: usize) -> Vec<RuleSpec> {
    let mut rules = vec![
        adder("rule.w0a", "work.k0a", "state.c0", 1),
        adder("rule.w0b", "work.k0b", "state.c0", 1),
    ];
    for k in 1..=levels {
        for side in ["a", "b"] {
            rules.push(rule(
                &format!("rule.{side}{k}"),
                Trigger::Change {
                    watched: def(&format!("state.c{}", k - 1)),
                },
                vec![emit("x", &format!("state.c{k}"), add(1))],
            ));
        }
    }
    rules
}

fn formula(
    cohort: &Digest,
    wave: u32,
    parent: &Digest,
    rule_fp: &Digest,
    target: &str,
    artifact: &Digest,
) -> Digest {
    let mut e = CanonicalEncoder::new();
    e.push_str("emission");
    e.push_digest(cohort);
    e.push_u32(wave);
    e.push_digest(parent);
    e.push_digest(rule_fp);
    e.push_u64(1);
    tag("x").canonicalize(&mut e);
    a().canonicalize(&mut e);
    def(target).canonicalize(&mut e);
    a().canonicalize(&mut e);
    e.push_digest(artifact);
    e.finish()
}

fn parents_of(ids: &[Digest]) -> Digest {
    let mut p = ids.to_vec();
    p.sort();
    p.dedup();
    let mut e = CanonicalEncoder::new();
    e.push_str("parents_emission_v1");
    e.push_u64(p.len() as u64);
    for x in p {
        e.push_bytes(x.as_bytes());
    }
    e.finish()
}

/// AT-I33 maximal configured depth: a multi-parent chain exactly as long as
/// the configured `max_wave_depth` (16) activates, evaluates waves 0 … 16 with
/// two parents at every level, and every level's emission identities equal the
/// frozen formula over the complete two-identity parent set of the level
/// before. One level more is rejected statically; through the depth seam it
/// converts both pending seeds to strictly later work, typed, and they run at
/// the next due time. The `u32::MAX` depth bound is total. No panic anywhere.
/// Kills: truncating or selecting parents at depth, or an unchecked wave index.
#[test]
fn at_i33_maximal_configured_depth_multi_parent_chain() {
    const D: usize = 16;
    let mut b = budgets(8);
    b.max_wave_depth = D as u32;
    let mut f = activate(&chain_manifest(D + 1));
    let init = || {
        vec![
            initial("rule.w0a", &a(), 100, "work.k0a"),
            initial("rule.w0b", &a(), 100, "work.k0b"),
        ]
    };
    let mut e = f.engine(b, chain_rules(D), init());
    let keys: Vec<WorkKey> = e.obligations().keys().cloned().collect();
    fixture::clear_observation(&mut e);
    let r = run(&mut e, 100);
    assert_eq!(r[0].outcome, CohortOutcome::Committed);
    assert_eq!(r[0].waves.len(), D + 1);
    let cohort = fixture::observation(&e).prewave[0].cohort_identity.clone();
    let artifact = e.epoch_registry().current().unwrap().record_hash();
    let fp = |id: &str| e.rule_set().rule(&def(id)).unwrap().fingerprint().clone();
    let mut previous: Vec<Digest> = keys
        .iter()
        .map(|k| {
            let mut p = CanonicalEncoder::new();
            p.push_str("parents_workkey_v1");
            p.push_digest(&k.identity_digest());
            formula(
                &cohort,
                0,
                &p.finish(),
                &fp(k.producer_definition_id.as_str()),
                "state.c0",
                &artifact,
            )
        })
        .collect();
    previous.sort();
    for k in 0..=D {
        let wave = &r[0].waves[k];
        assert_eq!(wave.committed.len(), 1, "one write to c{k}");
        assert_eq!(value(&e, &format!("state.c{k}"), &a()), Some(2));
        let actual = wave.committed[0].provenance.retained.clone();
        if k > 0 {
            let parent = parents_of(&previous);
            let mut expected: Vec<Digest> = ["a", "b"]
                .iter()
                .map(|s| {
                    formula(
                        &cohort,
                        k as u32,
                        &parent,
                        &fp(&format!("rule.{s}{k}")),
                        &format!("state.c{k}"),
                        &artifact,
                    )
                })
                .collect();
            expected.sort();
            assert_eq!(actual, expected, "level {k}");
        } else {
            assert_eq!(actual, previous, "level 0");
        }
        previous = actual;
    }
    // One level more: static rejection at the door.
    let mut f = activate(&chain_manifest(D + 1));
    assert!(f
        .rule_set(b, chain_rules(D + 1))
        .unwrap_err()
        .iter()
        .any(|x| matches!(x, RuleSetError::WaveDepthExceeded { .. })));
    // Through the depth seam: typed conversion, strictly later, then executed.
    let mut f = activate(&chain_manifest(D + 1));
    let rules = chain_rules(D + 1);
    let mut g = f.genesis(budgets(8), vec![], init());
    g.rule_set = f
        .registry
        .activate_rule_set_without_depth_bound(
            &f.profile,
            &RuleSetSpec {
                profile_id: profile_id(),
                budgets: b,
                rules,
                baselines: vec![],
            },
        )
        .unwrap();
    let mut e = Engine::genesis(g).unwrap();
    let r = run(&mut e, 100);
    assert_eq!(r[0].waves.len(), D + 1);
    let conversions = &r[0].waves[D].depth_conversions;
    assert_eq!(conversions.len(), 2);
    assert!(conversions.iter().all(|k| k.due_time == LogicalTime(101)));
    assert_eq!(value(&e, &format!("state.c{}", D + 1), &a()), None);
    run(&mut e, 101);
    assert_eq!(value(&e, &format!("state.c{}", D + 1), &a()), Some(2));
    // The extreme declared bound is total.
    let mut wide = budgets(8);
    wide.max_wave_depth = u32::MAX;
    let mut f = activate(&chain_manifest(D + 1));
    let mut e = f.engine(wide, chain_rules(D), init());
    assert_eq!(run(&mut e, 100)[0].outcome, CohortOutcome::Committed);
}

// ================================================================ AT-I39 / AT-I40

fn pid(name: &str) -> ProfileId {
    ProfileId::new(name).unwrap()
}

/// A profile's manifest; `extra` definitions exist only in that profile.
fn member_manifest(p: &ProfileId, extra: &[&str]) -> ProfileManifest {
    let specs = ["state.stress", "state.echo", "state.mood"]
        .iter()
        .chain(extra.iter())
        .map(|d| {
            let mut s = int_spec(d, Authority::SparkOwned, -1_000_000, 1_000_000);
            s.profile_id = p.clone();
            s
        })
        .collect();
    ProfileManifest::new(p.clone(), BoundedText::new("composed").unwrap(), specs)
}

fn member_rules() -> Vec<RuleSpec> {
    vec![
        adder("rule.a", "work.one", "state.stress", 30),
        adder("rule.b", "work.two", "state.stress", 15),
        with_schedule(
            rule(
                "rule.alarm",
                Trigger::Crossing {
                    watched: def("state.stress"),
                    threshold: 40,
                    direction: Direction::Rising,
                },
                vec![emit("ring", "state.echo", add(1))],
            ),
            reevaluate_after("follow", 10, "work.follow", "rule.follow"),
        ),
        adder("rule.follow", "work.follow", "state.mood", 1),
    ]
}

fn member_work(name: &str) -> Vec<(&'static str, u64, &'static str)> {
    if name == "world-r" {
        vec![("rule.a", 100, "work.one"), ("rule.a", 105, "work.one")]
    } else {
        vec![("rule.a", 100, "work.one"), ("rule.b", 100, "work.two")]
    }
}

/// One member's per-profile evidence.
#[derive(Debug, PartialEq, Eq)]
struct Member {
    reports: Vec<CohortReport>,
    digests: (Digest, Digest),
    cohorts: Vec<Digest>,
    first_cohort_formula: Digest,
}

/// A deterministic host composition of per-profile engines (D-C2-2): one
/// activation registry for every profile (the single-registry convention),
/// each horizon presented to each engine in `order`.
fn composed(
    names: &[&str],
    descending: bool,
    pacing: u32,
    horizons: &[u64],
) -> BTreeMap<String, Member> {
    let mut registry = ActivationRegistry::new();
    let mut engines: BTreeMap<String, (Engine, Vec<CohortReport>, Digest)> = BTreeMap::new();
    for name in names {
        let p = pid(name);
        let profile = registry.activate(&member_manifest(&p, &[])).unwrap();
        let rule_set = registry
            .activate_rule_set(
                &profile,
                &RuleSetSpec {
                    profile_id: p.clone(),
                    budgets: budgets(pacing),
                    rules: member_rules(),
                    baselines: vec![],
                },
            )
            .unwrap();
        let cfg = ConfigRevision::build(
            p.clone(),
            BoundedText::new("composed.config").unwrap(),
            Vec::<ConfigEntry>::new(),
        )
        .unwrap();
        let mut e = Engine::genesis(EngineGenesis {
            profile,
            rule_set,
            config: cfg,
            timeline: TimelineGenesis {
                timeline_epoch: TimelineEpoch(0),
                sequencer: SourceId::new(SEQUENCER).unwrap(),
                window_width: 2,
            },
            start_time: LogicalTime(0),
            initial_work: member_work(name)
                .into_iter()
                .map(|(r, t, k)| initial(r, &a(), t, k))
                .collect(),
        })
        .unwrap();
        let keys: Vec<WorkKey> = e
            .obligations()
            .keys()
            .filter(|k| k.due_time == LogicalTime(100))
            .cloned()
            .collect();
        let formula = scheduled_cohort_identity(&p, LogicalTime(100), &keys);
        fixture::clear_observation(&mut e);
        engines.insert(name.to_string(), (e, Vec::new(), formula));
    }
    let mut order: Vec<String> = engines.keys().cloned().collect();
    if descending {
        order.reverse();
    }
    for h in horizons {
        for name in &order {
            let (e, rs, _) = engines.get_mut(name).unwrap();
            rs.extend(reports(&drive(e, &advance(*h))));
        }
    }
    engines
        .into_iter()
        .map(|(name, (e, reports, formula))| {
            let cohorts = fixture::observation(&e)
                .prewave
                .iter()
                .filter(|p| p.wave_index == 0)
                .map(|p| p.cohort_identity.clone())
                .collect();
            (
                name,
                Member {
                    reports,
                    digests: digest_pair(&e),
                    cohorts,
                    first_cohort_formula: formula,
                },
            )
        })
        .collect()
}

/// AT-I40 + AT-I39 cross-profile companion, under the accepted per-profile
/// architecture: two profiles with identical rules and work at one due time
/// form two cohorts with distinct identities, each exactly the formula over
/// its own members; every per-profile result (canonical reports including
/// emission identities, batch digests and obligation records; cohort
/// identities; engine and stable-boundary digests) is byte-identical whether
/// a third profile is present or absent, under either composition order, the
/// ascending `(due_time, profile_id)` interleaving, and a pacing budget of one.
/// No cause can cross profiles: a rule targeting another profile's definition
/// is rejected at the door, and another profile's command is refused. Kills:
/// cohort identity without the profile, and any shared cross-profile state
/// (the third profile's work would perturb the first two).
#[test]
fn at_i39_i40_composed_per_profile_oracle() {
    let base = composed(&["world-p", "world-q"], false, 50, &[120]);
    let (p, q) = (&base["world-p"], &base["world-q"]);
    assert_eq!(p.cohorts[0], p.first_cohort_formula);
    assert_eq!(q.cohorts[0], q.first_cohort_formula);
    assert_ne!(p.cohorts[0], q.cohorts[0], "two cohorts at one due time");
    assert_eq!(p.reports.len(), 2, "cohort 100 and the follow-up at 110");
    for variant in [
        composed(&["world-p", "world-q", "world-r"], false, 50, &[120]),
        composed(&["world-p", "world-q", "world-r"], true, 50, &[120]),
        composed(
            &["world-p", "world-q", "world-r"],
            false,
            50,
            &[100, 105, 110, 115, 120],
        ),
        composed(&["world-p", "world-q"], false, 1, &[120]),
        composed(&["world-p", "world-q", "world-r"], true, 1, &[100, 120]),
    ] {
        assert_eq!(&variant["world-p"], p);
        assert_eq!(&variant["world-q"], q);
    }
    assert_eq!(&composed(&["world-p"], false, 50, &[120])["world-p"], p);
    // Cross-profile causation is impossible.
    let mut registry = ActivationRegistry::new();
    let (pp, qq) = (pid("world-p"), pid("world-q"));
    let profile_p = registry.activate(&member_manifest(&pp, &[])).unwrap();
    registry
        .activate(&member_manifest(&qq, &["state.q_only"]))
        .unwrap();
    assert!(registry
        .activate_rule_set(
            &profile_p,
            &RuleSetSpec {
                profile_id: pp.clone(),
                budgets: budgets(8),
                rules: vec![adder("rule.cross", "work.one", "state.q_only", 1)],
                baselines: vec![],
            },
        )
        .is_err());
    let rule_set = registry
        .activate_rule_set(
            &profile_p,
            &RuleSetSpec {
                profile_id: pp.clone(),
                budgets: budgets(8),
                rules: member_rules(),
                baselines: vec![],
            },
        )
        .unwrap();
    let mut e = Engine::genesis(EngineGenesis {
        profile: profile_p,
        rule_set,
        config: ConfigRevision::build(
            pp.clone(),
            BoundedText::new("composed.config").unwrap(),
            Vec::<ConfigEntry>::new(),
        )
        .unwrap(),
        timeline: TimelineGenesis {
            timeline_epoch: TimelineEpoch(0),
            sequencer: SourceId::new(SEQUENCER).unwrap(),
            window_width: 2,
        },
        start_time: LogicalTime(0),
        initial_work: vec![],
    })
    .unwrap();
    let mut foreign = command_request("cmd.q", 1, 1, "cmd.any", a(), vec![], vec![]);
    foreign.profile_id = qq;
    let r = drive(&mut e, &Request::Command(foreign));
    assert!(matches!(
        r.last().unwrap().outcome(),
        Outcome::CompletedCommandNotFinalized(FinalizationRefusal::WrongProfile { .. })
    ));
}
