# S.P.A.R.K. Phase 0 Second Correction Report

**Date:** 2026-08-25  
**Correction package:** v0.3  
**Basis:** `PHASE_0_CODEX_INDEPENDENT_REREVIEW_2026-08-25.md`  
**Scope:** Close the sole remaining Phase-1 blocker B-01A plus the one residual matrix-status minor. No Rust implementation. No causal primitive added. No controlling-blueprint change.

## Independent re-review result

```text
VERDICT: REVISE_PHASE_0
B-01: OPEN
B-02: CLOSED
B-03: CLOSED
MAJORS: NONE
PHASE-1 AUTHORIZATION: NO
```

The reviewer found one remaining defect:

> Canonical ordinal ownership and stream finality were undefined.

The reviewer also identified one minor traceability defect: matrix R-059 used undefined status `CALIBRATE`.

## B-01A correction

### Frozen decision

Each profile timeline epoch now has exactly one active authoritative timeline sequencer.

External canonical input uses two distinct phases:

```text
STAGE -> FENCE/FINALIZE -> CANONICAL COMMAND BARRIERS
```

### Arrival-independence

Receiving an envelope cannot mutate canonical state.

Commands are staged by `(profile, timeline_epoch, input_ordinal)`.

The sequencer later supplies a hash-linked `TimelineFence` that commits to the complete ordered digest of a contiguous ordinal range.

Only a valid fence promotes commands into canonical history.

Therefore:

- `(n+1,n)` and `(n,n+1)` arrival can finalize the same stream;
- a gap cannot be silently skipped;
- same-ordinal conflicting payloads poison the slot and block finalization;
- first arrival does not win;
- a non-sequencer cannot issue canonical ordinals/fences;
- finalized history cannot be rewritten by a late conflict;
- service and embedded forms share the same logical mechanism.

### Why this is not a new runtime primitive

`TimelineFence` and `TimelineSequencerAuthority` are protocol/integration lifecycle constructs controlling canonical input admission. They do not add a new causal/world-state primitive to the frozen grammar.

## Files revised

- `ADR-0003-deterministic-clock-rng-and-commit-order.md`
- `ADR-0005-service-and-embedded-semantic-equivalence.md`
- `PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
- `PHASE_0_STATUS.md`
- `PHASE_0_GATE_REPORT.md`

## Matrix minor

R-059 now uses the defined status:

```text
PENDING_CALIBRATION
```

## Phase-1 status

**NO.**

A final independent re-review must close B-01A and confirm no regression of B-02/B-03 before Phase 1 is authorized.
