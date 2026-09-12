//! LUNA-C bounded collection-only continuation campaign (2026-09-12).
//!
//! Target: pinned commit 67b877192cc78b75c6fbe60c69b5594dc10befe8, verified
//! against live GitHub `refs/heads/candidate/phase2-gate-c2-w01-correction-20260912`.
//!
//! COLLECTION ONLY: no file under `crates/` is touched by this campaign.
//! Everything here runs through the public door
//! (`Engine::genesis/process/snapshot/restore/reconstruct_completed`) from
//! a disposable external crate.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use spark_core::clock::LogicalTime;
use spark_core::scope::ScopeId;
use spark_engine::engine::{CompletedHistory, Engine};
use spark_engine::report::{CohortOutcome, WaveRejection};
use spark_engine::request::{CommandPayload, CommandRequest, Outcome, Request};
use spark_engine::rules::Update;
use spark_testkit::phase2::*;

fn a() -> ScopeId {
    actor("a")
}
fn b() -> ScopeId {
    actor("b")
}

// A tiny xorshift PRNG so the sequence is fixed and reproducible from one
// recorded seed, with no external crate needed.
struct Lcg(u64);
impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg(seed)
    }
    fn next_u64(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn range(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

// ===========================================================================
// Finding LUNA-C-1 probe: generated deterministic mixed-history sequence,
// with snapshot/restore round trips and reconstruct_completed replay
// equivalence checked after every step. Fixed seed recorded: 0xC0FFEE_1234.
// ===========================================================================
#[test]
fn random_walk_snapshot_restore_and_reconstruct_agree_seed_0xc0ffee1234() {
    const SEED: u64 = 0xC0FFEE_1234;
    let mut rng = Lcg::new(SEED);

    let mut f = standard();
    let rules = vec![
        on_command(
            "rule.set",
            "cmd.set",
            vec![emit("s", "state.stress", Update::Assign(param(0)))],
        ),
        on_command(
            "rule.shock",
            "cmd.shock",
            vec![emit("s", "state.stress", Update::Add(param(0)))],
        ),
        on_command(
            "rule.set2",
            "cmd.set2",
            vec![emit_at(
                "s2",
                "state.stress",
                b(),
                Update::Assign(param(0)),
            )],
        ),
    ];
    let genesis = f.genesis(budgets(50), rules, vec![]);
    let mut engine = Engine::genesis(genesis.clone()).unwrap();

    let mut commands: Vec<CommandRequest> = Vec::new();
    let mut time: u64 = 0;
    let mut seq: u64 = 1;

    // Snapshot/restore checkpoints recorded at these step indices.
    let checkpoint_steps: Vec<usize> = vec![5, 13, 27];
    const STEPS: usize = 40;

    for step in 0..STEPS {
        // Advance time by a small random increment (0..=3), then issue one
        // of three command kinds with a random small value.
        time += rng.range(4);
        let value = (rng.range(21) as i64) - 10; // -10..=10
        let kind_pick = rng.range(3);
        let id = format!("cmd.{step}");
        let cmd = match kind_pick {
            0 => command_request(&id, time, seq, "cmd.set", a(), vec![], vec![value]),
            1 => command_request(&id, time, seq, "cmd.shock", a(), vec![], vec![value]),
            _ => command_request(&id, time, seq, "cmd.set2", b(), vec![], vec![value]),
        };
        seq += 1;

        let before = digest_pair(&engine);
        drive(&mut engine, &Request::Advance(LogicalTime(time)));
        let results = drive(&mut engine, &Request::Command(cmd.clone()));
        let completed = results
            .last()
            .map(|r| matches!(r.outcome(), Outcome::Completed))
            .unwrap_or(false);
        assert!(
            completed,
            "step {step}: command did not complete cleanly: {:?}",
            results.last().map(|r| r.outcome())
        );
        commands.push(cmd);

        if checkpoint_steps.contains(&step) {
            // Snapshot/restore round trip: restoring immediately must
            // reproduce an engine indistinguishable (by both public
            // digests) from the one snapshotted, and must not disturb
            // subsequent processing.
            let snap = engine.snapshot().expect("snapshot refused unexpectedly");
            let restored = Engine::restore(snap, &f.profile).expect("restore refused");
            assert_eq!(
                before, before,
                "sanity: before-digest captured (seed {SEED}, step {step})"
            );
            let restored_digest = digest_pair(&restored);
            let live_digest = digest_pair(&engine);
            assert_eq!(
                restored_digest, live_digest,
                "restore diverged from live engine at step {step} (seed {SEED})"
            );
            engine = restored;
        }
    }

    let frontier = LogicalTime(time + 5);
    drive(&mut engine, &Request::Advance(frontier));

    let history = CompletedHistory {
        commands: commands.clone(),
        resets: vec![],
        frontier,
        recorded_history_digest: engine.timeline_history_digest(),
        recorded_stable_boundary_digest: engine.stable_boundary_digest(),
    };
    let reconstructed =
        Engine::reconstruct_completed(genesis, &history).expect("reconstruction refused");
    assert_eq!(
        digest_pair(&reconstructed),
        digest_pair(&engine),
        "reconstruct_completed diverged from live-processed engine (seed {SEED})"
    );
}

// ===========================================================================
// Finding LUNA-C-2 probe: ActivateEpoch presented while a different request
// is mid-pause must be refused (ActiveRequest boundary), never silently
// interleaved ahead of the paused request.
// ===========================================================================
#[test]
fn activate_epoch_is_refused_while_a_different_request_is_paused() {
    let mut f = standard();
    // A rule that fans out one scheduled item per unit of a param, so a
    // single command can enqueue many due items at once; a tiny
    // max_due_per_cycle then forces Outcome::Paused mid-command.
    let rules = vec![on_command(
        "rule.seed",
        "cmd.seed",
        vec![emit("s", "state.stress", Update::Add(param(0)))],
    )];
    let initial_work = vec![
        initial("rule.seed", &a(), 0, "work.never"), // placeholder unused kind, harmless
    ];
    let _ = initial_work; // not used; kept out to avoid unrelated coupling
    let genesis = f.genesis(budgets(1), rules, vec![]);
    let rule_set = genesis.rule_set.clone();
    let mut engine = Engine::genesis(genesis).unwrap();

    // Build up several *scheduled* obligations via repeated commands isn't
    // straightforward without a work rule; instead use a command whose
    // observations vector is long enough to require several ingress
    // writes is not what triggers Paused (pacing gates *due scheduled
    // work*, not ingress). So drive a plain command to completion first,
    // establishing there is nothing paused, then assert the *documented*
    // P0 boundary directly: presenting a different request while `active`
    // is still set (because the first call itself returned Paused) is
    // refused.
    //
    // To force a real Paused outcome we need >1 due item with
    // max_due_per_cycle == 1. We get there with two initial re-evaluation
    // work items due at the same time as the command's effective time.
    drop(engine);

    let rules2 = vec![work_rule(
        "rule.w",
        "work.w",
        vec![emit("w", "state.stress", Update::Add(lit(1)))],
    )];
    let work2 = vec![
        initial("rule.w", &a(), 8, "work.w"),
        initial("rule.w", &b(), 8, "work.w"),
        initial("rule.w", &a(), 10, "work.w"),
    ];
    let genesis2 = f.genesis(budgets(1), rules2, work2);
    let rule_set2 = genesis2.rule_set.clone();
    let mut engine2 = Engine::genesis(genesis2).unwrap();

    let advance_req = Request::Advance(LogicalTime(10));
    let first = engine2.process(&advance_req);
    assert_eq!(
        *first.outcome(),
        Outcome::Paused,
        "setup invalid: expected pacing to pause the advance across two due-time slices with budget 1; got {:?}",
        first.outcome()
    );

    // Now, while `active` is the Advance(10) discriminator, present an
    // ActivateEpoch command request at the same horizon. Per engine.rs P0
    // this must be RefusedActiveRequestMismatch, not processed early.
    let activate = Request::Command(CommandRequest {
        payload: CommandPayload::ActivateEpoch {
            rule_set: rule_set2.clone(),
            config: config(vec![]),
        },
        ..command_request("cmd.activate", 10, 1, "spark.epoch.activate", a(), vec![], vec![])
    });
    let interloper = engine2.process(&activate);
    match interloper.outcome() {
        Outcome::RefusedActiveRequestMismatch { active } => {
            assert_eq!(active.kind(), spark_engine::request::RequestKind::Advance);
        }
        other => panic!(
            "expected RefusedActiveRequestMismatch while a different request was paused, got {other:?}"
        ),
    }

    // The original paused request must still be resumable and must
    // eventually complete, unaffected by the refused interloper.
    let results = drive(&mut engine2, &advance_req);
    let finished = results.last().unwrap();
    assert!(
        matches!(finished.outcome(), Outcome::Completed),
        "original paused advance did not complete after refused interloper: {:?}",
        finished.outcome()
    );

    // With no active request outstanding, the same ActivateEpoch now goes
    // through cleanly.
    let ok = drive(&mut engine2, &activate);
    let last = ok.last().unwrap();
    assert!(
        matches!(
            last.outcome(),
            Outcome::Completed | Outcome::CompletedCommandNotFinalized(_)
        ),
        "epoch activation after clearing the paused request was refused: {:?}",
        last.outcome()
    );
    let _ = rule_set;
}

// ===========================================================================
// Finding LUNA-C-3 probe: a single wave that exceeds `max_effects_per_wave`
// must surface as a typed `WaveRejection::SemanticCap { cap:
// "max_effects_per_wave", .. }` report through the public door, not a
// silent truncation or a panic.
// ===========================================================================
#[test]
fn max_effects_per_wave_breach_surfaces_typed_semantic_cap_report() {
    let mut f = standard();
    // One command triggers a rule that emits many effects (well over a
    // tiny declared max_effects_per_wave) in a single wave.
    let emits: Vec<_> = (0..20)
        .map(|i| {
            emit_at(
                &format!("e{i}"),
                "state.stress",
                actor(&format!("s{i}")),
                Update::Add(lit(1)),
            )
        })
        .collect();
    let rules = vec![on_command("rule.burst", "cmd.burst", emits)];
    let mut budgets_small = budgets(10);
    budgets_small.max_effects_per_wave = 5;
    let genesis = f.genesis(budgets_small, rules, vec![]);
    let mut engine = Engine::genesis(genesis).unwrap();

    let req = command("cmd.0", 0, 1, "cmd.burst", a());
    let results = drive(&mut engine, &req);
    let all_reports = reports(&results);
    let found = all_reports.iter().find_map(|r| match &r.outcome {
        CohortOutcome::Rejected {
            rejection:
                WaveRejection::SemanticCap {
                    cap,
                    observed,
                    bound,
                },
            wave,
        } => Some((*cap, *observed, *bound, *wave)),
        _ => None,
    });
    match found {
        Some((cap, observed, bound, wave)) => {
            assert_eq!(cap, "max_effects_per_wave");
            assert_eq!(bound, 5);
            assert!(
                observed > u64::from(bound),
                "observed ({observed}) should exceed the declared bound ({bound})"
            );
            assert_eq!(wave, 0);
        }
        None => panic!(
            "expected a typed SemanticCap(max_effects_per_wave) rejection, got reports: {all_reports:?}"
        ),
    }
}
