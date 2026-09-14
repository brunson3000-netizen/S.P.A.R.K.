# MCI research map and coverage

Prepared: 2026-09-14. Status: ALL 54 SOURCES SCREENED; INTEGRATION QUALIFICATION OPEN.

Start with the [comprehensive assessment](research/comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md), [governance packet](research/categories/GOV.md) and [gap ledger](research/comparison/MCI_REVIEW_GAPS_2026-09-14.md). [Fable's brief](FABLE_MCI_RESEARCH_BRIEF.md) now points to completed screening evidence and remaining qualification. [Preparation instructions](FABLE_SOURCE_REVIEW.md) retain the source custody/verification command.

## Functional categories

Keep upstream trees intact. Each source has one filing home and additional relevant tags; governance-related mechanisms are cross-referenced into GOV.

| Category | Scope | Sources filed here |
|---|---|---:|
| [Governance and policy](research/categories/GOV.md) | Authority, approvals, delegation limits, spending controls and policy lifecycle. | 3 |
| [Coordination and supervision](research/categories/COORD.md) | Assignment, scheduling, worker lifecycle, durable recovery and cancellation. | 16 |
| [Security and containment](research/categories/SEC.md) | Enforced process, filesystem, network and sandbox boundaries. | 9 |
| [Basic tools and adapters](research/categories/TOOLS.md) | Deterministic commands, application adapters, structured results and tool execution. | 3 |
| [Protocols and discovery](research/categories/PROTO.md) | MCP/ACP, capability identity, registry, routing and tool teaching. | 8 |
| [Context and memory](research/categories/CONTEXT.md) | Retrieval, syntax slices, compression, persistent memory and freshness. | 4 |
| [Audit, observability and provenance](research/categories/AUDIT.md) | Canonical evidence, derived telemetry, artifact origin and verification. | 6 |
| [Testing and qualification](research/categories/TEST.md) | Contract tests, fault injection, replay, mutation and comparison experiments. | 5 |

## Actual research state

This assessment reuses 20 pin-matching prior first traces, adds six scoped code studies, 25 narrow code screens and three documentation screens. Every locked source has a disposition. One focused ctx-zip adverse reproduction was executed; the upstream suites and integration experiments were not run in this pass. E2B's T-SANDBOX-02 remains IN PROGRESS and has not been reassigned or closed.

Counts describe evidence coverage, not equivalent depth. Screening complete, first trace complete, runtime qualified and consumer accepted are separate claims. [Machine-readable coverage](research/MCI_REVIEW_COVERAGE_2026-09-14.json) records evidence level, status and gaps per source. Source files are available through the prior preparation and pinned GitHub links; no new claim is made about Fable's local checkout.

## Complete source routing and current disposition

| Source / exact pin | Home | Also relevant | Evidence | Primary disposition |
|---|---|---|---|---|
| [cli-anything](https://github.com/HKUDS/CLI-Anything/tree/810c18b0d1ab9b234bc996c9fd999318523a3ef0) | TOOLS | PROTO, TEST | [NARROW_CODE_SCREEN](research/sources/CLI_ANYTHING_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [rmcp](https://github.com/modelcontextprotocol/rust-sdk/tree/3075dc9152d4678775f20634fcb467a7b995dbab) | PROTO | GOV, TOOLS, TEST | [PRIOR_FIRST_TRACE_REUSED](research/sources/RMCP.md) | REUSE_CODE |
| [toolhive](https://github.com/stacklok/toolhive/tree/e532cf07d45fa99f3e4e63819396a3e9c9fd763f) | SEC | GOV, PROTO, AUDIT | [PRIOR_FIRST_TRACE_REUSED](research/sources/TOOLHIVE.md) | BORROW_PATTERN |
| [goose](https://github.com/aaif-goose/goose/tree/50666ae0b9a51e260b52b7efbab2e4e020346e94) | SEC | GOV, COORD, TOOLS | [PRIOR_FIRST_TRACE_REUSED](research/sources/GOOSE_SECURITY.md) | BORROW_PATTERN |
| [mcp-inspector](https://github.com/modelcontextprotocol/inspector/tree/795b1bb30ac845b7baa7cb3df8ec0b693882ca1d) | TEST | PROTO, SEC | [NARROW_CODE_SCREEN](research/sources/MCP_INSPECTOR_MCI_SCREEN_2026-09-14.md) | EXPERIMENT_NOW |
| [wasmtime](https://github.com/bytecodealliance/wasmtime/tree/817c58787f432bcdbbb87679011f72c5bc80dbda) | SEC | GOV, TEST | [PRIOR_FIRST_TRACE_REUSED](research/sources/WASMTIME_WASI.md) | EXPERIMENT_NOW |
| [extism](https://github.com/extism/extism/tree/d5da29759bba88645f886d9e12d3f4e4376df7b3) | SEC | GOV, TOOLS, TEST | [PRIOR_FIRST_TRACE_REUSED](research/sources/EXTISM.md) | EXPERIMENT_NOW |
| [ast-grep](https://github.com/ast-grep/ast-grep/tree/45b5eb6705b4c24e04746137d259874abf1087ad) | CONTEXT | TOOLS, TEST | [PRIOR_FIRST_TRACE_REUSED](research/sources/AST_GREP_TREE_SITTER.md) | EXPERIMENT_NOW |
| [tree-sitter](https://github.com/tree-sitter/tree-sitter/tree/1b8407d1e718f2a26e2886c03cc55622d8d1d7bd) | CONTEXT | TOOLS, TEST | [PRIOR_FIRST_TRACE_REUSED](research/sources/AST_GREP_TREE_SITTER.md) | EXPERIMENT_NOW |
| [toxiproxy](https://github.com/Shopify/toxiproxy/tree/40f7fd31bee529d824116bd2a11a9e3425e904ec) | TEST | SEC | [PRIOR_FIRST_TRACE_REUSED](research/sources/WIREMOCK_TOXIPROXY.md) | EXPERIMENT_NOW |
| [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs/tree/6b193047bf2c5626da5dc5f3a23b58ab9bd3f130) | TEST | TOOLS | [PRIOR_FIRST_TRACE_REUSED](research/sources/WIREMOCK_TOXIPROXY.md) | EXPERIMENT_NOW |
| [cargo-nextest](https://github.com/nextest-rs/nextest/tree/8527c325bf9f0e5dcdcb26fa80e3ed7dc7f3e35e) | TEST | TOOLS | [PRIOR_FIRST_TRACE_REUSED](research/sources/CARGO_NEXTEST_MUTANTS.md) | EXPERIMENT_NOW |
| [cargo-mutants](https://github.com/sourcefrog/cargo-mutants/tree/fe82f1832778a591ab74248010fb40e699defafe) | TEST | TOOLS | [PRIOR_FIRST_TRACE_REUSED](research/sources/CARGO_NEXTEST_MUTANTS.md) | EXPERIMENT_NOW |
| [rtk](https://github.com/rtk-ai/rtk/tree/d0c2985155568d1d76fca03bc65d5098f136bbcd) | TOOLS | CONTEXT, AUDIT, TEST | [PRIOR_FIRST_TRACE_REUSED](research/sources/RTK.md) | EXPERIMENT_NOW |
| [acp-spec](https://github.com/agentclientprotocol/agent-client-protocol/tree/ada6b108389a63a2625298f2be3eacde33a1d8c5) | PROTO | GOV, COORD | [DOCUMENTATION_SCREEN](research/sources/ACP_SPEC_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [acp-rust-sdk](https://github.com/agentclientprotocol/rust-sdk/tree/3a6d0ae88dbaa09fcb74e26761e3643a75b2015e) | PROTO | GOV, COORD, TEST | [NARROW_CODE_SCREEN](research/sources/ACP_RUST_SDK_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [wasmcloud](https://github.com/wasmCloud/wasmCloud/tree/1f82c28e638372fbf5a924c96214a4c3629e99f8) | SEC | COORD, PROTO | [NARROW_CODE_SCREEN](research/sources/WASMCLOUD_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [nushell](https://github.com/nushell/nushell/tree/b6e6562a9a385df439ae864714c609c60314fde9) | TOOLS | PROTO, SEC | [NARROW_CODE_SCREEN](research/sources/NUSHELL_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [opentelemetry-collector](https://github.com/open-telemetry/opentelemetry-collector/tree/a35b7a8db49df923c5add3dc34872b6e0b3af683) | AUDIT | COORD, TEST | [PRIOR_FIRST_TRACE_REUSED](research/sources/OTEL_COLLECTOR.md) | BORROW_PATTERN |
| [temporal](https://github.com/temporalio/temporal/tree/9ab3a9f770da20df7d94bcc0030f28eec7b0b947) | COORD | AUDIT, TEST | [NARROW_CODE_SCREEN](research/sources/TEMPORAL_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [openhands-sdk](https://github.com/OpenHands/software-agent-sdk/tree/c37007429be8b4465a83487dc1fd0914df0ea734) | COORD | GOV, SEC, TOOLS | [NARROW_CODE_SCREEN](research/sources/OPENHANDS_SDK_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [langgraph](https://github.com/langchain-ai/langgraph/tree/e539ac122f4126f6dd850581c1494948cf620e31) | COORD | GOV, CONTEXT, TEST | [NARROW_CODE_SCREEN](research/sources/LANGGRAPH_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [n8n](https://github.com/n8n-io/n8n/tree/4169b55bf3b3e6c255d7361642bc5243bd04345a) | COORD | GOV, TOOLS, AUDIT | [NARROW_CODE_SCREEN](research/sources/N8N_MCI_SCREEN_2026-09-14.md) | ARCHIVE_REFERENCE |
| [microsoft-agent-framework](https://github.com/microsoft/agent-framework/tree/1cd06c5a2058a172eebadf5d9d7c3fa45c519520) | COORD | GOV, PROTO, TEST | [NARROW_CODE_SCREEN](research/sources/MICROSOFT_AGENT_FRAMEWORK_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [autogen](https://github.com/microsoft/autogen/tree/027ecf0a379bcc1d09956d46d12d44a3ad9cee14) | COORD | GOV, TEST | [NARROW_CODE_SCREEN](research/sources/AUTOGEN_MCI_SCREEN_2026-09-14.md) | ARCHIVE_REFERENCE |
| [cedar](https://github.com/cedar-policy/cedar/tree/2f4019fd645cc8d4a4c0c1f8bd0280c77d754e28) | GOV | SEC, TEST | [SCOPED_CODE_STUDY](research/sources/CEDAR_MCI_REVIEW_2026-09-14.md) | BORROW_PATTERN |
| [cloudflare-agents](https://github.com/cloudflare/agents/tree/46760e635ce9599add0abbfe6c1a34af0d5d44f1) | COORD | GOV, CONTEXT, PROTO | [NARROW_CODE_SCREEN](research/sources/CLOUDFLARE_AGENTS_MCI_SCREEN_2026-09-14.md) | ARCHIVE_REFERENCE |
| [opa](https://github.com/open-policy-agent/opa/tree/961849b565d2457b80c168f254325b7d5b91461e) | GOV | SEC, AUDIT, TEST | [SCOPED_CODE_STUDY](research/sources/OPA_MCI_REVIEW_2026-09-14.md) | ARCHIVE_REFERENCE |
| [claude-code](https://github.com/anthropics/claude-code/tree/b5932767f3acbd07da25367064827e5cb81f43de) | COORD | GOV, TOOLS | [DOCUMENTATION_SCREEN](research/sources/CLAUDE_CODE_MCI_SCREEN_2026-09-14.md) | ARCHIVE_REFERENCE |
| [gemini-cli](https://github.com/google-gemini/gemini-cli/tree/9c1b0a610534d6f8120964cf2672c07807d8fc90) | COORD | GOV, SEC, TOOLS | [NARROW_CODE_SCREEN](research/sources/GEMINI_CLI_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [google-adk-python](https://github.com/google/adk-python/tree/460715b6c62c8e9ab00931c502381ee0364e39b6) | COORD | GOV, TOOLS, AUDIT | [NARROW_CODE_SCREEN](research/sources/GOOGLE_ADK_PYTHON_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [openai-codex](https://github.com/openai/codex/tree/516f2780fd227a80cd9fe89488f5039245090b71) | COORD | GOV, SEC, PROTO | [NARROW_CODE_SCREEN](research/sources/OPENAI_CODEX_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [openai-agents-python](https://github.com/openai/openai-agents-python/tree/fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292) | COORD | GOV, TOOLS, AUDIT | [NARROW_CODE_SCREEN](research/sources/OPENAI_AGENTS_PYTHON_MCI_SCREEN_2026-09-14.md) | BORROW_IDEA |
| [swe-agent](https://github.com/SWE-agent/SWE-agent/tree/3ea751c087f32b16e039a2233dd6eefecef325d5) | COORD | SEC, TOOLS, TEST | [NARROW_CODE_SCREEN](research/sources/SWE_AGENT_MCI_SCREEN_2026-09-14.md) | ARCHIVE_REFERENCE |
| [cosign](https://github.com/sigstore/cosign/tree/633e8f4303b7db64c369bdfa315374a8bd511ac6) | AUDIT | GOV, SEC | [NARROW_CODE_SCREEN](research/sources/COSIGN_MCI_SCREEN_2026-09-14.md) | DEFER |
| [in-toto](https://github.com/in-toto/in-toto/tree/e352b43ad7cb8915d84c36d791aa61346152a0a3) | AUDIT | GOV, TEST | [NARROW_CODE_SCREEN](research/sources/IN_TOTO_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [slsa-verifier](https://github.com/slsa-framework/slsa-verifier/tree/30d0be3bbab553fc51557377baba2f7572dfc212) | AUDIT | GOV, SEC, TEST | [NARROW_CODE_SCREEN](research/sources/SLSA_VERIFIER_MCI_SCREEN_2026-09-14.md) | ARCHIVE_REFERENCE |
| [ard-spec](https://github.com/ards-project/ard-spec/tree/b76f235a8f461876ad4f1e77abd0eb0eb302b48d) | PROTO | GOV | [PRIOR_FIRST_TRACE_REUSED](research/sources/ARD.md) | BORROW_PATTERN |
| [progressive-mcp-guardian](https://github.com/S1LV3RJ1NX/mcp-guardian/tree/4c6a04537b9bc548744b4146168b8fa9896069cb) | PROTO | GOV, CONTEXT | [PRIOR_FIRST_TRACE_REUSED](research/sources/PROGRESSIVE_MCP_GUARDIAN.md) | EXPERIMENT_NOW |
| [mcp-gateway-registry](https://github.com/agentic-community/mcp-gateway-registry/tree/7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73) | PROTO | GOV, SEC, AUDIT | [PRIOR_FIRST_TRACE_REUSED](research/sources/MCP_GATEWAY_REGISTRY.md) | BORROW_PATTERN |
| [context-compress](https://github.com/Open330/context-compress/tree/59fae35a7b383876a34f84090f6da978e230795a) | CONTEXT | TOOLS, AUDIT, TEST | [PRIOR_FIRST_TRACE_REUSED](research/sources/CONTEXT_COMPRESS.md) | BORROW_PATTERN |
| [agent-skills-spec](https://github.com/agentskills/agentskills/tree/69ef37e9424c0a7ea9dd2293b559e43ec8176379) | PROTO | GOV, CONTEXT | [PRIOR_FIRST_TRACE_REUSED](research/sources/AGENT_SKILLS.md) | BORROW_PATTERN |
| [agentgateway](https://github.com/agentgateway/agentgateway/tree/5e5633bffe6dc5fdd29256648987197f366e462a) | PROTO | GOV, SEC, AUDIT | [NARROW_CODE_SCREEN](research/sources/AGENTGATEWAY_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [agenttrace](https://github.com/Klepsiphron/agenttrace/tree/9a10f9aae3bc508ddca83093b2d25aede5ad5bd0) | AUDIT | GOV, TEST | [PRIOR_FIRST_TRACE_REUSED](research/sources/AGENTTRACE.md) | BORROW_PATTERN |
| [openziti-mcp-gateway](https://github.com/openziti/mcp-gateway/tree/8f99623d95d2f5223d2fa12b9f125688d8c80bf9) | SEC | GOV, PROTO | [NARROW_CODE_SCREEN](research/sources/OPENZITI_MCP_GATEWAY_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [e2b-runtime](https://github.com/e2b-dev/runtime/tree/cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497) | SEC | GOV, COORD, AUDIT | [DOCUMENTATION_SCREEN](research/sources/E2B_RUNTIME_MCI_SCREEN_2026-09-14.md) (active deeper trace) | DEFER |
| [agent-observability](https://github.com/KryptosAI/agent-observability/tree/2658eef467225f376e2e92dc1465839eda2bc113) | AUDIT | GOV, TEST | [PRIOR_FIRST_TRACE_REUSED](research/sources/AGENT_OBSERVABILITY.md) | ARCHIVE_REFERENCE |
| [ctx-zip](https://github.com/karthikscale3/ctx-zip/tree/76580f7ba1555c891928702743ac74412c7fac60) | CONTEXT | TOOLS, AUDIT, TEST | [NARROW_CODE_SCREEN](research/sources/CTX_ZIP_MCI_SCREEN_2026-09-14.md) | BORROW_IDEA |
| [paperclip](https://github.com/paperclipai/paperclip/tree/13368c518303e886a5c9445fbc69afcbdf0a9228) | GOV | COORD, AUDIT | [SCOPED_CODE_STUDY](research/sources/PAPERCLIP_MCI_REVIEW_2026-09-14.md) | BORROW_PATTERN |
| [openclaw](https://github.com/openclaw/openclaw/tree/b0638604941dda7eb80095481576898f875caf08) | COORD | GOV, SEC, CONTEXT | [NARROW_CODE_SCREEN](research/sources/OPENCLAW_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [hermes-agent](https://github.com/NousResearch/hermes-agent/tree/ee4452991d17534aa561f31ee55596d082aa94e7) | COORD | GOV, SEC, CONTEXT | [NARROW_CODE_SCREEN](research/sources/HERMES_AGENT_MCI_SCREEN_2026-09-14.md) | BORROW_PATTERN |
| [gastown](https://github.com/gastownhall/gastown/tree/649b832b7672bc7a2dbef26f5983aba6198b819b) | COORD | GOV, AUDIT, TEST | [SCOPED_CODE_STUDY](research/sources/GASTOWN_MCI_REVIEW_2026-09-14.md) | BORROW_IDEA |
| [openshell](https://github.com/NVIDIA/OpenShell/tree/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0) | SEC | GOV, COORD, AUDIT | [SCOPED_CODE_STUDY](research/sources/OPENSHELL_MCI_REVIEW_2026-09-14.md) | BORROW_PATTERN |
| [nemoclaw](https://github.com/NVIDIA/NemoClaw/tree/3ea2d5f9a515d4176e0745214743a48ff4868f4c) | SEC | GOV, COORD, PROTO | [SCOPED_CODE_STUDY](research/sources/NEMOCLAW_MCI_REVIEW_2026-09-14.md) | BORROW_PATTERN |

## Supplemental material

[Grok Build](research/sources/GROK_BUILD.md): BORROW_PATTERN. Existing source trace is outside the 54-source locks. Exact remote tree at `37949780c144e37df692e3d669051a21fec24f20` was retrieved without truncation, totals 73,042,597 tracked blob bytes (~69.7 MiB), and has a root Apache-2.0 license. It is reasonably sized by the earlier cutoff, but local checkout remains unverified and no supplemental gitlink was added in this pass. This does not increase the harvested denominator.

[Cloudways Managed AI Agents](research/candidates/CLOUDWAYS_MANAGED_AI_AGENTS.md): ARCHIVE_REFERENCE, documentation-only. Useful managed operations/interface prior art; no public control-plane implementation was established. It is distinct from Cloudflare Agents. OpenClaw/Hermes now have separate pinned source screens above; earlier Cloudways intake language predates that harvesting.

Historical consolidation remains [historical evidence](EXTERNAL_TOOL_RESEARCH_CONSOLIDATION.md). Source records supersede older capability guesses only within their stated scope and exact pins.
