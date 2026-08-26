# ADR-0005 — Service and Embedded Semantic Equivalence

**Status:** REVISED AFTER THIRD INDEPENDENT PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN; transport implementation PROVISIONAL

## Context

S.P.A.R.K. must expose equivalent logical semantics as a standalone service and an embeddable runtime.

The final Phase-0 ingress correction requires not only shared sequencing/fencing, but shared **ordinal credit-window admission** so bounded staging capacity cannot behave differently under reversed delivery.

## Decision

Both service and embedded forms implement the same canonical timeline-ingress contract:

1. one active sequencer per profile timeline epoch;
2. deterministic admission window derived from finalized frontier;
3. one reserved logical staging slot per eligible ordinal;
4. token-bound admission;
5. out-of-window retry result with no eviction;
6. positive STAGED acknowledgement;
7. sequencer fence only after acknowledgements for the full range;
8. contiguous digest/hash-linked finalization;
9. individual canonical command barriers after finalization.

Transport batching, request arrival order, workers, subscribers, connection/session identities, and ordinary pre-staging transport queues are not canonical.

## Equivalence criterion

Given the same:

```text
supported core semantics/build
root seed
starting stable snapshot hash
profile manifest content hash
configuration revision hash
behavior epoch
timeline epoch
active sequencer identity/grant
admission window width
starting finalized frontier/fence hash
effective trust-domain/capability context
same logical command envelopes with their admission-window tokens
same sequencer retries after window advancement
same fence chain
```

service and embedded forms must produce:

- the same logical staging status for every submission;
- the same poisoned/idempotent slot state;
- the same finalized canonical command stream;
- the same canonical state/output hashes.

## Reversed-delivery requirement

Within a fixed admission window, reversing command delivery order cannot change:

- which eligible ordinal slots are occupied;
- whether the frontier slot remains available;
- fence eligibility;
- canonical finalization.

An ordinal outside the tokenized window returns the same retryable result even if transport delay means it physically arrives after a later frontier advance.

## Stage/fence protocol

Service mode exposes the logical stages explicitly.

Embedded mode may expose an optimized convenience API, but it must internally enforce the same:

```text
window token
-> ordinal-slot staging
-> STAGED acknowledgement
-> fence precondition
-> finalization
```

It may not treat an in-memory call order as implicit finality.

## Transport resource exhaustion

Transport implementations may reject requests before logical staging because of local socket/request resource exhaustion.

Such rejection is not a canonical timeline decision.

Equivalence tests are run inside declared transport resource bounds so the same logical submissions reach the shared admission boundary.

Logical canonical staging itself uses the deterministic ordinal-window contract and has no first-arrival overflow policy.

## Trust-domain routing

Existing frozen isolation remains:

- MCI output allowlists cannot route to real-work adapters;
- cross-profile state/ID/subscription/acknowledgement paths reject unless explicitly bridged read-only;
- profiles have distinct timeline epochs/sequencer authority;
- inspection dry-run has no production finality/commit route.

## Required verification

1. capacity-two reversed-delivery fixture across service, embedded, and concurrent-worker paths;
2. old-token late arrival versus explicit retry under new token;
3. full eligible window with missing frontier still accepts frontier because its slot is reserved;
4. same-ordinal collision in opposite delivery order;
5. stage-ack/fence precondition parity;
6. partial-prefix finalization and deterministic window slide;
7. poisoned-state epoch-reset recovery parity;
8. transport queue is not reused as canonical staging;
9. existing session/subscriber/numeric-canonicalization and MCI/inspection isolation tests remain passing.
