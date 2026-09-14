# Experiment / Contract Proposal — SPARK Flight Recorder V0

Status: READY_FOR_IMPLEMENTATION — NOT RUN
Created: 2026-09-13

## Question

Can SPARK record enough canonical runtime evidence to reconstruct concurrent worker/tool activity deterministically while keeping telemetry/export optional, low-context and non-authoritative?

This proposal does not authorize implementation/adoption. It defines the qualification target for the next build stage.

## Core hypothesis

A small Rust event ledger can provide reliable causal history with modest overhead if large payloads remain in the evidence/artifact store and telemetry is derived after canonical commit.

Target:

`operation → canonical flight event + artifact refs → optional OTel/export projection`

## V0 event contract

Every load-bearing operation emits immutable events under a versioned schema.

Minimum fields:

```text
schema_version
event_id
event_kind
wall_time
monotonic_or_session_sequence
session_id / work_key
operation_id
parent_operation_id
actor_id
attempt_or_generation
provenance_class
capability_id
capability_version_or_adapter_pin
authority_decision_ref
input_artifact_ref + digest + bounded_preview
output_artifact_ref + digest + bounded_preview
status / error_class / cancellation_reason
start/end/duration
resource_observations
token_observations
cost_observations
confidentiality/redaction_class
telemetry_aliases (optional OTel trace/span IDs)
```

`provenance_class` must at least distinguish:
- `HOST_CANONICAL`
- `INTERCEPTED`
- `SELF_REPORTED`
- `DERIVED`.

## Canonical storage rule

Large/full payloads are not embedded by default.

1. Full evidence/artifact receives immutable identity/full digest.
2. Flight event stores digest/reference + bounded sanitized preview.
3. Event and evidence binding is validated before the operation is reported as fully recorded.
4. Derived indexes/telemetry may be rebuilt from the canonical event/evidence state.

## Experiment workload

Run a deterministic synthetic multi-agent/tool workload containing:
- root coordinator
- 4 concurrent child workers
- nested child from one worker
- two tools with the same display name but different stable capability IDs
- overlapping async calls
- retries/attempt replacement
- one cancelled call
- one timeout
- one large-output call
- one payload containing a fake secret marker
- one self-reported annotation
- one derived classification/cost estimate.

The workload should be replayable and should not require paid model inference for the recorder mechanics test.

## Fault cases

### Causality/concurrency
1. Overlapping parent/child tool calls.
2. Concurrent operations finishing out of start order.
3. Same display tool name from two adapters.
4. Stale attempt finishes after replacement attempt.
5. Cancellation races with terminal result.

Pass: canonical reconstruction recovers correct parent, actor, capability, attempt and terminal state without relying on wall-clock/export order.

### Evidence/payload
6. Tool output > recorder preview bound.
7. Output contains secret marker/PII-like data.
8. Referenced artifact missing/corrupt before event commit.
9. Artifact valid, telemetry exporter unavailable.

Pass: full evidence remains in artifact layer, preview obeys byte/redaction policy, digest validates, and exporter failure cannot invalidate canonical operation evidence.

### Telemetry delivery
10. OTel adapter disabled.
11. OTel endpoint unavailable.
12. Sending queue full with reject-on-overflow.
13. Sending queue full with block-on-overflow under bounded export worker context.
14. Persistent queue restart.
15. Export permanent error.
16. Retry exhaustion.
17. Multiple consumers finish out of order.
18. Telemetry sampling/filter intentionally drops event.

Pass: every canonical event remains queryable and causal history unchanged; delivery state accurately records enqueue/deliver/drop/reject where available.

### Process failure
19. Crash after evidence artifact commit but before event commit.
20. Crash after event commit but before telemetry projection.
21. Crash after projection enqueue but before delivery acknowledgement.

Pass: recovery can identify/reconcile incomplete canonical states; telemetry can be regenerated/retried without inventing or losing authoritative operations.

### Provenance/derived fields
22. Self-reported annotation contradicts host canonical event.
23. Derived cost is later corrected by provider/account record.
24. Agent-reported rationale conflicts with observed action.

Pass: provenance classes prevent weaker/derived facts from overwriting host canonical facts; corrections append/supersede rather than rewrite history.

## Measurements

Record:
- canonical event write latency p50/p95/p99
- artifact bind/commit latency
- bytes/event without full payloads
- disk growth per 10k events
- replay/reconstruction time
- additional operation latency
- OTel adapter CPU/memory overhead
- queue rejection/drop count during exporter outage
- number of canonical events versus delivered/exported events
- causal reconstruction error count (target 0)
- secret marker leakage into canonical previews/exported telemetry (target 0 unless explicitly allowed).

## Acceptance criteria

1. **Zero causal ambiguity** across the deterministic workload.
2. Canonical event/evidence survives every telemetry fault case.
3. Export pipeline absence/failure never changes tool/worker authority or operation result.
4. Large payloads do not enter canonical event rows beyond bounded previews.
5. Sensitive markers are redacted according to policy before telemetry projection.
6. Self-reported/derived observations cannot replace host canonical state.
7. Replay can produce a stable operation tree and terminal states from canonical records only.
8. OTel is removable/replaceable without changing canonical schema semantics.

## Implementation comparison

V0 should compare two canonical storage approaches only if both are cheap enough:
- append-oriented durable file/event journal + index
- SQLite WAL event table + artifact references.

Do not include OpenTelemetry Collector as a canonical-storage candidate; it is deliberately downstream.

## Expected outputs

- Rust event schema candidate
- canonical-store recommendation
- event/evidence commit protocol
- OTel projection mapping
- retention/redaction rules
- replay/tree reconstruction validator
- measured overhead
- failure/repair ledger updates
- adoption/rejection recommendation.
