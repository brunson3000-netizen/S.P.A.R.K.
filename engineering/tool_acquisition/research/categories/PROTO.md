# Protocols and discovery — study packet

Status: ORGANIZED FOR REVIEW; source qualification remains open.

MCP/ACP, capability identity, registry, routing and tool teaching.

[Full map](../../MCI_RESEARCH_MAP.md) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md) · [Research protocol](../../RESEARCH_PROTOCOL.md)

## Source routing

These are investigation assignments, not claims that each upstream implements the desired behavior.

| Source | Filing role | Existing evidence state | Inspect for MCI |
|---|---|---|---|
| [cli-anything](https://github.com/HKUDS/CLI-Anything/tree/810c18b0d1ab9b234bc996c9fd999318523a3ef0) | Cross-reference from TOOLS | NOT_TRACED_IN_ACTIVE_LEDGER | Adapter generation, typed command contracts and protecting tests |
| [rmcp](https://github.com/modelcontextprotocol/rust-sdk/tree/3075dc9152d4678775f20634fcb467a7b995dbab) | Home | [FIRST_TRACE_RECORDED](../../research/sources/RMCP.md) | Transport/session lifecycle versus host authorization |
| [toolhive](https://github.com/stacklok/toolhive/tree/e532cf07d45fa99f3e4e63819396a3e9c9fd763f) | Cross-reference from SEC | [FIRST_TRACE_RECORDED](../../research/sources/TOOLHIVE.md) | Declared permissions versus applied confinement and verified package activation |
| [mcp-inspector](https://github.com/modelcontextprotocol/inspector/tree/795b1bb30ac845b7baa7cb3df8ec0b693882ca1d) | Cross-reference from TEST | NOT_TRACED_IN_ACTIVE_LEDGER | Protocol diagnostics, unsafe inputs and reproducible contract probes |
| [acp-spec](https://github.com/agentclientprotocol/agent-client-protocol/tree/ada6b108389a63a2625298f2be3eacde33a1d8c5) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Client-agent permissions, sessions and cancellation contract |
| [acp-rust-sdk](https://github.com/agentclientprotocol/rust-sdk/tree/3a6d0ae88dbaa09fcb74e26761e3643a75b2015e) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Rust ACP transport and permission/cancel boundary behavior |
| [wasmcloud](https://github.com/wasmCloud/wasmCloud/tree/1f82c28e638372fbf5a924c96214a4c3629e99f8) | Cross-reference from SEC | NOT_TRACED_IN_ACTIVE_LEDGER | Capability-provider isolation and host/control-plane dependency cost |
| [nushell](https://github.com/nushell/nushell/tree/b6e6562a9a385df439ae864714c609c60314fde9) | Cross-reference from TOOLS | NOT_TRACED_IN_ACTIVE_LEDGER | Structured command/plugin inputs, outputs and OS authority |
| [microsoft-agent-framework](https://github.com/microsoft/agent-framework/tree/1cd06c5a2058a172eebadf5d9d7c3fa45c519520) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Supervisor/worker handoffs, authority inheritance and recovery |
| [cloudflare-agents](https://github.com/cloudflare/agents/tree/46760e635ce9599add0abbfe6c1a34af0d5d44f1) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Durable actor identity, state and platform-dependent authority |
| [openai-codex](https://github.com/openai/codex/tree/516f2780fd227a80cd9fe89488f5039245090b71) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Host approval, sandbox, session and process lifecycle boundaries |
| [ard-spec](https://github.com/ards-project/ard-spec/tree/b76f235a8f461876ad4f1e77abd0eb0eb302b48d) | Home | [FIRST_TRACE_RECORDED](../../research/sources/ARD.md) | Discovery metadata, identity and limits of declared authority |
| [progressive-mcp-guardian](https://github.com/S1LV3RJ1NX/mcp-guardian/tree/4c6a04537b9bc548744b4146168b8fa9896069cb) | Home | [FIRST_TRACE_RECORDED](../../research/sources/PROGRESSIVE_MCP_GUARDIAN.md) | Progressive tool disclosure and index admission versus call-time policy |
| [mcp-gateway-registry](https://github.com/agentic-community/mcp-gateway-registry/tree/7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73) | Home | [FIRST_TRACE_RECORDED](../../research/sources/MCP_GATEWAY_REGISTRY.md) | Filtered discovery, call-time authorization and signed hop binding |
| [agent-skills-spec](https://github.com/agentskills/agentskills/tree/69ef37e9424c0a7ea9dd2293b559e43ec8176379) | Home | [FIRST_TRACE_RECORDED](../../research/sources/AGENT_SKILLS.md) | Teaching/package metadata versus execution authority and provenance |
| [agentgateway](https://github.com/agentgateway/agentgateway/tree/5e5633bffe6dc5fdd29256648987197f366e462a) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Gateway authentication, target binding and current-policy enforcement |
| [openziti-mcp-gateway](https://github.com/openziti/mcp-gateway/tree/8f99623d95d2f5223d2fa12b9f125688d8c80bf9) | Cross-reference from SEC | NOT_TRACED_IN_ACTIVE_LEDGER | Network identity/authorization versus tool-level operation authority |
| [nemoclaw](https://github.com/NVIDIA/NemoClaw/tree/3ea2d5f9a515d4176e0745214743a48ff4868f4c) | Cross-reference from SEC | [INTAKE_ONLY](../../research/candidates/NVIDIA_OPENSHELL_NEMOCLAW_COMMUNITY_INTAKE_2026-09-14.md) | OpenShell integration, configuration-to-enforcement and router boundaries |

Keep one source study and cross-reference its relevant findings here. For every retained finding record exact pin, module/function, protecting tests, observed limitations, MCI location, dependency cost, Rust fit and one evidence-backed disposition. Required negative tests and completion conditions are in Fable's brief. Do not promote a source to qualified merely because this packet lists it.

## Review focus

Reuse the [discovery matrix](../comparison/DISCOVERY_MATRIX.md). Compare schema validation, identity normalization, progressive disclosure, version negotiation, cancellation, streaming terminal results and backpressure. Include Grok Build's local/remote contract with its uninspected remote-backend limitation. For possible federation, evaluate protocol-only boundaries; do not assume shared filesystems. Required result: transport/discovery/teaching capability comparison with authority kept in the consuming host.
