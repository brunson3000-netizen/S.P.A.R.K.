# Security and containment — study packet

Status: ORGANIZED FOR REVIEW; source qualification remains open.

Enforced process, filesystem, network and sandbox boundaries.

[Full map](../../MCI_RESEARCH_MAP.md) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md) · [Research protocol](../../RESEARCH_PROTOCOL.md)

## Source routing

These are investigation assignments, not claims that each upstream implements the desired behavior.

| Source | Filing role | Existing evidence state | Inspect for MCI |
|---|---|---|---|
| [toolhive](https://github.com/stacklok/toolhive/tree/e532cf07d45fa99f3e4e63819396a3e9c9fd763f) | Home | [FIRST_TRACE_RECORDED](../../research/sources/TOOLHIVE.md) | Declared permissions versus applied confinement and verified package activation |
| [goose](https://github.com/aaif-goose/goose/tree/50666ae0b9a51e260b52b7efbab2e4e020346e94) | Home | [FIRST_TRACE_RECORDED](../../research/sources/GOOSE_SECURITY.md) | Tool inspection, permission modes and execution enforcement |
| [mcp-inspector](https://github.com/modelcontextprotocol/inspector/tree/795b1bb30ac845b7baa7cb3df8ec0b693882ca1d) | Cross-reference from TEST | NOT_TRACED_IN_ACTIVE_LEDGER | Protocol diagnostics, unsafe inputs and reproducible contract probes |
| [wasmtime](https://github.com/bytecodealliance/wasmtime/tree/817c58787f432bcdbbb87679011f72c5bc80dbda) | Home | [FIRST_TRACE_RECORDED](../../research/sources/WASMTIME_WASI.md) | Host-granted WASI capabilities, fuel/epoch limits and cancellation |
| [extism](https://github.com/extism/extism/tree/d5da29759bba88645f886d9e12d3f4e4376df7b3) | Home | [FIRST_TRACE_RECORDED](../../research/sources/EXTISM.md) | Manifest loading authority, host functions and timeout scope |
| [toxiproxy](https://github.com/Shopify/toxiproxy/tree/40f7fd31bee529d824116bd2a11a9e3425e904ec) | Cross-reference from TEST | [FIRST_TRACE_RECORDED](../../research/sources/WIREMOCK_TOXIPROXY.md) | Transport failure injection and recovery evidence |
| [wasmcloud](https://github.com/wasmCloud/wasmCloud/tree/1f82c28e638372fbf5a924c96214a4c3629e99f8) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Capability-provider isolation and host/control-plane dependency cost |
| [nushell](https://github.com/nushell/nushell/tree/b6e6562a9a385df439ae864714c609c60314fde9) | Cross-reference from TOOLS | NOT_TRACED_IN_ACTIVE_LEDGER | Structured command/plugin inputs, outputs and OS authority |
| [openhands-sdk](https://github.com/OpenHands/software-agent-sdk/tree/c37007429be8b4465a83487dc1fd0914df0ea734) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Agent execution, approval gates and runtime isolation |
| [cedar](https://github.com/cedar-policy/cedar/tree/2f4019fd645cc8d4a4c0c1f8bd0280c77d754e28) | Cross-reference from GOV | NOT_TRACED_IN_ACTIVE_LEDGER | Typed policy evaluation, deny/error semantics and caller enforcement seam |
| [opa](https://github.com/open-policy-agent/opa/tree/961849b565d2457b80c168f254325b7d5b91461e) | Cross-reference from GOV | NOT_TRACED_IN_ACTIVE_LEDGER | Policy evaluation, bundle/version lifecycle and enforcement integration |
| [gemini-cli](https://github.com/google-gemini/gemini-cli/tree/9c1b0a610534d6f8120964cf2672c07807d8fc90) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Policy/trust gates, shell execution and child-agent controls |
| [openai-codex](https://github.com/openai/codex/tree/516f2780fd227a80cd9fe89488f5039245090b71) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Host approval, sandbox, session and process lifecycle boundaries |
| [swe-agent](https://github.com/SWE-agent/SWE-agent/tree/3ea751c087f32b16e039a2233dd6eefecef325d5) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Agent action parser, environment boundaries and benchmark failure behavior |
| [cosign](https://github.com/sigstore/cosign/tree/633e8f4303b7db64c369bdfa315374a8bd511ac6) | Cross-reference from AUDIT | NOT_TRACED_IN_ACTIVE_LEDGER | Artifact identity and trust verification; not behavioral safety approval |
| [slsa-verifier](https://github.com/slsa-framework/slsa-verifier/tree/30d0be3bbab553fc51557377baba2f7572dfc212) | Cross-reference from AUDIT | NOT_TRACED_IN_ACTIVE_LEDGER | Source/build provenance verification; maintenance/reference lane |
| [mcp-gateway-registry](https://github.com/agentic-community/mcp-gateway-registry/tree/7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73) | Cross-reference from PROTO | [FIRST_TRACE_RECORDED](../../research/sources/MCP_GATEWAY_REGISTRY.md) | Filtered discovery, call-time authorization and signed hop binding |
| [agentgateway](https://github.com/agentgateway/agentgateway/tree/5e5633bffe6dc5fdd29256648987197f366e462a) | Cross-reference from PROTO | NOT_TRACED_IN_ACTIVE_LEDGER | Gateway authentication, target binding and current-policy enforcement |
| [openziti-mcp-gateway](https://github.com/openziti/mcp-gateway/tree/8f99623d95d2f5223d2fa12b9f125688d8c80bf9) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Network identity/authorization versus tool-level operation authority |
| [e2b-runtime](https://github.com/e2b-dev/runtime/tree/cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497) | Home | IN_PROGRESS | MicroVM lifecycle, snapshot freshness, cleanup and resource enforcement |
| [openclaw](https://github.com/openclaw/openclaw/tree/b0638604941dda7eb80095481576898f875caf08) | Cross-reference from COORD | [INTAKE_ONLY](../../research/candidates/OPENCLAW_HERMES_COMMUNITY_INTAKE_2026-09-14.md) | Persistent agent roles, tool permissions and supervisor limits |
| [hermes-agent](https://github.com/NousResearch/hermes-agent/tree/ee4452991d17534aa561f31ee55596d082aa94e7) | Cross-reference from COORD | [INTAKE_ONLY](../../research/candidates/OPENCLAW_HERMES_COMMUNITY_INTAKE_2026-09-14.md) | Persistent memory, tool gates and worker lifecycle |
| [openshell](https://github.com/NVIDIA/OpenShell/tree/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0) | Home | [INTAKE_ONLY](../../research/candidates/NVIDIA_OPENSHELL_NEMOCLAW_COMMUNITY_INTAKE_2026-09-14.md) | Runtime enforcement of process/filesystem/network policy and revocation |
| [nemoclaw](https://github.com/NVIDIA/NemoClaw/tree/3ea2d5f9a515d4176e0745214743a48ff4868f4c) | Home | [INTAKE_ONLY](../../research/candidates/NVIDIA_OPENSHELL_NEMOCLAW_COMMUNITY_INTAKE_2026-09-14.md) | OpenShell integration, configuration-to-enforcement and router boundaries |

Keep one source study and cross-reference its relevant findings here. For every retained finding record exact pin, module/function, protecting tests, observed limitations, MCI location, dependency cost, Rust fit and one evidence-backed disposition. Required negative tests and completion conditions are in Fable's brief. Do not promote a source to qualified merely because this packet lists it.

## Review focus

Reuse the [containment matrix](../comparison/CONTAINMENT_MATRIX.md) and [recovery addendum](../CONTAINMENT_RECOVERY_ADDENDUM_2026-09-14.md). E2B T-SANDBOX-02 is already active; do not duplicate its researcher. Compare declared versus applied policy, host-side source loading, process/file/network escape surfaces, initialization versus invocation budgets, orphan cleanup and snapshot freshness. Required result: enforced boundary matrix with Linux/Windows/Android support marked verified, documented or unknown per mechanism.
