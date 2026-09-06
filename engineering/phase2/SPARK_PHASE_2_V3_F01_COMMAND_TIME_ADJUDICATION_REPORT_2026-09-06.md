# S.P.A.R.K. Gate C1 — V3-F01 Command-Time Adjudication Report

**Date:** 2026-09-06
**Agent:** Claude Code (Fable 5.1), Gate C1 foundational architecture adjudicator
**Repository / branch:** `/home/chromikey/Projects/SPARK` / `phase1-refoundation-v2`
**HEAD at entry:** `8d0dba92845d9a0dde08dca7c25ec161ca85822e` (independent V2 review)
**Reviewed candidate:** `983a01fd6807b8c627c97d2673a1f40c54cc059c` (sole parent of HEAD)
**Verdict:** `COMMAND_TIME_ADJUDICATION_CANDIDATE_READY`

## 1. Summary

The independent V2 review was right on both counts it returned to the foundational
path. A frontier that tracks *progress* is pacing-causal, and a refusal keyed to it
violates the retained-state law. The V2 `Φ`, SH-1/SH-2, R-8, and the `due_time ≤ Φ`
defense are withdrawn in full.

The chosen rule is unified time-ordered processing. Finalized commands become pending
inputs at their `effective_time`. The existing catch-up call consumes scheduled slices
and pending commands in one ascending canonical-time order under one budget, so a
command is reached only after every slice with an earlier time, in every budget. The
value that gates whether a command may enter history is `max(F, C)`: `F` the host
horizon set by `advance`, `C` the largest effective time already finalized. Both are host
inputs, so admission is pacing-neutral by construction. Failure is a non-canonical,
retryable staging or fence result in the ADR-0003 §6 shape; there is no canonical
command refusal at all.

Two scalar state items are added, each forced by a demonstrated counterexample: `F`
enters the engine digest (review §5.3), and a per-timeline execution cursor `X` makes
the finalized-but-unexecuted set explicit (review §5.4 needs a pending command to
survive across calls). Both are defined completely: owner, type, initialization, update
on every path, digest commitment, snapshot/restore, and equal-state behavior. Nothing
else time-valued is retained. Global canonical time is non-decreasing by construction,
so backwards decay and cooldown evaluation are unreachable without any check.

Both mandatory counterexamples are traced (candidate §6). In §5.4 the sequence is
`A, B, cmd@21` at budget 1 and budget 2; only call count differs. In §5.3 both engines
return the same non-canonical admission result because the deciding value `F` is equal
and committed; no stored distinction between "consumed at 10" and "consumed at 20"
exists or is needed.

Three Operator amendments are proposed and not enacted: `F` and `X` into the v1 §8
engine digest, and an engine-level effective-time admission rule as an ADR-0003
addendum. Gate C3 wording is not cited as authority.

## 2. Verification gate

| Check | Result |
|---|---|
| Branch | `phase1-refoundation-v2` |
| HEAD at entry | `8d0dba92845d9a0dde08dca7c25ec161ca85822e` |
| Reviewed candidate `983a01f…` in history | pass; sole parent of HEAD |
| Adjudication `379f8dc…`, review `513c398…`, first pass `03daa82…`, baseline `5fd556b…` | all ancestors |
| Worktree at entry | clean |
| Material contradiction with mission | none |

## 3. Resolutions

| Question | Resolution |
|---|---|
| 1. Command after paced, incomplete scheduled processing | The command is a pending input at `effective_time`; the loop reaches it only after every scheduled slice with time `< e` (and `= e` for its profile). Unreached means deferred: nothing emitted, `X` unchanged. Budget decides only which call reaches it. |
| 2. Constraint on effective times | Admission into history requires `effective_time ≥ max(F, C)`, checked at staging and at fence at the engine boundary; failure is retryable and non-mutating. Finalized effective times are therefore non-decreasing in ordinal order, and no cohort ever evaluates earlier than a committed time. |
| 3. Additional canonical time state | Yes, exactly `F` (horizon; Phase-1 `LogicalClock`, now digest-committed) and `X` (execution cursor per profile timeline). `C` is a derived index of the timeline digest. Every dependency on `Φ` is removed. |

Temporary deferral and canonical refusal are distinguished: deferral is "not yet
reached"; the only rejections are non-canonical (`BackwardClockAdvance`,
`EFFECTIVE_TIME_BEHIND_FRONTIER`); the only canonical refusal touching commands is the
pre-existing v1 §5 obligation execution refusal.

## 4. Alternatives rejected

ALT-R (V2 refusal with `Φ`): pacing-causal, uncommitted. ALT-N (execute immediately):
command observes budget-dependent state. ALT-I (unbudgeted implicit drain): violates
blueprint §19.4. ALT-D (separate execute operation with budgeted drain and a
non-canonical deferral result): viable but larger. ALT-T (host advances as timeline
commands): would force a per-profile time model; out of scope. ALT-L (canonical
per-command refusal with a pending list): larger state and out-of-ordinal barriers.

## 5. Deliverables and validation

| File | Change |
|---|---|
| `engineering/phase2/SPARK_PHASE_2_V3_F01_COMMAND_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md` | added |
| `engineering/phase2/SPARK_PHASE_2_V3_F01_COMMAND_TIME_ADJUDICATION_REPORT_2026-09-06.md` | added (this file) |
| `engineering/PHASE_STATUS.md` | one bounded candidate-status subsection |

Validation before commit: `git diff --check` pass; every repository path cited by the
three changed files exists; no trailing whitespace, tabs, or markers; the V2 candidate,
its matrix, all reviews, the adjudication, and all of `crates/` unchanged. No Rust, no
tests, no test execution. The commit's parent is `8d0dba92845d9a0dde08dca7c25ec161ca85822e`;
its hash is in the handoff manifest and the completion message.

## 6. Supplementary compute

**Observed:** two bounded probes of `~/.local/bin/swarm-mci-dev --help` (25 s;
20 s with stdin closed) did not return, and the Operator reported mid-mission
that the presumed harness command opened the S.W.A.R.M. MCI desktop GUI. That
observation is preserved: the command is the desktop launcher and was not the
intended headless caller surface. No compute request was submitted.

**Not established:** the desktop launch did not prove that the Development
Compute gateway was unavailable.

**Bounded search result:** this S.P.A.R.K. engineering pass did not locate a
documented headless invocation within its bounded repository/evidence search
and correctly stopped troubleshooting rather than expanding scope. Its
architectural conclusions therefore did not rely on supplementary compute.

**Follow-up:** S.W.A.R.M. now documents the recovered, previously verified
headless development invocation as
`.venv/bin/python -m tools.dev_compute.orchestrate ...` in its canonical
`tools/dev_compute/README.md`. This evidence correction adds no S.W.A.R.M.
runtime dependency to S.P.A.R.K. and does not change S.P.A.R.K. architecture.

## 7. Unresolved decisions

Per-profile timelines versus one global horizon (ALT-T recorded); the stuck-staged-
command remedy (ADR-0003 §11 epoch reset versus a non-canonical guard on `advance`); the
equal-time tie rule (scheduled before command); and amendments A-1…A-3, which gate
implementability and are the Operator's to decide.

## 8. Nonclaims

Not claimed: V3-F01 closure; acceptance of any prior candidate; Phase-2 freeze or
acceptance; Phase-2 implementation or Phase-3 authorization; Operator acceptance;
enactment of A-1…A-3; any Phase-1 change; any executed test; cross-platform parity; any
G.A.M.E. change.

## 9. Recommendation

Decide A-1…A-3. Then authorize the separated V3-F01 writer pass under candidate §10's
checklist and the review's §11 items 4–6, followed by independent Codex review. Do not
release the Phase-2 implementation writer.
