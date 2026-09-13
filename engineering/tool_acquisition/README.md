# S.P.A.R.K. External Tool Acquisition Program

Status: `HIGH_PRIORITY_HARVESTED__SECONDARY_WAVE1_PINNED__QUALIFICATION_NEXT`

## Mission

S.P.A.R.K. owns the research, acquisition, qualification, and reusable engineering-tool **preparation lane**. It may collect external source, isolate reusable patterns, build qualification plans, and later create S.P.A.R.K.-owned tool services when authorized.

S.P.A.R.K. does **not** own S.W.A.R.M. architecture, authority, governance, policy, adoption decisions, credentials, or canonical state. S.W.A.R.M. is only a potential future consumer. Nothing in this directory grants permission to edit, install into, configure, or govern S.W.A.R.M.

S.P.A.R.K. is also the consolidation destination for useful NIM Engineering Tools work as that responsibility transitions away from the NIM-specific project.

## Acquisition doctrine

1. Acquire before rebuilding when a mature external tool already solves the deterministic problem.
2. Pin source before studying or qualifying it.
3. Keep source acquisition separate from dependency introduction.
4. Preserve upstream license/provenance and review reuse terms before copying code into S.P.A.R.K. implementation.
5. Prefer narrow deterministic interfaces over raw shell/desktop authority.
6. Treat model output as evidence or proposal, never as deterministic truth merely because a tool produced it.
7. Keep transport, sandbox, policy evaluator, registry, agent harness, and application adapter roles separate.
8. An external tool's metadata, permissions, identity, or workflow state never becomes another project's authority by import.
9. Every candidate must have a disposition: direct reuse, wrapper/adaptor, pattern borrowing, reference only, or reject.
10. Harvesting means research/code custody and qualification preparation—not automatic activation.

## Current acquisition state

### High-priority set

The 13 high-priority upstreams are pinned under `external/harvest/high_priority/` and locked by `UPSTREAM_LOCK.json`. These are intended for immediate local source custody and qualification.

### Secondary Wave 1

Twenty-three secondary upstreams are pinned under `external/harvest/secondary/` and locked by `SECONDARY_HARVEST_LOCK.json`.

Secondary gitlinks intentionally use `update = none` and `shallow = true`. Normal recursive submodule synchronization therefore does not materialize the entire secondary catalog. Use `scripts/harvest_secondary_source.sh` to pull one selected source at its exact pin.

Source pinning is complete for these Wave 1 lanes:

- agent/client and tool-fabric protocols;
- structured command/plugin models;
- durable workflow and agent-runtime references;
- policy/evaluator comparisons;
- corporate/open-source agent-product mechanics;
- evidence, signing, and provenance references.

AutoGen and SLSA Verifier are retained as maintenance/reference sources rather than preferred new foundations.

### Demand-gated application sources

Very large external-application source trees remain demand-gated until a concrete adapter qualification requires them: LLVM/LLDB, RenderDoc, Godot, LibreOffice, QGIS, GIMP/Krita/Inkscape, and Ollama/ComfyUI.

The Rust Tool Contract v0 + Blender Shadow Adapter experiment remains pending implementation. Harvest activation did not authorize it.

## Immediate lanes

- Tool protocol and contract fabric
- Tool discovery / registry / teaching
- Containment and capability enforcement
- Deterministic repository introspection and context compression
- Test acceleration and falsification
- Provider/network fault injection and replay
- Evidence / provenance / observability
- Agent-harness and coordinator scaffolding references
- External-application adapters useful to G.A.M.E. and other projects

## Current next work

Qualification, not integration. Produce bounded qualification packets and capability mappings before any harvested source enters a runtime, dependency manifest, installation, or consumer architecture.

See:

- `MISSION_PLAN_V1.md`
- `SECONDARY_HARVEST_PROTOCOL.md`
- `UPSTREAM_LOCK.json`
- `SECONDARY_HARVEST_LOCK.json`
- `pending_actions/`
