# Pattern Card — Canonical Flight Event Before Telemetry Projection

PATTERN: Canonical Event First, Derived Telemetry Second
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source convergence

- AgentTrace @ `9a10f9aae3bc508ddca83093b2d25aede5ad5bd0`
- agent-observability @ `2658eef467225f376e2e92dc1465839eda2bc113`
- OpenTelemetry Collector @ `a35b7a8db49df923c5add3dc34872b6e0b3af683`
- grok-build durable memory/capture patterns
- SPARK existing evidence doctrine.

## Problem

Telemetry pipelines are intentionally lossy/derived: they may sample, queue, retry, transform, redact, batch, reject, reorder or drop observations. Self-reporting can omit/fabricate events, and interception/export layers may lose causality. Treating telemetry as canonical evidence makes absence, ordering and semantic meaning unreliable.

## Mechanism

On every load-bearing runtime operation:

1. Host assigns stable event/operation identity.
2. Host performs/records authority decision separately.
3. Host commits a canonical immutable flight event and required evidence/artifact references.
4. Only after canonical commit, derive a bounded telemetry projection.
5. Projection may be queued/batched/retried/exported through OTel or another backend.
6. Export/delivery state is recorded separately from operation/evidence state.

Target shape:

`operation/authority → canonical flight event + evidence refs → derived telemetry projection → queue/process/batch/export`

Never invert this into `operation → telemetry backend → reconstructed truth`.

## Canonical event properties

Recommended minimum fields:

- `schema_version`
- `event_id` — sortable stable UUID/ULID-style identity
- `event_kind`
- wall-clock timestamp plus monotonic/host sequence where available
- `session_id` / mission/work key
- `operation_id`
- `parent_operation_id` / causal parent
- `actor_id` / worker identity
- `attempt` / generation
- `provenance_class`:
  - `HOST_CANONICAL`
  - `INTERCEPTED`
  - `SELF_REPORTED`
  - `DERIVED`
- stable capability/tool identity + adapter/version/source pin
- authority decision/grant **reference**, not a grant embedded in telemetry
- input artifact/evidence reference + full digest + bounded sanitized preview
- output/result artifact/evidence reference + digest + bounded sanitized preview
- status/result/error/cancel/retry state
- duration/resource observations
- token/cost observations with source/method/version + `estimated` flag
- confidentiality/redaction class
- downstream correlation IDs (e.g. OTel trace/span IDs) as aliases, never canonical identity.

## Evidence strategy

Large/sensitive payloads remain in the canonical evidence/artifact layer.

Flight events contain:
- full digest/reference
- content type/size
- bounded sanitized preview only when policy permits.

This keeps the recorder cheap while preserving navigability to full evidence.

## Benefits

- exporter/backend outage cannot erase execution history
- telemetry sampling/drop does not create false “nothing happened” conclusions
- deterministic causal reconstruction under concurrency/retries
- clean privacy boundary
- multiple telemetry/export formats can evolve without changing canonical semantics
- billing/correctness/authority estimates remain visibly derived.

## Risks / failure modes

- canonical write latency placed on every operation
- recorder storage pressure
- event committed but referenced artifact missing, or vice versa
- sequence/attempt identity reused incorrectly
- sensitive preview leakage
- telemetry projection accidentally promoted into policy truth.

These require a bounded durable recorder protocol, not removing the canonical layer.

## Boundaries crossed

Runtime/authority → canonical event store.
Canonical event → evidence/artifact store.
Canonical event → derived telemetry pipeline.
Telemetry pipeline → external backend/UI.

## Determinization relevance

CRITICAL. Event identity, ordering, provenance class, artifact binding, schema validation, retention and projection are deterministic runtime responsibilities.

## Likely architectural location

Rust supervisor/runtime evidence plane, upstream of OpenTelemetry/export integrations.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

REIMPLEMENT_IN_RUST.

Use external projects as schema/behavior prior art; keep the canonical recorder inside the authoritative Rust core.

## Required invariants

1. Canonical event/evidence commit does not depend on telemetry exporter availability.
2. Telemetry/export failure cannot change operation result or authority.
3. Every concurrent/retried operation has unique identity + attempt/generation.
4. Causal parent is explicit; no global mutable “current trace.”
5. Self-reported/derived/intercepted facts are labeled differently from host-canonical facts.
6. Full payload is referenced by digest; preview is bounded/redacted.
7. Estimated cost/token/quality values carry source/method/version.
8. Export delivery states are distinct from canonical-recorded state.
9. Downstream trace/span IDs are correlation aliases only.
10. Canonical event records are immutable after commit; corrections append superseding events rather than rewriting history.
