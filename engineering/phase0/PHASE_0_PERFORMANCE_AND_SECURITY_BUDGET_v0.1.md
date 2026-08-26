# S.P.A.R.K. Phase 0 Performance and Security Budget v0.1

**Status:** PROVISIONAL ENGINEERING BUDGET / not a gameplay-content freeze  
**Date:** 2026-08-25  
**Purpose:** Establish early guardrails so Phase 1–3 code is measurable and bounded. Exact values may be revised from evidence without changing frozen semantics.

## 1. Budget Principles

1. Idle cost scales with **due work**, not total dormant definition count.
2. Active cost scales with **active actors, changed state, sparse edges, due obligations, and aggregate scopes**, not total world cells/population.
3. Canonical work is bounded; overflow is deferred deterministically and made visible.
4. Relationship, memory, explanation provenance, and queues are sparse/bounded by policy.
5. Security is deny-by-default and local-first. Input validation happens before canonical mutation.
6. No performance optimization may weaken authority, determinism, migration safety, or explanation correctness.

## 2. Initial Performance Targets

These are **benchmark targets**, not claims about achieved performance.

| Metric | Initial target | Classification |
|---|---:|---|
| Dormant definition idle cost | No O(total definitions) scan on ordinary clock advance | hard architectural budget |
| Trigger/rule lookup | indexed/due-set access; no global hourly polling | hard architectural budget |
| Canonical propagation depth per commit wave | default 16; profile may lower; raising requires validation | provisional safety default |
| Direct effects/fan-out from one rule occurrence | default warning at 128; hard reject/default cap 256 unless approved profile budget says otherwise | provisional safety default |
| Bounded explanation immediate source references per state/output | 64 retained references before summarization/pruning | provisional storage default |
| Rich named-actor selected memories | default soft target 5–25; profile hard cap must be explicit | blueprint-derived target |
| Active goals per rich named actor | typical 1–5; higher profile cap explicit | blueprint-derived target |
| Service command payload | default 256 KiB; bulk/profile administrative paths use separate explicit limits | provisional security/perf default |
| Actor batch evaluation request | default 512 actors/request; benchmark before raising | provisional service default |
| Explain/inspect query latency | p95 <= 100 ms on reference desktop fixture once Phase 3 exists | provisional usability target |
| Canonical actor-choice evaluation | p95 <= 1 ms/active named actor on reference desktop microbenchmark once Phase 4 exists | provisional engineering target |
| 100 active named actors choice cycle | p95 <= 50 ms reference desktop benchmark, excluding host pathfinding/render/TTS | provisional engineering target |
| Dormant settlement aggregate update | target <= 0.25 ms/scope/update in reference fixture | provisional engineering target |
| Catch-up | must be measured per elapsed game-year and scale with scheduled aggregate obligations, not reconstructed microhistory | hard architecture + calibration |
| Persistence | report bytes per actor/scope and growth slope; no unbounded lifetime transcript | hard architecture |

### Budget-change rule

If profiling shows a provisional number is poorly chosen, engineer may change it with benchmark evidence and a budget revision. Changing a frozen architectural property requires ADR/operator escalation as defined in the blueprint.

## 3. Security Budget

### Network/service posture

- local-only bind by default;
- ephemeral/local authentication token for service sessions;
- unrecognized client/profile/version/message/capability is denied;
- no direct client database access;
- no arbitrary code execution or profile `eval`;
- no remote/cloud dependency in canonical core.

### Capability posture

Baseline capability families remain narrowly scoped to signal/context write, state/rules/explanation read, output subscription, approved config access, and profile administration.

MCI social mode must never receive capabilities for:

- real work assignment;
- tool execution;
- credentials;
- model routing;
- economic/spend authority;
- governance;
- file mutation;
- task cancellation;
- review approval.

### Input/resource limits

- all collections have explicit maximum sizes at protocol validation boundaries;
- recursive/nested payload structures are schema-bounded;
- oversized explanation/profile queries are rejected or paginated;
- message IDs/idempotency keys are bounded strings, not arbitrary blobs;
- rate limiting exists at service boundary before Phase 3 gate passes;
- unknown fields default to rejection for security-sensitive mutation messages unless schema compatibility rules explicitly allow them.

### Mutation posture

- host-owned canonical state has no S.P.A.R.K.-initiated write path;
- config mutation uses explicit allowlist, capability, range validation, and revision/epoch visibility;
- profile reload is validated before activation;
- production state is not directly writable through AI inspection endpoints.

## 4. Telemetry Required to Enforce Budgets

At minimum:

- due work by cadence/type;
- evaluation/commit counts;
- edge fan-out and propagation depth;
- deferred/truncated work;
- active/warm/dormant counts;
- candidate behavior counts and score distributions;
- memory/relationship/provenance growth;
- query/explanation latency;
- bytes persisted per actor/scope;
- deterministic fixture hash divergence;
- rule/definition hit rate and dead/unused definitions.

## 5. Phase Gates

- **Phase 1:** benchmark harness exists before performance claims.
- **Phase 2:** propagation/fan-out/depth budgets tested adversarially.
- **Phase 3:** authentication/capability/input/rate-limit adversarial tests pass before service considered usable.
- **Phase 4+:** actor/state/memory defaults may be recalibrated only from measured workload/gameplay evidence.

## 6. Explicit Non-Claims

This document does **not** freeze:

- final world size;
- final concurrent player count;
- final active-envelope radius;
- final trigger catalog size;
- final actor count;
- final persistence backend;
- final transport;
- final TTS performance.

Those require joint S.P.A.R.K./host profiling.
