# Pattern Card — Stable Capability Identity

PATTERN: Stable Capability Identity Separate from Location and Credential
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- ARD spec `ards-project/ard-spec` @ `b76f235a8f461876ad4f1e77abd0eb0eb302b48d`
- Negative contrast: progressive MCP Guardian indexes entries by bare `tool.name` across multiple upstream servers.

## Problem

Bare human-readable names collide. Network locations change. Security principals rotate. Using any of those as the only capability identity makes federation, audit and authorization brittle.

## Mechanism

Assign each capability/resource a stable globally unique logical identifier. Keep separately:
- display name
- current network/adapter location
- version/schema identity
- publisher/provenance
- dynamic security principal/credential

ARD demonstrates this with domain-anchored `urn:air:` identifiers and separate `url`/`data` plus trust identity.

## Benefits

- collision-resistant catalog keys
- location can change without changing capability identity
- audit and evidence can bind to stable referent
- federation can merge catalogs safely
- authority can target a logical resource instead of a transient endpoint

## Risks / failure modes

- namespace squatting without publisher binding
- stale identifier → location mapping
- alias/display-name confusion
- version ambiguity if version is not bound during execution

## Boundaries crossed

Catalog ↔ resolver: stable ID resolves to current descriptor/adapter.
Resolver ↔ authority: grant names stable resource identity.
Authority ↔ execution: call binds stable ID + current version/location + principal.

## Determinization relevance

HIGH. Parsing, collision detection, alias resolution and binding checks should be deterministic.

## Likely architectural location

Capability Registry / Tool Fabric identity layer.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. Two upstream resources with identical display/tool names remain distinct.
2. Alias or location change cannot change granted authority.
3. Execution evidence records stable capability ID plus resolved version/location.
4. Discovery source cannot overwrite another publisher’s namespace without validated authority.
