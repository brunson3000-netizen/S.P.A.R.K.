# S.P.A.R.K. — Fable Overnight Architecture Closure Mission: Synthesis

**Date:** 2026-08-26
**Agent:** Claude Code (Fable, HIGH effort), overnight autonomous mission per `PHASE_1_FABLE_OVERNIGHT_ARCHITECTURE_PROMPT.md`
**Mode honored:** architecture/falsification/planning only — no production Rust written, no implementation file changed, no Phase-2 implementation.
**Tree state:** branch `phase1-refoundation-v2`, HEAD `87da5be`, verified 221/221 tests green before and after probing; the only repository changes from this mission are documentation and probe-evidence files.

---

## 1. Final B-01 recommendation

**Repair the staged-identity defect by making identity registration coextensive with positive staging** — the staged registries become a derived index over the `Staged` slots, and nothing else:

- register identities only on staging into an empty slot (unchanged);
- **remove** the formerly staged envelope's two registry entries when its slot poisons;
- **move** (not copy) entries from the staged to the finalized registries at fence promotion;
- never register or screen the poisoning claimant — poison evidence stays an unconditional set-insert;
- epoch reset clears both (unchanged).

Consequences: contested command IDs and source-sequence pairs become **unclaimed** after a contest, identically in every arrival order; the canonical state digest needs **no change at all**, because the registries become a pure function of state the digest already commits to (staged envelopes + finalized history). Both governing rules — complete state commitment (R1) and per-claim-set order independence (R2) — are then satisfied structurally. Estimated production blast radius: ~15 lines in `timeline.rs`; no public API, digest encoding, frozen contract, or scheduler change. Rejected alternatives (commit-registries-to-digest; register-the-union; fence-time-only checks) are analyzed in the architecture document §4.

Full design: `engineering/phase1/PHASE_1_FABLE_FINAL_ADMISSION_ARCHITECTURE_2026-08-26.md`.

## 2. Additional Phase-1 issues found — 3

Independently reproduced probe evidence in `engineering/phase1/fable-overnight-baseline/FABLE_OVERNIGHT_PROBE_EVIDENCE_2026-08-26.txt`:

1. **The `(source_id, source_sequence)` registry carries the identical B-01 defect** (probe P2: equal digests, then `Err(SourceSequenceConflict)` vs `Ok(newly_staged)` on the same input). The previously recorded probe demonstrated only the command-ID half; a repair covering one registry would have passed the old probe and still been broken. Folded into the B-01 architecture — both registries repaired together.
2. **Identity screening is bypassed on occupied slots** (probe P3: a conflicting command ID poisons an occupied ordinal instead of being identity-rejected). Ruled **deliberate and load-bearing** — screening the poison branch would make the evidence set arrival-order-dependent — but it is currently undocumented, and it is precisely where a well-meaning closure writer could break AT-F1. Documented as a binding rule (§4.3) and pinned by regression test AT-H5.
3. **m-02 (MINOR):** `ProfileManifest::manifest_content_hash` is construction-order-sensitive for duplicate-ID manifests (probe P6) — invalid input that can never activate, so no artifact is affected, but the canonical hash function should be a multiset function on every input (ADR-0004). One-line fix specified; operator may bundle or defer.

Also positively established: an epoch reset fully erases the divergence (probe P4), so the defect's blast radius is confined to the current epoch's unfinalized state — consistent with the correction report's severity call.

## 3. State-commitment audit (mission goal 2)

Every retained field of every canonical state machine was enumerated and classified (architecture §5.1). Result: **one violation of the equal-digest/equal-behavior invariant exists in the entire Phase-1 tree — the two timeline staged-identity registries** (the known open finding, now with its architecture). Everything else is either directly committed to a canonical digest (scheduler slots with full tracked evidence; timeline history/config/slots; state cells; activation lineage) or a pure derived function of committed state (finalized identity registries; `last_finalized_source_sequence`; `ActivationRegistry::identities`; `StateStore` definitions via content addressing). No `HashMap`/`HashSet`, wall clock, ambient RNG, float, or unordered canonical iteration anywhere in canonical code.

## 4. Order-independence audit (mission goal 3)

Timeline poison evidence, scheduler conflict evidence (at and beyond both caps), activation hashes, config revisions and their duplicate diagnostics, reset evidence, and every drain/report/finalize ordering are order-independent or stable-total-ordered — verified against the existing corpus plus fresh probes. The staged-identity registries are the sole order-dependent exception (the defect). One honestly-stated residual is documented (§4.5): mid-contest interleavings of identity-*reusing* claims can diverge across arrival orders under any admission-time-screening model, but only with **visibly different** digests — never equal-digest hidden divergence — and per-ordered-input determinism always holds. The Codex review plan makes hunting for any equal-digest counterexample an explicit falsification target.

## 5. Simplifications — 3 accepted (1 mandatory, 1 recommended, 1 optional), 1 rejected

- **S1 (mandatory, part of the repair):** stop letting promoted commands' entries linger in the staged registries as duplicates of the finalized entries — removes the last redundant authoritative state in the kernel and makes the derived-index invariant provable.
- **S2 (recommended):** unify the line-for-line-identical `ConflictEvidence`/`SlotPoisonEvidence` implementations into one shared bounded-claim-set type in `spark-core`; their non-drift is currently guarded by comments where a type could stand. No public surface or encoding changes.
- **S3 (optional):** simplify the provably-always-true retention bound in post-fence slot cleanup, carrying the proof as a comment.
- **S4 (rejected):** removing the defensive equal-hash tolerance in identity screening — kept; it remains reachable through the finalized-registry path.

## 6. Exact next Opus/Codex workflow

1. **Operator** grants the authorization in §8 below (and decides the §7.2 bundling option).
2. **Opus writer pass** (Claude Code, Opus, HIGH) executes `PHASE_1_OPUS_FINAL_IMPLEMENTATION_PLAN_2026-08-26.md` on `phase1-refoundation-v2`: Step 1 encodes the AT-H corpus from `PHASE_1_FINAL_CLOSURE_TEST_MATRIX_2026-08-26.md` against the inherited tree and records the red baseline (expected: AT-H1/H2/H4 red, AT-H5–H9 green — any other split stops the pass); Step 2 lands the ~15-line repair (+S1, + bundled options); Step 3 runs the full gate battery; Step 4 commits the completion report and status update. Hard stops as listed; no digest, evidence, scheduler, or contract changes permitted.
3. **Codex independent review** (HIGH) per `PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_PLAN_2026-08-26.md`: section-by-section conformance to the admission architecture, red-baseline replay, fresh interleaving-sweep falsification, a from-scratch hidden-state hunt, closed-item regression table, full gate re-run. Verdict vocabulary: `PHASE_1_CLOSED` / `REVISE_PHASE_1` / `ESCALATE_ARCHITECTURE_PROCESS_REVIEW`.
4. On `PHASE_1_CLOSED`: update `PHASE_STATUS.md`; Phase 2 still requires its own operator authorization and begins with a Fable architecture pass per the readiness blueprint.

## 7. Is the Phase-1 closure architecture ready?

**YES.** The remaining defect is fully characterized (both registries, all facets), its repair is specified to the transition table and lemma level with rejected alternatives on record, the acceptance corpus is written with a predicted red/green split, the writer plan is bounded, and the review plan is falsification-grade. No frozen Phase-0 contract needs to change. The only thing between here and the closure pass is the operator authorization that the previous pass explicitly reserved — hence the final verdict below.

## 8. Operator decisions required — 2 (1 blocking, 1 option)

1. **BLOCKING — authorize the Phase-1 final closure implementation pass** (the B-01 admission-semantics repair per the admission architecture §3, with contested identities becoming unclaimed after a contest, plus S1 and the AT-H corpus). This is the "separate operator authorization" the final digest correction report reserved; nothing in this mission could or did pre-empt it.
2. **OPTION — bundling:** include m-02 (manifest-hash multiset fix), S2 (evidence-type unification), and S3 (retention-bound simplification) in the same pass — **recommended**, all small and contract-safe — or defer them, recording m-02 as accepted debt.

## 9. Phase-2 readiness summary

`engineering/phase2/PHASE_2_FABLE_READINESS_BLUEPRINT_2026-08-26.md` records: every Phase-2 entry seam already exists and is named (evaluator landing zone behind the `pub(crate)` write paths, activated-schema ground truth, deterministic work feed, qualified randomness, panic-free template, batch-API contract); seven binding inherited invariants (led by R1/R2 with mandatory AT-G-style tests for every new retained store); seven enumerated open questions reserved for the Phase-2 architecture pass; and five inherited debts (Windows/Android executable parity foremost). Verdict: **PREPARED, PENDING PHASE-1 CLOSURE** — no architectural unknown blocks Phase-2 planning; Phase 2 remains unauthorized.

## 10. Deliverables index

| # | Artifact |
|---|---|
| 1 | `engineering/phase1/PHASE_1_FABLE_FINAL_ADMISSION_ARCHITECTURE_2026-08-26.md` |
| 2 | `engineering/phase1/PHASE_1_FINAL_CLOSURE_TEST_MATRIX_2026-08-26.md` |
| 3 | `engineering/phase1/PHASE_1_OPUS_FINAL_IMPLEMENTATION_PLAN_2026-08-26.md` |
| 4 | `engineering/phase1/PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_PLAN_2026-08-26.md` |
| 5 | `engineering/phase2/PHASE_2_FABLE_READINESS_BLUEPRINT_2026-08-26.md` |
| 6 | this synthesis |
| evidence | `engineering/phase1/fable-overnight-baseline/FABLE_OVERNIGHT_PROBE_EVIDENCE_2026-08-26.txt` |

Repository copies are canonical; the `~/Downloads` synthesis, tarball, and SHA-256 are disposable handoff copies.

## 11. Final verdict

**OPERATOR_DECISION_REQUIRED** — the closure architecture is complete and ready to execute; it awaits only the reserved authorization (§8.1). No architecture change is required.

PHASE_2_IMPLEMENTATION_AUTHORIZATION: NO
