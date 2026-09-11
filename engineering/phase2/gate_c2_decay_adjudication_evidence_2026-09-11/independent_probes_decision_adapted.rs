//! Second-correction reviewer probes. External crate; candidate is unchanged.
#![cfg(test)]
use spark_core::{clock::LogicalTime, timeline::DerivedIndexFault, value::CanonicalValue};
use spark_engine::{engine::{Engine, RestoreError}, fixture, report::CohortOutcome,
    request::{CommandPayload, Request}, rules::*};
use spark_testkit::phase2::*;

fn state(e: &Engine) -> (i64, LogicalTime) {
    let c = e.state().get(&profile_id(), &def("state.stress"), &actor("a")).unwrap();
    (value(e, "state.stress", &actor("a")).unwrap(), c.updated_at)
}
fn run(e: &mut Engine, t: u64) -> Vec<spark_engine::report::CohortReport> {
    reports(&drive(e, &advance(t)))
}
fn make(start: i64, rate: i64, cadence: i64, seed_at: u64, times: &[u64], watch: bool, shock: Option<u64>) -> Engine {
    let mut f = standard();
    let mut rules = vec![
        on_command("rule.seed", "cmd.seed", vec![emit("s", "state.stress", Update::Assign(lit(start)))]),
        work_rule("rule.d", "work.d", vec![emit("d", "state.stress", decay(rate, cadence))]),
        work_rule("rule.shock", "work.shock", vec![emit("s", "state.stress", Update::Add(lit(5)))]),
    ];
    if watch { rules.push(rule("rule.w", Trigger::Change { watched: def("state.stress") },
        vec![emit("e", "state.echo", Update::Add(lit(1)))])); }
    let mut work: Vec<_> = times.iter().map(|t| initial("rule.d", &actor("a"), *t, "work.d")).collect();
    if let Some(t) = shock { work.push(initial("rule.shock", &actor("a"), t, "work.shock")); }
    let mut e = Engine::genesis(f.genesis_with(budgets(50), rules, vec![baseline("state.stress", 0)], work)).unwrap();
    drive(&mut e, &command("cmd.seed", seed_at, 1, "cmd.seed", actor("a")));
    e
}

#[test]
fn r01_recovery_reapplication_and_timestamp() {
    for t in [1, 6, 7, 11, 23, 60] {
        let mut e = make(-19, 3, 7, 0, &[t], false, None);
        let mut clone = e.clone();
        let r = run(&mut e, t);
        assert_eq!(r[0].canonical_time, LogicalTime(t));
        assert_eq!(state(&e), ((-19 + 3 * (t / 7) as i64).min(0), LogicalTime(t)));
        fixture::extract_least_due_slice(&mut clone, LogicalTime(t)).unwrap();
        fixture::apply_committed_effects(&mut clone, &r[0].waves[0].committed, LogicalTime(t));
        assert_eq!(clone.engine_state_digest(), e.engine_state_digest());
    }
}

fn tuned(times: &[u64], barrier: u64, start: i64) -> Engine {
    let mut f = standard();
    let rules = vec![
        on_command("rule.seed", "cmd.seed", vec![emit("s", "state.stress", Update::Assign(lit(start)))]),
        work_rule("rule.d", "work.d", vec![emit("d", "state.stress", Update::Decay {
            rate: Param::Config { key: def("tune.rate") }, cadence: Param::Config { key: def("tune.cadence") }
        })]),
    ];
    let mut g = f.genesis_with(budgets(50), rules, vec![baseline("state.stress",0)],
        times.iter().map(|t| initial("rule.d", &actor("a"), *t, "work.d")).collect());
    g.config = config(vec![("tune.rate",CanonicalValue::Int(2)),("tune.cadence",CanonicalValue::Int(7))]);
    let rs = g.rule_set.clone();
    let mut e = Engine::genesis(g).unwrap();
    drive(&mut e, &command("cmd.seed",0,1,"cmd.seed",actor("a")));
    let mut c = command_request("cmd.epoch",barrier,2,"spark.epoch.activate",actor("a"),vec![],vec![]);
    c.payload = CommandPayload::ActivateEpoch { rule_set: rs,
        config: config(vec![("tune.rate",CanonicalValue::Int(5)),("tune.cadence",CanonicalValue::Int(4))]) };
    let reports = reports(&drive(&mut e, &Request::Command(c)));
    assert!(matches!(reports.last().unwrap().outcome, CohortOutcome::EpochActivated { .. }));
    e
}

#[test]
fn r02_activation_endpoint_is_owned_by_closing_segment() {
    for sign in [-1, 1] {
        // Old steps 7,14,21 (2 each), then new step 25 (5).
        let mut catchup = tuned(&[25],21,100*sign);
        let mut split = tuned(&[21,25],21,100*sign);
        assert_eq!(state(&split), (94*sign,LogicalTime(21)));
        run(&mut catchup,25); run(&mut split,25);
        assert_eq!(state(&catchup),(89*sign,LogicalTime(25)));
        assert_eq!(state(&catchup),state(&split));
    }
}

#[test]
fn r02_residual_is_unbilled_observation() {
    for sign in [-1,1] {
        // Residual (14,20] closes unpaid; new segment first step is 24.
        let mut e = tuned(&[23,24],20,100*sign);
        run(&mut e,23); assert_eq!(state(&e),(96*sign,LogicalTime(23)));
        run(&mut e,24); assert_eq!(state(&e),(91*sign,LogicalTime(24)));
        println!("residual sign={sign}: {:?}",state(&e));
    }
}

#[test]
fn r03_unmoved_writes_fire_watchers_and_reject_shocks() {
    for (start,rate,t) in [(100,0,11),(0,5,11),(100,5,1),(-1,5,11)] {
        let mut e = make(start,rate,7,0,&[t],true,None);
        let before = value(&e,"state.echo",&actor("a")).unwrap();
        let r = run(&mut e,t);
        assert_eq!(r[0].outcome,CohortOutcome::Committed);
        assert_eq!(r[0].waves[0].committed.len(),1);
        assert_eq!(state(&e).1,LogicalTime(t));
        assert_eq!(value(&e,"state.echo",&actor("a")),Some(before+1));
        let mut clash = make(start,rate,7,0,&[t],false,Some(t));
        let pre = clash.state().canonical_state_digest();
        let r = run(&mut clash,t);
        assert!(matches!(r[0].outcome,CohortOutcome::Rejected { .. }));
        assert_eq!(pre,clash.state().canonical_state_digest());
    }
}

#[test]
fn r03_saturated_recovery_partitions() {
    for start in [-11,11] {
        let mut one = make(start,3,7,0,&[61],false,None);
        let mut split = make(start,3,7,0,&[8,17,29,44,61],false,None);
        run(&mut one,61); run(&mut split,61);
        assert_eq!(state(&one),(0,LogicalTime(61)));
        assert_eq!(state(&one),state(&split));
    }
}

#[test]
fn r04_per_wave_store_and_index_discriminator() {
    let mut f = standard();
    let sink = work_rule("rule.sink","work.sink",vec![emit("s","state.energy",Update::Add(lit(1)))]);
    let first = with_schedule(on_command("rule.first","cmd.go",vec![emit("f","state.stress",Update::Add(lit(1)))]),
        reevaluate_after("a",10,"work.sink","rule.sink"));
    let second = with_schedule(rule("rule.second",Trigger::Change { watched: def("state.stress") },
        vec![emit("s","state.mood",Update::Add(lit(1)))]),reevaluate_after("b",20,"work.sink","rule.sink"));
    let g = f.genesis(budgets(50),vec![first,second,sink],vec![]);
    let profile = g.profile.clone();
    let mut e = Engine::genesis(g).unwrap();
    fixture::probe_wave_invariants(&mut e,true);
    drive(&mut e,&command("cmd.go",3,1,"cmd.go",actor("a")));
    let points = &fixture::observation(&e).committed_waves;
    assert_eq!(points.iter().map(|p|(p.wave_index,p.obligation_count,p.scheduled_work_count,p.bidirectional_invariant)).collect::<Vec<_>>(),
        vec![(0,1,1,true),(1,2,2,true)]);
    assert!(fixture::timeline_indexes_recompute(&e));
    let last = e.finalized_commands().last().unwrap();
    let fault = DerivedIndexFault::DropFinalizedCommandIdentity(last.envelope.command_id.clone());
    let mut snapshot = e.snapshot().unwrap();
    fixture::snapshot_inject_timeline_index_fault(&mut snapshot,fault.clone());
    assert_eq!(Engine::restore(snapshot,&profile).unwrap_err(),RestoreError::DerivedIndexesInconsistent);
    fixture::inject_timeline_index_fault(&mut e,fault);
    assert!(!fixture::timeline_indexes_recompute(&e));
}

#[test]
fn r2prime_grid_behavior_is_observable_after_non_decay_write() {
    let mut e = make(100,10,10,9,&[10],false,None);
    assert_eq!(state(&e),(100,LogicalTime(9)));
    run(&mut e,10);
    println!("one logical tick after assignment: {:?}",state(&e));
    assert_eq!(state(&e),(90,LogicalTime(10)));
    let mut shock = make(100,10,10,0,&[30,40],false,Some(31));
    run(&mut shock,31); assert_eq!(state(&shock),(75,LogicalTime(31)));
    run(&mut shock,40); assert_eq!(state(&shock),(65,LogicalTime(40)));
}

#[test]
fn r2prime_frozen_q7_full_cadence_after_fresh_assignment() {
    // No previous decay evaluation, residual, saturation or epoch transition.
    // Q7's inputs: current=100, baseline=0, rate=10, elapsed=10-9=1,
    // cadence=10. Zero WHOLE elapsed steps, so value remains 100.
    let mut e = make(100,10,10,9,&[10],false,None);
    run(&mut e,10);
    assert_eq!(state(&e),(90,LogicalTime(10)),"v1 Q7: no whole cadence has elapsed since updated_at=9");
}

#[test]
fn r2prime_frozen_q7_full_cadence_after_separate_shock() {
    let mut e = make(100,10,10,0,&[30,40],false,Some(31));
    run(&mut e,31); assert_eq!(state(&e),(75,LogicalTime(31)));
    run(&mut e,40);
    assert_eq!(state(&e),(65,LogicalTime(40)),"v2 4.3 retains Q7: nine units after the shock is not a whole cadence");
}
