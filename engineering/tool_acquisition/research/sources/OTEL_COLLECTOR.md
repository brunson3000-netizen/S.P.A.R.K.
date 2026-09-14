# Source Study — OpenTelemetry Collector

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: OpenTelemetry Collector
- Upstream: `https://github.com/open-telemetry/opentelemetry-collector`
- Commit: `a35b7a8db49df923c5add3dc34872b6e0b3af683`
- License: Apache-2.0
- Runtime: Go component graph of receivers, processors, exporters, connectors and extensions
- Primary evidence: `docs/internal-architecture.md`, `service/internal/graph/`, `internal/fanoutconsumer/traces.go`, `exporter/exporterhelper/internal/base_exporter.go`, `retry_sender.go`, `queue_sender.go`, `queuebatch/*`, `queue/*`, `consumer/consumererror/*`.

## Problem framing

PROBLEM: Telemetry needs to be received from many producers, transformed/fanned out, buffered across downstream failures, retried, batched and exported to different observability backends without embedding every backend in the producing application.

USER: application/runtime telemetry producers and operators of telemetry pipelines.

INPUT: traces, metrics, logs or profiles plus pipeline configuration and component-specific metadata/context.

OUTPUT: transformed/batched telemetry delivered to one or more exporters/backends, with Collector self-observability about queues/retries/rejections.

AUTHORITY: Observational pipeline only. Collector components can transform/drop/export telemetry but do not own the authority of the operation that produced the telemetry.

TRUST: producer signal content, component configurations, processor/exporter behavior, declared consumer mutation capabilities, storage extension and exporter error classification.

STATE: component graph, in-memory queues/batches, optional persistent sending queues, retry state, exporter connections, component telemetry.

FAILURE: queue full/rejection, processor/consumer error, exporter timeout, permanent error, retry exhaustion, shutdown, storage corruption/error, exporter/backend outage. Some paths intentionally drop telemetry after terminal delivery failure.

## Trace A — configured pipeline graph

Collector configuration defines receiver → processor(s) → exporter(s) pipelines. The service builds a directed component graph and connects consumers according to configured signal pipelines.

Fanout is explicit when one producer feeds multiple consumers/exporters.

### Fanout mutation contract

`internal/fanoutconsumer/traces.go` partitions consumers using `Capabilities().MutatesData`.

For mutating consumers it clones trace data as needed; non-mutating consumers may share the same read-only pdata. All consumers are called and errors are aggregated rather than short-circuiting the remaining consumers.

This is a useful optimization pattern, but the declaration is load-bearing: a component that mutates while claiming read-only can corrupt data another asynchronous consumer believes is stable.

SPARK implication: event/evidence ownership and mutability should be structural where possible. Immutable canonical flight events avoid relying on plugin honesty for shared mutable state.

## Trace B — exporter sender chain

`BaseExporter` builds a sender chain around the actual exporter:

`optional QueueBatch → observability reporting → optional Retry → optional Timeout → actual exporter`

Exact source construction:
- queueing disabled by default unless configured
- retries disabled by default unless configured
- exporter timeout defaults to 5 seconds unless changed
- when queue is enabled, producer `Send()` succeeds after enqueue unless `wait_for_result` is selected; queue consumers perform the later retry/export path.

On synchronous send/enqueue failure, `BaseExporter.Send` logs `Exporting failed. Rejecting data.` and returns the error upstream.

Collector therefore supports either backpressure/error propagation or asynchronous acceptance depending on queue configuration. It does not create an exactly-once semantic guarantee.

## Trace C — queue capacity / backpressure

`queuebatch.Config` exposes:
- `QueueSize`
- `Sizer`: requests/items/bytes
- `NumConsumers`
- `WaitForResult`
- `BlockOnOverflow`
- optional persistent `StorageID`
- optional batching/partitioning.

Rules:
- queue size and consumer count must be positive
- persistent queues do not support `wait_for_result`
- `BlockOnOverflow=false` → full queue returns retryable `ErrQueueIsFull` immediately
- `BlockOnOverflow=true` → producer waits for space or context cancellation
- an individual item larger than memory-queue capacity is rejected
- if `wait_for_result=false`, memory queue removes caller cancellation/deadline from the stored context so receiver cancellation after enqueue does not cancel later asynchronous export.

This demonstrates a core observability tradeoff:
- block/reject preserves pressure visibility upstream
- async enqueue decouples producer latency from exporter health but also means producer success does not mean backend delivery success.

## Trace D — retry semantics

`retrySender` uses exponential backoff around downstream exporter send.

It stops retrying when:
- send succeeds
- downstream error is marked permanent
- backoff has no remaining retry
- `MaxElapsedTime` would be exceeded
- request context deadline would expire before next retry
- request is cancelled/timed out
- Collector is shutting down.

Throttle/retry-after delays can lengthen the normal backoff.

Retry is therefore bounded/reasoned delivery machinery, not durable truth.

## Trace E — persistent sending queue

With `StorageID`, queue entries are encoded to bytes and stored through a Collector storage extension together with queue metadata/read/write indexes. Encoding is explicitly responsible for serializing request **and context**.

Important mechanics:
- item + updated write metadata are persisted together with storage `Batch`
- dispatched items remain stored until downstream processing calls `Done.OnDone`
- currently-dispatched items are tracked so a shutdown/crash can be recovered
- on startup, incomplete dispatched items can be moved back to the queue
- if consumer returns a **shutdown error**, item is deliberately retained for restart
- for any other terminal consume result—including permanent export failure or retry exhaustion—the queue removes the item and frees capacity.

Therefore persistent queue means **delivery state survives supported restarts**, not “telemetry can never be lost.” Terminal exporter errors still retire/drop the queued telemetry, storage decode failures can discard entries, and upstream processors may already have transformed/dropped data before the queue.

SPARK implication: persistent telemetry queue cannot be canonical evidence storage.

## Trace F — queue consumer / completion

`asyncQueue` starts `NumConsumers` goroutines. Each:
→ reads queued request
→ invokes queuebatch consumer
→ consumer invokes next sender/export chain
→ `Done.OnDone(err)` decides queue cleanup.

With batching disabled, `disabledBatcher.Consume` simply calls the downstream sender and passes its error to `Done`.

With multiple consumers, several requests can be in flight concurrently. Queue dispatch is ordered, but completion/export observation can occur out of order due to parallelism/retries. Logical operation order therefore requires an explicit producer-owned event/sequence identity; transport/export order is not enough.

## Trace G — shutdown

Exporter helper shutdown order:
1. stop retry sender
2. shut down queue sender
3. shut down underlying exporter.

QueueBatch shutdown drains the queue, but because retry has already stopped, each remaining request is attempted only once while draining. Persistent queue treats shutdown errors specially so appropriate in-flight items can be retried after restart.

This is pragmatic delivery behavior, not a canonical commit protocol.

## Tests/invariants extracted

Relevant source/tests protect:
- queue capacity and block-vs-reject behavior
- persistent queue metadata/recovery and dispatched-item handling
- queue observation metrics
- async queue startup/shutdown
- batch configuration validation
- retry behavior and permanent errors
- fanout cloning/read-only behavior based on mutation capability.

The core architecture makes loss/retry/drop semantics explicit enough to derive strong negative requirements for a SPARK flight recorder.

## Boundary findings

### Canonical operation ↔ telemetry producer
Collector begins after telemetry exists. It cannot prove that an unreported operation did not happen. Canonical operation/event creation must be owned by the runtime being observed.

### Producer ↔ queue
Enqueue acknowledgement is not export acknowledgement unless configured to wait for result. This state distinction must remain visible.

### Queue ↔ backend delivery
Persistent queue improves survivability but deletes terminally failed items. “Persisted in sending queue” is not “durably accepted by observability backend,” and neither is canonical execution evidence.

### Fanout ↔ mutability
Consumer-declared mutability controls data cloning. Mutable derived telemetry is acceptable; canonical SPARK events should avoid this class by immutable event records plus derived projections.

### Context ↔ persistence
Persistent queue encoding can serialize request context, but what is preserved is defined by the exporter’s encoding implementation. Required identity/provenance cannot live only in ephemeral Go context; it belongs in the canonical event/data itself.

## Determinization candidates

- bounded asynchronous export queue
- explicit queue capacity/backpressure policy
- retry classification/backoff
- delivery-state metrics
- exporter fanout
- immutable-to-derived projection
- persistent delivery spool after canonical commit
- stable exporter correlation IDs
- redaction/filter processors operating on derived copies.

## Material extracted

PATTERN: canonical event first, telemetry/export projection second.

PATTERN: delivery spool and canonical evidence store are different systems.

PATTERN: queue/retry/backpressure state must be observable as distinct states: recorded, enqueued, delivered, rejected, dropped.

PATTERN: immutable canonical event data avoids fanout mutability hazards.

NEGATIVE LESSON: persistent queue is not an exactly-once/canonical ledger.

NEGATIVE LESSON: absence from exported telemetry never proves absence of an operation.

## Primary disposition

BORROW_PATTERN.

Use OpenTelemetry Collector or OTel-compatible exporters as an **optional downstream observability plane**, not as the source of truth for SPARK execution, authority, evidence or semantic health.
