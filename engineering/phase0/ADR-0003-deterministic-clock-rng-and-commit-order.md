# ADR-0003 — Canonical Input Transactions, Deterministic Clock, Random Addresses, and Commit Order

**Status:** REVISED AFTER INDEPENDENT PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN

## Context

S.P.A.R.K. requires deterministic replay across declared Windows/Linux/Android support builds. The host advances simulation time. Canonical behavior must not depend on service request arrival order, thread interleaving, transport batch partitioning, or the number of `advance_time` calls used to reach the same logical point.

The independent Phase-0 review found that the original commit-wave description did not define how concurrent or split host inputs become a canonical ordered stream.

## Decision

### Canonical command envelope

Every command that can affect canonical state or canonical evaluation is normalized at the engine boundary into a `CanonicalCommandEnvelope` containing at least:

```text
command_id             stable idempotency identity
profile_id             target profile/trust domain
effective_time         monotonic integer simulation time
source_id              stable logical source identity; not a transport session ID
source_sequence        monotonic sequence within that logical source
input_ordinal          unique total-order position in the profile timeline
command_kind
canonical_payload_hash
```

The active manifest/config/behavior epoch is resolved at the command barrier and recorded with the accepted command/result.

Transport-only metadata such as HTTP connection, WebSocket subscriber, session nonce, worker identity, receipt timestamp, and packet/batch boundaries is not canonical input.

### Canonical ordering

1. Accepted canonical commands are processed strictly by `(effective_time, input_ordinal)`.
2. `input_ordinal` provides the total order for commands sharing the same logical time.
3. `source_sequence` is independently validated so reordering, gaps, duplicates, and source replay can be detected according to protocol policy.
4. Arrival order is never an implicit tie-breaker.
5. Two different payloads claiming the same `command_id` or canonical ordinal are rejected.
6. Ambiguous commands without the required canonical ordering information are rejected rather than ordered by transport timing.

### Barrier rule

Each accepted canonical command is a transaction barrier:

```text
current stable snapshot
-> apply/validate the command's direct canonical ingress
-> evaluate all zero-delay work caused by that command as deterministic commit waves
-> reach a complete stable commit boundary
-> make that completed state visible to the next canonical command
```

A single command schema may contain a bounded atomic collection, such as multiple observations that the host requires to become visible together. That collection is one canonical command and one initial ingress transaction; it does not create a new runtime primitive.

Transport batches are packaging only. Splitting or combining the same canonical command stream across HTTP calls, embedded calls, or worker queues cannot change semantics.

### Logical time and `advance_time`

- Wall-clock time is never consulted by canonical evaluation.
- The host advances a monotonic integer simulation clock.
- `advance_time(target)` processes due work by logical due time and stable semantic order until the target is fully reached or an explicit work budget yields control.
- Yielding because of a runtime work quota changes completion latency only. It does not create a new occurrence, RNG address, or semantic boundary.
- The committed logical time may not advance past incomplete due work at an earlier logical time.
- Reaching ten days in one request versus ten one-day requests, with no additional canonical inputs, must produce the same final canonical state/output hashes.

### Scheduler occurrence identity

Scheduled/recurring work derives occurrence identity from the obligation/trigger schedule and its persisted logical occurrence index, never from loop count, worker count, transport call count, or number of catch-up chunks.

### Profile/config activation

A hot tune, structural profile activation, or migration that can affect canonical outcomes is itself applied at an explicit canonical barrier with an effective logical time and ordinal. Activation cannot occur mid-wave.

### Snapshots

A persistence snapshot may represent only a complete stable commit boundary. A snapshot taken during propagation must either wait for the boundary or use an isolated immutable snapshot of the last completed boundary.

### Randomness

Canonical stochastic decisions derive a semantic random address from:

```text
root_seed
+ behavior_epoch / exact behavior-artifact context
+ rule_or_trigger_id
+ scope_or_actor_id
+ logical occurrence/index
```

There is no shared mutable global RNG stream.

### Numeric model

- canonical probabilities and scores use integer/fixed-point arithmetic where practical;
- canonical wire/config values that participate in deterministic arithmetic have one validated normalized representation;
- floating-point convenience is excluded from canonical hashing/evaluation unless explicitly proven deterministic across the declared platform envelope.

### Commit waves

Within one command barrier:

```text
stable snapshot
-> evaluate due/activated rules
-> collect effects
-> deterministic total sort
-> validate authority/bounds/cycle constraints
-> atomically commit the effect batch
-> enqueue newly affected delayed/future work
-> repeat only where v1 zero-delay semantics explicitly allow another acyclic wave
-> stable boundary
```

Zero-delay recursive causal cycles are invalid in v1.

## Consequences

- Concurrent service requests cannot silently define world history.
- Embedded and service batch partitioning becomes semantically irrelevant.
- Replay fixtures can distinguish canonical command order from transport order.
- Quota/catch-up implementation choices cannot alter RNG occurrence indexing.

## Verification

1. identical canonical commands submitted as one transport batch, split batches, and concurrent service requests produce identical results;
2. same-time host observation and actor evaluation with explicit opposite ordinals produce the two explicitly ordered results; missing order is rejected;
3. ten-day advance once versus one-day advance ten times produces equal hashes when no other inputs intervene;
4. randomized insertion/worker interleaving with colliding equal-time effects preserves hashes;
5. duplicate identical commands are idempotent; duplicate IDs/ordinals with differing payloads are rejected;
6. profile/config reload arriving during a wave activates only at its declared barrier or rolls back;
7. snapshot during heavy propagation resolves to a complete stable commit boundary;
8. same canonical fixture hashes execute on Linux, Windows, and Android before cross-platform determinism is declared complete.
