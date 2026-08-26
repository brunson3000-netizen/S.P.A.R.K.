# ADR-0006 — Persistence, Content-Addressed Behavior Artifacts, Replay, and Bounded Provenance

**Status:** REVISED AFTER INDEPENDENT PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN semantics; persistence backend PROVISIONAL

## Context

S.P.A.R.K. requires continuity, migration safety, and bounded practical explanations without recreating the shelved Living Chronicle.

The independent Phase-0 review found that version strings/epochs alone did not bind old saves and delayed obligations to the exact behavior artifacts that created them.

## Decision

### Persist canonical continuity state

Persist only what is needed to continue deterministically within the declared support envelope:

- logical simulation time and last committed canonical input ordinal;
- root seed and behavior epoch;
- core semantics/schema version;
- exact profile manifest content hash;
- exact configuration revision hash;
- scheduler/delayed obligations;
- S.P.A.R.K.-owned StateCells;
- sparse directed relationships;
- selected memories and beliefs;
- active goals;
- cooldowns/occurrence counters;
- bounded source references for active/recent explanations;
- voice profiles where enabled.

### Immutable behavior-artifact store

A canonical profile/config artifact that may be referenced by a save, active epoch, or delayed obligation is immutable and content-addressed.

- A human version label may be reused accidentally; a content hash may not.
- Referenced artifacts must remain retained in the save package/repository artifact store or be resolvable through an approved local artifact resolver.
- An artifact is never silently overwritten in place.
- Failure to resolve an artifact required by the declared compatibility mode is explicit.

### Save compatibility envelope

**Continuation without migration** is supported only when the runtime can resolve the exact required:

```text
core semantics compatibility
profile manifest content hash
configuration revision hash
behavior epoch artifacts
definition fingerprints referenced by persisted state/work
```

If exact continuation is unavailable, the save must use an explicit declared migration or fail.

### Delayed-obligation binding

Every delayed obligation records enough identity to prevent execution under accidental new semantics:

```text
obligation_id
due_time
logical occurrence/index
creator rule/trigger ID
creator definition fingerprint
creator behavior epoch
creator manifest/config hashes
execution mode
```

Supported execution modes are explicit:

1. **materialized effect** — all canonical effect semantics needed later were frozen into the obligation when scheduled; later rule edits do not reinterpret it.
2. **rule re-evaluation** — later execution requires resolving the exact originating rule/profile/config artifact identified by hash.

No obligation may silently switch modes or execute using the currently active rule merely because the ID matches.

### Atomic migration/checkpoint activation

Migration follows:

```text
resolve source artifacts
-> validate declared migration path
-> create isolated candidate snapshot
-> transform
-> validate authority/IDs/bounds/obligations
-> bind target hashes/epoch
-> atomically activate target checkpoint
```

Failure leaves the source save/checkpoint intact and active.

### Outcome replay versus mechanism replay

The system distinguishes:

- **mechanism replay:** re-execute canonical commands/rules using the exact behavior artifacts and ordering contract;
- **outcome replay:** apply a separately captured authoritative sequence of already-resolved outcomes/effect batches.

They are different support modes and cannot be silently substituted.

A normal S.P.A.R.K. save is a bounded continuity snapshot, not an exhaustive event log. If the retained material is insufficient for a requested replay mode, that replay request fails explicitly.

### Dormant/aggregate history boundary

Promotion from dormant/aggregate simulation may materialize current actor state from declared aggregate commitments and profile rules, but it may not invent unrecorded individual memories, relationships, actions, or microhistory and then present them as historical fact.

### Do not persist by default

- every eligibility check or failed probability roll;
- ordinary ephemeral appraisals/choice scores;
- every dialogue candidate;
- low-value interactions;
- exhaustive world history or complete lifetime transcripts.

A persistent interpretation that has behavioral continuity may be represented explicitly as a normal declared `StateCell`.

### Bounded provenance

Explanation provenance is bounded and deterministic.

When pruning/summarization occurs, the explanation record reports at least:

```text
coverage_status = complete | truncated | unknown
retained_source_count
omitted_source_count where knowable
deterministic_pruning_policy/version
summary_hash where a derived summary exists
```

A derived summary is not an authoritative source reference. If causal provenance was never retained, inspection says so and does not reconstruct imaginary causes.

## Consequences

- Old saves cannot silently inherit new rule semantics.
- Delayed obligations remain stable across profile reloads.
- Migration becomes atomic and falsifiable.
- Replay claims are explicit rather than Chronicle-by-implication.
- Explanations remain bounded without pretending truncated evidence is complete.

## Verification

1. save with delayed obligation, replace active profile, prove exact originating semantics are used or explicit migration occurs;
2. same human version label with different content hash does not satisfy continuation;
3. missing required exact artifact fails explicitly;
4. materialized-effect and rule-re-evaluation delayed obligations behave according to their declared modes;
5. migration failure leaves source checkpoint unchanged;
6. mechanism-replay and outcome-replay fixtures are separate and unsupported substitution fails;
7. snapshot during propagation represents a complete stable commit boundary;
8. dormant-to-active promotion does not synthesize unrecorded microhistory;
9. provenance pruning produces deterministic coverage/truncation metadata and never invents causes;
10. audit confirms no universal append-only world event ledger becomes a core dependency.
