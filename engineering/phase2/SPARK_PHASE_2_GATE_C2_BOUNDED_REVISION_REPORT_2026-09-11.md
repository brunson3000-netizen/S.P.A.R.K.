# S.P.A.R.K. Gate C2 — Bounded-Revision Implementation Report

**Date:** 2026-09-11
**Writer:** Claude Code (Opus 5, `claude-opus-5`), the separated correction writer. The
independent reviewer is Codex and is not the writer.
**Mission:** `engineering/phase2/SPARK_PHASE_2_GATE_C2_BOUNDED_REVISION_MISSION_2026-09-11.md`
**Branch:** `candidate/phase2-gate-c2-bounded-revision-20260911`
**Verdict:** `GATE_C2_BOUNDED_REVISION_READY_FOR_INDEPENDENT_REVIEW`

Nothing is accepted or promoted. Production is unchanged. Phase 3, G.A.M.E. changes,
Fable's research runtime, force-push, reset and paid compute were not used.

## 1. Summary

The revision corrects every finding of the independent review at
`00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0` (C2-01 … C2-08; D-C2-7, D-C2-11 and D-C2-13
revised; C2-09 restated), converts the review's seven failing counterexamples into
repository tests, and completes the oracle cases the review marked partial or
revision-required, each with a negative control. Completing the oracle surfaced one
further production defect, now fixed: exact-duplicate **obligation** emissions were not
canonicalized by emission identity (§5). The workspace passes **374 tests, 0 failed,
0 ignored** (232 inherited Phase-1 tests unmodified; 88 Gate C2 writer tests, a
documented subset adapted to corrected surfaces, none weakened; 54 new), formatting,
all-target clippy, the strict core/engine lint, metadata, both whitespace ranges, the five
static targets, the release workload, and 45 external compile probes (27 inherited, 18
new). The review's original corpus, run byte-unchanged, now fails to compile only on the
three surfaces the review's own corrections removed; its mechanically adapted form passes
9/9 (§8).

Nine representation choices where the frozen text is silent are flagged for review (§4).
None changes a frozen requirement or a Phase-1 encoding; none is, in the writer's
judgment, an unresolved architecture conflict, so no Operator question is raised (§11).

## 2. Lineage and custody

| Item | Value |
|---|---|
| Worktree | `/home/chromikey/Projects/SPARK-gate-c2-rev` (new, isolated) |
| Base (direct parent of checkpoint 0) | `00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0`, the review commit, verified local = tracking = live |
| Reviewed candidate | `053d1dc1131ec47be94b60513fad9ea8389cde0c` (ancestor) |
| Production | `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` (ancestor; live `phase1-refoundation-v2`, unchanged) |
| Checkpoint 0 | `b12ac9a` — mission recorded before any code change |
| Checkpoint 1 | `99f8774` — C2-01 … C2-08, D-C2-7/11/13; correction tests; corpus runs |
| Checkpoint 2 | `19930f3` — oracle completion, enqueue canonicalization, unforgeable diagnostics |
| Checkpoint 3 | documentation and evidence only (hash in the receipt; a commit cannot record its own hash) |

Every pre-existing branch and worktree was left intact (`preservation.json` lists the
worktrees). Preservation, checked mechanically (`check_preservation.py`): the seven
inherited integration-test files, all 17 inherited inline test modules, the manifests and
the lockfile are byte-identical to production; `schedule`, `drain_due`, `stage` and
`submit_fence` are byte-identical; `spark-core` changes relative to the review commit are
purely additive (+13/−0, one read-only accessor, §4 R-9).

## 3. Per-finding disposition

"Red" is the review's original corpus against the unchanged candidate sources, re-run in
this session before any production change
(`original_corpus_pre_correction.txt`: 3 passed, 7 failed — reproducing the review).

| Finding | Disposition | Correction (code) | Executed evidence |
|---|---|---|---|
| **C2-01** parent sets | **Corrected** | `plan_wave` unions, for every fired derived seed, the watched group and every condition-read group changed in the wave (`RuleSpec::condition_cell_reads`); no-candidate groups add nothing; set semantics dedupe | red→green `c2_01_…_converted` with the stress-only negative control; `c2_01_unchanged_background_contributes_no_parent` (equal identity, different batch digest); `c2_01_multi_target_operation_takes_the_union`; depth-2 `at_i6c_e_…` |
| **C2-02** scope fan-out | **Corrected** | door counts distinct (target, scope mapping) pairs + delay edges; each `Materialize` expansion bounded (`MaterializedFanOutExceeded`) | red→green converted test (exact bound admits, over-bound refuses; Subject vs Fixed distinct); `c2_02_materialized_expansion_is_bounded` |
| **C2-03** nested sub-IDs | **Corrected** | nested effects `claim` in the rule's single namespace, with qualification | red→green converted test for unequal (10/15) **and** equal (10/10) payloads; nested-vs-top-level collision; qualification; positive control keeps both equal causes (20) |
| **C2-04** widened arithmetic | **Corrected** | `weighted_aggregate` keeps sum × weight in checked `i128`, one floor; all-additive and staged rule bodies keep `i128` intermediates, one conversion | red→green converted `MAX` aggregate; `+MAX,+MAX,−MAX = MAX`; staged `(MAX+MAX)·0.5 = MAX`; true `i128` overflow typed; AT-I24 boundary corpus; AT-I7e |
| **C2-05** baseline, cadence, epoch rates | **Corrected** | see §4 R-1/R-2/R-3: rule-set baseline declaration materialized into `StateCell.baseline`; decay targets the field; door rejects decay without declared baseline and invalid literals; config parameters validated per epoch; pure decay commits at its last whole step; epoch-by-epoch integration over the retained lineage | red→green remainder (80/80/80) and epoch-rate (90) converted tests; exhaustive 32-subset chunk-invariance sweep (value **and** commit time); barrier composition incl. unaligned barrier; interval without the operation integrates nothing; field-not-declaration target; recovery; door/genesis/activation validation; sanctioned compositions (AT-I22) |
| **C2-06** reserved kind | **Corrected** | command cohort binds kind ↔ payload both ways (`EpochActivationRejected` with a reason; still finalized, `Completed`); door rejects rules triggered by the reserved kind | red→green converted test with registry/rule set/lineage unchanged; reverse direction applies no ingress; door refusal; positive control |
| **C2-07** oracle overstatement | **Corrected** | `cohort_identity` removed from `CohortReport` (§4 R-7); `command_deferred` removed — `Paused` with `deferred_cohort_count == 0` is the frozen-field equivalent; every P/R row addressed (§6); claims corrected (§9) | AT-I46(a) pin now destructures the frozen set; probes for `cohort_identity`, a pre-wave field, `F`, `ActiveRequest`, `command_deferred` |
| **C2-08** refusal construction | **Corrected** | every `FinalizationRefusal` variant `#[non_exhaustive]`; external `{ .. }` matching still compiles | probes: unit and struct variant construction fail; `FinalizationRecord` absent; positive control matches a refusal. The unchanged corpus's construction line now fails to compile |
| **C2-09** note | **Restated** | none required | fresh workload §10; `cells_of` scans the cell map — §12 |
| **D-C2-3** | **Withdrawn**, replaced by C2-05 | | |
| **D-C2-5** | **Revised** (C2-06) | | |
| **D-C2-7** | **Revised** | §4 R-4: exact originating-artifact resolution in the lineage + creator epoch record + current-epoch identity; barrier-scoped config | `d_c2_7_…_originating_artifact` kills "historical hash present anywhere"; `RuleSuperseded` for a changed current rule; `d_c2_7_config_reads_are_barrier_scoped` |
| **D-C2-11** | **Revised** | extraction refusal inside `process` → typed sticky `Outcome::StoreInvariantViolated(ExtractionRefusal)` fail-stop (§4 R-5) | `d_c2_11_…`: earlier cohort honestly reported, not dequeue-eligible, snapshot/reset refused, sticky, restore recovers |
| **D-C2-13** | **Revised** | materialized emissions use the new `creator_rule_fingerprint` record field (§4 R-6) | `d_c2_13_…`: equal identities for payloads 10 and 15, equal to the independent formula; the old record-hash component differs |
| D-C2-1, -2, -4, -6, -8, -9, -10, -12, -14, -15 | **Preserved** as accepted, with the review's caveats | | |

## 4. Flagged representation choices (for the reviewer)

**R-1 Baseline declaration channel (C2-05).** v1 Q7 makes `StateCell.baseline` the decay
target and requires rejecting decay rules on definitions without baseline semantics, while
freezing `DefinitionSpec`/`StateCell`/manifest/config encodings; it names no declaration
channel, and Phase 1 never populates the field. The only artifact the rule-set door sees
that Phase 2 may shape is the rule set, so `RuleSetSpec.baselines` declares
`(definition, value)` (validated: known, numeric, not host-owned, within bounds, unique;
part of `ruleset_content_hash`). Engine writes materialize the declaration into the
existing field **once** (never overwritten); a cell last written before its definition's
baseline was declared takes the current declaration, which the decay commit then
materializes. The rule's former `toward` operand is removed. A declaration changed by a
later epoch therefore seeds new cells but does not silently re-target existing ones
(pinned by `c2_05_decay_moves_toward_the_cells_baseline_field`). If the Operator prefers
epoch-re-targeting baselines, that is a bounded follow-up, not an encoding change.

**R-2 Cadence and epoch integration (C2-05).** Q7's formula counts whole cadence steps
between `updated_at` and now; the review showed committing at `now` drops the remainder. A
pure decay transform now commits its value at the end of its last whole step (the value
the cell truly has at that time), so the remainder survives and N separate evaluations
equal one catch-up for any evaluation times (swept exhaustively). Integration is epoch by
epoch: each whole step takes the parameters of the epoch in effect at the step's end (a
boundary exactly at an activation time belongs to the old epoch, matching that scheduled
work at that time runs before the activating command); a partial step completes under the
next epoch; an interval under an epoch whose artifact lacks the operation integrates
nothing. Any non-decay write — including a rule body combining decay and a shock — commits
at the cohort time and re-anchors. Rate and cadence are rule parameters: a literal or a
config key, the key read under each interval's epoch.

**R-3 Retained epoch lineage.** Both R-2 and D-C2-7 need each epoch's rule set, config and
activation time. They are retained as a lineage (the v1 §8 immutable-activation-artifact
class). They add no canonical content: every rule set and config is the artifact whose
hash the digest-committed `EpochRegistry` record binds, and each activation time is the
effective time of the finalized command whose semantic hash that record binds. The lineage
is therefore a derived index outside the frozen `engine_state_digest` composition
(unchanged, AT-I35) and is validated at restore (`RestoreError::EpochLineageInvalid`,
tested with shifted, truncated and genesis-timed tampering).

**R-4 D-C2-7 compatibility rule, stated exactly.** A re-evaluation record executes iff
(1) its exact originating artifact resolves in the lineage — an activated rule set with
the recorded content hash containing the rule with the recorded fingerprint — and its
creator artifact hash is an epoch record of this registry (else `RuleResolutionFailed`);
and (2) the current epoch carries that rule bit-identically (else `RuleSuperseded`); so
executed logic is always the exact originating logic and never superseded behavior.
(3) Config inputs are ADR-0004 class-1 hot-tunable and read barrier-scoped: a point read
uses the config of the epoch in effect at evaluation, and decay integrates each interval
under that interval's epoch. Every config read is thus an exact, hash-identified artifact
of the lineage. This reconciles ADR-0006's exact-artifact requirement with AT-I23; the
reviewer should confirm it is the intended reading.

**R-5 Store-invariant fail-stop (D-C2-11).** An extraction refusal inside `process` is an
implementation or tampering defect; repeating the request cannot help. It now applies the
accepted D-8 discipline: a typed, sticky `StoreInvariantViolated` carrying the refusal, no
snapshot or reset, restore of the last committed snapshot recovers. The extraction attempt
mutated nothing; `ActiveRequest` and any cohort committed earlier in the call did change,
and the result reports those cohorts.

**R-6 Record field (D-C2-13).** `ObligationRecord.creator_rule_fingerprint` (v1 §5 lists a
creator fingerprint) supplies the v3 §4.4 rule-fingerprint component for materialized
emissions; it enters the record hash (a Phase-2, unaccepted encoding).

**R-7 Report field removal (C2-07).** v3 §3.4 lists cohort identity among the canonical
report's contents; FINAL AT-I46(a) — later and controlling — forbids a `cohort_identity`
field. The field is removed; cohort identity remains bound into every emission identity and
`effect_batch_digest` the report carries, and is observed through `test-support`.
`PacingDiagnostics` is now exactly the frozen v3 §3.4 field list and, per v3 AT-I32,
`#[non_exhaustive]` with no public constructor.

**R-8 Enqueue canonicalization (§5).** v2 §3.2's idempotency is applied to obligation
emissions: exact duplicate enqueue claims fold; one identity with a different claim is a
contested emission (least identity reported).

**R-9 Additive `spark-core` accessor.** `Scheduler::slot_commitment(&WorkKey)` (read-only,
bounded lookup, exposes neither payload nor evidence) lets the wave enqueue preflight check
every touched key's **full** commitment rather than its slot shape (AT-I8).

## 5. Defect found by the completed oracle

`at_i6c_d_derived_redelivery` (exact redelivery of a derived emission through the
seed-duplication seam) produced two obligations, at occurrences 0 and 1, for one emission
identity: candidates folded by identity, but enqueue claims were not canonicalized. Fixed
in checkpoint 2 (R-8); the same test and `at_i6a_…` now show a single obligation and
byte-identical reports and digests versus the non-duplicated run, and a contested claim
rejects. The defect is unreachable without the seam (engine seeds are unique by
construction), which is exactly the defense-in-depth layer v2 §3.2 requires.

## 6. Per-entry oracle status after revision

Status: **S** executed coverage supports the stated behavior (not a universal proof);
**P** partial, remaining gap named. Files: **B** `phase2_bounded_revision.rs`, **O**
`phase2_oracle_completion.rs`, **X** `phase2_compile_probes.rs`, others as in the review.

| Entry | Review | Now | Evidence / remaining gap |
|---|---|---|---|
| AT-I1 | P | S | O `at_i1_…`: post-wave digest = untouched clone + same effects; sequential re-read control differs |
| AT-I3 | P | S | O: 24 declaration permutations × 2 insertion orders, RESULT coalescing + singleton TRANSFORM + additive |
| AT-I4 | P | S | O: each of six `WorkKey` fields moves cohort identity; engine-level formula |
| AT-I5 | P | S | O: renamed/reordered IDs — same rejection evidence, same results |
| AT-I6a | P | S | O: engine-level exact fold / contested rejection in every permutation (seam); obligation emissions fold (§5) |
| AT-I6c | R | S | C2-01 corrected; B union/background; O (d) redelivery, (e) depth 2, (f) forged empty set |
| AT-I6d | P | S | O renaming sub-case (identity = complete-set formula, never the minimum); probe |
| AT-I7e | P | S | O three-term `i128` overflow typed and atomic |
| AT-I8 | P | S | O full-commitment preflight (shape-only control); invariant after every committed wave |
| AT-I13 | P | P | unchanged honest interpretation: a fixed manifest cannot migrate a definition |
| AT-I14 | P | S | B exact originating artifact, unknown creator, `RuleSuperseded`; R-4 |
| AT-I15 | P | S | O contested records from wave-1 derived emissions under one rule set, both orders |
| AT-I16 | P | S | O derived competing claims, ascending identity, pacing split unchanged |
| AT-I17 | R | S | B nested sub-IDs |
| AT-I19 | R | S | B scope breadth and materialized expansion |
| AT-I20b | P | S | O effects/enqueue caps, terminal, replay, retry; candidate cap as before |
| AT-I20c | P | S | O (f) combined oversized cohort with co-target and threshold inputs |
| AT-I20d | P | S | X `EvalView` unreachable; `PacingDiagnostics` unforgeable; evaluator view has no diagnostics by construction |
| AT-I21 | P | P | O interference: determinism, priority, exact identity; depth overflow still reached only through the test-support depth seam (sound static validation precludes it) |
| AT-I22 | R | S | B remainder, exhaustive chunk sweep, baseline; O sanctioned compositions and cross-family rejection |
| AT-I23 | R | S | B epoch-crossing decay (90), compositions, unaligned barrier, absent-operation interval; F additive test relabelled as config point-read scoping |
| AT-I24 | R | S | B widened aggregate; O `i128` boundary corpus |
| AT-I25 | R | S | B union, unchanged background, multi-target |
| AT-I26 | P | S | O each Phase-2 store alone moves exactly its component and the engine digest |
| AT-I27 | P | S | O permuted cooldown/occurrence construction through an epoch activation and a timeline reset; contested-claim permutations and caps as before |
| AT-I28 | P | P | B lineage tampering refused at restore; an injected **timeline** derived-index inconsistency is still not executed (no core seam) |
| AT-I30 | P | S | O 300 dormant scopes: same cohort, candidate set, effects, identities, outer selections |
| AT-I31 | P | S | O aggregate moves only at its cadence; promotion reads the committed aggregate |
| AT-I32 | R | S | 45 probes incl. C2-08, diagnostics, internal types; strict lint |
| AT-I33 | P | P | O exact 64-parent identity at the candidate cap, cap−1 typed; multi-parent chain at depth 2, not at the maximal configured depth |
| AT-I35 | P | S | O golden bytes for `None`/`Advance`/`Command`, independent boundary reference, three mutants |
| AT-I38 | P | S | O parent set covers a cause beyond the presentation cap |
| AT-I39 | P | P | O (E) heartbeat absent from recomputed batch digests; cross-profile companion remains per-profile (D-C2-2) |
| AT-I40 | P | P | unchanged: per-profile engine accepted (D-C2-2) |
| AT-I41 | P | S | O `effect_batch_v3` recomputed from components for two cohorts in one call; cohort-scoped wave index; drain-scoped mutant falsified |
| AT-I44 | P | S | O (c′) mixed slice: conflicts first, complete removal, post-extraction digest |
| AT-I46 | R | S | field removed; corrected pin; probes; (c) schema stability by construction (no feature-gated report fields) |
| AT-I50 | P | S | O (d) a different payload under the same envelope fields diverges |
| all others | S | S | unchanged, re-executed in the 374-test run |

## 7. Tests: added, adapted, none weakened

Added: 25 in B, 29 in O, 18 probes in X. Adapted Gate C2 writer tests (inherited Phase-1
tests untouched), each preserving or strengthening its check:

- `phase2_finalization_and_replay`: refusal expectations constructed externally → full-field
  `matches!` guards (C2-08); the changed-rule assertion `RuleResolutionFailed` →
  the more specific `RuleSuperseded` (R-4).
- `phase2_request_boundary`: `command_deferred` → `Paused` with `deferred_cohort_count == 0`.
- `phase2_waves_and_pacing` / `phase2_stores_and_semantics`: `cohort_identity` comparisons read
  the test-support seam; decay fixtures declare the baseline they now require (`toward: 20`
  → declared baseline 20); the depth-seam `RuleSetSpec` literal gains `baselines: vec![]`;
  the AT-I46(a) destructuring pins the corrected field set.
- `phase2_rule_set_door`: the decay-reducer case declares a baseline and additionally
  asserts no `DecayWithoutBaseline`.
- `phase2_compile_probes`: struct literals gain the new fields; the engine-literal probe
  names `fail_stop`.
- `effects` unit test: `decay` → `move_toward`, adding segment-composition and widened
  aggregate assertions.

## 8. The review's counterexample corpus

| Run | Tree | Result |
|---|---|---|
| unchanged, before correction | review worktree at `00d647e` | 3 passed, 7 failed — reproduces the review |
| unchanged, after correction | this candidate | does not compile, only at: `FinalizationRefusal::WrongProfile` construction (C2-08), `report.cohort_identity` (C2-07), `Update::Decay { toward, … }` (C2-05) |
| adapted A1–A4, after correction | this candidate, `test-support` enabled | **9 passed, 0 failed** |

A1 removes the construction probe (its compiling was the defect); A2 reads cohort identity
through the seam; A3 declares the baseline the `toward: lit(0)` operand expressed and spells
rate/cadence as `Param`s, values unchanged; A4 enables `test-support` in the disposable crate
because of A2. Each is documented in `independent_counterexamples_adapted.rs`.

## 9. Corrections to earlier claims

- **Red-first.** The original writer's baseline README records every AT-I entry as
  inexpressible against the inherited tree; that is a baseline statement, not an executed
  red run for each implementation slice. Engine implementation landed in `b7d849f` and most
  request/wave/finalization tests in later checkpoints, so the original report's
  "red-first" claim is **overstated** and is corrected here. History is not rewritten. In
  this revision, the converted tests have executed red evidence (the pre-correction corpus
  run, recorded before any production change); the oracle-completion tests were written
  after the corrections and are **not** claimed red-first — their negative controls
  (named in each test) are the discriminating evidence.
- **Coverage.** The original §6 table's "I" labels are superseded by §6 above.
- **AT-I23.** The original additive config-read test is relabelled; real decay coverage is
  in B.
- **AT-I46(a).** The original pin required the forbidden field; corrected.

## 10. Validation (fresh; tree `19930f3`)

`checks.json` records exact commands, exit codes and elapsed seconds; each `<name>.txt`
holds the full output.

| Check | Result |
|---|---|
| `cargo fmt --all --check` | exit 0 |
| `cargo test --workspace --all-features --no-fail-fast` | **374 passed, 0 failed, 0 ignored** |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| strict core/engine lint | exit 0 |
| `cargo metadata --format-version 1` | exit 0 |
| `git diff --check 00d647e HEAD` / `7e3a0aa HEAD` | first pass failed only on raw corpus logs (trailing spaces/blank line); logs normalized and regenerated; both ranges clean on the exact staged final tree (`whitespace-final.txt`) |
| five Windows/Android `cargo check --all-targets` | all exit 0 (static only) |
| original corpus unchanged / adapted | fails to compile only on the three corrected surfaces / 9 passed, 0 failed |
| preservation | no failures (`preservation.json`) |
| release workload | exit 0; preflight 3.5 / 2.5 / 2.5 µs at 1k / 4k / 16k finalized commands; forbidden scan 10.7 µs → 1.14 ms; whole finalization 1.2 → 14.4 ms; 16k build 129.9 s |

## 11. Operator decision status

No genuine unresolved architecture conflict blocked the pass: every correction implements
the frozen requirement the review named, and each place the frozen text is silent is filled
explicitly and flagged (§4) rather than invented silently. The two choices most likely to
merit Operator attention after independent review are R-1 (baselines seed cells once) and
R-4 (barrier-scoped config under exact-artifact rule resolution). No Operator question is
raised by this pass.

## 12. Limitations

1. Whole finalization remains `O(history)` through the unchanged Phase-1 `submit_fence`
   (inherited; §10 figures are one-machine measurements, not benchmarks). Pre-wave
   `engine_state_digest` re-hashes whole stores; `StateStore::cells_of` filters the complete
   cell map, so aggregation's physical scan is not bounded — AT-I30 asserts report counts
   and selections only.
2. The lineage grows by one artifact pair per behavior epoch (bounded by activations).
3. AT-I13, AT-I21, AT-I28 (timeline index injection), AT-I33 (maximal depth), AT-I39/AT-I40
   (cross-profile) remain partial as stated in §6.
4. No durable storage, process-crash recovery, durable mailbox, transport, GAME integration,
   or executable Windows/Android parity; static target checks only.
5. No independent review of this revision exists yet.

Next action: independent Codex HIGH review under
`engineering/phase2/SPARK_PHASE_2_GATE_C2_REVISION_INDEPENDENT_REVIEW_MISSION_2026-09-11.md`.
