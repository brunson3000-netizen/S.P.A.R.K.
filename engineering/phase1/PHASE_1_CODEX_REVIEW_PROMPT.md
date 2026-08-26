# S.P.A.R.K. Phase 1 — Independent Codex Implementation Review

## Role

You are the **independent implementation reviewer** for S.P.A.R.K. Phase 1.

**Effort level: HIGH.**

Claude Code authored the implementation. You did not.

Do not begin Phase 2. Do not rewrite the implementation merely to make it pass. Review, falsify, run tests, inspect code/history, and produce evidence.

## Autonomy

Work autonomously within this review mission.

You are pre-authorized to inspect repository files/history, run local Cargo/Git diagnostic commands, create bounded temporary review fixtures, and create/commit the final review artifact if the tree is otherwise clean.

Do **not** ask the operator for routine permission to read files, grep/search, run Cargo commands, inspect metadata, create review-only tests/fixtures, or write the final review.

Stop only for genuinely destructive, paid/external, credentialed, architecture-changing, or out-of-scope actions.

## Controlling evidence

Read before judging:

1. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md`
2. all accepted `engineering/phase0/ADR-000*.md`
3. `engineering/phase0/PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
4. `engineering/phase0/PHASE_0_CODEX_CLOSURE_REVIEW_2026-08-25.md`
5. `engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md`
6. `engineering/phase1/PHASE_1_CLAUDE_IMPLEMENTATION_REPORT_2026-08-25.md`
7. `engineering/PHASE_STATUS.md`
8. implementation commit `336d4e3b0fec1e5ef5db6edcf591254bfbf32702` and subsequent Phase-1 evidence commits

The repository and executable behavior outrank Claude's report.

## Central question

Does the Phase-1 Rust implementation faithfully embody the frozen Phase-0 contracts sufficiently to authorize Phase 2, **without Phase 2 having been implemented early**?

## Required independent attacks

### A. Timeline ingress / finality

Attack the actual implementation.

Verify:

- one effective sequencer authority per profile timeline epoch;
- admission window derived deterministically from frontier;
- one reserved slot per eligible ordinal;
- frontier cannot be crowded out;
- old-token late arrival cannot become opportunistically valid;
- exact duplicate is idempotent;
- distinct same-ordinal payloads poison independently of arrival order;
- gaps prevent atomic fence finalization;
- wrong digest and previous-fence hash reject atomically;
- non-sequencer stage/fence reject;
- old-epoch sequencer rejects after reset/handoff;
- partial-prefix finalization slides the window deterministically and preserves valid staged tail;
- fence finality correctly requires staged/unpoisoned contiguous state;
- reversed/concurrent logical delivery yields identical canonical digests.

Add adversarial review tests if useful. Do not weaken existing tests.

### B. Authority / write-class safety

Try to find public or indirect paths that:

- mutate `host_owned` through a S.P.A.R.K. path;
- mutate `derived` through a client/effect path;
- change authority/write class under the same ID;
- bypass `AuthorityCatalog`;
- create undeclared state;
- exploit public constructors, mutable fields, helper visibility, or direct collection access.

A pass requires structural enforcement, not merely absence of current callers.

### C. Determinism

Search for hidden nondeterminism:

- unordered `HashMap`/`HashSet` iteration in canonical semantics;
- wall clock;
- ambient/global RNG;
- thread/OS identity;
- unstable serialization;
- floating-point canonical arithmetic;
- manifest hash order dependence;
- platform-dependent byte/integer encoding;
- occurrence indexing tied incorrectly to call partition.

Repeat replay/adversarial fixtures.

### D. Canonical hashing / identity

Review `CanonicalEncoder`, manifest hashes, definition fingerprints, fence digests/hashes, random-address derivation, and scenario digests.

Attack:

- field-boundary collisions;
- definition-order variation;
- reused human version label with changed content;
- authority change under same ID;
- omission of behaviorally meaningful fields;
- inclusion of non-semantic fields;
- ambiguous enum/string encoding.

Explicitly rule on Claude's deviation:

> no dedicated `ConfigRevision` / `config_revision_hash` type was implemented.

Determine whether Phase-1 requirement #13 is satisfied compositionally, is a non-blocking deferral, or is a defect requiring correction before Phase 2.

### E. Scheduler / clock / RNG

Verify:

- backward time rejects;
- equal-time due work has a stable total order;
- ordering is semantically appropriate rather than an accidental insertion-order shortcut;
- random-address results do not depend on call order;
- occurrence indexes do not silently become worker/batch indexes.

### F. IDs / scopes / value model

Check:

- namespaced ID validation/enforcement;
- scope collision resistance across kinds;
- fixed-point/integer representation bounds;
- malformed canonical values cannot enter through public constructors;
- custom taxonomy escape hatches remain data, not executable semantics.

### G. Workspace / dependency architecture

Inspect `cargo metadata` and actual Cargo manifests.

Pay particular attention to:

- `spark-core` dev-dependency on `spark-testkit`;
- whether that test-only cycle creates architectural inversion or future coupling risk;
- whether the dependency-direction test misses dev/build/target-specific dependencies that matter;
- `blake3` feature/license/platform posture.

Do not fail merely because a dev dependency exists; judge the actual architecture.

### H. Cross-platform claims

Independently rerun what is locally possible.

Clearly distinguish:

- Linux execution/test;
- Windows target `cargo check`;
- Android target `cargo check`;
- actual Windows/Android execution determinism, which remains unproven unless executed.

Do not promote a later executable portability gate into a Phase-1 blocker if the accepted brief explicitly deferred it.

### I. Scope discipline

Search for premature Phase-2 implementation:

- propagation/effect runtime;
- service transport;
- persistence backend;
- actor behavior;
- MCI/game adapters;
- dialogue/voice;
- broad catalogs;
- scripting/plugin mechanisms.

### J. Test quality

Do not accept "45 tests pass" by itself.

Verify the complete 20-item minimum corpus in `PHASE_1_IMPLEMENTATION_BRIEF.md` and whether tests falsify the architectural property rather than mirror implementation shape.

## Required commands

At minimum run and record:

```bash
git status --short
git log --oneline --decorate -8
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo metadata --format-version 1
```

Rerun Windows/Android `cargo check` targets if already installed and doing so requires no operator intervention.

## Classification

- **BLOCKER:** Phase 2 would build on wrong authority, determinism, finality/order, identity, or core API semantics.
- **MAJOR:** serious Phase-1 defect requiring correction before Phase 2.
- **MINOR:** cleanup/test/documentation/traceability issue not threatening Phase-2 semantics.

## Required output

Return exactly:

### 1. VERDICT
Choose:
- `PASS_PHASE_1`
- `REVISE_PHASE_1`
- `ESCALATE_TO_OPERATOR`

### 2. BLOCKERS
List or `NONE`.

### 3. MAJORS
List or `NONE`.

### 4. MINORS
List or `NONE`.

### 5. TEST / TOOL RESULTS
Include fmt, clippy, test result/count, target checks, and working-tree status.

### 6. TIMELINE / FINALITY RESULT

### 7. AUTHORITY RESULT

### 8. DETERMINISM / IDENTITY RESULT
Explicitly rule on the missing dedicated `ConfigRevision` hash/type.

### 9. SCOPE / DEPENDENCY RESULT

### 10. CROSS-PLATFORM RESULT

### 11. PHASE-2 AUTHORIZATION
Exactly `YES` or `NO`, then concise rationale.

## Artifact handling

Save canonical evidence as:

```text
engineering/phase1/PHASE_1_CODEX_INDEPENDENT_REVIEW_2026-08-26.md
```

Copy the identical file to:

```text
~/Downloads/PHASE_1_CODEX_INDEPENDENT_REVIEW_2026-08-26.md
```

Repository copy is canonical evidence. Downloads copy is disposable transfer convenience.

If Git is clean apart from the new review artifact, commit only that review artifact with an evidence-only commit message.

Then stop.
