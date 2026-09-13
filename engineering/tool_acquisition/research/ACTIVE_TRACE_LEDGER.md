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
Patterns:
- `patterns/DECLARATIVE_WORKLOAD_SECURITY_ENVELOPE.md`
- `patterns/DECLARED_VS_APPLIED_CONFINEMENT.md`
- `patterns/PROVENANCE_PIN_BEFORE_ACTIVATION.md`
Failure record: `failures/TOOLHIVE.md`
Disposition: BORROW_PATTERN; full product not selected for SPARK core.

Confirmed:
- versioned RunConfig makes executable, transport, permissions, network, secrets-by-reference and middleware inspectable/restartable
- creation policy check runs before persistent state or workload start
- explicit network-isolation/topology contradictions can fail rather than silently pretend confinement exists
- runtime/applied state remains separate evidence from declared permission profile
- MCP list/call shaping and Cedar call-time authorization are distinct mechanisms
- authz denies unknown methods and malformed/non-JSON protected requests by default
- project package provenance can be constrained by catalog/lock/key and pinned to durable content/provenance state
- first-use without prior/catalog expectation can still be TOFU
- executable image provenance defaults to warn, not hard failure
- tool-call filter itself passes through on request-body read error
- documented middleware-order gap permits a later mutating webhook to rename a call into a tool excluded by the earlier `--tools` filter; independent call-time authorization is therefore load-bearing.

SPARK implications:
- Tool Contract / RunSpec should contain the complete declared execution + confinement envelope
- activation requires applied-state verification, not just configuration review
- search/list/visibility filters are never authority
- required provenance failures must stop activation; warn-only cannot be accepted evidence
- provenance/content identity must survive mutable names/tags.

## Active trace

### T-RMCP-01 — Official Rust MCP SDK (`modelcontextprotocol/rust-sdk` / rmcp)
State: IN_PROGRESS
Pin: `3075dc9152d4678775f20634fcb467a7b995dbab`
Goal: determine what SPARK should directly reuse versus wrap/reimplement.
Priority traces:
1. client/server initialization + capability negotiation + transport lifecycle
2. tool registration/list/call schemas and request/response typing
3. task/continuation/cancellation/notification semantics at the pinned MCP revision
4. transport identity/session assumptions and failure semantics
5. tests that separate protocol interoperability from host authority.
Standing constraint: MCP discovery/tasks/continuations are protocol state, not grants. SPARK call-time authority remains host-owned.

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

## Current convergence after seven traces

Candidate middle-layer shape:

`trust source → find capability → learn capability → [host reauthorization] → execute capability → compact result`

backed by:

`immutable raw evidence / durable work state → derived searchable index/manifest → bounded retrieval/view`

and a separate teaching plane:

`trusted skill identity + compact metadata → activated instructions → on-demand resources`

with a runtime plane:

`versioned Tool Contract / RunSpec → host policy → launch → applied-state verification → health/quarantine`

The implementation direction is converging on host-owned Rust components for trust/provenance, stable identity, authority, worker lifecycle, durable evidence/work state, compact views and runtime verification.

## Next durable update

Complete T-RMCP-01, then compare its reusable Rust protocol substrate against SPARK’s host-owned authority/lifecycle requirements. After that, resume deterministic utility candidates (ast-grep/tree-sitter, nextest/mutants, wiremock/Toxiproxy) and containment candidates (Goose, Wasmtime, Extism).
