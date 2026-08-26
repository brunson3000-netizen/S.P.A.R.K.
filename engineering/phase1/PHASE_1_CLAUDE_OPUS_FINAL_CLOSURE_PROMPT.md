# S.P.A.R.K. Phase 1 — Opus Final Closure Implementation Mission

## Role

**IMPLEMENTATION WRITER**

**Model:** Opus
**Effort:** HIGH

The operator has explicitly authorized the final Phase-1 closure implementation and the three Fable-recommended cleanups.

Work autonomously. Do not ask the operator for routine permission.

## Read first

1. `engineering/phase1/PHASE_1_FABLE_OVERNIGHT_SYNTHESIS_2026-08-26.md`
2. `engineering/phase1/PHASE_1_FABLE_FINAL_ADMISSION_ARCHITECTURE_2026-08-26.md`
3. `engineering/phase1/PHASE_1_FINAL_CLOSURE_TEST_MATRIX_2026-08-26.md`
4. `engineering/phase1/PHASE_1_OPUS_FINAL_IMPLEMENTATION_PLAN_2026-08-26.md`
5. `engineering/phase1/PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_PLAN_2026-08-26.md`
6. `engineering/phase1/PHASE_1_FINAL_CLOSURE_AUTHORIZATION_2026-08-26.md`
7. `engineering/phase1/PHASE_1_FINAL_DIGEST_CORRECTION_REPORT_2026-08-26.md`
8. current `phase1-refoundation-v2` source and tests

The Fable final admission architecture and operator authorization control this mission.

## Mission

Execute the existing Fable implementation plan exactly.

### Mandatory B-01 repair

Make staged identity registries pure derived indexes of positively staged commands:

- on successful stage into an empty ordinal, register the command ID and `(source_id, source_sequence)` claims;
- when that staged ordinal becomes poisoned, remove the formerly staged envelope's command-ID and source-sequence claims;
- do not register or screen the poisoning claimant through those registries;
- on successful fence promotion, MOVE the staged identity claims into finalized registries and remove them from staged registries;
- epoch reset clears/preserves exactly what the Fable architecture specifies.

The goal is structural:

> opposite arrival orders for the same contested claim set must leave identical canonical state and identical future admission behavior.

Do not "fix" this by merely hashing the hidden registries.

### Bundle authorized cleanups

#### m-02
Make `ProfileManifest::manifest_content_hash` construction-order independent for duplicate-ID invalid manifests, as specified by Fable. Preserve activation rejection semantics.

#### S2
Unify the duplicated bounded-claim-set machinery used by scheduler conflict evidence and timeline poison evidence if the Fable implementation plan specifies it is safe. Preserve:
- canonical encoding;
- exposed presentation limits;
- tracked-state limits;
- truncation behavior;
- all existing public behavior.

#### S3
Apply the retention-bound simplification specified by Fable and carry its proof/intent in a concise comment.

## Test-first rule

Before production changes:

1. add/enable the AT-H corpus from `PHASE_1_FINAL_CLOSURE_TEST_MATRIX_2026-08-26.md`;
2. run it against the inherited implementation;
3. record the predicted red/green split in the final report;
4. if the split materially differs from Fable's prediction, STOP and report the discrepancy instead of blindly implementing.

Do not use unrelated malformed inputs to simulate an architectural failure.

## Closed-item protection

Do not regress:

- B-01 semantic/admission separation, structured acknowledgements, fences, poison evidence, epoch handoff;
- B-02 trusted activation / spark-engine boundary;
- B-03 scheduler conflict poisoning;
- B-04 full definition identity and atomic validation;
- M-01 ConfigRevision;
- M-02 history/state/outcome-transcript digest model;
- M-03 panic-free canonical API/lint policy;
- M-04 qualified random addresses;
- m-01 dependency/feature hygiene;
- reconstruction restriction;
- Fable crate boundaries.

## Scope

No Phase-2 code.

Do not implement:
- propagation/effect evaluation;
- delayed-obligation execution;
- persistence backend;
- service/network transport;
- actor behavior;
- MCI/game adapters;
- dialogue/voice;
- broad catalogs;
- scripting/plugin runtime;
- runtime LLM.

## Required validation

Run:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo metadata --format-version 1
```

Also run the existing strict canonical lint gate for `spark-core` and `spark-engine`.

Rerun all already-installed Windows/Android static target checks without operator prompting.

Do not claim Windows/Android execution.

## Evidence / report

Create canonical report:

`engineering/phase1/PHASE_1_FINAL_CLOSURE_IMPLEMENTATION_REPORT_2026-08-26.md`

Copy identical transfer report to:

`~/Downloads/PHASE_1_FINAL_CLOSURE_IMPLEMENTATION_REPORT_2026-08-26.md`

The report must include:

- branch and exact commits;
- AT-H inherited red/green baseline;
- exact B-01 transition changes;
- m-02/S2/S3 dispositions;
- files changed;
- test count/result;
- strict lint result;
- cross-target static checks;
- closed-item regression status;
- any deviation;
- explicit `PHASE_2_AUTHORIZATION: NO`.

## Git discipline

Preserve all previous evidence/history.

Use coherent commits.
Final worktree must be clean.

## Stop conditions

Stop and report BLOCKED only if:
- the observed AT-H baseline contradicts the Fable architecture materially;
- implementation requires changing a frozen Phase-0 contract;
- a new causal primitive is required;
- Phase-2 code is required;
- a paid/credentialed/external action is required;
- unrelated destructive work is required.

Do not stop for routine engineering.

## Final operator response

Return only:

- `PHASE_1_FINAL_CLOSURE_WRITER_STATUS: COMPLETE` or `BLOCKED`
- branch
- commit hash(es)
- AT-H baseline result
- final test total/result
- fmt
- clippy
- all-features clippy
- strict canonical lint
- Windows static checks
- Android static checks
- canonical report path
- Downloads report path
- genuine deviation/blocker
- `PHASE_2_AUTHORIZATION: NO`

Then stop.
