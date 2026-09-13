# Pattern Card — Ordered Call-Time Permission Pipeline

PATTERN: Ordered Call-Time Permission Pipeline
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- xAI grok-build @ `37949780c144e37df692e3d669051a21fec24f20`
- `xai-grok-workspace/src/permission/manager/mod.rs`
- permission policy/preflight/resolution modules
- `xai-grok-pager/docs/user-guide/22-permissions-and-safety.md`
- contrast: MCP Guardian’s discovery-index admission
- convergence: MCP Gateway’s call-time method/tool authorization

## Problem

Automation modes such as auto-approve are useful, but they become dangerous if they bypass hard policy or if the model itself effectively owns the approval state.

## Mechanism

Evaluate a concrete proposed action through ordered host-owned gates:
1. deny-capable pre-tool hooks / immutable hard constraints
2. current merged policy rules (`deny > ask > allow`)
3. scoped remembered decisions where applicable
4. deterministic built-in read-only/safe classifications
5. mode-specific prompt/auto policy for what remains
6. execute only after host admission.

Broad automation is therefore a lower-priority convenience policy, not a superuser grant.

## Dependencies

- typed action/tool identity
- current subject/session/project identity
- deterministic policy evaluator
- precedence rules
- optional classifier/heuristics for convenience only
- current call envelope
- audit/decision reason

## Benefits

- hard deny survives “always approve” modes
- user/admin/project policy has explicit precedence
- revocation/current-state checks happen on the concrete call
- model reasoning can suggest risk but cannot mint authority
- decision reasons can be audited consistently.

## Risks / failure modes

- compatibility fallbacks blur namespaces
- policy sources with unclear precedence become escalation paths
- classifier treated as authority rather than convenience
- overly broad remembered prefix grants
- model-provided tool name/arguments not normalized before matching
- policy snapshots become stale across long sessions.

## Boundaries crossed

Model → host: proposal only.
Host policy → executor: admitted concrete action.
Managed/admin policy → local/user policy: precedence must be explicit and non-bypassable.
Remembered decision → current call: scoped exactness/normalization required.

## Agent-facing surface

The agent sees a deterministic allow, denial, or user-approval requirement. It does not need the internal policy stack in prompt context.

## Determinization relevance

CRITICAL. This is authoritative runtime machinery and belongs wholly outside model judgment.

## Likely architectural location

Rust Authority Broker / Tool Fabric admission path.

## Finding class

PATTERN + ALGORITHM + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. Deny/hard invariants cannot be overridden by auto/always-approve/model output.
2. Every executable call is re-evaluated against current authority.
3. Decision order is explicit and test-pinned.
4. Policy-evaluation failure denies.
5. Classifiers/LLMs may force caution or suggest a decision but cannot widen hard authority.
6. Remembered grants/denies are scoped to stable subject/resource/action identity.
7. Decision evidence records which gate allowed/denied the call.
