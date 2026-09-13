# Pattern Card — Filtered Discovery + Call-Time Reauthorization

PATTERN: Filtered Discovery + Call-Time Reauthorization
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- MCP Gateway & Registry @ `7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73`
- `registry/api/search_routes.py`
- `registry/auth/tool_filter.py`
- `registry/auth/access_resolver.py`
- `auth_server/server.py`
- `tests/integration/test_tool_level_access.py`
- `tests/auth_server/unit/test_server.py`
- Negative contrast: progressive MCP Guardian index-membership admission.

## Problem

Broad discovery and minimal agent context are useful, but a capability can become unauthorized between discovery and invocation. Reusing catalog visibility as execution permission creates stale/ambient authority.

## Mechanism

1. Authenticate discovery caller.
2. Prune search/list results to what the caller may currently see.
3. Let the agent select/learn a capability.
4. On each concrete execution, parse the actual operation and target.
5. Re-read/recompute current authority.
6. Validate method + specific capability/tool.
7. Deny if the operation cannot be inspected or authority cannot be established.
8. Only then route to the executor.

## Dependencies

- stable capability identity
- authenticated caller identity
- visibility policy
- current authority/grant store
- typed call envelope / inspectable operation
- fail-closed policy engine

## Benefits

- discovery can be broad without becoming ambient authority
- revocation can take effect between search and call
- stale catalog entries cannot authorize execution
- hidden tools stay out of context while execution remains independently governed

## Risks / failure modes

- policy-resolution drift between discovery service and executor
- inconsistent resource identity normalization across hops
- request body/operation not inspectable
- caching authority too aggressively
- compatibility fallbacks accidentally merge policy namespaces

## Boundaries crossed

Agent ↔ catalog: visibility decision.
Catalog ↔ learner: descriptor only.
Learner ↔ execution: no authority transfer.
Execution ingress ↔ authority broker: call-time target + method + capability check.
Authority broker ↔ adapter: admitted call only.

## Agent-facing surface

The agent does not need to understand the two policy stages. It receives discoverable candidates and a deterministic denial if current execution authority does not permit the selected action.

## Determinization relevance

VERY HIGH. Visibility, revocation, identity binding, method/tool validation and admission are deterministic host responsibilities.

## Likely architectural location

Capability Catalog + host-owned Authority Broker.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. Discovery/learning never mint a grant.
2. Every execution checks current authority.
3. Authority lookup failure denies.
4. Uninspectable privileged call denies.
5. Visibility and execution policy may share source data but must be separate decisions.
6. Identity normalization is identical across discovery and execution or bound by stable ID.
