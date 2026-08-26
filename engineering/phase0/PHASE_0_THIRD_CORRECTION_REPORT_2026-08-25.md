# S.P.A.R.K. Phase 0 Third Correction Report

**Date:** 2026-08-25  
**Correction package:** v0.4  
**Basis:** `PHASE_0_CODEX_FINAL_REREVIEW_2026-08-25.md`  
**Scope:** Close the sole remaining bounded-staging admission defect and the matrix formatting minor. No Rust. No causal primitive. No blueprint change.

## Independent review result

```text
VERDICT: REVISE_PHASE_0
B-01A: OPEN (bounded staging overflow only)
B-02: CLOSED
B-03: CLOSED
MAJORS: NONE
PHASE-1 AUTHORIZATION: NO
```

## Remaining defect

A generic finite staging buffer that rejects whichever request arrives after capacity is reached permits higher ordinals to occupy all capacity before the frontier arrives.

## Frozen correction

Canonical staging is now an **ordinal credit window**, not a FIFO/generic queue.

At frontier `n` with width `W`, ordinals:

```text
n ... n+W-1
```

each have one dedicated logical slot.

No eligible higher ordinal can consume the frontier slot.

Every command carries the admission-window token under which the sequencer submitted it. An ordinal outside that token's range returns `NOT_IN_ADMISSION_WINDOW`; it is not opportunistically accepted later merely because network delay changes physical arrival time.

The sequencer retries such a command under a later valid token after the frontier advances.

There is no arrival-based eviction.

Before a fence may be sent, the sequencer must receive positive STAGED acknowledgements for the entire fenced range.

A poisoned slot is not repaired by winner selection; recovery requires explicit timeline-epoch reset from the unchanged finalized frontier.

## Capacity-two required result

With frontier `n` and width two, both:

```text
n+1, n+2, n
n, n+1, n+2
```

resolve logically as:

```text
n     STAGED
n+1   STAGED
n+2   NOT_IN_ADMISSION_WINDOW
```

Then the same fence through `n+1` finalizes identically.

## Matrix minor

The blank line separating R-101 through R-105 from the main Markdown table has been removed. New R-106 through R-108 explicitly cover ordinal-window capacity, retry, and fence acknowledgement.

## Phase 1

**NOT AUTHORIZED pending independent closure review.**
