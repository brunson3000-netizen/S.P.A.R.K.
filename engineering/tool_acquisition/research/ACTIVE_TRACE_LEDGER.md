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
Disposition: BORROW_PATTERN.

### T-MCPG-01 — Progressive MCP Guardian
State: FIRST TRACE COMPLETE
Source pin: `4c6a04537b9bc548744b4146168b8fa9896069cb`
Record: `sources/PROGRESSIVE_MCP_GUARDIAN.md`
Pattern: `patterns/PROGRESSIVE_TOOL_DISCLOSURE.md`
Disposition: EXPERIMENT_NOW for progressive disclosure; authority/keying model not adopted.

### T-MCPREG-01 — MCP Gateway & Registry
State: FIRST TRACE COMPLETE
Source pin: `7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73`
Record: `sources/MCP_GATEWAY_REGISTRY.md`
Patterns: `patterns/FILTERED_DISCOVERY_CALLTIME_REAUTH.md`, `patterns/SIGNED_INTERNAL_HOP_BINDING.md`
Disposition: BORROW_PATTERN.

### T-CTX-01 — context-compress
State: FIRST TRACE COMPLETE
Source pin: `59fae35a7b383876a34f84090f6da978e230795a`
Record: `sources/CONTEXT_COMPRESS.md`
Pattern: `patterns/SEARCHABLE_SPILLOVER.md`

Confirmed path:
- execution captures stdout/stderr under hard timeout/cap
- presentation result is compressed/budgeted separately from `indexableStdout`
- large intent-bearing output is indexed into FTS5 and searched only within the new source ID
- search returns bounded BM25/trigram/fuzzy snippets and is globally response-budgeted

Critical correction:
- indexed corpus is not immutable/raw evidence: ANSI is stripped; capture hard cap can stop later bytes; chunking trims/drops some structure and overlaps some plain-text chunks; default search hit is a snippet (up to ~1500 chars), not a full raw-section read; store is ephemeral by default and maxIndexedSources defaults to 500.
- the upstream phrase “retrieve full content of any section” is stronger than the actual search API guarantee.
- `round-trip.test.ts` proves identical search hits after persistent reopen, not original-input reconstruction.

Security boundary:
- “sandboxed subprocess” is not OS containment. Project security docs explicitly trust the LLM and state no OS-level sandbox + unrestricted outbound network.

Disposition: BORROW_PATTERN for searchable spillover, with mandatory SPARK strengthening:
`immutable raw artifact → derived search index → compact agent view`.

### T-SKILL-01 — Agent Skills
State: IN_PROGRESS
Goal: trace discovery → metadata load → `SKILL.md` load → optional scripts/references/assets; identify minimum interoperable format, progressive loading behavior, and trust/authority gaps.
Source pin: `69ef37e9424c0a7ea9dd2293b559e43ec8176379`.

## Cross-project hypotheses under test

H1 — Tiny doorway: large capability universes can be exposed through a small discovery/learning/execution surface without materially harming task success.

H2 — Small context, full evidence: agent-visible material can be bounded while canonical original evidence remains reliably retrievable and attributable. Context-compress supports searchable spillover but does NOT satisfy canonical/full-evidence preservation by itself.

H3 — Discovery is advisory: finding or learning a capability never grants execution authority.

H4 — Skills teach; tools act: procedural knowledge/package formats remain distinct from executable authority.

H5 — Deterministic middleware should absorb catalog filtering, evidence indexing, known routing, state bookkeeping and other tasks that do not require model judgment.

H6 — Stable capability identity must be independent of display name, physical location and dynamic credential.

H7 — When authorization crosses a process boundary, downstream components should receive a short-lived target-bound audience-specific assertion rather than trusting mutable forwarding headers.

H8 — Search/chunk indexes are derived acceleration state, not canonical evidence; raw evidence requires separate immutable custody.

## Current convergence after four traces

Candidate middle-layer shape:

`find_capability → learn_capability → [host reauthorization] → execute_capability → compact result`

backed by:

`immutable raw evidence → derived searchable index → bounded retrieval/view`

## Next durable update

Complete T-SKILL-01, then synthesize the P0 worker-facing interface and experiment matrix before starting the deeper RTK/source-compression comparison.
