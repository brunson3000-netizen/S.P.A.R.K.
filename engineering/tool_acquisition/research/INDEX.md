# Tool Architecture Research Index

Status: ACTIVE

This directory is the durable research notebook for external tool architecture.

## Function-based study

Start with [MCI research map](../MCI_RESEARCH_MAP.md), [Fable's brief](../FABLE_MCI_RESEARCH_BRIEF.md), and [governance packet](categories/GOV.md). Eight category indexes cross-reference the existing source/pattern/failure records. They do not change upstream custody or mark pending research complete.

## Structure

- `sources/` — what each project actually does at the pinned commit
- `patterns/` — cross-project reusable mechanisms
- `comparison/` — matrices across security, agent surface, licensing, architecture
- `failures/` — failure lessons and weak evidence
- `experiments/` — controlled tests and results
- `candidates/` — disposition buckets and implementation candidates
- `provenance/` — source-freeze records and pointers to canonical lock manifests
- `ACTIVE_TRACE_LEDGER.md` — current execution-path research state

## Canonical source custody

Source pins remain authoritative in:
- `../UPSTREAM_LOCK.json`
- `../SECONDARY_HARVEST_LOCK.json`
- `../MIDDLE_LAYER_HARVEST_LOCK.json`
- `../COMMUNITY_HARVEST_LOCK.json`

Pinned external source is quarantined under `external/harvest/`.

## Original discovery priority (historical ordering; see active ledger)

Governance-first comprehensive organization now follows the Operator's 2026-09-14 direction. Consult `ACTIVE_TRACE_LEDGER.md` for live trace ownership; its E2B trace remains active.

### Earlier sequence

P0 middle-layer qualification:
1. ARD specification — discovery contract
2. progressive MCP Guardian — tiny meta-tool surface
3. MCP Gateway & Registry — governed capability inventory/control plane
4. context-compress — compact view with retrievable evidence
5. Agent Skills — capability teaching/package format

Follow with agentgateway, observability references, E2B, OpenZiti, and ctx-zip comparison evidence.

## Research rule

Source-specific notes answer “what does X do?” Pattern notes answer “what have multiple independent projects taught us about problem Y?”
