# ADR-0004 — Profile Schema, Validation, Stable IDs, Authority Identity, and Change Classes

**Status:** REVISED AFTER INDEPENDENT PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN semantics; exact serialization format PROVISIONAL

## Context

S.P.A.R.K. freezes the causal grammar while keeping domain vocabulary editable. Traits, triggers, behaviors, religions, denominations, weights, cadences, dialogue templates, and packs must usually be added or changed without Rust work.

The independent Phase-0 review found that immutable IDs alone did not prevent authority from being silently changed under an existing ID, and that exact behavior artifacts needed stronger content identity for save continuation.

## Decision

### Profile manifest

Every active profile is a versioned logical manifest that declares, where applicable:

- immutable namespaced ID;
- definition kind/domain/layer;
- value type and bounds;
- authority mode and implied write class;
- valid scopes;
- lifecycle/update semantics;
- consumers/effects;
- version/deprecation/migration metadata;
- configuration mutability class.

The exact on-disk syntax remains provisional. The canonical logical representation used for validation/content hashing must be deterministic and format-independent.

### Content identity

Validated artifacts receive immutable content identities:

```text
manifest_content_hash
config_revision_hash
definition_fingerprint
```

A version/display label may aid humans but is never sufficient by itself to identify behavior.

A `definition_fingerprint` includes every immutable semantic identity property, including at minimum:

- definition ID;
- definition kind;
- value type;
- authority mode;
- implied write class;
- scope compatibility where changing it would alter ownership/meaning.

### Stable IDs and immutable authority

Persisted IDs may be disabled, deprecated, or explicitly migrated. They may not be silently repurposed or deleted from active saves.

Once a definition ID has entered persisted/active canonical state:

- its authority/write class cannot change under that ID;
- an alias cannot disguise such a change;
- a change of authority/write class requires a new ID, explicit migration, authority ADR, and operator approval.

### Three change classes

1. **Hot-tunable parameter change** — bounded probabilities, cadences, thresholds, weights, decay/recovery, cooldowns, severity, salience/repetition limits that the schema explicitly marks mutable.
2. **Structural profile change** — definitions, topology, scopes, packs, templates, taxonomy/profile vocabulary; requires controlled validation, content hashing, checkpoint, and atomic activation.
3. **Engine-semantic change** — new primitive, authority semantics, canonical arithmetic, transaction semantics, or protocol capability not representable by the current contract; requires Rust + ADR and operator escalation.

Authority/write-class changes are never ordinary structural reloads.

### Behavior epoch

Any accepted profile/config change that can alter canonical outcomes activates a new explicit behavior epoch at a canonical command barrier. The active manifest hash and config revision hash are bound to that epoch.

### Atomic activation

Hot tune/structural activation follows:

```text
parse
-> validate
-> canonicalize/hash
-> compatibility/authority checks
-> stage isolated candidate
-> checkpoint if required
-> activate at declared canonical barrier
```

Failure leaves the prior active profile/config untouched.

### Profile/trust-domain isolation

Profile isolation includes more than manifest naming:

- separate canonical state stores or enforced profile partition keys;
- profile-qualified ID resolution;
- profile-qualified behavior epochs/config revisions;
- non-composable capability/trust-domain grants;
- cross-profile reference rejection unless an explicit read-only bridge is defined;
- isolated output subscriptions/routing.

The game and MCI profiles may not resolve or mutate each other's private state by ID collision or alias.

### Taxonomy rule

The current domain/family counts are a **provisional baseline profile vocabulary**, not an exhaustive hard-coded Rust catalog. What is frozen is that these distinctions are expressible as profile data through the shared grammar.

### Appraisal persistence rule

Appraisals and choice-context scores are normally ephemeral computations. A profile may represent a behaviorally persistent interpretation using an ordinary declared `StateCell`; doing so does not add a new primitive.

### Validator requirements

Reject:

- ambiguous ownership;
- immutable identity mismatch;
- unsupported scope;
- unknown operation;
- invalid bounds/units;
- illegal cycles;
- undeclared arbitrary mutation;
- incompatible artifact/epoch references;
- alias cycles;
- cross-profile private references;
- unbounded graph/schema constructs.

Warn on dead definitions, redundant concepts, excessive fan-out, missing recovery for accumulating pressures, and unsupported host dependencies.

### Security rule

Profile content is declarative data. No arbitrary code execution, native dynamic modules, eval, or general scripting language enters the v1 profile path.

## Consequences

- Content breadth remains editable without expanding engine semantics.
- Authority cannot be acquired by changing metadata.
- Saves can identify exact profile/config content rather than trusting reused version strings.
- Game/MCI isolation becomes a state/ID/routing property, not merely a profile-name convention.

## Verification

- invalid-profile corpus;
- add curiosity, blood moon, and religion/denomination without Rust changes;
- change authority/write class through reload, alias, migration, and restore: reject unless new-ID authority migration ceremony is used;
- reused version string with different content produces different hash and cannot satisfy an exact-artifact save reference;
- hot tune/structural reload mid-wave activates only at a canonical barrier or rolls back;
- cross-profile ID/reference/alias resolution attacks are rejected;
- default ephemeral appraisal snapshot exclusion plus explicit persistent-interpretation-as-StateCell fixture;
- reject executable/general-script payloads.
