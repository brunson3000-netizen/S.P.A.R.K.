# S.P.A.R.K. Phase 0 Performance and Security Budget v0.2

**Status:** PROVISIONAL ENGINEERING BUDGET / not a gameplay-content freeze  
**Date:** 2026-08-25  
**Supersedes:** `PHASE_0_PERFORMANCE_AND_SECURITY_BUDGET_v0.1.md`  
**Purpose:** Close Phase-0 boundedness/security findings while keeping tunable numeric limits provisional.

## 1. Hard Budget Principles

1. Idle cost scales with **due work**, not total dormant definition count.
2. Active cost scales with **active actors, changed state, sparse edges, due obligations, and aggregate scopes**, not total world cells/population.
3. A global/scoped trigger may update aggregate/inherited exposure state, but ordinary occurrence handling may not enumerate every dormant actor or addressable cell.
4. Canonical work is bounded. Admission, queueing, propagation, and explanation limits are explicit; overflow is deterministically deferred, aggregated, paginated, or rejected and made visible.
5. Relationship, memory, provenance, alias/migration chains, graph expansion, and delayed work are policy-bounded.
6. Security is deny-by-default and local-first. Validation/admission happens before canonical mutation.
7. No performance optimization may weaken authority, determinism, migration safety, profile isolation, or explanation correctness.
8. Transport/session/backpressure behavior may affect delivery/admission but not the causal result of an already accepted canonical command.

## 2. Initial Performance Targets

These are benchmark targets or provisional safety defaults, not claims about achieved performance.

| Metric | Initial target | Classification |
|---|---:|---|
| Dormant definition idle cost | no O(total definitions) scan on ordinary clock advance | hard architectural budget |
| Global trigger actor/cell fan-out | no ordinary O(total dormant population/cells) materialization; aggregate/inherited exposure until due/activation | hard architectural budget |
| Trigger/rule lookup | indexed/due-set access; no global hourly polling | hard architectural budget |
| Canonical propagation depth per command barrier | default 16 | provisional safety default |
| Direct effects/fan-out from one rule occurrence | warning 128; default hard cap 256 unless validated profile budget changes it | provisional safety default |
| New delayed obligations from one rule occurrence | default hard cap 256 | provisional safety default |
| Total pending delayed obligations | deployment/profile must declare a finite cap; reference default 100,000 | provisional resource default |
| Accepted canonical ingress queue | finite deployment cap; reference default 4,096 commands before deterministic backpressure/rejection | provisional service default |
| Static profile definitions | reference validation cap 16,384 | provisional schema default |
| Static propagation rules | reference validation cap 32,768 | provisional schema default |
| Static rule-graph edges | reference validation cap 131,072 | provisional schema default |
| Alias/migration reference-chain depth | default hard cap 8; cycles always reject | provisional schema default |
| Structured payload nesting | default max depth 16 | provisional protocol default |
| Namespaced ID/idempotency string length | default max 256 UTF-8 bytes after validation | provisional protocol default |
| General profile description/string field | default max 8 KiB unless schema field declares smaller explicit limit | provisional schema default |
| Explanation immediate source refs | 64 retained refs before deterministic truncation/summarization metadata | provisional storage default |
| Rich named-actor selected memories | soft target 5–25; profile hard cap explicit | blueprint-derived target |
| Active goals per rich named actor | typical 1–5; higher profile cap explicit | blueprint-derived target |
| Service ordinary command payload | default 256 KiB; bulk/profile admin paths separately bounded | provisional service default |
| Actor batch evaluation request | default 512 actors/request | provisional service default |
| Explain/inspect query | p95 <= 100 ms on reference desktop fixture after Phase 3; bounded page/work quota | provisional usability target |
| Canonical actor-choice evaluation | p95 <= 1 ms/active named actor on reference desktop microbenchmark after Phase 4 | provisional engineering target |
| 100 active named actors choice cycle | p95 <= 50 ms reference desktop, excluding host pathfinding/render/TTS | provisional engineering target |
| Dormant settlement aggregate update | target <= 0.25 ms/scope/update in reference fixture | provisional engineering target |
| Catch-up | scale with scheduled aggregate obligations, not reconstructed microhistory | hard architecture + calibration |
| Persistence | report bytes per actor/scope and growth slope; no unbounded lifetime transcript | hard architectural budget |

### Budget-change rule

Any provisional numeric default may change from benchmark evidence and a versioned budget revision. A change to a frozen architectural property requires ADR/operator escalation.

## 3. Deterministic Backpressure and Internal Queue Rules

1. Every internal queue has a finite declared admission limit.
2. Hitting a runtime work quota does not change canonical ordering, RNG occurrence indexes, or already accepted command semantics.
3. Due work that is safely deferrable is resumed in the same deterministic order.
4. A queue that cannot safely defer must reject the creating command atomically before partial canonical commit.
5. Repeated delayed-work creation is charged against both per-occurrence expansion limits and the target queue's finite admission budget.
6. Aggregation may replace many equivalent dormant obligations only when the aggregation rule is itself declared deterministic/profile-valid and preserves required semantics.
7. Backlog age, rejection, aggregation, and deferred counts are telemetry-visible.

## 4. Global/Scoped Exposure Rule

A world/region/faction/religion trigger is represented initially at the narrowest useful aggregate/inherited scopes.

Dormant individuals inherit or materialize actor-specific exposure only when:

- they become active/warm;
- an actor-specific obligation is independently due;
- a declared aggregate-to-individual rule requires bounded sampling/materialization.

A scaling fixture with 10x dormant population must show that ordinary global-trigger occurrence cost does not grow proportionally with dormant actor count.

## 5. Security and Trust-Domain Budget

### Network/service posture

- local-only bind by default;
- ephemeral/local authentication token for service sessions;
- unrecognized client/profile/version/message/capability denied;
- no direct client database access;
- no arbitrary code execution/profile eval;
- no remote/cloud dependency in canonical core.

### Capability/trust posture

Capabilities are scoped inside a trust domain and are non-composable across profiles unless an explicit safe bridge is defined.

MCI social mode has:

- explicit input allowlist for sanitized/coarse telemetry;
- explicit output-type allowlist for fictional presentation;
- no output routing to real-work adapters;
- no state/config/ID-resolution access into the game or S.W.A.R.M. production-control trust domains.

MCI must never receive or synthesize authority for:

- real work assignment;
- tool execution;
- credentials;
- model routing;
- economic/spend authority;
- governance;
- file mutation;
- task cancellation;
- review approval.

Inspection dry-run/staging uses an isolated snapshot and isolated configuration candidate. It has no production commit path.

### Input/resource limits

- every collection has an explicit maximum;
- recursive/nested structures are schema-bounded;
- oversized queries are rejected or paginated;
- IDs/keys/strings are bounded;
- profile graph size, alias depth, migration depth, and expansion are bounded;
- rate limiting/backpressure exists before the Phase-3 service gate;
- unknown fields default to rejection for security-sensitive mutation messages unless compatibility policy explicitly allows them.

### Mutation posture

- host-owned canonical state has no S.P.A.R.K.-initiated write path;
- authority/write class is immutable for a persisted definition ID;
- config mutation uses allowlist, capability, range validation, content hash, canonical activation barrier, and behavior-epoch visibility;
- production state is not writable through AI inspection endpoints.

## 6. Telemetry Required to Enforce Budgets

At minimum:

- due work by cadence/type;
- command acceptance/rejection/idempotency;
- queue depth/backlog age;
- evaluation/commit counts;
- edge fan-out/propagation depth;
- delayed-obligation creation/defer/reject/aggregate counts;
- active/warm/dormant counts;
- global-trigger materialization counts;
- candidate behavior counts/score distributions;
- memory/relationship/provenance growth;
- provenance truncation/coverage counts;
- query/explanation latency/work quota;
- bytes persisted per actor/scope;
- deterministic fixture hash divergence;
- rule/definition hit rate and dead/unused definitions;
- profile graph/alias/migration complexity;
- cross-profile capability/reference/routing rejection counts.

## 7. Phase Gates

- **Phase 1:** benchmark harness exists before performance claims; canonical command/clock/RNG fixtures include quota-partition invariance.
- **Phase 2:** propagation/fan-out/depth/delayed-work budgets tested adversarially.
- **Phase 3:** authentication/capability/input/rate-limit/profile-isolation/admission tests pass before service considered usable.
- **Phase 4+:** actor/state/memory defaults recalibrated only from measured workload/gameplay evidence.
- **Cross-platform acceptance:** executable canonical fixture hash parity must include Android, not compilation compatibility alone.

## 8. Explicit Non-Claims

This document does not freeze final:

- world size;
- concurrent player count;
- active-envelope geometry;
- trigger catalog size;
- actor count;
- persistence backend;
- transport;
- TTS performance;
- provisional numeric queue/schema limits above.

Those require joint S.P.A.R.K./host profiling.
