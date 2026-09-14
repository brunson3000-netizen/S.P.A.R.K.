# Coordination and supervision — study packet

Status: CATEGORY SCREENING ASSESSMENT COMPLETE; INTEGRATION QUALIFICATION OPEN.

Assignment, scheduling, worker lifecycle, durable recovery and cancellation.

[Complete assessment](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md) · [Source map](../../MCI_RESEARCH_MAP.md) · [Coverage](../MCI_REVIEW_COVERAGE_2026-09-14.json) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md)

## Findings and comparison

Use the [coordination comparison](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md#coordination-and-supervision). Temporal contributes durable cancellation intent; LangGraph contributes explicit replay semantics; Gas Town contributes liveness/role failure cases. Codex, ADK, Microsoft and OpenHands are harness/approval references. Supervisor replacement must recover deterministic state and obtain fresh authority.

| Compare | Useful mechanism | Do not infer |
|---|---|---|
| Temporal vs LangGraph | History-backed intent vs checkpoint/interrupt replay | Cancel acknowledged means external effect stopped |
| Gas Town vs Grok | Liveness roles vs coordinator-owned admission/fencing patterns | Process identity or heartbeat means ownership |
| OpenClaw vs Hermes | Policy source diagnostics vs deny-floor ordering | Agent overrides or config errors preserve parent limits |
| Framework products | Approval queues, handoff and typed lifecycle | Framework session state is canonical SWARM authority |

AutoGen and SWE-agent stay reference-only at their maintained/superseded pins. Cloudflare is platform-bound; Cloudways is separate hosted-operations documentation. No framework hierarchy is selected wholesale.

## Source evidence and dispositions

One source home, multiple functional cross-references. A narrow screen is explicitly less evidence than a scoped code study; every qualification state remains open. Reasons, exact pins, modules, dependencies and limitations are in the linked record.

| Source | Filing role | Evidence level | Primary disposition |
|---|---|---|---|
| [goose](../sources/GOOSE_SECURITY.md) | From SEC | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [acp-spec](../sources/ACP_SPEC_MCI_SCREEN_2026-09-14.md) | From PROTO | DOCUMENTATION_SCREEN | BORROW_PATTERN |
| [acp-rust-sdk](../sources/ACP_RUST_SDK_MCI_SCREEN_2026-09-14.md) | From PROTO | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [wasmcloud](../sources/WASMCLOUD_MCI_SCREEN_2026-09-14.md) | From SEC | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [opentelemetry-collector](../sources/OTEL_COLLECTOR.md) | From AUDIT | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [temporal](../sources/TEMPORAL_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openhands-sdk](../sources/OPENHANDS_SDK_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [langgraph](../sources/LANGGRAPH_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [n8n](../sources/N8N_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [microsoft-agent-framework](../sources/MICROSOFT_AGENT_FRAMEWORK_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [autogen](../sources/AUTOGEN_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [cloudflare-agents](../sources/CLOUDFLARE_AGENTS_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [claude-code](../sources/CLAUDE_CODE_MCI_SCREEN_2026-09-14.md) | Home | DOCUMENTATION_SCREEN | ARCHIVE_REFERENCE |
| [gemini-cli](../sources/GEMINI_CLI_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [google-adk-python](../sources/GOOGLE_ADK_PYTHON_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openai-codex](../sources/OPENAI_CODEX_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openai-agents-python](../sources/OPENAI_AGENTS_PYTHON_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_IDEA |
| [swe-agent](../sources/SWE_AGENT_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [e2b-runtime](../sources/E2B_RUNTIME_MCI_SCREEN_2026-09-14.md) | From SEC | DOCUMENTATION_SCREEN — active trace preserved | DEFER |
| [paperclip](../sources/PAPERCLIP_MCI_REVIEW_2026-09-14.md) | From GOV | SCOPED_CODE_STUDY | BORROW_PATTERN |
| [openclaw](../sources/OPENCLAW_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [hermes-agent](../sources/HERMES_AGENT_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [gastown](../sources/GASTOWN_MCI_REVIEW_2026-09-14.md) | Home | SCOPED_CODE_STUDY | BORROW_IDEA |
| [openshell](../sources/OPENSHELL_MCI_REVIEW_2026-09-14.md) | From SEC | SCOPED_CODE_STUDY | BORROW_PATTERN |
| [nemoclaw](../sources/NEMOCLAW_MCI_REVIEW_2026-09-14.md) | From SEC | SCOPED_CODE_STUDY | BORROW_PATTERN |

## Further qualification focus

Compare assignment ownership, leases/fencing, duplicate delivery, bounded fan-out, cancellation, worker death and supervisor restart. Separate primary coordinator decisions from supervisor enforcement and worker execution. Include [Grok Build](../sources/GROK_BUILD.md), [durable capture](../patterns/DURABLE_CAPTURE_RECONCILIATION.md) and [continuation handles](../patterns/CONTINUATION_HANDLES_NOT_GRANTS.md). Cloudways is an operations/UI reference only. Required result: lifecycle comparison and the smallest transferable mechanisms, with cloud services and runtime costs explicit.

