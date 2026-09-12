# S.P.A.R.K. Gate C2 — Independent Review Mission for the Decay-Write Resolution Candidate

**Issued:** 2026-09-12, by the separated Gate C2 writer (Claude Code, Opus 5), under the
Operator's Phase-2 authorization recorded in
`SPARK_GATE_C2_DECAY_WRITE_RESOLUTION_MISSION_2026-09-12.md`. This is the canonical mission
for the independent review of this candidate. **The reviewer must be an agent other than
this writer and must not repair the candidate in the reviewer role.** Writer checks are not
independent and must not be relabelled as such.

## Exact scope

| Item | Value |
|---|---|
| Candidate branch | `candidate/phase2-gate-c2-decay-write-resolution-20260912` |
| Candidate commit | the exact hash in the writer's publication receipt (a commit cannot record its own hash) |
| Code-and-test tree | `b1608ddc4470d73b9c9eaf5ce26c61697000be38` — the last commit touching any crate file; later commits add documentation and evidence only. Verify this rather than assuming it |
| Checkpoint 1 (implementation contract, alone, before any production change) | `ddee0a2…`, resolved from the branch history |
| Mission checkpoint / direct base | `a3227eb423e75851b05b9992dfde6424a7d8932d` |
| Controlling independent review | `e00f248e25f6d34f1041e219f74085649714c19d` (`review/phase2-gate-c2-decay-adjudication-independent-20260911`) |
| Previously reviewed candidate | `544c5f6f99d8dabf9855f8ac68f5666286aa9741` |
| Production baseline | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` (`phase1-refoundation-v2`) |
| Operator resolution | `engineering/phase2/SPARK_GATE_C2_DECAY_WRITE_RESOLUTION_MISSION_2026-09-12.md` |
| Immutable prior decision | `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md` (D-1 … D-7, S-1 … S-13) |
| Writer implementation contract | `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_WRITE_IMPLEMENTATION_CONTRACT_2026-09-12.md` |
| Writer report | `engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_WRITE_RESOLUTION_REPORT_2026-09-12.md` |
| Writer evidence | `engineering/phase2/gate_c2_decay_write_resolution_evidence_2026-09-12/` |
| Preferred review branch | `review/phase2-gate-c2-decay-write-resolution-independent-20260912` |

Before starting:
- verify every hash above against live GitHub (`git ls-remote --heads origin`), and record
  the live heads;
- work in a fresh isolated worktree based exactly on the candidate commit;
- preserve every existing branch and worktree; reset nothing.

## Authority

**Authorized:** review and fresh validation; disposable probes and mutants in external
crates or archives; a review document and bounded evidence; a normal commit and push on a
new review branch; live-ref verification.

**Not authorized:** repairing the candidate, accepting Gate C2 on the Operator's behalf,
production promotion, Phase-3 work, G.A.M.E. changes, merging research runtimes,
force-push, reset, paid external compute.

## Precedence

For decay cadence, residual and commit-time semantics the **Operator adjudication of
2026-09-11 (D-1 … D-7, S-1 … S-13) controls**, and the **Operator resolution of 2026-09-12**
controls the four matters that adjudication §5 left open. The adjudication remains
immutable; the resolution does not amend it. Otherwise precedence is unchanged from the
controlling review §1: the Operator acceptance freeze and the Revision-2 review and pins;
the Revision-2 architecture and oracle; FINAL; retained V2 sections; freezes and matrices
v1 → v2 → v3; the ActiveRequest decision; ADR-0001 … ADR-0006. Withdrawn Phi, SH-1/SH-2 and
architectural R-8 stay withdrawn.

**Do not re-litigate the Operator's decisions.** Review whether the candidate implements
them exactly, whether the evidence discriminates them, and whether anything else regressed.

## What to decide

1. **Custody and lineage.** Is the published candidate the commit the receipt names? Is its
   history exactly checkpoint 1 (contract alone), checkpoint 2 (production change),
   checkpoint 3 (tests), then documentation and evidence? Do production, the previously
   reviewed candidate and the controlling review remain reachable ancestors at their pinned
   hashes, and every other branch and worktree unchanged?

2. **The contract was recorded first.** Verify that the implementation contract is the
   first commit after the mission checkpoint, that it contains nothing else, and that it is
   byte-unchanged since. Judge whether it maps the write paths *accurately* — in particular
   its claim that direct host ingress can never reach a decayable cell, and its claim that
   `effects::reduce` consults the pre-wave value only on the ADDITIVE branch.

3. **Behavior 1, with your own executed discriminators.** An additive change must settle the
   applicable overdue decay/recovery against the existing value and then apply its delta.
   Check at minimum: the pinned discriminator (100 at 0, rate 10 / cadence 10, `+5` at 15 →
   `(95, 15)` then `(85, 20)`, equal to the history with an explicit evaluation at 10);
   aligned, unaligned and not-yet-due writes; repeated additions; several additive rules in
   one wave; saturation at a nonzero baseline; rate 0; recovery in mirror image; scheduled
   additive work as well as command writes; materialized obligation effects.

4. **Behavior 2.** An explicit replacement must set the declared value, unreduced by earlier
   decay, and must not re-phase the grid. Confirm that the previously pinned AT-I22′
   vectors — fresh assignment at 9 then decay at 10 → `(90, 10)`, and post-shock decay at 40
   after a shock at 31 → `(65, 40)` — are unchanged in value and in rationale.

5. **Behaviors 3, 4 and 5.** Removal and restoration; two activations at one logical time,
   changed and unchanged; a decay evaluation of an absent cell. These were reported
   implementation behavior in your predecessor's §5 and are now normative. Verify each with
   your own vectors, in both directions, and verify the interaction the resolution adds:
   an additive write landing while the operation is *removed* must still settle the closing
   segment's whole steps, and one landing at a parameter-changing barrier must bill the
   closing endpoint at the old rate exactly once.

6. **No second charge, no partial settlement, no unreported transition.** Verify that a
   settled write leaves `updated_at = now` so no endpoint is billed twice; that a refused
   wave — cross-family and bounds — leaves both the value and the commit time untouched;
   that the retained atomic mixed-family rejection is not weakened or used as the settlement
   path; and that a crossing watcher still compares committed pre-wave to committed new
   value. Decide independently whether the committed-to-committed comparison is the right
   reading of the retained atomic-wave and effect-reporting rules, and say so either way.

7. **The two declared boundaries.** `Scale`/`Clamp` and composed rule bodies are deliberately
   not settled (report §4). Judge whether leaving them is faithful to the Operator's two
   named categories or whether it is an inconsistency that needs an Operator decision. The
   writer explicitly discloses that a composed body without a decay stage still forfeits
   earlier unapplied steps. State whether that is acceptable as disclosed, and whether the
   double-charge argument for bodies that *do* carry a decay stage holds.

8. **Supersession.** The old lost-prewrite observation must be historical behavior, not a
   passing normative requirement. Verify that no checked-in test asserts it; that the
   controlling review's `own_probes.rs` is byte-identical in its own evidence directory and
   fails at exactly `own_open_lost_prewrite_step`; and that the adapted copy differs by
   exactly one literal on one line. Confirm the failure is classified by the cargo result
   and the failing assertion, not by a wrapper exit code.

9. **Bounded change and preservation.** Exactly four crate paths may change relative to the
   checkpoint, with exactly one replaced production line. `spark-core`, the manifests, the
   lockfile, the seven inherited integration test files, the 17 inline Phase-1 test modules
   and the protected functions — including `decay_walk` and `baseline_of` — must be
   byte-identical. All 627 pre-existing `engineering/` files must be byte-identical.

10. **Retained work.** Every previously accepted disposition must stay intact:
    C2-01 … C2-09 as dispositioned, C2R-01 … C2R-04, C2R2-01, D-C2-1 … D-C2-15 as accepted,
    R-1 and R-3 … R-9, the AT-I8/20b/21/28/33/39/40 completions, the AT-I13 P row, and every
    stated limitation. Re-disposition every oracle row, or state explicitly which rows you
    inherit from the controlling review and why that inheritance is sound.

## Required fresh validation

Run every check with HEAD pinned to the exact candidate commit, and record exact commands,
exit codes and complete output:

- `cargo fmt --all --check`
- `cargo test --workspace --all-features --no-fail-fast`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- the strict core/engine all-feature library lint
- `cargo metadata --format-version 1`
- `git diff --check` over the candidate, production and controlling-review ranges
- the five Windows/Android static target checks
- the release offline `gate_c2_workload`
- preservation, including the bounded-change and immutability claims
- the controlling review's own probes **byte-unchanged**, the writer's resolution-adapted
  copy, and the prior decision-adapted probe corpus
- the writer's 17 mutation controls, re-run against this tree
- independent probes and mutants **of your own** for behaviors 1 … 5, for both declared
  boundaries, and for refusal atomicity

Classify every expected historical failure by the actual cargo result and the failing
assertion. An expected failure is not a pass against a superseded requirement.

## Deliverables

- A review document with one verdict:
  `GATE_C2_DECAY_WRITE_RESOLUTION_ACCEPTABLE_FOR_OPERATOR_ACCEPTANCE`, a bounded-revision
  verdict, or a blocking verdict.
- A disposition for each of the five approved behaviors, for both declared boundaries, and
  for every oracle row (or an explicit, justified inheritance).
- Any new finding, with a concrete failing vector.
- Bounded evidence, in `engineering/phase2/`.
- A publication receipt with the review commit hash and local = tracking = live GitHub
  synchronization, plus confirmation that the candidate, production and every other branch
  and worktree remain as captured.

No acceptance, production promotion or Phase-3 work follows from the review itself.
