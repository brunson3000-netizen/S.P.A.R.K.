# S.P.A.R.K. Gate C2 — Phase-2 Implementation Mission (as received)

**Received:** 2026-09-11, from the Operator, by the separated implementation writer
(Claude Code, Opus 5). Recorded verbatim in substance; this file is the canonical mission
text for this writer pass and is not regenerated or paraphrased elsewhere.

## Mission

Execute S.P.A.R.K. Gate C2: implement the complete frozen Phase-2 architecture and
acceptance oracle end to end.

The Operator explicitly authorizes the entire Phase-2 implementation, its tests, checkpoint
commits, candidate publication, and independent-review handoff. Do not stop for routine
authorization during Phase 2. Stop only for a genuine unresolved architecture decision,
unsafe conflict, credentials problem, or required Operator action.

## Authority boundaries

- Phase-2 implementation: AUTHORIZED.
- Normal commits and pushes to the Phase-2 candidate branch: AUTHORIZED.
- Production promotion: not part of this writer pass; independent review comes first.
- Phase 3: UNAUTHORIZED.
- G.A.M.E. changes and integration: OUT OF SCOPE.
- Paid external compute: NOT AUTHORIZED by this mission.
- Fable's experimental research runtime remains noncanonical and must not be merged.

## Baseline and controlling records

- Repository: `/home/chromikey/Projects/SPARK`
- Frozen production baseline: `phase1-refoundation-v2` at
  `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5`
- Controlling acceptance:
  `engineering/phase2/SPARK_PHASE_2_V3_F01_OPERATOR_ACCEPTANCE_FREEZE_2026-09-10.md`
- Controlling independent review:
  `engineering/phase2/SPARK_PHASE_2_CODEX_V3_F01_REV2_INDEPENDENT_REVIEW_2026-09-10.md`
  at `e55b1da1c9049b1de58fcb06c65eea59939f2a57`
- Accepted Revision-2 candidate: `5ecc95c033bf3a7744bb7862bf959066e6561670`
- Development handoff: `engineering/SPARK_DEVELOPMENT_HANDOFF_2026-09-09.md`

Begin by reading all applicable repository instructions and the complete accepted Phase-2
stack and acceptance matrices in their recorded precedence. Fetch and verify live GitHub.
Preserve every existing worktree, branch, uncommitted file, and unrelated artifact.

Create an isolated implementation worktree and a new branch based exactly on live
production. Preferred branch: `candidate/phase2-gate-c2-implementation-20260911`. If it
already exists, inspect it and preserve divergent work rather than overwriting it.

Record this received mission as
`engineering/phase2/SPARK_PHASE_2_GATE_C2_IMPLEMENTATION_MISSION_2026-09-11.md`.
Then implement all frozen Phase-2 production Rust and executable acceptance tests. Work in
bounded, reviewable checkpoints, but continue automatically through the entire phase.

## Required implementation discipline

1. Convert every applicable frozen acceptance-oracle requirement into a meaningful
   red-first Rust test or compile probe. Record inherited-tree baselines before production
   changes. Do not weaken, delete, ignore, or rewrite a test merely to make it pass.
2. Preserve all Phase-0 and Phase-1 semantics, encodings, digests, identities, authority
   boundaries, ordering, and existing tests.
3. Implement the frozen rule/effect runtime, including validated artifacts, deterministic
   evaluation, wave processing, aggregation/conflict behavior, checked arithmetic, semantic
   caps, bounded provenance, obligations, rejection behavior, and canonical reports.
4. Implement deterministic cohort-granular scheduled-work processing: live least
   `(due_time, profile_id)` slice; full fingerprint compare-and-take; atomic
   scheduler/obligation extraction; conflicted versus executable membership; cohort-local
   evaluation time; live horizon expansion; budget-neutral canonical semantics; paced,
   unbudgeted, and catch-up equivalence at every cohort boundary.
5. Implement request-boundary state and processing: horizon frontier `F`; exact
   `ActiveRequest` discriminator; set before mutation; exact pause/resume and restart
   matching; mismatch refusal without mutation; stable-boundary digest placement;
   completion cleanup; request-bound results and dequeue eligibility.
6. Implement atomic single-command finalization: complete P-1 through P-9 preflight;
   clean-staging invariant; positive acknowledgment before one-ordinal fence;
   deterministic `fence.<decimal ordinal>` identity; byte-identical timeline on every
   ordinary refusal; sticky fail-stop `FINALIZATION_ENTAILMENT_VIOLATED` handling with no
   stable-boundary or snapshot publication.
7. Apply both controlling precision pins: same-frontier reset records replay in ascending
   `reset_index`; production P-7 through P-9 use bounded indexed lookup. Implement PX-1's
   three read-only Phase-1 accessors or an equivalent bounded private seam. Never scan
   ever-growing finalized history in production. Change no canonical encoding, admission
   semantics, identity, or mutation authority.
8. Implement the frozen snapshot/replay validation surface required by Phase 2:
   completed-boundary history reconstruction only under all declared inputs; paused
   recovery requires the complete committed snapshot and exact request; restore rejects
   invalid ActiveRequest/frontier combinations, digest mismatch, and timeline staging;
   derived private indexes are reconstructed or validated rather than trusted. Do not claim
   real durable-storage or process-crash recovery unless separately implemented and
   actually tested; those remain later gates.
9. Implement mandatory tests omitted from the disposable model: AT-I48(j), entailment
   fault injection and sticky fail-stop; AT-I49(b), completed non-head presentation cannot
   remove the actual FIFO head.
10. Enforce the frozen facade: no host-accessible mutable scheduler or timeline; no public
    staging, fencing, arbitrary removal, cancellation, ActiveRequest abandonment, or
    evaluation access to pacing/frontier/request state; test-support observation remains
    absent from production dependency edges and canonical reports.

Checkpoint the work by coherent dependency slices. Each checkpoint must compile, format,
pass its relevant tests, and leave no unexplained files. Continue until every Phase-2
acceptance entry is implemented or an exact, evidence-backed blocker is proven.

## Required validation before completion

`cargo fmt --all --check`; `cargo test --workspace --all-features`;
`cargo clippy --workspace --all-targets --all-features -- -D warnings`; the existing
stricter core/engine lint profile; `cargo metadata --format-version 1`; `git diff --check`;
applicable Windows and Android static target checks; all Phase-2 acceptance,
negative-control, digest, replay, pacing, facade, and inherited regression tests.

Run representative long-history and workload checks proving the indexed preflight does not
degrade with finalized-history length. Report measurements honestly; do not call toy or
static checks production benchmarks.

## Required deliverables

- `engineering/phase2/SPARK_PHASE_2_GATE_C2_IMPLEMENTATION_REPORT_2026-09-11.md`
- a bounded evidence directory under `engineering/phase2/`
- an exact independent Codex review mission for the completed implementation

Commit all authorized Phase-2 work in clear checkpoints. Push the candidate branch normally
and verify local HEAD, tracking ref, and live GitHub tip are identical. Do not merge or
advance production.

## Required receipt

Baseline; candidate branch and final commit; checkpoint commits; production files and tests
added or changed; total test results; lint, formatting, metadata, static-target, replay, and
workload results; acceptance-oracle coverage; honest limitations; worktree status;
local/tracking/live-remote equality; exact independent-review mission path; whether any
genuine Operator decision remains.

Do not stop after planning or the first implementation slice. Complete Gate C2's entire
writer pass.
