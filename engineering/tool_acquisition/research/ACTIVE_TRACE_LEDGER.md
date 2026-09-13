# Active Trace Ledger

Status: ACTIVE
Updated: 2026-09-13

Repository records, not conversation history, carry research state. Research method: `../RESEARCH_PROTOCOL.md`.

## P0 trace queue

### T-ARD-01 — Agentic Resource Discovery
State: FIRST TRACE COMPLETE
Source pin: `b76f235a8f461876ad4f1e77abd0eb0eb302b48d`
Record: `sources/ARD.md`
Patterns: `patterns/DISCOVERY_EXECUTION_SEPARATION.md`, `patterns/STABLE_CAPABILITY_IDENTITY.md`
Comparison: `comparison/DISCOVERY_MATRIX.md`

Confirmed:
- ARD is discovery/description, not execution authority
- mandatory REST search floor; pageSize default 10/max 100
- stable domain-anchored resource identifier separate from mutable URL and security principal
- relevance score explicitly cannot mean trust/compliance/safety
- authentication/invocation delegated to native artifact protocol
- federation modes `none`, `referrals`, `auto`
- official conformance tooling checks manifests, well-known resolution and registry search behavior

Gap recorded:
- search result can contain only stable ID, while normative full-entry retrieval by identifier is out of scope; SPARK needs explicit deterministic descriptor resolution for `learn_capability`.

Primary disposition: BORROW_PATTERN.

### T-MCPG-01 — Progressive MCP Guardian
State: FIRST TRACE COMPLETE
Source pin: `4c6a04537b9bc548744b4146168b8fa9896069cb`
Record: `sources/PROGRESSIVE_MCP_GUARDIAN.md`
Pattern: `patterns/PROGRESSIVE_TOOL_DISCLOSURE.md`

Confirmed:
- startup probes upstream `list_tools`
- deterministic allow/block filtering builds `ToolIndex`
- agent sees exactly `search_tools`, `get_schema`, `execute_tool`
- execution checks index membership, audits, forwards, audits result

Gaps recorded:
- discovery index doubles as execution admission
- stored schema not locally enforced before forwarding
- static-header/token-passthrough helper disconnected from active traced path
- bare tool name is catalog key, allowing cross-server name collision

Primary disposition: EXPERIMENT_NOW for progressive disclosure; authority/keying model not adopted.

### T-MCPREG-01 — MCP Gateway & Registry
State: IN_PROGRESS
Goal: trace registration/discovery → identity/access check → routing → audit; separately trace A2A discovery → peer-to-peer handoff.

### T-CTX-01 — context-compress
State: QUEUED
Goal: trace output capture → raw evidence persistence/index → compact reference → search/retrieval; prove or falsify original-evidence recoverability.
Source pin: `59fae35a7b383876a34f84090f6da978e230795a`.

### T-SKILL-01 — Agent Skills
State: QUEUED
Goal: trace discovery → metadata load → `SKILL.md` load → optional scripts/references/assets; identify interoperability and trust/authority gaps.

## Cross-project hypotheses under test

H1 — Tiny doorway: large capability universes can be exposed through a small discovery/learning/execution surface without materially harming task success.

H2 — Small context, full evidence: visible material can be bounded while original evidence remains reliably retrievable and attributable.

H3 — Discovery is advisory: finding or learning a capability never grants execution authority. ARD strongly supports this; MCP Guardian is a negative contrast.

H4 — Skills teach; tools act: procedural knowledge/package formats remain distinct from executable authority.

H5 — Deterministic middleware should absorb catalog filtering, evidence indexing, known routing, state bookkeeping and other tasks that do not require model judgment.

H6 — Stable capability identity must be independent of display name, physical location and dynamic credential.

## Current convergence after two traces

MCP Guardian supplies the clearest tiny interaction loop.
ARD supplies the stronger separation and identity semantics.

Candidate experiment shape:
`find_capability → learn_capability → [host reauthorization] → execute_capability → evidence/result`

## Next durable update

Trace MCP Gateway & Registry’s control-plane and data-plane paths, looking specifically for whether discovery, scope, identity and routing are cleanly separated or accidentally coupled.
