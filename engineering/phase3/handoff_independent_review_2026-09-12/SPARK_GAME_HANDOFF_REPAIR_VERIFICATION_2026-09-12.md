# Repair verification — SPARK–GAME safe handoff corrections

**Reviewer:** the same independent adversarial reviewer that produced
`INDEPENDENT_REVIEW.md` (verdict `HANDOFF_REVIEW_BOUNDED_REVISION_REQUIRED`; 0 BLOCKER,
1 MAJOR, 6 MINOR, 4 NOTE) against `d8ab37ed6489843380d27e381c716021a18b7fd9`.
Review only: nothing edited, committed, pushed; no existing worktree or branch touched.

**Date:** 2026-09-12.

## 1. Commit verified

**Assigned target:** `40147801e9ad091840cf9aa2eedf0a537ecf8a18`. Verified live:

```
$ git ls-remote --heads origin | grep spark-game-safe-handoff
40147801e9ad091840cf9aa2eedf0a537ecf8a18	refs/heads/candidate/spark-game-safe-handoff-20260912
exit 0
```

(at the time of the check; the branch later advanced to `83f883b…` — see §6.)

Commits under verification, beyond the reviewed `d8ab37e`:

```
4014780 Add a positive control to the replacement RN02 diagnostic
beebac3 Bring the status index up to date
747e44a Correct the contract after recovering prior GAME integration records
f0de9d5 Correct the handoff artifacts under independent review
```

**Isolation re-confirmed at the target:**

```
$ git diff --stat 67b877192cc78b75c6fbe60c69b5594dc10befe8 4014780 -- crates/
(no output)                                                    exit 0
$ git rev-parse 4014780:crates
7907f4d729104fd5dbfd4adad46e66cf09aa13dd     ← unchanged from the pinned candidate
$ cargo metadata --format-version 1 --offline   (repository root)
members: 3  — spark-core, spark-engine, spark-testkit            exit 0
```

**Disk:** `df -h /` before starting 7.5G free (94%); at the end 7.0G. My own cargo dir,
worktree and two probe crates were removed afterwards; nothing belonging to anyone else was
touched. The 3G floor was never approached.

## 2. Per-finding verification

| id | sev | status | evidence |
|---|---|---|---|
| **F-01** | MAJOR | **VERIFIED FIXED** | D-10 replaced; see §3. Every sentence I named is corrected: §3 preamble now says an earlier revision "did not meet that standard … §3.2 records what was wrong"; the RN02 row is rewritten around the two-path oracle comparison; §4's "D-10 confirms there is no observable canonical consequence" is gone; §7.1 item 4 now reads "thirteen diagnostics … Twelve are discriminating in the strict sense … The thirteenth, D-10b, is a determinism check and is labelled as one, not counted as discrimination"; the printed message states the oracle, the debt and the binding-clamp control. New **§3.2** records the failure openly, including "the overstated claim had already been offered to the Operator in support of an acceptance recommendation". |
| **F-02** | MINOR | **VERIFIED FIXED** | **Affiliation:** `ExternalAffiliationRef` is a separate namespace; **E-15** asserts `NotStable` on re-pointing, `NotInjective` on a second affiliation claiming a bound scope, idempotent identical re-bind, and namespace separation **in both directions** (`external_of(settlement).is_none()`, `affiliation_of(actor).is_none()`). **Defer:** **E-14** asserts the exact `Deferred { WorldBusy, not_before: None }`, wildlife 100 and hunts 0, and — the strong part — `executed_rations == 1`, proving deferral is per intent, not per batch. **Bounds:** `max_message_bytes` is genuinely **removed from the contract**, not just the README (`grep -rn max_message_bytes engineering/` returns only the response document describing the removal); §9.1 now declares two bounds and explains why no byte bound exists. **E-16** asserts `capacity_exceeded` **and** `batch.intents.len() == 2` — reported, never truncated. README's claim is narrowed to an enumeration of what is exercised, with the correction recorded. |
| **F-03** | MINOR | **VERIFIED FIXED** | The SONNET row now reads "**not stated by its artifacts**", with a paragraph distinguishing "written for the campaign" from "covering a vector the suite did not". |
| **F-04** | MINOR | **NOT FIXED** | Disposition §7.1 item 3 still reads verbatim: "Four adversarial passes by **three parties** other than the candidate's writer found **zero** reproducible defects." `git diff d8ab37e 4014780` shows that line untouched, and `grep -rn "three parties" engineering/` finds it still live at `SPARK_PHASE_2_GATE_C2_CAMPAIGN_DISPOSITION_2026-09-12.md:229`. The response document's claim that it was withdrawn is false — see **NF-01**. |
| **F-05** | MINOR | **VERIFIED FIXED — exceeds the finding** | See §4. |
| **F-06** | MINOR | **VERIFIED FIXED** | `assert!(errs.iter().any(\|e\| matches!(e, RuleSetError::ZeroPacingBudget)))` — the exact variant, replacing `!errs.is_empty()`. |
| **F-07** | MINOR | **VERIFIED FIXED** | Assertions inside `d1` renumbered "D-1 check 1…8"; the disposition states the convention once ("a bare `D-n` names a **test function** and a numbered 'check' inside one names an **assertion**") and the §3 row now cites "**D-1b** (the test `d1b_luna001_absent_cell_...`)" and "**D-1 check 6** (an assertion inside the test `d1_luna001_...`)". The collision is gone. |
| **F-08** | NOTE | **VERIFIED FIXED** | Renamed `e8_the_whole_proof_is_reproducible_across_two_in_process_runs`; message now ends "across two constructions IN ONE PROCESS. No cross-process and no cross-platform determinism is claimed by this test." |
| **F-09** | NOTE | **VERIFIED FIXED** | `assert_eq!(err, RestoreError::ArtifactBindingMismatch, …)` replaces the three-variant disjunction, with a comment that step 1 of restore validation is reached before the epoch chain or digest. |
| **F-10** | NOTE | **VERIFIED FIXED** | Disposition §2 now names both continuations as separate `claude-sonnet-5` agent sessions dispatched by the main engineer, and explicitly separates what is *attested* from what is *verifiable from the artifacts* (no `crates/` change, isolated `/tmp` workspaces, no corrective candidate) — which is the distinction the finding asked for. |
| **F-11** | NOTE | **VERIFIED FIXED** | `engineering/PHASE_STATUS.md` now records the KEEP, the campaign and, in bold, "**Gate C2 acceptance is the Operator's decision and has not been given.**" The stale "awaits a fresh independent review" is gone. |

**10 of 11 verified fixed; 1 not fixed (F-04).**

## 3. F-01 in detail, and the coordinator's question about the positive control

**The arithmetic checks out, independently.** `state.stress` seeded 100 at t=1, declared
baseline 0, decay 7 per cadence 3, read at t=19: 18 logical units ÷ cadence 3 = 6 whole
cadences; 6 × 7 = 42 debt; 100 − 42 = **58**. t=19 → t=20 is 1 unit, under one cadence, so
no further billing; 58 + 10 = **68**. I recomputed this by hand before running anything, and
then confirmed the engine agrees with my own probe (`/tmp/handoff-review-20260912/diag`):

```
BODY A (additive only, [Add(10)]):          value at t=19 = Some(58)  committed=Some(68)
BODY B (mixed, [Add(10), Clamp non-binding]): value at t=19 = Some(58)  committed=Some(68)
BODY C (mixed, [Add(10), Clamp binding 60]):  value at t=19 = Some(58)  committed=Some(60)
```

**Does the clamp really select a different path, and does it really change no value?** Yes,
and I verified it at the source rather than taking it:

- `engine.rs:951` — `let all_additive = ops.iter().all(|o| matches!(o.update, Update::Add(_) | Update::Subtract(_)))`. This is a **purely syntactic** test on the `Update` variants with no value-dependent short-circuit, so a non-binding clamp still makes the body non-all-additive. Body A returns `Intent::AddDelta`; body B cannot.
- Body B has two ops, so the single-op branch `if let [only] = ops` does not match either.
- Body B therefore reaches the composed reducer, where `engine.rs:1052` computes
  `start = self.settled(target, scope, ctx.now)?` when `is_additive_composition` (Add present, no Decay) — **this is exactly the RN02 second walk** — and `engine.rs:1067` folds the clamp.

So body B performs two `settled` walks where body A performs one, and both commit the oracle
value. That is a genuine discriminator: under explanation (a) the two-walk path would have
been billed 42 twice and landed below 68.

**The coordinator's question — is the control sufficient, or could body B take the additive
path while a binding clamp is applied afterwards? The control IS sufficient, and I can close
the residual gap by exhaustion rather than leave it open.** `Update::Clamp` is applied to a
runtime value at exactly **two** sites in the entire engine:

```
$ grep -rn "Update::Clamp|TransformFamily::Clamp" crates/spark-engine/src/
rules.rs:395   canonical encoding (hashing) — not value application
rules.rs:418   static family classification
rules.rs:430   static family classification
rules.rs:471   tag string
rules.rs:1259  static validation (min > max => IncoherentClamp)
engine.rs:987  single-op Transform branch      <- unreachable for a 2-op body
engine.rs:1067 composed reducer fold           <- downstream of settled() at 1052
```

Nothing in the reduce or commit path applies a clamp, and the declared `ValueConstraint`
**refuses** rather than clamps (proved independently by D-1). So a committed value of exactly
60 for a two-op body can only have been produced at `engine.rs:1067`, which lies inside the
branch that called `settled()`. The hypothetical world — additive path plus a downstream
clamp — does not exist at this tree. The control closes the hole it was built for.

**One honest qualification, recorded as a NOTE rather than a defect.** The *path-selection*
fact is not observable through the report surface. I checked: `CommittedEffect` exposes
`write_path` (only `SparkEffect` / `CommitDerived`) and `Provenance` (retained source
digests), and my probe found both bodies report `retained_sources=1, write_path=spark_effect`
— identical. So the test cannot assert the path directly; it establishes it by the control
plus the source facts above. That is inherent to the hypothesis: RN02 asserts the two paths
produce the *same* value, so no value-based observation can separate them, and the pinned
tree exposes no walk counter. This is a limit of what any test could do here, not a defect in
this one.

**D-10b is honestly labelled.** Renamed `d10b_rn02_the_composed_result_is_also_reproducible_run_to_run`,
commented "This is a DETERMINISM check, not the RN02 discriminator", and printing
"run-to-run reproducibility only — this does not discriminate RN02". §7.1 item 4 excludes it
from the discrimination count. Correct.

## 4. F-05 in detail

The correction substantially **exceeds** what my finding asked for, and it found a record my
own search missed. My F-05 search covered `outstanding request|Busy|pull-results`; it did not
grep for `BehaviorIntent` or required-capability language, and so I did not surface
`GAME_PRODUCT_DESIGN_FOUNDATION_V1.md`. I verified the new citation against G.A.M.E.
`origin/main` (`a73fbac…`, still live) and it is **exact**:

```
$ git show origin/main:project_records/foundation/GAME_PRODUCT_DESIGN_FOUNDATION_V1.md
line 3:  Status: CANONICAL — OPERATOR-APPROVED PRODUCT/DESIGN DIRECTION — NON-CONSTITUTIONAL
§11.3:   **SETTLED DIRECTION:** … "A S.P.A.R.K. `BehaviorIntent` or gameplay-hook candidate
         is never an executable G.A.M.E. command. G.A.M.E. validates feasibility and may
         execute, reject, defer, or translate it."
         **REQUIRED CAPABILITY:** stable entity identities, monotonic authoritative logical
         time, typed/scoped host observations, actor/affiliation context, advisory outputs,
         and explicit host-confirmed outcomes. Exact transport, serialization, process
         boundary, and embedded-versus-service deployment remain open.
```

Contract §1.2.1's status line, the quoted sentence, and the six-capability mapping table are
all faithful. `BehaviorIntent` is indeed G.A.M.E.'s own vocabulary, recovered not invented.

**§1.2.2 fairly represents the readiness closeout.** It quotes the "working research;
non-operative" classification, "neither side's proposal is adopted by this record", and the
exact "bounded pull/`Busy`/ack result flow … remain proposals" line I cited. It then states
the corrected provenance explicitly: "the accurate provenance statement is **not** 'G.A.M.E.
has no record of this design'. It is: G.A.M.E. has a current record that names the design and
classifies it as an unadopted proposal". §4.4 and the §11.2 row now read "named but expressly
unadopted". That is precisely the correction F-05 asked for.

**The retention-until-acknowledgment divergence I specifically named is genuinely recorded**,
at §9.3, as its own bolded paragraph: "**Device-side retention is deliberately absent from V1,
and this is a divergence worth naming.** The prior research design (§1.2.3) has the device
*retain* an unacknowledged report and redeliver it, which would close row 3 above. V1 has no
retention because the pinned engine has no durable state to retain it in: a retained report
held only in memory is lost by exactly the failure it exists to survive. Retention is
therefore recorded as a **candidate resolution of the §9.4 durability gap**." §1.2.2 also
tabulates four divergences from the G.A.M.E.-side proposal. Fully closed.

## 5. New material: SF-01, SF-04, SF-05, and the self-correction report

### SF-01 — the fix is correct and complete, as far as I can attack it

The defect is real and was serious: at `d8ab37e` the device projected from
`self.project(result.reports())` on the completing call alone, so a paced request lost every
intent committed before its last pause, silently. The fix accumulates per active request,
keyed by correlation. I checked all six branches of `interpret`:

| outcome | behavior | correct? |
|---|---|---|
| `Paused` | `accumulate(correlation, reports)` | ✓ |
| `RefusedHorizonBehindFrontier` | `discard_if_mine(correlation)` | ✓ — guarded by correlation, so it cannot touch another request's accumulation |
| `RefusedActiveRequestMismatch` (**Busy**) | **neither accumulate nor discard** | ✓ — the interloper cannot disturb the active accumulation; the comment says exactly this |
| `FinalizationEntailmentViolated` / `StoreInvariantViolated` | `discard_if_mine` → `FailStopped` | ✓ — no stable boundary, so nothing may ever be exported |
| `CompletedCommandNotFinalized` | accumulate → take → project → publish with the typed `refusal` carried | ✓ — see §6 for my check that the boundary really is stable |
| `Completed` | accumulate → take → project → publish | ✓ |

`take_accumulated()` clears both the vector and `accumulated_for`, so a terminal outcome
cannot leave residue for a later batch.

**Regression tests are strong.** E-12 asserts `pauses >= 1` (so the probe genuinely exercises
pacing), exactly **2** intents — which catches duplication as well as loss — and their
canonical times `[5, 6]` in order. E-6e asserts both slices survive an interleaved `Busy`.
E-11 now asserts `batch.intents.is_empty()` on a re-presented completed command, closing the
double-publish path the new `CompletedWithRefusedCommand` variant would otherwise open.

**My own adversarial probe** (`/tmp/handoff-review-20260912/probe2/tests/attack.rs`), harder
than the shipped tests — three due times, budget 1, and on *every* pause both a `Busy`
interloper and a stale-horizon request injected:

```
pauses=2 busy_interleaves=2
intents=3 canonical_times=[5, 6, 7]
re-present -> Completed with empty batch
ATTACK RESULT: no loss, no duplication, no double publish
exit 0
```

**I found no remaining path where an intent can be lost or double-published.**

### SF-04 — sound

The at-most-once key is now `(activation_hash, CorrelationId, batch_digest)`. **E-13** is a
complete test of the change: the batch carries the session activation; two batches differing
*only* in activation produce different keys; a ledger that applied one returns `Apply` (not
`AlreadyApplied`) for the other; and re-admitting the original still returns
`AlreadyApplied`. Contract §7.2 and the §11.2 row are updated consistently.

### SF-05 — sound

`max_intents_per_batch` is live as an outbound **capacity hint**: an oversized batch is
delivered whole with `capacity_exceeded` set, never truncated, because the work behind it is
already committed. E-16 asserts both halves. This is the right shape — truncating committed
advisory output would lose work the engine had already done.

### The self-correction report is accurate and does not understate

`SPARK_GAME_CONTRACT_V1_SELF_CORRECTION_REPORT_2026-09-12.md` describes SF-01 as a **BLOCKING
DEFECT** and calls the failure shape "the worst possible … for an advisory seam" — which is
fair, not softened. Its technical account matches the code I read at `d8ab37e`: the old
`interpret` did project from `result.reports()` alone, so its claim that E-12 "fails on the
pre-correction code" is correct by inspection. It names all three edge cases (Busy,
fail-stop, unfinalized command) and all three are implemented as described. It opens by
stating that "self-review never satisfies independence". I found nothing understated.

## 6. The delta beyond the assigned target: `4014780 → 83f883b`

Checked as instructed, and **the coordinator's assertion holds: the delta is genuinely
non-substantive.**

```
$ git diff --stat 4014780 83f883b -- engineering/phase3/spark_game_contract_prototype_2026-09-12/ \
      engineering/phase2/gate_c2_campaign_disposition_evidence_2026-09-12/ crates/
(no output)                                                                   exit 0
$ git diff --stat 4014780 83f883b
 engineering/PHASE_STATUS.md                                 |  9 ++++++++-
 engineering/phase3/SPARK_GAME_INTEGRATION_CONTRACT_V1_...md | 11 +++++++++--
 2 files changed, 17 insertions(+), 3 deletions(-)
```

Zero change to prototype code, tests, diagnostics, evidence or `crates/`, so the test results
in §7 carry forward to `83f883b` unchanged. `PHASE_STATUS.md` corrects a stale prototype test
count (14 → 17, matching my own measured 17) and adds a reference to the independent review.
Contract §7.2 gains only a clause naming `CorrelationId` where a sentence had run into the
digest block.

**Does the new §9.5 paragraph accurately describe what the code does? On substance, yes —
and I verified the load-bearing part rather than reading it.** The paragraph says a completed
horizon whose command was not finalized still publishes its committed cohorts, with the typed
refusal carried "rather than flattened into a boolean or discarded with the batch". That is
exactly what `interpret`'s `CompletedCommandNotFinalized` arm does: it publishes an ordinary
`IntentBatch` in a `CompletedWithRefusedCommand` response carrying the engine's typed
`refusal` as its own field.

The claim worth testing was the *premise* — that such a boundary is real committed work at
all. My probe (`probe2/tests/attack2.rs`):

```
after finalized command: frontier=LogicalTime(10) snapshot_publishable=true
CompletedWithRefusedCommand: intents=0 refusals=0 refusal=CommandIdentityConflict { command_id: CommandId(cmd.tick.1) }
after refused command:   frontier=LogicalTime(10) snapshot_publishable=true
exit 0
```

The frontier is intact and a stable-boundary snapshot is still publishable after the refusal,
so calling the boundary's cohorts committed work is sound. The paragraph does **not**
overstate the code.

One precision point, recorded as **NF-02** below rather than waved through: the closing
sentence "Only the two sticky fail-stops publish nothing" is true within the paragraph's
subject (outcomes at a completed horizon) but false read across all outcomes — `Paused`,
`Busy`, `StaleHorizon` and an oversized-message rejection also publish nothing. This
understates how many paths publish nothing; it does not overstate the code.

## 7. Commands run, exit codes, real output

| # | command | exit | result |
|---|---|---|---|
| 1 | `git ls-remote --heads origin \| grep spark-game-safe-handoff` | 0 | `40147801e9ad091840cf9aa2eedf0a537ecf8a18` (later `83f883b…`) |
| 2 | `df -h /` before / after | 0 | 7.5G free (94%) → 7.0G |
| 3 | `git worktree add --detach /tmp/handoff-review-20260912/wt3 4014780` | 0 | detached HEAD at 4014780 |
| 4 | `git diff --stat 67b87719… 4014780 -- crates/` | 0 | **no output** |
| 5 | `git rev-parse 4014780:crates` | 0 | `7907f4d729104fd5dbfd4adad46e66cf09aa13dd` |
| 6 | `cargo metadata --format-version 1 --offline` (root) | 0 | **3** members |
| 7 | `cargo test --offline` (prototype) | 0 | **17 passed; 0 failed; 0 ignored** — e1, e2, e3, e3b, e4, e5, e6, e7, e8, e9, e10, e11, **e12, e13, e14, e15, e16** |
| 8 | `cargo test --offline -- --test-threads=1 --nocapture` (diagnostics harness, rebuilt by me against my own worktree) | 0 | **13 passed; 0 failed; 0 ignored**, incl. `d10_rn02_the_repeated_settlement_walk_has_no_canonical_effect` printing "both commit 68 against a hand-computed oracle, with 42 of decay debt already billed once; a binding clamp at 60 does change the result" |
| 9 | reviewer probe `probe_paths_differ_observably` | 0 | A=68, B(non-binding)=68, C(binding 60)=60; both report `retained_sources=1 write_path=spark_effect` |
| 10 | reviewer probe `attack_three_slices_with_interleaved_busy_and_stale` | 0 | `pauses=2 busy_interleaves=2`; `intents=3 canonical_times=[5, 6, 7]`; re-present → empty batch |
| 11 | reviewer probe `unfinalized_command_boundary_is_still_a_stable_boundary` | 0 | frontier `LogicalTime(10)`, `snapshot_publishable=true` before and after the refusal |
| 12 | `grep -rn "three parties" engineering/` | 0 | still live at `SPARK_PHASE_2_GATE_C2_CAMPAIGN_DISPOSITION_2026-09-12.md:229` → **F-04 not fixed** |
| 13 | `grep -rn "max_message_bytes" engineering/` | 0 | only the response document describing its removal → genuinely removed from the contract |
| 14 | `grep -rn "Update::Clamp\|TransformFamily::Clamp" crates/spark-engine/src/` | 0 | 7 hits; only `engine.rs:987` and `engine.rs:1067` apply a clamp to a value |
| 15 | `git show origin/main:…/GAME_PRODUCT_DESIGN_FOUNDATION_V1.md` (G.A.M.E.) | 0 | status and §11.3 exactly as the contract cites |
| 16 | `git diff --stat 4014780 83f883b -- <prototype> <diagnostics> crates/` | 0 | **no output** — delta is documentation-only |

Files I wrote: `INDEPENDENT_REVIEW.md` and this file, both under
`/tmp/handoff-review-20260912/`. Worktree, cargo dir and both probe crates removed.

## 8. New findings raised against the corrections

| id | severity | what is wrong |
|---|---|---|
| **NF-01** | **MAJOR** | **The review-response document reports a correction that was never made.** `SPARK_GAME_HANDOFF_REVIEW_RESPONSE_2026-09-12.md` disposition table, F-04 row, states: "**Accepted and corrected.** 'three parties' is withdrawn. §7.1 item 3 now claims only what the artifacts establish: four passes, none run by the candidate's writer, under collection-only missions, with no count of distinct parties asserted." No such edit exists. `SPARK_PHASE_2_GATE_C2_CAMPAIGN_DISPOSITION_2026-09-12.md:229` still reads "Four adversarial passes by **three parties** other than the candidate's writer". So two things stand: the unsubstantiated party count (original F-04), and a false statement about the corrections in the document whose purpose is to let a reviewer navigate them. The response document does say its corrections are "unverified by any party other than their author until the reviewer verifies the repairs", which is the right posture and is why this was caught — but the disposition is the document that reaches the Operator, and a reader trusting that row is misled. **Required:** apply the edit the response describes, or correct the response row. |
| **NF-02** | MINOR | Contract §9.5, new paragraph, closing sentence: "Only the two sticky fail-stops publish nothing, because they publish no stable boundary at all." True within the paragraph's subject (outcomes at a completed horizon), false across all outcomes — `Paused`, `Rejected::Busy`, `Rejected::StaleHorizon` and `Rejected::MessageTooLarge` also publish nothing, and the stated reason ("publish no stable boundary") applies to them too. This understates rather than overstates the code, so it is a precision defect, not a false capability claim. **Required:** scope the sentence, e.g. "of the outcomes that complete a horizon, only the two sticky fail-stops publish nothing". |

## 9. Verdict

The repair pass is, with one exception, genuinely good work rather than cosmetic compliance.
The MAJOR was not merely patched: the replacement D-10 discriminates by running two different
reduction paths on identical state against a hand-computed oracle, it carries a positive
control against the exact hole the first replacement left, the superseded assertion is kept
under an honest name and explicitly excluded from the discrimination count, and §3.2 records
the failure in the document that goes to the Operator instead of quietly substituting a better
test. F-05 was closed more thoroughly than I asked, recovering a CANONICAL Operator-approved
G.A.M.E. record my own search had missed and tabulating four divergences from it. SF-01 — a
real blocking defect, self-found — is correctly and completely fixed, and survived a harder
probe than the shipped regression tests. `crates/` is still untouched, the workspace still has
three members, and the diagnostics and prototype suites pass at 13 and 17.

But one finding of eleven was not fixed, and the document that disposes the findings states in
detail that it was. That is the one failure mode a repair-verification pass exists to catch,
and I will not sign it off silently: the correction is one sentence, the claim that it was made
is already published, and the two must be reconciled before these artifacts are relied on.

**Verdict: `HANDOFF_REPAIRS_INCOMPLETE`**

Not foundational — nothing structural is wrong, no gate is accepted, no reserved choice is
frozen, and ten of eleven findings are properly closed. Completing this pass requires exactly
two edits: withdraw "three parties" from disposition §7.1 item 3 (or correct the response row
that says it already was), and scope the §9.5 sentence in NF-02. Neither requires new evidence
or a new test.
