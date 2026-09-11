// Mechanically adapted copy of the review's
// `gate_c2_independent_review_evidence_2026-09-11/independent_counterexamples.rs`.
// The original is run first, byte-unchanged (see README). It cannot compile
// against the corrected candidate, because two corrections the review itself
// required remove surfaces it uses. Every adaptation is listed here; no
// assertion, fixture value, or expected value is changed.
//
//   A1  `refusal_type_must_not_be_constructible_externally` is removed. Its
//       *compiling* was the C2-08 counterexample; after the correction it must
//       not compile, which the unchanged run's compile error and the repository
//       probe "C2-08 Rev2 §10 a unit FinalizationRefusal is not constructible"
//       demonstrate.
//   A2  `report.cohort_identity` (the field FINAL AT-I46(a) forbids, C2-07) is
//       read through the test-support observation seam instead:
//       `fixture::observation(&e).prewave[0].cohort_identity`.
//   A3  v1 Q7 (C2-05): a decay rule's target must declare baseline semantics,
//       and the rule has no `toward` operand. The two decay probes declare the
//       baseline 0 their `toward: lit(0)` expressed, via `genesis_with`, and
//       spell rate/cadence as `Param`s (`decay(10, 10)` /
//       `decay_config_rate("tune.rate", 10)`); values are unchanged.
//   A4  Because of A2, the disposable crate enables spark-engine's
//       `test-support` feature (as spark-testkit's dev-dependency edge does):
//       without it the seam is absent, which is AT-I46(b) working — an
//       external production consumer cannot observe the pre-wave cohort
//       identity. The runner records the flag in the log header.
#![cfg(test)]
use spark_testkit::phase2::*;
use spark_engine::rules::*;
use spark_engine::request::{CommandPayload, Request};

use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::value::FixedPoint;
use spark_core::authority::Authority;
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::profile::text::BoundedText;

#[test]
fn fanout_must_count_scope_breadth() {
    let mut f = standard();
    let mut b = budgets(10); b.max_fan_out = 1;
    let r = on_command("rule.spread", "cmd.go", vec![
        emit_at("a", "state.stress", actor("a"), Update::Add(lit(1))),
        emit_at("b", "state.stress", actor("b"), Update::Add(lit(1))),
    ]);
    let minted = f.rule_set(b, vec![r]);
    assert!(minted.is_err(), "door admitted two distinct target scopes with max_fan_out=1");
}

#[test]
fn materialized_subids_must_be_unique() {
    let mut f = standard();
    let mut r = on_command("rule.delay", "cmd.go", vec![]);
    r.schedules.push(ScheduleOp { sub_id: tag("delay"), delay: 1, work_kind: work("work.one"), scope: ScopeRef::Subject,
        mode: ScheduleMode::Materialize { effects: vec![
            emit("same", "state.stress", Update::Add(lit(10))),
            emit("same", "state.stress", Update::Add(lit(15))),
        ]}});
    let minted = f.rule_set(budgets(10), vec![r]);
    assert!(minted.is_err(), "door admitted duplicate emitting sub-IDs inside Materialize");
}

#[test]
fn aggregate_keeps_i128_until_after_weighting() {
    let manifest = ProfileManifest::new(profile_id(), BoundedText::new("wide").unwrap(), vec![
        int_spec("state.stress", Authority::SparkOwned, i64::MIN, i64::MAX),
        int_spec("state.total", Authority::Derived, i64::MIN, i64::MAX),
    ]);
    let mut f = activate(&manifest);
    let setup = on_command("rule.setup", "cmd.setup", vec![
        emit_at("a", "state.stress", actor("a"), Update::Assign(lit(i64::MAX))),
        emit_at("b", "state.stress", actor("b"), Update::Assign(lit(i64::MAX))),
    ]);
    let aggregate = on_command("rule.aggregate", "cmd.aggregate", vec![emit("sum", "state.total", Update::Aggregate { source: def("state.stress"), weight: FixedPoint::from_raw(500_000) })]);
    let mut e = f.engine(budgets(10), vec![setup, aggregate], vec![]);
    e.process(&command("cmd.setup", 0, 1, "cmd.setup", actor("a")));
    assert_eq!(value(&e,"state.stress",&actor("b")), Some(i64::MAX));
    let result = e.process(&command("cmd.aggregate", 1, 2, "cmd.aggregate", actor("a")));
    println!("aggregate outcome: {:?}", result.reports());
    assert_eq!(value(&e,"state.total",&actor("a")), Some(i64::MAX), "floor((MAX+MAX)*0.5) fits i64");
}

fn parent_hash(parents: &[Digest]) -> Digest {
    let mut p = parents.to_vec(); p.sort(); p.dedup();
    let mut e = CanonicalEncoder::new(); e.push_str("parents_emission_v1"); e.push_u64(p.len() as u64);
    for x in p { e.push_bytes(x.as_bytes()); } e.finish()
}
fn schedule_identity(cohort: &Digest, parents: &[Digest], r: &RuleSpec, artifact: &Digest) -> Digest {
    let mut e=CanonicalEncoder::new(); e.push_str("emission"); e.push_digest(cohort); e.push_u32(1);
    e.push_digest(&parent_hash(parents)); e.push_digest(&r.fingerprint()); e.push_u64(1); tag("later").canonicalize(&mut e);
    actor("a").canonicalize(&mut e); def("rule.watch").canonicalize(&mut e); actor("a").canonicalize(&mut e); e.push_digest(artifact); e.finish()
}
#[test]
fn derived_identity_binds_every_changed_eligibility_input() {
    let mut f=standard();
    let setup=work_rule("rule.setup", "work.setup", vec![
        emit("x","state.stress",Update::Add(lit(1))), emit("y","state.mood",Update::Add(lit(1))) ]);
    let mut watch=rule("rule.watch",Trigger::Change { watched:def("state.stress") },vec![]);
    watch.conditions.push(compare(Expr::Input(cell("state.mood")),CmpOp::Gt,lit(0)));
    watch.schedules.push(reevaluate_after("later",1,"work.leaf","rule.leaf"));
    let leaf=work_rule("rule.leaf","work.leaf",vec![]);
    let mut e=f.engine(budgets(10),vec![setup,watch.clone(),leaf],vec![initial("rule.setup",&actor("a"),10,"work.setup")]);
    let result=e.process(&advance(10)); let report=&result.reports()[0];
    let changed=&report.waves[0].committed;
    assert_eq!(changed.len(),2);
    let all:Vec<Digest>=changed.iter().flat_map(|c|c.provenance.retained.clone()).collect();
    let stress=changed.iter().find(|c|c.definition==def("state.stress")).unwrap().provenance.retained.clone();
    let artifact=e.epoch_registry().current().unwrap().record_hash();
    // A2
    let cohort=&spark_engine::fixture::observation(&e).prewave[0].cohort_identity;
    let actual=e.obligations().keys().next().and_then(|k|e.obligations().get(k)).unwrap().sole_record().unwrap().creator_emission_identity();
    assert_ne!(actual,&schedule_identity(cohort,&stress,&watch,&artifact),"negative control: implementation binds watched target alone");
    assert_eq!(actual,&schedule_identity(cohort,&all,&watch,&artifact),"required union includes changed mood used to become eligible");
}

#[test]
fn epoch_activation_requires_reserved_command_kind() {
    let mut f=standard(); let mut e=f.engine(budgets(10),vec![],vec![]);
    let rs=f.rule_set(budgets(10),vec![]).unwrap();
    let mut c=command_request("cmd.wrong",1,1,"cmd.ordinary",actor("a"),vec![],vec![]);
    c.payload=CommandPayload::ActivateEpoch { rule_set:rs, config:config(vec![]) };
    let result=e.process(&Request::Command(c)); println!("epoch outcome: {:?}",result.reports());
    assert_eq!(e.behavior_epoch(),1,"ordinary command kind must not activate an epoch");
}

#[test]
fn decay_preserves_elapsed_cadence_remainder() {
    fn run(times: &[u64]) -> i64 {
        let mut f=standard();
        let setup=on_command("rule.setup","cmd.setup",vec![emit("x","state.stress",Update::Assign(lit(100)))]);
        // A3
        let decay=work_rule("rule.decay","work.decay",vec![emit("x","state.stress",decay(10,10))]);
        let mut e=spark_engine::engine::Engine::genesis(f.genesis_with(budgets(10),vec![setup,decay],vec![baseline("state.stress",0)],times.iter().map(|t|initial("rule.decay",&actor("a"),*t,"work.decay")).collect())).unwrap();
        e.process(&command("cmd.setup",0,1,"cmd.setup",actor("a")));
        drive(&mut e,&advance(20)); value(&e,"state.stress",&actor("a")).unwrap()
    }
    assert_eq!(run(&[10,20]),80,"aligned positive control");
    assert_eq!(run(&[20]),80,"one-shot positive control");
    assert_eq!(run(&[15,20]),80,"five units of remainder at 15 must survive to 20");
}

#[test]
fn new_decay_rate_must_not_apply_before_activation_barrier() {
    let mut f=standard();
    let setup=on_command("rule.setup","cmd.setup",vec![emit("x","state.stress",Update::Assign(lit(100)))]);
    // A3
    let decay=work_rule("rule.decay","work.decay",vec![emit("x","state.stress",decay_config_rate("tune.rate",10))]);
    let mut g=f.genesis_with(budgets(10),vec![setup,decay],vec![baseline("state.stress",0)],vec![initial("rule.decay",&actor("a"),60,"work.decay")]);
    g.config=config(vec![("tune.rate",spark_core::value::CanonicalValue::Int(1))]);
    let rs=g.rule_set.clone(); let mut e=spark_engine::engine::Engine::genesis(g).unwrap();
    e.process(&command("cmd.setup",0,1,"cmd.setup",actor("a")));
    let mut c=command_request("cmd.activate",50,2,"spark.epoch.activate",actor("a"),vec![],vec![]);
    c.payload=CommandPayload::ActivateEpoch { rule_set:rs,config:config(vec![("tune.rate",spark_core::value::CanonicalValue::Int(5))]) };
    e.process(&Request::Command(c));
    assert_eq!(e.behavior_epoch(),2);
    let result=e.process(&advance(60)); println!("hot-tuned decay outcome: {:?}",result.reports());
    assert_eq!(value(&e,"state.stress",&actor("a")),Some(90),"5 steps at rate 1 before barrier 50, 1 step at rate 5 after; never six steps at rate 5");
}

#[test]
fn positive_trailing_same_frontier_resets_replay_in_index_order() {
    let mut f=standard(); let g=f.genesis(budgets(3),vec![],vec![]);
    let mut e=spark_engine::engine::Engine::genesis(g.clone()).unwrap();
    e.process(&advance(20));
    e.reset_timeline_epoch(spark_core::timeline::TimelineEpoch(1),spark_core::id::SourceId::new("seq.one").unwrap()).unwrap();
    e.reset_timeline_epoch(spark_core::timeline::TimelineEpoch(2),spark_core::id::SourceId::new("seq.two").unwrap()).unwrap();
    let mut resets=e.epoch_resets().to_vec(); resets.reverse();
    let h=spark_engine::engine::CompletedHistory { commands:vec![], resets, frontier:e.frontier(),recorded_history_digest:e.timeline_history_digest(),recorded_stable_boundary_digest:e.stable_boundary_digest() };
    let rebuilt=spark_engine::engine::Engine::reconstruct_completed(g,&h).unwrap();
    assert_eq!(rebuilt.timeline_state_digest(),e.timeline_state_digest());
    assert_eq!(rebuilt.stable_boundary_digest(),e.stable_boundary_digest());
    assert_eq!(rebuilt.epoch_resets(),e.epoch_resets());
}

#[test]
fn positive_command_active_encoding_has_kind_horizon_and_identity() {
    let c=command("cmd.one",20,1,"cmd.go",actor("a")); let d=c.discriminator();
    fn string(out:&mut Vec<u8>,s:&str) { out.extend_from_slice(&(s.len() as u64).to_le_bytes()); out.extend_from_slice(s.as_bytes()); }
    let mut expected=vec![]; string(&mut expected,"active_request.some"); string(&mut expected,"command");
    expected.extend_from_slice(&20u64.to_le_bytes()); expected.extend_from_slice(d.identity().as_bytes());
    let mut actual=CanonicalEncoder::new(); spark_engine::request::canonicalize_active_request(Some(&d),&mut actual);
    assert_eq!(actual.into_bytes(),expected);
}
