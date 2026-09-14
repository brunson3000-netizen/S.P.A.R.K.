# MCI research map and coverage

Prepared: 2026-09-14. Status: CATEGORY ORGANIZATION COMPLETE; COMPREHENSIVE SOURCE QUALIFICATION OPEN.

Start with [Fable's assignment](FABLE_MCI_RESEARCH_BRIEF.md) and the separate [governance packet](research/categories/GOV.md). Source preparation remains in [FABLE_SOURCE_REVIEW.md](FABLE_SOURCE_REVIEW.md). This map organizes research for S.W.A.R.M.'s MCI; it does not decide its architecture or policy.

## Functional categories

Each source has one filing home and additional relevant tags. A source's governance mechanisms belong in the governance study even when its filing home is elsewhere. Keep upstream trees intact; separate study notes, not copied source trees.

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

The current active ledger contains 14 completed first-trace groups: these cover 20 of the 54 harvested sources plus Grok Build outside that inventory. Six community sources have intake records only. E2B is in progress. The other 27 have no completed first trace recorded in that ledger; some have earlier consolidated research, particularly CLI-Anything and Cedar. This is a ledger coverage measure, not a claim that those sources have never been studied.

A first trace is not complete qualification, a listed upstream test is not an executed test, and an experiment proposal is not a result. Existing findings and experiments retain their recorded status. E2B's active work remains owned by its existing researcher; use its published output and avoid duplicate assignment.

The prior preparation report verified 51 top-level checkouts and five nested repositories. OpenClaw, Goose and SLSA Verifier exceed the default cutoff and require explicit selection or pinned remote inspection. Pinning in this repository does not prove source files exist on Fable's machine.

## Complete harvested-source routing

Status codes: FIRST = first trace recorded; INTAKE = candidate framing only; ACTIVE = E2B in progress; OPEN = no completed first trace in active ledger. Every row still needs the completion assessment in Fable's brief.

| Source | Home | Also relevant | State / evidence | First MCI question |
|---|---|---|---|---|
| [cli-anything](https://github.com/HKUDS/CLI-Anything/tree/810c18b0d1ab9b234bc996c9fd999318523a3ef0) | TOOLS | PROTO, TEST | OPEN | Adapter generation, typed command contracts and protecting tests |
| [rmcp](https://github.com/modelcontextprotocol/rust-sdk/tree/3075dc9152d4678775f20634fcb467a7b995dbab) | PROTO | GOV, TOOLS, TEST | [FIRST](research/sources/RMCP.md) | Transport/session lifecycle versus host authorization |
| [toolhive](https://github.com/stacklok/toolhive/tree/e532cf07d45fa99f3e4e63819396a3e9c9fd763f) | SEC | GOV, PROTO, AUDIT | [FIRST](research/sources/TOOLHIVE.md) | Declared permissions versus applied confinement and verified package activation |
| [goose](https://github.com/aaif-goose/goose/tree/50666ae0b9a51e260b52b7efbab2e4e020346e94) | SEC | GOV, COORD, TOOLS | [FIRST](research/sources/GOOSE_SECURITY.md) | Tool inspection, permission modes and execution enforcement |
| [mcp-inspector](https://github.com/modelcontextprotocol/inspector/tree/795b1bb30ac845b7baa7cb3df8ec0b693882ca1d) | TEST | PROTO, SEC | OPEN | Protocol diagnostics, unsafe inputs and reproducible contract probes |
| [wasmtime](https://github.com/bytecodealliance/wasmtime/tree/817c58787f432bcdbbb87679011f72c5bc80dbda) | SEC | GOV, TEST | [FIRST](research/sources/WASMTIME_WASI.md) | Host-granted WASI capabilities, fuel/epoch limits and cancellation |
| [extism](https://github.com/extism/extism/tree/d5da29759bba88645f886d9e12d3f4e4376df7b3) | SEC | GOV, TOOLS, TEST | [FIRST](research/sources/EXTISM.md) | Manifest loading authority, host functions and timeout scope |
| [ast-grep](https://github.com/ast-grep/ast-grep/tree/45b5eb6705b4c24e04746137d259874abf1087ad) | CONTEXT | TOOLS, TEST | [FIRST](research/sources/AST_GREP_TREE_SITTER.md) | Deterministic syntax queries and bounded code extraction |
| [tree-sitter](https://github.com/tree-sitter/tree-sitter/tree/1b8407d1e718f2a26e2886c03cc55622d8d1d7bd) | CONTEXT | TOOLS, TEST | [FIRST](research/sources/AST_GREP_TREE_SITTER.md) | Syntax boundaries, parse errors and extraction fidelity |
| [toxiproxy](https://github.com/Shopify/toxiproxy/tree/40f7fd31bee529d824116bd2a11a9e3425e904ec) | TEST | SEC | [FIRST](research/sources/WIREMOCK_TOXIPROXY.md) | Transport failure injection and recovery evidence |
| [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs/tree/6b193047bf2c5626da5dc5f3a23b58ab9bd3f130) | TEST | TOOLS | [FIRST](research/sources/WIREMOCK_TOXIPROXY.md) | Semantic provider mocks and deterministic retry/cancel tests |
| [cargo-nextest](https://github.com/nextest-rs/nextest/tree/8527c325bf9f0e5dcdcb26fa80e3ed7dc7f3e35e) | TEST | TOOLS | [FIRST](research/sources/CARGO_NEXTEST_MUTANTS.md) | Fast isolated execution, result accounting and flaky-test handling |
| [cargo-mutants](https://github.com/sourcefrog/cargo-mutants/tree/fe82f1832778a591ab74248010fb40e699defafe) | TEST | TOOLS | [FIRST](research/sources/CARGO_NEXTEST_MUTANTS.md) | Mutation coverage of important invariants rather than pass counts |
| [rtk](https://github.com/rtk-ai/rtk/tree/d0c2985155568d1d76fca03bc65d5098f136bbcd) | TOOLS | CONTEXT, AUDIT, TEST | [FIRST](research/sources/RTK.md) | Compact output with faithful failures and retrievable raw evidence |
| [acp-spec](https://github.com/agentclientprotocol/agent-client-protocol/tree/ada6b108389a63a2625298f2be3eacde33a1d8c5) | PROTO | GOV, COORD | OPEN | Client-agent permissions, sessions and cancellation contract |
| [acp-rust-sdk](https://github.com/agentclientprotocol/rust-sdk/tree/3a6d0ae88dbaa09fcb74e26761e3643a75b2015e) | PROTO | GOV, COORD, TEST | OPEN | Rust ACP transport and permission/cancel boundary behavior |
| [wasmcloud](https://github.com/wasmCloud/wasmCloud/tree/1f82c28e638372fbf5a924c96214a4c3629e99f8) | SEC | COORD, PROTO | OPEN | Capability-provider isolation and host/control-plane dependency cost |
| [nushell](https://github.com/nushell/nushell/tree/b6e6562a9a385df439ae864714c609c60314fde9) | TOOLS | PROTO, SEC | OPEN | Structured command/plugin inputs, outputs and OS authority |
| [opentelemetry-collector](https://github.com/open-telemetry/opentelemetry-collector/tree/a35b7a8db49df923c5add3dc34872b6e0b3af683) | AUDIT | COORD, TEST | [FIRST](research/sources/OTEL_COLLECTOR.md) | Delivery queues and retry distinct from canonical evidence retention |
| [temporal](https://github.com/temporalio/temporal/tree/9ab3a9f770da20df7d94bcc0030f28eec7b0b947) | COORD | AUDIT, TEST | OPEN | Durable workflow replay, duplicate effects, deadlines and recovery |
| [openhands-sdk](https://github.com/OpenHands/software-agent-sdk/tree/c37007429be8b4465a83487dc1fd0914df0ea734) | COORD | GOV, SEC, TOOLS | OPEN | Agent execution, approval gates and runtime isolation |
| [langgraph](https://github.com/langchain-ai/langgraph/tree/e539ac122f4126f6dd850581c1494948cf620e31) | COORD | GOV, CONTEXT, TEST | OPEN | Checkpoint/resume, human interrupts and stale approval semantics |
| [n8n](https://github.com/n8n-io/n8n/tree/4169b55bf3b3e6c255d7361642bc5243bd04345a) | COORD | GOV, TOOLS, AUDIT | OPEN | Workflow permissions, credentials and retry effects; constrained reuse terms |
| [microsoft-agent-framework](https://github.com/microsoft/agent-framework/tree/1cd06c5a2058a172eebadf5d9d7c3fa45c519520) | COORD | GOV, PROTO, TEST | OPEN | Supervisor/worker handoffs, authority inheritance and recovery |
| [autogen](https://github.com/microsoft/autogen/tree/027ecf0a379bcc1d09956d46d12d44a3ad9cee14) | COORD | GOV, TEST | OPEN | Multi-agent termination and coordination failure lessons; reference lane |
| [cedar](https://github.com/cedar-policy/cedar/tree/2f4019fd645cc8d4a4c0c1f8bd0280c77d754e28) | GOV | SEC, TEST | OPEN | Typed policy evaluation, deny/error semantics and caller enforcement seam |
| [cloudflare-agents](https://github.com/cloudflare/agents/tree/46760e635ce9599add0abbfe6c1a34af0d5d44f1) | COORD | GOV, CONTEXT, PROTO | OPEN | Durable actor identity, state and platform-dependent authority |
| [opa](https://github.com/open-policy-agent/opa/tree/961849b565d2457b80c168f254325b7d5b91461e) | GOV | SEC, AUDIT, TEST | OPEN | Policy evaluation, bundle/version lifecycle and enforcement integration |
| [claude-code](https://github.com/anthropics/claude-code/tree/b5932767f3acbd07da25367064827e5cb81f43de) | COORD | GOV, TOOLS | OPEN | Public product/hook reference only; not full open-source implementation |
| [gemini-cli](https://github.com/google-gemini/gemini-cli/tree/9c1b0a610534d6f8120964cf2672c07807d8fc90) | COORD | GOV, SEC, TOOLS | OPEN | Policy/trust gates, shell execution and child-agent controls |
| [google-adk-python](https://github.com/google/adk-python/tree/460715b6c62c8e9ab00931c502381ee0364e39b6) | COORD | GOV, TOOLS, AUDIT | OPEN | Agent/tool callbacks, delegation, runtime limits and evidence |
| [openai-codex](https://github.com/openai/codex/tree/516f2780fd227a80cd9fe89488f5039245090b71) | COORD | GOV, SEC, PROTO | OPEN | Host approval, sandbox, session and process lifecycle boundaries |
| [openai-agents-python](https://github.com/openai/openai-agents-python/tree/fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292) | COORD | GOV, TOOLS, AUDIT | OPEN | Handoffs, guardrails, approval semantics and tracing provenance |
| [swe-agent](https://github.com/SWE-agent/SWE-agent/tree/3ea751c087f32b16e039a2233dd6eefecef325d5) | COORD | SEC, TOOLS, TEST | OPEN | Agent action parser, environment boundaries and benchmark failure behavior |
| [cosign](https://github.com/sigstore/cosign/tree/633e8f4303b7db64c369bdfa315374a8bd511ac6) | AUDIT | GOV, SEC | OPEN | Artifact identity and trust verification; not behavioral safety approval |
| [in-toto](https://github.com/in-toto/in-toto/tree/e352b43ad7cb8915d84c36d791aa61346152a0a3) | AUDIT | GOV, TEST | OPEN | Attested step inputs/outputs and evidence-chain validation |
| [slsa-verifier](https://github.com/slsa-framework/slsa-verifier/tree/30d0be3bbab553fc51557377baba2f7572dfc212) | AUDIT | GOV, SEC, TEST | OPEN | Source/build provenance verification; maintenance/reference lane |
| [ard-spec](https://github.com/ards-project/ard-spec/tree/b76f235a8f461876ad4f1e77abd0eb0eb302b48d) | PROTO | GOV | [FIRST](research/sources/ARD.md) | Discovery metadata, identity and limits of declared authority |
| [progressive-mcp-guardian](https://github.com/S1LV3RJ1NX/mcp-guardian/tree/4c6a04537b9bc548744b4146168b8fa9896069cb) | PROTO | GOV, CONTEXT | [FIRST](research/sources/PROGRESSIVE_MCP_GUARDIAN.md) | Progressive tool disclosure and index admission versus call-time policy |
| [mcp-gateway-registry](https://github.com/agentic-community/mcp-gateway-registry/tree/7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73) | PROTO | GOV, SEC, AUDIT | [FIRST](research/sources/MCP_GATEWAY_REGISTRY.md) | Filtered discovery, call-time authorization and signed hop binding |
| [context-compress](https://github.com/Open330/context-compress/tree/59fae35a7b383876a34f84090f6da978e230795a) | CONTEXT | TOOLS, AUDIT, TEST | [FIRST](research/sources/CONTEXT_COMPRESS.md) | Compression, evidence handles and semantic loss |
| [agent-skills-spec](https://github.com/agentskills/agentskills/tree/69ef37e9424c0a7ea9dd2293b559e43ec8176379) | PROTO | GOV, CONTEXT | [FIRST](research/sources/AGENT_SKILLS.md) | Teaching/package metadata versus execution authority and provenance |
| [agentgateway](https://github.com/agentgateway/agentgateway/tree/5e5633bffe6dc5fdd29256648987197f366e462a) | PROTO | GOV, SEC, AUDIT | OPEN | Gateway authentication, target binding and current-policy enforcement |
| [agenttrace](https://github.com/Klepsiphron/agenttrace/tree/9a10f9aae3bc508ddca83093b2d25aede5ad5bd0) | AUDIT | GOV, TEST | [FIRST](research/sources/AGENTTRACE.md) | Host instrumentation, context races and persistence versus export |
| [openziti-mcp-gateway](https://github.com/openziti/mcp-gateway/tree/8f99623d95d2f5223d2fa12b9f125688d8c80bf9) | SEC | GOV, PROTO | OPEN | Network identity/authorization versus tool-level operation authority |
| [e2b-runtime](https://github.com/e2b-dev/runtime/tree/cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497) | SEC | GOV, COORD, AUDIT | ACTIVE | MicroVM lifecycle, snapshot freshness, cleanup and resource enforcement |
| [agent-observability](https://github.com/KryptosAI/agent-observability/tree/2658eef467225f376e2e92dc1465839eda2bc113) | AUDIT | GOV, TEST | [FIRST](research/sources/AGENT_OBSERVABILITY.md) | Self-report versus independent audit; installation and payload gaps |
| [ctx-zip](https://github.com/karthikscale3/ctx-zip/tree/76580f7ba1555c891928702743ac74412c7fac60) | CONTEXT | TOOLS, AUDIT, TEST | OPEN | Compression fidelity, evidence recovery and workload comparison |
| [paperclip](https://github.com/paperclipai/paperclip/tree/13368c518303e886a5c9445fbc69afcbdf0a9228) | GOV | COORD, AUDIT | [INTAKE](research/candidates/PAPERCLIP_COMMUNITY_INTAKE_2026-09-14.md) | Approval version binding, concurrent budgets, leases and operator stop |
| [openclaw](https://github.com/openclaw/openclaw/tree/b0638604941dda7eb80095481576898f875caf08) | COORD | GOV, SEC, CONTEXT | [INTAKE](research/candidates/OPENCLAW_HERMES_COMMUNITY_INTAKE_2026-09-14.md) | Persistent agent roles, tool permissions and supervisor limits |
| [hermes-agent](https://github.com/NousResearch/hermes-agent/tree/ee4452991d17534aa561f31ee55596d082aa94e7) | COORD | GOV, SEC, CONTEXT | [INTAKE](research/candidates/OPENCLAW_HERMES_COMMUNITY_INTAKE_2026-09-14.md) | Persistent memory, tool gates and worker lifecycle |
| [gastown](https://github.com/gastownhall/gastown/tree/649b832b7672bc7a2dbef26f5983aba6198b819b) | COORD | GOV, AUDIT, TEST | [INTAKE](research/candidates/GASTOWN_COMMUNITY_INTAKE_2026-09-14.md) | Coordinator/supervisor roles, recovery, merge and ownership races |
| [openshell](https://github.com/NVIDIA/OpenShell/tree/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0) | SEC | GOV, COORD, AUDIT | [INTAKE](research/candidates/NVIDIA_OPENSHELL_NEMOCLAW_COMMUNITY_INTAKE_2026-09-14.md) | Runtime enforcement of process/filesystem/network policy and revocation |
| [nemoclaw](https://github.com/NVIDIA/NemoClaw/tree/3ea2d5f9a515d4176e0745214743a48ff4868f4c) | SEC | GOV, COORD, PROTO | [INTAKE](research/candidates/NVIDIA_OPENSHELL_NEMOCLAW_COMMUNITY_INTAKE_2026-09-14.md) | OpenShell integration, configuration-to-enforcement and router boundaries |

Exact pins, local paths and status fields: [MCI_RESEARCH_CATALOG.json](MCI_RESEARCH_CATALOG.json). Canonical custody remains in the four existing harvest locks and gitlinks; this catalog does not replace them.

## Supplementary material and gaps

- [Grok Build](research/sources/GROK_BUILD.md): existing Rust permission, trust, subagent, memory and remote-contract study at `37949780c144e37df692e3d669051a21fec24f20`. Include in GOV, COORD, CONTEXT, PROTO and AUDIT. Its source is not in the 54-source locks/gitlinks; resolve measured source custody before claiming local review readiness. Existing record also names a distinct sync revision: do not substitute it silently for the study commit.
- [Cloudways](research/candidates/CLOUDWAYS_MANAGED_AI_AGENTS.md): hosted operations reference in COORD, GOV and SEC; no public managed-control-plane implementation established. Do not purchase hosting for this review.
- [Cross-project consolidation](EXTERNAL_TOOL_RESEARCH_CONSOLIDATION.md), [source ledger](RESEARCH_SOURCE_LEDGER.md), and [historical archive](research_archive/ALL_OUTSIDE_TOOL_RESEARCH.md): retain earlier corporate prompts, NIM engineering-tool work, Cedar spike, Cloudflare OS concepts and GAME foreman/worker lessons. These are historical findings; trace original evidence where a conclusion will be relied upon. Cloudflare Agents is not proof of every Cloudflare OS/Gatekeeper claim.
- AgentTeams stays watchlisted under the previous intake decision. Large application sources stay demand-gated. Keep Blender, image editors, Godot, LLDB, RenderDoc, LibreOffice, QGIS and local inference/workflow applications under TOOLS as future adapter targets, not additional harvested repositories.
- Claude Code's public repository, n8n's restricted source terms and MCP Inspector's unresolved license evidence require explicit source/reuse classification. Public visibility is not code-reuse permission.

## MCI fit baseline

Operator direction makes governance the immediate research priority and retains these tools for later functions. Existing acquisition records favor a Rust authoritative core, narrow deterministic tool interfaces, bounded workers, durable evidence and consumer-owned authority. Use these as research comparison criteria. Verify current S.W.A.R.M. constitution, Agent Rules and architecture records before claiming compliance, proposing a policy replacement or treating a possible mechanism as an adopted MCI requirement. S.P.A.R.K.'s research lane cannot grant consumer authority.
