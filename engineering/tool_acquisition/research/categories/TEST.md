# Testing and qualification — study packet

Status: CATEGORY SCREENING ASSESSMENT COMPLETE; INTEGRATION QUALIFICATION OPEN.

Contract tests, fault injection, replay, mutation and comparison experiments.

[Complete assessment](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md) · [Source map](../../MCI_RESEARCH_MAP.md) · [Coverage](../MCI_REVIEW_COVERAGE_2026-09-14.json) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md)

## Findings and comparison

Use the [qualification matrix](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md#testing-and-qualification) and [gap ledger](../comparison/MCI_REVIEW_GAPS_2026-09-14.md).

| Tool/evidence | Current status | Intended qualification use |
|---|---|---|
| wiremock-rs / Toxiproxy | Prior source study; experiment unrun | Observe unauthorized dispatch, delay, loss and retry duplication |
| cargo-nextest / cargo-mutants | Prior source study; experiment unrun | Bounded test execution and targeted invariant falsification |
| MCP Inspector | Handler screen; live tests unrun | Protocol fixtures including automatic extra resource probes |
| ctx-zip local harness | EXECUTED: four adverse assertions reproduced | Evidence overwrite, shared reference, mutation and ignored serializer |
| Upstream tests cited in new cards | READ, NOT RUN | Design evidence only; no hosted/local pass claimed |

No old experiment is promoted to PASS. A source-test count, smoke success or source author's documentation is not independent qualification.

## Source evidence and dispositions

One source home, multiple functional cross-references. A narrow screen is explicitly less evidence than a scoped code study; every qualification state remains open. Reasons, exact pins, modules, dependencies and limitations are in the linked record.

| Source | Filing role | Evidence level | Primary disposition |
|---|---|---|---|
| [cli-anything](../sources/CLI_ANYTHING_MCI_SCREEN_2026-09-14.md) | From TOOLS | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [rmcp](../sources/RMCP.md) | From PROTO | PRIOR_FIRST_TRACE_REUSED | REUSE_CODE |
| [mcp-inspector](../sources/MCP_INSPECTOR_MCI_SCREEN_2026-09-14.md) | Home | NARROW_CODE_SCREEN | EXPERIMENT_NOW |
| [wasmtime](../sources/WASMTIME_WASI.md) | From SEC | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [extism](../sources/EXTISM.md) | From SEC | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [ast-grep](../sources/AST_GREP_TREE_SITTER.md) | From CONTEXT | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [tree-sitter](../sources/AST_GREP_TREE_SITTER.md) | From CONTEXT | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [toxiproxy](../sources/WIREMOCK_TOXIPROXY.md) | Home | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [wiremock-rs](../sources/WIREMOCK_TOXIPROXY.md) | Home | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [cargo-nextest](../sources/CARGO_NEXTEST_MUTANTS.md) | Home | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [cargo-mutants](../sources/CARGO_NEXTEST_MUTANTS.md) | Home | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [rtk](../sources/RTK.md) | From TOOLS | PRIOR_FIRST_TRACE_REUSED | EXPERIMENT_NOW |
| [acp-rust-sdk](../sources/ACP_RUST_SDK_MCI_SCREEN_2026-09-14.md) | From PROTO | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [opentelemetry-collector](../sources/OTEL_COLLECTOR.md) | From AUDIT | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [temporal](../sources/TEMPORAL_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [langgraph](../sources/LANGGRAPH_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [microsoft-agent-framework](../sources/MICROSOFT_AGENT_FRAMEWORK_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [autogen](../sources/AUTOGEN_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [cedar](../sources/CEDAR_MCI_REVIEW_2026-09-14.md) | From GOV | SCOPED_CODE_STUDY | BORROW_PATTERN |
| [opa](../sources/OPA_MCI_REVIEW_2026-09-14.md) | From GOV | SCOPED_CODE_STUDY | ARCHIVE_REFERENCE |
| [swe-agent](../sources/SWE_AGENT_MCI_SCREEN_2026-09-14.md) | From COORD | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [in-toto](../sources/IN_TOTO_MCI_SCREEN_2026-09-14.md) | From AUDIT | NARROW_CODE_SCREEN | BORROW_PATTERN |
| [slsa-verifier](../sources/SLSA_VERIFIER_MCI_SCREEN_2026-09-14.md) | From AUDIT | NARROW_CODE_SCREEN | ARCHIVE_REFERENCE |
| [context-compress](../sources/CONTEXT_COMPRESS.md) | From CONTEXT | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [agenttrace](../sources/AGENTTRACE.md) | From AUDIT | PRIOR_FIRST_TRACE_REUSED | BORROW_PATTERN |
| [agent-observability](../sources/AGENT_OBSERVABILITY.md) | From AUDIT | PRIOR_FIRST_TRACE_REUSED | ARCHIVE_REFERENCE |
| [ctx-zip](../sources/CTX_ZIP_MCI_SCREEN_2026-09-14.md) | From CONTEXT | NARROW_CODE_SCREEN | BORROW_IDEA |
| [gastown](../sources/GASTOWN_MCI_REVIEW_2026-09-14.md) | From COORD | SCOPED_CODE_STUDY | BORROW_IDEA |

## Further qualification focus

Index all existing experiments under ../experiments/ and preserve NOT RUN status. Compare contract tests, deterministic replay, provider semantic faults, transport failures, mutation testing and sandbox adversarial probes. Use fixed workloads and measure task correctness, missed evidence, token/context cost, latency, retries and recovery. Required result: qualification matrix showing tests located, read, run, failed, unavailable or still proposed. Do not treat test counts or a source README as independent assurance.

