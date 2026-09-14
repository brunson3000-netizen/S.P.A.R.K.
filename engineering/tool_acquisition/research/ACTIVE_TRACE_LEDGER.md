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

## Latest completed trace — containment / plugin boundary

Comparison: `comparison/CONTAINMENT_MATRIX.md`
Experiment: `experiments/WASM_PLUGIN_BOUNDARY_EXPERIMENT.md`
Failures: `failures/GOOSE_SECURITY.md`, `failures/EXTISM.md`

Confirmed:
- Goose is an inspection/safety layer, not containment: strongest transfers are monotonic tightening of permission outcomes and deterministic intended-effect/egress extraction; model reviewers remain defense-in-depth and can fail open.
- raw Wasmtime/WASI supplies the clearest Rust capability boundary: no filesystem/network by default, explicit preopens/socket policy/host functions, explicit resource limits, deterministic fuel and independent epoch/wall-clock interruption.
- Extism is a higher-level Wasmtime plugin runtime with convenient manifest/PDK controls, read-only/read-write WASI preopens, HTTP host filtering, timeout/cancel, memory/output limits, and distinct initialization/per-call fuel budgets.
- Extism host-side module acquisition (`Wasm::File` / `Wasm::Url`) happens before guest containment and is enabled by default crate features; source acquisition authority must remain separate from guest runtime authority.
- an Extism manifest host allowlist is hostname-oriented and is not by itself a complete egress contract; redirect/DNS/private-address semantics require adversarial qualification.
- both Wasmtime and Extism allow arbitrary native host functions; every linked host function is an authority-bearing API generated from host-owned grants.
- strongest target shape:
  `proposal → effect inspection → host authorization → verified artifact → generated runtime capabilities → contained execution → applied-authority evidence → result/evidence`.

Current recommendation: use raw Wasmtime/WASI as the first experiment baseline. Promote Extism only if it materially reduces integration/maintenance cost while passing the same authority and evidence invariants.

No containment runtime was installed, activated or adopted by completing this research. The experiment remains NOT RUN.

## Prior completed trace — output reduction

Comparison: `comparison/OUTPUT_REDUCTION_MATRIX.md`
Experiment: `experiments/OUTPUT_REDUCTION_BENCHMARK.md`
Failures: `failures/RTK.md`

Strongest combined shape:
`authorized operation → immutable raw full-digest artifact → RTK-like deterministic reducer → searchable derived index → compact agent view`.

## Active trace

### T-OBS-01 — AgentTrace + agent-observability + OpenTelemetry Collector
State: IN_PROGRESS
Pins:
- AgentTrace `9a10f9aae3bc508ddca83093b2d25aede5ad5bd0`
- agent-observability `2658eef467225f376e2e92dc1465839eda2bc113`
- OpenTelemetry Collector `a35b7a8db49df923c5add3dc34872b6e0b3af683`

Goal: determine the smallest trustworthy flight-recorder/observability contract for SPARK workers, tools and multi-agent trees without turning telemetry into canonical semantic truth.

#### AgentTrace — FIRST TRACE COMPLETE
Record: `sources/AGENTTRACE.md`
Failures: `failures/AGENTTRACE.md`
Disposition: BORROW_PATTERN.

Confirmed:
- local SQLite/WAL schema provides run → trace → tool-call hierarchy, parent IDs, related-trace links, token/cost/latency/error fields and higher-level agent-usage events
- trace row + tool calls + run aggregate update are one SQLite transaction
- observability is explicitly secondary: trace-rate limiting executes the operation but drops the trace; cleanup/alert/webhook failure is prevented from breaking the traced operation
- cost is locally derived from caller-supplied token/model data and approximate pricing; it is not billing truth
- raw prompt/output/tool payloads are stored without automatic PII/secret redaction
- OTLP export is a lossy projection at this pin: parent IDs, tool-call rows and arbitrary trace links are not preserved as OTel causality
- SDK tool-call attribution uses one mutable `activeTraceContext` field rather than async-task-local state; overlapping async traces on one instance can misattribute tool calls.

SPARK implications:
- canonical execution evidence must exist below any sample/drop-capable telemetry
- causal context must be explicit/task-local/actor-owned
- telemetry payloads should prefer artifact digest/reference + bounded sanitized preview
- estimated cost needs pricing/source provenance and cannot own economic authority
- exporters are projections, never the canonical event model.

#### Next source

agent-observability is next. Trace its MCP interception/audit path, persistence model, tool/LLM cost/error coverage, payload/privacy handling and failure behavior before comparing it to AgentTrace.

Priority after that:
- OpenTelemetry Collector receiver → processor → exporter semantics
- stable event identity, causality and backpressure/drop behavior
- flight-recorder schema + experiment/convergence matrix.

Standing doctrine:
- canonical authority/evidence/state remain SPARK-owned
- telemetry is derived observation, never a grant or semantic-health authority
- exporter/collector failure must not alter execution authority or canonical results
- parent/child and tool-call correlation IDs are for causality/evidence navigation, not authorization
- high-volume payloads should be artifact references/digests rather than repeated raw context whenever possible.

## Recovery history

Commit `1aaac8a8e1670f5bf3e659c01dc6f7b8a213dffe` recorded recovery from repository state and identified Extism + containment convergence as unfinished. Subsequent records closed containment and moved the active queue to observability. The recovery record remains in git history.

## Next durable update

Complete agent-observability first trace, then OTel Collector; finish T-OBS-01 with an observability matrix and flight-recorder experiment/schema proposal.
