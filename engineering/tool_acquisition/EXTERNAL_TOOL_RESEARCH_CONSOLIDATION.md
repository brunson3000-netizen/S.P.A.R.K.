# Cross-Project External Tool Research Consolidation

This is the canonical S.P.A.R.K. intake summary for recovered outside-tool research. It preserves findings while stripping project-specific authority claims that S.P.A.R.K. does not own.

## 1. NIM Engineering Tools

Recovered product thesis:
- independent bounded workers can add real engineering value when their output is evidence rather than authority;
- the durable asset is a provider-neutral engineering capability/evidence contract, not the NIM provider itself;
- preserve deterministic execution, bounded concurrency, health/quarantine/retry, raw evidence, provenance and compatibility surfaces;
- generic capability candidates are `review`, `tests`, `falsify`, `research`;
- high-value missing deterministic helpers identified by independent model elicitation: contract/evidence verification, bounded concurrency checks, provider cassette/fault testing, structured validation deltas, symbol slicing, failure-transition replay, and an authority-aware mission/fact ledger.

S.P.A.R.K. implication: inherit the engineering-tool problem set and provider-neutral seam. Do not inherit NVIDIA-specific policy into the upper tool contract.

## 2. Historical S.W.A.R.M. engineering-tool gap research

Durable lesson: the best external helper converts a plausible model conclusion into a small replayable deterministic fact. Prioritize trustworthy state/evidence compression, contract/invariant verification, bounded fresh context, and proof of failure transitions. Model calls are strongest at hypothesis generation and synthesis; deterministic utilities should own facts they can compute.

S.P.A.R.K. implication: acquisition should favor tools that reduce silent-wrongness and context cost, not merely add another agent framework.

## 3. MCP / rmcp compatibility research

Recovered boundary:
- MCP is a good discovery/schema/serialization/transport/invocation protocol;
- MCP metadata, remote names, discovery visibility, task handles and trace context are not authority;
- call-time revalidation belongs to the consuming host;
- stale discovery must not become admission;
- long-running protocol state must not silently become project task truth.

S.P.A.R.K. implication: harvest official Rust SDK and Inspector. Build any future adapter so a consumer can supply its own authority/admission semantics.

## 4. Cedar authorization research

Cedar fit was strong as an evaluator substrate, with an important fail-closed caveat: Cedar can return Allow while reporting policy evaluation errors due to skip-on-error semantics. A consuming host must own active policy version, hard invariants, current authority/resource checks and final enforcement.

S.P.A.R.K. implication: preserve as secondary evaluator research. Do not activate in the current acquisition lane because it reaches directly into a consumer's authorization semantics.

## 5. Cloudflare OS / Gatekeepers / Gadgets / Blueprints

Recovered patterns:
- resources are introduced narrowly rather than ambiently inherited;
- resource existence, connection, introduction, capability grant and side-effect authorization are distinct states;
- Gatekeepers combine external-service mediation, scoped capabilities, OAuth/credential handling, logging, approvals and simulation concepts;
- Gadgets treat generated applications as sandboxed machine-addressable objects;
- Blueprints separate reusable system templates from user-specific live resources/credentials;
- provider/resource bindings can be abstract rather than hard-wired;
- information-flow/observer controls matter when generated/shared applications touch differently scoped resources.

S.P.A.R.K. implication: retain as architecture/source research; Cloudflare-specific infrastructure is not a default dependency.

## 6. CLI-Anything

Completed verdict: strong tool-architecture reference, not a wholesale dependency.

Harvest lessons:
- probe before mutate;
- drive the real backend, not a toy reimplementation;
- explicit state/session models;
- JSON machine mode;
- tool teaching colocated as skill documentation;
- capability/provider discovery and preflight;
- preview bundles as a protocol;
- verify final artifacts rather than trusting exit code.

Caution: catalog quality is uneven and its package/installer boundary is not sufficient authority for a governed host.

## 7. OpenTelemetry research

Recovered principle: semantic/canonical truth must stay in the producer/owner; OTel is downstream observation only. Strong implementation evidence exists for optional, bounded, nonblocking export and for keeping SDK types out of core contracts.

S.P.A.R.K. implication: retain OTel/Collector as an interoperability/evidence-export option, not as mission state or authority truth.

## 8. Corporate agent/prompt/scaffolding research

Cross-company convergence:
- bounded assignments;
- explicit tool contracts;
- selective/scoped context loading;
- external durable work/session state;
- compaction/handoff packets;
- subagents/workers;
- deterministic workflow controls where sequencing need not consume model reasoning;
- permission/policy enforcement outside prompts;
- task/progress ledgers and composable termination as useful precedents.

S.P.A.R.K. implication: acquire mechanics, not vendor prompt prose. Prefer generated briefings from durable records and narrow tool interfaces.

## 9. Claude Code architecture evidence

Useful findings:
- independent context is not automatically epistemic independence;
- subagents/teams have different inheritance and return semantics;
- permissions are harness-enforced rather than message-authorized;
- shared tasks, dependencies, claiming and mailboxes are concrete coordination mechanisms.

S.P.A.R.K. implication: use these as qualification criteria when designing multi-agent tooling; do not assume a product's collaboration semantics should become S.P.A.R.K.'s own.

## 10. G.A.M.E. Foreman/worker research

Durable finding: invest first in bounded inspection, reproduction, test selection and evidence retrieval. Those deterministic tools remain valuable even if a Foreman/multi-agent architecture is rejected. Useful external precedents include SWE-agent interface design, Anthropic parallel research, Temporal activity semantics, Erlang supervision and AutoGen/Magentic-One, but no single framework proved the desired G.A.M.E. topology.

S.P.A.R.K. implication: the acquisition program should provide reusable development tools to G.A.M.E. without forcing G.A.M.E. to adopt S.P.A.R.K.'s coordinator design.

## 11. External application control

Current reference harness targets with direct project value:
- Blender — 3D generation/render inspection;
- GIMP/Krita/Inkscape — 2D asset manipulation;
- Godot — game project/headless engine operations;
- LLDB — native debugger and DAP workflows;
- RenderDoc — GPU capture inspection;
- LibreOffice — document conversion/automation;
- QGIS — geospatial workflows;
- Ollama/ComfyUI and similar local services — structured local capability adapters.

S.P.A.R.K. implication: these are adapter targets, not architecture owners. Preserve them as downstream consumers of a future tool contract.

## 12. Orchestration frameworks

LangGraph, Temporal, n8n, Microsoft Agent Framework/AutoGen/Magentic-One, OpenHands and similar systems remain useful references for durable workflows, graphs, retries, UI/runtime separation and multi-agent coordination. They are secondary until a concrete S.P.A.R.K. gap requires them; importing an orchestration platform early would risk building a second system around the system.
