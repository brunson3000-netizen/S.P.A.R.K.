# S.P.A.R.K. Gate C2 — Final Independent Review Mission for the Decay Adjudication Candidate (Codex HIGH)

**Issued:** 2026-09-11, by the separated Gate C2 writer (Claude Code, Opus 5), under the
Operator's Phase-2 authorization. This is the canonical mission for the final independent
review of Gate C2. The reviewer is independent of the writer and must not repair the
candidate in the reviewer role.

## Exact scope

| Item | Value |
|---|---|
| Candidate branch | `candidate/phase2-gate-c2-decay-adjudication-20260911` |
| Candidate commit | the exact hash in the writer's publication receipt (a commit cannot record its own hash). Its code-and-test tree is checkpoint 1, `244c54b66a9c5bdfe8dd49e28b05cc0f5c255637`. The final commit changes documentation and evidence only |
| Checkpoint 0 (decision record, before any implementation step) | `0c27ad89833d0c88162803ab41dec177424b49bb` |
| Direct base | `6b167d84f21fb60f374a2ab45c62d5c6a040790a` (the controlling second-correction review) |
| Previously reviewed candidate | `6968a4af917c9a32761be371c3e12ec12ba0f1bd` |
| Production baseline | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` (`phase1-refoundation-v2`) |
| Operator decision | `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md` |
| Writer mission | `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_ADJUDICATION_WRITER_MISSION_2026-09-11.md` |
| Writer report | `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_ADJUDICATION_REPORT_2026-09-11.md` |
| Writer evidence | `engineering/phase2/gate_c2_decay_adjudication_evidence_2026-09-11/` |
| Controlling findings | `engineering/phase2/SPARK_PHASE_2_CODEX_GATE_C2_DECAY_ORACLE_REVISION_INDEPENDENT_REVIEW_2026-09-11.md` (C2R2-01 and all retained dispositions) |

Before starting:
- verify every hash against live GitHub;
- work in a fresh isolated worktree based exactly on the candidate commit;
- preserve every existing branch and worktree.

## Authority

**Authorized:**
- review and fresh validation;
- disposable probes and mutants;
- a review document and bounded evidence;
- a normal commit and push on a new review branch (preferred
  `review/phase2-gate-c2-decay-adjudication-independent-20260911`);
- live-ref verification.

**Not authorized:** repairing the candidate, production promotion, Phase 3, G.A.M.E.
changes, merging research runtimes, force-push, reset, paid external compute.

## Precedence

For decay cadence, residual and commit-time semantics, the **Operator adjudication
controls**. It explicitly supersedes the conflicting v1 Q7 and v2 §4.3 duration language,
and AT-I22/AT-I23, as itemized in its §3 (S-1 … S-13). Otherwise precedence is unchanged
from the controlling review §1:
- the Operator acceptance freeze and the Revision-2 independent review and pins;
- the Revision-2 architecture and oracle;
- the FINAL architecture and oracle;
- retained V2 sections;
- freezes and matrices v1 → v2 → v3;
- the ActiveRequest decision;
- ADR-0001 … ADR-0006.

Withdrawn Phi, SH-1/SH-2 and architectural R-8 stay withdrawn. The fixed grid is now
controlling policy. **Do not re-litigate the decision.** Review whether the candidate
implements it exactly and whether the evidence discriminates it.

## What to decide

1. **Decision record.** Is the adjudication document faithful to the Operator decision
   (D-1 … D-7)? Is each superseded, clarified or retained passage (S-1 … S-13) correctly
   identified by path and line at the base commit? Does it add no semantics beyond the
   decision? Was it committed alone, before any implementation step, and left byte-unchanged
   since? Are historical records untouched?
2. **Implementation conformance.** With your own executed discriminators, verify each rule:
   - D-1 fixed grid;
   - D-2 no re-phasing by ordinary non-decay writes, both command and scheduled shocks, and
     rule-body non-decay stages;
   - D-3 a parameter-changing activation closes the segment, discards the residual and starts
     a new grid at the barrier, while an unchanged activation keeps the grid;
   - D-4 endpoint ownership;
   - D-5 no pre-activation charge;
   - D-6 canonical commits for moved, unmoved, saturated, zero-rate and no-whole-step
     evaluations;
   - D-7 the retained nonretroactivity and saturation corrections.

   The writer claims **no executable line changed**, only comments. Verify this and the
   audit reasoning in report §3.
3. **Oracle.** Do AT-I22′ and AT-I23′ (adjudication §4) have discriminating executed
   support? In `phase2_decay_adjudication.rs`, check each vector class:
   fresh-assignment, post-shock, activation-boundary, residual-discard, shortening,
   lengthening, saturation, recovery, replay/restore/pacing, chunking, and the generated
   sweep with writes. For each: are the hand values correct, and does each discriminate
   the superseded elapsed-since-write reading and the rejected residual/endpoint
   alternatives? Is the reference model (`model`) an independent enumeration rather than a
   restatement of `decay_walk`?
4. **Rejected-reading assertions.** Were assertions enforcing the elapsed-since-write reading
   removed or updated only where required, and each documented? The review probes
   `r2prime_frozen_q7_*` stay byte-unchanged in the review evidence. They are expected to
   fail only at the two superseded tuples. Check that the decision-adapted copy differs by
   exactly those two literals.
5. **Undecided matters.** The adjudication §5 lists four matters the Operator did not
   decide:
   - removing and restoring the operation;
   - grid steps before an unevaluated non-decay write;
   - two activations at one logical time;
   - decay of an absent cell.

   Is each implementation behavior stated honestly in report §6 and not presented as an
   Operator ruling? Does any of them contradict D-1 … D-7? If a matter needs an Operator
   decision, say so explicitly. Do not treat it as a defect of this candidate unless it
   violates the decision.
6. **Retained work.** All previously accepted dispositions must stay intact: C2-01/02/03/04/06/08;
   D-C2-1 … D-C2-15 as accepted; R-1 and R-3 … R-9; the AT-I8/20b/21/28/33/39/40
   completions; the AT-I13 P row; and every stated limitation. No inherited test, manifest,
   lockfile, inline Phase-1 test module or protected function may change.

## Required fresh validation

- `cargo fmt --all --check`
- `cargo test --workspace --all-features --no-fail-fast`
- all-target, all-feature clippy with warnings denied
- the strict core/engine lint
- `cargo metadata`
- candidate-range and production-range `git diff --check`
- the five Windows/Android static target checks
- the release workload
- preservation, including the comment-only and immutability checks
- all three prior reviews' corpora, plus the decision-adapted probe copy
- the writer's ten mutation controls and the controlling review's four own mutants
- the 17 default-feature surface controls
- independent probes and mutants of your own for every decision rule

## Deliverables

- A review document with a verdict: `GATE_C2_DECAY_ADJUDICATION_ACCEPTABLE_FOR_OPERATOR_ACCEPTANCE`,
  a bounded-revision verdict, or a blocking verdict.
- Dispositions for each decision rule and for C2R2-01, C2-05, C2-07 and C2R-04, and a
  disposition for every oracle row.
- Bounded evidence.
- A publication receipt with the review commit hash and local/tracking/live
  synchronization.

No acceptance, production promotion or Phase-3 work follows from the review itself.
