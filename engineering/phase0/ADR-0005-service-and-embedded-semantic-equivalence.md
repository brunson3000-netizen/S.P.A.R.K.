# ADR-0005 — Service and Embedded Semantic Equivalence

**Status:** ACCEPTED FOR PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN; transport implementation PROVISIONAL

## Context

S.P.A.R.K. must work as a standalone service and as an embeddable runtime. The blueprint requires both forms to expose equivalent logical semantics through one versioned integration contract.

## Decision

1. Canonical causal evaluation lives in reusable Rust engine crates, never in the service transport layer.
2. Service and embedded entry points translate into the same internal command/query model.
3. The public protocol is defined independently of internal Rust structs and host internals.
4. Serialization is not itself semantic authority; canonicalization happens at the engine boundary.
5. The first service transport may use local HTTP/JSON plus a push channel, but this is replaceable without changing engine semantics.
6. Embedded integrations may bypass serialization for performance but must obey the same validation, authority, sequencing, profile, and acknowledgement semantics.
7. Capability negotiation and profile selection exist in both logical modes even if an embedded host can establish them in-process.

## Equivalence criterion

Given the same:

```text
supported build
profile/manifest versions
root seed / behavior epoch
starting snapshot
ordered logical host inputs
```

service and embedded paths must produce the same canonical output/state hashes.

## Consequences

- Performance-sensitive games are not forced through IPC.
- MCI/browser/CLI tools can use a local service without forking behavior semantics.
- Any adapter that needs a special semantic exception becomes an architecture review item rather than an incidental patch.

## Verification

- shared fixture corpus executed through both paths;
- canonical output/state hash equality;
- malformed-input behavior parity;
- capability/profile negotiation parity.
