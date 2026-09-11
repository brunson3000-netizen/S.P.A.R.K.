# S.P.A.R.K. Gate C2 — Second Bounded-Correction Writer Mission (as received)

**Received:** 2026-09-11, from the Operator, by the separated correction writer (Claude
Code, Opus 5, `claude-opus-5`). This is the canonical mission text for this writer pass,
recorded before any code change. The writer is not the independent reviewer.

## Authority

The Operator has authorized Phase 2 end-to-end: routine implementation, testing,
documentation, commits, normal pushes, and live verification, without further
authorization requests.

Not authorized: production promotion, Phase 3, G.A.M.E. changes, force-push, reset,
discarding work, paid compute.

## Controlling records

| Item | Value |
|---|---|
| Controlling review branch | `review/phase2-gate-c2-revision-independent-20260911` |
| Controlling review commit (preceding evidence commit) | `3a0b51463e1874ad8b02dd3a3261933fcd2e22f4` |
| Review document | `engineering/phase2/SPARK_PHASE_2_CODEX_GATE_C2_REVISION_INDEPENDENT_REVIEW_2026-09-11.md` |
| Review evidence | `engineering/phase2/gate_c2_revision_independent_review_evidence_2026-09-11/` |
| Reviewed candidate | `5b7fcf50161a65dc83b4806b513f01c0a4943ea5` |
| Production baseline (`phase1-refoundation-v2`) | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` |
| New candidate branch | `candidate/phase2-gate-c2-decay-oracle-revision-20260911`, created directly from `3a0b514` |
| Worktree | `/home/chromikey/Projects/SPARK-gate-c2-rev2` (fresh, isolated) |

The live references were fetched with pruning and verified: local, tracking, and live GitHub
refs agree on the review commit; the reviewed candidate and production are ancestors of it.

The published Codex review is the **controlling defect and oracle specification**. Its
findings are not weakened, paraphrased away, or selectively ignored. Frozen-source
precedence is the review's §1: Operator acceptance freeze and Revision-2 independent
review/pins; Revision-2 architecture/oracle; FINAL architecture/oracle; retained V2
sections; freezes and matrices v1 → v2 → v3 with corrected Opus provenance; the
ActiveRequest decision; ADR-0001 … ADR-0006.

## Required work

1. Preserve every accepted correction and interpretation (review §3–§5: C2-01, -02, -03,
   -04, -06, -08 closed; D-C2-5, -7, -11, -13 revised and accepted; D-C2-1, -2, -4, -6,
   -8, -9, -10, -12, -14, -15 preserved; R-1, R-3 … R-9 accepted; R-2 rejected).
2. Correct **C2R-01 … C2R-04**:

   | ID | Required correction (review §2) |
   |---|---|
   | C2R-01 | The decay remainder representation must not change frozen commit time: FINAL §4 makes every inherited `now`/`at` the cohort's canonical time. Reconcile the v1 Q7 remainder and chunk obligations with that cohort-time commit contract; merely restoring `commit_at = now` is not a repair. |
   | C2R-02 | New rates or cadence must never apply before their activation barrier — cadence shortening, lengthening, residual intervals — with recovery and replay controls. |
   | C2R-03 | Saturated decay, zero-rate transitions, chunking, replay, and effect reapplication must satisfy the frozen invariance requirements (Q7, AT-I22), including value **and** commit time. |
   | C2R-04 | Complete the applicable AT-I8, AT-I20b, AT-I21, AT-I22, AT-I23, AT-I28, AT-I33, AT-I39, and AT-I40 gaps identified by the review. |

3. Add meaningful negative controls that kill the named wrong implementations.
4. Keep accepted scope limitations honest: no claim of migration, multi-profile runtime,
   durable recovery, or platform runtime parity that was not implemented.

## Stop rule

If no representation can satisfy both the frozen remainder contract and the frozen
cohort-time commit contract, preserve all unaffected completed work and stop with one
precise architecture conflict for the Operator. Do not silently change the architecture.

## Discipline

- No historical checkpoint or inherited test is rewritten to manufacture red-first
  provenance; no forbidden production API is added merely to simplify a probe.
- The 232 inherited Phase-1 tests stay byte-identical. A Gate C2 writer test may be adapted
  only where a corrected surface requires it, never weakened; each adaptation is listed in
  the report with its reason.
- Where frozen text leaves a representation open, choose one explicitly, justify it from
  the frozen text, and flag it for review.

## Required validation (fresh)

The complete Gate C2 validation suite: `cargo fmt --all --check`;
`cargo test --workspace --all-features --no-fail-fast`;
`cargo clippy --workspace --all-targets --all-features -- -D warnings`; the strict
core/engine lint; `cargo metadata --format-version 1`; candidate-range `git diff --check`;
the five Windows/Android static target checks; the release workload; preservation checks;
all inherited tests; the independent probes (both reviews' corpora); mutation controls.

## Deliverables

- `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_ORACLE_REVISION_REPORT_2026-09-11.md`
- `engineering/phase2/gate_c2_decay_oracle_revision_evidence_2026-09-11/` (bounded
  evidence: exact commands and outputs)
- a new canonical Codex HIGH independent-review mission,
  `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_ORACLE_REVISION_INDEPENDENT_REVIEW_MISSION_2026-09-11.md`

Commit coherent checkpoints; push the candidate branch normally; verify local HEAD =
tracking ref = live remote ref; leave every existing branch and worktree intact.

The completion message states: the exact branch, commit, lineage, checkpoints, the
disposition of C2R-01 … C2R-04 and of every remaining oracle row, validation totals,
limitations, synchronization status, and the independent-review mission path.
