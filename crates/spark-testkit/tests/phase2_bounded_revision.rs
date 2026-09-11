//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 bounded revision: the corrections required by the independent
//! review at `00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0`.
//!
//! Each of the review's seven failing independent counterexamples is converted
//! here into a repository test (`*_converted`), with the review's fixture and
//! expected value, alongside the negative control that kills the named wrong
//! implementation. Further tests cover the corrected contracts the review
//! required: C2-01 … C2-06, D-C2-7, D-C2-11, D-C2-13, and the engine's one
//! derived index (the epoch artifact lineage). C2-07's report-surface and
//! C2-08's construction probes live in `phase2_compile_probes.rs`.
//!
//! Red-first record: the converted tests were first run, as the review's
//! original external corpus, against the unchanged candidate sources
//! (`gate_c2_bounded_revision_evidence_2026-09-11/original_corpus_pre_correction.txt`,
//! 3 passed / 7 failed) before any production change in this revision.

use spark_core::authority::Authority;
use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, CanonicalEncoder, Digest};
use spark_core::scheduler::{OccurrenceIndex, WorkKey};
use spark_core::scope::ScopeId;
use spark_core::timeline::TimelineEpoch;
use spark_core::value::{CanonicalValue, FixedPoint};
use spark_engine::engine::{Engine, ExtractionRefusal, GenesisError, RestoreError};
use spark_engine::fixture;
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::profile::text::BoundedText;
use spark_engine::report::{CohortOutcome, CohortReport, ObligationRefusal, WaveRejection};
use spark_engine::request::{CommandPayload, CommandRequest, Outcome, Request};
use spark_engine::rules::{
    ActivatedRuleSet, CmpOp, Expr, RuleSetError, RuleSpec, ScheduleMode, ScheduleOp, ScopeRef,
    Trigger, Update,
};
use spark_testkit::phase2::*;

fn bron() -> ScopeId {
    actor("bron")
}

fn add(v: i64) -> Update {
    Update::Add(lit(v))
}

fn run(e: &mut Engine, t: u64) -> Vec<CohortReport> {
    reports(&drive(e, &advance(t)))
}

// ---------------------------------------------------------------- independent formulas
//
// Written from the frozen text (v3 §4.3/§4.4), not by calling the engine.

fn parents_emission(parents: &[Digest]) -> Digest {
    let mut p = parents.to_vec();
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

fn parents_workkey(key: &WorkKey) -> Digest {
    let mut e = CanonicalEncoder::new();
    e.push_str("parents_workkey_v1");
    e.push_digest(&key.identity_digest());
    e.finish()
}

struct Emission<'a> {
    cohort: &'a Digest,
    wave: u32,
    parent: Digest,
    rule_fingerprint: &'a Digest,
    sub_ids: &'a [&'a str],
    producer: &'a ScopeId,
    target: &'a str,
    target_scope: &'a ScopeId,
    artifact: &'a Digest,
}

fn emission(x: &Emission<'_>) -> Digest {
    let mut e = CanonicalEncoder::new();
    e.push_str("emission");
    e.push_digest(x.cohort);
    e.push_u32(x.wave);
    e.push_digest(&x.parent);
    e.push_digest(x.rule_fingerprint);
    e.push_u64(x.sub_ids.len() as u64);
    for s in x.sub_ids {
        tag(s).canonicalize(&mut e);
    }
    x.producer.canonicalize(&mut e);
    def(x.target).canonicalize(&mut e);
    x.target_scope.canonicalize(&mut e);
    e.push_digest(x.artifact);
    e.finish()
}

fn last_cohort(e: &Engine) -> Digest {
    fixture::observation(e)
        .prewave
        .last()
        .unwrap()
        .cohort_identity
        .clone()
}

fn artifact(e: &Engine) -> Digest {
    e.epoch_registry().current().unwrap().record_hash()
}

fn sole_obligation(e: &Engine) -> spark_engine::obligation::ObligationRecord {
    let k = e.obligations().keys().next().unwrap().clone();
    e.obligations()
        .get(&k)
        .unwrap()
        .sole_record()
        .unwrap()
        .clone()
}

// ================================================================ C2-01

fn watch_rule(schedule: bool, emits: Vec<spark_engine::rules::EmitOp>) -> RuleSpec {
    let mut watch = rule(
        "rule.watch",
        Trigger::Change {
            watched: def("state.stress"),
        },
        emits,
    );
    watch
        .conditions
        .push(compare(Expr::Input(cell("state.mood")), CmpOp::Gt, lit(0)));
    if schedule {
        watch
            .schedules
            .push(reevaluate_after("later", 1, "work.leaf", "rule.leaf"));
    }
    watch
}

/// The review's C2-01 counterexample, converted: wave 0 changes both
/// `state.stress` and `state.mood`; a propagation rule watches stress and is
/// eligible only because mood > 0. Its creator identity must bind the union of
/// both changed groups (v3 §4.3 rules 2/5). Negative control: the stress-only
/// (watched-group-only) formula — the candidate's behavior — differs.
#[test]
fn c2_01_derived_identity_binds_every_changed_eligibility_input_converted() {
    let mut f = standard();
    let setup = work_rule(
        "rule.setup",
        "work.setup",
        vec![
            emit("x", "state.stress", add(1)),
            emit("y", "state.mood", add(1)),
        ],
    );
    let watch = watch_rule(true, vec![]);
    let leaf = work_rule("rule.leaf", "work.leaf", vec![]);
    let mut e = f.engine(
        budgets(10),
        vec![setup, watch.clone(), leaf],
        vec![initial("rule.setup", &actor("a"), 10, "work.setup")],
    );
    let result = e.process(&advance(10));
    let changed = &result.reports()[0].waves[0].committed;
    assert_eq!(changed.len(), 2);
    let all: Vec<Digest> = changed
        .iter()
        .flat_map(|c| c.provenance.retained.clone())
        .collect();
    let stress = changed
        .iter()
        .find(|c| c.definition == def("state.stress"))
        .unwrap()
        .provenance
        .retained
        .clone();
    let cohort = fixture::observation(&e).prewave[0].cohort_identity.clone();
    let art = artifact(&e);
    let fp = watch.fingerprint();
    let expect = |parents: &[Digest]| {
        emission(&Emission {
            cohort: &cohort,
            wave: 1,
            parent: parents_emission(parents),
            rule_fingerprint: &fp,
            sub_ids: &["later"],
            producer: &actor("a"),
            target: "rule.watch",
            target_scope: &actor("a"),
            artifact: &art,
        })
    };
    let actual = sole_obligation(&e).creator_emission_identity().clone();
    assert_eq!(
        actual,
        expect(&all),
        "required union includes the changed mood group read to become eligible"
    );
    assert_ne!(
        actual,
        expect(&stress),
        "negative control: the watched-group-only parent set"
    );
}

/// C2-01 / AT-I25 unchanged background: a read group with no candidates this
/// wave contributes no parent. Two fixtures differ only in the unchanged
/// background value of the condition's read cell: identical parent sets, so
/// identical emission identity; the background is committed instead by the
/// wave's `effect_batch_digest` (through the pre-wave engine digest), which
/// differs.
#[test]
fn c2_01_unchanged_background_contributes_no_parent() {
    let run_with = |mood: i64| {
        let mut f = standard();
        let setup = work_rule(
            "rule.setup",
            "work.setup",
            vec![emit("x", "state.stress", add(1))],
        );
        let set_mood = on_command(
            "rule.mood",
            "cmd.mood",
            vec![emit("m", "state.mood", Update::Assign(param(0)))],
        );
        let watch = watch_rule(true, vec![]);
        let leaf = work_rule("rule.leaf", "work.leaf", vec![]);
        let mut e = f.engine(
            budgets(10),
            vec![setup, set_mood, watch.clone(), leaf],
            vec![initial("rule.setup", &actor("a"), 10, "work.setup")],
        );
        e.process(&Request::Command(command_request(
            "cmd.mood",
            1,
            1,
            "cmd.mood",
            actor("a"),
            vec![],
            vec![mood],
        )));
        let r = e.process(&advance(10));
        let report = r.reports()[0].clone();
        let stress = report.waves[0].committed[0].provenance.retained.clone();
        let cohort = last_cohort(&e);
        let art = artifact(&e);
        let fp = watch.fingerprint();
        let expected = emission(&Emission {
            cohort: &cohort,
            wave: 1,
            parent: parents_emission(&stress),
            rule_fingerprint: &fp,
            sub_ids: &["later"],
            producer: &actor("a"),
            target: "rule.watch",
            target_scope: &actor("a"),
            artifact: &art,
        });
        let actual = sole_obligation(&e).creator_emission_identity().clone();
        assert_eq!(actual, expected, "parents = the changed stress group only");
        (actual, report.waves[1].effect_batch_digest.clone())
    };
    let (id3, batch3) = run_with(3);
    let (id7, batch7) = run_with(7);
    assert_eq!(id3, id7, "background state never enters causal identity");
    assert_ne!(
        batch3, batch7,
        "background state is committed by the batch digest"
    );
}

/// C2-01 / AT-I25 multi-target: one propagation operation writing two targets
/// has, for both emissions, the parent set equal to the union over both read
/// groups, with no per-target selection.
#[test]
fn c2_01_multi_target_operation_takes_the_union() {
    let mut f = standard();
    let setup = work_rule(
        "rule.setup",
        "work.setup",
        vec![
            emit("x", "state.stress", add(1)),
            emit("y", "state.mood", add(1)),
        ],
    );
    let watch = watch_rule(
        false,
        vec![
            emit("e1", "state.echo", add(1)),
            emit("e2", "state.echo2", add(1)),
        ],
    );
    let mut e = f.engine(
        budgets(10),
        vec![setup, watch.clone()],
        vec![initial("rule.setup", &actor("a"), 10, "work.setup")],
    );
    let r = e.process(&advance(10));
    let waves = &r.reports()[0].waves;
    let all: Vec<Digest> = waves[0]
        .committed
        .iter()
        .flat_map(|c| c.provenance.retained.clone())
        .collect();
    let stress_only = waves[0]
        .committed
        .iter()
        .find(|c| c.definition == def("state.stress"))
        .unwrap()
        .provenance
        .retained
        .clone();
    let cohort = fixture::observation(&e).prewave[0].cohort_identity.clone();
    let art = artifact(&e);
    let fp = watch.fingerprint();
    for (sub, target) in [("e1", "state.echo"), ("e2", "state.echo2")] {
        let committed = waves[1]
            .committed
            .iter()
            .find(|c| c.definition == def(target))
            .unwrap();
        let expect = |parents: &[Digest]| {
            emission(&Emission {
                cohort: &cohort,
                wave: 1,
                parent: parents_emission(parents),
                rule_fingerprint: &fp,
                sub_ids: &[sub],
                producer: &actor("a"),
                target,
                target_scope: &actor("a"),
                artifact: &art,
            })
        };
        assert_eq!(
            committed.provenance.retained,
            vec![expect(&all)],
            "{target}"
        );
        assert_ne!(
            committed.provenance.retained,
            vec![expect(&stress_only)],
            "{target}: negative control"
        );
    }
}

// ================================================================ C2-02

/// The review's C2-02 counterexample, converted: one rule emitting to
/// `state.stress@actor:a` and `@actor:b` has fan-out 2 (distinct target ×
/// declared scope breadth). Negative control: a distinct-definition count (1)
/// would admit it at `max_fan_out = 1`.
#[test]
fn c2_02_fanout_counts_scope_breadth_converted() {
    let spread = || {
        on_command(
            "rule.spread",
            "cmd.go",
            vec![
                emit_at("a", "state.stress", actor("a"), add(1)),
                emit_at("b", "state.stress", actor("b"), add(1)),
            ],
        )
    };
    let mut f = standard();
    let mut b = budgets(10);
    b.max_fan_out = 1;
    let errors = f.rule_set(b, vec![spread()]).unwrap_err();
    assert_eq!(
        errors,
        vec![RuleSetError::FanOutExceeded {
            rule_id: def("rule.spread"),
            fan_out: 2,
            bound: 1
        }]
    );
    let mut f = standard();
    b.max_fan_out = 2;
    assert!(
        f.rule_set(b, vec![spread()]).is_ok(),
        "exactly at the bound"
    );
    // Subject and a fixed scope are distinct declared mappings of one target.
    let mixed = on_command(
        "rule.mixed",
        "cmd.go",
        vec![
            emit("s", "state.stress", add(1)),
            emit_at("f", "state.stress", actor("a"), add(1)),
        ],
    );
    let mut f = standard();
    b.max_fan_out = 1;
    assert!(f.rule_set(b, vec![mixed]).is_err());
}

fn materializing(effects: Vec<spark_engine::rules::EmitOp>) -> RuleSpec {
    let mut r = on_command("rule.delay", "cmd.go", vec![]);
    r.schedules.push(ScheduleOp {
        sub_id: tag("delay"),
        delay: 1,
        work_kind: work("work.one"),
        scope: ScopeRef::Subject,
        mode: ScheduleMode::Materialize { effects },
    });
    r
}

/// C2-02, delayed materialized effects: the frozen expansion executes as one
/// later seed, so its declared breadth is bounded too — exact bound admits,
/// one over refuses.
#[test]
fn c2_02_materialized_expansion_is_bounded() {
    let three = || {
        materializing(vec![
            emit_at("m1", "state.stress", actor("a"), add(1)),
            emit_at("m2", "state.stress", actor("b"), add(1)),
            emit_at("m3", "state.stress", actor("c"), add(1)),
        ])
    };
    let mut b = budgets(10);
    b.max_fan_out = 2;
    let mut f = standard();
    assert_eq!(
        f.rule_set(b, vec![three()]).unwrap_err(),
        vec![RuleSetError::MaterializedFanOutExceeded {
            rule_id: def("rule.delay"),
            sub_id: tag("delay"),
            fan_out: 3,
            bound: 2
        }]
    );
    b.max_fan_out = 3;
    let mut f = standard();
    assert!(f.rule_set(b, vec![three()]).is_ok());
}

// ================================================================ C2-03

/// The review's C2-03 counterexample, converted: duplicate nested sub-IDs in a
/// `Materialize` schedule are refused at the door — with unequal payloads
/// (which would collide at execution) and with equal payloads (which would
/// fold and lose one contribution).
#[test]
fn c2_03_materialized_subids_must_be_unique_converted() {
    for (a, b) in [(10, 15), (10, 10)] {
        let mut f = standard();
        let r = materializing(vec![
            emit("same", "state.stress", add(a)),
            emit("same", "state.stress", add(b)),
        ]);
        assert_eq!(
            f.rule_set(budgets(10), vec![r]).unwrap_err(),
            vec![RuleSetError::DuplicateSubId {
                rule_id: def("rule.delay"),
                sub_id: tag("same")
            }],
            "deltas {a}/{b}"
        );
    }
    // One namespace per rule: a nested sub-ID may not reuse a top-level one.
    let mut r = materializing(vec![emit("same", "state.stress", add(1))]);
    r.emits.push(emit("same", "state.mood", add(1)));
    let mut f = standard();
    assert!(f.rule_set(budgets(10), vec![r]).unwrap_err().contains(
        &RuleSetError::DuplicateSubId {
            rule_id: def("rule.delay"),
            sub_id: tag("same")
        }
    ));
    // Qualification is validated for nested sub-IDs too.
    let long = format!("rule.{}", "x".repeat(249));
    let mut r = materializing(vec![emit("xy", "state.stress", add(1))]);
    r.rule_id = def(&long);
    let mut f = standard();
    assert!(f
        .rule_set(budgets(10), vec![r])
        .unwrap_err()
        .iter()
        .any(|e| matches!(e, RuleSetError::QualifiedIdInvalid { .. })));
}

/// C2-03 positive control: distinct nested sub-IDs with **equal** deltas are
/// two causes; both apply (`10 + 10`), never folded into one.
#[test]
fn c2_03_distinct_nested_subids_keep_both_contributions() {
    let mut f = standard();
    let r = materializing(vec![
        emit("m1", "state.stress", add(10)),
        emit("m2", "state.stress", add(10)),
    ]);
    let mut e = f.engine(budgets(10), vec![r], vec![]);
    e.process(&command("cmd.go", 1, 1, "cmd.go", bron()));
    let r = run(&mut e, 2);
    assert_eq!(r[0].outcome, CohortOutcome::Committed);
    assert_eq!(value(&e, "state.stress", &bron()), Some(20));
    assert_eq!(r[0].waves[0].committed[0].provenance.retained.len(), 2);
}

// ================================================================ C2-04

fn wide_manifest() -> ProfileManifest {
    ProfileManifest::new(
        profile_id(),
        BoundedText::new("wide").unwrap(),
        vec![
            int_spec("state.stress", Authority::SparkOwned, i64::MIN, i64::MAX),
            int_spec("state.total", Authority::Derived, i64::MIN, i64::MAX),
        ],
    )
}

/// The review's C2-04 counterexample, converted: two cells each `i64::MAX`,
/// weight 0.5: the valid result `floor((MAX + MAX) · 0.5) = MAX`. Negative
/// control: narrowing the accumulated sum to `i64` before weighting rejects.
#[test]
fn c2_04_aggregate_keeps_i128_until_after_weighting_converted() {
    let mut f = activate(&wide_manifest());
    let setup = on_command(
        "rule.setup",
        "cmd.setup",
        vec![
            emit_at(
                "a",
                "state.stress",
                actor("a"),
                Update::Assign(lit(i64::MAX)),
            ),
            emit_at(
                "b",
                "state.stress",
                actor("b"),
                Update::Assign(lit(i64::MAX)),
            ),
        ],
    );
    let aggregate = on_command(
        "rule.aggregate",
        "cmd.aggregate",
        vec![emit(
            "sum",
            "state.total",
            Update::Aggregate {
                source: def("state.stress"),
                weight: FixedPoint::from_raw(500_000),
            },
        )],
    );
    let mut e = f.engine(budgets(10), vec![setup, aggregate], vec![]);
    e.process(&command("cmd.setup", 0, 1, "cmd.setup", actor("a")));
    assert_eq!(value(&e, "state.stress", &actor("b")), Some(i64::MAX));
    let r = e.process(&command("cmd.aggregate", 1, 2, "cmd.aggregate", actor("a")));
    assert_eq!(r.reports()[0].outcome, CohortOutcome::Committed);
    assert_eq!(value(&e, "state.total", &actor("a")), Some(i64::MAX));
}

/// C2-04: true `i128` overflow stays a typed rejection, never a panic.
#[test]
fn c2_04_aggregate_i128_overflow_is_typed() {
    let mut f = activate(&wide_manifest());
    let setup = on_command(
        "rule.setup",
        "cmd.setup",
        ["a", "b", "c"]
            .iter()
            .map(|w| emit_at(w, "state.stress", actor(w), Update::Assign(lit(i64::MAX))))
            .collect(),
    );
    let aggregate = on_command(
        "rule.aggregate",
        "cmd.aggregate",
        vec![emit(
            "sum",
            "state.total",
            Update::Aggregate {
                source: def("state.stress"),
                weight: FixedPoint::from_raw(i64::MAX),
            },
        )],
    );
    let mut e = f.engine(budgets(10), vec![setup, aggregate], vec![]);
    e.process(&command("cmd.setup", 0, 1, "cmd.setup", actor("a")));
    let before = e.state().canonical_state_digest();
    let r = e.process(&command("cmd.aggregate", 1, 2, "cmd.aggregate", actor("a")));
    assert_eq!(
        r.reports()[0].outcome,
        CohortOutcome::Rejected {
            wave: 0,
            rejection: WaveRejection::Arithmetic { what: "aggregate" }
        }
    );
    assert_eq!(before, e.state().canonical_state_digest());
}

/// C2-04 rule-body audit (`engine.rs` all-additive and staged paths): every
/// intermediate is `i128`, one conversion at the end. `+MAX, +MAX, −MAX` is
/// exactly `+MAX`; a staged `+MAX` then `×0.5` over a `MAX` cell is `MAX`.
/// Negative control: `i64` intermediates reject both.
#[test]
fn c2_04_rule_body_intermediates_are_widened() {
    let mut f = activate(&wide_manifest());
    let additive = on_command(
        "rule.additive",
        "cmd.additive",
        vec![
            emit("a", "state.stress", add(i64::MAX)),
            emit("b", "state.stress", add(i64::MAX)),
            emit("c", "state.stress", Update::Subtract(lit(i64::MAX))),
        ],
    );
    let staged = on_command(
        "rule.staged",
        "cmd.staged",
        vec![
            emit("a", "state.stress", add(i64::MAX)),
            emit(
                "s",
                "state.stress",
                Update::Scale(FixedPoint::from_raw(500_000)),
            ),
        ],
    );
    let mut e = f.engine(budgets(10), vec![additive, staged], vec![]);
    let r = e.process(&command("cmd.a", 1, 1, "cmd.additive", bron()));
    assert_eq!(r.reports()[0].outcome, CohortOutcome::Committed);
    assert_eq!(value(&e, "state.stress", &bron()), Some(i64::MAX));
    let r = e.process(&command("cmd.s", 2, 2, "cmd.staged", bron()));
    assert_eq!(r.reports()[0].outcome, CohortOutcome::Committed);
    assert_eq!(
        value(&e, "state.stress", &bron()),
        Some(i64::MAX),
        "floor((MAX + MAX) · 0.5)"
    );
    // A final result outside `i64` is still a typed rejection.
    let r = e.process(&command("cmd.a2", 3, 3, "cmd.additive", bron()));
    assert!(matches!(
        r.reports()[0].outcome,
        CohortOutcome::Rejected {
            rejection: WaveRejection::Arithmetic { .. },
            ..
        }
    ));
}

// ================================================================ C2-05

/// A decay fixture: `state.stress` set to 100 at t = 0, declared baseline
/// `baseline`, a decay rule by `update` evaluated at `times`.
fn decay_fixture(times: &[u64], update: Update, base: i64) -> Engine {
    let mut f = standard();
    let setup = on_command(
        "rule.setup",
        "cmd.setup",
        vec![emit("x", "state.stress", Update::Assign(lit(100)))],
    );
    let decay_rule = work_rule(
        "rule.decay",
        "work.decay",
        vec![emit("x", "state.stress", update)],
    );
    let mut e = Engine::genesis(
        f.genesis_with(
            budgets(10),
            vec![setup, decay_rule],
            vec![baseline("state.stress", base)],
            times
                .iter()
                .map(|t| initial("rule.decay", &actor("a"), *t, "work.decay"))
                .collect(),
        ),
    )
    .unwrap();
    e.process(&command("cmd.setup", 0, 1, "cmd.setup", actor("a")));
    e
}

fn stress_cell(e: &Engine) -> (i64, LogicalTime) {
    let c = e
        .state()
        .get(&profile_id(), &def("state.stress"), &actor("a"))
        .unwrap();
    let v = match c.value {
        CanonicalValue::Int(v) => v,
        _ => panic!(),
    };
    (v, c.updated_at)
}

/// The review's C2-05 remainder counterexample, converted: value 100, target 0,
/// cadence 10, rate 10: evaluations at {10, 20}, {20}, and {15, 20} all yield
/// 80. Negative control: measuring from `updated_at` but committing at `now`
/// drops the 5-unit remainder at 15 and yields 90.
#[test]
fn c2_05_decay_preserves_elapsed_cadence_remainder_converted() {
    let at = |times: &[u64]| {
        let mut e = decay_fixture(times, decay(10, 10), 0);
        drive(&mut e, &advance(20));
        stress_cell(&e).0
    };
    assert_eq!(at(&[10, 20]), 80, "aligned positive control");
    assert_eq!(at(&[20]), 80, "one-shot positive control");
    assert_eq!(
        at(&[15, 20]),
        80,
        "five units of remainder at 15 survive to 20"
    );
}

/// C2-05 general chunk invariance (v1 Q7: catch-up over N missed steps in one
/// evaluation is bit-identical to separate evaluations): every subset of
/// unaligned evaluation times before 40 gives the value **and** commit time of
/// the single evaluation at 40.
#[test]
fn c2_05_chunk_invariance_holds_for_every_evaluation_subset() {
    let grid = [7u64, 13, 20, 26, 33];
    let mut single = decay_fixture(&[40], decay(10, 10), 0);
    drive(&mut single, &advance(40));
    let reference = stress_cell(&single);
    assert_eq!(reference, (60, LogicalTime(40)));
    for mask in 0u32..(1 << grid.len()) {
        let mut times: Vec<u64> = grid
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, t)| *t)
            .collect();
        times.push(40);
        let mut e = decay_fixture(&times, decay(10, 10), 0);
        drive(&mut e, &advance(40));
        assert_eq!(stress_cell(&e), reference, "evaluations at {times:?}");
    }
    // The commit time is the end of the last whole step, so the remainder is
    // visible: one evaluation at 15 moves one step and commits at 10.
    let mut e = decay_fixture(&[15], decay(10, 10), 0);
    drive(&mut e, &advance(15));
    assert_eq!(stress_cell(&e), (90, LogicalTime(10)));
}

fn activate_request(
    id: &str,
    t: u64,
    seq: u64,
    kind_tag: &str,
    rule_set: ActivatedRuleSet,
    cfg: spark_engine::profile::config::ConfigRevision,
) -> Request {
    Request::Command(CommandRequest {
        command_kind: kind(kind_tag),
        payload: CommandPayload::ActivateEpoch {
            rule_set,
            config: cfg,
        },
        ..command_request(id, t, seq, kind_tag, actor("a"), vec![], vec![])
    })
}

/// The epoch-bound rate fixture: rate `tune.rate` = 1 at genesis; epoch 2
/// activated at `barrier` with rate 5; decay evaluated at `times`.
fn rate_fixture(times: &[u64], barrier: u64) -> Engine {
    let mut f = standard();
    let setup = on_command(
        "rule.setup",
        "cmd.setup",
        vec![emit("x", "state.stress", Update::Assign(lit(100)))],
    );
    let decay_rule = work_rule(
        "rule.decay",
        "work.decay",
        vec![emit(
            "x",
            "state.stress",
            decay_config_rate("tune.rate", 10),
        )],
    );
    let mut g = f.genesis_with(
        budgets(10),
        vec![setup, decay_rule],
        vec![baseline("state.stress", 0)],
        times
            .iter()
            .map(|t| initial("rule.decay", &actor("a"), *t, "work.decay"))
            .collect(),
    );
    g.config = config(vec![("tune.rate", CanonicalValue::Int(1))]);
    let rs = g.rule_set.clone();
    let mut e = Engine::genesis(g).unwrap();
    e.process(&command("cmd.setup", 0, 1, "cmd.setup", actor("a")));
    let r = reports(&drive(
        &mut e,
        &activate_request(
            "cmd.activate",
            barrier,
            2,
            "spark.epoch.activate",
            rs,
            config(vec![("tune.rate", CanonicalValue::Int(5))]),
        ),
    ));
    assert_eq!(
        r.last().unwrap().outcome,
        CohortOutcome::EpochActivated { behavior_epoch: 2 }
    );
    e
}

/// The review's AT-I23 counterexample, converted: value 100 at 0, cadence 10,
/// rate 1, rate 5 activated at 50, first decay at 60: five old-rate steps plus
/// one new-rate step = 90. Negative control: the current rate applied to the
/// whole elapsed span gives 70.
#[test]
fn c2_05_new_decay_rate_must_not_apply_before_activation_barrier_converted() {
    let mut e = rate_fixture(&[60], 50);
    assert_eq!(e.behavior_epoch(), 2);
    drive(&mut e, &advance(60));
    assert_eq!(
        stress_cell(&e).0,
        90,
        "5 steps at rate 1 before barrier 50, 1 step at rate 5 after; never six steps at rate 5"
    );
}

/// AT-I23 chunk invariance across the barrier, and an unaligned barrier: a
/// pre-barrier evaluation at the barrier time (scheduled work at 50 runs
/// before the activating command at 50) composes with the later one; with the
/// barrier at 55 the step ending at 60 takes the rate in effect at its end.
#[test]
fn c2_05_epoch_bound_rates_compose_across_the_barrier() {
    for (times, barrier) in [
        (vec![60], 50),
        (vec![50, 60], 50),
        (vec![30, 60], 50),
        (vec![60], 55),
        (vec![55, 60], 55),
    ] {
        let mut e = rate_fixture(&times, barrier);
        drive(&mut e, &advance(60));
        assert_eq!(
            stress_cell(&e),
            (90, LogicalTime(60)),
            "{times:?} / {barrier}"
        );
    }
}

/// C2-05 / AT-I23: an interval in which the executing decay operation was not
/// part of the active artifact integrates nothing. Epoch 1 has no decay rule
/// and no baseline; epoch 2 (at 50) introduces both and arms decay work due
/// at 60: exactly one step elapsed under a decay law (100 → 90), and the
/// cell's baseline is materialized from epoch 2's declaration. Negative
/// control: retroactive integration from the cell's `updated_at` gives 40.
#[test]
fn c2_05_decay_integrates_only_under_an_artifact_carrying_it() {
    let mut f = standard();
    let setup = on_command(
        "rule.setup",
        "cmd.setup",
        vec![emit("x", "state.stress", Update::Assign(lit(100)))],
    );
    let mut e = f.engine(budgets(10), vec![setup.clone()], vec![]);
    e.process(&command("cmd.setup", 0, 1, "cmd.setup", actor("a")));
    assert_eq!(
        e.state()
            .get(&profile_id(), &def("state.stress"), &actor("a"))
            .unwrap()
            .baseline,
        None,
        "no declaration, no baseline"
    );
    let decay_rule = work_rule(
        "rule.decay",
        "work.decay",
        vec![emit("x", "state.stress", decay(10, 10))],
    );
    let arm = with_schedule(
        on_command("rule.arm", "cmd.arm", vec![]),
        reevaluate_after("d", 10, "work.decay", "rule.decay"),
    );
    let rs2 = f
        .rule_set_with(
            budgets(10),
            vec![setup, decay_rule, arm],
            vec![baseline("state.stress", 0)],
        )
        .unwrap();
    drive(
        &mut e,
        &activate_request(
            "cmd.activate",
            50,
            2,
            "spark.epoch.activate",
            rs2,
            config(vec![]),
        ),
    );
    drive(&mut e, &command("cmd.arm", 50, 3, "cmd.arm", actor("a")));
    drive(&mut e, &advance(60));
    let cell = e
        .state()
        .get(&profile_id(), &def("state.stress"), &actor("a"))
        .unwrap();
    assert_eq!(cell.value, CanonicalValue::Int(90));
    assert_eq!(cell.baseline, Some(CanonicalValue::Int(0)));
}

/// C2-05 / v1 Q7: the target is the cell's `StateCell.baseline` field. A cell
/// whose baseline was materialized as 20 keeps moving toward 20 after a later
/// epoch declares 30 (the declaration seeds a cell's baseline once; it is
/// never silently re-targeted). Negative control: reading the current
/// declaration stops at 30.
#[test]
fn c2_05_decay_moves_toward_the_cells_baseline_field() {
    let mut f = standard();
    let setup = on_command(
        "rule.setup",
        "cmd.setup",
        vec![emit("x", "state.stress", Update::Assign(lit(100)))],
    );
    let decay_rule = work_rule(
        "rule.decay",
        "work.decay",
        vec![emit("x", "state.stress", decay(50, 10))],
    );
    let mut e = Engine::genesis(f.genesis_with(
        budgets(10),
        vec![setup.clone(), decay_rule.clone()],
        vec![baseline("state.stress", 20)],
        vec![initial("rule.decay", &actor("a"), 20, "work.decay")],
    ))
    .unwrap();
    e.process(&command("cmd.setup", 0, 1, "cmd.setup", actor("a")));
    let materialized = e
        .state()
        .get(&profile_id(), &def("state.stress"), &actor("a"))
        .unwrap()
        .baseline
        .clone();
    assert_eq!(materialized, Some(CanonicalValue::Int(20)));
    let rs2 = f
        .rule_set_with(
            budgets(10),
            vec![setup, decay_rule],
            vec![baseline("state.stress", 30)],
        )
        .unwrap();
    drive(
        &mut e,
        &activate_request(
            "cmd.activate",
            5,
            2,
            "spark.epoch.activate",
            rs2,
            config(vec![]),
        ),
    );
    drive(&mut e, &advance(20));
    assert_eq!(
        stress_cell(&e).0,
        20,
        "moves toward the cell's field, not the new declaration"
    );
}

/// Recovery from below the baseline moves up and never overshoots.
#[test]
fn c2_05_recovery_moves_up_to_the_baseline() {
    let mut f = standard();
    let setup = on_command(
        "rule.setup",
        "cmd.setup",
        vec![emit("x", "state.stress", Update::Assign(lit(0)))],
    );
    let decay_rule = work_rule(
        "rule.decay",
        "work.decay",
        vec![emit("x", "state.stress", decay(3, 10))],
    );
    let mut e = Engine::genesis(f.genesis_with(
        budgets(10),
        vec![setup, decay_rule],
        vec![baseline("state.stress", 20)],
        vec![
            initial("rule.decay", &actor("a"), 20, "work.decay"),
            initial("rule.decay", &actor("a"), 200, "work.decay"),
        ],
    ))
    .unwrap();
    e.process(&command("cmd.setup", 0, 1, "cmd.setup", actor("a")));
    drive(&mut e, &advance(20));
    assert_eq!(stress_cell(&e).0, 6);
    drive(&mut e, &advance(200));
    assert_eq!(stress_cell(&e).0, 20, "converges exactly at the baseline");
}

/// C2-05 / v1 Q7 at the door: a decay rule on a definition without declared
/// baseline semantics is rejected; baseline declarations are validated
/// (unknown, host-owned, out of bounds, duplicate); literal parameters are
/// validated; config parameters are validated at genesis and at activation.
#[test]
fn c2_05_baseline_semantics_and_parameters_are_validated() {
    let decaying = || {
        work_rule(
            "rule.decay",
            "work.decay",
            vec![emit("x", "state.stress", decay(1, 10))],
        )
    };
    let mut f = standard();
    assert_eq!(
        f.rule_set(budgets(10), vec![decaying()]).unwrap_err(),
        vec![RuleSetError::DecayWithoutBaseline {
            rule_id: def("rule.decay"),
            target: def("state.stress")
        }]
    );
    for (decl, expected) in [
        (baseline("state.nothing", 0), "invalid"),
        (baseline("state.food", 0), "invalid"),
        (baseline("state.bounded", 11), "invalid"),
    ] {
        let mut f = standard();
        let errors = f
            .rule_set_with(budgets(10), vec![], vec![decl.clone()])
            .unwrap_err();
        assert_eq!(
            errors,
            vec![RuleSetError::BaselineInvalid {
                definition: decl.definition.clone()
            }],
            "{expected}"
        );
    }
    let mut f = standard();
    assert!(f
        .rule_set_with(budgets(10), vec![], vec![baseline("state.bounded", 10)])
        .is_ok());
    let mut f = standard();
    assert_eq!(
        f.rule_set_with(
            budgets(10),
            vec![],
            vec![baseline("state.stress", 1), baseline("state.stress", 2)]
        )
        .unwrap_err(),
        vec![RuleSetError::DuplicateBaseline {
            definition: def("state.stress")
        }]
    );
    let bad = |u: Update| {
        work_rule(
            "rule.decay",
            "work.decay",
            vec![emit("x", "state.stress", u)],
        )
    };
    let mut f = standard();
    assert!(f
        .rule_set_with(
            budgets(10),
            vec![bad(decay(-1, 10))],
            vec![baseline("state.stress", 0)]
        )
        .unwrap_err()
        .contains(&RuleSetError::InvalidDecayParameter {
            rule_id: def("rule.decay")
        }));
    let mut f = standard();
    assert!(f
        .rule_set_with(
            budgets(10),
            vec![bad(decay(1, 0))],
            vec![baseline("state.stress", 0)]
        )
        .unwrap_err()
        .contains(&RuleSetError::ZeroCadence {
            rule_id: def("rule.decay")
        }));
    // A config-sourced rate must be >= 0 in every epoch's config.
    let mut f = standard();
    let mut g = f.genesis_with(
        budgets(10),
        vec![bad(decay_config_rate("tune.rate", 10))],
        vec![baseline("state.stress", 0)],
        vec![],
    );
    g.config = config(vec![("tune.rate", CanonicalValue::Int(-1))]);
    assert_eq!(
        Engine::genesis(g.clone()).unwrap_err(),
        GenesisError::InvalidDecayParameter(def("tune.rate"))
    );
    g.config = config(vec![("tune.rate", CanonicalValue::Int(2))]);
    let rs = g.rule_set.clone();
    let mut e = Engine::genesis(g).unwrap();
    let r = reports(&drive(
        &mut e,
        &activate_request(
            "cmd.activate",
            5,
            1,
            "spark.epoch.activate",
            rs,
            config(vec![("tune.rate", CanonicalValue::Int(-3))]),
        ),
    ));
    assert_eq!(
        r.last().unwrap().outcome,
        CohortOutcome::EpochActivationRejected {
            reason: "invalid_decay_parameter"
        }
    );
    assert_eq!(e.behavior_epoch(), 1);
}

// ================================================================ C2-06

/// The review's C2-06 counterexample, converted: an ordinary command kind
/// carrying an activation payload finalizes as an ordinary command and
/// activates nothing. Negative control: payload-only dispatch changes epoch
/// 1 → 2.
#[test]
fn c2_06_epoch_activation_requires_reserved_command_kind_converted() {
    let mut f = standard();
    let mut e = f.engine(budgets(10), vec![], vec![]);
    let rs = f.rule_set(budgets(10), vec![]).unwrap();
    let registry = e.epoch_registry().canonical_digest();
    let rule_set_hash = e.rule_set().content_hash().clone();
    let r = e.process(&activate_request(
        "cmd.wrong",
        1,
        1,
        "cmd.ordinary",
        rs,
        config(vec![]),
    ));
    assert_eq!(
        e.behavior_epoch(),
        1,
        "ordinary command kind must not activate an epoch"
    );
    assert_eq!(r.outcome(), &Outcome::Completed);
    assert_eq!(
        r.reports()[0].outcome,
        CohortOutcome::EpochActivationRejected {
            reason: "activation_payload_requires_reserved_kind"
        }
    );
    assert_eq!(e.epoch_registry().canonical_digest(), registry);
    assert_eq!(e.rule_set().content_hash(), &rule_set_hash);
    assert_eq!(e.timeline_frontier_ordinal().0, 1, "finalized as usual");
    assert_eq!(fixture::lineage_len(&e), 1);
}

/// C2-06, the other direction: the reserved kind with a host payload applies
/// no ingress and runs no rule, still finalizes; and the door refuses a rule
/// triggered by the reserved kind. Positive control: reserved kind +
/// activation payload activates.
#[test]
fn c2_06_reserved_kind_binds_the_activation_payload_both_ways() {
    let mut f = standard();
    let mut e = f.engine(budgets(10), vec![], vec![]);
    let r = e.process(&Request::Command(command_request(
        "cmd.reserved",
        1,
        1,
        "spark.epoch.activate",
        bron(),
        vec![("state.food", 5)],
        vec![],
    )));
    assert_eq!(r.outcome(), &Outcome::Completed);
    assert_eq!(
        r.reports()[0].outcome,
        CohortOutcome::EpochActivationRejected {
            reason: "reserved_kind_requires_activation_payload"
        }
    );
    assert!(r.reports()[0].ingress.is_empty());
    assert_eq!(value(&e, "state.food", &bron()), None);
    assert_eq!(e.timeline_frontier_ordinal().0, 1);
    let mut f = standard();
    assert_eq!(
        f.rule_set(
            budgets(10),
            vec![on_command(
                "rule.bad",
                "spark.epoch.activate",
                vec![emit("x", "state.stress", add(1))]
            )]
        )
        .unwrap_err(),
        vec![RuleSetError::ReservedCommandKind {
            rule_id: def("rule.bad")
        }]
    );
    let mut f = standard();
    let mut e = f.engine(budgets(10), vec![], vec![]);
    let rs = f.rule_set(budgets(10), vec![]).unwrap();
    e.process(&activate_request(
        "cmd.ok",
        1,
        1,
        "spark.epoch.activate",
        rs,
        config(vec![]),
    ));
    assert_eq!(e.behavior_epoch(), 2);
    assert_eq!(fixture::lineage_len(&e), 2);
}

// ================================================================ D-C2-7

/// D-C2-7: a re-evaluation record resolves its exact originating artifact.
/// Epoch 1 activates `{rule.a}`; epoch 2 `{rule.a, rule.b}`. A record naming
/// `rule.b` with epoch 1's ruleset hash — present in the registry, but not an
/// artifact containing `rule.b` — refuses. Negative control: the candidate's
/// "current rule matches and the hash appears in some epoch record" accepts
/// it. Positive: the same record with epoch 2's hash executes; an unknown
/// creator artifact refuses.
#[test]
fn d_c2_7_reevaluation_resolves_the_exact_originating_artifact() {
    let a = work_rule("rule.a", "work.a", vec![emit("x", "state.stress", add(1))]);
    let b = work_rule("rule.b", "work.b", vec![emit("x", "state.mood", add(1))]);
    let build = || {
        let mut f = standard();
        let g = f.genesis(budgets(10), vec![a.clone()], vec![]);
        let rs1 = g.rule_set.clone();
        let mut e = Engine::genesis(g).unwrap();
        let rs2 = f.rule_set(budgets(10), vec![a.clone(), b.clone()]).unwrap();
        drive(
            &mut e,
            &activate_request(
                "cmd.activate",
                1,
                1,
                "spark.epoch.activate",
                rs2.clone(),
                config(vec![]),
            ),
        );
        (e, rs1, rs2)
    };
    let k = WorkKey {
        due_time: LogicalTime(100),
        profile_id: profile_id(),
        producer_definition_id: def("rule.b"),
        scope_id: bron(),
        occurrence_index: OccurrenceIndex(0),
        work_kind: work("work.b"),
    };
    let record = |e: &Engine, hash: &Digest, art: Digest| {
        fixture::reevaluation_record(
            k.clone(),
            def("rule.b"),
            e.rule_set()
                .rule(&def("rule.b"))
                .unwrap()
                .fingerprint()
                .clone(),
            hash.clone(),
            hash_bytes(b"forged"),
            art,
        )
    };
    let (mut e, rs1, _) = build();
    let r = record(&e, rs1.content_hash(), artifact(&e));
    fixture::schedule_record(&mut e, r);
    let out = run(&mut e, 100);
    assert!(matches!(
        out[0].outcome,
        CohortOutcome::ObligationRefused(ObligationRefusal::RuleResolutionFailed { .. })
    ));
    assert_eq!(value(&e, "state.mood", &bron()), None);
    let (mut e, _, rs2) = build();
    let r = record(&e, rs2.content_hash(), artifact(&e));
    fixture::schedule_record(&mut e, r);
    assert_eq!(run(&mut e, 100)[0].outcome, CohortOutcome::Committed);
    assert_eq!(value(&e, "state.mood", &bron()), Some(1));
    let (mut e, _, rs2) = build();
    let r = record(&e, rs2.content_hash(), hash_bytes(b"no such epoch"));
    fixture::schedule_record(&mut e, r);
    assert!(matches!(
        run(&mut e, 100)[0].outcome,
        CohortOutcome::ObligationRefused(ObligationRefusal::RuleResolutionFailed { .. })
    ));
}

/// D-C2-7 compatibility rule, part (3): a re-evaluation obligation created in
/// epoch 1 and executed in epoch 2 reads config point-wise under the epoch in
/// effect at its evaluation time (barrier-scoped hot tuning), while the rule
/// logic is the bit-identical originating rule.
#[test]
fn d_c2_7_config_reads_are_barrier_scoped() {
    let rate = work_rule(
        "rule.rate",
        "work.rate",
        vec![emit(
            "r",
            "state.stress",
            Update::Add(Expr::Input(spark_engine::rules::Input::Config {
                key: def("tune.rate"),
            })),
        )],
    );
    let mut f = standard();
    let mut g = f.genesis(
        budgets(10),
        vec![rate],
        vec![
            initial("rule.rate", &bron(), 40, "work.rate"),
            initial("rule.rate", &bron(), 60, "work.rate"),
        ],
    );
    g.config = config(vec![("tune.rate", CanonicalValue::Int(1))]);
    let rs = g.rule_set.clone();
    let mut e = Engine::genesis(g).unwrap();
    let created_in_epoch_1 = e.obligations().keys().count();
    assert_eq!(created_in_epoch_1, 2);
    drive(
        &mut e,
        &activate_request(
            "cmd.activate",
            50,
            1,
            "spark.epoch.activate",
            rs,
            config(vec![("tune.rate", CanonicalValue::Int(5))]),
        ),
    );
    assert_eq!(
        value(&e, "state.stress", &bron()),
        Some(1),
        "evaluated at 40, epoch 1"
    );
    drive(&mut e, &advance(60));
    assert_eq!(
        value(&e, "state.stress", &bron()),
        Some(6),
        "evaluated at 60, epoch 2"
    );
}

// ================================================================ D-C2-11

/// D-C2-11 revised: a store disagreement found by the extraction inside
/// `process` is returned as a typed sticky fail-stop carrying the refusal —
/// never a silent repeated `Paused`. The cohort committed earlier in the same
/// call stays committed (reported honestly); snapshot and reset are refused;
/// restoring the last committed snapshot recovers.
#[test]
fn d_c2_11_store_invariant_violation_is_typed_and_sticky() {
    let mut f = standard();
    let g = f.genesis(
        budgets(10),
        vec![work_rule(
            "rule.a",
            "work.one",
            vec![emit("x", "state.stress", add(1))],
        )],
        vec![
            initial("rule.a", &bron(), 10, "work.one"),
            initial("rule.a", &bron(), 20, "work.one"),
        ],
    );
    let profile = g.profile.clone();
    let mut e = Engine::genesis(g).unwrap();
    let committed = e.snapshot().unwrap();
    let k20 = e
        .obligations()
        .keys()
        .find(|k| k.due_time == LogicalTime(20))
        .unwrap()
        .clone();
    fixture::tamper_obligations(&mut e, &k20, None);
    let r = e.process(&advance(20));
    assert_eq!(
        r.outcome(),
        &Outcome::StoreInvariantViolated(ExtractionRefusal::ObligationRecordMissing {
            key: k20.clone()
        })
    );
    assert_eq!(r.reports().len(), 1, "the cohort at 10 committed first");
    assert_eq!(r.reports()[0].outcome, CohortOutcome::Committed);
    assert_eq!(value(&e, "state.stress", &bron()), Some(1));
    assert!(e.is_fail_stopped());
    assert!(!r.terminates(r.presented()), "not dequeue-eligible");
    assert!(e.snapshot().is_err());
    assert!(e
        .reset_timeline_epoch(
            TimelineEpoch(1),
            spark_core::id::SourceId::new("seq.x").unwrap()
        )
        .is_err());
    let again = e.process(&advance(20));
    assert_eq!(again.outcome(), r.outcome(), "sticky");
    assert!(again.reports().is_empty());
    let mut restored = Engine::restore(committed, &profile).unwrap();
    assert_eq!(
        restored.process(&advance(20)).outcome(),
        &Outcome::Completed
    );
    assert_eq!(value(&restored, "state.stress", &bron()), Some(2));
}

// ================================================================ D-C2-13

/// D-C2-13: a materialized obligation's emission identity is payload-
/// independent. Two histories whose only difference is the frozen delta (10
/// vs 15) execute the same `WorkKey` in the same cohort: equal identities,
/// different values; both equal the independent formula with the creator
/// rule's fingerprint. Negative control: a record-hash fingerprint component
/// (the candidate's) differs between the two.
#[test]
fn d_c2_13_materialized_identity_is_payload_independent() {
    let schedule = {
        let mut r = on_command("rule.sched", "cmd.sched", vec![]);
        r.schedules.push(ScheduleOp {
            sub_id: tag("later"),
            delay: 5,
            work_kind: work("work.m"),
            scope: ScopeRef::Subject,
            mode: ScheduleMode::Materialize {
                effects: vec![emit("m", "state.stress", Update::Add(param(0)))],
            },
        });
        r
    };
    let run_with = |delta: i64| {
        let mut f = standard();
        let mut e = f.engine(budgets(10), vec![schedule.clone()], vec![]);
        e.process(&Request::Command(command_request(
            "cmd.s",
            1,
            1,
            "cmd.sched",
            bron(),
            vec![],
            vec![delta],
        )));
        let record = sole_obligation(&e);
        let r = run(&mut e, 6);
        let identity = r[0].waves[0].committed[0].provenance.retained[0].clone();
        let cohort = last_cohort(&e);
        let art = artifact(&e);
        let expected = emission(&Emission {
            cohort: &cohort,
            wave: 0,
            parent: parents_workkey(record.key()),
            rule_fingerprint: record.creator_rule_fingerprint(),
            sub_ids: &["m"],
            producer: &bron(),
            target: "state.stress",
            target_scope: &bron(),
            artifact: &art,
        });
        assert_eq!(identity, expected);
        assert_eq!(record.creator_rule_fingerprint(), &schedule.fingerprint());
        let mut rfp = CanonicalEncoder::new();
        rfp.push_str("materialized_obligation_v1");
        rfp.push_digest(&record.record_hash());
        (identity, value(&e, "state.stress", &bron()), rfp.finish())
    };
    let (id10, v10, legacy10) = run_with(10);
    let (id15, v15, legacy15) = run_with(15);
    assert_eq!((v10, v15), (Some(10), Some(15)));
    assert_eq!(id10, id15, "payload never enters emission identity");
    assert_ne!(
        legacy10, legacy15,
        "negative control: a record-hash component would make them differ"
    );
}

// ================================================================ lineage (derived index)

/// The per-epoch artifact lineage is validated at restore like the timeline's
/// derived indexes (AT-I28 pattern): a tampered activation time or a missing
/// entry is refused even though neither enters the committed digest; the
/// untampered snapshot restores and continues identically.
#[test]
fn lineage_is_validated_at_restore() {
    let mut e = rate_fixture(&[60], 50);
    let snap = e.snapshot().unwrap();
    let mut shifted = snap.clone();
    fixture::snapshot_set_activation_time(&mut shifted, 1, Some(LogicalTime(40)));
    let mut truncated = snap.clone();
    fixture::snapshot_truncate_lineage(&mut truncated);
    let mut genesis_timed = snap.clone();
    fixture::snapshot_set_activation_time(&mut genesis_timed, 0, Some(LogicalTime(0)));
    for (name, s) in [
        ("shifted", shifted),
        ("truncated", truncated),
        ("genesis timed", genesis_timed),
    ] {
        assert_eq!(
            Engine::restore(s, &rate_profile()).unwrap_err(),
            RestoreError::EpochLineageInvalid,
            "{name}"
        );
    }
    let mut restored = Engine::restore(snap, &rate_profile()).unwrap();
    drive(&mut e, &advance(60));
    drive(&mut restored, &advance(60));
    assert_eq!(
        e.stable_boundary_digest(),
        restored.stable_boundary_digest()
    );
    assert_eq!(stress_cell(&restored).0, 90);
}

/// The activated profile the rate fixture uses (a fresh door activation of
/// the same manifest yields the same activation identity).
fn rate_profile() -> spark_engine::activation::ActivatedProfile {
    standard().profile
}
