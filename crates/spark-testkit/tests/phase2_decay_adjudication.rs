//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 decay adjudication: the Operator's fixed-grid decision
//! (`SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md`, D-1 …
//! D-7) and the amended AT-I22′ / AT-I23′.
//!
//! Every vector runs in both directions: decay from above the baseline and
//! recovery from below it (its mirror image). It runs through the public
//! door: commands for writes and activations, scheduled work for decay
//! evaluations and scheduled shocks. The exact values of each vector are
//! written out by hand. A brute-force reference model of the decision must
//! agree with them. The model enumerates grid points one by one, uses no
//! division, and shares no code with the engine. Each test names the
//! superseded reading or the wrong implementation it discriminates. The
//! compiled mutants are in the adjudication evidence.

use spark_core::clock::LogicalTime;
use spark_core::scope::ScopeId;
use spark_core::value::CanonicalValue;
use spark_engine::engine::{CompletedHistory, Engine, EngineGenesis};
use spark_engine::profile::config::ConfigRevision;
use spark_engine::report::CohortOutcome;
use spark_engine::request::{CommandPayload, CommandRequest, Outcome, Request};
use spark_engine::rules::{Param, Update};
use spark_testkit::phase2::*;
use std::collections::BTreeSet;

fn a() -> ScopeId {
    actor("a")
}

/// One command of a timeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Write {
    /// `cmd.set`: assign the value (an ordinary non-decay write).
    Assign(i64),
    /// `cmd.shock`: add the delta (an ordinary non-decay write).
    Shock(i64),
    /// Activate an epoch whose resolved decay `(rate, cadence)` is given.
    Activate(i64, u64),
}

/// A decay timeline. The decay operation's rate and cadence are hot-tunable
/// config keys, and `genesis` gives their initial values. `commands` are
/// `(effective time, write)` in ascending time order. `evals` are the due
/// times of scheduled decay evaluations. `kicks` are the due times of
/// scheduled `+kick` shocks, a separate scheduled boundary (v2 §4.3).
#[derive(Clone, Debug)]
struct Timeline {
    genesis: (i64, u64),
    commands: Vec<(u64, Write)>,
    evals: Vec<u64>,
    kicks: Vec<u64>,
    kick: i64,
}

impl Timeline {
    fn new(genesis: (i64, u64), commands: Vec<(u64, Write)>, evals: &[u64]) -> Self {
        Timeline {
            genesis,
            commands,
            evals: evals.to_vec(),
            kicks: vec![],
            kick: 0,
        }
    }

    fn with_evals(&self, evals: &[u64]) -> Self {
        Timeline {
            evals: evals.to_vec(),
            ..self.clone()
        }
    }

    /// The mirror image across baseline 0: recovery instead of decay.
    fn mirrored(&self) -> Self {
        let mut t = self.clone();
        for (_, w) in &mut t.commands {
            *w = match *w {
                Write::Assign(v) => Write::Assign(-v),
                Write::Shock(d) => Write::Shock(-d),
                other => other,
            };
        }
        t.kick = -t.kick;
        t
    }

    fn end(&self) -> u64 {
        self.evals
            .iter()
            .chain(&self.kicks)
            .copied()
            .chain(self.commands.iter().map(|(t, _)| *t))
            .max()
            .unwrap_or(0)
    }
}

type Cell = Option<(i64, LogicalTime)>;

fn tune(rate: i64, cadence: u64) -> ConfigRevision {
    config(vec![
        ("tune.rate", CanonicalValue::Int(rate)),
        ("tune.cadence", CanonicalValue::Int(cadence as i64)),
    ])
}

fn cell(e: &Engine) -> Cell {
    e.state()
        .get(&profile_id(), &def("state.stress"), &a())
        .map(|c| match c.value {
            CanonicalValue::Int(v) => (v, c.updated_at),
            _ => panic!("numeric"),
        })
}

struct Run {
    engine: Engine,
    genesis: EngineGenesis,
    commands: Vec<CommandRequest>,
    /// The cell after each stop.
    seen: Vec<Cell>,
}

fn submit(engine: &mut Engine, genesis: &EngineGenesis, c: &CommandRequest, restore: bool) {
    let r = drive(engine, &Request::Command(c.clone()));
    assert_eq!(r.last().unwrap().outcome(), &Outcome::Completed);
    let last = reports(&r).last().unwrap().outcome.clone();
    match c.payload {
        CommandPayload::ActivateEpoch { .. } => {
            assert!(matches!(last, CohortOutcome::EpochActivated { .. }))
        }
        _ => assert_eq!(last, CohortOutcome::Committed),
    }
    if restore {
        let snapshot = engine.snapshot().unwrap();
        *engine = Engine::restore(snapshot, &genesis.profile).unwrap();
    }
}

/// Runs a timeline through the public door with pacing budget `pacing`. At
/// each stop `s`, in ascending order, it first submits every command with
/// effective time `<= s`. Scheduled work due at or before a command's
/// effective time runs before that command. It then advances the engine to
/// `s` and records the cell. With `restore_after = Some(i)` it snapshots and
/// restores the engine after the `i`-th command and continues from the
/// restored instance.
fn run(t: &Timeline, pacing: u32, stops: &[u64], restore_after: Option<usize>) -> Run {
    let mut f = standard();
    let rules = vec![
        on_command(
            "rule.set",
            "cmd.set",
            vec![emit("s", "state.stress", Update::Assign(param(0)))],
        ),
        on_command(
            "rule.shock",
            "cmd.shock",
            vec![emit("s", "state.stress", Update::Add(param(0)))],
        ),
        work_rule(
            "rule.kick",
            "work.kick",
            vec![emit("k", "state.stress", Update::Add(lit(t.kick)))],
        ),
        work_rule(
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
        ),
    ];
    let work = t
        .evals
        .iter()
        .map(|d| initial("rule.d", &a(), *d, "work.d"))
        .chain(
            t.kicks
                .iter()
                .map(|k| initial("rule.kick", &a(), *k, "work.kick")),
        )
        .collect();
    let mut g = f.genesis_with(
        budgets(pacing),
        rules,
        vec![baseline("state.stress", 0)],
        work,
    );
    g.config = tune(t.genesis.0, t.genesis.1);
    let genesis = g.clone();
    let rule_set = g.rule_set.clone();
    let mut engine = Engine::genesis(g).unwrap();
    let commands: Vec<CommandRequest> = t
        .commands
        .iter()
        .enumerate()
        .map(|(i, (at, w))| {
            let id = format!("cmd.{i}");
            let seq = i as u64 + 1;
            match *w {
                Write::Assign(v) => command_request(&id, *at, seq, "cmd.set", a(), vec![], vec![v]),
                Write::Shock(d) => {
                    command_request(&id, *at, seq, "cmd.shock", a(), vec![], vec![d])
                }
                Write::Activate(rate, cadence) => CommandRequest {
                    payload: CommandPayload::ActivateEpoch {
                        rule_set: rule_set.clone(),
                        config: tune(rate, cadence),
                    },
                    ..command_request(&id, *at, seq, "spark.epoch.activate", a(), vec![], vec![])
                },
            }
        })
        .collect();
    let mut next = 0;
    let mut seen = Vec::new();
    for &s in stops {
        while next < commands.len() && commands[next].effective_time.0 <= s {
            submit(
                &mut engine,
                &genesis,
                &commands[next],
                restore_after == Some(next),
            );
            next += 1;
        }
        drive(&mut engine, &advance(s));
        seen.push(cell(&engine));
    }
    while next < commands.len() {
        submit(
            &mut engine,
            &genesis,
            &commands[next],
            restore_after == Some(next),
        );
        next += 1;
    }
    let end = t.end().max(stops.last().copied().unwrap_or(0));
    drive(&mut engine, &advance(end));
    Run {
        engine,
        genesis,
        commands,
        seen,
    }
}

/// The reference model of the decision. It is written without the engine's
/// arithmetic and returns the cell after each stop.
///
/// - It enumerates the grid points of every parameter segment one by one. An
///   activation that changes `(rate, cadence)` starts a new grid at its
///   barrier. A point ending exactly at a barrier belongs to the closing
///   segment. The closing segment's unfinished residual is never a point
///   (D-1, D-3 … D-5).
/// - Scheduled work at a time runs before commands at that time.
/// - A decay evaluation applies the points in `(updated_at, now]` one step at
///   a time, linearly toward baseline 0 and never past it. It commits at
///   `now` whether or not the value moved (D-6, D-7).
/// - A non-decay write sets the value and `updated_at` and leaves the grid
///   alone (D-2).
fn model(t: &Timeline, stops: &[u64]) -> Vec<Cell> {
    let end = t.end().max(stops.last().copied().unwrap_or(0));
    let mut epochs = vec![(0u64, t.genesis)];
    for (at, w) in &t.commands {
        if let Write::Activate(rate, cadence) = w {
            epochs.push((*at, (*rate, *cadence)));
        }
    }
    let mut points: Vec<(u64, i64)> = Vec::new();
    let mut origin = 0u64;
    for (i, &(start, (rate, cadence))) in epochs.iter().enumerate() {
        if i > 0 && epochs[i - 1].1 != (rate, cadence) {
            origin = start;
        }
        let stop = epochs.get(i + 1).map_or(end, |n| n.0);
        let mut p = origin + cadence;
        while p <= stop {
            if p > start {
                points.push((p, rate));
            }
            p += cadence;
        }
    }
    #[derive(Clone, Copy)]
    enum Ev {
        Eval,
        Kick,
        Command(Write),
    }
    // Scheduled work (class 0) before commands (class 1) at one time; the
    // sort is stable, so commands keep their order.
    let mut events: Vec<(u64, u8, Ev)> = t
        .evals
        .iter()
        .map(|e| (*e, 0, Ev::Eval))
        .chain(t.kicks.iter().map(|k| (*k, 0, Ev::Kick)))
        .chain(t.commands.iter().map(|(at, w)| (*at, 1, Ev::Command(*w))))
        .collect();
    events.sort_by_key(|(at, class, _)| (*at, *class));
    for k in &t.kicks {
        assert!(
            !t.evals.contains(k),
            "a coincident kick is a rejection, not modeled"
        );
    }
    let toward = |v: i64, rate: i64| {
        if v > 0 {
            (v - rate).max(0)
        } else {
            (v + rate).min(0)
        }
    };
    let (mut value, mut last): (Option<i64>, u64) = (None, 0);
    let mut out = Vec::new();
    let mut i = 0;
    for &s in stops {
        while i < events.len() && events[i].0 <= s {
            let (at, _, ev) = events[i];
            match ev {
                Ev::Eval => {
                    if let Some(mut v) = value {
                        for &(p, rate) in &points {
                            if last < p && p <= at {
                                v = toward(v, rate);
                            }
                        }
                        value = Some(v);
                        last = at;
                    }
                }
                Ev::Kick => {
                    value = Some(value.unwrap_or(0) + t.kick);
                    last = at;
                }
                Ev::Command(Write::Assign(v)) => {
                    value = Some(v);
                    last = at;
                }
                Ev::Command(Write::Shock(d)) => {
                    value = Some(value.unwrap_or(0) + d);
                    last = at;
                }
                Ev::Command(Write::Activate(..)) => {}
            }
            i += 1;
        }
        out.push(value.map(|v| (v, LogicalTime(last))));
    }
    out
}

/// Asserts the hand-written `(stop, value, commit time)` rows for `t` and for
/// its mirror image (value negated). Both the reference model and the engine
/// must produce them at every stop.
fn check(t: &Timeline, rows: &[(u64, i64, u64)]) {
    let stops: Vec<u64> = rows.iter().map(|r| r.0).collect();
    for (sign, t) in [(1, t.clone()), (-1, t.mirrored())] {
        let want: Vec<Cell> = rows
            .iter()
            .map(|&(_, v, at)| Some((sign * v, LogicalTime(at))))
            .collect();
        assert_eq!(model(&t, &stops), want, "model, sign {sign}: {t:?}");
        assert_eq!(
            run(&t, 50, &stops, None).seen,
            want,
            "engine, sign {sign}: {t:?}"
        );
    }
}

fn subsets(items: &[u64]) -> Vec<Vec<u64>> {
    (0u32..(1 << items.len()))
        .map(|mask| {
            items
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, t)| *t)
                .collect()
        })
        .collect()
}

use Write::{Activate, Assign, Shock};

// ================================================================ D-2 fresh assignment

/// AT-I22′(d), D-2: a fresh assignment does not re-phase the grid. The cell is
/// created at 9 with 100 (rate 10, cadence 10). The grid step ending at 10
/// applies at 10, one logical tick after the write: `(90, 10)`. The
/// superseded Q7 duration reading requires a whole cadence after the write:
/// `(100, 10)`, then `(90, 19)`. The catch-up at 20 takes both points 10 and
/// 20 (80; the duration reading gives 90). A reassignment at 17 of an
/// existing cell likewise takes the point at 20 (40; the duration reading
/// gives 50). Kills the elapsed-since-write count (`C2-05-relative-steps`), the
/// backdated commit, and skipping unmoved evaluations (`(90, 10)` at 19).
#[test]
fn d2_fresh_assignment_keeps_the_fixed_grid() {
    let fresh = Timeline::new((10, 10), vec![(9, Assign(100))], &[10, 19, 20, 30]);
    check(
        &fresh,
        &[
            (9, 100, 9),
            (10, 90, 10),
            (19, 90, 19),
            (20, 80, 20),
            (30, 70, 30),
        ],
    );
    check(&fresh.with_evals(&[20]), &[(20, 80, 20)]);
    let reassigned = Timeline::new(
        (10, 10),
        vec![(0, Assign(100)), (17, Assign(50))],
        &[5, 20, 27, 30],
    );
    check(
        &reassigned,
        &[
            (5, 100, 5),
            (17, 50, 17),
            (20, 40, 20),
            (27, 40, 27),
            (30, 30, 30),
        ],
    );
}

// ================================================================ D-2 post-shock

/// AT-I22′(c)(d), D-2, adjudication S-6: a separate shock does not re-phase
/// the grid. The review's composition runs as follows. Decay at 30 gives 70.
/// A scheduled +5 shock at 31 gives 75. Decay at 40 takes the point at 40,
/// giving 65; the superseded duration reading keeps 75 for nine elapsed
/// units. Decay at 41 finds no point (65, committed at 41). Decay at 50 gives
/// 55. The same holds for a command shock at 39, for catch-up across the
/// shock, and in mirror image. Kills the elapsed-since-write count.
#[test]
fn d2_post_shock_decay_keeps_the_fixed_grid() {
    let scheduled = Timeline {
        kicks: vec![31],
        kick: 5,
        ..Timeline::new((10, 10), vec![(0, Assign(100))], &[30, 40, 41, 50])
    };
    check(
        &scheduled,
        &[
            (30, 70, 30),
            (31, 75, 31),
            (40, 65, 40),
            (41, 65, 41),
            (50, 55, 50),
        ],
    );
    check(
        &scheduled.with_evals(&[30, 50]),
        &[(30, 70, 30), (31, 75, 31), (50, 55, 50)],
    );
    let commanded = Timeline::new(
        (10, 10),
        vec![(0, Assign(100)), (39, Shock(5))],
        &[30, 40, 50],
    );
    check(
        &commanded,
        &[(30, 70, 30), (39, 75, 39), (40, 65, 40), (50, 55, 50)],
    );
}

// ================================================================ D-3 / D-4 activation boundary

/// AT-I23′(b)(f), D-3/D-4: old rate 2 / cadence 7, then rate 5 / cadence 4
/// activated at 21. The old segment owns the points 7, 14 and 21, giving 94.
/// The new grid's first point is 25, giving 89. Scheduled work due at 21 runs
/// before the activating command and evaluates under the old epoch: `(94,
/// 21)`. With one catch-up evaluation at 25, or evaluations at 24 and 25, the
/// point at 21 is still the closing segment's. Kills dropping the endpoint
/// from the closing segment (96 at 24, 91 at 25), a mutant that the
/// evaluation at 21 alone cannot see.
#[test]
fn d3_d4_step_ending_at_activation_belongs_to_the_closing_segment() {
    let t = Timeline::new(
        (2, 7),
        vec![(0, Assign(100)), (21, Activate(5, 4))],
        &[21, 24, 25],
    );
    check(&t, &[(21, 94, 21), (24, 94, 24), (25, 89, 25)]);
    check(&t.with_evals(&[25]), &[(25, 89, 25)]);
    check(&t.with_evals(&[24, 25]), &[(24, 94, 24), (25, 89, 25)]);
}

// ================================================================ D-3 residual discard

/// AT-I23′(c), D-3/D-5: old rate 2 / cadence 7, then rate 5 / cadence 4
/// activated at 19. The old segment bills 7 and 14, giving 96. Its unfinished
/// residual `(14, 19]` is billed at no rate. The new grid is 23, 27, … . The
/// wrong implementations this kills give these values:
/// - billing the residual as one whole old-rate step: 94 by 22;
/// - carrying the old origin into the new grid (points 22, 26): 91 at 22;
/// - an absolute grid from 0 (points 20, 24): 91 at 22;
/// - the elapsed-since-write count: 96 at 27.
#[test]
fn d3_residual_interval_is_discarded_at_a_parameter_change() {
    let t = Timeline::new(
        (2, 7),
        vec![(0, Assign(100)), (19, Activate(5, 4))],
        &[19, 22, 23, 26, 27],
    );
    check(
        &t,
        &[
            (19, 96, 19),
            (22, 96, 22),
            (23, 91, 23),
            (26, 91, 26),
            (27, 86, 27),
        ],
    );
    check(&t.with_evals(&[27]), &[(27, 86, 27)]);
    check(&t.with_evals(&[22]), &[(22, 96, 22)]);
    check(&t.with_evals(&[26]), &[(26, 91, 26)]);
}

// ================================================================ D-3 / D-5 shortening

/// AT-I23′(a)(d), D-3/D-5: shortening at an unaligned barrier. The cells
/// start at rate 1 / cadence 100, and rate 5 / cadence 10 is activated at 53.
/// The old grid has no point by 53, and the new grid is 63, 73, … . So the
/// value is 100 at 60 and 62, 95 at 63, and 90 at 73. A cadence-only
/// shortening (rate 3, cadence 10 → 7 at 25) bills old points 10 and 20,
/// giving 94. The new grid is 32, 39, giving 94 at 30 and 91 at 32. Kills an
/// absolute grid (95 at 60; 91 at 30) and the elapsed-since-write count (100
/// at 63).
#[test]
fn d3_d5_shortened_cadence_starts_a_new_grid_at_the_barrier() {
    let t = Timeline::new(
        (1, 100),
        vec![(0, Assign(100)), (53, Activate(5, 10))],
        &[53, 60, 62, 63, 73],
    );
    check(
        &t,
        &[
            (53, 100, 53),
            (60, 100, 60),
            (62, 100, 62),
            (63, 95, 63),
            (73, 90, 73),
        ],
    );
    check(&t.with_evals(&[73]), &[(73, 90, 73)]);
    let cadence_only = Timeline::new(
        (3, 10),
        vec![(0, Assign(100)), (25, Activate(3, 7))],
        &[25, 30, 32, 39],
    );
    check(
        &cadence_only,
        &[(25, 94, 25), (30, 94, 30), (32, 91, 32), (39, 88, 39)],
    );
}

// ================================================================ D-3 / D-5 lengthening

/// AT-I23′(a)(d)(e), D-3/D-5: lengthening at an unaligned barrier. The cells
/// start at rate 1 / cadence 10, and rate 5 / cadence 100 is activated at 55.
/// The old points 10 … 50 give 95, and the residual `(50, 55]` is
/// discarded. The new grid's first point is 155. The wrong implementations
/// this kills give these values:
/// - an absolute grid (point 100): 90 at 150;
/// - the carried origin (point 150): 90 at 150;
/// - billing the residual at the old rate: 94 at 60.
///
/// An activation that leaves rate 1 / cadence 10 unchanged does not move the
/// grid, giving 94 at 60. Restarting the grid at every activation gives 95.
#[test]
fn d3_d5_lengthened_cadence_starts_a_new_grid_at_the_barrier() {
    let t = Timeline::new(
        (1, 10),
        vec![(0, Assign(100)), (55, Activate(5, 100))],
        &[55, 60, 150, 154, 155],
    );
    check(
        &t,
        &[
            (55, 95, 55),
            (60, 95, 60),
            (150, 95, 150),
            (154, 95, 154),
            (155, 90, 155),
        ],
    );
    check(&t.with_evals(&[155]), &[(155, 90, 155)]);
    check(&t.with_evals(&[150]), &[(150, 95, 150)]);
    let unchanged = Timeline::new(
        (1, 10),
        vec![(0, Assign(100)), (55, Activate(1, 10))],
        &[60],
    );
    check(&unchanged, &[(60, 94, 60)]);
}

// ================================================================ D-6 / D-7 saturation

/// AT-I22′(b)(d), D-6/D-7: saturation across a barrier and unmoved commits.
/// The cell starts at 12 with rate 5 / cadence 10. Points 10 and 20 give 7,
/// then 2. The rate 4 / cadence 3 activation at 25 discards `(20, 25]`. The
/// first new point, 28, saturates at the baseline: 0, never −2. Each
/// evaluation commits `(value, now)`, including an unmoved one (25, 26) and a
/// saturated one (31, 40). Every one of the 64 evaluation subsets ending at
/// 40 gives `(0, 40)`. The rate-zero segment 30–60 commits unmoved values.
/// Its point at 60 ends at the next activation and bills rate 0. The new
/// segment's first point is 70. Kills skipping unmoved evaluations (`(0, 28)`
/// at 31) and the backdated commit (`(2, 20)` at 25).
#[test]
fn d6_d7_saturation_and_unmoved_evaluations_commit_at_canonical_time() {
    let t = Timeline::new(
        (5, 10),
        vec![(0, Assign(12)), (25, Activate(4, 3))],
        &[10, 20, 25, 26, 28, 31, 40],
    );
    check(
        &t,
        &[
            (10, 7, 10),
            (20, 2, 20),
            (25, 2, 25),
            (26, 2, 26),
            (28, 0, 28),
            (31, 0, 31),
            (40, 0, 40),
        ],
    );
    for mut times in subsets(&[10, 20, 25, 26, 28, 31]) {
        times.push(40);
        check(&t.with_evals(&times), &[(40, 0, 40)]);
    }
    let zero = Timeline::new(
        (10, 10),
        vec![
            (0, Assign(100)),
            (30, Activate(0, 10)),
            (60, Activate(10, 10)),
        ],
        &[30, 45, 60, 65, 70],
    );
    check(
        &zero,
        &[
            (30, 70, 30),
            (45, 70, 45),
            (60, 70, 60),
            (65, 70, 65),
            (70, 60, 70),
        ],
    );
}

// ================================================================ D-1 chunking

/// The composite timeline: a fresh assignment at 9, rate 2 / cadence 7, then
/// rate 5 / cadence 4 at 19, a +5 shock at 29, and rate 3 / cadence 6 at 36.
fn composite() -> Timeline {
    Timeline::new(
        (2, 7),
        vec![
            (9, Assign(100)),
            (19, Activate(5, 4)),
            (29, Shock(5)),
            (36, Activate(3, 6)),
        ],
        &[14, 19, 23, 27, 33, 40, 45],
    )
}

/// AT-I22′(a)(d), D-1: chunk invariance after unaligned writes and across
/// barriers. In the composite timeline the grid points are 14 (rate 2); 23,
/// 27, 31, 35 (rate 5; 35 closes its segment at 36); and 42 (rate 3).
/// Point 7 precedes the write at 9 and is not in `(9, …]`. Point 21
/// is never a point: it falls in the residual of the segment closed at 19.
/// The cell reads `(88, 27)` before the shock and `(80, 45)` at the end under
/// every subset of evaluations before the shock and every subset after it.
/// Both directions are checked. Kills the elapsed-since-write count.
#[test]
fn d1_chunking_across_unaligned_writes_and_barriers() {
    let t = composite();
    check(
        &t,
        &[
            (14, 98, 14),
            (19, 98, 19),
            (23, 93, 23),
            (27, 88, 27),
            (29, 93, 29),
            (33, 88, 33),
            (40, 83, 40),
            (45, 80, 45),
        ],
    );
    for (before, after) in subsets(&[10, 14, 19, 21, 23])
        .into_iter()
        .map(|s| (s, vec![]))
        .chain(
            subsets(&[31, 35, 36, 41, 42])
                .into_iter()
                .map(|s| (vec![], s)),
        )
    {
        let mut evals = before.clone();
        evals.push(27);
        evals.extend(after.iter().copied());
        evals.push(45);
        check(&t.with_evals(&evals), &[(27, 88, 27), (45, 80, 45)]);
    }
}

// ================================================================ replay / restore

/// AT-I23′(g): replay, restore and pacing reproduce the decision. For the
/// composite timeline in both directions:
/// - pacing budget 1 gives the same cells and digests as budget 50;
/// - history reconstruction is digest-identical;
/// - snapshot/restore after each of the four commands, then continuing, is
///   digest-identical and reads the same cells at every stop.
#[test]
fn d_replay_restore_and_pacing_reproduce_the_decision() {
    let stops = [14, 19, 23, 27, 29, 33, 40, 45];
    for t in [composite(), composite().mirrored()] {
        let full = run(&t, 50, &stops, None);
        assert_eq!(full.seen, model(&t, &stops));
        let paced = run(&t, 1, &stops, None);
        assert_eq!(paced.seen, full.seen);
        assert_eq!(digest_pair(&paced.engine), digest_pair(&full.engine));
        let history = CompletedHistory {
            commands: full.commands.clone(),
            resets: vec![],
            frontier: full.engine.frontier(),
            recorded_history_digest: full.engine.timeline_history_digest(),
            recorded_stable_boundary_digest: full.engine.stable_boundary_digest(),
        };
        let replayed = Engine::reconstruct_completed(full.genesis.clone(), &history).unwrap();
        assert_eq!(digest_pair(&replayed), digest_pair(&full.engine));
        assert_eq!(cell(&replayed), cell(&full.engine));
        for i in 0..t.commands.len() {
            let restored = run(&t, 50, &stops, Some(i));
            assert_eq!(restored.seen, full.seen, "restore after command {i}");
            assert_eq!(
                digest_pair(&restored.engine),
                digest_pair(&full.engine),
                "restore after command {i}"
            );
        }
    }
}

// ================================================================ generated sweep

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

/// Deterministically generated timelines with non-decay writes. The
/// generation fills the gap the review noted: the earlier sweep
/// (`phase2_decay_oracle_revision.rs`) seeds at 0 and never writes again.
/// Each timeline has:
/// - a fresh assignment at an unaligned time 1 … 10;
/// - up to five further commands at distinct times 11 … 95: reassignments,
///   shocks of either sign, and activations that change both parameters,
///   change only the rate, or repeat the parameters unchanged;
/// - evaluations at random times, including before the cell exists.
///
/// Activations removing the operation are excluded. The adjudication does not
/// decide them (§5.1); the earlier sweep pins their existing behavior.
fn generated() -> Vec<Timeline> {
    let mut g = Lcg(0xAD1D_C2E3);
    let mut out = Vec::new();
    while out.len() < 40 {
        let genesis = g.parameters();
        let mut commands = vec![(1 + g.below(10), Assign(g.below(241) as i64 - 120))];
        let mut times = BTreeSet::new();
        let count = g.below(6);
        while (times.len() as u64) < count {
            times.insert(11 + g.below(85));
        }
        let mut parameters = genesis;
        for t in times {
            let w = match g.below(5) {
                0 => Assign(g.below(241) as i64 - 120),
                1 => Shock(g.below(41) as i64 - 20),
                2 => Activate(parameters.0, parameters.1),
                3 => {
                    parameters = (g.below(13) as i64, parameters.1);
                    Activate(parameters.0, parameters.1)
                }
                _ => {
                    parameters = g.parameters();
                    Activate(parameters.0, parameters.1)
                }
            };
            commands.push((t, w));
        }
        let evals: Vec<u64> = (1..=100).filter(|t| *t == 100 || g.below(6) == 0).collect();
        out.push(Timeline::new(genesis, commands, &evals));
    }
    out
}

/// AT-I22′/AT-I23′ generalized: over 40 generated timelines with writes, the
/// engine's cell after every evaluation equals the reference model's. The
/// sweep must cover each command shape, including activations that change
/// the parameters and activations that do not. Kills the elapsed-since-write
/// count and the grid mutants.
#[test]
fn d_generated_timelines_with_writes_match_the_reference_model() {
    let mut shapes = BTreeSet::new();
    for (i, t) in generated().iter().enumerate() {
        let mut parameters = t.genesis;
        for (_, w) in &t.commands[1..] {
            shapes.insert(match *w {
                Assign(_) => "reassign",
                Shock(_) => "shock",
                Activate(r, c) if (r, c) == parameters => "unchanged",
                Activate(r, c) => {
                    parameters = (r, c);
                    "changed"
                }
            });
        }
        let r = run(t, 50, &t.evals, None);
        assert_eq!(r.seen, model(t, &t.evals), "timeline {i}: {t:?}");
    }
    assert_eq!(
        shapes,
        BTreeSet::from(["changed", "reassign", "shock", "unchanged"])
    );
}
