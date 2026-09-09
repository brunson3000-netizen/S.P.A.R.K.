# S.P.A.R.K. — Development Handoff

**Recorded:** 2026-09-09

## Next task

The next implementation task, after authorization, is the bounded V3-F01 production
correction: add deterministic cohort-granular due-work extraction so each admitted
`(due_time, profile_id)` cohort is removed before its pre-wave digest while deferred work
remains unchanged.

## Prerequisites

- Independent review and acceptance of the current V3-F01 candidate and acceptance matrix.
- Architecture freeze and explicit Operator authorization for Phase-2 implementation.
- A passing paced/catch-up equivalence fixture at each cohort boundary, with existing
  Phase-1 identity, ordering, encoding, and digest tests still green.
- The Fable research closeout remains non-authoritative; its experimental runtime changes
  are not part of this task.

## Decisions still needed

- Operator disposition of the remaining V3-F01/ADR-0003 staging question and any proposed
  command-time admission rule before implementation is authorized.
- For later GAME integration: the transaction unit and late-input policy, durable snapshot
  contract, and atomic report-application/acknowledgment identity.

Repository synchronization does not establish an architecture gate, production
implementation authorization, or real process-crash recovery evidence. Those remain open.
