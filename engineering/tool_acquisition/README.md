# S.P.A.R.K. External Tool Acquisition Program

Status: `HIGH_PRIORITY_SOURCE_HARVESTED__QUALIFICATION_NEXT`

## Mission

S.P.A.R.K. owns the research, acquisition, qualification, and reusable engineering-tool **preparation lane**. It may collect external source, isolate reusable patterns, build qualification plans, and later create S.P.A.R.K.-owned tool services when authorized.

S.P.A.R.K. does **not** own S.W.A.R.M. architecture, authority, governance, policy, adoption decisions, credentials, or canonical state. S.W.A.R.M. is only a potential future consumer. Nothing in this directory grants permission to edit, install into, configure, or govern S.W.A.R.M.

S.P.A.R.K. is also the consolidation destination for the useful NIM Engineering Tools work as that responsibility transitions away from the NIM-specific project.

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
10. High-priority harvesting means research/code custody and qualification preparation—not automatic activation.

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

Secondary subjects remain in `pending_actions/` until separately activated.

## Current online state

The high-priority source set is pinned in `external/harvest/high_priority/` and locked by `UPSTREAM_LOCK.json`. This is source custody only; activation remains `NONE`.
