# ADR-0004 — Profile Schema, Validation, Stable IDs, and Change Classes

**Status:** ACCEPTED FOR PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN semantics; exact serialization format PROVISIONAL

## Context

S.P.A.R.K. freezes the causal grammar while keeping domain vocabulary editable. Traits, triggers, behaviors, religions, denominations, weights, cadences, dialogue templates, and packs must usually be added or changed without Rust work.

## Decision

### Profile manifest

Every active profile is a versioned manifest that declares:

- immutable namespaced IDs;
- definition kind/domain/layer;
- value type and bounds;
- authority mode;
- valid scopes;
- lifecycle/update semantics;
- consumers/effects where applicable;
- version/deprecation/migration metadata;
- configuration mutability class.

The exact on-disk syntax (YAML/JSON/TOML/etc.) remains provisional until implementation comparison, but the logical schema is frozen.

### Stable IDs

Persisted IDs may be disabled, deprecated, aliased, or migrated. They may not be silently repurposed or deleted from active saves.

### Three change classes

1. **Hot-tunable parameter change** — bounded probabilities, cadences, thresholds, weights, decay/recovery, cooldowns, severity, salience/repetition limits.
2. **Structural profile change** — definitions, topology, scopes, packs, templates; requires controlled validation/reload/checkpoint.
3. **Engine-semantic change** — new primitive, authority semantics, canonical arithmetic, transaction semantics, or protocol capability not representable by current contract; requires Rust + ADR and operator escalation where blueprint requires.

### Validator requirements

Reject definitions with ambiguous ownership, unsupported scope, unknown operation, invalid bounds/units, illegal cycles, undeclared arbitrary mutation, or incompatible versions.

Warn on dead definitions, duplicate/redundant concepts, excessive fan-out, missing recovery for accumulating pressures, and unsupported host dependencies.

### Security rule

Profile content is declarative data. No arbitrary code execution, dynamic native modules, eval, or general scripting language enters the v1 profile path.

## Consequences

- Content breadth can grow without expanding engine semantics.
- Save compatibility has a defined path.
- Profile authors receive early feedback before a bad ontology becomes runtime state.

## Verification

- invalid-profile corpus;
- add `trait.curiosity`, `trigger.supernatural.blood_moon`, and religion/denomination without Rust changes;
- migration/deprecation fixtures;
- reject executable/general-script payloads.
