# S.P.A.R.K. Phase 1 — Claude Code Implementation Mission

## Mode

**IMPLEMENTATION WRITER — AUTONOMOUS WITHIN THE AUTHORIZED SCOPE**

Work directly in the canonical S.P.A.R.K. repository.

Do not ask the operator to babysit routine engineering steps.

Within this mission you are pre-authorized to:

- inspect/read repository files and Git history;
- create/edit/delete files required by Phase 1;
- create Rust crates/modules/tests;
- run local formatting, linting, compilation, tests, benchmarks, and repository scripts;
- use ordinary local Git operations;
- make bounded refactors needed to complete Phase 1;
- download/install normal free Rust crate dependencies through Cargo when needed, provided they are mature and permissively licensed;
- create and commit the Phase-1 implementation and evidence;
- copy the final report to `~/Downloads/`.

Batch routine actions and continue autonomously.

## Stop and ask the operator ONLY if

- a proposed action changes a frozen architecture/product invariant;
- a new causal runtime primitive appears necessary;
- authority ownership must change;
- a destructive action would affect unrelated operator data;
- paid/cloud/external credentials or spend are required;
- an irreversible migration or backward-incompatible product decision is required;
- the repository contains unexpected conflicting production work that cannot be reconciled safely;
- the task would materially expand beyond Phase 1.

Do **not** stop merely to ask permission for ordinary file edits, tests, Cargo operations, commits, or local repository organization.

## Required reading before implementation

Read in this order:

1. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md`
2. all accepted `engineering/phase0/ADR-000*.md`
3. `engineering/phase0/PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
4. `engineering/phase0/PHASE_0_CODEX_CLOSURE_REVIEW_2026-08-25.md`
5. `engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md`
6. `engineering/PHASE_STATUS.md`

Treat later accepted Phase-0 ADR corrections as controlling where they refine the blueprint without changing it.

## Mission

Implement **Phase 1 — Rust Core Skeleton** exactly within the implementation brief.

The goal is not maximal code volume.

The goal is to embody the Phase-0 invariants in a small, testable Rust foundation so later phases cannot accidentally invent different authority, ordering, determinism, or profile-identity semantics.

## Required architecture behavior

### Timeline ingress

Implement the v0.4 accepted model:

```text
exclusive sequencer
-> deterministic admission window
-> one reserved slot per eligible ordinal
-> stage
-> idempotent/poison handling
-> STAGED acknowledgements
-> sequencer digest fence
-> contiguous atomic finalization
-> per-command stable barrier
```

Arrival order must not choose canonical history.

### Authority

Represent authority/write class so invalid write paths are difficult or impossible to call accidentally.

Do not create a generic public "set arbitrary state" API.

### Determinism

No canonical semantics may depend on:

- wall clock;
- thread scheduling;
- HashMap iteration order;
- global RNG state;
- request/session IDs;
- transport arrival order.

### Profiles

Use stable namespaced IDs, explicit authority/write class, exact content hashing, and immutable definition fingerprints.

The precise profile file format is still provisional; implement only enough loader/validation structure to support Phase-1 fixtures cleanly.

## Implementation style

- clear Rust over abstraction theater;
- small modules with explicit dependency direction;
- no general scripting engine;
- no async kernel;
- no `unsafe` in canonical Phase-1 code;
- deterministic collections/order where semantics require it;
- errors are typed and testable;
- public API surface stays narrow;
- comments explain invariants, not obvious syntax.

## Test expectations

Implement the complete minimum test corpus in `PHASE_1_IMPLEMENTATION_BRIEF.md`.

Add extra tests where they materially falsify your own implementation.

Do not weaken a test to make code pass.

## Dependency discipline

Before adding a crate:

1. confirm it is actually useful;
2. prefer mature, small, common crates;
3. prefer MIT/Apache-2.0/BSD/ISC-style licensing;
4. document any nontrivial dependency choice in the final report.

Do not add network services or runtime cloud dependencies.

## Git discipline

Before writing:

```text
confirm repository root
inspect git status
inspect current Phase-0 evidence
```

Do not rewrite Phase-0 evidence.

Keep implementation commits coherent. A final squashed single commit is not required if a small meaningful commit sequence improves evidence, but the working tree must be clean when finished.

## Required completion artifact

Create the canonical report:

```text
engineering/phase1/PHASE_1_CLAUDE_IMPLEMENTATION_REPORT_2026-08-25.md
```

It must include:

- implementation verdict;
- exact commit(s);
- workspace/crate structure;
- requirements implemented;
- tests added and results;
- determinism evidence;
- authority evidence;
- admission-window/finality evidence;
- dependency/license summary;
- Linux validation performed;
- Windows/Android checks performed or explicitly deferred;
- deviations from the implementation brief;
- known risks;
- files changed;
- explicit statement that Phase 2 remains unauthorized pending independent review.

Copy the identical report to:

```text
~/Downloads/PHASE_1_CLAUDE_IMPLEMENTATION_REPORT_2026-08-25.md
```

Repository copy is canonical evidence. Downloads copy is disposable transfer convenience.

## Final response to operator

When complete, report only:

- `PHASE_1_WRITER_STATUS: COMPLETE` or `BLOCKED`
- final commit hash
- `cargo fmt --check` result
- `cargo clippy` result
- `cargo test` result
- Phase-1 test count
- Windows validation status
- Android validation status
- canonical report path
- Downloads report path
- any genuine blocker/deviation
- `PHASE_2_AUTHORIZATION: NO`

Then stop.
