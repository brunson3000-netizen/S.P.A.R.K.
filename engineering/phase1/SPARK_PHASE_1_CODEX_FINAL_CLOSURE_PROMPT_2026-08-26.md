# S.P.A.R.K. Phase 1 — Codex Final Closure Review Mission

## Role

**INDEPENDENT AUDITOR / FINAL PHASE-1 CLOSURE GATE**

**Project:** S.P.A.R.K.
**Reviewer:** Codex
**Effort:** HIGH

Opus implemented the final Phase-1 closure pass.
You did not write it.

Do not begin Phase 2.
Do not modify production code to make it pass.
Review/falsify only.

## Artifact naming rule

Every report or evidence file you create for this mission MUST begin with:

`SPARK_`

Canonical final report:

`engineering/phase1/SPARK_PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_2026-08-26.md`

Disposable Downloads copy:

`~/Downloads/SPARK_PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_2026-08-26.md`

## Autonomy

Work autonomously.

You are pre-authorized to:
- inspect repository files and Git history;
- run Cargo/Git/grep/metadata commands;
- run existing tests;
- create bounded temporary adversarial probes under /tmp or another disposable review location;
- remove temporary probes after evidence capture;
- rerun installed Windows/Android static checks;
- create and commit only the final review artifact if the tree is otherwise clean;
- copy the final report to Downloads.

Do not ask the operator for routine permission.

Stop only for:
- destructive unrelated action;
- paid/external resource;
- credentials;
- frozen architecture change;
- scope expansion.

## Required reading

Read in order:

1. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md`
2. accepted Phase-0 ADRs
3. `engineering/phase0/PHASE_0_CODEX_CLOSURE_REVIEW_2026-08-25.md`
4. all Phase-1 Codex reviews
5. `engineering/phase1/PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md`
6. `engineering/phase1/PHASE_1_FABLE_FINAL_ADMISSION_ARCHITECTURE_2026-08-26.md`
7. `engineering/phase1/PHASE_1_FINAL_CLOSURE_TEST_MATRIX_2026-08-26.md`
8. `engineering/phase1/PHASE_1_OPUS_FINAL_IMPLEMENTATION_PLAN_2026-08-26.md`
9. `engineering/phase1/PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_PLAN_2026-08-26.md`
10. `engineering/phase1/PHASE_1_FINAL_CLOSURE_IMPLEMENTATION_REPORT_2026-08-26.md`
11. current `phase1-refoundation-v2` source and tests
12. implementation commits from the writer report

Repository behavior outranks reports.

## Central gate

Determine whether Phase 1 can be declared CLOSED.

The intended closure property is:

> For the same canonical claim set, opposite arrival order must not leave equal canonical digests with different future admission behavior.

The final writer claims the staged command/source identity registries are now pure derived indexes of positively staged commands.

You must independently verify this and search for equivalent hidden-state defects elsewhere.

## A. Reproduce the inherited defect first

Before judging the fix, independently reproduce the pre-fix defect against the pre-production-change commit identified by the writer report.

Required historical counterexamples:

1. command-ID contested slot:
   - opposite claim order;
   - equal canonical digest;
   - later same command;
   - divergent result.

2. source-sequence contested slot:
   - same shape;
   - divergent later result.

3. promoted identities lingering in staged registries.

Record the red baseline.

## B. Final B-01 repair

Attack the corrected implementation:

- staging into empty slot registers command/source identity;
- idempotent restage makes no registry change;
- Staged(A)+distinct B poisons and removes A's staged command/source claims;
- poisoning claimant B is not registered;
- already-poisoned slot accepts evidence insertion unconditionally, without identity screening;
- fence promotion moves staged claims into finalized registries and removes staged copies;
- epoch reset clears staged identity state and preserves finalized identity/history as declared.

### Required permutation tests

For contested command ID and contested source-sequence ID:

- A,B versus B,A -> identical canonical state digest;
- later same command/source-sequence applied to both -> identical disposition and next state digest;
- 3-way contests across every permutation -> identical canonical state;
- exact duplicates remain idempotent;
- poison evidence remains arrival-order independent;
- poison cannot finalize.

### Required derived-index checks

At every observable stable point, prove:

```text
staged_command_identity
==
{ command_id -> semantic_hash(env) | Staged(env) in slots }

staged_source_sequence_identity
==
{ (source_id, source_sequence) -> semantic_hash(env) | Staged(env) in slots }
```

If internal maps are inaccessible externally, use in-crate tests, public behavior, state digests, or review-only instrumentation that does not modify production behavior.

## C. Hidden-state / equal-digest audit

Perform a fresh hunt across Phase 1 for:

> same canonical state digest + same next canonical input -> different result/state because of retained hidden state.

Inspect:
- timeline slots;
- staged/finalized identity registries;
- poison evidence;
- scheduler conflict evidence;
- activation lineage;
- StateStore schema/cells;
- ConfigRevision/profile manifests;
- clock;
- random-address state;
- any persistent helper maps/counters/indexes.

Any retained behavior-relevant state must either:
- participate in canonical state identity, or
- be fully derivable from state that does.

This is the highest-value free-form falsification section.

## D. m-02 manifest hash

Verify:
- duplicate-ID invalid manifests hash as a multiset / construction-order independent;
- valid manifest hashes are unchanged by the fix;
- activation still rejects duplicate IDs;
- no canonical artifact behavior changed for valid manifests.

## E. S2 evidence unification

Verify scheduler conflict evidence and timeline poison evidence:
- still preserve their previous public presentation semantics;
- still retain all behavior-relevant tracked hashes internally;
- canonical encodings are unchanged relative to the pre-S2 implementation for equivalent states;
- >cap behavior remains bounded and order-independent;
- different retained tracked sets produce different canonical state digests;
- forgotten >cap claims remain deliberately behaviorally collapsed.

## F. S3 retention simplification

Verify the simplified predicate is semantically equivalent for:
- ordinary partial-prefix fence;
- full-window fence;
- high frontier;
- ordinal-space exhaustion boundary;
- staged tail preservation.

No new mutation/order difference may appear.

## G. Closed-item regression

Recheck at least:

- B-02 trusted activation;
- B-03 scheduler conflict poisoning;
- B-04 full definition fingerprint + atomic validation;
- M-01 ConfigRevision;
- M-02 history/state/outcome-transcript digests;
- M-03 panic-free canonical API + strict lints;
- M-04 profile/artifact random addresses;
- dependency/feature hygiene;
- reconstruction restriction;
- Fable crate boundaries;
- no Phase-2 scope.

## H. Writer-baseline prediction deviations

The writer reports AT-H6 and AT-H8 were red where a Fable summary line grouped them as green.

Independently determine whether:
- this was genuinely a harmless prediction-summary inconsistency;
- or it indicates the writer overrode a stop condition improperly.

Do not accept the writer's explanation automatically.

## I. Scope / portability

Confirm no Phase-2 implementation exists.

Rerun installed static checks.

Preserve the fact that Windows/Android execution determinism remains unproven and is not required for this Phase-1 closure gate.

## Required commands

At minimum:

```bash
git status --short
git branch --show-current
git log --oneline --decorate -15
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo metadata --format-version 1
```

Run the strict canonical lint gate for `spark-core` and `spark-engine`.

Rerun installed Windows/Android static target checks if no operator intervention is required.

## Test-quality audit

Do not accept 232/232 alone.

Audit:
- AT-H corpus;
- historical red baseline;
- AT-A through AT-G;
- original Phase-1 corpus 1–20;
- external compile probes;
- tests historically found insufficient.

Create independent adversarial probes.

## Severity

BLOCKER:
- any equal-digest/hidden-behavior divergence;
- any first-arrival authoritative state;
- trusted activation bypass;
- scheduler executable conflict winner;
- immutable identity failure;
- panic-capable project-owned canonical API;
- equivalent foundational defect.

MAJOR:
- serious Phase-1 defect requiring correction before Phase 2.

MINOR:
- non-blocking cleanup/documentation/test issue.

## Required verdict

Choose exactly one:

- `PHASE_1_CLOSED`
- `REVISE_PHASE_1`
- `ESCALATE_ARCHITECTURE_PROCESS_REVIEW`

Phase 2 may be recommended only with `PHASE_1_CLOSED`.

## Required report structure

### 1. VERDICT
### 2. HISTORICAL DEFECT REPRODUCTION
### 3. FINAL B-01 CLOSURE
### 4. HIDDEN-STATE / STATE-EQUIVALENCE AUDIT
### 5. M-02 / S2 / S3 CLOSURE
### 6. CLOSED-ITEM REGRESSION
### 7. TEST-QUALITY RESULT
### 8. NEW FINDINGS
### 9. TEST / TOOL RESULTS
### 10. PORTABILITY DEBT
### 11. PHASE-1 CLOSURE
Exactly `YES` or `NO`.
### 12. PHASE-2 AUTHORIZATION RECOMMENDATION
Exactly `YES` or `NO`, then concise rationale.

## Artifact handling

Canonical repository report:

`engineering/phase1/SPARK_PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_2026-08-26.md`

Downloads copy:

`~/Downloads/SPARK_PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_2026-08-26.md`

If the tree is clean apart from the report, commit only that evidence file.

Then stop.
