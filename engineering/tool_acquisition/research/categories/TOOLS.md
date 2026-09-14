# Basic tools and adapters — study packet

Status: ORGANIZED FOR REVIEW; source qualification remains open.

Deterministic commands, application adapters, structured results and tool execution.

[Full map](../../MCI_RESEARCH_MAP.md) · [Fable brief](../../FABLE_MCI_RESEARCH_BRIEF.md) · [Research protocol](../../RESEARCH_PROTOCOL.md)

## Source routing

These are investigation assignments, not claims that each upstream implements the desired behavior.

| Source | Filing role | Existing evidence state | Inspect for MCI |
|---|---|---|---|
| [cli-anything](https://github.com/HKUDS/CLI-Anything/tree/810c18b0d1ab9b234bc996c9fd999318523a3ef0) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Adapter generation, typed command contracts and protecting tests |
| [rmcp](https://github.com/modelcontextprotocol/rust-sdk/tree/3075dc9152d4678775f20634fcb467a7b995dbab) | Cross-reference from PROTO | [FIRST_TRACE_RECORDED](../../research/sources/RMCP.md) | Transport/session lifecycle versus host authorization |
| [goose](https://github.com/aaif-goose/goose/tree/50666ae0b9a51e260b52b7efbab2e4e020346e94) | Cross-reference from SEC | [FIRST_TRACE_RECORDED](../../research/sources/GOOSE_SECURITY.md) | Tool inspection, permission modes and execution enforcement |
| [extism](https://github.com/extism/extism/tree/d5da29759bba88645f886d9e12d3f4e4376df7b3) | Cross-reference from SEC | [FIRST_TRACE_RECORDED](../../research/sources/EXTISM.md) | Manifest loading authority, host functions and timeout scope |
| [ast-grep](https://github.com/ast-grep/ast-grep/tree/45b5eb6705b4c24e04746137d259874abf1087ad) | Cross-reference from CONTEXT | [FIRST_TRACE_RECORDED](../../research/sources/AST_GREP_TREE_SITTER.md) | Deterministic syntax queries and bounded code extraction |
| [tree-sitter](https://github.com/tree-sitter/tree-sitter/tree/1b8407d1e718f2a26e2886c03cc55622d8d1d7bd) | Cross-reference from CONTEXT | [FIRST_TRACE_RECORDED](../../research/sources/AST_GREP_TREE_SITTER.md) | Syntax boundaries, parse errors and extraction fidelity |
| [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs/tree/6b193047bf2c5626da5dc5f3a23b58ab9bd3f130) | Cross-reference from TEST | [FIRST_TRACE_RECORDED](../../research/sources/WIREMOCK_TOXIPROXY.md) | Semantic provider mocks and deterministic retry/cancel tests |
| [cargo-nextest](https://github.com/nextest-rs/nextest/tree/8527c325bf9f0e5dcdcb26fa80e3ed7dc7f3e35e) | Cross-reference from TEST | [FIRST_TRACE_RECORDED](../../research/sources/CARGO_NEXTEST_MUTANTS.md) | Fast isolated execution, result accounting and flaky-test handling |
| [cargo-mutants](https://github.com/sourcefrog/cargo-mutants/tree/fe82f1832778a591ab74248010fb40e699defafe) | Cross-reference from TEST | [FIRST_TRACE_RECORDED](../../research/sources/CARGO_NEXTEST_MUTANTS.md) | Mutation coverage of important invariants rather than pass counts |
| [rtk](https://github.com/rtk-ai/rtk/tree/d0c2985155568d1d76fca03bc65d5098f136bbcd) | Home | [FIRST_TRACE_RECORDED](../../research/sources/RTK.md) | Compact output with faithful failures and retrievable raw evidence |
| [nushell](https://github.com/nushell/nushell/tree/b6e6562a9a385df439ae864714c609c60314fde9) | Home | NOT_TRACED_IN_ACTIVE_LEDGER | Structured command/plugin inputs, outputs and OS authority |
| [openhands-sdk](https://github.com/OpenHands/software-agent-sdk/tree/c37007429be8b4465a83487dc1fd0914df0ea734) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Agent execution, approval gates and runtime isolation |
| [n8n](https://github.com/n8n-io/n8n/tree/4169b55bf3b3e6c255d7361642bc5243bd04345a) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Workflow permissions, credentials and retry effects; constrained reuse terms |
| [claude-code](https://github.com/anthropics/claude-code/tree/b5932767f3acbd07da25367064827e5cb81f43de) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Public product/hook reference only; not full open-source implementation |
| [gemini-cli](https://github.com/google-gemini/gemini-cli/tree/9c1b0a610534d6f8120964cf2672c07807d8fc90) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Policy/trust gates, shell execution and child-agent controls |
| [google-adk-python](https://github.com/google/adk-python/tree/460715b6c62c8e9ab00931c502381ee0364e39b6) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Agent/tool callbacks, delegation, runtime limits and evidence |
| [openai-agents-python](https://github.com/openai/openai-agents-python/tree/fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Handoffs, guardrails, approval semantics and tracing provenance |
| [swe-agent](https://github.com/SWE-agent/SWE-agent/tree/3ea751c087f32b16e039a2233dd6eefecef325d5) | Cross-reference from COORD | NOT_TRACED_IN_ACTIVE_LEDGER | Agent action parser, environment boundaries and benchmark failure behavior |
| [context-compress](https://github.com/Open330/context-compress/tree/59fae35a7b383876a34f84090f6da978e230795a) | Cross-reference from CONTEXT | [FIRST_TRACE_RECORDED](../../research/sources/CONTEXT_COMPRESS.md) | Compression, evidence handles and semantic loss |
| [ctx-zip](https://github.com/karthikscale3/ctx-zip/tree/76580f7ba1555c891928702743ac74412c7fac60) | Cross-reference from CONTEXT | NOT_TRACED_IN_ACTIVE_LEDGER | Compression fidelity, evidence recovery and workload comparison |

Keep one source study and cross-reference its relevant findings here. For every retained finding record exact pin, module/function, protecting tests, observed limitations, MCI location, dependency cost, Rust fit and one evidence-backed disposition. Required negative tests and completion conditions are in Fable's brief. Do not promote a source to qualified merely because this packet lists it.

## Review focus

Use [CLI-Anything consolidation](../../EXTERNAL_TOOL_RESEARCH_CONSOLIDATION.md) and [output reduction matrix](../comparison/OUTPUT_REDUCTION_MATRIX.md). Trace probe → typed input → real backend → effect → artifact verification → result. Compare side effects, preview/dry run, error fidelity, raw-evidence retrieval, installation footprint and minimum worker instructions. Future application adapters remain demand-gated: Blender; GIMP/Krita/Inkscape; Godot; LLDB/RenderDoc; LibreOffice; QGIS; Ollama/ComfyUI. Required result: reusable tool contract lessons and adapter opportunities, not implementations.
