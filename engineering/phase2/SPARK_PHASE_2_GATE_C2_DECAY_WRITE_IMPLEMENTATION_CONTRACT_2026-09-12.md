# S.P.A.R.K. Phase 2 — Gate C2 decay-write resolution: implementation contract

**Date:** 2026-09-12.
**Written by:** the separated Gate C2 writer (Claude Code, Opus 5, `claude-opus-5`).
**Authority:** `SPARK_GATE_C2_DECAY_WRITE_RESOLUTION_MISSION_2026-09-12.md`, recorded at
`a3227eb423e75851b05b9992dfde6424a7d8932d`, which records the Operator's acceptance of the
additive-settlement recommendation and resolves the four matters left open by
`SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md` §5.
**Status:** recorded **before** any production change, as the mission requires. It maps the
actual write paths the approved semantics touch. It adds no semantics of its own; where it
records a boundary the Operator did not decide, it says so.

## 1. What the resolution changes, and what it does not

The mission's five approved behaviors are checked against the implementation at the
checkpoint. Only behavior 1 requires a production change.

| Approved behavior | Implementation at `a3227eb` | Action |
|---|---|---|
| 1. Additive change settles applicable overdue decay/recovery first | An additive write folds its delta onto the raw committed pre-wave value and rebases `updated_at` to `now`, forfeiting every unapplied grid step in `(updated_at, now]` | **Change** (§3) |
| 2. Explicit replacement sets the declared value; earlier decay does not reduce it; neither restarts the grid | `Intent::Result` commits the declared value; the grid is derived from the activation lineage, never from `updated_at` | Preserve; make normative (§4) |
| 3. Removal/restoration: nothing accrues while absent, restoration starts a fresh grid, closing whole steps stay applicable, residual discarded | `decay_walk` sets `segment = None` for an epoch whose artifact lacks the operation, and takes a fresh origin at the restoring barrier | Preserve; make normative (§4) |
| 4. Same-time activations processed in committed order; change-and-reversal restarts the grid; unchanged parameters preserve it | The lineage carries one `EpochArtifacts` per activation, including zero-length epochs; the segment identity test is on resolved `(rate, cadence)` | Preserve; make normative (§4) |
| 5. A single decay operation on an absent cell creates neither cell nor effect; a composed body may still initialize a value | `group_intent`'s single-operation `Update::Decay` arm returns `Ok(None)` when the cell or its numeric value is absent; the composed-body arm starts from `current.unwrap_or(0)` with `anchor = now` | Preserve; make normative (§4) |

Behaviors 2–5 are the behavior the independent review of 2026-09-12 executed and reported
in its §5. This candidate does not re-derive them; it promotes them from reported
implementation behavior to checked-in normative tests, which is exactly what the Operator's
resolution makes them.

## 2. The write paths that reach a decayable cell

A decay/recovery operation is admissible only on a target with a declared baseline
(`rules.rs::check_emit`, `DecayWithoutBaseline`), and at most one decay rule and one decay
operation exist per target (`validate_reducers`, `SecondDecayReducer`, v2 §4.3). A cell is
therefore associated with **at most one** decay operation identity per rule set, and that
identity — rule, qualified sub-ID, target, scope mapping — is the one `decay_walk` already
resolves in each epoch.

| Path | Mechanism at the checkpoint | Family | Settlement |
|---|---|---|---|
| Command-triggered rule, all-additive group | `group_intent` → `Intent::AddDelta`; `effects::reduce` ADDITIVE branch folds onto `pre(definition, scope)` | ADDITIVE | **Yes** |
| Scheduled-work rule, all-additive group | identical to the above; the seed differs, the reducer does not | ADDITIVE | **Yes** |
| Materialized obligation effect, `FrozenIntent::AddDelta` | replayed into `Intent::AddDelta` and reduced on the same branch | ADDITIVE | **Yes** |
| Several rules contributing additive deltas to one target | one ADDITIVE group: checked `i128` fold applied **once** to the settled base | ADDITIVE | **Yes, once** |
| `Update::Assign`, `Update::Aggregate`, `FrozenIntent::Result` | `Intent::Result`; the reducer commits the declared value and never reads the pre-wave cell | RESULT | **No** — this is the explicit replacement of behavior 2 |
| `Update::Scale`, `Update::Clamp` | `Intent::Transform`; resolved from the pre-wave value inside `group_intent` | TRANSFORM | **No** — see §5 |
| Composed rule body (mixed stages) | `Intent::Transform { RuleBody }`; stages fold in declared order, a `Decay` stage walking the same fixed grid | TRANSFORM | **No** — see §5 |
| Single `Update::Decay` | `Intent::Transform { Decay }`; `decay_walk` from `cell.updated_at` to `now` | TRANSFORM | n/a — it *is* the evaluation |
| Direct command ingress (`CommandPayload::Host` observations) | `store.observe_host_owned`, outside the reducer | — | **Not reachable.** Ingress writes only `Authority::HostOwned` cells; `write_path_of` gives a host-owned target no evaluator write path, so `engine.rs:766` refuses any rule emission — decay included — on such a target (AT-I10). A host-owned cell can therefore never carry decay debt. |
| Epoch activation (`CommandPayload::ActivateEpoch`) | appends to the lineage | — | Not a cell write; it moves the grid under D-3 … D-5, never `updated_at`. |

## 3. The change

`EvalView::settled(definition, scope, now)` returns the value that the target's decay
operation would commit at `now`, or the raw committed value when the target has no decay
operation anywhere in the retained lineage, or `None` when the cell is absent or
non-numeric. It reuses `decay_walk` and `baseline_of` unchanged, so every fixed-grid
property of D-1 … D-7 — segment origins, endpoint ownership, residual discard, linear
floor-exact `i128` steps, baseline clamping — is the *same* computation an explicit
evaluation performs.

The settlement operation is the decay operation for the target carried by the **latest**
epoch in the retained lineage that carries one. In the ordinary case this is the current
epoch. Searching backwards is what makes behavior 3 hold for an additive write that lands
while the operation is removed: the identity still resolves, `decay_walk` contributes
nothing for the epochs that lack the operation, and the closing segment's whole steps stay
applicable. This reads only committed, restore-validated state (the activation lineage and
the cell), so it introduces no `StateCell` field, no encoding change, and no new canonical
content (adjudication S-4).

`plan_wave` computes one settled pre-value per distinct `(definition, scope)` among the
canonical candidates, before reduction, and passes the lookup to `effects::reduce` in place
of the raw pre-wave read. `reduce`'s signature, its `pre` contract, and its inline tests are
untouched; `pre` is consulted only by the ADDITIVE branch, so no other family's arithmetic
can change. A settlement failure is a `WaveRejection` raised before any commit, so the wave
is rejected atomically and no partial settlement is written — the same failure atomicity the
existing arithmetic paths have.

Ordering, activation ownership, bounds and reporting follow unchanged rules:

- **Ordering and activation ownership.** Settlement happens inside the wave that is already
  evaluating at the cohort's canonical time `now` (FINAL §4), against the stable pre-wave
  snapshot. It consults the same lineage, with the same barrier ownership (D-4) and the same
  no-pre-activation-charge rule (D-5), that a decay evaluation at `now` would.
- **Bounds and arithmetic.** The settled value is produced by `move_toward` in checked
  `i128` and converted once; the additive fold then applies the existing checked `i128`
  addition and single `i64` conversion. Declared bounds are validated afterwards by
  `store.validate_effect`, exactly as before.
- **Reporting and one commit per endpoint.** Settlement is **not** a second committed
  effect. The cell transitions once, from its pre-wave committed value to the settled sum,
  at `now`. There is no backdated commit, no implicit intermediate write, and no second
  charge at the same grid endpoint: a later evaluation reads `updated_at = now` and finds
  only the grid points in `(now, …]`.
- **Watchers and thresholds.** `Trigger::Crossing` continues to compare the **committed**
  pre-wave value with the committed new value. A watcher observes committed transitions, and
  settlement produces exactly one. Making the watcher compare against the settled
  intermediate instead would invent an unreported state transition, which the mission
  forbids; deriving it from the retained atomic-wave and effect-reporting rules gives the
  committed-to-committed comparison. This is recorded as a deliberate, tested boundary.
- **Family conflict.** A group that mixes ADDITIVE with any other family still rejects
  atomically through the unchanged `FamilyMixture` path. Settlement is not routed through
  it and does not weaken it.
- **Determinism.** The settled value is a function of committed state alone, so replay,
  snapshot/restore and every pacing budget reproduce it.

## 4. Superseded interpretation

The adjudication's §5.2 observation — "grid steps that end **before** a non-decay write that
no decay evaluation preceded … are never applied" — described implementation behavior that
the Operator had expressly not decided. For **additive** writes the Operator has now decided
against it. From this candidate it is **historical behavior, not a normative requirement**:

- It is not restated as a passing requirement anywhere in the checked-in suite.
- The former behavior is retained as a **negative control**: a mutation that restores the
  raw pre-wave base must fail the new additive settlement tests.
- For **explicit replacement** the observation still holds and is now normative under
  behavior 2: earlier decay must not reduce a declared replacement.
- Historical records and the original review probes are preserved byte-identical. Any
  adapted probe is a separate copy whose delta is explained in the report.

The mission's illustrative discriminator is the pinned contract: baseline 0, 100 at 0, rate
10, cadence 10; `+5` at 15 gives `(95, 15)` and a decay evaluation at 20 gives `(85, 20)`;
explicitly evaluating decay at 10 first gives the same numbers; replacing with 100 at 15
instead gives `(100, 15)` and `(90, 20)`. Recovery mirrors all of it. These are tuple
vectors, not a promise of full-history digest equality between different event histories.

## 5. Boundaries this contract does not cross

The Operator's resolution names two categories: additive change and explicit replacement.
Two implemented categories are neither, and this candidate deliberately leaves them alone
rather than inventing architecture for them:

1. **`Scale` and `Clamp`** resolve from the pre-wave value without being an addition or a
   declared replacement.
2. **A composed rule body** folds its declared stages in declared order. When the body
   contains a `Decay` stage — the v2 §4.3 sanctioned additive-plus-decay composition — that
   stage already applies the grid points in `(updated_at, now]`, so an implicit settlement
   would charge the same endpoints twice, which the mission forbids. The profile expresses
   the settle-then-add order by declaring the `Decay` stage first.

Both keep their existing, independently reviewed semantics, and both are pinned by new
tests so the boundary is visible rather than accidental. A body without a `Decay` stage that
contains additive stages therefore still forfeits earlier unapplied steps; that is the
unchanged behavior, disclosed here, and it is a candidate for a future Operator decision.
