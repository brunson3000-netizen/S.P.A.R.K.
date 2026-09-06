# S.P.A.R.K. Gate C1 — V3-F01 Foundational Logical-Time Adjudication Report

**Date:** 2026-09-06
**Agent:** Claude Code (Fable 5.1), Gate C1 foundational architecture adjudicator
**Repository:** `/home/chromikey/Projects/SPARK`
**Branch:** `phase1-refoundation-v2`
**HEAD at entry:** `513c3982d5b9cd14f86ec07369662f3a178f1d95`
**Verdict:** `FOUNDATIONAL_TIME_ADJUDICATION_CANDIDATE_READY`

## 1. Executive summary

The Codex F-01 blocker is confirmed: the inherited record required catch-up partition
equivalence (R-038, ADR-0003 §15), bound wave-created work to "current logical time + 1"
(v2 §11), and permitted one drain per "canonical boundary" (v2 §11), without defining
whose logical time or which boundary. For a cohort that creates delayed work, no reading
of those words that treats the host catch-up call as the boundary satisfies all three.

One rule is chosen and justified: **horizon expansion with per-barrier canonical time.**
A host catch-up call is a horizon, not a time. It deterministically expands into one
canonical logical-time barrier per resident due time inside the horizon, including due
times created during the call. Every scheduled cohort evaluates with `now` equal to its
own `due_time`; every command cohort with `now` equal to its finalized `effective_time`.
Created work is eligible exactly when the global cohort sequence reaches its due time,
inside the same call if that lies within the horizon and the pacing budget permits.
"Canonical boundary" in v2 §11 means a logical-time barrier or command barrier, never a
host call. Command barriers never interleave inside a call's expansion; their placement
is canonical input history.

Under this rule the mandatory `C@100` / delayed `D` counterexample produces the same
`WorkKey`, cohort sequence, pre-wave digests, reports, and final state whether the host
advances to 101 in one call or in two. A prefix lemma and composition theorem derive
catch-up equivalence for work-producing cohorts without assuming equal created keys,
which was the missing induction premise in the rejected candidate's Theorem P.

No pacing value, cohort identity, conflict semantic, host authority boundary, Phase-1
behavior, or engine-digest composition changes. The rejected candidate is preserved
untouched. Codex findings F-02 through F-07 remain open for the next writer.

## 2. Verification gate

| Check | Result |
|---|---|
| Branch is `phase1-refoundation-v2` | pass |
| Convergence baseline `5fd556bfad958bda4439560cbfc5f4e537ce375a` in history | pass (ancestor of HEAD) |
| Rejected candidate `03daa82032cecc6ed84407b440bc9eab06cbcd69` in history | pass (ancestor of HEAD; sole parent `5fd556b`) |
| Codex review `SPARK_PHASE_2_CODEX_V3_F01_INDEPENDENT_REVIEW_2026-09-06.md` present | pass (committed at `513c398`) |
| Worktree clean at entry | pass (`git status --porcelain` empty) |
| Lineage recorded | `5fd556b` → `03daa82` → `513c398` |
| Material contradiction with mission | none found |

## 3. Material reviewed

Read in full or in the cited sections: the Codex V3-F01 independent review; the rejected
V3-F01 correction candidate, its acceptance matrix, and writer report; the Codex final
v3 confirmation §7; freeze v1 (Q2–Q7, §5, §6, §8), v2 (§5, §6, §11–§14, C-9, C-11), v3
(§3, §4, §7); acceptance matrices v2 (AT-I20–AT-I22) and v3 (AT-I20, AT-I21, AT-I39–
AT-I41); the Codex adversarial review §7.6 and v2 rereview §7.2; blueprint §18, §19.4,
§27.4, §28; ADR-0003 in full; the requirement coverage matrix rows R-037/R-038/R-041/
R-044; the Phase-0 correction report B-01; Phase-1 admission architecture §5.1;
`engineering/PHASE_STATUS.md`; the convergence protocol §5–§6; and Phase-1 source
`clock.rs`, `scheduler.rs` (`WorkKey`, `schedule`, `drain_due`), and `timeline.rs`
(`SemanticCommandEnvelope`).

## 4. Answers to the seven required resolutions

| # | Question | Resolution |
|---|---|---|
| 1 | One barrier at final time, or expansion? | **Expansion.** One host call expands into one canonical logical-time barrier per resident due time `≤ T`, including due times created during the call. Barriers carry no canonical identity; the cohort is the canonical unit. |
| 2 | Which `now` when evaluated work creates delayed work? | The **creating cohort's canonical time**: `due_time` for scheduled cohorts, `effective_time` for command cohorts. Never the horizon, never the clock frontier. |
| 3 | When is work created during catch-up eligible if due inside the horizon? | At its own barrier, **inside the same host call**, provided the pacing budget reaches it; otherwise it stays resident and byte-identical for the next call. Eligibility is a function of logical time and budget consumption only. |
| 4 | Command barriers versus expanded barriers? | Commands execute at their history position with `now = effective_time`, never inside a call's expansion, and never drain scheduled work. A command between stepwise calls is a distinct canonical history (v2 §6.2 retained). |
| 5 | Partition equivalence incl. work-producing cohorts? | Lemma 1 (least-cohort invariance), Lemma 2 (prefix), Theorem P′ (`Run(Run(S,T1),T2) = Run(S,T2)`), P′-pacing, P′-commands — candidate §8. Equality of created keys is derived from equal states and equal `now`, not assumed. |
| 6 | Compatibility with clock, scheduler, commit order, cross-platform? | `LogicalClock::advance_to` unchanged (frontier := `T`; equal-horizon calls resume); `drain_due` unchanged and unused by the evaluator; commit order per cohort unchanged; all arithmetic is checked `u64`; no canonical evaluation reads the clock. |
| 7 | Retained / clarified / superseded / inconsistent? | Candidate §9. Superseded: the host-call reading of v2 §11's second bullet and the rejected candidate's Lemma V. Clarified: v2 §11 first bullet, AT-I21 `now`, v1 Q3 `at`, v1 Q7 decay `now`, v1 §6 cooldown, v3 §3.1 sequence, v2 §6.2 "drain cycle", ADR-0003 §14 reading, `effective_time`. Everything else retained. |

## 5. Alternatives rejected

A1 final-time single barrier (violates ADR-0003 §15 / R-038 through chunk-dependent
`WorkKey.due_time`); A2 per-cohort time with call-pinned drain set (created work's
eligibility depends on call count); A3 the rejected candidate's recompute loop under the
per-cohort reading (mechanically equivalent to the chosen rule but contradicts v2 §11
literally and cannot state the proof); A5 forbid multi-tick catch-up (contradicts
blueprint §18.3 and R-038's own test); A6 execute in-horizon created work inside the
creating barrier (zero-delay recursion). Sub-decisions: pacing budget per host call
(latency purpose; semantics-neutral); command `now` = `effective_time` rather than the
clock frontier (avoids making the frontier R1 retained state).

## 6. Scope discipline observed

- No Rust, no tests, no Phase-1 source touched.
- The rejected candidate, its matrix, its writer report, and the Codex review are
  unmodified.
- No extraction API designed beyond the constraints in candidate §10 (explicit horizon,
  cohort-time `now`, recompute after each boundary, no clock read, checked arithmetic).
- No change to pacing values, executable-cohort identity, conflict semantics, host
  authority, or the engine-digest composition; the one pacing clarification (one budget
  per host call across the expansion) is stated explicitly, not silently.
- No supplementary external compute was used (candidate U-6).
- Writer/reviewer separation preserved: this document feeds a new Opus V3-F01 writer
  pass and a later independent Codex review; it decides nothing on their behalf.

## 7. Deliverables and commit

| Item | Value |
|---|---|
| Candidate | `engineering/phase2/SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md` |
| Report | `engineering/phase2/SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_REPORT_2026-09-06.md` |
| Status pointer | bounded subsection added to `engineering/PHASE_STATUS.md` under Phase 2 |
| Checks | `git diff --check` and a reference-existence check over every repository path cited by the two new documents (recorded in the commit-verification block below) |
| Parent commit | `513c3982d5b9cd14f86ec07369662f3a178f1d95` |
| Resulting commit | returned to the Operator with this report and verifiable as the sole child of the parent above (a document cannot carry its own hash without a rewrite) |
| Downloads copy | `~/Downloads/SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_REPORT_2026-09-06.md` (disposable transfer copy) |

## 8. Pre-commit worktree accounting and checks

At the end of the pass, before the single bounded commit:

```text
tracked, modified:   engineering/PHASE_STATUS.md   (+17 lines, one candidate-status subsection)
untracked, new:      engineering/phase2/SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md
                     engineering/phase2/SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_REPORT_2026-09-06.md
```

Checks run before the commit:

```text
git diff --check                                      PASS (no whitespace errors)
reference-existence over every `engineering/…` and
`crates/…` path cited by the three changed files      PASS (22 unique paths, 0 missing)
trailing whitespace / tabs / TODO markers             none
```

No other tracked file is modified, no Rust or test file is touched, no historical
artifact is edited, and no other untracked file is introduced. After the commit the
worktree is clean. The parent of the commit is
`513c3982d5b9cd14f86ec07369662f3a178f1d95`. The resulting hash, changed-file list, and
final `git status` are returned to the Operator in the handoff message. The
`~/Downloads` copy of this report is a disposable transfer copy; the repository copy is
canonical.

## 9. Unresolved risks (summary; full text in candidate §11)

U-1 `effective_time` monotonicity is a host-history property, not enforced. U-2 the
clock frontier remains outside the engine digest as in Phase 1; flagged, not decided.
U-3 long horizons under pacing need repeated equal-horizon calls; cost scales with
created work. U-4 ADR-0003 §14 "due/zero-delay evaluation" read as the command's own
evaluation. U-5 Codex F-02…F-07 remain open. U-6 no NIM panel was run.

## 10. Explicit nonclaims

Not claimed: V3-F01 closure; repair or acceptance of the rejected candidate; Phase-2
freeze or acceptance; Phase-2 implementation or Phase-3 authorization; Operator
acceptance; any Phase-1 change; any pacing/identity/conflict/authority change; any
engine-digest change; any Rust or test; cross-platform parity; any G.A.M.E. change.

## 11. Operator recommendation

Record this adjudication as additive Gate C1 history. Authorize the separated Opus HIGH
V3-F01 correction writer pass under candidate §10's constraints and the Codex §9 items
2–6, followed by independent Codex HIGH review. Do not release the Phase-2
implementation writer.
