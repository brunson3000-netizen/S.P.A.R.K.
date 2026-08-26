# S.P.A.R.K. Phase 1 — Final Closure Implementation Plan (Opus writer pass)

**Date:** 2026-08-26
**Writer:** Claude Code — Opus, HIGH effort
**Controlling architecture:** `PHASE_1_FABLE_FINAL_ADMISSION_ARCHITECTURE_2026-08-26.md` (binding; the writer's latitude is naming, in-file organization, and test phrasing — not semantics, not check ordering, not visibility)
**Acceptance:** `PHASE_1_FINAL_CLOSURE_TEST_MATRIX_2026-08-26.md` (AT-H, mandatory in full)
**Precondition:** operator authorization per §7.1 of the architecture; §7.2 option decided (m-02/S2/S3 in or out).
**Scope police:** bounded correction. No Phase-2 code, no new dependency, no new public API, no digest-encoding change, no frozen-contract change, no crate-boundary change.

---

## Step 0 — branch and baseline

Work on `phase1-refoundation-v2` from current HEAD (all prior evidence preserved). Verify `cargo test --workspace` is green (221) before starting.

## Step 1 — tests first (one commit)

Encode the AT-H corpus (matrix, in full — including AT-H5/H9 regression pins that are expected green) against the inherited tree. Run it; record the verbatim red/green split to `engineering/phase1/final-closure-baseline/BASELINE_FINAL_CLOSURE_FAILURES.txt`. Expected: AT-H1, AT-H2, AT-H4 red (plus AT-H11 red if bundled); AT-H5–AT-H9 green; AT-H3 red or not-yet-compiling depending on the seam chosen. **Any deviation from the expected split: stop and report; do not adapt the tests to reality silently.** Commit tests + baseline only.

## Step 2 — the repair (one commit)

All changes in `crates/spark-core/src/timeline.rs`:

1. **Poison-transition deregistration.** In `stage`'s `Some(SlotState::Staged { .. })` conflict arm (timeline.rs:1348–1372): before installing `SlotState::Poisoned`, remove the formerly staged envelope's two registry entries (`staged_command_identity.remove(&envelope.command_id)`, `staged_source_sequence_identity.remove(&(source_id, source_sequence))` — fields taken from the *staged* envelope held in the slot, not from the incoming claimant). Carry the safe-removal lemma (§3.2) as a comment. The incoming claimant remains unregistered, as today.
2. **Finalization cleanup (S1).** In `submit_fence`'s promotion loop (timeline.rs:1518–1532): as each command's identities are inserted into the finalized registries, remove its entries from the staged registries. After this, state the derived-index invariant (§3.1) in the struct-field comment at timeline.rs:966–969, replacing the current wording.
3. **Documentation duties (part of the change, not optional):** module-level statement of the §4.3 rule ("identity screening guards admission into positive staging only; evidence records are unconditional"); update the invariant-3 wording in the module header to say contested identities are unclaimed after a contest.
4. *(If bundled)* **m-02:** in `crates/spark-engine/src/profile/manifest.rs`, sort definitions by full canonical encoding (encode each `DefinitionSpec` block first, sort the encoded blocks bytewise, then push) so `manifest_content_hash` is a multiset function on every input. **S2:** extract the shared bounded-claim-set type into `spark-core` (new module, e.g. `evidence.rs`), reimplement `ConflictEvidence`/`SlotPoisonEvidence` internals on it without changing any public accessor, constant, or canonical encoding — assert digest stability via the untouched AT-G suite. **S3:** simplify the post-fence retention predicate with the proof comment.

Run the full suite: AT-H fully green, all prior tests green, nothing modified except as above.

## Step 3 — gates (same commit as 2 or separate)

`cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `--all-features` variant; the explicit strict canonical lint gate for `spark-core`/`spark-engine` (deny `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `arithmetic_side_effects`); `cargo test --workspace`; `cargo metadata` policy tests; `cargo check --workspace` and `--all-targets` for `x86_64-pc-windows-gnu`, `x86_64-pc-windows-msvc`, `aarch64-linux-android`, `x86_64-linux-android`, `armv7-linux-androideabi` (static only; claim nothing more).

## Step 4 — completion report (final commit)

`engineering/phase1/PHASE_1_OPUS_FINAL_CLOSURE_REPORT_2026-08-26.md` (or dated as executed): red baseline verbatim; per-change disposition against the architecture §3.1 table; closed-item preservation table (B-01…M-04, m-01, reconstruction restriction, crate boundaries); test count delta; residuals carried forward (`Ord::clamp` boundary, Windows/Android executable debt, plus m-02 if deferred); genuine-deviation section (empty or honest). Update `engineering/PHASE_STATUS.md`. Transfer copy to `~/Downloads/`. End with `PHASE_2_AUTHORIZATION: NO` pending independent review.

## Hard stops (unchanged from the convergence decision)

- Any AT-H test that appears to conflict with a frozen Phase-0 contract, or with a closed item (especially AT-F1/AT-G), is a stop-and-report, never a silent redesign or a test edit.
- Do not touch digest encodings, evidence insertion, fence validation order, or the scheduler.
- Do not weaken or delete any listed test to make the invariant convenient.
- If the repair appears to require registering the poisoning claimant, or screening the poison branch, stop: that is the rejected union model (§4.2/§4.3 of the architecture).

## Estimated blast radius

~15 lines of production change in `timeline.rs` for the core repair (two removal sites + comments); the corpus is the bulk of the work. This is deliberate: the architecture does the deciding, the writer does the proving.
