# S.P.A.R.K. Phase 0 Gate Report — Third Corrected Writer Pass v0.4

**Date:** 2026-08-25  
**Current gate:** BOUNDED ADMISSION CORRECTION COMPLETE / INDEPENDENT CLOSURE REVIEW REQUIRED  
**Phase 1 authorization:** **NO**

## Sole reviewed blocker before v0.4

Arrival-dependent staging-capacity overflow.

## v0.4 correction

- deterministic frontier-derived admission window;
- one dedicated slot per ordinal;
- frontier capacity can never be consumed by a higher ordinal;
- envelope binds to the admission-window token under which it was submitted;
- out-of-window command is retryable and consumes no canonical staging capacity;
- no arrival-based eviction;
- sequencer must receive STAGED acknowledgement for full fence range before fencing;
- poisoned staging recovery requires explicit timeline-epoch reset;
- service/embedded/concurrent paths share the same logical contract.

## Closure condition

Authorize Phase 1 only if independent review returns:

```text
B-01A = CLOSED
B-02 = CLOSED
B-03 = CLOSED
REMAINING BLOCKERS = NONE
PHASE-1 AUTHORIZATION = YES
```
