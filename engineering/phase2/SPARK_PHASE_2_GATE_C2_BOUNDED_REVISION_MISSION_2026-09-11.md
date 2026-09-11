# S.P.A.R.K. Gate C2 — Bounded-Revision Writer Mission (as received)

**Received:** 2026-09-11, from the Operator, by the separated correction writer (Claude
Code, Opus 5). This is the canonical mission text for this writer pass. The writer is
not the independent reviewer.

## Authority

The Operator authorized all Phase-2 work: routine branch creation, isolated worktrees,
implementation, tests, documentation, commits, normal pushes, and live-remote
verification, without repeated authorization. Stop only for a genuine unresolved
architecture decision or a task the Operator must personally perform.

Not authorized: production promotion, Phase 3, G.A.M.E. changes, merging Fable's
research runtime, force-push, reset, discarding work, paid external compute.

## Controlling records

| Item | Value |
|---|---|
| Controlling review branch | `review/phase2-gate-c2-independent-20260911` |
| Controlling review commit (preceding evidence commit) | `00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0` |
| Review document | `engineering/phase2/SPARK_PHASE_2_CODEX_GATE_C2_INDEPENDENT_REVIEW_2026-09-11.md` |
| Review evidence | `engineering/phase2/gate_c2_independent_review_evidence_2026-09-11/` |
| Original candidate | `053d1dc1131ec47be94b60513fad9ea8389cde0c` |
| Production baseline | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` |
| New candidate branch | `candidate/phase2-gate-c2-bounded-revision-20260911`, based exactly on `00d647e` |
| Worktree | `/home/chromikey/Projects/SPARK-gate-c2-rev` |

The published Codex review is the **complete controlling defect specification**. Its
findings are not weakened, paraphrased away, or selectively ignored. Frozen-source
precedence is the review's: acceptance freeze and Revision-2 review/pins, Revision-2
architecture/oracle, FINAL architecture/oracle, retained V2 sections, freezes and matrices
v1 → v2 → v3, the ActiveRequest decision, ADR-0001 … ADR-0006.

## Required corrections (from the review, by ID)

| ID | Required correction |
|---|---|
| C2-01 | Derived parent sets are the complete canonical union of every changed group read for eligibility (trigger and condition reads), preserving empty-background exclusion, transitivity, deduplication. Multi-target, unchanged-background, multi-wave fixtures. |
| C2-02 | Static fan-out counts the declared scope-mapping expansion (distinct target × scope breadth), including delayed materialized effects per their execution bound; exact-bound and over-bound scope fixtures. |
| C2-03 | Every nested materialized emitting operation claims a unique, qualified sub-ID in the rule's frozen namespace at the door; unequal- and equal-payload negative controls. |
| C2-04 | Aggregate keeps checked `i128` through weighting, one floor, then conversion; audit and correct the `i64` intermediates in rule-body arithmetic against the same numeric contract. |
| C2-05 | Frozen v1 Q7 baseline (no `toward` substitution; door rejects decay rules on definitions without baseline semantics), cadence remainder preserved, epoch-bound rates applied only from the activation barrier; real decay/recovery and the two sanctioned composition fixtures; no relabelled additive test. |
| C2-06 | Reserved epoch-activation command kind bound to the activation payload in both directions; refusal without artifact mutation; ordinary finalization and completion unchanged; Phase-1 admission unchanged. |
| C2-07 | Remove the forbidden `cohort_identity` report field; adjudicate `PacingDiagnostics.command_deferred`; complete the applicable missing oracle cases (every P/R row of the review's per-entry table) with named negative controls; correct overstated coverage and red-first claims honestly without rewriting history. |
| C2-08 | `FinalizationRefusal` is not constructible outside `spark_engine`; add the exact Revision-2 oracle §10 probe. |
| C2-09 | Note only: keep performance limitations honest. |

Dispositions requiring revision: **D-C2-3** (rejected; C2-05), **D-C2-5** (C2-06),
**D-C2-7** (exact originating-artifact resolution and an explicit, tested compatibility
rule with barrier-scoped hot rates), **D-C2-11** (typed invariant-error contract; no
silent repeated `Paused`), **D-C2-13** (payload-independent materialized emission
identity preserving originating identity/fingerprint semantics). Accepted dispositions
D-C2-1, -2, -4, -6, -8, -9, -10, -12, -14, -15 are preserved, with the review's stated
caveats.

## Discipline

- Convert the review's independent counterexamples into meaningful repository tests;
  run the original corpus unchanged and record its result honestly.
- Every new oracle case carries a negative control that kills the named wrong
  implementation.
- Preserve every inherited test (232 Phase-1 tests byte-identical) and the accepted
  V3-F01 architecture. Gate C2 writer tests may be adapted only where a corrected
  surface requires it, never weakened; each adaptation is listed in the report.
- Do not silently invent architecture. Where frozen text leaves a representation open,
  choose one explicitly, justify it from the frozen text, and flag it for review. If
  frozen sources truly conflict under their precedence rules, preserve unaffected work
  and ask one precise Operator question.

## Required validation (fresh)

`cargo fmt --all --check`; `cargo test --workspace --all-features`;
`cargo clippy --workspace --all-targets --all-features -- -D warnings`; the strict
core/engine lint; `cargo metadata --format-version 1`; candidate-range `git diff --check`;
the five Windows/Android static target checks; the release workload; the original
independent counterexample corpus; all new correction and oracle tests.

## Deliverables

- `engineering/phase2/SPARK_PHASE_2_GATE_C2_BOUNDED_REVISION_REPORT_2026-09-11.md`
- `engineering/phase2/gate_c2_bounded_revision_evidence_2026-09-11/` (exact commands
  and outputs)
- `engineering/phase2/SPARK_PHASE_2_GATE_C2_REVISION_INDEPENDENT_REVIEW_MISSION_2026-09-11.md`
  for Codex HIGH, naming the exact corrected scope and prior findings.

Commit coherent checkpoints; push the candidate branch normally; verify local HEAD =
tracking ref = live GitHub ref; leave all existing branches and worktrees intact.
