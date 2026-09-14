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
| T-OBS-01 | AgentTrace + agent-observability + OpenTelemetry Collector | `9a10f9aa...` / `2658eef4...` / `a35b7a8d...` | `sources/AGENTTRACE.md`, `sources/AGENT_OBSERVABILITY.md`, `sources/OTEL_COLLECTOR.md` | REIMPLEMENT_IN_RUST canonical recorder; OTel downstream |

## Latest completed trace — observability / flight recorder

Comparison: `comparison/OBSERVABILITY_MATRIX.md`
Pattern: `patterns/CANONICAL_EVENT_TELEMETRY_PROJECTION.md`
Experiment/contract: `experiments/SPARK_FLIGHT_RECORDER_V0.md`
Failures:
- `failures/AGENTTRACE.md`
- `failures/AGENT_OBSERVABILITY.md`
- `failures/OTEL_COLLECTOR.md`

### AgentTrace

Confirmed:
- useful run → trace → tool-call vocabulary, parent links and SQLite/WAL storage
- trace + tool calls + run aggregate update transactionally persisted
- observability is intentionally secondary; rate limiting may drop a trace while work executes
- one mutable `activeTraceContext` is unsafe for overlapping async traces
- raw payload duplication/redaction risk
- local OTLP export is a lossy projection of richer local causality
- token/cost fields are derived observations, not billing truth.

### agent-observability

Confirmed:
- default path is model self-reporting, not independent audit
- synthetic “decision rationale” is derived from tool/output after the fact
- fallback proxy lacks request-ID correlation for `tools/call`
- claimed payload bound is not enforced in database path
- dashboard/access/install behavior has assurance/supply-chain gaps
- useful mainly as negative evidence for recorder design.

### OpenTelemetry Collector

Confirmed:
- component graph and fanout are mature derived-telemetry machinery
- fanout clones according to consumer-declared mutation capability and aggregates errors without stopping other consumers
- exporter queueing/retry are optional; queueing and retries are disabled by default in helper unless enabled
- bounded queue can block or reject on overflow
- asynchronous enqueue acknowledgement is distinct from backend delivery
- retry stops on permanent error, retry exhaustion/max elapsed time, deadline/cancel or shutdown
- persistent sending queue stores request + encoding-defined context and recovers supported in-flight shutdown/restart states
- terminal non-shutdown consume error removes the persistent queue item: persistent delivery queue is **not** canonical evidence
- multiple consumers/retries can complete out of logical operation order; producer-owned event/sequence identity is required.

### Convergence

Target SPARK architecture:

`host operation / authority`
→ `canonical immutable flight event + evidence/artifact references`
→ `derived bounded telemetry projection`
→ `optional OTel queue/process/batch/export`
→ `dashboard/backend`.

Core rules:
- canonical event/evidence commit occurs before sample/drop-capable telemetry
- telemetry/exporter state never becomes authority or semantic truth
- event provenance distinguishes `HOST_CANONICAL`, `INTERCEPTED`, `SELF_REPORTED`, `DERIVED`
- causal context is explicit/task-local/actor-owned, never one global mutable “current trace”
- large/sensitive payloads remain artifact references/digests with bounded sanitized previews
- export states (`ENQUEUED`, `DELIVERED`, `DROPPED`, etc.) remain distinct from canonical-recorded state
- OTel IDs are correlation aliases, not canonical runtime identity.

Disposition:
- canonical recorder: **REIMPLEMENT_IN_RUST** inside authoritative SPARK runtime
- OTel/Collector: **BORROW_PATTERN / optional downstream integration**
- no recorder/collector implementation was activated by this research; experiment is NOT RUN.

## Active trace

### T-SANDBOX-02 — E2B Runtime / disposable microVM prior art
State: IN PROGRESS
Source:
- E2B Runtime `e2b-dev/runtime`
- pin `cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497`
- category: disposable sandbox runtime / Firecracker microVM prior art

Goal: determine which disposable-machine mechanisms transfer to SPARK’s sandbox-cell/runtime theory without importing E2B’s cloud/control-plane assumptions.

Priority traces:
1. sandbox creation → Firecracker/microVM process → filesystem/rootfs → network → execution → shutdown
2. snapshot/resume state: what is actually snapshotted, identity/freshness semantics, and what survives pause/resume
3. host ↔ guest boundary: command/file/env/network APIs, authentication and authority surfaces
4. untrusted code containment: namespaces/cgroups/seccomp/jailer/microVM boundaries, device/filesystem/network exposure
5. lifecycle state machine: create/start/pause/resume/kill/timeout/cleanup, orphan recovery
6. resource limits: CPU/memory/disk/network/process counts and enforcement location
7. evidence/observability: execution output, exit status, logs, resource metrics and artifact export
8. failure semantics and tests: what maintainers protect around escape, stale snapshots, cleanup and resource exhaustion
9. transfer classification: direct code/reference versus patterns versus too cloud-specific.

Standing sandbox doctrine:
- “sandbox” means an enforced authority boundary, not a temp directory/subprocess label
- Rust supervisor remains authoritative; disposable runtime is subordinate execution substrate
- sandbox creation does not grant capabilities beyond the host-generated contract
- snapshots/resume handles are state references, not authority grants
- external/cloud identity/control-plane mechanics are not automatically applicable to local SPARK runtime
- evidence must survive sandbox teardown through the canonical artifact/flight-recorder plane.

## Next durable update

Trace E2B creation/execution/lifecycle path end-to-end; record source study + failure lessons before considering OpenZiti/agentgateway zero-trust gateway lane.
