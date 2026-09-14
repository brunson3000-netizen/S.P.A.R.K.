# Active Trace Ledger

Status: ACTIVE
Updated: 2026-09-13

Repository records, not conversation history, carry research state. Research method: `../RESEARCH_PROTOCOL.md`.

## Completed first traces

| ID | Source | Pin | Record | Primary disposition |
|---|---|---|---|---|
| T-ARD-01 | Agentic Resource Discovery | `b76f235a8f461876ad4f1e77abd0eb0eb302b48d` | `sources/ARD.md` | BORROW_PATTERN |
| T-MCPG-01 | progressive MCP Guardian | `4c6a04537b9bc548744b4146168b8fa9896069cb` | `sources/PROGRESSIVE_MCP_GUARDIAN.md` | EXPERIMENT_NOW |
| T-MCPREG-01 | MCP Gateway & Registry | `7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73` | `sources/MCP_GATEWAY_REGISTRY.md` | BORROW_PATTERN |
| T-CTX-01 | context-compress | `59fae35a7b383876a34f84090f6da978e230795a` | `sources/CONTEXT_COMPRESS.md` | BORROW_PATTERN |
| T-SKILL-01 | Agent Skills | `69ef37e9424c0a7ea9dd2293b559e43ec8176379` | `sources/AGENT_SKILLS.md` | BORROW_PATTERN |
| T-GROKBUILD-01 | xAI grok-build | `37949780c144e37df692e3d669051a21fec24f20` | `sources/GROK_BUILD.md` | BORROW_PATTERN |
| T-TOOLHIVE-01 | ToolHive | `e532cf07d45fa99f3e4e63819396a3e9c9fd763f` | `sources/TOOLHIVE.md` | BORROW_PATTERN |
| T-RMCP-01 | official Rust MCP SDK | `3075dc9152d4678775f20634fcb467a7b995dbab` | `sources/RMCP.md` | REUSE_CODE |
| T-SLICE-01 | ast-grep + Tree-sitter | `45b5eb67...` / `1b8407d1...` | `sources/AST_GREP_TREE_SITTER.md` | EXPERIMENT_NOW |
| T-TESTUTIL-01 | cargo-nextest + cargo-mutants | `8527c325...` / `fe82f183...` | `sources/CARGO_NEXTEST_MUTANTS.md` | EXPERIMENT_NOW |
| T-FAULT-01 | wiremock-rs + Toxiproxy | `6b193047...` / `40f7fd31...` | `sources/WIREMOCK_TOXIPROXY.md` | EXPERIMENT_NOW |
| T-RTK-01 | RTK + context-compress | `d0c29851...` / `59fae35a...` | `sources/RTK.md` | EXPERIMENT_NOW |
| T-CONTAIN-01 | Goose + Wasmtime + Extism | `50666ae0...` / `817c5878...` / `d5da2975...` | `sources/GOOSE_SECURITY.md`, `sources/WASMTIME_WASI.md`, `sources/EXTISM.md` | EXPERIMENT_NOW |

## Active trace — T-OBS-01

Sources:
- AgentTrace `9a10f9aae3bc508ddca83093b2d25aede5ad5bd0`
- agent-observability `2658eef467225f376e2e92dc1465839eda2bc113`
- OpenTelemetry Collector `a35b7a8db49df923c5add3dc34872b6e0b3af683`

Goal: determine the smallest trustworthy flight-recorder/observability contract for SPARK workers, tools and multi-agent trees without turning telemetry into canonical semantic truth.

### AgentTrace — FIRST TRACE COMPLETE

Record: `sources/AGENTTRACE.md`
Failures: `failures/AGENTTRACE.md`
Disposition: BORROW_PATTERN.

Confirmed:
- useful run → trace → tool-call vocabulary, parent links and local SQLite/WAL storage
- trace + tool calls + run aggregates are transactionally persisted
- observability is intentionally secondary: rate limiting may drop a trace while the operation executes; alert/webhook failures do not change operation outcome
- cost is caller/price-table derived, not billing truth
- raw input/output/tool payloads are stored without automatic redaction
- OTLP export is a lossy projection at this pin: local parent/tool/link semantics are not preserved
- one mutable `activeTraceContext` is unsafe for overlapping async traces; causal context must be explicit/task-local/actor-owned.

### agent-observability — FIRST TRACE COMPLETE

Record: `sources/AGENT_OBSERVABILITY.md`
Failures: `failures/AGENT_OBSERVABILITY.md`
Disposition: ARCHIVE_REFERENCE / BORROW_IDEA.

Confirmed:
- primary mode is model self-reporting through an MCP observability server; it is annotation, not independent evidence
- each reported tool call automatically creates a synthetic “decision” whose rationale is the output summary/tool name; this is not observed reasoning
- proxy fallback has no JSON-RPC request-ID correlation map: `_obs_*` metadata is attached to request but read from response, so ordinary tool responses lose tool/input/start attribution
- proxy end-to-end test only checks `tools/list`, not `tools/call` correlation
- database stores full JSON input/output without the README’s claimed 64KB output cap; only summary is truncated
- grade is only an error-rate heuristic and cost is a fixed rough token estimate at this pin
- dashboard has no authentication middleware and no explicit loopback bind despite exposing raw session/tool data
- npm postinstall mutates global agent MCP configs and installs an `@latest` runtime command; acquisition/install/activation are improperly conflated.

SPARK implication:
- distinguish `SELF_REPORTED`, `INTERCEPTED`, `HOST_CANONICAL`, and `DERIVED` event provenance
- independent recorder must assign stable event/call IDs and correlate request/response before forwarding
- never synthesize rationale/correctness/billing truth from an observed action without labeling it derived
- enforce payload bounds/redaction/artifact-reference conversion in code
- observability installation must not silently widen or modify another agent’s runtime behavior.

### OpenTelemetry Collector — IN PROGRESS

Next trace:
- receiver → consumer → processor → exporter pipeline
- queue/batch/retry/backpressure/drop semantics
- context/correlation propagation
- error handling and shutdown
- what can be lost or reordered under exporter failure
- why OTel remains a derived projection beneath SPARK canonical state/evidence.

## Standing observability doctrine

- canonical authority/evidence/state remain SPARK-owned
- telemetry is derived observation, never a grant or semantic-health authority
- required canonical event/evidence capture occurs before any sample/drop-capable telemetry path
- exporter/collector failure must not alter execution authority or canonical results
- correlation IDs prove causality/navigation only, not authorization
- large/sensitive payloads should be canonical artifact references/digests plus bounded sanitized previews
- token/cost/grade fields carry source/method/version and are estimates unless backed by an authoritative provider record.

## Next durable update

Complete OpenTelemetry Collector source trace; then finish T-OBS-01 with `comparison/OBSERVABILITY_MATRIX.md` and a host-owned flight-recorder contract/experiment proposal.
