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
Disposition: BORROW_PATTERN with mandatory strengthening: `immutable raw artifact → derived search index → compact agent view`.

### T-SKILL-01 — Agent Skills
State: FIRST TRACE COMPLETE
Source pin: `69ef37e9424c0a7ea9dd2293b559e43ec8176379`
Record: `sources/AGENT_SKILLS.md`
Pattern: `patterns/SKILLS_TEACH_AUTHORITY_ACTS.md`

Confirmed:
- portable skill = directory with required `SKILL.md`; optional scripts/references/assets
- three-tier disclosure: metadata → full instructions → supporting resources
- catalog can be only name/description/location; reference generator uses that shape
- dedicated activation can constrain names, list resources and enforce consent/permissions
- project-level skills conventionally shadow user-level skills
- project skills may be untrusted; client guide recommends a trust gate
- `allowed-tools` exists but is explicitly experimental
- bundled scripts can execute arbitrary ecosystem/package tooling; tool/script authority is outside the spec

SPARK implication:
- prefer `SKILL.md` compatibility as teaching package prior art
- add host-owned canonical identity/provenance/version/trust
- skill activation never grants executable authority
- resource readability and script/tool execution are separate permissions

Disposition: BORROW_PATTERN.

### T-GROKBUILD-01 — xai-org/grok-build
State: IN_PROGRESS
Source pin: `37949780c144e37df692e3d669051a21fec24f20` (main, observed 2026-09-13)
Repository: `xai-org/grok-build`
License: Apache-2.0
Language: Rust
Initial relevance: coding-agent harness/TUI with durable memory, agent-host daemon, MCP startup, workflow pause/stop, folder-trust startup gate, subagents, telemetry and session/workspace controls visible in current source revision metadata.
Goal: trace the actual worker/host/session/tool/memory paths, not feature descriptions. Priority questions: what state is authoritative, how tool authority is gated, how subagents/workflows are bounded, how memory is isolated/durable, how MCP/extensions are loaded, and what the model sees.

## Cross-project hypotheses under test

H1 — Tiny doorway: large capability universes can be exposed through a small discovery/learning/execution surface without materially harming task success.

H2 — Small context, full evidence: agent-visible material can be bounded while canonical original evidence remains reliably retrievable and attributable. Context-compress supports searchable spillover but does not satisfy canonical/full-evidence preservation by itself.

H3 — Discovery is advisory: finding or learning a capability never grants execution authority.

H4 — Skills teach; tools act: procedural knowledge/package formats remain distinct from executable authority. Agent Skills strongly supports the teaching/package half; host policy must own the authority half.

H5 — Deterministic middleware should absorb catalog filtering, evidence indexing, known routing, state bookkeeping and other tasks that do not require model judgment.

H6 — Stable capability identity must be independent of display name, physical location and dynamic credential.

H7 — When authorization crosses a process boundary, downstream components should receive a short-lived target-bound audience-specific assertion rather than trusting mutable forwarding headers.

H8 — Search/chunk indexes are derived acceleration state, not canonical evidence; raw evidence requires separate immutable custody.

H9 — Instruction packages need provenance/trust/version identity separate from their local human-readable name and from executable grants.

## Current convergence after five traces

Candidate middle-layer shape:

`find_capability → learn_capability → [host reauthorization] → execute_capability → compact result`

backed by:

`immutable raw evidence → derived searchable index → bounded retrieval/view`

and a separate teaching plane:

`compact skill metadata → activated instructions → on-demand resources`

where teaching never widens execution authority.

## Next durable update

Trace xai-org/grok-build first, because it is a current Rust coding-agent harness and may provide stronger implementation evidence for session ownership, memory, workflows/subagents, tool/MCP loading and host/model boundaries than the format-oriented sources above.
