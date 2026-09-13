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
Disposition: BORROW_PATTERN.

Confirmed:
- portable skill = directory with required `SKILL.md`; optional scripts/references/assets
- three-tier disclosure: metadata → full instructions → supporting resources
- catalog can be only name/description/location
- project skills may be untrusted and need a trust gate
- `allowed-tools` is experimental
- script/tool execution authority remains client/harness-owned.

### T-GROKBUILD-01 — xai-org/grok-build
State: FIRST TRACE COMPLETE
Source pin: `37949780c144e37df692e3d669051a21fec24f20`
Source revision: `c4ea71cfdbcdb21e32e41bc25a0043d7d4836714`
Repository: `xai-org/grok-build`
License: Apache-2.0
Language: Rust
Record: `sources/GROK_BUILD.md`
Patterns:
- `patterns/ORDERED_CALLTIME_PERMISSION_PIPELINE.md`
- `patterns/REPOSITORY_TRUST_GATE.md`
- `patterns/DURABLE_CAPTURE_RECONCILIATION.md`
- `patterns/SUBAGENTS_SHARED_BACKENDS_CLAMPED_AUTHORITY.md`
Disposition: BORROW_PATTERN; narrow code reuse remains unqualified.

Confirmed:
- host-owned permission machinery evaluates concrete tool calls and preserves hard deny above broad automation modes
- repository-local MCP/LSP/policy/instructions/skills are explicitly treated as behavior-bearing supply-chain input and gated by per-workspace trust before loading
- “no risky project config currently present” is provisional, not durable trust
- memory v2 separates global/workspace topics, immutable observation inbox, protected/generated manifest, durable state DB and lexical index
- existing memory edits require prior read snapshot and reject stale concurrent replacement
- observation capture uses deterministic job identity, leases/fencing, immutable files, outcome hashes and reconciliation across crash windows
- child agents share parent/root filesystem/terminal/hooks/process machinery while permission/resource inheritance is host-resolved and bounded
- Grok Bot schemas expose explicit async continuation/await semantics and prompt-surface budget tests; backend service implementation was not independently present in this checkout, so that server-side behavior is contract evidence only
- remote tool proxies bind session identity and unify progress/terminal streaming through a common host handle.

SPARK implications:
- authority, trust, durable work state and subagent lifecycle should be Rust host state machines/actors, not prompt conventions
- repository content must pass a trust boundary before it can teach or configure executable behavior
- memory/evidence should use canonical durable state plus derived bounded indexes/views
- subagents may share machinery but must not mint broader authority than the parent/host grants
- asynchronous worker turns should use stable continuation handles rather than model polling/re-send behavior.

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

H10 — Repository/project content is a potentially hostile behavior source until a host trust decision admits that scope; absence of behavior-bearing config is not future authorization.

H11 — Durable agent work should use deterministic job identity, lease/fencing, immutable artifacts and replayable reconciliation instead of conversational recovery.

H12 — Child agents should inherit shared host backends and bounded capability state but never become independent roots of authority by default.

## Current convergence after six traces

Candidate middle-layer shape:

`trust source → find capability → learn capability → [host reauthorization] → execute capability → compact result`

backed by:

`immutable raw evidence / durable work state → derived searchable index/manifest → bounded retrieval/view`

and a separate teaching plane:

`trusted skill identity + compact metadata → activated instructions → on-demand resources`

where teaching never widens execution authority.

The implementation direction is converging on host-owned Rust components for:
- trust and provenance
- stable capability identity
- call-time authority
- worker/subagent admission and lifecycle
- durable job/evidence state
- deterministic compact-view generation.

## Next durable update

Resume the remaining high-priority architecture candidates. ToolHive is the strongest next gateway/runtime study; official Rust MCP (`rmcp`) remains the strongest direct Rust protocol-reuse candidate. Neither is adopted by this ledger.
