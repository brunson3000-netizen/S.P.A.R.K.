# S.P.A.R.K. Gate C2 — Independent Implementation Review Mission

**Date:** 2026-09-11
**Role:** Independent reviewer (Codex, HIGH reasoning). You are **not** the implementation
writer. Do not repair the candidate; report findings. Writer/reviewer separation is binding.

## Authority

- Review of the Gate C2 Phase-2 implementation candidate: AUTHORIZED.
- Publishing your review document and evidence on a new review branch, normal commit and
  push: AUTHORIZED.
- Merging, fast-forwarding, or otherwise advancing production (`phase1-refoundation-v2`):
  NOT AUTHORIZED by this mission. Production promotion requires a separate Operator decision
  after your verdict.
- Phase 3, G.A.M.E. changes, paid external compute, and merging the Fable research runtime:
  NOT AUTHORIZED.
- Never force-push, reset, or discard existing work. Preserve every worktree and branch.

## Exact inputs

- Repository: `/home/chromikey/Projects/SPARK`; remote
  `https://github.com/brunson3000-netizen/S.P.A.R.K..git`.
- Production baseline: `phase1-refoundation-v2` at
  `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5`.
- Candidate branch: `candidate/phase2-gate-c2-implementation-20260911`. The exact final
  candidate commit is stated in the writer's completion receipt (a document cannot contain
  its own commit hash). Verify it equals the live remote tip, descends from the baseline, and
  contains these checkpoint commits in order:
  - `4b02532` checkpoint 0 — mission and inherited baseline;
  - `aad4633` checkpoint 1 — frozen `spark-core` additive surfaces;
  - `b7d849f` checkpoint 2 — rule/effect runtime, rule-set door, engine composition;
  - `5fe883f` checkpoint 3 — request boundary, waves, pacing, rejection tests;
  - `0423d91` checkpoint 4 — finalization, consumer, replay, facade probes;
  - `4997d56220000edef3a2b4cd987feff7900d2488` checkpoint 5 — stores/semantics suite and
    the long-history workload example (the last code-and-test commit);
  - the final documentation commit — evidence, report, this mission, and the status update
    (its hash is in the receipt; it changes no code or test).
- Writer mission: `engineering/phase2/SPARK_PHASE_2_GATE_C2_IMPLEMENTATION_MISSION_2026-09-11.md`.
- Writer report: `engineering/phase2/SPARK_PHASE_2_GATE_C2_IMPLEMENTATION_REPORT_2026-09-11.md`.
- Evidence: `engineering/phase2/gate_c2_evidence_2026-09-11/`.

## Controlling architecture and oracle (read in recorded precedence)

`engineering/phase2/SPARK_PHASE_2_V3_F01_OPERATOR_ACCEPTANCE_FREEZE_2026-09-10.md` and its
stack: the Revision-2 independent review (`e55b1da`) and its two precision pins; the
Revision-2 candidate and oracle; the V3-F01 FINAL candidate and oracle; the retained V2
candidate sections (X-1 … X-4, rejection classes R-1 … R-7, equivalence preconditions);
Phase-2 freezes v1 → v2 → v3 (Opus v3 is the provenance record); matrices v1 → v3; the
Operator `ActiveRequest` decision; ADR-0001 … ADR-0006.

## What to verify

1. **Scope and preservation.** Only additive `spark-core` changes (X-1 … X-4 in
   `scheduler.rs`, PX-1 accessors and `derived_indexes_consistent` in `timeline.rs`); no
   Phase-0/1 semantic, encoding, digest, identity, authority, or ordering change; all 232
   inherited tests unmodified and passing; `schedule`, `drain_due`, `stage`, `submit_fence`
   untouched; the scheduler digest refactor byte-identical.
2. **Rule/effect runtime** against freezes v1–v3: the closed operation enum, the rule-set
   door checks, the effect model (emission identity, parent-set contexts, candidate
   canonicalization, the typed reducer and its mixture rulings), provenance coverage, the
   numeric model's single floor rule and checked arithmetic, semantic caps, complete
   transition preflight before any mutation, obligations (claim sets, multi-target
   fingerprint binding, mode immutability), occurrence allocation order, cooldowns, decay,
   aggregation, thresholds after complete reduction, depth conversion.
3. **Scheduled processing**: live least `(due_time, profile_id)` slice, full fingerprint
   compare-and-take, the atomic cross-store extraction preflight, executable-only identity
   and pacing, conflicted slices, cohort-local `now`, horizon expansion, the pacing selector
   and oversized exception, and paced/unbudgeted/catch-up equivalence at every cohort.
4. **Request boundary**: `F`, `ActiveRequest` (set before mutation, exact resume, mismatch
   refusal without mutation, cleanup), `stable_boundary_digest` placement,
   request-bound results and `terminates` dequeue eligibility.
5. **Finalization**: P-1 … P-9 completeness against every `stage`/`submit_fence` refusal
   path, the entailment argument, clean staging, positive acknowledgement before the
   one-ordinal fence, `fence.<ordinal>` identity, byte-identical refusal of the complete
   private timeline state, the sticky fail-stop with no boundary or snapshot publication,
   and **pin 2**: P-7 … P-9 use only the PX-1 indexed lookups, never a history scan.
6. **Snapshot/restore/replay**: restore validation order; derived indexes validated;
   completed-boundary reconstruction only under all declared inputs; paused recovery only
   from the snapshot plus the exact request; **pin 1** same-frontier resets in ascending
   `reset_index`.
7. **Facade**: no host-reachable mutable scheduler or timeline, no staging/fencing/removal/
   cancellation/abandonment path, no evaluator access to pacing/frontier/request state;
   `test-support` absent from production edges and from canonical reports.
8. **Oracle coverage**: every AT-I entry the report marks implemented is executed by a
   meaningful test that kills its named wrong implementation; every entry marked partial or
   interpreted is honestly described; no inherited test weakened, ignored, or rewritten.
9. **Writer decisions and interpretations** listed in the report (for example the pacing
   budget's exclusion from `ruleset_content_hash`, per-profile engine composition, decay
   baseline fallback, command payload shape, epoch activation scope, extraction refusal
   inside `process`, test-support seams) — accept, reject, or require revision each.
10. **Honesty of limitations**, especially the workload measurement and the inherited
    `O(history)` cost of Phase-1 `submit_fence`'s history digest, and every
    "not implemented / later gate" statement.

## Validation to rerun fresh

`cargo fmt --all --check`; `cargo test --workspace --all-features`;
`cargo clippy --workspace --all-targets --all-features -- -D warnings`; the strict
core/engine library lint (`-D warnings -D clippy::unwrap_used -D clippy::expect_used
-D clippy::panic -D clippy::indexing_slicing -D clippy::arithmetic_side_effects`);
`cargo metadata --format-version 1`; `git diff --check` over the candidate range; the five
Windows/Android `cargo check --workspace --all-targets --target …` runs; the release
workload example (`cargo run --release --offline -p spark-testkit --example
gate_c2_workload`). Attempt genuine counterexamples against the claims, not only the happy
paths the writer tested.

## Deliverables

- `engineering/phase2/SPARK_PHASE_2_CODEX_GATE_C2_INDEPENDENT_REVIEW_2026-09-11.md` with
  findings (BLOCKER / MAJOR / MINOR / NOTE, each with a concrete failure scenario and file
  and line anchors), per-entry oracle coverage disposition, and a verdict:
  `GATE_C2_ACCEPTABLE_FOR_OPERATOR_ACCEPTANCE`, `GATE_C2_BOUNDED_REVISION_REQUIRED`, or
  `GATE_C2_FOUNDATIONAL_REVISION_REQUIRED`.
- A bounded evidence directory with fresh command outputs.
- Commit on a new review branch based on the exact candidate commit; push normally; verify
  local, tracking, and live remote equality; report the review commit hash in your receipt.

Phase 3 remains unauthorized regardless of verdict. Acceptance and production promotion are
Operator decisions after your review.
