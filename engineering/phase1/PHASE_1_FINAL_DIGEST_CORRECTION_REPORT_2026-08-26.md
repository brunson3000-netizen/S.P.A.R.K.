# S.P.A.R.K. Phase 1 — Final Digest Correction Report

**Date:** 2026-08-26
**Branch:** `phase1-refoundation-v2`
**Inherited HEAD:** `c6aa3ca`
**Writer:** Claude Code (Opus, HIGH effort)
**Scope:** bounded implementation correction only. No architecture redesign,
no Phase 2 work.

---

## 1. Summary

Codex's independent review of Re-Foundation v2 returned `CONVERGED` on the
foundational architecture with one remaining MAJOR: the canonical
scheduler and timeline state digests committed only to the *exposed*
16-hash presentation of contested-slot evidence, while the implementation
retained up to 256 behavior-relevant claim hashes internally. Two states
could therefore agree on every canonicalized field, hold different hidden
tracked claims, and then react differently to the very same next claim.
Equal canonical digests concealed behaviorally distinct states — a false
canonical-state commitment.

That defect is now closed for both places it occurred, and the required
rule is satisfied:

> Every retained internal state value that can change future canonical
> behavior participates in canonical state identity.

`ConflictEvidence` (scheduler) and `SlotPoisonEvidence` (timeline) now
canonicalize **every** claim hash they still track, plus the truncation
flag. Presentation is unchanged at 16 hashes. All previously closed items
and the Re-Foundation v2 architecture are preserved: 221 tests pass, up
from 213, with the eight new assertions added and none removed.

One genuine out-of-scope finding of the same defect *class* was discovered
and reproduced during the pass. It is reported in §7, not fixed, because
its correct repair is a change to B-01 admission semantics rather than a
digest change, and this correction is bounded to conflict evidence and
poison evidence.

---

## 2. Commits

| Commit | Content |
| --- | --- |
| `42b22ab` | **Tests first.** The AT-G hidden-evidence corpus plus its recorded red baseline against the inherited tree. No production code touched. |
| `b759192` | **The correction.** `ConflictEvidence` and `SlotPoisonEvidence` commit their full tracked claim sets to canonical state. |
| (this commit) | This report and the phase-status update. A commit cannot stamp its own hash into its own content, so it is named by position rather than by hash. |

---

## 3. Step 1 — the red baseline, before any production change

The eight AT-G assertions were written and run against the inherited tree
at `c6aa3ca` before a single line of production code changed. Recorded
verbatim in
`engineering/phase1/final-digest-correction-baseline/BASELINE_HIDDEN_EVIDENCE_FAILURES.txt`.

```text
running 8 tests
test hidden_conflict_evidence_is_committed_to_canonical_state ....... FAILED
test hidden_conflict_evidence_changes_the_reaction_to_the_next_claim  FAILED
test hidden_poison_evidence_is_committed_to_canonical_state ......... FAILED
test hidden_poison_evidence_changes_the_reaction_to_the_next_claim .. FAILED
test conflict_permutations_converge_at_every_mandated_set_size ...... ok
test claims_forgotten_beyond_the_tracking_cap_stay_collapsed ........ ok
test poison_permutations_converge_at_every_mandated_set_size ........ ok
test poison_claims_forgotten_beyond_the_tracking_cap_stay_collapsed . ok

test result: FAILED. 4 passed; 4 failed
```

The split is the point. The four failures are the two hidden-evidence
discrimination tests and the two paired behavioral-divergence tests, in
the scheduler and in the timeline — exactly the collision Codex's probe
found, reproduced independently in both places. The four passes are the
guards the correction was required *not* to break: arrival-order
convergence at every mandated set size, and the deliberate collapse of
claims dropped past the tracking cap.

The failing assertion in each case is a digest equality that should have
been an inequality:

```text
assertion `left != right` failed: conflict states with different retained
tracked claims must not share a canonical state digest
  left:  Digest(233a063dd20e886b0471edb0a473c3719cb5b9a3fa33871cf983054aad47b309)
 right:  Digest(233a063dd20e886b0471edb0a473c3719cb5b9a3fa33871cf983054aad47b309)
```

### 3.1 How the tests hide the difference

A hidden-state test is only meaningful if the difference is genuinely
hidden. Each discrimination test therefore asserts first that the two
states are indistinguishable across the **entire exposed surface** —
retained hash set, omitted count, truncation flag, and slot status — so a
digest difference can only come from hidden tracked claims and never from
something a caller could already see:

```rust
assert_eq!(exposed_conflict(&left), exposed_conflict(&right));
assert_eq!(exposed_conflict(&left).1, 2);   // same omitted count
assert!(!exposed_conflict(&left).2);        // same truncation flag
```

Both sets hold the sixteen smallest hashes plus two more; they differ only
in which two. For the scheduler the claim hashes are constructed directly,
because `WorkPayload` is defined to carry an already-canonical opaque
payload hash, so any 32-byte digest is a payload a real producer could
present. For the timeline the semantic hash is *computed* from the
envelope and cannot be chosen, so the test builds a pool of envelopes,
sorts them by semantic hash, and discovers which sixteen fall inside the
exposed window rather than constructing them.

### 3.2 How the tests prove the difference is behavioral

Discrimination alone would only prove the representations differ. Each
discrimination test is paired with one that submits *the same* next claim
to both states and shows they evolve differently — which is what makes the
hidden difference behavior-relevant, and therefore what obliges canonical
state identity to commit to it:

```rust
left.schedule(claim(200));      // already tracked by `left`
right.schedule(claim(200));     // unknown to `right`

assert_eq!(exposed_conflict(&left).1, 2);   // idempotent
assert_eq!(exposed_conflict(&right).1, 3);  // a new distinct claim
assert_ne!(left.drain_due(..), right.drain_due(..));

// therefore they were distinct beforehand, and the digest had to say so:
assert_ne!(before_left, before_right);
```

The final assertion is the one that failed against the inherited tree.

---

## 4. The correction

### 4.1 Scheduler conflict evidence

`ConflictEvidence::canonicalize` is new and commits to the full tracked
set; `WorkKeyConflict::canonicalize` is **removed**, and
`Scheduler::canonical_state_digest` no longer routes a conflicted slot
through `to_conflict`:

```rust
SlotState::Conflicted(evidence) => {
    inner.push_str("slot.conflicted");
    evidence.canonicalize(&mut inner);   // was: evidence.to_conflict(key).canonicalize(..)
}
```

Removing `canonicalize` from `WorkKeyConflict` rather than leaving it
unused is deliberate. `WorkKeyConflict` is a *presentation* value — the
smallest sixteen hashes plus a count and a flag — and the defect was
precisely that a presentation was mistaken for canonical state. With no
`canonicalize` method on it at all, that mistake is no longer available to
make.

### 4.2 Timeline poison evidence

`SlotPoisonEvidence::canonicalize` previously projected through
`competing_semantic_hashes()`; it now encodes the full `competing` set.
This mirrors the scheduler exactly, which is the point: the kernel's two
poisoned-state representations must not drift apart.

### 4.3 What is encoded, and what deliberately is not

Each evidence value now encodes: the tracked-set length, every tracked
digest in ascending order, and the truncation flag.

The exposed count and the omitted count are **dropped** from the encoding.
Both are total functions of the tracked set, so they add no discrimination
whatsoever; keeping them would have been encoding the projection again
alongside the thing it projects from. The truncation flag is **kept**
because it is independent retained state: it records that claims beyond
the cap were forgotten, which does not follow from the tracked set alone.

Claims dropped past `MAX_CONFLICT_TRACKED_CLAIMS` /
`MAX_POISON_TRACKED_CLAIMS` are genuinely not retained and stay
deliberately collapsed. Two claim sets differing only past the cap must
still share a digest, because the digest commits to what the kernel
remembers, never to what it deliberately forgot. That is asserted, not
assumed — `claims_forgotten_beyond_the_tracking_cap_stay_collapsed` and
its poison twin are `assert_eq!` on digests, and both were green before
the change and remain green after it. Codex explicitly approved the
bounded tracking cap as an order-independent refinement; this correction
does not enlarge or weaken it.

### 4.4 Presentation is unchanged

`WorkKeyConflict::competing_payload_hashes` and
`SlotPoisonEvidence::competing_semantic_hashes` still expose sixteen
hashes; `omitted_distinct` and `evidence_truncated` are unchanged;
`MAX_CONFLICT_EVIDENCE` and `MAX_POISON_EVIDENCE` are still 16 and
`MAX_*_TRACKED_CLAIMS` still 256. No public signature changed, no type was
added or removed from the public surface, and no caller-visible behavior
changed except the digests themselves.

### 4.5 One test edit, and why

`spark-core`'s unit test `three_way_conflict_converges_in_all_six_orders`
folded each drained `WorkKeyConflict` into a digest through the method
this change removes. It now compares the drained reports **as values**,
which is strictly stronger than comparing a digest of them: value equality
implies digest equality and additionally catches any field a hand-written
encoder might have omitted. No assertion was weakened or deleted.

---

## 5. Preservation of closed items

| Item | Result | Evidence |
| --- | --- | --- |
| B-01 canonical identity/finality | **PRESERVED** | 24 `timeline_admission` integration tests and the ported v1 corpus pass unchanged. Finalized history is untouched: a poisoned slot can never be finalized, so no poison evidence has ever entered `canonical_history_digest`. AT-G4 asserts the two hidden-evidence ingresses agree on `canonical_history_digest` while differing on `canonical_state_digest`. |
| B-02 trusted activation | **PRESERVED** | `spark-engine` untouched. The external compile-probe suite (1 positive control + 22 forbidden composition paths) passes unchanged. |
| B-03 scheduler conflict | **PRESERVED AND STRENGTHENED** | No first-arrival winner, no executable conflicted payload, poison semantics unchanged. AT-B1–AT-B6 pass unchanged; AT-G1–AT-G3 close the digest gap Codex found. |
| B-04 immutable fingerprint coverage | **PRESERVED** | `spark-engine` untouched. |
| M-01 `ConfigRevision` | **PRESERVED** | Untouched; suite passes. |
| M-02 digest model | **CLOSED** | This correction. |
| M-03 panic-free canonical API | **PRESERVED** | Explicit strict lint gate re-run and passing for both canonical crates; the new encoders contain no unchecked arithmetic, indexing, `unwrap`, `expect`, or `panic`. |
| M-04 qualified random addresses | **PRESERVED** | Untouched; suite passes. |
| m-01 dependency hygiene | **PRESERVED** | `workspace_dependency_direction` (3 tests, incl. feature hygiene) passes; no dependency, feature, or crate boundary changed. |
| Reconstruction restriction | **PRESERVED** | `resume_at_frontier` remains behind `test-support`; AT-D3 passes. |
| Fable crate boundaries | **PRESERVED** | Both edits are inside `spark-core`; the canonical kernel / engine trust boundary / testkit split is unchanged. |

No frozen Phase-0 contract was reopened. No Phase-2 code was written. No
new dependency was added.

---

## 6. Validation results

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | **PASS** |
| `cargo clippy --workspace --all-targets -- -D warnings` | **PASS**, zero warnings |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | **PASS**, zero warnings |
| Explicit strict canonical lint gate (`spark-core`, `spark-engine`, all targets/features, `-D warnings` plus `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `arithmetic_side_effects` denied) | **PASS** |
| `cargo test --workspace` | **PASS**, 221 passed / 0 failed |
| `cargo metadata --format-version 1` | **PASS** |
| Windows static checks (`cargo check --workspace` and `--all-targets`) | **PASS** on `x86_64-pc-windows-gnu`, `x86_64-pc-windows-msvc` |
| Android static checks (`cargo check --workspace` and `--all-targets`) | **PASS** on `aarch64-linux-android`, `x86_64-linux-android`, `armv7-linux-androideabi` |
| Working tree after the writer commits | clean |

### Test breakdown (221, up from 213)

| Target | Count |
| --- | --- |
| `spark-core` unit | 61 |
| `spark-core` timeline integration | 24 |
| `spark-engine` unit | 38 |
| `spark-testkit` unit (scenario + composition) | 11 |
| `refoundation_adversarial` (v1 corpus, ported) | 27 |
| `refoundation_v2_adversarial` (AT-A…AT-F) | 25 |
| **`final_digest_correction` (AT-G)** | **8** |
| `workspace_dependency_direction` (incl. feature hygiene) | 3 |
| `external_compile_probes` | 1 (23 external crate compilations) |
| doc-tests (`spark-core` 15, `spark-engine` 8) | 23 |

The eight new tests are the entire delta. Nothing was removed, skipped, or
weakened.

**The Windows and Android results are static compile checks only.** No
link, execution, fixture replay, or cross-platform digest comparison was
performed on either platform, and none is claimed. That portability debt
is unchanged from the previous report.

---

## 7. Genuine deviation / finding

**No deviation from the brief.** Both named application points were
corrected as specified, all mandated tests were added, and nothing outside
that scope was changed.

**One out-of-scope finding, reproduced and not fixed.** While auditing for
other retained state that can change future canonical behavior, one more
instance of the same *class* was found and confirmed by probe. Full probe
source and output:
`engineering/phase1/final-digest-correction-baseline/OUT_OF_SCOPE_STAGED_IDENTITY_PROBE.txt`.

`TimelineIngress` keeps `staged_command_identity` and
`staged_source_sequence_identity` registries that reject later reuse of a
command id or `(source, sequence)` pair by a semantically different
envelope. Only the branch that stages into an **empty** slot registers
anything; the envelope that *poisons* an already-staged slot is never
registered. Two ingresses fed the same two contesting envelopes in
opposite orders therefore reach an identical canonical state digest while
holding different identity registries:

```text
state digests equal: true
forward  : Err(CommandIdentityConflict { command_id: CommandId(cmd.a) })
reversed : Ok(Acknowledged(StageAcknowledgement { .. }))
post digests equal: false
```

This is retained internal state that changes future canonical behavior and
does not participate in canonical state identity — the same rule this
correction enforces. It is nevertheless **not** fixable the same way, and
that is why it is reported rather than patched:

1. The divergence is **arrival-order sensitive**, not merely hidden.
   Adding the registries to the digest would not repair it; it would only
   make the divergence visible, and it would break the arrival-order
   independence of poisoned-slot state that B-01 and AT-F1 require. That
   would weaken a closed item, which this pass is forbidden to do.
2. The correct repair is to make registration a function of the claim set
   rather than of which envelope happened to arrive first — a change to
   B-01 admission semantics, i.e. a design change, in a pass explicitly
   told not to redesign architecture and to apply its rule to scheduler
   conflict evidence and timeline poison evidence.

It is smaller in blast radius than the original B-03 defect: it cannot
restore arrival-order authority over which payload *executes*, cannot make
a poisoned slot finalizable, and cannot alter finalized history. It
affects only whether a later command reusing a contested identity is
admitted. It nevertheless has the same shape as the defect just closed,
and it is recommended for separate operator authorization before Phase-1
closure.

**Residual carried forward unchanged:** the fully qualified
`Ord::clamp(value, min, max)` standard-library boundary documented in the
v2 report and accepted by Codex, and the Windows/Android executable
portability debt.

---

## 8. Files changed

```text
crates/spark-core/src/scheduler.rs                                   (canonicalization + one unit test)
crates/spark-core/src/timeline.rs                                    (canonicalization)
crates/spark-testkit/tests/final_digest_correction.rs                (new, 8 assertions)
engineering/phase1/final-digest-correction-baseline/
    BASELINE_HIDDEN_EVIDENCE_FAILURES.txt                            (new, red baseline)
    OUT_OF_SCOPE_STAGED_IDENTITY_PROBE.txt                           (new, §7 finding)
engineering/phase1/PHASE_1_FINAL_DIGEST_CORRECTION_REPORT_2026-08-26.md   (this report)
engineering/PHASE_STATUS.md                                          (status refresh)
```

Transfer copy, identical:
`~/Downloads/PHASE_1_FINAL_DIGEST_CORRECTION_REPORT_2026-08-26.md`

---

PHASE_2_AUTHORIZATION: NO
