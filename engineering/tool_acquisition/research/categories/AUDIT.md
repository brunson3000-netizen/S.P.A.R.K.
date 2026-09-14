# Audit, observability and provenance — study packet

Status: ORGANIZED FOR REVIEW; source qualification remains open.

Canonical evidence, derived telemetry, artifact origin and verification.

[Full map](../../MCI_RESEARCH_MAP.md) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md) · [Research protocol](../../RESEARCH_PROTOCOL.md)

## Source routing

These are investigation assignments, not claims that each upstream implements the desired behavior.

| Source | Filing role | Existing evidence state | Inspect for MCI |
|---|---|---|---|
| [toolhive](https://github.com/stacklok/toolhive/tree/e532cf07d45fa99f3e4e63819396a3e9c9fd763f) | Cross-reference from SEC | [FIRST_TRACE_RECORDED](../../research/sources/TOOLHIVE.md) | Declared permissions versus applied confinement and verified package activation |
| [rtk](https://github.com/rtk-ai/rtk/tree/d0c2985155568d1d76fca03bc65d5098f136bbcd) | Cross-reference from TOOLS | [FIRST_TRACE_RECORDED](../../research/sources/RTK.md) | Compact output with faithful failures and retrievable raw evidence |
| [opentelemetry-collector](https://github.com/open-telemetry/opentelemetry-collector/tree/a35b7a8db49df923c5add3dc34872b6e0b3af683) | Home | [FIRST_TRACE_RECORDED](../../research/sources/OTEL_COLLECTOR.md) | Delivery queues and retry distinct from canonical evidence retention |
| [temporal](https://github.com/temporalio/temporal/tree/9ab3a9f770da20df7d94bcc0030f28eec7b0b947) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Durable workflow replay, duplicate effects, deadlines and recovery |
| [n8n](https://github.com/n8n-io/n8n/tree/4169b55bf3b3e6c255d7361642bc5243bd04345a) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Workflow permissions, credentials and retry effects; constrained reuse terms |
| [opa](https://github.com/open-policy-agent/opa/tree/961849b565d2457b80c168f254325b7d5b91461e) | Cross-reference from GOV | NOT_TRACED_IN_ACTIVE_LEDGER | Policy evaluation, bundle/version lifecycle and enforcement integration |
| [google-adk-python](https://github.com/google/adk-python/tree/460715b6c62c8e9ab00931c502381ee0364e39b6) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Agent/tool callbacks, delegation, runtime limits and evidence |
| [openai-agents-python](https://github.com/openai/openai-agents-python/tree/fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Handoffs, guardrails, approval semantics and tracing provenance |
| [cosign](https://github.com/sigstore/cosign/tree/633e8f4303b7db64c369bdfa315374a8bd511ac6) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Artifact identity and trust verification; not behavioral safety approval |
| [in-toto](https://github.com/in-toto/in-toto/tree/e352b43ad7cb8915d84c36d791aa61346152a0a3) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Attested step inputs/outputs and evidence-chain validation |
| [slsa-verifier](https://github.com/slsa-framework/slsa-verifier/tree/30d0be3bbab553fc51557377baba2f7572dfc212) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Source/build provenance verification; maintenance/reference lane |
| [mcp-gateway-registry](https://github.com/agentic-community/mcp-gateway-registry/tree/7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73) | Cross-reference from PROTO | [FIRST_TRACE_RECORDED](../../research/sources/MCP_GATEWAY_REGISTRY.md) | Filtered discovery, call-time authorization and signed hop binding |
| [context-compress](https://github.com/Open330/context-compress/tree/59fae35a7b383876a34f84090f6da978e230795a) | Cross-reference from CONTEXT | [FIRST_TRACE_RECORDED](../../research/sources/CONTEXT_COMPRESS.md) | Compression, evidence handles and semantic loss |
| [agentgateway](https://github.com/agentgateway/agentgateway/tree/5e5633bffe6dc5fdd29256648987197f366e462a) | Cross-reference from PROTO | NOT_TRACED_IN_ACTIVE_LEDGER | Gateway authentication, target binding and current-policy enforcement |
| [agenttrace](https://github.com/Klepsiphron/agenttrace/tree/9a10f9aae3bc508ddca83093b2d25aede5ad5bd0) | Home | [FIRST_TRACE_RECORDED](../../research/sources/AGENTTRACE.md) | Host instrumentation, context races and persistence versus export |
| [e2b-runtime](https://github.com/e2b-dev/runtime/tree/cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497) | Cross-reference from SEC | IN_PROGRESS | MicroVM lifecycle, snapshot freshness, cleanup and resource enforcement |
| [agent-observability](https://github.com/KryptosAI/agent-observability/tree/2658eef467225f376e2e92dc1465839eda2bc113) | Home | [FIRST_TRACE_RECORDED](../../research/sources/AGENT_OBSERVABILITY.md) | Self-report versus independent audit; installation and payload gaps |
| [ctx-zip](https://github.com/karthikscale3/ctx-zip/tree/76580f7ba1555c891928702743ac74412c7fac60) | Cross-reference from CONTEXT | NOT_TRACED_IN_ACTIVE_LEDGER | Compression fidelity, evidence recovery and workload comparison |
| [paperclip](https://github.com/paperclipai/paperclip/tree/13368c518303e886a5c9445fbc69afcbdf0a9228) | Cross-reference from GOV | [INTAKE_ONLY](../../research/candidates/PAPERCLIP_COMMUNITY_INTAKE_2026-09-14.md) | Approval version binding, concurrent budgets, leases and operator stop |
| [gastown](https://github.com/gastownhall/gastown/tree/649b832b7672bc7a2dbef26f5983aba6198b819b) | Cross-reference from COORD | [INTAKE_ONLY](../../research/candidates/GASTOWN_COMMUNITY_INTAKE_2026-09-14.md) | Coordinator/supervisor roles, recovery, merge and ownership races |
| [openshell](https://github.com/NVIDIA/OpenShell/tree/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0) | Cross-reference from SEC | [INTAKE_ONLY](../../research/candidates/NVIDIA_OPENSHELL_NEMOCLAW_COMMUNITY_INTAKE_2026-09-14.md) | Runtime enforcement of process/filesystem/network policy and revocation |

Keep one source study and cross-reference its relevant findings here. For every retained finding record exact pin, module/function, protecting tests, observed limitations, MCI location, dependency cost, Rust fit and one evidence-backed disposition. Required negative tests and completion conditions are in Fable's brief. Do not promote a source to qualified merely because this packet lists it.

## Review focus

Reuse [observability comparison](../comparison/OBSERVABILITY_MATRIX.md), [canonical projection pattern](../patterns/CANONICAL_EVENT_TELEMETRY_PROJECTION.md) and the proposed flight-recorder experiment. Separate authoritative recording from self-report, intercepted data, derived labels and export delivery. Compare crash persistence, causal identity, redaction, evidence retention and signature/provenance limits. Required result: evidence schema lessons and optional export/provenance candidates. Retention policy remains the consumer's decision.
