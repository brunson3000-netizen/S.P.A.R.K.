# Pattern Card — Discovery Is Advisory

PATTERN: Discovery / Execution Separation
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- ARD spec `ards-project/ard-spec` @ `b76f235a8f461876ad4f1e77abd0eb0eb302b48d`
- Contrast: progressive MCP Guardian @ `4c6a04537b9bc548744b4146168b8fa9896069cb`

## Problem

A system that uses the same mechanism to discover a capability and authorize its execution risks turning visibility into authority.

## Mechanism

Separate lifecycle stages:
1. discover a candidate capability/resource
2. retrieve/learn its descriptor/schema
3. independently evaluate identity, current grant and hard policy
4. execute only after call-time authorization

Discovery results may contain relevance/trust metadata, but none of it is itself an execution grant.

## Dependencies

- stable capability identity
- catalog/search service
- descriptor/schema retrieval
- independent authority/grant evaluator
- executor/broker

## Benefits

- prevents catalog membership from becoming ambient authority
- permits broad search with narrow execution rights
- supports federated/untrusted discovery sources
- separates ranking errors from security decisions

## Risks / failure modes

- accidental shortcut where result score/scope/index membership is reused as authorization
- stale discovery metadata
- descriptor substitution between search and call
- authority drift after learning but before execution

## Boundaries crossed

Discovery ↔ authority: MUST be explicit and fail closed.
Descriptor ↔ execution: stable ID/version binding required.
Trust metadata ↔ policy: evidence input only; host policy owns decision.

## Agent-facing surface

Agent may discover and learn broadly. It should not possess or infer that a successful discovery means execution is allowed.

## Determinization relevance

VERY HIGH. Identity resolution, current grants, hard invariants, schema validation and admission should be deterministic host machinery.

## Likely architectural location

Capability Catalog → Authority Broker boundary.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariant

`find_capability` and `learn_capability` can never mint, widen or imply an execution grant. `execute_capability` must reauthorize against current host state every call.
