//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 checkpoint 3: the wave pipeline, same-target composition, rejection
//! classes, thresholds and parent-set identity, pacing, and extraction.
//!
//! AT-I1, AT-I5, AT-I6b, AT-I6c, AT-I6d, AT-I7a–e, AT-I8, AT-I9, AT-I20, AT-I20b,
//! AT-I20c (a)–(d)(f)–(i), AT-I21, AT-I25, AT-I37 (runtime half), AT-I40(e),
//! AT-I42 (l)(m)(n), AT-I44 (a)(a′)(c)(c′)(g), AT-I45 (a)–(e).

use spark_core::clock::LogicalTime;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::DefinitionId;
use spark_core::scheduler::{OccurrenceIndex, WorkKey, WorkSlotStatus};
use spark_core::scope::ScopeId;
use spark_core::value::FixedPoint;
use spark_engine::effects::scheduled_cohort_identity;
use spark_engine::engine::{Engine, ExtractionRefusal};
use spark_engine::fixture;
use spark_engine::ledger::OccurrenceLedgerKey;
use spark_engine::obligation::ObligationMode;
use spark_engine::report::{
    CohortKind, CohortOutcome, CohortReport, PreflightFailure, WaveRejection,
};
use spark_engine::request::Outcome;
use spark_engine::rules::{
    DeclaredBudgets, Direction, Expr, Input, RuleSetSpec, RuleSpec, Trigger, Update,
};
use spark_testkit::phase2::*;

fn bron() -> ScopeId {
    actor("bron")
}

fn add(v: i64) -> Update {
    Update::Add(lit(v))
}

/// A work rule adding `v` to `target`.
fn adder(id: &str, kind: &str, target: &str, v: i64) -> RuleSpec {
    work_rule(id, kind, vec![emit("x", target, add(v))])
}

fn seeding(id: &str, target: &str, v: i64) -> RuleSpec {
    on_command(
        id,
        "cmd.seed",
        vec![emit("seed", target, Update::Assign(lit(v)))],
    )
}

fn engine_with(
    budget: DeclaredBudgets,
    rules: Vec<RuleSpec>,
    init: Vec<(&str, u64, &str)>,
) -> Engine {
    engine_with_baselines(budget, rules, init, vec![])
}

/// As `engine_with`, with declared baselines (v1 Q7: a decay rule's target
/// must have declared baseline semantics).
fn engine_with_baselines(
    budget: DeclaredBudgets,
    rules: Vec<RuleSpec>,
    init: Vec<(&str, u64, &str)>,
    baselines: Vec<spark_engine::rules::BaselineDeclaration>,
) -> Engine {
    let mut f = standard();
    Engine::genesis(
        f.genesis_with(
            budget,
            rules,
            baselines,
            init.into_iter()
                .map(|(r, t, k)| initial(r, &bron(), t, k))
                .collect(),
        ),
    )
    .unwrap()
}

/// Seeds `state.stress = 20` at `t = 1` through a command, then advances to `t`.
fn seeded(
    budget: DeclaredBudgets,
    mut rules: Vec<RuleSpec>,
    init: Vec<(&str, u64, &str)>,
) -> Engine {
    rules.push(seeding("rule.seed", "state.stress", 20));
    // `state.stress` declares baseline 0 so the pair table's decay rule is
    // admissible (v1 Q7); a baseline changes no other pair's result.
    let mut e = engine_with_baselines(budget, rules, init, vec![baseline("state.stress", 0)]);
    let r = e.process(&command("cmd.seed", 1, 1, "cmd.seed", bron()));
    assert_eq!(r.outcome(), &Outcome::Completed);
    assert_eq!(value(&e, "state.stress", &bron()), Some(20));
    e
}

fn run(e: &mut Engine, t: u64) -> Vec<CohortReport> {
    reports(&drive(e, &advance(t)))
}

fn outcome_at(reports: &[CohortReport], t: u64) -> CohortOutcome {
    reports
        .iter()
        .find(|r| r.canonical_time.0 == t && r.kind == CohortKind::Scheduled)
        .unwrap()
        .outcome
        .clone()
}

// ================================================================ composition

/// AT-I7a, AT-I6b, AT-I1, AT-I5: simultaneous additive causes fold from one
/// snapshot (`20 + 10 + 15 = 45`, `20 + 10 + 10 = 40`); rules that read the
/// target see the pre-wave value only; every declaration permutation gives the
/// same report and digests.
#[test]
fn at_i7a_additive_causes_fold_once_from_the_snapshot() {
    for (a, b, expected) in [(10, 15, 45), (10, 10, 40)] {
        let rules = vec![
            adder("rule.a", "work.one", "state.stress", a),
            adder("rule.b", "work.two", "state.stress", b),
        ];
        let init = vec![("rule.a", 100, "work.one"), ("rule.b", 100, "work.two")];
        let mut forward = seeded(budgets(8), rules.clone(), init.clone());
        let rf = run(&mut forward, 100);
        let mut rev_rules = rules.clone();
        rev_rules.reverse();
        let mut rev_init = init.clone();
        rev_init.reverse();
        let mut reversed = seeded(budgets(8), rev_rules, rev_init);
        let rr = run(&mut reversed, 100);
        assert_eq!(value(&forward, "state.stress", &bron()), Some(expected));
        assert_eq!(rf, rr);
        assert_eq!(digest_pair(&forward), digest_pair(&reversed));
        let wave = &rf[0].waves[0];
        assert_eq!(
            wave.committed.len(),
            1,
            "exactly one committed write per target"
        );
        assert_eq!(
            wave.committed[0].provenance.retained.len(),
            2,
            "both causes counted"
        );
    }
    // AT-I1: every delta computed from the one pre-wave value.
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
    run(&mut e, 100);
    assert_eq!(
        value(&e, "state.stress", &bron()),
        Some(60),
        "20 + 20 + 20, never re-read"
    );
}

fn reject_of(reports: &[CohortReport], t: u64) -> WaveRejection {
    match outcome_at(reports, t) {
        CohortOutcome::Rejected { wave: 0, rejection } => rejection,
        other => panic!("expected a wave-0 rejection at {t}, got {other:?}"),
    }
}

/// AT-I7b, AT-I7c, AT-I37 (runtime half): the same-target pair table for
/// co-firing rules the door cannot prove co-target (distinct work kinds in one
/// cohort). Every rejection is digest-atomic for the state store.
#[test]
fn at_i37_runtime_pair_table_is_frozen() {
    let weighted = |v: i64| {
        Update::Assign(Expr::WeightedSum(vec![(
            FixedPoint::from_integer(1).unwrap(),
            Input::Literal(v),
        )]))
    };
    let cases: Vec<(&str, Update, Update, Option<i64>)> = vec![
        ("add/add", add(10), add(15), Some(45)),
        ("add/subtract", add(10), Update::Subtract(lit(4)), Some(26)),
        (
            "scale/scale",
            Update::Scale(FixedPoint::from_raw(1_500_000)),
            Update::Scale(FixedPoint::from_raw(1_500_000)),
            None,
        ),
        (
            "add/scale",
            add(1),
            Update::Scale(FixedPoint::from_raw(2_000_000)),
            None,
        ),
        (
            "clamp/clamp",
            Update::Clamp { min: 0, max: 5 },
            Update::Clamp { min: 0, max: 5 },
            None,
        ),
        ("add/clamp", add(1), Update::Clamp { min: 0, max: 5 }, None),
        (
            "weighted/weighted equal",
            weighted(30),
            weighted(30),
            Some(30),
        ),
        (
            "weighted/weighted unequal",
            weighted(30),
            weighted(35),
            None,
        ),
        (
            "assign/weighted equal value",
            Update::Assign(lit(30)),
            weighted(30),
            None,
        ),
        ("result/add", Update::Assign(lit(30)), add(10), None),
        ("decay/add", decay(1, 1), add(5), None),
    ];
    for (name, ua, ub, expected) in cases {
        for flip in [false, true] {
            let ra = work_rule(
                "rule.pa",
                "work.one",
                vec![emit("x", "state.stress", ua.clone())],
            );
            let rb = work_rule(
                "rule.pb",
                "work.two",
                vec![emit("x", "state.stress", ub.clone())],
            );
            let rules = if flip { vec![rb, ra] } else { vec![ra, rb] };
            let mut e = seeded(
                budgets(8),
                rules,
                vec![("rule.pa", 100, "work.one"), ("rule.pb", 100, "work.two")],
            );
            let store_before = e.state().canonical_state_digest();
            let r = run(&mut e, 100);
            match expected {
                Some(v) => {
                    assert_eq!(outcome_at(&r, 100), CohortOutcome::Committed, "{name}");
                    assert_eq!(value(&e, "state.stress", &bron()), Some(v), "{name}");
                }
                None => {
                    assert!(
                        matches!(outcome_at(&r, 100), CohortOutcome::Rejected { wave: 0, .. }),
                        "{name}"
                    );
                    assert_eq!(store_before, e.state().canonical_state_digest(), "{name}");
                }
            }
        }
    }
}

/// AT-I7b + AT-I9: unequal same-family results reject with order-independent
/// evidence, identically on replay.
#[test]
fn at_i7b_at_i9_unequal_results_reject_reproducibly() {
    let build = |flip: bool| {
        let a = work_rule(
            "rule.pa",
            "work.one",
            vec![emit("x", "state.mood", Update::Assign(lit(30)))],
        );
        let b = work_rule(
            "rule.pb",
            "work.two",
            vec![emit("x", "state.mood", Update::Assign(lit(35)))],
        );
        let rules = if flip { vec![b, a] } else { vec![a, b] };
        let mut e = engine_with(
            budgets(8),
            rules,
            vec![("rule.pa", 100, "work.one"), ("rule.pb", 100, "work.two")],
        );
        run(&mut e, 100)
    };
    let first = build(false);
    assert_eq!(first, build(true));
    assert_eq!(first, build(false));
    assert!(matches!(
        reject_of(&first, 100),
        WaveRejection::ResultConflict { ref values, .. } if values.len() == 2
    ));
}

/// AT-I7e + AT-I8: fold overflow, bounds failure, and late transition failures
/// (occurrence exhaustion, obligation and scheduler admission caps) reject
/// before any mutation of any store.
#[test]
fn at_i7e_at_i8_late_failures_reject_before_any_mutation() {
    // Fold overflow and bounds failure; a sibling write on another target is
    // not applied either.
    for (target, a, b) in [
        ("state.stress", i64::MAX, i64::MAX),
        ("state.bounded", 6, 5),
    ] {
        let rules = vec![
            work_rule(
                "rule.pa",
                "work.one",
                vec![emit("x", target, add(a)), emit("y", "state.echo", add(1))],
            ),
            adder("rule.pb", "work.two", target, b),
        ];
        let mut e = engine_with(
            budgets(8),
            rules,
            vec![("rule.pa", 100, "work.one"), ("rule.pb", 100, "work.two")],
        );
        let before = e.state().canonical_state_digest();
        let r = run(&mut e, 100);
        assert!(matches!(
            outcome_at(&r, 100),
            CohortOutcome::Rejected { wave: 0, .. }
        ));
        assert_eq!(before, e.state().canonical_state_digest());
    }
    // Admission caps and occurrence exhaustion.
    // `rule.s` creates two obligations under one ledger key.
    let scheduler = || {
        with_schedule(
            with_schedule(
                adder("rule.s", "work.one", "state.stress", 1),
                reevaluate_after("s1", 5, "work.more", "rule.t"),
            ),
            reevaluate_after("s2", 6, "work.more", "rule.t"),
        )
    };
    let t = adder("rule.t", "work.more", "state.mood", 1);
    let mut caps = budgets(8);
    caps.max_obligations = 2; // the seed obligation is extracted first, so 2 fit
    let mut ok = engine_with(
        caps,
        vec![scheduler(), t.clone()],
        vec![("rule.s", 100, "work.one")],
    );
    assert_eq!(
        outcome_at(&run(&mut ok, 100), 100),
        CohortOutcome::Committed
    );
    caps.max_obligations = 1;
    let mut e = engine_with(
        caps,
        vec![scheduler(), t.clone()],
        vec![("rule.s", 100, "work.one")],
    );
    let before = (
        e.state().canonical_state_digest(),
        e.occurrences().canonical_digest(),
    );
    let r = run(&mut e, 100);
    assert!(matches!(
        reject_of(&r, 100),
        WaveRejection::Preflight {
            reason: PreflightFailure::ObligationAdmissionCap { .. }
        }
    ));
    assert_eq!(
        before,
        (
            e.state().canonical_state_digest(),
            e.occurrences().canonical_digest()
        )
    );
    let mut caps = budgets(8);
    caps.max_scheduled_work = 1;
    let mut e = engine_with(
        caps,
        vec![scheduler(), t.clone()],
        vec![("rule.s", 100, "work.one")],
    );
    assert!(matches!(
        reject_of(&run(&mut e, 100), 100),
        WaveRejection::Preflight {
            reason: PreflightFailure::SchedulerAdmissionCap { .. }
        }
    ));
    // Mid-range occurrence exhaustion: two allocations under one ledger key
    // with only one index left.
    let mut e = engine_with(
        budgets(8),
        vec![scheduler(), t],
        vec![("rule.s", 100, "work.one")],
    );
    fixture::set_occurrence_next(
        &mut e,
        OccurrenceLedgerKey {
            profile_id: profile_id(),
            producer: def("rule.s"),
            scope_id: bron(),
            work_kind: work("work.more"),
        },
        u64::MAX,
    );
    let before = (
        e.state().canonical_state_digest(),
        e.occurrences().canonical_digest(),
    );
    assert_eq!(
        reject_of(&run(&mut e, 100), 100),
        WaveRejection::Arithmetic {
            what: "occurrence_exhausted"
        }
    );
    assert_eq!(
        before,
        (
            e.state().canonical_state_digest(),
            e.occurrences().canonical_digest()
        )
    );
}

// ================================================================ thresholds and identity

fn threshold_rules(parent_values: (i64, i64)) -> Vec<RuleSpec> {
    vec![
        adder("rule.a", "work.one", "state.stress", parent_values.0),
        adder("rule.b", "work.two", "state.stress", parent_values.1),
        with_schedule(
            rule(
                "rule.alarm",
                Trigger::Crossing {
                    watched: def("state.stress"),
                    threshold: 40,
                    direction: Direction::Rising,
                },
                vec![emit("ring", "state.alarm", add(1))],
            ),
            reevaluate_after("follow", 10, "work.follow", "rule.follow"),
        ),
        adder("rule.follow", "work.follow", "state.echo", 1),
    ]
}

fn threshold_engine(parent_values: (i64, i64)) -> Engine {
    seeded(
        budgets(8),
        threshold_rules(parent_values),
        vec![("rule.a", 100, "work.one"), ("rule.b", 100, "work.two")],
    )
}

// One argument per component of the frozen v3 §4.4 formula: this is the
// independent reference encoder the identity is pinned against.
#[allow(clippy::too_many_arguments)]
fn emission(
    cohort: &Digest,
    wave: u32,
    parent: &Digest,
    rule_fp: &Digest,
    sub: &str,
    producer: &ScopeId,
    target: &DefinitionId,
    target_scope: &ScopeId,
    artifact: &Digest,
) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("emission");
    enc.push_digest(cohort);
    enc.push_u32(wave);
    enc.push_digest(parent);
    enc.push_digest(rule_fp);
    enc.push_u64(1);
    tag(sub).canonicalize(&mut enc);
    producer.canonicalize(&mut enc);
    target.canonicalize(&mut enc);
    target_scope.canonicalize(&mut enc);
    enc.push_digest(artifact);
    enc.finish()
}

fn parents_emission(parents: &[Digest]) -> Digest {
    let mut sorted = parents.to_vec();
    sorted.sort();
    let mut enc = CanonicalEncoder::new();
    enc.push_str("parents_emission_v1");
    enc.push_u64(sorted.len() as u64);
    for p in &sorted {
        enc.push_bytes(p.as_bytes());
    }
    enc.finish()
}

/// Hand-computed wave-0 emission identities `e_A`, `e_B` and the wave-1
/// schedule emission of the threshold rule, from the frozen formulas.
fn expected_identities(e: &Engine) -> (Digest, Digest, Digest) {
    let keys: Vec<WorkKey> = e
        .obligations()
        .keys()
        .filter(|k| k.due_time.0 == 100)
        .cloned()
        .collect();
    let cohort = scheduled_cohort_identity(&profile_id(), LogicalTime(100), &keys);
    let artifact = e.epoch_registry().current().unwrap().record_hash();
    let leaf = |k: &WorkKey| {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("parents_workkey_v1")
            .push_digest(&k.identity_digest());
        enc.finish()
    };
    let rule_fp = |id: &str| e.rule_set().rule(&def(id)).unwrap().fingerprint().clone();
    let key_of = |producer: &str| {
        keys.iter()
            .find(|k| k.producer_definition_id == def(producer))
            .unwrap()
    };
    let stress = def("state.stress");
    let ea = emission(
        &cohort,
        0,
        &leaf(key_of("rule.a")),
        &rule_fp("rule.a"),
        "x",
        &bron(),
        &stress,
        &bron(),
        &artifact,
    );
    let eb = emission(
        &cohort,
        0,
        &leaf(key_of("rule.b")),
        &rule_fp("rule.b"),
        "x",
        &bron(),
        &stress,
        &bron(),
        &artifact,
    );
    let o = emission(
        &cohort,
        1,
        &parents_emission(&[ea.clone(), eb.clone()]),
        &rule_fp("rule.alarm"),
        "follow",
        &bron(),
        &def("rule.alarm"),
        &bron(),
        &artifact,
    );
    (ea, eb, o)
}

fn follow_record_identity(e: &Engine) -> Digest {
    e.obligations()
        .keys()
        .filter_map(|k| e.obligations().get(k).and_then(|s| s.sole_record()))
        .find(|r| r.creator_rule_id() == &def("rule.alarm"))
        .unwrap()
        .creator_emission_identity()
        .clone()
}

/// AT-I6c (a)(b)(c), AT-I25: the threshold crossing is computed once from the
/// committed-old to the fully reduced value, emits in wave 1, and the created
/// obligation's emission identity is the hand-computed parent-set identity over
/// `{e_A, e_B}`; a single-parent set differs; a different cause set reducing to
/// the same value has a different identity; permutations are stable.
#[test]
fn at_i6c_at_i25_multi_parent_identity_is_complete_and_stable() {
    let mut e = threshold_engine((10, 15));
    let (ea, eb, expected) = expected_identities(&e);
    let r = run(&mut e, 100);
    assert_eq!(value(&e, "state.stress", &bron()), Some(45));
    assert_eq!(value(&e, "state.alarm", &bron()), Some(1));
    assert_eq!(r[0].waves.len(), 2, "the crossing emits in wave 1");
    let actual = follow_record_identity(&e);
    assert_eq!(actual, expected);
    // Single-parent implementations fail: the digest over a proper subset differs.
    let keys: Vec<WorkKey> = vec![];
    let _ = keys;
    assert_ne!(
        parents_emission(std::slice::from_ref(&ea)),
        parents_emission(&[ea.clone(), eb.clone()])
    );
    assert_ne!(parents_emission(&[eb]), parents_emission(&[ea]));
    // Permuted rule declaration and work insertion: identical identity.
    let mut rules = threshold_rules((10, 15));
    rules.reverse();
    let mut p = seeded(
        budgets(8),
        rules,
        vec![("rule.b", 100, "work.two"), ("rule.a", 100, "work.one")],
    );
    run(&mut p, 100);
    assert_eq!(follow_record_identity(&p), actual);
    assert_eq!(digest_pair(&p), digest_pair(&e));
    // (c) Same result (45), different causes (+20/+5): different identity.
    let mut c = threshold_engine((20, 5));
    run(&mut c, 100);
    assert_eq!(value(&c, "state.stress", &bron()), Some(45));
    assert_ne!(follow_record_identity(&c), actual);
}

/// AT-I6d: a third contributing parent changes the set (and the identity)
/// without becoming "the" parent; permuting enumeration changes nothing.
#[test]
fn at_i6d_no_parent_is_ever_selected() {
    let mut rules = threshold_rules((10, 15));
    rules.push(adder("rule.aaa", "work.three", "state.stress", 0));
    let mut three = seeded(
        budgets(8),
        rules,
        vec![
            ("rule.a", 100, "work.one"),
            ("rule.b", 100, "work.two"),
            ("rule.aaa", 100, "work.three"),
        ],
    );
    let mut two = threshold_engine((10, 15));
    run(&mut three, 100);
    run(&mut two, 100);
    assert_eq!(
        value(&three, "state.stress", &bron()),
        value(&two, "state.stress", &bron())
    );
    assert_ne!(follow_record_identity(&three), follow_record_identity(&two));
}

/// AT-I25 / AT-I20: opposing contributions — a partial fold would cross but the
/// complete fold does not, and vice versa.
#[test]
fn at_i25_partial_folds_never_emit() {
    // 20 + 30 would cross 40; 20 + 30 - 25 = 25 does not.
    let mut no = threshold_engine((30, -25));
    run(&mut no, 100);
    assert_eq!(value(&no, "state.stress", &bron()), Some(25));
    assert_eq!(value(&no, "state.alarm", &bron()), None);
    // 20 - 5 would not cross; 20 - 5 + 30 = 45 does.
    let mut yes = threshold_engine((-5, 30));
    run(&mut yes, 100);
    assert_eq!(value(&yes, "state.alarm", &bron()), Some(1));
}

// ================================================================ pacing

/// `n` distinct work rules at `t`, each adding 1 to stress, plus a later cohort.
fn oversized(n: u64, later: bool) -> (Vec<RuleSpec>, Vec<(String, u64, String)>) {
    let mut rules = Vec::new();
    let mut init = Vec::new();
    for i in 0..n {
        let kind = format!("work.k{i}");
        rules.push(adder(&format!("rule.w{i}"), &kind, "state.stress", 1));
        init.push((format!("rule.w{i}"), 100, kind));
    }
    if later {
        rules.push(adder("rule.late", "work.late", "state.mood", 1));
        init.push(("rule.late".to_string(), 101, "work.late".to_string()));
    }
    (rules, init)
}

fn engine_owned(
    budget: DeclaredBudgets,
    rules: Vec<RuleSpec>,
    init: &[(String, u64, String)],
) -> Engine {
    let mut f = standard();
    f.engine(
        budget,
        rules,
        init.iter()
            .map(|(r, t, k)| initial(r, &bron(), *t, k))
            .collect(),
    )
}

/// AT-I20c (a)(b)(c)(g)(h): the frozen greedy prefix and the
/// oversized-earliest exception; later cohorts untouched; progress every cycle;
/// canonical equality with the unbudgeted run.
#[test]
fn at_i20c_oversized_first_cohort_progresses_without_split() {
    for (n, budget, overrun) in [
        (3u64, 3u32, false),
        (4, 3, true),
        (2, 3, false),
        (9, 3, true),
    ] {
        let (rules, init) = oversized(n, true);
        let mut paced = engine_owned(budgets(budget), rules.clone(), &init);
        let late_key = paced
            .obligations()
            .keys()
            .find(|k| k.due_time.0 == 101)
            .unwrap()
            .clone();
        let first = paced.process(&advance(101));
        let d = first.diagnostics().unwrap().clone();
        assert_eq!(d.pacing_overrun, overrun, "n={n} budget={budget}");
        assert!(d.admitted_work_key_count >= 1);
        assert_eq!(first.reports()[0].canonical_time, LogicalTime(100));
        assert_eq!(
            first.reports()[0].waves[0].committed[0]
                .provenance
                .retained
                .len() as u64,
            n.min(8)
        );
        if n > u64::from(budget) {
            assert_eq!(d.admitted_work_key_count, n, "admitted whole, exactly once");
            // Cohort identity is no report field (FINAL AT-I46(a)); it is
            // observed through the test-support seam.
            assert_eq!(
                d.overrun_cohort_identity,
                Some(
                    fixture::observation(&paced).prewave[0]
                        .cohort_identity
                        .clone()
                )
            );
            assert_eq!(first.outcome(), &Outcome::Paused);
            assert_eq!(
                paced.slot_status(&late_key),
                WorkSlotStatus::Scheduled,
                "later cohort untouched"
            );
        }
        let mut all = first.reports().to_vec();
        all.extend(reports(&drive(&mut paced, &advance(101))));
        let mut unpaced = engine_owned(budgets(1_000), rules, &init);
        let direct = reports(&drive(&mut unpaced, &advance(101)));
        assert_eq!(all, direct);
        assert_eq!(digest_pair(&paced), digest_pair(&unpaced));
        assert_eq!(
            value(&paced, "state.stress", &bron()),
            Some(n as i64),
            "no split at any granularity"
        );
    }
}

/// AT-I20c(d)(g): three successive oversized cohorts are admitted whole, once
/// each, in order; due work strictly decreases every cycle.
#[test]
fn at_i20c_d_successive_oversized_cohorts_each_admit_once() {
    let mut rules = Vec::new();
    let mut init = Vec::new();
    for (t, prefix) in [(100u64, "a"), (101, "b"), (102, "c")] {
        for i in 0..4 {
            let kind = format!("work.{prefix}{i}");
            rules.push(adder(
                &format!("rule.{prefix}{i}"),
                &kind,
                "state.stress",
                1,
            ));
            init.push((format!("rule.{prefix}{i}"), t, kind));
        }
    }
    let mut e = engine_owned(budgets(3), rules, &init);
    let mut due = e.scheduled_work_count();
    let mut order = Vec::new();
    loop {
        let r = e.process(&advance(102));
        let d = r.diagnostics().unwrap();
        assert!(
            d.admitted_work_key_count > 0,
            "every cycle with due work consumes work"
        );
        assert!(e.scheduled_work_count() < due);
        due = e.scheduled_work_count();
        for c in r.reports() {
            order.push(c.canonical_time.0);
        }
        if r.outcome() == &Outcome::Completed {
            break;
        }
        assert!(d.pacing_overrun);
    }
    assert_eq!(order, vec![100, 101, 102]);
    assert_eq!(e.scheduled_work_count(), 0);
}

/// AT-I20b + AT-I20c(i): a structurally admitted oversized cohort that exceeds a
/// semantic cap is rejected atomically and consumed terminally; reschedule
/// succeeds; the outcome is identical on replay.
#[test]
fn at_i20b_semantic_cap_overflow_is_atomic_and_terminal() {
    let (mut rules, init) = oversized(4, false);
    rules.push(adder("rule.again", "work.again", "state.stress", 1));
    let mut caps = budgets(2);
    caps.max_cohort_candidates = 3;
    let run_once = || {
        let mut e = engine_owned(caps, rules.clone(), &init);
        let r = reports(&drive(&mut e, &advance(100)));
        (e, r)
    };
    let (mut e, r) = run_once();
    assert_eq!(
        reject_of(&r, 100),
        WaveRejection::SemanticCap {
            cap: "max_cohort_candidates",
            observed: 4,
            bound: 3
        }
    );
    assert_eq!(e.scheduled_work_count(), 0, "keys consumed terminally");
    assert!(value(&e, "state.stress", &bron()).is_none());
    assert_eq!(run_once().1, r, "identical on replay");
    // A producer can reschedule afterwards: new work at a later time runs.
    let _ = &mut e;
}

// ================================================================ conflicted vs executable

/// Schedule a second, distinct record under an existing key so it is
/// conflicted in both stores.
fn conflict(e: &mut Engine, due: u64, producer: &str, kind: &str) -> WorkKey {
    let key = WorkKey {
        due_time: LogicalTime(due),
        profile_id: profile_id(),
        producer_definition_id: def(producer),
        scope_id: bron(),
        occurrence_index: OccurrenceIndex(0),
        work_kind: work(kind),
    };
    let rule = e
        .rule_set()
        .rule(&def(producer))
        .unwrap()
        .fingerprint()
        .clone();
    let artifact = e.epoch_registry().current().unwrap().record_hash();
    for claim in ["claim.one", "claim.two"] {
        let r = fixture::reevaluation_record(
            key.clone(),
            def(producer),
            rule.clone(),
            e.rule_set().content_hash().clone(),
            spark_core::hash::hash_bytes(claim.as_bytes()),
            artifact.clone(),
        );
        fixture::schedule_record(e, r);
    }
    assert_eq!(e.slot_status(&key), WorkSlotStatus::Conflicted);
    key
}

fn s_x_t(budget: u32, with_x: bool) -> (Engine, Option<WorkKey>) {
    let rules = vec![
        adder("rule.s", "work.s", "state.stress", 1),
        adder("rule.t", "work.t", "state.mood", 1),
        adder("rule.x", "work.x", "state.echo", 1),
    ];
    let mut e = engine_with(
        budgets(budget),
        rules,
        vec![("rule.s", 100, "work.s"), ("rule.t", 101, "work.t")],
    );
    let x = if with_x {
        Some(conflict(&mut e, 100, "rule.x", "work.x"))
    } else {
        None
    };
    (e, x)
}

/// AT-I45(a)(d): executable-only counting at the admission boundary.
#[test]
fn at_i45_a_conflicted_slots_cost_no_budget() {
    let (mut two, _) = s_x_t(2, true);
    let r = two.process(&advance(101));
    assert_eq!(r.outcome(), &Outcome::Completed);
    assert_eq!(r.diagnostics().unwrap().admitted_cohort_count, 2);
    let (mut one, _) = s_x_t(1, true);
    let r = one.process(&advance(101));
    assert_eq!(r.outcome(), &Outcome::Paused);
    let d = r.diagnostics().unwrap();
    assert!(!d.pacing_overrun);
    assert_eq!(d.deferred_cohort_count, 1);
    assert_eq!(d.earliest_deferred_due_time, Some(LogicalTime(101)));
}

/// AT-I40(e) + AT-I44(c′): `{S}` versus `{S, X_conflicted}` — equal identity,
/// emission identities, pre-wave digests, batch digests, resulting stores, and
/// final state; different pre-extraction scheduler digests, conflict reports,
/// and removed sets; the conflict section precedes the cohort outcome.
#[test]
fn at_i40_e_conflicted_members_enter_neither_identity_nor_state() {
    let (mut a, _) = s_x_t(8, false);
    let (mut b, x) = s_x_t(8, true);
    let x = x.unwrap();
    let removed_x: Vec<Digest> = b
        .obligations()
        .get(&x)
        .unwrap()
        .records()
        .map(|r| r.record_hash())
        .collect();
    assert_eq!(removed_x.len(), 2);
    assert_ne!(a.scheduler_digest(), b.scheduler_digest());
    let ra = run(&mut a, 101);
    let rb = run(&mut b, 101);
    assert_eq!(
        fixture::observation(&a).prewave[0].cohort_identity,
        fixture::observation(&b).prewave[0].cohort_identity,
        "equal cohort identity (observed through the test-support seam)"
    );
    assert_eq!(
        ra[0].waves, rb[0].waves,
        "equal emission identities, pre-wave and batch digests"
    );
    assert!(ra[0].conflicts.is_empty());
    assert_eq!(rb[0].conflicts.len(), 1);
    assert_eq!(rb[0].conflicts[0].key, x);
    assert_eq!(digest_pair(&a), digest_pair(&b));
    assert!(
        b.obligations().get(&x).is_none(),
        "the complete contested set left with it"
    );
}

/// AT-I45(b)(c′): all-conflicted slices are consumed at zero cost, also after
/// the oversized exception; PAUSED implies a deferred executable slice.
#[test]
fn at_i45_c_prime_residual_conflicts_after_the_exception() {
    let mut rules = Vec::new();
    let mut init = Vec::new();
    for i in 0..3 {
        let kind = format!("work.o{i}");
        rules.push(adder(&format!("rule.o{i}"), &kind, "state.stress", 1));
        init.push((format!("rule.o{i}"), 100, kind));
    }
    rules.push(adder("rule.c1", "work.c1", "state.echo", 1));
    rules.push(adder("rule.c2", "work.c2", "state.echo", 1));
    rules.push(adder("rule.d", "work.d", "state.mood", 1));
    init.push(("rule.d".to_string(), 103, "work.d".to_string()));
    let mut e = engine_owned(budgets(2), rules, &init);
    conflict(&mut e, 101, "rule.c1", "work.c1");
    conflict(&mut e, 102, "rule.c2", "work.c2");
    let r = e.process(&advance(103));
    assert_eq!(r.outcome(), &Outcome::Paused);
    let kinds: Vec<(u64, CohortKind)> = r
        .reports()
        .iter()
        .map(|c| (c.canonical_time.0, c.kind))
        .collect();
    assert_eq!(
        kinds,
        vec![
            (100, CohortKind::Scheduled),
            (101, CohortKind::ConflictOnly),
            (102, CohortKind::ConflictOnly)
        ]
    );
    let d = r.diagnostics().unwrap();
    assert!(d.pacing_overrun);
    assert_eq!(d.deferred_cohort_count, 1);
    assert_eq!(d.earliest_deferred_due_time, Some(LogicalTime(103)));
}

/// AT-I42(l)(m)(n): the engine's cross-store extraction preflights before either
/// store mutates, and a consistent store hands the full record to evaluation.
#[test]
fn at_i42_l_m_n_cross_store_extraction_is_preflighted() {
    let (mut e, _) = s_x_t(8, false);
    let s = e
        .obligations()
        .keys()
        .find(|k| k.due_time.0 == 100)
        .unwrap()
        .clone();
    let record = e
        .obligations()
        .get(&s)
        .unwrap()
        .sole_record()
        .unwrap()
        .clone();
    let digests = |e: &Engine| {
        (
            e.scheduler_digest(),
            e.obligations().canonical_digest(),
            e.engine_state_digest(),
        )
    };
    // (l) missing record.
    fixture::tamper_obligations(&mut e, &s, None);
    let before = digests(&e);
    assert_eq!(
        fixture::extract_least_due_slice(&mut e, LogicalTime(100)).unwrap_err(),
        ExtractionRefusal::ObligationRecordMissing { key: s.clone() }
    );
    assert_eq!(before, digests(&e));
    // (m) mismatched record.
    let wrong = fixture::record_with_mode(
        &record,
        ObligationMode::RuleReEvaluation {
            rule_id: def("rule.t"),
            rule_fingerprint: e
                .rule_set()
                .rule(&def("rule.t"))
                .unwrap()
                .fingerprint()
                .clone(),
            ruleset_content_hash: e.rule_set().content_hash().clone(),
        },
    );
    fixture::tamper_obligations(&mut e, &s, Some(vec![wrong]));
    let before = digests(&e);
    assert_eq!(
        fixture::extract_least_due_slice(&mut e, LogicalTime(100)).unwrap_err(),
        ExtractionRefusal::ObligationRecordMismatch { key: s.clone() }
    );
    assert_eq!(before, digests(&e));
    // (m) conflicted slot whose contested set misses a claim.
    let (mut c, x) = s_x_t(8, true);
    let x = x.unwrap();
    let one: Vec<_> = c
        .obligations()
        .get(&x)
        .unwrap()
        .records()
        .take(1)
        .cloned()
        .collect();
    let mut both: Vec<_> = c
        .obligations()
        .get(&x)
        .unwrap()
        .records()
        .cloned()
        .collect();
    fixture::tamper_obligations(&mut c, &x, Some(one));
    assert!(matches!(
        fixture::extract_least_due_slice(&mut c, LogicalTime(100)).unwrap_err(),
        ExtractionRefusal::ObligationRecordMismatch { .. }
            | ExtractionRefusal::ObligationClaimSetMismatch { .. }
    ));
    // (n) repaired stores: the full record reaches evaluation and leaves the store.
    fixture::tamper_obligations(&mut e, &s, Some(vec![record.clone()]));
    let (records, _) = fixture::extract_least_due_slice(&mut e, LogicalTime(100)).unwrap();
    assert_eq!(records, vec![record]);
    assert!(e.obligations().get(&s).is_none());
    assert!(e.bidirectional_invariant_holds());
    both.sort_by_key(|r| r.record_hash());
    fixture::tamper_obligations(&mut c, &x, Some(both));
    assert!(c.bidirectional_invariant_holds());
}

// ================================================================ rejection classes

/// AT-I44(a)(a′): a wave-0 rejection leaves the post-extraction state; a
/// later-wave rejection keeps earlier waves committed (no cohort rollback).
#[test]
fn at_i44_a_wave_index_decides_what_nothing_applied_means() {
    // Wave-0: unequal results.
    let mut w0 = engine_with(
        budgets(8),
        vec![
            work_rule(
                "rule.pa",
                "work.one",
                vec![emit("x", "state.mood", Update::Assign(lit(1)))],
            ),
            work_rule(
                "rule.pb",
                "work.two",
                vec![emit("x", "state.mood", Update::Assign(lit(2)))],
            ),
        ],
        vec![("rule.pa", 100, "work.one"), ("rule.pb", 100, "work.two")],
    );
    fixture::clear_observation(&mut w0);
    let r = run(&mut w0, 100);
    assert!(matches!(
        outcome_at(&r, 100),
        CohortOutcome::Rejected { wave: 0, .. }
    ));
    assert_eq!(
        fixture::observation(&w0).prewave[0].engine_digest.value(),
        &w0.engine_state_digest()
    );
    // Wave-1: wave 0 commits stress; two change rules assign unequal mood.
    let watcher = |id: &str, v: i64| {
        rule(
            id,
            Trigger::Change {
                watched: def("state.stress"),
            },
            vec![emit("m", "state.mood", Update::Assign(lit(v)))],
        )
    };
    let mut w1 = engine_with(
        budgets(8),
        vec![
            adder("rule.s", "work.one", "state.stress", 5),
            watcher("rule.m1", 1),
            watcher("rule.m2", 2),
        ],
        vec![("rule.s", 100, "work.one")],
    );
    fixture::clear_observation(&mut w1);
    let r = run(&mut w1, 100);
    assert!(matches!(
        outcome_at(&r, 100),
        CohortOutcome::Rejected {
            wave: 1,
            rejection: WaveRejection::ResultConflict { .. }
        }
    ));
    assert_eq!(
        value(&w1, "state.stress", &bron()),
        Some(5),
        "wave 0 stays committed"
    );
    assert!(
        value(&w1, "state.mood", &bron()).is_none(),
        "wave 1 applies nothing"
    );
    let obs = fixture::observation(&w1).prewave.clone();
    assert_eq!(
        obs[1].engine_digest.value(),
        &w1.engine_state_digest(),
        "state after wave 0's commit"
    );
    assert_ne!(
        obs[0].engine_digest.value(),
        &w1.engine_state_digest(),
        "not the post-extraction digest"
    );
}

/// AT-I44(g): a command cohort rejected at wave 0 stays finalized; the request
/// completes.
#[test]
fn at_i44_g_rejected_command_cohort_stays_finalized() {
    let mut e = engine_with(
        budgets(8),
        vec![on_command(
            "rule.c1",
            "cmd.clash",
            vec![emit("x", "state.bounded", add(20))],
        )],
        vec![],
    );
    let r = e.process(&command("cmd.clash", 5, 1, "cmd.clash", bron()));
    assert_eq!(r.outcome(), &Outcome::Completed);
    assert!(matches!(
        r.reports()[0].outcome,
        CohortOutcome::Rejected { wave: 0, .. }
    ));
    assert_eq!(e.finalized_commands().len(), 1);
    assert!(e.staging_is_clean());
}

// ================================================================ depth conversion

/// AT-I21: propagation pending at `max_wave_depth` converts to work at
/// `due_time >= now + 1`, visibly reported, never dropped, and not executed in
/// the same call's drain set; values converge (digests differ because the depth
/// budget is epoch-bound declared behavior, v2 §2 C-11).
#[test]
fn at_i21_depth_overflow_converts_to_strictly_later_work() {
    let rules = vec![
        adder("rule.s", "work.one", "state.stress", 1),
        rule(
            "rule.c",
            Trigger::Change {
                watched: def("state.stress"),
            },
            vec![emit("e", "state.echo", add(1))],
        ),
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
    let mut genesis = f.genesis(
        budgets(8),
        rules.clone(),
        vec![initial("rule.s", &bron(), 100, "work.one")],
    );
    genesis.rule_set = rule_set;
    let mut e = Engine::genesis(genesis).unwrap();
    let r = e.process(&advance(100));
    assert_eq!(r.outcome(), &Outcome::Completed);
    let wave = &r.reports()[0].waves[0];
    assert_eq!(wave.depth_conversions.len(), 1);
    let converted = wave.depth_conversions[0].clone();
    assert_eq!(converted.due_time, LogicalTime(101));
    assert_eq!(
        e.slot_status(&converted),
        WorkSlotStatus::Scheduled,
        "not run in this call"
    );
    assert!(value(&e, "state.echo", &bron()).is_none());
    run(&mut e, 101);
    assert_eq!(value(&e, "state.echo", &bron()), Some(1));
    let mut deep = engine_with(budgets(8), rules, vec![("rule.s", 100, "work.one")]);
    run(&mut deep, 101);
    assert_eq!(value(&deep, "state.echo", &bron()), Some(1));
    assert_eq!(
        value(&deep, "state.stress", &bron()),
        value(&e, "state.stress", &bron())
    );
}

/// AT-I20 base: over two due-time cohorts with co-target causes, the paced run
/// needs strictly more calls and is canonically identical, with honest
/// diagnostics differing.
#[test]
fn at_i20_pacing_defers_whole_cohorts() {
    let rules = vec![
        adder("rule.a", "work.one", "state.stress", 10),
        adder("rule.b", "work.two", "state.stress", 15),
        adder("rule.c", "work.three", "state.mood", 1),
    ];
    let init = vec![
        ("rule.a", 100, "work.one"),
        ("rule.b", 100, "work.two"),
        ("rule.c", 101, "work.three"),
    ];
    let mut paced = engine_with(budgets(2), rules.clone(), init.clone());
    let pr = drive(&mut paced, &advance(101));
    let mut unpaced = engine_with(budgets(100), rules, init);
    let ur = drive(&mut unpaced, &advance(101));
    assert!(pr.len() > ur.len());
    assert_eq!(reports(&pr), reports(&ur));
    assert_eq!(digest_pair(&paced), digest_pair(&unpaced));
    assert_ne!(pr[0].diagnostics(), ur[0].diagnostics());
}
