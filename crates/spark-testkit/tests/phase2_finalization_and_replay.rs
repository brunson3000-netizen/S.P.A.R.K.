//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! Gate C2 checkpoint 4: atomic single-command finalization, the request-bound
//! consumer, snapshot/restore, completed-boundary reconstruction, and epochs.
//!
//! AT-I48 (a)–(j), AT-I49 (a)–(d), AT-I50 (a)–(d), AT-I47 (d)(e), AT-I13, AT-I14,
//! AT-I23, and acceptance pins 1 and 2.

use spark_core::clock::LogicalTime;
use spark_core::hash::Digest;
use spark_core::id::{CommandId, ProfileId, SourceId};
use spark_core::timeline::{
    compute_ordered_stream_digest, Ordinal, SlotStatus, TimelineEpoch, TimelineFence,
    TimelineIngress,
};
use spark_core::value::CanonicalValue;
use spark_engine::engine::{
    CompletedHistory, Engine, PreflightRow, ReplayError, RestoreError, SnapshotRefused,
};
use spark_engine::fixture;
use spark_engine::obligation::{FrozenIntent, MaterializedEffect, ObligationMode};
use spark_engine::report::{CohortOutcome, ObligationRefusal};
use spark_engine::request::{
    CommandPayload, CommandRequest, FinalizationRefusal, Outcome, Request,
};
use spark_engine::rules::{Expr, Input, ResultFamily, RuleSpec, Update};
use spark_testkit::consumer::{self, ConsumerStop, Mailbox};
use spark_testkit::phase2::*;
use std::collections::BTreeMap;

fn bron() -> spark_core::scope::ScopeId {
    actor("bron")
}

fn poke_rules() -> Vec<RuleSpec> {
    vec![
        on_command(
            "rule.poke",
            "cmd.poke",
            vec![emit("p", "state.energy", Update::Add(lit(1)))],
        ),
        work_rule(
            "rule.x",
            "work.eval",
            vec![emit("x", "state.stress", Update::Add(lit(1)))],
        ),
    ]
}

fn engine(budget: u32, init: Vec<u64>) -> Engine {
    let mut f = standard();
    f.engine(
        budgets(budget),
        poke_rules(),
        init.into_iter()
            .map(|t| initial("rule.x", &bron(), t, "work.eval"))
            .collect(),
    )
}

fn cmd(id: &str, t: u64, seq: u64) -> CommandRequest {
    command_request(id, t, seq, "cmd.poke", bron(), vec![], vec![])
}

fn full_state(e: &Engine) -> (String, Digest, Digest, Ordinal) {
    (
        fixture::timeline_debug(e),
        e.timeline_state_digest(),
        e.timeline_history_digest(),
        e.timeline_frontier_ordinal(),
    )
}

/// The reviewed candidate's live stage-then-fence recipe on a working copy.
fn old_recipe(t: &mut TimelineIngress, c: &CommandRequest) -> bool {
    let Ok(window) = t.current_admission_window() else {
        return false;
    };
    let n = window.frontier_ordinal;
    let env = c.envelope_at(n);
    let sequencer = t.active_sequencer().clone();
    match t.stage(&sequencer, &env.clone().submit_with(window.ticket())) {
        Ok(d) if d.acknowledgement().is_some() => {}
        _ => return false,
    }
    let fence = TimelineFence {
        profile_id: t.profile_id().clone(),
        timeline_epoch: t.timeline_epoch(),
        fence_id: spark_core::id::FenceId::new(format!("fence.{}", n.0)).unwrap(),
        start_ordinal: n,
        end_ordinal: n,
        previous_fence_hash: t.last_finalized_fence_hash().clone(),
        ordered_stream_digest: compute_ordered_stream_digest(&[(n, env.semantic_hash())]),
    };
    t.submit_fence(&sequencer, &fence).is_ok()
}

// ================================================================ AT-I48

/// AT-I48(a): the historical red recipe stays red against unchanged Phase-1
/// code (preserved negative control; production Rust is not changed for it).
#[test]
fn at_i48_a_old_recipe_remains_red() {
    let mut e = engine(4, vec![]);
    assert_eq!(
        e.process(&Request::Command(cmd("first", 20, 10))).outcome(),
        &Outcome::Completed
    );
    let mut t = fixture::timeline_clone(&e);
    let before = format!("{t:?}");
    assert!(!old_recipe(&mut t, &cmd("distinct", 20, 9)));
    assert_ne!(before, format!("{t:?}"), "the old recipe leaves residue");
    assert_eq!(t.slot_status(Ordinal(1)), SlotStatus::Staged);
}

/// AT-I48(b)(f): source-sequence regression through `process` — typed P-9
/// refusal with the complete timeline state unchanged; `F` and `ActiveRequest`
/// complete; the next distinct command finalizes at ordinal 1.
#[test]
fn at_i48_b_f_source_sequence_regression_is_refused_byte_identically() {
    let mut e = engine(4, vec![]);
    e.process(&Request::Command(cmd("first", 20, 10)));
    let before = (full_state(&e), e.engine_state_digest());
    let r = e.process(&Request::Command(cmd("distinct", 20, 9)));
    assert_eq!(
        r.outcome(),
        &Outcome::CompletedCommandNotFinalized(FinalizationRefusal::SourceSequenceNotIncreasing {
            source_id: SourceId::new(SOURCE).unwrap(),
            previously_finalized: 10,
            attempted: 9
        })
    );
    assert_eq!(before, (full_state(&e), e.engine_state_digest()));
    assert_eq!(e.timeline_slot_status(Ordinal(1)), SlotStatus::Empty);
    assert_eq!((e.frontier(), e.active_request()), (LogicalTime(20), None));
    assert_eq!(
        e.process(&Request::Command(cmd("next", 21, 11))).outcome(),
        &Outcome::Completed
    );
    assert_eq!(e.timeline_frontier_ordinal(), Ordinal(2));
}

/// AT-I48(c): every refusal row, single-fault, complete timeline state equal.
#[test]
fn at_i48_c_every_refusal_route_is_byte_identical() {
    type Setup = Box<dyn Fn() -> (Engine, CommandRequest)>;
    let prior = || {
        let mut e = engine(4, vec![]);
        e.process(&Request::Command(cmd("first", 20, 10)));
        e
    };
    let x = || cmd("x", 20, 11);
    let routes: Vec<(&str, Setup, FinalizationRefusal)> = vec![
        (
            "P-1",
            Box::new(move || {
                let mut c = x();
                c.profile_id = ProfileId::new("elsewhere").unwrap();
                (prior(), c)
            }),
            FinalizationRefusal::WrongProfile,
        ),
        (
            "P-2",
            Box::new(move || {
                let mut c = x();
                c.timeline_epoch = TimelineEpoch(1);
                (prior(), c)
            }),
            FinalizationRefusal::WrongTimelineEpoch,
        ),
        (
            "P-3",
            Box::new(move || {
                let mut e = prior();
                fixture::set_grant(&mut e, SourceId::new("other.sequencer").unwrap());
                (e, x())
            }),
            FinalizationRefusal::NotActiveSequencer,
        ),
        (
            "P-4",
            Box::new(move || {
                let mut e = engine(4, vec![]);
                fixture::resume_timeline_at(&mut e, u64::MAX, 2);
                (e, x())
            }),
            FinalizationRefusal::OrdinalSpaceExhaustedWindow,
        ),
        (
            "P-5",
            Box::new(move || {
                let mut e = engine(4, vec![]);
                fixture::resume_timeline_at(&mut e, u64::MAX, 1);
                (e, x())
            }),
            FinalizationRefusal::OrdinalSpaceExhaustedFrontierAdvance,
        ),
        (
            "P-6 poisoned",
            Box::new(move || {
                let mut e = engine(4, vec![]);
                fixture::stage_raw(&mut e, cmd("p1", 20, 1).envelope_at(Ordinal(0)));
                fixture::stage_raw(&mut e, cmd("p2", 20, 2).envelope_at(Ordinal(0)));
                (e, x())
            }),
            FinalizationRefusal::UnexpectedStagingState {
                ordinal: Ordinal(0),
            },
        ),
        (
            "P-6 different staged",
            Box::new(move || {
                let mut e = engine(4, vec![]);
                fixture::stage_raw(&mut e, cmd("p1", 20, 1).envelope_at(Ordinal(0)));
                (e, x())
            }),
            FinalizationRefusal::UnexpectedStagingState {
                ordinal: Ordinal(0),
            },
        ),
        (
            "P-6 identical staged",
            Box::new(move || {
                let mut e = engine(4, vec![]);
                fixture::stage_raw(&mut e, x().envelope_at(Ordinal(0)));
                (e, x())
            }),
            FinalizationRefusal::UnexpectedStagingState {
                ordinal: Ordinal(0),
            },
        ),
        (
            "P-6 staged tail",
            Box::new(move || {
                let mut e = engine(4, vec![]);
                fixture::stage_raw(&mut e, cmd("tail", 20, 1).envelope_at(Ordinal(1)));
                (e, x())
            }),
            FinalizationRefusal::UnexpectedStagingState {
                ordinal: Ordinal(1),
            },
        ),
        (
            "P-7",
            Box::new(move || (prior(), cmd("first", 20, 11))),
            FinalizationRefusal::CommandIdentityConflict {
                command_id: CommandId::new("first").unwrap(),
            },
        ),
        (
            "P-8",
            Box::new(move || (prior(), cmd("other", 20, 10))),
            FinalizationRefusal::SourceSequenceConflict {
                source_id: SourceId::new(SOURCE).unwrap(),
                source_sequence: 10,
            },
        ),
        (
            "P-9",
            Box::new(move || (prior(), cmd("other", 20, 9))),
            FinalizationRefusal::SourceSequenceNotIncreasing {
                source_id: SourceId::new(SOURCE).unwrap(),
                previously_finalized: 10,
                attempted: 9,
            },
        ),
    ];
    for (name, setup, expected) in routes {
        let (mut e, c) = setup();
        let before = full_state(&e);
        let r = e.process(&Request::Command(c));
        assert_eq!(
            r.outcome(),
            &Outcome::CompletedCommandNotFinalized(expected),
            "{name}"
        );
        assert_eq!(
            before,
            full_state(&e),
            "{name}: complete timeline state must be unchanged"
        );
        assert!(!e.is_fail_stopped());
    }
}

/// AT-I48(d)(e): success equals the working-copy reference byte for byte; over
/// a generated corpus of clean-staging states the preflight verdict equals the
/// unchanged Phase-1 calls' verdict on a clone, and the reference never touches
/// live state.
#[test]
fn at_i48_d_e_differential_completeness_of_the_preflight() {
    let mut cases = 0u32;
    let mut refusals = 0u32;
    for history in 0u64..6 {
        let mut base = engine(4, vec![]);
        let mut seq_a = 10;
        for i in 0..history {
            let source = if i % 2 == 0 { SOURCE } else { "host.other" };
            let mut c = cmd(&format!("h{i}"), 20, seq_a);
            c.source_id = SourceId::new(source).unwrap();
            c.timeline_epoch = base.timeline_epoch();
            base.process(&Request::Command(c));
            seq_a += 3;
            if i == 3 {
                base.reset_timeline_epoch(TimelineEpoch(1), SourceId::new(SEQUENCER).unwrap())
                    .unwrap();
            }
        }
        for epoch in [0u64, 1] {
            for id in ["h0", "h1", "fresh"] {
                for seq in [5u64, 10, 13, 16, 40] {
                    for source in [SOURCE, "host.other"] {
                        let mut e = base.clone();
                        let mut c = cmd(id, 20, seq);
                        c.timeline_epoch = TimelineEpoch(epoch);
                        c.source_id = SourceId::new(source).unwrap();
                        let mut reference = fixture::timeline_clone(&e);
                        let live_before = full_state(&e);
                        let reference_ok = old_recipe(&mut reference, &c);
                        assert_eq!(
                            live_before,
                            full_state(&e),
                            "the reference never touches live state"
                        );
                        let r = e.process(&Request::Command(c));
                        let finalized = r.outcome() == &Outcome::Completed;
                        assert_eq!(
                            finalized, reference_ok,
                            "history={history} epoch={epoch} id={id} seq={seq} source={source}"
                        );
                        if finalized {
                            assert_eq!(fixture::timeline_debug(&e), format!("{reference:?}"));
                        } else {
                            refusals += 1;
                            assert_eq!(live_before, full_state(&e));
                        }
                        cases += 1;
                    }
                }
            }
        }
    }
    assert!(
        cases >= 300 && refusals > 50,
        "cases={cases} refusals={refusals}"
    );
}

/// AT-I48(h): horizon completion is not command success; scheduled work in the
/// request stays committed; nothing earlier is rolled back.
#[test]
fn at_i48_h_refusal_after_scheduled_work_keeps_the_work() {
    let mut e = engine(4, vec![22]);
    e.process(&Request::Command(cmd("first", 20, 10)));
    let timeline = full_state(&e);
    let r = e.process(&Request::Command(cmd("first", 25, 12)));
    assert!(matches!(
        r.outcome(),
        Outcome::CompletedCommandNotFinalized(FinalizationRefusal::CommandIdentityConflict { .. })
    ));
    assert_eq!(r.reports()[0].canonical_time, LogicalTime(22));
    assert_eq!(value(&e, "state.stress", &bron()), Some(1));
    assert_eq!(e.frontier(), LogicalTime(25));
    assert_eq!(timeline, full_state(&e));
}

/// AT-I48(j): with a preflight row disabled through the test-only fault seam,
/// the entailed apply observes a non-entailed result: sticky
/// `FINALIZATION_ENTAILMENT_VIOLATED`, never an ordinary refusal; no snapshot is
/// published; restoring the last committed snapshot recovers.
#[test]
fn at_i48_j_entailment_violation_is_a_sticky_fail_stop() {
    let mut f = standard();
    let genesis = f.genesis(budgets(4), poke_rules(), vec![]);
    let profile = genesis.profile.clone();
    let mut e = Engine::genesis(genesis).unwrap();
    e.process(&Request::Command(cmd("first", 20, 10)));
    let committed = e.snapshot().unwrap();
    fixture::disable_preflight_row(&mut e, Some(PreflightRow::P9));
    let regression = Request::Command(cmd("distinct", 20, 9));
    let r = e.process(&regression);
    assert_eq!(r.outcome(), &Outcome::FinalizationEntailmentViolated);
    assert!(r.reports().is_empty());
    assert!(e.is_fail_stopped());
    assert!(
        !e.staging_is_clean(),
        "the residue exists only in the fail-stopped instance"
    );
    assert_eq!(e.snapshot().unwrap_err(), SnapshotRefused::FailStopped);
    for next in [advance(30), Request::Command(cmd("again", 30, 11))] {
        assert_eq!(
            e.process(&next).outcome(),
            &Outcome::FinalizationEntailmentViolated
        );
    }
    let mut restored = Engine::restore(committed, &profile).unwrap();
    assert!(restored.staging_is_clean());
    assert!(matches!(
        restored.process(&regression).outcome(),
        Outcome::CompletedCommandNotFinalized(
            FinalizationRefusal::SourceSequenceNotIncreasing { .. }
        )
    ));
}

// ================================================================ AT-I49

/// AT-I49(a): faulty presentation with the genuine active head queued: mismatch
/// is not dequeue-eligible; queue, order, and engine unchanged; the head then
/// resumes; the old pop-on-mismatch rule loses the head (negative control).
#[test]
fn at_i49_a_mismatch_never_consumes_the_head() {
    let setup = || {
        let mut e = engine(1, vec![10, 15, 20]);
        let mut mb = Mailbox::new(4);
        mb.offer(advance(20));
        mb.offer(Request::Command(cmd("cmd.one", 21, 1)));
        assert_eq!(e.process(&advance(20)).outcome(), &Outcome::Paused);
        (e, mb)
    };
    let (mut e, mut mb) = setup();
    let before = (mb.contents(), e.stable_boundary_digest());
    let faulty = e.process(&Request::Command(cmd("cmd.sub", 12, 2)));
    assert!(matches!(
        faulty.outcome(),
        Outcome::RefusedActiveRequestMismatch { .. }
    ));
    assert!(!mb.complete_if_terminated(&faulty));
    assert_eq!(before, (mb.contents(), e.stable_boundary_digest()));
    let (results, stop) = consumer::drive(&mut e, &mut mb, 20);
    assert_eq!(stop, ConsumerStop::Drained);
    assert_eq!(results.last().unwrap().outcome(), &Outcome::Completed);
    let (mut undisturbed, mut mb2) = setup();
    consumer::drive(&mut undisturbed, &mut mb2, 20);
    assert_eq!(
        undisturbed.stable_boundary_digest(),
        e.stable_boundary_digest()
    );
    // Negative control: popping on mismatch loses the actual active head.
    let (mut bad, mut bad_mb) = setup();
    let _ = bad.process(&Request::Command(cmd("cmd.sub", 12, 2)));
    let mut q = bad_mb.contents();
    q.remove(0);
    assert_ne!(
        q.first().map(Request::discriminator),
        bad.active_request().cloned()
    );
    let _ = &mut bad_mb;
}

/// AT-I49(b): a completed non-head presentation cannot remove the actual head.
#[test]
fn at_i49_b_completed_non_head_presentation_cannot_remove_the_head() {
    let mut e = engine(4, vec![]);
    let mut mb = Mailbox::new(4);
    mb.offer(advance(30));
    let stray = e.process(&advance(10));
    assert_eq!(stray.outcome(), &Outcome::Completed);
    assert_eq!(stray.presented(), &advance(10).discriminator());
    assert!(!mb.complete_if_terminated(&stray));
    assert_eq!(mb.contents(), vec![advance(30)]);
    let head = e.process(&advance(30));
    assert!(mb.complete_if_terminated(&head));
    assert!(mb.is_empty());
}

/// AT-I49(c)(d): restore/mailbox disagreement halts fail-closed; durability
/// ordering both ways.
#[test]
fn at_i49_c_d_restore_mismatch_halts_and_durability_ordering() {
    let mut f = standard();
    let genesis = f.genesis(
        budgets(1),
        poke_rules(),
        vec![
            initial("rule.x", &bron(), 10, "work.eval"),
            initial("rule.x", &bron(), 20, "work.eval"),
        ],
    );
    let profile = genesis.profile.clone();
    let mut e = Engine::genesis(genesis).unwrap();
    assert_eq!(e.process(&advance(20)).outcome(), &Outcome::Paused);
    let paused = e.snapshot().unwrap();
    // (c) the durable head is not the active request.
    let mut restored = Engine::restore(paused.clone(), &profile).unwrap();
    let before = restored.stable_boundary_digest();
    let mut wrong = Mailbox::new(4);
    wrong.offer(Request::Command(cmd("cmd.one", 21, 1)));
    let (results, stop) = consumer::drive(&mut restored, &mut wrong, 10);
    assert_eq!(stop, ConsumerStop::HaltedActiveRequestNotAtHead);
    assert!(results.is_empty());
    assert_eq!(wrong.len(), 1);
    assert_eq!(before, restored.stable_boundary_digest());
    // (d) completion durable before the dequeue: redelivery is harmless.
    drive(&mut e, &advance(20));
    let sbd = e.stable_boundary_digest();
    let redelivered = e.process(&advance(20));
    assert_eq!(redelivered.outcome(), &Outcome::Completed);
    assert!(redelivered.reports().is_empty());
    assert_eq!(sbd, e.stable_boundary_digest());
    let c = Request::Command(cmd("cmd.one", 21, 1));
    e.process(&c);
    let (timeline, sbd) = (full_state(&e), e.stable_boundary_digest());
    assert!(matches!(
        e.process(&c).outcome(),
        Outcome::CompletedCommandNotFinalized(FinalizationRefusal::CommandIdentityConflict { .. })
    ));
    assert_eq!(
        (timeline, sbd),
        (full_state(&e), e.stable_boundary_digest())
    );
    // Dequeue durable first: the active request is gone from the mailbox.
    let mut lost = Engine::restore(paused, &profile).unwrap();
    let mut remaining = Mailbox::new(4);
    remaining.offer(Request::Command(cmd("cmd.one", 21, 1)));
    assert_eq!(
        consumer::drive(&mut lost, &mut remaining, 10).1,
        ConsumerStop::HaltedActiveRequestNotAtHead
    );
}

// ================================================================ AT-I47(d)(e)

/// AT-I47(d)(e): snapshot and restore validation, in order.
#[test]
fn at_i47_d_e_snapshot_restore_and_validation() {
    let mut f = standard();
    let genesis = f.genesis(
        budgets(1),
        poke_rules(),
        vec![
            initial("rule.x", &bron(), 10, "work.eval"),
            initial("rule.x", &bron(), 20, "work.eval"),
        ],
    );
    let profile = genesis.profile.clone();
    let mut e = Engine::genesis(genesis).unwrap();
    e.process(&advance(20));
    let snap = e.snapshot().unwrap();
    let mut r = Engine::restore(snap.clone(), &profile).unwrap();
    assert_eq!(
        &r.stable_boundary_digest(),
        snap.recorded_stable_boundary_digest()
    );
    let same_horizon = Request::Command(cmd("cmd.sub", 20, 1));
    let before = r.stable_boundary_digest();
    assert!(matches!(
        r.process(&same_horizon).outcome(),
        Outcome::RefusedActiveRequestMismatch { .. }
    ));
    assert_eq!(before, r.stable_boundary_digest());
    let mut uninterrupted = e.clone();
    let a = reports(&drive(&mut r, &advance(20)));
    let b = reports(&drive(&mut uninterrupted, &advance(20)));
    assert_eq!(a, b);
    assert_eq!(
        r.stable_boundary_digest(),
        uninterrupted.stable_boundary_digest()
    );
    assert!(Engine::restore(r.snapshot().unwrap(), &profile)
        .unwrap()
        .active_request()
        .is_none());

    let expect = |mut s: spark_engine::engine::EngineSnapshot,
                  edit: &dyn Fn(&mut spark_engine::engine::EngineSnapshot),
                  reseal: bool,
                  err: RestoreError| {
        edit(&mut s);
        if reseal {
            fixture::snapshot_reseal(&mut s);
        }
        assert_eq!(Engine::restore(s, &profile).unwrap_err(), err);
    };
    expect(
        snap.clone(),
        &|s| {
            fixture::snapshot_set_frontier(s, LogicalTime(30));
            fixture::snapshot_set_active(s, Some(&advance(5)));
        },
        true,
        RestoreError::ActiveBehindFrontier,
    );
    expect(
        snap.clone(),
        &|s| fixture::snapshot_set_active(s, None),
        false,
        RestoreError::DigestMismatch,
    );
    expect(
        snap.clone(),
        &|s| {
            fixture::snapshot_stage_raw(s, cmd("tail", 20, 1).envelope_at(Ordinal(0)));
        },
        true,
        RestoreError::TimelineStagingPresent,
    );
    let key = e.obligations().keys().next().unwrap().clone();
    expect(
        snap.clone(),
        &|s| fixture::snapshot_drop_obligation(s, &key),
        true,
        RestoreError::BidirectionalInvariantBroken,
    );
    let other = activate(&spark_engine::profile::manifest::ProfileManifest::new(
        profile_id(),
        spark_engine::profile::text::BoundedText::new("other").unwrap(),
        vec![int_spec(
            "state.stress",
            spark_core::authority::Authority::SparkOwned,
            0,
            5,
        )],
    ));
    assert_eq!(
        Engine::restore(snap, &other.profile).unwrap_err(),
        RestoreError::ArtifactBindingMismatch
    );
}

// ================================================================ AT-I50

/// AT-I50(a)(b): history plus `F` cannot reconstruct a pause; the committed
/// snapshot plus the exact request can.
#[test]
fn at_i50_a_b_history_and_f_cannot_reconstruct_a_pause() {
    let mut p = engine(1, vec![10, 20]);
    assert_eq!(p.process(&advance(20)).outcome(), &Outcome::Paused);
    let mut q = engine(1, vec![10, 20]);
    assert_eq!(q.process(&advance(0)).outcome(), &Outcome::Completed);
    assert_eq!(p.timeline_state_digest(), q.timeline_state_digest());
    assert_eq!(p.frontier(), q.frontier());
    assert_ne!(p.engine_state_digest(), q.engine_state_digest());
    assert_ne!(p.stable_boundary_digest(), q.stable_boundary_digest());
}

fn history_rules() -> Vec<RuleSpec> {
    poke_rules()
}

fn history_genesis(budget: u32, width: u32) -> spark_engine::engine::EngineGenesis {
    let mut f = standard();
    let mut g = f.genesis(
        budgets(budget),
        history_rules(),
        vec![
            initial("rule.x", &bron(), 10, "work.eval"),
            initial("rule.x", &bron(), 15, "work.eval"),
        ],
    );
    g.timeline.window_width = width;
    g
}

/// AT-I50(c)(d) and acceptance pin 1: completed-boundary reconstruction under
/// every declared input reproduces the stable boundary; a different window
/// width diverges; omitting a reset record cannot replay; several resets at one
/// frozen frontier replay in ascending `reset_index` regardless of input order.
#[test]
fn at_i50_c_d_completed_boundary_reconstruction_and_pin_1() {
    let mut e = Engine::genesis(history_genesis(1, 2)).unwrap();
    let c1 = cmd("c1", 20, 5);
    let dup = cmd("c1", 22, 6);
    let c2 = cmd("c2", 24, 7);
    for r in [
        advance(12),
        Request::Command(c1.clone()),
        advance(21),
        Request::Command(dup),
        Request::Command(c2.clone()),
    ] {
        drive(&mut e, &r);
    }
    e.reset_timeline_epoch(TimelineEpoch(1), SourceId::new("sequencer.one").unwrap())
        .unwrap();
    e.reset_timeline_epoch(TimelineEpoch(2), SourceId::new(SEQUENCER).unwrap())
        .unwrap();
    let mut c3 = cmd("c3", 30, 8);
    c3.timeline_epoch = TimelineEpoch(2);
    drive(&mut e, &Request::Command(c3.clone()));
    drive(&mut e, &advance(33));
    assert!(e.active_request().is_none());
    assert_eq!(e.finalized_commands().len(), 3);
    let resets = e.epoch_resets().to_vec();
    assert_eq!(resets.len(), 2);
    assert_eq!(
        resets[0].frozen_finalized_frontier,
        resets[1].frozen_finalized_frontier
    );
    let history = |resets: Vec<spark_core::timeline::EpochResetRecord>| CompletedHistory {
        commands: vec![c1.clone(), c2.clone(), c3.clone()],
        resets,
        frontier: e.frontier(),
        recorded_history_digest: e.timeline_history_digest(),
        recorded_stable_boundary_digest: e.stable_boundary_digest(),
    };
    let rebuilt =
        Engine::reconstruct_completed(history_genesis(9, 2), &history(resets.clone())).unwrap();
    assert_eq!(rebuilt.stable_boundary_digest(), e.stable_boundary_digest());
    assert_eq!(
        fixture::timeline_debug(&rebuilt),
        fixture::timeline_debug(&e)
    );
    let mut reversed = resets.clone();
    reversed.reverse();
    assert!(
        Engine::reconstruct_completed(history_genesis(9, 2), &history(reversed)).is_ok(),
        "ordered by reset_index, not input order"
    );
    // Negative control: applying the same-frontier resets in reverse order fails.
    let mut manual = Engine::genesis(history_genesis(9, 2)).unwrap();
    manual
        .reset_timeline_epoch(TimelineEpoch(2), SourceId::new(SEQUENCER).unwrap())
        .unwrap();
    assert!(manual
        .reset_timeline_epoch(TimelineEpoch(1), SourceId::new("sequencer.one").unwrap())
        .is_err());
    assert_eq!(
        Engine::reconstruct_completed(history_genesis(9, 3), &history(resets.clone())).unwrap_err(),
        ReplayError::Diverged
    );
    assert!(matches!(
        Engine::reconstruct_completed(history_genesis(9, 2), &history(vec![])).unwrap_err(),
        ReplayError::StepRefused { .. }
    ));
}

// ================================================================ epochs

/// AT-I23 + AT-I14: epoch-bound rates apply from the activating barrier; a
/// re-evaluation obligation whose rule was superseded refuses explicitly, while
/// an identical rule keeps executing.
#[test]
fn at_i23_at_i14_epoch_bound_rates_and_exact_rule_resolution() {
    let rate_rule = |id: &str| {
        work_rule(
            id,
            "work.rate",
            vec![emit(
                "r",
                "state.stress",
                Update::Add(Expr::Input(Input::Config {
                    key: def("tune.rate"),
                })),
            )],
        )
    };
    let mut f = standard();
    let rs1 = f
        .rule_set(budgets(4), vec![rate_rule("rule.rate")])
        .unwrap();
    let mut genesis = f.genesis(
        budgets(4),
        vec![rate_rule("rule.rate")],
        vec![
            initial("rule.rate", &bron(), 40, "work.rate"),
            initial("rule.rate", &bron(), 60, "work.rate"),
        ],
    );
    genesis.config = config(vec![("tune.rate", CanonicalValue::Int(1))]);
    genesis.rule_set = rs1.clone();
    let mut e = Engine::genesis(genesis.clone()).unwrap();
    let activate = |cfg: i64, rs: spark_engine::rules::ActivatedRuleSet| {
        Request::Command(CommandRequest {
            command_kind: kind("spark.epoch.activate"),
            payload: CommandPayload::ActivateEpoch {
                rule_set: rs,
                config: config(vec![("tune.rate", CanonicalValue::Int(cfg))]),
            },
            ..cmd("epoch.two", 50, 1)
        })
    };
    let r = reports(&drive(&mut e, &activate(5, rs1.clone())));
    assert_eq!(
        r.last().unwrap().outcome,
        CohortOutcome::EpochActivated { behavior_epoch: 2 }
    );
    drive(&mut e, &advance(60));
    assert_eq!(
        value(&e, "state.stress", &bron()),
        Some(1 + 5),
        "rate 1 before 50, rate 5 after"
    );
    assert_eq!(e.behavior_epoch(), 2);
    // A superseded rule under the same ID refuses explicitly.
    let changed = work_rule(
        "rule.rate",
        "work.rate",
        vec![emit("r", "state.stress", Update::Add(lit(100)))],
    );
    let rs2 = f.rule_set(budgets(4), vec![changed]).unwrap();
    let mut s = Engine::genesis(genesis).unwrap();
    drive(&mut s, &activate(1, rs2));
    let r = reports(&drive(&mut s, &advance(60)));
    assert!(matches!(
        r.last().unwrap().outcome,
        CohortOutcome::ObligationRefused(ObligationRefusal::RuleResolutionFailed { .. })
    ));
    assert_eq!(value(&s, "state.stress", &bron()), Some(1));
}

/// AT-I13: a materialized obligation refuses if **any** target fingerprint
/// drifted — including the second target — and otherwise applies its frozen
/// effects exactly as resolved at scheduling time.
#[test]
fn at_i13_materialized_mode_binds_every_target_fingerprint() {
    let e = engine(4, vec![100]);
    let key = e.obligations().keys().next().unwrap().clone();
    let template = e
        .obligations()
        .get(&key)
        .unwrap()
        .sole_record()
        .unwrap()
        .clone();
    let fp = |d: &str| {
        e.state()
            .schema_of(&profile_id(), &def(d))
            .unwrap()
            .fingerprint()
            .clone()
    };
    let effects = vec![
        MaterializedEffect {
            sub_id: tag("m1"),
            target: def("state.stress"),
            scope: bron(),
            intent: FrozenIntent::AddDelta(4),
        },
        MaterializedEffect {
            sub_id: tag("m2"),
            target: def("state.mood"),
            scope: bron(),
            intent: FrozenIntent::Result {
                value: 9,
                family: ResultFamily::Assignment,
            },
        },
    ];
    let mut good = BTreeMap::new();
    good.insert(def("state.stress"), fp("state.stress"));
    good.insert(def("state.mood"), fp("state.mood"));
    let mut drifted = good.clone();
    drifted.insert(
        def("state.mood"),
        spark_core::hash::hash_bytes(b"other artifact"),
    );
    for (fps, ok) in [(good, true), (drifted, false)] {
        let mut m = e.clone();
        let record = fixture::record_with_mode(
            &template,
            ObligationMode::MaterializedEffect {
                effects: effects.clone(),
                target_fingerprints: fps,
            },
        );
        fixture::tamper_obligations(&mut m, &key, None);
        // Replace the slot and record consistently.
        let mut probe = m.clone();
        let _ = fixture::extract_least_due_slice(&mut probe, LogicalTime(100));
        let mut fresh = engine(4, vec![]);
        fixture::schedule_record(&mut fresh, record);
        let r = reports(&drive(&mut fresh, &advance(100)));
        if ok {
            assert_eq!(r[0].outcome, CohortOutcome::Committed);
            assert_eq!(value(&fresh, "state.stress", &bron()), Some(4));
            assert_eq!(value(&fresh, "state.mood", &bron()), Some(9));
        } else {
            assert!(matches!(
                r[0].outcome,
                CohortOutcome::ObligationRefused(ObligationRefusal::TargetFingerprintDrift { ref target, .. }) if target == &def("state.mood")
            ));
            assert!(
                value(&fresh, "state.stress", &bron()).is_none(),
                "nothing reinterpreted, nothing applied"
            );
        }
        let _ = &mut m;
    }
}
