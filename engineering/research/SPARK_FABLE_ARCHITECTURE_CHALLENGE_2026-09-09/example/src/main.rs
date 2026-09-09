//! RESEARCH / PROPOSED DESIGN — NOT ADOPTED. Runs the architectural checks and
//! prints PASS/FAIL lines. Exit code 1 if any check fails. Not a benchmark.

use spark_core::clock::LogicalTime;
use spark_core::scope::ScopeKind;
use spark_fable_example::*;
use std::collections::BTreeMap;

fn ruleset() -> Ruleset {
    let settlement = scope(ScopeKind::Settlement, "pontafique");
    let household = scope(ScopeKind::Household, "h1");
    let actor = scope(ScopeKind::Actor, "a1");
    let region = scope(ScopeKind::Region, "r1");
    let mut decay = BTreeMap::new();
    decay.insert(def("pressure.drought"), 1); // drought eases 1 per tick
    Ruleset {
        decay_per_tick: decay,
        thresholds: vec![
            ThresholdRule {
                id: def("rule.scarcity_to_hunger"),
                watch: (def("pressure.food_scarcity"), settlement.clone()),
                at_least: 30,
                emit: Effect::AddDelta { target: (def("exposure.hunger"), household.clone()), delta: 20 },
            },
            ThresholdRule {
                id: def("rule.hunger_to_choice"),
                watch: (def("exposure.hunger"), household.clone()),
                at_least: 20,
                emit: Effect::ScheduleWork {
                    producer: def("rule.actor_choice"),
                    scope: actor.clone(),
                    delay: 20,
                    body: WorkBody {
                        effects: vec![
                            Effect::AddDelta { target: (def("intent.hunting"), actor.clone()), delta: 1 },
                            Effect::ScheduleWork {
                                producer: def("rule.hunting_result"),
                                scope: region.clone(),
                                delay: 100,
                                body: WorkBody { effects: vec![Effect::AddDelta { target: (def("wildlife.population"), region.clone()), delta: -10 }] },
                            },
                        ],
                    },
                },
            },
        ],
        max_wave_depth: 4,
    }
}

/// drought -> food scarcity -> household hunger -> actor choice (delayed 20)
/// -> hunting result (delayed 100, beyond a horizon of 50).
fn seed(engine: &mut Engine, with_defect: bool) {
    let settlement = scope(ScopeKind::Settlement, "pontafique");
    engine.seed_work(
        LogicalTime(10),
        "trigger.drought",
        &settlement,
        WorkBody {
            effects: vec![
                Effect::AddDelta { target: (def("pressure.drought"), settlement.clone()), delta: 50 },
                Effect::ScheduleWork {
                    producer: def("trigger.drought.followup"),
                    scope: settlement.clone(),
                    delay: 5,
                    body: WorkBody { effects: vec![Effect::AddDelta { target: (def("pressure.food_scarcity"), settlement.clone()), delta: 30 }] },
                },
            ],
        },
    );
    engine.seed_work(
        LogicalTime(30),
        "trigger.rain",
        &settlement,
        WorkBody { effects: vec![Effect::AddDelta { target: (def("pressure.drought"), settlement.clone()), delta: -10 }] },
    );
    if with_defect {
        engine.seed_work(
            LogicalTime(40),
            "trigger.bad_rule",
            &settlement,
            WorkBody {
                effects: vec![
                    Effect::AddDelta { target: (def("pressure.drought"), settlement.clone()), delta: 999 },
                    Effect::ScheduleWork { producer: def("trigger.bad_rule"), scope: settlement.clone(), delay: 0, body: WorkBody { effects: vec![] } },
                ],
            },
        );
    }
}

fn fresh(mode: Mode, with_defect: bool) -> Engine {
    let mut e = Engine::new(profile(), ruleset(), mode);
    seed(&mut e, with_defect);
    e
}

fn run_all(engine: &mut Engine, reqs: &[Request], budget: u32) -> (Vec<Outcome>, usize) {
    let mut mb = Mailbox::new(64);
    for r in reqs {
        assert_eq!(mb.offer(r.clone()), Offer::Accepted);
    }
    let log = drain_mailbox(&mut mb, engine, budget);
    let calls = log.len();
    (log, calls)
}

fn trace_summary(e: &Engine) -> Vec<(u64, String, usize, usize, bool)> {
    e.trace
        .iter()
        .map(|r| (r.at.0, format!("{}", r.batch_digest)[..12].to_string(), r.committed.len(), r.scheduled.len(), r.rejection.is_some()))
        .collect()
}

struct Report {
    lines: Vec<String>,
    failures: usize,
}
impl Report {
    fn check(&mut self, name: &str, ok: bool, detail: String) {
        self.lines.push(format!("{} {} — {}", if ok { "PASS" } else { "FAIL" }, name, detail));
        if !ok {
            self.failures += 1;
        }
    }
    fn expect_detects(&mut self, name: &str, wrong_differs: bool, detail: String) {
        // A wrong variant must be *detected* (i.e. differ from the correct run).
        self.check(name, wrong_differs, detail);
    }
}

fn main() {
    let mut rep = Report { lines: Vec::new(), failures: 0 };
    let adv50 = vec![Request::Advance { to: LogicalTime(50) }];

    // C1: identical canonical results under different budgets.
    let mut e1 = fresh(Mode::Correct, false);
    let (_, calls1) = run_all(&mut e1, &adv50, 1);
    let mut e3 = fresh(Mode::Correct, false);
    let (_, calls3) = run_all(&mut e3, &adv50, 3);
    let mut eu = fresh(Mode::Correct, false);
    let (_, callsu) = run_all(&mut eu, &adv50, u32::MAX);
    rep.check(
        "C1 budget-invariance",
        e1.trace == eu.trace && e3.trace == eu.trace && e1.boundary_digest() == eu.boundary_digest() && e3.boundary_digest() == eu.boundary_digest(),
        format!("budget1={} calls, budget3={} calls, unbudgeted={} call(s); {} cohorts; diagnostics differ: {}", calls1, calls3, callsu, eu.trace.len(), e1.diagnostics != eu.diagnostics),
    );

    // C2: one large catch-up equals many small catch-ups (with decay and work landing inside).
    let steps: Vec<Request> = (1..=10).map(|i| Request::Advance { to: LogicalTime(i * 5) }).collect();
    let mut es = fresh(Mode::Correct, false);
    let (_, calls_s) = run_all(&mut es, &steps, u32::MAX);
    rep.check(
        "C2 catch-up partition equivalence",
        es.trace == eu.trace && es.boundary_digest() == eu.boundary_digest(),
        format!("10 x Advance(+5) in {} calls vs 1 x Advance(50): traces equal, boundary digests equal; per-cohort pre-engine digests equal at every cohort", calls_s),
    );

    // C3: second request waits through the first request's pause; backpressure.
    let mut e = fresh(Mode::Correct, false);
    let mut mb = Mailbox::new(2);
    assert_eq!(mb.offer(Request::Advance { to: LogicalTime(50) }), Offer::Accepted);
    let o1 = consumer_step(&mut mb, &mut e, 1); // pauses after cohort@10
    let paused = matches!(o1, Some(Outcome::Paused { .. }));
    let offer2 = mb.offer(Request::Input { command_id: "cmd-1".into(), at: LogicalTime(60), effects: vec![Effect::AddDelta { target: (def("pressure.drought"), scope(ScopeKind::Settlement, "pontafique")), delta: 5 }] });
    let offer3 = mb.offer(Request::Advance { to: LogicalTime(70) }); // capacity 2 -> full
    let head_still_first = matches!(mb.queue.front(), Some(Request::Advance { to }) if to.0 == 50);
    let trace_len_after_pause = e.trace.len();
    let log = drain_mailbox(&mut mb, &mut e, 1);
    let input_ran_last = e.trace.last().map(|r| r.at.0 == 60).unwrap_or(false);
    rep.check(
        "C3 second request waits while first is paused",
        paused && offer2 == Offer::Accepted && offer3 == Offer::MailboxFull && head_still_first && trace_len_after_pause == 1 && input_ran_last && e.clock.now().0 == 60,
        format!("first call paused={}, second accepted={:?}, third={:?}, head unchanged={}, remaining calls={}, input executed last at t=60, clock=60", paused, offer2, offer3, head_still_first, log.len()),
    );

    // C4: delayed work within and beyond the horizon.
    let within = eu.trace.iter().any(|r| r.at.0 == 15) && eu.trace.iter().any(|r| r.at.0 == 35);
    let beyond_resident = eu.scheduler.next_due_time() == Some(LogicalTime(135));
    let mut later = eu.clone();
    let (_, _) = run_all(&mut later, &[Request::Advance { to: LogicalTime(200) }], u32::MAX);
    let beyond_ran = later.trace.last().map(|r| r.at.0 == 135 && r.committed.iter().any(|(k, v)| k.0.as_str() == "wildlife.population" && *v == 0)).unwrap_or(false);
    rep.check(
        "C4 delayed work within and beyond horizon",
        within && beyond_resident && beyond_ran,
        format!("work created at 10 ran at 15 and at 35 inside Advance(50); work due 135 stayed resident; ran under Advance(200): {}", beyond_ran),
    );

    // C5: failure path — zero-delay feedback rejects the whole cohort atomically; identical under budgets.
    let mut ed1 = fresh(Mode::Correct, true);
    run_all(&mut ed1, &adv50, 1);
    let mut edu = fresh(Mode::Correct, true);
    run_all(&mut edu, &adv50, u32::MAX);
    let rej = edu.trace.iter().find(|r| r.at.0 == 40);
    let rejected_cleanly = rej.map(|r| r.rejection.is_some() && r.committed.is_empty() && r.scheduled.is_empty()).unwrap_or(false);
    let drought_after = edu.read(&(def("pressure.drought"), scope(ScopeKind::Settlement, "pontafique")), LogicalTime(50), None);
    let key_consumed = edu.scheduler.next_due_time() == Some(LogicalTime(135));
    rep.check(
        "C5 typed cohort rejection is atomic and budget-invariant",
        rejected_cleanly && ed1.trace == edu.trace && key_consumed && drought_after < 999,
        format!("cohort@40 rejected ({:?}); nothing committed (+999 discarded, drought={}), WorkKey consumed, traces equal under budget 1 and unbudgeted", rej.and_then(|r| r.rejection.clone()), drought_after),
    );

    // C6: past-dated request and duplicate command rejected non-canonically.
    let mut e6 = eu.clone();
    let before = e6.boundary_digest();
    let past = e6.process(&Request::Input { command_id: "late".into(), at: LogicalTime(30), effects: vec![] }, 8);
    let ok1 = e6.process(&Request::Input { command_id: "c9".into(), at: LogicalTime(50), effects: vec![] }, 8);
    let dup = e6.process(&Request::Input { command_id: "c9".into(), at: LogicalTime(55), effects: vec![] }, 8);
    rep.check(
        "C6 past-dated and duplicate requests rejected without mutation",
        matches!(past, Outcome::Rejected(RejectReason::HorizonBehindClock { .. })) && matches!(ok1, Outcome::Completed { .. }) && matches!(dup, Outcome::Rejected(RejectReason::DuplicateCommand { .. })) && e6.trace.len() == eu.trace.len() + 1,
        format!("Input@30 after clock=50 -> {:?}; equal-time input accepted; redelivered command_id -> {:?}; digest before rejection {}", past, dup, &format!("{}", before)[..12]),
    );

    // C7: snapshot at a pause boundary, restore, resume: identical to uninterrupted.
    let mut e7 = fresh(Mode::Correct, false);
    let mut mb7 = Mailbox::new(4);
    mb7.offer(Request::Advance { to: LogicalTime(50) });
    let _ = consumer_step(&mut mb7, &mut e7, 2); // paused after two cohorts
    let snapshot = e7.clone(); // everything retained is a field of Engine
    let durable_mailbox = mb7.clone();
    drop(e7);
    let mut restored = snapshot;
    restored.diagnostics.clear(); // noncanonical; not part of the snapshot contract
    let mut mb_r = durable_mailbox;
    drain_mailbox(&mut mb_r, &mut restored, 2);
    rep.check(
        "C7 snapshot at pause + resume equals uninterrupted run",
        restored.trace == eu.trace && restored.boundary_digest() == eu.boundary_digest(),
        "snapshot = cells + scheduler + bodies + occurrences + finalized + clock; durable mailbox re-presents the head".to_string(),
    );

    // W1: the inherited drain-whole-prefix shape must be detected.
    let mut w1 = fresh(Mode::WrongDrainWholePrefix, false);
    run_all(&mut w1, &adv50, u32::MAX);
    let mut w1s = fresh(Mode::WrongDrainWholePrefix, false);
    run_all(&mut w1s, &steps, u32::MAX);
    let pre_digests_differ = w1.trace.iter().zip(eu.trace.iter()).any(|(a, b)| a.pre_engine_digest != b.pre_engine_digest);
    rep.expect_detects(
        "W1 detects drain-whole-prefix defect (V3-F01 shape)",
        w1.trace != eu.trace && w1.trace != w1s.trace,
        format!("one-call trace has {} cohorts vs {} correct (created-inside-horizon work skipped); pre-engine digests differ: {}; stepwise vs one-call differ within the wrong variant itself", w1.trace.len(), eu.trace.len(), pre_digests_differ),
    );

    // W2: evaluating at the horizon instead of due time must be detected.
    let mut w2 = fresh(Mode::WrongEvalAtHorizon, false);
    run_all(&mut w2, &adv50, u32::MAX);
    let mut w2s = fresh(Mode::WrongEvalAtHorizon, false);
    run_all(&mut w2s, &steps, u32::MAX);
    rep.expect_detects(
        "W2 detects evaluate-at-horizon defect",
        w2.trace != w2s.trace,
        "one-call vs stepwise traces differ under the wrong variant (decay elapsed and created due times depend on horizon); correct variant: equal".to_string(),
    );

    // C8 (self-challenge counterexample, expected limitation): three items at one due time
    // form one cohort; one defective item discards all three, and budget 1 cannot pause
    // inside the cohort. This documents the transaction-unit tradeoff; it is not a pass/fail
    // of the design but of the experiment's ability to expose it.
    let mut e8 = Engine::new(profile(), ruleset(), Mode::Correct);
    let s8 = scope(ScopeKind::Settlement, "pontafique");
    for (i, producer) in ["trigger.a", "trigger.b", "trigger.c"].iter().enumerate() {
        let mut effects = vec![Effect::AddDelta { target: (def("pressure.drought"), s8.clone()), delta: 10 + i as i64 }];
        if i == 1 {
            effects.push(Effect::ScheduleWork { producer: def(producer), scope: s8.clone(), delay: 0, body: WorkBody { effects: vec![] } });
        }
        e8.seed_work(LogicalTime(20), producer, &s8, WorkBody { effects });
    }
    let o8 = e8.process(&Request::Advance { to: LogicalTime(20) }, 1);
    let one_cohort_three_items = e8.trace.len() == 1 && e8.trace[0].rejection.is_some() && e8.cells.is_empty();
    rep.check(
        "C8 counterexample exposed: whole-time-slice transaction unit",
        matches!(o8, Outcome::Completed { .. }) && one_cohort_three_items,
        "3 same-time items = 1 cohort: one zero-delay defect discarded all three (+10,+11,+12 never committed); budget 1 admitted all three at once. Smallest correction = per-WorkKey transactions in key order (Operator decision, see 04_SELF_CHALLENGE.md)".to_string(),
    );

    // T1: noncanonical telemetry (queue depth, calls waited, wall time) does not touch canonical results.
    let t0 = std::time::Instant::now();
    let mut et = fresh(Mode::Correct, false);
    let mut mbt = Mailbox::new(8);
    mbt.offer(Request::Advance { to: LogicalTime(50) });
    mbt.offer(Request::Advance { to: LogicalTime(60) });
    let depth_at_offer = mbt.queue.len();
    let mut calls_waited_by_second = 0u32;
    let mut per_call_ns = Vec::new();
    while let Some(o) = { let c = std::time::Instant::now(); let r = consumer_step(&mut mbt, &mut et, 1); per_call_ns.push(c.elapsed().as_nanos()); r } {
        if mbt.queue.len() == 2 && matches!(o, Outcome::Paused { .. }) {
            calls_waited_by_second += 1;
        }
    }
    let wall_ns = t0.elapsed().as_nanos();
    let canonical_prefix_equal = et.trace.iter().take(eu.trace.len()).cloned().collect::<Vec<_>>() == eu.trace;
    rep.check(
        "T1 telemetry is observable and canonically inert",
        canonical_prefix_equal && calls_waited_by_second == 3,
        format!("queue depth at offer={}, second request waited {} paused calls, {} calls total, wall={}us (toy model timing, NOT Spark throughput); canonical prefix equals unbudgeted run", depth_at_offer, calls_waited_by_second, per_call_ns.len(), wall_ns / 1000),
    );

    // S1 (self-challenge A3): "pause retains nothing" is false if the clock moves only at
    // completion. Advance(50) pauses after cohorts through t=35 committed; the mailbox is lost;
    // the host offers Advance(20) then Input@20. CompletionOnly admits both and writes a cell
    // backwards in time (updated_at 35 -> 20). PerCohort rejects Advance(20).
    let req50 = Request::Advance { to: LogicalTime(50) };
    // Three paused calls (budget 1) commit cohorts 10, 15, 30. The mailbox is then lost.
    let mut eco = fresh(Mode::Correct, false); eco.clock_policy = ClockPolicy::CompletionOnly;
    let mut epc = fresh(Mode::Correct, false); epc.clock_policy = ClockPolicy::PerCohort;
    for e in [&mut eco, &mut epc] { for _ in 0..3 { let _ = e.process(&req50, 1); } }
    let hunger = (def("exposure.hunger"), scope(ScopeKind::Household, "h1")); // updated_at = 15
    let late_adv = Request::Advance { to: LogicalTime(12) };
    let late_in = Request::Input { command_id: "late-obs".into(), at: LogicalTime(12), effects: vec![Effect::AddDelta { target: hunger.clone(), delta: 1 }] };
    let co_adv = eco.process(&late_adv, 8);
    let co_in = eco.process(&late_in, 8);
    let co_backwards = eco.cells.get(&hunger).map(|c| c.updated_at.0 == 12).unwrap_or(false);
    let pc_adv = epc.process(&late_adv, 8);
    let pc_in = epc.process(&late_in, 8);
    let pc_clock = epc.clock.now().0;
    rep.check(
        "S1 self-challenge: clock-at-completion-only admits backwards writes; per-cohort clock rejects",
        matches!(co_adv, Outcome::Completed { .. }) && matches!(co_in, Outcome::Completed { .. }) && co_backwards
            && matches!(pc_adv, Outcome::Rejected(RejectReason::HorizonBehindClock { .. })) && matches!(pc_in, Outcome::Rejected(RejectReason::HorizonBehindClock { .. })) && pc_clock == 30,
        format!("CompletionOnly: Advance(12) and Input@12 accepted after a pause that committed t=30; hunger.updated_at rewritten 15->12 (backwards). PerCohort: both rejected, clock={} = simulated-through time", pc_clock),
    );

    // C9 (D1 evidence): per-WorkKey transactions keep budget/partition/snapshot invariance and
    // shrink the C8 blast radius to the defective item alone.
    let per_item = |mode: Mode, defect: bool| { let mut e = fresh(mode, defect); e.txn_unit = TxnUnit::PerWorkKey; e };
    let mut p1 = per_item(Mode::Correct, false); run_all(&mut p1, &adv50, 1);
    let mut pu = per_item(Mode::Correct, false); run_all(&mut pu, &adv50, u32::MAX);
    let mut ps = per_item(Mode::Correct, false); run_all(&mut ps, &steps, u32::MAX);
    let mut p8 = Engine::new(profile(), ruleset(), Mode::Correct); p8.txn_unit = TxnUnit::PerWorkKey;
    for (i, producer) in ["trigger.a", "trigger.b", "trigger.c"].iter().enumerate() {
        let mut effects = vec![Effect::AddDelta { target: (def("pressure.drought"), s8.clone()), delta: 10 + i as i64 }];
        if i == 1 { effects.push(Effect::ScheduleWork { producer: def(producer), scope: s8.clone(), delay: 0, body: WorkBody { effects: vec![] } }); }
        p8.seed_work(LogicalTime(20), producer, &s8, WorkBody { effects });
    }
    let _ = p8.process(&Request::Advance { to: LogicalTime(20) }, 8);
    let drought8 = p8.read(&(def("pressure.drought"), s8.clone()), LogicalTime(20), None);
    let rejected_only_b = p8.trace.len() == 3 && p8.trace.iter().filter(|r| r.rejection.is_some()).count() == 1;
    rep.check(
        "C9 per-WorkKey unit: invariance holds, blast radius shrinks",
        p1.trace == pu.trace && ps.trace == pu.trace && p1.boundary_digest() == pu.boundary_digest() && ps.boundary_digest() == pu.boundary_digest() && rejected_only_b && drought8 == 22,
        format!("budget 1 / unbudgeted / stepwise equal under PerWorkKey; C8 fixture: 3 records, only trigger.b rejected, drought = 10 + 12 = {}", drought8),
    );

    println!("RESEARCH / PROPOSED DESIGN — NOT ADOPTED. spark-fable-example results:");
    for l in &rep.lines {
        println!("{}", l);
    }
    println!("--- canonical trace (correct, Advance(50)): (at, batch_digest[..12], committed, scheduled, rejected)");
    for t in trace_summary(&eu) {
        println!("  {:?}", t);
    }
    println!("--- final boundary digest: {}", eu.boundary_digest());
    println!("{} check(s) failed", rep.failures);
    if rep.failures > 0 {
        std::process::exit(1);
    }
}
