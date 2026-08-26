# S.P.A.R.K. Phase 1 — Independent Codex Correction Re-Review

## Role
You are the independent reviewer for the corrected S.P.A.R.K. Phase-1 Rust core skeleton.

**Effort level: HIGH.**

Claude Code authored the implementation and correction. You did not.

Do not begin Phase 2. Review/falsify only. You may create bounded temporary adversarial fixtures and remove them after recording evidence.

## Autonomy
Work autonomously. Do not ask the operator for routine permission to read files, inspect Git, grep/search, run Cargo commands, inspect metadata, create review-only fixtures, or write the final review.

Stop only for destructive unrelated actions, paid/external resources, credentials, frozen architecture changes, or scope expansion.

## Read first
1. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md`
2. accepted `engineering/phase0/ADR-000*.md`
3. `engineering/phase0/PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
4. `engineering/phase0/PHASE_0_CODEX_CLOSURE_REVIEW_2026-08-25.md`
5. `engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md`
6. `engineering/phase1/PHASE_1_CODEX_INDEPENDENT_REVIEW_2026-08-26.md`
7. `engineering/phase1/PHASE_1_CORRECTION_BRIEF_v0.1.md`
8. `engineering/phase1/PHASE_1_CLAUDE_CORRECTION_REPORT_2026-08-26.md`
9. correction commit `b744c95`
10. subsequent evidence-only commits

Repository behavior outranks reports.

## Central question
Did the bounded correction actually close B-01 through B-04, M-01 through M-04, and m-01 sufficiently to authorize Phase 2?

## Required regression attacks

### B-01 — Canonical envelope identity/finality
Independently attempt:
- changed `effective_time`, `source_id`, `source_sequence`, or `command_kind` under same ordinal/command/payload;
- same command ID reused for a distinct semantic envelope;
- same `(source_id, source_sequence)` reused for a distinct command;
- descending finalized source sequence;
- interleaved independent sources;
- unauthorized epoch reset/handoff;
- epoch rollback/reuse;
- behaviorally distinct finalized envelopes producing the same history digest;
- token metadata differences incorrectly becoming behavior identity.

Verify stage/fence/finalized records commit to all behaviorally meaningful envelope fields.

### B-02 — Structural authority safety
Attempt:
- undeclared definition fabrication;
- `host_owned` mutation via S.P.A.R.K.-effect path;
- `derived` mutation via client/effect path;
- public invocation of host/evaluator mutation;
- wrong value type;
- out-of-bounds value;
- disallowed scope;
- cross-profile ID collision;
- runtime schema mutation after activation.

Inspect visibility, constructors, exposed collections, clone/mutability escape hatches.

### B-03 — Scheduler determinism
Test:
- equal-time logical work inserted in opposite orders;
- explicit semantic occurrence indexes;
- exact duplicate handling;
- conflicting duplicate semantic key;
- occurrence overflow;
- absence of call-order/global-counter identity.

### B-04 — Immutable definition identity / atomic validation
Attack:
- same ID/authority, changed value type;
- changed valid scopes;
- changed definition kind;
- built-in `Trigger` vs `Custom("trigger")`;
- different custom scope kinds;
- failed multi-definition validation followed by corrected retry;
- same DefinitionId across independent profiles;
- fingerprint omission/collision.

### M-01 — ConfigRevision
Verify a real config object/hash exists and that content, insertion order, profile context, and human revision labels behave correctly.

### M-02 — Replay/digest evidence
Test:
- empty scenario vs staged-unfinalized command;
- clean vs poisoned staging;
- distinct finalized full envelope;
- differing operation result/error transcript;
- equivalent reversed-delivery scenarios;
- 50+ repeated runs.

### M-03 — IDs/scopes/value bounds
Attack:
- oversized/malformed IDs;
- missing namespace where contract requires one;
- custom-scope collisions;
- relationship representation;
- FixedPoint overflow;
- integer/fixed bounds;
- oversized categorical/custom strings;
- timeline arithmetic overflow;
- scheduler occurrence overflow;
- malformed public constructors.

### M-04 — Random-address profile/artifact context
Verify:
- different profiles differ;
- different behavior-artifact hashes differ;
- same semantic address repeats identically;
- call order remains irrelevant.

### m-01 — Dependency direction
Inspect `cargo metadata` and verify:
- removed core->testkit dev edge;
- dependency-policy check covers relevant dependency kinds;
- normal production direction remains valid;
- dev-only dependencies do not leak into shipped graph.

## General checks
Also verify:
- no Phase-2 scope creep;
- no unsafe Rust in canonical Phase-1 crates;
- no wall clock/global RNG/unordered canonical iteration;
- no platform-dependent canonical encoding;
- no false Windows/Android execution claim.

## Required commands
At minimum run:
```bash
git status --short
git log --oneline --decorate -10
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo metadata --format-version 1
```
Rerun installed Windows/Android target checks if no operator intervention is required.

## Classification
- BLOCKER: surviving foundational defect in authority, canonical finality/identity, scheduler semantics, immutable identity, or equivalent.
- MAJOR: serious Phase-1 defect requiring correction before Phase 2.
- MINOR: non-blocking cleanup/documentation/test issue.

## Required output
Return exactly:

### 1. VERDICT
- `PASS_PHASE_1`
- `REVISE_PHASE_1`
- `ESCALATE_TO_OPERATOR`

### 2. B-01 CLOSURE
`CLOSED` or `OPEN`.

### 3. B-02 CLOSURE
`CLOSED` or `OPEN`.

### 4. B-03 CLOSURE
`CLOSED` or `OPEN`.

### 5. B-04 CLOSURE
`CLOSED` or `OPEN`.

### 6. MAJOR CLOSURE
M-01, M-02, M-03, M-04: each `CLOSED` or `OPEN`.

### 7. MINOR CLOSURE
m-01: `CLOSED` or `OPEN`.

### 8. NEW FINDINGS
List or `NONE`.

### 9. TEST / TOOL RESULTS
Include fmt, clippy, tests/count, metadata, target checks, final working-tree status.

### 10. CONVERGENCE RESULT
State whether the same foundational defect classes survived the bounded correction.

### 11. PHASE-2 AUTHORIZATION
Exactly `YES` or `NO`, then concise rationale.

## Artifact handling
Save canonical evidence as:
```text
engineering/phase1/PHASE_1_CODEX_CORRECTION_REREVIEW_2026-08-26.md
```

Copy the identical report to:
```text
~/Downloads/PHASE_1_CODEX_CORRECTION_REREVIEW_2026-08-26.md
```

Repository copy is canonical. Downloads is disposable transfer convenience.

If Git is clean apart from the new review artifact, commit only that evidence file.

Then stop.
