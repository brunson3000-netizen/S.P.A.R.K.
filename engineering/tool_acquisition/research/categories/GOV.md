# Governance and policy — study packet

Status: CATEGORY SCREENING ASSESSMENT COMPLETE; INTEGRATION QUALIFICATION OPEN.

Authority, approvals, delegation limits, spending controls and policy lifecycle.

[Complete assessment](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md) · [Source map](../../MCI_RESEARCH_MAP.md) · [Coverage](../MCI_REVIEW_COVERAGE_2026-09-14.json) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md)

## Findings and comparison

Start with the [governance comparison](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md#governance-and-policy). Cedar, Codex/Grok, ADK/Microsoft, Paperclip and OpenShell/NemoClaw provide distinct policy, approval, budget and enforcement mechanisms. None supplies the complete consumer authority contract.

| Mechanism | Selected evidence | Transfer and qualification limit |
|---|---|---|
| Policy decision | Cedar; OPA comparison | Explicit decision plus diagnostics; error handling is a host policy choice |
| Exact approval | Codex; ADK; Microsoft | Bind identity, action, args, resource and tool/policy revision; same-name replacement is insufficient |
| Budget/stop | Paperclip | Real observed-spend/admission/cancel wiring; prospective reservation and physical stop remain open |
| Enforced bounds | OpenShell; Wasmtime/Extism; NemoClaw | Distinguish requested, applied, rejected and uncertain; no BestEffort success presented as enforced |
| Delegation | Grok; Gas Town; OpenClaw comparison | Disposable agents with fresh grants and fenced attempts; role/config overrides are not authority |
| Gateway admission | agentgateway; OpenZiti; registry | Current subject/target/method and settled args before dispatch; discovery is advisory |
| Audit | Canonical recorder studies; ctx-zip adverse evidence | Preserve original evidence independently of agent report or compressed views |

Immediate negative cases are in [the gap ledger](../comparison/MCI_REVIEW_GAPS_2026-09-14.md). Governance material remains filed here even when its code source has another home. [Grok Build](../sources/GROK_BUILD.md) is supplemental, not one of the 54 source locks. Research records and retrieved memory are evidence; they do not grant authority to alter another project's policies or software.

## Source evidence and dispositions

One source home, multiple functional cross-references. A narrow screen is explicitly less evidence than a scoped code study; every qualification state remains open. Reasons, exact pins, modules, dependencies and limitations are in the linked record.

| Source | Filing role | Evidence level | Primary disposition |
|---|---|---|---|
| [rmcp](../sources/RMCP.md) | From PROTO | PRIOR_FIRST_TRACE_REUSED | REUSE_CODE |
| [toolhive](../sources/TOOLHIVE.md) | From SEC | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [goose](../sources/GOOSE_SECURITY.md) | From SEC | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [wasmtime](../sources/WASMTIME_WASI.md) | From SEC | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [extism](../sources/EXTISM.md) | From SEC | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [acp-spec](../sources/ACP_SPEC_MCI_SCREEN_2026-09-14.md) | From PROTO | DOCUMENTATION_SCREEN | BORROW_PATTERN |
| [acp-rust-sdk](../sources/ACP_RUST_SDK_MCI_SCREEN_2026-09-14.md) | From PROTO | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openhands-sdk](../sources/OPENHANDS_SDK_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [langgraph](../sources/LANGGRAPH_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [n8n](../sources/N8N_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [microsoft-agent-framework](../sources/MICROSOFT_AGENT_FRAMEWORK_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [autogen](../sources/AUTOGEN_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [cedar](../sources/CEDAR_MCI_REVIEW_2026-09-14.md) | Home | SCOPED_CODE_STUDY | BORROW_PATTERN |
| [cloudflare-agents](../sources/CLOUDFLARE_AGENTS_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [opa](../sources/OPA_MCI_REVIEW_2026-09-14.md) | Home | SCOPED_CODE_STUDY | ARCHIVE_REFERENCE |
| [claude-code](../sources/CLAUDE_CODE_MCI_SCREEN_2026-09-14.md) | From COORD | DOCUMENTATION_SCREEN | ARCHIVE_REFERENCE |
| [gemini-cli](../sources/GEMINI_CLI_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [google-adk-python](../sources/GOOGLE_ADK_PYTHON_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openai-codex](../sources/OPENAI_CODEX_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openai-agents-python](../sources/OPENAI_AGENTS_PYTHON_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_IDEA |
| [cosign](../sources/COSIGN_MCI_SCREEN_2026-09-14.md) | From AUDIT | NARROW_CODE_SCREEN | DEFER |
| [in-toto](../sources/IN_TOTO_MCI_SCREEN_2026-09-14.md) | From AUDIT | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [slsa-verifier](../sources/SLSA_VERIFIER_MCI_SCREEN_2026-09-14.md) | From AUDIT | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [ard-spec](../sources/ARD.md) | From PROTO | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [progressive-mcp-guardian](../sources/PROGRESSIVE_MCP_GUARDIAN.md) | From PROTO | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [mcp-gateway-registry](../sources/MCP_GATEWAY_REGISTRY.md) | From PROTO | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [agent-skills-spec](../sources/AGENT_SKILLS.md) | From PROTO | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [agentgateway](../sources/AGENTGATEWAY_MCI_SCREEN_2026-09-14.md) | From PROTO | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [agenttrace](../sources/AGENTTRACE.md) | From AUDIT | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [openziti-mcp-gateway](../sources/OPENZITI_MCP_GATEWAY_MCI_SCREEN_2026-09-14.md) | From SEC | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [e2b-runtime](../sources/E2B_RUNTIME_MCI_SCREEN_2026-09-14.md) | From SEC | DOCUMENTATION_SCREEN — active trace preserved | DEFER |
| [agent-observability](../sources/AGENT_OBSERVABILITY.md) | From AUDIT | PRIOR_FIRST_TRACE_REUSED | ARCHIVE_REFERENCE |
| [paperclip](../sources/PAPERCLIP_MCI_REVIEW_2026-09-14.md) | Home | SCOPED_CODE_STUDY | BORROW_PATTERN |
| [openclaw](../sources/OPENCLAW_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [hermes-agent](../sources/HERMES_AGENT_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [gastown](../sources/GASTOWN_MCI_REVIEW_2026-09-14.md) | From COORD | SCOPED_CODE_STUDY | BORROW_IDEA |
| [openshell](../sources/OPENSHELL_MCI_REVIEW_2026-09-14.md) | From SEC | SCOPED_CODE_STUDY | BORROW_PATTERN |
| [nemoclaw](../sources/NEMOCLAW_MCI_REVIEW_2026-09-14.md) | From SEC | SCOPED_CODE_STUDY | BORROW_PATTERN |



## Prior governance pattern library

These prior pattern records remain available. The new source studies above update the evaluator and community-source evidence; prior records are not silently promoted to runtime qualification. Preserve each source record's evidence limits. None of these mechanisms settles S.W.A.R.M.'s policy.

| MCI concern | Existing material | Transfer assessment | Remaining qualification |
|---|---|---|---|
| Concrete action admission | [Ordered permission pipeline](../patterns/ORDERED_CALLTIME_PERMISSION_PIPELINE.md); [Grok Build source](../sources/GROK_BUILD.md) | Host-owned ordering is a strong pattern candidate; model requests remain proposals | Map current MCI precedence and prove policy-error, remembered-grant and bypass-mode behavior |
| Discovery versus authorization | [Call-time reauthorization](../patterns/FILTERED_DISCOVERY_CALLTIME_REAUTH.md); [gateway study](../sources/MCP_GATEWAY_REGISTRY.md) | Reusable boundary pattern; a visible tool is not a grant | Trace every execution ingress, stale catalog and revoked permission |
| Identity across process boundaries | [Signed hop binding](../patterns/SIGNED_INTERNAL_HOP_BINDING.md) | Bind validated identity, target, method and audience | Replay, wrong audience, expiry, target substitution; MCI trust-root choice remains open |
| Child-agent delegation | [Clamped child authority](../patterns/SUBAGENTS_SHARED_BACKENDS_CLAMPED_AUTHORITY.md) | Useful host admission/lifecycle mechanism | Verify nested delegation cannot widen capability or budget; shared local backends do not imply cross-machine filesystem authority |
| Repository instructions and trust | [Repository trust](../patterns/REPOSITORY_TRUST_GATE.md); [skills boundary](../patterns/SKILLS_TEACH_AUTHORITY_ACTS.md) | Treat loaded instructions/config as external inputs | Trust invalidation after content changes, untrusted instructions, plugin/folder trust separation |
| Policy evaluator | [New Cedar study](../sources/CEDAR_MCI_REVIEW_2026-09-14.md); [OPA study](../sources/OPA_MCI_REVIEW_2026-09-14.md) | Pinned Allow-with-errors behavior rechecked in code and inline test; host still owns enforcement | Execute the shared consumer error/version corpus; current evidence is static |
| Declared versus actual confinement | [Containment matrix](../comparison/CONTAINMENT_MATRIX.md); [recovery addendum](../CONTAINMENT_RECOVERY_ADDENDUM_2026-09-14.md) | Security enforcement is a dependency of credible governance | Validate manifest acquisition before guest isolation, initialization budgets, host callbacks and runtime-version differences |
| Evidence independent of the agent | [Observability matrix](../comparison/OBSERVABILITY_MATRIX.md) | Host canonical recorder separated from sampled telemetry | Crash ordering, missing self-report, redaction, correlation and durable evidence after teardown |
| Package activation | [Provenance pattern](../patterns/PROVENANCE_PIN_BEFORE_ACTIVATION.md) | Separate acquired bytes, verified identity and activation permission | Signature failure, mutable tags, trust-anchor change; verification does not prove safety |


## Combined transfer proposal

[Bound, versioned action before dispatch](../patterns/BOUND_VERSIONED_ACTION_BEFORE_DISPATCH.md) connects evaluator, approval, reservation, enforcement and evidence findings. It is proposed research prior art, not accepted consumer policy or implemented code.
