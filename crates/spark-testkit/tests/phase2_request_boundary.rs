//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 checkpoint 3: the serialized request boundary, cohort-granular
//! processing, and the request-boundary digests.
//!
//! AT-I43 (a)(b)(c)(d)(e)(f)(g)(h)(j)(k), AT-I39 (A)(B)(C)(D), AT-I20d, AT-I47
//! (a)(b)(c)(f)(g)(h), and the AT-I35 encoding pins for `canonicalize(ActiveRequest)`,
//! `stable_boundary_digest`, and the `engine_state_digest` component list.

use spark_core::clock::LogicalTime;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::SourceId;
use spark_core::scheduler::{DueWorkItem, Scheduler, WorkPayload};
use spark_core::timeline::{Ordinal, TimelineEpoch};
use spark_engine::engine::{stable_boundary_digest_of, Engine, FinalizeOp};
use spark_engine::fixture;
use spark_engine::report::{CohortKind, CohortOutcome, CohortReport, WaveRejection};
use spark_engine::request::{canonicalize_active_request, Outcome, Request};
use spark_engine::rules::{RuleSpec, Update};
use spark_testkit::consumer::{self, ConsumerStop, Mailbox};
use spark_testkit::phase2::*;

/// `A@10` enqueues `D` at `10 + 5 = 15`; `B@20`; a command rule adds energy.
fn base_rules() -> Vec<RuleSpec> {
    vec![
        with_schedule(
            work_rule(
                "rule.a",
                "work.eval",
                vec![emit("inc", "state.stress", Update::Add(lit(10)))],
            ),
            reevaluate_after("follow", 5, "work.eval", "rule.d"),
        ),
        work_rule(
            "rule.b",
            "work.eval",
            vec![emit("inc", "state.stress", Update::Add(lit(1)))],
        ),
        work_rule(
            "rule.d",
            "work.eval",
            vec![emit("set", "state.mood", Update::Assign(lit(7)))],
        ),
        on_command(
            "rule.cmd",
            "cmd.poke",
            vec![emit("poke", "state.energy", Update::Add(lit(3)))],
        ),
    ]
}

fn base_engine(budget: u32) -> Engine {
    let mut f = standard();
    f.engine(
        budgets(budget),
        base_rules(),
        vec![
            initial("rule.a", &actor("bron"), 10, "work.eval"),
            initial("rule.b", &actor("bron"), 20, "work.eval"),
        ],
    )
}

fn times(reports: &[CohortReport]) -> Vec<u64> {
    reports.iter().map(|r| r.canonical_time.0).collect()
}

fn poke(id: &str, t: u64, seq: u64) -> Request {
    command(id, t, seq, "cmd.poke", actor("bron"))
}

/// AT-I43(a), spelled out: `A@10`'s wave-0 rule creates `D@15`; with budget 1
/// successive calls of `Advance(20)` return PAUSED, PAUSED, COMPLETED; a
/// `Command@21` offered after call 1 is processed only afterwards.
#[test]
fn at_i43_a_lifecycle_successive_results_are_spelled_out() {
    let mut e = base_engine(1);
    let adv = advance(20);
    let mut mailbox = Mailbox::new(4);
    mailbox.offer(adv.clone());
    let r1 = e.process(&adv);
    assert_eq!(r1.outcome(), &Outcome::Paused);
    assert_eq!(times(r1.reports()), vec![10]);
    assert_eq!(e.frontier(), LogicalTime(0));
    assert_eq!(e.active_request(), Some(&adv.discriminator()));
    mailbox.offer(poke("cmd.one", 21, 1));
    let (results, stop) = consumer::drive(&mut e, &mut mailbox, 10);
    assert_eq!(stop, ConsumerStop::Drained);
    let outcomes: Vec<_> = results.iter().map(|r| r.outcome().clone()).collect();
    assert_eq!(
        outcomes,
        vec![Outcome::Paused, Outcome::Completed, Outcome::Completed]
    );
    assert_eq!(times(results[0].reports()), vec![15]);
    assert_eq!(times(results[1].reports()), vec![20]);
    assert_eq!(times(results[2].reports()), vec![21]);
    assert_eq!(results[2].reports()[0].kind, CohortKind::Command);
    assert_eq!(e.frontier(), LogicalTime(21));
    assert_eq!(e.active_request(), None);
}

fn run_budget(
    budget: u32,
) -> (
    Engine,
    Vec<CohortReport>,
    Vec<spark_engine::report::PacingDiagnostics>,
) {
    let mut e = base_engine(budget);
    let mut all = Vec::new();
    let mut diags = Vec::new();
    for req in [advance(20), poke("cmd.one", 21, 1)] {
        for r in drive(&mut e, &req) {
            all.extend(r.reports().to_vec());
            diags.push(r.diagnostics().cloned().unwrap());
        }
    }
    (e, all, diags)
}

/// AT-I43(b), AT-I39(D), AT-I20d: budgets 1, 2, 3 give identical canonical
/// reports, final digests, `F`, and `ActiveRequest`; diagnostics differ.
#[test]
fn at_i43_b_budgets_are_canonically_neutral_and_diagnostics_differ() {
    let (e1, r1, d1) = run_budget(1);
    let (e2, r2, d2) = run_budget(2);
    let (e3, r3, d3) = run_budget(3);
    assert_eq!(times(&r1), vec![10, 15, 20, 21]);
    assert_eq!(r1, r2);
    assert_eq!(r1, r3);
    assert_eq!(digest_pair(&e1), digest_pair(&e2));
    assert_eq!(digest_pair(&e1), digest_pair(&e3));
    assert_eq!(e1.frontier(), e3.frontier());
    assert_ne!(d1, d2, "honest telemetry must differ across budgets");
    assert!(d1.iter().any(|d| d.deferred_cohort_count > 0));
    assert!(d3.iter().all(|d| d.deferred_cohort_count == 0));
    // Deterministic: replaying the same history and budget reproduces them.
    let (_, _, d1_again) = run_budget(1);
    assert_eq!(d1, d1_again);
}

/// AT-I39(A) and AT-I47(c): one catch-up equals stepwise completed catch-ups
/// per cohort (reports, pre-wave digests, final stable boundary); appending
/// `ActiveRequest` to the per-cohort digest would break that equality.
#[test]
fn at_i39_a_catch_up_equals_stepwise_and_placement_negative_control() {
    let mut one = base_engine(3);
    let one_reports = reports(&drive(&mut one, &advance(20)));
    let one_obs = fixture::observation(&one).prewave.clone();
    let mut step = base_engine(3);
    let mut step_reports = Vec::new();
    let mut step_active = Vec::new();
    for t in [10, 12, 15, 17, 20] {
        let req = advance(t);
        let results = drive(&mut step, &req);
        for r in &results {
            for _ in r.reports() {
                step_active.push(req.discriminator());
            }
        }
        step_reports.extend(reports(&results));
    }
    let step_obs = fixture::observation(&step).prewave.clone();
    assert_eq!(one_reports, step_reports);
    assert_eq!(times(&one_reports), vec![10, 15, 20]);
    assert_eq!(one.stable_boundary_digest(), step.stable_boundary_digest());
    assert_eq!(one_obs.len(), step_obs.len());
    let mut active_would_differ = false;
    for (i, (a, b)) in one_obs.iter().zip(&step_obs).enumerate() {
        assert_eq!(
            a.engine_digest, b.engine_digest,
            "cohort {i} pre-wave digest"
        );
        let with = |active: &Request| {
            let mut enc = CanonicalEncoder::new();
            enc.push_digest(a.engine_digest.value());
            canonicalize_active_request(Some(&active.discriminator()), &mut enc);
            enc.finish()
        };
        let one_active = with(&advance(20));
        let mut enc = CanonicalEncoder::new();
        enc.push_digest(b.engine_digest.value());
        canonicalize_active_request(Some(&step_active[i]), &mut enc);
        if one_active != enc.finish() {
            active_would_differ = true;
        }
    }
    assert!(
        active_would_differ,
        "ActiveRequest in the per-cohort digest must break equality"
    );
}

/// AT-I39(B): `C1@100`, `C2@101`: C1's pre-wave scheduler digest equals an
/// independently constructed scheduler holding exactly `{C2}`, and differs from
/// the pre-extraction digest.
#[test]
fn at_i39_b_pre_wave_digest_is_pinned_against_an_independent_scheduler() {
    let mut f = standard();
    let mut e = f.engine(
        budgets(4),
        vec![
            work_rule(
                "rule.c1",
                "work.eval",
                vec![emit("x", "state.stress", Update::Add(lit(1)))],
            ),
            work_rule(
                "rule.c2",
                "work.eval",
                vec![emit("x", "state.mood", Update::Add(lit(1)))],
            ),
        ],
        vec![
            initial("rule.c1", &actor("bron"), 100, "work.eval"),
            initial("rule.c2", &actor("bron"), 101, "work.eval"),
        ],
    );
    let c2 = e
        .obligations()
        .keys()
        .find(|k| k.due_time.0 == 101)
        .unwrap()
        .clone();
    let c2_hash = e
        .obligations()
        .get(&c2)
        .unwrap()
        .sole_record()
        .unwrap()
        .record_hash();
    let pre_extraction = e.scheduler_digest();
    drive(&mut e, &advance(101));
    let obs = fixture::observation(&e).prewave.clone();
    let mut reference = Scheduler::new();
    reference.schedule(DueWorkItem {
        key: c2,
        payload: WorkPayload::new(c2_hash),
    });
    assert_eq!(
        obs[0].scheduler_digest.value(),
        &reference.canonical_state_digest()
    );
    assert_eq!(
        obs[1].scheduler_digest.value(),
        &Scheduler::new().canonical_state_digest()
    );
    assert_ne!(obs[0].scheduler_digest.value(), &pre_extraction);
}

/// AT-I39(C): outer-loop selections equal slices consumed plus one.
#[test]
fn at_i39_c_outer_loop_selection_count_is_slices_plus_one() {
    let rules = vec![work_rule(
        "rule.x",
        "work.eval",
        vec![emit("x", "state.stress", Update::Add(lit(1)))],
    )];
    let init = vec![
        initial("rule.x", &actor("bron"), 1, "work.eval"),
        initial("rule.x", &actor("bron"), 1_000_000, "work.eval"),
    ];
    let mut f = standard();
    let mut e = f.engine(budgets(9), rules.clone(), init.clone());
    fixture::clear_observation(&mut e);
    let r = e.process(&advance(1_000_000));
    assert_eq!(r.outcome(), &Outcome::Completed);
    assert_eq!(r.reports().len(), 2);
    assert_eq!(fixture::observation(&e).outer_selections, 3);
    let mut f = standard();
    let mut s = f.engine(budgets(9), rules, init);
    let mut stepwise = reports(&drive(&mut s, &advance(1)));
    stepwise.extend(reports(&drive(&mut s, &advance(1_000_000))));
    assert_eq!(r.reports().to_vec(), stepwise);
}

/// AT-I43(d): the start rule `h >= F`.
#[test]
fn at_i43_d_start_rule_refuses_behind_frontier_without_mutation() {
    let mut e = base_engine(4);
    drive(&mut e, &advance(20));
    let before = (digest_pair(&e), e.timeline_state_digest());
    let r = e.process(&poke("cmd.late", 15, 1));
    assert_eq!(
        r.outcome(),
        &Outcome::RefusedHorizonBehindFrontier {
            frontier: LogicalTime(20),
            horizon: LogicalTime(15)
        }
    );
    assert_eq!(before, (digest_pair(&e), e.timeline_state_digest()));
    assert_eq!(e.active_request(), None);
    assert_eq!(
        e.process(&poke("cmd.eq", 20, 2)).outcome(),
        &Outcome::Completed
    );
    assert_eq!(
        e.process(&poke("cmd.later", 25, 3)).outcome(),
        &Outcome::Completed
    );
    assert_eq!(e.finalized_commands().len(), 2);
}

fn three_cohort_engine(budget: u32) -> Engine {
    let mut f = standard();
    f.engine(
        budgets(budget),
        vec![
            work_rule(
                "rule.x",
                "work.eval",
                vec![emit("x", "state.stress", Update::Add(lit(1)))],
            ),
            on_command(
                "rule.cmd",
                "cmd.poke",
                vec![emit("p", "state.energy", Update::Add(lit(1)))],
            ),
        ],
        vec![
            initial("rule.x", &actor("bron"), 10, "work.eval"),
            initial("rule.x", &actor("bron"), 15, "work.eval"),
            initial("rule.x", &actor("bron"), 20, "work.eval"),
        ],
    )
}

/// AT-I43(e) and (k): SB-01. After work through 15 while `F = 0`, a substituted
/// `Command@12` and a higher `Advance(25)` are refused byte-identically, driven
/// directly with no consumer; the exact head then completes.
#[test]
fn at_i43_e_sb01_substitution_is_refused_without_mutation() {
    let mut e = three_cohort_engine(2);
    let head = advance(20);
    let r = e.process(&head);
    assert_eq!(r.outcome(), &Outcome::Paused);
    assert_eq!(times(r.reports()), vec![10, 15]);
    let cell = e
        .state()
        .get(&profile_id(), &def("state.stress"), &actor("bron"))
        .unwrap()
        .clone();
    assert_eq!(cell.updated_at, LogicalTime(15));
    assert_eq!(e.frontier(), LogicalTime(0));
    let before = (digest_pair(&e), e.timeline_state_digest(), e.frontier());
    for sub in [poke("cmd.sub", 12, 1), advance(25)] {
        let refused = e.process(&sub);
        assert_eq!(
            refused.outcome(),
            &Outcome::RefusedActiveRequestMismatch {
                active: head.discriminator()
            }
        );
        assert_eq!(refused.presented(), &sub.discriminator());
        assert!(refused.reports().is_empty());
        assert_eq!(
            before,
            (digest_pair(&e), e.timeline_state_digest(), e.frontier())
        );
        assert_eq!(e.active_request(), Some(&head.discriminator()));
    }
    assert_eq!(e.process(&head).outcome(), &Outcome::Completed);
    assert_eq!(e.frontier(), LogicalTime(20));
    assert_eq!(e.active_request(), None);
}

/// AT-I43(f) revised: a paused **Command** refuses a same-kind, same-horizon
/// command with a different id/payload (kills a kind+horizon-only
/// discriminator); the Advance-active variant is retained.
#[test]
fn at_i43_f_same_kind_same_horizon_substitution_is_refused() {
    let mut e = three_cohort_engine(1);
    let head = Request::Command(command_request(
        "cmd.p1",
        20,
        1,
        "cmd.poke",
        actor("bron"),
        vec![],
        vec![1],
    ));
    assert_eq!(e.process(&head).outcome(), &Outcome::Paused);
    let sub = Request::Command(command_request(
        "cmd.p2",
        20,
        2,
        "cmd.poke",
        actor("bron"),
        vec![],
        vec![2],
    ));
    assert_eq!(sub.discriminator().kind(), head.discriminator().kind());
    assert_eq!(
        sub.discriminator().horizon(),
        head.discriminator().horizon()
    );
    let before = (digest_pair(&e), e.timeline_state_digest());
    assert!(matches!(
        e.process(&sub).outcome(),
        Outcome::RefusedActiveRequestMismatch { .. }
    ));
    assert_eq!(before, (digest_pair(&e), e.timeline_state_digest()));
    let mut a = three_cohort_engine(1);
    assert_eq!(a.process(&advance(20)).outcome(), &Outcome::Paused);
    assert!(matches!(
        a.process(&poke("cmd.x", 20, 1)).outcome(),
        Outcome::RefusedActiveRequestMismatch { .. }
    ));
}

/// AT-I43(g): an equal-horizon advance after completion completes with no cohort
/// and a byte-identical stable boundary.
#[test]
fn at_i43_g_equal_horizon_advance_is_idempotent() {
    let mut e = base_engine(4);
    drive(&mut e, &advance(20));
    let sbd = e.stable_boundary_digest();
    let r = e.process(&advance(20));
    assert_eq!(r.outcome(), &Outcome::Completed);
    assert!(r.reports().is_empty());
    assert_eq!(sbd, e.stable_boundary_digest());
}

/// AT-I43(h) and AT-I48(d)(i): finalization at completion — no ordinal before
/// completion, a positive acknowledgement then one one-ordinal fence named
/// `fence.<ordinal>`, the command cohort at `now = effective_time` after every
/// slice `<= 21`, clean staging at every boundary; a budget-deferred command
/// leaves the timeline unchanged and matches the unbudgeted run.
#[test]
fn at_i43_h_command_finalization_happens_at_completion() {
    let req = poke("cmd.one", 21, 1);
    let mut paced = base_engine(1);
    fixture::clear_observation(&mut paced);
    let mut seen_deferred = false;
    let mut results = Vec::new();
    loop {
        let timeline_before = paced.timeline_state_digest();
        let r = paced.process(&req);
        assert!(paced.staging_is_clean());
        if r.outcome() == &Outcome::Paused {
            assert_eq!(timeline_before, paced.timeline_state_digest());
            assert_eq!(paced.timeline_frontier_ordinal(), Ordinal(0));
            // A command request pausing with no executable slice left in its
            // horizon has its command deferred (A6) — derivable from the
            // frozen diagnostic fields (C2-07 adjudication).
            if r.diagnostics().unwrap().deferred_cohort_count == 0 {
                seen_deferred = true;
            }
            results.push(r);
            continue;
        }
        assert_eq!(r.outcome(), &Outcome::Completed);
        results.push(r);
        break;
    }
    assert!(
        seen_deferred,
        "budget 1 must defer the command at least once"
    );
    assert_eq!(
        fixture::observation(&paced).finalize_ops,
        vec![
            FinalizeOp::PositiveStagedAcknowledgement {
                ordinal: Ordinal(0)
            },
            FinalizeOp::OneOrdinalFence {
                ordinal: Ordinal(0),
                fence_id: "fence.0".to_string()
            }
        ]
    );
    let all = reports(&results);
    assert_eq!(times(&all), vec![10, 15, 20, 21]);
    assert_eq!(all.last().unwrap().kind, CohortKind::Command);
    let mut unpaced = base_engine(100);
    let one = drive(&mut unpaced, &req);
    assert_eq!(reports(&one), all);
    assert_eq!(digest_pair(&unpaced), digest_pair(&paced));
    assert_eq!(unpaced.timeline_frontier_ordinal(), Ordinal(1));
}

/// AT-I43(j): checked delayed-time arithmetic rejects the wave atomically.
#[test]
fn at_i43_j_delayed_time_overflow_rejects_the_wave() {
    let mut f = standard();
    let mut e = f.engine(
        budgets(4),
        vec![with_schedule(
            on_command(
                "rule.far",
                "cmd.poke",
                vec![emit("x", "state.stress", Update::Add(lit(1)))],
            ),
            reevaluate_after("far", u64::MAX, "work.eval", "rule.far"),
        )],
        vec![],
    );
    let r = e.process(&poke("cmd.far", 5, 1));
    assert_eq!(r.outcome(), &Outcome::Completed);
    let cohort = &r.reports()[0];
    assert_eq!(
        cohort.outcome,
        CohortOutcome::Rejected {
            wave: 0,
            rejection: WaveRejection::Arithmetic {
                what: "delayed_time"
            }
        }
    );
    assert!(value(&e, "state.stress", &actor("bron")).is_none());
    assert_eq!(e.scheduled_work_count(), 0);
    assert_eq!(
        e.finalized_commands().len(),
        1,
        "the command stays finalized (FINAL §13)"
    );
}

fn discriminators() -> Vec<Option<Request>> {
    vec![
        None,
        Some(advance(20)),
        Some(Request::Command(command_request(
            "cmd.id1",
            20,
            1,
            "cmd.poke",
            actor("bron"),
            vec![],
            vec![],
        ))),
        Some(advance(21)),
        Some(Request::Command(command_request(
            "cmd.id2",
            20,
            1,
            "cmd.poke",
            actor("bron"),
            vec![],
            vec![],
        ))),
    ]
}

/// AT-I47(a), stated exactly: five `ActiveRequest` values over one engine digest
/// and one `F` give five distinct stable-boundary digests; `F` alone also
/// discriminates. (Omission of kind or horizon is not killable here — see the
/// encoding pin.)
#[test]
fn at_i47_a_stable_boundary_digest_discriminates_f_and_active_request() {
    let esd = base_engine(1).engine_state_digest();
    let sbds: std::collections::BTreeSet<Digest> = discriminators()
        .iter()
        .map(|r| {
            let d = r.as_ref().map(Request::discriminator);
            stable_boundary_digest_of(&esd, LogicalTime(0), d.as_ref())
        })
        .collect();
    assert_eq!(sbds.len(), 5);
    assert_ne!(
        stable_boundary_digest_of(&esd, LogicalTime(0), None),
        stable_boundary_digest_of(&esd, LogicalTime(1), None)
    );
}

/// AT-I35 encoding pins, against an independent reference encoder written from
/// FINAL §6.2 / §14: `canonicalize(ActiveRequest)`, the stable-boundary
/// composition, and the engine-digest component list. Mutants that omit kind or
/// horizon fail the byte comparison.
#[test]
fn at_i35_active_request_and_boundary_encodings_are_pinned() {
    let adv = advance(20);
    let mut id = CanonicalEncoder::new();
    id.push_str("request_advance_v1").push_u64(20);
    let identity = id.finish();
    assert_eq!(adv.discriminator().identity(), &identity);
    let reference = |kind: bool, horizon: bool| {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("active_request.some");
        if kind {
            enc.push_str("advance");
        }
        if horizon {
            enc.push_u64(20);
        }
        enc.push_digest(&identity);
        enc.into_bytes()
    };
    let mut actual = CanonicalEncoder::new();
    canonicalize_active_request(Some(&adv.discriminator()), &mut actual);
    let actual = actual.into_bytes();
    assert_eq!(actual, reference(true, true));
    assert_ne!(
        actual,
        reference(false, true),
        "omitting kind must fail the pin"
    );
    assert_ne!(
        actual,
        reference(true, false),
        "omitting horizon must fail the pin"
    );
    let mut none = CanonicalEncoder::new();
    canonicalize_active_request(None, &mut none);
    let mut none_ref = CanonicalEncoder::new();
    none_ref.push_str("active_request.none");
    assert_eq!(none.into_bytes(), none_ref.into_bytes());

    let mut e = base_engine(1);
    e.process(&adv);
    let mut esd = CanonicalEncoder::new();
    esd.push_str("engine_state")
        .push_digest(&e.state().canonical_state_digest())
        .push_digest(&e.scheduler_digest())
        .push_digest(&e.timeline_state_digest())
        .push_digest(&e.epoch_registry().canonical_digest())
        .push_digest(&e.obligations().canonical_digest())
        .push_digest(&e.occurrences().canonical_digest())
        .push_digest(&e.cooldowns().canonical_digest());
    assert_eq!(esd.finish(), e.engine_state_digest());
    let mut sbd = CanonicalEncoder::new();
    sbd.push_str("stable_boundary_v1")
        .push_digest(&e.engine_state_digest())
        .push_u64(e.frontier().0);
    canonicalize_active_request(e.active_request(), &mut sbd);
    assert_eq!(sbd.finish(), e.stable_boundary_digest());
}

/// AT-I47(b): equal stable boundaries reached by permuted construction respond
/// identically to the same next requests.
#[test]
fn at_i47_b_equal_boundaries_respond_identically() {
    let build = |reversed: bool| {
        let mut init = vec![
            initial("rule.a", &actor("bron"), 10, "work.eval"),
            initial("rule.b", &actor("bron"), 20, "work.eval"),
        ];
        let mut rules = base_rules();
        if reversed {
            init.reverse();
            rules.reverse();
        }
        let mut f = standard();
        let mut e = f.engine(budgets(1), rules, init);
        e.process(&advance(20));
        e
    };
    let mut a = build(false);
    let mut b = build(true);
    assert_eq!(a.stable_boundary_digest(), b.stable_boundary_digest());
    for next in [
        poke("cmd.x", 12, 1),
        advance(20),
        advance(5),
        poke("cmd.y", 20, 2),
    ] {
        let ra = a.process(&next);
        let rb = b.process(&next);
        assert_eq!(ra, rb);
        assert_eq!(a.stable_boundary_digest(), b.stable_boundary_digest());
    }
}

/// AT-I47(f)(g)(h): absence after completion and refusals; set before the
/// first cohort mutation; no other transition (an epoch reset leaves `F` and
/// `ActiveRequest` byte-identical).
#[test]
fn at_i47_f_g_h_active_request_lifecycle() {
    let mut e = base_engine(1);
    fixture::clear_observation(&mut e);
    let head = advance(20);
    e.process(&head);
    assert!(fixture::observation(&e).prewave[0].active_request_set);
    let (f_before, a_before) = (e.frontier(), e.active_request().cloned());
    e.reset_timeline_epoch(TimelineEpoch(1), SourceId::new(SEQUENCER).unwrap())
        .unwrap();
    assert_eq!(
        (e.frontier(), e.active_request().cloned()),
        (f_before, a_before)
    );
    drive(&mut e, &head);
    assert_eq!(e.active_request(), None);
    assert_eq!(e.frontier(), LogicalTime(20));
    let sbd = e.stable_boundary_digest();
    e.process(&advance(5));
    assert_eq!(e.active_request(), None);
    assert_eq!(sbd, e.stable_boundary_digest());
}
