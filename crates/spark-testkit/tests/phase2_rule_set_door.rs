//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 checkpoint 2: the rule-set artifact and its activation door.
//!
//! AT-I3 (declaration-order invariance), AT-I10 (host-owned targets rejected
//! atomically), AT-I17 (sub-ID uniqueness), AT-I18 (zero-delay cycles), AT-I19
//! (fan-out and depth bounds, boundary cases), AT-I20c(e) (zero pacing budget
//! refused atomically), AT-I22/AT-I24 (one decay and one aggregation reducer
//! per target), AT-I37 (activation-time pair classification where
//! co-targeting is statically provable), AT-I35 (Phase-1 lineage digest
//! unchanged by rule-set activation).

use spark_core::value::FixedPoint;
use spark_engine::rules::{CmpOp, Condition, RuleSetError, Trigger, Update};
use spark_testkit::phase2::*;

fn scale(f: i64) -> Update {
    Update::Scale(FixedPoint::from_raw(f))
}

fn change(
    id: &str,
    watched: &str,
    emits: Vec<spark_engine::rules::EmitOp>,
) -> spark_engine::rules::RuleSpec {
    rule(
        id,
        Trigger::Change {
            watched: def(watched),
        },
        emits,
    )
}

/// AT-I3: permuted declaration activates to the same content hash, over a
/// corpus with equal and unequal additive contributions, same-family equal
/// results, and a singleton transform.
#[test]
fn at_i3_rule_declaration_order_cannot_change_the_content_hash() {
    let rules = vec![
        on_command(
            "rule.add.ten",
            "cmd.hit",
            vec![emit("a", "state.stress", Update::Add(lit(10)))],
        ),
        on_command(
            "rule.add.fifteen",
            "cmd.hit",
            vec![emit("a", "state.stress", Update::Add(lit(15)))],
        ),
        on_command(
            "rule.add.ten.again",
            "cmd.hit",
            vec![emit("a", "state.stress", Update::Add(lit(10)))],
        ),
        on_command(
            "rule.result.one",
            "cmd.set",
            vec![emit("r", "state.mood", Update::Assign(lit(30)))],
        ),
        on_command(
            "rule.result.two",
            "cmd.set",
            vec![emit("r", "state.mood", Update::Assign(lit(30)))],
        ),
        on_command(
            "rule.transform",
            "cmd.scale",
            vec![emit("s", "state.energy", scale(1_500_000))],
        ),
    ];
    let mut hashes = std::collections::BTreeSet::new();
    let n = rules.len();
    for shift in 0..n {
        let mut f = standard();
        let mut permuted = rules.clone();
        permuted.rotate_left(shift);
        if shift % 2 == 1 {
            permuted.reverse();
        }
        hashes.insert(
            f.rule_set(budgets(4), permuted)
                .unwrap()
                .content_hash()
                .clone(),
        );
    }
    assert_eq!(hashes.len(), 1);
}

/// AT-I10: a rule targeting a host-owned definition is rejected atomically; the
/// rule-set lineage is unchanged.
#[test]
fn at_i10_host_owned_target_is_rejected_atomically() {
    let mut f = standard();
    let before = f.registry.rule_set_lineage_digest();
    let errors = f
        .rule_set(
            budgets(4),
            vec![on_command(
                "rule.bad",
                "cmd.x",
                vec![emit("a", "state.food", Update::Add(lit(1)))],
            )],
        )
        .unwrap_err();
    assert!(errors
        .iter()
        .any(|e| matches!(e, RuleSetError::HostOwnedTarget { .. })));
    assert_eq!(before, f.registry.rule_set_lineage_digest());
    assert_eq!(f.registry.rule_set_lineage_len(), 0);
}

/// AT-I17 (sub-ID discipline): duplicate sub-IDs within one rule are rejected.
#[test]
fn at_i17_duplicate_sub_ids_are_rejected() {
    let mut f = standard();
    let errors = f
        .rule_set(
            budgets(4),
            vec![on_command(
                "rule.dup",
                "cmd.x",
                vec![
                    emit("same", "state.stress", Update::Add(lit(1))),
                    emit("same", "state.mood", Update::Add(lit(1))),
                ],
            )],
        )
        .unwrap_err();
    assert!(errors
        .iter()
        .any(|e| matches!(e, RuleSetError::DuplicateSubId { .. })));
}

/// AT-I18: direct, mutual, and long zero-delay cycles are rejected with the
/// cycle named; one delay edge makes the same graph admissible.
#[test]
fn at_i18_zero_delay_cycles_are_rejected_and_delay_edges_admit() {
    let direct = vec![change(
        "rule.a",
        "state.stress",
        vec![emit("a", "state.stress", Update::Add(lit(1)))],
    )];
    let mutual = vec![
        change(
            "rule.a",
            "state.stress",
            vec![emit("a", "state.mood", Update::Add(lit(1)))],
        ),
        change(
            "rule.b",
            "state.mood",
            vec![emit("b", "state.stress", Update::Add(lit(1)))],
        ),
    ];
    let long = vec![
        change(
            "rule.a",
            "state.stress",
            vec![emit("a", "state.mood", Update::Add(lit(1)))],
        ),
        change(
            "rule.b",
            "state.mood",
            vec![emit("b", "state.energy", Update::Add(lit(1)))],
        ),
        change(
            "rule.c",
            "state.energy",
            vec![emit("c", "state.stress", Update::Add(lit(1)))],
        ),
    ];
    for (graph, members) in [
        (direct, vec!["rule.a"]),
        (mutual, vec!["rule.a", "rule.b"]),
        (long, vec!["rule.a", "rule.b", "rule.c"]),
    ] {
        let mut f = standard();
        let errors = f.rule_set(budgets(4), graph).unwrap_err();
        let named: Vec<_> = members.iter().map(|m| def(m)).collect();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, RuleSetError::ZeroDelayCycle { rules } if rules == &named)),
            "{errors:?}"
        );
        assert_eq!(f.registry.rule_set_lineage_len(), 0);
    }
    // The mutual loop closed through a declared delay edge instead activates.
    let mut f = standard();
    let delayed = vec![
        change(
            "rule.a",
            "state.stress",
            vec![emit("a", "state.mood", Update::Add(lit(1)))],
        ),
        with_schedule(
            work_rule(
                "rule.b",
                "work.b",
                vec![emit("b", "state.stress", Update::Add(lit(1)))],
            ),
            reevaluate_after("later", 1, "work.b", "rule.b"),
        ),
        with_schedule(
            change("rule.c", "state.mood", vec![]),
            reevaluate_after("hop", 1, "work.b", "rule.b"),
        ),
    ];
    assert!(f.rule_set(budgets(4), delayed).is_ok());
}

/// AT-I19: static fan-out and depth bounds; the boundary case activates.
#[test]
fn at_i19_fan_out_and_depth_bounds_are_static_with_exact_boundaries() {
    let chain = |len: usize| {
        let defs = [
            "state.stress",
            "state.mood",
            "state.energy",
            "state.alarm",
            "state.echo",
            "state.echo2",
        ];
        let mut rules = vec![on_command(
            "rule.head",
            "cmd.go",
            vec![emit("h", defs[0], Update::Add(lit(1)))],
        )];
        for i in 0..len {
            rules.push(change(
                &format!("rule.hop{i}"),
                defs[i],
                vec![emit("h", defs[i + 1], Update::Add(lit(1)))],
            ));
        }
        rules
    };
    let mut at_bound = budgets(4);
    at_bound.max_wave_depth = 3;
    let mut f = standard();
    assert!(
        f.rule_set(at_bound, chain(3)).is_ok(),
        "depth exactly at the bound activates"
    );
    let mut f = standard();
    let errors = f.rule_set(at_bound, chain(4)).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| matches!(e, RuleSetError::WaveDepthExceeded { depth: 4, bound: 3 })));

    let wide = on_command(
        "rule.wide",
        "cmd.x",
        vec![
            emit("a", "state.stress", Update::Add(lit(1))),
            emit("b", "state.mood", Update::Add(lit(1))),
            emit("c", "state.energy", Update::Add(lit(1))),
        ],
    );
    let mut fan = budgets(4);
    fan.max_fan_out = 3;
    let mut f = standard();
    assert!(
        f.rule_set(fan, vec![wide.clone()]).is_ok(),
        "fan-out exactly at the bound activates"
    );
    fan.max_fan_out = 2;
    let mut f = standard();
    assert!(f
        .rule_set(fan, vec![wide])
        .unwrap_err()
        .iter()
        .any(|e| matches!(
            e,
            RuleSetError::FanOutExceeded {
                fan_out: 3,
                bound: 2,
                ..
            }
        )));
}

/// AT-I20c(e): `max_due_per_cycle == 0` is refused at the door, atomically: the
/// rule-set lineage digest equals the pre-activation value.
#[test]
fn at_i20c_e_zero_pacing_budget_is_refused_atomically() {
    let mut f = standard();
    let before = (
        f.registry.rule_set_lineage_digest(),
        f.registry.lineage_digest(),
    );
    let errors = f.rule_set(budgets(0), vec![]).unwrap_err();
    assert!(errors.contains(&RuleSetError::ZeroPacingBudget));
    assert_eq!(
        before,
        (
            f.registry.rule_set_lineage_digest(),
            f.registry.lineage_digest()
        )
    );
}

/// AT-I22 / AT-I24 / AT-I37: one decay and one aggregation reducer per target.
#[test]
fn at_i22_at_i24_one_reducer_per_target() {
    let decay = |id: &str| {
        work_rule(
            id,
            "work.decay",
            vec![emit(
                "d",
                "state.stress",
                spark_testkit::phase2::decay(1, 10),
            )],
        )
    };
    let mut f = standard();
    // The baseline is declared, so the defect is the second reducer (the two
    // same-trigger transforms are also a statically provable mixture), never
    // a missing baseline.
    let errors = f
        .rule_set_with(
            budgets(4),
            vec![decay("rule.decay.a"), decay("rule.decay.b")],
            vec![baseline("state.stress", 0)],
        )
        .unwrap_err();
    assert!(errors.contains(&RuleSetError::SecondDecayReducer {
        target: def("state.stress")
    }));
    assert!(!errors
        .iter()
        .any(|e| matches!(e, RuleSetError::DecayWithoutBaseline { .. })));
    let agg = |id: &str| {
        on_command(
            id,
            "cmd.agg",
            vec![emit_at(
                "g",
                "state.total",
                region("north"),
                Update::Aggregate {
                    source: def("state.stress"),
                    weight: FixedPoint::from_integer(1).unwrap(),
                },
            )],
        )
    };
    let mut f = standard();
    assert!(f
        .rule_set(budgets(4), vec![agg("rule.agg.a"), agg("rule.agg.b")])
        .unwrap_err()
        .iter()
        .any(|e| matches!(e, RuleSetError::SecondAggregationReducer { .. })));
    let mut f = standard();
    assert!(f.rule_set(budgets(4), vec![agg("rule.agg.a")]).is_ok());
}

/// AT-I37 (activation half): pairs of rules sharing a trigger and a target are
/// classified at the door wherever co-targeting is statically provable.
#[test]
fn at_i37_statically_provable_pairs_are_classified_at_activation() {
    let updates: Vec<(&str, Update)> = vec![
        ("add", Update::Add(lit(1))),
        ("subtract", Update::Subtract(lit(1))),
        ("scale", scale(1_500_000)),
        ("clamp", Update::Clamp { min: 0, max: 5 }),
        ("result", Update::Assign(lit(3))),
    ];
    let accepted = |a: &str, b: &str| {
        matches!(
            (a, b),
            ("add", "add")
                | ("add", "subtract")
                | ("subtract", "add")
                | ("subtract", "subtract")
                | ("result", "result")
        )
    };
    for (na, ua) in &updates {
        for (nb, ub) in &updates {
            for order in [false, true] {
                let ra = on_command(
                    "rule.first",
                    "cmd.pair",
                    vec![emit("x", "state.stress", ua.clone())],
                );
                let rb = on_command(
                    "rule.second",
                    "cmd.pair",
                    vec![emit("x", "state.stress", ub.clone())],
                );
                let rules = if order { vec![rb, ra] } else { vec![ra, rb] };
                let mut f = standard();
                let result = f.rule_set(budgets(4), rules);
                assert_eq!(
                    result.is_ok(),
                    accepted(na, nb),
                    "{na}/{nb}: {:?}",
                    result.err()
                );
                if let Err(errors) = result {
                    assert!(errors
                        .iter()
                        .any(|e| matches!(e, RuleSetError::StaticFamilyMixture { .. })));
                }
            }
        }
    }
}

/// Conditions and probability gates validate their inputs and rates.
#[test]
fn conditions_and_gates_are_validated() {
    let mut f = standard();
    let r = with_condition(
        on_command(
            "rule.gate",
            "cmd.x",
            vec![emit("a", "state.stress", Update::Add(lit(1)))],
        ),
        Condition::Chance {
            sub_id: tag("gate"),
            rate: FixedPoint::from_raw(2_000_000),
        },
    );
    assert!(f
        .rule_set(budgets(4), vec![r])
        .unwrap_err()
        .iter()
        .any(|e| matches!(e, RuleSetError::ChanceRateOutOfRange { .. })));
    let mut f = standard();
    let r = with_condition(
        on_command(
            "rule.cmp",
            "cmd.x",
            vec![emit("a", "state.stress", Update::Add(lit(1)))],
        ),
        compare(
            spark_engine::rules::Expr::Input(cell("state.never.declared")),
            CmpOp::Gt,
            lit(0),
        ),
    );
    assert!(f
        .rule_set(budgets(4), vec![r])
        .unwrap_err()
        .iter()
        .any(|e| matches!(e, RuleSetError::UnknownDefinition { .. })));
}

/// AT-I35: activating a rule set leaves the Phase-1 definition lineage digest
/// byte-identical, and repeating an identical rule-set activation does not
/// grow the rule-set lineage.
#[test]
fn at_i35_rule_set_activation_leaves_the_phase1_lineage_digest_unchanged() {
    let mut f = standard();
    let lineage = f.registry.lineage_digest();
    let rules = vec![on_command(
        "rule.a",
        "cmd.x",
        vec![emit("a", "state.stress", Update::Add(lit(1)))],
    )];
    let first = f.rule_set(budgets(4), rules.clone()).unwrap();
    let second = f.rule_set(budgets(4), rules).unwrap();
    assert_eq!(first, second);
    assert_eq!(f.registry.lineage_digest(), lineage);
    assert_eq!(f.registry.rule_set_lineage_len(), 1);
}
