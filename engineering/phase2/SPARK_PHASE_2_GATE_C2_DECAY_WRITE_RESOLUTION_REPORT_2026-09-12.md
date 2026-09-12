# S.P.A.R.K. Phase 2 — Gate C2 decay-write resolution: writer report

**Date:** 2026-09-12. **Written by:** the separated Gate C2 writer (Claude Code, Opus 5,
`claude-opus-5`), in an isolated worktree on
`candidate/phase2-gate-c2-decay-write-resolution-20260912`.
**Authority:** `SPARK_GATE_C2_DECAY_WRITE_RESOLUTION_MISSION_2026-09-12.md`, recorded at
`a3227eb423e75851b05b9992dfde6424a7d8932d`.
**Status:** candidate for independent review. This report accepts nothing, promotes no
production, and begins no Phase-3 work. Gate C2 acceptance remains the Operator's.

## 1. Lineage and custody

| Commit | Role |
|---|---|
| `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` | Production and Operator acceptance freeze (`phase1-refoundation-v2`); retained ancestor |
| `544c5f6f99d8dabf9855f8ac68f5666286aa9741` | Previously reviewed candidate (decay adjudication) |
| `e00f248e25f6d34f1041e219f74085649714c19d` | Controlling independent review, verdict `GATE_C2_DECAY_ADJUDICATION_ACCEPTABLE_FOR_OPERATOR_ACCEPTANCE` |
| `a3227eb423e75851b05b9992dfde6424a7d8932d` | Mission checkpoint: the Operator's resolution and this writer mission, recorded before implementation |
| `ddee0a2…` | Checkpoint 1: the implementation contract, alone, before any production change |
| `a9c15fd…` | Checkpoint 2: the production change and the adapted reference model |
| `b1608ddc4470d73b9c9eaf5ce26c61697000be38` | Checkpoint 3: the new normative test suite — the last commit touching any crate file |

`custody.json` pins every controlling ref against live GitHub; all eight matched their
expected hashes at capture. Every worktree and branch listed there was preserved. This
document cannot contain its own commit hash; the publication receipt supplies it.

## 2. What the Operator resolved, and what changed

The mission records the Operator's acceptance of the recommendation distinguishing additive
updates from explicit replacements, and settles the four matters that
`SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md` §5 expressly left open.
The implementation contract, recorded before any production change, maps each approved
behavior onto the actual write paths.

| Approved behavior | Disposition |
|---|---|
| 1. An additive change settles applicable overdue decay/recovery first, then applies its delta | **Changed.** The only production change in this candidate |
| 2. An explicit replacement sets the declared value, which earlier decay never reduces; neither write restarts the grid | **Already conformant**; now normative and tested |
| 3. Removal suspends accrual, restoration starts a fresh grid, closing whole steps stay applicable, residual discarded | **Already conformant**; now normative and tested, and subject to behavior 1 |
| 4. Same-time activations in committed order; change-and-reversal restarts the grid, unchanged parameters preserve it | **Already conformant**; now normative and tested |
| 5. A single decay operation on an absent cell creates neither cell nor effect | **Already conformant**; now normative and tested |

Behaviors 2–5 are exactly the behavior the controlling review executed and reported in its
§5. This candidate does not re-derive them; it promotes them from reported implementation
behavior to checked-in requirements, which is what the resolution makes them. That is also
why the controlling review's own probes for removal/restoration, same-time activations and
absent cells still pass **byte-unchanged** against this tree.

## 3. The production change

Two files change, adding 104 lines and replacing exactly one:

| File | Change |
|---|---|
| `crates/spark-engine/src/rules.rs` | `+19` — `ActivatedRuleSet::decay_operation(target)`, the single decay/recovery operation on a target as `(rule, qualified sub-ID, scope mapping)`. Admission guarantees uniqueness (v2 §4.3, `SecondDecayReducer`) |
| `crates/spark-engine/src/engine.rs` | `+85 −1` — `EvalView::settlement_operation`, `EvalView::settled`, and one settled pre-value per `(definition, scope)` computed in `plan_wave` before reduction |

`EvalView::settled(definition, scope, now)` returns the value the target's decay operation
would commit at `now`, the raw committed value when the retained lineage carries no decay
operation for the target, or `None` for an absent or non-numeric cell. It reuses `decay_walk`
and `baseline_of` **unchanged** — both are pinned byte-identical against the checkpoint by
`check_preservation.py` — so every D-1 … D-7 property is the same computation an explicit
evaluation performs: segment origins, endpoint ownership at a barrier, discarded residual,
linear floor-exact `i128` steps, exact baseline clamping.

`settlement_operation` resolves the identity from the **latest** epoch of the retained
lineage that carries one. In the ordinary case that is the current epoch. Searching backwards
is what makes behavior 3 hold for an additive write that lands while the operation is
removed: the identity still resolves, `decay_walk` integrates nothing over the epochs that
lack the operation, and the closing segment's whole steps stay applicable. The
`RES-current-epoch-only` mutant shows this is load-bearing, not decoration.

The one replaced line is the reducer's pre-wave read:

```
-        let reduced = reduce(&canonical, &|d, s| view.cell(d, s))?;
+        let reduced = reduce(&canonical, &|d, s| settled.get(&(d, s)).copied())?;
```

`effects::reduce` itself is untouched — its signature, its `pre` contract and its inline
tests are unchanged — and `pre` is consulted only by the ADDITIVE branch, so no other
family's arithmetic can change. Settlement is computed once per `(definition, scope)` before
reduction, so several additive contributions to one target settle exactly once.

Properties preserved, each with a named test or mutant:

- **No second charge at an endpoint.** A later evaluation reads `updated_at = now` and sees
  only points in `(now, …]`. `settlement_bills_the_barrier_endpoint_once_at_the_old_rate`
  runs the same barrier history with and without a scheduled evaluation at the barrier and
  requires both to agree.
- **No backdated commit, no new field, no encoding change.** The grid stays a function of the
  committed activation lineage (adjudication S-4); nothing is written to `StateCell` beyond
  the single committed effect at the cohort's canonical time (D-6, FINAL §4).
- **Failure atomicity.** A settlement failure is a `WaveRejection` raised before any commit.
  `a_refused_wave_settles_nothing` checks both a retained cross-family rejection and a bounds
  refusal and requires the cell to keep its value *and* its commit time.
- **The forbidden family path is untouched.** Settlement is not routed through it; the
  coincident scheduled decay-plus-shock rejection still holds atomically.
- **Watchers observe committed transitions.** `Trigger::Crossing` still compares the committed
  pre-wave value with the committed new value. Settlement produces exactly one committed
  transition; comparing against the settled intermediate would invent an unreported state
  change. `RES-watcher-sees-settled-intermediate` is the control.
- **Determinism.** The settled value is a function of committed state alone, so replay,
  snapshot/restore and every pacing budget reproduce it.

## 4. Boundaries deliberately not crossed

The resolution names two categories. Two implemented categories are neither, and this
candidate leaves them alone rather than inventing architecture (implementation contract §5):

1. **`Scale` and `Clamp`** resolve from the committed value without being an addition or a
   declared replacement. `scale_keeps_its_existing_semantics` pins this.
2. **A composed rule body** folds its declared stages in declared order. When the body carries
   a `Decay` stage — the v2 §4.3 sanctioned additive-plus-decay composition — that stage
   already applies the points in `(updated_at, now]`, so an implicit settlement would charge
   the same endpoints twice. A profile expresses settle-then-add by declaring the decay stage
   first. `a_composed_body_folds_its_declared_stages_in_order` pins this with a vector that
   discriminates the two orders through the baseline clamp (3, not 5).

**A body without a decay stage that contains additive stages therefore still forfeits earlier
unapplied steps.** That is unchanged behavior, disclosed here rather than silently extended,
and it is a candidate for a future Operator decision. It is not presented as a ruling.

## 5. Superseded interpretation

The adjudication's §5.2 observation — grid steps ending before a non-decay write that no
evaluation preceded "are never applied" — described behavior the Operator had expressly not
decided. For **additive** writes the Operator has now decided against it. From this candidate
it is **historical behavior, not a normative requirement**:

- it is asserted as a requirement nowhere in the checked-in suite;
- `additive_settlement_kills_the_former_lost_debt_reading` states its exact tuples and
  requires the engine not to produce them;
- the `RES-lost-debt` mutant restores it and is killed by eight tests;
- for **explicit replacement** the observation still holds and is now normative under
  behavior 2.

The controlling review's `own_probes.rs` runs **byte-unchanged** and fails at exactly one
test, `own_open_lost_prewrite_step`. The adapted copy differs by **one literal on one line**
(`95` → `85`) and passes 9/9. The original remains byte-identical in the review's own
evidence directory. All 627 pre-existing `engineering/` files at the checkpoint are
byte-identical.

The pinned discriminator of the resolution: baseline 0, 100 at 0, rate 10, cadence 10; `+5`
at 15 gives `(95, 15)` and the evaluation at 20 gives `(85, 20)`; explicitly evaluating decay
at 10 first gives the same numbers; replacing with 100 at 15 instead gives `(100, 15)` and
`(90, 20)`. Recovery mirrors all of it. These are tuple vectors, not a promise of
full-history digest equality between different event histories.

## 6. New normative tests

`crates/spark-testkit/tests/phase2_decay_write_resolution.rs` — 15 tests, every vector
hand-valued, every applicable vector run in both directions, all through the public door
(commands for writes and activations, scheduled work for evaluations, shocks and composed
bodies).

| Test | Requirement discriminated |
|---|---|
| `an_additive_write_settles_overdue_decay_before_applying_its_delta` | Behavior 1; the two histories must agree numerically |
| `additive_settlement_kills_the_former_lost_debt_reading` | The negative control: the superseded tuples stated and rejected |
| `an_explicit_replacement_is_never_reduced_by_earlier_decay` | Behavior 2, against behavior 1 in the same history |
| `settlement_counts_exactly_the_grid_points_that_have_ended` | Aligned, unaligned and not-yet-due writes; the endpoint at `now` included |
| `repeated_and_co_firing_additions_never_charge_a_grid_point_twice` | Repeated additions; two additive rules in one wave settle once |
| `settlement_saturates_at_the_baseline_and_is_inert_at_rate_zero` | Nonzero baseline, saturation, rate 0 |
| `removal_suspends_accrual_and_restoration_starts_a_fresh_grid` | Behavior 3, including an additive write inside the absent window |
| `same_time_activations_are_processed_in_committed_order` | Behavior 4, changed and unchanged, plus an additive write at the barrier time |
| `settlement_bills_the_barrier_endpoint_once_at_the_old_rate` | D-4/D-5 under settlement; no second charge |
| `a_decay_evaluation_never_creates_an_absent_cell` | Behavior 5, and an additive write with nothing to settle |
| `a_composed_body_folds_its_declared_stages_in_order` | Boundary 2, discriminated through the baseline clamp |
| `scale_keeps_its_existing_semantics` | Boundary 1 |
| `a_watcher_sees_one_committed_transition_and_the_reported_effect_matches` | Threshold crossing and AT-I1 effect reapplication |
| `a_refused_wave_settles_nothing` | Refusal atomicity, cross-family and bounds |
| `replay_restore_and_pacing_reproduce_settlement` | Determinism on identical histories |

`phase2_decay_adjudication.rs` keeps its 10 tests and its hand-written vectors unchanged.
Its independent reference model — which enumerates grid points one at a time, uses no
division and shares no code with the engine — now settles additive events the same way, in
17 changed lines. Its generated 40-timeline sweep, which includes assignments,
reassignments, shocks, and changed and unchanged activations, agrees with the engine under
the new semantics. That agreement is cross-validation by a second implementation, not a
restatement of `decay_walk`.

## 7. Validation

All 15 required checks exit 0, on tree `b1608ddc4470d73b9c9eaf5ce26c61697000be38`.
`checks.json` records exact commands, exit codes and durations; the logs retain complete
whitespace-normalized output.

| Required check | Result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo test --workspace --all-features --no-fail-fast` | **415 passed, 0 failed, 0 ignored** |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| Strict core/engine all-feature library lint | PASS; warnings, unwrap, expect, panic, indexing/slicing and arithmetic-side-effects denied |
| `cargo metadata --format-version 1` | PASS |
| `git diff --check` over the checkpoint, production and review ranges | All three PASS |
| Windows x86_64 GNU/MSVC `cargo check --workspace --all-targets` | Both PASS (static only) |
| Android aarch64/armv7/x86_64 same static checks | All PASS |
| Preservation | PASS, no failures |
| Release offline `gate_c2_workload` | PASS |
| Controlling review's own probes, byte-unchanged | 8 passed, **1 expected failure** at the superseded observation |
| Resolution-adapted copy (one literal) | 9 passed, 0 failed |
| Prior decision-adapted probe corpus | 9 passed, 0 failed |
| Mutation controls | 32 baseline tests pass; **17/17 compiled and killed**, 45 assertion kills; 2 specified prior-test survivals hold |

The 17 mutants are the preceding pass's 10 with byte-identical patches — the fixed-grid walk
is reused unchanged, so every D-1 … D-7 discriminator must still kill them — plus 7 new ones:
`RES-lost-debt`, `RES-decay-reduces-replacement`, `RES-watcher-sees-settled-intermediate`,
`RES-settle-composed-body`, `RES-settle-scale`, `RES-current-epoch-only` and
`RES-exclude-endpoint-at-the-write`. Each compiled and was killed by assertion, never by a
relabelled compile failure. `C2-05-relative-steps`, the superseded elapsed-since-write count,
is additionally killed by two of the new resolution vectors.

Preservation: `spark-core` untouched; `Cargo.toml`, `Cargo.lock` and all three crate
manifests untouched; the seven inherited integration test files, 17 inline Phase-1 test
modules and all protected functions (`schedule`, `drain_due`, `stage`, `submit_fence`,
`derived_indexes_consistent`, `decay_walk`, `baseline_of`) byte-identical; exactly four
crate paths changed; all 627 pre-existing `engineering/` files byte-identical.

## 8. Performance and retained limitations

Release `gate_c2_workload`, one four-core Linux host, with contemporaneous build load. These
are not production latency guarantees.

| Finalized history | History build (s) | Indexed preflight (µs/call) | Scan control (µs/call) | Whole finalization (ms/call) |
|---:|---:|---:|---:|---:|
| 1,000 | 0.325 | 2.373 | 5.392 | 0.737 |
| 4,000 | 6.364 | 2.443 | 39.117 | 3.497 |
| 16,000 | 112.785 | 2.482 | 902.733 | 13.240 |

Settlement walks the retained activation lineage once per written `(definition, scope)` per
wave, the same walk an explicit evaluation performs; it adds no new scan class. Whole
finalization remains O(history) through preserved Phase-1 code, and pre-wave digests still
scan stores. Every prior physical-scan limitation stands.

Retained limitations, unchanged and not narrowed by this pass: AT-I13 remains P —
fixed-manifest support without definition migration. AT-I39/AT-I40 remain per-profile
composition, not a shared multi-profile runtime or cross-profile transaction. AT-I21 depth
overflow uses the test seam; AT-I33's maximum is the configured finite bound. Replay and
snapshot/restore are in memory only: there is no durable persistence, crash recovery,
mailbox, transport, service or G.A.M.E. integration. Windows and Android coverage is static
compilation only. C2-09 remains the disclosed finalization/history performance limitation.

## 9. Open matters and the exact next action

No Operator decision is pending for this candidate. One behavior is disclosed as outside the
resolution's two named categories and unchanged: a composed rule body without a decay stage
that contains additive stages still forfeits earlier unapplied grid steps (§4). It is
reported, not ruled on.

The writer's own checks are **not** an independent review. The exact next action is the
independent review described in
`SPARK_PHASE_2_GATE_C2_DECAY_WRITE_RESOLUTION_INDEPENDENT_REVIEW_MISSION_2026-09-12.md`,
performed by an agent other than this writer. Gate C2 acceptance remains pending until that
review completes and the Operator decides. No production promotion and no Phase-3 work
follows from this candidate.
