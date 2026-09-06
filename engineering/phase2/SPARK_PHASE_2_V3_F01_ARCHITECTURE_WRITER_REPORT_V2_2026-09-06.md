# S.P.A.R.K. Gate C1 — V3-F01 Correction Writer Report (Second Pass)

**Date:** 2026-09-06
**Agent:** Claude Code (Opus 5), Gate C1 architecture correction writer
**Repository:** `/home/chromikey/Projects/SPARK`
**Branch:** `phase1-refoundation-v2`
**Verdict:** `CANDIDATE_READY_FOR_INDEPENDENT_REVIEW`

## 1. Executive summary

The separated V3-F01 correction candidate now exists, built on Fable's foundational time
adjudication as its **proposed** design basis, and resolves the independent review's seven
findings together rather than one at a time.

The owned selection token is gone. Scheduled work is consumed through a least-due
**compare-and-take** whose target is fixed by the operation, never named by the caller,
guarded by an explicit eligibility **horizon** and by a fingerprint that binds key
identity, slot state, payload content, and the complete tracked conflict claim set. That
one change closes the replay, non-least-selection, time-of-check-to-time-of-use, and
unconstructible-not-due defects at once, and it does so by conceding what the first pass
denied: the comparison value is retainable and forgeable, and it is safe precisely because
it carries no authority.

Conflicted slots return to their inherited place. They are extracted atomically with their
operational slice so none lingers to pollute a later cohort's pre-wave digest, but they
are members of neither the executable cohort identity nor the pacing budget. That restores
the v1 budget over `DrainOutcome::due` and reverses the first pass's D-3 and D-4.

The new canonical report fields are withdrawn. Pre-wave digest observation moves to the
`test-support` seam that already exists in this workspace and is already protected by a
passing feature-hygiene test, so the oracle gains its observation point without widening
any production surface.

Partition equivalence is now stated with preconditions and with the distinction the
mission required: completed-horizon equivalence when a segment is drained, equal-prefix
equivalence under differing budgets, and an explicit refusal to claim equal state at equal
call index when different call counts supply different total budget.

One item the adjudication deliberately left open had to be closed to make the design
implementable, and it is closed as a bounded guard rather than a new foundational ruling.
Command effective-time ordering is stated as two supported-history conditions, sourced to
the existing Gate C3 contract item "monotonic logical execution point", detected and
refused fail-closed at the command-execution door in the shape v1 §5 already uses for a
finalized obligation that must refuse. The ADR-0003 staging question is explicitly left
undecided.

No remaining contradiction in the adjudication was found. Where it was silent rather than
wrong, §4.5 of the candidate says so and bounds the addition.

## 2. Exact baseline and lineage

| Item | Value |
|---|---|
| Branch verified live | `phase1-refoundation-v2` |
| HEAD at entry (this commit's parent) | `379f8dc355125e6bca09801e4a66b0b35ec9720c` |
| Worktree at entry | clean (`git status --porcelain` empty) |
| **Foundational adjudication commit** | **`379f8dc355125e6bca09801e4a66b0b35ec9720c`** |
| Adjudication parent | `513c3982d5b9cd14f86ec07369662f3a178f1d95` |
| Independent review commit | `513c3982d5b9cd14f86ec07369662f3a178f1d95` |
| Rejected first-pass candidate commit | `03daa82032cecc6ed84407b440bc9eab06cbcd69` |
| Convergence baseline | `5fd556bfad958bda4439560cbfc5f4e537ce375a` |
| Verified lineage | `5fd556b` → `03daa82` → `513c398` → `379f8dc` |
| Ancestry check | all four verified as ancestor-or-HEAD by `git merge-base --is-ancestor` |

The adjudication report omitted its own commit hash, as the mission noted. That hash is
`379f8dc355125e6bca09801e4a66b0b35ec9720c`, and it is now recorded in the candidate §1,
in this report, and in the `PHASE_STATUS.md` pointer. The adjudication documents
themselves were **not** edited to insert it; recording it in new documents avoids
rewriting existing history.

An isolated workspace was not required: the worktree was clean at entry, no unrelated
tracked change existed at any point, and this pass touched only new files plus one bounded
`PHASE_STATUS.md` insertion.

## 3. Material read in this pass

Project instructions and `engineering/PHASE_STATUS.md`;
`engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` §5–§7; the foundational adjudication
candidate and its report; the independent Codex V3-F01 review in full; the rejected
correction candidate and its acceptance matrix in full, plus the rejected writer report's
accounting sections; the Codex final v3 confirmation §7; freezes v1 (§3, Q2–Q7, §5, §6,
§8), v2 (§5, §6, §7, §8, §11, §12, §13, C-9, C-11), v3 (§3, §4, §7); acceptance matrices
v2 (AT-I20–AT-I22) and v3 (AT-I20, AT-I21, AT-I39–AT-I41); the Codex adversarial review
§7.6 and v2 rereview §7.2; blueprint §18, §19.4, §27.4, §28; ADR-0003 in full;
requirement rows R-037/R-038/R-041/R-044; the Phase-0 correction report B-01; and the
Phase-1 admission architecture §5.1.

Phase-1 source inspected directly: `crates/spark-core/src/clock.rs` (`LogicalTime`,
`LogicalClock::advance_to`); `crates/spark-core/src/scheduler.rs` (`WorkKey` field order
and `identity_digest`, `WorkPayload`, `DueWorkItem`, `SlotState`, `WorkSlotStatus`,
`ConflictEvidence` with `MAX_CONFLICT_EVIDENCE` and `MAX_CONFLICT_TRACKED_CLAIMS`,
`WorkKeyConflict`, `DrainOutcome`, `schedule`, `drain_due`, `slot_status`, `conflict_of`,
`canonical_state_digest`); `crates/spark-core/src/timeline.rs`
(`SemanticCommandEnvelope::effective_time`); and the `test-support` feature declarations
in all three crate manifests together with `spark-testkit`'s hygiene test inventory.

## 4. How each finding was resolved

| Finding | Resolution | Where |
|---|---|---|
| F-01 | Adopt horizon expansion and cohort-local evaluation time; rebuild the proofs with explicit preconditions and a prefix induction that derives created keys; withdraw Lemma V and Theorem P | candidate §4, §10 |
| F-02 | Delete the token; least-due compare-and-take; comparison value explicitly non-authorizing and conceded retainable | candidate §5, §5.6 |
| F-03 | Explicit horizon argument; `NoSliceDue` carrying the live least resident due time; horizon separated from evaluation time | candidate §5.4, §11 item 7 |
| F-04 | Operational slice versus executable cohort; identity and budget over `Scheduled` only; D-3/D-4 withdrawn | candidate §6 |
| F-05 | Every review §5 counterexample and §7 gap encoded as a named test with an explicit **Kills** line | matrix §3–§13 |
| F-06 | CE-8 withdrawn; observation through the existing `test-support` seam and its passing hygiene test | candidate §9, matrix §12 |
| F-07 | Exact borrow statement claiming only what non-lexical lifetimes provide; structural-non-retention overclaim withdrawn | candidate §5.5 |

## 5. Mission items beyond the seven findings

1. **Horizon expansion, cohort-local time, eligibility, command placement** — candidate
   §4.1–§4.4, loop §7, matrix AT-I39/AT-I43.
2. **Equivalence preconditions; completed-horizon versus equal-prefix** — candidate §10.1
   (PE-A…PE-D), §10.3 with the explicit non-claim that equal call counts imply equal
   state; matrix AT-I39(D) encodes the non-claim as a negative control.
3. **Command effective-time ordering and scheduling into traversed time** — candidate
   §4.5: SH-1 and SH-2, sourced to Gate C3's "monotonic logical execution point",
   enforced by a typed atomic refusal at the execution door, with the ADR-0003 staging
   question left open and the decay/cooldown consequence given as the evidence; matrix
   AT-I43(e)(f)(g)(h).
4. **Compare-and-take replacing the token; horizon separated from evaluation time** —
   candidate §5; matrix AT-I42.
5. **Executable-cohort identity and pacing preserved; conflicted extracted but not
   budgeted** — candidate §6; matrix AT-I40(e), AT-I45.
6. **Internal/test-only observation** — candidate §9; matrix AT-I46.
7. **Rejection classes, post-extraction state, store cleanup, retry** — candidate §8
   (eight classes R-1…R-8, with the clarification that no rollback mechanism exists);
   matrix AT-I44.
8. **Borrow wording and supersession map** — candidate §5.5 and §12 (three sub-maps:
   against the rejected pass, against v1/v2/v3 and the adjudication, and five new
   consistency edits CE-7′…CE-11′).

### 5.1 Adjudication §10 downstream constraints, item by item

| Constraint | Satisfied by |
|---|---|
| 1 explicit horizon; "not due" = least resident `> T` | X-1/X-3 signatures; `NoSliceDue`; AT-I42(f) |
| 2 evaluation `now` is the cohort's; horizon unreachable from evaluation | §4.1, §11 item 7; AT-I32, AT-I43(d) |
| 3 recompute after every stable boundary; no plan or cursor | loop §7 A2; §10.6; AT-I29 |
| 4 pre-wave capture with exactly the current cohort extracted | §7 A4; AT-I39(B) |
| 5 least-live-due compare-and-take; guard does not grant selection | §5 X-3; AT-I42(c) |
| 6 iterate resident due times, not ticks | §4.2; AT-I39(C) |
| 7 checked delayed-time arithmetic | §4.3, R-3; AT-I43(i) |
| 8 one budget per host call; diagnostics per call; no new fields | §7 A1/A6; §5 X-4; AT-I45(d) |
| 9 command `now` = `effective_time`; no scheduled drain; no interleaving | §4.4; AT-I43(a)(b)(c) |
| 10 no clock read in canonical evaluation | §4.1, §11 item 4; AT-I32 |
| 11 restate proofs in the Lemma 1 / Lemma 2 / P′ form | §10.2–§10.3 |
| 12 matrix obligations (work-producing catch-up, multi-profile pacing cut, effect `at` / decay / cooldown equality, command placement, constructible not-due, overflow) | AT-I39(A)(D)(E), AT-I40(a)(d), AT-I43(b)(d)(i), AT-I42(f) |

Constraint 9's instruction not to add an `effective_time` admission rule in this pass was
weighed against mission item 3, which requires the supported-history conditions to be
stated and enforced or sourced. The candidate adds **no ingress admission rule**, which is
what the adjudication reserved; it adds an execution-door refusal, sources the condition
to an existing contract, and records the divergence and its reasoning openly in candidate
§4.5 and §14 item 2 so the reviewer can overrule it.

## 6. Deliverables and changed files

| File | Change |
|---|---|
| `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_V2_2026-09-06.md` | added (780 lines) |
| `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_V2_2026-09-06.md` | added (529 lines) |
| `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_V2_2026-09-06.md` | added (this file) |
| `engineering/PHASE_STATUS.md` | one bounded candidate-status subsection added under Phase 2 |

Preserved unchanged, byte for byte: the rejected correction candidate, the rejected
acceptance matrix, the rejected first-pass writer report, the independent Codex review,
the foundational adjudication candidate and report, every Phase-0/1/2 historical artifact,
and all production Rust and tests.

## 7. Validation performed

| Check | Result |
|---|---|
| `git diff --check` on the full change set | pass, no whitespace errors |
| Reference existence for every `engineering/…` and `crates/…` path cited by the four changed files | pass, 0 missing |
| Cross-document reference existence (companion filenames) | pass |
| Trailing whitespace, tabs, TODO/TBD/FIXME markers | none |
| Rejected and historical documents unmodified | verified by `git status` and by the commit's file list |
| No Rust or test file touched | verified by the commit's file list |
| Commit parent, changed files, final worktree | recorded in §9 |

No test was executed and none is claimed to have been executed; no production Rust exists
to run the specified oracle against.

## 8. Unresolved risks

1. **The foundational basis is itself under review.** The candidate adopts the
   adjudication as a proposal. Rejecting horizon expansion or cohort-local evaluation time
   invalidates this candidate rather than requiring a patch.
2. **SH-1/SH-2 enforcement point** (candidate §4.5, §14 item 2). The execution-door
   refusal is a bounded choice; ingress rejection under ADR-0003 §5 remains open and is
   explicitly not decided. A reviewer may prefer the other point, both, or a different
   definition of the canonical time frontier.
3. **`Φ` reconstructibility** (candidate §14 item 5). If the canonical time frontier is
   not derivable from committed state, it is retained state and must enter the v1 §8
   engine digest under R1. Matrix AT-I29(d) is written to fail if the candidate's claim is
   wrong.
4. **Adjudication U-2 remains open.** Two engines with equal engine digests but different
   clock frontiers answer a backward `advance` differently. Pre-existing Phase-1/v1 §8
   classification; neither introduced nor resolved here.
5. **Cohort-identity non-injectivity** (candidate §6). A deliberate, stated consequence of
   restoring executable-only identity; a reviewer wanting slice-injective identity must
   reopen v3 §4.1 explicitly.
6. **Engine-internal visibility.** ADR-0001's dependency direction forces X-1…X-4 to be
   `pub` on `spark-core`. The prohibition on host exposure is contractual and
   compile-probed, not a language-level guarantee. This is stated plainly rather than
   claimed away.
7. **No adversarial review of this pass exists.** Writer/reviewer separation is intact.

## 9. Supplementary compute

One bounded attempt was made under existing authorization. The presumed harness command
at `~/.local/bin/swarm-mci-dev` was invoked with a 25-second bound and did not return; the
process was terminated by the bound and **no request was submitted**, so no partial worker
evidence exists beyond this record. The Operator subsequently observed that the command
opened the S.W.A.R.M. MCI desktop GUI. That observation is preserved: the command was the
MCI desktop launcher, not the headless caller surface, and it does **not** establish
Development Compute gateway unavailability. The writer failed only to locate a documented
headless invocation in its bounded S.P.A.R.K. search and correctly stopped troubleshooting
instead of expanding scope. S.W.A.R.M. subsequently documented the verified headless
invocation as
`.venv/bin/python -m tools.dev_compute.orchestrate ...` in
`tools/dev_compute/README.md`. This follow-up creates no S.P.A.R.K. runtime dependency.

This candidate therefore rests on the frozen record, the independent review, the
foundational adjudication, and direct inspection of the Phase-1 source. Supplementary
compute holds no project authority in any case.

## 10. Commit accounting

Pre-commit worktree:

```text
tracked, modified:   engineering/PHASE_STATUS.md
untracked, new:      engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_V2_2026-09-06.md
                     engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_V2_2026-09-06.md
                     engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_V2_2026-09-06.md
```

The commit's parent is `379f8dc355125e6bca09801e4a66b0b35ec9720c`. Its resulting hash,
changed-file list, and the final `git status` are verified after the commit and returned
to the Operator in the completion message. The `~/Downloads` copies of this report and the
handoff archive are disposable transfer copies; the repository copies are canonical.

## 11. Explicit nonclaims

Not claimed: V3-F01 closure; acceptance of the foundational adjudication; repair or
acceptance of the rejected candidate; Phase-2 architecture freeze or acceptance; Phase-2
implementation or Phase-3 authorization; Operator acceptance; any Phase-1 change; any
change to pacing values, host authority, conflict semantics, or the engine-digest
composition; any executed test; cross-platform runtime or digest parity; any G.A.M.E.
change.

## 12. Verdict and recommendation

`CANDIDATE_READY_FOR_INDEPENDENT_REVIEW`

Route the candidate and its matrix to an independent Codex HIGH review covering, at
minimum: the adopted time basis and its preconditions; the compare-and-take authority
argument, especially the conceded replay case; the executable-versus-operational split and
its identity non-injectivity; the SH-1/SH-2 guard and its enforcement point; the eight
rejection classes; and whether the matrix's **Kills** lines actually discriminate. Do not
release the Phase-2 implementation writer.
