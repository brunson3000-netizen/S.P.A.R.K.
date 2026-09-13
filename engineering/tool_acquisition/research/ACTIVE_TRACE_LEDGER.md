# Active Trace Ledger

Status: ACTIVE
Updated: 2026-09-13

This file is the durable checkpoint for ongoing external-tool reverse engineering. Update it as facts become load-bearing.

## Research method

`../RESEARCH_PROTOCOL.md` is active. Repository records, not conversation history, carry research state.

## P0 trace queue

### T-ARD-01 — Agentic Resource Discovery
State: QUEUED
Goal: trace query → registry/resource match → descriptor retrieval → invocation handoff; identify exact separation between discovery and execution/authority.
Deliverables: source note, pattern card, failure notes, interface burden estimate, disposition.

### T-MCPG-01 — Progressive MCP Guardian
State: IN_PROGRESS
Goal: trace `search_tools` → `get_schema` → `execute_tool` through actual handlers, scope enforcement, upstream call and audit path; inspect tests protecting blocked tools and schema exposure.
Known so far: public agent surface is three meta-tools; source pin `4c6a04537b9bc548744b4146168b8fa9896069cb`.
Open: exact modules/functions and test coverage.

### T-MCPREG-01 — MCP Gateway & Registry
State: QUEUED
Goal: trace capability registration/discovery → identity/access check → routing → audit; separately trace agent discovery → peer-to-peer A2A handoff.

### T-CTX-01 — context-compress
State: QUEUED
Goal: trace command/output capture → raw evidence persistence/index → compression → compact reference → later search/retrieval; prove or falsify byte-level recoverability of originals.
Source pin: `59fae35a7b383876a34f84090f6da978e230795a`.

### T-SKILL-01 — Agent Skills
State: QUEUED
Goal: trace skill discovery → metadata load → `SKILL.md` instruction load → optional resource/script access; identify minimum interoperable format and trust/authority gaps.

## Cross-project hypotheses under test

H1 — Tiny doorway: large capability universes can be exposed through a small discovery/learning/execution surface without materially harming task success.

H2 — Small context, full evidence: agent-visible material can be aggressively bounded while original evidence remains reliably retrievable and attributable.

H3 — Discovery is advisory: finding or learning a capability must not itself grant execution authority.

H4 — Skills teach; tools act: procedural knowledge/package formats should remain distinct from executable authority.

H5 — Deterministic middleware should absorb catalog filtering, evidence indexing, known routing, state bookkeeping, and other tasks that do not require model judgment.

## Next durable update

Record actual file/function/test paths for T-MCPG-01, then create the first pattern card for progressive tool disclosure. Do not advance T-MCPG-01 to COMPLETE until implementation path and failure semantics are evidenced.
