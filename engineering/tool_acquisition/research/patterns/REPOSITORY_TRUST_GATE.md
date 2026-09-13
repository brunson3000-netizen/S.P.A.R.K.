# Pattern Card — Repository Trust Before Behavioral Loading

PATTERN: Repository Trust Gate Before Behavioral Loading
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- xAI grok-build @ `37949780c144e37df692e3d669051a21fec24f20`
- `xai-grok-shell/src/agent/folder_trust.rs`
- workspace folder-trust/trust-store implementation and tests
- convergence: Agent Skills warns that project skills may be untrusted

## Problem

A cloned repository can ship files that look like configuration/documentation but cause an agent to spawn servers, execute plugins, widen permissions, load instructions or activate skills. Treating repository presence as consent turns “open this repo” into behavior installation.

## Mechanism

Before loading project-scoped behavior:
1. scan for behavior-bearing repository configuration
2. identify the canonical workspace
3. resolve a per-workspace trust decision from feature policy + durable trust store + interactivity
4. permit or suppress project-scoped MCP/LSP/policy/instructions/skills accordingly
5. keep user/global/bundled sources separate.

A key refinement: “no behavior-bearing config exists right now” is a **provisional allow**, not durable trust. Do not cache it as permission for future repo content.

## Dependencies

- canonical workspace identity
- bounded config scan
- durable trust store
- explicit source/scope labels
- loaders that all consult one shared gate
- cache invalidation/reconciliation rules.

## Benefits

- blocks clone-time policy/instruction/tool injection
- one decision protects multiple loaders consistently
- avoids repeated permission prompts for already trusted workspaces
- preserves distinction between project content and user-owned configuration.

## Risks / failure modes

- loader bypasses shared gate
- a provisional “nothing dangerous found” result is cached as permanent trust
- folder trust accidentally implies plugin/credential/execution trust
- path canonicalization or aliasing changes workspace identity
- broad roots (`HOME`, filesystem root) become over-wide trust grants.

## Boundaries crossed

Repository → runtime configuration.
Repository → instruction/skill context.
Repository → executable server/plugin definitions.
Trust store → loader admission.

## Agent-facing surface

Ideally none until trust needs explicit user input. Untrusted project behavior simply remains unavailable rather than being advertised and denied repeatedly.

## Determinization relevance

VERY HIGH. Scanning, canonical identity, trust persistence and loader gating are deterministic host functions.

## Likely architectural location

Workspace bootstrap / capability acquisition boundary before the Tool Fabric materializes project-scoped resources.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. Repository presence is not trust.
2. All project-scoped behavior loaders share one trust gate.
3. “No risky config currently present” never becomes a durable future grant.
4. Folder trust does not imply unrelated plugin/credential/execution grants.
5. Canonical path identity and symlink/alias handling are test-pinned.
6. Untrusted project behavior stays invisible/unloaded, not merely marked with a warning.
