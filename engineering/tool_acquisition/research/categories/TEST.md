# Testing and qualification — study packet

Status: ORGANIZED FOR REVIEW; source qualification remains open.

Contract tests, fault injection, replay, mutation and comparison experiments.

[Full map](../../MCI_RESEARCH_MAP.md) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md) · [Research protocol](../../RESEARCH_PROTOCOL.md)

## Source routing

These are investigation assignments, not claims that each upstream implements the desired behavior.

| Source | Filing role | Existing evidence state | Inspect for MCI |
|---|---|---|---|
| [cli-anything](https://github.com/HKUDS/CLI-Anything/tree/810c18b0d1ab9b234bc996c9fd999318523a3ef0) | Cross-reference from TOOLS | NOT_TRACED_IN_ACTIVE_LEDGER | Adapter generation, typed command contracts and protecting tests |
| [rmcp](https://github.com/modelcontextprotocol/rust-sdk/tree/3075dc9152d4678775f20634fcb467a7b995dbab) | Cross-reference from PROTO | [FIRST_TRACE_RECORDED](../../research/sources/RMCP.md) | Transport/session lifecycle versus host authorization |
| [mcp-inspector](https://github.com/modelcontextprotocol/inspector/tree/795b1bb30ac845b7baa7cb3df8ec0b693882ca1d) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Protocol diagnostics, unsafe inputs and reproducible contract probes |
| [wasmtime](https://github.com/bytecodealliance/wasmtime/tree/817c58787f432bcdbbb87679011f72c5bc80dbda) | Cross-reference from SEC | [FIRST_TRACE_RECORDED](../../research/sources/WASMTIME_WASI.md) | Host-granted WASI capabilities, fuel/epoch limits and cancellation |
| [extism](https://github.com/extism/extism/tree/d5da29759bba88645f886d9e12d3f4e4376df7b3) | Cross-reference from SEC | [FIRST_TRACE_RECORDED](../../research/sources/EXTISM.md) | Manifest loading authority, host functions and timeout scope |
| [ast-grep](https://github.com/ast-grep/ast-grep/tree/45b5eb6705b4c24e04746137d259874abf1087ad) | Cross-reference from CONTEXT | [FIRST_TRACE_RECORDED](../../research/sources/AST_GREP_TREE_SITTER.md) | Deterministic syntax queries and bounded code extraction |
| [tree-sitter](https://github.com/tree-sitter/tree-sitter/tree/1b8407d1e718f2a26e2886c03cc55622d8d1d7bd) | Cross-reference from CONTEXT | [FIRST_TRACE_RECORDED](../../research/sources/AST_GREP_TREE_SITTER.md) | Syntax boundaries, parse errors and extraction fidelity |
| [toxiproxy](https://github.com/Shopify/toxiproxy/tree/40f7fd31bee529d824116bd2a11a9e3425e904ec) | Home | [FIRST_TRACE_RECORDED](../../research/sources/WIREMOCK_TOXIPROXY.md) | Transport failure injection and recovery evidence |
| [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs/tree/6b193047bf2c5626da5dc5f3a23b58ab9bd3f130) | Home | [FIRST_TRACE_RECORDED](../../research/sources/WIREMOCK_TOXIPROXY.md) | Semantic provider mocks and deterministic retry/cancel tests |
| [cargo-nextest](https://github.com/nextest-rs/nextest/tree/8527c325bf9f0e5dcdcb26fa80e3ed7dc7f3e35e) | Home | [FIRST_TRACE_RECORDED](../../research/sources/CARGO_NEXTEST_MUTANTS.md) | Fast isolated execution, result accounting and flaky-test handling |
| [cargo-mutants](https://github.com/sourcefrog/cargo-mutants/tree/fe82f1832778a591ab74248010fb40e699defafe) | Home | [FIRST_TRACE_RECORDED](../../research/sources/CARGO_NEXTEST_MUTANTS.md) | Mutation coverage of important invariants rather than pass counts |
| [rtk](https://github.com/rtk-ai/rtk/tree/d0c2985155568d1d76fca03bc65d5098f136bbcd) | Cross-reference from TOOLS | [FIRST_TRACE_RECORDED](../../research/sources/RTK.md) | Compact output with faithful failures and retrievable raw evidence |
| [acp-rust-sdk](https://github.com/agentclientprotocol/rust-sdk/tree/3a6d0ae88dbaa09fcb74e26761e3643a75b2015e) | Cross-reference from PROTO | NOT_TRACED_IN_ACTIVE_LEDGER | Rust ACP transport and permission/cancel boundary behavior |
| [opentelemetry-collector](https://github.com/open-telemetry/opentelemetry-collector/tree/a35b7a8db49df923c5add3dc34872b6e0b3af683) | Cross-reference from AUDIT | [FIRST_TRACE_RECORDED](../../research/sources/OTEL_COLLECTOR.md) | Delivery queues and retry distinct from canonical evidence retention |
| [temporal](https://github.com/temporalio/temporal/tree/9ab3a9f770da20df7d94bcc0030f28eec7b0b947) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Durable workflow replay, duplicate effects, deadlines and recovery |
| [langgraph](https://github.com/langchain-ai/langgraph/tree/e539ac122f4126f6dd850581c1494948cf620e31) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Checkpoint/resume, human interrupts and stale approval semantics |
| [microsoft-agent-framework](https://github.com/microsoft/agent-framework/tree/1cd06c5a2058a172eebadf5d9d7c3fa45c519520) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Supervisor/worker handoffs, authority inheritance and recovery |
| [autogen](https://github.com/microsoft/autogen/tree/027ecf0a379bcc1d09956d46d12d44a3ad9cee14) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Multi-agent termination and coordination failure lessons; reference lane |
| [cedar](https://github.com/cedar-policy/cedar/tree/2f4019fd645cc8d4a4c0c1f8bd0280c77d754e28) | Cross-reference from GOV | NOT_TRACED_IN_ACTIVE_LEDGER | Typed policy evaluation, deny/error semantics and caller enforcement seam |
| [opa](https://github.com/open-policy-agent/opa/tree/961849b565d2457b80c168f254325b7d5b91461e) | Cross-reference from GOV | NOT_TRACED_IN_ACTIVE_LEDGER | Policy evaluation, bundle/version lifecycle and enforcement integration |
| [swe-agent](https://github.com/SWE-agent/SWE-agent/tree/3ea751c087f32b16e039a2233dd6eefecef325d5) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Agent action parser, environment boundaries and benchmark failure behavior |
| [in-toto](https://github.com/in-toto/in-toto/tree/e352b43ad7cb8915d84c36d791aa61346152a0a3) | Cross-reference from AUDIT | NOT_TRACED_IN_ACTIVE_LEDGER | Attested step inputs/outputs and evidence-chain validation |
| [slsa-verifier](https://github.com/slsa-framework/slsa-verifier/tree/30d0be3bbab553fc51557377baba2f7572dfc212) | Cross-reference from AUDIT | NOT_TRACED_IN_ACTIVE_LEDGER | Source/build provenance verification; maintenance/reference lane |
| [context-compress](https://github.com/Open330/context-compress/tree/59fae35a7b383876a34f84090f6da978e230795a) | Cross-reference from CONTEXT | [FIRST_TRACE_RECORDED](../../research/sources/CONTEXT_COMPRESS.md) | Compression, evidence handles and semantic loss |
| [agenttrace](https://github.com/Klepsiphron/agenttrace/tree/9a10f9aae3bc508ddca83093b2d25aede5ad5bd0) | Cross-reference from AUDIT | [FIRST_TRACE_RECORDED](../../research/sources/AGENTTRACE.md) | Host instrumentation, context races and persistence versus export |
| [agent-observability](https://github.com/KryptosAI/agent-observability/tree/2658eef467225f376e2e92dc1465839eda2bc113) | Cross-reference from AUDIT | [FIRST_TRACE_RECORDED](../../research/sources/AGENT_OBSERVABILITY.md) | Self-report versus independent audit; installation and payload gaps |
| [ctx-zip](https://github.com/karthikscale3/ctx-zip/tree/76580f7ba1555c891928702743ac74412c7fac60) | Cross-reference from CONTEXT | NOT_TRACED_IN_ACTIVE_LEDGER | Compression fidelity, evidence recovery and workload comparison |
| [gastown](https://github.com/gastownhall/gastown/tree/649b832b7672bc7a2dbef26f5983aba6198b819b) | Cross-reference from COORD | [INTAKE_ONLY](../../research/candidates/GASTOWN_COMMUNITY_INTAKE_2026-09-14.md) | Coordinator/supervisor roles, recovery, merge and ownership races |

Keep one source study and cross-reference its relevant findings here. For every retained finding record exact pin, module/function, protecting tests, observed limitations, MCI location, dependency cost, Rust fit and one evidence-backed disposition. Required negative tests and completion conditions are in Fable's brief. Do not promote a source to qualified merely because this packet lists it.

## Review focus

Index all existing experiments under ../experiments/ and preserve NOT RUN status. Compare contract tests, deterministic replay, provider semantic faults, transport failures, mutation testing and sandbox adversarial probes. Use fixed workloads and measure task correctness, missed evidence, token/context cost, latency, retries and recovery. Required result: qualification matrix showing tests located, read, run, failed, unavailable or still proposed. Do not treat test counts or a source README as independent assurance.
