# Failure Lessons — OpenTelemetry Collector

Source pin: `a35b7a8db49df923c5add3dc34872b6e0b3af683`
Status: FIRST TRACE COMPLETE

## OT-001 — Persistent sending queue is not canonical durable evidence

FAILURE MODE: telemetry is treated as safely preserved because it entered a disk-backed sending queue.

CAUSE: persistent queue exists to improve delivery reliability. Once a queued request reaches a terminal non-shutdown downstream result—permanent error, retry exhaustion, deadline/cancel or other consume error—the item is removed from storage. Decode/storage failures can also discard items.

SPARK LESSON: canonical flight events/evidence must commit before the OTel/export delivery spool. A persistent exporter queue may improve eventual telemetry delivery but cannot be the authoritative event ledger.

CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN with canonical/telemetry separation.

## OT-002 — Queue acceptance is not backend delivery

FAILURE MODE: producer success is interpreted as exporter/backend acknowledgement.

CAUSE: with queueing enabled and `wait_for_result=false`, `Send()` returns after successful enqueue. Consumers export later. Persistent queues do not support `wait_for_result` at this pin.

SPARK LESSON: record delivery states distinctly: `CANONICAL_RECORDED`, `EXPORT_ENQUEUED`, `EXPORT_DELIVERED`, `EXPORT_REJECTED/DROPPED`. Never collapse them into “logged.”

CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN.

## OT-003 — Queue pressure can reject or stall telemetry

FAILURE MODE: observability silently disappears under load or stalls the producing path unexpectedly.

CAUSE: bounded queue either returns `ErrQueueIsFull` immediately (`block_on_overflow=false`) or waits for space/context cancellation (`true`). Oversized memory-queue items are rejected.

SPARK LESSON: canonical recorder capacity and telemetry exporter capacity are separate budgets. Export pressure must not be able to erase canonical events or unexpectedly become an execution-authority dependency.

CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN.

## OT-004 — Retry exhaustion and permanent errors retire telemetry

FAILURE MODE: an exporter outage/policy failure is assumed to retry forever.

CAUSE: retry sender stops on permanent errors, retry/backoff exhaustion, maximum elapsed time, request deadline/cancel, or shutdown. In a persistent queue, terminal non-shutdown completion removes the item.

SPARK LESSON: exporter retry policy is bounded delivery policy. Canonical evidence retention must have its own lifecycle and must not depend on exporter retry windows.

CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN.

## OT-005 — Fanout correctness depends on honest mutation declarations

FAILURE MODE: one derived telemetry consumer mutates data another consumer expects to remain unchanged.

CAUSE: Collector clones pdata based on `Capabilities().MutatesData`. Non-mutating consumers may share read-only pdata; mutating consumers receive clones as required.

SPARK LESSON: canonical flight events should be immutable by construction. Derived exporters/processors receive copies/projections so correctness does not depend on extension self-declaration.

CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN.

## OT-006 — Parallel queue consumers can change observation completion order

FAILURE MODE: export arrival order is used as the logical order of operations.

CAUSE: queues can run multiple consumer goroutines. Requests are dequeued in order, but concurrent exports/retries can finish out of order. Batch partitioning can further change grouping/timing.

SPARK LESSON: canonical event IDs and a host-owned per-session/operation sequence are required. Export timestamps/order are delivery observations only.

CONFIDENCE: HIGH as architectural consequence of the source concurrency model.

DISPOSITION: BORROW_PATTERN.

## OT-007 — Required provenance cannot live only in process context

FAILURE MODE: identity/provenance needed to interpret a telemetry event is lost when data crosses queue/process/restart boundaries.

CAUSE: persistent queue relies on exporter-specific `Encoding.Marshal/Unmarshal` to decide what request context is serialized. Context preservation is therefore implementation-specific rather than guaranteed by the queue abstraction.

SPARK LESSON: load-bearing event identity, provenance, actor, operation and artifact references belong in the canonical event schema. Context propagation is supplemental correlation, never the only copy.

CONFIDENCE: HIGH for abstraction boundary; exact preserved fields vary by encoding.

DISPOSITION: BORROW_PATTERN.
