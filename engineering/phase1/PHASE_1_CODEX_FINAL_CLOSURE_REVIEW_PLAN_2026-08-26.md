# S.P.A.R.K. Phase 1 — Final Closure Independent Review Plan (Codex)

**Date:** 2026-08-26
**Reviewer:** Codex — HIGH effort, independent; writer self-assessments carry no evidentiary weight; repository behavior outranks every report.
**Trigger:** runs after the Opus final closure pass (`PHASE_1_OPUS_FINAL_IMPLEMENTATION_PLAN_2026-08-26.md`) completes.
**Controlling documents:** the Fable final admission architecture (conformance target), the AT-H closure test matrix (coverage target), ADR-0003/0005 and the frozen determinism constitution (contract ceiling).

---

## 1. Mandate

This is the review that decides Phase-1 closure. Its center of gravity is one question:

> After the repair, is the timeline ingress's **complete** retained state — including every identity registry — a function of digest-committed state, arrival-order-independent per claim set, and did nothing previously closed regress?

## 2. Required conformance checks (section-by-section against the architecture)

1. **§3.1 transition table** — verify each of the seven transitions in the repository source, not the report: registration only in the empty-slot branch; both entries removed on the Staged→Poisoned transition (taken from the slot's envelope, not the claimant); both entries removed at promotion; claimant never registered; poison-branch evidence insert unconditional; epoch reset clears both.
2. **§3.2 lemma** — confirm the removal cannot evict a foreign registration (check the screening still guarantees at-most-one distinct staged envelope per identity key at all times, including across a window slide).
3. **§3.4 non-changes** — diff-level confirmation that no digest encoding, public signature, error variant, evidence representation, fence-validation ordering, or scheduler line changed (beyond authorized S2/S3/m-02 if bundled; S2 must leave every canonical encoding and public accessor byte-identical, provable by the untouched AT-G suite).
4. **Test-first discipline** — re-run the AT-H corpus against the extracted pre-repair commit; the recorded red baseline must reproduce exactly (AT-H1/H2/H4 red, AT-H5–H9 green).
5. **Matrix coverage** — every AT-H test present with the mandated assertions (digest equalities and disposition-value equalities, not error-shape checks); the "explicitly unchanged tests" checklist verified unmodified via `git diff`.

## 3. Required independent falsification (beyond the corpus)

Construct fresh probes, not copies of the writer's tests:

1. **Interleaving sweep:** for claim sets of 2–4 envelopes over 1–2 ordinals, enumerate all arrival permutations; assert equal final state digests per per-slot claim set and equal responses to a battery of subsequent probes (contested-ID reuse, fresh IDs, fence attempts, epoch reset then resubmission). Include permutations where contests interact with a window slide (partial-prefix fence between claims).
2. **Mid-contest reuse residual (§4.5):** verify the honestly-stated residual is exactly as stated — a mid-contest reuse may diverge across interleavings **only** with visibly different digests (never equal digests with different future behavior). Any equal-digest/different-behavior pair anywhere in the sweep is a MAJOR finding.
3. **Hidden-state hunt, repeated from scratch:** re-run the mission's audit style over the post-repair tree — enumerate every retained field of `TimelineIngress`, `Scheduler`, `ActivationRegistry`, `StateStore`, and prove each is either digest-committed or a pure function of digest-committed state. The staged registries must now fall in the second class.
4. **Evidence-screening trap (AT-H5):** attempt a variant implementation check — if any code path consults the identity registries before inserting poison evidence, fail conformance (§4.3).
5. **Finalized-identity permanence:** contested-then-reset-then-reused identities must never collide with *finalized* identity uniqueness; probe reuse of finalized IDs across epoch resets.

## 4. Closed-item regression table (must all be verified, not assumed)

B-01 (admission/finality semantics, now including the repaired registries), B-02 (activation door untouched, external compile probes pass), B-03 (scheduler untouched, digest-complete), B-04, M-01, M-02 (digest model incl. AT-G), M-03 (strict lint gate re-run by the reviewer), M-04, m-01 (dependency/feature hygiene), reconstruction restriction (AT-D), Fable crate boundaries.

## 5. Tooling (reviewer re-runs everything)

fmt, clippy (both variants), the explicit strict canonical lint gate, full workspace tests, metadata policy, all five Windows/Android static targets. Scope audit: no Phase-2 machinery, no new dependency, no new public surface.

## 6. Verdict vocabulary

- `PHASE_1_CLOSED` — repair conforms, corpus sufficient, no new finding at MAJOR-or-above, no regression. This verdict, and only this verdict, ends Phase 1.
- `REVISE_PHASE_1` — bounded defects in the pass; enumerate with severity and exact evidence.
- `ESCALATE_ARCHITECTURE_PROCESS_REVIEW` — only if a finding contradicts the admission architecture itself (per the standing convergence rule: same-class third failure escalates to Fable, not to another writer loop).

Report must additionally state: Phase-2 authorization recommendation (which remains NO until the operator separately authorizes Phase 2 even under `PHASE_1_CLOSED`), residuals carried forward, and the exact HEAD reviewed.
