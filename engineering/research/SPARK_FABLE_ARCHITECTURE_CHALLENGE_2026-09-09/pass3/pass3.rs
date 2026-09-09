//! RESEARCH / PROPOSED DESIGN — NOT ADOPTED. Pass 3: smallest complete flow-control and
//! result-delivery design, exercised on the real research engine. Crash boundaries are an
//! in-memory fault-injection MODEL (clone-and-drop); they do not prove process durability.
use spark_core::clock::LogicalTime;
use spark_core::scope::ScopeKind;
use spark_fable_example_pass2::*;
use std::collections::BTreeMap;

/// One outstanding request; results pulled by GAME; one retained report until acknowledged.
/// Output is bounded by `chunk` records per report: a request that produces more pauses at
/// the chunk boundary (a delivery boundary, never a causal one) and continues after ack.
#[derive(Clone)]
struct Device { engine: Engine, active: Option<Request>, retained: Option<Chunk>, next_seq: u64, chunk: u32, acked_through: u64 }
#[derive(Clone, Debug, PartialEq, Eq)]
struct Chunk { seq: u64, records: Vec<CohortRecord>, final_for_request: bool, clock: LogicalTime }
#[derive(Debug, PartialEq, Eq)]
enum OfferReply { Accepted, Busy, UnackedReport, Refused(RejectReason) }
impl Device {
    fn new(e: Engine, chunk: u32) -> Self { Device { engine: e, active: None, retained: None, next_seq: 1, chunk, acked_through: 0 } }
    /// Backpressure is the `Busy`/`UnackedReport` reply: GAME keeps its own event; nothing is queued in Spark.
    fn offer(&mut self, r: Request) -> OfferReply {
        if self.retained.is_some() { return OfferReply::UnackedReport; }
        if self.active.is_some() { return OfferReply::Busy; }
        // Early, non-canonical refusal of stale/duplicate requests (also re-checked by the engine).
        if r.horizon() < self.engine.clock.now() { return OfferReply::Refused(RejectReason::HorizonBehindClock { horizon: r.horizon(), clock: self.engine.clock.now() }); }
        if let Request::Input { command_id, .. } = &r { if self.engine.finalized.contains_key(command_id) { return OfferReply::Refused(RejectReason::DuplicateCommand { command_id: command_id.clone() }); } }
        self.active = Some(r); OfferReply::Accepted
    }
    /// One bounded step of work. Produces a chunk when the chunk fills or the request completes.
    fn step(&mut self, host_budget: u32) {
        if self.retained.is_some() { return; } // output storage full: pause, canonical results untouched
        let Some(req) = self.active.clone() else { return };
        let room = self.chunk.saturating_sub(self.engine.undelivered.len() as u32).max(1);
        let outcome = self.engine.process(&req, host_budget.min(room));
        let done = !matches!(outcome, Outcome::Paused { .. });
        if let Outcome::Completed { report, .. } = &outcome { self.engine.undelivered = report.records.clone(); }
        if done || self.engine.undelivered.len() as u32 >= self.chunk {
            let records = std::mem::take(&mut self.engine.undelivered);
            self.retained = Some(Chunk { seq: self.next_seq, records, final_for_request: done, clock: self.engine.clock.now() });
            self.next_seq += 1;
            if done { self.active = None; }
        }
    }
    fn poll(&self) -> Option<Chunk> { self.retained.clone() }
    fn ack(&mut self, seq: u64) -> bool { match &self.retained { Some(c) if c.seq == seq => { self.acked_through = seq; self.retained = None; true } _ => false } }
}
/// GAME side: applies reports exactly once by sequence number.
#[derive(Clone, Default)]
struct Game { applied_seqs: Vec<u64>, effects_applied: usize, dup_redeliveries: u32 }
impl Game { fn apply(&mut self, c: &Chunk) { if self.applied_seqs.contains(&c.seq) { self.dup_redeliveries += 1; return; } self.applied_seqs.push(c.seq); self.effects_applied += c.records.len(); } }

fn fixture(chunk: u32) -> Device {
    let s = scope(ScopeKind::Settlement, "pontafique");
    let mut e = Engine::new(profile(), Ruleset { decay_per_tick: BTreeMap::new(), thresholds: vec![], max_wave_depth: 2 }, Mode::Correct);
    e.txn_unit = TxnUnit::PerWorkKey;
    for i in 0..12u64 { let n = format!("a{}", i); let a = scope(ScopeKind::Actor, &n); e.seed_work(LogicalTime(10), "timer.daily", &a, WorkBody { effects: vec![Effect::AddDelta { target: (def("need.food"), a.clone()), delta: 1 }] }); }
    let _ = s; Device::new(e, chunk)
}

fn main() {
    let mut fails = 0; let mut say = |ok: bool, n: &str, d: String| { println!("{} {} — {}", if ok { "PASS" } else { "FAIL" }, n, d); if !ok { fails += 1; } };
    let adv = Request::Advance { to: LogicalTime(10) };
    // Reference: unbudgeted, unbounded output, no faults.
    let mut r = fixture(u32::MAX); assert_eq!(r.offer(adv.clone()), OfferReply::Accepted); r.step(u32::MAX);
    let ref_chunk = r.poll().unwrap(); let ref_digest = r.engine.boundary_digest(); let ref_records = ref_chunk.records.clone();

    // B1: GAME offers faster than Spark processes: second offer while active -> Busy (no queue, no loss: GAME still holds it).
    let mut d = fixture(5); let mut g = Game::default();
    assert_eq!(d.offer(adv.clone()), OfferReply::Accepted);
    d.step(2);
    let b1 = d.offer(Request::Advance { to: LogicalTime(20) });
    say(b1 == OfferReply::Busy, "B1 producer faster than consumer -> Busy backpressure, nothing queued or lost", format!("{:?}; active request kept; GAME retains its own event", b1));
    // B3: one request produces 12 records > chunk 5 -> chunked delivery, pause at output boundary.
    d.step(u32::MAX); let c1 = d.poll().unwrap();
    // B2: GAME stops consuming: further steps do nothing (output full), engine state frozen, canonical results unchanged.
    let before = d.engine.boundary_digest(); d.step(u32::MAX); d.step(u32::MAX);
    say(c1.records.len() == 5 && !c1.final_for_request && d.engine.boundary_digest() == before && d.poll() == Some(c1.clone()), "B2/B3 output bounded to chunk=5; Spark pauses while GAME does not consume; state unchanged", format!("chunk seq={} records={} final={}", c1.seq, c1.records.len(), c1.final_for_request));
    g.apply(&c1);
    // B5: ack lost -> poll again returns the same seq; GAME dedups by seq; then ack succeeds.
    let again = d.poll().unwrap(); g.apply(&again);
    say(again.seq == c1.seq && g.dup_redeliveries == 1 && g.effects_applied == 5 && d.ack(c1.seq), "B5 lost ack: redelivery has the same seq; GAME applies once; ack then clears", format!("effects applied={} duplicate redeliveries detected={}", g.effects_applied, g.dup_redeliveries));
    // B4: crash after commit before GAME receives (MODEL: clone-and-drop of the device = engine + retained chunk).
    d.step(u32::MAX); let snapshot = d.clone(); drop(d); let mut d = snapshot; // restarted from persisted device state
    let c2 = d.poll().expect("retained chunk survives restart (model)"); g.apply(&c2); assert!(d.ack(c2.seq));
    d.step(u32::MAX); let c3 = d.poll().unwrap(); g.apply(&c3); assert!(d.ack(c3.seq));
    let all: Vec<CohortRecord> = [c1.records.clone(), c2.records.clone(), c3.records.clone()].concat();
    say(c3.final_for_request && all == ref_records && d.engine.boundary_digest() == ref_digest && g.effects_applied == 12 && d.active.is_none(), "B4 restart after commit, before delivery (modeled): chunks 1..3 == unchunked reference; digest equal; 12 effects applied once", format!("seqs {:?}", g.applied_seqs));
    // B6: old request presented again -> explicit refusal, never silent; new input at clock accepted.
    // Finding: Advance(T) with T == clock is ACCEPTED as a legal empty advance (equal time is allowed); it completes with
    // zero records, never re-executes anything. A truly old request (T < clock) is refused. Tested with Advance(5).
    let old = d.offer(Request::Advance { to: LogicalTime(5) }); let ok = d.offer(Request::Input { command_id: "x1".into(), at: LogicalTime(10), effects: vec![] });
    d.step(u32::MAX); let c4 = d.poll().unwrap(); assert!(d.ack(c4.seq));
    let dup = d.offer(Request::Input { command_id: "x1".into(), at: LogicalTime(10), effects: vec![] });
    say(matches!(old, OfferReply::Refused(RejectReason::HorizonBehindClock { .. })) && ok == OfferReply::Accepted && matches!(dup, OfferReply::Refused(RejectReason::DuplicateCommand { .. })), "B6 redelivered old Advance and duplicate Input refused explicitly", format!("{:?} / {:?}", old, dup));
    // Equivalence across pacing and chunking variations.
    let mut eq = true;
    for (chunk, budget) in [(1u32, 1u32), (3, 2), (7, u32::MAX), (u32::MAX, 4)] {
        let mut v = fixture(chunk); assert_eq!(v.offer(adv.clone()), OfferReply::Accepted); let mut recs = vec![];
        loop { v.step(budget); if let Some(c) = v.poll() { recs.extend(c.records.clone()); let fin = c.final_for_request; v.ack(c.seq); if fin { break; } } }
        eq &= recs == ref_records && v.engine.boundary_digest() == ref_digest;
    }
    say(eq, "EQ records and digests identical across chunk sizes 1/3/7/inf and budgets 1/2/inf/4", "delivery boundaries never change causal results".to_string());
    // Memory inventory after everything is acked.
    say(d.retained.is_none() && d.engine.undelivered.is_empty() && d.engine.trace.len() == 13 && d.engine.bodies.len() == 12, "MEM after ack: retained report 0, undelivered 0; GAP: engine.trace holds 13 records (test oracle) and 12 consumed bodies are never released", format!("finalized ids={} bodies={} scheduler={} -> release rules in PASS3_REPORT.md", d.engine.finalized.len(), d.engine.bodies.len(), d.engine.scheduler.len()));
    println!("{} check(s) failed", fails); if fails > 0 { std::process::exit(1); }
}
