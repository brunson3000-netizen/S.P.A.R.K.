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
- discovery/description is separate from execution authority
- stable domain-anchored resource identifier separate from mutable URL/security principal
- relevance score explicitly cannot mean trust/compliance/safety
- auth/invocation delegated to native protocol
- official conformance tooling
Gap: no normative full-entry lookup by identifier; SPARK needs deterministic descriptor resolution.
Disposition: BORROW_PATTERN.

### T-MCPG-01 — Progressive MCP Guardian
State: FIRST TRACE COMPLETE
Source pin: `4c6a04537b9bc548744b4146168b8fa9896069cb`
Record: `sources/PROGRESSIVE_MCP_GUARDIAN.md`
Pattern: `patterns/PROGRESSIVE_TOOL_DISCLOSURE.md`

Confirmed: three-tool doorway and deterministic allow/block catalog filtering.
Gaps: index-as-authority coupling; no local param/schema enforcement; auth helper integration disconnect; bare-name collisions.
Disposition: EXPERIMENT_NOW for progressive disclosure; authority/keying model not adopted.

### T-MCPREG-01 — MCP Gateway & Registry
State: FIRST TRACE COMPLETE
Source pin: `7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73`
Record: `sources/MCP_GATEWAY_REGISTRY.md`
Patterns: `patterns/FILTERED_DISCOVERY_CALLTIME_REAUTH.md`, `patterns/SIGNED_INTERNAL_HOP_BINDING.md`
Comparison: `comparison/DISCOVERY_MATRIX.md`

Confirmed paths:
1. authenticated semantic search → raw search → per-user asset/tool pruning → backend URL redaction → bounded shaped results
2. MCP request → body capture → `/validate` → concrete method/tool extraction → current scope lookup → method+tool authorization → signed short-lived internal hop token → backend
3. A2A discovery → either direct/default P2P mode or reverse-proxy mode; proxied mode separates gateway credential (`X-Authorization`) from target credential (`Authorization`) and enforces per-agent `invoke_agent`

Key invariants:
- missing/empty tool allowlist fails closed
- uninspectable/malformed privileged body fails closed
- scope repository errors deny
- search visibility does not replace call-time authorization
- caller headers do not replace signed internal claims
- A2A gateway credential must not leak to target agent

Risk recorded:
- compatibility fallback allows non-HTTP MCP methods to be matched through the tools list; method/tool grant namespaces are not fully structural.

Disposition: BORROW_PATTERN; full product not selected for core adoption.

### T-CTX-01 — context-compress
State: IN_PROGRESS
Goal: trace output capture → raw evidence persistence/index → compact reference → search/retrieval; prove or falsify original-evidence recoverability.
Source pin: `59fae35a7b383876a34f84090f6da978e230795a`.

### T-SKILL-01 — Agent Skills
State: QUEUED
Goal: trace discovery → metadata load → `SKILL.md` load → optional scripts/references/assets; identify interoperability and trust/authority gaps.

## Cross-project hypotheses under test

H1 — Tiny doorway: large capability universes can be exposed through a small discovery/learning/execution surface without materially harming task success.

H2 — Small context, full evidence: visible material can be bounded while original evidence remains reliably retrievable and attributable.

H3 — Discovery is advisory: finding or learning a capability never grants execution authority. ARD and MCP Gateway support this; MCP Guardian is a negative contrast.

H4 — Skills teach; tools act: procedural knowledge/package formats remain distinct from executable authority.

H5 — Deterministic middleware should absorb catalog filtering, evidence indexing, known routing, state bookkeeping and other tasks that do not require model judgment.

H6 — Stable capability identity must be independent of display name, physical location and dynamic credential.

H7 — When an authorization decision crosses a process boundary, downstream components should receive a short-lived, target-bound, audience-specific assertion rather than trusting mutable forwarding headers.

## Current convergence after three traces

Candidate middle-layer shape:

`find_capability → learn_capability → [host reauthorization] → execute_capability → evidence/result`

Back-end machinery can be rich; worker-facing surface should stay tiny.

## Next durable update

Complete T-CTX-01 and test the “small context, full evidence” claim at source level: determine exactly what is stored, how it is keyed/indexed, whether complete raw material is retrievable, and what deletion/size/fidelity semantics exist.
