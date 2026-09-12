# S.P.A.R.K. Phase 2 — Gate C2 C2W-01: correction to the decay-write implementation contract

**Date:** 2026-09-12.
**Written by:** the separated Gate C2 writer (Claude Code, Opus 5, `claude-opus-5`).
**Authority:** the existing Operator authorization recorded in
`SPARK_GATE_C2_DECAY_WRITE_RESOLUTION_MISSION_2026-09-12.md`, and the controlling
independent review
`SPARK_PHASE_2_CODEX_GATE_C2_DECAY_WRITE_RESOLUTION_INDEPENDENT_REVIEW_2026-09-12.md`
at `e39690dc51b63fa09fe7c3f30ebd48aa2c16686f`, verdict
`GATE_C2_DECAY_WRITE_RESOLUTION_REQUIRES_BOUNDED_REVISION`, finding **C2W-01**.
**Status:** an **additive correction**. It records the corrections before the production
change and is preserved as history afterwards.

This document does not rewrite anything. `SPARK_PHASE_2_GATE_C2_DECAY_WRITE_IMPLEMENTATION_CONTRACT_2026-09-12.md`,
`SPARK_PHASE_2_GATE_C2_DECAY_WRITE_RESOLUTION_REPORT_2026-09-12.md`, the immutable
2026-09-11 adjudication, the 2026-09-12 Operator resolution and every published evidence
directory stay byte-unchanged. Where this record and the earlier contract disagree, **this
record controls**, and the earlier text stands as history.

## 1. What the review established

C2W-01 is a conformance failure, not a policy question. A rule body that performs an
additive change and finishes with a transform stage was lowered to a pre-resolved
`TransformFamily::RuleBody` intent, and `effects::reduce` consults the settled pre-value
only on its ADDITIVE branch. The additive stage therefore never saw settlement, and
committing `updated_at = now` permanently excluded the unapplied endpoints.

Two executed public-door vectors, baseline 0, rate 10 / cadence 10, no evaluation before
the write:

| Existing cell | Body at 15 | Required | Previously produced |
|---|---|---|---|
| `100@0` | `Add(+5); Clamp[-1000,1000]` | `(95, 15)` then `(85, 20)` | `(105, 15)` then `(95, 20)` |
| `-100@0` | `Add(-5); Scale(1)` | `(-95, 15)` then `(-85, 20)` | `(-105, 15)` then `(-95, 20)` |

Both bodies are additive events with an identity finishing stage. Neither declares an
explicit replacement or an explicit decay stage. An identity transform cannot convert an
additive event into an exception to approved behavior 1.

## 2. Corrections to the earlier contract

### 2.1 Contract §5, boundary 2 — superseded in part

The earlier contract said a composed rule body keeps its existing semantics, and disclosed
that "a body without a `Decay` stage that contains additive stages therefore still forfeits
earlier unapplied steps … a candidate for a future Operator decision."

**That disclosure is withdrawn.** It was not deferral authority, and no further Operator
decision is required. The corrected rule is:

> A composed rule body that performs an **additive change** and declares **no debt-consuming
> `Decay` stage** is an additive event. Its **starting value** is the value settled to the
> cohort's canonical time; its declared stages then fold in declared order from that value.

The part of boundary 2 that survives is narrower and is **retained**: a body that **does**
declare a `Decay` stage already accounts for the grid in declared stage order, so no
implicit settlement is applied to it.

### 2.2 The double-charge argument — corrected in scope

The earlier report's reasoning for exempting *all* bodies was too broad, and one sentence of
its test commentary was wrong. The review is correct on both points:

- The double-charge argument holds only for a body that declares a `Decay` stage. That stage
  passes the cell's own `updated_at` to `decay_walk`; nothing marks the endpoints consumed
  inside the body, so prepending settlement there would charge the same endpoints twice.
  For `20@5` with rate 3 / cadence 6 evaluated at 12, settling to 14, adding 5 and walking
  endpoints 6 and 12 again gives 13 instead of the correct 19.
- The earlier claim that a declared decay stage would then "have nothing to do" was
  **inaccurate**, and is corrected here. It does not affect any committed value, because no
  implicit settlement was ever applied to such a body; it was a wrong explanation of a
  correct behavior.
- For a body with **no** `Decay` stage there is no second walk at all, so no endpoint can be
  charged twice. The exemption never had a basis there.

### 2.3 Contract §5, boundary 1 — retained unchanged

A **standalone** `Scale` or `Clamp`, reduced as a single-operation `Transform`, resolves from
the committed value. It is neither an additive change nor a declared replacement, and the
review accepted this boundary explicitly. It is unchanged. A body consisting only of
transform stages, with no `Add` or `Subtract`, likewise settles nothing: there is no additive
event in it. The presence of a transform **alongside** an `Add` does not exempt that additive
composition — that is the whole of C2W-01.

### 2.4 Explicit replacements inside a body

Settlement applies to the body's **starting value** only. A declared `Assign` or `Aggregate`
stage overwrites whatever precedes it, so a declared replacement is never reduced by earlier
decay, wherever it appears in the stage order. This is stage-aware by construction, and it
satisfies the review's requirement that the finding "does not demand charging debt against a
declared replacement."

### 2.5 Contract §2, write-path table — corrected rows

| Path | Corrected family and settlement |
|---|---|
| Composed rule body with an `Add`/`Subtract` stage and **no** `Decay` stage | Additive composition. **Settles its starting value, once.** |
| Composed rule body with a `Decay` stage | Unchanged: declared stage order, the decay stage consumes the debt, no implicit settlement |
| Composed rule body of transform stages only | Unchanged: no additive event, no settlement |
| Standalone `Scale` / `Clamp` | Unchanged: resolves from the committed value |

The earlier §2 wording "a composed rule body … **No** — see §5" is superseded for the first
row only. Every other row of that table is confirmed by the review and stands.

### 2.6 One wording caveat confirmed

The review noted that `plan_wave` eagerly *computes* settlement for every canonical target,
including replacement and transform targets, even though only the ADDITIVE branch consumes
the result. The earlier contract's "an explicit replacement never consults this" is accurate
about reducer arithmetic but not about helper execution. That is recorded here as a wording
correction; no reachable defect follows from it, and the behavior is unchanged.

## 3. Invariants this correction must not disturb

The corrected rule changes the body's **starting value** only. Everything else is preserved,
and each item is pinned by a test in this candidate:

- **Declared stage order** inside every body, including bodies with a `Decay` stage.
- **Explicit replacement** unreduced by earlier decay, anywhere in the stage order.
- **Single charging.** A settled body commits `updated_at = now`, so a later walk sees only
  endpoints in `(now, …]`; a body with a decay stage still charges its endpoints exactly once
  through that stage.
- **Canonical time.** One committed effect at the cohort's canonical time (D-6, FINAL §4);
  no backdated commit, no `StateCell` field, no encoding change.
- **Atomic refusal.** Settlement is read-only inside `plan_wave`; a failure is a
  `WaveRejection` raised before any commit, so a refused wave settles nothing — neither value
  nor commit time. `FamilyMixture` is unchanged and is not a settlement path.
- **Committed-to-committed watchers.** `Trigger::Crossing` still compares the committed
  pre-wave value with the committed new value. The review confirmed this is the correct
  retained reading of V2 §4.2 and V3 §4.3; the settlement intermediate is neither a committed
  state nor an effect.
- **Fixed grid.** `decay_walk` and `baseline_of` are reused byte-unchanged, so D-1 … D-7 —
  segment origins, barrier endpoint ownership, discarded residual, linear floor-exact `i128`
  steps, baseline clamping — are the same computation an explicit evaluation performs.

## 4. Scope

This correction resolves C2W-01 and nothing else. It accepts no gate, promotes no
production, begins no Phase-3 work, and opens no new Operator question. Every disposition
inherited by the controlling review — including its explicit inheritance of all AT-I1 … AT-I50
rows from `e00f248e25f6d34f1041e219f74085649714c19d` §6, and every retained limitation —
stands unchanged.
