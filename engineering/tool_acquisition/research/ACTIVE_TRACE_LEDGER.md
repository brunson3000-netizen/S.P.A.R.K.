# Active Trace Ledger

Status: ACTIVE
Updated: 2026-09-13

Repository records, not conversation history, carry research state. Research method: `../RESEARCH_PROTOCOL.md`.

## Completed first traces

### T-ARD-01 — Agentic Resource Discovery
State: FIRST TRACE COMPLETE
Pin: `b76f235a8f461876ad4f1e77abd0eb0eb302b48d`
Record: `sources/ARD.md`
Patterns: `DISCOVERY_EXECUTION_SEPARATION`, `STABLE_CAPABILITY_IDENTITY`
Disposition: BORROW_PATTERN.

### T-MCPG-01 — Progressive MCP Guardian
State: FIRST TRACE COMPLETE
Pin: `4c6a04537b9bc548744b4146168b8fa9896069cb`
Record: `sources/PROGRESSIVE_MCP_GUARDIAN.md`
Pattern: `PROGRESSIVE_TOOL_DISCLOSURE`
Disposition: EXPERIMENT_NOW for progressive disclosure; authority/keying model rejected.

### T-MCPREG-01 — MCP Gateway & Registry
State: FIRST TRACE COMPLETE
Pin: `7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73`
Record: `sources/MCP_GATEWAY_REGISTRY.md`
Patterns: `FILTERED_DISCOVERY_CALLTIME_REAUTH`, `SIGNED_INTERNAL_HOP_BINDING`
Disposition: BORROW_PATTERN.

### T-CTX-01 — context-compress
State: FIRST TRACE COMPLETE
Pin: `59fae35a7b383876a34f84090f6da978e230795a`
Record: `sources/CONTEXT_COMPRESS.md`
Pattern: `SEARCHABLE_SPILLOVER`
Disposition: BORROW_PATTERN with strengthening: `immutable raw artifact → derived search index → compact agent view`.

### T-SKILL-01 — Agent Skills
State: FIRST TRACE COMPLETE
Pin: `69ef37e9424c0a7ea9dd2293b559e43ec8176379`
Record: `sources/AGENT_SKILLS.md`
Pattern: `SKILLS_TEACH_AUTHORITY_ACTS`
Disposition: BORROW_PATTERN; prefer `SKILL.md` compatibility while host owns provenance/trust/authority.

### T-GROKBUILD-01 — xai-org/grok-build
State: FIRST TRACE COMPLETE
Pin: `37949780c144e37df692e3d669051a21fec24f20`
Source revision: `c4ea71cfdbcdb21e32e41bc25a0043d7d4836714`
Record: `sources/GROK_BUILD.md`
Patterns: `ORDERED_CALLTIME_PERMISSION_PIPELINE`, `REPOSITORY_TRUST_GATE`, `DURABLE_CAPTURE_RECONCILIATION`, `SUBAGENTS_SHARED_BACKENDS_CLAMPED_AUTHORITY`
Disposition: BORROW_PATTERN; narrow code reuse remains unqualified.

### T-TOOLHIVE-01 — ToolHive
State: FIRST TRACE COMPLETE
Pin: `e532cf07d45fa99f3e4e63819396a3e9c9fd763f`
Record: `sources/TOOLHIVE.md`
Patterns: `DECLARATIVE_WORKLOAD_SECURITY_ENVELOPE`, `DECLARED_VS_APPLIED_CONFINEMENT`, `PROVENANCE_PIN_BEFORE_ACTIVATION`
Failure record: `failures/TOOLHIVE.md`
Disposition: BORROW_PATTERN; full product not selected for SPARK core.

### T-RMCP-01 — Official Rust MCP SDK (`rmcp`)
State: FIRST TRACE COMPLETE
Pin: `3075dc9152d4678775f20634fcb467a7b995dbab`
Version/license: rmcp 3.3.0 / Apache-2.0 / Rust 1.88+
Record: `sources/RMCP.md`
Patterns: `PROTOCOL_SUBSTRATE_HOST_AUTHORITY`, `CONTINUATION_HANDLES_NOT_GRANTS`
Failure record: `failures/RMCP.md`
Disposition: REUSE_CODE for MCP protocol substrate; host authority/durable state explicitly excluded.

Confirmed:
- official typed client/server lifecycle, protocol/capability negotiation, request association, cancellation/progress, transports, subscriptions and tool router are strong direct reuse candidates
- typed tool parameter deserialization happens before handler invocation
- 2026-07-28 stateless HTTP is automatic; legacy sessions are adapter-local and extensible through SessionManager
- task extension is capability-gated and tested, but bundled TaskManager state is in-memory rather than crash-durable
- request/task/session IDs are correlation/lifecycle state, never host grants
- default client cache can serve stale responses as successful results on re-fetch failure; authority-sensitive consumers must disable that behavior
- local session request-association marker is non-serialized; cross-process session adapters need an explicit association mechanism
- `initialized` notification is not a security gate
- local ToolRouter name keys can replace duplicate routes; canonical SPARK identity must wrap the wire namespace
- conformance workflow now runs 2025-11-25 + 2026-07-28 client/server suites; old Feb-2026 0.16.0 audit is historical and invalidated for current-pin capability claims.

SPARK implication:
- use rmcp rather than rebuilding MCP wire semantics
- wrap it beneath canonical capability identity, trust, call-time authority, cost/governance, containment, durable work/evidence and supervisor health.

## Active trace

### T-SLICE-01 — ast-grep + tree-sitter deterministic context slicing
State: IN_PROGRESS
Pins:
- ast-grep `45b5eb6705b4c24e04746137d259874abf1087ad`
- tree-sitter `1b8407d1e718f2a26e2886c03cc55622d8d1d7bd`
Goal: determine whether syntax-aware deterministic search/slicing can materially reduce worker context while preserving symbol/structure evidence.
Priority traces:
1. parse/query path and structural match identity
2. language/parser loading and error-tree behavior
3. machine-readable output/ranges and rewrite semantics
4. failure behavior on partial/invalid code
5. benchmark design versus grep + fixed-line windows.
Planned experiment: same repository questions under grep/window vs ast-grep/tree-sitter; measure bytes/tokens delivered, recall of needed definitions/call sites, false context, latency and deterministic reproducibility.

## Cross-project hypotheses under test

H1 — A tiny discovery/learning/execution doorway can scale to a large capability universe.
H2 — Small context requires full canonical evidence outside the prompt.
H3 — Discovery is advisory; execution reauthorizes at call time.
H4 — Skills teach; tools act.
H5 — Deterministic middleware should absorb filtering, indexing, known routing and bookkeeping.
H6 — Stable capability identity is independent of display name/location/credential.
H7 — Cross-process authorization decisions should use bounded target-specific assertions, not mutable headers.
H8 — Search/index state is derived; raw evidence is canonical.
H9 — Instruction packages require provenance/version identity separate from local name and executable grants.
H10 — Repository/project content is untrusted behavior until a host trust gate admits it.
H11 — Durable work uses deterministic identity, leases/fencing, immutable artifacts and reconciliation.
H12 — Child agents share bounded host machinery without becoming new authority roots.
H13 — Declared confinement is not accepted until applied runtime authority is independently observed.
H14 — External executable activation requires an explicit provenance state; warning-only verification is not acceptance.
H15 — Protocol correctness/interoperability belongs below, not inside, canonical authority semantics.
H16 — Continuation/task/session handles are references only and require current authorization on each operation.

## Current convergence after eight traces

`trust source → find capability → learn capability → [host reauthorization] → execute capability → compact result`

backed by:

`immutable raw evidence / durable work state → derived searchable index/manifest → bounded retrieval/view`

teaching plane:

`trusted skill identity + compact metadata → activated instructions → on-demand resources`

runtime plane:

`versioned Tool Contract / RunSpec → host policy → launch → applied-state verification → health/quarantine`

protocol adapters such as rmcp sit **below** canonical identity/authority and **above** transport/wire mechanics.

## Next durable update

Complete ast-grep/tree-sitter first trace and record a concrete context-reduction experiment. Then qualify nextest/mutants, wiremock/Toxiproxy, RTK comparison, and containment candidates Goose/Wasmtime/Extism.
