# Pattern Card — Protocol Substrate Below Host Authority

PATTERN: Reuse Protocol Mechanics Without Delegating Authority
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- official Rust MCP SDK (`rmcp`) @ `3075dc9152d4678775f20634fcb467a7b995dbab`
- service/client/server lifecycle
- typed tool router/macros
- transports/session manager
- task extension + conformance tests
- convergence: ARD discovery separation, MCP Gateway call-time authorization, grok-build host authority.

## Problem

A protocol SDK can correctly implement transport, lifecycle, schemas and task/session semantics while remaining intentionally ignorant of application-specific identity, authority, cost, governance and containment. Treating protocol validity as business authorization collapses two different layers.

## Mechanism

Use the protocol SDK for:
- wire models
- version/capability negotiation
- transport/session correlation
- typed parsing/serialization
- cancellation/progress
- protocol task/subscription state
- conformance/interoperability.

Wrap every executable application operation in a host boundary that separately supplies:
- stable canonical resource/capability identity
- authenticated subject/context
- current grant/policy decision
- schema/hard-invariant validation as needed
- cost/governance checks
- sandbox/confinement
- canonical evidence.

Protocol IDs and declarations are inputs to that host layer, never authority themselves.

## Benefits

- avoids reimplementing a moving standard
- retains official ecosystem interoperability
- keeps security/authority semantics project-owned and stable
- permits replacing/upgrading transport SDK without migrating canonical authority model
- makes conformance evidence reusable without confusing it with acceptance evidence.

## Risks / failure modes

- host starts trusting remote tool names as canonical identity
- negotiated capability interpreted as permission
- MCP session/task/request ID treated as bearer credential
- adapter lets SDK cache return stale state for authority decisions
- transport restart silently changes host resource identity
- SDK lifecycle state is exposed as supervisor health truth.

## Boundaries crossed

MCP peer → protocol adapter: typed/untrusted protocol data.
Protocol adapter → SPARK broker: normalized canonical request.
SPARK broker → executable adapter: independently authorized operation.
Execution → MCP response: protocol serialization only after host evidence/result formation.

## Determinization relevance

CRITICAL. The adapter/broker split is deterministic infrastructure.

## Likely architectural location

MCP adapter beneath SPARK Tool Fabric/Authority Broker.

## Finding class

PATTERN + CODE + TEST/INVARIANT

## Primary disposition

REUSE_CODE for rmcp protocol substrate; BORROW_PATTERN for the boundary.

## Required invariants

1. Protocol discovery/capability/session/task state never mints grants.
2. Every side-effecting call crosses current host authorization.
3. Remote wire names map through a host canonical identity table.
4. Economic/governance authority remains outside MCP.
5. Trace/progress/session metadata is correlation-only.
6. Adapter health and MCP connection lifecycle do not become canonical system health by implication.
7. Protocol SDK upgrades rerun compatibility/conformance and host-boundary tests.
