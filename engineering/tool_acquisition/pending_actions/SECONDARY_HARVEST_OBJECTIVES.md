# Secondary Harvest Objectives — Wave 1 Activated

Status: `WAVE1_SOURCE_PINNED__QUALIFICATION_PENDING`

The Operator activated the secondary harvest protocol on 2026-09-13. Activation authorizes reproducible source acquisition and consolidation only. It does not authorize build, installation, runtime activation, dependency introduction, or consumer adoption.

## Tool fabric / interface — SOURCE PINNED

- Agent Client Protocol specification and Rust SDK.
- wasmCloud.
- Nushell.
- OpenTelemetry Collector.

Next state: qualification against SPARK capability contracts.

## Durable orchestration / agent runtime — SOURCE PINNED

- Temporal.
- OpenHands software-agent SDK / Agent Server.
- LangGraph.
- n8n.
- Microsoft Agent Framework.
- AutoGen / Magentic-One historical material.

AutoGen is upstream maintenance-mode and is retained as architecture evidence; Microsoft Agent Framework is the current Microsoft implementation reference.

## Authorization / resource mediation — SOURCE PINNED

- Cedar.
- Open Policy Agent / Rego comparison.
- Cloudflare Agents for Gatekeeper/Gadgets/sub-agent isolation/information-flow patterns.

No evaluator or imported framework becomes SPARK or consumer authority merely because it is harvested.

## Agent-facing product mechanics — SOURCE PINNED

- Claude Code.
- Gemini CLI.
- Google ADK Python.
- OpenAI Codex.
- OpenAI Agents Python SDK.
- SWE-agent.

These are pattern/code research sources for permissions, sandboxing, context management, task state, subagents, handoffs, tool surfaces, and agent-computer interfaces.

## Evidence / provenance — SOURCE PINNED

- Sigstore Cosign.
- in-toto.
- SLSA Verifier.

SLSA Verifier is upstream maintenance-status reference material. Adoption of a supply-chain toolchain remains conditional on SPARK actually distributing or compiling acquired tools.

## External application adapters — PENDING_DEMAND / PENDING_IMPLEMENTATION

The activation does not justify indiscriminate cloning of heavyweight application repositories. Code-level source harvest remains demand-gated for:

- LLVM/LLDB;
- RenderDoc;
- Godot;
- LibreOffice;
- QGIS;
- GIMP/Krita/Inkscape;
- Ollama/ComfyUI.

The Rust Tool Contract v0 + Blender Shadow Adapter experiment remains `PENDING_IMPLEMENTATION`. Its existing experiment record remains authoritative until separately activated.

## Storage / repeated-harvest optimization — PENDING_THRESHOLD

A local content-addressed upstream snapshot store remains pending until repeated source checkout cost demonstrates that the added mechanism is justified.

## Current next action

Perform H2 qualification and H3 capability mapping. Pull secondary source trees locally only when a specific qualification requires them; use `scripts/harvest_secondary_source.sh` so exact pins remain enforced.
