# S.P.A.R.K. Phase 1 — Final Independent Codex Review of Re-Foundation v2

## Role

You are the independent implementation reviewer for S.P.A.R.K. Phase 1 Re-Foundation v2.

**Reviewer:** Codex
**Effort:** HIGH

Fable performed the architecture/process review.
Claude Code Opus implemented it.
You did neither.

Do not begin Phase 2. Do not rewrite production code to make it pass. Review and falsify only.

## Autonomy

Work autonomously. You may inspect repository/history, run Cargo/Git/grep/metadata, create bounded temporary adversarial fixtures/external compile probes, remove them after recording evidence, rerun installed Windows/Android static target checks, and create/commit only the final review artifact if the tree is otherwise clean.

Do not ask the operator for routine permission.

## Required evidence

Read:

1. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md`
2. accepted `engineering/phase0/ADR-000*.md`
3. `engineering/phase0/PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
4. `engineering/phase0/PHASE_0_CODEX_CLOSURE_REVIEW_2026-08-25.md`
5. `engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md`
6. all prior Phase-1 Codex reviews
7. `engineering/phase1/PHASE_1_CONVERGENCE_DECISION.md`
8. `engineering/phase1/PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md`
9. `engineering/phase1/PHASE_1_REFOUNDATION_V2_IMPLEMENTATION_BRIEF.md`
10. `engineering/phase1/PHASE_1_REFOUNDATION_V2_IMPLEMENTATION_REPORT_2026-08-26.md`
11. branch `phase1-refoundation-v2`
12. implementation commits `562ff20`, `33ad640`, `d25856e`
13. current source/tests

Repository behavior outranks reports.

## Central gate

Determine whether Phase 1 can close and Phase 2 may be authorized.

Perform both:
1. conformance review against Fable Sections 3–12;
2. free-form adversarial falsification for equivalent-severity defects.

## B-02 trusted activation

Independently prove/falsify:

- `spark-core` exposes no activation/state trust machinery;
- complete activation ceremony lives in `spark-engine`;
- raw `DefinitionDeclaration` or equivalent partial mint input is not publicly usable;
- valid `DefinitionSpec` cannot self-activate through an alternate public path;
- external code cannot construct `ActivatedProfile`, `ActivatedDefinition`, or `StateStore`;
- `StateStore` cannot accept raw caller-authored authority/type/scope/fingerprint facts;
- write methods are not public;
- activation performs full validation + fingerprint + lineage + atomic commit;
- failed activation leaves lineage unchanged;
- identical independent registries + identical manifests yield identical artifact hashes;
- divergent authority/manifests visibly diverge in activation/store/lineage hashes;
- caller cannot assert built-in/custom kind identity bit;
- no production dependency enables `test-support`.

Use real external compile probes where the property is “must be impossible.”

## B-03 scheduler conflict

Test:

- full semantic WorkKey;
- independent producer/scope/profile occurrence domains;
- same key + same payload idempotent;
- same key + different payload -> conflict/poison;
- neither payload remains executable;
- A/B vs B/A -> identical scheduler digest;
- A/B vs B/A -> identical drain result;
- 3-way conflict all six permutations converge;
- exact duplicate into conflict is state-idempotent;
- drain/report/reschedule deterministic;
- evidence retention/truncation order-independent;
- > tracking cap claim sets converge across permutations;
- different competing claim sets produce different state digests;
- no first-arrival-wins path remains.

Pay special attention to the Opus refinement of capped evidence tracking.

## M-03 panic-free canonical API

Attack:

- invalid runtime `CanonicalTag::try_from_static`;
- invalid `canonical_tag!` literal compile failure;
- method-call and fully-qualified clamp paths;
- reversed bounds;
- checked fixed add/sub overflow;
- incoherent `FixedRange`;
- frontier/window behavior near `u64::MAX`;
- scheduler occurrence arithmetic;
- malformed/unbounded IDs/tags/categories;
- hidden panic paths through trait-provided/std methods;
- public constructors that create states later assumed valid.

Inspect whether the strict lint policy is genuinely active on canonical crates.

Judge whether the documented boundary around fully-qualified standard-library `Ord::clamp` is acceptable under Fable/Phase-0.

## Reconstruction authority

Verify:

- `resume_at_frontier` absent from default production surface;
- only test/test-support can access it;
- production dependency graph cannot enable test-support;
- no alternate production constructor injects frontier/fence/history state;
- arbitrary test frontier cannot masquerade as validated continuation;
- no persistence backend implemented early.

## Closed-item regression

Recheck:

- B-01 semantic/admission separation, acknowledgements/finality, epoch handoff, poison evidence;
- B-04 full fingerprint + atomic validation;
- M-01 ConfigRevision unique-key deterministic validation;
- M-02 history/state/outcome-transcript digest model;
- M-04 profile/artifact random addresses;
- m-01 dependency direction + feature hygiene.

## Fable conformance

For substantive Fable Sections 3–12 classify:

- IMPLEMENTED AS SPECIFIED
- IMPLEMENTED WITH ACCEPTABLE REFINEMENT
- NOT IMPLEMENTED
- REGRESSION / CONFLICT

Specifically rule on Opus's conflict-evidence tracking-cap refinement.

## Test quality

Do not accept 213/213 alone.

Audit:
- test-first baseline;
- external compile-probe harness + positive control;
- compile-fail doctests;
- strengthened replacements for previously insufficient tests;
- original Phase-1 corpus 1–20;
- AT-A through AT-F.

Create independent adversarial cases where useful.

## Scope / portability

Confirm no Phase-2 rule/effect evaluator, persistence, service/network transport, actor behavior, MCI/game adapters, dialogue/voice, broad catalogs, scripting/plugin runtime, or runtime LLM.

Rerun installed Windows/Android static checks. Preserve the later executable portability debt.

## Required commands

At minimum:

```bash
git status --short
git branch --show-current
git log --oneline --decorate -12
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo metadata --format-version 1
```

Also inspect/execute the strict canonical lint gate.

Rerun installed target checks if no operator intervention is required.

## Convergence rule

If an equivalent-severity B-02/B-03/M-03 defect survives, return:

`ARCHITECTURE_PROCESS_CONVERGENCE_FAILURE`

Do not recommend another ordinary correction loop.

## Required output

Return exactly:

### 1. VERDICT
- `PASS_PHASE_1`
- `REVISE_PHASE_1`
- `ESCALATE_ARCHITECTURE_PROCESS_REVIEW`

### 2. B-02 TRUSTED ACTIVATION
`CLOSED` or `OPEN`.

### 3. B-03 SCHEDULER CONFLICT
`CLOSED` or `OPEN`.

### 4. M-03 PANIC-FREE CANONICAL API
`CLOSED` or `OPEN`.

### 5. CLOSED-ITEM REGRESSION
B-01, B-04, M-01, M-02, M-04, m-01 — each `PASS` or `FAIL`.

### 6. FABLE CONFORMANCE

### 7. NEW FINDINGS
List by severity or `NONE`.

### 8. TEST / TOOL RESULTS
Include branch/HEAD, fmt, clippy, all-features clippy, strict lint, tests/count, metadata, static targets, final tree status.

### 9. TEST-QUALITY RESULT

### 10. CONVERGENCE RESULT
Exactly:
- `CONVERGED`
- `ARCHITECTURE_PROCESS_CONVERGENCE_FAILURE`

### 11. PHASE-1 CLOSURE
Exactly `YES` or `NO`.

### 12. PHASE-2 AUTHORIZATION
Exactly `YES` or `NO`, then concise rationale.

## Artifact handling

Create canonical:

`engineering/phase1/PHASE_1_CODEX_REFOUNDATION_V2_INDEPENDENT_REVIEW_2026-08-26.md`

Copy identical file to:

`~/Downloads/PHASE_1_CODEX_REFOUNDATION_V2_INDEPENDENT_REVIEW_2026-08-26.md`

Repository copy is canonical evidence. Downloads is disposable transfer convenience.

If the tree is clean apart from the review artifact, commit only that evidence file.

Then stop.
