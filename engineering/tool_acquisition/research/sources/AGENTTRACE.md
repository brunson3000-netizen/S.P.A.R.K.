# Source Study — AgentTrace

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: AgentTrace
- Upstream: `https://github.com/Klepsiphron/agenttrace`
- Commit: `9a10f9aae3bc508ddca83093b2d25aede5ad5bd0`
- VERSION: `0.4.24`
- License: MIT
- Runtime: TypeScript/Node.js core SDK + Python mirror + SQLite/WAL; CLI/dashboard/middleware integrations
- Primary evidence: `packages/sdk/src/index.ts`, `types.ts`, `storage.ts`, `index.test.ts`, `docs/ARCHITECTURE.md`, `SECURITY.md`.

## Problem framing

PROBLEM: Developers cannot reconstruct multi-step/multi-agent behavior, token/cost use, tool calls and failures reliably from final answers or terminal logs.

USER: agent developers/operators, CI, local dashboards and evaluation tooling.

INPUT: explicitly instrumented run/trace lifecycle calls, token metadata, tool-call records, agent-usage events; framework middleware; CLI wrapping.

OUTPUT: local SQLite run/trace/tool/action records, aggregates, trees, alerts/webhooks, JSON/CSV/OTLP export.

AUTHORITY: Observational only. AgentTrace does not authorize the work it traces. Its SDK deliberately allows execution to continue when tracing is rate-limited, and alert/webhook/cleanup failures are swallowed so they do not become execution authority.

TRUST: Caller-supplied event fields, token counts/model/provider, cost calculator, timestamps and parent IDs. Trace content may include sensitive raw prompts/tool inputs/outputs.

STATE: SQLite/WAL database, retention settings, budgets, webhook config/secrets, alert history, optional JSONL self-tracking log.

FAILURE: recording may be skipped by rate limiter; database/storage writes may fail synchronously; alert/webhook/cleanup failures are designed not to fail the traced operation; retention can delete historical observations.

## Trace A — operation → local flight record

`AgentTrace.trace(name, fn, options)`
→ generate/accept trace ID and parent ID
→ if trace-rate limit exceeded: execute `fn()` and **skip trace recording**
→ establish in-memory active trace context
→ execute agent operation
→ `recordToolCall` appends tool records to active context
→ finally compute latency/cost/status
→ auto-create run if needed
→ SQLite transaction inserts trace + child tool-call rows + increments run aggregate stats
→ optional cleanup
→ alerts checked best-effort
→ webhook delivery fired best-effort
→ return/rethrow original operation result/error.

Important property: the operation result is primary; observability is intentionally secondary.

## Trace B — local data model / causality

Core model:
- `Run` — a session/group of traces and aggregate totals
- `Trace` — one operation with status/input/output/tokens/model/provider/latency/cost/error/metadata/`parentId`
- `ToolCall` — trace-owned input/output/latency/success/error/timestamp
- `AgentUsageRecord` — higher-level named action/session usage
- `trace_links` — arbitrary related-trace edges
- `TraceContext` — `traceId`, optional `parentSpanId`, metadata.

SQLite uses WAL, foreign keys for run→trace and trace→tool-call, parent-id index, and pairwise trace-link table.

`createTrace()` is transactional across the trace row, its tool calls and run aggregate update. This is useful prior art: event persistence and aggregate update should not drift independently.

Tree traversal is cycle-safe in query code, but `parent_id` is not declared as a foreign key, allowing dangling/distributed parent references.

## Trace C — multi-agent context

`createChild(parentContext)` generates a new child trace ID and sets its `parentSpanId` to the parent trace ID. The child context can be passed to another `AgentTrace.trace()` call and stored as `parent_id`.

This is a simple, portable causal-link model and is suitable as observational correlation.

### Important concurrency limit

The TypeScript SDK stores tool-call collection context in a single mutable instance field:

`activeTraceContext: { traceId, toolCalls } | null`

`trace()` saves the previous value, replaces the field, awaits arbitrary user code, then drains/restores it.

That supports ordinary synchronous nesting, but is not async-task-local state. Two overlapping async `trace()` calls on the same `AgentTrace` instance can interleave and attach `recordToolCall()` calls to the wrong active trace or restore stale context.

No AsyncLocalStorage/task-local isolation was found, and the tests inspected cover ordinary in-trace collection rather than overlapping asynchronous traces.

SPARK implication: causal context must be explicit or task-local/actor-owned, never a process-global/single-instance mutable “current span” for concurrent workers.

## Trace D — cost/token semantics

Token usage is caller/integration supplied. Cost is calculated locally from model price tables or a host-provided calculator.

The default table is approximate and falls back to generic rates for unknown models. Therefore:
- token counts are observations supplied by the producer
- computed cost is **derived telemetry**, not billing truth
- pricing version/source must be attributable if cost is used operationally.

Budget/alert decisions in an observability layer must not become hard economic authority without provider/account truth.

## Trace E — OpenTelemetry export

AgentTrace can export OTLP JSON without an OTel SDK dependency.

Current exporter:
- converts each AgentTrace trace into one internal span
- derives OTel trace/span IDs deterministically from the AgentTrace trace ID
- includes status, latency, cost, run ID, token counts, model/provider and metadata
- includes stringified input/output attributes truncated to ~2048 characters.

Important loss of semantics in this exporter:
- stored `parentId` is not emitted as `parentSpanId`
- stored tool calls are not exported as child spans/events
- arbitrary trace links are not exported
- a single AgentTrace trace ID is used as the source for both OTel trace ID and span ID, rather than grouping child AgentTrace traces under one shared OTel trace ID.

Therefore OTLP export at this pin is a useful integration projection, not a lossless representation of the local causal model.

SPARK implication: canonical flight-recorder events should be richer than any one exporter. OTel is a derived projection.

## Trace F — data/privacy boundary

AgentTrace stores full JSON-serialized trace input/output and tool-call input/output in SQLite. It does not automatically redact PII/secrets. Security docs explicitly require callers to redact before tracing.

At-rest protection is delegated to filesystem/disk encryption; webhook secrets are stored plaintext in the database.

A SPARK flight recorder should default to artifact IDs/digests and bounded/sanitized previews for large/sensitive payloads, rather than duplicating arbitrary raw tool/LLM content into every telemetry record.

## Tests/invariants extracted

Tests protect:
- success/error trace capture and original exception rethrow
- tool calls captured even when enclosing trace errors
- tool call outside active trace is not persisted
- retention cleanup behavior
- run/trace storage and parent/child/link tree behavior
- token/cost calculations and custom calculator injection
- schema and local storage behavior.

First-trace gap: no overlapping-async trace-context test was found around the single `activeTraceContext` field.

## Boundary findings

### Execution ↔ telemetry
Strong reference: telemetry can be dropped/degraded without changing execution outcome. This matches SPARK doctrine.

### Event producer ↔ semantic truth
Weak if treated as truth: input/output, token usage, model/provider, parent ID and cost are producer-supplied or derived. They must carry provenance rather than being treated as authoritative state.

### Trace payload ↔ evidence
AgentTrace stores payload copies. SPARK should instead prefer canonical evidence/artifact references plus bounded previews where practical.

### Parent/child correlation ↔ authority
Correlation is useful for causality only. A parent ID proves neither delegation nor grant.

## Determinization candidates

- stable event IDs and event types
- actor/session/run/parent correlation
- monotonic timestamps + duration
- tool-call/result/error envelopes
- token/cost observations with provenance
- artifact references/digests
- retention/batching/export policy
- local append/persist + derived aggregates
- exporter adapters.

## Material extracted

PATTERN: observability must remain non-blocking/derived from canonical execution state.

PATTERN: local flight recorder with run → operation → tool-call hierarchy.

PATTERN: transactional event + aggregate update.

NEGATIVE LESSON: do not use one mutable “active trace” field for concurrent async workers.

NEGATIVE LESSON: an exporter is a projection; missing parent/tool/link information proves it cannot be the canonical record.

## Primary disposition

BORROW_PATTERN.

AgentTrace is a strong interface/schema reference but not a direct foundation for the Rust authoritative core. Its most valuable transfer is the operator-facing event vocabulary and the deliberate separation between execution and telemetry failure.
