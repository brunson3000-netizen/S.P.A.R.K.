# S.P.A.R.K. Gate C1 — V3-F01 Bounded Correction Writer Report, Revision 2

**Date:** 2026-09-10
**Agent:** Claude Code (Opus 5, `claude-opus-5`), separated V3-F01 bounded correction writer
**Repository:** `/home/chromikey/Projects/SPARK` (worktree
`/home/chromikey/Projects/SPARK-v3-f01-rev2-writer`)
**Branch:** `candidate/v3-f01-bounded-correction-rev2-20260910`
**Mission:** `engineering/phase2/SPARK_PHASE_2_V3_F01_BOUNDED_CORRECTION_WRITER_MISSION_REV2_2026-09-10.md`
**Verdict:** `CANDIDATE_READY_FOR_INDEPENDENT_REVIEW`

## 1. Summary

The Revision-2 package corrects the three findings of the final independent review and its
five oracle-precision items, as a bounded delta to the FINAL candidate, which it supersedes
as a proposal without rewriting it.

- **FINAL-01 (BLOCKER).** Finalization is now one engine-internal operation. It runs a
  complete read-only preflight (nine rows, each anchored to the Phase-1 check it
  discharges), then an apply — the unchanged `stage` followed by the unchanged
  `submit_fence` — whose success the preflight entails. Every refusal leaves the entire
  timeline byte-identical. Unexpected staging fails closed under a clean-staging invariant
  that restore also enforces. A working-copy transaction is kept only as the differential
  oracle. The old recipe is withdrawn and is executably red.
- **FINAL-02 (MAJOR).** Results name the request presented. A mismatch terminates nothing
  and is never dequeue-eligible. The consumer removes the head only on a result that
  terminates that head, and halts fail-closed if the head and `ActiveRequest` disagree.
  Dequeue durability follows completion durability.
- **FINAL-03 (MAJOR).** History-only reconstruction is scoped to completed boundaries
  under explicit genesis, timeline-metadata, payload, and `F` preconditions. Paused
  recovery requires the committed snapshot plus the exact durable request. The
  "any equal-or-greater horizon" statement is withdrawn.

## 2. Model designation

The mission names Claude Code Opus at high reasoning. This pass ran on Claude Code with
**Opus 5** (`claude-opus-5`), the model selected for the session. This pass records the model
and does not claim an effort setting.

## 3. Exact baseline and lineage

| Item | Value |
|---|---|
| Live remote at entry (`git ls-remote`, after `git fetch --all --prune`) | `phase1-refoundation-v2` = `e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8`; `review/v3-f01-final-20260910` = `5b19d7b5aa935b65fcfad1d8bda9a210d5a5684d`; `candidate/v3-f01-final-correction-20260910` = `74d044ddd2b59c3c2eff558beeeeb515390dbe0f`; `review/v3-f01-serialized-boundary-20260910` = `772e38da0d130d7a1225ba2eb60a75b3999336fe`; `research/fable-architecture-challenge-2026-09-09` = `03d36dd649641b28b232f4650b773413db634611` |
| Controlling review commit (base) | `5b19d7b5aa935b65fcfad1d8bda9a210d5a5684d`; local, tracking, and remote equal |
| Ancestry | `74d044d…` is the sole parent of `5b19d7b…` (verified by `git merge-base --is-ancestor` and `git log --format=%P`); `772e38d…` is the sole parent of `74d044d…` |
| Base contents verified | the final independent review, its evidence directory, and this mission |
| Parent of this pass's commit | `5b19d7b5aa935b65fcfad1d8bda9a210d5a5684d` |
| Branch | `candidate/v3-f01-bounded-correction-rev2-20260910`, the preferred name; absent locally and remotely at entry, so created new |
| Worktrees at entry (all preserved) | primary `/home/chromikey/Projects/SPARK` on `review/v3-f01-final-20260910` at `5b19d7b` (clean); `/home/chromikey/Projects/SPARK-v3-f01-writer` on the FINAL candidate at `74d044d`; `/home/chromikey/Projects/SPARK-fable-challenge-worktree` on the research branch at `03d36dd`. A new worktree was added for this pass; no existing worktree was modified |
| Other branches preserved | `backup/production-github-0b1ea6a`, `backup/production-local-47729bb`, `master`, `phase1-refoundation`, `reconcile/production-2026-09-09`, and all branches above |

No reset, discard, stash, force-push, merge, or production advance was performed.

## 4. Material read in this pass

Read in full: the Revision-2 mission; the final independent review and every file in
`v3_f01_final_independent_review_evidence_2026-09-10/` (README, `run_regressions.py`,
`source_sequence_regression.rs`, and the regression, model, lint, and static-target logs);
the FINAL candidate, FINAL oracle, FINAL writer report, and FINAL model; the `ActiveRequest`
Operator decision; the serialized-boundary independent review; the prior (FINAL) writer
mission and its controlling-record list; ADR-0003; `engineering/PHASE_STATUS.md`; the
development handoff. The convergence protocol was consulted for the Gate C1 role rules.

Phase-1 source inspected directly: `crates/spark-core/src/timeline.rs` lines 1–1799 (types,
errors, registry invariant, `stage`, `submit_fence`, `reset_epoch`, both digests),
`crates/spark-core/src/id.rs` (identifier syntax and length bound, used for D-10), and
`crates/spark-core/Cargo.toml` (the `test-support` feature). The V2-03 … V2-10 material,
Phase-1 scheduler and clock, and the remaining ADRs are carried through the FINAL candidate,
whose corresponding sections this revision leaves unchanged; they were not re-read line by
line in this pass.

## 5. Deliverables and changed files

| File | Change |
|---|---|
| `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_REV2_2026-09-10.md` | added |
| `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_ORACLE_V3_F01_REV2_2026-09-10.md` | added |
| `engineering/phase2/SPARK_PHASE_2_V3_F01_BOUNDED_CORRECTION_WRITER_REPORT_REV2_2026-09-10.md` | added (this file) |
| `engineering/phase2/v3_f01_rev2_bounded_correction_evidence_2026-09-10/SPARK_EVIDENCE_README.md` | added |
| `…/model.py`, `results.txt`, `results.json` | added (disposable model and its output) |
| `…/rev2_finalization_probe.rs`, `run_rev2_probe.py`, `rev2-probe.txt` | added (disposable Phase-1 probe, runner, output) |
| `…/historical-red-probe-rerun.txt` | added (fresh re-run output of the review's unchanged probes) |
| `…/workspace-tests.txt`, `clippy.txt`, `strict.txt`, `static-targets.txt` | added (fresh validation logs; trailing whitespace and terminal blank lines trimmed for `git diff --check`) |
| `engineering/PHASE_STATUS.md` | one bounded subsection added under Phase 2 |

That is 16 files. Unchanged byte for byte: every earlier candidate, oracle, matrix,
report, review, mission, and model directory, including the review's evidence directory
(its probes were run in place and not edited); all production Rust, manifests, tests, and
`Cargo.lock`.

## 6. Disposition of each finding

| Finding | Disposition | Candidate | Oracle | Evidence |
|---|---|---|---|---|
| FINAL-01 operation, owner, input, errors, exclusivity, boundary, publication | CORRECTED | §4.1–§4.8 | AT-I48 | model F1–F6; probe |
| FINAL-01 every refusal byte-identical, including slots, poison evidence, indexes, history, fences, epoch/window metadata, frontier | CORRECTED | §4.7 | AT-I48(b)(c) | model F2 (12 routes); probe (11 routes, `Debug` plus both digests) |
| FINAL-01 positive acknowledgement before fence; one command barrier | PRESERVED | §4.5, §4.9 | AT-I48(d) | model F3; probe |
| FINAL-01 empty frontier and unexpected staged/poisoned state | SPECIFIED (I-CS, P-6, restore 2b) | §4.3, §7 | AT-I48(c)(g) | model F2, F5; probe routes 8–11 |
| FINAL-01 ordinal/window exhaustion | SPECIFIED (P-4, P-5) | §4.4 | AT-I48(c) | model F2; probe routes 6–7 |
| FINAL-01 no rollback of earlier work; D-2 | PRESERVED | §4.7 | AT-I48(h) | model F6 |
| FINAL-01 regression; old recipe red; Phase-1 unchanged | ADDED | §4.10 | AT-I48(a)(b)(f) | historical re-run; model F1, F2b, F4; probe `--negative-control` |
| FINAL-02 mismatch never pops or loses the head | CORRECTED | §5.1–§5.3 | AT-I49(a)(b) | model Q1, Q1b, Q1c |
| FINAL-02 fail-closed restore/mailbox mismatch; durable exact request | CORRECTED | §5.3, §5.4 | AT-I49(c)(d) | model Q2, Q3, Q3b |
| FINAL-02 lifecycle, consumer contract, model terminal classes | CORRECTED | §5.5 | AT-I49 | model `TERMINAL_FOR_PRESENTED`, `drive` |
| FINAL-03 completed-boundary scope and preconditions | CORRECTED | §6.1 | AT-I50(c)(d) | model R3, R3b |
| FINAL-03 same history and `F`, different pause | PROVED | §6.2 | AT-I50(a) | model R1 |
| FINAL-03 snapshot plus exact request; substitution refused | PROVED | §6.2 | AT-I50(b) | model R2 |
| FINAL-03 withdrawn claim; payload non-reconstruction | CORRECTED | §6.3 | AT-I50(d) | — |
| Oracle AT-I43(f) | CORRECTED | §8 | AT-I43(f) | model P-a |
| Oracle AT-I47(a) and encoding pins | CORRECTED | §8 | AT-I47(a), AT-I35 | model P-b |
| Oracle AT-I40(e) | CORRECTED | §8 | AT-I40(e) | model P-c |
| Oracle AT-I39(C) | CORRECTED | §8 | AT-I39(C) | model P-d |
| Oracle AT-I43(a) | CORRECTED | §8 | AT-I43(a) | model P-e |

New decisions flagged for the reviewer (candidate §12): D-6 preflight plus entailed apply
rather than a working-copy transaction; D-7 clean-staging invariant with fail-closed refusal
and the restore check; D-8 entailment violation is a fail-stop internal error; D-9
request-bound results and the consumer halt; D-10 `fence_id = "fence." ‖ decimal(n)`. PX-1
(three read-only Phase-1 accessors) is a proposal only.

## 7. Validation performed in this pass (fresh)

Environment: Linux x86_64, rustc 1.98.0 (88d9e12ae 2026-08-18), cargo 1.98.0
(797e8a9bc 2026-08-05), Python 3.12.3. All runs are from this session; no earlier result
is presented as fresh.

| Check | Result |
|---|---|
| `python3 model.py results.json` (evidence directory) | **58/58 PASS** (34 carried, 24 new); a second run produced byte-identical `results.txt` and `results.json` |
| `run_rev2_probe.py` — REV2 preflight over the unchanged Phase-1 library, disposable external package | PASS: 11 refusal routes byte-identical (`Debug` and both digests); success equals the working-copy reference; follow-on finalizes; the old composition mutates on 6 routes; the `--negative-control` run fails as required. Re-run after `rustfmt`; output in `rev2-probe.txt` |
| Review's `run_regressions.py`, unchanged, re-run in place | still confirms all three defects, the FINAL model re-runs 34/34 (`historical-red-probe-rerun.txt`) — historical red evidence |
| `cargo test --workspace --offline` | **232 passed, 0 failed, 0 ignored**, doctests included |
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| Strict core/engine library clippy (`-D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::indexing_slicing -D clippy::arithmetic_side_effects`) | PASS |
| `cargo metadata --format-version 1` | PASS |
| `cargo check --workspace --all-targets --target …` for `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`, `x86_64-pc-windows-gnu`, `x86_64-pc-windows-msvc` | all five PASS — static compile coverage only |
| `git diff --check` over the staged package; reference existence of every `engineering/…` and `crates/…` path cited by the new documents; no production Rust, manifest, test, or lockfile in the change set | run immediately before commit; results in the completion receipt |

**Unavailable, and not inferred:** Phase-2 acceptance execution (no implementation exists);
real process-crash and durable-mailbox testing; Windows/Android executable replay and digest
parity. Supplementary compute was not used. The Python model is not production code and
not crash evidence; the Rust probe is not a Phase-2 engine.

## 8. Commit and remote verification

A document cannot contain its own commit hash. The completion receipt states the candidate
commit, its parent `5b19d7b5aa935b65fcfad1d8bda9a210d5a5684d`, the branch, and the equality of
local HEAD, the tracking ref, and the live remote tip (`git ls-remote`) after a normal push.

## 9. Limitations and residual matters

1. Without PX-1, preflight rows P-7 … P-9 scan finalized history per command.
2. Multi-fault refusal reasons follow preflight row order (candidate §4.4); verdicts are
   differentially equal to the Phase-1 calls.
3. D-8 introduces a fail-stop handling for an implementation defect; it adds no host
   operation but is new specified behavior for the reviewer to judge.
4. AT-I48(j) and AT-I49(b) are specified but not modelled.
5. No abandonment path; a lost durable command request halts the consumer (unchanged
   design, now stated precisely).
6. The model uses truncated SHA-256 over JSON, not the production encoder, and omits the
   poison-evidence caps; the probe reaches exhaustion states only through the Phase-1
   `test-support` constructor.
7. No adversarial review of this pass exists yet; writer/reviewer separation is intact.

## 10. Independent-review handoff

Route the exact candidate commit to **independent Codex HIGH review**. The following need
scrutiny at minimum:

- completeness of the P-1 … P-9 table against every refusal path of `stage` and
  `submit_fence`, and the entailment lemma (§4.4, §4.5);
- D-6 (why not the working copy), D-7 (fail-closed refusal of states Phase-1 would accept),
  D-8 (fail-stop on entailment violation), D-10 (fence identity);
- the publication point and the claim that the timeline is mutated only by finalization and
  the epoch-reset path (§4.6);
- the request-bound dequeue rule, the consumer halt, and the durability ordering (§5);
- the §6.1 preconditions and whether any reconstruction input is still missing;
- whether each new or revised **Kills:** line discriminates its named wrong implementation.

Do not release the Phase-2 implementation writer. V3-F01 remains **OPEN**; acceptance,
freeze, production implementation, and Phase 3 remain unclaimed and unauthorized.
