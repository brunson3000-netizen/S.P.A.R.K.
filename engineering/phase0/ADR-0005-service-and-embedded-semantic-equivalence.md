# ADR-0005 — Service and Embedded Semantic Equivalence

**Status:** REVISED AFTER SECOND INDEPENDENT PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN; transport implementation PROVISIONAL

## Context

S.P.A.R.K. must work as a standalone service and as an embeddable runtime with equivalent logical semantics.

The Phase-0 review process established that equivalence cannot begin only after an accepted stream mysteriously exists. Service and embedded forms must share the same logical construction of that stream: exclusive sequencer authority, staging, explicit digest fences, contiguous finalization, and canonical command barriers.

## Decision

1. Canonical causal evaluation lives in reusable Rust engine crates, never in service transport.
2. Service and embedded entry points implement the same logical timeline-ingress contract from ADR-0003:
   - one active sequencer per profile timeline epoch;
   - canonical envelopes;
   - isolated staging;
   - collision/gap validation;
   - sequencer-authored digest fence;
   - contiguous atomic finalization;
   - canonical command barriers.
3. A request becoming merely **staged** is not canonical admission.
4. Transport batching, arrival order, worker scheduling, subscriber pressure, connection identity, and session nonce cannot choose final command order.
5. Embedded integrations may bypass serialization and may offer a stage+fence convenience call, but they may not bypass sequencer/finality semantics.
6. Capability negotiation/profile selection exist in both logical modes.
7. Read-only queries remain outside canonical timeline history unless a later ADR deliberately defines an authoritative query-side effect.
8. Canonical validation/canonicalization occurs at the shared engine boundary.

## Equivalence criterion

Given the same:

```text
supported core build / semantics version
root seed
starting stable snapshot hash
exact profile manifest content hash
exact configuration revision hash
behavior epoch
timeline epoch
active sequencer identity/grant
same effective trust-domain/capability context
same set of staged canonical envelopes/payloads
same valid ordered TimelineFence chain
```

service and embedded forms must:

- finalize the same canonical command set in the same ordinal order;
- accept/reject the same conflicting/gapped/fenced inputs at the logical boundary;
- produce the same canonical state/output hashes.

Delivery order of the staged envelopes is explicitly **not** part of this tuple.

## Transport/admission distinction

Transport authentication, coarse rate limiting, malformed framing, and resource availability may reject a request before it reaches logical staging.

For conformance fixtures, once the same logically valid requests are presented inside declared resource limits, service and embedded paths must apply the same sequencer/staging/fence rules.

A transport may not substitute "first request received" for timeline finality.

## Canonical representation

- canonical integer/fixed-point values have one validated normalized logical representation;
- semantically equivalent accepted wire representations normalize to the same canonical payload hash;
- ambiguous numeric values reject.

## Trust-domain routing

- MCI output allowlists cannot route to real-work adapters;
- cross-profile subscriptions/references reject unless explicitly defined as a safe bridge;
- each profile has its own timeline epoch/sequencer authority;
- inspection dry-run/staging uses isolated snapshots/config candidates and has no production timeline-finalization or commit route.

## Consequences

- Concurrent HTTP delivery and embedded call ordering cannot diverge the canonical timeline.
- The accepted/finalized stream itself is now part of the equivalence contract.
- Multiple upstream data sources may exist, but one profile sequencer explicitly orders them before canonical finality.

## Required verification

1. same envelopes delivered `(n+1,n)` and `(n,n+1)` plus the same fence -> identical finalized stream/hashes;
2. conflicting same-ordinal envelopes in opposite delivery orders -> same fence rejection/no partial finalization;
3. gap/fence tests match across service and embedded forms;
4. non-sequencer staging/fence attempts reject in both forms;
5. different transport batch partitioning/session IDs/subscriber counts preserve the same logical result;
6. embedded stage+fence convenience API matches explicit service staging/fencing;
7. capability/config/manifest/timeline-epoch mismatch is detected;
8. semantically equivalent numeric serialization normalizes equally; ambiguous values reject;
9. MCI routing and inspection-isolation negative tests remain passing.
