# Context and memory — study packet

Status: ORGANIZED FOR REVIEW; source qualification remains open.

Retrieval, syntax slices, compression, persistent memory and freshness.

[Full map](../../MCI_RESEARCH_MAP.md) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md) · [Research protocol](../../RESEARCH_PROTOCOL.md)

## Source routing

These are investigation assignments, not claims that each upstream implements the desired behavior.

| Source | Filing role | Existing evidence state | Inspect for MCI |
|---|---|---|---|
| [ast-grep](https://github.com/ast-grep/ast-grep/tree/45b5eb6705b4c24e04746137d259874abf1087ad) | Home | [FIRST_TRACE_RECORDED](../../research/sources/AST_GREP_TREE_SITTER.md) | Deterministic syntax queries and bounded code extraction |
| [tree-sitter](https://github.com/tree-sitter/tree-sitter/tree/1b8407d1e718f2a26e2886c03cc55622d8d1d7bd) | Home | [FIRST_TRACE_RECORDED](../../research/sources/AST_GREP_TREE_SITTER.md) | Syntax boundaries, parse errors and extraction fidelity |
| [rtk](https://github.com/rtk-ai/rtk/tree/d0c2985155568d1d76fca03bc65d5098f136bbcd) | Cross-reference from TOOLS | [FIRST_TRACE_RECORDED](../../research/sources/RTK.md) | Compact output with faithful failures and retrievable raw evidence |
| [langgraph](https://github.com/langchain-ai/langgraph/tree/e539ac122f4126f6dd850581c1494948cf620e31) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Checkpoint/resume, human interrupts and stale approval semantics |
| [cloudflare-agents](https://github.com/cloudflare/agents/tree/46760e635ce9599add0abbfe6c1a34af0d5d44f1) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Durable actor identity, state and platform-dependent authority |
| [progressive-mcp-guardian](https://github.com/S1LV3RJ1NX/mcp-guardian/tree/4c6a04537b9bc548744b4146168b8fa9896069cb) | Cross-reference from PROTO | [FIRST_TRACE_RECORDED](../../research/sources/PROGRESSIVE_MCP_GUARDIAN.md) | Progressive tool disclosure and index admission versus call-time policy |
| [context-compress](https://github.com/Open330/context-compress/tree/59fae35a7b383876a34f84090f6da978e230795a) | Home | [FIRST_TRACE_RECORDED](../../research/sources/CONTEXT_COMPRESS.md) | Compression, evidence handles and semantic loss |
| [agent-skills-spec](https://github.com/agentskills/agentskills/tree/69ef37e9424c0a7ea9dd2293b559e43ec8176379) | Cross-reference from PROTO | [FIRST_TRACE_RECORDED](../../research/sources/AGENT_SKILLS.md) | Teaching/package metadata versus execution authority and provenance |
| [ctx-zip](https://github.com/karthikscale3/ctx-zip/tree/76580f7ba1555c891928702743ac74412c7fac60) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Compression fidelity, evidence recovery and workload comparison |
| [openclaw](https://github.com/openclaw/openclaw/tree/b0638604941dda7eb80095481576898f875caf08) | Cross-reference from COORD | [INTAKE_ONLY](../../research/candidates/OPENCLAW_HERMES_COMMUNITY_INTAKE_2026-09-14.md) | Persistent agent roles, tool permissions and supervisor limits |
| [hermes-agent](https://github.com/NousResearch/hermes-agent/tree/ee4452991d17534aa561f31ee55596d082aa94e7) | Cross-reference from COORD | [INTAKE_ONLY](../../research/candidates/OPENCLAW_HERMES_COMMUNITY_INTAKE_2026-09-14.md) | Persistent memory, tool gates and worker lifecycle |

Keep one source study and cross-reference its relevant findings here. For every retained finding record exact pin, module/function, protecting tests, observed limitations, MCI location, dependency cost, Rust fit and one evidence-backed disposition. Required negative tests and completion conditions are in Fable's brief. Do not promote a source to qualified merely because this packet lists it.

## Review focus

Include [Grok Build memory](../sources/GROK_BUILD.md), [syntax slices](../patterns/SYNTAX_BOUNDED_CONTEXT_SLICING.md), [spillover](../patterns/SEARCHABLE_SPILLOVER.md) and existing context/output experiments. Compare fidelity, retrieval of omitted evidence, stale-memory handling, concurrent write rejection, canonical-versus-derived state, payload limits and actual context cost. Required result: information-loss and recovery comparison; memory content never becomes policy authority merely by retrieval.
