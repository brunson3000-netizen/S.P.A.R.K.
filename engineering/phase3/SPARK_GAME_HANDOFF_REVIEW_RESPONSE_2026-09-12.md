# Independent review of the safe-handoff artifacts — findings and their disposition

**Date:** 2026-09-12. **Author:** the local main engineer (Claude Code, Opus 5), the writer
of the reviewed material. **This is the writer's response to a review, not a review.**

**Reviewed commit:** `d8ab37ed6489843380d27e381c716021a18b7fd9`.
**Reviewer:** an independent adversarial reviewer (a separate Claude Code, Opus 5 session)
that authored none of the material, under a bounded review assignment with no implementation
authority, working in its own detached worktree and modifying nothing.
**Verdict:** `HANDOFF_REVIEW_BOUNDED_REVISION_REQUIRED` — 0 BLOCKER, 1 MAJOR, 6 MINOR,
4 NOTE.
**Full review, preserved verbatim:**
`handoff_independent_review_2026-09-12/SPARK_GAME_HANDOFF_INDEPENDENT_REVIEW_2026-09-12.md`.
**Repair verification, preserved verbatim:**
`handoff_independent_review_2026-09-12/SPARK_GAME_HANDOFF_REPAIR_VERIFICATION_2026-09-12.md`,
verdict `HANDOFF_REPAIRS_INCOMPLETE` — 10 of 11 findings verified fixed, one (F-04) not, plus
two new findings NF-01 (MAJOR) and NF-02 (MINOR). Both new findings are dispositioned in §5.

The review independently reproduced and confirmed, among other things: the exact commit
against live GitHub; that `crates/` is untouched and both crate trees are
`7907f4d729104fd5dbfd4adad46e66cf09aa13dd`; exactly three workspace members; the prototype's
test, fmt and clippy results; every evidence checksum; the whole elapsed-time and coverage
accounting against both campaign branches; that C2W-RN01 and C2W-RN02 are carried forward
with nothing softened; that the contract covers every Gate C3 surface, freezes no reserved
choice and does not breach the authority boundary; that the durability gap statement is
accurate (indeed understated — `EngineSnapshot`'s fields are `pub(crate)` with no `serde` at
all, so an external consumer cannot serialize it by hand); and, searching every ref by both
filename and content, that **no Operator acceptance of Gate C2 exists anywhere in the
repository**.

## 0. A correction to this document, raised by the repair-verification pass

**This document previously reported a correction that had not been made.** Its F-04 row
stated in detail that "three parties" had been withdrawn from the disposition and described
the replacement text. The edit had silently failed to apply — it was written against an
anchor carrying emphasis markers the target line did not have — and nobody noticed, because
the row was written from intent rather than from the file.

The repair-verification pass caught it, by the direct method of `grep -rn "three parties"`,
and raised it as **NF-01 (MAJOR)**: two things stood, the original unsubstantiated party
count and a false statement in the very document meant to let a reviewer navigate the
corrections.

Both are now fixed, and each fix was asserted to have landed before being committed rather
than assumed. The failure is recorded here rather than quietly overwritten, for the same
reason §1 records the D-10 failure: the disposition is what reaches the Operator, and a
reader who trusted that row was misled.

**The standing lesson, recorded against this document rather than against the reviewer:**
a claim that an edit was made is a claim about a file, and it must be checked against the
file. Every edit in the second correction pass is now verified present by an assertion that
fails loudly if the anchor does not match.

## Disposition of every finding

| id | severity | disposition |
|---|---|---|
| **F-01** | MAJOR | **Accepted and corrected.** See §1. |
| F-02 | MINOR | **Accepted and corrected.** See §2. |
| F-03 | MINOR | **Accepted and corrected.** The SONNET "30 novel" figure is withdrawn; the table now reads "not stated by its artifacts", with a sentence explaining that "written for the campaign" and "covering a vector the suite did not" are different claims. |
| F-04 | MINOR | **Accepted, but this row was published before the edit had actually landed — see §0.** The edit is now applied and verified present: §7.1 item 3 claims only what the artifacts establish, with no count of distinct parties. |
| F-05 | MINOR | **Already corrected before the review returned**, by the writer's own recovery of the same records, in commit `747e44a` (new contract §1.2.2, §4.4 and §11.2). The retention-until-acknowledgment divergence the reviewer specifically named is recorded at §9.3 as a divergence and a candidate resolution of the §9.4 durability gap. See §3. |
| F-06 | MINOR | **Accepted and corrected.** D-3's zero-budget half now asserts the exact `RuleSetError::ZeroPacingBudget` variant instead of a non-empty error list. |
| F-07 | MINOR | **Accepted and corrected.** Assertion labels inside `d1` are renumbered "D-1 check *n*", and the disposition states once that a bare `D-n` names a test function while a "check" names an assertion inside one. The §3 row now cites both precisely. |
| F-08 | NOTE | **Accepted and corrected.** `e8_…_deterministic_across_independent_runs` is renamed `e8_the_whole_proof_is_reproducible_across_two_in_process_runs`, and its message states that no cross-process or cross-platform determinism is claimed. |
| F-09 | NOTE | **Accepted and corrected.** D-6 now asserts exactly `RestoreError::ArtifactBindingMismatch` rather than a three-variant disjunction, with a comment noting that step 1 of restore validation is reached before the epoch chain or the digest. |
| F-10 | NOTE | **Accepted and recorded.** Disposition §2 now names the continuation testers as separate Claude Sonnet 5 (`claude-sonnet-5`) agent sessions dispatched by the main engineer under collection-only missions, and distinguishes what is *attested* from what is *verifiable* from the artifacts (no `crates/` change, isolated `/tmp` workspaces, no corrective candidate). |
| F-11 | NOTE | **Already corrected before the review returned**, in commit `beebac3`, which brings `engineering/PHASE_STATUS.md` up to the KEEP and the campaign and states that Gate C2 acceptance has not been given. |

## 1. F-01 (MAJOR) — the diagnostic that did not discriminate

**The finding is correct, and it is the kind that matters most**: a claim offered to the
Operator in support of an acceptance recommendation, whose own evidence did not carry it.

The original D-10 ran one composed-body history through two freshly constructed engines in
the same process and asserted they agreed. That is reproducibility. Had the repeated
settlement walk perturbed the committed result, **both** runs would have been perturbed
identically and the test would still have passed — while printing "RN02 repeated settlement
has no observable canonical effect".

**The replacement discriminates by exercising two different code paths on identical state,
against an oracle computed from the declared semantics rather than from the engine:**

- `state.stress` seeded to 100 at t=1, declared baseline 0, decay 7 per cadence of 3, left
  to t=19. Eighteen units is six whole cadences, so 6 × 7 = 42 of debt. Oracle: **58**. Both
  engines hold 58.
- At t=20: body **A** `[Add(10)]` takes the all-additive `AddDelta` reduction path; body
  **B** `[Add(10), Clamp(-WIDE, +WIDE)]` has a clamp that binds nothing but makes the body
  mixed, selecting the settled path with its repeated `settled` call. Fewer than one cadence
  elapses, so the oracle says both commit **68**.
- Both do. A walk that billed debt a second time would have driven the settled path below
  68 while leaving the additive path at 68.

The old assertion is kept as **D-10b**, renamed to say what it proves — run-to-run
reproducibility — and explicitly marked as *not* discriminating for RN02. The diagnostics
now number 13, of which 12 are discriminating in the strict sense and one (D-10b) is
labelled a determinism check and not counted as discrimination.

Every sentence the reviewer identified as overstated is corrected: the §3 preamble, the
RN02 row, the §4 sentence, §7.1 item 4, and the printed message. Disposition §3.2 records
what was wrong and what replaced it, rather than quietly substituting a better test.

## 2. F-02 (MINOR) — three Gate C3 surfaces the prototype did not exercise

The reviewer found the README's "supports every logical surface Gate C3 enumerates" claim
unsupported in three places. All three are resolved, two by building the missing coverage
and one by removing a surface that should not have been declared:

- **Affiliation identity.** Gate C3 enumerates "stable external entity **and affiliation**
  references" as two surfaces. The prototype had no affiliation concept. It now has
  `ExternalAffiliationRef` as a **separate namespace** from `ExternalEntityRef`, with the
  same injectivity and stability checks; `IntentSubject` distinguishes an entity-addressed
  from an affiliation-addressed intent; the fixture binds actors and households as entities
  and settlements, regions and watersheds as affiliations, and carries a second intent
  channel, `intent.ration`, declared at settlement scope. **E-15** proves injectivity,
  stability, idempotent re-binding and namespace separation; **E-2** now asserts that the
  first proof produces one entity-addressed and one affiliation-addressed intent, and the
  fake host dispatches on channel so that rationing moves food supply and hunting moves
  wildlife — a host that treated every channel alike would be executing an action nobody
  advised.
- **The `Deferred` disposition.** `DeferReason` existed and no test produced one. **E-14**
  now marks an actor busy and asserts `Deferred { WorldBusy }`, that the world does not
  move, and that deferral is **per intent, not per batch** — the affiliation-addressed
  intent in the same batch still executes.
- **Declared bounds.** `max_intents_per_batch` was a dead field. It is now live as the
  outbound **capacity hint** of contract §9.1: an oversized batch is delivered whole with
  `capacity_exceeded` set, never truncated, because the work behind it is already committed.
  **E-16** proves it. `max_message_bytes` is **removed from the contract** rather than
  implemented: the seam selects no wire format, so a byte count would bound something the
  contract does not define. A transport that adds one declares its own.

The README's claim is narrowed to the surfaces actually exercised, and the correction is
recorded there rather than silently absorbed.

## 3. F-05 and F-11 — findings already closed before the review returned

The reviewer's addendum correctly observed that the local branch carried two commits beyond
the reviewed one and declined to inspect them, since its assignment named `d8ab37e`. That
was the right call. For the record, and verifiable at the published branch:

- `747e44a` recovers and cites `GAME_PRODUCT_DESIGN_FOUNDATION_V1.md` §11.3 (CANONICAL,
  Operator-approved) and `GAME_IMPLEMENTATION_READINESS_CLOSEOUT_20260909.md`, corrects the
  §4.4 provenance statement from "no record adopting" to "named but expressly unadopted",
  tabulates the four divergences from the G.A.M.E.-side proposal, and records the
  retention-until-acknowledgment divergence the reviewer named. It also fixes a blocking
  prototype defect the writer found during that recovery and the reviewer did not see: the
  device dropped every intent committed before a pause in a paced request.
- `beebac3` brings `engineering/PHASE_STATUS.md` up to date.

Both were pushed as they were made; the reviewer's "not published to GitHub" observation
reflects the ref listing it took at the start of its run.

**This does not make those findings self-reviewed into closure.** They, and every correction
in §1 and §2, are unverified by any party other than their author until the reviewer
verifies the repairs under the same assignment.

## 4. Validation after the corrections

| Check | Result |
|---|---|
| diagnostics harness | **13 passed, 0 failed, 0 ignored** (was 12; D-10 replaced, D-10b added) |
| prototype `cargo test --offline` | **17 passed, 0 failed, 0 ignored** (was 12 at the reviewed commit) |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --offline --all-targets -- -D warnings` | PASS |
| five Windows/Android `cargo check` targets | PASS — **COMPILE-ONLY** |
| files under `crates/` changed | **none** |

## 5. The repair-verification pass and its two new findings

The reviewer verified the corrections at `40147801e9ad091840cf9aa2eedf0a537ecf8a18` under the
same assignment, re-running everything itself: diagnostics **13 passed**, prototype
**17 passed**, `crates/` untouched at tree `7907f4d729104fd5dbfd4adad46e66cf09aa13dd`,
workspace still exactly three members. It confirmed **10 of 11** findings fixed, and went
further than verification in three places worth recording:

- it **independently recomputed** the D-10 oracle by hand before running anything (58 at
  t=19, 68 after the body), then reproduced A=68, B=68, C=60 with its own probe;
- it **closed by exhaustion** the residual question the writer could not settle from the
  author's position — whether body B could take the additive path with a clamp applied
  downstream. `Update::Clamp` reaches a runtime value at exactly two sites in the engine, one
  unreachable for a two-operation body and the other downstream of the `settled()` call, and
  the declared `ValueConstraint` refuses rather than clamps. The hypothetical path does not
  exist at this tree;
- it **attacked SF-01 harder than the shipped regression tests** — three due times, budget 1,
  and on every pause both a `Busy` interloper and a stale-horizon request — and found exactly
  three intents at canonical times `[5, 6, 7]`, no loss, no duplication, no double publish.

It also recorded one honest limit the writer should not paper over: **path selection is not
observable through the report surface.** `write_path` and `Provenance` are identical for both
bodies, so D-10 establishes the path by its positive control plus source facts rather than by
direct assertion. That is inherent to a hypothesis asserting the two paths agree — no
value-based observation can separate them, and the pinned tree exposes no walk counter.

### NF-01 (MAJOR) — a reported correction that had not been made

**Accepted in full.** See §0. The F-04 edit is applied and its presence asserted; this
document's false row is corrected and the failure recorded rather than overwritten.

### NF-02 (MINOR) — an under-scoped sentence in contract §9.5

**Accepted and corrected.** The sentence "Only the two sticky fail-stops publish nothing" was
true within its paragraph's subject — outcomes at a completed horizon — and false read across
all outcomes, since `Paused`, `Rejected::Busy`, `Rejected::StaleHorizon` and
`Rejected::MessageTooLarge` also publish nothing, for the same reason. It is now scoped to the
outcomes that complete a horizon and names the other unpublished cases explicitly. The
reviewer classified this as understating rather than overstating the code, and separately
verified by probe that the paragraph's load-bearing premise holds: after an unfinalized
command the frontier is intact and a stable-boundary snapshot is still publishable, so calling
that boundary's cohorts committed work is sound.

## 6. What the review did not change

No gate is accepted. No reserved choice is frozen. The contract remains a candidate, Gate C3
is not frozen, Gate C4 is not begun, no G.A.M.E. file is modified, and production is neither
merged nor deployed. The Gate C2 acceptance recommendation stands, with its five recorded
limitations and now two further facts on its face: one of the diagnostics supporting it had to
be replaced after independent review, and this response document itself had to be corrected
after reporting a fix that had not been made.
