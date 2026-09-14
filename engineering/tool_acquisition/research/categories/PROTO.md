# Protocols and discovery — study packet

Status: CATEGORY SCREENING ASSESSMENT COMPLETE; INTEGRATION QUALIFICATION OPEN.

MCP/ACP, capability identity, registry, routing and tool teaching.

[Complete assessment](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md) · [Source map](../../MCI_RESEARCH_MAP.md) · [Coverage](../MCI_REVIEW_COVERAGE_2026-09-14.json) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md)

## Findings and comparison

Use the [protocol comparison](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md#protocols-and-discovery) and existing [discovery matrix](../comparison/DISCOVERY_MATRIX.md).

| Function | Candidate | Boundary |
|---|---|---|
| MCP wire/lifecycle | rmcp | Session/task/capability state is not a grant or durable work store |
| Compact discovery | ARD; Guardian; registry | Display/index membership never replaces call-time authorization |
| Teaching packages | Agent Skills | Content is external input, not executable authority |
| Client-agent lifecycle | ACP spec and Rust SDK | Optional cancellation and draft/release version differences require an explicit adapter contract |
| Admission/identity | agentgateway; OpenZiti; n8n naming reference | Bind original identity and current policy; don't route solely by display alias |

Maintain a canonical ID/provenance mapping and reauthorize at every execution ingress. Grok's remote tool contract is useful supplemental schema evidence; its hidden remote backend remains uninspected.

## Source evidence and dispositions

One source home, multiple functional cross-references. A narrow screen is explicitly less evidence than a scoped code study; every qualification state remains open. Reasons, exact pins, modules, dependencies and limitations are in the linked record.

| Source | Filing role | Evidence level | Primary disposition |
|---|---|---|---|
| [cli-anything](../sources/CLI_ANYTHING_MCI_SCREEN_2026-09-14.md) | From TOOLS | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [rmcp](../sources/RMCP.md) | Home | PRIOR_FIRST_TRACE_REUSED | REUSE_CODE |
| [toolhive](../sources/TOOLHIVE.md) | From SEC | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [mcp-inspector](../sources/MCP_INSPECTOR_MCI_SCREEN_2026-09-14.md) | From TEST | NARROW_CODE_SCREEN | EXPERIMENT_NOW |
| [acp-spec](../sources/ACP_SPEC_MCI_SCREEN_2026-09-14.md) | Home | DOCUMENTATION_SCREEN | BORROW_PATTERN |
| [acp-rust-sdk](../sources/ACP_RUST_SDK_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [wasmcloud](../sources/WASMCLOUD_MCI_SCREEN_2026-09-14.md) | From SEC | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [nushell](../sources/NUSHELL_MCI_SCREEN_2026-09-14.md) | From TOOLS | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [microsoft-agent-framework](../sources/MICROSOFT_AGENT_FRAMEWORK_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [cloudflare-agents](../sources/CLOUDFLARE_AGENTS_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [openai-codex](../sources/OPENAI_CODEX_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [ard-spec](../sources/ARD.md) | Home | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [progressive-mcp-guardian](../sources/PROGRESSIVE_MCP_GUARDIAN.md) | Home | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [mcp-gateway-registry](../sources/MCP_GATEWAY_REGISTRY.md) | Home | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [agent-skills-spec](../sources/AGENT_SKILLS.md) | Home | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [agentgateway](../sources/AGENTGATEWAY_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openziti-mcp-gateway](../sources/OPENZITI_MCP_GATEWAY_MCI_SCREEN_2026-09-14.md) | From SEC | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [nemoclaw](../sources/NEMOCLAW_MCI_REVIEW_2026-09-14.md) | From SEC | SCOPED_CODE_STUDY | BORROW_PATTERN |

## Further qualification focus

Reuse the [discovery matrix](../comparison/DISCOVERY_MATRIX.md). Compare schema validation, identity normalization, progressive disclosure, version negotiation, cancellation, streaming terminal results and backpressure. Include Grok Build's local/remote contract with its uninspected remote-backend limitation. For possible federation, evaluate protocol-only boundaries; do not assume shared filesystems. Required result: transport/discovery/teaching capability comparison with authority kept in the consuming host.

