# External Tool Acquisition Mission Plan V1

## Objective

Maintain one SPARK-owned, reproducible evidence base for external tools and code that can improve agent tooling, deterministic engineering, containment, observability, orchestration, and external-application adapters—without silently introducing production dependencies or assuming authority over consuming projects.

## H0 — Cross-project consolidation — COMPLETE

The useful external-tool research recovered from NIM, SWARM, GAME, and SPARK work is normalized under `engineering/tool_acquisition/`. Consumer-specific authority assumptions are treated as boundary constraints, not SPARK decisions.

## H1A — High-priority harvest — COMPLETE

Thirteen high-priority sources are pinned under `external/harvest/high_priority/` with exact revisions in `UPSTREAM_LOCK.json`.

Focus:

- MCP/tool contracts and discovery;
- isolated tool runtimes and containment;
- deterministic repository introspection;
- fast tests and mutation testing;
- provider/network mocking and fault injection.

## H1B — Secondary Wave 1 harvest — COMPLETE: SOURCE PINNING

Twenty-three secondary sources are pinned under `external/harvest/secondary/` with exact revisions in `SECONDARY_HARVEST_LOCK.json`.

Secondary source pinning covers:

1. ACP specification and Rust SDK;
2. wasmCloud, Nushell, and OpenTelemetry Collector;
3. Temporal, OpenHands SDK, LangGraph, n8n;
4. Microsoft Agent Framework and AutoGen/Magentic-One historical material;
5. Cedar, OPA, and Cloudflare Agents;
6. Claude Code, Gemini CLI, Google ADK, OpenAI Codex, OpenAI Agents SDK, SWE-agent;
7. Cosign, in-toto, and SLSA verification reference material.

These sources intentionally use `update = none`; they are selectively materialized with `scripts/harvest_secondary_source.sh` rather than all being downloaded during a normal recursive sync.

AutoGen and SLSA Verifier are maintenance/reference sources, not preferred new foundations.

## H1C — Heavy application-source harvest — PENDING_DEMAND

Do not clone heavyweight upstreams merely because adapter work may someday use them. Activate a source only when its adapter qualification needs code-level inspection.

Demand-gated set:

- LLVM/LLDB;
- RenderDoc;
- Godot;
- LibreOffice;
- QGIS;
- GIMP/Krita/Inkscape;
- Ollama/ComfyUI.

The Blender Rust Tool Contract v0 + Shadow Adapter experiment remains separately `PENDING_IMPLEMENTATION`.

## H2 — Qualification packets — NEXT

Build short qualification packets. High-priority candidates are first; secondary candidates are pulled forward when they compete for the same capability or materially inform the contract.

Every packet must state:

- capability/problem;
- exact upstream surface needed;
- license and reuse conditions;
- build/runtime/dependency cost;
- attack/failure surface;
- deterministic qualification test;
- evidence/audit path;
- token/context reduction opportunity;
- integration shape;
- replacement/removal path;
- disposition: `DIRECT_REUSE`, `WRAP`, `BORROW_PATTERN`, `REFERENCE_ONLY`, or `REJECT`.

H2 does not authorize runtime adoption.

## H3 — Capability map

Normalize candidates by capability, not product name. Current capability families include:

- `tool.protocol.mcp`
- `tool.protocol.acp`
- `tool.discovery.registry`
- `tool.teaching.skill`
- `tool.runtime.isolated`
- `tool.security.inspect`
- `tool.policy.evaluate`
- `repo.syntax.query`
- `repo.symbol.slice`
- `test.execute.fast`
- `test.falsify.mutation`
- `provider.http.mock`
- `provider.network.fault`
- `workflow.durable`
- `agent.supervise`
- `agent.session.transport`
- `artifact.preview.bundle`
- `evidence.trace.export`
- `artifact.provenance.verify`
- `external_app.adapter`

A product may provide several capabilities; multiple products may compete for one capability. Selection occurs at the capability layer.

## H4 — Experiment and implementation selection

Only after qualification and capability comparison select bounded experiments or implementations.

Selection favors:

- frequent engineering value;
- reduction of silent wrongness;
- deterministic offload;
- Rust-first compatibility;
- least privilege;
- auditability;
- reversibility;
- small dependency/runtime cost;
- explicit consumer demand.

The first implementation should prove a reusable contract, not install the largest available platform.

## H5 — Provenance/distribution hardening — CONDITIONAL

If SPARK begins compiling, redistributing, or vendoring harvested code, activate provenance work around signing, attestations, source locks, and reproducible artifact verification. Source pinning alone does not require adopting an entire supply-chain framework.

## Hard boundary

SPARK may research, acquire, qualify, and prepare tools. It does not gain authority over SWARM, GAME, or another consumer. A consumer's adoption, permissions, credentials, canonical state, and governance remain owned by that consumer.
