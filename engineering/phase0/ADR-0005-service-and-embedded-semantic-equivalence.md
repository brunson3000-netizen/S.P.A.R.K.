# ADR-0005 — Service and Embedded Semantic Equivalence

**Status:** REVISED AFTER INDEPENDENT PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN; transport implementation PROVISIONAL

## Context

S.P.A.R.K. must work as a standalone service and as an embeddable runtime. Both forms expose equivalent logical semantics through one versioned integration contract.

The independent Phase-0 review found that the prior equivalence tuple omitted canonical input framing, capability/config context, and several transport/session influences that must be explicitly semantic or explicitly inert.

## Decision

1. Canonical causal evaluation lives in reusable Rust engine crates, never in service transport.
2. Service and embedded entry points normalize into the same `CanonicalCommandEnvelope` and query model.
3. The public protocol is defined independently of internal Rust structs and host internals.
4. Canonical validation/canonicalization occurs at the engine boundary in both modes.
5. Embedded integrations may bypass serialization for performance but may not bypass canonical command validation, authority, ordering, profile, configuration, acknowledgement, or trust-domain rules.
6. Capability negotiation and profile selection exist in both logical modes.
7. Transport batches are packaging only and cannot create semantic barriers.
8. Subscriber presence/backpressure, request arrival order, network connection identity, worker identity, and session nonce are causally inert unless a future ADR deliberately promotes a field into canonical input.

## Equivalence criterion

Given the same:

```text
supported core build / semantics version
root seed
starting stable snapshot hash
exact profile manifest content hash
exact configuration revision hash
behavior epoch
same effective trust-domain/capability set for the accepted operations
same ordered CanonicalCommandEnvelope stream
```

service and embedded paths must produce the same canonical state/output hashes.

The command stream includes effective logical time and canonical `input_ordinal`. A transport/session identifier is not a substitute for a logical `source_id`.

## Acceptance boundary versus canonical equivalence

Transport authentication, rate limiting, malformed-input rejection, and resource admission may determine whether a request becomes an accepted canonical command. Once the same command has been accepted in both modes, transport details cannot change its causal result.

Tests must separately verify parity of security/validation decisions where both modes expose the relevant boundary.

## Canonical representation

- canonical integer/fixed-point values have one validated logical representation;
- semantically equivalent accepted wire representations normalize to the same canonical payload hash;
- ambiguous numeric representations are rejected rather than allowed to create platform/serializer-dependent behavior.

## Trust-domain routing

Profile/output routing is part of the integration contract:

- MCI output allowlists cannot route to real-work adapters;
- cross-profile subscriptions/references are rejected unless explicitly defined as a safe bridge;
- inspection dry-run/staging operates on isolated snapshots and has no production commit route.

## Consequences

- Performance-sensitive games are not forced through IPC.
- Concurrent HTTP requests cannot become an accidental ordering protocol.
- Session/subscriber behavior cannot perturb simulation truth.
- Service and embedded fixtures compare complete semantic context rather than only seed/profile labels.

## Verification

- same canonical command stream through service and embedded paths with different transport batch partitioning;
- concurrent service delivery versus deterministic embedded delivery;
- different session IDs/subscriber counts/backpressure with identical accepted commands;
- capability/config/manifest mismatch is detected rather than silently compared;
- semantically equivalent numeric serialization normalizes equally; ambiguous values reject;
- malformed-input/security decision parity where applicable;
- MCI routing and inspection-isolation negative tests.
