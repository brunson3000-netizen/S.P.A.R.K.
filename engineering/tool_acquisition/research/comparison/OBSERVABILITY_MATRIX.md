# Observability / Flight-Recorder Comparison Matrix

Status: FIRST CONVERGENCE PASS
Updated: 2026-09-13

Sources:
- AgentTrace `9a10f9aae3bc508ddca83093b2d25aede5ad5bd0`
- agent-observability `2658eef467225f376e2e92dc1465839eda2bc113`
- OpenTelemetry Collector `a35b7a8db49df923c5add3dc34872b6e0b3af683`

| Dimension | AgentTrace | agent-observability | OTel Collector | SPARK implication |
|---|---|---|---|---|
| Primary role | local agent tracing SDK/dashboard | self-reporting MCP observer + fallback proxy | telemetry routing/processing/export platform | canonical recorder must sit before all three classes |
| Evidence source | explicit application instrumentation | model self-reporting or interception | producer telemetry | provenance class is mandatory |
| Independent of observed agent | partly; host/app calls SDK | no in default mode | yes after producer emits telemetry | host runtime should own canonical capture |
| Event hierarchy | run → trace → tool call; parent/link support | session → tool call/decision/audit | OTel signal-specific spans/logs/metrics | host schema should model operation/parent/attempt/tool result explicitly |
| Stable causality | parent IDs + links | weak; proxy lacks request-ID correlation | OTel trace/span IDs if correctly produced | generate stable IDs at runtime, not reconstruct later |
| Concurrency safety | one mutable active trace field is unsafe under overlapping async work | proxy fallback uses global step fallback and broken request/response metadata correlation | fanout/queues are concurrent by design | explicit/task-local/actor-owned context only |
| Raw payload handling | stores full input/output/tool payload JSON | stores full input/output despite claimed cap | depends on producer/processors/exporters | canonical artifacts + digest/ref; bounded redacted preview in events |
| Redaction/privacy | caller responsibility | effectively none in traced storage path | configurable by processors, not inherent truth guarantee | enforce privacy before/export projection; canonical policy remains host-owned |
| Cost/tokens | caller tokens + approximate/model price table | caller tokens + rough fixed cost estimate | whatever producer emits | mark source/method/version/estimated; never billing authority by default |
| “Decision” semantics | explicit trace metadata; not private reasoning capture | auto-synthesizes decision from tool/output | arbitrary attributes/events | separate observed action, reported rationale, derived label, authority decision |
| Local persistence | SQLite/WAL | SQLite/WAL | optional exporter persistent queue/storage extensions | canonical log gets independent durable store |
| Transactional coherence | trace + tool calls + run aggregate transaction | mostly row-by-row | queue metadata/item storage batches; pipeline not canonical transaction | canonical event/artifact commit semantics must be explicit |
| Sampling/drop | rate limiter may skip trace while work runs | missed self-report simply disappears | processors/queues/exporters may reject/drop | absence from telemetry never proves no operation |
| Queue/backpressure | not core | not core | bounded queues block or reject; async enqueue decouples producer | delivery pressure separate from canonical recorder pressure |
| Retry | webhooks secondary | none meaningful | bounded exponential retry; permanent/error exhaustion stops | retry is delivery policy, not evidence retention |
| Persistent delivery queue | no | no | optional; restart survivability | useful downstream spool, not canonical ledger |
| Terminal export failure | trace stays local if already recorded | local DB independent of dashboard | queued item removed after terminal non-shutdown consume error | canonical record must already exist |
| Fanout | dashboard/export functions | dashboard/API | first-class, clone based on mutation capability | canonical event immutable; derived consumers get projections/copies |
| Export fidelity | OTLP export loses some local parent/tool/link semantics | REST/dashboard only | native OTel pipeline | exporter is always a projection, never source of truth |
| Installation safety | library/app integration | postinstall mutates global agent configs + `@latest` command | explicit deployed service/components | acquisition/install/activation/config mutation remain distinct gates |
| Best transfer | event vocabulary, local trace trees, non-blocking telemetry doctrine | negative assurance lessons; intercepted-vs-self-reported distinction | queue/retry/fanout/export architecture | implement host-owned Rust event ledger, export to OTel optionally |
| Disposition | BORROW_PATTERN | ARCHIVE_REFERENCE / BORROW_IDEA | BORROW_PATTERN | REIMPLEMENT_IN_RUST canonical flight recorder |

## Convergence

All three reinforce the same split from different directions:

1. **AgentTrace** shows a useful operator-facing event vocabulary and the value of keeping observability failure from changing application execution.
2. **agent-observability** shows why model self-reporting and post-hoc inference cannot be called independent audit evidence.
3. **OpenTelemetry Collector** shows a mature delivery plane where queueing, retries, fanout, transformation and exporter failure are explicit—and therefore proves why that delivery plane must not be the canonical ledger.

Target SPARK shape:

`host operation / authority decision`
→ `canonical immutable flight event + evidence/artifact refs`
→ `derived projection(s)`
→ `optional OTel queue/process/batch/export`
→ `dashboard/backends`

## State distinctions to preserve

For one event, these are distinct states and may occur in different order/failure conditions:

- operation proposed
- authority evaluated
- operation admitted/rejected
- operation started
- canonical event recorded
- evidence artifact committed
- operation completed/failed/cancelled
- telemetry projection created
- export enqueued
- export delivered
- export dropped/rejected
- external backend indexed/displayed.

No later telemetry state retroactively changes the authoritative operation/evidence state.

## Recommended implementation direction

Build a small Rust flight-recorder contract inside SPARK first. Give it a derived OTel adapter rather than embedding OTel SDK types into the core event schema.

A Collector deployment can then be optional and non-blocking. Its queue/retry/fanout strengths become useful without turning Collector availability or semantics into a runtime dependency.
