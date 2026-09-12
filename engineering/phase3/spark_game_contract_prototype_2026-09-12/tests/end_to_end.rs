//! Executable evidence for the S.P.A.R.K.–G.A.M.E. Integration Contract V1
//! candidate, against a **fake** host.
//!
//! **Scope limit, stated once and meant throughout:** a passing run here is
//! preparatory evidence about the S.P.A.R.K. side of the contract. It is **not**
//! proof of G.A.M.E. integration, and no test in this file may be cited as one.
//! A real adapter in the G.A.M.E. repository against a pinned accepted
//! S.P.A.R.K. artifact is Gate C4 and is not authorized.

use spark_core::clock::LogicalTime;
use spark_core::scope::ScopeId;
use spark_core::value::CanonicalValue;
use spark_engine::engine::{CompletedHistory, Engine};
use spark_engine::request::Request;

use spark_game_contract_prototype::fake_host::{FakeHost, FakeWorld};
use spark_game_contract_prototype::fixture::{self, advance, host_command};
use spark_game_contract_prototype::*;

// ------------------------------------------------------------------ helpers

fn open_device() -> (PrototypeDevice, SessionDescriptor) {
    let (engine, profile) = fixture::device_engine();
    PrototypeDevice::open(
        engine,
        profile,
        fixture::entity_map(),
        fixture::intent_channels(),
        CONTRACT_VERSION,
        DeclaredBounds::default(),
    )
    .expect("the fixture session opens")
}

/// Contract §4.4: pull results by re-presenting the same request.
fn present_to_completion(d: &mut PrototypeDevice, r: &Request) -> DeviceResponse {
    for _ in 0..10_000 {
        match d.present(r) {
            DeviceResponse::Paused => continue,
            other => return other,
        }
    }
    panic!("the request never terminated");
}

fn expect_completed(resp: DeviceResponse) -> (IntentBatch, Vec<CohortRefusal>) {
    match resp {
        DeviceResponse::Completed {
            batch,
            refusals,
            capacity_exceeded,
        } => {
            assert!(
                !capacity_exceeded,
                "no fixture batch should exceed the declared capacity hint"
            );
            (batch, refusals)
        }
        other => panic!("expected a completed boundary, got {other:?}"),
    }
}

fn cell(e: &Engine, definition: &str, scope: &ScopeId) -> Option<i64> {
    e.state()
        .get(&fixture::profile_id(), &fixture::def(definition), scope)
        .and_then(|c| match &c.value {
            CanonicalValue::Int(i) => Some(*i),
            _ => None,
        })
}

// ===================================================================== E-1
// Contract §3: version, profile and capability negotiation.

#[test]
fn e1_session_negotiation_is_fail_closed_on_major_and_downgrades_minor() {
    let (_d, session) = open_device();
    assert_eq!(session.negotiated_version, CONTRACT_VERSION);
    assert!(session.capabilities.contains(&Capability::IntentV1));
    // §3.3: `persist.v1` must never be advertised — there is no persistence
    // backend behind the engine's snapshot (§9.4).
    assert!(
        !session
            .capabilities
            .iter()
            .any(|c| c.tag().starts_with("persist")),
        "E-1a: durability must not be advertised"
    );

    // A different major is refused and no session opens.
    let (engine, profile) = fixture::device_engine();
    let err = PrototypeDevice::open(
        engine,
        profile,
        fixture::entity_map(),
        fixture::intent_channels(),
        ProtocolVersion { major: 2, minor: 0 },
        DeclaredBounds::default(),
    )
    .err()
    .expect("E-1b: an unsupported major must refuse");
    assert_eq!(
        err,
        Rejected::UnsupportedProtocolVersion { supported_major: 1 }
    );

    // A higher minor is accepted at the device's own minor.
    let (engine, profile) = fixture::device_engine();
    let (d, s) = PrototypeDevice::open(
        engine,
        profile,
        fixture::entity_map(),
        fixture::intent_channels(),
        ProtocolVersion { major: 1, minor: 9 },
        DeclaredBounds::default(),
    )
    .expect("E-1c: a higher minor is accepted");
    assert_eq!(s.negotiated_version.minor, CONTRACT_VERSION.minor);
    assert_eq!(d.negotiated_version().minor, CONTRACT_VERSION.minor);

    // §3.2: a message naming a different profile pair is refused.
    let (d2, s2) = open_device();
    assert!(d2
        .check_profile(&s2.manifest_content_hash, &s2.activation_hash)
        .is_ok());
    let wrong = spark_core::hash::hash_bytes(b"not this profile");
    assert_eq!(
        d2.check_profile(&wrong, &s2.activation_hash),
        Err(Rejected::ProfileMismatch)
    );
    println!("E-1 PASS: version fail-closed on major, minor downgraded, profile pair enforced");
}

// ===================================================================== E-2
// Protocol §5 Gate C4 first proof, end to end across the seam.

/// Drives the whole canonical sequence and returns the final world and device.
fn run_first_proof(world: FakeWorld) -> (PrototypeDevice, FakeHost, Vec<IntentBatch>) {
    let (mut d, _s) = open_device();
    let mut host = FakeHost::new(world);
    let mut batches = Vec::new();

    // 1-3. G.A.M.E. supplies physical/economic observations: a drought and a
    //      reduced food supply at the settlement.
    let tick = host_command(
        "cmd.tick.1",
        10,
        1,
        "cmd.world_tick",
        fixture::settlement(),
        &[("world.drought", 30), ("world.food_supply", 20)],
    );
    let (batch, refusals) = expect_completed(present_to_completion(&mut d, &tick));
    assert!(
        refusals.is_empty(),
        "E-2a: no cohort refusal expected: {refusals:?}"
    );
    batches.push(batch.clone());

    // 4. S.P.A.R.K. supplies an advisory intent; G.A.M.E. decides.
    let report = host.deliver(&batch);
    assert_eq!(report.correlation, batch.correlation);
    assert_eq!(report.batch_digest, batch.batch_digest);

    // 5-6. Confirmed outcome re-enters only as a typed observation.
    let confirm = host_command(
        "cmd.confirm.1",
        11,
        2,
        "cmd.confirm",
        fixture::region(),
        &[("world.wildlife", host.world.wildlife)],
    );
    let (batch2, refusals2) = expect_completed(present_to_completion(&mut d, &confirm));
    assert!(refusals2.is_empty(), "E-2b: {refusals2:?}");
    batches.push(batch2);
    (d, host, batches)
}

#[test]
fn e2_canonical_first_proof_runs_end_to_end_across_the_seam() {
    let (d, host, batches) = run_first_proof(FakeWorld::default());
    let e = d.engine();

    // drought + food supply -> affordability: 3*30 - 1*20 = 70.
    assert_eq!(
        cell(e, "econ.food_price", &fixture::settlement()),
        Some(70),
        "E-2c: affordability"
    );
    // -> household exposure
    assert_eq!(
        cell(e, "household.exposure", &fixture::household()),
        Some(70),
        "E-2d: household exposure"
    );
    // -> actor stress
    assert_eq!(
        cell(e, "actor.stress", &fixture::actor()),
        Some(70),
        "E-2e: actor stress"
    );
    // -> actor choice, exported as advisory intents on two channels: one
    //    addressed to an ENTITY (the actor), one to an AFFILIATION (the
    //    settlement). Gate C3 requires both reference kinds.
    assert_eq!(batches[0].intents.len(), 2, "E-2f: two advisory intents");
    let ration = batches[0]
        .intents
        .iter()
        .find(|i| i.channel.as_str() == "intent.ration")
        .expect("E-2f1: the affiliation-addressed intent");
    assert_eq!(
        ration.subject,
        IntentSubject::Affiliation(ExternalAffiliationRef("game:settlement/1".to_string())),
        "E-2f2: a settlement-scoped intent addresses an affiliation, not an entity"
    );
    let intent = batches[0]
        .intents
        .iter()
        .find(|i| i.channel.as_str() == "intent.hunt")
        .expect("E-2f3: the entity-addressed intent");
    assert_eq!(
        intent.subject,
        IntentSubject::Entity(ExternalEntityRef("game:actor/1".to_string()))
    );
    assert_eq!(intent.value, CanonicalValue::Int(1));

    // -> G.A.M.E. alone executed the hunt, in its own world numbers
    assert_eq!(host.world.executed_hunts, 1, "E-2g: the host executed once");
    assert_eq!(
        host.world.wildlife, 90,
        "E-2h: wildlife declined by host action"
    );

    // -> wildlife decline re-entered as host truth and produced neighbouring
    //    settlement feedback: 100 - 90 = 10.
    assert_eq!(
        cell(e, "neighbour.pressure", &fixture::neighbour()),
        Some(10),
        "E-2i: neighbouring-settlement feedback"
    );
    assert_eq!(
        cell(e, "actor.stress", &fixture::neighbour_actor()),
        Some(10),
        "E-2j: feedback reached the neighbouring actor"
    );

    // The second boundary produced no new intent (stress 10 is below the
    // fixture threshold of 40), so the batch is empty rather than absent.
    assert!(
        batches[1].intents.is_empty(),
        "E-2k: no intent below threshold"
    );
    println!(
        "E-2 PASS: drought -> affordability -> exposure -> actor choice -> hunting -> \
         wildlife decline -> neighbouring-settlement feedback, across the real seam \
         (fake host: preparatory evidence only)"
    );
}

// ===================================================================== E-3
// Contract §2/§8: G.A.M.E. alone decides legality. A refused intent moves
// nothing in the world, and S.P.A.R.K. learns that only through observation.

#[test]
fn e3_a_refused_intent_changes_nothing_in_the_world() {
    let world = FakeWorld {
        forbidden: vec![ExternalEntityRef("game:actor/1".to_string())],
        ..FakeWorld::default()
    };
    let (d, host, batches) = run_first_proof(world);

    assert_eq!(host.world.executed_hunts, 0, "E-3a: nothing was executed");
    assert_eq!(host.world.wildlife, 100, "E-3b: the world is unchanged");
    assert_eq!(
        batches[0]
            .intents
            .iter()
            .filter(|i| i.channel.as_str() == "intent.hunt")
            .count(),
        1,
        "E-3c: SPARK still advised the hunt — the refusal is the host's, not SPARK's"
    );

    // And S.P.A.R.K.'s own causal state reflects the *observed* world, not the
    // advice: wildlife never declined, so there is no neighbouring pressure.
    assert_eq!(
        cell(d.engine(), "neighbour.pressure", &fixture::neighbour()),
        Some(0),
        "E-3d: no feedback from an action that never happened"
    );
    println!("E-3 PASS: a host rejection leaves the world and the feedback loop untouched");
}

#[test]
fn e3b_unknown_subject_and_unsupported_channel_are_named_rejections() {
    // The host knows neither the actor nor the settlement.
    let world = FakeWorld {
        known: vec![],
        known_affiliations: vec![],
        ..FakeWorld::default()
    };
    let (mut d, _s) = open_device();
    let mut host = FakeHost::new(world);
    let tick = host_command(
        "cmd.tick.1",
        10,
        1,
        "cmd.world_tick",
        fixture::settlement(),
        &[("world.drought", 30), ("world.food_supply", 20)],
    );
    let (batch, _) = expect_completed(present_to_completion(&mut d, &tick));
    let report = host.deliver(&batch);
    assert!(
        report.per_intent.iter().all(|d| *d
            == IntentDisposition::Rejected {
                reason: RejectionReason::UnknownSubject
            }),
        "E-3b1: every intent, entity- and affiliation-addressed alike, is an \
         UnknownSubject rejection: {:?}",
        report.per_intent
    );

    let world2 = FakeWorld {
        supported_channels: vec![],
        ..FakeWorld::default()
    };
    let mut host2 = FakeHost::new(world2);
    let report2 = host2.deliver(&batch);
    assert!(
        report2.per_intent.iter().all(|d| *d
            == IntentDisposition::Rejected {
                reason: RejectionReason::UnsupportedChannel
            }),
        "E-3b2: every intent is an UnsupportedChannel rejection: {:?}",
        report2.per_intent
    );
    println!("E-3b PASS: the rejection vocabulary is closed and names the actual cause");
}

// ===================================================================== E-4
// Contract §7.2: duplicate delivery is acknowledged, never re-applied.

#[test]
fn e4_duplicate_batch_delivery_applies_at_most_once() {
    let (mut d, _s) = open_device();
    let mut host = FakeHost::new(FakeWorld::default());
    let tick = host_command(
        "cmd.tick.1",
        10,
        1,
        "cmd.world_tick",
        fixture::settlement(),
        &[("world.drought", 30), ("world.food_supply", 20)],
    );
    let (batch, _) = expect_completed(present_to_completion(&mut d, &tick));

    let r1 = host.deliver(&batch);
    assert_eq!(host.world.wildlife, 90);
    assert_eq!(host.world.executed_hunts, 1);
    assert_eq!(host.world.executed_rations, 1);

    // Redeliver the identical batch three more times.
    for _ in 0..3 {
        let r = host.deliver(&batch);
        assert_eq!(
            r.per_intent, r1.per_intent,
            "E-4a: a redelivery is acknowledged identically"
        );
    }
    assert_eq!(
        host.world.wildlife, 90,
        "E-4b: the world moved exactly once"
    );
    assert_eq!(host.world.executed_hunts, 1, "E-4c: executed exactly once");
    assert_eq!(
        host.deliveries(&batch),
        4,
        "E-4d: all four deliveries were seen"
    );
    println!("E-4 PASS: at-most-once application keyed by (correlation, batch_digest)");
}

// ===================================================================== E-5
// Contract §4.2/§9.5: stale and out-of-order input is refused, never applied.

#[test]
fn e5_stale_horizon_is_refused_with_the_frontier_named() {
    let (mut d, _s) = open_device();
    let tick = host_command(
        "cmd.tick.1",
        20,
        1,
        "cmd.world_tick",
        fixture::settlement(),
        &[("world.drought", 10), ("world.food_supply", 10)],
    );
    expect_completed(present_to_completion(&mut d, &tick));
    let before = d.engine().state().canonical_state_digest();

    let late = host_command(
        "cmd.tick.late",
        5,
        2,
        "cmd.world_tick",
        fixture::settlement(),
        &[("world.drought", 99), ("world.food_supply", 0)],
    );
    let resp = present_to_completion(&mut d, &late);
    match resp {
        DeviceResponse::Rejected(Rejected::StaleHorizon { frontier, horizon }) => {
            assert_eq!(frontier, LogicalTime(20));
            assert_eq!(horizon, LogicalTime(5));
        }
        other => panic!("E-5a: expected StaleHorizon, got {other:?}"),
    }
    assert_eq!(
        d.engine().state().canonical_state_digest(),
        before,
        "E-5b: a stale command must change nothing"
    );
    println!("E-5 PASS: stale input refused with the frontier named; state byte-identical");
}

// ===================================================================== E-6
// Contract §4.4: one outstanding request, `Busy`, pull by re-presenting.

#[test]
fn e6_a_second_request_while_one_is_active_is_refused_busy() {
    // Two scheduled cohorts at distinct due times with budget 1 force a pause.
    let (engine, profile) = fixture::device_engine_paced(1, &[5, 6]);
    let (mut d, _s) = PrototypeDevice::open(
        engine,
        profile,
        fixture::entity_map(),
        fixture::intent_channels(),
        CONTRACT_VERSION,
        DeclaredBounds::default(),
    )
    .expect("session");

    let first = advance(6);
    assert_eq!(
        d.present(&first),
        DeviceResponse::Paused,
        "E-6a: the first call must pause"
    );

    // A *different* request while one is active.
    let other = advance(7);
    match d.present(&other) {
        DeviceResponse::Rejected(Rejected::Busy { active_horizon, .. }) => {
            assert_eq!(
                active_horizon,
                LogicalTime(6),
                "E-6b: Busy names the active horizon"
            );
        }
        other => panic!("E-6c: expected Busy, got {other:?}"),
    }

    // Pulling results means re-presenting the same request.
    let done = present_to_completion(&mut d, &first);
    assert!(
        matches!(done, DeviceResponse::Completed { .. }),
        "E-6d: the original request completes on re-presentation"
    );
    // The interloper must not have disturbed the active request's accumulation.
    let (batch, _) = expect_completed(done);
    assert_eq!(
        batch.intents.len(),
        2,
        "E-6e: both paced slices' intents survive the interleaved Busy refusal"
    );
    println!(
        "E-6 PASS: one outstanding request; Busy for another; results pulled by re-presenting"
    );
}

// ===================================================================== E-7
// Contract §9.1: bounded message size, refused whole.

#[test]
fn e7_oversized_observation_list_is_refused_whole() {
    let (engine, profile) = fixture::device_engine();
    let (mut d, _s) = PrototypeDevice::open(
        engine,
        profile,
        fixture::entity_map(),
        fixture::intent_channels(),
        CONTRACT_VERSION,
        DeclaredBounds {
            max_observations_per_command: 1,
            max_intents_per_batch: 256,
        },
    )
    .expect("session");
    let before = d.engine().state().canonical_state_digest();
    let big = host_command(
        "cmd.big",
        10,
        1,
        "cmd.world_tick",
        fixture::settlement(),
        &[("world.drought", 1), ("world.food_supply", 2)],
    );
    match d.present(&big) {
        DeviceResponse::Rejected(Rejected::MessageTooLarge { limit, observed }) => {
            assert_eq!((limit, observed), (1, 2));
        }
        other => panic!("E-7a: expected MessageTooLarge, got {other:?}"),
    }
    assert_eq!(
        d.engine().state().canonical_state_digest(),
        before,
        "E-7b: nothing may be partially processed"
    );
    println!("E-7 PASS: an oversized message is refused whole, with no partial processing");
}

// ===================================================================== E-8
// Contract §7/§10.1: determinism. Two independent runs agree byte for byte.

#[test]
fn e8_the_whole_proof_is_reproducible_across_two_in_process_runs() {
    let (d1, h1, b1) = run_first_proof(FakeWorld::default());
    let (d2, h2, b2) = run_first_proof(FakeWorld::default());

    assert_eq!(b1, b2, "E-8a: identical intent batches, including digests");
    assert_eq!(
        d1.engine().engine_state_digest(),
        d2.engine().engine_state_digest(),
        "E-8b: identical engine state digest"
    );
    assert_eq!(
        d1.engine().stable_boundary_digest(),
        d2.engine().stable_boundary_digest(),
        "E-8c: identical stable boundary digest"
    );
    assert_eq!(h1.world.wildlife, h2.world.wildlife);
    println!(
        "E-8 PASS: the whole sequence, including every batch digest, is reproducible \
         across two constructions IN ONE PROCESS. No cross-process and no cross-platform \
         determinism is claimed by this test."
    );
}

// ===================================================================== E-9
// Contract §10.1: deterministic canonical replay from completed history.

#[test]
fn e9_completed_history_replays_to_the_same_digests() {
    let (d, _h, _b) = run_first_proof(FakeWorld::default());
    let e = d.engine();

    let history = CompletedHistory {
        commands: e
            .finalized_commands()
            .iter()
            .map(|f| command_of(&f.envelope))
            .collect(),
        resets: e.epoch_resets().to_vec(),
        frontier: e.frontier(),
        recorded_history_digest: e.timeline_history_digest(),
        recorded_stable_boundary_digest: e.stable_boundary_digest(),
    };
    let (genesis_engine, _p) = fixture::device_engine();
    drop(genesis_engine);
    let replayed = Engine::reconstruct_completed(fixture::genesis_inputs(), &history)
        .expect("E-9a: completed history must replay");
    assert_eq!(
        replayed.engine_state_digest(),
        e.engine_state_digest(),
        "E-9b: replayed engine state digest must match"
    );
    assert_eq!(
        replayed.stable_boundary_digest(),
        e.stable_boundary_digest(),
        "E-9c: replayed stable boundary digest must match"
    );
    println!("E-9 PASS: completed-boundary replay reproduces both recorded digests");
}

/// Rebuild the `CommandRequest` the fixture issued from its finalized envelope.
/// The payload is reconstructed from the fixture's own knowledge of what it
/// sent, because an envelope stores only the payload hash.
fn command_of(
    envelope: &spark_core::timeline::SemanticCommandEnvelope,
) -> spark_engine::request::CommandRequest {
    let c = fixture::replayable_command(envelope.command_id.as_str())
        .expect("every fixture command is replayable");
    assert_eq!(
        c.canonical_payload_hash(),
        envelope.canonical_payload_hash,
        "the reconstructed payload must hash to the recorded payload hash"
    );
    c
}

// ==================================================================== E-10
// Contract §9.3/§9.4: interruption, restart, and the durability limit.

#[test]
fn e10_snapshot_restore_mid_sequence_reproduces_the_uninterrupted_run() {
    // Uninterrupted reference run.
    let (reference, _h, ref_batches) = run_first_proof(FakeWorld::default());

    // Interrupted run: snapshot after the first boundary, restore, continue.
    let (mut d, _s) = open_device();
    let mut host = FakeHost::new(FakeWorld::default());
    let tick = host_command(
        "cmd.tick.1",
        10,
        1,
        "cmd.world_tick",
        fixture::settlement(),
        &[("world.drought", 30), ("world.food_supply", 20)],
    );
    let (batch, _) = expect_completed(present_to_completion(&mut d, &tick));
    host.deliver(&batch);

    let snapshot = d
        .engine()
        .snapshot()
        .expect("E-10a: a stable boundary publishes a snapshot");
    let restored = Engine::restore(snapshot, d.profile()).expect("E-10b: restore");
    let (mut d2, _s2) = PrototypeDevice::open(
        restored,
        d.profile().clone(),
        fixture::entity_map(),
        fixture::intent_channels(),
        CONTRACT_VERSION,
        DeclaredBounds::default(),
    )
    .expect("session");

    let confirm = host_command(
        "cmd.confirm.1",
        11,
        2,
        "cmd.confirm",
        fixture::region(),
        &[("world.wildlife", host.world.wildlife)],
    );
    let (batch2, _) = expect_completed(present_to_completion(&mut d2, &confirm));

    assert_eq!(
        batch2, ref_batches[1],
        "E-10c: the post-restore boundary must be byte-identical to the uninterrupted one"
    );
    assert_eq!(
        d2.engine().engine_state_digest(),
        reference.engine().engine_state_digest(),
        "E-10d: the restored engine converges to the same state"
    );
    println!(
        "E-10 PASS: snapshot/restore across a boundary reproduces the uninterrupted run. \
         LIMIT: the snapshot is an in-memory value; this is NOT durable process-recovery \
         evidence (contract §9.4)."
    );
}

#[test]
fn e11_re_presenting_a_completed_command_is_refused_not_re_executed() {
    // Contract §9.3, row 3: the host cannot re-derive a lost batch by
    // re-presenting a completed command. This is exactly why §9.4's durability
    // gap matters, and the test records that consequence rather than hiding it.
    let (mut d, _s) = open_device();
    let tick = host_command(
        "cmd.tick.1",
        10,
        1,
        "cmd.world_tick",
        fixture::settlement(),
        &[("world.drought", 30), ("world.food_supply", 20)],
    );
    let (_batch, _) = expect_completed(present_to_completion(&mut d, &tick));
    let before = d.engine().state().canonical_state_digest();

    let resp = present_to_completion(&mut d, &tick);
    match resp {
        DeviceResponse::CompletedWithRefusedCommand { batch, refusal, .. } => {
            // The horizon completed and the typed refusal is carried; because
            // nothing new committed, the batch is empty rather than absent.
            assert!(batch.intents.is_empty(), "E-11c: nothing new was committed");
            println!("E-11: carried refusal = {refusal:?}");
        }
        other => panic!("E-11a: expected a typed finalization refusal, got {other:?}"),
    }
    assert_eq!(
        d.engine().state().canonical_state_digest(),
        before,
        "E-11b: the refused resubmission must change nothing"
    );
    println!(
        "E-11 PASS: a completed command cannot be replayed through the door. \
         CONSEQUENCE: a host that loses a batch before applying it cannot re-derive it \
         (contract §9.3/§9.4)."
    );
}

// ==================================================================== E-12
// Contract §6.2/§4.3: a paced request spreads its committed cohorts over
// several `process` calls, and the batch is published only at the completed
// boundary. Every intent committed before a pause must still reach the host.
//
// This is a regression test for a real defect in an earlier revision of this
// prototype, which projected only from the completing `ProcessResult` and
// therefore silently dropped every intent committed before the last pause.

#[test]
fn e12_intents_committed_before_a_pause_are_not_lost() {
    // Heartbeat work at two distinct due times with budget 1: the request
    // pauses after the first due time's cohort, which has already committed an
    // intent-channel effect.
    let (engine, profile) = fixture::device_engine_paced(1, &[5, 6]);
    let (mut d, _s) = PrototypeDevice::open(
        engine,
        profile,
        fixture::entity_map(),
        fixture::intent_channels(),
        CONTRACT_VERSION,
        DeclaredBounds::default(),
    )
    .expect("session");

    let req = advance(6);
    let mut pauses = 0;
    let resp = loop {
        match d.present(&req) {
            DeviceResponse::Paused => {
                pauses += 1;
                continue;
            }
            other => break other,
        }
    };
    assert!(
        pauses >= 1,
        "E-12a: the probe needs an actually paced request"
    );
    let (batch, _) = expect_completed(resp);
    assert_eq!(
        batch.intents.len(),
        2,
        "E-12b: one intent per due time must survive, including the one \
         committed before the pause (got {})",
        batch.intents.len()
    );
    assert_eq!(
        batch.intents[0].canonical_time,
        LogicalTime(5),
        "E-12c: the first intent is the pre-pause one, in canonical order"
    );
    assert_eq!(batch.intents[1].canonical_time, LogicalTime(6));
    println!(
        "E-12 PASS: {pauses} pause(s); both paced slices' intents reached the host in \
         canonical order"
    );
}

// ==================================================================== E-13
// Contract §7.2: the at-most-once key carries the session's activation
// identity, so two engines activated from different profile content can never
// share a key even if they produce an identical request and an identical batch.

#[test]
fn e13_the_at_most_once_key_separates_distinct_activations() {
    let (mut d, session) = open_device();
    let tick = host_command(
        "cmd.tick.1",
        10,
        1,
        "cmd.world_tick",
        fixture::settlement(),
        &[("world.drought", 30), ("world.food_supply", 20)],
    );
    let (batch, _) = expect_completed(present_to_completion(&mut d, &tick));
    assert_eq!(
        batch.session_activation, session.activation_hash,
        "E-13a: the batch carries the session's activation identity"
    );

    // The same batch content under a different activation is a different key.
    let mut other = batch.clone();
    other.session_activation = spark_core::hash::hash_bytes(b"a different activation");
    assert_ne!(
        ApplicationLedger::key_of(&batch),
        ApplicationLedger::key_of(&other),
        "E-13b: differing only in activation must not collide"
    );

    // And a ledger that applied one must not consider the other already applied.
    let mut ledger = ApplicationLedger::default();
    assert_eq!(ledger.admit(&batch), ApplicationDecision::Apply);
    assert_eq!(
        ledger.admit(&other),
        ApplicationDecision::Apply,
        "E-13c: a different activation's batch is not deduplicated away"
    );
    assert_eq!(ledger.admit(&batch), ApplicationDecision::AlreadyApplied);
    println!("E-13 PASS: the at-most-once key is (activation, correlation, batch_digest)");
}

// ==================================================================== E-14
// Contract §8: the defer half of the outcome vocabulary. An independent
// reviewer recorded that `DeferReason` existed but that no test ever produced
// a `Deferred` disposition, so the surface was declared and unexercised.

#[test]
fn e14_a_deferred_intent_defers_without_moving_the_world() {
    let world = FakeWorld {
        busy: vec![ExternalEntityRef("game:actor/1".to_string())],
        ..FakeWorld::default()
    };
    let (mut d, _s) = open_device();
    let mut host = FakeHost::new(world);
    let tick = host_command(
        "cmd.tick.1",
        10,
        1,
        "cmd.world_tick",
        fixture::settlement(),
        &[("world.drought", 30), ("world.food_supply", 20)],
    );
    let (batch, _) = expect_completed(present_to_completion(&mut d, &tick));
    let report = host.deliver(&batch);

    let hunt_index = batch
        .intents
        .iter()
        .position(|i| i.channel.as_str() == "intent.hunt")
        .expect("E-14a: the hunt intent");
    assert_eq!(
        report.per_intent[hunt_index],
        IntentDisposition::Deferred {
            reason: DeferReason::WorldBusy,
            not_before: None
        },
        "E-14b: a busy subject defers rather than rejects"
    );
    assert_eq!(
        host.world.wildlife, 100,
        "E-14c: a deferred intent moves nothing in the world"
    );
    assert_eq!(host.world.executed_hunts, 0, "E-14d: nothing executed");

    // A defer is not a rejection: the affiliation-addressed intent in the same
    // batch still executed, so deferral is per intent, not per batch.
    assert_eq!(
        host.world.executed_rations, 1,
        "E-14e: deferral is per intent, not per batch"
    );
    println!("E-14 PASS: Deferred{{WorldBusy}} is produced, is per intent, and moves nothing");
}

// ==================================================================== E-15
// Contract §4.1: affiliation references carry the same two hard requirements
// as entity references — injective and stable — and the two namespaces are
// separate.

#[test]
fn e15_affiliation_mapping_enforces_injectivity_and_stability() {
    let mut m = fixture::entity_map();

    // Stability: an affiliation's scope may not change.
    let err = m
        .bind_affiliation(
            ExternalAffiliationRef("game:settlement/1".to_string()),
            fixture::neighbour(),
        )
        .expect_err("E-15a: re-pointing an affiliation must be refused");
    assert!(
        matches!(err, AffiliationMappingError::NotStable { .. }),
        "E-15b: expected NotStable, got {err:?}"
    );

    // Injectivity: a second affiliation may not claim a bound scope.
    let err = m
        .bind_affiliation(
            ExternalAffiliationRef("game:settlement/99".to_string()),
            fixture::settlement(),
        )
        .expect_err("E-15c: a second affiliation on one scope must be refused");
    assert!(
        matches!(err, AffiliationMappingError::NotInjective { .. }),
        "E-15d: expected NotInjective, got {err:?}"
    );

    // Re-binding the identical pair is idempotent, not an error.
    m.bind_affiliation(
        ExternalAffiliationRef("game:settlement/1".to_string()),
        fixture::settlement(),
    )
    .expect("E-15e: an identical re-bind is a no-op");

    // The two namespaces are separate: an affiliation scope resolves as an
    // affiliation subject, never as an entity.
    assert_eq!(
        m.subject_of(&fixture::settlement()),
        Some(IntentSubject::Affiliation(ExternalAffiliationRef(
            "game:settlement/1".to_string()
        )))
    );
    assert_eq!(
        m.subject_of(&fixture::actor()),
        Some(IntentSubject::Entity(ExternalEntityRef(
            "game:actor/1".to_string()
        )))
    );
    assert!(m.external_of(&fixture::settlement()).is_none());
    assert!(m.affiliation_of(&fixture::actor()).is_none());
    println!("E-15 PASS: affiliation references are injective, stable and namespace-separate");
}

// ==================================================================== E-16
// Contract §9.1: the outbound batch bound is a capacity hint. An oversized
// batch is delivered WHOLE with `capacity_exceeded` set — never truncated,
// because the work behind it is already committed.

#[test]
fn e16_an_oversized_batch_is_reported_not_truncated() {
    let (engine, profile) = fixture::device_engine();
    let (mut d, _s) = PrototypeDevice::open(
        engine,
        profile,
        fixture::entity_map(),
        fixture::intent_channels(),
        CONTRACT_VERSION,
        DeclaredBounds {
            max_observations_per_command: 64,
            max_intents_per_batch: 1, // deliberately too small
        },
    )
    .expect("session");
    let tick = host_command(
        "cmd.tick.1",
        10,
        1,
        "cmd.world_tick",
        fixture::settlement(),
        &[("world.drought", 30), ("world.food_supply", 20)],
    );
    let resp = present_to_completion(&mut d, &tick);
    match resp {
        DeviceResponse::Completed {
            batch,
            capacity_exceeded,
            ..
        } => {
            assert!(
                capacity_exceeded,
                "E-16a: a batch above the declared hint must report it"
            );
            assert_eq!(
                batch.intents.len(),
                2,
                "E-16b: and must still carry every intent — no truncation"
            );
        }
        other => panic!("E-16c: expected a completed boundary, got {other:?}"),
    }
    println!("E-16 PASS: the outbound bound is a reported hint, never a silent truncation");
}
