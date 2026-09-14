# Governance and policy — study packet

Status: ORGANIZED FOR REVIEW; source qualification remains open.

Authority, approvals, delegation limits, spending controls and policy lifecycle.

[Full map](../../MCI_RESEARCH_MAP.md) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md) · [Research protocol](../../RESEARCH_PROTOCOL.md)

## Source routing

These are investigation assignments, not claims that each upstream implements the desired behavior.

| Source | Filing role | Existing evidence state | Inspect for MCI |
|---|---|---|---|
| [rmcp](https://github.com/modelcontextprotocol/rust-sdk/tree/3075dc9152d4678775f20634fcb467a7b995dbab) | Cross-reference from PROTO | [FIRST_TRACE_RECORDED](../../research/sources/RMCP.md) | Transport/session lifecycle versus host authorization |
| [toolhive](https://github.com/stacklok/toolhive/tree/e532cf07d45fa99f3e4e63819396a3e9c9fd763f) | Cross-reference from SEC | [FIRST_TRACE_RECORDED](../../research/sources/TOOLHIVE.md) | Declared permissions versus applied confinement and verified package activation |
| [goose](https://github.com/aaif-goose/goose/tree/50666ae0b9a51e260b52b7efbab2e4e020346e94) | Cross-reference from SEC | [FIRST_TRACE_RECORDED](../../research/sources/GOOSE_SECURITY.md) | Tool inspection, permission modes and execution enforcement |
| [wasmtime](https://github.com/bytecodealliance/wasmtime/tree/817c58787f432bcdbbb87679011f72c5bc80dbda) | Cross-reference from SEC | [FIRST_TRACE_RECORDED](../../research/sources/WASMTIME_WASI.md) | Host-granted WASI capabilities, fuel/epoch limits and cancellation |
| [extism](https://github.com/extism/extism/tree/d5da29759bba88645f886d9e12d3f4e4376df7b3) | Cross-reference from SEC | [FIRST_TRACE_RECORDED](../../research/sources/EXTISM.md) | Manifest loading authority, host functions and timeout scope |
| [acp-spec](https://github.com/agentclientprotocol/agent-client-protocol/tree/ada6b108389a63a2625298f2be3eacde33a1d8c5) | Cross-reference from PROTO | NOT_TRACED_IN_ACTIVE_LEDGER | Client-agent permissions, sessions and cancellation contract |
| [acp-rust-sdk](https://github.com/agentclientprotocol/rust-sdk/tree/3a6d0ae88dbaa09fcb74e26761e3643a75b2015e) | Cross-reference from PROTO | NOT_TRACED_IN_ACTIVE_LEDGER | Rust ACP transport and permission/cancel boundary behavior |
| [openhands-sdk](https://github.com/OpenHands/software-agent-sdk/tree/c37007429be8b4465a83487dc1fd0914df0ea734) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Agent execution, approval gates and runtime isolation |
| [langgraph](https://github.com/langchain-ai/langgraph/tree/e539ac122f4126f6dd850581c1494948cf620e31) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Checkpoint/resume, human interrupts and stale approval semantics |
| [n8n](https://github.com/n8n-io/n8n/tree/4169b55bf3b3e6c255d7361642bc5243bd04345a) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Workflow permissions, credentials and retry effects; constrained reuse terms |
| [microsoft-agent-framework](https://github.com/microsoft/agent-framework/tree/1cd06c5a2058a172eebadf5d9d7c3fa45c519520) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Supervisor/worker handoffs, authority inheritance and recovery |
| [autogen](https://github.com/microsoft/autogen/tree/027ecf0a379bcc1d09956d46d12d44a3ad9cee14) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Multi-agent termination and coordination failure lessons; reference lane |
| [cedar](https://github.com/cedar-policy/cedar/tree/2f4019fd645cc8d4a4c0c1f8bd0280c77d754e28) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Typed policy evaluation, deny/error semantics and caller enforcement seam |
| [cloudflare-agents](https://github.com/cloudflare/agents/tree/46760e635ce9599add0abbfe6c1a34af0d5d44f1) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Durable actor identity, state and platform-dependent authority |
| [opa](https://github.com/open-policy-agent/opa/tree/961849b565d2457b80c168f254325b7d5b91461e) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Policy evaluation, bundle/version lifecycle and enforcement integration |
| [claude-code](https://github.com/anthropics/claude-code/tree/b5932767f3acbd07da25367064827e5cb81f43de) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Public product/hook reference only; not full open-source implementation |
| [gemini-cli](https://github.com/google-gemini/gemini-cli/tree/9c1b0a610534d6f8120964cf2672c07807d8fc90) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Policy/trust gates, shell execution and child-agent controls |
| [google-adk-python](https://github.com/google/adk-python/tree/460715b6c62c8e9ab00931c502381ee0364e39b6) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Agent/tool callbacks, delegation, runtime limits and evidence |
| [openai-codex](https://github.com/openai/codex/tree/516f2780fd227a80cd9fe89488f5039245090b71) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Host approval, sandbox, session and process lifecycle boundaries |
| [openai-agents-python](https://github.com/openai/openai-agents-python/tree/fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Handoffs, guardrails, approval semantics and tracing provenance |
| [cosign](https://github.com/sigstore/cosign/tree/633e8f4303b7db64c369bdfa315374a8bd511ac6) | Cross-reference from AUDIT | NOT_TRACED_IN_ACTIVE_LEDGER | Artifact identity and trust verification; not behavioral safety approval |
| [in-toto](https://github.com/in-toto/in-toto/tree/e352b43ad7cb8915d84c36d791aa61346152a0a3) | Cross-reference from AUDIT | NOT_TRACED_IN_ACTIVE_LEDGER | Attested step inputs/outputs and evidence-chain validation |
| [slsa-verifier](https://github.com/slsa-framework/slsa-verifier/tree/30d0be3bbab553fc51557377baba2f7572dfc212) | Cross-reference from AUDIT | NOT_TRACED_IN_ACTIVE_LEDGER | Source/build provenance verification; maintenance/reference lane |
| [ard-spec](https://github.com/ards-project/ard-spec/tree/b76f235a8f461876ad4f1e77abd0eb0eb302b48d) | Cross-reference from PROTO | [FIRST_TRACE_RECORDED](../../research/sources/ARD.md) | Discovery metadata, identity and limits of declared authority |
| [progressive-mcp-guardian](https://github.com/S1LV3RJ1NX/mcp-guardian/tree/4c6a04537b9bc548744b4146168b8fa9896069cb) | Cross-reference from PROTO | [FIRST_TRACE_RECORDED](../../research/sources/PROGRESSIVE_MCP_GUARDIAN.md) | Progressive tool disclosure and index admission versus call-time policy |
| [mcp-gateway-registry](https://github.com/agentic-community/mcp-gateway-registry/tree/7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73) | Cross-reference from PROTO | [FIRST_TRACE_RECORDED](../../research/sources/MCP_GATEWAY_REGISTRY.md) | Filtered discovery, call-time authorization and signed hop binding |
| [agent-skills-spec](https://github.com/agentskills/agentskills/tree/69ef37e9424c0a7ea9dd2293b559e43ec8176379) | Cross-reference from PROTO | [FIRST_TRACE_RECORDED](../../research/sources/AGENT_SKILLS.md) | Teaching/package metadata versus execution authority and provenance |
| [agentgateway](https://github.com/agentgateway/agentgateway/tree/5e5633bffe6dc5fdd29256648987197f366e462a) | Cross-reference from PROTO | NOT_TRACED_IN_ACTIVE_LEDGER | Gateway authentication, target binding and current-policy enforcement |
| [agenttrace](https://github.com/Klepsiphron/agenttrace/tree/9a10f9aae3bc508ddca83093b2d25aede5ad5bd0) | Cross-reference from AUDIT | [FIRST_TRACE_RECORDED](../../research/sources/AGENTTRACE.md) | Host instrumentation, context races and persistence versus export |
| [openziti-mcp-gateway](https://github.com/openziti/mcp-gateway/tree/8f99623d95d2f5223d2fa12b9f125688d8c80bf9) | Cross-reference from SEC | NOT_TRACED_IN_ACTIVE_LEDGER | Network identity/authorization versus tool-level operation authority |
| [e2b-runtime](https://github.com/e2b-dev/runtime/tree/cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497) | Cross-reference from SEC | IN_PROGRESS | MicroVM lifecycle, snapshot freshness, cleanup and resource enforcement |
| [agent-observability](https://github.com/KryptosAI/agent-observability/tree/2658eef467225f376e2e92dc1465839eda2bc113) | Cross-reference from AUDIT | [FIRST_TRACE_RECORDED](../../research/sources/AGENT_OBSERVABILITY.md) | Self-report versus independent audit; installation and payload gaps |
| [paperclip](https://github.com/paperclipai/paperclip/tree/13368c518303e886a5c9445fbc69afcbdf0a9228) | Home | [INTAKE_ONLY](../../research/candidates/PAPERCLIP_COMMUNITY_INTAKE_2026-09-14.md) | Approval version binding, concurrent budgets, leases and operator stop |
| [openclaw](https://github.com/openclaw/openclaw/tree/b0638604941dda7eb80095481576898f875caf08) | Cross-reference from COORD | [INTAKE_ONLY](../../research/candidates/OPENCLAW_HERMES_COMMUNITY_INTAKE_2026-09-14.md) | Persistent agent roles, tool permissions and supervisor limits |
| [hermes-agent](https://github.com/NousResearch/hermes-agent/tree/ee4452991d17534aa561f31ee55596d082aa94e7) | Cross-reference from COORD | [INTAKE_ONLY](../../research/candidates/OPENCLAW_HERMES_COMMUNITY_INTAKE_2026-09-14.md) | Persistent memory, tool gates and worker lifecycle |
| [gastown](https://github.com/gastownhall/gastown/tree/649b832b7672bc7a2dbef26f5983aba6198b819b) | Cross-reference from COORD | [INTAKE_ONLY](../../research/candidates/GASTOWN_COMMUNITY_INTAKE_2026-09-14.md) | Coordinator/supervisor roles, recovery, merge and ownership races |
| [openshell](https://github.com/NVIDIA/OpenShell/tree/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0) | Cross-reference from SEC | [INTAKE_ONLY](../../research/candidates/NVIDIA_OPENSHELL_NEMOCLAW_COMMUNITY_INTAKE_2026-09-14.md) | Runtime enforcement of process/filesystem/network policy and revocation |
| [nemoclaw](https://github.com/NVIDIA/NemoClaw/tree/3ea2d5f9a515d4176e0745214743a48ff4868f4c) | Cross-reference from SEC | [INTAKE_ONLY](../../research/candidates/NVIDIA_OPENSHELL_NEMOCLAW_COMMUNITY_INTAKE_2026-09-14.md) | OpenShell integration, configuration-to-enforcement and router boundaries |

Keep one source study and cross-reference its relevant findings here. For every retained finding record exact pin, module/function, protecting tests, observed limitations, MCI location, dependency cost, Rust fit and one evidence-backed disposition. Required negative tests and completion conditions are in Fable's brief. Do not promote a source to qualified merely because this packet lists it.

## Governance findings already available

This is a synthesis of repository research at baseline 7ee1f7f, not a fresh upstream code verification. Preserve each source record's evidence limits. None of these mechanisms settles S.W.A.R.M.'s policy.

| MCI concern | Existing material | Transfer assessment | Remaining qualification |
|---|---|---|---|
| Concrete action admission | [Ordered permission pipeline](../patterns/ORDERED_CALLTIME_PERMISSION_PIPELINE.md); [Grok Build source](../sources/GROK_BUILD.md) | Host-owned ordering is a strong pattern candidate; model requests remain proposals | Map current MCI precedence and prove policy-error, remembered-grant and bypass-mode behavior |
| Discovery versus authorization | [Call-time reauthorization](../patterns/FILTERED_DISCOVERY_CALLTIME_REAUTH.md); [gateway study](../sources/MCP_GATEWAY_REGISTRY.md) | Reusable boundary pattern; a visible tool is not a grant | Trace every execution ingress, stale catalog and revoked permission |
| Identity across process boundaries | [Signed hop binding](../patterns/SIGNED_INTERNAL_HOP_BINDING.md) | Bind validated identity, target, method and audience | Replay, wrong audience, expiry, target substitution; MCI trust-root choice remains open |
| Child-agent delegation | [Clamped child authority](../patterns/SUBAGENTS_SHARED_BACKENDS_CLAMPED_AUTHORITY.md) | Useful host admission/lifecycle mechanism | Verify nested delegation cannot widen capability or budget; shared local backends do not imply cross-machine filesystem authority |
| Repository instructions and trust | [Repository trust](../patterns/REPOSITORY_TRUST_GATE.md); [skills boundary](../patterns/SKILLS_TEACH_AUTHORITY_ACTS.md) | Treat loaded instructions/config as external inputs | Trust invalidation after content changes, untrusted instructions, plugin/folder trust separation |
| Policy evaluator | [Recovered Cedar findings](../../EXTERNAL_TOOL_RESEARCH_CONSOLIDATION.md) | Potential evaluator substrate; host still owns enforcement | Recheck recorded Allow-with-errors behavior at the locked pin; compare OPA with an identical test corpus |
| Declared versus actual confinement | [Containment matrix](../comparison/CONTAINMENT_MATRIX.md); [recovery addendum](../CONTAINMENT_RECOVERY_ADDENDUM_2026-09-14.md) | Security enforcement is a dependency of credible governance | Validate manifest acquisition before guest isolation, initialization budgets, host callbacks and runtime-version differences |
| Evidence independent of the agent | [Observability matrix](../comparison/OBSERVABILITY_MATRIX.md) | Host canonical recorder separated from sampled telemetry | Crash ordering, missing self-report, redaction, correlation and durable evidence after teardown |
| Package activation | [Provenance pattern](../patterns/PROVENANCE_PIN_BEFORE_ACTIVATION.md) | Separate acquired bytes, verified identity and activation permission | Signature failure, mutable tags, trust-anchor change; verification does not prove safety |

## Governance priority for Fable

1. Verify the current MCI consumer requirements and assess existing Grok Build, gateway, ToolHive, Goose, rmcp and observability findings against them. Resolve Grok Build's custody gap; reuse passing evidence unless invalidated.
2. Trace Cedar and OPA policy decisions and error handling. Compare them as evaluators, including the host code needed to enforce a result.
3. Trace Paperclip approvals, budget accounting, agent ownership and operator stop through execution. Its intake currently contains README claims, not protecting code/tests.
4. Inspect OpenShell/NemoClaw enforcement and Gas Town supervision for whether a stop or revocation actually reaches executing work.
5. Screen every remaining GOV-tagged source. Deepen promising distinct mechanisms; record an evidence-backed reference/defer/reject finding for duplicates or weak fits. Carry governance-relevant failures and tests into this packet even when their source home is another category.

## Concrete governance questions

- What authenticates the Operator/agent, and what code binds that identity to the exact action, resource and active policy version?
- Does approval cover the specific bytes/configuration/action, and what invalidates it? Can a changed task reuse an old approval?
- Can lower-level policy, a prompt, skill, memory entry, child definition or convenience mode widen authority?
- Where are budgets reserved before concurrent work, reconciled after uncertain usage, and released after failure? Which limits are enforced versus advisory?
- What does pause, cancel, revoke or terminate actually stop: new admission, queued work, current process, remote request, or only a UI label?
- What happens during evaluator error, timeout, missing identity, disconnected supervisor, restart or lost acknowledgement?
- Which records are authoritative decisions, execution observations, agent claims, or derived telemetry?
- Which functions could be borrowed narrowly into Rust, wrapped behind a process boundary, or used only as invariant/test references?

Do not invent MCI precedence, spending policy, retention periods, trust anchors or federation authority to fill a gap. Record a consumer decision dependency and continue source work that does not depend on it.

## Expected governance result

A comparison by mechanism, a short ranked shortlist, and code-candidate entries containing exact commit/path/symbol, dependencies, license evidence, required host integration, protecting tests and failure limits. Label candidates for reuse separately from copied/compiled/tested/adopted code. This pass copies no upstream implementation.

Initial hypothesis to challenge: governance is a composition of host policy/admission, bounded coordinator lifecycle, enforced execution boundaries and durable evidence. No single framework in the collection has yet been shown to supply the complete MCI governance system.
