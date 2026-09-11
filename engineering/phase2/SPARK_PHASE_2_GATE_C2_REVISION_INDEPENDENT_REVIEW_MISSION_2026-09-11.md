# S.P.A.R.K. Gate C2 — Independent Review Mission for the Bounded Revision (Codex HIGH)

**Issued:** 2026-09-11, by the separated correction writer (Claude Code, Opus 5), under the
Operator's Phase-2 authorization. The reviewer is independent of the writer and must not
repair the candidate in the reviewer role.

## Exact scope

| Item | Value |
|---|---|
| Candidate branch | `candidate/phase2-gate-c2-bounded-revision-20260911` |
| Candidate commit | the exact hash in the writer's publication receipt (a commit cannot record its own hash); its code-and-test tree is checkpoint 2 `19930f3` — the final commit changes documentation and evidence only |
| Direct base | `00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0` (the controlling Gate C2 independent review) |
| Reviewed original candidate | `053d1dc1131ec47be94b60513fad9ea8389cde0c` |
| Production baseline | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` (`phase1-refoundation-v2`) |
| Writer mission | `engineering/phase2/SPARK_PHASE_2_GATE_C2_BOUNDED_REVISION_MISSION_2026-09-11.md` |
| Writer report | `engineering/phase2/SPARK_PHASE_2_GATE_C2_BOUNDED_REVISION_REPORT_2026-09-11.md` |
| Writer evidence | `engineering/phase2/gate_c2_bounded_revision_evidence_2026-09-11/` |
| Prior findings | `engineering/phase2/SPARK_PHASE_2_CODEX_GATE_C2_INDEPENDENT_REVIEW_2026-09-11.md` |

Verify every hash against live GitHub before starting. Work in a fresh isolated worktree
based exactly on the candidate commit. Preserve every existing branch and worktree.

## Authority

Authorized: review, fresh validation, disposable probes, a review document and bounded
evidence, a normal commit and push on a new review branch (preferred
`review/phase2-gate-c2-revision-independent-20260911`), live-ref verification. Not
authorized: repairing the candidate, production promotion, Phase 3, G.A.M.E. changes,
merging Fable's research runtime, force-push, reset, paid external compute.

## Precedence

As the prior review: acceptance freeze and Revision-2 independent review/pins, Revision-2
architecture/oracle, FINAL architecture/oracle, retained V2 sections, freezes and matrices
v1 → v2 → v3, the ActiveRequest decision, ADR-0001 … ADR-0006. Withdrawn Phi, SH-1/SH-2 and
R-8 stay withdrawn.

## What to decide

For each prior finding, whether the correction is complete and frozen-conformant, with an
executed discriminator:

| Prior finding | Writer's claimed correction (verify, do not assume) |
|---|---|
| C2-01 | Derived parent set = union over the watched group and every condition-read group changed in the wave (`engine.rs` `plan_wave`, `RuleSpec::condition_cell_reads`) |
| C2-02 | Fan-out = distinct (target, declared scope mapping) pairs + delay edges; each `Materialize` expansion bounded separately |
| C2-03 | Nested materialized sub-IDs claim the rule's single namespace, qualification checked |
| C2-04 | Aggregate, all-additive and staged rule-body arithmetic in checked `i128`, one final conversion |
| C2-05 | Baseline declared in the rule-set artifact, materialized into `StateCell.baseline` once, decay targets that field, door rejects decay without declared baseline; commit at the last whole step; epoch-by-epoch integration over a retained, restore-validated lineage |
| C2-06 | Reserved kind ↔ activation payload bound both ways; door rejects rules triggered by the reserved kind |
| C2-07 | `cohort_identity` removed from `CohortReport`; `command_deferred` removed (derivable); every partial/revision row completed in `phase2_oracle_completion.rs`; overstated claims corrected in the report |
| C2-08 | `FinalizationRefusal` variants `#[non_exhaustive]`; external construction probes |
| D-C2-7 | Exact originating-artifact resolution plus current-epoch identity; explicit compatibility rule with barrier-scoped config |
| D-C2-11 | Extraction refusal inside `process` → typed sticky `StoreInvariantViolated` fail-stop |
| D-C2-13 | Materialized emissions use the creator rule fingerprint (new record field) |
| C2-09 | Unchanged note; limitation restated |

Scrutinize especially the writer's **flagged interpretations** (report §7): the
baseline-declaration channel and once-only materialization (R-1), the decay step/epoch
integration rule (R-2), the lineage as a restore-validated derived index outside the frozen
digest composition (R-3), the D-C2-7 compatibility rule (R-4), the new fail-stop class
(R-5), the added record field (R-6), removal of the report field against v3 §3.4's content
list by FINAL precedence (R-7), enqueue-claim canonicalization (R-8), and the additive
spark-core X-5 accessor (R-9). Reject any that silently narrows or replaces a frozen
requirement; say which frozen text controls.

Also verify: the converted counterexamples genuinely kill the review's named wrong
implementations; the original corpus result (unchanged run fails to compile only on the
three corrected surfaces; adapted run 9/9) and its adaptations A1–A4; the new negative
controls; the corrected red-first statement; that no inherited test, manifest, lockfile,
inline Phase-1 test module or protected Phase-1 function changed; and that the per-entry
oracle table in the report does not overstate coverage.

## Required fresh validation

`cargo fmt --all --check`; `cargo test --workspace --all-features`; all-target/all-feature
clippy with warnings denied; the strict core/engine lint; `cargo metadata`; candidate-range
and production-range `git diff --check`; the five Windows/Android static target checks; the
release workload; the prior review's original counterexample corpus against the candidate;
independent probes of your own for each correction.

## Deliverables

A review document with a verdict (`GATE_C2_REVISION_ACCEPTABLE_FOR_OPERATOR_DECISION` or a
bounded-revision/blocking verdict), per-finding and per-entry dispositions, bounded
evidence, and a publication receipt with the review commit hash and local/tracking/live
synchronization. No acceptance or promotion follows from the review itself.
