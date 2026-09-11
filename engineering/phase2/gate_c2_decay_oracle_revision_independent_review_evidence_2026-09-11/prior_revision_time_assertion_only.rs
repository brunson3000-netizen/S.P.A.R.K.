//! Independent reviewer probes. Candidate dependencies are unchanged.
#![cfg(test)]
use spark_core::{
    authority::Authority,
    clock::LogicalTime,
    hash::{CanonicalEncoder, Digest},
    value::{CanonicalValue, FixedPoint},
};
use spark_engine::profile::{manifest::ProfileManifest, text::BoundedText};
use spark_engine::{
    engine::{Engine, ExtractionRefusal, RestoreError},
    fixture,
    report::CohortOutcome,
    request::{CommandPayload, Outcome, Request},
    rules::*,
};
use spark_testkit::phase2::*;

fn decay_engine(times: &[u64], initial_value: i64, rate: i64, cadence: i64) -> Engine {
    let mut f = standard();
    let setup = on_command(
        "rule.seed",
        "cmd.seed",
        vec![emit(
            "seed",
            "state.stress",
            Update::Assign(lit(initial_value)),
        )],
    );
    let d = work_rule(
        "rule.decay",
        "work.decay",
        vec![emit("d", "state.stress", decay(rate, cadence))],
    );
    let g = f.genesis_with(
        budgets(50),
        vec![setup, d],
        vec![baseline("state.stress", 0)],
        times
            .iter()
            .map(|t| initial("rule.decay", &actor("a"), *t, "work.decay"))
            .collect(),
    );
    let mut e = Engine::genesis(g).unwrap();
    drive(&mut e, &command("cmd.seed", 0, 1, "cmd.seed", actor("a")));
    e
}
fn cell_state(e: &Engine) -> (i64, LogicalTime) {
    let c = e
        .state()
        .get(&profile_id(), &def("state.stress"), &actor("a"))
        .unwrap();
    (value(e, "state.stress", &actor("a")).unwrap(), c.updated_at)
}
fn activate_epoch(
    e: &mut Engine,
    time: u64,
    seq: u64,
    rs: ActivatedRuleSet,
    cfg: spark_engine::profile::config::ConfigRevision,
) {
    let mut c = command_request(
        &format!("cmd.epoch{seq}"),
        time,
        seq,
        "spark.epoch.activate",
        actor("a"),
        vec![],
        vec![],
    );
    c.payload = CommandPayload::ActivateEpoch {
        rule_set: rs,
        config: cfg,
    };
    let result = drive(e, &Request::Command(c));
    assert!(matches!(
        reports(&result).last().unwrap().outcome,
        CohortOutcome::EpochActivated { .. }
    ));
}
#[test]
fn c2_05_saturation_must_preserve_claimed_value_and_commit_time_chunk_invariance() {
    let mut one = decay_engine(&[150], 100, 10, 10);
    let mut chunks = decay_engine(&[50, 100, 150], 100, 10, 10);
    drive(&mut one, &advance(150));
    drive(&mut chunks, &advance(150));
    assert_eq!(cell_state(&one).0, 0);
    assert_eq!(cell_state(&chunks).0, 0);
    println!(
        "saturation single={:?} chunks={:?}",
        cell_state(&one),
        cell_state(&chunks)
    );
    assert_eq!(
        cell_state(&one),
        cell_state(&chunks),
        "R-2 claims value AND commit-time invariance for arbitrary evaluation times"
    );
}
#[test]
fn c2_05_shorter_cadence_must_not_bill_steps_ending_before_activation() {
    let mut f = standard();
    let seed = on_command(
        "rule.seed",
        "cmd.seed",
        vec![emit("s", "state.stress", Update::Assign(lit(100)))],
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
        budgets(50),
        vec![seed, d],
        vec![baseline("state.stress", 0)],
        vec![initial("rule.d", &actor("a"), 60, "work.d")],
    );
    g.config = config(vec![
        ("tune.rate", CanonicalValue::Int(1)),
        ("tune.cadence", CanonicalValue::Int(100)),
    ]);
    let rs = g.rule_set.clone();
    let mut e = Engine::genesis(g).unwrap();
    drive(&mut e, &command("cmd.seed", 0, 1, "cmd.seed", actor("a")));
    activate_epoch(
        &mut e,
        50,
        2,
        rs,
        config(vec![
            ("tune.rate", CanonicalValue::Int(5)),
            ("tune.cadence", CanonicalValue::Int(10)),
        ]),
    );
    drive(&mut e, &advance(60));
    println!(
        "shortened cadence result={:?}; new-rate step ends must be after barrier 50",
        cell_state(&e)
    );
    assert_eq!(
        cell_state(&e).0,
        95,
        "only the step ending at 60 belongs to the new epoch; 10,20,30,40,50 cannot use its rate"
    );
}
#[test]
fn c2_05_effect_reapplication_at_frozen_cohort_time_must_reproduce_state() {
    let mut e = decay_engine(&[15], 100, 10, 10);
    let mut clone = e.clone();
    let r = reports(&drive(&mut e, &advance(15)));
    assert_eq!(r[0].canonical_time, LogicalTime(15));
    assert_eq!(cell_state(&e).0, 90);
    fixture::extract_least_due_slice(&mut clone, LogicalTime(15)).unwrap();
    fixture::apply_committed_effects(&mut clone, &r[0].waves[0].committed, LogicalTime(15));
    println!(
        "actual={:?}; frozen-at reapplication={:?}",
        cell_state(&e),
        cell_state(&clone)
    );
    assert_eq!(
        e.engine_state_digest(),
        clone.engine_state_digest(),
        "FINAL §4 and AT-I1: resolved effects at canonical cohort time reproduce the wave"
    );
}
#[test]
fn c2_05_recovery_and_unsaturated_remainders_positive() {
    for start in [-73, 73] {
        let mut one = decay_engine(&[42], start, 7, 10);
        let mut chunks = decay_engine(&[11, 27, 42], start, 7, 10);
        drive(&mut one, &advance(42));
        drive(&mut chunks, &advance(42));
        assert_eq!(cell_state(&one), cell_state(&chunks));
        assert_eq!(
            cell_state(&one),
            (if start < 0 { -45 } else { 45 }, LogicalTime(42))
        );
        assert_eq!(
            one.state()
                .get(&profile_id(), &def("state.stress"), &actor("a"))
                .unwrap()
                .baseline,
            Some(CanonicalValue::Int(0))
        );
    }
    let mut f = standard();
    let d = work_rule(
        "rule.d",
        "work.d",
        vec![emit("d", "state.stress", decay(1, 1))],
    );
    assert!(f
        .rule_set(budgets(5), vec![d])
        .unwrap_err()
        .iter()
        .any(|e| matches!(e, RuleSetError::DecayWithoutBaseline { .. })));
}
#[test]
fn c2_02_scope_fanout_and_materialized_edge_controls() {
    for delayed in [false, true] {
        for count in [2, 3] {
            let effects = (0..count)
                .map(|i| {
                    emit_at(
                        &format!("e{i}"),
                        "state.stress",
                        actor(&format!("s{i}")),
                        Update::Add(lit(2)),
                    )
                })
                .collect();
            let mut r = on_command("rule.r", "cmd.r", if delayed { vec![] } else { effects });
            if delayed {
                r.schedules.push(ScheduleOp {
                    sub_id: tag("later"),
                    delay: 1,
                    scope: ScopeRef::Subject,
                    work_kind: work("work.x"),
                    mode: ScheduleMode::Materialize {
                        effects: (0..count)
                            .map(|i| {
                                emit_at(
                                    &format!("e{i}"),
                                    "state.stress",
                                    actor(&format!("s{i}")),
                                    Update::Add(lit(2)),
                                )
                            })
                            .collect(),
                    },
                });
            }
            let mut b = budgets(5);
            b.max_fan_out = 2;
            let mut f = standard();
            assert_eq!(
                f.rule_set(b, vec![r]).is_ok(),
                count == 2,
                "delayed={delayed}, count={count}"
            );
        }
    }
}
#[test]
fn c2_03_nested_ids_share_namespace_across_schedules() {
    for same in [true, false] {
        let mut f = standard();
        let mut r = on_command("rule.r", "cmd.r", vec![]);
        for (s, id) in [
            ("first", "nested"),
            ("second", if same { "nested" } else { "other" }),
        ] {
            r.schedules.push(ScheduleOp {
                sub_id: tag(s),
                delay: 1,
                scope: ScopeRef::Subject,
                work_kind: work("work.x"),
                mode: ScheduleMode::Materialize {
                    effects: vec![emit(id, "state.stress", Update::Add(lit(9)))],
                },
            });
        }
        let result = f.rule_set(budgets(5), vec![r]);
        if same {
            assert!(result
                .unwrap_err()
                .iter()
                .any(|e| matches!(e, RuleSetError::DuplicateSubId { .. })));
        } else {
            assert!(result.is_ok());
        }
    }
}
#[test]
fn c2_04_negative_widened_aggregate_and_staged_cancellation() {
    let m = ProfileManifest::new(
        profile_id(),
        BoundedText::new("wide").unwrap(),
        vec![
            int_spec("state.stress", Authority::SparkOwned, i64::MIN, i64::MAX),
            int_spec("state.total", Authority::Derived, i64::MIN, i64::MAX),
        ],
    );
    let mut f = activate(&m);
    let seed = on_command(
        "rule.seed",
        "cmd.seed",
        vec![
            emit_at(
                "a",
                "state.stress",
                actor("a"),
                Update::Assign(lit(i64::MIN)),
            ),
            emit_at(
                "b",
                "state.stress",
                actor("b"),
                Update::Assign(lit(i64::MIN)),
            ),
        ],
    );
    let sum = on_command(
        "rule.sum",
        "cmd.sum",
        vec![emit(
            "sum",
            "state.total",
            Update::Aggregate {
                source: def("state.stress"),
                weight: FixedPoint::from_raw(500000),
            },
        )],
    );
    let body = on_command(
        "rule.body",
        "cmd.body",
        vec![
            emit("a", "state.stress", Update::Add(lit(i64::MAX))),
            emit("b", "state.stress", Update::Add(lit(i64::MAX))),
            emit("c", "state.stress", Update::Subtract(lit(i64::MAX))),
        ],
    );
    let mut e = f.engine(budgets(5), vec![seed, sum, body], vec![]);
    drive(&mut e, &command("cmd.seed", 0, 1, "cmd.seed", actor("a")));
    drive(&mut e, &command("cmd.sum", 1, 2, "cmd.sum", actor("a")));
    assert_eq!(value(&e, "state.total", &actor("a")), Some(i64::MIN));
    drive(&mut e, &command("cmd.body", 2, 3, "cmd.body", actor("c")));
    assert_eq!(value(&e, "state.stress", &actor("c")), Some(i64::MAX));
}
#[test]
fn c2_06_both_kind_payload_mismatches_leave_artifacts_unchanged() {
    let mut f = standard();
    let mut e = f.engine(budgets(4), vec![], vec![]);
    let before = e.epoch_registry().canonical_digest();
    let mut c = command_request("cmd.bad", 1, 1, "cmd.ordinary", actor("a"), vec![], vec![]);
    c.payload = CommandPayload::ActivateEpoch {
        rule_set: e.rule_set().clone(),
        config: e.config().clone(),
    };
    for req in [
        Request::Command(c),
        command("cmd.bad2", 2, 2, "spark.epoch.activate", actor("a")),
    ] {
        let rr = drive(&mut e, &req);
        assert_eq!(rr.last().unwrap().outcome(), &Outcome::Completed);
        assert!(matches!(
            reports(&rr).last().unwrap().outcome,
            CohortOutcome::EpochActivationRejected { .. }
        ));
        assert_eq!(e.epoch_registry().canonical_digest(), before);
    }
}
#[test]
fn d_c2_11_mismatch_before_first_cohort_is_sticky_without_boundary() {
    let mut f = standard();
    let mut e = f.engine(
        budgets(5),
        vec![work_rule(
            "rule.r",
            "work.r",
            vec![emit("e", "state.stress", Update::Add(lit(1)))],
        )],
        vec![initial("rule.r", &actor("a"), 10, "work.r")],
    );
    let key = e.obligations().keys().next().unwrap().clone();
    fixture::tamper_obligations(&mut e, &key, None);
    let r = e.process(&advance(10));
    assert!(matches!(
        r.outcome(),
        Outcome::StoreInvariantViolated(ExtractionRefusal::ObligationRecordMissing { .. })
    ));
    assert!(r.reports().is_empty());
    assert!(!r.terminates(r.presented()));
    assert!(e.snapshot().is_err());
    assert_eq!(e.process(&advance(11)).outcome(), r.outcome());
}
#[test]
fn r3_lineage_restore_binds_multiple_activations_across_reset() {
    let mut f = standard();
    let g = f.genesis(budgets(5), vec![], vec![]);
    let profile = g.profile.clone();
    let mut e = Engine::genesis(g).unwrap();
    let rs = e.rule_set().clone();
    activate_epoch(&mut e, 10, 1, rs.clone(), config(vec![]));
    e.reset_timeline_epoch(
        spark_core::timeline::TimelineEpoch(1),
        spark_core::id::SourceId::new("seq.two").unwrap(),
    )
    .unwrap();
    let mut c = command_request(
        "cmd.next",
        20,
        2,
        "spark.epoch.activate",
        actor("a"),
        vec![],
        vec![],
    );
    c.timeline_epoch = spark_core::timeline::TimelineEpoch(1);
    c.payload = CommandPayload::ActivateEpoch {
        rule_set: rs,
        config: config(vec![]),
    };
    drive(&mut e, &Request::Command(c));
    assert_eq!(e.behavior_epoch(), 3);
    let s = e.snapshot().unwrap();
    let good = Engine::restore(s.clone(), &profile).unwrap();
    assert_eq!(good.stable_boundary_digest(), e.stable_boundary_digest());
    for index in [0, 1, 2] {
        let mut bad = s.clone();
        fixture::snapshot_set_activation_time(&mut bad, index, Some(LogicalTime(999)));
        assert_eq!(
            Engine::restore(bad, &profile).unwrap_err(),
            RestoreError::EpochLineageInvalid
        );
    }
}
fn parent_union(ids: &[Digest]) -> Digest {
    let mut ids = ids.to_vec();
    ids.sort();
    ids.dedup();
    let mut x = CanonicalEncoder::new();
    x.push_str("parents_emission_v1");
    x.push_u64(ids.len() as u64);
    for id in ids {
        x.push_bytes(id.as_bytes());
    }
    x.finish()
}
#[test]
fn c2_01_fixed_scope_condition_reads_join_parent_union_once() {
    let mut f = standard();
    let seed = work_rule(
        "rule.seed",
        "work.seed",
        vec![
            emit("s", "state.stress", Update::Add(lit(2))),
            emit_at("m", "state.mood", actor("b"), Update::Add(lit(3))),
        ],
    );
    let mut watcher = rule(
        "rule.watch",
        Trigger::Change {
            watched: def("state.stress"),
        },
        vec![emit("r", "state.echo", Update::Add(lit(1)))],
    );
    let expr = Expr::Input(Input::Cell {
        definition: def("state.mood"),
        scope: ScopeRef::Fixed(actor("b")),
        absent: 0,
    });
    watcher
        .conditions
        .push(compare(expr.clone(), CmpOp::Gt, lit(0)));
    watcher.conditions.push(compare(expr, CmpOp::Lt, lit(10)));
    let fingerprint = watcher.fingerprint();
    let mut e = f.engine(
        budgets(5),
        vec![seed, watcher],
        vec![initial("rule.seed", &actor("a"), 10, "work.seed")],
    );
    let r = reports(&drive(&mut e, &advance(10)));
    assert_eq!(r[0].waves.len(), 2);
    let parents: Vec<_> = r[0].waves[0]
        .committed
        .iter()
        .flat_map(|c| c.provenance.retained.clone())
        .collect();
    assert_eq!(parents.len(), 2);
    let formula = |ids: &[Digest]| {
        let mut x = CanonicalEncoder::new();
        x.push_str("emission");
        x.push_digest(&fixture::observation(&e).prewave[0].cohort_identity);
        x.push_u32(1);
        x.push_digest(&parent_union(ids));
        x.push_digest(&fingerprint);
        x.push_u64(1);
        tag("r").canonicalize(&mut x);
        actor("a").canonicalize(&mut x);
        def("state.echo").canonicalize(&mut x);
        actor("a").canonicalize(&mut x);
        x.push_digest(&e.epoch_registry().current().unwrap().record_hash());
        x.finish()
    };
    let actual = &r[0].waves[1].committed[0].provenance.retained[0];
    assert_eq!(actual, &formula(&parents));
    for id in &parents {
        assert_ne!(actual, &formula(std::slice::from_ref(id)));
    }
}
#[test]
fn d_c2_7_exact_rule_identity_and_barrier_config_controls() {
    for changed in [false, true] {
        let mut f = standard();
        let d = work_rule(
            "rule.r",
            "work.r",
            vec![emit(
                "e",
                "state.stress",
                Update::Add(Expr::Input(Input::Config {
                    key: def("tune.rate"),
                })),
            )],
        );
        let mut g = f.genesis(
            budgets(5),
            vec![d.clone()],
            vec![initial("rule.r", &actor("a"), 30, "work.r")],
        );
        g.config = config(vec![("tune.rate", CanonicalValue::Int(2))]);
        let mut e = Engine::genesis(g).unwrap();
        let mut revised = d;
        if changed {
            revised.emits[0].update = Update::Add(lit(99));
        }
        let rs = f.rule_set(budgets(5), vec![revised]).unwrap();
        activate_epoch(
            &mut e,
            20,
            1,
            rs,
            config(vec![("tune.rate", CanonicalValue::Int(7))]),
        );
        let r = reports(&drive(&mut e, &advance(30)));
        if changed {
            assert!(matches!(
                r[0].outcome,
                CohortOutcome::ObligationRefused(
                    spark_engine::report::ObligationRefusal::RuleSuperseded { .. }
                )
            ));
            assert_eq!(value(&e, "state.stress", &actor("a")), None);
        } else {
            assert_eq!(value(&e, "state.stress", &actor("a")), Some(7));
        }
    }
}
#[test]
fn d_c2_13_materialized_creator_fingerprint_survives_epoch_replacement() {
    let run = |delta| {
        let mut f = standard();
        let mut r = on_command("rule.r", "cmd.r", vec![]);
        r.schedules.push(ScheduleOp {
            sub_id: tag("later"),
            delay: 20,
            scope: ScopeRef::Subject,
            work_kind: work("work.m"),
            mode: ScheduleMode::Materialize {
                effects: vec![emit("m", "state.stress", Update::Add(param(0)))],
            },
        });
        let fp = r.fingerprint();
        let mut e = f.engine(budgets(5), vec![r], vec![]);
        drive(
            &mut e,
            &Request::Command(command_request(
                "cmd.make",
                1,
                1,
                "cmd.r",
                actor("a"),
                vec![],
                vec![delta],
            )),
        );
        let rec = e
            .obligations()
            .get(e.obligations().keys().next().unwrap())
            .unwrap()
            .sole_record()
            .unwrap()
            .clone();
        assert_eq!(rec.creator_rule_fingerprint(), &fp);
        let rs = f.rule_set(budgets(5), vec![]).unwrap();
        activate_epoch(&mut e, 10, 2, rs, config(vec![]));
        let result = reports(&drive(&mut e, &advance(21)));
        assert_eq!(value(&e, "state.stress", &actor("a")), Some(delta));
        let actual = result[0].waves[0].committed[0].provenance.retained[0].clone();
        let mut parent = CanonicalEncoder::new();
        parent.push_str("parents_workkey_v1");
        parent.push_digest(&rec.key().identity_digest());
        let art = e.epoch_registry().current().unwrap().record_hash();
        let mut expected = CanonicalEncoder::new();
        expected.push_str("emission");
        expected.push_digest(
            &fixture::observation(&e)
                .prewave
                .last()
                .unwrap()
                .cohort_identity,
        );
        expected.push_u32(0);
        expected.push_digest(&parent.finish());
        expected.push_digest(&fp);
        expected.push_u64(1);
        tag("m").canonicalize(&mut expected);
        actor("a").canonicalize(&mut expected);
        def("state.stress").canonicalize(&mut expected);
        actor("a").canonicalize(&mut expected);
        expected.push_digest(&art);
        assert_eq!(
            actual,
            expected.finish(),
            "the creator fingerprint survives replacement"
        );
        (actual, rec.record_hash(), art)
    };
    let a = run(-9);
    let b = run(17);
    // Activation binds the preceding fence hash, so these later epoch artifacts
    // legitimately differ with the earlier parameter-bearing command payload.
    assert_ne!(a.2, b.2);
    assert_ne!(a.0, b.0);
    assert_ne!(a.1, b.1);
}
#[test]
fn r8_enqueue_redelivery_folds_and_contested_payload_rejects_atomically() {
    let build = || {
        let mut f = standard();
        let mut r = work_rule("rule.r", "work.r", vec![]);
        r.schedules.push(ScheduleOp {
            sub_id: tag("later"),
            delay: 5,
            scope: ScopeRef::Subject,
            work_kind: work("work.m"),
            mode: ScheduleMode::Materialize {
                effects: vec![emit("m", "state.stress", Update::Add(param(0)))],
            },
        });
        f.engine(
            budgets(5),
            vec![r],
            vec![initial("rule.r", &actor("a"), 10, "work.r")],
        )
    };
    let mut plain = build();
    let mut duplicate = build();
    fixture::set_seed_duplication(&mut duplicate, Some(false));
    assert_eq!(
        reports(&drive(&mut plain, &advance(10))),
        reports(&drive(&mut duplicate, &advance(10)))
    );
    assert_eq!(
        plain.stable_boundary_digest(),
        duplicate.stable_boundary_digest()
    );
    assert_eq!(duplicate.scheduled_work_count(), 1);
    let mut contested = build();
    fixture::set_seed_duplication(&mut contested, Some(true));
    let r = reports(&drive(&mut contested, &advance(10)));
    assert!(matches!(
        r[0].outcome,
        CohortOutcome::Rejected {
            rejection: spark_engine::report::WaveRejection::ContestedEmission { .. },
            ..
        }
    ));
    assert_eq!(contested.scheduled_work_count(), 0);
    assert_eq!(
        &contested.engine_state_digest(),
        fixture::observation(&contested).prewave[0]
            .engine_digest
            .value()
    );
}
