# S.P.A.R.K. Gate C1 — Serialized Request Boundary Report

**Date:** 2026-09-06
**Agent:** Claude Code (Fable 5.1), Gate C1 foundational architecture adjudicator
**Repository / branch:** `/home/chromikey/Projects/SPARK` / `phase1-refoundation-v2`
**HEAD at entry:** `53a8432b9f0f3bad99c273d4ff7b52f4c63b5340` (command-time candidate)
**Verdict:** `SERIALIZED_REQUEST_BOUNDARY_ADDENDUM_READY`

## 1. Summary

The command-time candidate is reconciled with the Operator's serialized-request
direction. ALT-U's unified pending-command loop, the execution cursor `X`, the finalized
time ceiling `C`, and amendments A-1…A-3 are withdrawn. In their place: a bounded
mailbox feeds one consumer; one request is active until its defined horizon completes;
budget exhaustion pauses it and no later request may execute; a command is staged,
fenced, and executed atomically at its request's completion after catch-up to its
effective time; a request may start only if its horizon is at least `F`, the horizon of
the last completed request.

The engine holds **no active-request state** at any stable boundary; the mailbox head
is the active request and the consumer owns it. The only added canonical state is the
scalar `F`. The digest contradiction the Operator identified is resolved by keeping `F`
out of the per-cohort `engine_state_digest` (so pre-wave and batch digests stay
partition-invariant) and committing it in a `stable_boundary_digest` used by snapshots,
restore, and equal-state tests (invariant at equal completed horizon). One v1 §8
amendment remains, with exact wording, proposed and not enacted.

A disposable executable model demonstrates every required scenario, eleven checks, all
passing, including a negative control showing precisely where including `F` in the
per-cohort digest would break catch-up equality.

## 2. Verification and worktree accounting

| Check | Result |
|---|---|
| Branch / HEAD at entry | `phase1-refoundation-v2` / `53a8432…`, the adjudication commit named by the Operator |
| Lineage | `…379f8dc → 983a01f → 8d0dba9 → 53a8432` |
| Worktree at entry | **not clean**: three committed documents carried uncommitted edits correcting their supplementary-compute wording (`…WRITER_REPORT_V2…`, `…COMMAND_TIME_ADJUDICATION_CANDIDATE…`, `…COMMAND_TIME_ADJUDICATION_REPORT…`) |
| Handling | those edits are unrelated to this mission; they were **left unstaged and untouched** and are not in this commit; the committed versions remain history |
| This commit | only new files plus one bounded `PHASE_STATUS.md` subsection |

## 3. The six points

| # | Resolution | Where |
|---|---|---|
| 1 | Accept (mailbox, non-canonical, backpressure when full) → Start (`process(head)`, `h ≥ F` else `HORIZON_BEHIND_FRONTIER`) → Process → Pause (`PAUSED`, `F` unchanged, head stays) → Resume (same head) → Complete (catch-up done; command staged+fenced+executed atomically; `F := h`) → Next only after `COMPLETED` or rejection | addendum §1 |
| 2 | Minimum active-request state in the engine: **none**; the consumer owns the head. Replay = timeline plus `F`; restart = stores plus `F` with the head re-presented. No messaging platform. | addendum §2, model S5 |
| 3 | `h ≥ F` at request start; `F` set only at completion; time-ordered processing inside a request. Past-dated commands never reach staging; `now ≥ updated_at` by construction. | addendum §3, model S4/S4b/S6 |
| 4 | `X` removed; `C` dropped; `F` kept as the single demonstrably required scalar. | addendum §4 |
| 5 | `engine_state_digest` unchanged and excluding `F` (per-cohort, partition-invariant); `stable_boundary_digest = H("stable_boundary_v1" ‖ engine_state_digest ‖ F)` for snapshots and equal-state tests (invariant at equal completed horizon). | addendum §5, model S3/S3b/S3c/S3d |
| 6 | A-1, A-2, A-3 disappear. One v1 §8 amendment remains: the `HorizonFrontier` row and the `stable_boundary_digest` definition, exact wording in addendum §6. No ADR amendment. | addendum §6 |

## 4. Model

`engineering/phase2/serialized_request_model_2026-09-06/model.py` — Python, standard
library only, separate from production crates, architecture evidence only. Scenarios:
two requests with the second arriving during a pause (S1); budgets 1 and 2 (S2); one
catch-up versus five completed catch-ups with a work-producing cohort landing inside
the horizon (S3, S3b, S3c, S3d); past-dated input (S4) and review §5.3 equal-digest
engines (S4b); restart from stores plus `F` with the head re-presented (S5); the
non-negative-elapsed guard never trips (S6). Output: `results.txt`, `results.json`.
Run: `python3 model.py results.json`. Result at commit: `ALL PASS`, eleven checks.

## 5. Supplementary compute

Used once, headless, per `/home/chromikey/AI/tools/dev_compute/README.md`; no GUI.
One `falsification` worker at fast depth (`openai/gpt-oss-20b`, 52.5 s, request
`mci-dev-277d0d84ec7e`, panel `COMPLETE`) was asked to construct counterexamples for
pacing causality, backwards evaluation, restart divergence, and equal-digest divergence.
It constructed none and named the assumptions each attack needed — deterministic slice
order with no state change on pause; `F` updated only at completion; snapshot capturing
pre-pause work; `F` in the stable-boundary digest — which are exactly the addendum's
invariants and the model's assertions. Credential-free records are preserved under
`engineering/phase2/serialized_request_model_2026-09-06/dev_compute_evidence/`. This is
non-authoritative and decided nothing.

## 6. Retained and carried forward

All reviewed scheduled-processing and compare-and-take successes; the V2 review's
V2-03…V2-10 corrections via the command-time candidate §10 checklist, with the
`X`-related rows deleted and AT-I43 recast as the request-lifecycle oracle.

## 7. Nonclaims

Not claimed: V3-F01 closure; acceptance or freeze; enactment of the v1 §8 amendment;
production implementation or the mailbox transport; Phase-2 implementation or Phase-3
authorization; any Phase-1 change; any G.A.M.E. change; that the model is production
or performance evidence.

## 8. Recommendation

Decide the single v1 §8 amendment. Then authorize the separated V3-F01 writer pass on
the addendum plus the carried-forward checklist, followed by independent Codex review.
