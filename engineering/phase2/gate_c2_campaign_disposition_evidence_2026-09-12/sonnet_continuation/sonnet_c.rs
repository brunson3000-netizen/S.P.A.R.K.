//! SONNET-C bounded collection-only continuation campaign.
//! Target: 67b877192cc78b75c6fbe60c69b5594dc10befe8
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use spark_core::clock::LogicalTime;
use spark_core::id::SourceId;
use spark_core::timeline::TimelineEpoch;
use spark_engine::engine::{CompletedHistory, Engine, ReplayError, RestoreError, SnapshotRefused};
use spark_engine::fixture;
use spark_engine::obligation::ObligationMode;
use spark_engine::report::{CohortKind, CohortOutcome, WaveRejection};
use spark_engine::request::{CommandRequest, FinalizationRefusal, Outcome, Request};
use spark_engine::rules::RuleSpec;
use spark_testkit::phase2::*;

fn bron() -> spark_core::scope::ScopeId {
    actor("bron")
}

fn poke_rules() -> Vec<RuleSpec> {
    vec![
        on_command(
            "rule.poke",
            "cmd.poke",
            vec![emit("p", "state.energy", spark_engine::rules::Update::Add(lit(1)))],
        ),
        work_rule(
            "rule.x",
            "work.eval",
            vec![emit("x", "state.stress", spark_engine::rules::Update::Add(lit(1)))],
        ),
    ]
}

/// budget `b`, initial re-evaluation work due at each of `init`.
fn engine(b: u32, init: Vec<u64>) -> Engine {
    let mut f = standard();
    f.engine(
        budgets(b),
        poke_rules(),
        init.into_iter()
            .map(|t| initial("rule.x", &bron(), t, "work.eval"))
            .collect(),
    )
}

fn cmd(id: &str, t: u64, seq: u64) -> CommandRequest {
    command_request(id, t, seq, "cmd.poke", bron(), vec![], vec![])
}

// ================================================================ SC-1
//
// Interrupted request boundary: a command pauses mid-flight (budget forces
// two cohorts across the same horizon), a snapshot is taken while
// `ActiveRequest` is `Some`, restored, and then resumed three ways: the same
// request, a different request while still mismatched, and the same request
// again to confirm the mismatch attempt left no residue.

#[test]
fn sc1_snapshot_restore_mid_paused_command_then_resume() {
    // budget 1, two re-evaluation obligations due at/under horizon 20, plus a
    // command due at 20 too, so the command's own presentation pauses at least
    // once before it can finalize.
    let mut e = engine(1, vec![10, 20]);
    let request = Request::Command(cmd("c1", 20, 5));
    let r1 = e.process(&request);
    assert_eq!(r1.outcome(), &Outcome::Paused, "expected a mid-command pause");
    assert!(e.active_request().is_some(), "ActiveRequest must be set while paused");
    let active_before = e.active_request().cloned().unwrap();

    // Snapshot while ActiveRequest is Some(Paused-command).
    let snap = e.snapshot().expect("snapshot must succeed: not fail-stopped");
    assert_eq!(snap.active_request(), Some(&active_before));

    // Independent restore target: build a fresh activation and restore into it.
    let mut f2 = standard();
    let fixture2 = &f2; // profile identity for restore binding
    let profile_ref = &fixture2.profile;
    let restored = Engine::restore(snap, profile_ref).expect("restore of a mid-paused snapshot must succeed (horizon >= frontier)");
    assert_eq!(restored.active_request(), Some(&active_before));
    assert_eq!(restored.frontier(), e.frontier());

    // (a) Present a DIFFERENT request while active is still Some: must be
    // refused with RefusedActiveRequestMismatch, and must not corrupt state.
    let mut probe = restored.clone();
    let other = Request::Command(cmd("other", 20, 6));
    let mismatch = probe.process(&other);
    match mismatch.outcome() {
        Outcome::RefusedActiveRequestMismatch { active } => {
            assert_eq!(active, &active_before);
        }
        other => panic!("expected RefusedActiveRequestMismatch, got {other:?}"),
    }
    // State must be byte-identical to before the mismatched attempt.
    assert_eq!(probe.stable_boundary_digest(), restored.stable_boundary_digest());
    assert_eq!(probe.active_request(), Some(&active_before));

    // (b) Present the SAME request on the restored engine: must resume and
    // reach the identical terminal state as an unbroken drive of a twin
    // engine that never snapshotted/restored.
    let mut resumed = restored;
    let final_outcomes = drive(&mut resumed, &request);
    let last = final_outcomes.last().unwrap();
    assert_eq!(last.outcome(), &Outcome::Completed);

    let mut twin = engine(1, vec![10, 20]);
    drive(&mut twin, &request);
    assert_eq!(resumed.stable_boundary_digest(), twin.stable_boundary_digest());
    assert_eq!(resumed.engine_state_digest(), twin.engine_state_digest());
    assert_eq!(resumed.frontier(), twin.frontier());

    // (c) After the failed mismatch attempt above (on a clone `probe`), doing
    // the correct resume on that same clone must also still succeed, proving
    // the refused attempt left no residue on the engine it touched.
    let mut probe_resume = probe;
    let po = drive(&mut probe_resume, &request);
    assert_eq!(po.last().unwrap().outcome(), &Outcome::Completed);
    assert_eq!(probe_resume.stable_boundary_digest(), twin.stable_boundary_digest());
}

// ================================================================ SC-2
//
// Epoch reset while ActiveRequest is Some(Paused): reset_timeline_epoch is
// documented (engine.rs, at reset_timeline_epoch) to change neither `F` nor
// `ActiveRequest`. Confirm that empirically, then confirm the eventual
// resume of the stale-epoch command finalizes-refused (WrongTimelineEpoch,
// D-2) rather than silently finalizing under the wrong epoch, and that the
// engine is left usable afterward.

#[test]
fn sc2_epoch_reset_mid_paused_command_then_resume_refuses_stale_epoch() {
    let mut e = engine(1, vec![10, 20]);
    let request = Request::Command(cmd("c1", 20, 5));
    let r1 = e.process(&request);
    assert_eq!(r1.outcome(), &Outcome::Paused);
    let active_before = e.active_request().cloned().unwrap();
    let frontier_before = e.frontier();

    // Reset the timeline epoch mid-pause.
    e.reset_timeline_epoch(TimelineEpoch(1), SourceId::new("sequencer.two").unwrap())
        .expect("reset_timeline_epoch is documented to be independent of ActiveRequest");

    // Per doc: changes neither F nor ActiveRequest.
    assert_eq!(e.frontier(), frontier_before, "reset must not move F");
    assert_eq!(
        e.active_request(),
        Some(&active_before),
        "reset must not touch ActiveRequest"
    );

    // Resume the SAME (now stale-epoch) command to completion.
    let results = drive(&mut e, &request);
    let last = results.last().unwrap();
    match last.outcome() {
        Outcome::CompletedCommandNotFinalized(FinalizationRefusal::WrongTimelineEpoch { .. }) => {}
        other => panic!(
            "expected CompletedCommandNotFinalized(WrongTimelineEpoch) resuming a command \
             stamped for a superseded epoch, got {other:?}"
        ),
    }
    // ActiveRequest must clear even though the command itself wasn't finalized (D-2).
    assert!(e.active_request().is_none(), "ActiveRequest must clear after the horizon completes");
    assert_eq!(e.frontier(), LogicalTime(20), "F must advance to the horizon regardless of D-2");

    // Engine must remain usable: a fresh command stamped for the NEW epoch
    // at a later horizon must finalize normally.
    let mut c2 = cmd("c2", 30, 6);
    c2.timeline_epoch = TimelineEpoch(1);
    let r2 = drive(&mut e, &Request::Command(c2));
    assert_eq!(r2.last().unwrap().outcome(), &Outcome::Completed);
}

// ================================================================ SC-3
//
// Fail-stop stickiness via a genuine scheduler/ObligationStore disagreement
// (tamper_obligations dropping the claim set of a key the scheduler still
// carries): every subsequent call -- including a DIFFERENT request type --
// must return the identical sticky outcome, snapshot() must be refused, and
// only restoring an earlier (pre-tamper) snapshot recovers a live engine.

#[test]
fn sc3_store_invariant_violated_is_sticky_across_different_requests_and_blocks_snapshot() {
    let mut e = engine(4, vec![100]);
    // A pre-tamper snapshot for the recovery leg.
    let clean_snapshot = e.snapshot().expect("clean engine snapshots fine");

    let key = e.obligations().keys().next().unwrap().clone();
    // Drop the claim set the scheduler still expects to find at extraction.
    fixture::tamper_obligations(&mut e, &key, None);

    let advance_req = advance(100);
    let r1 = e.process(&advance_req);
    match r1.outcome() {
        Outcome::StoreInvariantViolated(_) => {}
        other => panic!("expected StoreInvariantViolated, got {other:?}"),
    }
    let sticky = r1.outcome().clone();

    // Repeat the SAME request: must return the identical sticky outcome.
    let r2 = e.process(&advance_req);
    assert_eq!(r2.outcome(), &sticky);

    // Present a DIFFERENT request kind entirely: must still return the same
    // sticky outcome (fail-stop precedes even the ActiveRequest/P0 checks).
    let cmd_req = Request::Command(cmd("after-faulted", 200, 1));
    let r3 = e.process(&cmd_req);
    assert_eq!(
        r3.outcome(),
        &sticky,
        "fail-stop must be checked before any per-request logic, for any request shape"
    );

    // No snapshot may be published from a fail-stopped instance.
    assert_eq!(e.snapshot().unwrap_err(), SnapshotRefused::FailStopped);

    // Only restoring the prior committed snapshot recovers a live engine.
    let mut f2 = standard();
    let recovered = Engine::restore(clean_snapshot, &f2.profile)
        .expect("restoring the pre-tamper snapshot must succeed");
    let r4 = recovered.clone().process(&advance_req);
    assert_ne!(
        r4.outcome(),
        &sticky,
        "the recovered engine must not itself be fail-stopped"
    );
    let _ = &mut f2; // silence unused-mut if the compiler flags it
}

// ================================================================ SC-4
//
// Duplicate/conflicting WorkKey staged directly via snapshot_stage_raw at the
// real frontier ordinal (per the campaign's own recorded observation: using
// the true frontier ordinal, not an arbitrary one, so staging actually
// happens instead of silently reporting success for an out-of-window
// ordinal). Confirms restore fails closed if staging residue is present.

#[test]
fn sc4_snapshot_with_staged_residue_at_real_frontier_refuses_restore() {
    let mut e = engine(4, vec![]);
    // Drive once so there is committed history to snapshot over.
    drive(&mut e, &Request::Command(cmd("seed", 5, 1)));

    // Stage directly into the LIVE engine's timeline at the real frontier
    // ordinal (per the campaign's own recorded observation: using the true
    // frontier ordinal, not an arbitrary one, so staging actually happens
    // instead of silently reporting success for an out-of-window ordinal).
    let real_ordinal = e.timeline_frontier_ordinal();
    let envelope = cmd("staged-direct", 10, 2).envelope_at(real_ordinal);
    let staged_ok = fixture::stage_raw(&mut e, envelope);
    assert!(staged_ok, "staging at the engine's real frontier ordinal must succeed");
    assert!(!e.staging_is_clean(), "the engine must now observe staged residue");

    // Snapshot the engine WITH the staged residue present, then confirm
    // restore fails closed on it.
    let snap = e.snapshot().expect("snapshotting staged-but-unfenced residue is not itself refused");
    let mut f2 = standard();
    let result = Engine::restore(snap, &f2.profile);
    assert_eq!(
        result.unwrap_err(),
        RestoreError::TimelineStagingPresent,
        "restore must fail closed on any staged-but-unfenced residue (Revision 2 §7, step 2b)"
    );
    let _ = &mut f2;
}

// ================================================================ SC-5
//
// Backpressure / boundedness: a command-triggered rule that schedules more
// re-evaluation obligations in one wave than `max_enqueue_per_wave` allows.
// Confirms the typed `WaveRejection::SemanticCap` fires, the wave rejects
// atomically (V2-06: earlier waves/ingress stay committed, nothing from the
// offending wave partially applies), and the engine is NOT fail-stopped --
// it stays live for later requests.

#[test]
fn sc5_max_enqueue_per_wave_semantic_cap_rejects_atomically_without_fail_stop() {
    let mut f = standard();
    let rule = with_schedule(
        with_schedule(
            on_command(
                "rule.fanout",
                "cmd.fanout",
                vec![emit("i", "state.energy", spark_engine::rules::Update::Add(lit(1)))],
            ),
            reevaluate_after("s1", 1, "work.eval", "rule.fanout"),
        ),
        reevaluate_after("s2", 1, "work.eval", "rule.fanout"),
    );
    // max_enqueue_per_wave = 1, but the command's own wave tries to enqueue 2
    // schedule ops (s1, s2) at once.
    let mut b = budgets(10);
    b.max_enqueue_per_wave = 1;
    let genesis = f.genesis(b, vec![rule], vec![]);
    let mut e = Engine::genesis(genesis).unwrap();

    let fanout_cmd = command_request("fanout-cmd", 5, 1, "cmd.fanout", bron(), vec![], vec![]);
    let results = drive(&mut e, &Request::Command(fanout_cmd));
    let last = results.last().unwrap();
    assert_eq!(last.outcome(), &Outcome::Completed, "the horizon completes; the wave rejection is reported per-cohort, not a fail-stop");

    let all_reports: Vec<_> = last.reports().to_vec();
    let cohort = all_reports
        .iter()
        .find(|r| {
            matches!(
                r.outcome,
                CohortOutcome::Rejected {
                    rejection: WaveRejection::SemanticCap { cap: "max_enqueue_per_wave", .. },
                    ..
                }
            )
        })
        .unwrap_or_else(|| panic!("expected a max_enqueue_per_wave SemanticCap rejection, got {all_reports:?}"));
    match &cohort.outcome {
        CohortOutcome::Rejected {
            wave,
            rejection: WaveRejection::SemanticCap { cap, observed, bound },
        } => {
            assert_eq!(*cap, "max_enqueue_per_wave");
            assert_eq!(*observed, 2, "both schedule ops counted toward the cap");
            assert_eq!(*bound, 1);
            assert_eq!(*wave, 0, "the command's own ingress wave (wave 0) is what overflows");
        }
        other => panic!("{other:?}"),
    }
    // Nothing from the rejected wave applied: no re-evaluation obligation
    // was actually scheduled (the store has no obligations pending).
    assert!(
        e.obligations().is_empty(),
        "the rejected wave's enqueue attempts must not partially land"
    );
    // But the ingress effect that happens before wave evaluation (the direct
    // command emit "i") -- confirm whether it is retained or rolled back,
    // since V2-06 says waves BEFORE the rejected one stay committed, and the
    // command's own emit is itself wave 0's content, not a prior wave.
    // Either way, the engine must remain live and usable afterward:
    assert!(!e.snapshot().is_err(), "engine must not be fail-stopped by an ordinary semantic-cap rejection");
    let r2 = drive(&mut e, &Request::Command(cmd("after-cap", 6, 2)));
    assert_eq!(r2.last().unwrap().outcome(), &Outcome::Completed, "engine remains usable for later commands");
}

// ================================================================ SC-6
//
// A genuine WorkKey conflict staged directly via `schedule_record` (the
// commutative mirror of `Scheduler::schedule`): two obligation records under
// the identical `WorkKey` with distinct emission identities (hence distinct
// canonical payload hashes). Confirms extraction and cohort reporting treat
// it as `ConflictOnly` -- nothing committed for that key, no rule chosen
// arbitrarily -- and that the contested state survives a snapshot/restore
// round trip byte-for-byte (a contested claim set is a legitimate state, not
// corruption).

#[test]
fn sc6_conflicting_work_key_extracts_and_reports_conflict_only_and_survives_restore() {
    // A donor engine just to mint one real WorkKey and a base record's
    // originating artifact hashes so the two records resolve in the lineage.
    let mut f = standard();
    let donor = f.engine(budgets(4), poke_rules(), vec![initial("rule.x", &bron(), 50, "work.eval")]);
    let key = donor.obligations().keys().next().unwrap().clone();
    let template = donor
        .obligations()
        .get(&key)
        .unwrap()
        .sole_record()
        .unwrap()
        .clone();

    let ruleset_content_hash = match template.mode() {
        ObligationMode::RuleReEvaluation {
            ruleset_content_hash,
            ..
        } => ruleset_content_hash.clone(),
        other => panic!("expected RuleReEvaluation mode, got {other:?}"),
    };
    let mut f2 = standard();
    let mut e = f2.engine(budgets(4), poke_rules(), vec![]);
    let a = fixture::reevaluation_record(
        key.clone(),
        template.creator_rule_id().clone(),
        template.creator_rule_fingerprint().clone(),
        ruleset_content_hash.clone(),
        spark_core::hash::hash_bytes(b"sonnet-c emission A"),
        template.creator_behavior_artifact_hash().clone(),
    );
    let b = fixture::reevaluation_record(
        key.clone(),
        template.creator_rule_id().clone(),
        template.creator_rule_fingerprint().clone(),
        ruleset_content_hash.clone(),
        spark_core::hash::hash_bytes(b"sonnet-c emission B"),
        template.creator_behavior_artifact_hash().clone(),
    );
    fixture::schedule_record(&mut e, a);
    fixture::schedule_record(&mut e, b);

    let claim_set = e.obligations().get(&key).unwrap();
    assert!(claim_set.is_contested(), "two distinct payloads under one key must contest");

    let results = drive(&mut e, &advance(50));
    assert_eq!(results.last().unwrap().outcome(), &Outcome::Completed);
    let all_reports = reports(&results);
    let conflict_report = all_reports
        .iter()
        .find(|r| r.kind == CohortKind::ConflictOnly)
        .unwrap_or_else(|| panic!("expected a ConflictOnly cohort report, got {all_reports:?}"));
    assert_eq!(conflict_report.outcome, CohortOutcome::ConflictOnly);
    assert!(
        !conflict_report.conflicts.is_empty(),
        "the conflict section must name the contested key"
    );
    assert!(
        value(&e, "state.stress", &bron()).is_none(),
        "neither contested record's effect may be arbitrarily applied"
    );
    assert!(!e.snapshot().is_err(), "a contested claim set does not fail-stop the engine");

    // Round-trip: the contested state must survive snapshot/restore exactly.
    let mut e2 = f2.engine(budgets(4), poke_rules(), vec![]);
    let a2 = fixture::reevaluation_record(
        key.clone(),
        template.creator_rule_id().clone(),
        template.creator_rule_fingerprint().clone(),
        ruleset_content_hash.clone(),
        spark_core::hash::hash_bytes(b"sonnet-c emission A"),
        template.creator_behavior_artifact_hash().clone(),
    );
    let b2 = fixture::reevaluation_record(
        key.clone(),
        template.creator_rule_id().clone(),
        template.creator_rule_fingerprint().clone(),
        ruleset_content_hash.clone(),
        spark_core::hash::hash_bytes(b"sonnet-c emission B"),
        template.creator_behavior_artifact_hash().clone(),
    );
    fixture::schedule_record(&mut e2, a2);
    fixture::schedule_record(&mut e2, b2);
    let snap = e2.snapshot().unwrap();
    let mut f3 = standard();
    let restored = Engine::restore(snap, &f3.profile).expect("restoring a contested-but-consistent claim set must succeed");
    assert_eq!(restored.stable_boundary_digest(), e2.stable_boundary_digest());
    assert!(restored.obligations().get(&key).unwrap().is_contested());
    let _ = &mut f3;
}

// ================================================================ SC-7
//
// `reconstruct_completed` given a history whose command list includes one
// that, replayed against the declared genesis, does NOT finalize (a
// deliberately corrupted/adversarial history: a real `CompletedHistory` can
// never legitimately contain a not-finalized command, since only actually
// finalized commands are recorded, so this is testing the refusal path, not
// a real workflow). Confirms replay refuses closed (`StepRefused`) rather
// than silently producing a divergent engine, and does not confuse this
// with an ordinary `Diverged` digest mismatch.

#[test]
fn sc7_reconstruct_completed_refuses_a_history_with_a_non_finalizing_command() {
    let mut f = standard();
    let genesis = f.genesis(budgets(9), poke_rules(), vec![]);
    // A command stamped for a timeline epoch that will never match genesis's
    // epoch 0, so finalize_command must refuse it (WrongTimelineEpoch) during
    // replay instead of completing.
    let mut bad = cmd("bad", 10, 1);
    bad.timeline_epoch = TimelineEpoch(7);
    let history = CompletedHistory {
        commands: vec![bad],
        resets: vec![],
        frontier: LogicalTime(10),
        recorded_history_digest: spark_core::hash::hash_bytes(b"irrelevant, refused before digest check"),
        recorded_stable_boundary_digest: spark_core::hash::hash_bytes(b"irrelevant, refused before digest check"),
    };
    let err = Engine::reconstruct_completed(genesis, &history).unwrap_err();
    match err {
        ReplayError::StepRefused { step } => {
            assert!(step.contains("command 0"), "step should name the offending command: {step}");
        }
        other => panic!("expected StepRefused, got {other:?} -- a non-finalizing command must not be silently accepted or reported as a plain digest Diverged"),
    }
}
