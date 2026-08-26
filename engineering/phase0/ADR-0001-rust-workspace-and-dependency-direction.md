# ADR-0001 — Rust Workspace and Dependency Direction

**Status:** REVISED AFTER INDEPENDENT PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN unless operator reopens language or deployment architecture

## Context

S.P.A.R.K. must run as a standalone service and embeddable deterministic runtime across Windows, Linux, and Android. The controlling blueprint freezes Rust as the production language and requires the canonical core to remain independent of expression, voice, service transport, rendering, TTS, game, and MCI concerns.

The first independent Phase-0 review found the original dependency diagram visually ambiguous even though its written rules were sound. This revision replaces the diagram with explicit mechanically testable dependency policy.

## Decision

Use a Rust workspace centered on one reusable canonical engine.

### Allowed dependency direction

The exact crate graph remains an engineer implementation choice, but the following direction is authoritative:

- `spark-core` may depend only on approved deterministic utility/library dependencies. It may not depend on another S.P.A.R.K. product-layer crate.
- `spark-profile` may depend on `spark-core`.
- `spark-protocol` defines the versioned external logical contract independently of service transport and host internals. It must not depend on `spark-service` or host adapters.
- `spark-persistence` may depend on `spark-core` and `spark-profile`.
- `spark-inspection` may depend on canonical read interfaces from `spark-core`, `spark-profile`, and persistence abstractions where required.
- `spark-expression` may depend on `spark-core` and `spark-profile`; it consumes semantic state/intents and does not become causal authority.
- `spark-voice` may depend on expression/profile interfaces needed to map a persistent abstract voice identity to backend-neutral performance requests.
- `spark-service` may depend on the canonical engine/profile/protocol/persistence/inspection/expression/voice layers needed to expose the standalone service.
- host adapters may depend on the public protocol and/or a narrow embedded facade. They must not duplicate canonical causal rules or acquire direct internal state-write authority.

### Invariants

1. `spark-core` may not depend on service transport, game/MCI adapters, TTS, rendering, browser frameworks, runtime LLMs, or platform-specific UI frameworks.
2. Canonical causal evaluation exists in one engine implementation shared by service and embedded use.
3. Profile loading/validation is separated from host adapters.
4. Platform-specific facilities remain behind narrow adapters and outside canonical evaluation.
5. Dependency rules are enforced mechanically in CI rather than inferred from a diagram.

## Consequences

- Cross-platform behavior is easier to replay and compare.
- The core remains embeddable in constrained game/Android environments.
- Service/UI/platform convenience cannot quietly become causal semantics.
- Some boundary glue duplication is acceptable to protect authority and determinism.

## Rejected alternatives

- **CPython production core:** violates frozen language/embeddability constraints.
- **One monolithic application crate:** makes transport/UI/platform concerns too easy to leak into canonical logic.
- **Plugin-first architecture:** contradicts the frozen decision against a universal plugin runtime.

## Verification

- workspace dependency graph check in CI;
- prohibited-dependency denylist for canonical crates;
- Linux and Windows canonical fixture execution;
- executable Android canonical fixture-replay path before the cross-platform acceptance criterion is declared passed.
