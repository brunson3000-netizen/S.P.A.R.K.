# S.P.A.R.K. Gate C2 — Independent Review Mission for the C2W-01 Correction Candidate

**Issued:** 2026-09-12, by the separated Gate C2 writer (Claude Code, Opus 5), under the
existing Operator authorization. This is the canonical mission for the independent review of
this candidate. **The reviewer must be an agent other than this writer, and other than the
reviewer whose finding this candidate closes, unless no third agent is available — in which
case say so in the report.** The reviewer must not repair the candidate in the reviewer
role. Writer checks are not independent and must not be relabelled as such.

## Exact scope

| Item | Value |
|---|---|
| Candidate branch | `candidate/phase2-gate-c2-w01-correction-20260912` |
| Candidate commit | the exact hash in the writer's publication receipt (a commit cannot record its own hash) |
| Code-and-test tree | `d1d9f8e55c33ecb5dddccc0e4799cd8f9aaccb40` — the last commit touching any crate file; later commits add documentation and evidence only. Verify this rather than assuming it |
| Checkpoint 1 (contract correction, alone, before any production change) | `bda58aa…`, resolved from the branch history |
| Controlling review / direct base | `e39690dc51b63fa09fe7c3f30ebd48aa2c16686f` (`review/phase2-gate-c2-decay-write-resolution-independent-20260912`) |
| Reviewed candidate | `cc3dc70182b428f7cbc953ae722f85b39d407dfa` |
| Production baseline | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` (`phase1-refoundation-v2`) |
| Operator resolution | `engineering/phase2/SPARK_GATE_C2_DECAY_WRITE_RESOLUTION_MISSION_2026-09-12.md` |
| Immutable prior decision | `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md` |
| Contract correction | `engineering/phase2/SPARK_PHASE_2_GATE_C2_W01_CONTRACT_CORRECTION_2026-09-12.md` |
| Writer report | `engineering/phase2/SPARK_PHASE_2_GATE_C2_W01_CORRECTION_REPORT_2026-09-12.md` |
| Writer evidence | `engineering/phase2/gate_c2_w01_correction_evidence_2026-09-12/` |
| Operator's next step after KEEP | `engineering/phase2/SPARK_PHASE_2_GATE_C2_OPERATOR_NEXT_AUTHORIZED_STEP_2026-09-12.md` |
| Preferred review branch | `review/phase2-gate-c2-w01-correction-independent-20260912` |

Before starting: verify every hash against live GitHub and record the live heads; work in a
fresh isolated worktree or clone based exactly on the candidate commit; preserve every
existing branch and worktree, and reset nothing.

## Authority

**Authorized:** review and fresh validation; disposable probes and mutants in external
crates or archives; a review document and bounded evidence; a normal commit and push on a
new review branch; live-ref verification.

**Not authorized:** repairing the candidate, accepting Gate C2 on the Operator's behalf,
production promotion, Phase-3 work, G.A.M.E. changes, merging research runtimes,
force-push, reset, paid external compute. Do **not** begin the adversarial test campaign as
part of this review; it is the step *after* a KEEP verdict.

## Precedence

The 2026-09-12 Operator resolution controls additive settlement and behaviors 2–5; the
immutable 2026-09-11 adjudication (D-1 … D-7, S-1 … S-13) controls grids, barriers and
canonical time. Otherwise retain the precedence of `e00f248…` §1 as carried forward by the
controlling review §1. Withdrawn Phi, SH-1/SH-2 and architectural R-8 remain withdrawn.
**Do not re-litigate the Operator's decisions**, and do not reopen findings the controlling
review closed unless a changed dependency makes them live again.

## What to decide

1. **C2W-01 is actually closed.** Run the controlling review's `own_probes.rs`
   **byte-unchanged** against this candidate. It must pass 9/9, with
   `own_composed_clamp_must_settle` giving `(95, 15)` then `(85, 20)` and
   `own_composed_scale_must_settle_recovery` giving `(-95, 15)` then `(-85, 20)`. Then
   construct your **own** composed-body vectors, in both directions, and decide whether the
   rule is implemented end to end rather than special-cased to the reviewer's two shapes.

2. **The corrected rule is the right rule.** The contract correction states it as: a body
   that performs an additive change and declares no debt-consuming `Decay` stage settles its
   starting value. Judge whether that is faithful to approved behavior 1, whether
   "no `Decay` stage" is the right test for "no debt-consuming stage", and whether any
   composed shape you can construct still forfeits overdue debt or charges it twice. Include
   at minimum: transform-first bodies, multi-stage bodies, `Aggregate` stages, bodies whose
   additive stage is a `Subtract`, and bodies on derived (`commit_derived`) targets.

3. **The correction was recorded first.** Verify that the contract correction is the first
   commit after the controlling review, contains nothing else, and is byte-unchanged since.
   Judge whether it is an honest *additive* correction: it must not rewrite the immutable
   adjudication, the Operator resolution, the earlier contract or report, the controlling
   review, or any published evidence. Confirm it withdraws the earlier deferral disclosure
   explicitly and corrects the inaccurate double-charge commentary the review identified.

4. **The preserved boundaries really are preserved.** With your own vectors: a body that
   declares a `Decay` stage keeps its declared stage order and charges its endpoints exactly
   once; standalone and transform-only `Scale`/`Clamp` settle nothing; a declared `Assign` or
   `Aggregate` inside a body wins in either stage order and is never reduced by earlier
   decay. Decide whether the transform-only extension of the accepted boundary is sound or
   is itself a new inconsistency.

5. **The invariants the correction must not disturb.** Independently re-establish: single
   charging across successive composed writes and at a parameter-changing barrier; canonical
   commit times with no backdated commit and no encoding change; atomic refusal, with a
   refused composed wave leaving both value and commit time untouched and consuming no debt;
   committed-to-committed watcher comparison; removal/restoration through the retained
   lineage search; determinism under replay, snapshot/restore and every pacing budget.

6. **Bounded change and preservation.** Exactly two crate paths may change relative to the
   controlling review, with exactly one replaced production line. `spark-core`, the
   manifests, the lockfile, the seven inherited integration files, the 17 inline Phase-1 test
   modules and the protected functions — `decay_walk`, `baseline_of`, `settled`,
   `settlement_operation` and `effects::reduce` included — must be byte-identical. All 782
   pre-existing `engineering/` files must be byte-identical.

7. **Mutation evidence honesty.** The writer carries 16 mutants forward with byte-identical
   patches and adds 4. One prior mutant, `RES-settle-composed-body`, patched the line this
   correction replaces and is **replaced** by `W01-settle-every-body`. Verify that no mutant
   is counted as killed by a compile failure, that the substitution is a genuine equivalent
   rather than a weakening, and re-run the controls yourself with one target directory per
   archive.

8. **Retained work and oracle.** Verify the writer's claim that the controlling review's
   qualification — behavior 1 not fully supported — is now closed, and that everything else
   in its inheritance of `e00f248…` §6 (AT-I1 … AT-I50) and all retained findings and
   limitations stand. Re-disposition the rows whose dependencies this candidate changed, or
   state explicitly which you inherit and why that inheritance is sound.

## Required fresh validation

Run every check with HEAD pinned to the exact candidate commit, recording exact commands,
exit codes and complete output:

- `cargo fmt --all --check`
- `cargo test --workspace --all-features --no-fail-fast`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- the strict core/engine all-feature library lint
- `cargo metadata --format-version 1`
- `git diff --check` over the review, production and reviewed-candidate ranges
- the five Windows/Android static target checks
- the release offline `gate_c2_workload`
- preservation, including the bounded-change and immutability claims
- your own `own_probes.rs` **byte-unchanged**; the historical adjudication-review corpus
  byte-unchanged, which must still fail at exactly `own_open_lost_prewrite_step`; and the
  resolution-adapted and decision-adapted copies
- the writer's 20 mutation controls, re-run against this tree with isolated targets
- independent probes and mutants **of your own** for every item in "What to decide"

Classify every expected historical failure by the actual cargo result and the failing
assertion, never by a wrapper exit code.

## Deliverables

- A review document with one verdict:
  `GATE_C2_W01_CORRECTION_KEEP` (acceptable for Operator acceptance), a bounded-revision
  verdict, or a blocking verdict.
- A disposition for C2W-01, for each of the five approved behaviors, for each preserved
  boundary, and for every oracle row (or an explicit, justified inheritance).
- Any new finding, with a concrete failing vector.
- Bounded evidence under `engineering/phase2/`.
- A publication receipt with the review commit hash, local = tracking = live GitHub
  synchronization, and confirmation that the candidate, production and every other branch
  and worktree remain as captured.

On a KEEP verdict, the Operator's next authorized step is the adversarial test campaign in
`SPARK_PHASE_2_GATE_C2_OPERATOR_NEXT_AUTHORIZED_STEP_2026-09-12.md`. No acceptance,
production promotion or Phase-3 work follows from the review itself.
