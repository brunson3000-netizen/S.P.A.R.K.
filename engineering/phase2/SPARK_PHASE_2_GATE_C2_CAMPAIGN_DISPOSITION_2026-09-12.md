# S.P.A.R.K. Gate C2 — consolidated adversarial-campaign disposition and acceptance recommendation

**Date:** 2026-09-12.
**Author:** the local main engineer (Claude Code, Opus 5, `claude-opus-5`), acting under
`engineering/SPARK_GAME_SAFE_HANDOFF_MISSION_2026-09-12.md` §1.
**Pinned build under disposition:** `67b877192cc78b75c6fbe60c69b5594dc10befe8`
(`candidate/phase2-gate-c2-w01-correction-20260912`), crate tree
`7907f4d729104fd5dbfd4adad46e66cf09aa13dd`.
**Controlling independent review:** `21a4fec666ca493f6ac4d5194ec6e9c5380ce40c`, verdict
`GATE_C2_W01_CORRECTION_KEEP`.
**Status:** a disposition and a **recommendation**. It accepts nothing. Gate C2 acceptance
is the Operator's decision and has not been given (§7).

## 1. What the receipts actually show

The mission required this section to be honest about elapsed time and novel coverage rather
than about intent. Reading the campaign artifacts themselves:

| Campaign | Window | Recorded elapsed | Cases | Novel cases |
|---|---|---|---|---|
| LUNA (original) | 30 min from 11:30:30Z | closeout at 11:34:00Z — **about 4 min** | 15 | **2** (13 inherited, by its own index) |
| SONNET (original) | 30 min from 11:30:40Z | handoff written ~11:40Z — **about 11 min** | 30 | **not stated by its artifacts** |

No SONNET artifact records how many of its 30 cases were novel against the existing
427-test workspace suite, and this table does not supply a figure it cannot source. Its
report describes all 30 as written for the campaign, but "written for the campaign" and
"covering a vector the suite did not" are different claims and only the first is recorded.

**Neither original campaign ran its full 30 minutes.** Together they consumed roughly 15 of
a granted 60 minutes. That is a limitation of the evidence, not a defect of the product, and
it is the reason a bounded continuation was arranged rather than the original results simply
being relabelled as complete.

Both original campaigns reported **no confirmed product defect**. That is a negative result
on the surface each exercised within the time it actually spent. It is not proof that no
defect exists, and nothing below treats it as one.

## 2. Bounded collection-only continuation (actually executed)

Two independent testers, neither the writer of the candidate nor the author of this
disposition, ran the **unused** portions of the original windows. Roles were preserved:
collection only, no product edits, no corrective candidates, isolated `/tmp` workspaces,
disposable external harnesses against the pinned commit verified against live GitHub.

| Continuation | Budget (unused time) | Actual elapsed | New cases | Reproducible bugs |
|---|---|---|---|---|
| LUNA-C | 26 min | **~7.2 min** (12:25:36Z → 12:32:50Z) | 3 | **0** |
| SONNET-C | 19 min | **~10.3 min** (12:26:13Z → 12:36:29Z) | 7 | **0** |

Both continuations were run by separate Claude Sonnet 5 (`claude-sonnet-5`) agent sessions
dispatched by the main engineer with collection-only missions. Recording this because an
independent reviewer noted that the continuation documents attest their own independence
without naming who held the role. What can be **verified** from the artifacts rather than
attested is the behavior: no file under `crates/` changed, the workspaces were isolated
under `/tmp`, and no corrective candidate was opened.

Neither continuation used its whole budget either; each stopped when its highest-value
vectors were exhausted rather than padding the clock. Recorded honestly: total adversarial
testing across all four passes is roughly **32 minutes** against a nominal 60, with **10 new
discriminating cases** added by the continuations and **zero reproducible defects** found.

Evidence: `gate_c2_campaign_disposition_evidence_2026-09-12/luna_continuation/` and
`.../sonnet_continuation/`, each carrying the campaign report, findings index, full test
source, complete captured run and SHA-256 sums.

New coverage the continuations added: seeded generated histories with mid-sequence
snapshot/restore and `reconstruct_completed` replay equivalence; epoch activation presented
while a different request is paused; `max_effects_per_wave` and `max_enqueue_per_wave`
semantic-cap reporting; snapshot/restore of a mid-paused command with correct, mismatched
and re-presented resumption; `reset_timeline_epoch` during a pause and the stale-epoch
finalization refusal that follows; fail-stop stickiness for `StoreInvariantViolated`
including refusal to publish a snapshot; staged-but-unfenced timeline residue rejected at
restore; a genuine conflicting `WorkKey` reported as `ConflictOnly` and surviving a
snapshot round trip; and `reconstruct_completed` refusing an adversarial history containing
a non-finalizing command.

## 3. Disposition of every recorded observation

The mission authorised the main engineer to diagnose these against requirements and
fixtures, and warned that source implementation alone is not proof of intended behavior.
Each disposition below therefore rests on a **discriminating test** that would come out
differently if the competing explanation were the true one. An independent review found
that an earlier revision of one of them — D-10 — did not meet that standard, and it has
been replaced; §3.2 records what was wrong and what changed. The diagnostics are in
`gate_c2_campaign_disposition_evidence_2026-09-12/diagnostics/`, run against the pinned
crate tree: **13 passed / 0 failed**. Throughout this document a bare `D-n` names a **test
function** and a numbered "check" inside one names an **assertion** within that test, so a
citation resolves against the source without re-deriving it.

| Observation | Competing explanations | Discriminating result | Disposition |
|---|---|---|---|
| **LUNA-001** — `i64::MAX`/`i64::MIN` assignment to `state.pressure` refused `OutOfBounds`, cell absent | (a) behavior for extreme values is unspecified; (b) the refusal is the declared `ValueConstraint` doing its job | **D-1**: the declared maximum `1_000_000_000_000` **commits**; declared-max **+1** refuses; `i64::MAX` produces the **byte-identical** refusal to declared-max+1; the same holds symmetrically at the minimum | **(b). Not unspecified, and not a defect.** The refusal boundary tracks the profile's declared constraint, not the machine integer range. The campaign's own "expected behavior was not specified" is the one claim here that is wrong; the manifest specifies it. |
| LUNA-001 (cell absent) | (a) refusal erases the cell; (b) absence is the unchanged initial condition | **D-1b** (the test `d1b_luna001_absent_cell_...`): with a refused *first* write the cell stays absent. **D-1 check 6** (an assertion inside the test `d1_luna001_...`): with a prior committed value, four consecutive refusals leave that value intact | **(b).** Refusal never erases. |
| **LUNA-002** — zero-rate evaluation commits canonical time | (a) a silent state change; (b) a value-preserving evaluation that still advances time | **D-9**: value identical before and after; frontier advances to the requested horizon | **(b).** Matches retained D-6 behavior, as the campaign itself recorded. Not a defect. |
| LUNA-003 — seeded sequences replay deterministically | — | reconfirmed independently by LUNA-C's 40-step seeded random walk with snapshot/restore and full replay equivalence | Passing observation. |
| **SONNET T-1** — resubmission returns `CommandIdentityConflict` rather than an idempotent `Completed` | (a) the engine rejects repeated *work*; (b) it rejects reuse of a command *identity* (P-7/D-2) | **D-2**: exact resubmission is a typed refusal leaving **both** digests byte-identical; the **same payload under a fresh command id and sequence finalizes normally** and moves the engine | **(b).** Identity-scoped, exactly as D-2 requires. Not a defect. |
| **SONNET T-2 / T-3** — zero-delay reschedule and zero pacing budget fail before runtime | (a) a runtime hang avoided by luck; (b) static admission refusals that make the hazard unreachable | **D-3**: both fail `rule_set` activation with typed errors, so **no `Engine` can be constructed that carries either hazard** | **(b).** Structural, not incidental. Not a defect. |
| **SONNET T-4** — an oversized first slice completes within a budget of 1 | (a) the pacing budget is ignored; (b) the first slice is always admitted to guarantee progress, and pacing still applies afterwards | **D-4**: with work at **two** due times and budget 1 the call **pauses**, reports `pacing_overrun = true`, and defers the later cohort — a budget that was simply ignored would have completed both | **(b).** A declared progress guarantee with honest overrun reporting. Not a defect. Recorded in integration contract §9.2, because an adapter must expect it. |
| **SONNET T-5** — request-level `Completed` while the cell stayed absent | (a) a refusal was swallowed; (b) two-level reporting, with the refusal at the cohort level | **D-5**: exactly one cohort carries `Rejected { InvalidEffect }` naming `state.pressure`, inside a request whose boundary completed | **(b).** Not a defect — and important enough that integration contract §9.5 makes reading both levels a contract obligation, since an adapter reading only the request level would believe a refused write succeeded. |
| **SONNET T-6** — a "different" profile restored successfully | (a) restore fails to validate profile identity; (b) content addressing made the second profile *identical*, so it was never a distinct profile | **D-6**: a manifest with genuinely different content (an added definition) yields a different `activation_hash`, and restoring across it is refused with `ArtifactBindingMismatch` | **(b), and the declared coverage gap is now closed.** |
| SONNET gap — obligation-drop bidirectional-invariant mutation (needed an internal `WorkKey`) | — | **D-7**: the key is reachable from the public `ObligationStore::keys()`; dropping one claim set **and resealing the digest** is caught by `BidirectionalInvariantBroken`, not by the digest check | **Gap closed.** The invariant, not the digest, is what catches it. |
| SONNET gap — a command staged directly into a snapshot | — | **D-8**: staging at the real frontier ordinal and resealing is refused with `TimelineStagingPresent` (Revision-2 §7 step 2b) | **Gap closed.** |
| **C2W-RN02** — composed settlement performs two pure walks, not one | (a) the repeated walk bills decay debt a second time, so a body taking the settled path commits a lower value than the same arithmetic on the additive-only path; (b) the repeat is read-only, and both paths commit the value the declared semantics predict | **D-10** (revised, §3.2): with 42 units of decay debt already billed once, the **all-additive** `AddDelta` path and the **mixed** settled path — two different code paths on identical state — both commit **68**, the hand-computed oracle value. A second billing walk would put the settled path below it. A positive control with a **binding** clamp at 60 commits exactly 60, proving the transform stage is genuinely evaluated rather than dropped. **D-10b** separately records run-to-run reproducibility, and is named as the determinism check it is. | **(b).** Recorded as a **disclosed cost limitation** (§4). Deliberately **not** optimized. |

### 3.1 One new observation, recorded rather than buried

**H-OBS-01 — a `test-support` seam reports success for a stage it did not perform.**
`spark_engine::fixture::stage_raw` and `fixture::snapshot_stage_raw` return
`Result::is_ok()`. `TimelineIngress::stage` returns **`Ok(StageDisposition::NotInAdmissionWindow)`**
for an ordinal outside the current admission window — a correct, retryable refusal that
consumes no capacity. The seams therefore answer `true` for an envelope they did not stage.

Proved by **D-11**: staging at frontier ordinal + 1000 reports `true`, and the resealed
snapshot then restores successfully because nothing was actually staged.

- **Severity: non-blocking, and not a product defect.** Both functions are
  `#[cfg(any(test, feature = "test-support"))]`. No production code path is affected, and no
  canonical behavior changes.
- **Why it is recorded anyway:** a tester trusting that boolean can believe it mutated a
  snapshot when it did not, and would then report "restore accepted a tampered snapshot" as
  a false defect. It is exactly the shape of finding that wastes an independent review.
- **No repair is proposed here.** Changing a `test-support` seam on the pinned candidate
  would modify the crate tree the KEEP review certified as byte-identical, for a
  non-blocking clarity gain. It is recorded for a future bounded pass to decide on.

### 3.2 A diagnostic that did not meet this document's own standard, and what replaced it

An independent review of the first revision of this disposition recorded one MAJOR finding,
and it was right. The original D-10 ran the identical composed-body history through two
freshly constructed engines **in the same process** and asserted that the committed value
and engine-state digest agreed. That establishes reproducibility. It does **not** establish
that the repeated settlement walk is read-only: had the second walk perturbed the result,
both runs would have been perturbed identically and the test would still have passed. The
test nevertheless printed "RN02 repeated settlement has no observable canonical effect",
and three sentences of this document presented it as discriminating.

The replacement compares **two different code paths on identical state**, against an oracle
computed from the declared semantics rather than from the engine:

- state: `state.stress` seeded to 100 at t=1 with a declared baseline of 0 and a decay of 7
  per cadence of 3, left to accrue to t=19. Eighteen logical units is six whole cadences, so
  6 × 7 = 42 of debt: the oracle says **58**, and both engines hold 58.
- at t=20, body **A** is `[Add(10)]` — all-additive, so the `AddDelta` reduction path — and
  body **B** is `[Add(10), Clamp(-WIDE, +WIDE)]`, whose clamp binds nothing but makes the
  body mixed, selecting the settled path with its repeated `settled` call.
- the oracle says both must commit **68** (no further cadence elapses between 19 and 20).
  Both do. A second billing walk would have driven the settled path below 68 while leaving
  the additive path at 68.
- **a positive control closes the obvious hole in that argument.** Checks 1–5 would also pass
  if the clamp stage were simply dropped, because body B would then *be* body A and the
  comparison would prove nothing. So the same body is run once more with a clamp that
  **binds**, at 60: the result changes to exactly 60, and differs from the additive-only 68.
  The transform stage is therefore really evaluated, and body B really is the mixed path.

The old assertion is retained as **D-10b**, renamed to say what it actually proves —
run-to-run reproducibility — and explicitly marked as not discriminating for RN02.

The finding is recorded here rather than quietly fixed, because the overstated claim had
already been offered to the Operator in support of an acceptance recommendation.

## 4. Carried report-accuracy qualifications

Both nonblocking qualifications from the KEEP review are carried forward **accurately and
unchanged in substance**, per the mission's instruction not to optimize merely to erase a
report caveat.

**C2W-RN01 — mutation substitution is split, not individually equivalent.** The writer's 16
carried patch tuples are byte-identical to their predecessors. `W01-settle-every-body`
retains `!declares_decay` and therefore changes only non-decay bodies; it is **not**
individually equivalent to the removed unconditional `RES-settle-composed-body`.
`W01-settle-decay-body` supplies the explicit-decay double-charge error separately. Only
their **combined** coverage retains the old error and extends it to transform-only bodies.
This disposition repeats that precise mapping rather than the label, and counts no mutation
as killed by failure to patch or compile.

**C2W-RN02 — two pure walks, not one.** Writer report §7 claims composed settlement walks
lineage once per written target/scope per wave. In fact `group_intent` calls `settled` to
obtain its starting value and the unchanged `plan_wave` calls `settled` again for every
canonical target; the `RuleBody` reducer consumes the already-resolved transform value. This
is **repeated read-only computation**, not double billing and not an unreported commit. No
new scan class is introduced; the claimed walk count understates work. D-10 confirms there
is no observable canonical consequence. **Recorded as a disclosed performance limitation and
not optimized in this pass** — an unsolicited optimization of frozen, independently reviewed
code to make a report read better is the wrong trade.

## 5. Demonstrated blockers

**None.** No campaign, continuation or diagnostic in this disposition established a
reproducible blocking defect on the pinned build. Consequently no repair was made, and no
repair required independent verification. Had a blocker been found, the mission's discipline
would apply: a recorded decision, a bounded candidate, validation, evidence, separated
independent review.

## 6. Validation reuse and what was not rerun

The KEEP review's fresh validation ran with HEAD at exactly
`67b877192cc78b75c6fbe60c69b5594dc10befe8`: **427 workspace tests passed, 0 failed, 0
ignored**, with formatting, all-target/all-feature clippy with warnings denied, strict
core/engine lint, `cargo metadata`, whitespace checks and five static Windows/Android target
builds all passing.

**That evidence is reused unchanged, not reproduced.** The crate tree it certifies,
`7907f4d729104fd5dbfd4adad46e66cf09aa13dd`, is byte-identical to the tree every test in this
disposition ran against — verified by `git rev-parse HEAD:crates` on the handoff branch. The
mission directs reuse of unchanged evidence and forbids rerunning whole suites for their own
sake; rerunning 427 tests against a byte-identical tree would produce no new information at
real disk and time cost.

Nothing in this disposition modifies any file under `crates/`.

## 7. Gate C2 acceptance recommendation

**Recommendation: ACCEPT Gate C2 at `67b877192cc78b75c6fbe60c69b5594dc10befe8`, with the
four limitations of §7.2 recorded on the face of the acceptance.**

This is a recommendation to the Operator. **It is not an acceptance, and no Operator
acceptance of Gate C2 exists in this repository.** A search of the engineering records finds
the controlling statement to the contrary in
`SPARK_PHASE_2_GATE_C2_OPERATOR_NEXT_AUTHORIZED_STEP_2026-09-12.md` §4: "Acceptance remains
the Operator's decision, taken after the independent review and informed by whatever the
campaign finds." That campaign has now run, and this document is the "whatever it finds".

### 7.1 What supports the recommendation

1. An independent review of the exact candidate returned `KEEP` with C2W-01 closed and no
   new blocking code defect established.
2. 427 workspace tests, formatting, all-target clippy, strict lint, metadata and five static
   cross-target builds pass on that exact tree.
3. Four adversarial passes by three parties other than the candidate's writer found **zero**
   reproducible defects.
4. Every observation those passes recorded now has a **discriminating** disposition, not an
   argument from source reading — thirteen diagnostics, all passing. Twelve are
   discriminating in the strict sense: each would have come out differently had the defect
   explanation been the true one. The thirteenth, D-10b, is a determinism check and is
   labelled as one, not counted as discrimination. One earlier diagnostic failed this
   standard and was replaced after independent review (§3.2).
5. Three coverage gaps the campaigns declared open are now closed (D-6, D-7, D-8), and the
   continuations added ten further discriminating cases across pausing, epoch reset,
   fail-stop stickiness, semantic caps, work-key conflict and adversarial replay.

### 7.2 Limitations that must travel with any acceptance

1. **Adversarial time was roughly 32 of 60 granted minutes.** Four independent passes finding
   nothing is meaningful; it is not exhaustive, and no campaign claims fuzzing,
   property-based search, concurrency or performance coverage.
2. **C2W-RN01 and C2W-RN02 stand uncorrected by choice** (§4). RN02 in particular means the
   documented per-wave settlement walk count understates actual work.
3. **H-OBS-01 stands unrepaired** (§3.1): a `test-support` seam can mislead a future tester.
4. **Durability, crash recovery, G.A.M.E. integration, performance evidence and executable
   Windows/Android parity remain later gates.** Snapshot/restore evidence is in-memory
   round-trip evidence; it is not durable process-recovery proof, and the cross-platform
   evidence is compile-only.

### 7.3 What is unblocked, and what is not, either way

Independent of the acceptance decision, protocol §6 permits integration requirements and
test fixtures as working material. That work is complete and published alongside this
disposition:
`engineering/phase3/SPARK_GAME_INTEGRATION_CONTRACT_V1_2026-09-12.md` and
`engineering/phase3/spark_game_contract_prototype_2026-09-12/`.

Blocked until Gate C2 acceptance: freezing the integration contract (Gate C3), implementing
the production S.P.A.R.K. device surface, implementing the G.A.M.E. adapter (Gate C4), and
building the minimal persistent boundary that a restart-safe handoff needs.

## 8. Nonclaims

This disposition does not accept Gate C2, promote anything to production, merge or deploy
anything, change any G.A.M.E. file, open Phase 3, claim exhaustive adversarial coverage,
claim absence of defects, or claim cross-platform runtime parity.
