# ADR-0003 — Deterministic Clock, Random Addresses, and Commit Order

**Status:** ACCEPTED FOR PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN

## Context

S.P.A.R.K. requires deterministic replay across declared Windows/Linux/Android support builds. The host advances simulation time; broad catalogs and many actors must not force global polling or a shared mutable RNG stream.

## Decision

### Logical time

- Canonical evaluation uses a host-advanced monotonic integer simulation clock.
- Wall-clock time is never consulted by canonical rule evaluation.
- Equal-time work has a stable total ordering based on semantic IDs/sequence fields, never container iteration order.

### Scheduler

- Work is scheduled by due time/cadence/obligation.
- Advancing time evaluates only due work plus work activated by committed changes.
- If work budgets are exceeded, overflow is deferred deterministically and surfaced as a warning/metric.

### Randomness

Canonical stochastic decisions derive a semantic random address from:

```text
root_seed
+ behavior_epoch/profile version context
+ rule_or_trigger_id
+ scope_or_actor_id
+ logical occurrence/index
```

There is no shared mutable global RNG stream for canonical behavior.

### Numeric model

- canonical probabilities and choice scores use integer/fixed-point arithmetic where practical;
- floating-point convenience may be used outside canonical evaluation or where explicitly proven deterministic within the declared support envelope;
- bounds/clamps are definition-driven and deterministic.

### Commit waves

Evaluation follows:

```text
stable snapshot
-> evaluate due rules
-> collect effects
-> deterministic sort
-> validate authority/bounds/cycle constraints
-> commit batch
-> enqueue newly affected future/delayed work
```

Zero-delay recursive causal cycles are invalid in v1.

## Consequences

- Concurrency, batching, and unrelated iteration changes do not perturb unrelated stochastic results.
- Replay fixtures can detect semantic divergence.
- Determinism rules constrain some implementation shortcuts, intentionally.

## Verification

- same seed/input fixture hash across Linux/Windows;
- randomized insertion-order property tests;
- concurrency/batch partition invariance tests;
- no wall-clock dependency audit in canonical crates;
- zero-delay cycle rejection corpus.
