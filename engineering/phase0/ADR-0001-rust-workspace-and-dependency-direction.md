# ADR-0001 — Rust Workspace and Dependency Direction

**Status:** ACCEPTED FOR PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN unless operator reopens language or deployment architecture

## Context

S.P.A.R.K. must run as a standalone service and embeddable deterministic runtime across Windows, Linux, and Android. The controlling blueprint freezes Rust as the production language and requires the core to remain independent of expression, voice, service, rendering, TTS, game, and MCI concerns.

## Decision

Use a Rust workspace with dependency direction centered on a small deterministic core.

Reference boundaries:

```text
spark-core
  ^      ^
  |      |
spark-profile     spark-protocol
  ^      ^             ^
  |      |             |
spark-persistence  spark-inspection
          ^             ^
          |             |
      spark-expression  spark-service
             ^           ^
             |           |
         spark-voice   adapters/*
```

The exact crate graph may be refined, but these rules are invariant:

1. `spark-core` may not depend on service transport, game/MCI adapters, TTS, rendering, browser frameworks, or runtime LLMs.
2. `spark-expression` and `spark-voice` consume semantic core outputs; they do not become causal authorities.
3. `spark-service` wraps the same canonical engine used by the embedded API.
4. Host adapters translate host state/messages; they do not duplicate causal rules.
5. Profile loading/validation is separated from host adapters so the same profile semantics work in service and embedded modes.
6. Platform-specific facilities stay behind narrow adapters and out of canonical evaluation.

## Consequences

- Cross-platform behavior is easier to test and replay.
- The core remains embeddable in constrained game/Android environments.
- UI/service convenience cannot quietly become part of causal semantics.
- Some duplication of boundary glue is acceptable to preserve clean authority/dependency direction.

## Rejected alternatives

- **CPython production core:** violates frozen language/embeddability constraints.
- **One monolithic application crate:** makes service/UI/platform concerns too easy to leak into canonical logic.
- **Plugin-first architecture:** contradicts the frozen decision against a universal plugin runtime.

## Verification

- workspace dependency graph check in CI;
- prohibited-dependency denylist for canonical crates;
- Linux/Windows build and deterministic fixtures;
- Android-compatible core build/check target from Phase 1 onward.
