# Coordination and supervision — study packet

Status: ORGANIZED FOR REVIEW; source qualification remains open.

Assignment, scheduling, worker lifecycle, durable recovery and cancellation.

[Full map](../../MCI_RESEARCH_MAP.md) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md) · [Research protocol](../../RESEARCH_PROTOCOL.md)

## Source routing

These are investigation assignments, not claims that each upstream implements the desired behavior.

| Source | Filing role | Existing evidence state | Inspect for MCI |
|---|---|---|---|
| [goose](https://github.com/aaif-goose/goose/tree/50666ae0b9a51e260b52b7efbab2e4e020346e94) | Cross-reference from SEC | [FIRST_TRACE_RECORDED](../../research/sources/GOOSE_SECURITY.md) | Tool inspection, permission modes and execution enforcement |
| [acp-spec](https://github.com/agentclientprotocol/agent-client-protocol/tree/ada6b108389a63a2625298f2be3eacde33a1d8c5) | Cross-reference from PROTO | NOT_TRACED_IN_ACTIVE_LEDGER | Client-agent permissions, sessions and cancellation contract |
| [acp-rust-sdk](https://github.com/agentclientprotocol/rust-sdk/tree/3a6d0ae88dbaa09fcb74e26761e3643a75b2015e) | Cross-reference from PROTO | NOT_TRACED_IN_ACTIVE_LEDGER | Rust ACP transport and permission/cancel boundary behavior |
| [wasmcloud](https://github.com/wasmCloud/wasmCloud/tree/1f82c28e638372fbf5a924c96214a4c3629e99f8) | Cross-reference from SEC | NOT_TRACED_IN_ACTIVE_LEDGER | Capability-provider isolation and host/control-plane dependency cost |
| [opentelemetry-collector](https://github.com/open-telemetry/opentelemetry-collector/tree/a35b7a8db49df923c5add3dc34872b6e0b3af683) | Cross-reference from AUDIT | [FIRST_TRACE_RECORDED](../../research/sources/OTEL_COLLECTOR.md) | Delivery queues and retry distinct from canonical evidence retention |
| [temporal](https://github.com/temporalio/temporal/tree/9ab3a9f770da20df7d94bcc0030f28eec7b0b947) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Durable workflow replay, duplicate effects, deadlines and recovery |
| [openhands-sdk](https://github.com/OpenHands/software-agent-sdk/tree/c37007429be8b4465a83487dc1fd0914df0ea734) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Agent execution, approval gates and runtime isolation |
| [langgraph](https://github.com/langchain-ai/langgraph/tree/e539ac122f4126f6dd850581c1494948cf620e31) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Checkpoint/resume, human interrupts and stale approval semantics |
| [n8n](https://github.com/n8n-io/n8n/tree/4169b55bf3b3e6c255d7361642bc5243bd04345a) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Workflow permissions, credentials and retry effects; constrained reuse terms |
| [microsoft-agent-framework](https://github.com/microsoft/agent-framework/tree/1cd06c5a2058a172eebadf5d9d7c3fa45c519520) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Supervisor/worker handoffs, authority inheritance and recovery |
| [autogen](https://github.com/microsoft/autogen/tree/027ecf0a379bcc1d09956d46d12d44a3ad9cee14) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Multi-agent termination and coordination failure lessons; reference lane |
| [cloudflare-agents](https://github.com/cloudflare/agents/tree/46760e635ce9599add0abbfe6c1a34af0d5d44f1) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Durable actor identity, state and platform-dependent authority |
| [claude-code](https://github.com/anthropics/claude-code/tree/b5932767f3acbd07da25367064827e5cb81f43de) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Public product/hook reference only; not full open-source implementation |
| [gemini-cli](https://github.com/google-gemini/gemini-cli/tree/9c1b0a610534d6f8120964cf2672c07807d8fc90) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Policy/trust gates, shell execution and child-agent controls |
| [google-adk-python](https://github.com/google/adk-python/tree/460715b6c62c8e9ab00931c502381ee0364e39b6) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Agent/tool callbacks, delegation, runtime limits and evidence |
| [openai-codex](https://github.com/openai/codex/tree/516f2780fd227a80cd9fe89488f5039245090b71) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Host approval, sandbox, session and process lifecycle boundaries |
| [openai-agents-python](https://github.com/openai/openai-agents-python/tree/fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Handoffs, guardrails, approval semantics and tracing provenance |
| [swe-agent](https://github.com/SWE-agent/SWE-agent/tree/3ea751c087f32b16e039a2233dd6eefecef325d5) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Agent action parser, environment boundaries and benchmark failure behavior |
| [e2b-runtime](https://github.com/e2b-dev/runtime/tree/cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497) | Cross-reference from SEC | IN_PROGRESS | MicroVM lifecycle, snapshot freshness, cleanup and resource enforcement |
| [paperclip](https://github.com/paperclipai/paperclip/tree/13368c518303e886a5c9445fbc69afcbdf0a9228) | Cross-reference from GOV | [INTAKE_ONLY](../../research/candidates/PAPERCLIP_COMMUNITY_INTAKE_2026-09-14.md) | Approval version binding, concurrent budgets, leases and operator stop |
| [openclaw](https://github.com/openclaw/openclaw/tree/b0638604941dda7eb80095481576898f875caf08) | Home | [INTAKE_ONLY](../../research/candidates/OPENCLAW_HERMES_COMMUNITY_INTAKE_2026-09-14.md) | Persistent agent roles, tool permissions and supervisor limits |
| [hermes-agent](https://github.com/NousResearch/hermes-agent/tree/ee4452991d17534aa561f31ee55596d082aa94e7) | Home | [INTAKE_ONLY](../../research/candidates/OPENCLAW_HERMES_COMMUNITY_INTAKE_2026-09-14.md) | Persistent memory, tool gates and worker lifecycle |
| [gastown](https://github.com/gastownhall/gastown/tree/649b832b7672bc7a2dbef26f5983aba6198b819b) | Home | [INTAKE_ONLY](../../research/candidates/GASTOWN_COMMUNITY_INTAKE_2026-09-14.md) | Coordinator/supervisor roles, recovery, merge and ownership races |
| [openshell](https://github.com/NVIDIA/OpenShell/tree/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0) | Cross-reference from SEC | [INTAKE_ONLY](../../research/candidates/NVIDIA_OPENSHELL_NEMOCLAW_COMMUNITY_INTAKE_2026-09-14.md) | Runtime enforcement of process/filesystem/network policy and revocation |
| [nemoclaw](https://github.com/NVIDIA/NemoClaw/tree/3ea2d5f9a515d4176e0745214743a48ff4868f4c) | Cross-reference from SEC | [INTAKE_ONLY](../../research/candidates/NVIDIA_OPENSHELL_NEMOCLAW_COMMUNITY_INTAKE_2026-09-14.md) | OpenShell integration, configuration-to-enforcement and router boundaries |

Keep one source study and cross-reference its relevant findings here. For every retained finding record exact pin, module/function, protecting tests, observed limitations, MCI location, dependency cost, Rust fit and one evidence-backed disposition. Required negative tests and completion conditions are in Fable's brief. Do not promote a source to qualified merely because this packet lists it.

## Review focus

Compare assignment ownership, leases/fencing, duplicate delivery, bounded fan-out, cancellation, worker death and supervisor restart. Separate primary coordinator decisions from supervisor enforcement and worker execution. Include [Grok Build](../sources/GROK_BUILD.md), [durable capture](../patterns/DURABLE_CAPTURE_RECONCILIATION.md) and [continuation handles](../patterns/CONTINUATION_HANDLES_NOT_GRANTS.md). Cloudways is an operations/UI reference only. Required result: lifecycle comparison and the smallest transferable mechanisms, with cloud services and runtime costs explicit.
