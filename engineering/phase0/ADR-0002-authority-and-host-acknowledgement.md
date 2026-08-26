# ADR-0002 — State Authority and Host Acknowledgement

**Status:** ACCEPTED FOR PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN

## Context

The host owns physical and economic reality and final action execution. S.P.A.R.K. owns causal, social, psychological, relationship, selected-memory, belief, goal, and advisory behavior state. Derived state is deterministic and not independently writable.

## Decision

Every state definition declares exactly one authority mode:

```text
host_owned
spark_owned
derived
```

### Canonical rules

1. `host_owned` values enter S.P.A.R.K. only through validated host observations/outcomes.
2. S.P.A.R.K. may read host-owned values and derive pressures but may not canonically commit a host-owned mutation.
3. `spark_owned` values are canonical within S.P.A.R.K. and change only through validated engine effects/config migrations.
4. `derived` values are recomputed from declared authoritative inputs and may not be independently written.
5. `BehaviorIntent` and `GameplayHookCandidate` are advisory semantic outputs, not execution commands.
6. Host execution status is explicit: accepted/executed, rejected, deferred, or failed.
7. Only host-confirmed outcomes may be treated as facts about physical/economic execution.
8. The MCI social profile is structurally incapable of gaining execution, governance, credential, model-routing, budget, file-mutation, or review authority.

## Transaction implication

An intent may influence S.P.A.R.K.-owned expectation/state before host acknowledgement only when the profile explicitly models anticipation. It must not be mistaken for the actual host outcome.

## Consequences

- S.P.A.R.K. can reason richly without becoming a second game/economy authority.
- Rejected/deferred actions remain explainable rather than silently assumed to have happened.
- Authority can be tested as a property rather than trusted as convention.

## Verification

- property tests reject canonical writes to host-owned state;
- derived-state direct-write rejection tests;
- host accept/reject/defer/fail integration fixtures;
- negative capability enumeration for MCI social mode;
- adversarial protocol tests for authority escalation attempts.
