//! RESEARCH — synthetic GAME-shaped workload drill. All workloads synthetic; all timings
//! machine-specific toy-model numbers, NOT Spark throughput. Not a benchmark.
use spark_core::clock::LogicalTime;
use spark_core::scope::ScopeKind;
use spark_fable_example_pass2::*;
use std::collections::BTreeMap;
use std::time::Instant;

fn main() {
    println!("RESEARCH / NOT ADOPTED — synthetic workload drill (machine-specific toy timings)");
    println!("budget | steps | max_step_us | total_engine_us | max_queue_depth | backpressured | completion_latency_steps(per request)");
    for budget in [1u32, 4, 16, u32::MAX] {
        let s = scope(ScopeKind::Settlement, "pontafique");
        let mut e = Engine::new(profile(), Ruleset { decay_per_tick: BTreeMap::new(), thresholds: vec![], max_wave_depth: 2 }, Mode::Correct);
        e.txn_unit = TxnUnit::PerWorkKey;
        // same-time timer burst: 64 actors due at t=100 (daily tick shape), each schedules its next tick (+100)
        for i in 0..64u64 {
            let name = format!("a{}", i); let a = scope(ScopeKind::Actor, &name);
            e.seed_work(LogicalTime(100), "timer.daily", &a, WorkBody { effects: vec![Effect::AddDelta { target: (def("need.food"), a.clone()), delta: 1 }, Effect::ScheduleWork { producer: def("timer.daily"), scope: a.clone(), delay: 100, body: WorkBody { effects: vec![] } }] });
        }
        // trickle: observations at t=10,20,...,90 then Advance(100), then Advance(150)
        let mut mb = Mailbox::new(4);
        let mut pending: Vec<Request> = (1..=9).map(|i| Request::Input { command_id: format!("obs-{}", i), at: LogicalTime(i * 10), effects: vec![Effect::AddDelta { target: (def("pressure.drought"), s.clone()), delta: 1 }] }).collect();
        pending.push(Request::Advance { to: LogicalTime(100) });
        pending.push(Request::Advance { to: LogicalTime(150) });
        let total = pending.len();
        let (mut steps, mut max_step, mut engine_total, mut max_depth, mut backpressured) = (0u32, 0u128, 0u128, 0usize, 0u32);
        let mut latency: Vec<u32> = vec![];
        let mut started_at_step: Option<u32> = None;
        let mut idx = 0;
        loop {
            // producer offers one request per step (slow consumer shape): backpressure counted, retried next step
            if idx < total { match mb.offer(pending[idx].clone()) { Offer::Accepted => idx += 1, Offer::MailboxFull => backpressured += 1 } }
            max_depth = max_depth.max(mb.queue.len());
            if mb.queue.is_empty() { if idx >= total { break; } else { continue; } }
            if started_at_step.is_none() { started_at_step = Some(steps); }
            let t = Instant::now();
            let o = consumer_step(&mut mb, &mut e, budget);
            let us = t.elapsed().as_micros();
            steps += 1; engine_total += us; max_step = max_step.max(us);
            if let Some(Outcome::Completed { .. }) | Some(Outcome::Rejected(_)) = o { latency.push(steps - started_at_step.unwrap_or(steps)); started_at_step = None; }
        }
        println!("{:>6} | {:>5} | {:>11} | {:>15} | {:>15} | {:>13} | {:?}", if budget == u32::MAX { "inf".to_string() } else { budget.to_string() }, steps, max_step, engine_total, max_depth, backpressured, latency);
        assert_eq!(e.trace.len(), 9 + 64, "canonical record count is budget-independent (next ticks at 200 stay resident beyond Advance(150))");
    }
    println!("note: a transaction-count budget bounds transactions per step, not wall time; queue waiting (steps before start) is separate from engine time (per-step duration).");
}
