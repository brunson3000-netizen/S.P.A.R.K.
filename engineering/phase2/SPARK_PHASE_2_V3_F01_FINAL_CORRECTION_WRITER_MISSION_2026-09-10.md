# S.P.A.R.K. V3-F01 — Final Correction Writer Mission

## Role and authority

Act as the separated V3-F01 architecture correction writer. Use Claude Code Opus at high
reasoning. The independent reviewer is Codex and is not the writer.

The Operator authorized `ActiveRequest` on 2026-09-10. You are authorized to create the
bounded candidate branch, write and revise architecture/oracle documents, run applicable
non-production models and repository checks, commit the candidate, push the candidate
branch, and provide the exact commit for independent review. Do not merge, modify
production Rust, open Phase 2 implementation, open Phase 3, or modify G.A.M.E.

## Baseline

- Repository: `/home/chromikey/Projects/SPARK`
- Production branch: `phase1-refoundation-v2`
- Required remote review branch:
  `review/v3-f01-serialized-boundary-20260910`
- Required review commit or a verified direct descendant:
  `bb1ccbb41f3a53948e3a87cd2b310e92ab1fa1d3`
- Create candidate branch:
  `candidate/v3-f01-final-correction-20260910`

Fetch live remote state first. Preserve uncommitted work. If the named candidate branch
or its intended worktree already exists, inspect and reuse it only when its lineage and
ownership are unambiguous; otherwise stop with one precise report. Never reset, discard,
or force-push.

## Read before writing

Read the complete controlling record, not excerpts:

1. `engineering/phase2/SPARK_PHASE_2_V3_F01_ACTIVE_REQUEST_OPERATOR_DECISION_2026-09-10.md`
2. `engineering/phase2/SPARK_PHASE_2_CODEX_V3_F01_SERIALIZED_BOUNDARY_INDEPENDENT_REVIEW_2026-09-10.md`
3. the serialized-request addendum, report, model, and model outputs;
4. the command-time adjudication and report, especially its V2-03 through V2-10
   downstream checklist;
5. the V3-F01 V2 candidate, matrix, writer report, and independent review;
6. the foundational-time adjudication and first V3-F01 review cycle;
7. Phase-2 freezes v1 through v3, their matrices, and the final v3 confirmation;
8. Phase-0 blueprint and ADR-0001 through ADR-0006;
9. live Phase-1 scheduler, clock, timeline, engine state, manifests, and tests;
10. `engineering/PHASE_STATUS.md`, the convergence protocol, and development handoff.

## Deliverables

Create a final bounded correction package that supersedes the earlier V3-F01 candidates
as a proposal while preserving them unchanged as history:

1. one consolidated architecture correction candidate;
2. one consolidated acceptance-test oracle;
3. one writer report with exact lineage, changed-file accounting, verification, residual
   nonclaims, and the exact independent-review handoff;
4. the smallest necessary status update in `engineering/PHASE_STATUS.md`.

Use `SPARK_`-prefixed filenames dated `2026-09-10`. Do not rewrite prior review or
candidate artifacts.

## Binding architecture

Carry forward without reopening:

- serialized one-head-at-a-time request processing;
- horizon expansion over resident due times;
- cohort-local `now = due_time`;
- live least-slice recomputation and full-fingerprint compare-and-take;
- atomic engine-owned cross-store extraction after complete preflight;
- executable-only cohort identity and pacing;
- conflicted-slot extraction/reporting without executable identity or pacing cost;
- commands finalized and executed together only after their horizon drains;
- `F` updated only at request completion;
- `engine_state_digest` excluding request-boundary protocol state;
- `stable_boundary_digest` including `F` and the authorized `ActiveRequest`;
- the withdrawal of `Phi`, execution cursor `X`, separate ceiling `C`, and command-time
  amendments A-1 through A-3;
- no new host, service, transport, cancellation, arbitrary-removal, or batch-scheduling
  surface.

Define `ActiveRequest` completely: owner, fields, canonical encoding, initialization,
start, exact-resume match, mismatch refusal, pause, completion, rejection, digest,
snapshot/restore, and every failure path. Do not use a prose mailbox promise as the only
enforcement.

## Required oracle

Correct every V2-03 through V2-10 item and every false assertion named by the independent
reviews. Include red-first tests that necessarily kill:

- whole-prefix draining and pre-extraction digest capture;
- horizon-time evaluation and call-entry due-work plans;
- nonleast or stale-fingerprint extraction;
- non-atomic scheduler/obligation-store extraction;
- conflicted work entering executable identity or pacing;
- whole-cohort rollback after a later-wave rejection;
- paused-request substitution, including a lower-horizon command after later work was
  already processed;
- restart with the exact request versus a same-horizon different request;
- omission of `F` or `ActiveRequest` from `stable_boundary_digest`;
- inclusion of either in per-cohort `engine_state_digest`;
- active-request residue after completion or start-time refusal;
- budget-dependent canonical cohort or command results.

Preserve all inherited tests. Do not claim Rust tests ran because this pass writes no
Rust. Run the disposable Python model after extending it to cover `ActiveRequest`, the
substitution counterexample, restart matching, digest discrimination, and cleanup.

## Completion condition

Return `CANDIDATE_READY_FOR_INDEPENDENT_REVIEW` only if the package is internally
consistent, the model and document checks pass, the candidate branch is clean and
published, and the remote tip is verified. Otherwise return the exact blocker without
claiming V3-F01 closure.

The completion receipt must include the candidate commit, parent, branch, changed files,
checks and outcomes, unresolved matters, and a direct statement that implementation and
Phase 3 remain unauthorized pending independent review.
