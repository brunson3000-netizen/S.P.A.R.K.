# S.P.A.R.K. Phase 2 — Gate C2 C2W-01 correction: writer report

**Date:** 2026-09-12. **Written by:** the separated Gate C2 writer (Claude Code, Opus 5,
`claude-opus-5`), in a fresh isolated worktree on
`candidate/phase2-gate-c2-w01-correction-20260912`.
**Authority:** the existing Operator authorization in
`SPARK_GATE_C2_DECAY_WRITE_RESOLUTION_MISSION_2026-09-12.md`, and the controlling
independent review at `e39690dc51b63fa09fe7c3f30ebd48aa2c16686f`, verdict
`GATE_C2_DECAY_WRITE_RESOLUTION_REQUIRES_BOUNDED_REVISION`.
**Status:** candidate for independent review. No gate accepted, no production promoted, no
Phase-3 work begun. No further Operator decision was needed for C2W-01.

## 1. Lineage and custody

| Commit | Role |
|---|---|
| `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` | Production and Operator acceptance freeze; retained ancestor |
| `cc3dc70182b428f7cbc953ae722f85b39d407dfa` | The reviewed candidate |
| `e39690dc51b63fa09fe7c3f30ebd48aa2c16686f` | **Controlling independent review; direct base** |
| `bda58aa…` | Checkpoint 1: the contract correction, alone, before any production change |
| `d1d9f8e55c33ecb5dddccc0e4799cd8f9aaccb40` | Checkpoint 2: the production change and the new regression suite — the last commit touching any crate file |

`custody.json` pins **12** controlling refs against live GitHub; all matched. Production,
every prior candidate and every prior review remain at their pinned hashes, and all 17
worktrees were preserved. This document cannot contain its own commit hash; the publication
receipt supplies it.

## 2. C2W-01 — what was wrong and what changed

The review found a conformance failure, not a policy question: a body performing an additive
change and finishing with a transform stage was lowered to a pre-resolved
`TransformFamily::RuleBody` intent, and `effects::reduce` consults the settled pre-value only
on its ADDITIVE branch. The additive stage therefore never saw settlement, and committing
`updated_at = now` permanently excluded the unapplied endpoints.

The corrected rule, recorded in
`SPARK_PHASE_2_GATE_C2_W01_CONTRACT_CORRECTION_2026-09-12.md` **before** any production
change:

> A composed rule body that performs an **additive change** and declares **no
> debt-consuming `Decay` stage** is an additive event. Its **starting value** is settled to
> the cohort's canonical time; its declared stages then fold in declared order from that
> value.

The production change is **+24 / −1 line in one file**,
`crates/spark-engine/src/engine.rs`. The single replaced line is the body's starting value:

```
-        let mut v = i128::from(current.unwrap_or(0));
+        let declares_decay = …;
+        let is_additive_composition = !declares_decay && …;
+        let start = if is_additive_composition { self.settled(target, scope, ctx.now)? }
+                    else { current };
+        let mut v = i128::from(start.unwrap_or(0));
```

`decay_walk`, `baseline_of`, `settled`, `settlement_operation` and `effects::reduce` are all
reused **byte-unchanged** — pinned as protected functions by `check_preservation.py` — so
every D-1 … D-7 property is the same computation an explicit evaluation performs.

The review's two blocking vectors now produce the required values, and its `own_probes.rs`
runs **byte-unchanged and passes 9/9**:

| Existing cell | Body at 15 | Required | Now produced |
|---|---|---|---|
| `100@0` | `Add(+5); Clamp[-1000,1000]` | `(95, 15)` → `(85, 20)` | `(95, 15)` → `(85, 20)` |
| `-100@0` | `Add(-5); Scale(1)` | `(-95, 15)` → `(-85, 20)` | `(-95, 15)` → `(-85, 20)` |

## 3. What the correction deliberately does not disturb

Each item is pinned by a named test and by a compiled mutant.

| Preserved | Test | Mutant that would break it |
|---|---|---|
| A body that declares a `Decay` stage is not settled and keeps its declared stage order; its endpoints are charged once through that stage | `a_body_that_declares_a_decay_stage_keeps_its_order_and_charges_once` | `W01-settle-decay-body` |
| The accepted standalone `Scale`/`Clamp` boundary, extended consistently to transform-only bodies | `transform_only_paths_are_not_additive_events` | `W01-settle-every-body`, `RES-settle-scale` |
| An explicit replacement inside a body wins in either stage order and is never reduced by earlier decay | `a_declared_replacement_inside_a_body_wins_in_either_stage_order` | `W01-replacement-held-up-by-decay`, `RES-decay-reduces-replacement` |
| Single charging across successive composed writes and at a parameter-changing barrier | `successive_composed_writes_never_charge_a_grid_point_twice`, `a_composed_write_at_a_barrier_bills_the_closing_endpoint_once` | `RES-exclude-endpoint-at-the-write`, `ADJ-drop-closing-endpoint` |
| Canonical commit times; no backdated commit, no `StateCell` field, no encoding change | inherited D-6 suite | `C2R-01-backdated-commit`, `C2R-03-skip-unmoved` |
| Atomic refusal: a refused composed wave settles nothing, value or commit time | `a_refused_composed_wave_settles_nothing` | — (positive control: the next valid write still settles both endpoints) |
| Committed-to-committed watcher comparison | `a_composed_write_is_one_committed_transition_a_watcher_can_see` | `RES-watcher-sees-settled-intermediate` |
| Removal/restoration and the retained lineage search | `a_composed_write_settles_the_closing_segment_while_the_operation_is_removed` | `RES-current-epoch-only` |

The review's double-charge analysis is accepted in full and recorded in the correction §2.2.
It holds only for a body that declares a `Decay` stage — that stage passes the cell's own
`updated_at` to `decay_walk`, so prepending settlement would walk the same endpoints twice
(`20@5`, rate 3 / cadence 6 at 12: 13 instead of 19). The earlier report's claim that such a
stage would then "have nothing to do" was inaccurate and is corrected. For a body with no
decay stage there is no second walk at all, so the exemption never had a basis there.

## 4. New normative tests

`crates/spark-testkit/tests/phase2_decay_write_composition.rs` — 12 hand-valued tests
through the public door. Recovery vectors are written out rather than negated, because
`Scale` floors toward negative infinity and is not symmetric.

| Test | Discriminates |
|---|---|
| `an_additive_composition_settles_before_its_declared_stages` | The review's two vectors and their mirrors, plus a transform-first body |
| `a_composed_catch_up_equals_an_explicit_pre_evaluation` | Catch-up equals an explicit evaluation at the grid point |
| `nonidentity_transforms_resolve_from_the_settled_sum` | `Scale(0.5)` gives 47 (and −48 recovering), not 52; a `Clamp[-96,96]` that would bind only on an unsettled sum stays inert |
| `successive_composed_writes_never_charge_a_grid_point_twice` | Repeated composed writes at 15, 16 and 25 |
| `a_body_that_declares_a_decay_stage_keeps_its_order_and_charges_once` | Both stage orders, separated through the baseline clamp (3 versus 5) |
| `transform_only_paths_are_not_additive_events` | Standalone and transform-only bodies, both signs |
| `a_declared_replacement_inside_a_body_wins_in_either_stage_order` | `Assign` before and after the addition |
| `a_composed_write_settles_the_closing_segment_while_the_operation_is_removed` | Removal at 13, composed write at 20, restoration at 22, both signs |
| `a_composed_write_at_a_barrier_bills_the_closing_endpoint_once` | Barrier at 12 with and without a scheduled evaluation, both signs |
| `a_refused_composed_wave_settles_nothing` | Bounds refusal, then a valid write proving no endpoint was consumed |
| `a_composed_write_is_one_committed_transition_a_watcher_can_see` | Falling threshold at 97 and AT-I1 effect reapplication |
| `replay_restore_and_pacing_reproduce_composed_settlement` | Determinism on identical composed histories |

The 15 tests of `phase2_decay_write_resolution.rs` and the 10 of
`phase2_decay_adjudication.rs` are unchanged and still pass.

## 5. Validation

All 15 required checks exit 0 on the final code-and-test tree.

| Required check | Result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo test --workspace --all-features --no-fail-fast` | **427 passed, 0 failed, 0 ignored** |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| Strict core/engine all-feature library lint | PASS |
| `cargo metadata --format-version 1` | PASS |
| `git diff --check` over the review, production and reviewed-candidate ranges | All three PASS |
| Five Windows/Android static target checks | All PASS (static only) |
| Preservation | PASS, no failures |
| Release offline `gate_c2_workload` | PASS |
| **Controlling review's `own_probes.rs`, byte-unchanged** | **9 passed, 0 failed** — C2W-01 closed |
| Reviewer's materialization probe | 1 passed, 0 failed |
| Historical adjudication-review probes, byte-unchanged | 8 passed, **1 expected failure** at `own_open_lost_prewrite_step`, disposition unchanged |
| Resolution-adapted and decision-adapted corpora | 9/9 each |
| Mutation controls | 42 baseline tests pass; **20/20 compiled and killed**, 57 assertion kills; 2 specified prior-test survivals |

Sixteen mutants are carried forward with byte-identical patches and re-verified; four are
new C2W-01 controls. One prior mutant, `RES-settle-composed-body`, patched the exact line
this correction replaces; re-running it unchanged would be a compile failure relabelled as a
kill, so it is **replaced** by `W01-settle-every-body` rather than dropped, and
`check_evidence.py` asserts both facts.

Preservation: `spark-core`, manifests and lockfile untouched; seven inherited integration
files, 17 inline Phase-1 modules and ten protected functions byte-identical; exactly two
crate paths changed; all **782** pre-existing `engineering/` files byte-identical, including
all four published evidence directories, the immutable adjudication, the Operator
resolution, the earlier contract and report, and the controlling review itself.

## 6. Oracle and retained dispositions

This candidate inherits the controlling review's §5 inheritance unchanged: **every row of
`e00f248e25f6d34f1041e219f74085649714c19d` §6, AT-I1 … AT-I50**, with its exact S/P
qualifications and scope, plus C2-01 … C2-08, C2R-01 … C2R-04, C2R2-01, D-C2-1 … D-C2-15,
R-1 and R-3 … R-9, and the AT-I8/20b/21/28/33/39/40 completions. The one qualification the
review attached to that inheritance — that the added Operator behavior-1 obligation was
**not fully supported** — is what this candidate closes. Approved behaviors 1 … 5 now all
have executed support, on both the simple and the composed additive paths.

Retained limitations, unchanged and not narrowed: AT-I13 remains P (fixed-manifest support,
no definition migration); AT-I39/40 remain per-profile composition; AT-I21 depth overflow
uses the test seam and AT-I33's maximum is the configured finite bound; replay and
snapshot/restore are in memory only, with no durable persistence, crash recovery, mailbox,
transport, service or G.A.M.E. integration; Windows and Android coverage is static
compilation only; C2-09 remains the disclosed finalization/history performance limitation.

## 7. Performance

Release `gate_c2_workload`, one four-core Linux host, with contemporaneous build load. Not a
production latency guarantee.

| Finalized history | History build (s) | Indexed preflight (µs/call) | Scan control (µs/call) | Whole finalization (ms/call) |
|---:|---:|---:|---:|---:|
| 1,000 | 0.356 | 2.493 | 5.261 | 0.835 |
| 4,000 | 7.733 | 2.407 | 40.175 | 4.172 |
| 16,000 | 128.594 | 2.459 | 1,355.370 | 16.280 |

Composed settlement adds no new scan class: it is the same lineage walk an explicit
evaluation performs, once per written `(definition, scope)` per wave, and it runs only for a
body that is an additive composition. Inherited O(history) finalization and pre-wave store
scans remain.

## 8. Open matters and the exact next action

**No open Operator question remains for C2W-01.** The earlier report's disclosure — that a
body without a decay stage may forfeit earlier unapplied steps — is withdrawn by the
correction record; it was not deferral authority and is no longer the behavior.

The writer's own checks are **not** an independent review. The exact next action is the
review described in
`SPARK_PHASE_2_GATE_C2_W01_CORRECTION_INDEPENDENT_REVIEW_MISSION_2026-09-12.md`, performed
by an agent other than this writer. After an independent KEEP verdict, the Operator's next
authorized step is the adversarial test campaign recorded in
`SPARK_PHASE_2_GATE_C2_OPERATOR_NEXT_AUTHORIZED_STEP_2026-09-12.md`: independent testers
record reproducible bugs and evidence against the pinned build for the main engineer, and do
not repair product code. Gate C2 acceptance remains the Operator's, pending that review. No
production promotion and no Phase-3 work follows from this candidate.
