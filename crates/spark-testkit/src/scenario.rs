//! Deterministic scenario replay harness.
//!
//! A [`Scenario`] is a scripted, self-contained sequence of timeline
//! ingress operations. [`run_scenario`] executes it against a fresh
//! [`TimelineIngress`] and returns a [`ScenarioReplay`] carrying two
//! independent digests (Phase-1 correction brief M-02):
//!
//! - `ingress_state_digest`: `TimelineIngress::canonical_state_digest()`,
//!   the full observable ingress state (staged/poisoned slots included,
//!   not only finalized history).
//! - `transcript_digest`: a hash of every operation's actual result
//!   (`Staged`/`Poisoned`/`NotInAdmissionWindow`/a specific error, or a
//!   fence's `Ok`/specific error), in order.
//!
//! The Phase-1 writer pass's `run_scenario` discarded every stage/fence
//! result and its digest omitted enough ingress state that an empty
//! scenario and a scenario with one successfully staged unfinalized
//! command hashed identically — an incomplete projection asserted to be a
//! canonical replay digest. Neither digest here can exhibit that
//! collision: `ingress_state_digest` hashes staged slots directly, and
//! `transcript_digest` no longer discards operation outcomes.

use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, CanonicalEncoder, Digest};
use spark_core::id::{CommandId, FenceId, ProfileId, SourceId};
use spark_core::timeline::{
    compute_ordered_stream_digest, semantic_envelope_hash, CommandEnvelope, FenceError, Ordinal,
    StageError, StageOutcome, TimelineEpoch, TimelineFence, TimelineIngress,
};

/// One scripted operation against a [`TimelineIngress`].
#[derive(Debug, Clone)]
pub enum ScenarioOp {
    /// Stage one command at `ordinal`, tagged so distinct/duplicate
    /// payloads can be scripted deliberately. `effective_time` and
    /// `source_sequence` are derived deterministically from `ordinal` (see
    /// [`build_envelope`]) so a fence op can reconstruct the identical
    /// semantic identity without replaying admission-window state.
    Stage {
        ordinal: u64,
        command_tag: String,
        payload_tag: String,
    },
    /// Submit a fence over `[start, end]`, computed from the given
    /// `(ordinal, command_tag, payload_tag)` triples, which must match
    /// what was actually staged for that range by prior `Stage` ops.
    Fence {
        start: u64,
        end: u64,
        fence_tag: String,
        commands: Vec<(u64, String, String)>,
    },
}

/// A complete, self-contained deterministic scenario.
#[derive(Debug, Clone)]
pub struct Scenario {
    pub profile_id: ProfileId,
    pub timeline_epoch: TimelineEpoch,
    pub sequencer: SourceId,
    pub window_width: u32,
    pub ops: Vec<ScenarioOp>,
}

/// The result of replaying one [`Scenario`]: the final ingress state
/// digest and a digest of every operation's actual outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioReplay {
    pub ingress_state_digest: Digest,
    pub transcript_digest: Digest,
}

fn payload_hash(tag: &str) -> Digest {
    hash_bytes(tag.as_bytes())
}

/// Builds the envelope for `ordinal` deterministically from scenario
/// constants plus the given tags: `effective_time = LogicalTime(ordinal)`
/// and `source_sequence = ordinal`. Used both to actually stage a command
/// and, independently, by a `Fence` op to recompute the exact semantic
/// identity of commands it expects to have been staged earlier — it does
/// not depend on any historical admission-window snapshot, because
/// [`semantic_envelope_hash`] never includes the admission-window token.
fn build_envelope(
    scenario: &Scenario,
    token: spark_core::timeline::AdmissionWindowToken,
    ordinal: u64,
    command_tag: &str,
    payload_tag: &str,
) -> CommandEnvelope {
    CommandEnvelope {
        command_id: CommandId::new(command_tag).unwrap(),
        profile_id: scenario.profile_id.clone(),
        timeline_epoch: scenario.timeline_epoch,
        effective_time: LogicalTime(ordinal),
        source_id: scenario.sequencer.clone(),
        source_sequence: ordinal,
        input_ordinal: Ordinal(ordinal),
        admission_window_token: token,
        command_kind: "scenario.command".to_string(),
        canonical_payload_hash: payload_hash(payload_tag),
    }
}

fn push_stage_result(enc: &mut CanonicalEncoder, result: &Result<StageOutcome, StageError>) {
    enc.push_str("stage");
    match result {
        Ok(StageOutcome::Staged) => {
            enc.push_str("ok.staged");
        }
        Ok(StageOutcome::AlreadyStagedIdempotent) => {
            enc.push_str("ok.idempotent");
        }
        Ok(StageOutcome::Poisoned) => {
            enc.push_str("ok.poisoned");
        }
        Ok(StageOutcome::NotInAdmissionWindow) => {
            enc.push_str("ok.not_in_window");
        }
        Err(StageError::WrongProfile) => {
            enc.push_str("err.wrong_profile");
        }
        Err(StageError::WrongTimelineEpoch) => {
            enc.push_str("err.wrong_epoch");
        }
        Err(StageError::NotActiveSequencer) => {
            enc.push_str("err.not_active_sequencer");
        }
        Err(StageError::StaleOrInvalidAdmissionWindowToken) => {
            enc.push_str("err.stale_token");
        }
        Err(StageError::CommandIdentityConflict { command_id }) => {
            enc.push_str("err.command_identity_conflict");
            command_id.canonicalize(enc);
        }
        Err(StageError::SourceSequenceConflict {
            source_id,
            source_sequence,
        }) => {
            enc.push_str("err.source_sequence_conflict");
            source_id.canonicalize(enc);
            enc.push_u64(*source_sequence);
        }
        Err(StageError::OrdinalArithmeticOverflow) => {
            enc.push_str("err.ordinal_overflow");
        }
    }
}

fn push_fence_result(enc: &mut CanonicalEncoder, result: &Result<(), FenceError>) {
    enc.push_str("fence");
    match result {
        Ok(()) => {
            enc.push_str("ok");
        }
        Err(FenceError::WrongProfile) => {
            enc.push_str("err.wrong_profile");
        }
        Err(FenceError::WrongTimelineEpoch) => {
            enc.push_str("err.wrong_epoch");
        }
        Err(FenceError::NotActiveSequencer) => {
            enc.push_str("err.not_active_sequencer");
        }
        Err(FenceError::StartNotAtFrontier) => {
            enc.push_str("err.start_not_at_frontier");
        }
        Err(FenceError::EndOutsideStageableHorizon) => {
            enc.push_str("err.end_outside_horizon");
        }
        Err(FenceError::RangeNotFullyStaged) => {
            enc.push_str("err.range_not_fully_staged");
        }
        Err(FenceError::RangeContainsPoisoned) => {
            enc.push_str("err.range_contains_poisoned");
        }
        Err(FenceError::DigestMismatch) => {
            enc.push_str("err.digest_mismatch");
        }
        Err(FenceError::PreviousFenceHashMismatch) => {
            enc.push_str("err.previous_fence_hash_mismatch");
        }
        Err(FenceError::SourceSequenceNotIncreasing {
            source_id,
            previously_finalized,
            attempted,
        }) => {
            enc.push_str("err.source_sequence_not_increasing");
            source_id.canonicalize(enc);
            enc.push_u64(*previously_finalized);
            enc.push_u64(*attempted);
        }
        Err(FenceError::OrdinalArithmeticOverflow) => {
            enc.push_str("err.ordinal_overflow");
        }
    }
}

/// Executes `scenario` against a fresh [`TimelineIngress`] and returns the
/// final ingress-state digest together with a digest of every operation's
/// actual outcome.
pub fn run_scenario(scenario: &Scenario) -> ScenarioReplay {
    let mut ingress = TimelineIngress::new(
        scenario.profile_id.clone(),
        scenario.timeline_epoch,
        scenario.sequencer.clone(),
        scenario.window_width,
    );
    let mut transcript = CanonicalEncoder::new();
    transcript.push_str("scenario_transcript");

    for op in &scenario.ops {
        match op {
            ScenarioOp::Stage {
                ordinal,
                command_tag,
                payload_tag,
            } => {
                let token = ingress.current_admission_window().token;
                let envelope = build_envelope(scenario, token, *ordinal, command_tag, payload_tag);
                let result = ingress.stage(&scenario.sequencer, &envelope);
                push_stage_result(&mut transcript, &result);
            }
            ScenarioOp::Fence {
                start,
                end,
                fence_tag,
                commands,
            } => {
                let window = ingress.current_admission_window();
                let ordered: Vec<(Ordinal, Digest)> = commands
                    .iter()
                    .map(|(ordinal, command_tag, payload_tag)| {
                        let envelope = build_envelope(
                            scenario,
                            window.token.clone(),
                            *ordinal,
                            command_tag,
                            payload_tag,
                        );
                        (Ordinal(*ordinal), semantic_envelope_hash(&envelope))
                    })
                    .collect();
                let fence = TimelineFence {
                    profile_id: window.profile_id,
                    timeline_epoch: window.timeline_epoch,
                    fence_id: FenceId::new(fence_tag.clone()).unwrap(),
                    start_ordinal: Ordinal(*start),
                    end_ordinal: Ordinal(*end),
                    previous_fence_hash: window.last_finalized_fence_hash,
                    ordered_stream_digest: compute_ordered_stream_digest(&ordered),
                };
                let result = ingress.submit_fence(&scenario.sequencer, &fence);
                push_fence_result(&mut transcript, &result);
            }
        }
    }

    ScenarioReplay {
        ingress_state_digest: ingress.canonical_state_digest(),
        transcript_digest: transcript.finish(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_scenario() -> Scenario {
        Scenario {
            profile_id: ProfileId::new("game-world").unwrap(),
            timeline_epoch: TimelineEpoch(1),
            sequencer: SourceId::new("sequencer.primary").unwrap(),
            window_width: 2,
            ops: vec![
                ScenarioOp::Stage {
                    ordinal: 0,
                    command_tag: "cmd.0".to_string(),
                    payload_tag: "payload.0".to_string(),
                },
                ScenarioOp::Stage {
                    ordinal: 1,
                    command_tag: "cmd.1".to_string(),
                    payload_tag: "payload.1".to_string(),
                },
                // Out-of-window at time of staging; part of the
                // scripted, deterministic outcome.
                ScenarioOp::Stage {
                    ordinal: 2,
                    command_tag: "cmd.2".to_string(),
                    payload_tag: "payload.2".to_string(),
                },
                ScenarioOp::Fence {
                    start: 0,
                    end: 1,
                    fence_tag: "fence.1".to_string(),
                    commands: vec![
                        (0, "cmd.0".to_string(), "payload.0".to_string()),
                        (1, "cmd.1".to_string(), "payload.1".to_string()),
                    ],
                },
                // Ordinal 2 is now in-window under the fresh token.
                ScenarioOp::Stage {
                    ordinal: 2,
                    command_tag: "cmd.2".to_string(),
                    payload_tag: "payload.2".to_string(),
                },
                ScenarioOp::Fence {
                    start: 2,
                    end: 2,
                    fence_tag: "fence.2".to_string(),
                    commands: vec![(2, "cmd.2".to_string(), "payload.2".to_string())],
                },
            ],
        }
    }

    fn empty_scenario() -> Scenario {
        Scenario {
            profile_id: ProfileId::new("game-world").unwrap(),
            timeline_epoch: TimelineEpoch(1),
            sequencer: SourceId::new("sequencer.primary").unwrap(),
            window_width: 2,
            ops: vec![],
        }
    }

    fn one_staged_unfinalized_scenario() -> Scenario {
        Scenario {
            profile_id: ProfileId::new("game-world").unwrap(),
            timeline_epoch: TimelineEpoch(1),
            sequencer: SourceId::new("sequencer.primary").unwrap(),
            window_width: 2,
            ops: vec![ScenarioOp::Stage {
                ordinal: 0,
                command_tag: "cmd.0".to_string(),
                payload_tag: "payload.0".to_string(),
            }],
        }
    }

    /// Phase-1 test corpus item 19: the same fixture repeated many times
    /// produces identical canonical hashes.
    #[test]
    fn same_scenario_replayed_many_times_produces_identical_digest() {
        let scenario = sample_scenario();
        let first = run_scenario(&scenario);
        for _ in 0..50 {
            let replay = run_scenario(&scenario);
            assert_eq!(replay.ingress_state_digest, first.ingress_state_digest);
            assert_eq!(replay.transcript_digest, first.transcript_digest);
        }
    }

    /// M-02: an empty scenario must not hash identically to a scenario
    /// with one successfully staged, unfinalized command.
    #[test]
    fn empty_scenario_differs_from_one_staged_unfinalized_command() {
        let empty = run_scenario(&empty_scenario());
        let one_staged = run_scenario(&one_staged_unfinalized_scenario());
        assert_ne!(empty.ingress_state_digest, one_staged.ingress_state_digest);
        assert_ne!(empty.transcript_digest, one_staged.transcript_digest);
    }

    /// M-02: poisoned staging must digest differently from clean staging.
    #[test]
    fn poisoned_staging_differs_from_clean_staging() {
        let clean = one_staged_unfinalized_scenario();
        let mut poisoned = clean.clone();
        poisoned.ops.push(ScenarioOp::Stage {
            ordinal: 0,
            command_tag: "cmd.conflict".to_string(),
            payload_tag: "payload.conflict".to_string(),
        });

        let clean_result = run_scenario(&clean);
        let poisoned_result = run_scenario(&poisoned);
        assert_ne!(
            clean_result.ingress_state_digest,
            poisoned_result.ingress_state_digest
        );
        assert_ne!(
            clean_result.transcript_digest,
            poisoned_result.transcript_digest
        );
    }

    /// M-02: a behaviorally different finalized envelope (here: a
    /// different payload for the same ordinal/command) changes the
    /// finalized history digest.
    #[test]
    fn different_finalized_envelope_changes_history_digest() {
        fn finalized_scenario(payload_tag: &str) -> Scenario {
            Scenario {
                profile_id: ProfileId::new("game-world").unwrap(),
                timeline_epoch: TimelineEpoch(1),
                sequencer: SourceId::new("sequencer.primary").unwrap(),
                window_width: 2,
                ops: vec![
                    ScenarioOp::Stage {
                        ordinal: 0,
                        command_tag: "cmd.0".to_string(),
                        payload_tag: payload_tag.to_string(),
                    },
                    ScenarioOp::Fence {
                        start: 0,
                        end: 0,
                        fence_tag: "fence.1".to_string(),
                        commands: vec![(0, "cmd.0".to_string(), payload_tag.to_string())],
                    },
                ],
            }
        }
        let a = run_scenario(&finalized_scenario("payload.a"));
        let b = run_scenario(&finalized_scenario("payload.b"));
        assert_ne!(a.ingress_state_digest, b.ingress_state_digest);
    }

    /// M-02: a scenario whose operation outcomes differ (here: a fence
    /// that succeeds versus one that is rejected) must produce a
    /// different transcript digest even if it doesn't affect the final
    /// ingress state.
    #[test]
    fn differing_operation_outcome_changes_transcript_digest() {
        let mut valid_fence = one_staged_unfinalized_scenario();
        valid_fence.ops.push(ScenarioOp::Fence {
            start: 0,
            end: 0,
            fence_tag: "fence.ok".to_string(),
            commands: vec![(0, "cmd.0".to_string(), "payload.0".to_string())],
        });

        let mut wrong_digest_fence = one_staged_unfinalized_scenario();
        wrong_digest_fence.ops.push(ScenarioOp::Fence {
            start: 0,
            end: 0,
            fence_tag: "fence.bad".to_string(),
            // Wrong payload tag produces a mismatched ordered-stream
            // digest, so this fence attempt is rejected.
            commands: vec![(0, "cmd.0".to_string(), "payload.wrong".to_string())],
        });

        let ok_result = run_scenario(&valid_fence);
        let rejected_result = run_scenario(&wrong_digest_fence);
        assert_ne!(
            ok_result.transcript_digest,
            rejected_result.transcript_digest
        );
    }
}
