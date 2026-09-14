# MCI tool-source assessment

## Decision

Keep the collection as a set of interchangeable mechanisms, not a candidate replacement for SWARM. Governance research should concentrate on a host-owned admission contract, current policy and identity, bounded delegation, enforced execution, and durable evidence. Agent frameworks can supply adapters and interface lessons; their roles, prompts, sessions and checkpoints do not become authority.

Every one of the 54 locked sources now has a screening disposition and evidence pointer. Twenty retain pin-matching prior first traces; six have new scoped code studies; twenty-five have new narrow code screens; three have documentation screens. The latter include E2B, whose existing deeper trace remains in progress. One focused ctx-zip reproduction was executed. This closes the catalog-wide static assessment, not runtime qualification or an exhaustive audit of every retained mechanism. [Coverage](../MCI_REVIEW_COVERAGE_2026-09-14.json) records the distinction per source.

The dispositions are 29 BORROW_PATTERN, 11 EXPERIMENT_NOW, 8 ARCHIVE_REFERENCE, 3 BORROW_IDEA, 2 DEFER and 1 REUSE_CODE. REUSE_CODE is the prior rmcp protocol recommendation, conditional on adapter qualification; it does not mean a dependency was introduced. EXPERIMENT_NOW records priority, not an executed or approved runtime adoption.

## Consumer requirements and interpretation

Consumer baseline is [SWARM commit 673b705d34ec173f6c359d9fca81d7c539219ada](https://github.com/brunson3000-netizen/SWARM/tree/673b705d34ec173f6c359d9fca81d7c539219ada). The following are research comparison criteria derived from the cited records, not new policy:

| Criterion | Recorded consumer need | Research consequence |
|---|---|---|
| R1 — MCI separation | MCI displays state and requests commands; UI loss must not destroy Core state | Keep operational truth, approvals and evidence outside visual widgets |
| R2 — trusted identity | Node descriptors/capabilities do not carry trusted authority; bound ports stamp identity | Never accept model-supplied identity or discovery membership as a grant |
| R3 — replaceable agents | Supervisors, coordinators and workers are disposable; deterministic state/protocol carries continuity | Replacements need fresh admission, task/attempt identity and fencing; no stale lease inheritance |
| R4 — bounded computation | Provider-neutral services, local/free paths and future task-spend controls | Keep paid/model SDKs in adapters; distinguish observed cost from reserved authority |
| R5 — independent evidence | Stable IDs, event chains, live state, historical telemetry and lifetime ledger are distinct | Commit evidence before sample/drop-capable telemetry; label estimates and derived state |
| R6 — reversible scoped editing | Panel Designer is temporary and panel-scoped; protected controls and secrets remain outside normal design editing | Declarative proposals need validation and current scope; editing state is not execution authority |

Primary consumer sources: [MCI product specification §§1,4,16–28](https://github.com/brunson3000-netizen/SWARM/blob/673b705d34ec173f6c359d9fca81d7c539219ada/docs/MCI_V2_SPEC.md), [MCI architecture: package and persistence boundaries](https://github.com/brunson3000-netizen/SWARM/blob/673b705d34ec173f6c359d9fca81d7c539219ada/docs/MCI_V2_ARCHITECTURE.md), [Plexus architecture: nodes, authority and typed fabrics](https://github.com/brunson3000-netizen/SWARM/blob/673b705d34ec173f6c359d9fca81d7c539219ada/docs/PLEXUS_CORE_ARCHITECTURE.md), [disposable-supervisor clarification](https://github.com/brunson3000-netizen/SWARM/blob/673b705d34ec173f6c359d9fca81d7c539219ada/project_records/decisions/2026-09-12_CONSTITUTION_298_AND_DISPOSABLE_SUPERVISOR_CLARIFICATION.md).

The [governance index](https://github.com/brunson3000-netizen/SWARM/blob/673b705d34ec173f6c359d9fca81d7c539219ada/project_records/governance/GOVERNANCE_INDEX.md) requires doctrine-gate verification before active reliance. That gate was not run in SWARM, and this assessment makes no constitutional-compliance, accepted-topology or implementation-authorization claim. Rust fit below is the acquisition lane's preference for small deterministic tooling; it does not change MCI's documented Qt/QML/Python presentation architecture.

## Governance and policy

The best near-term study shortlist is ordered by the distinct governance gap it addresses, rather than audience size.

| Priority | Mechanism and code evidence | Value for MCI | Boundary or blocker |
|---|---|---|---|
| First | [Cedar authorizer and public FFI](../sources/CEDAR_MCI_REVIEW_2026-09-14.md) | Rust evaluator with typed decision and diagnostics | An errored forbid can be skipped while another permit allows; host must apply its own diagnostics/error rule |
| First | [Codex ToolRuntime orchestrator](../sources/OPENAI_CODEX_MCI_SCREEN_2026-09-14.md); [Grok permission pipeline](../sources/GROK_BUILD.md) | Ordered call-time admission, explicit sandbox attempt and constrained escalation | Core/session dependencies are substantial; no wholesale import or inherited policy precedence |
| First | [ADK confirmation resolver](../sources/GOOGLE_ADK_PYTHON_MCI_SCREEN_2026-09-14.md); [Microsoft standing approvals](../sources/MICROSOFT_AGENT_FRAMEWORK_MCI_SCREEN_2026-09-14.md) | Bind approval to recorded call, server and arguments; consume/reconcile responses | Add immutable tool version and policy epoch; Microsoft's test intentionally permits same-name implementation replacement |
| First | [Paperclip approvals and budget services](../sources/PAPERCLIP_MCI_REVIEW_2026-09-14.md) | Conditional approval transition, retry no-op, live admission and cancellation wiring | Approval/effect crash window and prospective concurrent budget reservation remain unqualified |
| First | [OpenShell Landlock path](../sources/OPENSHELL_MCI_REVIEW_2026-09-14.md); [NemoClaw policy boundary](../sources/NEMOCLAW_MCI_REVIEW_2026-09-14.md) | Distinguish requested, applied, rejected and uncertain enforcement | BestEffort can continue without Landlock; requirement inclusion is not an excess-permission check |
| Next | [agentgateway RBAC](../sources/AGENTGATEWAY_MCI_SCREEN_2026-09-14.md); [OpenZiti CallPolicy](../sources/OPENZITI_MCP_GATEWAY_MCI_SCREEN_2026-09-14.md) | Method/resource policy and settle-once arguments before dispatch | Empty rules may allow; local canonical paths are not filesystem confinement |
| Comparison | [OPA SDK/rego](../sources/OPA_MCI_REVIEW_2026-09-14.md) | Alternative error and versioned-policy semantics | Go/service or separately qualified Wasm boundary; not needed merely to obtain a boolean |

Cedar and OPA demonstrate why evaluator output is only one input to admission. Defaults are product choices: Cedar skips policy errors; OPA exposes strict builtin error handling and generic decision values. Neither supplies SWARM's authenticated subject, fresh policy state, action version, resource identity, reservation or host enforcement. A shared adversarial corpus is more informative than choosing a policy language by popularity.

Paperclip contributes real operational wiring beyond README claims: conditional approval resolution, observed-cost calculations, invocation blocking, and cancellation hooks into heartbeat work. It does not establish a hard mission cap before parallel spend in the traced paths. A prospective cap needs an atomic reservation/release contract and explicit uncertainty after provider timeout. A UI should show committed spend, reserved exposure and unknown settlement separately.

Approval interfaces also differ materially. ADK binds historical function-call identity and arguments. Microsoft separates tool-wide and argument-specific standing rules and hosted-server identity, yet its test suite deliberately accepts a same-name tool upgrade during approval resume. That is not automatically a defect in its contract; it is insufficient for an MCI grant intended to cover exact executable identity. Gemini's compound-shell tests and Hermes' deny-floor ordering offer additional negative cases, but Hermes' configuration-error path cannot be assumed fail-closed. [Gemini](../sources/GEMINI_CLI_MCI_SCREEN_2026-09-14.md); [Hermes](../sources/HERMES_AGENT_MCI_SCREEN_2026-09-14.md).

## Coordination and supervision

Prefer deterministic task/attempt state and explicit lifecycle commands over a fixed hierarchy of named agents. Gas Town offers concrete supervisor roles and liveness files, but a heartbeat timestamp is not ownership or permission. Its corrupt pause state returns an error; the consumer must not discard that error and interpret the remaining false value as permission to run. [Gas Town](../sources/GASTOWN_MCI_REVIEW_2026-09-14.md).

Temporal and LangGraph provide complementary lifecycle lessons. Temporal's cancellation handler records durable intent and schedules follow-up; cancellation acceptance is not physical process termination. LangGraph interruption resumes the node from the beginning, so code before the interrupt can execute again. Its guarded late writes prevent a stale attempt from updating graph state but do not undo external effects. Borrow durable intent, explicit replay and idempotency, while preserving a separate stop receipt and current attempt epoch. [Temporal](../sources/TEMPORAL_MCI_SCREEN_2026-09-14.md); [LangGraph](../sources/LANGGRAPH_MCI_SCREEN_2026-09-14.md).

OpenHands, ADK, Codex and Microsoft offer adapter/harness mechanisms. OpenClaw's per-agent policy selection can override global lists and preserves an empty allow list as allow-all, which is not a parent-to-child authority intersection. OpenAI Agents' parallel input guardrail is not a mandatory pre-effect host gate. AutoGen and SWE-agent belong in the historical/reference lane at these pins; Cloudflare's durable workflow wait is useful API prior art tied to its platform. [All-source coverage](../MCI_REVIEW_COVERAGE_2026-09-14.json).

The MCI operator surface should distinguish assigned, admitted, running, cancellation-requested, stopped, orphaned, reconciling and completed. Completion requires accepted evidence, not merely a worker's final message. This vocabulary is a proposed interface comparison, not an implemented state machine.

## Security and containment

Use raw Wasmtime as the existing proposed baseline for a small plugin containment experiment and compare Extism's implementation savings against its larger host-side manifest-loading boundary. Guest isolation begins after acquisition/loading decisions; a manifest can exercise host authority before guest code runs. Host functions and I/O need their own resource/timeout accounting. Prior containment source records and addendum remain the evidence basis. [Wasmtime](../sources/WASMTIME_WASI.md); [Extism](../sources/EXTISM.md); [containment comparison](CONTAINMENT_MATRIX.md).

OpenShell makes enforcement modes explicit but can continue without Landlock in BestEffort. Its inspected positive device-path test is not a hostile escape test. wasmCloud's address policy adds useful mapped-IP and special-address checks, with private IPs permitted by default; resolving an IP once is not proof of the full connect-time boundary. OpenZiti rejects remote path-policy configurations in its tests, appropriately limiting assumptions about local canonical paths. These mechanisms are complementary; none alone proves all tool effects confined. [OpenShell](../sources/OPENSHELL_MCI_REVIEW_2026-09-14.md); [wasmCloud](../sources/WASMCLOUD_MCI_SCREEN_2026-09-14.md); [OpenZiti](../sources/OPENZITI_MCP_GATEWAY_MCI_SCREEN_2026-09-14.md).

E2B remains a separately active microVM study. Its documentation describes a Linux Firecracker control/data-plane separation and snapshot-backed lifecycle. That supports retaining it as prior art, not claiming tested isolation, native Windows availability or fresh grants on resume. Do not buy or deploy an E2B service to close a documentation screen. [E2B](../sources/E2B_RUNTIME_MCI_SCREEN_2026-09-14.md).

## Basic tools and adapters

CLI-Anything, Nushell and RTK belong here, with protocol and evidence cross-links. The useful common shape is a deterministic command contract with typed inputs, bounded visible outputs, exact exit/error semantics and immutable evidence references. Application-specific implementation remains behind an adapter.

CLI-Anything's Blender harness separates editing state and undo/redo, but its save helper truncates in place while describing the operation as atomic; its inspected tests do not require actual Blender. Retain command and test design, then qualify a real application shadow adapter with crash-safe writes. Nushell's typed protocol negotiates semantic versions and features, deliberately normalizing prereleases. RTK's compact output is a projection, not a replacement for raw output or authority. [CLI-Anything](../sources/CLI_ANYTHING_MCI_SCREEN_2026-09-14.md); [Nushell](../sources/NUSHELL_MCI_SCREEN_2026-09-14.md); [RTK](../sources/RTK.md).

Do not classify every agent framework as a basic tool: its scheduling, memory, credential and approval behavior has materially different dependencies. File each source once and cross-reference its small useful mechanisms.

## Protocols and discovery

rmcp is the strongest existing Rust code-reuse candidate for MCP mechanics. Its source study explicitly separates protocol capability negotiation from business authority and identifies in-memory task/session and stale-cache caveats. Reauthorize each actual invocation outside discovery/router state. [rmcp](../sources/RMCP.md).

ARD, progressive disclosure, registry and skills studies support a compact find → learn → request interface. Stable canonical identity must be separate from display aliases, advertised capabilities and transport session IDs. n8n's MCP resolver is useful collision-handling prior art but its full workflow platform and license constraints make it a reference choice. ACP adds client-agent lifecycle vocabulary; optional generic cancellation cannot substantiate an operator stop indicator. [ARD](../sources/ARD.md); [Guardian](../sources/PROGRESSIVE_MCP_GUARDIAN.md); [registry](../sources/MCP_GATEWAY_REGISTRY.md); [skills](../sources/AGENT_SKILLS.md); [n8n](../sources/N8N_MCI_SCREEN_2026-09-14.md); [ACP](../sources/ACP_SPEC_MCI_SCREEN_2026-09-14.md).

## Context and memory

Preserve canonical bytes first, then derive syntax slices, compact previews, search indexes and summaries. ast-grep and Tree-sitter offer deterministic structural extraction; correctness requires measuring missed targets and parse-health errors, not merely token savings. context-compress offers searchable spillover but is not itself a byte-exact immutable archive. Grok's existing study contributes read-before-write snapshots and stale-edit rejection. [Structural slicing](../sources/AST_GREP_TREE_SITTER.md); [context-compress](../sources/CONTEXT_COMPRESS.md); [Grok](../sources/GROK_BUILD.md).

The ctx-zip local reproduction is a concrete rejection of direct evidence-store reuse: same-name calls share a filename and earlier payloads are overwritten; callers' nested messages are mutated; a supplied serializer is ignored. The correct transfer is the idea of compact retrievable references, implemented over immutable call-specific artifacts. Four adverse assertions held at exact pinned bytes under Node 24.19.0. [Failure and reproduction](../failures/CTX_ZIP_EVIDENCE_OVERWRITE_2026-09-14.md).

Historical memory and external study notes remain context. They do not authorize policy edits, software removal, new grants or consequential actions. This boundary follows the consumer identity/admission model and prevents research artifacts from masquerading as governing instructions.

## Audit, observability and provenance

Keep a canonical host event/evidence record separate from OTel projection, best-effort tracing and model self-report. The prior studies show why: AgentTrace has a shared mutable async context risk and lossy OTLP export; agent-observability's proxy does not correlate ordinary tool replies correctly; Collector enqueue and persistence semantics do not equal a canonical evidence commit. [Observability matrix](OBSERVABILITY_MATRIX.md).

MCI should display current state, historical samples, lifetime aggregates and cost estimates as distinct products. Use stable event IDs and causal/task/attempt references. Sensitive or large payloads should be referenced with bounded sanitized previews, and redaction/retention must not destroy the evidence needed for accepted completion.

Cosign and in-toto concern artifact provenance, not behavioral permission. in-toto verification can execute layout inspection commands, so it cannot be treated as automatically passive. SLSA Verifier's maintenance status supports reference-only use. Defer a signing/distribution dependency until an actual package workflow needs it. [Cosign](../sources/COSIGN_MCI_SCREEN_2026-09-14.md); [in-toto](../sources/IN_TOTO_MCI_SCREEN_2026-09-14.md); [SLSA](../sources/SLSA_VERIFIER_MCI_SCREEN_2026-09-14.md).

## Testing and qualification

Use wiremock-rs and Toxiproxy to observe denied-before-dispatch, delayed replies, connection loss and retry duplication. nextest and cargo-mutants can improve execution and falsification of established Rust tests, but neither substitutes for project-required gates or independent review. MCP Inspector can send real calls and extra display-related resource probes, so qualification fixtures must record actual RPC traffic. [Fault tools](../sources/WIREMOCK_TOXIPROXY.md); [Rust test tools](../sources/CARGO_NEXTEST_MUTANTS.md); [Inspector](../sources/MCP_INSPECTOR_MCI_SCREEN_2026-09-14.md).

A next qualification batch should use deterministic fixtures, without paid inference:

| Gate | Required falsification | Acceptance observation |
|---|---|---|
| Admission | Erroring forbid with allow; empty policy; missing identity; stale epoch | No side effect unless the explicit host contract admits it |
| Approval | Changed arguments, tool digest, resource revision or policy epoch; replay; forged response | Fresh matching approval or clear denial; no ambiguous grant reuse |
| Delegation | Replaced supervisor, expired lease, child escalation, duplicate work | Stale attempt fenced; child bounds preserved; one accepted effect |
| Budget | Parallel requests at cap, duplicate completion, lost billing acknowledgement | Atomic reservations and explicit unsettled exposure |
| Stop | Cancellation acknowledgement while child still runs; orphaned process | Requested and physically stopped remain distinct |
| Containment | Host manifest load, callback hang, denied file/network, unavailable backend | Required enforcement fails closed and produces applied-state evidence |
| Evidence | Same-name calls, concurrent writes, changed reader file, missing telemetry | Original bytes remain retrievable and correctly attributed |
| Recovery | Crash before/after effect or evidence commit; duplicate retry | Idempotent convergence or explicit reconciliation, never invented success |

These are proposed qualification gates, not completed experiments. The [gap ledger](MCI_REVIEW_GAPS_2026-09-14.md) binds remaining work to the relevant mechanisms.

## Source custody and supplements

All 54 pins are preserved; source preparation remains 51 top-level plus five nested repositories verified in the prior preparation workspace, not newly verified on Fable's machine. OpenClaw, Goose and SLSA Verifier remain above the normal size cutoff and were available through pinned remote source evidence.

Grok Build remains supplemental: its pinned recursive tree was retrieved without truncation, totaling 73,042,597 tracked blob bytes (about 69.7 MiB), and the root Apache-2.0 license was read. It is not in the 54-source locks/gitlinks and no local checkout was verified in this pass. Remote source availability resolves code-reading access, not the remaining local-custody gap. Cloudways remains a documentation-only managed-operations reference, separate from Cloudflare Agents; its private control plane is not credited as harvested code.

The most useful immediate output for Fable is the separate governance packet plus these source-specific dispositions. Later work can reuse the other seven categories without redoing the catalog sweep. No framework, dependency, service or consumer policy was adopted by this assessment.
