# Secondary Harvest Protocol — Wave 1

Status: `SOURCE_PINNED__AUTO_CHECKOUT_DISABLED__ACTIVATION_NONE`

## Purpose

Acquire reproducible custody of secondary external-tool source without turning source acquisition into runtime adoption, dependency introduction, installation, or consumer authority.

Wave 1 pins 23 upstream repositories under `external/harvest/secondary/`. Exact revisions are recorded in `SECONDARY_HARVEST_LOCK.json`.

## Safety model

Secondary repositories are git submodules with both:

- `update = none`
- `shallow = true`

The first setting is deliberate. A normal `git submodule update --init --recursive` must not cause the entire secondary catalog to materialize locally. Several repositories are large, and source custody does not justify an uncontrolled disk/network cost.

The second setting recommends a shallow checkout when a secondary source is deliberately materialized. The authoritative evidence is the exact gitlink and lock-file commit, not whatever happens to be upstream HEAD later.

## State distinctions

For Wave 1:

- researched: yes;
- upstream repository identified: yes;
- exact revision pinned: yes;
- online SPARK custody record: yes;
- locally downloaded: only when explicitly selected through the harvest helper;
- built: no;
- installed: no;
- activated: no;
- adopted as a SPARK dependency: no;
- adopted by any consumer project: no.

## Selective local harvest

Use:

```bash
engineering/tool_acquisition/scripts/harvest_secondary_source.sh list
engineering/tool_acquisition/scripts/harvest_secondary_source.sh <source-id>
```

The helper reads `SECONDARY_HARVEST_LOCK.json`, overrides `update = none` only for the selected submodule invocation, checks out the recorded pin, and fails unless the resulting HEAD exactly matches the lock.

It does not build, execute, install, add dependencies, modify Cargo manifests, or authorize a consuming project to use the source.

## Wave 1 lanes

### Tool fabric and interface

ACP specification, ACP Rust SDK, wasmCloud, Nushell, and OpenTelemetry Collector.

### Durable orchestration and agent runtime

Temporal, OpenHands SDK/Agent Server, LangGraph, n8n, Microsoft Agent Framework, and AutoGen/Magentic-One historical material.

AutoGen is retained because its architecture remains useful evidence. Its upstream currently identifies it as maintenance-mode and directs new development toward Microsoft Agent Framework, so it is not a preferred new dependency candidate.

### Policy and resource mediation

Cedar, OPA, and Cloudflare Agents. Evaluator code and policy languages are subordinate mechanisms; they do not become SPARK or consumer authority by import.

### Agent-facing product mechanics

Claude Code, Gemini CLI, Google ADK, OpenAI Codex, OpenAI Agents SDK, and SWE-agent.

These are source mines for patterns such as sandbox/approval boundaries, task state, subagents, handoffs, scoped permissions, compaction, workspace trust, and narrow agent-computer interfaces. They are not templates to copy wholesale.

### Evidence and provenance

Cosign, in-toto, and SLSA Verifier. `slsa-verifier` is retained as a reference snapshot because upstream now marks it in maintenance status.

## Deferred heavyweight application-source wave

The secondary activation does not silently clone very large application repositories merely because adapters are interesting. The following remain `PENDING_DEMAND` until an adapter qualification actually requires their upstream source:

- LLVM/LLDB;
- RenderDoc;
- Godot;
- LibreOffice;
- QGIS;
- GIMP/Krita/Inkscape;
- Ollama/ComfyUI.

The existing Rust Tool Contract v0 + Blender Shadow Adapter experiment remains `PENDING_IMPLEMENTATION`; source harvesting does not activate it.

## Qualification rule

Pinning is not endorsement. Before any candidate becomes an executable, library, sidecar, adapter, or copied-code source, produce a qualification packet covering:

1. exact capability needed;
2. exact source surface needed;
3. license/reuse conditions;
4. build/runtime/dependency cost;
5. attack and shell-escape surface;
6. deterministic qualification test;
7. audit/evidence path;
8. replacement/removal path;
9. recommended disposition: direct reuse, wrapper, pattern borrowing, reference only, or reject.

## Boundary

SPARK owns this research/acquisition/preparation lane. It does not thereby gain authority over SWARM, GAME, or any other consuming project. Consumer adoption remains a separate decision in that consumer's own authority domain.
