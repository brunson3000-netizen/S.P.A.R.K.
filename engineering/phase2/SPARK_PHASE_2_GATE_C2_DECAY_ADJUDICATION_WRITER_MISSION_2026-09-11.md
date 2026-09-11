# S.P.A.R.K. Gate C2 — Decay Adjudication Writer Mission (as received)

**Received:** 2026-09-11, from the Operator, by the separated Gate C2 writer (Claude Code,
Opus 5, `claude-opus-5`). This is the canonical mission text for this writer pass. It is
recorded before any implementation adjustment, together with the Operator adjudication it
implements. The writer is not the independent reviewer.

## Controlling records

| Item | Value |
|---|---|
| Controlling review branch | `review/phase2-gate-c2-decay-oracle-revision-independent-20260911` |
| Controlling review commit (preceding evidence commit) | `6b167d84f21fb60f374a2ab45c62d5c6a040790a` |
| Reviewed candidate | `6968a4af917c9a32761be371c3e12ec12ba0f1bd` |
| Operator decision | `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md` |
| New candidate branch | `candidate/phase2-gate-c2-decay-adjudication-20260911`, created directly from `6b167d8` |
| Worktree | `/home/chromikey/Projects/SPARK-gate-c2-adjudication` (fresh, isolated) |

Live references were fetched with pruning. The review commit is the live head of its branch.
The reviewed candidate is its parent.

## Operator decision (as received)

Accept fixed-grid decay as the controlling Phase-2 architecture. This explicitly
supersedes the conflicting Q7/v2 duration language:

- Decay cadence follows a deterministic fixed grid.
- Ordinary non-decay writes do not restart or re-phase that grid.
- An activation that changes rate or cadence closes the old segment, discards its
  unfinished residual interval, and starts a new grid at the activation barrier.
- A step ending exactly at activation belongs to the closing segment.
- New parameters never charge time before their activation.
- Every decay evaluation — including unmoved, saturated, zero-rate, or no-whole-step
  evaluations — commits at the cohort's canonical time.
- Previously corrected nonretroactivity and saturation behavior remains controlling.

## Required work

1. Record the decision in a new immutable adjudication document before any implementation
   adjustment. Do not rewrite historical records. Identify precisely which Q7, v2 §4.3,
   AT-I22 and AT-I23 language is superseded or clarified.
2. Audit the existing implementation against the decision. Make only necessary corrections.
3. Update the oracle and tests so that the following vectors discriminate the accepted
   policy: fresh-assignment, post-shock, activation-boundary, residual-discard,
   shortening, lengthening, saturation, recovery, replay and chunking.
4. Remove or update assertions that enforce the rejected elapsed-since-write
   interpretation, and document why.
5. Run the complete suite: Gate C2 validation, preservation, probes, mutants, static
   targets, strict lint and the workload.
6. Produce a decision implementation report, bounded evidence and a canonical final Codex
   HIGH independent-review mission.

## Authority and limits

The Operator has authorized Phase 2 end-to-end. Commit and push normally, then verify
local = tracking = live remote. Do not promote production. Do not begin Phase 3 during this
writer pass. Do not force-push, reset or discard pre-existing work. Preserve every existing
branch and worktree. Use no paid compute.

## Completion message

It states:
- the candidate branch and exact commit;
- the lineage and the decision record;
- code changes, if any;
- test totals and validation results;
- limitations;
- synchronization status;
- the final independent-review mission path.
