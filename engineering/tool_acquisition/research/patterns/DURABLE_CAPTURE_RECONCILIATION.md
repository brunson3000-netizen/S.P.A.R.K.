# Pattern Card — Durable Capture With Reconciliation

PATTERN: Durable Capture With Immutable Artifacts and Reconciliation
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- xAI grok-build @ `37949780c144e37df692e3d669051a21fec24f20`
- `xai-grok-memory/src/v2.rs`
- `v2_capture.rs`
- `v2_capture_tests.rs`
- `v2_access.rs` + tests
- agent prompt memory contract

## Problem

Long-lived agent state/evidence must survive crashes, retries and concurrent workers without silently duplicating, overwriting or losing the durable fact that later work depends on.

## Mechanism

Separate canonical and derived state:

1. enqueue deterministic work identity
2. claim it under a bounded lease/attempt
3. publish immutable content-addressable/hash-verifiable artifact files
4. commit a transactional outcome row/hash
5. update derived search/index and cursors
6. reconcile on restart to repair supported partial-order crash states
7. fence stale workers and reject conflicting retries.

For editable durable topic files, require read-before-write and compare a content digest immediately before atomic replacement. Derived bounded manifests/indexes are regenerated and can be rolled back/repaired.

## Dependencies

- durable transactional state database
- immutable artifact directory
- deterministic job IDs
- lease/fencing semantics
- outcome digest
- atomic file publication/replacement
- derived index/manifest rebuild
- bounded reconciliation scanner.

## Benefits

- retry-safe work
- crash recovery without model reconstruction
- canonical artifact/state remains separate from compact views
- stale workers cannot commit after lease transfer
- concurrent edits cannot silently clobber newer memory
- bounded manifest gives cheap discovery over deeper durable state.

## Risks / failure modes

- network filesystem cannot provide the same shared coordination guarantees
- artifact write succeeds but transactional outcome does not
- outcome commits but index/cursor does not
- stale worker writes after lease reassignment
- “same job” retry produces different bytes
- manifest refresh fails after editable topic write
- recovery scans become unbounded.

## Boundaries crossed

Worker → durable job state: lease required.
Worker → artifact store: immutable publish.
Artifact store → transaction: outcome/digest binding.
Transaction → derived index: replayable/rebuildable.
Model-visible memory → editable durable file: prior-read digest required.

## Agent-facing surface

The agent should see compact manifests/references and ordinary read/write tools. Lease, reconciliation, hashing and indexing remain invisible host machinery.

Memory should be labeled as historical context; live project state and external facts must be reverified before use.

## Determinization relevance

CRITICAL. This is exactly the kind of bookkeeping/recovery the model should never own.

## Likely architectural location

Evidence/Memory/Work-State layer under the MCI/runtime.

## Finding class

PATTERN + ALGORITHM + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. Job identity is deterministic for the same logical capture range.
2. Same committed job + same bytes is idempotent; different bytes conflict.
3. Stale lease owners cannot commit.
4. Canonical artifacts/outcome state precede derived index advancement.
5. Every supported partial crash state has deterministic reconciliation.
6. Recovery work is bounded.
7. Existing mutable durable files require a current read snapshot before replacement.
8. Derived indexes/manifests are rebuildable and never canonical truth.
