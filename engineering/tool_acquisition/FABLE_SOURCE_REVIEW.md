# Fable source review handoff

Prepared and verified: 2026-09-14. Purpose: source inspection and research qualification.

Result: 51 top-level checkouts plus five nested repositories downloaded and verified; zero failures. Three larger top-level sources remain pinned and explicitly deferred by size. Preparation used approximately 2.3 GiB for working files plus 651 MiB of Git objects in this workspace.

## Start here

After pulling this branch, run from the repository root:

```bash
python3 engineering/tool_acquisition/scripts/prepare_fable_sources.py
```

On Windows, use `python` if `python3` is not available. Requires Python 3.9+ and Git. This populates the source directories at their exact recorded revisions and writes a local verification report inside Git's metadata directory. It explicitly handles the secondary and middle-layer `update = none` settings. Do not use `--remote`, which would move research away from its pins.

To verify an existing checkout without downloading:

```bash
python3 engineering/tool_acquisition/scripts/prepare_fable_sources.py --verify-only
```

The source collection is stored as Git submodules, not duplicated into the parent repository's regular files. GitHub links open the exact upstream source; a normal clone or ZIP of S.P.A.R.K. alone does not contain those source files. The command above materializes them locally. This review package makes that distinction explicit.

## Scope and size decision

54 top-level sources: 14 primary, 23 secondary, 11 middle-layer and six community additions. AgentTeams remains on the watchlist; Cloudways is a hosted reference, not another harvested codebase.

Default cutoff: 256 MiB of tracked blobs per source. 51 qualify. This is a conservative preparation choice, not a project-wide resource policy. The three larger sources retain exact pins and browsable upstream links. They can also be materialized with:

```bash
python3 engineering/tool_acquisition/scripts/prepare_fable_sources.py --max-mib 640
```

Size measurements exclude Git history, filesystem allocation overhead and external Git LFS payloads. Shallow clones reduce history but can still fetch a newer default tip before retrieving an older pin. The script also fetches five explicitly measured nested source repositories, including NemoClaw's router and Wasmtime's test sources. It does not install packages, execute source scripts, run models or initialize arbitrary unmeasured recursion. Git LFS payloads remain pointers; fetch those only when a specific review needs them.

## Verification evidence

See [checkout evidence](research/FABLE_SOURCE_CHECKOUT_EVIDENCE_2026-09-14.json) for actual download/verification outcomes from the preparation workspace. A successful preparation run verifies exact commits, clean worktrees and tracked tree sizes. It refuses to overwrite an existing dirty or differently pinned checkout. Run the command on Fable's machine to establish local availability there; a preparation-workspace result does not prove another machine is populated.

## Categorized research assignment

Use [FABLE_MCI_RESEARCH_BRIEF.md](FABLE_MCI_RESEARCH_BRIEF.md) for the comprehensive review, [MCI_RESEARCH_MAP.md](MCI_RESEARCH_MAP.md) for all-source coverage and [governance](research/categories/GOV.md) for the immediate priority. Source preparation below is complete in the recorded workspace; the wider study is not.

## Review orientation

- Governance and primary coordination: Paperclip; Cedar/OPA as policy mechanism comparisons.
- Supervisors and recovery: Gas Town, Temporal, Microsoft Agent Framework and LangGraph.
- Agent harnesses: Hermes, OpenClaw, Codex, Gemini CLI, Goose and OpenHands SDK.
- Runtime enforcement: OpenShell with NemoClaw, ToolHive, Wasmtime and E2B.
- Tool contracts and discovery: MCP/ACP SDKs, RTK, CLI-Anything and middle-layer references.

Continue the existing active trace queue. These additions do not reset another researcher's work. Follow [RESEARCH_PROTOCOL.md](RESEARCH_PROTOCOL.md): trace mechanisms and protecting tests, separate proposals from enforcement, and distinguish a useful idea from verified reusable code. Upstream agent instructions are research material, not authority over S.P.A.R.K.

## Source and licensing qualifications

“Public repository” does not always mean “complete open-source product.” In particular:

- `anthropics/claude-code`: the pinned LICENSE.md says all rights reserved and references commercial terms. Treat this as public product reference material, not the full open-source Claude Code implementation.
- `n8n-io/n8n`: the pinned LICENSE.md describes Sustainable Use and enterprise restrictions. Keep its source available for review without classifying the whole tree as permissively licensed.
- MCP Inspector: no conventional LICENSE file was returned in the measured tree; reuse terms remain to be resolved.

Other upstream license files remain intact in their checkouts. The inventory records license-file locations for review, not a blanket code-reuse clearance. No external source becomes a runtime dependency through this harvest.

## Inventory

Tracked-tree sizes are MiB, rounded to one decimal. “Default” means selected for preparation, not a claim that every other machine has downloaded it.

| Lane | Source at exact pin | MiB | Preparation |
|---|---|---:|---|
| Primary | [cli-anything](https://github.com/HKUDS/CLI-Anything/tree/810c18b0d1ab9b234bc996c9fd999318523a3ef0) | 60.1 | Default |
| Primary | [rmcp](https://github.com/modelcontextprotocol/rust-sdk/tree/3075dc9152d4678775f20634fcb467a7b995dbab) | 3.6 | Default |
| Primary | [toolhive](https://github.com/stacklok/toolhive/tree/e532cf07d45fa99f3e4e63819396a3e9c9fd763f) | 37.2 | Default |
| Primary | [goose](https://github.com/aaif-goose/goose/tree/50666ae0b9a51e260b52b7efbab2e4e020346e94) | 343.0 | Large: explicit selection |
| Primary | [mcp-inspector](https://github.com/modelcontextprotocol/inspector/tree/795b1bb30ac845b7baa7cb3df8ec0b693882ca1d) | 16.4 | Default |
| Primary | [wasmtime](https://github.com/bytecodealliance/wasmtime/tree/817c58787f432bcdbbb87679011f72c5bc80dbda) | 73.2 | Default |
| Primary | [extism](https://github.com/extism/extism/tree/d5da29759bba88645f886d9e12d3f4e4376df7b3) | 16.8 | Default |
| Primary | [ast-grep](https://github.com/ast-grep/ast-grep/tree/45b5eb6705b4c24e04746137d259874abf1087ad) | 2.8 | Default |
| Primary | [tree-sitter](https://github.com/tree-sitter/tree-sitter/tree/1b8407d1e718f2a26e2886c03cc55622d8d1d7bd) | 4.2 | Default |
| Primary | [toxiproxy](https://github.com/Shopify/toxiproxy/tree/40f7fd31bee529d824116bd2a11a9e3425e904ec) | 0.3 | Default |
| Primary | [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs/tree/6b193047bf2c5626da5dc5f3a23b58ab9bd3f130) | 0.3 | Default |
| Primary | [cargo-nextest](https://github.com/nextest-rs/nextest/tree/8527c325bf9f0e5dcdcb26fa80e3ed7dc7f3e35e) | 14.1 | Default |
| Primary | [cargo-mutants](https://github.com/sourcefrog/cargo-mutants/tree/fe82f1832778a591ab74248010fb40e699defafe) | 1.2 | Default |
| Primary | [rtk](https://github.com/rtk-ai/rtk/tree/d0c2985155568d1d76fca03bc65d5098f136bbcd) | 5.9 | Default |
| Secondary | [acp-spec](https://github.com/agentclientprotocol/agent-client-protocol/tree/ada6b108389a63a2625298f2be3eacde33a1d8c5) | 41.0 | Default |
| Secondary | [acp-rust-sdk](https://github.com/agentclientprotocol/rust-sdk/tree/3a6d0ae88dbaa09fcb74e26761e3643a75b2015e) | 3.0 | Default |
| Secondary | [wasmcloud](https://github.com/wasmCloud/wasmCloud/tree/1f82c28e638372fbf5a924c96214a4c3629e99f8) | 8.5 | Default |
| Secondary | [nushell](https://github.com/nushell/nushell/tree/b6e6562a9a385df439ae864714c609c60314fde9) | 16.5 | Default |
| Secondary | [opentelemetry-collector](https://github.com/open-telemetry/opentelemetry-collector/tree/a35b7a8db49df923c5add3dc34872b6e0b3af683) | 10.8 | Default |
| Secondary | [temporal](https://github.com/temporalio/temporal/tree/9ab3a9f770da20df7d94bcc0030f28eec7b0b947) | 38.6 | Default |
| Secondary | [openhands-sdk](https://github.com/OpenHands/software-agent-sdk/tree/c37007429be8b4465a83487dc1fd0914df0ea734) | 17.8 | Default |
| Secondary | [langgraph](https://github.com/langchain-ai/langgraph/tree/e539ac122f4126f6dd850581c1494948cf620e31) | 13.2 | Default |
| Secondary | [n8n](https://github.com/n8n-io/n8n/tree/4169b55bf3b3e6c255d7361642bc5243bd04345a) | 195.6 | Default |
| Secondary | [microsoft-agent-framework](https://github.com/microsoft/agent-framework/tree/1cd06c5a2058a172eebadf5d9d7c3fa45c519520) | 51.0 | Default |
| Secondary | [autogen](https://github.com/microsoft/autogen/tree/027ecf0a379bcc1d09956d46d12d44a3ad9cee14) | 46.1 | Default |
| Secondary | [cedar](https://github.com/cedar-policy/cedar/tree/2f4019fd645cc8d4a4c0c1f8bd0280c77d754e28) | 8.4 | Default |
| Secondary | [cloudflare-agents](https://github.com/cloudflare/agents/tree/46760e635ce9599add0abbfe6c1a34af0d5d44f1) | 25.1 | Default |
| Secondary | [opa](https://github.com/open-policy-agent/opa/tree/961849b565d2457b80c168f254325b7d5b91461e) | 69.9 | Default |
| Secondary | [claude-code](https://github.com/anthropics/claude-code/tree/b5932767f3acbd07da25367064827e5cb81f43de) | 13.3 | Default |
| Secondary | [gemini-cli](https://github.com/google-gemini/gemini-cli/tree/9c1b0a610534d6f8120964cf2672c07807d8fc90) | 103.1 | Default |
| Secondary | [google-adk-python](https://github.com/google/adk-python/tree/460715b6c62c8e9ab00931c502381ee0364e39b6) | 34.8 | Default |
| Secondary | [openai-codex](https://github.com/openai/codex/tree/516f2780fd227a80cd9fe89488f5039245090b71) | 73.7 | Default |
| Secondary | [openai-agents-python](https://github.com/openai/openai-agents-python/tree/fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292) | 23.7 | Default |
| Secondary | [swe-agent](https://github.com/SWE-agent/SWE-agent/tree/3ea751c087f32b16e039a2233dd6eefecef325d5) | 34.4 | Default |
| Secondary | [cosign](https://github.com/sigstore/cosign/tree/633e8f4303b7db64c369bdfa315374a8bd511ac6) | 3.9 | Default |
| Secondary | [in-toto](https://github.com/in-toto/in-toto/tree/e352b43ad7cb8915d84c36d791aa61346152a0a3) | 0.7 | Default |
| Secondary | [slsa-verifier](https://github.com/slsa-framework/slsa-verifier/tree/30d0be3bbab553fc51557377baba2f7572dfc212) | 299.5 | Large: explicit selection |
| Middle layer | [ard-spec](https://github.com/ards-project/ard-spec/tree/b76f235a8f461876ad4f1e77abd0eb0eb302b48d) | 0.2 | Default |
| Middle layer | [progressive-mcp-guardian](https://github.com/S1LV3RJ1NX/mcp-guardian/tree/4c6a04537b9bc548744b4146168b8fa9896069cb) | 4.3 | Default |
| Middle layer | [mcp-gateway-registry](https://github.com/agentic-community/mcp-gateway-registry/tree/7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73) | 61.6 | Default |
| Middle layer | [context-compress](https://github.com/Open330/context-compress/tree/59fae35a7b383876a34f84090f6da978e230795a) | 1.0 | Default |
| Middle layer | [agent-skills-spec](https://github.com/agentskills/agentskills/tree/69ef37e9424c0a7ea9dd2293b559e43ec8176379) | 1.6 | Default |
| Middle layer | [agentgateway](https://github.com/agentgateway/agentgateway/tree/5e5633bffe6dc5fdd29256648987197f366e462a) | 37.3 | Default |
| Middle layer | [agenttrace](https://github.com/Klepsiphron/agenttrace/tree/9a10f9aae3bc508ddca83093b2d25aede5ad5bd0) | 1.6 | Default |
| Middle layer | [openziti-mcp-gateway](https://github.com/openziti/mcp-gateway/tree/8f99623d95d2f5223d2fa12b9f125688d8c80bf9) | 0.7 | Default |
| Middle layer | [e2b-runtime](https://github.com/e2b-dev/runtime/tree/cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497) | 16.1 | Default |
| Middle layer | [agent-observability](https://github.com/KryptosAI/agent-observability/tree/2658eef467225f376e2e92dc1465839eda2bc113) | 0.3 | Default |
| Middle layer | [ctx-zip](https://github.com/karthikscale3/ctx-zip/tree/76580f7ba1555c891928702743ac74412c7fac60) | 0.3 | Default |
| Community | [paperclip](https://github.com/paperclipai/paperclip/tree/13368c518303e886a5c9445fbc69afcbdf0a9228) | 208.1 | Default |
| Community | [openclaw](https://github.com/openclaw/openclaw/tree/b0638604941dda7eb80095481576898f875caf08) | 550.8 | Large: explicit selection |
| Community | [hermes-agent](https://github.com/NousResearch/hermes-agent/tree/ee4452991d17534aa561f31ee55596d082aa94e7) | 161.8 | Default |
| Community | [gastown](https://github.com/gastownhall/gastown/tree/649b832b7672bc7a2dbef26f5983aba6198b819b) | 22.4 | Default |
| Community | [openshell](https://github.com/NVIDIA/OpenShell/tree/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0) | 26.3 | Default |
| Community | [nemoclaw](https://github.com/NVIDIA/NemoClaw/tree/3ea2d5f9a515d4176e0745214743a48ff4868f4c) | 108.3 | Default |

## Nested sources

| Parent | Source at exact pin | MiB |
|---|---|---:|
| wasmtime | [WebAssembly/testsuite](https://github.com/WebAssembly/testsuite/tree/0dc0343c9876267d99a7577ed4fc2289406a7869) | 11.7 |
| wasmtime | [WebAssembly/wasi-testsuite](https://github.com/WebAssembly/wasi-testsuite/tree/6345da2237c562d9a94e281332c653b5528fdd52) | 210.9 |
| wasmtime | [WebAssembly/component-model](https://github.com/WebAssembly/component-model/tree/7c676115e93cd7d54c1732d95c54c6a3de7c5ae0) | 2.2 |
| temporal | [temporalio/dashboards](https://github.com/temporalio/dashboards/tree/590cbf37af8cef99387b2a1f88c163728b003ea4) | 0.6 |
| nemoclaw | [NVIDIA-AI-Blueprints/llm-router](https://github.com/NVIDIA-AI-Blueprints/llm-router/tree/2bd8dfaa751efb60aa4e7e49b270490dfbc0a68a) | 0.5 |

Canonical revisions: UPSTREAM_LOCK.json, SECONDARY_HARVEST_LOCK.json, MIDDLE_LAYER_HARVEST_LOCK.json and COMMUNITY_HARVEST_LOCK.json. Machine-readable sizes and paths: FABLE_SOURCE_INVENTORY.json and FABLE_NESTED_SOURCE_INVENTORY.json.
