# Pattern Card — Progressive Tool Disclosure

PATTERN: Progressive Tool Disclosure
STATUS: STRONG_CANDIDATE
CONFIDENCE: MEDIUM-HIGH pending independent SPARK experiment

## Source evidence
- Project: progressive MCP Guardian
- Upstream: `S1LV3RJ1NX/mcp-guardian`
- Commit: `4c6a04537b9bc548744b4146168b8fa9896069cb`
- Primary modules: `src/mcp_guardian/proxy.py`, `index.py`, `search/keyword.py`, `upstream.py`
- Tests: `tests/test_proxy.py`, `tests/test_index.py`
- Benchmark: `benchmarks/experiment-5-security/bench_security.py` plus token/search/scaling experiments

## Problem

Large tool catalogs consume context before work begins and make tool choice harder.

## Mechanism

Expose a tiny stable agent surface:
1. search compact capability metadata
2. retrieve one exact schema on demand
3. execute the selected capability

Keep full schemas and routing metadata outside the model context until requested.

## Dependencies

- capability/tool catalog
- compact searchable metadata
- exact schema store
- resolver from capability identity to executor
- independent execution-authority check (required by SPARK even though the reference project couples this to index membership)

## Benefits

- much smaller startup context
- large tool universe becomes practical
- deterministic catalog filtering/ranking possible
- agent can learn only the capability it needs
- tool teaching and tool execution can be separated

## Risks / failure modes

- bad search ranking hides the right capability
- extra round trips add latency
- opaque discovery can make capabilities effectively unreachable
- search results themselves can grow without explicit limits
- dangerous authority coupling if “discoverable” is treated as “authorized”
- stale catalog/schema risk

## Boundaries crossed

Agent ↔ catalog: typed meta-tools; compact results should be byte/count bounded.
Catalog ↔ executor: must preserve stable capability identity.
Discovery ↔ authority: must be separate; discovery is advisory only.
Executor ↔ tool: call-time validation and current grant check required.

## Agent-facing surface

Reference shape:
- `find_capability(query)`
- `learn_capability(id)`
- `execute_capability(id, input)`
- result/evidence reference

The exact names are not adopted; the shape is under experiment.

## Determinization relevance

HIGH. Search/filter/schema lookup/admission prechecks/routing metadata are deterministic middleware candidates.

## Likely architectural location

Tool Fabric / Capability Catalog / Resource Discovery layer between agent and executable adapters.

## Finding class

PATTERN + TEST/INVARIANT + ALGORITHM

## Primary disposition

EXPERIMENT_NOW

## Justification

Independent ecosystem convergence and a clean reference implementation support the pattern, but SPARK must prove task success and enforce independent call-time authority before choosing an implementation.

## Required invariants for a SPARK experiment

1. Catalog visibility never grants execution authority.
2. Hidden/unauthorized capability cannot be learned or executed through alternate naming.
3. Search output has hard item/byte bounds.
4. Exact schema retrieval is attributable to a stable capability ID/version.
5. Execution revalidates identity, current grant, schema/input and hard invariants.
6. Catalog/search failure cannot silently broaden capability access.
7. Full evidence remains observable outside the agent context.
