# S.P.A.R.K. — Development Handoff

**Recorded:** 2026-09-09; **updated:** 2026-09-10 after Operator architecture acceptance

## Current gate

V3-F01 is **CLOSED**. The Phase-2 architecture correction is **ACCEPTED and FROZEN**.
Candidate `5ecc95c033bf3a7744bb7862bf959066e6561670` and its complete supersession
stack are accepted under independent review `e55b1da1c9049b1de58fcb06c65eea59939f2a57`:

- `engineering/phase2/SPARK_PHASE_2_CODEX_V3_F01_REV2_INDEPENDENT_REVIEW_2026-09-10.md`
- `engineering/phase2/SPARK_PHASE_2_V3_F01_OPERATOR_ACCEPTANCE_FREEZE_2026-09-10.md`

The Operator explicitly authorized review/acceptance publication and architecture-only
production promotion. Phase 0 and Phase 1 remain closed and unchanged. Phase-2 production
implementation and Phase 3 remain **unauthorized**. The earlier staging/command-time
architecture decision is resolved by the accepted stack; it is no longer a pending Gate C1
Operator decision.

## Exact next authorization and task

The Operator must explicitly authorize **Gate C2: implement the frozen Phase-2 rule/effect
runtime and acceptance oracle, including V3-F01 and both precision pins**, preserving
writer/reviewer separation. After that authorization, implement only the frozen scope:
horizon expansion/cohort-local time; least-slice extraction across scheduler/obligations;
F and exact ActiveRequest lifecycle; preflighted finalization and clean staging;
request-bound dequeue/durability ordering; scoped replay and restore validation.

Controlling precision pins:

1. Multiple epoch-reset records at one frozen frontier replay in ascending `reset_index`.
2. Production P-7 through P-9 use bounded indexed lookup, preferably PX-1's three read-only
   accessors or an equivalent bounded private seam. No encoding, authority, identity or
   admission semantics change. Ever-growing-history scans are not a production option.

The retained acceptance matrix and FINAL/Revision-2 oracle deltas govern implementation.
Encode meaningful red-first tests, including entailment fault injection, consumer request
binding, canonical encoding pins, facade compile probes and paced/catch-up equivalence at
every cohort boundary; preserve all inherited Phase-1 tests and semantics.

## Evidence and future gates

The independent review records 58/58 model checks, disposable Rust probes with negative
controls, 2,496 additional differential cases, eight reset-order fixtures, 232 workspace
tests and passing formatting/lint/metadata/whitespace/five static target checks. These
establish architecture evidence and unchanged Phase-1 regression health.

Real process-crash recovery, durable snapshots/mailboxes, GAME integration, performance
evidence and executable Windows/Android replay/digest parity remain future gates. GAME's
transaction unit, late-input policy and atomic report application/acknowledgment identity
remain later integration decisions. Phase 3 requires separate authorization after Phase-2
acceptance. No joint cross-project readiness is claimed.

Fable architecture-challenge research at `03d36dd649641b28b232f4650b773413db634611`
remains noncanonical and unmerged. Its experimental runtime changes are excluded.
All existing worktrees and unrelated branches are preserved; production advances only
through the accepted architecture-only lineage. Exact publication and post-promotion
validation hashes/equality are supplied in the completion receipt.
