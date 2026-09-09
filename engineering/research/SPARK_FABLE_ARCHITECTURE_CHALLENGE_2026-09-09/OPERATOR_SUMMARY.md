# Operator summary — Fable architecture challenge of S.P.A.R.K.

**RESEARCH / PROPOSED DESIGN — NOT ADOPTED.** 2026-09-09. Baseline `phase1-refoundation-v2`
at `47729bb719d8ae95ae00c4e195fa64d0e27bfdb7` (clean, 232/232 tests). Research worktree
`/home/chromikey/Projects/SPARK-fable-challenge-worktree`, branch
`research/fable-architecture-challenge-2026-09-09`. Production branch untouched.

## Opinion

1. **Phase 1 is good and should be kept whole.** The scheduler, clock, hashing, ids, activation
   door, and state store are exactly the parts a deterministic discrete-event engine needs.
   The executable example runs on the real kernel with one 10-line read-only accessor.
2. **V3-F01 is a spec/mechanism mismatch, not a runtime defect, and it has a two-line fix:**
   iterate `drain_due(least_due_time)` instead of calling `drain_due(horizon)`. The
   compare-and-take protocol, slot fingerprints, extraction slices, frontier `Φ`, cursor `X`,
   and ceiling `C` are unnecessary. The example reproduces the defect (W1) and the fix (C2).
3. **The serialized-request addendum is right about the shape and wrong about one rule.**
   Keeping the frontier unchanged by a pause lets a lost-mailbox host write time backwards
   (check S1). The clock must advance as cohorts commit. No new state is needed; `F` is the
   existing `LogicalClock`.
4. **Phase 2's real blocker is process, not architecture:** 9,665 lines of specification and
   no loop. Acceptance oracles written for nonexistent code produced the "false assertion"
   findings. The persistence surface required by ADR-0006 and by any game save was never
   noticed because nothing ever needed to persist.
5. **Generality to trim (as proposals):** one engine per profile; emission identity and
   parent-set digests deferred to an explanation feature; the multi-sequencer timeline kept
   but used one command at a time; cohort identity replaced by `WorkKey` identity if D1 is
   decided per-item.

## Recommended route: **focused refactor** (details in `05_ROUTE_AND_PLAN.md`)

Keep Phase 1 verbatim; add `rules/`, `evaluate/`, `request/` to `spark-engine` and a thin
`spark-host` crate; replace the Phase-2 prose acceptance matrix with executable fixtures
against the loop; close V3-F01 by construction. First game-sandbox demonstration
(drought → scarcity → hunger → choice → hunting → wildlife) is reachable in the order of
two to three focused implementation passes plus one independent review, not another
adjudication cycle.

## Decisions for the Operator (everything else is engineering)

| Id | Decision | Recommendation |
|---|---|---|
| D1 | transaction unit: equal-time slice (frozen v3) or per-`WorkKey` | per-`WorkKey` |
| D2 | late-dated host inputs: strict reject vs. adapter re-date with `observed_at` | strict in engine, re-date in adapter |
| D3 | snapshot codec: home and format | `spark-core` encode surfaces, `spark-host` codec, versioned header with activation hash |
| D4 | replace the Phase-2 prose acceptance matrix with executable fixtures as the Gate C2 oracle | yes |
| D5 | one engine instance per profile (drops multi-profile cohort sequencing) | yes |

## Archive contents

`OPERATOR_SUMMARY.md`, `00_FIRST_PRINCIPLES_SKETCH.md`, `01_BASELINE_AND_CRITIQUE.md`,
`02_FABLE_DESIGN.md`, `03_EXAMPLE_RESULTS.md`, `04_SELF_CHALLENGE.md`,
`05_ROUTE_AND_PLAN.md`, `MANIFEST.md` (baseline, inventory, checksums, commit hashes),
`example/` (Rust, run instructions inside `03_EXAMPLE_RESULTS.md`), `dev_compute_evidence/`
(supplementary compute, non-authoritative), `scheduler_accessor.patch`.
