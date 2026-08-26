# ADR-0002 — State Authority, Immutable Write Class, and Host Acknowledgement

**Status:** REVISED AFTER INDEPENDENT PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN

## Context

The host owns physical and economic reality and final action execution. S.P.A.R.K. owns causal, social, psychological, relationship, selected-memory, belief, goal, and advisory behavior state. Derived state is deterministic and not independently writable.

The independent Phase-0 review identified two loopholes: authority metadata could be changed through profile lifecycle operations, and host acceptance of an intent was not sufficiently separated from confirmation that the action actually occurred.

## Decision

Every state definition declares exactly one authority mode:

```text
host_owned
spark_owned
derived
```

Each authority mode implies an immutable canonical write class:

```text
host_owned   -> host_ingress_only
spark_owned  -> spark_effect_or_explicit_migration
derived      -> evaluator_only
```

### Immutable authority identity

For any definition ID that has ever been persisted, referenced by a save, or activated in a behavior epoch:

1. `authority` and its implied write class are immutable identity properties.
2. A hot tune, structural reload, alias, restore, or ordinary migration may not change them.
3. Changing authority/write class requires:
   - a new definition ID;
   - an explicit migration specification;
   - an authority ADR;
   - operator approval.
4. An alias may not hide an authority/write-class change.
5. Snapshot restore must validate the definition fingerprint before restoring canonical state.
6. No migration is allowed to fabricate host-owned or derived truth. Host-owned values remain sourced from the host; derived values remain evaluator-produced.

### Canonical write rules

1. `host_owned` values enter S.P.A.R.K. only through validated authorized host observations or host-confirmed outcomes.
2. S.P.A.R.K. may read host-owned values and derive pressures but may not canonically commit a host-owned mutation.
3. `spark_owned` values are canonical within S.P.A.R.K. and change only through validated engine effects or an explicitly authorized migration.
4. `derived` values are recomputed from declared authoritative inputs and may not be independently written.
5. `BehaviorIntent` and `GameplayHookCandidate` are advisory semantic outputs, not execution commands.

### Intent lifecycle

Host acknowledgement is separated into two concepts.

**Disposition / receipt stage**

```text
accepted
rejected
deferred
```

`accepted` means only that the host accepted responsibility for handling the intent. It is not proof of execution.

**Outcome stage**

```text
executed
failed
cancelled
```

Only an `executed` outcome with validated actual effects may introduce facts about physical/economic execution into causal feedback.

A deferred intent remains non-executed unless a later valid outcome is supplied.

### Acknowledgement binding

Every disposition/outcome must:

- bind to an outstanding `intent_id`;
- bind to the correct profile/trust domain and logical host identity;
- carry an idempotency identity;
- reject duplicate payload conflicts;
- reject forged, stale, cross-profile, or incompatible acknowledgements;
- preserve deterministic handling of duplicate identical acknowledgements.

### MCI and inspection trust-domain isolation

The MCI social profile is not protected merely by capability names.

It additionally requires:

- an explicit MCI output-type allowlist;
- no route from MCI outputs to real-work-consuming adapters;
- rejection of cross-profile state references, subscriptions, config mutation, and acknowledgements;
- non-composable capabilities across trust domains;
- inspection staging/dry-run on an isolated snapshot with no production commit path.

## Transaction implication

An intent may influence explicitly modeled S.P.A.R.K.-owned anticipation state before outcome only when the profile declares that anticipation. It must never be represented as the actual physical/economic outcome.

## Consequences

- S.P.A.R.K. cannot gain economy/world authority through a profile reload.
- Accepted-but-never-executed actions cannot create phantom success.
- MCI fiction cannot leak into real S.W.A.R.M. control through generic shared output families.
- Authority becomes a lifecycle invariant that can be property-tested.

## Verification

- reject canonical writes to `host_owned`;
- reject direct writes to `derived`;
- attempt authority/write-class changes through hot tune, reload, alias, migration, and restore;
- host accepted-but-not-executed fixture produces no physical/economic feedback;
- duplicate/stale/forged/cross-profile/out-of-order acknowledgement suite;
- MCI emits every shared output family and proves real-work-consuming families structurally unavailable;
- inspection dry-run proves production snapshot/hash remains unchanged.
