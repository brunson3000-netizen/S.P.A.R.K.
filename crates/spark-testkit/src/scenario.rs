//! Deterministic scenario replay harness.
//!
//! A [`Scenario`] is a scripted, self-contained sequence of timeline
//! ingress operations. [`run_scenario`] executes it against a fresh
//! [`TimelineIngress`] and returns a [`ScenarioReplay`] carrying three
//! independent digests:
//!
//! - `history_digest`: [`TimelineIngress::canonical_history_digest`], the
//!   finalized semantic history. **This is the value replay equivalence is
//!   judged on.** Two scenarios that finalize the same commands behind the
//!   same fence chain must agree here regardless of when each command was
//!   staged relative to a frontier advance, because no admission
//!   credential is retained anywhere in the ingress.
//! - `ingress_state_digest`: [`TimelineIngress::canonical_state_digest`],
//!   the complete observable ingress state including staged/poisoned
//!   slots — so an empty scenario cannot hash identically to a scenario
//!   with one successfully staged unfinalized command.
//! - `transcript_digest`: a hash of every operation's actual structured
//!   result in order, so two scenarios whose *outcomes* differ are
//!   distinguishable even when their final state happens to coincide.
//!
//! The transcript records the acknowledgement/finalization evidence the
//! re-founded ingress issues, not merely a coarse enum tag: a
//! `StageAcknowledgement`'s bound ordinal, command ID, semantic hash, and
//! slot state all reach the transcript, as does a `FinalizationResult`'s
//! finalized range and fence hash.

use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, CanonicalEncoder, Digest};
use spark_core::id::StableIdError;
use spark_core::id::{CanonicalTag, CommandId, FenceId, ProfileId, SourceId};
use spark_core::timeline::{
    compute_ordered_stream_digest, AdmissionTicket, CommandKind, FenceError, FinalizationResult,
    Ordinal, SemanticCommandEnvelope, StageDisposition, StageError, TimelineConfigError,
    TimelineEpoch, TimelineFence, TimelineIngress,
};
use std::fmt;

/// Rejects a scenario that cannot be run as written.
///
/// The harness reports malformed scenario input rather than panicking on
/// it: a fixture harness that panics on bad input is a harness that can
/// mask a real regression behind an unrelated crash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioError {
    /// The ingress configuration is unusable (e.g. a zero window width).
    Config(TimelineConfigError),
    /// A scripted command/fence tag is not a valid canonical identifier.
    MalformedIdentifier { tag: String, error: StableIdError },
}

impl fmt::Display for ScenarioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScenarioError::Config(e) => write!(f, "{e}"),
            ScenarioError::MalformedIdentifier { tag, error } => {
                write!(f, "scenario identifier '{tag}' is invalid: {error}")
            }
        }
    }
}

impl std::error::Error for ScenarioError {}

impl From<TimelineConfigError> for ScenarioError {
    fn from(e: TimelineConfigError) -> Self {
        ScenarioError::Config(e)
    }
}

fn identifier<T>(tag: &str, built: Result<T, StableIdError>) -> Result<T, ScenarioError> {
    built.map_err(|error| ScenarioError::MalformedIdentifier {
        tag: tag.to_string(),
        error,
    })
}

/// One scripted operation against a [`TimelineIngress`].
#[derive(Debug, Clone)]
pub enum ScenarioOp {
    /// Stage one command at `ordinal`, tagged so distinct/duplicate
    /// payloads can be scripted deliberately. `effective_time` and
    /// `source_sequence` are derived deterministically from `ordinal` (see
    /// [`build_envelope`]) so a fence op can reconstruct the identical
    /// semantic identity.
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

/// The result of replaying one [`Scenario`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioReplay {
    /// Finalized semantic history only — the replay-equivalence value.
    pub history_digest: Digest,
    /// Complete observable ingress state, including staged slots.
    pub ingress_state_digest: Digest,
    /// Every operation's structured outcome, in order.
    pub transcript_digest: Digest,
}

fn payload_hash(tag: &str) -> Digest {
    hash_bytes(tag.as_bytes())
}

/// Builds the semantic envelope for `ordinal` deterministically from
/// scenario constants plus the given tags: `effective_time =
/// LogicalTime(ordinal)` and `source_sequence = ordinal`.
///
/// Note that this takes no admission credential: a semantic envelope is
/// admission-free by construction, which is why a `Fence` op can recompute
/// the exact identity of commands staged under an earlier window without
/// replaying any admission state.
pub fn build_envelope(
    scenario: &Scenario,
    ordinal: u64,
    command_tag: &str,
    payload_tag: &str,
) -> Result<SemanticCommandEnvelope, ScenarioError> {
    Ok(SemanticCommandEnvelope {
        command_id: identifier(command_tag, CommandId::new(command_tag))?,
        profile_id: scenario.profile_id.clone(),
        timeline_epoch: scenario.timeline_epoch,
        effective_time: LogicalTime(ordinal),
        source_id: scenario.sequencer.clone(),
        source_sequence: ordinal,
        input_ordinal: Ordinal(ordinal),
        command_kind: CommandKind::new(SCENARIO_COMMAND_KIND),
        canonical_payload_hash: payload_hash(payload_tag),
    })
}

/// The canonical command kind every scenario command carries, validated
/// by the compiler rather than at runtime.
const SCENARIO_COMMAND_KIND: CanonicalTag = CanonicalTag::from_static("scenario.command");

fn push_stage_result(enc: &mut CanonicalEncoder, result: &Result<StageDisposition, StageError>) {
    enc.push_str("stage");
    match result {
        Ok(StageDisposition::Acknowledged(ack)) => {
            enc.push_str("ok.acknowledged");
            enc.push_str(ack.slot_state().tag());
            enc.push_u64(ack.input_ordinal().0);
            ack.command_id().canonicalize(enc);
            enc.push_digest(ack.semantic_envelope_hash());
            enc.push_digest(ack.acknowledgement_hash());
        }
        Ok(StageDisposition::Poisoned(record)) => {
            enc.push_str("ok.poisoned");
            enc.push_u64(record.input_ordinal().0);
            enc.push_digest(record.rejected_semantic_hash());
        }
        Ok(StageDisposition::NotInAdmissionWindow(advice)) => {
            enc.push_str("ok.not_in_window");
            enc.push_u64(advice.attempted_ordinal().0);
            enc.push_u64(advice.frontier_ordinal().0);
            enc.push_u64(advice.window_end().0);
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
        Err(StageError::StaleOrInvalidAdmissionTicket) => {
            enc.push_str("err.stale_ticket");
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
        Err(StageError::OrdinalSpaceExhausted(e)) => {
            enc.push_str("err.ordinal_space_exhausted");
            enc.push_u64(e.frontier_ordinal.0);
            enc.push_u32(e.window_width);
        }
    }
}

fn push_fence_result(enc: &mut CanonicalEncoder, result: &Result<FinalizationResult, FenceError>) {
    enc.push_str("fence");
    match result {
        Ok(finalized) => {
            enc.push_str("ok");
            finalized.fence_id().canonicalize(enc);
            enc.push_u64(finalized.start_ordinal().0);
            enc.push_u64(finalized.end_ordinal().0);
            enc.push_u64(finalized.finalized_command_count());
            enc.push_u64(finalized.new_frontier_ordinal().0);
            enc.push_digest(finalized.fence_hash());
            enc.push_digest(finalized.canonical_history_digest());
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
        Err(FenceError::RangeNotFullyStaged { ordinal }) => {
            enc.push_str("err.range_not_fully_staged");
            enc.push_u64(ordinal.0);
        }
        Err(FenceError::RangeContainsPoisoned { ordinal }) => {
            enc.push_str("err.range_contains_poisoned");
            enc.push_u64(ordinal.0);
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
        Err(FenceError::OrdinalSpaceExhausted(e)) => {
            enc.push_str("err.ordinal_space_exhausted");
            enc.push_u64(e.frontier_ordinal.0);
            enc.push_u32(e.window_width);
        }
    }
}

/// Executes `scenario` against a fresh [`TimelineIngress`].
pub fn run_scenario(scenario: &Scenario) -> Result<ScenarioReplay, ScenarioError> {
    let mut ingress = TimelineIngress::new(
        scenario.profile_id.clone(),
        scenario.timeline_epoch,
        scenario.sequencer.clone(),
        scenario.window_width,
    )?;
    let mut transcript = CanonicalEncoder::new();
    transcript.push_str("scenario_transcript");

    for op in &scenario.ops {
        match op {
            ScenarioOp::Stage {
                ordinal,
                command_tag,
                payload_tag,
            } => {
                // A scenario always stages against the *current* window,
                // so the credential is always fresh; the point of the
                // harness is that this choice cannot affect the result.
                let ticket: AdmissionTicket = match ingress.current_admission_window() {
                    Ok(window) => window.ticket(),
                    Err(exhausted) => {
                        push_stage_result(
                            &mut transcript,
                            &Err(StageError::OrdinalSpaceExhausted(exhausted)),
                        );
                        continue;
                    }
                };
                let envelope = build_envelope(scenario, *ordinal, command_tag, payload_tag)?;
                let submission = envelope.submit_with(ticket);
                let result = ingress.stage(&scenario.sequencer, &submission);
                push_stage_result(&mut transcript, &result);
            }
            ScenarioOp::Fence {
                start,
                end,
                fence_tag,
                commands,
            } => {
                let mut ordered: Vec<(Ordinal, Digest)> = Vec::with_capacity(commands.len());
                for (ordinal, command_tag, payload_tag) in commands {
                    let envelope = build_envelope(scenario, *ordinal, command_tag, payload_tag)?;
                    ordered.push((Ordinal(*ordinal), envelope.semantic_hash()));
                }
                let fence = TimelineFence {
                    profile_id: ingress.profile_id().clone(),
                    timeline_epoch: ingress.timeline_epoch(),
                    fence_id: identifier(fence_tag, FenceId::new(fence_tag.clone()))?,
                    start_ordinal: Ordinal(*start),
                    end_ordinal: Ordinal(*end),
                    previous_fence_hash: ingress.last_finalized_fence_hash().clone(),
                    ordered_stream_digest: compute_ordered_stream_digest(&ordered),
                };
                let result = ingress.submit_fence(&scenario.sequencer, &fence);
                push_fence_result(&mut transcript, &result);
            }
        }
    }

    Ok(ScenarioReplay {
        history_digest: ingress.canonical_history_digest(),
        ingress_state_digest: ingress.canonical_state_digest(),
        transcript_digest: transcript.finish(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scenario(window_width: u32, ops: Vec<ScenarioOp>) -> Scenario {
        Scenario {
            profile_id: ProfileId::new("game-world").unwrap(),
            timeline_epoch: TimelineEpoch(1),
            sequencer: SourceId::new("sequencer.primary").unwrap(),
            window_width,
            ops,
        }
    }

    fn stage(ordinal: u64, command: &str, payload: &str) -> ScenarioOp {
        ScenarioOp::Stage {
            ordinal,
            command_tag: command.to_string(),
            payload_tag: payload.to_string(),
        }
    }

    fn sample_scenario() -> Scenario {
        scenario(
            2,
            vec![
                stage(0, "cmd.0", "payload.0"),
                stage(1, "cmd.1", "payload.1"),
                // Out-of-window at time of staging; part of the scripted,
                // deterministic outcome.
                stage(2, "cmd.2", "payload.2"),
                ScenarioOp::Fence {
                    start: 0,
                    end: 1,
                    fence_tag: "fence.1".to_string(),
                    commands: vec![
                        (0, "cmd.0".to_string(), "payload.0".to_string()),
                        (1, "cmd.1".to_string(), "payload.1".to_string()),
                    ],
                },
                // Ordinal 2 is now in-window under the fresh window.
                stage(2, "cmd.2", "payload.2"),
                ScenarioOp::Fence {
                    start: 2,
                    end: 2,
                    fence_tag: "fence.2".to_string(),
                    commands: vec![(2, "cmd.2".to_string(), "payload.2".to_string())],
                },
            ],
        )
    }

    fn empty_scenario() -> Scenario {
        scenario(2, vec![])
    }

    fn one_staged_unfinalized_scenario() -> Scenario {
        scenario(2, vec![stage(0, "cmd.0", "payload.0")])
    }

    /// Phase-1 test corpus item 19: the same fixture repeated many times
    /// produces identical canonical hashes.
    #[test]
    fn same_scenario_replayed_many_times_produces_identical_digest() {
        let scenario = sample_scenario();
        let first = run_scenario(&scenario).unwrap();
        for _ in 0..50 {
            let replay = run_scenario(&scenario).unwrap();
            assert_eq!(replay, first);
        }
    }

    /// An empty scenario must not hash identically to a scenario with one
    /// successfully staged, unfinalized command.
    #[test]
    fn empty_scenario_differs_from_one_staged_unfinalized_command() {
        let empty = run_scenario(&empty_scenario()).unwrap();
        let one_staged = run_scenario(&one_staged_unfinalized_scenario()).unwrap();
        assert_ne!(empty.ingress_state_digest, one_staged.ingress_state_digest);
        assert_ne!(empty.transcript_digest, one_staged.transcript_digest);
        // Neither finalized anything, so finalized history is legitimately
        // identical — which is exactly why the two digests are separate.
        assert_eq!(empty.history_digest, one_staged.history_digest);
    }

    /// Poisoned staging must digest differently from clean staging.
    #[test]
    fn poisoned_staging_differs_from_clean_staging() {
        let clean = one_staged_unfinalized_scenario();
        let mut poisoned = clean.clone();
        poisoned
            .ops
            .push(stage(0, "cmd.conflict", "payload.conflict"));

        let clean_result = run_scenario(&clean).unwrap();
        let poisoned_result = run_scenario(&poisoned).unwrap();
        assert_ne!(
            clean_result.ingress_state_digest,
            poisoned_result.ingress_state_digest
        );
        assert_ne!(
            clean_result.transcript_digest,
            poisoned_result.transcript_digest
        );
    }

    /// A behaviorally different finalized envelope changes finalized
    /// history.
    #[test]
    fn different_finalized_envelope_changes_history_digest() {
        fn finalized_scenario(payload_tag: &str) -> Scenario {
            scenario(
                2,
                vec![
                    stage(0, "cmd.0", payload_tag),
                    ScenarioOp::Fence {
                        start: 0,
                        end: 0,
                        fence_tag: "fence.1".to_string(),
                        commands: vec![(0, "cmd.0".to_string(), payload_tag.to_string())],
                    },
                ],
            )
        }
        let a = run_scenario(&finalized_scenario("payload.a")).unwrap();
        let b = run_scenario(&finalized_scenario("payload.b")).unwrap();
        assert_ne!(a.history_digest, b.history_digest);
        assert_ne!(a.ingress_state_digest, b.ingress_state_digest);
    }

    /// A scenario whose operation outcomes differ must produce a different
    /// transcript digest even when the final state coincides.
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

        let ok_result = run_scenario(&valid_fence).unwrap();
        let rejected_result = run_scenario(&wrong_digest_fence).unwrap();
        assert_ne!(
            ok_result.transcript_digest,
            rejected_result.transcript_digest
        );
    }

    /// A zero window width is a configuration error, not a panic.
    #[test]
    fn zero_window_width_is_reported_not_panicked() {
        assert_eq!(
            run_scenario(&scenario(0, vec![])),
            Err(ScenarioError::Config(
                TimelineConfigError::WindowWidthMustBeAtLeastOne
            ))
        );
    }

    /// A malformed scripted identifier is reported, not panicked on: the
    /// harness must never crash in a way that could mask a regression.
    #[test]
    fn malformed_scenario_identifier_is_reported_not_panicked() {
        let result = run_scenario(&scenario(2, vec![stage(0, "Not A Valid Id", "payload.0")]));
        assert!(matches!(
            result,
            Err(ScenarioError::MalformedIdentifier { .. })
        ));
    }
}
