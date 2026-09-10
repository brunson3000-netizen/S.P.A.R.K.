# S.P.A.R.K. Gate C1 — V3-F01 Final Correction Writer Report

**Date:** 2026-09-10
**Agent:** Claude Code (Fable 5.1), separated V3-F01 final correction writer
**Repository:** `/home/chromikey/Projects/SPARK` (worktree
`/home/chromikey/Projects/SPARK-v3-f01-writer`)
**Branch:** `candidate/v3-f01-final-correction-20260910`
**Verdict:** `CANDIDATE_READY_FOR_INDEPENDENT_REVIEW`

## 1. Executive summary

The final bounded V3-F01 correction package exists. It carries forward, without
reopening, the foundation the serialized-boundary review accepted — serialized
one-head-at-a-time requests, horizon expansion, cohort-local `now = due_time`, live
least-slice compare-and-take with a full fingerprint, atomic engine-owned cross-store
extraction after complete preflight, executable-only identity and pacing, conflicted
extraction without identity or cost, commands finalized and executed together at their
request's completion, `F` updated only at completion, the two-digest structure, and the
withdrawal of `Phi`, `X`, `C`, and A-1 through A-3 — and closes SB-01 with the
Operator-authorized `ActiveRequest` discriminator.

`ActiveRequest` is defined completely: owner, fields, canonical encoding,
initialization, start, exact-resume match, mismatch refusal, pause, completion, both
start-time refusals, digest placement, snapshot and restore validation, observation,
and every transition, with the statement that no other transition exists. Exact-head
resumption is enforced at the engine boundary; the consumer contract is a liveness
convenience, not the safety mechanism.

The consolidated oracle corrects every V2-03 through V2-10 item and every false
assertion the independent reviews named, deletes the `Phi`, SH-1/SH-2, and R-8 rows,
rewrites AT-I43 as the request-lifecycle oracle, adds AT-I47 for the AT-G obligations of
`F` and `ActiveRequest`, and maps the mission's twelve required kills to named tests.

The disposable Python model was extended and run: 34 checks, all passing, including
every check of the 2026-09-06 model re-run under the new mechanism.

## 2. Model designation (recorded deviation)

The mission text names "Claude Code Opus at high reasoning" as the writer. This pass
was performed by Claude Code running **Fable 5.1**, the model the Operator launched for
the session. Following the project's provenance discipline (the v3 freeze
mis-attribution correction), the producing model is recorded truthfully in every new
document rather than copied from the mission text. No architecture decision depends on
which model wrote it; the independent reviewer may weigh this as they see fit. The
`PHASE_STATUS.md` constraint that Fable be used sparingly is noted; the session model was
not selected by this pass.

## 3. Exact baseline and lineage

| Item | Value |
|---|---|
| Live remote fetched at entry | `origin/phase1-refoundation-v2` = `e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8`; `origin/review/v3-f01-serialized-boundary-20260910` = `772e38da0d130d7a1225ba2eb60a75b3999336fe` |
| Required review commit | `bb1ccbb41f3a53948e3a87cd2b310e92ab1fa1d3` — verified ancestor of HEAD by `git merge-base --is-ancestor` |
| Candidate branch at entry | `candidate/v3-f01-final-correction-20260910` already existed in this worktree at `772e38d`, one commit ahead of `bb1ccbb` (the Operator decision and mission commit), clean, not yet on the remote; lineage and ownership unambiguous, so it was reused per the mission |
| Parent of this pass's commit | `772e38da0d130d7a1225ba2eb60a75b3999336fe` |
| Worktree at entry | clean (`git status --porcelain` empty) |
| Verified lineage | `5fd556b` → `03daa82` → `513c398` → `379f8dc` → `983a01f` → `8d0dba9` → `53a8432` → `30626ab` → `47729bb` → … → `e9e26e8` → `bb1ccbb` → `772e38d` |
| Ancestry checks | `53a8432`, `30626ab`, `bb1ccbb` each verified as ancestors of HEAD |

No reset, discard, stash, or force-push was performed. No uncommitted work existed to
preserve.

## 4. Material read in this pass

Read in full: the writer mission; the Operator `ActiveRequest` decision; the
serialized-boundary independent review; the serialized-request addendum, report, model
(`model.py`, `results.txt`), and evidence directory listing; the command-time
adjudication candidate and report; the V2 candidate, V2 matrix, V2 writer report, and V2
independent review; the foundational-time adjudication candidate and report; the first
V3-F01 candidate, matrix, writer report (§1–§3), and independent review; the final v3
confirmation; Phase-2 freezes v1, v2, v3 and matrices v1, v2, v3; ADR-0001 through
ADR-0006; blueprint §18, §19.3–§19.5, §28, §35 and its section index; the performance
and security budget §3; requirement rows R-037, R-038, R-041, R-044, R-096; the Codex
adversarial review §7.6 and v2 rereview §7.2; `engineering/PHASE_STATUS.md`; the
convergence protocol; the development handoff.

Phase-1 source inspected directly: `crates/spark-core/src/clock.rs` (full),
`crates/spark-core/src/scheduler.rs` (types, `schedule`, `drain_due`, observation,
`canonical_state_digest`), `crates/spark-core/src/timeline.rs` (module documentation,
`SemanticCommandEnvelope`, `SubmittedCommand`, the public `TimelineIngress` surface),
`crates/spark-core/src/hash.rs` (`CanonicalEncoder` API), `crates/spark-core/src/lib.rs`,
`crates/spark-engine/src/lib.rs`, `crates/spark-engine/src/state.rs` (public surface),
the workspace and three crate manifests, and
`crates/spark-testkit/tests/workspace_dependency_direction.rs`.

## 5. Deliverables and changed-file accounting

| File | Change |
|---|---|
| `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_FINAL_2026-09-10.md` | added (812 lines) |
| `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_ORACLE_V3_F01_FINAL_2026-09-10.md` | added (586 lines) |
| `engineering/phase2/SPARK_PHASE_2_V3_F01_FINAL_CORRECTION_WRITER_REPORT_2026-09-10.md` | added (this file) |
| `engineering/phase2/v3_f01_final_correction_model_2026-09-10/model.py` | added (disposable model) |
| `engineering/phase2/v3_f01_final_correction_model_2026-09-10/results.txt` | added (run output) |
| `engineering/phase2/v3_f01_final_correction_model_2026-09-10/results.json` | added (run output) |
| `engineering/PHASE_STATUS.md` | one bounded subsection added under Phase 2 (+18 lines) |

Preserved unchanged, byte for byte: every prior V3-F01 candidate, matrix, adjudication,
addendum, report, and review; the 2026-09-06 model directory; every Phase-0/1/2
historical artifact; all production Rust, manifests, and tests. Verified by
`git status` and the commit's file list (§8).

## 6. How the mission's required content is met

| Mission requirement | Where |
|---|---|
| Carry forward the binding architecture without reopening | candidate §3 (table of twelve items plus the adopted A4 rule), §4, §7, §8, §9, §10, §12, §14, §19 |
| Define `ActiveRequest` completely | candidate §6.1–§6.7 (owner, fields, encoding, initialization, start, exact-resume, mismatch refusal, pause, completion, rejection paths, digest, snapshot/restore, observation, "no other transition") |
| Not a prose mailbox promise as the only enforcement | candidate §5 Start row, §6.3, §17 item 4; oracle AT-I43(k) |
| Correct V2-03 … V2-10 | candidate §10 (V2-08), §11 (V2-05), §12 (V2-04, V2-10), §13 (V2-06); oracle §14.2 |
| Correct every false assertion named by the reviews | oracle §1 last row and §14.2 (V2 review §8 items 1–12 each mapped) |
| Red-first tests killing the twelve named wrong implementations | oracle §14.3 |
| Preserve all inherited tests | oracle §1 "UNCHANGED" rows; §2 row; discipline paragraph |
| Do not claim Rust tests ran | this report §7; oracle discipline paragraph; candidate nonclaims |
| Extend and run the Python model for `ActiveRequest`, substitution, restart matching, digest discrimination, cleanup | model S7–S10, S3e, S9, S9b; candidate §21 |
| `SPARK_`-prefixed filenames dated 2026-09-10; no rewrite of prior artifacts | §5 |
| Smallest necessary `PHASE_STATUS.md` update | one subsection |

Decisions the record left silent, each flagged in candidate §22: D-1 command ordinal
assigned at finalization as the timeline frontier; D-2 `F` advances on a finalization
refusal; D-3 all-conflicted slices consumed after the oversized exception (V2-10
resolved by loop change, alternative recorded); D-4 `Advance` identity is `H(T)`; D-5
restore validation order.

## 7. Verification performed

| Check | Result |
|---|---|
| `python3 model.py results.json` in `v3_f01_final_correction_model_2026-09-10/` | `ALL PASS`, 34 checks, 0 failures (`results.txt`) |
| Every check of the 2026-09-06 model re-run inside the new model (S1–S6, S3b, S3c, S3d, S4b) | pass |
| 2026-09-06 model directory unchanged | verified by `git status` (no modification) |
| `git diff --check` over the full change set | pass (recorded in §8) |
| Reference existence for every `engineering/…` and `crates/…` path cited by the new documents | pass, 0 missing (§8) |
| Cross-document companion filenames | pass |
| Trailing whitespace, tabs, TODO/TBD/FIXME markers in new documents | none |
| Historical documents unmodified; no Rust, manifest, or test file touched | verified by the commit's file list |
| Rust / Cargo tests | **not run and not claimed**: this pass writes no Rust; the 232-test baseline recorded at the September 9 reconciliation is not presented as fresh evidence |
| Supplementary compute | **not used** in this pass; the conclusions are direct deductions from the pinned record, the live Phase-1 source, and the model |

## 8. Commit and remote verification

Recorded after the commit and push, in the completion receipt returned to the Operator
(a document cannot contain its own commit hash). The receipt states: candidate commit,
parent `772e38da0d130d7a1225ba2eb60a75b3999336fe`, branch, changed files, the check
outcomes above, the remote tip of `candidate/v3-f01-final-correction-20260910` as
verified by `git ls-remote`, and the unresolved matters below.

## 9. Residual nonclaims and unresolved matters

Not claimed: V3-F01 closure; acceptance of this or any prior candidate; enactment of the
v1 §8 amendment beyond the Operator's authorization of its text; Phase-2 architecture
freeze; Phase-2 production Rust or Phase-3 authorization; any Phase-1 change; any
G.A.M.E. change; any executed Rust test; cross-platform parity; that the model is
production or performance evidence.

Unresolved (candidate §22): no `ActiveRequest` abandonment path exists by design and a
deployment needing one requires a separately authorized decision; Phase-3 read-only
exposure of `F` / `ActiveRequest` and mailbox capacity remain non-normative;
cohort-identity non-injectivity over slices remains a stated consequence; no adversarial
review of this pass exists.

## 10. Independent-review handoff

Route to independent Codex HIGH review, at minimum covering: the `ActiveRequest`
definition and whether any transition or failure path is missing (candidate §6.3, §6.5);
the ordering of the mismatch check before the frontier check (§9 P0); decisions D-1
through D-5 (§22); the engine-owned extraction's preflight completeness for conflicted
claim sets (§11); the wave-index rejection rule (§13); the post-exception consumption of
all-conflicted slices (§9, D-3); the exact boundaries of the equivalence claims (§16);
and whether every **Kills** line in the oracle discriminates its named wrong
implementation. Do not release the Phase-2 implementation writer. Implementation and
Phase 3 remain unauthorized pending that review.
