# ADR-0006 — Persistence, Versioning, Behavior Epochs, and Bounded Provenance

**Status:** ACCEPTED FOR PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN semantics; backend PROVISIONAL

## Context

S.P.A.R.K. requires continuity, migration safety, and bounded practical explanations without recreating the shelved Living Chronicle.

## Decision

### Persist canonical continuity state

Persist only what is needed to continue deterministically within the declared support envelope:

- logical simulation time;
- root seed and behavior epoch;
- profile/schema/content versions;
- scheduler/delayed obligations;
- S.P.A.R.K.-owned StateCells;
- sparse directed relationships;
- selected memories and beliefs;
- active goals;
- cooldowns/occurrence counters;
- bounded source references for active/recent explanations;
- voice profiles where enabled.

### Do not persist by default

- every eligibility check or failed probability roll;
- ephemeral appraisals;
- every candidate score/dialogue option;
- low-value interactions;
- exhaustive world history or complete lifetime transcripts.

### Versioning

- definitions have immutable IDs and explicit versions;
- semantic profile/rule changes create explicit behavior epochs;
- structural changes use controlled reload/migration;
- unsupported replay/migration fails explicitly rather than silently approximating;
- snapshots are persistence/acceleration mechanisms, not replacements for declared authoritative inputs.

### Provenance

Explanation stores only bounded source references sufficient to answer practical current/recent “why” questions. If provenance was never retained or was pruned, inspection reports that limitation and does not reconstruct imaginary history.

## Consequences

- Persistence remains bounded and compatible with large worlds.
- Explanations stay evidence-based.
- Chronicle scope cannot quietly return through telemetry/logging convenience.

## Verification

- snapshot field whitelist test;
- migration/deprecation corpus;
- replay epoch tests;
- snapshot size benchmarks;
- degraded/missing provenance explanation fixture;
- audit that no universal append-only world event ledger becomes a core dependency.
