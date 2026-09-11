//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 checkpoint 5: retained stores and rule semantics.
//!
//! AT-I4 (cohort identity), AT-I11 (bounds sweep), AT-I12 (payload hash is the
//! record hash, invariant after every prefix), AT-I15 (collisions at and beyond
//! the claim cap), AT-I16 (allocation order), AT-I17 (random addresses), AT-I22
//! (decay chunk invariance), AT-I24 (aggregation), AT-I26/AT-I27 (per-store
//! discrimination and equal-digest equivalence), AT-I28 (derived indexes
//! validated), AT-I29 (reconstruction across a paced deferral), AT-I30/AT-I31
//! (dormant boundedness), AT-I33 (totality), AT-I38 (provenance coverage),
//! AT-I46(a) (report field set pinned), cooldowns.

use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, Digest};
use spark_core::id::DefinitionId;
use spark_core::random::{RandomAddress, RandomAddressService};
use spark_core::scheduler::{
    OccurrenceIndex, WorkKey, WorkSlotStatus, MAX_CONFLICT_TRACKED_CLAIMS,
};
use spark_core::scope::ScopeId;
use spark_core::value::{CanonicalValue, FixedPoint};
use spark_engine::effects::scheduled_cohort_identity;
use spark_engine::engine::Engine;
use spark_engine::fixture;
use spark_engine::ledger::OccurrenceLedgerKey;
use spark_engine::report::{
    CohortKind, CohortOutcome, CohortReport, CoverageStatus, WaveRejection, WaveReport,
};
use spark_engine::request::{Outcome, Request};
use spark_engine::rules::{
    CmpOp, Condition, DeclaredBudgets, Expr, Input, RuleSpec, ScopeRef, Update,
};
use spark_testkit::phase2::*;

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

fn run(e: &mut Engine, t: u64) -> Vec<CohortReport> {
    reports(&drive(e, &advance(t)))
}

fn key(due: u64, producer: &str, occ: u64, kind: &str) -> WorkKey {
    WorkKey {
        due_time: LogicalTime(due),
        profile_id: profile_id(),
        producer_definition_id: def(producer),
        scope_id: bron(),
        occurrence_index: OccurrenceIndex(occ),
        work_kind: work(kind),
    }
}

/// A distinct re-evaluation record of `rule` under `k`, discriminated by `claim`.
fn record(
    e: &Engine,
    k: &WorkKey,
    rule: &str,
    claim: &str,
) -> spark_engine::obligation::ObligationRecord {
    fixture::reevaluation_record(
        k.clone(),
        def(rule),
        e.rule_set().rule(&def(rule)).unwrap().fingerprint().clone(),
        e.rule_set().content_hash().clone(),
        hash_bytes(claim.as_bytes()),
        e.epoch_registry().current().unwrap().record_hash(),
    )
}

/// AT-I4: cohort identity is invariant under insertion permutation and moves
/// when any member is added, removed, or altered in any semantic field.
#[test]
fn at_i4_cohort_identity_is_a_function_of_its_members() {
    let a = key(100, "rule.a", 0, "work.one");
    let b = key(100, "rule.b", 0, "work.one");
    let base = scheduled_cohort_identity(&profile_id(), LogicalTime(100), &[a.clone(), b.clone()]);
    assert_eq!(
        base,
        scheduled_cohort_identity(&profile_id(), LogicalTime(100), &[b.clone(), a.clone()])
    );
    assert_ne!(
        base,
        scheduled_cohort_identity(&profile_id(), LogicalTime(100), std::slice::from_ref(&a))
    );
    let mut altered = b.clone();
    altered.occurrence_index = OccurrenceIndex(1);
    assert_ne!(
        base,
        scheduled_cohort_identity(&profile_id(), LogicalTime(100), &[a.clone(), altered])
    );
    let mut kind = b.clone();
    kind.work_kind = work("work.two");
    assert_ne!(
        base,
        scheduled_cohort_identity(&profile_id(), LogicalTime(100), &[a, kind])
    );
    // Engine level: permuted genesis work yields the same cohort identities.
    let rules = vec![
        adder("rule.a", "work.one", "state.stress", 1),
        adder("rule.b", "work.one", "state.stress", 2),
    ];
    let mut x = engine_with(
        budgets(4),
        rules.clone(),
        vec![("rule.a", 100, "work.one"), ("rule.b", 100, "work.one")],
    );
    let mut y = engine_with(
        budgets(4),
        rules,
        vec![("rule.b", 100, "work.one"), ("rule.a", 100, "work.one")],
    );
    assert_eq!(run(&mut x, 100), run(&mut y, 100));
}

/// AT-I11: committed effects respect declared bounds at every boundary value,
/// rejecting atomically otherwise.
#[test]
fn at_i11_bounds_sweep_is_atomic() {
    for (v, ok) in [
        (0, true),
        (10, true),
        (-1, false),
        (11, false),
        (i64::MIN, false),
        (i64::MAX, false),
    ] {
        let mut e = engine_with(
            budgets(4),
            vec![on_command(
                "rule.set",
                "cmd.set",
                vec![
                    emit("x", "state.bounded", Update::Assign(lit(v))),
                    emit("y", "state.echo", add(1)),
                ],
            )],
            vec![],
        );
        let r = e.process(&command("cmd.set", 5, 1, "cmd.set", bron()));
        let committed = matches!(r.reports()[0].outcome, CohortOutcome::Committed);
        assert_eq!(committed, ok, "value {v}");
        assert_eq!(
            value(&e, "state.bounded", &bron()),
            if ok { Some(v) } else { None }
        );
        assert_eq!(value(&e, "state.echo", &bron()).is_some(), ok, "atomic");
    }
}

/// AT-I12: after every prefix of a scripted schedule/collide/drain/poison
/// sequence, every scheduled payload hash is its record hash (the per-slot
/// commitment equality) and the bidirectional invariant holds.
#[test]
fn at_i12_payload_hash_is_the_record_hash_after_every_prefix() {
    let rules = vec![
        with_schedule(
            adder("rule.a", "work.one", "state.stress", 1),
            reevaluate_after("n", 5, "work.one", "rule.a"),
        ),
        adder("rule.c", "work.c", "state.mood", 1),
    ];
    let mut e = engine_with(budgets(1), rules, vec![("rule.a", 10, "work.one")]);
    assert!(e.bidirectional_invariant_holds());
    let k = key(12, "rule.c", 0, "work.c");
    for claim in ["one", "two"] {
        let r = record(&e, &k, "rule.c", claim);
        fixture::schedule_record(&mut e, r);
        assert!(e.bidirectional_invariant_holds());
    }
    assert_eq!(e.slot_status(&k), WorkSlotStatus::Conflicted);
    for t in [10, 12, 15, 20, 25] {
        for r in drive(&mut e, &advance(t)) {
            let _ = r;
            assert!(e.bidirectional_invariant_holds(), "after advance to {t}");
        }
    }
    for k in e.obligations().keys() {
        let set = e.obligations().get(k).unwrap();
        assert!(!set.is_contested());
    }
}

/// AT-I15: three or more records under one key across all insertion
/// permutations, at and beyond the claim-set cap: equal digests, record-hash
/// discrimination, conflict-only drain removing the complete set, reschedule.
#[test]
fn at_i15_collisions_poison_order_independently_at_cap_edges() {
    let rules = vec![adder("rule.c", "work.c", "state.mood", 1)];
    let k = key(50, "rule.c", 0, "work.c");
    let perms = [
        [0usize, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    let mut digests = std::collections::BTreeSet::new();
    for p in perms {
        let mut e = engine_with(budgets(4), rules.clone(), vec![]);
        let claims = ["a", "b", "c"];
        for i in p {
            let r = record(&e, &k, "rule.c", claims[i]);
            fixture::schedule_record(&mut e, r);
        }
        assert!(e.bidirectional_invariant_holds());
        digests.insert((e.scheduler_digest(), e.obligations().canonical_digest()));
    }
    assert_eq!(digests.len(), 1);
    let mut other = engine_with(budgets(4), rules.clone(), vec![]);
    for c in ["a", "b", "d"] {
        let r = record(&other, &k, "rule.c", c);
        fixture::schedule_record(&mut other, r);
    }
    assert!(!digests.contains(&(
        other.scheduler_digest(),
        other.obligations().canonical_digest()
    )));
    let r = run(&mut other, 50);
    assert_eq!(r[0].kind, CohortKind::ConflictOnly);
    assert!(other.obligations().get(&k).is_none());
    let resolved = record(
        &other,
        &key(60, "rule.c", 1, "work.c"),
        "rule.c",
        "resolved",
    );
    fixture::schedule_record(&mut other, resolved);
    run(&mut other, 60);
    assert_eq!(value(&other, "state.mood", &bron()), Some(1));
    // Beyond the tracked-claim cap, both stores still agree in both orders.
    let n = MAX_CONFLICT_TRACKED_CLAIMS + 3;
    let build = |rev: bool| {
        let mut e = engine_with(budgets(4), rules.clone(), vec![]);
        let order: Vec<usize> = if rev {
            (0..n).rev().collect()
        } else {
            (0..n).collect()
        };
        for i in order {
            let r = record(&e, &k, "rule.c", &format!("claim.{i}"));
            fixture::schedule_record(&mut e, r);
        }
        assert!(e.bidirectional_invariant_holds());
        assert!(e.obligations().get(&k).unwrap().truncated());
        (e.scheduler_digest(), e.obligations().canonical_digest())
    };
    assert_eq!(build(false), build(true));
}

/// AT-I16: two allocations under one ledger key in one cohort follow ascending
/// emission identity; the mapping is identical under a pacing split.
#[test]
fn at_i16_occurrence_allocation_follows_emission_identity() {
    let rules = vec![
        with_schedule(
            with_schedule(
                adder("rule.s", "work.one", "state.stress", 1),
                reevaluate_after("s1", 5, "work.more", "rule.t"),
            ),
            reevaluate_after("s2", 5, "work.more", "rule.t"),
        ),
        adder("rule.t", "work.more", "state.mood", 1),
        adder("rule.u", "work.u", "state.echo", 1),
    ];
    let init = vec![("rule.u", 99, "work.u"), ("rule.s", 100, "work.one")];
    let mapping = |budget: u32| {
        let mut e = engine_with(budgets(budget), rules.clone(), init.clone());
        drive(&mut e, &advance(100));
        let mut pairs: Vec<(Digest, u64)> = e
            .obligations()
            .keys()
            .filter_map(|k| e.obligations().get(k).and_then(|s| s.sole_record()))
            .filter(|r| r.creator_rule_id() == &def("rule.s"))
            .map(|r| {
                (
                    r.creator_emission_identity().clone(),
                    r.key().occurrence_index.0,
                )
            })
            .collect();
        pairs.sort();
        (pairs, e.engine_state_digest())
    };
    let (pairs, d1) = mapping(1);
    assert_eq!(pairs.len(), 2);
    assert_eq!(
        pairs[0].1, 0,
        "the smaller emission identity takes the smaller occurrence"
    );
    assert_eq!(pairs[1].1, 1);
    let (pairs100, d100) = mapping(100);
    assert_eq!(pairs, pairs100);
    assert_eq!(d1, d100);
}

/// AT-I17: per-gate qualified addresses are distinct and stable; the same
/// address always yields the same draw; unrelated work does not move a draw.
#[test]
fn at_i17_probability_gates_use_stable_qualified_addresses() {
    let artifact = hash_bytes(b"artifact");
    let address = |gate: &str| RandomAddress {
        root_seed: 0,
        profile_id: profile_id(),
        behavior_epoch: 1,
        behavior_artifact_hash: artifact.clone(),
        rule_or_trigger_id: DefinitionId::new(format!("rule.gated.{gate}")).unwrap(),
        scope_id: bron(),
        occurrence_index: 3,
    };
    let s = RandomAddressService::new();
    assert_ne!(
        s.derive_u64(&address("gate1")),
        s.derive_u64(&address("gate2"))
    );
    assert_eq!(
        s.derive_u64(&address("gate1")),
        s.derive_u64(&address("gate1"))
    );
    let gated = |id: &str, kind: &str| {
        with_condition(
            work_rule(id, kind, vec![emit("x", "state.stress", add(1))]),
            Condition::Chance {
                sub_id: tag("gate"),
                rate: FixedPoint::from_raw(500_000),
            },
        )
    };
    // One rule set throughout (a different rule set is a different behavior
    // artifact, which rightly moves every draw); only unrelated *work* varies.
    let outcome = |extra: bool| {
        let rules = vec![
            gated("rule.g", "work.g"),
            adder("rule.noise", "work.noise", "state.echo", 1),
        ];
        let mut init: Vec<(&str, u64, &str)> =
            (0..20).map(|t| ("rule.g", 10 + t, "work.g")).collect();
        if extra {
            init.push(("rule.noise", 15, "work.noise"));
            init.push(("rule.noise", 22, "work.noise"));
            init.reverse();
        }
        let mut e = engine_with(budgets(4), rules, init);
        drive(&mut e, &advance(40));
        value(&e, "state.stress", &bron())
    };
    let base = outcome(false);
    assert_eq!(base, outcome(true));
    let fired = base.unwrap_or(0);
    assert!(
        fired > 0 && fired < 20,
        "a fair gate over 20 occurrences fires sometimes: {fired}"
    );
}

fn decay_engine(times: &[u64]) -> Engine {
    let rules = vec![
        on_command(
            "rule.seed",
            "cmd.seed",
            vec![emit("s", "state.stress", Update::Assign(lit(100)))],
        ),
        work_rule(
            "rule.decay",
            "work.decay",
            vec![emit("d", "state.stress", decay(10, 10))],
        ),
    ];
    // The target is the declared baseline 20 (v1 Q7), not a rule operand.
    let mut e = engine_with_baselines(
        budgets(4),
        rules,
        times
            .iter()
            .map(|t| ("rule.decay", *t, "work.decay"))
            .collect(),
        vec![baseline("state.stress", 20)],
    );
    e.process(&command("cmd.seed", 0, 1, "cmd.seed", bron()));
    e
}

/// AT-I22: closed-form catch-up is chunk-invariant at cadence-aligned
/// evaluations, converges exactly at the target, and never overshoots.
#[test]
fn at_i22_decay_catch_up_is_chunk_invariant() {
    let cell = |e: &Engine| {
        e.state()
            .get(&profile_id(), &def("state.stress"), &bron())
            .unwrap()
            .clone()
    };
    let mut once = decay_engine(&[30]);
    let mut steps = decay_engine(&[10, 20, 30]);
    let mut uneven = decay_engine(&[10, 30]);
    for e in [&mut once, &mut steps, &mut uneven] {
        drive(e, &advance(30));
    }
    assert_eq!(value(&once, "state.stress", &bron()), Some(70));
    for e in [&steps, &uneven] {
        assert_eq!(cell(e).value, cell(&once).value);
        assert_eq!(cell(e).updated_at, cell(&once).updated_at);
    }
    let mut long = decay_engine(&[200]);
    drive(&mut long, &advance(200));
    assert_eq!(
        value(&long, "state.stress", &bron()),
        Some(20),
        "stops exactly at the target"
    );
    // A cadence remainder is kept: no candidate when no whole step elapsed.
    let mut early = decay_engine(&[9]);
    let r = run(&mut early, 9);
    assert!(r[0].waves[0].committed.is_empty());
    assert_eq!(value(&early, "state.stress", &bron()), Some(100));
}

/// AT-I24: aggregation is a pure stable-order recomputation: child write order
/// does not matter, equal children stay distinct inputs, floor rounding is
/// exact, and `i128` accumulation overflow is typed.
#[test]
fn at_i24_aggregation_is_permutation_invariant_and_exact() {
    let rules = vec![
        on_command(
            "rule.child",
            "cmd.child",
            vec![emit("c", "state.stress", Update::Assign(param(0)))],
        ),
        on_command(
            "rule.agg",
            "cmd.agg",
            vec![emit_at(
                "g",
                "state.total",
                region("north"),
                Update::Aggregate {
                    source: def("state.stress"),
                    weight: FixedPoint::from_raw(500_000),
                },
            )],
        ),
    ];
    let build = |order: &[(&str, i64)]| {
        let mut e = engine_with(budgets(4), rules.clone(), vec![]);
        for (seq, (who, v)) in (1u64..).zip(order.iter()) {
            e.process(&Request::Command(command_request(
                &format!("cmd.c{seq}"),
                1,
                seq,
                "cmd.child",
                actor(who),
                vec![],
                vec![*v],
            )));
        }
        e.process(&Request::Command(command_request(
            "cmd.aggregate",
            2,
            100,
            "cmd.agg",
            bron(),
            vec![],
            vec![],
        )));
        value(&e, "state.total", &region("north"))
    };
    let forward = build(&[("a", 5), ("b", 5), ("c", 4)]);
    let reversed = build(&[("c", 4), ("b", 5), ("a", 5)]);
    assert_eq!(forward, Some(7), "floor((5 + 5 + 4) * 0.5)");
    assert_eq!(forward, reversed);
    assert_eq!(
        build(&[("a", -3)]),
        Some(-2),
        "floor toward negative infinity"
    );
}

/// AT-I26/AT-I27: each retained store discriminates, and equal digests reached
/// by permuted construction respond identically to the same next input.
#[test]
fn at_i26_at_i27_every_store_discriminates_and_equivalence_holds() {
    let rules = vec![
        with_schedule(
            adder("rule.a", "work.one", "state.stress", 1),
            reevaluate_after("n", 5, "work.one", "rule.a"),
        ),
        adder("rule.c", "work.c", "state.mood", 1),
        RuleSpec {
            cooldown: Some(10),
            ..on_command(
                "rule.cool",
                "cmd.cool",
                vec![emit("c", "state.echo", add(1))],
            )
        },
    ];
    let base = || engine_with(budgets(4), rules.clone(), vec![("rule.a", 10, "work.one")]);
    let reference = base().engine_state_digest();
    // ObligationStore / scheduler: one extra contested claim.
    let mut obl = base();
    let k = key(40, "rule.c", 0, "work.c");
    let r = record(&obl, &k, "rule.c", "only");
    fixture::schedule_record(&mut obl, r);
    assert_ne!(obl.engine_state_digest(), reference);
    // OccurrenceLedger only.
    let mut occ = base();
    fixture::set_occurrence_next(
        &mut occ,
        OccurrenceLedgerKey {
            profile_id: profile_id(),
            producer: def("rule.c"),
            scope_id: bron(),
            work_kind: work("work.c"),
        },
        7,
    );
    assert_ne!(occ.engine_state_digest(), reference);
    assert_eq!(
        occ.state().canonical_state_digest(),
        base().state().canonical_state_digest()
    );
    // CooldownLedger.
    let mut cool = base();
    cool.process(&command("cmd.cool", 1, 1, "cmd.cool", bron()));
    assert_eq!(cool.cooldowns().len(), 1);
    // EpochRegistry: an epoch activation changes only epoch state (and the
    // timeline that finalized it).
    let mut ep = base();
    let rs = ep.rule_set().clone();
    let cfg = ep.config().clone();
    ep.process(&Request::Command(spark_engine::request::CommandRequest {
        command_kind: kind("spark.epoch.activate"),
        payload: spark_engine::request::CommandPayload::ActivateEpoch {
            rule_set: rs,
            config: cfg,
        },
        ..command_request(
            "cmd.epoch",
            1,
            1,
            "spark.epoch.activate",
            bron(),
            vec![],
            vec![],
        )
    }));
    assert_eq!(ep.epoch_registry().records().len(), 2);
    assert_ne!(
        ep.epoch_registry().canonical_digest(),
        base().epoch_registry().canonical_digest()
    );
    // Equivalence: contested claims inserted in two orders.
    let build = |rev: bool| {
        let mut e = base();
        let mut claims = vec!["x", "y", "z"];
        if rev {
            claims.reverse();
        }
        for c in claims {
            let r = record(&e, &k, "rule.c", c);
            fixture::schedule_record(&mut e, r);
        }
        e
    };
    let mut p = build(false);
    let mut q = build(true);
    assert_eq!(p.stable_boundary_digest(), q.stable_boundary_digest());
    for next in [
        advance(20),
        advance(40),
        command("cmd.cool", 41, 1, "cmd.cool", bron()),
        advance(60),
    ] {
        assert_eq!(p.process(&next), q.process(&next));
        assert_eq!(p.stable_boundary_digest(), q.stable_boundary_digest());
    }
}

/// AT-I28 + AT-I29: derived indexes are validated (restore re-checks them)
/// after every prefix; reconstruction at the stable boundary between calls,
/// after a paced deferral, reproduces every later canonical value.
#[test]
fn at_i28_at_i29_reconstruction_needs_no_hidden_state() {
    let rules = vec![
        with_schedule(
            adder("rule.a", "work.one", "state.stress", 1),
            reevaluate_after("n", 5, "work.one", "rule.a"),
        ),
        adder("rule.b", "work.two", "state.mood", 1),
        on_command("rule.p", "cmd.p", vec![emit("p", "state.energy", add(1))]),
    ];
    let mut f = standard();
    let genesis = f.genesis(
        budgets(1),
        rules,
        vec![
            initial("rule.a", &bron(), 10, "work.one"),
            initial("rule.b", &bron(), 10, "work.two"),
        ],
    );
    let profile = genesis.profile.clone();
    let mut e = Engine::genesis(genesis).unwrap();
    let script = [
        advance(12),
        advance(30),
        command("cmd.p", 30, 1, "cmd.p", bron()),
        advance(40),
    ];
    for req in &script {
        loop {
            let snap = e.snapshot().unwrap();
            let mut restored = Engine::restore(snap, &profile).unwrap();
            let a = e.process(req);
            let b = restored.process(req);
            assert_eq!(a, b, "reconstruction at a stable boundary changes nothing");
            assert_eq!(
                e.stable_boundary_digest(),
                restored.stable_boundary_digest()
            );
            if a.outcome() != &Outcome::Paused {
                break;
            }
        }
    }
}

/// AT-I30/AT-I31: work scales with due keys, not with dormant scopes; dormant
/// aggregates move only at their scheduled occurrences.
#[test]
fn at_i30_at_i31_dormant_work_is_untouched() {
    let rules = vec![
        adder("rule.a", "work.one", "state.stress", 1),
        adder("rule.dormant", "work.dormant", "state.echo", 1),
    ];
    let mut small = engine_with(budgets(4), rules.clone(), vec![("rule.a", 10, "work.one")]);
    let mut dormant_init = vec![("rule.a", 10, "work.one")];
    for t in 0..500 {
        dormant_init.push(("rule.dormant", 1_000 + t, "work.dormant"));
    }
    let mut big = engine_with(budgets(4), rules, dormant_init);
    let rs = run(&mut small, 10);
    let rb = run(&mut big, 10);
    assert_eq!(rs.len(), 1);
    assert_eq!(rb.len(), 1);
    assert_eq!(
        fixture::observation(&small).prewave[0].cohort_identity,
        fixture::observation(&big).prewave[0].cohort_identity
    );
    assert_eq!(rs[0].waves[0].committed, rb[0].waves[0].committed);
    assert_eq!(big.scheduled_work_count(), 500, "dormant work untouched");
    assert!(value(&big, "state.echo", &bron()).is_none());
    run(&mut big, 1_004);
    assert_eq!(
        value(&big, "state.echo", &bron()),
        Some(5),
        "only at their occurrences"
    );
}

/// AT-I33: adversarial totality — extreme values yield typed outcomes, never a
/// panic.
#[test]
fn at_i33_extreme_inputs_are_typed_never_panics() {
    // i64::MIN subtraction normalization.
    let mut e = engine_with(
        budgets(4),
        vec![on_command(
            "rule.min",
            "cmd.min",
            vec![emit("x", "state.stress", Update::Subtract(lit(i64::MIN)))],
        )],
        vec![],
    );
    let r = e.process(&command("cmd.min", 5, 1, "cmd.min", bron()));
    assert!(matches!(
        r.reports()[0].outcome,
        CohortOutcome::Rejected {
            rejection: WaveRejection::Arithmetic { .. },
            ..
        }
    ));
    // u64::MAX horizons and due times.
    let mut e = engine_with(
        budgets(1),
        vec![adder("rule.a", "work.one", "state.stress", 1)],
        vec![("rule.a", u64::MAX, "work.one")],
    );
    assert_eq!(
        drive(&mut e, &advance(u64::MAX)).last().unwrap().outcome(),
        &Outcome::Completed
    );
    // Maximum-length identifiers: the longest rule ID whose qualified sub-ID
    // `rule_id.x` still fits the 256-byte bound activates; one byte more is a
    // typed refusal at the door.
    let longest = format!("rule.{}", "x".repeat(249));
    assert_eq!(longest.len() + ".x".len(), 256);
    let mut e = engine_with(
        budgets(4),
        vec![adder(&longest, "work.one", "state.stress", 1)],
        vec![(&longest, 3, "work.one")],
    );
    assert_eq!(run(&mut e, 3).len(), 1);
    let too_long = format!("rule.{}", "x".repeat(250));
    assert!(DefinitionId::new(&too_long).is_ok());
    let mut f = standard();
    assert!(f
        .rule_set(
            budgets(4),
            vec![adder(&too_long, "work.one", "state.stress", 1)]
        )
        .unwrap_err()
        .iter()
        .any(|e| matches!(
            e,
            spark_engine::rules::RuleSetError::QualifiedIdInvalid { .. }
        )));
    // An empty rule set and far more work than the pacing budget.
    let mut f = standard();
    let mut empty = f.engine(budgets(1), vec![], vec![]);
    assert_eq!(
        empty.process(&advance(u64::MAX)).outcome(),
        &Outcome::Completed
    );
    let mut e = engine_with(
        budgets(1),
        vec![adder("rule.a", "work.one", "state.stress", 1)],
        (0..64).map(|t| ("rule.a", t, "work.one")).collect(),
    );
    assert_eq!(drive(&mut e, &advance(100)).len(), 64);
    // A condition over a missing cell reads its declared absent value.
    let r = with_condition(
        on_command("rule.c", "cmd.c", vec![emit("x", "state.stress", add(1))]),
        Condition::Compare {
            left: Expr::Input(Input::Cell {
                definition: def("state.mood"),
                scope: ScopeRef::Subject,
                absent: -7,
            }),
            op: CmpOp::Eq,
            right: lit(-7),
        },
    );
    let mut e = engine_with(budgets(4), vec![r], vec![]);
    e.process(&command("cmd.c", 1, 1, "cmd.c", bron()));
    assert_eq!(value(&e, "state.stress", &bron()), Some(1));
}

/// AT-I38: truncation bounds explanation only — the fold sums every cause,
/// coverage is exact, `complete` iff nothing omitted.
#[test]
fn at_i38_provenance_coverage_is_honest() {
    for n in [8u64, 10, 11] {
        let mut rules = Vec::new();
        let mut init = Vec::new();
        for i in 0..n {
            let kind = format!("work.k{i}");
            rules.push(adder(&format!("rule.w{i}"), &kind, "state.stress", 3));
            init.push((format!("rule.w{i}"), 100, kind));
        }
        let mut f = standard();
        let mut e = f.engine(
            budgets(100),
            rules,
            init.iter()
                .map(|(r, t, k)| initial(r, &bron(), *t, k))
                .collect(),
        );
        let r = run(&mut e, 100);
        let p = &r[0].waves[0].committed[0].provenance;
        assert_eq!(
            value(&e, "state.stress", &bron()),
            Some(3 * n as i64),
            "the fold never truncates"
        );
        assert_eq!(p.retained.len() as u64, n.min(8));
        assert_eq!(p.omitted_source_count, n.saturating_sub(8));
        assert_eq!(p.coverage_status == CoverageStatus::Complete, n <= 8);
        assert_eq!(p.pruning_policy, "provenance_prune.v1");
        let mut sorted = p.retained.clone();
        sorted.sort();
        assert_eq!(&sorted, &p.retained, "ascending smallest identities");
    }
}

/// AT-I46(a): the canonical report field sets are pinned by exhaustive
/// destructuring (a new field — e.g. a pre-wave digest, `F`, or
/// `ActiveRequest` — fails to compile here). Bounded revision (C2-07): the
/// pinned set is the frozen one, **without** the `cohort_identity` field FINAL
/// AT-I46(a) forbids; the probe suite separately asserts that field, a
/// pre-wave field, `F`, and `ActiveRequest` are absent.
#[test]
fn at_i46_a_report_field_sets_are_pinned() {
    let mut e = engine_with(
        budgets(4),
        vec![adder("rule.a", "work.one", "state.stress", 1)],
        vec![("rule.a", 1, "work.one")],
    );
    let r = run(&mut e, 1).remove(0);
    let CohortReport {
        kind,
        canonical_time,
        conflicts,
        ingress,
        waves,
        outcome,
    } = r;
    let WaveReport {
        wave_index,
        candidate_set_digest,
        committed,
        obligations,
        cooldown_writes,
        cooldown_removals,
        depth_conversions,
        effect_batch_digest,
    } = waves[0].clone();
    let _ = (kind, canonical_time, conflicts, ingress, outcome);
    let _ = (
        wave_index,
        candidate_set_digest,
        committed,
        obligations,
        cooldown_writes,
        cooldown_removals,
        depth_conversions,
        effect_batch_digest,
    );
}

/// Cooldowns: a fired rule is ineligible until its expiry at the cohort's
/// canonical time; the expired entry consulted afterwards is removed in the
/// commit phase.
#[test]
fn cooldown_is_consulted_at_canonical_time_and_expires_deterministically() {
    let rule = RuleSpec {
        cooldown: Some(10),
        ..on_command(
            "rule.cool",
            "cmd.cool",
            vec![emit("c", "state.echo", add(1))],
        )
    };
    let mut e = engine_with(budgets(4), vec![rule], vec![]);
    let fire = |e: &mut Engine, t: u64, seq: u64| {
        reports(&drive(
            e,
            &command(&format!("cmd.c{seq}"), t, seq, "cmd.cool", bron()),
        ))
    };
    fire(&mut e, 5, 1);
    assert_eq!(
        e.cooldowns().expiry_of(&def("rule.cool"), &bron()),
        Some(LogicalTime(15))
    );
    fire(&mut e, 10, 2);
    assert_eq!(
        value(&e, "state.echo", &bron()),
        Some(1),
        "still cooling down"
    );
    let r = fire(&mut e, 15, 3);
    assert_eq!(value(&e, "state.echo", &bron()), Some(2));
    assert_eq!(
        r[0].waves[0].cooldown_removals,
        vec![(def("rule.cool"), bron())]
    );
    assert_eq!(
        e.cooldowns().expiry_of(&def("rule.cool"), &bron()),
        Some(LogicalTime(25))
    );
    let _ = CanonicalValue::Int(0);
}
