// Disposable writer probe of the Revision-2 single-command finalization mechanism over the
// UNCHANGED Phase-1 public TimelineIngress API. Architecture evidence only: not production
// code, not a Phase-2 engine, and not a change to any workspace crate, manifest, or test.
//
// For every refusal route it compares the complete timeline state (the derived `Debug`
// rendering, which includes every private slot, identity index, registry, fence, epoch
// record, and the frontier, plus `canonical_state_digest` and `canonical_history_digest`)
// before and after. It runs three compositions:
//   OLD  - the reviewed candidate's recipe: live `stage`, then live `submit_fence`;
//   REV2 - complete read-only preflight, then the entailed live `stage` + `submit_fence`;
//   REF  - an isolated working copy (`TimelineIngress: Clone`) running the unchanged calls,
//          used only as a differential reference for REV2's verdict.
use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, Digest};
use spark_core::id::{CanonicalTag, CommandId, FenceId, ProfileId, SourceId};
use spark_core::timeline::*;

#[derive(Clone)]
struct Req {
    command_id: &'static str,
    profile: &'static str,
    epoch: u64,
    source: &'static str,
    seq: u64,
}

fn req(command_id: &'static str, seq: u64) -> Req {
    Req {
        command_id,
        profile: "probe",
        epoch: 0,
        source: "s",
        seq,
    }
}

fn envelope(r: &Req, ordinal: u64) -> SemanticCommandEnvelope {
    SemanticCommandEnvelope {
        command_id: CommandId::new(r.command_id).unwrap(),
        profile_id: ProfileId::new(r.profile).unwrap(),
        timeline_epoch: TimelineEpoch(r.epoch),
        effective_time: LogicalTime(20),
        source_id: SourceId::new(r.source).unwrap(),
        source_sequence: r.seq,
        input_ordinal: Ordinal(ordinal),
        command_kind: CommandKind::new(CanonicalTag::new("probe.command").unwrap()),
        canonical_payload_hash: hash_bytes(r.command_id.as_bytes()),
    }
}

// Deterministic fence derivation (Revision-2 candidate: fence_id = "fence." ++ decimal ordinal).
fn fence(t: &TimelineIngress, e: &SemanticCommandEnvelope) -> TimelineFence {
    TimelineFence {
        profile_id: t.profile_id().clone(),
        timeline_epoch: t.timeline_epoch(),
        fence_id: FenceId::new(format!("fence.{}", e.input_ordinal.0)).unwrap(),
        start_ordinal: e.input_ordinal,
        end_ordinal: e.input_ordinal,
        previous_fence_hash: t.last_finalized_fence_hash().clone(),
        ordered_stream_digest: compute_ordered_stream_digest(&[(
            e.input_ordinal,
            e.semantic_hash(),
        )]),
    }
}

#[derive(Debug, PartialEq)]
enum Refusal {
    WrongProfile,
    WrongTimelineEpoch,
    NotActiveSequencer,
    WindowExhausted,
    FrontierExhausted,
    UnexpectedStagingState(u64, SlotStatus),
    CommandIdentityConflict,
    SourceSequenceConflict,
    SourceSequenceNotIncreasing {
        previously_finalized: u64,
        attempted: u64,
    },
}

// Complete read-only preflight. Every Phase-1 predicate that `stage` or `submit_fence` can
// fail for a one-ordinal finalization at the frontier is evaluated here without mutation.
// The identity/sequence predicates read `finalized_commands()`; this is exact because the
// Phase-1 registries are never cleared and, with no slot present (checked first), the
// staged registries are empty by the derived-index invariant (timeline.rs lines 938-967).
fn preflight(
    t: &TimelineIngress,
    grant: &SourceId,
    r: &Req,
) -> Result<SemanticCommandEnvelope, Refusal> {
    if ProfileId::new(r.profile).unwrap() != *t.profile_id() {
        return Err(Refusal::WrongProfile);
    }
    if TimelineEpoch(r.epoch) != t.timeline_epoch() {
        return Err(Refusal::WrongTimelineEpoch);
    }
    if grant != t.active_sequencer() {
        return Err(Refusal::NotActiveSequencer);
    }
    let window = t
        .current_admission_window()
        .map_err(|_| Refusal::WindowExhausted)?;
    let n = window.frontier_ordinal.0;
    if n.checked_add(1).is_none() {
        return Err(Refusal::FrontierExhausted);
    }
    for o in n..=window.window_end.0 {
        let s = t.slot_status(Ordinal(o));
        if s != SlotStatus::Empty {
            return Err(Refusal::UnexpectedStagingState(o, s));
        }
    }
    let e = envelope(r, n);
    let h = e.semantic_hash();
    let fin = t.finalized_commands();
    if fin
        .iter()
        .any(|f| f.envelope.command_id == e.command_id && f.semantic_envelope_hash != h)
    {
        return Err(Refusal::CommandIdentityConflict);
    }
    if fin.iter().any(|f| {
        f.envelope.source_id == e.source_id
            && f.envelope.source_sequence == e.source_sequence
            && f.semantic_envelope_hash != h
    }) {
        return Err(Refusal::SourceSequenceConflict);
    }
    if let Some(prev) = fin
        .iter()
        .rev()
        .find(|f| f.envelope.source_id == e.source_id)
    {
        if e.source_sequence <= prev.envelope.source_sequence {
            return Err(Refusal::SourceSequenceNotIncreasing {
                previously_finalized: prev.envelope.source_sequence,
                attempted: e.source_sequence,
            });
        }
    }
    Ok(e)
}

// Entailed apply: after a passing preflight, only these results are possible.
fn apply(
    t: &mut TimelineIngress,
    grant: &SourceId,
    e: SemanticCommandEnvelope,
) -> FinalizationResult {
    let ticket = t.current_admission_window().unwrap().ticket();
    let d = t.stage(grant, &e.clone().submit_with(ticket)).unwrap();
    let ack = d
        .acknowledgement()
        .expect("entailed positive STAGED acknowledgement");
    assert_eq!(ack.slot_state(), AcknowledgedSlotState::NewlyStaged);
    assert!(ack.covers(&e));
    let f = fence(t, &e);
    let r = t.submit_fence(grant, &f).expect("entailed fence success");
    assert_eq!(
        (
            r.start_ordinal(),
            r.end_ordinal(),
            r.finalized_command_count()
        ),
        (e.input_ordinal, e.input_ordinal, 1)
    );
    r
}

fn rev2(t: &mut TimelineIngress, grant: &SourceId, r: &Req) -> Result<FinalizationResult, Refusal> {
    let e = preflight(t, grant, r)?;
    Ok(apply(t, grant, e))
}

// The reviewed candidate's composition, verbatim in shape: stage, then fence, no undo.
fn old(t: &mut TimelineIngress, grant: &SourceId, r: &Req) -> Result<(), String> {
    let window = t
        .current_admission_window()
        .map_err(|e| format!("window: {e:?}"))?;
    let e = envelope(r, window.frontier_ordinal.0);
    let d = t
        .stage(grant, &e.clone().submit_with(window.ticket()))
        .map_err(|x| format!("stage: {x:?}"))?;
    if d.acknowledgement().is_none() {
        return Err(format!("stage: {}", d.tag()));
    }
    let f = fence(t, &e);
    t.submit_fence(grant, &f)
        .map(|_| ())
        .map_err(|x| format!("fence: {x:?}"))
}

fn snap(t: &TimelineIngress) -> (String, Digest, Digest) {
    (
        format!("{t:?}"),
        t.canonical_state_digest(),
        t.canonical_history_digest(),
    )
}

fn grant() -> SourceId {
    SourceId::new("sequencer").unwrap()
}

fn fresh(width: u32) -> TimelineIngress {
    TimelineIngress::new(
        ProfileId::new("probe").unwrap(),
        TimelineEpoch(0),
        grant(),
        width,
    )
    .unwrap()
}

fn at(frontier: u64, width: u32) -> TimelineIngress {
    TimelineIngress::resume_at_frontier(
        ProfileId::new("probe").unwrap(),
        TimelineEpoch(0),
        grant(),
        width,
        Ordinal(frontier),
    )
    .unwrap()
}

fn with_finalized_s10() -> TimelineIngress {
    let mut t = fresh(2);
    rev2(&mut t, &grant(), &req("first", 10)).unwrap();
    t
}

// Direct Phase-1 staging used ONLY to construct unexpected (non-engine-reachable) states.
fn raw_stage(t: &mut TimelineIngress, r: &Req, ordinal: u64) {
    let ticket = t.current_admission_window().unwrap().ticket();
    let _ = t
        .stage(&grant(), &envelope(r, ordinal).submit_with(ticket))
        .unwrap();
}

struct Route {
    name: &'static str,
    build: fn() -> TimelineIngress,
    request: Req,
    expect: Refusal,
    old_leaves_residue: bool,
    reference_succeeds: bool,
}

fn main() {
    let g = grant();
    let routes = vec![
        Route {
            name: "source-sequence regression S:10 then distinct S:9",
            build: with_finalized_s10,
            request: req("distinct", 9),
            expect: Refusal::SourceSequenceNotIncreasing {
                previously_finalized: 10,
                attempted: 9,
            },
            old_leaves_residue: true,
            reference_succeeds: false,
        },
        Route {
            name: "command-id reuse",
            build: with_finalized_s10,
            request: req("first", 11),
            expect: Refusal::CommandIdentityConflict,
            old_leaves_residue: false,
            reference_succeeds: false,
        },
        Route {
            name: "source-sequence pair reuse",
            build: with_finalized_s10,
            request: req("other", 10),
            expect: Refusal::SourceSequenceConflict,
            old_leaves_residue: false,
            reference_succeeds: false,
        },
        Route {
            name: "wrong timeline epoch",
            build: with_finalized_s10,
            request: Req {
                epoch: 1,
                ..req("other", 11)
            },
            expect: Refusal::WrongTimelineEpoch,
            old_leaves_residue: false,
            reference_succeeds: false,
        },
        Route {
            name: "wrong profile",
            build: with_finalized_s10,
            request: Req {
                profile: "elsewhere",
                ..req("other", 11)
            },
            expect: Refusal::WrongProfile,
            old_leaves_residue: false,
            reference_succeeds: false,
        },
        Route {
            name: "window arithmetic exhausted",
            build: || at(u64::MAX, 2),
            request: req("x", 1),
            expect: Refusal::WindowExhausted,
            old_leaves_residue: false,
            reference_succeeds: false,
        },
        Route {
            name: "frontier advance exhausted",
            build: || at(u64::MAX, 1),
            request: req("x", 1),
            expect: Refusal::FrontierExhausted,
            old_leaves_residue: true,
            reference_succeeds: false,
        },
        Route {
            name: "unexpected poisoned frontier slot",
            build: || {
                let mut t = fresh(2);
                raw_stage(&mut t, &req("p1", 1), 0);
                raw_stage(&mut t, &req("p2", 2), 0);
                t
            },
            request: req("x", 3),
            expect: Refusal::UnexpectedStagingState(0, SlotStatus::Poisoned),
            old_leaves_residue: true,
            reference_succeeds: false,
        },
        Route {
            name: "unexpected different envelope staged at frontier",
            build: || {
                let mut t = fresh(2);
                raw_stage(&mut t, &req("p1", 1), 0);
                t
            },
            request: req("x", 3),
            expect: Refusal::UnexpectedStagingState(0, SlotStatus::Staged),
            old_leaves_residue: true,
            reference_succeeds: false,
        },
        Route {
            name: "unexpected identical envelope staged at frontier",
            build: || {
                let mut t = fresh(2);
                raw_stage(&mut t, &req("x", 3), 0);
                t
            },
            request: req("x", 3),
            expect: Refusal::UnexpectedStagingState(0, SlotStatus::Staged),
            old_leaves_residue: true,
            reference_succeeds: true,
        },
        Route {
            name: "unexpected staged tail above an empty frontier",
            build: || {
                let mut t = fresh(2);
                raw_stage(&mut t, &req("tail", 1), 1);
                t
            },
            request: req("x", 3),
            expect: Refusal::UnexpectedStagingState(1, SlotStatus::Staged),
            old_leaves_residue: true,
            reference_succeeds: true,
        },
    ];
    let mut old_residue_routes = 0;
    for r in &routes {
        // REV2 on live state: typed refusal, complete state byte-identical.
        let mut t = (r.build)();
        let before = snap(&t);
        let got = rev2(&mut t, &g, &r.request).err();
        assert_eq!(got.as_ref(), Some(&r.expect), "{}: REV2 verdict", r.name);
        assert_eq!(
            before,
            snap(&t),
            "{}: REV2 refusal must leave the entire timeline byte-identical",
            r.name
        );
        // REF: isolated working copy, unchanged calls; live never touched.
        let live = (r.build)();
        let mut copy = live.clone();
        let reference_ok = old(&mut copy, &g, &r.request).is_ok();
        assert_eq!(
            reference_ok, r.reference_succeeds,
            "{}: reference verdict",
            r.name
        );
        // OLD on live state.
        let mut o = (r.build)();
        let o_before = snap(&o);
        let o_res = old(&mut o, &g, &r.request);
        let residue = o_before != snap(&o);
        assert_eq!(
            residue, r.old_leaves_residue,
            "{}: OLD residue classification",
            r.name
        );
        if residue {
            old_residue_routes += 1;
        }
        println!(
            "ROUTE {:<52} REV2={:?} byte-identical=true | REF ok={} | OLD result={:?} mutated={}",
            r.name,
            r.expect,
            reference_ok,
            o_res.as_ref().map(|_| "ok"),
            residue
        );
    }
    // Differential: on clean-staging states REV2 refuses exactly when the reference refuses.
    // (Routes 8-11 are unclean states; REV2 refuses them by the fail-closed staging rule.)
    // Success path: REV2 result equals the reference working copy's result byte-for-byte.
    let mut a = with_finalized_s10();
    let mut b = a.clone();
    let res = rev2(&mut a, &g, &req("second", 11)).unwrap();
    old(&mut b, &g, &req("second", 11)).unwrap();
    assert_eq!(snap(&a), snap(&b));
    assert_eq!(res.new_frontier_ordinal(), Ordinal(2));
    println!("SUCCESS REV2 == REF byte-identical; positive NewlyStaged ack preceded the one-ordinal fence; frontier 2");
    // Follow-on: after the regression refusal, the next distinct command finalizes under REV2,
    // while the OLD residue lets it poison ordinal 1 (no fence can then finalize it).
    let mut t = with_finalized_s10();
    let _ = rev2(&mut t, &g, &req("distinct", 9));
    rev2(&mut t, &g, &req("next", 11)).unwrap();
    assert_eq!(t.frontier_ordinal(), Ordinal(2));
    let mut o = with_finalized_s10();
    let _ = old(&mut o, &g, &req("distinct", 9));
    let follow = old(&mut o, &g, &req("next", 11));
    assert!(follow.is_err() && o.slot_status(Ordinal(1)) == SlotStatus::Poisoned);
    println!("FOLLOW-ON REV2: next distinct command finalized at ordinal 1 | OLD: {:?}, ordinal 1 poisoned", follow);
    println!(
        "OLD composition mutated the live timeline on {old_residue_routes} of {} routes that REV2 refuses byte-identically",
        routes.len()
    );
    if std::env::args().any(|a| a == "--negative-control") {
        // Red control: the OLD composition must fail the byte-identity requirement.
        let mut o = with_finalized_s10();
        let before = snap(&o);
        let _ = old(&mut o, &g, &req("distinct", 9));
        assert_eq!(
            before,
            snap(&o),
            "OLD composition refusal atomicity is false"
        );
    }
    println!("ALL REV2 FINALIZATION PROBES PASS");
}
