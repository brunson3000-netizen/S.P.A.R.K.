# Security and containment — study packet

Status: CATEGORY SCREENING ASSESSMENT COMPLETE; INTEGRATION QUALIFICATION OPEN.

Enforced process, filesystem, network and sandbox boundaries.

[Complete assessment](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md) · [Source map](../../MCI_RESEARCH_MAP.md) · [Coverage](../MCI_REVIEW_COVERAGE_2026-09-14.json) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md)

## Findings and comparison

Use the [security comparison](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md#security-and-containment), prior [containment matrix](../comparison/CONTAINMENT_MATRIX.md) and [recovery addendum](../CONTAINMENT_RECOVERY_ADDENDUM_2026-09-14.md).

| Layer | Candidates | Key limit |
|---|---|---|
| Plugin runtime | Wasmtime baseline; Extism comparison | Host loading/callbacks remain part of the authority boundary |
| OS enforcement | OpenShell | BestEffort may continue without Landlock |
| Network addresses | wasmCloud | Default private-IP allowance; connect-time chain unqualified |
| Tool path admission | OpenZiti | Canonical-path check is not an OS sandbox |
| Applied-policy feedback | NemoClaw | Requirement inclusion permits extra entries; ambiguous writes require reconciliation |
| Disposable VM | E2B | T-SANDBOX-02 remains active; only documentation screen added here |

Platform evidence, including explicit unknown Windows/Android support, is recorded in [the gap ledger](../comparison/MCI_REVIEW_GAPS_2026-09-14.md#platform-evidence-limits). No sandbox was installed or activated.

## Source evidence and dispositions

One source home, multiple functional cross-references. A narrow screen is explicitly less evidence than a scoped code study; every qualification state remains open. Reasons, exact pins, modules, dependencies and limitations are in the linked record.

| Source | Filing role | Evidence level | Primary disposition |
|---|---|---|---|
| [toolhive](../sources/TOOLHIVE.md) | Home | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [goose](../sources/GOOSE_SECURITY.md) | Home | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [mcp-inspector](../sources/MCP_INSPECTOR_MCI_SCREEN_2026-09-14.md) | From TEST | NARROW_CODE_SCREEN | EXPERIMENT_NOW |
| [wasmtime](../sources/WASMTIME_WASI.md) | Home | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [extism](../sources/EXTISM.md) | Home | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [toxiproxy](../sources/WIREMOCK_TOXIPROXY.md) | From TEST | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [wasmcloud](../sources/WASMCLOUD_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [nushell](../sources/NUSHELL_MCI_SCREEN_2026-09-14.md) | From TOOLS | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openhands-sdk](../sources/OPENHANDS_SDK_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [cedar](../sources/CEDAR_MCI_REVIEW_2026-09-14.md) | From GOV | SCOPED_CODE_STUDY | BORROW_PATTERN |
| [opa](../sources/OPA_MCI_REVIEW_2026-09-14.md) | From GOV | SCOPED_CODE_STUDY | ARCHIVE_REFERENCE |
| [gemini-cli](../sources/GEMINI_CLI_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openai-codex](../sources/OPENAI_CODEX_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [swe-agent](../sources/SWE_AGENT_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [cosign](../sources/COSIGN_MCI_SCREEN_2026-09-14.md) | From AUDIT | NARROW_CODE_SCREEN | DEFER |
| [slsa-verifier](../sources/SLSA_VERIFIER_MCI_SCREEN_2026-09-14.md) | From AUDIT | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [mcp-gateway-registry](../sources/MCP_GATEWAY_REGISTRY.md) | From PROTO | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [agentgateway](../sources/AGENTGATEWAY_MCI_SCREEN_2026-09-14.md) | From PROTO | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openziti-mcp-gateway](../sources/OPENZITI_MCP_GATEWAY_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [e2b-runtime](../sources/E2B_RUNTIME_MCI_SCREEN_2026-09-14.md) | Home | DOCUMENTATION_SCREEN — active trace preserved | DEFER |
| [openclaw](../sources/OPENCLAW_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [hermes-agent](../sources/HERMES_AGENT_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [openshell](../sources/OPENSHELL_MCI_REVIEW_2026-09-14.md) | Home | SCOPED_CODE_STUDY | BORROW_PATTERN |
| [nemoclaw](../sources/NEMOCLAW_MCI_REVIEW_2026-09-14.md) | Home | SCOPED_CODE_STUDY | BORROW_PATTERN |

## Further qualification focus

Reuse the [containment matrix](../comparison/CONTAINMENT_MATRIX.md) and [recovery addendum](../CONTAINMENT_RECOVERY_ADDENDUM_2026-09-14.md). E2B T-SANDBOX-02 is already active; do not duplicate its researcher. Compare declared versus applied policy, host-side source loading, process/file/network escape surfaces, initialization versus invocation budgets, orphan cleanup and snapshot freshness. Required result: enforced boundary matrix with Linux/Windows/Android support marked verified, documented or unknown per mechanism.

