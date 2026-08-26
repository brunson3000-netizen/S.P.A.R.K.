# S.P.A.R.K. Phase 1 — Final Closure Implementation Report

**Date:** 2026-08-26
**Writer:** Claude Code (Opus, HIGH effort), implementation-writer mission per
`PHASE_1_CLAUDE_OPUS_FINAL_CLOSURE_PROMPT.md`
**Authorization:** `PHASE_1_FINAL_CLOSURE_AUTHORIZATION_2026-08-26.md` (operator, AUTHORIZED;
Phase 2 NOT AUTHORIZED)
**Controlling architecture:** `PHASE_1_FABLE_FINAL_ADMISSION_ARCHITECTURE_2026-08-26.md`
**Acceptance:** `PHASE_1_FINAL_CLOSURE_TEST_MATRIX_2026-08-26.md` (AT-H, in full)
**Plan executed:** `PHASE_1_OPUS_FINAL_IMPLEMENTATION_PLAN_2026-08-26.md`

**Status:** `PHASE_1_FINAL_CLOSURE_WRITER_STATUS: COMPLETE`
**Independent Codex review per `PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_PLAN_2026-08-26.md`
remains mandatory before Phase 1 can close.**

---

## 1. Branch and exact commits

| | |
|---|---|
| Branch | `phase1-refoundation-v2` |
| Inherited HEAD (pre-pass) | `6a764ab` — *Authorize final S.P.A.R.K. Phase 1 closure implementation* |
| Pre-production-change reference | `87da5be` — the last commit before this pass touched any Rust |
| Step 1 commit (tests + baseline) | `ee2e850` — *Encode the AT-H final closure corpus against the inherited tree* |
| Step 2 commit (repair + cleanups) | `b8b7414` — *Repair B-01 staged identity registries as derived indexes* |
| Step 4 commit (this report + status) | recorded in §12 |

Git discipline: all previous evidence and history preserved; no rebase, amend, or force
push; no file deleted. The worktree is clean at the end of the pass.

---

## 2. AT-H inherited red/green baseline

Verbatim capture:
`engineering/phase1/final-closure-baseline/BASELINE_FINAL_CLOSURE_FAILURES.txt`

The entire AT-H corpus was encoded and run against the **inherited** implementation before
any production file was touched (commit `ee2e850` changes no production behavior).

| Test | Fable prediction | Observed on the inherited tree |
|---|---|---|
| AT-H1 `contested_command_identity_is_arrival_order_independent` | red | **RED** |
| AT-H2 `contested_source_sequence_is_arrival_order_independent` | red | **RED** |
| AT-H3 `staged_registries_are_a_derived_index_over_staged_slots` | red or not-yet-compiling | **RED** (compiles; fails at `contest poisons ordinal 1`) |
| AT-H4 `contested_identities_are_unclaimed_after_the_contest` | red | **RED** |
| AT-H5 `poison_evidence_insertion_is_unconditional_and_unscreened` | green | **GREEN** |
| AT-H6 `finalization_moves_identity_from_staged_to_finalized_registries` | green (per the summary line) | **RED** — see §3 |
| AT-H7 `epoch_reset_convergence_is_preserved` | green | **GREEN** |
| AT-H8 `three_way_contest_with_reuse_converges_in_all_orders` | green (per the summary line) | **RED** — see §3 |
| AT-H9 `hidden_evidence_guards_stay_green` | green | **GREEN** |
| AT-H11 `duplicate_id_manifest_hash_is_order_independent` (m-02 bundled) | red | **RED** |
| AT-H11 companion `valid_manifest_hashes_are_unchanged_by_the_duplicate_id_fix` (writer-added pin) | n/a | **GREEN** |

Raw totals at the baseline: `spark-core` in-crate 0 passed / 2 failed;
`spark-testkit --test final_closure` 4 passed / 5 failed.

Every red failure reproduces the staged-identity defect the admission architecture
describes, in the exact terms of probes P1/P2/P5 (equal digests, then divergent answers to
the same identity-reuse submission) or of S1 (promoted entries lingering in the staged
registries). No test predicted green was red for an unexplained reason, and no test
predicted red was green.

---

## 3. The two prediction deviations, and why the pass did not stop

The stop condition is *"the observed AT-H baseline contradicts the Fable architecture
materially."* Two tests deviated from the synthesis's one-line summary
(`AT-H1/H2/H4 red, AT-H5–AT-H9 green`), and in both cases the deviation is a
**bookkeeping inconsistency inside the prediction itself**, resolved by each test's own
specification in the matrix. Both are red for exactly the defect the architecture
describes, so the observed baseline **confirms** the architecture rather than
contradicting it.

**AT-H6.** The matrix specifies AT-H6(b) as *"via AT-H3's mechanism, the staged registries
no longer contain the promoted entries (S1)"*. S1 is part of the repair, so AT-H6 cannot
possibly hold on the inherited tree; the summary line grouped AT-H6 with the regression
pins by position rather than by content. Observed failure is precisely
`!staged_command_identity.contains_key(&e0.command_id)` after fence promotion — S1's
subject.

**AT-H8.** The matrix specifies AT-H8 as *"Extends AT-H1/H4 past the pair case and catches
any residual first/second-arrival asymmetry in the removal logic"*, with reuse probes for
every contested identity. On the inherited tree the first arrival of a three-way contest
registers its claims and the other two never do, so reuse of the first arrival's command
ID is rejected in the orders where it arrived first and admitted otherwise. Observed
failure: `expected Acknowledged(NewlyStaged), got Err(CommandIdentityConflict { command_id:
CommandId(cmd.a) })` — the identical defect as AT-H1, in strictly stronger form.

Recorded as a genuine deviation in §11. No test was weakened, adapted, or skipped to
reconcile the prediction.

---

## 4. Exact B-01 transition changes

All in `crates/spark-core/src/timeline.rs`. Against the architecture §3.1 transition table:

| Transition | Required behavior | Implemented |
|---|---|---|
| Stage into empty slot (screening passed) | register both entries | **unchanged** — `stage`'s `None` arm still inserts into both registries after screening, in the same order, with the same errors |
| Empty-slot attempt fails identity screening | no trace, no slot touched | **unchanged** — early `Err(CommandIdentityConflict)` / `Err(SourceSequenceConflict)` before any mutation |
| Idempotent restage of a staged envelope | no registry action | **unchanged** |
| `Staged(A)` + distinct `B` → `Poisoned{h(A),h(B)}` | remove A's two entries; B still never registered | **CHANGED** — the `Some(SlotState::Staged { .. })` conflict arm now binds the slot's envelope as `staged`, captures `staged.command_id` and `(staged.source_id, staged.source_sequence)`, and removes both entries before installing `SlotState::Poisoned`. The keys are taken from the **staged** envelope, never from the incoming claimant. The claimant is still not registered and still not screened. |
| Further claim on a poisoned slot | evidence insert only | **unchanged** — the `Some(SlotState::Poisoned(_))` arm is byte-for-byte the same unconditional `evidence.insert(semantic_hash)`; no registry is consulted or mutated |
| Fence promotes `Staged` slots to finalized history | remove each promoted envelope's staged entries | **CHANGED (S1)** — the promotion loop in `submit_fence` now removes `envelope.command_id` and the source-sequence key from the staged registries in the same iteration that inserts them into the finalized ones. The claims *move*; no duplicate remains. |
| Epoch reset | clear slots + both staged registries | **unchanged** — `reset_epoch` still clears `slots`, `staged_command_identity`, and `staged_source_sequence_identity`, and still leaves the permanent registries and finalized history intact |

The **safe-removal lemma** (architecture §3.2) is carried as a comment at the poisoning
removal site: the entry under `staged.command_id` was created by this very envelope,
because empty-slot screening admits at most one semantically *distinct* staged envelope per
command ID, and a semantically identical envelope has an equal semantic hash — which covers
`input_ordinal` — so it would be this same slot. The removal therefore cannot evict a
foreign registration. The same argument covers the source-sequence key.

**Documentation duties (part of the change).**

- The `staged_command_identity` / `staged_source_sequence_identity` field comment now
  states the derived-index invariant explicitly, in both directions, plus *why* the digest
  must not simply hash these maps (doing so would satisfy R1 by breaking R2 and regress
  AT-F1).
- Module-header invariant 3 now says identity claims are reserved **only** while an
  envelope is finalized or positively staged, so contested identities become unclaimed
  after a contest, in every arrival order — the coherent reading of ADR-0003 §11's
  no-arrival-winner rule.
- Module-header invariant 3 now also states the §4.3 binding rule: *identity screening
  guards admission into positive staging only; a claim recorded as contest evidence is
  recorded by semantic hash, unconditionally.* This is the asymmetry AT-H5 pins.

**Resulting invariant, now true at every observable point:**

```text
staged_command_identity         == { env.command_id -> h(env)                          | Staged(env) ∈ slots }
staged_source_sequence_identity == { (env.source_id, env.source_sequence) -> h(env)     | Staged(env) ∈ slots }
```

**What deliberately did not change** (architecture §3.4), confirmed at diff level: no
digest encoding (`canonical_state_digest` / `canonical_history_digest` bodies are
untouched), no public signature, no type, no error variant, no evidence representation, no
fence-validation ordering, no frozen-contract behavior, and not one line of the scheduler's
own logic. The empty-slot screening's `existing == semantic_hash` tolerance is **kept** (S4
rejected): it remains reachable through the finalized registries' `.or_else` chain.

---

## 5. m-02 / S2 / S3 dispositions

### m-02 — IMPLEMENTED

`crates/spark-engine/src/profile/manifest.rs`. `manifest_content_hash` now encodes each
`DefinitionSpec` first and sorts by `(definition ID, full canonical encoding)` rather than
by ID alone, making the hash a true multiset function on **every** input including invalid
duplicate-ID manifests (ADR-0004's determinism requirement).

The ID remains the primary sort key deliberately, so that for every manifest that can
actually activate — IDs unique, hence no tie to break — the encoding order, and therefore
the hash, is bit-for-bit what it was before. Activation rejection semantics are untouched:
nothing in the activation ceremony was modified, and duplicate-ID manifests are still
rejected exactly as before. Proved by AT-H11 (order independence on duplicate IDs), the
writer-added companion pin (a valid manifest's exact digest, recorded from the inherited
tree, still matches), and the §7 encoding-stability evidence.

### S2 — IMPLEMENTED

New `crates/spark-core/src/evidence.rs`, declared `pub(crate)` so the change adds **no
public API**. It holds `BoundedClaimSet<const EXPOSED: usize, const TRACKED: usize>`, the
single implementation of a bounded order-independent contested-claim set: idempotent
commutative smallest-first insert, tracking cap with truncation flag, exposed projection,
exact omitted count, and the canonical encoding.

`spark_core::timeline::SlotPoisonEvidence` and `spark_core::scheduler::ConflictEvidence`
are now newtypes over `BoundedClaimSet` at their own caps, delegating every method. The
caps stay with the consumers as const parameters rather than being fixed in the shared
module, because they are policy about how much each machine may remember.

Preserved, and verified: canonical encoding (byte-identical — see §7), exposed presentation
limits (`MAX_POISON_EVIDENCE`, `MAX_CONFLICT_EVIDENCE` = 16), tracked-state limits
(`MAX_POISON_TRACKED_CLAIMS`, `MAX_CONFLICT_TRACKED_CLAIMS` = 256), truncation behavior,
and every public accessor (`competing_semantic_hashes`, `omitted_distinct`,
`evidence_truncated`, `WorkKeyConflict`'s fields and accessors) with unchanged signatures
and semantics. The AT-G suite, which is the drift detector for exactly this, is unmodified
and green.

### S3 — IMPLEMENTED

`crates/spark-core/src/timeline.rs`, post-fence slot retention. The predicate is now
`|ord, _| *ord >= new_frontier`; the always-true `*ord <= new_window_end` conjunct and the
`window_end()` call that computed it are gone.

The proof is carried as a comment: `stage` only inserts at an ordinal inside the window
open at that moment; the window is `[frontier, frontier + width - 1]` for a fixed width;
the frontier is monotonically nondecreasing; so every slot is at or below the current
window end at all times, and a promotion strictly *raises* the frontier and hence the
window end. Behavior is identical in every case, including ordinal-space exhaustion, where
the old code's `map_or(u64::MAX, …)` fallback already retained the whole staged tail.

### S1 — IMPLEMENTED (mandatory, folded into §4)

### S4 — REJECTED, as the architecture directs. The defensive equal-hash tolerance stays.

---

## 6. Files changed

| File | Kind | Change |
|---|---|---|
| `crates/spark-core/src/timeline.rs` | production | B-01 poison-transition deregistration; S1 promotion move; S3 retention simplification; S2 `SlotPoisonEvidence` reimplementation; invariant/rule documentation; AT-H3/AT-H6 in-crate test module |
| `crates/spark-core/src/scheduler.rs` | production | S2 `ConflictEvidence` reimplementation only — no scheduling, conflict, drain, or digest logic touched |
| `crates/spark-core/src/evidence.rs` | production (new) | S2 shared `BoundedClaimSet`, `pub(crate)` |
| `crates/spark-core/src/lib.rs` | production | declares `pub(crate) mod evidence;` |
| `crates/spark-engine/src/profile/manifest.rs` | production | m-02 |
| `crates/spark-testkit/tests/final_closure.rs` | test (new) | AT-H1, H2, H4, H5, H7, H8, H9, H11 + the valid-manifest hash pin |
| `engineering/phase1/final-closure-baseline/BASELINE_FINAL_CLOSURE_FAILURES.txt` | evidence (new) | verbatim inherited red/green baseline |
| `engineering/phase1/final-closure-baseline/ENCODING_STABILITY_EVIDENCE_2026-08-26.txt` | evidence (new) | pre/post canonical-digest diff for the bundled cleanups |
| `engineering/phase1/PHASE_1_FINAL_CLOSURE_IMPLEMENTATION_REPORT_2026-08-26.md` | report (new) | this document |
| `engineering/PHASE_STATUS.md` | status | updated |

**No pre-existing test file was modified.** `git diff --name-only 6a764ab -- crates/spark-core/tests
crates/spark-testkit/tests` lists only this pass's own new `final_closure.rs`, so the
matrix's "explicitly unchanged tests" checklist — `reused_command_id_for_different_envelope_at_different_ordinal_conflicts`,
`reused_source_sequence_for_different_command_conflicts`, the poison-order/evidence tests,
the fence/finalization suite, and the scheduler suite in full — is verified unmodified by
construction, and all of it is green.

No new dependency was added; `Cargo.toml`, `Cargo.lock`, and the feature graph are
untouched.

---

## 7. Canonical encoding stability evidence for the bundled cleanups

`engineering/phase1/final-closure-baseline/ENCODING_STABILITY_EVIDENCE_2026-08-26.txt`

The AT-G suite proves *relative* equalities; it would not by itself catch a cleanup that
shifted every digest by a constant. So one temporary probe printed canonical digests for
conflicted scheduler slots and poisoned timeline slots at every mandated claim-set size
(2, 5, 17, 64, 256 = the tracking cap, and 300 = past it) plus valid profile manifest
content hashes, and was run twice: in a clean `git worktree` at the pre-change commit
`87da5be`, and against the post-change tree. The outputs are **identical** — every
canonical digest is byte-for-byte unchanged across m-02, S2, and S3. The probe source and
both outputs are recorded in the evidence file; the probe itself was removed from the tree.

---

## 8. Test count and result

`cargo test --workspace` — **232 passed / 0 failed / 0 ignored** (up from the inherited
221; +11, nothing removed, nothing weakened, nothing `#[ignore]`d).

| Target | Tests |
|---|---|
| `spark-core` unit (incl. AT-H3, AT-H6) | 63 (was 61) |
| `spark-core` `tests/timeline_admission.rs` | 24 |
| `spark-engine` unit | 38 |
| `spark-testkit` unit | 11 |
| `spark-testkit` `tests/external_compile_probes.rs` | 1 (positive control + 22 forbidden compositions) |
| `spark-testkit` `tests/final_closure.rs` (AT-H) | 9 (new) |
| `spark-testkit` `tests/final_digest_correction.rs` (AT-G) | 8 |
| `spark-testkit` `tests/refoundation_adversarial.rs` | 27 |
| `spark-testkit` `tests/refoundation_v2_adversarial.rs` (AT-A…AT-F) | 25 |
| `spark-testkit` `tests/workspace_dependency_direction.rs` | 3 |
| Doc-tests `spark_core` | 15 |
| Doc-tests `spark_engine` | 8 |
| **Total** | **232** |

Post-repair AT-H: all eleven assertions green, including the four in-crate/behavioral tests
that were red at the baseline and the two prediction-deviation tests of §3.

---

## 9. Gates

| Gate | Result |
|---|---|
| `cargo fmt --check` | **PASS** |
| `cargo clippy --workspace --all-targets -- -D warnings` | **PASS** |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | **PASS** |
| Strict canonical lint gate — `cargo clippy -p spark-core -p spark-engine --all-targets --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::indexing_slicing -D clippy::arithmetic_side_effects` | **PASS** |
| `cargo test --workspace` | **PASS** — 232/0 |
| `cargo metadata --format-version 1` | **PASS** (and the dependency-direction/feature-hygiene policy tests that consume it are green) |

Three clippy findings arose during the pass and were fixed rather than allowed:
`cloned_ref_to_slice_refs` at three call sites in the new tests, and one
`doc_lazy_continuation` in the new module-header prose. No `#[allow]` was added to
production code.

### Cross-target static checks

All five installed targets, both `--workspace` and `--workspace --all-targets`:

| Target | `cargo check` | `cargo check --all-targets` |
|---|---|---|
| `x86_64-pc-windows-gnu` | **PASS** | **PASS** |
| `x86_64-pc-windows-msvc` | **PASS** | **PASS** |
| `aarch64-linux-android` | **PASS** | **PASS** |
| `x86_64-linux-android` | **PASS** | **PASS** |
| `armv7-linux-androideabi` | **PASS** | **PASS** |

**These are static checks only.** No Windows or Android execution is claimed; executable
parity on those targets remains the inherited debt it was.

---

## 10. Closed-item regression status

Verified, not assumed: every item's own test corpus is unmodified (§6) and green (§8).

| Item | Status | Evidence |
|---|---|---|
| **B-01** semantic/admission separation, structured acknowledgements, fences, poison evidence, epoch handoff | **PRESERVED and completed** | `timeline_admission.rs` 24/24; AT-A…AT-F 25/25; AT-F1 poison order-independence re-asserted by AT-H9; the repair adds admission-identity order independence without touching acknowledgement, fence, or epoch logic |
| **B-02** trusted activation / spark-engine boundary | **PRESERVED** | `external_compile_probes.rs` green — the positive control plus 22 forbidden composition paths still each fail for the right reason; `spark-engine`'s activation door untouched apart from m-02's hash tie-break |
| **B-03** scheduler conflict poisoning | **PRESERVED** | scheduler suite green; no scheduling/conflict/drain/digest logic changed; S2 is representation-only, with byte-identical digests proved in §7 |
| **B-04** full definition identity and atomic validation | **PRESERVED** | `spark-engine` suite green; definition canonicalization unchanged (m-02 changes only the order in which already-canonical blocks are concatenated, and only for duplicate IDs) |
| **M-01** ConfigRevision | **PRESERVED** | untouched; `spark-engine` suite green |
| **M-02** history/state/outcome-transcript digest model | **PRESERVED** | AT-G 8/8 unmodified and green; AT-H9 restates the hidden-evidence guards; §7 proves digest values unmoved |
| **M-03** panic-free canonical API / lint policy | **PRESERVED** | strict lint gate re-run and green with no new `#[allow]` in production code; the new `evidence.rs` carries the same no-panic discipline, including the "reported rather than asserted" unreachable branch |
| **M-04** qualified random addresses | **PRESERVED** | untouched; `spark-core` suite green |
| **m-01** dependency/feature hygiene | **PRESERVED** | `workspace_dependency_direction.rs` 3/3; no dependency added; `test-support` still off by default and dev-edge-only |
| **Reconstruction restriction** (AT-D) | **PRESERVED** | `resume_at_frontier` still `#[cfg(any(test, feature = "test-support"))]`; AT-H3/AT-H6 are in-crate `#[cfg(test)]` tests and add no seam |
| **Fable crate boundaries** | **PRESERVED** | no crate moved; `evidence.rs` is `pub(crate)` inside `spark-core`, adding no public surface and no cross-crate edge |

Additionally: no `HashMap`/`HashSet`, wall clock, ambient RNG, float, or unordered canonical
iteration was introduced. `BoundedClaimSet` is a `BTreeSet` behind a total-ordered API.

---

## 11. Deviations

1. **AT-H6 and AT-H8 were red at the baseline where the synthesis's summary line predicted
   green.** Fully analyzed in §3. Both are internal inconsistencies between the synthesis's
   one-line tally and each test's own specification in the matrix; both are red for exactly
   the defect the architecture describes; the architecture is confirmed, not contradicted.
   The pass continued rather than stopping, on the ground that the stop condition requires a
   *material* contradiction of the architecture.

2. **AT-H3 and AT-H6 are in-crate `#[cfg(test)]` unit tests in `timeline.rs`,** not
   `spark-testkit` integration tests. The matrix explicitly sanctions this form ("or an
   in-crate unit test in `timeline.rs`"), and it was chosen over exposing a registry
   accessor under `test-support` precisely so the pass adds no new seam and no new public
   surface.

3. **One writer-added test beyond the matrix:** `valid_manifest_hashes_are_unchanged_by_the_duplicate_id_fix`,
   pinning a valid manifest's exact content hash recorded from the inherited tree. It is a
   strengthening, not a substitution: AT-H11 is present in full and unmodified.

No other deviation. Nothing in the mission's stop-condition list was encountered: no frozen
Phase-0 contract needed to change, no new causal primitive was required, no Phase-2 code was
required, no paid/credentialed/external action was required, and no destructive work was
required.

---

## 12. Residuals carried forward

- **Windows/Android executable parity.** Static `cargo check` only, on all five targets. No
  execution is claimed. Unchanged inherited debt.
- **`Ord::clamp` standard-library boundary.** Documented in `spark_core::value`, accepted by
  Codex, unchanged.
- **The §4.5 residual, unchanged and honestly restated.** Full arrival-order independence
  over arbitrary interleavings that include identity-conflicting cross-ordinal claims is not
  achievable while admission-time screening exists, under any model: a reuse claim arriving
  *mid-contest* is screened against the then-current registry and may be rejected in one
  interleaving and admitted in another. That divergence is **visible** — the resulting
  digests differ, because a staged slot exists in one state and not the other — and is never
  the equal-digest/different-behavior divergence this pass repaired. Per-ordered-input
  determinism holds throughout. AT-H5's permutation sweep deliberately permutes only claims
  on an already-poisoned slot for this reason; the Codex review plan §3.2 makes hunting for
  any equal-digest counterexample an explicit falsification target, and the writer found
  none.
- **Phase-2 entry seams** remain as the readiness blueprint describes; nothing in this pass
  opened one.

---

## 13. Scope statement

No Phase-2 code was written. Nothing in this pass implements propagation or effect
evaluation, delayed-obligation execution, a persistence backend, service or network
transport, actor behavior, MCI/game adapters, dialogue or voice, broad catalogs, a
scripting/plugin runtime, or a runtime LLM. No new public API, no new dependency, no digest
encoding change, no frozen-contract change, no crate-boundary change.

---

## 14. Verdict

`PHASE_1_FINAL_CLOSURE_WRITER_STATUS: COMPLETE`

The B-01 staged-identity defect is repaired structurally, per the Fable admission
architecture and the operator's authorization: the staged registries are now a derived index
over the positively staged slots, so opposite arrival orders for the same contested claim set
leave identical canonical state **and** identical future admission behavior. The three
authorized cleanups are bundled, with byte-identical canonical digests proved against the
pre-change tree. All gates and all cross-target static checks pass, 232/232 tests are green,
and no previously closed item regressed.

This is a writer self-assessment and carries no evidentiary weight. Independent Codex review
per `PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_PLAN_2026-08-26.md` remains mandatory; only a
`PHASE_1_CLOSED` verdict from that review ends Phase 1.

`PHASE_2_AUTHORIZATION: NO`
