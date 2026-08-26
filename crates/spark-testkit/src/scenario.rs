//! Deterministic scenario replay harness.
//!
//! A [`Scenario`] is a scripted, self-contained sequence of timeline
//! ingress operations. [`run_scenario`] executes it against a fresh
//! [`TimelineIngress`] and returns a canonical digest of the resulting
//! state. Running the same scenario repeatedly and comparing digests is
//! how Phase-1 test corpus item 19 ("same fixture repeated many times
//! produces identical canonical hashes") is falsified - a nondeterministic
//! bug (unordered iteration, ambient time, accidental shared mutable
//! state) would show up as digest drift across runs.

use spark_core::clock::LogicalTime;
use spark_core::hash::{hash_bytes, Digest};
use spark_core::id::{CommandId, FenceId, ProfileId, SourceId};
use spark_core::timeline::{
    compute_ordered_stream_digest, CommandEnvelope, Ordinal, TimelineEpoch, TimelineFence,
    TimelineIngress,
};

/// One scripted operation against a [`TimelineIngress`].
#[derive(Debug, Clone)]
pub enum ScenarioOp {
    /// Stage one command at `ordinal`, tagged so distinct/duplicate
    /// payloads can be scripted deliberately. The stage result is not
    /// asserted here; a scenario is a replay fixture, not a test body -
    /// callers that need to assert intermediate outcomes should drive
    /// `TimelineIngress` directly (see `spark-core`'s integration
    /// tests) and reserve scenarios for whole-run digest stability.
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

fn payload_hash(tag: &str) -> Digest {
    hash_bytes(tag.as_bytes())
}

/// Executes `scenario` against a fresh [`TimelineIngress`] and returns
/// the final canonical state digest.
pub fn run_scenario(scenario: &Scenario) -> Digest {
    let mut ingress = TimelineIngress::new(
        scenario.profile_id.clone(),
        scenario.timeline_epoch,
        scenario.sequencer.clone(),
        scenario.window_width,
    );

    for op in &scenario.ops {
        match op {
            ScenarioOp::Stage {
                ordinal,
                command_tag,
                payload_tag,
            } => {
                let window = ingress.current_admission_window();
                let envelope = CommandEnvelope {
                    command_id: CommandId::new(command_tag.clone()).unwrap(),
                    profile_id: window.profile_id,
                    timeline_epoch: window.timeline_epoch,
                    effective_time: LogicalTime(*ordinal),
                    source_id: scenario.sequencer.clone(),
                    source_sequence: *ordinal,
                    input_ordinal: Ordinal(*ordinal),
                    admission_window_token: window.token,
                    command_kind: "scenario.command".to_string(),
                    canonical_payload_hash: payload_hash(payload_tag),
                };
                // A scenario replays a scripted history; whether an
                // individual stage attempt lands as Staged, idempotent,
                // poisoned, or out-of-window is itself part of the
                // deterministic outcome captured by the final state
                // digest, so the result is intentionally not asserted
                // or unwrapped here.
                let _ = ingress.stage(&scenario.sequencer, &envelope);
            }
            ScenarioOp::Fence {
                start,
                end,
                fence_tag,
                commands,
            } => {
                let ordered: Vec<(Ordinal, CommandId, Digest)> = commands
                    .iter()
                    .map(|(ordinal, command_tag, payload_tag)| {
                        (
                            Ordinal(*ordinal),
                            CommandId::new(command_tag.clone()).unwrap(),
                            payload_hash(payload_tag),
                        )
                    })
                    .collect();
                let window = ingress.current_admission_window();
                let fence = TimelineFence {
                    profile_id: window.profile_id,
                    timeline_epoch: window.timeline_epoch,
                    fence_id: FenceId::new(fence_tag.clone()).unwrap(),
                    start_ordinal: Ordinal(*start),
                    end_ordinal: Ordinal(*end),
                    previous_fence_hash: window.last_finalized_fence_hash,
                    ordered_stream_digest: compute_ordered_stream_digest(&ordered),
                };
                let _ = ingress.submit_fence(&scenario.sequencer, &fence);
            }
        }
    }

    ingress.canonical_state_digest()
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

    /// Phase-1 test corpus item 19: the same fixture repeated many times
    /// produces identical canonical hashes.
    #[test]
    fn same_scenario_replayed_many_times_produces_identical_digest() {
        let scenario = sample_scenario();
        let first = run_scenario(&scenario);
        for _ in 0..50 {
            assert_eq!(run_scenario(&scenario), first);
        }
    }
}
