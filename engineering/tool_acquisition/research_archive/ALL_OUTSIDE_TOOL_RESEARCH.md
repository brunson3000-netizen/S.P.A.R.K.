# Consolidated Outside Tool Research Archive

This file preserves the full normalized research notes harvested from prior project lanes for SPARK mission planning. Historical mentions of another project describe the original research context; they do not grant SPARK authority over that project.

# SOURCE FILE: AGENT_SCAFFOLDING.md

# Agent Scaffolding / Corporate Harness Research — Imported Findings

Mechanisms that recur across Claude Code, Codex/OpenAI agent tooling, Gemini CLI/ADK, Microsoft Agent Framework/AutoGen/Magentic-One, Cursor and related systems:
- explicit narrow tool contracts;
- scoped/conditional instruction/context loading;
- durable session/work state;
- compaction/handoff summaries;
- bounded subagent assignments and compressed returns;
- deterministic workflow scheduling where the sequence is known;
- host-side permission/policy engines;
- task/progress ledgers;
- termination/retry controls outside prose.

Acquisition rule: source-mine mechanics. Do not import large vendor prompt packages as governance.

---

# SOURCE FILE: CLI_ANYTHING.md

# CLI-Anything Study Summary for S.W.A.R.M. Tool Architecture

Date: 2026-09-13
Status: completed research reference; no canonical integration authorized or performed.

## Verdict

`STRONG_TOOL-ARCHITECTURE_REFERENCE`

CLI-Anything is most valuable to S.W.A.R.M. as a methodology and pattern library for making existing applications agent-usable. It should not become a S.W.A.R.M. authority core or be adopted wholesale.

## What it is operationally

CLI-Anything is a collection of independently packaged harnesses plus a common methodology and discovery layer. Harnesses expose professional applications or services through grouped CLI commands, structured JSON output, persistent state where needed, self-contained agent skill documentation, and tests. CLI-Hub provides discovery, installation metadata, capability/provider matrices, preflight, and cross-tool preview consumption.

Primary source: https://github.com/HKUDS/CLI-Anything
License: Apache-2.0.

## Strong patterns

1. **Probe before mutate.** Information/status/list commands are treated as first-class surfaces.
2. **Real backend execution.** The mature methodology explicitly says the adapter should drive the actual application/backend rather than silently recreating a toy substitute.
3. **State is explicit.** Stateful workflows have a session/project model instead of relying on hidden GUI state.
4. **Machine-readable mode.** `--json` is a standard agent surface.
5. **Tool teaching is packaged.** `SKILL.md` gives an agent a compact operating manual colocated with the tool.
6. **Discovery is capability-oriented.** CLI-Hub matrices separate task capability from the available providers and support preflight before installation.
7. **Preview is a protocol, not a GUI.** Preview bundles standardize manifest, summary, artifacts, fingerprints, and trajectories while leaving actual rendering to the real application.
8. **Verification should inspect outputs.** Methodology calls for format/content verification instead of trusting a zero exit code.
9. **Installed-command subprocess tests matter.** Harness quality includes testing the actual CLI entry point, not only internal functions.

## Important weaknesses

1. **Implementation quality is uneven across harnesses.** Some older harnesses predate the project's stronger “real software” rule, simulate application behavior, or allow real-backend E2E tests to skip when the application is unavailable.
2. **The methodology and catalog should be evaluated separately.** A good design rule does not prove every existing harness follows it.
3. **CLI-Hub is not a S.W.A.R.M.-grade authority boundary.** Its installer may execute registry-sourced command strings and can use `shell=True` when shell metacharacters are present. That is unacceptable for S.W.A.R.M. canonical authority even though the project treats the registry as trusted.
4. **The project's own security guidance is stricter than parts of the installer.** Harness security guidance says not to use `shell=True` and to validate subprocess arguments with allowlists.
5. **Python packaging and per-harness conventions are not the desired S.W.A.R.M. enforcement plane.** Rust should own contract validation, grants, audit, provenance, and execution admission.

## Contract lesson for S.W.A.R.M.

A S.W.A.R.M. tool contract should make the following mechanically visible:

- identity, version, origin, and provenance;
- purpose and intended scope;
- input schema;
- output schema;
- effect class and mutation level;
- required capabilities and concrete resource scopes;
- inspect/preflight pathways;
- error envelope, retry semantics, recovery, and idempotency;
- verification requirements;
- timeout/resource/output limits;
- audit fields;
- sandbox profile;
- transport bindings;
- qualification status.

CLI-Anything's conceptual agent-native criteria around syntax/control/introspection/rendering/verification/discovery are useful, but S.W.A.R.M. should add two explicit dimensions: **Authority/Effects** and **Provenance/Trust**.

## Best use for S.W.A.R.M.

Use CLI-Anything as a reference library for adapter design and agent-facing contracts, then implement S.W.A.R.M.'s authoritative tool fabric in Rust with stricter capability and provenance semantics.

## G.A.M.E. relevance

The Blender, Godot, LLDB, RenderDoc, LibreOffice, QGIS, and graphics/content harnesses are useful reference models for narrow worker-agent tool bundles. LLDB and RenderDoc are especially relevant for inspection/debugging; Blender and Godot for deterministic development/content workflows.

## Deferred experiment

The recommended first implementation experiment — Rust Tool Contract v0 plus a sandboxed Blender shadow adapter — is intentionally not started. It is recorded in the pending implementation bucket for later operator return.

---

# SOURCE FILE: CLOUDFLARE_OS.md

# Cloudflare OS — Imported Tool/Architecture Findings

High-value patterns:
- resource introduction distinct from existence/connection/grant/effect authorization;
- Gatekeepers for scoped external-resource mediation;
- generated Gadgets as sandboxed applications with machine-facing surfaces;
- Blueprints as capability/resource requirements rather than credential-bearing clones;
- human approval around consequential side effects;
- observer/information-flow controls;
- provider-independent bindings.

Disposition: secondary code harvest/research. Use as a pattern and optional external execution/resource environment candidate, not as S.P.A.R.K.'s or S.W.A.R.M.'s kernel.

---

# SOURCE FILE: DETERMINISTIC_ENGINEERING_UTILITIES.md

# Deterministic Engineering Utilities — Immediate Acquisition Targets

These candidates map directly to tool gaps recovered from NIM/G.A.M.E./S.W.A.R.M. research.

## ast-grep + tree-sitter
Use for syntax-aware repository search, symbol/context slicing and structured transformations. First qualification should compare useful-context bytes/tokens against grep/file-window baselines on Rust and Python tasks. Prefer ast-grep as the first consumable interface; tree-sitter is the parser substrate/reference.

## cargo-nextest
Use for fast, machine-addressable Rust test execution and partitioning. Qualification: run the same selected suite with Cargo test and nextest; compare semantics, output structure, filtering and failure evidence before adopting any workflow dependency.

## cargo-mutants
Use for falsification/mutation testing to find weak tests and unproved invariants. Qualification: bounded small-module experiment with strict time budget; retain only actionable surviving mutants.

## Toxiproxy
Use for deterministic latency, timeout, reset and connection-failure injection around provider/tool transports. Qualification: a local mock endpoint plus fixed toxic profiles and replayable expected outcomes.

## wiremock-rs
Use as a Rust-local HTTP server/mock substrate for provider cassettes and contract tests. Pair with Toxiproxy: wiremock controls semantic responses; Toxiproxy controls network failure characteristics.

These tools are preferable to asking an LLM to simulate facts that deterministic software can generate directly.

---

# SOURCE FILE: GAME_TOOLING.md

# G.A.M.E. External Tool Research — Imported Findings

G.A.M.E. research repeatedly favors reusable deterministic development tools over premature orchestration infrastructure.

High-value surfaces:
- reproduction harnesses;
- targeted repository/code inspection;
- test selection and fast execution;
- evidence retrieval;
- LLDB/native debugging;
- RenderDoc/GPU inspection;
- Blender asset/render automation;
- Godot/headless engine tooling where relevant;
- disposable test copies and fault injection.

S.P.A.R.K. should prepare these as reusable capabilities without imposing a Foreman, coordinator, or production-game topology.

---

# SOURCE FILE: MCP_RMCP.md

# MCP / rmcp — Imported Findings

Immediate action: harvest official Rust SDK and MCP Inspector.

Durable boundary:
- MCP may own transport, schema exchange, discovery mechanics and invocation;
- remote names and advertised metadata are local/untrusted descriptors, not canonical identity;
- discovery is advisory; execution needs current host admission;
- stale caches are never authority;
- continuation and task handles carry protocol correlation, not grants;
- timeout/cancel should map into host-owned typed outcomes;
- tracing metadata is correlation only.

S.P.A.R.K. use: create reusable protocol/adaptor components that a consuming project can place behind its own authority and resource policy.

---

# SOURCE FILE: NIM_ENGINEERING_TOOL_NEEDS.md

# NIM Engineering-Tool Needs — Imported Findings

Origin: NIM Engineering Tools research and independent tool-needs elicitation.

## Preserve
- provider-neutral `ComputeExecutionBackend` style seam;
- deterministic request validation, timeouts, bounded concurrency, retries, health/quarantine and evidence preservation;
- raw prompt/response evidence by reference/hash;
- one bounded worker as the primitive; panels are deterministic composition;
- capability vocabulary `review/tests/falsify/research` as research baseline, not frozen S.P.A.R.K. API.

## Acquire or build only if external acquisition fails
1. contract/evidence verifier;
2. bounded-concurrency checker;
3. provider cassette + network fault harness;
4. structured validation delta reporter;
5. symbol/context slicer;
6. failure-transition replay harness;
7. mission/fact/evidence ledger.

## S.P.A.R.K. correction
NIM-provider identity is an implementation backend, not the upper tool architecture. S.P.A.R.K.'s consolidation should stay provider-neutral and must not recreate another project's resource/credential/economic authority.

---

# SOURCE FILE: OBSERVABILITY_EVIDENCE.md

# Observability / Evidence Tool Research — Imported Findings

OpenTelemetry/OTLP work demonstrated a useful separation:
- project semantic truth remains project-owned;
- external telemetry is downstream observation;
- exporter failure must not redefine upstream health/authority;
- SDK types need not leak into core contracts;
- bounded queues and nonblocking export prevent observer backpressure from taking ownership of the system;
- identity/correlation metadata must not be confused with executable authority.

Disposition: OTel Collector and related tooling remain secondary acquisition targets for interoperability, test/evidence export and debugging.

---

# SOURCE FILE: OPEN_SOURCE_CANDIDATE_PASS.md

# Open-Source MCI Tool Candidates — Research Pass

Date: 2026-09-13
Status: research only; no adoption or integration authorization.

## Executive result

There is no compelling reason to replace the MCI or S.W.A.R.M. authority architecture with an external agent platform. There is, however, a strong set of open-source projects that can accelerate distinct layers of the tool fabric.

The most useful decomposition is:

- **S.W.A.R.M. remains the authority and canonical registry.**
- **Protocol libraries** handle external interoperability below authority.
- **Sandbox runtimes** enforce concrete resource boundaries below admission.
- **Qualification tools** independently test declared tool surfaces.
- **Agent/client protocols** can improve the MCI's window into agents without becoming governance.
- **Telemetry standards** can export evidence without becoming the evidence source of truth.

## Priority 1 — ToolHive

Project: https://github.com/stacklok/toolhive
Docs: https://docs.stacklok.com/toolhive/
License: Apache-2.0.

### What it is

ToolHive is an open-source runtime and management layer for MCP servers and agent skills. Its design separates runtime isolation, registry/discovery, gateway aggregation, and operator surfaces.

### Why S.W.A.R.M. should study it

The architecture closely matches a S.W.A.R.M. need: a tool is not merely discoverable; it runs inside a constrained environment with explicit filesystem/network/secrets policy, while a separate registry and gateway handle discovery and access. It also emphasizes centralized audit/observability and capability-oriented organization.

### Borrowable lessons

- separate registry, runtime, gateway, and UI/operator surfaces;
- permission profiles as data, not hidden runtime convention;
- isolate each external tool/server rather than sharing ambient process authority;
- preflight/availability before task execution;
- audit and observability at the gateway/runtime boundary;
- curate trusted registries instead of letting an agent install arbitrary endpoints;
- treat tool-description filtering/search as an output/token-cost problem.

### S.W.A.R.M. boundary

Do not outsource canonical S.W.A.R.M. tool identity, authority, provenance, or admission decisions to ToolHive. Direct reuse, if ever pursued, should be limited to a subordinate external MCP runtime or development/qualification environment.

Classification: `BORROW_PATTERN`, possible `DIRECT_REUSE_POSSIBLE` below authority.

## Priority 2 — Goose

Project: https://github.com/aaif-goose/goose
Docs: https://block.github.io/goose/
License: Apache-2.0.
Language: Rust.

### What it is

Goose is an open-source Rust agent runtime/application with CLI/desktop/API surfaces, MCP extensions, recipes, subagents, ACP interoperability, and a growing security inspection layer.

### Why S.W.A.R.M. should study it

Its source tree contains explicit security inspectors for adversarial content, egress, permissions, and repetition/tool behavior. The agent composes multiple inspectors rather than treating tool safety as one pre-call boolean. That layered pipeline is highly relevant to S.W.A.R.M.'s supervisor/tool-admission model.

### Borrowable lessons

- composable tool inspection stages;
- separate permission routing from tool implementation;
- egress inspection as an explicit layer;
- extension/recipe packaging for agent teaching;
- MCP and ACP abstractions that do not require one UI;
- security checks surrounding calls rather than embedded ad hoc in every tool.

### S.W.A.R.M. boundary

Goose is an agent product/runtime, not a replacement for S.W.A.R.M.'s authority core. Reuse should focus on patterns and carefully audited Rust modules only where they fit existing boundaries.

Classification: `BORROW_PATTERN`.

## Priority 3 — Official MCP Rust SDK (`rmcp`)

Project: https://github.com/modelcontextprotocol/rust-sdk
Docs: https://rust.sdk.modelcontextprotocol.io/
License: Apache-2.0.
Language: Rust.

### What it is

The official Rust implementation of Model Context Protocol client/server mechanics, including structured tools/resources/prompts, JSON schema integration, transport, and authorization-related support.

### Why S.W.A.R.M. should study/reuse it

S.W.A.R.M. does not need to reimplement a public interoperability wire protocol merely to preserve authority. `rmcp` is the strongest direct code-reuse candidate in this research set if MCP is exposed below a S.W.A.R.M.-owned tool contract.

### Required separation

MCP identity, tool annotations, list responses, and authorization must not become substitutes for S.W.A.R.M.'s canonical agent/tool identity, grants, economics, mutation policy, or provenance. MCP should be treated as a transport/binding.

Classification: `DIRECT_REUSE_POSSIBLE`, `BORROW_PATTERN`.

## Priority 4 — Wasmtime + WASI + cap-std

Projects:
- https://github.com/bytecodealliance/wasmtime
- https://github.com/bytecodealliance/cap-std

Licenses: permissive Apache-2.0/MIT-family as applicable.
Language: Rust.

### What it is

Wasmtime embeds WebAssembly components/modules in a host process. WASI and capability-oriented libraries such as cap-std allow the host to decide exactly which filesystem/network-like resources a component can access instead of inheriting ambient OS authority.

### Why S.W.A.R.M. should study it

This is the strongest Rust-native candidate for a future “compiled narrow tool” model: tool code receives only the imports/resources S.W.A.R.M. grants. A tool can be denied arbitrary filesystem, network, and process access by construction.

### Critical caution

Sandbox libraries are containment layers, not governance. They can have vulnerabilities and must be version-pinned, patched, regression-tested, and wrapped by S.W.A.R.M. admission. A 2026 Wasmtime WASI advisory involving hardlink/rename permission enforcement is exactly why the sandbox must not be treated as the sole authority boundary.

Classification: `DIRECT_REUSE_POSSIBLE` substrate plus `REIMPLEMENT_IN_RUST` S.W.A.R.M. wrapper.

## Priority 5 — Extism

Project: https://github.com/extism/extism
Docs: https://extism.org/docs/
License: BSD-3-Clause.

### What it is

A WebAssembly plugin framework with a Rust host SDK and manifest-driven limits such as allowed paths/hosts, memory constraints, hashes, variables, and host functions.

### Why it matters

Extism may provide a faster experimental path than building directly on Wasmtime. Its manifest resembles a capability grant and defaults can be made narrow. It is well suited for a bounded comparison: “does the higher-level abstraction preserve enough S.W.A.R.M. control, or should the project embed Wasmtime directly?”

Classification: `DIRECT_REUSE_POSSIBLE` for prototype; no governance role.

## Priority 6 — Agent Client Protocol (ACP)

Project: https://github.com/agentclientprotocol/agent-client-protocol
Site: https://agentclientprotocol.com/ and https://zed.dev/acp
License: Apache-2.0.

### What it is

A JSON-RPC protocol for connecting agents to clients/editors. The ecosystem includes a Rust SDK and separates agent/session messaging from the editor/client surface.

### MCI relevance

ACP is a strong candidate for the “MCI is the window” side of the architecture. It can inform how the MCI starts or attaches to agent sessions, streams updates, exposes limited client filesystem capabilities, and carries permission/elicitation interactions without embedding every agent's private protocol into the MCI.

### Boundary

ACP should never decide project governance or canonical action authority. It is an interaction/session transport.

Classification: `BORROW_PATTERN`, possible `DIRECT_REUSE_POSSIBLE` transport.

## Priority 7 — MCP Inspector

Project: https://github.com/modelcontextprotocol/inspector
License: MIT.

### What it is

The official MCP developer inspection utility with interactive and automation-oriented surfaces for listing and exercising MCP tools/resources/prompts and authorization flows.

### S.W.A.R.M. relevance

This is valuable primarily as an independent qualification oracle. A S.W.A.R.M. MCP binding should be testable from outside the S.W.A.R.M. codebase. Inspector-driven smoke/conformance tests can catch mismatches in discovery, schemas, calls, errors, authentication challenges, and transport behavior.

Classification: `DIRECT_REUSE_POSSIBLE` for development/qualification; not a production authority component.

## Priority 8 — wasmCloud

Project: https://github.com/wasmCloud/wasmCloud
Docs: https://wasmcloud.com/docs/
License: Apache-2.0.
Language: Rust-heavy ecosystem.

### Why study it

wasmCloud is a useful architecture reference for host-mediated capabilities and WebAssembly component contracts. Components operate through capabilities/providers rather than assuming unrestricted host access, and WIT provides explicit interface contracts.

### Why not adopt it wholesale

Its distributed/Kubernetes-oriented runtime is much larger than the current MCI tool-fabric need. The value is the capability-host model and component interface discipline.

Classification: `BORROW_CONCEPT`, `BORROW_PATTERN`.

## Priority 9 — Nushell

Project: https://github.com/nushell/nushell
Docs: https://www.nushell.sh/book/plugins.html
License: MIT.
Language: Rust.

### Why study it

Nushell treats pipeline values as structured data and has a versioned plugin protocol. Plugin registration discovers commands and caches capability information. Those are useful patterns for agent-facing output compression, command catalogs, compatibility negotiation, and separating structured results from terminal prose.

### Boundary

A general shell remains too much ambient authority for specialist S.W.A.R.M. agents. Borrow the structured-data and plugin-discovery patterns, not the broad shell surface.

Classification: `BORROW_PATTERN`, `SAFE_REFERENCE_ONLY` for shell use.

## Priority 10 — OpenTelemetry Collector

Project: https://github.com/open-telemetry/opentelemetry-collector
Docs: https://opentelemetry.io/docs/collector/
License: Apache-2.0.

### Why study it

Its receiver → processor → exporter pipeline and explicit component lifecycle are strong references for a tool evidence/telemetry fabric. S.W.A.R.M. could potentially export selected audit/telemetry through OTLP while keeping canonical evidence semantics internal.

Classification: `BORROW_PATTERN`; possible interoperability bridge.

## Secondary reference — OpenHands Software Agent SDK

Project: https://github.com/OpenHands/software-agent-sdk
License: MIT for the SDK.

The SDK/server architecture separates agents, conversations, tools, workspaces, events, and client communication. This is useful for comparison against MCI frontend/runtime boundaries and isolated workspaces, but it is Python-centric and does not justify replacing S.W.A.R.M. core architecture.

Classification: `BORROW_PATTERN`, `SAFE_REFERENCE_ONLY` near term.

## Secondary reference — Temporal

Project: https://github.com/temporalio/temporal
License: MIT.

Temporal's durable workflow/event-history model is worth retaining as a reference for long-running tool jobs, retries, recovery, idempotent activities, and crash-resume semantics. It is a substantial Go service stack and currently looks heavier than the problem S.W.A.R.M. needs to solve.

Classification: `BORROW_CONCEPT`, `SAFE_REFERENCE_ONLY` near term.

## Combined architecture lesson

The best ideas from these projects compose cleanly without adopting any one platform:

1. **Authoritative Rust Tool Contract** owned by S.W.A.R.M.
2. **Capability registry** owned by S.W.A.R.M., inspired by CLI-Anything and ToolHive discovery/preflight.
3. **Transport bindings** implemented with audited libraries such as `rmcp`, and possibly ACP on the MCI/agent side.
4. **Execution containment** supplied by an isolated process/container or Wasm capability runtime, with S.W.A.R.M. still performing admission.
5. **Layered pre/post-call inspection** inspired by Goose.
6. **Independent conformance/qualification** using MCP Inspector and S.W.A.R.M.-owned tests.
7. **Structured compact outputs** inspired by CLI-Anything and Nushell.
8. **Preview/evidence artifacts** using explicit manifests, hashes, and provenance.
9. **Telemetry export** through standard OTLP/OpenTelemetry only after canonical evidence is recorded.

## Strongest findings

- **Strongest next deep research target:** ToolHive.
- **Strongest Rust direct-reuse candidate:** official MCP Rust SDK (`rmcp`).
- **Strongest containment substrate to experiment with:** Wasmtime/cap-std, compared against Extism.
- **Strongest MCI-facing protocol candidate:** ACP.
- **Strongest runtime security-pattern source:** Goose.
- **Strongest independent interface test tool:** MCP Inspector.

## Recommendation

Do not start another implementation yet. Preserve this research, leave the Blender/Rust spike pending, and use the next tool-research cycle to inspect ToolHive and Goose source-level permission/admission paths plus a concrete `rmcp`/ACP/Wasmtime integration map. That gives S.W.A.R.M. a higher-confidence contract before code is promoted.
