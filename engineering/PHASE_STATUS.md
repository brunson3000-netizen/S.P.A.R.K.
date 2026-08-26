# S.P.A.R.K. Engineering Status

**Date:** 2026-08-26

## Phase 0

PASS.

## Phase 1

Writer candidate and bounded correction v0.1 both completed; the
independent Codex correction re-review returned `REVISE_PHASE_1` with
B-01, B-02, B-03 and M-01, M-02, M-03 still open — the same three
foundational defect classes (canonical identity/finality, authority/schema
provenance, scheduler semantic identity) that the writer pass had. Per the
established convergence rule, the correction loop was stopped
(`PHASE_1_CONVERGENCE_DECISION.md`).

**Re-foundation v1** completed on branch `phase1-refoundation`
(`6597b4a`, `845e7ca`) and received independent Codex review, which
returned `ESCALATE_ARCHITECTURE_PROCESS_REVIEW`: B-01, B-04, M-01, M-02,
M-04 and m-01 passed, but B-02 (authority provenance), B-03 (scheduler
conflict) and M-03 (panic paths) remained open — the same classes for the
third time. Per the convergence rule the operator convened the mandated
architecture/process review rather than another writer loop.

**Fable architecture/process review** returned
`READY_FOR_IMPLEMENTATION_REFOUNDATION_V2`
(`PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md`). Its
root-cause finding: the three failures were one failure expressed in three
places — trust written as a convention layered on a public surface instead
of made coextensive with the one boundary Rust enforces, the crate
boundary. No frozen Phase-0 contract needed to change.

**Re-foundation v2 completed** on branch `phase1-refoundation-v2`
(`562ff20`, `33ad640`, `d25856e`), test-first: the mandatory AT-A..AT-F
acceptance assertions were encoded against the inherited tree and eleven of
twelve failed before any implementation change. A new `spark-engine` crate
is now the Phase-1 trust boundary, holding the complete activation ceremony
and every intermediate mint behind one public door; the scheduler poisons a
contested work key with order-independent bounded evidence instead of
letting first arrival win; and the panic-free canonical API policy is
enforced by clippy lints and validated-domain types rather than by audit.
`resume_at_frontier` is feature-gated out of the production surface.

213 tests pass (up from 172), including 16 `compile_fail` doc-tests and an
external compile-probe suite that compiles real out-of-workspace consumer
crates against the default-feature surface: one positive control plus 22
forbidden composition paths that must each fail for the right reason. That
probe suite caught a genuine defect during the pass — removing
`FixedPoint`'s panicking `clamp` had silently handed the panic to the
standard library's `Ord::clamp`. fmt/clippy/strict-lint/test/metadata
gates and all five installed Windows/Android `cargo check` targets pass.
Windows/Android remain static checks only.

**Independent Codex review of the v2 pass** returned `CONVERGED` on the
foundational architecture — B-02, B-03's first-arrival defect and M-03 all
closed — with `REVISE_PHASE_1` for one remaining MAJOR: the canonical
scheduler and timeline state digests committed only to the exposed 16-hash
presentation of contested-slot evidence, so two states holding different
hidden tracked claims shared a digest and then reacted differently to the
same next claim.

**Final digest correction completed** (`42b22ab`, `b759192`), test-first:
the eight AT-G assertions were encoded against the inherited tree and four
failed — both hidden-evidence discrimination tests and both paired
behavioral-divergence tests, in the scheduler and the timeline — while the
four order-independence and cap-collapse guards stayed green. Conflict
evidence and poison evidence now canonicalize every claim hash they still
track; presentation stays at 16. 221 tests pass, up from 213, with nothing
removed or weakened. All fmt/clippy/strict-lint/test/metadata gates and all
five Windows/Android static targets pass.

One out-of-scope finding of the same class was reproduced and reported
rather than patched: the timeline's staged command-identity registries are
arrival-order sensitive hidden state, whose correct repair is a change to
B-01 admission semantics rather than a digest change. It awaits separate
operator authorization.

See `PHASE_1_REFOUNDATION_V2_IMPLEMENTATION_REPORT_2026-08-26.md` and
`PHASE_1_FINAL_DIGEST_CORRECTION_REPORT_2026-08-26.md`.

**Fable overnight architecture closure mission completed** (architecture/
falsification/planning only; no production code changed). The staged
command-identity defect was independently reproduced and extended: the
`(source, sequence)` registry carries the identical defect, and the
occupied-slot identity bypass was ruled deliberate and load-bearing. The
repair architecture — identity registration coextensive with positive
staging, making the registries a derived index over digest-committed
state with **no digest change** — is complete, with the AT-H closure test
matrix, a bounded Opus implementation plan, a Codex review plan, and a
Phase-2 readiness blueprint. State-commitment and order-independence
audits over the full tree found no further violation beyond one MINOR
(duplicate-ID manifest hash order sensitivity, m-02). Verdict:
`OPERATOR_DECISION_REQUIRED` — the closure pass awaits the operator
authorization reserved by the previous pass. See
`PHASE_1_FABLE_OVERNIGHT_SYNTHESIS_2026-08-26.md`.

**Final closure implementation completed** (`ee2e850`, `b8b7414`),
test-first under the operator authorization of
`PHASE_1_FINAL_CLOSURE_AUTHORIZATION_2026-08-26.md`. The AT-H corpus was
encoded against the inherited tree first: seven assertions failed and four
stayed green, and the two tests that failed where the Fable synthesis's
one-line tally predicted green (AT-H6, AT-H8) are red for exactly the
defect the architecture describes — a bookkeeping inconsistency inside the
prediction, not a contradiction of it, analyzed in §3 of the report.

The staged command-ID and `(source_id, source_sequence)` registries are
now **derived indexes over the positively staged slots and nothing else**:
a poisoned ordinal withdraws the formerly staged envelope's claims, the
poisoning claimant is still never registered and never screened, and fence
promotion *moves* claims into the permanent registries instead of leaving
duplicates. Contested identities therefore become unclaimed after a
contest, identically in every arrival order, so opposite arrival orders
leave identical canonical state *and* identical future admission
behavior — with **no digest change at all**, because the registries are
now a pure function of state the digest already commits to. Bundled: m-02
(manifest content hash is a multiset function on every input, with every
activatable manifest's hash bit-for-bit unchanged), S2 (the duplicated
scheduler/timeline bounded-claim-set machinery unified into one
`pub(crate)` `BoundedClaimSet`), and S3 (the provably always-true
retention conjunct dropped with its proof documented).

232 tests pass, up from 221, with no pre-existing test file modified and
nothing weakened. All fmt/clippy/strict-lint/test/metadata gates and all
ten Windows/Android static `cargo check` runs pass; Windows/Android remain
static checks only. A pre/post canonical-digest probe run in a clean
worktree at `87da5be` proves the three cleanups moved no digest value.
Verdict: `PHASE_1_FINAL_CLOSURE_WRITER_STATUS: COMPLETE`. See
`PHASE_1_FINAL_CLOSURE_IMPLEMENTATION_REPORT_2026-08-26.md`.

**Independent Codex review per
`PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_PLAN_2026-08-26.md` remains mandatory;
only a `PHASE_1_CLOSED` verdict ends Phase 1.**

Re-foundation v2 writer: Claude Code (Opus, HIGH effort).
Architecture/process review: Fable.
Independent review of the v2 pass: Codex.
Final digest correction writer: Claude Code (Opus, HIGH effort).
Overnight closure architecture: Fable.
Final closure implementation writer: Claude Code (Opus, HIGH effort).

## Phase 2

**NOT AUTHORIZED.**
