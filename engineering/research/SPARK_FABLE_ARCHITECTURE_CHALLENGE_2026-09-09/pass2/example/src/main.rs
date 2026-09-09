//! RESEARCH / PROPOSED DESIGN — NOT ADOPTED. Pass-2 adversarial checks. Exit 1 on any FAIL.
use spark_core::clock::LogicalTime;
use spark_core::hash::Digest;
use spark_core::scope::ScopeKind;
use spark_fable_example_pass2::*;
use std::collections::BTreeMap;

fn rules() -> Ruleset {
    let s = scope(ScopeKind::Settlement, "pontafique");
    let h = scope(ScopeKind::Household, "h1");
    Ruleset {
        decay_per_tick: BTreeMap::new(),
        thresholds: vec![ThresholdRule { id: def("rule.t"), watch: (def("pressure.food_scarcity"), s.clone()), at_least: 20, emit: Effect::AddDelta { target: (def("exposure.hunger"), h), delta: 20 } }],
        max_wave_depth: 4,
    }
}
fn engine(unit: TxnUnit) -> Engine { let mut e = Engine::new(profile(), rules(), Mode::Correct); e.txn_unit = unit; e }
fn add(target: &str, sc: &spark_core::scope::ScopeId, d: i64) -> Effect { Effect::AddDelta { target: (def(target), sc.clone()), delta: d } }
fn three_items(e: &mut Engine, with_defect: bool) {
    let s = scope(ScopeKind::Settlement, "pontafique");
    for (i, p) in ["trigger.a", "trigger.b", "trigger.c"].iter().enumerate() {
        let mut effects = vec![add("pressure.drought", &s, 10 + i as i64)];
        if with_defect && i == 1 { effects.push(Effect::ScheduleWork { producer: def(p), scope: s.clone(), delay: 0, body: WorkBody { effects: vec![] } }); }
        e.seed_work(LogicalTime(20), p, &s, WorkBody { effects });
    }
}
struct Rep { lines: Vec<String>, failures: usize }
impl Rep { fn check(&mut self, n: &str, ok: bool, d: String) { self.lines.push(format!("{} {} — {}", if ok { "PASS" } else { "FAIL" }, n, d)); if !ok { self.failures += 1; } } }

fn main() {
    let mut rep = Rep { lines: vec![], failures: 0 };
    let adv20 = Request::Advance { to: LogicalTime(20) };

    // P1: real brakes. Three same-time items, budget 1: exactly one transaction, two resident.
    let mut e = engine(TxnUnit::PerWorkKey); three_items(&mut e, false);
    let o1 = e.process(&adv20, 1);
    let one_done = e.trace.len() == 1 && e.scheduler.len() == 2 && matches!(o1, Outcome::Paused { .. });
    let snap = e.clone(); // in-memory restore (no codec): every retained field is on Engine
    drop(e);
    let mut r = snap; r.diagnostics.clear();
    let mut outs = vec![]; loop { let o = r.process(&adv20, 1); let done = !matches!(o, Outcome::Paused { .. }); outs.push(o); if done { break; } }
    let mut u = engine(TxnUnit::PerWorkKey); three_items(&mut u, false);
    let ou = u.process(&adv20, 9);
    let report_r = match outs.last() { Some(Outcome::Completed { report, .. }) => Some(report.clone()), _ => None };
    let report_u = match &ou { Outcome::Completed { report, .. } => Some(report.clone()), _ => None };
    rep.check("P1 brakes: budget 1 executes one item, two stay resident; pause/snapshot/resume == uninterrupted",
        one_done && r.trace == u.trace && r.boundary_digest() == u.boundary_digest() && r.cells == u.cells && report_r.as_ref().map(|x| x.records.len()) == Some(3) && report_r.as_ref().map(|x| &x.records) == report_u.as_ref().map(|x| &x.records),
        format!("after call 1: records={}, resident={}; resumed in {} more calls; 3 records in both reports; traces, cells, boundary digests equal", 1, 2, outs.len()));

    // P1b: per-item vs time-slice observable divergence: A lowers scarcity 25->5, B raises 5->25 across threshold 20.
    let mk = |unit| { let mut e = engine(unit); let s = scope(ScopeKind::Settlement, "pontafique");
        e.cells.insert((def("pressure.food_scarcity"), s.clone()), Cell { value: 25, updated_at: LogicalTime(0) });
        e.seed_work(LogicalTime(5), "trigger.a", &s, WorkBody { effects: vec![add("pressure.food_scarcity", &s, -20)] });
        e.seed_work(LogicalTime(5), "trigger.b", &s, WorkBody { effects: vec![add("pressure.food_scarcity", &s, 20)] }); e };
    let mut slice = mk(TxnUnit::TimeSlice); slice.process(&Request::Advance { to: LogicalTime(5) }, 9);
    let mut item = mk(TxnUnit::PerWorkKey); item.process(&Request::Advance { to: LogicalTime(5) }, 9);
    let hunger = (def("exposure.hunger"), scope(ScopeKind::Household, "h1"));
    let hs = slice.read(&hunger, LogicalTime(5), None); let hi = item.read(&hunger, LogicalTime(5), None);
    let mut sd = engine(TxnUnit::TimeSlice); three_items(&mut sd, true); sd.process(&adv20, 9);
    let mut idf = engine(TxnUnit::PerWorkKey); three_items(&mut idf, true); idf.process(&adv20, 9);
    rep.check("P1b per-item vs time-slice: threshold crossing and failure isolation differ observably", hs == 0 && hi == 20 && sd.cells.is_empty() && idf.cells.len() == 1,
        format!("same-time A(-20) then B(+20) on scarcity 25 with threshold 20: slice hunger={} (net 0, no crossing), per-item hunger={} (B crosses); one defective item: slice commits nothing, per-item commits the other two", hs, hi));

    // P2: report contract through the mailbox: successes + rejection through pauses; isolation; across restore.
    let build = || { let mut e = engine(TxnUnit::PerWorkKey); three_items(&mut e, true); let s = scope(ScopeKind::Settlement, "pontafique"); e.seed_work(LogicalTime(30), "trigger.d", &s, WorkBody { effects: vec![add("pressure.drought", &s, 1)] }); e };
    let mut e2 = build(); let mut mb = Mailbox::new(4);
    mb.offer(Request::Advance { to: LogicalTime(20) }); mb.offer(Request::Advance { to: LogicalTime(30) });
    let mut reports = vec![]; let mut calls = 0;
    // pause twice, then snapshot/restore mid-request, then finish
    for _ in 0..2 { calls += 1; let _ = consumer_step(&mut mb, &mut e2, 1); }
    let mut e2 = e2.clone(); e2.diagnostics.clear(); // in-memory restore; mailbox is host state and is kept
    while let Some(o) = consumer_step(&mut mb, &mut e2, 1) { calls += 1; if let Outcome::Completed { report, .. } = o { reports.push(report); } }
    let ok2 = reports.len() == 2 && reports[0].records.len() == 3 && reports[0].records.iter().all(|r| r.at.0 == 20) && reports[0].records.iter().filter(|r| r.rejection.is_some()).count() == 1
        && reports[1].records.len() == 1 && reports[1].records[0].at.0 == 30 && e2.undelivered.is_empty()
        && reports.iter().flat_map(|r| r.records.iter()).cloned().collect::<Vec<_>>() == e2.trace;
    rep.check("P2 report contract: exactly own records in order across pauses and in-memory restore; next request isolated", ok2,
        format!("{} calls; report 1 = 3 records at t=20 (2 committed, 1 rejected), report 2 = 1 record at t=30; undelivered empty; concatenation == trace. Restore is an in-memory clone: NO codec, NO crash-recovery or exactly-once claim", calls));

    // P3: hashing attack — same parent key, same child key, different child payloads, advance only to 10.
    let run3 = |child_delta: i64| { let mut e = engine(TxnUnit::PerWorkKey); let s = scope(ScopeKind::Settlement, "pontafique");
        e.seed_work(LogicalTime(10), "trigger.p", &s, WorkBody { effects: vec![Effect::ScheduleWork { producer: def("rule.child"), scope: s.clone(), delay: 5, body: WorkBody { effects: vec![add("pressure.drought", &s, child_delta)] } }] });
        e.process(&Request::Advance { to: LogicalTime(10) }, 9); e };
    let a = run3(1); let b = run3(2);
    let same_key = a.trace[0].scheduled[0].0 == b.trace[0].scheduled[0].0;
    rep.check("P3 batch digest distinguishes different child payloads under the same child key", same_key && a.trace[0].batch_digest != b.trace[0].batch_digest && a.trace[0] != b.trace[0],
        format!("same child WorkKey: {}; batch digests differ: {}; records differ: {}; (engine digests differ: {} — the scheduler commits to payload hashes regardless)", same_key, a.trace[0].batch_digest != b.trace[0].batch_digest, a.trace[0] != b.trace[0], a.engine_digest() != b.engine_digest()));

    // P3b: missing body and body/digest mismatch are typed rejections, never silent success.
    let mut m = engine(TxnUnit::PerWorkKey); let s = scope(ScopeKind::Settlement, "pontafique");
    m.seed_work(LogicalTime(10), "trigger.p", &s, WorkBody { effects: vec![add("pressure.drought", &s, 5)] }); m.bodies.clear();
    m.process(&Request::Advance { to: LogicalTime(10) }, 9);
    let missing_rejected = matches!(m.trace.first().map(|r| &r.rejection), Some(Some(CohortRejection::MissingBody { .. })));
    let mut c = engine(TxnUnit::PerWorkKey);
    let k = c.seed_work(LogicalTime(10), "trigger.p", &s, WorkBody { effects: vec![add("pressure.drought", &s, 5)] });
    let advertised = WorkBody { effects: vec![add("pressure.drought", &s, 5)] }.digest();
    c.bodies.insert(advertised.clone(), WorkBody { effects: vec![add("pressure.drought", &s, 500)] }); // corrupted content
    c.process(&Request::Advance { to: LogicalTime(10) }, 9);
    let mismatch_rejected = matches!(c.trace.first().map(|r| &r.rejection), Some(Some(CohortRejection::BodyDigestMismatch { .. }))) && c.cells.is_empty();
    let _ = (k, Digest::ZERO);
    rep.check("P3b missing body / digest mismatch rejected, nothing written", missing_rejected && mismatch_rejected, format!("missing body -> MissingBody: {}; corrupted body -> BodyDigestMismatch with no write: {}", missing_rejected, mismatch_rejected));

    // P4: commit-time key collision -> committed poison, report matches state; poisoned-only slice.
    let mut p = engine(TxnUnit::PerWorkKey);
    // Resident child key (rule.child, settlement, occurrence 0) at t=15 with payload X; the counter is then rewound (as a lost-counter restore would) so the rule reissues occurrence 0 with payload Y.
    p.seed_work(LogicalTime(15), "rule.child", &s, WorkBody { effects: vec![add("pressure.drought", &s, 1)] });
    p.occurrences.clear();
    p.seed_work(LogicalTime(10), "trigger.p", &s, WorkBody { effects: vec![add("pressure.food_scarcity", &s, 7), Effect::ScheduleWork { producer: def("rule.child"), scope: s.clone(), delay: 5, body: WorkBody { effects: vec![add("pressure.drought", &s, 2)] } }] });
    p.process(&Request::Advance { to: LogicalTime(10) }, 9);
    let rec = p.trace.last().cloned().unwrap();
    let child_key = rec.conflicts.first().cloned();
    let poisoned = child_key.as_ref().map(|k| p.scheduler.slot_status(k) == spark_core::scheduler::WorkSlotStatus::Conflicted).unwrap_or(false);
    let cell_committed = p.read(&(def("pressure.food_scarcity"), s.clone()), LogicalTime(10), None) == 7;
    let o15 = p.process(&Request::Advance { to: LogicalTime(15) }, 9);
    let poisoned_slice = matches!(&o15, Outcome::Completed { report, .. } if report.records.len() == 1 && report.records[0].conflicts.len() == 1 && report.records[0].committed.is_empty() && report.records[0].rejection.is_none()) && p.scheduler.is_empty();
    rep.check("P4 commit-time collision = committed poison; report matches state; poisoned-only slice reported not executed", rec.rejection.is_none() && rec.scheduled.is_empty() && poisoned && cell_committed && poisoned_slice,
        format!("cell write committed (scarcity=7), contested key listed under conflicts and Conflicted in the scheduler; at t=15 the poisoned slot drains as a conflict record with no execution (drought stays {})", p.read(&(def("pressure.drought"), s.clone()), LogicalTime(15), None)));

    // P4b: arithmetic near limits: delay that overflows -> TimeOverflow, no partial writes; identical in debug and release.
    let mut q = engine(TxnUnit::PerWorkKey);
    q.seed_work(LogicalTime(10), "trigger.p", &s, WorkBody { effects: vec![add("pressure.drought", &s, 9), Effect::ScheduleWork { producer: def("rule.child"), scope: s.clone(), delay: u64::MAX - 5, body: WorkBody { effects: vec![] } }] });
    q.process(&Request::Advance { to: LogicalTime(10) }, 9);
    let ov = matches!(q.trace.last().map(|r| &r.rejection), Some(Some(CohortRejection::TimeOverflow { .. }))) && q.cells.is_empty() && q.scheduler.is_empty();
    let mut q2 = engine(TxnUnit::PerWorkKey); q2.occurrences.insert((def("rule.child"), s.clone()), u64::MAX);
    q2.seed_work(LogicalTime(10), "trigger.p", &s, WorkBody { effects: vec![add("pressure.drought", &s, 9), Effect::ScheduleWork { producer: def("rule.child"), scope: s.clone(), delay: 1, body: WorkBody { effects: vec![] } }] });
    q2.process(&Request::Advance { to: LogicalTime(10) }, 9);
    let ov2 = matches!(q2.trace.last().map(|r| &r.rejection), Some(Some(CohortRejection::OccurrenceOverflow { .. }))) && q2.cells.is_empty();
    rep.check("P4b overflow: delay u64::MAX-5 at now=10 -> TimeOverflow; occurrence u64::MAX -> OccurrenceOverflow; no partial writes", ov && ov2,
        format!("build profile: {}; TimeOverflow rejected cleanly: {}; OccurrenceOverflow rejected cleanly: {}", if cfg!(debug_assertions) { "debug" } else { "release" }, ov, ov2));

    println!("RESEARCH / PROPOSED DESIGN — NOT ADOPTED. pass-2 results ({} profile):", if cfg!(debug_assertions) { "debug" } else { "release" });
    for l in &rep.lines { println!("{}", l); }
    println!("{} check(s) failed", rep.failures);
    if rep.failures > 0 { std::process::exit(1); }
}
