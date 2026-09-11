# S.P.A.R.K. Gate C2 independent implementation review

Date: 2026-09-11. Reviewer: Codex, independent of the implementation writer.

**Verdict: `GATE_C2_BOUNDED_REVISION_REQUIRED`.**

Exact candidate: `053d1dc1131ec47be94b60513fad9ea8389cde0c`, branch
`candidate/phase2-gate-c2-implementation-20260911`.
Review branch: `review/phase2-gate-c2-independent-20260911`, based directly on that
candidate. The publication receipt supplies the review commit, avoiding a self-reference.

The candidate preserves the inherited core and implements the important V3-F01 request,
extraction, finalization and replay mechanisms. It does **not** implement the complete
frozen rule semantics or acceptance oracle. Seven independently written contract assertions
fail against unchanged candidate sources, despite all 320 existing tests passing. Bounded
implementation and oracle corrections are required; this is not evidence that the accepted
least-slice/request/finalization architecture needs replacement. No production promotion,
Phase-3 work, GAME change, experimental merge, or candidate repair was performed.

## Authority, custody and preservation

Executed `SPARK_PHASE_2_GATE_C2_INDEPENDENT_REVIEW_MISSION_2026-09-11.md` under the
Operator's explicit authorization for review, validation, evidence, commit and normal push.
The original checkout and all eight pre-existing worktrees were preserved and clean when
inspected. No applicable ancestor or repository AGENTS.md, CLAUDE.md or CODEX.md was found.
The isolated review worktree is `/tmp/spark-gate-c2-review`.

Fetched origin with pruning and queried live GitHub refs. Candidate local/tracking/live
state agrees at the exact hash above; production remains
`7e3a0aae069a8bf840e4dfb74221cdf2687b1db5`. The direct-parent chain is:

`7e3a0aa` → `4b02532` → `aad4633` → `b7d849f` → `5fe883f` → `0423d91` →
`4997d56220000edef3a2b4cd987feff7900d2488` → `053d1dc`.

Every checkpoint matches the mission. The last commit changes documentation/evidence only.
The seven inherited integration-test files and seventeen actual inline test modules are
byte-identical; manifests and lockfile are unchanged. `schedule`, `drain_due`, `stage` and
`submit_fence` are byte-identical functions. Core changes are confined to additive scheduler
X-1–X-4/commitment helpers and timeline PX-1/index validation. The scheduler's factored
`slot_block` preserves the original encoding sequence exactly. Existing digest tests and
fresh scheduled/conflicted/cap-boundary reference pins pass. No inherited test was weakened,
ignored, removed or rewritten. `preservation.json` records full hashes and worktree states.

## Controlling interpretation

Applied the acceptance freeze and Revision-2 independent review/two pins first, then the
Revision-2 architecture/oracle deltas, FINAL architecture/oracle, retained V2 X-1–X-4,
R-1–R-7 and equivalence sections, freezes and matrices v1→v2→v3 (Opus provenance), the
ActiveRequest decision and ADR-0001–0006. Also consulted the implementation mission/report,
writer evidence, development handoff and convergence protocol. Historical statements that
Phase-2 implementation is unauthorized are superseded by the current Operator mission;
withdrawn Phi, SH-1/SH-2 and R-8 proposals remain withdrawn.

Anchors below refer to **candidate** source line numbers. They do not refer to a repaired
version. Test probes live only in the evidence directory and run in disposable external
crates with candidate path dependencies and the candidate lockfile.

## Findings requiring correction

### C2-01 — MAJOR: derived parent sets omit changed eligibility inputs

`crates/spark-engine/src/engine.rs:1804` builds parent sets only from the definition named
by `Trigger::Change`/`Crossing`; `engine.rs:1823` extends only that reduction's contributors.
Conditions are evaluated later at `engine.rs:483`, without adding their read groups.

Concrete public-door counterexample: wave 0 changes both `state.stress` and `state.mood`.
A propagation rule watches stress, requires mood > 0, and schedules an obligation. Both
changed groups made that emission eligible. Its recorded creator identity equals the
independent **stress-only** formula and differs from the required stress∪mood formula.
`derived_identity_binds_every_changed_eligibility_input` fails on the equality required by
v3 §4.3 rules 2/5 and AT-I25's multi-target clause. The negative-control equality to the
one-parent formula passes. Existing AT-I6c/AT-I25 tests exercise multiple contributors to
one target; they cannot kill this wrong implementation.

Correction: derive the complete canonical parent union for all changed groups actually
read for eligibility, preserving empty-background exclusion, transitivity and deduplication.
Add multi-target, unchanged-background and multi-wave identity fixtures.

### C2-02 — MAJOR: static fan-out ignores declared scope breadth

`crates/spark-engine/src/rules.rs:1179` counts distinct definition IDs plus schedules,
not target definitions across the declared scope mapping. A single rule emitting to
`state.stress@actor:a` and `state.stress@actor:b` activates with `max_fan_out = 1`.
`fanout_must_count_scope_breadth` fails: the door returns an activated artifact.

This contradicts v1 Q4's explicit scope-breadth bound and AT-I19. The existing test at
`phase2_rule_set_door.rs:223` counts different definitions, leaving this mutant alive.
Count the declared expansion, including delayed materialized effects as appropriate to
its execution bound, and test exact-bound/over-bound scope mappings.

### C2-03 — MAJOR: materialized emitting operations bypass sub-ID validation

`crates/spark-engine/src/rules.rs:1168` calls `check_emit` for nested materialized effects
but never calls the sub-ID `claim` closure used for ordinary effects. Two same-target
materialized effects with sub-ID `same`, deltas 10 and 15, pass activation. At execution
`engine.rs:1724` uses those sub-IDs as emission identity inputs, so distinct operations
collide and are rejected as contested; equal deltas would fold and lose one contribution.
`materialized_subids_must_be_unique` independently fails at the activation door.

v2 §3.2 and AT-I17 extend stable, qualified, unique sub-IDs to **every emitting operation**.
Validate nested IDs/qualification in the frozen namespace before minting an artifact;
add both unequal-payload and equal-payload negative controls.

### C2-04 — MAJOR: aggregate arithmetic narrows before the weighted result

`crates/spark-engine/src/engine.rs:458` converts the accumulated `i128` sum to `i64`
before applying the weight. Two source cells each holding `i64::MAX`, weight 500000
(0.5), have the valid result `i64::MAX`. The candidate instead returns
`Arithmetic { what: "aggregate" }` and writes nothing.
`aggregate_keeps_i128_until_after_weighting` fails; both source-cell setup assertions pass.

v1 Q1/§7 and AT-I24 require widened accumulation and one final floor, followed by result
conversion. Keep the complete weighted computation in checked `i128` until that point.
Also audit the `i64` intermediate accumulators in rule-body arithmetic
(`engine.rs:649`, `engine.rs:723`) against the same numeric contract rather than claiming
that lint success proves numeric semantics.

### C2-05 — MAJOR: decay changes the frozen baseline and time semantics

Three related discrepancies need resolution, not a partial-coverage acceptance:

- `engine.rs:701` substitutes the rule's `toward` expression when baseline is absent.
  The unchanged store creates cells with `baseline: None` (`state.rs:562`); the rule door
  never enforces v1 Q7's rejection of definitions lacking baseline semantics. D-C2-3 is
  a disclosed **change** to the frozen requirement, not its implementation.
- `engine.rs:699` measures from `updated_at`, while `engine.rs:2007` commits at `now`.
  With value 100, target 0, cadence 10, rate 10: evaluations at 10/20 and at 20 alone
  yield 80; at 15/20 they yield 90. The whole-step computation at 15 drops the remaining
  five time units. `decay_preserves_elapsed_cadence_remainder` fails after both aligned
  positive controls pass. This is an additional boundary probe, not a claim that the
  three histories have equal WorkKeys or engine digests. The writer's cadence-aligned
  restriction does not establish the general chunk/remainder claim.
- `engine.rs:708` applies the **current** rate to the entire elapsed span. Value 100 at
  time 0, cadence 10, rate 1, activation of rate 5 at time 50, first decay at 60 produces
  **70**. Epoch-scoped integration is five old-rate steps plus one new-rate step, **90**;
  the candidate retroactively applies the new rate to all six steps.
  `new_decay_rate_must_not_apply_before_activation_barrier` fails. AT-I23 says the new
  rate applies only from its activation barrier. The purported AT-I23 test at
  `phase2_finalization_and_replay.rs:742` tests additive reads of config, **not decay or
  recovery over an epoch-crossing elapsed interval**, so it cannot kill this defect.

Implement the frozen baseline, cadence and epoch-bound rate contract with a justified
representation. If that requires a genuine architecture decision, record the exact conflict
rather than silently narrowing Q7 or altering Phase-1 encodings. Add real decay/recovery
and sanctioned composition fixtures; do not relabel the additive test as full AT-I23.

### C2-06 — MAJOR: arbitrary command kinds can activate behavior epochs

`crates/spark-engine/src/engine.rs:1477` dispatches activation solely on payload variant.
The reserved `EPOCH_ACTIVATION_COMMAND_KIND` at `engine.rs:66` is not checked. A request
with kind `cmd.ordinary` and an `ActivateEpoch` payload finalizes and changes epoch 1 to 2.
`epoch_activation_requires_reserved_command_kind` fails, with `EpochActivated` in the report.

v1 Q5 requires the reserved epoch-activation command kind at its own barrier. Enforce the
kind/payload contract in both directions and test refusal without artifact mutation. Keep
ordinary timeline finalization semantics and request completion behavior as prescribed;
this finding does not authorize changing Phase-1 admission.

### C2-07 — MAJOR: the claimed oracle completion is materially overstated

The per-entry dispositions below identify missing tests and surviving wrong implementations.
In particular:

- FINAL AT-I46(a) explicitly forbids a canonical `cohort_identity` report field.
  `crates/spark-engine/src/report.rs:225` exposes exactly that field. The claimed pin at
  `phase2_stores_and_semantics.rs:820` destructures and **requires** it, so it pins the
  implementation rather than the frozen field set. The module's own header says the
  opposite. `PacingDiagnostics.command_deferred` is also an addition to the explicitly
  frozen diagnostic field list; document/adjudicate it instead of calling the list unchanged.
- AT-I35's new golden encoding coverage at `phase2_request_boundary.rs:575` covers None
  and Advance, not Some(Command); its stable-boundary reference calls the production
  active-request canonicalizer. The review's additional raw-byte Command control passes,
  but is not the writer's missing independent complete golden corpus.
- AT-I26/27 at `phase2_stores_and_semantics.rs:515` does not isolate all named stores:
  the obligation change also changes scheduler state; the cooldown case checks length,
  not digest discrimination; the epoch case changes the timeline too. Equal-pair testing
  alone cannot kill a constant/omitted store commitment. AT-I30/31 at line 664 uses future
  work in one scope, not many dormant scopes or an aggregate/promotion fixture.
- The baseline README says each test landed when first expressible. Engine implementation
  is in `b7d849f`, while most request/wave/finalization tests arrive in later checkpoints.
  A blanket “inexpressible on Phase 1” declaration is not an executed red-first test for
  each subsequent implementation slice. Preserve historical evidence and correct the claim;
  do not rewrite checkpoints to manufacture red-first provenance.

Complete the applicable missing oracle cases and use named negative controls. A disclosed
partial case remains unsatisfied; disclosure is valuable but does not waive the mission.
Remove the forbidden report surface or obtain an explicit architecture disposition before
claiming conformance. No new mutable scheduler/timeline host authority was found.

### C2-08 — MINOR: an explicitly forbidden finalization value is constructible

Revision-2 oracle §10 requires external construction of `FinalizationRefusal` to fail.
`crates/spark-engine/src/request.rs:227` is a public enum; an external crate constructs
`FinalizationRefusal::WrongProfile` successfully. The review's construction probe passes
(compilation is the counterexample). The 27 writer probes omit this required case.

This value alone cannot mutate the engine or forge a private-field `ProcessResult`; it is
an oracle/facade conformance defect, not a demonstrated authority bypass. Correct the type
boundary or explicitly adjudicate the requirement, and add the exact missing probe.

### C2-09 — NOTE: indexed preflight passes; whole finalization is still history-dependent

The fresh release workload reports:

| Finalized history | P-9 refusal path / call | Scan reference / call | Successful finalization / call |
|---|---:|---:|---:|
| 1,000 | 2.096 µs | 6.635 µs | 0.841 ms |
| 4,000 | 2.600 µs | 40.864 µs | 3.565 ms |
| 16,000 | 2.554 µs | 1.480 ms | 15.933 ms |

The 16k history build took 125.987 s. The refusal measurement includes `process` overhead,
not only the three map lookups. Successful samples append another 200 commands. Pin 2 is
satisfied by source and the observed scaling: `engine.rs:2130` onward uses PX-1 map lookups.
The unchanged `timeline.rs:1673` computes the complete history digest on each successful
fence; pre-wave engine digests likewise include whole stores/history. These are single-machine
measurements, not production latency guarantees or executable platform parity. The writer's
main performance limitation is honest. `StateStore::cells_of` also filters the complete cell
map (`state.rs:347`); report work counts do not prove bounded physical scan cost.

## Mechanisms independently checked

**Finalization:** P-1 profile, P-2 epoch and P-3 grant precede mutation; P-4 validates the
window; P-5 checks frontier increment; P-6 examines the full bounded window; P-7/P-8 use
permanent identity maps; P-9 uses the last finalized source sequence. I-CS makes staged
indexes empty. The envelope/ticket/frontier and previous-fence hash are derived under the
same exclusive borrow. `stage` must positively acknowledge NewlyStaged and cover the envelope
before the deterministic one-ordinal `fence.<n>` call. No intervening mutation invalidates
P-9. This discharges the unchanged stage/fence refusal paths, including poison/idempotent
slots, ticket/window/range/digest errors, source regression and ordinal exhaustion.

The fresh suite includes every P-row refusal with full Debug/private-state comparison,
360 clean-state differential cases across two sources and a reset, old-recipe red control,
success/follow-on, scheduled work retained on refusal, deferred command, and P-9-disabled
sticky fail-stop. `process` emits no successful boundary report on entailment violation;
snapshot and reset are refused, subsequent process calls fail-stop, and restoration of the
last committed clean snapshot recovers. The public digest getters themselves remain callable
on a fail-stopped instance; those computed values must not be treated as publishable stable
snapshots. No snapshot durability or real crash proof is claimed.

**Scheduling/request boundary:** live least-slice selection; structural executable-only
budget/oversized admission; zero-cost complete conflicted extraction; fingerprint compare
before any removal; full slot/store commitment comparison before scheduler extraction;
cohort-local `now`; created work strictly later; live horizon expansion; no retained plan or
budget carry. ActiveRequest is set before cohort mutation, exact-matched before the frontier
check, retained across pause and cleared with F at completion/refusal. Digest placement and
request-bound `terminates` checks pass. The consumer preserves the real FIFO head on wrong
presentation and halts on nonempty head/active disagreement.

**Wave transition:** immutable EvalView and validate-all/apply-all structure are sound for
the checked paths. Typed family reduction, complete target fold before crossing, checked
occurrence ranges, admission caps, cooldown expiry and bounded provenance are present.
Cross-store extraction checks complete commitments; wave enqueue preflight at
`engine.rs:1936` checks only Scheduled/Contested shape. The omitted AT-I8 invariant-fault
fixture must check full post-state commitment consistency, not just those shapes. No ordinary
public-input counterexample to cross-store atomic extraction was found.

**Restore/replay:** artifact binding and epoch chain first, active horizon ≥ F, clean staging,
derived indexes/grant and bidirectional invariant, then recomputed boundary digest. Snapshot
fields are private. Completed replay takes genesis, actual payload-bearing commands, reset
metadata, F and both expected digests; it is not paused recovery. Resets sort by reset_index
before commands and for the trailing sequence. Writer's reset fixture and the review's
additional **trailing same-frontier resets in reversed input order** pass. Paused recovery
uses the validated snapshot and exact request. No production repair was needed for either pin.

## Per-entry oracle disposition

Legend: **S** meaningful executed coverage supports the stated tested behavior; **P** partial
coverage, further frozen cases remain; **R** revision required due to a demonstrated defect
or a test contradicting the contract. S is not a universal proof or an acceptance waiver.
Every test referenced here ran in the fresh 320-test workspace run. Source/test paths are
under `crates/spark-testkit/tests/`; shorthand: **C** phase2_core_surfaces.rs, **D**
phase2_rule_set_door.rs, **W** phase2_waves_and_pacing.rs, **Q** phase2_request_boundary.rs,
**F** phase2_finalization_and_replay.rs, **S** phase2_stores_and_semantics.rs, **X**
phase2_compile_probes.rs. Line anchors identify the relevant test body.

| Entry | Writer | Review | Executed discriminator / remaining requirement |
|---|---|---|---|
| AT-I1 | I | P | W:107 distinguishes pre-wave reads (60 vs sequential reread); missing untouched-clone digest comparison for that exact fixture. |
| AT-I2 | I | S | X:231 forbids run_waves; W:1052 distinguishes later-wave reads/retained commits. |
| AT-I3 | I | P | D:46 hashes declaration permutations; W:107 covers additive engine permutations; not the full engine corpus of equal RESULT/singleton TRANSFORM permutations. |
| AT-I4 | I | P | S:101/C:394 test member/order discrimination; not every semantic WorkKey field in the identity sweep. |
| AT-I5 | I | P | W:173 family/target ordering; explicit rule-renaming and internal enumeration corpus incomplete. |
| AT-I6a | P | P | effects.rs:501 unit duplicate/contested reducer checks; no full wave/store atomic duplicate fixture. |
| AT-I6b | I | S | W:107: distinct +10/+10 yields 40 with two retained causes. |
| AT-I6c | P | R | W:576 one-target hand-computed parents; C2-01; missing derived redelivery, depth-2 and empty-parent wave rejection cases. |
| AT-I6d | P | P | W:614 lower third parent; X blocks parent type; no internal no-selection-accessor/renaming corpus. |
| AT-I7a | I | S | W:107 full additive fold and reversed construction equality. |
| AT-I7b | I | S | W:272 unequal RESULT rejection and pre-wave digest equality. |
| AT-I7c | I | S | W:173 distinguishes same-family equal results from equal-valued transforms/cross-family results. |
| AT-I7d | I | S | D:367/W:173 static/runtime incompatible family table, atomic outcomes. |
| AT-I7e | I | P | W:305 checked-domain/bounds failures; mathematical i128 accumulation-overflow case not independently exercised. |
| AT-I8 | P | P | W:305 occurrence mid-range and store/scheduler cap failures; missing wave post-state bidirectional commitment fault. |
| AT-I9 | I | S | W:272 repeated unequal-result report and digest equality. |
| AT-I10 | I | S | D:101 rejects host-owned target with unchanged lineage; inherited authority tests retained. |
| AT-I11 | I | S | S:146 bounds extremes and atomic companion effect; typed write-path checks retained. |
| AT-I12 | I | S | S:182 slot/store payload agreement across schedule/collide/drain prefixes. |
| AT-I13 | I interpreted | P | F:814 multi-target drift via seam, matching frozen execution; fixed-manifest engine cannot exercise a real definition migration. Honest interpretation, not migration proof. |
| AT-I14 | I | P | F:742 changed same-ID rule refusal and unchanged rule execution; exact originating artifact/config resolution requires D-C2-7 clarification below. |
| AT-I15 | P | P | S:215 three-way/cap permutations and complete contested drain; no wave-1 competing parent-derived record fixture. |
| AT-I16 | P | P | S:282 identity-ordered multi-allocation and pacing equality; no competing derived multi-parent allocations. |
| AT-I17 | I | R | D:123/S:328 ordinary/gate IDs and address stability; C2-03 nested emitting-ID hole. |
| AT-I18 | I | S | D:146 direct/mutual/long watched-edge cycles and delayed counterparts. |
| AT-I19 | I | R | D:223 definition-count/depth boundaries; C2-02 scope breadth bypass. |
| AT-I20 | I | S | W:1219/Q:131 whole-cohort pacing and canonical equality on tested histories. |
| AT-I20b | I | P | W:788 terminal candidate-cap rejection; full cap/retry matrix not established. |
| AT-I20c | I | P | W:688/745 boundaries, large cohorts and progress; threshold fixture is separate rather than the required oversized combined case. |
| AT-I20d | I | P | Q:131/ W pacing telemetry inequality and repeatability; X proves private view, not all internal evaluator-type non-reachability obligations. |
| AT-I21 | I seam | P | W:1160 strict-later conversion via depth seam; larger-depth value convergence honest, no equality across different artifact identities; interference case incomplete. |
| AT-I22 | P | R | S:420 aligned math only; C2-05 baseline/remainder and missing two sanctioned compositions. |
| AT-I23 | I | R | F:742 additive config read cannot kill retroactive decay-rate use; independent epoch-crossing decay fails (C2-05). |
| AT-I24 | P | R | S:456 small aggregate/floor permutations; valid widened aggregate fails C2-04; missing i128-boundary corpus. |
| AT-I25 | P | R | W:639 opposing partial folds; W:576 same-target parent set; independent changed two-target union fails C2-01. |
| AT-I26 | I | P | S:515 does not isolate/discriminate each named store (C2-07). |
| AT-I27 | I | P | S:515 equal contested construction and same-next inputs; per-store epoch/reset/interleaving corpus incomplete. |
| AT-I28 | C | P | C:504 index recomputation, restore runs it; no injected inconsistent-derived-index restore test. |
| AT-I29 | I | S | S:617 snapshot/restore before each request/resume; F paused recovery; X prohibits inter-wave entry. |
| AT-I30 | I | P | S:664 future work unchanged, not many dormant scopes/touch instrumentation (C2-07). |
| AT-I31 | I | P | S:664 scheduled additions only; no dormant aggregate/promotion oracle. |
| AT-I32 | I | R | Strict lint + 27 external negative probes pass; required refusal construction compiles (C2-08), evaluator-inside-type probes incomplete. |
| AT-I33 | P | P | S:694 numeric/time/ID/empty-set extremes; maximal parents/depth chains missing. |
| AT-I34 | I | S | All 232 inherited tests preserved, unignored and included in 320 passing tests. |
| AT-I35 | I | P | C:103/145 scheduler pins and Q:575 Advance/None composition; Command/full independent golden corpus missing in candidate (C2-07). Review's extra raw-byte Command probe passes. |
| AT-I36 | I | S | Fresh five all-target static builds pass; no executable parity claim. |
| AT-I37 | I | S | D:367/W:173 discriminating static/runtime pair tables for the enumerated families. |
| AT-I38 | P | P | S:782 exact arithmetic/coverage past cap; missing later-wave identity difference beyond retained provenance cap. |
| AT-I39 | P | P | Q:153/205/260 work-producing catch-up, reference pre-wave placement and selection count; full components/retired-heartbeat and cross-profile companion incomplete. |
| AT-I40 | P interpreted | P | C:394 multi-profile slice order; W:889 equal executable results vs conflict differences; per-profile engine is acceptable but not the full multi-profile engine oracle. |
| AT-I41 | P | P | Wave/cohort restart through catch-up tests; missing independent effect_batch_v3 recomputation corpus. |
| AT-I42 | I | S | C:79–434 and W:961 execute stale/nonleast/replay/cap/refinement/no-op and cross-store missing/mismatch/record transfer discriminators. |
| AT-I43 | I | S | Q lifecycle/start/substitution/resume/time/finalization/overflow cases meaningful; decay-rate correctness separately fails AT-I23. |
| AT-I44 | P | P | W:1052/1133 distinguish wave-0/later rejection and finalized rejected commands; mixed conflicted+rejecting slice (c′) missing. |
| AT-I45 | I | S | W:870/922 budget-2 discriminator and zero-cost conflicts after oversized admission. |
| AT-I46 | I | R | S:820 explicitly pins the forbidden cohort_identity field; feature absence passes, internal input-type/feature schema obligation incomplete (C2-07). |
| AT-I47 | I | S | Q:554/636/672, F:534 active/F discrimination, exact restore/resume, cleanup and mismatch behavior. |
| AT-I48 | I | S | F:110–380 full-state refusal cases, 360 differentials, positive stage/fence, follow-on, refusal retention and sticky injected fail-stop. |
| AT-I49 | I | S | F:420/461/478 wrong presentation, completed non-head, active/head disagreement and durability-order model. No real durable mailbox claim. |
| AT-I50 | I | P | F:634/668 pause distinction and completed reconstruction/reset/width controls; missing payload refusal not executed; review adds trailing-reset positive control. |

Superseded original AT-I6/AT-I7 forms and withdrawn Phi/R-8 fixtures are not revived.
The table intentionally narrows several writer “I” labels; a green test count is not a
per-entry completeness proof.

## Writer decisions and interpretations

| Decision | Disposition |
|---|---|
| D-C2-1 pacing excluded from ruleset_content_hash | **Accept** the semantic/pacing separation: FINAL PE-D permits budgets to differ at fixed artifact identity. Keep pacing excluded from all canonical causal commitments; accurately describe the compatibility/latency distinction. |
| D-C2-2 one engine per profile | **Accept** composition; v3 §2.2 states waves/epochs are per profile and cannot target another profile. Retain honest partial multi-profile oracle status; no cross-profile runtime claim. |
| D-C2-3 baseline fallback/aligned decay | **Reject as frozen conformance**; C2-05. No proof that Phase-1 inability to populate baseline authorizes replacing Q7. |
| D-C2-4 absent cells as additive/crossing zero | **Accept** explicit deterministic convention; inputs retain their separately declared absent value. |
| D-C2-5 Host/ActivateEpoch payload | **Require revision** for reserved-kind binding (C2-06). Host observation validation-before-apply and payload hashing otherwise fit the command barrier. |
| D-C2-6 fixed manifest, rule/config epoch activation, seed in config | **Accept** fixed-manifest scope and committed seed; migration is not implemented. Do not use this limitation to claim a real definition-drift lifecycle test. Rate-boundary correctness still requires C2-05. |
| D-C2-7 current-rule exact fingerprint plus historical ruleset hash | **Require revision/precise compatibility account**. It rejects changed fingerprints, but uses the current config/artifact at engine.rs:1094/1413, not a resolved originating config. ADR-0006 requires exact originating artifacts for re-evaluation, whereas AT-I23 requires barrier-scoped hot rates. State and test the compatibility rule explicitly; a historical hash's presence alone is not artifact resolution. Do not reinterpret old work silently. |
| D-C2-8 one rule body as explicit reducer | **Accept** composition mechanism, expressly sanctioned by v2 §4.2. Audit premature i64 intermediate failure and add both decay/shock composition tests; acceptance of the mechanism is not acceptance of missing numeric coverage. |
| D-C2-9 identical-trigger static mixture checks | **Accept as a conservative implemented subset**, with runtime rejection for other mixtures. Do not claim complete analysis of all statically provable scope aliases/eligibility relationships. Fan-out still requires C2-02. |
| D-C2-10 depth 0..=bound and edge-count static bound | **Accept** cohort-scoped interpretation. Conversion is strictly later, never dropped; seam test does not prove all interference cases. |
| D-C2-11 extraction refusal mapped to Paused | **Require revision** of the invariant-error contract/reporting. Direct extraction returns useful typed errors, but process discards them at engine.rs:1193 and can repeat Paused forever on corrupted state. No public-input reachability is claimed. “Nothing mutated” must mean this extraction attempt: ActiveRequest and any earlier cohorts may already have changed. |
| D-C2-12 derived RNG occurrence from 64 parent-digest bits | **Accept as the explicit deterministic address projection**; no arrival/pacing input. Do not claim an injective mapping from a 256-bit parent set to u64, or complete multi-wave RNG coverage. |
| D-C2-13 synthetic materialized fingerprint / schedule identity | **Require revision or a frozen-formula proof**. engine.rs:1693 hashes the whole record, including resolved numeric payload, into the rule-fingerprint component, contrary to v3 §4.4's payload-independent causal identity. Preserve the originating identity/fingerprint semantics; schedule-operation target=(rule,scope) is otherwise a reasonable declared encoding choice. This is a source-level conformance concern, not one of the seven red executable assertions. |
| D-C2-14 test-support seams | **Accept** isolation and use for unreachable refusal paths; feature hygiene passes. Missing tests cannot be excused by unreachability when the oracle specifically requires an injected case. C2-07/08 remain. |
| D-C2-15 three local lint allows | **Accept** the documented size/argument choices; strict library lint passes. They do not suppress the arithmetic or panic lint rules. |

## Fresh validation and bounded evidence

Evidence directory: `gate_c2_independent_review_evidence_2026-09-11/`.
`checks.json` records exact commands, exit codes and elapsed seconds. `run_validation.py`
reruns the prescribed checks; `check_preservation.py` verifies source/test/lineage custody;
`run_counterexamples.py` compiles the separate independent Rust probes against the unchanged
candidate with its locked dependencies. No probe was added to the candidate workspace tests.

| Validation | Result |
|---|---|
| cargo fmt --all --check | PASS |
| cargo test --workspace --all-features | **320 passed, 0 failed, 0 ignored**, including 232 unchanged inherited tests |
| cargo clippy --workspace --all-targets --all-features -- -D warnings | PASS |
| strict core/engine library lint, all features | PASS; warnings, unwrap, expect, panic, indexing/slicing, arithmetic side effects denied |
| cargo metadata --format-version 1 | PASS; dependency hygiene tests pass |
| git diff --check baseline..exact-candidate | PASS |
| Windows x86_64 GNU and MSVC static all-target checks | Both PASS |
| Android aarch64, armv7, x86_64 static all-target checks | All three PASS |
| release/offline gate_c2_workload | PASS; measurements above, full log retained |
| independent external probe corpus | **7 expected contract failures, 3 passing probes**, cargo exit 101 captured; runner verifies exact result count |
| two passing semantic controls | trailing same-frontier reset replay; raw-byte Some(Command) active encoding |
| third passing probe | external refusal construction succeeds, itself the C2-08 counterexample |
| baseline/checkpoint/final-doc scope | PASS; preservation.json |

Limits: finite test corpora and source analysis, not formal verification; no process-crash
recovery, durable storage/mailbox, service/transport, GAME integration, executable Windows/
Android digest parity, production performance certification or Phase-3 implementation.
The writer is honest about those deferred gates, the history cost, seam-built drift/depth
fixtures and several partial cases. Its broader oracle-complete/frozen-conformance claims
are not supported for the reasons above.

## Publication and next prescribed action

Publish this review and its bounded evidence by a normal commit/push on the review branch;
verify local HEAD, upstream tracking and live review ref equality. Recheck the live candidate
and production refs without moving them. The final receipt records the exact review hash and
synchronization result; no self-referential commit hash is embedded here.

**Next action:** return this exact candidate and findings to the separated implementation
writer for a bounded Gate C2 revision under the already-authorized Phase-2 mission. Correct
the demonstrated defects, resolve the specifically identified interpretation conflicts,
complete the missing applicable oracle cases with genuine negative controls, retain all
inherited tests, and publish a new exact candidate for a fresh independent review. Do not
repair this candidate in the reviewer role. Do not promote production or begin Phase 3.
Operator acceptance/promotion follows a successful independent verdict as prescribed by the
mission; this verdict supplies no acceptance or promotion.
