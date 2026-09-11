# S.P.A.R.K. Gate C2 — Phase-2 Implementation Report

**Date:** 2026-09-11
**Writer:** Claude Code (Opus 5, `claude-opus-5`), separated implementation writer. The
independent reviewer is Codex and is not the writer.
**Mission:** `engineering/phase2/SPARK_PHASE_2_GATE_C2_IMPLEMENTATION_MISSION_2026-09-11.md`
**Branch:** `candidate/phase2-gate-c2-implementation-20260911`
**Verdict:** `GATE_C2_CANDIDATE_READY_FOR_INDEPENDENT_REVIEW`

Phase-2 implementation was authorized by the Operator for this writer pass. Production
promotion, Phase 3, G.A.M.E. changes, and paid external compute were not authorized and did
not occur. The Fable research runtime was not merged or consulted as code.

## 1. Summary

The frozen Phase-2 rule/effect runtime and the accepted V3-F01 correction are implemented in
`spark-engine`, with the frozen additive surfaces in `spark-core`, and every applicable
acceptance-oracle entry is encoded as an executable Rust test or external compile probe —
fully for most entries, partially for some sub-cases listed in §6. The workspace passes
**320 tests, 0 failed, 0 ignored** (232 inherited, unmodified, plus 88 new), formatting,
all-target clippy, the strict core/engine lint, `cargo metadata`, and all five
Windows/Android static target checks. Both acceptance pins are implemented and tested. One
inherited performance property — whole finalization is `O(history)` because Phase-1
`submit_fence` re-hashes finalized history — is measured and disclosed (§8).

## 2. Baseline and lineage

| Item | Value |
|---|---|
| Live remote at entry | `phase1-refoundation-v2` = `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5` (also `accept/v3-f01-rev2-freeze-20260910`); `review/v3-f01-rev2-20260910` = `e55b1da1c9049b1de58fcb06c65eea59939f2a57`; candidate `5ecc95c033bf3a7744bb7862bf959066e6561670` on its branch |
| Controlling acceptance | `SPARK_PHASE_2_V3_F01_OPERATOR_ACCEPTANCE_FREEZE_2026-09-10.md` (in `7e3a0aa`) |
| Branch | `candidate/phase2-gate-c2-implementation-20260911`, absent locally and remotely at entry, created at `7e3a0aa` in the new worktree `/home/chromikey/Projects/SPARK-gate-c2` |
| Worktrees preserved (all clean at entry) | `/home/chromikey/Projects/SPARK` (`review/v3-f01-final-20260910`), `SPARK-v3-f01-rev2-writer`, `SPARK-v3-f01-writer`, `SPARK-fable-challenge-worktree`, `/tmp/spark-v3-f01-production`, `/tmp/spark-v3-f01-rev2-accept`, `/tmp/spark-v3-f01-rev2-review` |

Checkpoint commits (each compiles and passes its tests; see §9 for two lint findings fixed
forward):

| # | Commit | Content |
|---|---|---|
| 0 | `4b02532` | mission recorded; inherited baseline 232/0/0; red-first record |
| 1 | `aad4633` | `spark-core`: X-1 … X-4, PX-1 accessors, derived-index validation; 13 tests |
| 2 | `b7d849f` | `spark-engine` runtime, rule-set door, engine composition, seams, testkit fixtures; 12 tests |
| 3 | `5fe883f` | request-boundary and waves/pacing/rejection suites; 33 tests |
| 4 | `0423d91` | finalization/consumer/replay/epoch suite and Phase-2 external compile probes; 15 tests |
| 5 | `4997d56220000edef3a2b4cd987feff7900d2488` | stores/semantics suite, workload example, forward lint fix; 15 tests |
| 6 | final documentation commit (hash in the receipt) | evidence, this report, the review mission, PHASE_STATUS |

## 3. Reading performed

The acceptance freeze, the Revision-2 independent review, the Revision-2 candidate and
oracle, the FINAL candidate and oracle, the V2 candidate (loop, rejection classes,
equivalence, boundedness sections), Phase-2 freezes v1, v2, v3 and matrices v1, v2, v3, the
`ActiveRequest` decision, the serialized-boundary review, ADR-0003, the development handoff,
PHASE_STATUS, and the convergence protocol. No repository `AGENTS.md`, `CLAUDE.md`, or
`CODEX.md` exists. Phase-1 sources read directly: `scheduler.rs`, `timeline.rs`, `state.rs`,
`activation.rs`, `value.rs`, `hash.rs`, `clock.rs`, `evidence.rs`, `random.rs`, `id.rs`,
`scope.rs`, `authority.rs`, the profile modules, and the compile-probe and dependency-hygiene
tests. The foundational time adjudication and the matrix V2 candidate were consumed through
the FINAL/V2 texts that carry them rather than re-read line by line.

## 4. What was built

### 4.1 `spark-core` (additive only)

- `scheduler.rs`: X-1 `least_due_slice` (least slice only, no payload/evidence), X-2
  `SliceFingerprint` (the frozen per-slot block, now shared with `canonical_state_digest`
  byte-for-byte), per-slot `SlotCommitment`, X-3 `take_least_due_slice` (target fixed;
  typed byte-identical `NoSliceDue`/`SliceChanged` no-ops), X-4 `due_slice_summary`
  (noncanonical). `schedule` and `drain_due` untouched.
- `timeline.rs`: PX-1 `finalized_command_claim`, `finalized_source_sequence_claim`,
  `last_finalized_source_sequence` (bounded `BTreeMap` reads), and
  `derived_indexes_consistent` (restore validation only). `stage`/`submit_fence` untouched.

### 4.2 `spark-engine`

| Module | Frozen source |
|---|---|
| `rules.rs` — closed typed rule language, `DeclaredBudgets`, door validation | v1 Q1, Q4; v2 §4; v3 §3.2(a) |
| `activation.rs` — `activate_rule_set` door, separate rule-set lineage | v1 Q1 |
| `epoch.rs` — hash-chained `EpochRecord`/`EpochRegistry`; `behavior_artifact_hash` = record hash | v1 Q5 |
| `obligation.rs` — records (both modes, multi-target fingerprints), claim-set `ObligationStore` | v1 §5, v2 §8 |
| `ledger.rs` — `OccurrenceLedger`, `CooldownLedger` | v1 §6, v2 §9 |
| `effects.rs` — cohort/command identities, parent contexts, emission identity, canonicalization, typed reducer, provenance, numeric model | v2 §3–§5, §10; v3 §4 |
| `report.rs` — canonical cohort/wave reports, `PacingDiagnostics`, typed rejections | v3 §3.4; V2 cand. §8 |
| `request.rs` — requests, `RequestDiscriminator`, `canonicalize_active_request`, results, `terminates` | FINAL §6, §8; Rev2 §5 |
| `engine.rs` — the composition (below) | FINAL §9–§17; Rev2 §4–§7 |
| `state.rs` — read-only `cells_of`, `validate_effect`, `validate_observation` | v1 Q3 |
| `fixture.rs` — `test-support` seams only | FINAL §15 |

`Engine` owns every store by value and implements: genesis (epoch 1, initial work);
`process` (P0 … A7 with Revision-2 A6); the atomic cross-store extraction (FINAL §11: slot
commitments compared to the store's expected commitments before either store mutates); the
wave pipeline (evaluate from an `EvalView` that holds no horizon/`F`/`ActiveRequest`/pacing
state → canonicalize → reduce → thresholds and propagation after complete reduction → depth
conversion → complete preflight incl. caps, occurrence ranges, admission caps,
touched-key bidirectional consistency → infallible apply → `effect_batch_v3`); command
cohorts (ingress observations validated-all-then-applied, then rules of the command kind) and
epoch-activation cohorts; finalization (P-1 … P-9 with PX-1 lookups, entailed stage and
one-ordinal `fence.<n>` fence, sticky `FINALIZATION_ENTAILMENT_VIOLATED`); epoch reset;
snapshot (refused when fail-stopped); restore (artifact binding, epoch chain, `ActiveRequest`
vs `F`, I-CS, derived indexes, exhaustive bidirectional invariant, digest); and
completed-boundary reconstruction (resets in ascending `reset_index`, recomputed-digest
verification). Digests: `engine_state_digest` (v1 §8 composition, excludes `F` and
`ActiveRequest`) and `stable_boundary_digest`.

### 4.3 `spark-testkit`

`phase2.rs` (fixture builders through the production doors), `consumer.rs` (reference
request-bound bounded FIFO consumer — a test harness, not a production mailbox), seven
`phase2_*` test files, and `examples/gate_c2_workload.rs`.

## 5. Tests added (88)

| File | Tests | Scope |
|---|---:|---|
| `phase2_core_surfaces.rs` | 13 | AT-I42 scheduler level, AT-I35 block/cap pins, AT-I40 slices, PX-1 |
| `spark-engine` `effects` unit tests | 2 | v2 §4.4 mandated examples; numeric floor rule |
| `phase2_rule_set_door.rs` | 10 | AT-I3, I10, I17, I18, I19, I20c(e), I22/I24 reducers, I37 (static), I35 lineage |
| `phase2_request_boundary.rs` | 15 | AT-I43, I39 (A)–(D), I20d, I47 (a)(b)(c)(f)(g)(h), I35 encoding pins |
| `phase2_waves_and_pacing.rs` | 18 | AT-I1, I5–I9, I6c/6d, I7a–e, I20/20b/20c, I21, I25, I37 (runtime), I40(e), I42 (l)(m)(n), I44, I45 |
| `phase2_finalization_and_replay.rs` | 14 | AT-I48 (a)–(j), I49 (a)–(d), I47 (d)(e), I50 (a)–(d), I13, I14, I23, pin 1 |
| `phase2_compile_probes.rs` | 1 harness, 27 forbidden surfaces + positive control | AT-I32, I2, I6d, I20d, I29 boundary, I46 (b)(d) |
| `phase2_stores_and_semantics.rs` | 15 | AT-I4, I11, I12, I15, I16, I17, I22, I24, I26–I29, I30, I31, I33, I38, I46(a), cooldowns |

No inherited test was modified, weakened, ignored, or deleted. Red-first: every entry was
recorded as inexpressible against the inherited tree before any production change
(`gate_c2_evidence_2026-09-11/baseline/`); the historical FINAL-01 red recipe is re-executed
as a negative control (AT-I48(a)).

## 6. Acceptance-oracle coverage

Status: **I** implemented and executed; **P** partially — executed with the named sub-case
missing or covered only at unit level; **C** covered by construction plus an executed check.

| Entry | Status | Notes |
|---|---|---|
| AT-I1 | I | pre-wave-only reads, `20+20+20=60`; digest-vs-clone compared via permutation digests |
| AT-I2 | I | compile probe (`run_waves` unreachable); wave-1 sees exactly wave 0's commit (AT-I44 a′) |
| AT-I3, I4, I5 | I | door hash invariance; engine permutations; canonical order |
| AT-I6a | P | exact-duplicate fold and contested identity tested at the reducer (unit); engine-level duplicate emission is unreachable (identities include rule fingerprint, sub-ID, parent context) |
| AT-I6b | I | `+10/+10 → 40`, two causes |
| AT-I6c | P | (a)(b)(c) against hand-computed formulas; (d) unit level; (e) depth transitivity and (f) empty parent set not exercised (the `EmptyParentSet` rejection exists but is unreachable through the facade) |
| AT-I6d | P | third lower parent changes the set; compile probe; rule-renaming sub-case not written |
| AT-I7a–e | I | engine-level fold, conflict, coalescence, mixtures, overflow and bounds |
| AT-I8 | P | occurrence mid-range exhaustion, obligation and scheduler admission caps; the preflight bidirectional-invariant failure is not reachable without tampering and is not exercised in a wave |
| AT-I9, I10, I11, I12 | I | |
| AT-I13 | I (interpreted) | multi-target drift refusal and matching execution, via a seam-built record: under a fixed manifest, same-ID definition fingerprints cannot change (Phase-1 `IdentityConflict`) |
| AT-I14 | I | superseded rule refuses; identical rule executes |
| AT-I15 | P | 3-record permutations, discrimination, conflict-only drain, reschedule, beyond-cap agreement; a contested record originating from a wave-1 derived emission not constructed |
| AT-I16 | P | ascending-emission-identity allocation, pacing-split invariance; wave-N derived competing claims not constructed |
| AT-I17, I18, I19 | I | |
| AT-I20, I20b | I | |
| AT-I20c | I | (a)–(e), (g)–(i); (f) co-target additive causes in the oversized cohort (one committed write, exact sum); its threshold-input sub-case is covered by AT-I25 separately |
| AT-I20d | I | inequality, determinism, equal digests across budgets, compile probe |
| AT-I21 | I (seam) | conversion to `due ≥ now+1`, not run in the same call, converges in value; reachable only through the test-only depth-bound seam because sound static validation precludes runtime overflow; cross-depth-budget digests necessarily differ (declared behavior, v2 C-11) |
| AT-I22 | P | chunk invariance at cadence-aligned evaluations, exact convergence, remainder kept, cross-family rejection; the two sanctioned compositions are not separately pinned |
| AT-I23 | I | |
| AT-I24 | P | permutation invariance, exact floor, one-reducer door check; `i128` accumulation overflow not driven |
| AT-I25 | P | both partial-fold directions, parent-set coverage; the unchanged-background and multi-target-union fixtures not written |
| AT-I26, I27 | I | EpochRegistry, ObligationStore (contested), OccurrenceLedger, CooldownLedger |
| AT-I28 | C | the engine has no derived index of its own; timeline indexes validated at every restore |
| AT-I29 | I | snapshot/restore at every stable boundary incl. after paced deferral reproduces all results |
| AT-I30, I31 | I | |
| AT-I32 | I | strict lint on all new modules; 27 external probes |
| AT-I33 | P | `i64::MIN` negation, `u64::MAX` times/horizon, maximum qualified IDs, empty rule set, work far above budget; maximal parent-set cardinality not driven |
| AT-I34 | I | 232 inherited tests unmodified |
| AT-I35 | I | scheduler digest and fingerprint pins, `ActiveRequest`/boundary/engine-digest pins, Phase-1 lineage digest unchanged, inherited pinned-hash tests green |
| AT-I36 | I | five static targets |
| AT-I37 | I | static and runtime tables |
| AT-I38 | P | exact fold, coverage counts, `complete` iff nothing omitted, policy identity; later-wave identity difference beyond the cap not asserted |
| AT-I39 | P | (A)–(D); (E) the retired heartbeat is never computed anywhere, not separately asserted |
| AT-I40 | P (interpreted) | per-profile slices at scheduler level; (e) complete; engine is per-profile (see D-C2-2) |
| AT-I41 | P | cohort-scoped wave index exercised through catch-up equivalence; the explicit `effect_batch_v3` recomputation from components is not written |
| AT-I42 | I | (a)–(n) |
| AT-I43 | I | (a)–(k); (c) via cohort-local `updated_at` and decay |
| AT-I44 | P | (a)(a′)(b)(c)(d)(e)(f)(g); (c′) mixed slice whose executable part rejects not written |
| AT-I45 | I | (a)(b)(c′)(d); (e) via budget equivalence tests |
| AT-I46 | I | (a) field-set pin, (b) hygiene test + probe, (c) by construction, (d) probe, (e) seam |
| AT-I47 | I | (a)–(h) |
| AT-I48 | I | (a)–(j), incl. a 360-case differential corpus and sticky fail-stop fault injection |
| AT-I49 | I | (a)–(d) |
| AT-I50 | I | (a)–(d); the missing-payload case is specified, not executed |

## 7. Implementation decisions and interpretations (for the reviewer)

- **D-C2-1 Pacing budget outside artifact identity.** `max_due_per_cycle` is declared and
  door-validated (`>= 1`) but excluded from `ruleset_content_hash`. Including it made every
  emission identity differ across budgets (found by the red run of AT-I43(b)), contradicting
  FINAL §16 PE-D and v3 §3.4. Semantic caps stay hashed.
- **D-C2-2 One engine per profile.** `StateStore`, timeline, and epochs are per profile by
  Phase-1/v1 construction; foreign-profile effects are refused by `validate_write`.
  Multi-profile `(due_time, profile_id)` ordering is exercised at the scheduler level.
- **D-C2-3 Decay target and alignment.** Phase-1 never sets `StateCell.baseline`; decay moves
  toward the cell's baseline if present, else the rule's declared `toward`. Chunk invariance
  holds for cadence-aligned evaluations; a call with no whole step emits no candidate.
- **D-C2-4 Absent values.** An absent cell contributes `0` as the additive fold base and as a
  crossing's previous value; inputs declare their own `absent` value.
- **D-C2-5 Command payload.** `Host { subject, observations, params }` (observations are
  host-owned ingress validated-all-then-applied before wave 0) or `ActivateEpoch { rule_set,
  config }`; the payload hash is the envelope's `canonical_payload_hash`.
- **D-C2-6 Epoch scope.** Epoch activation changes rule set and config; the manifest is fixed
  per engine (definition migration is a later phase). The random root seed is the config key
  `random.root_seed`, so it is committed through the epoch record.
- **D-C2-7 Re-evaluation resolution.** The current rule set must contain the rule with the
  exact fingerprint, and the recorded rule-set hash must appear in the epoch registry. Any
  refusing record refuses its whole cohort (R-7, no wave).
- **D-C2-8 Rule-body reducer.** Several operations of one rule on one target compose in
  declared order into one candidate (`AddDelta` if all additive, else `RuleBody` transform).
- **D-C2-9 Static mixture scope.** "Statically provable co-targeting" is evaluated between
  rules with identical triggers; everything else is judged at runtime.
- **D-C2-10 Depth.** Waves `0..=max_wave_depth`; the static bound compares the longest
  zero-delay chain (edges) to `max_wave_depth`; overflow converts at the last wave.
- **D-C2-11 Extraction refusal inside `process`** (only reachable by tampering) returns a
  non-terminal `Paused` with `ActiveRequest` set and nothing mutated.
- **D-C2-12 Derived-wave gate addressing.** Probability gates at wave `>= 1` use the first
  eight bytes of the parent-context digest as the address occurrence.
- **D-C2-13 Materialized emissions** use `H("materialized_obligation_v1" ‖ record hash)` as
  the rule-fingerprint component; schedule-operation emissions target `(rule, scope)`.
- **D-C2-14 Test-support seams** (`fixture.rs`, `activate_rule_set_without_depth_bound`,
  `ObservedDigest`): observation, preflight-row fault injection, raw staging, timeline
  resume/clone/debug, grant override, occurrence setter, obligation tampering, snapshot
  tampering. All absent from production builds (hygiene test and probes).
- **D-C2-15 Lint allows** with justification: `result_large_err` on the extraction refusal,
  `too_many_arguments` on `EpochRecord::new`, `large_enum_variant` on `Request`.

## 8. Validation (fresh, this session)

| Check | Result |
|---|---|
| `cargo test --workspace --all-features` | **320 passed, 0 failed, 0 ignored** |
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| strict core/engine library lint | PASS |
| `cargo metadata --format-version 1` | PASS |
| `git diff --check 7e3a0aa HEAD` | PASS |
| five Windows/Android `cargo check --all-targets` | all PASS (static only) |
| Phase-2 external compile probes | positive control compiles; 27 forbidden surfaces fail for the named symbol |
| Replay | completed-boundary reconstruction reproduces `stable_boundary_digest` and the complete timeline state; width change → `REPLAY_DIVERGED`; missing reset → refused; reversed input reset order still replays by `reset_index`; manual reverse application fails |
| Workload (release, one machine) | preflight-only 2.06 / 2.38 / 2.43 µs at 1k / 4k / 16k finalized commands; forbidden scan 6 µs → 1.14 ms; whole finalization 0.85 → 14.6 ms |

## 9. Limitations and honest notes

1. **Whole finalization is `O(history)`.** Phase-1 `submit_fence` returns a
   `FinalizationResult` carrying `canonical_history_digest()`, which re-hashes all finalized
   commands and fences on every fence; building a 16 000-command history took 123 s. Every
   pre-wave `engine_state_digest` likewise re-hashes the timeline and state store. The
   indexed preflight (pin 2) is flat; the inherited costs are not changed (Phase 1 is
   closed) and belong to the performance gate.
2. Partial oracle sub-cases are listed in §6; none is claimed as executed.
3. No durable storage, process-crash recovery, durable mailbox, transport, service, or
   executable Windows/Android parity. Snapshots are in-memory values.
4. Canonical reports are presentation values (compared by value), not digest-committed.
5. Two lint findings were fixed forward rather than before commit: the checkpoint-1
   formatting slip (fixed in checkpoint 2) and two test-only clippy findings in the
   checkpoint-3 suite (fixed in checkpoint 5). Checkpoints 2–4 were committed after their
   tests passed but before a full all-target clippy run; the tree at checkpoint 5 passes
   every gate.
6. The workload figures are single-machine measurements, not production benchmarks.
7. No adversarial review of this pass exists yet.

## 10. Operator decision status and handoff

No genuine unresolved architecture decision blocked the pass. The interpretations in §7,
especially D-C2-1 (pacing budget outside artifact identity), D-C2-2 (per-profile engine), and
D-C2-6 (fixed manifest per engine), are flagged for independent judgment; if the reviewer
rejects one, it is a bounded revision.

Next action: hand the exact candidate commit to independent Codex HIGH review under
`engineering/phase2/SPARK_PHASE_2_GATE_C2_INDEPENDENT_REVIEW_MISSION_2026-09-11.md`.
Production promotion and Phase 3 remain unauthorized pending that review and a separate
Operator decision.
