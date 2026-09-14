# Audit, observability and provenance — study packet

Status: CATEGORY SCREENING ASSESSMENT COMPLETE; INTEGRATION QUALIFICATION OPEN.

Canonical evidence, derived telemetry, artifact origin and verification.

[Complete assessment](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md) · [Source map](../../MCI_RESEARCH_MAP.md) · [Coverage](../MCI_REVIEW_COVERAGE_2026-09-14.json) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md)

## Findings and comparison

Use the [audit comparison](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md#audit-observability-and-provenance) and [observability matrix](../comparison/OBSERVABILITY_MATRIX.md).

| Plane | Candidates | Decision |
|---|---|---|
| Canonical host record | Grok/prior recorder patterns | Independent event/evidence commit, stable causality and immutable artifacts |
| Derived telemetry | AgentTrace; OTel Collector | Useful schema/export patterns; async context and delivery limits stay explicit |
| Model self-report/proxy | agent-observability | Reference/negative evidence; insufficient recording fidelity |
| Signature/provenance | Cosign; in-toto; SLSA Verifier | Demand-gated; signature is not execution permission, and in-toto inspections can execute commands |

Show current state, historical observations, lifetime aggregates and estimates separately in MCI. Retention/redaction and trust-root choices remain consumer decisions.

## Source evidence and dispositions

One source home, multiple functional cross-references. A narrow screen is explicitly less evidence than a scoped code study; every qualification state remains open. Reasons, exact pins, modules, dependencies and limitations are in the linked record.

| Source | Filing role | Evidence level | Primary disposition |
|---|---|---|---|
| [toolhive](../sources/TOOLHIVE.md) | From SEC | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [rtk](../sources/RTK.md) | From TOOLS | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [opentelemetry-collector](../sources/OTEL_COLLECTOR.md) | Home | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [temporal](../sources/TEMPORAL_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [n8n](../sources/N8N_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [opa](../sources/OPA_MCI_REVIEW_2026-09-14.md) | From GOV | SCOPED_CODE_STUDY | ARCHIVE_REFERENCE |
| [google-adk-python](../sources/GOOGLE_ADK_PYTHON_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openai-agents-python](../sources/OPENAI_AGENTS_PYTHON_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_IDEA |
| [cosign](../sources/COSIGN_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | DEFER |
| [in-toto](../sources/IN_TOTO_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [slsa-verifier](../sources/SLSA_VERIFIER_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [mcp-gateway-registry](../sources/MCP_GATEWAY_REGISTRY.md) | From PROTO | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [context-compress](../sources/CONTEXT_COMPRESS.md) | From CONTEXT | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [agentgateway](../sources/AGENTGATEWAY_MCI_SCREEN_2026-09-14.md) | From PROTO | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [agenttrace](../sources/AGENTTRACE.md) | Home | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [e2b-runtime](../sources/E2B_RUNTIME_MCI_SCREEN_2026-09-14.md) | From SEC | DOCUMENTATION_SCREEN — active trace preserved | DEFER |
| [agent-observability](../sources/AGENT_OBSERVABILITY.md) | Home | PRIOR_FIRST_TRACE_REUSED | ARCHIVE_REFERENCE |
| [ctx-zip](../sources/CTX_ZIP_MCI_SCREEN_2026-09-14.md) | From CONTEXT | NARROW_CODE_SCREEN | BORROW_IDEA |
| [paperclip](../sources/PAPERCLIP_MCI_REVIEW_2026-09-14.md) | From GOV | SCOPED_CODE_STUDY | BORROW_PATTERN |
| [gastown](../sources/GASTOWN_MCI_REVIEW_2026-09-14.md) | From COORD | SCOPED_CODE_STUDY | BORROW_IDEA |
| [openshell](../sources/OPENSHELL_MCI_REVIEW_2026-09-14.md) | From SEC | SCOPED_CODE_STUDY | BORROW_PATTERN |

## Further qualification focus

Reuse [observability comparison](../comparison/OBSERVABILITY_MATRIX.md), [canonical projection pattern](../patterns/CANONICAL_EVENT_TELEMETRY_PROJECTION.md) and the proposed flight-recorder experiment. Separate authoritative recording from self-report, intercepted data, derived labels and export delivery. Compare crash persistence, causal identity, redaction, evidence retention and signature/provenance limits. Required result: evidence schema lessons and optional export/provenance candidates. Retention policy remains the consumer's decision.

