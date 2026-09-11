//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 bounded revision: the oracle cases the independent review found
//! missing or partial (its per-entry table, C2-07). Each test names its entry
//! and the wrong implementation it kills.
//!
//! Entries completed here: AT-I1, AT-I3, AT-I4, AT-I5, AT-I6a, AT-I6c(d)(e)(f),
//! AT-I6d (renaming), AT-I7e, AT-I8, AT-I15, AT-I16, AT-I20b, AT-I20c(f),
//! AT-I21 (interference), AT-I22 (compositions), AT-I24 (i128 corpus),
//! AT-I26/AT-I27 (isolated stores; reset/epoch crossings), AT-I30, AT-I31,
//! AT-I33, AT-I35 (golden vectors), AT-I38, AT-I39(E) + AT-I41, AT-I44(c′),
//! AT-I50(d). AT-I13 remains an honest interpretation (fixed manifest); AT-I40
//! remains per-profile (D-C2-2, accepted).

use spark_core::authority::Authority;
use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, CanonicalEncoder, Digest};
use spark_core::id::SourceId;
use spark_core::scheduler::{OccurrenceIndex, WorkKey, WorkSlotStatus};
use spark_core::scope::ScopeId;
use spark_core::timeline::TimelineEpoch;
use spark_core::value::{CanonicalValue, FixedPoint};
use spark_engine::effects::scheduled_cohort_identity;
use spark_engine::engine::{CompletedHistory, Engine, ReplayError};
use spark_engine::fixture;
use spark_engine::ledger::OccurrenceLedgerKey;
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::profile::text::BoundedText;
use spark_engine::report::{
    CohortOutcome, CohortReport, PreflightFailure, WaveRejection, WaveReport,
};
use spark_engine::request::{
    canonicalize_active_request, CommandPayload, CommandRequest, Outcome, Request,
};
use spark_engine::rules::{
    DeclaredBudgets, Direction, Expr, Input, RuleSetSpec, RuleSpec, ScheduleMode, ScheduleOp,
    ScopeRef, Trigger, Update,
};
use spark_testkit::phase2::*;
use std::collections::BTreeMap;

const ONE: i64 = 1_000_000;

/// A test-only engine mutation (a boxed seam call).
type Mutation = Box<dyn Fn(&mut Engine)>;

fn bron() -> ScopeId {
    actor("bron")
}

fn add(v: i64) -> Update {
    Update::Add(lit(v))
}

fn adder(id: &str, kind: &str, target: &str, v: i64) -> RuleSpec {
    work_rule(id, kind, vec![emit("x", target, add(v))])
}

fn engine_with(b: DeclaredBudgets, rules: Vec<RuleSpec>, init: Vec<(&str, u64, &str)>) -> Engine {
    engine_with_baselines(b, rules, init, vec![])
}

fn engine_with_baselines(
    b: DeclaredBudgets,
    rules: Vec<RuleSpec>,
    init: Vec<(&str, u64, &str)>,
    baselines: Vec<spark_engine::rules::BaselineDeclaration>,
) -> Engine {
    let mut f = standard();
    Engine::genesis(
        f.genesis_with(
            b,
            rules,
            baselines,
            init.into_iter()
                .map(|(r, t, k)| initial(r, &bron(), t, k))
                .collect(),
        ),
    )
    .unwrap()
}

/// `state.stress = 20` at `t = 1` through a command, then the given rules.
fn seeded(b: DeclaredBudgets, mut rules: Vec<RuleSpec>, init: Vec<(&str, u64, &str)>) -> Engine {
    rules.push(on_command(
        "rule.seed",
        "cmd.seed",
        vec![emit("seed", "state.stress", Update::Assign(lit(20)))],
    ));
    let mut e = engine_with(b, rules, init);
    assert_eq!(
        e.process(&command("cmd.seed", 1, 1, "cmd.seed", bron()))
            .outcome(),
        &Outcome::Completed
    );
    e
}

fn run(e: &mut Engine, t: u64) -> Vec<CohortReport> {
    reports(&drive(e, &advance(t)))
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

// One argument per component of the frozen v3 §4.4 formula.
#[allow(clippy::too_many_arguments)]
fn emission(
    cohort: &Digest,
    wave: u32,
    parent: &Digest,
    rule_fp: &Digest,
    sub: &str,
    producer: &ScopeId,
    target: &str,
    target_scope: &ScopeId,
    artifact: &Digest,
) -> Digest {
    let mut e = CanonicalEncoder::new();
    e.push_str("emission");
    e.push_digest(cohort);
    e.push_u32(wave);
    e.push_digest(parent);
    e.push_digest(rule_fp);
    e.push_u64(1);
    tag(sub).canonicalize(&mut e);
    producer.canonicalize(&mut e);
    def(target).canonicalize(&mut e);
    target_scope.canonicalize(&mut e);
    e.push_digest(artifact);
    e.finish()
}

fn artifact(e: &Engine) -> Digest {
    e.epoch_registry().current().unwrap().record_hash()
}

fn fp(e: &Engine, rule: &str) -> Digest {
    e.rule_set().rule(&def(rule)).unwrap().fingerprint().clone()
}

fn records_by(e: &Engine, creator: &str) -> Vec<spark_engine::obligation::ObligationRecord> {
    e.obligations()
        .keys()
        .filter_map(|k| e.obligations().get(k))
        .flat_map(|s| s.records().cloned().collect::<Vec<_>>())
        .filter(|r| r.creator_rule_id() == &def(creator))
        .collect()
}

/// Wave-0 identity of a scheduled-work rule emitting sub-ID `x` to
/// `state.stress@bron` in the cohort at `due`, from the frozen formula.
fn wave0_identity(e: &Engine, cohort: &Digest, key: &WorkKey, rule: &str) -> Digest {
    emission(
        cohort,
        0,
        &parents_workkey(key),
        &fp(e, rule),
        "x",
        &bron(),
        "state.stress",
        &bron(),
        &artifact(e),
    )
}

/// The rising-threshold watcher: stress crossing 40 rings the alarm (wave 1)
/// and schedules follow-up work.
fn alarm_rule(ring: Update) -> RuleSpec {
    with_schedule(
        rule(
            "rule.alarm",
            Trigger::Crossing {
                watched: def("state.stress"),
                threshold: 40,
                direction: Direction::Rising,
            },
            vec![emit("ring", "state.alarm", ring)],
        ),
        reevaluate_after("follow", 10, "work.follow", "rule.follow"),
    )
}

fn threshold_rules(a: i64, b: i64, ring: Update) -> Vec<RuleSpec> {
    vec![
        adder("rule.a", "work.one", "state.stress", a),
        adder("rule.b", "work.two", "state.stress", b),
        alarm_rule(ring),
        adder("rule.follow", "work.follow", "state.echo", 1),
    ]
}

fn threshold_engine(a: i64, b: i64) -> Engine {
    seeded(
        budgets(8),
        threshold_rules(a, b, add(1)),
        vec![("rule.a", 100, "work.one"), ("rule.b", 100, "work.two")],
    )
}

fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut out = Vec::new();
    for i in 0..items.len() {
        let mut rest = items.to_vec();
        let head = rest.remove(i);
        for mut tail in permutations(&rest) {
            tail.insert(0, head.clone());
            out.push(tail);
        }
    }
    out
}

// ================================================================ AT-I1

/// AT-I1: the engine state after a wave equals the pre-wave snapshot plus the
/// same effects applied to an untouched clone. Two rules read and write
/// `state.stress` (20): each delta is the pre-wave 20, so 60. Negative
/// control: a sequential re-read (20 → 40 → 80) differs in value and digest.
#[test]
fn at_i1_wave_result_is_the_snapshot_plus_its_effects() {
    let reader = |id: &str, kind: &str| {
        work_rule(
            id,
            kind,
            vec![emit(
                "r",
                "state.stress",
                Update::Add(Expr::Input(cell("state.stress"))),
            )],
        )
    };
    let mut e = seeded(
        budgets(8),
        vec![reader("rule.r1", "work.one"), reader("rule.r2", "work.two")],
        vec![("rule.r1", 100, "work.one"), ("rule.r2", 100, "work.two")],
    );
    let untouched = e.clone();
    let r = run(&mut e, 100);
    let committed = r[0].waves[0].committed.clone();
    assert_eq!(value(&e, "state.stress", &bron()), Some(60));
    let mut clone = untouched.clone();
    fixture::extract_least_due_slice(&mut clone, LogicalTime(100)).unwrap();
    fixture::apply_committed_effects(&mut clone, &committed, LogicalTime(100));
    assert_eq!(clone.engine_state_digest(), e.engine_state_digest());
    let mut sequential = untouched;
    fixture::extract_least_due_slice(&mut sequential, LogicalTime(100)).unwrap();
    let mut reread = committed;
    reread[0].value = CanonicalValue::Int(80);
    fixture::apply_committed_effects(&mut sequential, &reread, LogicalTime(100));
    assert_ne!(sequential.engine_state_digest(), e.engine_state_digest());
}

// ================================================================ AT-I3

/// AT-I3 engine corpus: equal RESULT coalescing, a singleton TRANSFORM, and an
/// additive fold, under every rule-declaration permutation and reversed work
/// insertion: identical reports and digests.
#[test]
fn at_i3_declaration_permutations_are_digest_identical() {
    let rules = vec![
        work_rule(
            "rule.r1",
            "work.one",
            vec![emit("x", "state.stress", Update::Assign(lit(30)))],
        ),
        work_rule(
            "rule.r2",
            "work.two",
            vec![emit("x", "state.stress", Update::Assign(lit(30)))],
        ),
        work_rule(
            "rule.t",
            "work.three",
            vec![emit(
                "x",
                "state.mood",
                Update::Scale(FixedPoint::from_raw(1_500_000)),
            )],
        ),
        adder("rule.e1", "work.four", "state.energy", 10),
    ];
    let init = vec![
        ("rule.r1", 100, "work.one"),
        ("rule.r2", 100, "work.two"),
        ("rule.t", 100, "work.three"),
        ("rule.e1", 100, "work.four"),
    ];
    let mut reference: Option<(Vec<CohortReport>, (Digest, Digest))> = None;
    for perm in permutations(&rules) {
        for reversed in [false, true] {
            let mut i = init.clone();
            if reversed {
                i.reverse();
            }
            let mut e = seeded(budgets(8), perm.clone(), i);
            let r = run(&mut e, 100);
            assert_eq!(r[0].outcome, CohortOutcome::Committed);
            let got = (r, digest_pair(&e));
            match &reference {
                None => reference = Some(got),
                Some(expected) => assert_eq!(&got, expected),
            }
        }
    }
    let (r, _) = reference.unwrap();
    assert_eq!(r[0].waves[0].committed.len(), 3);
}

// ================================================================ AT-I4

/// AT-I4: every semantic `WorkKey` field is in cohort identity.
#[test]
fn at_i4_every_workkey_field_moves_cohort_identity() {
    let base = WorkKey {
        due_time: LogicalTime(100),
        profile_id: profile_id(),
        producer_definition_id: def("rule.a"),
        scope_id: bron(),
        occurrence_index: OccurrenceIndex(0),
        work_kind: work("work.one"),
    };
    let id = |k: &WorkKey| {
        scheduled_cohort_identity(&profile_id(), LogicalTime(100), std::slice::from_ref(k))
    };
    let reference = id(&base);
    let mut variants = Vec::new();
    let mut k = base.clone();
    k.due_time = LogicalTime(101);
    variants.push(("due_time", k));
    let mut k = base.clone();
    k.profile_id = spark_core::id::ProfileId::new("elsewhere").unwrap();
    variants.push(("profile", k));
    let mut k = base.clone();
    k.producer_definition_id = def("rule.b");
    variants.push(("producer", k));
    let mut k = base.clone();
    k.scope_id = actor("other");
    variants.push(("scope", k));
    let mut k = base.clone();
    k.occurrence_index = OccurrenceIndex(1);
    variants.push(("occurrence", k));
    let mut k = base.clone();
    k.work_kind = work("work.two");
    variants.push(("work_kind", k));
    for (field, k) in variants {
        assert_ne!(id(&k), reference, "{field}");
    }
    // Engine level: the observed identity is the formula over the extracted
    // executable keys.
    let mut e = engine_with(
        budgets(4),
        vec![adder("rule.a", "work.one", "state.stress", 1)],
        vec![("rule.a", 100, "work.one")],
    );
    let key = e.obligations().keys().next().unwrap().clone();
    run(&mut e, 100);
    assert_eq!(
        fixture::observation(&e).prewave[0].cohort_identity,
        scheduled_cohort_identity(&profile_id(), LogicalTime(100), &[key])
    );
}

// ================================================================ AT-I5

/// AT-I5: renaming rules so their lexical order flips never turns an
/// incompatible group into an accepted one and never changes a reduced result
/// or its evidence.
#[test]
fn at_i5_renaming_never_changes_classification_or_results() {
    let pair = |x: &str, y: &str, ux: Update, uy: Update| {
        let mut e = seeded(
            budgets(8),
            vec![
                work_rule(x, "work.one", vec![emit("x", "state.stress", ux)]),
                work_rule(y, "work.two", vec![emit("x", "state.stress", uy)]),
            ],
            vec![(x, 100, "work.one"), (y, 100, "work.two")],
        );
        let r = run(&mut e, 100);
        (r[0].outcome.clone(), value(&e, "state.stress", &bron()))
    };
    for (x, y) in [
        ("rule.a", "rule.z"),
        ("rule.z", "rule.a"),
        ("rule.m", "rule.b"),
    ] {
        let (outcome, v) = pair(x, y, Update::Assign(lit(30)), Update::Assign(lit(35)));
        assert!(
            matches!(
                &outcome,
                CohortOutcome::Rejected {
                    wave: 0,
                    rejection: WaveRejection::ResultConflict { values, .. }
                } if values.iter().copied().collect::<Vec<_>>() == vec![30, 35]
            ),
            "{x}/{y}: {outcome:?}"
        );
        assert_eq!(v, Some(20));
        let (outcome, v) = pair(x, y, add(10), add(15));
        assert_eq!(outcome, CohortOutcome::Committed);
        assert_eq!(v, Some(45), "{x}/{y}");
    }
}

// ================================================================ AT-I6a / AT-I6c(d)(e)(f) / AT-I6d

fn param_delta(base: i64) -> Update {
    // base + param(0): the parameter changes the payload, never the identity.
    Update::Add(Expr::WeightedSum(vec![
        (FixedPoint::from_raw(ONE), Input::Literal(base)),
        (FixedPoint::from_raw(ONE), Input::Param { index: 0 }),
    ]))
}

/// AT-I6a at engine level (through the seed-duplication seam): an exact
/// duplicate emission in a wave applies once (`20 + 10 → 30`, one cause);
/// the same identity with a different payload rejects the wave atomically with
/// the pre-wave digest preserved — in every declaration permutation.
#[test]
fn at_i6a_exact_duplicate_folds_and_reused_identity_rejects() {
    let rules = vec![
        work_rule(
            "rule.d",
            "work.one",
            vec![emit("x", "state.stress", param_delta(10))],
        ),
        adder("rule.other", "work.two", "state.mood", 1),
    ];
    let init = vec![("rule.d", 100, "work.one"), ("rule.other", 100, "work.two")];
    for perm in permutations(&rules) {
        let mut exact = seeded(budgets(8), perm.clone(), init.clone());
        fixture::set_seed_duplication(&mut exact, Some(false));
        let r = run(&mut exact, 100);
        assert_eq!(r[0].outcome, CohortOutcome::Committed);
        assert_eq!(
            value(&exact, "state.stress", &bron()),
            Some(30),
            "applies once"
        );
        let stress = r[0].waves[0]
            .committed
            .iter()
            .find(|c| c.definition == def("state.stress"))
            .unwrap();
        assert_eq!(stress.provenance.retained.len(), 1, "one cause");
        let mut contested = seeded(budgets(8), perm, init.clone());
        fixture::set_seed_duplication(&mut contested, Some(true));
        let r = run(&mut contested, 100);
        assert!(matches!(
            r[0].outcome,
            CohortOutcome::Rejected {
                wave: 0,
                rejection: WaveRejection::ContestedEmission { .. }
            }
        ));
        assert_eq!(
            contested.engine_state_digest(),
            fixture::observation(&contested)
                .prewave
                .last()
                .unwrap()
                .engine_digest
                .value()
                .clone(),
            "pre-wave digest preserved"
        );
    }
}

/// AT-I6c(d): derived redelivery — an exact duplicate of a wave-1 derived
/// emission folds (the alarm rings once and the result equals the run without
/// duplication); one derived identity with distinct payloads rejects wave 1
/// atomically while wave 0 stays committed.
#[test]
fn at_i6c_d_derived_redelivery() {
    let build = || {
        seeded(
            budgets(8),
            threshold_rules(10, 15, param_delta(1)),
            vec![("rule.a", 100, "work.one"), ("rule.b", 100, "work.two")],
        )
    };
    let mut plain = build();
    let rp = run(&mut plain, 100);
    let mut exact = build();
    fixture::set_seed_duplication(&mut exact, Some(false));
    let re = run(&mut exact, 100);
    assert_eq!(value(&exact, "state.alarm", &bron()), Some(1), "rings once");
    assert_eq!(rp, re);
    assert_eq!(digest_pair(&plain), digest_pair(&exact));
    let mut contested = build();
    fixture::set_seed_duplication(&mut contested, Some(true));
    let r = run(&mut contested, 100);
    assert!(matches!(
        r[0].outcome,
        CohortOutcome::Rejected {
            wave: 1,
            rejection: WaveRejection::ContestedEmission { .. }
        }
    ));
    assert_eq!(
        value(&contested, "state.stress", &bron()),
        Some(45),
        "wave 0 committed"
    );
    assert_eq!(value(&contested, "state.alarm", &bron()), None);
}

/// AT-I6c(e): depth transitivity. A wave-2 emission's parents are the two
/// wave-1 emission identities it read to become eligible; perturbing a wave-0
/// parent (a third contributing rule) changes the wave-2 identity.
#[test]
fn at_i6c_e_depth_two_parents_are_wave_one_identities() {
    let chain = |extra: bool| {
        let mut rules = vec![
            adder("rule.a", "work.one", "state.stress", 10),
            adder("rule.b", "work.two", "state.stress", 15),
            rule(
                "rule.c1",
                Trigger::Change {
                    watched: def("state.stress"),
                },
                vec![emit("e", "state.echo", add(1))],
            ),
            rule(
                "rule.c2",
                Trigger::Change {
                    watched: def("state.stress"),
                },
                vec![emit("e", "state.echo2", add(1))],
            ),
            with_condition(
                rule(
                    "rule.d",
                    Trigger::Change {
                        watched: def("state.echo"),
                    },
                    vec![emit("ring", "state.alarm", add(1))],
                ),
                compare(
                    Expr::Input(cell("state.echo2")),
                    spark_engine::rules::CmpOp::Gt,
                    lit(0),
                ),
            ),
        ];
        let mut init = vec![("rule.a", 100, "work.one"), ("rule.b", 100, "work.two")];
        if extra {
            rules.push(adder("rule.aaa", "work.three", "state.stress", 0));
            init.push(("rule.aaa", 100, "work.three"));
        }
        let mut e = seeded(budgets(8), rules, init);
        let r = run(&mut e, 100);
        assert_eq!(r[0].waves.len(), 3);
        let wave1: Vec<Digest> = r[0].waves[1]
            .committed
            .iter()
            .flat_map(|c| c.provenance.retained.clone())
            .collect();
        assert_eq!(wave1.len(), 2);
        let alarm = r[0].waves[2].committed[0].provenance.retained.clone();
        // The seed command's own cohort cascades too; the scheduled cohort is
        // the last one whose wave 0 was observed.
        let cohort = fixture::observation(&e)
            .prewave
            .iter()
            .rfind(|p| p.wave_index == 0)
            .unwrap()
            .cohort_identity
            .clone();
        let expected = emission(
            &cohort,
            2,
            &parents_emission(&wave1),
            &fp(&e, "rule.d"),
            "ring",
            &bron(),
            "state.alarm",
            &bron(),
            &artifact(&e),
        );
        assert_eq!(alarm, vec![expected.clone()]);
        for single in &wave1 {
            assert_ne!(
                expected,
                emission(
                    &cohort,
                    2,
                    &parents_emission(std::slice::from_ref(single)),
                    &fp(&e, "rule.d"),
                    "ring",
                    &bron(),
                    "state.alarm",
                    &bron(),
                    &artifact(&e),
                ),
                "negative control: one wave-1 parent"
            );
        }
        expected
    };
    assert_ne!(
        chain(false),
        chain(true),
        "a wave-0 perturbation reaches wave 2"
    );
}

/// AT-I6c(f): a forged empty parent set at wave 1 is a typed evaluator defect;
/// the state equals the post-wave-0 (pre-wave-1) digest.
#[test]
fn at_i6c_f_forged_empty_parent_set_rejects_as_a_defect() {
    let mut e = threshold_engine(10, 15);
    fixture::forge_empty_parents(&mut e, true);
    let r = run(&mut e, 100);
    assert_eq!(
        r[0].outcome,
        CohortOutcome::Rejected {
            wave: 1,
            rejection: WaveRejection::EmptyParentSet
        }
    );
    let prewave1 = &fixture::observation(&e).prewave;
    let wave1 = prewave1.iter().rfind(|p| p.wave_index == 1).unwrap();
    assert_eq!(&e.engine_state_digest(), wave1.engine_digest.value());
    assert_eq!(value(&e, "state.alarm", &bron()), None);
}

/// AT-I6d (renaming sub-case): renaming a contributing rule so it becomes the
/// lexical minimum changes the identity only because the rule's content
/// changed — the identity is still the formula over the **complete** parent
/// set, never a selected parent; permuting enumeration alone changes nothing.
#[test]
fn at_i6d_renaming_a_parent_never_makes_it_the_identity() {
    let identity_of = |first: &str, reverse: bool| {
        let mut rules = vec![
            adder(first, "work.one", "state.stress", 10),
            adder("rule.b", "work.two", "state.stress", 15),
            alarm_rule(add(1)),
            adder("rule.follow", "work.follow", "state.echo", 1),
        ];
        let mut init = vec![(first, 100, "work.one"), ("rule.b", 100, "work.two")];
        if reverse {
            rules.reverse();
            init.reverse();
        }
        let mut e = seeded(budgets(8), rules, init);
        let r = run(&mut e, 100);
        let parents = r[0].waves[0].committed[0].provenance.retained.clone();
        assert_eq!(parents.len(), 2);
        let cohort = fixture::observation(&e).prewave[1].cohort_identity.clone();
        let actual = records_by(&e, "rule.alarm")[0]
            .creator_emission_identity()
            .clone();
        let formula = |p: &[Digest]| {
            emission(
                &cohort,
                1,
                &parents_emission(p),
                &fp(&e, "rule.alarm"),
                "follow",
                &bron(),
                "rule.alarm",
                &bron(),
                &artifact(&e),
            )
        };
        assert_eq!(actual, formula(&parents));
        let min = parents.iter().min().unwrap().clone();
        assert_ne!(actual, formula(&[min]), "never the minimum parent");
        actual
    };
    let original = identity_of("rule.a", false);
    assert_eq!(
        original,
        identity_of("rule.a", true),
        "enumeration only: identical"
    );
    assert_ne!(
        original,
        identity_of("rule.0a", false),
        "content (fingerprint) changed"
    );
}

// ================================================================ AT-I7e

/// AT-I7e: an accumulation that overflows even `i128` is a typed, atomic
/// rejection (three `i64::MAX · i64::MAX` weighted terms).
#[test]
fn at_i7e_i128_accumulation_overflow_is_typed_and_atomic() {
    let huge = (FixedPoint::from_raw(i64::MAX), Input::Literal(i64::MAX));
    let mut e = seeded(
        budgets(8),
        vec![work_rule(
            "rule.w",
            "work.one",
            vec![emit(
                "x",
                "state.stress",
                Update::Assign(Expr::WeightedSum(vec![huge.clone(), huge.clone(), huge])),
            )],
        )],
        vec![("rule.w", 100, "work.one")],
    );
    let r = run(&mut e, 100);
    assert_eq!(
        r[0].outcome,
        CohortOutcome::Rejected {
            wave: 0,
            rejection: WaveRejection::Arithmetic {
                what: "weighted_sum"
            }
        }
    );
    assert_eq!(
        &e.engine_state_digest(),
        fixture::observation(&e)
            .prewave
            .last()
            .unwrap()
            .engine_digest
            .value()
    );
}

// ================================================================ AT-I8

/// AT-I8: the wave's enqueue preflight checks **full** commitment consistency
/// of every touched key. The key the wave will create already holds a slot
/// committed to record X while the store holds record Y: both stores look
/// "Scheduled, uncontested" (the shape the candidate checked), yet the
/// commitments disagree, so the wave rejects before any mutation. Positive
/// control: a consistent pre-existing claim is a legitimate collision that
/// commits and poisons the key, with the invariant intact afterwards.
#[test]
fn at_i8_enqueue_preflight_checks_full_commitment_consistency() {
    let build = || {
        engine_with(
            budgets(8),
            vec![with_schedule(
                adder("rule.r", "work.one", "state.stress", 1),
                reevaluate_after("n", 5, "work.one", "rule.r"),
            )],
            vec![("rule.r", 10, "work.one")],
        )
    };
    let e0 = build();
    let next = e0.occurrences().next_for(&OccurrenceLedgerKey {
        profile_id: profile_id(),
        producer: def("rule.r"),
        scope_id: bron(),
        work_kind: work("work.one"),
    });
    let k = WorkKey {
        due_time: LogicalTime(15),
        profile_id: profile_id(),
        producer_definition_id: def("rule.r"),
        scope_id: bron(),
        occurrence_index: OccurrenceIndex(next),
        work_kind: work("work.one"),
    };
    let record = |e: &Engine, claim: &str| {
        fixture::reevaluation_record(
            k.clone(),
            def("rule.r"),
            fp(e, "rule.r"),
            e.rule_set().content_hash().clone(),
            hash_bytes(claim.as_bytes()),
            artifact(e),
        )
    };
    let mut bad = build();
    let x = record(&bad, "x");
    let y = record(&bad, "y");
    fixture::schedule_record(&mut bad, x);
    fixture::tamper_obligations(&mut bad, &k, Some(vec![y]));
    assert_eq!(bad.slot_status(&k), WorkSlotStatus::Scheduled);
    assert!(
        !bad.obligations().get(&k).unwrap().is_contested(),
        "the shape check alone sees a consistent pair"
    );
    let r = run(&mut bad, 10);
    assert_eq!(
        r[0].outcome,
        CohortOutcome::Rejected {
            wave: 0,
            rejection: WaveRejection::Preflight {
                reason: PreflightFailure::BidirectionalInvariant
            }
        }
    );
    assert_eq!(
        &bad.engine_state_digest(),
        fixture::observation(&bad).prewave[0].engine_digest.value()
    );
    let mut good = build();
    let x = record(&good, "x");
    fixture::schedule_record(&mut good, x);
    let r = run(&mut good, 10);
    assert_eq!(r[0].outcome, CohortOutcome::Committed);
    assert_eq!(good.slot_status(&k), WorkSlotStatus::Conflicted);
    assert!(good.bidirectional_invariant_holds());
}

/// AT-I8(b)-style post-state check: after every committed wave of a two-wave
/// cohort, and after the cohort, the bidirectional invariant holds.
#[test]
fn at_i8_invariant_holds_after_every_committed_wave() {
    let mut e = threshold_engine(10, 15);
    assert!(e.bidirectional_invariant_holds());
    let r = run(&mut e, 100);
    assert_eq!(r[0].waves.len(), 2);
    assert!(e.bidirectional_invariant_holds());
    run(&mut e, 110);
    assert!(e.bidirectional_invariant_holds());
}

// ================================================================ AT-I15 / AT-I16

/// AT-I15 (v3): a contested record originating from a wave-1 multi-parent
/// derived emission. Under **one** rule set, two histories reach the same
/// crossing through different cause sets (`+10/+15` and `+20/+5`): the
/// follow-up records share their `WorkKey` and mode (the "payloads"
/// coincide) but not their parent-derived creator identity, so they stay
/// discriminated. Inserted in either order into one engine they poison the key
/// with equal digests, and the drain reports and removes both.
#[test]
fn at_i15_contested_records_from_derived_emissions_stay_discriminated() {
    let rules = vec![
        adder("rule.a", "work.one", "state.stress", 10),
        adder("rule.b", "work.two", "state.stress", 15),
        adder("rule.c", "work.three", "state.stress", 20),
        adder("rule.d", "work.four", "state.stress", 5),
        alarm_rule(add(1)),
        adder("rule.follow", "work.follow", "state.echo", 1),
    ];
    let follow = |pair: [(&str, &str); 2]| {
        let mut e = seeded(
            budgets(8),
            rules.clone(),
            vec![(pair[0].0, 100, pair[0].1), (pair[1].0, 100, pair[1].1)],
        );
        run(&mut e, 100);
        assert_eq!(value(&e, "state.stress", &bron()), Some(45));
        records_by(&e, "rule.alarm").remove(0)
    };
    let ra = follow([("rule.a", "work.one"), ("rule.b", "work.two")]);
    let rb = follow([("rule.c", "work.three"), ("rule.d", "work.four")]);
    assert_eq!(ra.key(), rb.key());
    assert_eq!(ra.mode(), rb.mode(), "payloads coincide");
    assert_ne!(
        ra.record_hash(),
        rb.record_hash(),
        "parent sets discriminate"
    );
    let insert = |order: [&spark_engine::obligation::ObligationRecord; 2]| {
        let mut e = seeded(budgets(8), rules.clone(), vec![]);
        drive(&mut e, &advance(100));
        for r in order {
            fixture::schedule_record(&mut e, r.clone());
        }
        e
    };
    let mut p = insert([&ra, &rb]);
    let q = insert([&rb, &ra]);
    assert_eq!(p.slot_status(ra.key()), WorkSlotStatus::Conflicted);
    assert_eq!(components(&p), components(&q));
    let r = run(&mut p, 110);
    let conflict = r
        .iter()
        .flat_map(|c| c.conflicts.clone())
        .find(|c| &c.key == ra.key())
        .unwrap();
    assert!(conflict
        .competing_payload_hashes
        .contains(&ra.record_hash()));
    assert!(conflict
        .competing_payload_hashes
        .contains(&rb.record_hash()));
    assert!(p.obligations().get(ra.key()).is_none());
}

/// AT-I16 (v3): two wave-1 derived claims with different parent sets compete
/// for one occurrence-ledger key; occurrences follow ascending complete
/// emission identity, and the mapping is unchanged under a pacing budget that
/// splits the boundary into several `process` calls.
#[test]
fn at_i16_derived_competing_claims_allocate_by_emission_identity() {
    let mapping = |budget: u32| {
        let rules = vec![
            work_rule(
                "rule.w",
                "work.one",
                vec![
                    emit_at("a", "state.stress", actor("a"), add(1)),
                    emit_at("b", "state.stress", actor("b"), add(1)),
                ],
            ),
            {
                let mut m = rule(
                    "rule.m",
                    Trigger::Change {
                        watched: def("state.stress"),
                    },
                    vec![],
                );
                m.schedules.push(ScheduleOp {
                    sub_id: tag("s"),
                    delay: 10,
                    work_kind: work("work.m"),
                    scope: ScopeRef::Fixed(region("north")),
                    mode: ScheduleMode::ReEvaluate {
                        rule: def("rule.agg"),
                    },
                });
                m
            },
            adder("rule.agg", "work.m", "state.echo", 1),
            adder("rule.early", "work.early", "state.mood", 1),
        ];
        let mut e = engine_with(
            budgets(budget),
            rules,
            vec![
                ("rule.w", 100, "work.one"),
                ("rule.early", 50, "work.early"),
            ],
        );
        let results = drive(&mut e, &advance(100));
        let mut by_occurrence: BTreeMap<u64, Digest> = BTreeMap::new();
        for r in records_by(&e, "rule.m") {
            by_occurrence.insert(
                r.key().occurrence_index.0,
                r.creator_emission_identity().clone(),
            );
        }
        (results.len(), by_occurrence)
    };
    let (calls_paced, paced) = mapping(1);
    let (calls_unpaced, unpaced) = mapping(100);
    assert!(
        calls_paced > calls_unpaced,
        "the budget splits the boundary"
    );
    assert_eq!(paced, unpaced);
    let ids: Vec<&Digest> = paced.values().collect();
    assert_eq!(ids.len(), 2);
    assert!(ids[0] < ids[1], "ascending emission identity");
    assert_eq!(paced.keys().copied().collect::<Vec<_>>(), vec![0, 1]);
}

// ================================================================ AT-I20b / AT-I20c(f)

/// AT-I20b: each semantic cap — effects per wave, enqueues per wave, cohort
/// candidates — rejects before any canonical mutation, consumes the cohort's
/// keys terminally with a typed report, replays identically, and a retry under
/// the next occurrence succeeds.
#[test]
fn at_i20b_every_semantic_cap_is_atomic_terminal_replayable_and_retryable() {
    let wide = work_rule(
        "rule.wide",
        "work.one",
        vec![
            emit("a", "state.stress", add(1)),
            emit("b", "state.mood", add(1)),
            emit("c", "state.energy", add(1)),
        ],
    );
    let mut enqueuing = adder("rule.enq", "work.one", "state.stress", 1);
    for s in ["s1", "s2", "s3"] {
        enqueuing
            .schedules
            .push(reevaluate_after(s, 5, &format!("work.{s}"), "rule.enq"));
    }
    let retry = with_schedule(
        on_command("rule.retry", "cmd.retry", vec![]),
        reevaluate_after("r", 1, "work.retry", "rule.small"),
    );
    let small = adder("rule.small", "work.retry", "state.echo", 1);
    for (name, rule, cap) in [
        ("max_effects_per_wave", wide, 2u32),
        ("max_enqueue_per_wave", enqueuing, 2),
    ] {
        let mut b = budgets(8);
        match name {
            "max_effects_per_wave" => b.max_effects_per_wave = cap,
            _ => b.max_enqueue_per_wave = cap,
        }
        let build = || {
            engine_with(
                b,
                vec![rule.clone(), retry.clone(), small.clone()],
                vec![(rule.rule_id.as_str(), 10, "work.one")],
            )
        };
        let mut e = build();
        let before = e.state().canonical_state_digest();
        let r = run(&mut e, 10);
        assert!(
            matches!(
                &r[0].outcome,
                CohortOutcome::Rejected {
                    wave: 0,
                    rejection: WaveRejection::SemanticCap { cap: c, observed: 3, bound }
                } if *c == name && *bound == cap
            ),
            "{name}: {:?}",
            r[0].outcome
        );
        assert_eq!(before, e.state().canonical_state_digest(), "{name}");
        assert_eq!(e.scheduled_work_count(), 0, "{name}: consumed terminally");
        let mut replay = build();
        assert_eq!(run(&mut replay, 10), r, "{name}: replay");
        let r = reports(&drive(
            &mut e,
            &command("cmd.retry", 11, 1, "cmd.retry", bron()),
        ));
        assert_eq!(r.last().unwrap().outcome, CohortOutcome::Committed);
        let r = run(&mut e, 12);
        assert_eq!(
            r[0].outcome,
            CohortOutcome::Committed,
            "{name}: retry succeeds"
        );
        assert_eq!(value(&e, "state.echo", &bron()), Some(1));
    }
}

/// AT-I20c(f): the oversized cohort (6 keys, budget 2) contains equal
/// (`+10/+10`) and unequal (`+10/+15`) co-target causes and an opposing
/// threshold input set (`+50/−40` on energy, threshold 20). It is admitted
/// whole: each target gets one write equal to `pre + Σ`; the threshold
/// decision follows the complete fold (no crossing), which a split admitting
/// `+50` first would violate; the paced run equals the unbudgeted run.
#[test]
fn at_i20c_f_oversized_cohort_with_co_target_and_threshold_inputs() {
    let build = |budget: u32| {
        seeded(
            budgets(budget),
            vec![
                adder("rule.w1", "work.k1", "state.stress", 10),
                adder("rule.w2", "work.k2", "state.stress", 10),
                adder("rule.w3", "work.k3", "state.mood", 10),
                adder("rule.w4", "work.k4", "state.mood", 15),
                adder("rule.w5", "work.k5", "state.energy", 50),
                adder("rule.w6", "work.k6", "state.energy", -40),
                with_schedule(
                    rule(
                        "rule.gate",
                        Trigger::Crossing {
                            watched: def("state.energy"),
                            threshold: 20,
                            direction: Direction::Rising,
                        },
                        vec![emit("ring", "state.alarm", add(1))],
                    ),
                    reevaluate_after("after", 5, "work.after", "rule.after"),
                ),
                adder("rule.after", "work.after", "state.echo", 1),
            ],
            (1..=6)
                .map(|i| {
                    let r: &'static str = Box::leak(format!("rule.w{i}").into_boxed_str());
                    let k: &'static str = Box::leak(format!("work.k{i}").into_boxed_str());
                    (r, 100, k)
                })
                .collect(),
        )
    };
    let mut paced = build(2);
    let first = paced.process(&advance(100));
    let d = first.diagnostics().unwrap();
    assert!(d.pacing_overrun);
    assert_eq!(d.admitted_work_key_count, 6, "admitted whole");
    assert_eq!(first.outcome(), &Outcome::Completed);
    let wave = &first.reports()[0].waves[0];
    assert_eq!(wave.committed.len(), 3, "one write per target");
    assert_eq!(value(&paced, "state.stress", &bron()), Some(40));
    assert_eq!(value(&paced, "state.mood", &bron()), Some(25));
    assert_eq!(value(&paced, "state.energy", &bron()), Some(10));
    assert_eq!(
        value(&paced, "state.alarm", &bron()),
        None,
        "complete fold never crosses"
    );
    assert!(records_by(&paced, "rule.gate").is_empty());
    let mut unpaced = build(100);
    let one = unpaced.process(&advance(100));
    assert_eq!(one.reports(), first.reports());
    assert_eq!(digest_pair(&unpaced), digest_pair(&paced));
}

// ================================================================ AT-I21

/// AT-I21 with interfering due work: the converted continuation (due 101) and
/// an interfering key at 101 that changes the deferred rule's input are
/// processed deterministically, before later work, as one cohort whose
/// identity is exactly the formula over those keys.
#[test]
fn at_i21_interference_is_deterministic_prioritized_and_exact() {
    let build = || {
        let rules = vec![
            adder("rule.s", "work.one", "state.stress", 1),
            adder("rule.i", "work.i", "state.stress", 1),
            rule(
                "rule.c",
                Trigger::Change {
                    watched: def("state.stress"),
                },
                vec![emit("e", "state.echo", add(1))],
            ),
            adder("rule.late", "work.late", "state.mood", 1),
        ];
        let mut shallow = budgets(8);
        shallow.max_wave_depth = 0;
        let mut f = standard();
        let rule_set = f
            .registry
            .activate_rule_set_without_depth_bound(
                &f.profile,
                &RuleSetSpec {
                    profile_id: profile_id(),
                    budgets: shallow,
                    rules: rules.clone(),
                    baselines: vec![],
                },
            )
            .unwrap();
        let mut g = f.genesis(
            budgets(8),
            rules,
            vec![
                initial("rule.s", &bron(), 100, "work.one"),
                initial("rule.i", &bron(), 101, "work.i"),
                initial("rule.late", &bron(), 102, "work.late"),
            ],
        );
        g.rule_set = rule_set;
        Engine::genesis(g).unwrap()
    };
    let mut e = build();
    let interfering = e
        .obligations()
        .keys()
        .find(|k| k.due_time == LogicalTime(101))
        .unwrap()
        .clone();
    let r = run(&mut e, 102);
    let times: Vec<u64> = r.iter().map(|c| c.canonical_time.0).collect();
    assert_eq!(times, vec![100, 101, 102], "continuation before later work");
    let converted = r[0].waves[0].depth_conversions[0].clone();
    assert_eq!(converted.due_time, LogicalTime(101));
    let at_101 = fixture::observation(&e)
        .prewave
        .iter()
        .find(|p| {
            p.wave_index == 0
                && p.cohort_identity != fixture::observation(&e).prewave[0].cohort_identity
        })
        .unwrap()
        .cohort_identity
        .clone();
    assert_eq!(
        at_101,
        scheduled_cohort_identity(&profile_id(), LogicalTime(101), &[converted, interfering]),
        "exact boundary identity"
    );
    let mut again = build();
    assert_eq!(run(&mut again, 102), r, "deterministic");
    assert_eq!(digest_pair(&again), digest_pair(&e));
}

// ================================================================ AT-I22 compositions

fn decay_engine(rules: Vec<RuleSpec>, init: Vec<(&str, u64, &str)>) -> Engine {
    let mut rules = rules;
    rules.push(on_command(
        "rule.setup",
        "cmd.setup",
        vec![emit("x", "state.stress", Update::Assign(lit(100)))],
    ));
    let mut e = engine_with_baselines(budgets(8), rules, init, vec![baseline("state.stress", 0)]);
    e.process(&command("cmd.setup", 0, 1, "cmd.setup", bron()));
    e
}

/// AT-I22 (v2): the two sanctioned compositions of decay with a shock, and the
/// frozen cross-family rejection when they coincide in one wave.
#[test]
fn at_i22_sanctioned_compositions_and_cross_family_rejection() {
    // (1) One rule body: decay three steps (100 → 70), then +5, committed at
    //     the cohort time.
    let mut body = decay_engine(
        vec![work_rule(
            "rule.body",
            "work.body",
            vec![
                emit("d", "state.stress", decay(10, 10)),
                emit("s", "state.stress", add(5)),
            ],
        )],
        vec![("rule.body", 30, "work.body")],
    );
    run(&mut body, 30);
    let c = body
        .state()
        .get(&profile_id(), &def("state.stress"), &bron())
        .unwrap();
    assert_eq!(
        (c.value.clone(), c.updated_at),
        (CanonicalValue::Int(75), LogicalTime(30))
    );
    // (2) Separate scheduled boundaries: decay at 30 (70, committed at 30),
    //     shock at 31 (75, committed at 31), decay at 40 (the grid step ending
    //     at 40 elapsed since 31: 65), decay at 41 (no grid step in (40, 41]:
    //     65, committed at 41).
    //
    //     Adapted by the second correction (C2R-01): the former expectation
    //     (75 at 40, 65 at 41) required the shock to re-phase the cadence
    //     clock, which is representable only by a backdated or extra
    //     per-cell phase — rejected by FINAL §4 and the frozen `StateCell`
    //     encoding. Both compositions remain deterministic and declared.
    let mut separate = decay_engine(
        vec![
            work_rule(
                "rule.decay",
                "work.decay",
                vec![emit("d", "state.stress", decay(10, 10))],
            ),
            adder("rule.shock", "work.shock", "state.stress", 5),
        ],
        vec![
            ("rule.decay", 30, "work.decay"),
            ("rule.shock", 31, "work.shock"),
            ("rule.decay", 40, "work.decay"),
            ("rule.decay", 41, "work.decay"),
        ],
    );
    for (t, expected) in [(30, 70), (31, 75), (40, 65), (41, 65)] {
        run(&mut separate, t);
        let c = separate
            .state()
            .get(&profile_id(), &def("state.stress"), &bron())
            .unwrap();
        assert_eq!(
            (c.value.clone(), c.updated_at),
            (CanonicalValue::Int(expected), LogicalTime(t)),
            "t={t}: value and canonical commit time"
        );
    }
    // (3) Coincident decay and shock in one wave: cross-family, rejected with
    //     the pre-wave digest preserved.
    let mut both = decay_engine(
        vec![
            work_rule(
                "rule.decay",
                "work.decay",
                vec![emit("d", "state.stress", decay(10, 10))],
            ),
            adder("rule.shock", "work.shock", "state.stress", 5),
        ],
        vec![
            ("rule.decay", 30, "work.decay"),
            ("rule.shock", 30, "work.shock"),
        ],
    );
    let r = run(&mut both, 30);
    assert!(matches!(
        r[0].outcome,
        CohortOutcome::Rejected {
            wave: 0,
            rejection: WaveRejection::FamilyMixture { .. }
        }
    ));
    assert_eq!(
        &both.engine_state_digest(),
        fixture::observation(&both)
            .prewave
            .last()
            .unwrap()
            .engine_digest
            .value()
    );
}

// ================================================================ AT-I24

/// AT-I24 `i128` boundary corpus: exact floors at the extremes, and typed
/// rejection whenever the single final result leaves `i64`.
#[test]
fn at_i24_i128_boundary_corpus() {
    let wide = ProfileManifest::new(
        profile_id(),
        BoundedText::new("wide").unwrap(),
        vec![
            int_spec("state.stress", Authority::SparkOwned, i64::MIN, i64::MAX),
            int_spec("state.total", Authority::Derived, i64::MIN, i64::MAX),
        ],
    );
    let case = |children: &[i64], weight: i64| {
        let mut f = activate(&wide);
        let child = on_command(
            "rule.child",
            "cmd.child",
            vec![emit("c", "state.stress", Update::Assign(param(0)))],
        );
        let agg = on_command(
            "rule.agg",
            "cmd.agg",
            vec![emit_at(
                "g",
                "state.total",
                region("north"),
                Update::Aggregate {
                    source: def("state.stress"),
                    weight: FixedPoint::from_raw(weight),
                },
            )],
        );
        let mut e = f.engine(budgets(8), vec![child, agg], vec![]);
        for (i, v) in children.iter().enumerate() {
            e.process(&Request::Command(command_request(
                &format!("cmd.c{i}"),
                1,
                i as u64 + 1,
                "cmd.child",
                actor(&format!("c{i}")),
                vec![],
                vec![*v],
            )));
        }
        let r = e.process(&Request::Command(command_request(
            "cmd.agg",
            2,
            100,
            "cmd.agg",
            bron(),
            vec![],
            vec![],
        )));
        match &r.reports()[0].outcome {
            CohortOutcome::Committed => value(&e, "state.total", &region("north")),
            CohortOutcome::Rejected {
                rejection: WaveRejection::Arithmetic { what: "aggregate" },
                ..
            } => None,
            other => panic!("{other:?}"),
        }
    };
    assert_eq!(case(&[i64::MAX, i64::MIN], ONE), Some(-1));
    assert_eq!(case(&[i64::MIN, i64::MIN], ONE / 2), Some(i64::MIN));
    assert_eq!(case(&[i64::MAX, i64::MAX], ONE / 2), Some(i64::MAX));
    assert_eq!(case(&[-1, -2], ONE / 2), Some(-2), "floor(-1.5)");
    assert_eq!(case(&[i64::MAX, 1], ONE), None, "2^63 leaves i64");
    assert_eq!(case(&[i64::MIN], -ONE), None, "-MIN leaves i64");
    assert_eq!(
        case(&[5, 5, 5], ONE / 2),
        Some(7),
        "equal children stay distinct inputs"
    );
}

// ================================================================ AT-I26 / AT-I27

/// AT-I26: for each Phase-2 retained store, a state differing **only** in that
/// store differs in exactly that component and in the engine digest — which
/// kills an implementation that omits a store or commits it as a constant.
#[test]
fn at_i26_each_store_alone_moves_the_engine_digest() {
    let base = || {
        engine_with(
            budgets(4),
            vec![adder("rule.a", "work.one", "state.stress", 1)],
            vec![("rule.a", 10, "work.one")],
        )
    };
    let reference = base();
    let k = reference.obligations().keys().next().unwrap().clone();
    let mutations: Vec<(&str, usize, Mutation)> = vec![
        (
            "ObligationStore",
            4,
            Box::new(|e: &mut Engine| {
                let k = e.obligations().keys().next().unwrap().clone();
                let r = fixture::reevaluation_record(
                    k.clone(),
                    def("rule.a"),
                    fp(e, "rule.a"),
                    e.rule_set().content_hash().clone(),
                    hash_bytes(b"other claim"),
                    artifact(e),
                );
                fixture::tamper_obligations(e, &k, Some(vec![r]));
            }),
        ),
        (
            "OccurrenceLedger",
            5,
            Box::new(|e: &mut Engine| {
                fixture::set_occurrence_next(
                    e,
                    OccurrenceLedgerKey {
                        profile_id: profile_id(),
                        producer: def("rule.a"),
                        scope_id: bron(),
                        work_kind: work("work.one"),
                    },
                    9,
                )
            }),
        ),
        (
            "CooldownLedger",
            6,
            Box::new(|e: &mut Engine| {
                fixture::set_cooldown(e, def("rule.a"), bron(), LogicalTime(99))
            }),
        ),
        (
            "EpochRegistry",
            3,
            Box::new(|e: &mut Engine| fixture::append_epoch_record(e)),
        ),
    ];
    let _ = k;
    for (store, index, mutate) in mutations {
        let mut e = base();
        mutate(&mut e);
        let (a, b) = (components(&reference), components(&e));
        for i in 0..7 {
            if i == index {
                assert_ne!(a[i], b[i], "{store}: its own component moves");
            } else {
                assert_eq!(a[i], b[i], "{store}: component {i} must not move");
            }
        }
        assert_ne!(
            reference.engine_state_digest(),
            e.engine_state_digest(),
            "{store}: the engine digest commits it"
        );
    }
}

/// AT-I27: states with equal digests reached by permuted construction
/// (cooldowns and occurrence entries written in opposite orders) respond
/// identically — whole results and digests — to the same next inputs,
/// including an epoch activation and a timeline epoch reset.
#[test]
fn at_i27_equal_digests_respond_identically_across_epoch_and_reset() {
    let rules = vec![
        adder("rule.a", "work.one", "state.stress", 1),
        spark_engine::rules::RuleSpec {
            cooldown: Some(10),
            ..on_command(
                "rule.cool",
                "cmd.cool",
                vec![emit("c", "state.echo", add(1))],
            )
        },
    ];
    let build = |reverse: bool| {
        let mut e = engine_with(budgets(4), rules.clone(), vec![("rule.a", 30, "work.one")]);
        let occ = |p: &str| OccurrenceLedgerKey {
            profile_id: profile_id(),
            producer: def(p),
            scope_id: bron(),
            work_kind: work("work.x"),
        };
        let mut writes: Vec<Mutation> = vec![
            Box::new(|e: &mut Engine| {
                fixture::set_cooldown(e, def("rule.cool"), bron(), LogicalTime(12))
            }),
            Box::new(|e: &mut Engine| {
                fixture::set_cooldown(e, def("rule.cool"), actor("z"), LogicalTime(7))
            }),
            Box::new(move |e: &mut Engine| fixture::set_occurrence_next(e, occ("rule.p"), 3)),
            Box::new(move |e: &mut Engine| fixture::set_occurrence_next(e, occ("rule.q"), 5)),
        ];
        if reverse {
            writes.reverse();
        }
        for w in writes {
            w(&mut e);
        }
        e
    };
    let mut p = build(false);
    let mut q = build(true);
    assert_eq!(p.stable_boundary_digest(), q.stable_boundary_digest());
    let rs = p.rule_set().clone();
    let cfg = p.config().clone();
    let activation = Request::Command(CommandRequest {
        command_kind: kind("spark.epoch.activate"),
        payload: CommandPayload::ActivateEpoch {
            rule_set: rs,
            config: cfg,
        },
        ..command_request(
            "cmd.epoch",
            11,
            2,
            "spark.epoch.activate",
            bron(),
            vec![],
            vec![],
        )
    });
    let steps: Vec<Request> = vec![
        command("cmd.c1", 10, 1, "cmd.cool", bron()),
        activation,
        advance(20),
    ];
    for s in &steps {
        assert_eq!(drive(&mut p, s), drive(&mut q, s));
        assert_eq!(p.stable_boundary_digest(), q.stable_boundary_digest());
    }
    for e in [&mut p, &mut q] {
        e.reset_timeline_epoch(TimelineEpoch(1), SourceId::new("seq.two").unwrap())
            .unwrap();
    }
    assert_eq!(p.stable_boundary_digest(), q.stable_boundary_digest());
    let after_reset = Request::Command(CommandRequest {
        timeline_epoch: TimelineEpoch(1),
        ..command_request("cmd.c2", 25, 3, "cmd.cool", bron(), vec![], vec![])
    });
    for s in [after_reset, advance(40)] {
        let (a, b) = (drive(&mut p, &s), drive(&mut q, &s));
        assert_eq!(a, b);
        assert_eq!(p.stable_boundary_digest(), q.stable_boundary_digest());
    }
    // The pre-set cooldown (expiry 12) suppresses the command at 10; the one
    // at 25 fires once. Both engines agree at every step.
    assert_eq!(value(&p, "state.echo", &bron()), Some(1));
}

// ================================================================ AT-I30 / AT-I31

/// AT-I30: many dormant scopes (300 cells, no due work) change nothing about
/// the active cohort's semantic work: identical cohort identity, candidate
/// set, committed effects and emission identities, and outer-loop selection
/// count. (Physical scan cost is not claimed; see the report.)
#[test]
fn at_i30_dormant_scopes_are_not_touched_by_active_work() {
    let rules = vec![
        adder("rule.a", "work.one", "state.stress", 1),
        on_command(
            "rule.spawn",
            "cmd.spawn",
            vec![emit("s", "state.echo", Update::Assign(lit(1)))],
        ),
    ];
    let mut small = engine_with(budgets(4), rules.clone(), vec![("rule.a", 10, "work.one")]);
    let mut big = engine_with(budgets(4), rules, vec![("rule.a", 10, "work.one")]);
    for i in 0..300u64 {
        big.process(&command(
            &format!("cmd.s{i}"),
            1,
            i + 1,
            "cmd.spawn",
            actor(&format!("d{i}")),
        ));
    }
    fixture::clear_observation(&mut small);
    fixture::clear_observation(&mut big);
    let rs = run(&mut small, 10);
    let rb = run(&mut big, 10);
    let semantic = |w: &WaveReport| (w.candidate_set_digest.clone(), w.committed.clone());
    assert_eq!(semantic(&rs[0].waves[0]), semantic(&rb[0].waves[0]));
    assert_eq!(
        fixture::observation(&small).prewave[0].cohort_identity,
        fixture::observation(&big).prewave[0].cohort_identity
    );
    assert_eq!(
        fixture::observation(&small).outer_selections,
        fixture::observation(&big).outer_selections
    );
}

/// AT-I31: a dormant aggregate moves only at its scheduled occurrences, and
/// promotion reads the committed aggregate — never a recomputation from newer
/// children (no synthesized microhistory, ADR-0006 verification 8).
#[test]
fn at_i31_dormant_aggregate_updates_only_at_its_cadence() {
    let agg = with_schedule(
        work_rule(
            "rule.agg",
            "work.agg",
            vec![emit(
                "g",
                "state.total",
                Update::Aggregate {
                    source: def("state.stress"),
                    weight: FixedPoint::from_raw(ONE),
                },
            )],
        ),
        reevaluate_after("again", 10, "work.agg", "rule.agg"),
    );
    let child = on_command(
        "rule.child",
        "cmd.child",
        vec![emit("c", "state.stress", Update::Assign(param(0)))],
    );
    let promote = on_command(
        "rule.promote",
        "cmd.promote",
        vec![emit(
            "p",
            "state.mood",
            Update::Assign(Expr::Input(Input::Cell {
                definition: def("state.total"),
                scope: ScopeRef::Fixed(region("north")),
                absent: -1,
            })),
        )],
    );
    let mut f = standard();
    let mut e = f.engine(
        budgets(8),
        vec![agg, child, promote],
        vec![initial("rule.agg", &region("north"), 10, "work.agg")],
    );
    let child_cmd = |id: &str, t: u64, seq: u64, who: &str, v: i64| {
        Request::Command(command_request(
            id,
            t,
            seq,
            "cmd.child",
            actor(who),
            vec![],
            vec![v],
        ))
    };
    let total = |e: &Engine| value(e, "state.total", &region("north"));
    drive(&mut e, &child_cmd("cmd.a", 1, 1, "a", 5));
    drive(&mut e, &child_cmd("cmd.b", 2, 2, "b", 7));
    assert_eq!(total(&e), None, "no aggregate before its occurrence");
    drive(&mut e, &advance(10));
    assert_eq!(total(&e), Some(12));
    drive(&mut e, &child_cmd("cmd.c", 11, 3, "c", 100));
    drive(&mut e, &advance(15));
    assert_eq!(
        total(&e),
        Some(12),
        "children changed; aggregate waits for its cadence"
    );
    drive(&mut e, &command("cmd.p1", 16, 4, "cmd.promote", actor("p")));
    assert_eq!(
        value(&e, "state.mood", &actor("p")),
        Some(12),
        "promotion reads the committed aggregate"
    );
    drive(&mut e, &advance(20));
    assert_eq!(total(&e), Some(112));
    drive(&mut e, &command("cmd.p2", 21, 5, "cmd.promote", actor("q")));
    assert_eq!(value(&e, "state.mood", &actor("q")), Some(112));
}

// ================================================================ AT-I33

/// AT-I33: maximal parent-set cardinality at `max_cohort_candidates` (64
/// wave-0 causes) produces the exact identity over all 64 parents, with no
/// panic; one fewer candidate allowed is a typed cap rejection.
#[test]
fn at_i33_maximal_parent_set_is_exact_and_total() {
    let n = 64u32;
    let build = |cap: u32| {
        let mut rules = Vec::new();
        let mut init: Vec<(String, String)> = Vec::new();
        for i in 0..n {
            rules.push(adder(
                &format!("rule.w{i}"),
                &format!("work.k{i}"),
                "state.stress",
                1,
            ));
            init.push((format!("rule.w{i}"), format!("work.k{i}")));
        }
        rules.push(alarm_rule(add(1)));
        rules.push(adder("rule.follow", "work.follow", "state.echo", 1));
        let mut b = budgets(1000);
        b.max_cohort_candidates = cap;
        b.max_fan_out = 32;
        let init_refs: Vec<(&str, u64, &str)> = init
            .iter()
            .map(|(r, k)| (r.as_str(), 100, k.as_str()))
            .collect();
        seeded(b, rules, init_refs)
    };
    let mut e = build(n);
    let keys: Vec<WorkKey> = e
        .obligations()
        .keys()
        .filter(|k| k.due_time == LogicalTime(100))
        .cloned()
        .collect();
    assert_eq!(keys.len(), n as usize);
    let r = run(&mut e, 100);
    assert_eq!(r[0].outcome, CohortOutcome::Committed);
    assert_eq!(value(&e, "state.stress", &bron()), Some(20 + i64::from(n)));
    let cohort = fixture::observation(&e).prewave[1].cohort_identity.clone();
    let parents: Vec<Digest> = keys
        .iter()
        .map(|k| wave0_identity(&e, &cohort, k, k.producer_definition_id.as_str()))
        .collect();
    let expected = emission(
        &cohort,
        1,
        &parents_emission(&parents),
        &fp(&e, "rule.alarm"),
        "follow",
        &bron(),
        "rule.alarm",
        &bron(),
        &artifact(&e),
    );
    assert_eq!(
        records_by(&e, "rule.alarm")[0].creator_emission_identity(),
        &expected
    );
    let mut over = build(n - 1);
    let r = run(&mut over, 100);
    assert!(matches!(
        r[0].outcome,
        CohortOutcome::Rejected {
            rejection: WaveRejection::SemanticCap {
                cap: "max_cohort_candidates",
                ..
            },
            ..
        }
    ));
}

// ================================================================ AT-I35

fn golden_string(out: &mut Vec<u8>, s: &str) {
    out.extend_from_slice(&(s.len() as u64).to_le_bytes());
    out.extend_from_slice(s.as_bytes());
}

/// The independent reference for `canonicalize(ActiveRequest)`, written from
/// FINAL §6.2 as raw bytes.
fn golden_active(kind_tag: Option<&str>, horizon: u64, identity: &Digest) -> Vec<u8> {
    let mut out = Vec::new();
    match kind_tag {
        None => golden_string(&mut out, "active_request.none"),
        Some(k) => {
            golden_string(&mut out, "active_request.some");
            golden_string(&mut out, k);
            out.extend_from_slice(&horizon.to_le_bytes());
            out.extend_from_slice(identity.as_bytes());
        }
    }
    out
}

/// The independent reference for `stable_boundary_digest`: the Phase-1
/// encoder primitives only, never the production active-request canonicalizer.
fn reference_boundary(
    engine_digest: &Digest,
    frontier: u64,
    active: Option<(&str, u64, &Digest)>,
) -> Digest {
    let mut e = CanonicalEncoder::new();
    e.push_str("stable_boundary_v1");
    e.push_digest(engine_digest);
    e.push_u64(frontier);
    match active {
        None => {
            e.push_str("active_request.none");
        }
        Some((k, h, id)) => {
            e.push_str("active_request.some");
            e.push_str(k);
            e.push_u64(h);
            e.push_digest(id);
        }
    }
    e.finish()
}

/// AT-I35 (Revision-2 §7): golden byte vectors for `None`, `Some(Advance, 20)`
/// and `Some(Command, 20, id)`, and the stable-boundary digest over each from
/// an independent reference; mutants omitting `kind`, omitting `horizon`, or
/// reordering fields fail the byte comparison.
#[test]
fn at_i35_golden_active_request_and_boundary_vectors() {
    let build = || {
        engine_with(
            budgets(1),
            vec![
                adder("rule.a", "work.one", "state.stress", 1),
                adder("rule.b", "work.two", "state.mood", 1),
            ],
            vec![("rule.a", 10, "work.one"), ("rule.b", 15, "work.two")],
        )
    };
    let idle = build();
    let mut advancing = build();
    assert_eq!(advancing.process(&advance(20)).outcome(), &Outcome::Paused);
    let mut commanding = build();
    let cmd = command("cmd.one", 20, 1, "cmd.none", bron());
    assert_eq!(commanding.process(&cmd).outcome(), &Outcome::Paused);
    let cases: Vec<(&Engine, Option<&str>)> = vec![
        (&idle, None),
        (&advancing, Some("advance")),
        (&commanding, Some("command")),
    ];
    for (e, k) in cases {
        let active = e.active_request();
        assert_eq!(active.is_some(), k.is_some());
        let (h, id) = active
            .map(|a| (a.horizon().0, a.identity().clone()))
            .unwrap_or((0, Digest::ZERO));
        let mut enc = CanonicalEncoder::new();
        canonicalize_active_request(active, &mut enc);
        let actual = enc.into_bytes();
        assert_eq!(actual, golden_active(k, h, &id), "{k:?}");
        assert_eq!(
            e.stable_boundary_digest(),
            reference_boundary(
                &e.engine_state_digest(),
                e.frontier().0,
                k.map(|k| (k, h, &id))
            ),
            "{k:?}"
        );
        if let Some(k) = k {
            let mut no_kind = Vec::new();
            golden_string(&mut no_kind, "active_request.some");
            no_kind.extend_from_slice(&h.to_le_bytes());
            no_kind.extend_from_slice(id.as_bytes());
            let mut no_horizon = Vec::new();
            golden_string(&mut no_horizon, "active_request.some");
            golden_string(&mut no_horizon, k);
            no_horizon.extend_from_slice(id.as_bytes());
            let mut reordered = Vec::new();
            golden_string(&mut reordered, "active_request.some");
            reordered.extend_from_slice(&h.to_le_bytes());
            golden_string(&mut reordered, k);
            reordered.extend_from_slice(id.as_bytes());
            for (name, mutant) in [
                ("no kind", no_kind),
                ("no horizon", no_horizon),
                ("reordered", reordered),
            ] {
                assert_ne!(actual, mutant, "{k}: {name}");
            }
        }
    }
}

// ================================================================ AT-I38

/// AT-I38 (v3): provenance truncation never truncates the parent set. Under one
/// rule set and one cohort, nine zero-delta causes on stress are each gated by
/// `mood != i + 1`. The full history keeps all nine (the largest identity is
/// beyond `MAX_SOURCE_REFS`); the other silences exactly that largest cause.
/// Same bounded presentation, same arithmetic, different omitted count and
/// coverage — and a different later-wave emission identity, because the
/// parent set covers every canonicalized cause. Kills an implementation that
/// derives parents from the retained presentation.
#[test]
fn at_i38_parent_set_is_not_the_provenance_presentation() {
    let names: Vec<(String, String)> = (0..9)
        .map(|i| (format!("rule.w{i}"), format!("work.k{i}")))
        .collect();
    let rules = || {
        let mut v: Vec<RuleSpec> = (0..9)
            .map(|i| {
                with_condition(
                    adder(
                        &format!("rule.w{i}"),
                        &format!("work.k{i}"),
                        "state.stress",
                        0,
                    ),
                    compare(
                        Expr::Input(cell("state.mood")),
                        spark_engine::rules::CmpOp::Ne,
                        lit(i + 1),
                    ),
                )
            })
            .collect();
        v.push(with_schedule(
            rule(
                "rule.watch",
                Trigger::Change {
                    watched: def("state.stress"),
                },
                vec![],
            ),
            reevaluate_after("follow", 10, "work.follow", "rule.follow"),
        ));
        v.push(adder("rule.follow", "work.follow", "state.echo", 1));
        v.push(on_command(
            "rule.mood",
            "cmd.mood",
            vec![emit("m", "state.mood", Update::Assign(param(0)))],
        ));
        v
    };
    let build = |silence: Option<i64>| {
        let init: Vec<(&str, u64, &str)> = names
            .iter()
            .map(|(r, k)| (r.as_str(), 100, k.as_str()))
            .collect();
        let mut e = seeded(budgets(100), rules(), init);
        if let Some(j) = silence {
            e.process(&Request::Command(command_request(
                "cmd.mood",
                2,
                2,
                "cmd.mood",
                bron(),
                vec![],
                vec![j + 1],
            )));
        }
        e
    };
    let mut full = build(None);
    let keys: Vec<WorkKey> = full
        .obligations()
        .keys()
        .filter(|k| k.due_time == LogicalTime(100))
        .cloned()
        .collect();
    let rf = run(&mut full, 100);
    let cohort = fixture::observation(&full)
        .prewave
        .iter()
        .rfind(|p| p.wave_index == 0)
        .unwrap()
        .cohort_identity
        .clone();
    let (_, largest) = keys
        .iter()
        .map(|k| {
            let name = k.producer_definition_id.as_str().to_string();
            let i: i64 = name.trim_start_matches("rule.w").parse().unwrap();
            (wave0_identity(&full, &cohort, k, &name), i)
        })
        .max()
        .unwrap();
    let mut silenced = build(Some(largest));
    let rs = run(&mut silenced, 100);
    // The seed command's stress write also triggers the watcher (a cohort at
    // 11), so select the scheduled cohort at 100 in both runs.
    let at_100 = |r: &[CohortReport]| {
        r.iter()
            .find(|c| c.canonical_time == LogicalTime(100))
            .unwrap()
            .waves[0]
            .committed[0]
            .provenance
            .clone()
    };
    let (pf, ps) = (at_100(&rf), at_100(&rs));
    assert_eq!(pf.retained, ps.retained, "same bounded presentation");
    assert_eq!((pf.omitted_source_count, ps.omitted_source_count), (1, 0));
    assert_eq!(
        (pf.coverage_status, ps.coverage_status),
        (
            spark_engine::report::CoverageStatus::Truncated,
            spark_engine::report::CoverageStatus::Complete
        )
    );
    assert_eq!(
        value(&full, "state.stress", &bron()),
        value(&silenced, "state.stress", &bron())
    );
    assert_ne!(
        records_by(&full, "rule.watch")[0].creator_emission_identity(),
        records_by(&silenced, "rule.watch")[0].creator_emission_identity(),
        "the parent set covers the cause beyond the presentation cap"
    );
}

// ================================================================ AT-I39(E) / AT-I41

/// The independent `effect_batch_v3` reference, from v3 §4.5 and the
/// engine's documented committed-effect block.
fn reference_batch(
    epoch: u64,
    cohort_or_barrier: &Digest,
    prewave: &Digest,
    wave: &WaveReport,
    wave_index: u32,
) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("effect_batch_v3");
    profile_id().canonicalize(&mut enc);
    enc.push_u64(epoch);
    enc.push_digest(cohort_or_barrier);
    enc.push_digest(prewave);
    enc.push_u32(wave_index);
    enc.push_digest(&wave.candidate_set_digest);
    enc.push_u64(wave.committed.len() as u64);
    for e in &wave.committed {
        let mut inner = CanonicalEncoder::new();
        e.definition.canonicalize(&mut inner);
        e.scope.canonicalize(&mut inner);
        inner.push_str(e.write_path.tag());
        e.value.canonicalize(&mut inner);
        inner.push_u64(e.provenance.retained.len() as u64);
        for p in &e.provenance.retained {
            inner.push_digest(p);
        }
        inner.push_u64(e.provenance.omitted_source_count);
        enc.push_block(&inner);
    }
    enc.finish()
}

/// AT-I41 + AT-I39(E): two cohorts in one `process` call each run their own
/// wave chain from 0; every `effect_batch_digest` recomputes from its declared
/// components (cohort identity and a cohort-scoped wave index); the retired
/// heartbeat descriptor, computed independently, appears in none — and the
/// second cohort's wave-0 identities equal those of it alone.
#[test]
fn at_i41_batch_digest_recomputes_and_heartbeat_is_absent() {
    let rules = threshold_rules(10, 15, add(1));
    let mut both = seeded(
        budgets(8),
        rules.clone(),
        vec![("rule.a", 100, "work.one"), ("rule.b", 105, "work.two")],
    );
    fixture::clear_observation(&mut both);
    let r = run(&mut both, 105);
    assert_eq!(r.len(), 2);
    let obs = fixture::observation(&both).prewave.clone();
    let mut prewave_index = 0;
    for (ci, cohort_report) in r.iter().enumerate() {
        for w in &cohort_report.waves {
            let p = &obs[prewave_index];
            prewave_index += 1;
            assert_eq!(
                p.wave_index, w.wave_index,
                "cohort {ci}: cohort-scoped wave index"
            );
            let expected = reference_batch(
                1,
                &p.cohort_identity,
                p.engine_digest.value(),
                w,
                w.wave_index,
            );
            assert_eq!(
                w.effect_batch_digest, expected,
                "cohort {ci} wave {}",
                w.wave_index
            );
            // The retired v1 heartbeat barrier is not a component.
            let mut hb = CanonicalEncoder::new();
            hb.push_str("heartbeat");
            cohort_report.canonical_time.canonicalize(&mut hb);
            hb.push_digest(&p.cohort_identity);
            let heartbeat = hb.finish();
            assert_ne!(
                w.effect_batch_digest,
                reference_batch(1, &heartbeat, p.engine_digest.value(), w, w.wave_index),
                "heartbeat is absent"
            );
            if ci == 1 {
                assert_ne!(
                    w.effect_batch_digest,
                    reference_batch(
                        1,
                        &p.cohort_identity,
                        p.engine_digest.value(),
                        w,
                        w.wave_index + 1
                    ),
                    "a drain-scoped wave index is falsified"
                );
            }
        }
        assert_eq!(cohort_report.waves[0].wave_index, 0);
    }
    let mut alone = seeded(budgets(8), rules, vec![("rule.b", 105, "work.two")]);
    let ra = run(&mut alone, 105);
    assert_eq!(
        ra[0].waves[0].committed[0].provenance.retained,
        r[1].waves[0].committed[0].provenance.retained,
        "second cohort's wave-0 identity equals it alone"
    );
}

// ================================================================ AT-I44(c′)

/// AT-I44(c′): a slice `{S, X_conflicted}` whose scheduled member's wave 0
/// rejects reports the conflict section first, then the typed rejection; X's
/// claim set is removed completely; the post-state is the post-extraction
/// digest.
#[test]
fn at_i44_c_prime_mixed_slice_whose_scheduled_part_rejects() {
    let mut e = engine_with(
        budgets(8),
        vec![
            work_rule(
                "rule.s",
                "work.one",
                vec![emit("x", "state.bounded", Update::Assign(lit(11)))],
            ),
            adder("rule.x", "work.x", "state.echo", 1),
        ],
        vec![("rule.s", 101, "work.one")],
    );
    let x = WorkKey {
        due_time: LogicalTime(101),
        profile_id: profile_id(),
        producer_definition_id: def("rule.x"),
        scope_id: bron(),
        occurrence_index: OccurrenceIndex(0),
        work_kind: work("work.x"),
    };
    for claim in ["p", "q"] {
        let r = fixture::reevaluation_record(
            x.clone(),
            def("rule.x"),
            fp(&e, "rule.x"),
            e.rule_set().content_hash().clone(),
            hash_bytes(claim.as_bytes()),
            artifact(&e),
        );
        fixture::schedule_record(&mut e, r);
    }
    assert_eq!(e.slot_status(&x), WorkSlotStatus::Conflicted);
    let r = run(&mut e, 101);
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].conflicts.len(), 1);
    assert_eq!(r[0].conflicts[0].key, x);
    assert!(matches!(
        r[0].outcome,
        CohortOutcome::Rejected {
            wave: 0,
            rejection: WaveRejection::InvalidEffect { .. }
        }
    ));
    assert!(
        e.obligations().get(&x).is_none(),
        "the complete contested set is removed"
    );
    assert_eq!(e.slot_status(&x), WorkSlotStatus::Empty);
    assert_eq!(
        &e.engine_state_digest(),
        fixture::observation(&e).prewave[0].engine_digest.value(),
        "post-state is the post-extraction digest"
    );
    assert_eq!(
        value(&e, "state.echo", &bron()),
        None,
        "X never entered the candidate set"
    );
}

// ================================================================ AT-I50(d)

/// AT-I50(d), the payload case executed: a history whose command carries a
/// different payload under the same envelope fields cannot reproduce the
/// recorded history (the envelope binds only `canonical_payload_hash`, so the
/// payload must be supplied exactly).
#[test]
fn at_i50_d_replay_requires_the_exact_payload() {
    let mut f = standard();
    let g = f.genesis(
        budgets(4),
        vec![on_command(
            "rule.p",
            "cmd.p",
            vec![emit("x", "state.stress", Update::Add(param(0)))],
        )],
        vec![],
    );
    let mut e = Engine::genesis(g.clone()).unwrap();
    let original = command_request("cmd.one", 5, 1, "cmd.p", bron(), vec![], vec![7]);
    drive(&mut e, &Request::Command(original.clone()));
    drive(&mut e, &advance(9));
    let history = |c: CommandRequest| CompletedHistory {
        commands: vec![c],
        resets: vec![],
        frontier: e.frontier(),
        recorded_history_digest: e.timeline_history_digest(),
        recorded_stable_boundary_digest: e.stable_boundary_digest(),
    };
    let rebuilt = Engine::reconstruct_completed(g.clone(), &history(original.clone())).unwrap();
    assert_eq!(rebuilt.stable_boundary_digest(), e.stable_boundary_digest());
    let mut other = original;
    other.payload = CommandPayload::Host {
        subject: bron(),
        observations: vec![],
        params: vec![8],
    };
    assert_eq!(
        Engine::reconstruct_completed(g, &history(other)).unwrap_err(),
        ReplayError::Diverged
    );
}
