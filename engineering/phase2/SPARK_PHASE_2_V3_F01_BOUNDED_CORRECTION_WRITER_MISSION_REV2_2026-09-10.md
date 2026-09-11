# S.P.A.R.K. V3-F01 — Bounded Correction Writer Mission, Revision 2

**Date:** 2026-09-10
**Status:** Canonical mission for the separated bounded writer pass; not architecture acceptance.
**Revision:** New mission after final independent review; the prior final writer mission remains historical and unchanged.

## Role, authority, and baseline

Act as the separated architecture correction writer using Claude Code Opus with high
reasoning. Codex is the independent reviewer and must not write this correction.

The Operator has authorized the bounded writer/reviewer cycle, candidate documentation,
disposable models/checks, preservation-safe branch creation, normal commit/push, and remote
verification. **No new Operator decision is required merely to run this bounded pass.**
Do not implement production Rust, merge into production, reset/discard work, force-push,
claim acceptance/freeze, open Phase-2 implementation or Phase 3, or modify G.A.M.E.

- Repository: `/home/chromikey/Projects/SPARK`.
- Remote: `https://github.com/brunson3000-netizen/S.P.A.R.K..git`.
- Production: `phase1-refoundation-v2`, expected baseline
  `e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8`.
- Reviewed candidate: `74d044ddd2b59c3c2eff558beeeeb515390dbe0f`, sole parent
  `772e38da0d130d7a1225ba2eb60a75b3999336fe`.
- Base your new candidate on the published `review/v3-f01-final-20260910` review package.
  Verify it descends from the exact candidate and contains this mission and the final
  independent review; record its exact hash in your report.
- Preferred new branch: `candidate/v3-f01-bounded-correction-rev2-20260910`.

Read repository instructions first. Fetch live state, inspect tips, ancestry, all worktrees
and existing branches. Preserve existing work and use a clearly suffixed branch if the
preferred name contains divergent work. Never force expected values onto live refs.

## Required reading

Read in full the final independent review
`engineering/phase2/SPARK_PHASE_2_CODEX_V3_F01_FINAL_INDEPENDENT_REVIEW_2026-09-10.md`,
its isolated evidence directory, the final candidate/oracle/writer report/model, the
ActiveRequest Operator decision, and prior serialized-boundary review. Follow the prior
writer mission's controlling-record reading list, particularly V2-03 through V2-10,
ADR-0003, Phase-1 timeline/scheduler/digests, PHASE_STATUS, and the convergence protocol.

## Required bounded corrections

1. **FINAL-01: mechanically atomic single-command finalization.** Specify a concrete
   operation, owner, input, complete validation/error table, exclusive access, transaction
   boundary, and publication point. Either preflight everything before live mutation or
   specify an isolated transaction with infallible publication; do not simply rename the
   existing stage-then-fence recipe “atomic.” Every refusal leaves the **entire timeline
   byte-identical**, including slots, poison evidence, all identity indexes, history,
   fences, epoch/window metadata, and frontier. Preserve inherited Phase-1 stage/fence
   semantics; any needed additive surface is a proposal only. Preserve positive staging
   acknowledgement before fencing on success and finalization/execution within one command
   barrier. Define empty-frontier and unexpected staged/poisoned-state handling, plus
   ordinal/window exhaustion. Do not add rollback of earlier committed scheduled work or
   prior waves. Preserve D-2's distinction between horizon completion and command success.
2. **Executable source-sequence regression.** Retain the review's unchanged-Phase-1
   probe: finalize S:10, empty frontier, distinct S:9 positively stages, fence rejects
   `SourceSequenceNotIncreasing`, slot remains. Extend the disposable correction model
   to represent source sequencing and staged/finalized state so the old composition fails
   and the proposed transaction passes. Compare full state before/after all refusal
   routes, not merely error tags or finalized-history length. Preserve the old probe as
   historical red evidence; do not “fix” production Rust to make it pass.
3. **FINAL-02: mismatch/FIFO handling.** A mismatch refuses the presented request and
   cannot acknowledge, pop, drop, replace, or lose the actual active FIFO head. Specify
   request-bound terminal dequeue eligibility and fail-closed restore/mailbox mismatch.
   Keep the exact durable request until completion. Test faulty presentation with the
   genuine active head still queued; assert unchanged queue/order and engine state, then
   successful exact-head continuation. Correct lifecycle, consumer contract, model
   terminal classification, and oracle consistently.
4. **FINAL-03: replay scope.** History-only replay is for completed request boundaries
   under explicit genesis/artifact/epoch/cap and timeline-metadata preconditions. Paused
   recovery requires the committed snapshot with stores, `F`, and `ActiveRequest`, plus
   the exact durable mailbox request. Prove two states can share finalized history and
   `F` while differing in paused progress; prove snapshot/exact-request continuation and
   substitution refusal. Remove the claim that any equal-or-greater-horizon request can
   replace a paused one. Do not imply a command digest reconstructs its payload.
5. Correct the review's bounded oracle precision issues: same-kind/same-horizon command
   substitution; honest redundant-field discrimination claims and encoding pins; equal
   residual stores versus different removed sets; explicit outer-loop selection counting;
   and unambiguous work-producing lifecycle fixtures and successive call outcomes.

## Preserve without reopening

Retain the authorized ActiveRequest owner, full identity, encoding, set-before-mutation,
exact resume, pause retention, snapshot validation, and atomic completion with `F`.
Keep both in `stable_boundary_digest` and out of `engine_state_digest`. Preserve all
accepted earlier V2 corrections: horizon expansion, cohort-local time, live least-slice
full-fingerprint comparison, atomic scheduler/obligation extraction, executable-only
identity and pacing, complete conflicted extraction/report ordering, later-wave commit
retention, observation/facade boundaries, and withdrawn Phi/X/C/SH-1/SH-2/R-8 proposals.
Preserve all historical artifacts and inherited tests. No new cancellation, abandonment,
transport, mailbox implementation, or broad scheduler surface.

## Deliver, validate, and publish

Create newly named `SPARK_` architecture candidate, oracle, and writer report, clearly
marked Revision 2 proposals superseding the prior proposals without rewriting them.
Add a bounded PHASE_STATUS update and the executable disposable model/evidence.
Record exact base/parent/candidate lineage, changed files, each finding's disposition,
validation and unavailable checks, limitations, and independent-review handoff.

Run the model, isolated probes, `git diff --check`, applicable repository tests and
format/lint checks. Record actual outcomes; do not present old results as fresh or Python
as production/crash-recovery proof. Require negative controls that distinguish the old
incorrect recipe from the proposed mechanism. Do not weaken inherited tests.

Commit only this bounded package, push the new candidate branch normally, and verify
local HEAD = tracking ref = live remote tip. Return
`CANDIDATE_READY_FOR_INDEPENDENT_REVIEW` only when complete, internally consistent,
validated, clean, and published; otherwise report the concrete blocker. Return the exact
candidate commit/parent/branch and route to independent Codex HIGH review. V3-F01 stays
OPEN; acceptance, freeze, and production implementation remain unclaimed.
