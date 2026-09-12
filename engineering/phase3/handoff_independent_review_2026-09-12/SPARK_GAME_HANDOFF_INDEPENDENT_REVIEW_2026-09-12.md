# Independent adversarial review — SPARK–GAME safe handoff artifacts

**Reviewer:** independent adversarial reviewer (Claude Code, Opus 5, `claude-opus-5`).
Not the author of any material under review. Review only: no file in any repository was
edited, no commit, no push, no candidate opened, no existing worktree or branch touched.

**Date:** 2026-09-12.

## 1. Exact commit reviewed and live-GitHub verification

**Commit under review:** `d8ab37ed6489843380d27e381c716021a18b7fd9`
("Dispose the Gate C2 campaign and prepare the SPARK-GAME contract").

```
$ git -C /home/chromikey/Projects/SPARK ls-remote --heads origin
exit 0
```

Observed, verbatim, among the returned refs:

```
d8ab37ed6489843380d27e381c716021a18b7fd9	refs/heads/candidate/spark-game-safe-handoff-20260912
67b877192cc78b75c6fbe60c69b5594dc10befe8	refs/heads/candidate/phase2-gate-c2-w01-correction-20260912
21a4fec666ca493f6ac4d5194ec6e9c5380ce40c	refs/heads/review/phase2-gate-c2-w01-correction-independent-20260912
dfabd058e3d9fc820e8296a36380d93759cd7443	refs/heads/campaign/luna-adversarial-20260912
827a135346916a226aa8d24d7bc419effe1e3deb	refs/heads/adversarial/sonnet-campaign-20260912
```

The commit under review is exactly the live head of
`candidate/spark-game-safe-handoff-20260912` on `origin`. All four pinned predecessor
commits the artifacts rely on are live on `origin` at the hashes the artifacts name.

**G.A.M.E. live verification:**

```
$ git -C /home/chromikey/Projects/GAME_PROJECT ls-remote --heads origin main
a73fbac743adc1a00679ebc63b91027386718d71	refs/heads/main
exit 0
```

This is exactly the G.A.M.E. pin recorded in the contract's §0 table. Both copies of the
convergence protocol hash identically:

```
$ git -C .../GAME_PROJECT show origin/main:project_records/foundation/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md | sha256sum
611f21ff5e39575f414e245b8409a2b6f7811a5f2e164bd81a3c69e7cbf0a06a  -
$ sha256sum <worktree>/engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md
611f21ff5e39575f414e245b8409a2b6f7811a5f2e164bd81a3c69e7cbf0a06a
```

The contract's §0 pin sha256 `611f21ff…a06a` and its "byte-identical" claim are correct.

**Checkout used:** my own detached worktree at `/tmp/handoff-review-20260912/wt`, created
with `git worktree add --detach`, removed at the end of this review. Cargo target dir
`/tmp/handoff-review-20260912/cargo`, also removed. `df -h /` before building: **9.2G
free (92% used)**; after all builds: **7.6G free (94%)**. My own cargo dir peaked at
**112M**; the remaining decrease was not mine and I deleted nothing belonging to anyone
else. Free space never approached the 3G floor.

## 2. Findings

| id | severity | artifact and location | what is wrong | what the controlling authority requires |
|---|---|---|---|---|
| **F-01** | **MAJOR** | Disposition §3 preamble, §3 row "C2W-RN02", §4 final sentence, §7.1 item 4; evidence `diagnostics/diagnostics.rs` `d10_rn02_repeated_settlement_is_observably_idempotent` (lines 564–622) | **D-10 is not a discriminating test, but is presented as one three times.** D-10 runs the identical composed-body history through two freshly constructed engines *in the same process* and asserts the committed value and engine-state digest agree. That establishes reproducibility, not idempotence of the repeated settlement walk: if the second walk *did* perturb the committed result, both runs would perturb it identically and D-10 would still pass. The test does not even pin the semantically expected value (`clamp(0+100, 0, 60) = 60`); it only compares run 1 against run 2. Yet §3 states the standard as "a **discriminating test** that would come out differently if the competing explanation were the true one", tabulates D-10 against explanations "(a) repeated computation changes the committed result" vs "(b) read-only and observably idempotent" and concludes "(b)"; §4 asserts "**D-10 confirms there is no observable canonical consequence**"; and §7.1 item 4 advances "twelve diagnostics … **each of which would have come out differently had the defect explanation been the true one**" as a pillar of the Gate C2 acceptance recommendation. The printed message likewise reads "D-10 PASS: RN02 repeated settlement has no observable canonical effect" while asserting something strictly weaker. | Mission §1: "Source implementation alone is not proof of intended behavior"; and the disposition's own §3/§7.1 discrimination standard, which §7.1 offers to the Operator as support for acceptance. Protocol §4: "No state name is a substitute for its evidence." |
| **F-02** | MINOR | `spark_game_contract_prototype_2026-09-12/README.md` §"What it does and does not prove"; contract §4.1, §8, §9.1 | README claims the prototype demonstrates the S.P.A.R.K. surface "supports **every logical surface Gate C3 enumerates**". Three enumerated elements are not demonstrated: (a) **affiliation identity** — Gate C3 bullet 3 is "stable external entity **and affiliation** references" and contract §4.1 defines `ExternalAffiliationRef`, but the prototype contains no affiliation concept at all (`grep -rn -i affiliation src/ tests/` returns nothing); (b) **defer** — Gate C3 bullet 7 is "rejection/**defer** reasons"; `DeferReason::WorldBusy` exists in `fake_host.rs:131` but no E-test ever populates `FakeWorld::busy`, so no `Deferred` disposition is ever produced or asserted; (c) **bounded message size** — contract §9.1 declares three session bounds, the prototype's `DeclaredBounds` carries only two, and of those only `max_observations_per_command` is ever enforced (`src/lib.rs:424`); `max_intents_per_batch` is a dead field. | Protocol §5 Gate C3's enumerated surfaces; mission §5 "requirements-to-evidence inventory". The claim should be narrowed to the surfaces actually exercised, or the three gaps listed under "Known limits". |
| **F-03** | MINOR | Disposition §1 table, SONNET row, "Novel cases: **30**" | §1 is titled "What the receipts actually show" and the LUNA novelty figure is correctly sourced ("13 inherited, **by its own index**"). No SONNET artifact states that its 30 cases were novel — neither `HANDOFF.md`, `FINDINGS_INDEX.md` nor the campaign report uses the word. "30 novel" is the disposition author's own inference presented in a receipts-only table, and "novel" is nowhere defined relative to the existing 427-test workspace suite. | Mission §1: "Record actual elapsed testing and novel coverage" — honestly, i.e. sourced. |
| **F-04** | MINOR | Disposition §7.1 item 3: "Four adversarial passes by **three parties** other than the candidate's writer" | No artifact in the commit or on either campaign branch establishes the number of distinct parties. §2 establishes two continuation testers; the originals were LUNA and SONNET; that is consistent with two, three or four parties, and nothing recorded distinguishes them. The figure is unsubstantiated. | Mission §5: findings and provenance must be recorded, not asserted; protocol §7 requires each convergence artifact to record the evidence actually produced. |
| **F-05** | MINOR | Contract §4.4 "Provenance, stated precisely" and §11.2 row 3 | The literal wording ("a search of `origin/main` at `a73fbac…` finds **no G.A.M.E. record adopting** an outstanding-request/Busy/pull-results integration design") is **correct** — I verified it independently. But it is materially incomplete, and a Gate C4 adapter author reading it would conclude the G.A.M.E. records are silent, which they are not. `origin/main` contains `project_records/research/game_architecture_refoundation/GAME_IMPLEMENTATION_READINESS_CLOSEOUT_20260909.md:44`, which names this exact design — "The challenge closeout's bounded pull/`Busy`/ack result flow and related late-input, transaction-unit, retention, and durable-snapshot choices **remain proposals** … Neither side's proposal is adopted by this record" — and `…/sources/ASTRA_GAME_FOUNDATION_STAGE3_EXPERIMENTAL_HANDOFF_20260909.md:81` ("Additional new work receives pre-admission `Busy`"). The originating proposal lives in S.P.A.R.K.'s own `research/fable-architecture-challenge-2026-09-09` branch, and it includes **retention of the result until acknowledgment** — an element the contract's §4.4/§9.2 design deliberately drops ("There is no queue inside the device"), with the consequence stated in §9.3 row 3. That divergence from an existing named proposal is not flagged anywhere. | Mission §2: "**Recover existing accepted GAME integration decisions before designing replacements** … Reuse an already settled one-outstanding-request/Busy/pull-results design **if verified in current GAME records**". The conclusion drawn (not settled → proposal) is right; the recovery is under-reported. |
| **F-06** | MINOR | `diagnostics.rs` D-3, lines 257–270; Disposition §3 row "SONNET T-2 / T-3" | The T-2 half correctly matches the typed variant (`RuleSetError::ZeroDelay`). The T-3 half asserts only `!errs.is_empty()` with the message "D-3d: expected a typed **zero-budget** error". The assertion does not establish that the returned error concerns the zero pacing budget rather than anything else in the rule set, so the disposition's "**both** fail `rule_set` activation with typed errors" is weaker-supported than it reads. The load-bearing conclusion ("no `Engine` can be constructed that carries either hazard") *is* established, by `expect_err`. | The disposition's own §3 discrimination standard. |
| **F-07** | MINOR | Disposition §3 rows 1–2; `diagnostics.rs` lines 106, 128, 156 | Diagnostic-label collision. "D-1b" denotes two different things: an assertion label *inside* `d1` ("declared maximum + 1 must be an InvalidEffect refusal") and the separate test `d1b_luna001_absent_cell_…`. "D-1f" is cited in the §3 table as though it were a diagnostic, but it is an assertion label inside `d1`; there is no D-1f diagnostic. A reader cannot resolve the §3 citations against the evidence without reading the source. | Mission §5: evidence must be traceable without re-derivation; the operator-next-step record §3.3 requires a recorded finding to be actionable "without re-deriving it". |
| **F-08** | NOTE | `tests/end_to_end.rs` E-8 | "deterministic across **independent runs**" means two in-process constructions, not two process invocations. It does have real power (per-`HashMap` iteration order differs within a process), and no cross-process or cross-platform determinism is claimed anywhere, but the test name invites over-reading. |
| **F-09** | NOTE | `diagnostics.rs` D-6 lines 173–181; Disposition §3 row SONNET T-6 | The disposition states the refusal is `ArtifactBindingMismatch`; the assertion permits `ArtifactBindingMismatch \| EpochChainInvalid \| DigestMismatch`. The claim is nonetheless substantiated, because the captured `full-run.txt` prints the actual variant: "D-6 PASS: … refuses restore (**ArtifactBindingMismatch**)". Evidence carries the claim; the assertion alone would not. |
| **F-10** | NOTE | `luna_continuation/LUNA_C_CAMPAIGN_2026-09-12.md` §"Role and authority"; `sonnet_continuation/SONNET_C_CAMPAIGN_2026-09-12.md` §"Role and authority"; Disposition §2 | Independence of the two continuation testers is **self-attested only**. Neither continuation document records which model or agent ran it, unlike the original SONNET campaign, which records `claude-sonnet-5`. I can confirm the roles were honoured behaviourally (no `crates/` change, isolated `/tmp` workspaces, collection-only) but not who held them. Recorded as an unconfirmed concern, not a defect. |
| **F-11** | NOTE | `engineering/PHASE_STATUS.md` at `d8ab37e` (not one of the four artifacts; unmodified by this commit) | Line 19 and line 52 still read that the Gate C2 bounded-revision candidate "awaits a fresh independent review", which is stale against the KEEP at `21a4fec` that all four artifacts depend on. Outside the four artifacts and not required by the mission, but a reader navigating from the repository's status index would be misinformed about the very gate the disposition addresses. |

**Count by severity: 0 BLOCKER, 1 MAJOR, 6 MINOR, 4 NOTE.**

## 3. Answers to the questions

### A. Is the disposition honest?

**Elapsed-time and novel-coverage accounting: accurate, with one unsourced figure (F-03).**
I checked every number against the campaign artifacts on their own branches via `git show`,
without checking either branch out.

| Claim in disposition §1/§2 | Source | Verdict |
|---|---|---|
| LUNA start `11:30:30Z`, closeout `11:34:00Z`, "about 4 min" | `dfabd058:…/handoff.md` — "Closeout: `2026-09-12T11:34:00Z` (elapsed about 4 minutes)" | **Exact** |
| LUNA 15 cases, 13 inherited, 2 novel | `dfabd058:…/findings.md` — "15 tests total (13 inherited hostile public vectors plus 1 extreme/zero-rate and 1 seeded deterministic test)" | **Exact** |
| SONNET start `11:30:40Z`, handoff ~`11:40Z`, "about 11 min" | `827a135:…/HANDOFF.md` — "started `2026-09-12T11:30:40Z`, this handoff written `2026-09-12T11:40Z`ish" | **Exact** |
| SONNET 30 cases | `827a135:…/HANDOFF.md` — "30 hostile tests" | **Exact** |
| SONNET **30 novel** | *no source* | **Unsourced — F-03** |
| "roughly 15 of a granted 60 minutes" | 4 + 11 | **Correct** |
| LUNA-C budget 26 min, elapsed ~7.2 min (12:25:36→12:32:50), 3 new cases, 0 bugs | `LUNA_C_CAMPAIGN…md`, `full-run.txt` (3 tests, exit 0) | **Exact** (30−4=26; 7m14s) |
| SONNET-C budget 19 min, elapsed ~10.3 min (12:26:13→12:36:29), 7 new cases, 0 bugs | `SONNET_C_CAMPAIGN…md`, `full-run.txt` (7 tests, exit 0) | **Exact** (30−11=19; 10m16s) |
| "roughly 32 minutes against a nominal 60", "10 new discriminating cases", "zero reproducible defects" | sum; 3+7; four run logs | **Correct** |

The nine coverage items listed in §2 map one-to-one onto the ten continuation tests (the
semantic-cap phrase covers two). All twelve evidence files pass `sha256sum -c` against the
checked-in `SHA256SUMS.txt` (exit 0). The disposition is conspicuously honest where it
would have been easy not to be: it volunteers that **neither** original campaign ran its
window, that **neither continuation** used its budget either, and that "no confirmed
product defect … is not proof that no defect exists".

**Are the twelve diagnostics genuinely discriminating?**
Eleven of twelve are. **One is not: D-10 (F-01).** Specifically:

- **Discriminating and strongly asserted:** D-1 (declared max commits, max+1 refuses,
  `i64::MAX` asserted *byte-equal* to max+1, symmetric at the minimum, state-store digest
  unchanged across four refusals while the timeline digest correctly *does* move);
  D-1b (refused-first-write leaves no cell); D-2 (exact resubmission refuses with both
  digests byte-identical, *and* the same payload under a fresh identity finalizes and moves
  the engine — this is the textbook shape of a discriminating test); D-4 (pause +
  `pacing_overrun` + `deferred_cohort_count >= 1`, which an ignored budget could not
  produce); D-5 (exactly one cohort carries `Rejected{InvalidEffect}` naming
  `state.pressure` inside a completed request — a swallowed refusal would leave zero);
  D-7 (baseline control restores; resealed obligation-drop refuses with exactly
  `BidirectionalInvariantBroken`, so the digest cannot be what caught it); D-8 (exact
  `TimelineStagingPresent`); D-9 (value identical, frontier advanced).
- **Discriminating, assertion slightly weaker than the prose:** D-3 (F-06), D-6 (F-09).
- **Observation record, correctly scoped:** D-11 — asserts precisely what H-OBS-01 claims
  and no more.
- **NOT discriminating:** **D-10 (F-01)** — a determinism test doing duty as an idempotence
  test.

D-1 additionally *overturns* the campaign's own words ("expected behavior was not
specified"), which is the right direction for a disposition to push, and the disposition
says so plainly.

**C2W-RN01 and C2W-RN02: carried forward accurately, with no softening.** I compared
disposition §4 against the KEEP review at `21a4fec` lines 105–122 clause by clause. The
substance is preserved including the parts that are unflattering: the split-mutation
non-equivalence of `W01-settle-every-body`, the separate `W01-settle-decay-body` error,
"only their **combined** coverage", "counts no mutation as killed by failure to patch or
compile", and for RN02 the retained sentence "**the claimed walk count understates work**".
Both are repeated as precise mappings rather than labels, and both are carried into §7.2 as
limitations that must travel with any acceptance. **Nothing is softened.** The decision not
to optimize is correctly justified by the mission's own instruction ("Do not optimize merely
to erase a report caveat").

**Is any "no defect" claim stronger than its evidence?** Only via F-01. §5 ("Demonstrated
blockers: **None**") is correctly scoped to "reproducible blocking defect on the pinned
build". §1 explicitly refuses to treat a negative result as proof of absence. §7.2 item 1
concedes non-exhaustiveness and names the absent classes (fuzzing, property-based search,
concurrency, performance). §8 disclaims "absence of defects" outright. The single
overreach is §4's "D-10 confirms there is no observable canonical consequence" and §7.1
item 4's "each of which would have come out differently".

**Recommendation vs acceptance: properly separated, and confirmed independently.**
The disposition says so four times (header "Status", §7 heading, §7 first line, §8), and it
is true. I searched every ref in the repository:

- `git ls-tree -r --name-only <every ref> | grep -iE "OPERATOR_ACCEPT|ACCEPTANCE_FREEZE|GATE_C2.*ACCEPT|ACCEPT.*GATE_C2"` returns exactly two paths, both the **V3-F01** (Gate C1) acceptance: `engineering/phase2/SPARK_PHASE_2_V3_F01_OPERATOR_ACCEPTANCE_FREEZE_2026-09-10.md` and its copy inside a governance-proposal source snapshot.
- `git grep -l -iE "operator (has )?(hereby )?accept|gate c2 (is )?accepted|acceptance of gate c2 …"` across all heads returns no Gate C2 acceptance; every hit is a V3-F01 or a review/mission document.
- The controlling record the disposition cites, `SPARK_PHASE_2_GATE_C2_OPERATOR_NEXT_AUTHORIZED_STEP_2026-09-12.md` §4, does read verbatim: "Acceptance remains the Operator's decision, taken after the independent review and informed by whatever the campaign finds." It also states "Nothing in this record authorizes Gate C2 acceptance".
- The KEEP review itself states "KEEP does not accept Gate C2."

**NO Operator acceptance of Gate C2 exists anywhere in this repository. None is invented.**
The disposition's claim is correct and the separation is clean.

### B. Is the contract sound and correctly scoped?

**Gate C3 surface coverage: complete. Nothing missing.**

| Gate C3 enumerated surface | Contract |
|---|---|
| protocol-version negotiation | §3.1 (major fail-closed, minor downgrade both directions) |
| profile and capability negotiation | §3.2 (content-addressed pair), §3.3 (six tags) |
| stable external entity and affiliation references | §4.1 (injective + stable, both required with reasons) |
| monotonic logical execution point | §4.2 (device-enforced, does not trust the host) |
| typed and scoped host observations | §5 (typed, scoped, authority-respecting, declared-bounds) |
| advisory intent batches | §6 (contract-layer projection, canonical order) |
| host-confirmed outcomes, rejection/defer reasons | §8 (closed vocabularies, explicitly no free text) |
| idempotency, ordering, boundedness, compatibility, fail-closed | §7.1–§7.3, §6.2, §9.1, §3.1, §9.5 |
| deterministic canonical replay independent of transport | §10.1 |

The mission's additional required surfaces (exact correlation/idempotency, bounded message
size/backpressure, unsupported/stale/malformed handling) are all present (§7.1, §9.1–§9.2,
§9.5). Every S.P.A.R.K. type the contract names exists at the pinned tree — I spot-verified
`CommandPayload::Host`, `Request::Advance`, `Engine::reconstruct_completed`,
`CompletedHistory`, `Engine::restore`, `ActivateEpoch`, `Engine::active_request`,
`ProcessResult::terminates`.

**SETTLED vs ENGINEERING PROPOSAL: correctly distinguished, and the §4.4/§11.2 claim is
TRUE — see F-05 for what it omits.** I searched G.A.M.E. `origin/main` at
`a73fbac743adc1a00679ebc63b91027386718d71` (confirmed live) for
`outstanding request|one outstanding|single outstanding`, `\bBusy\b`, and
`pull.result|pull_result|poll.result|pull-results`. **Result, reported either way as
required:** no *adopting* record exists, so the contract's conclusion is right. But two
records do discuss exactly this design, both explicitly non-adopted:

1. `project_records/research/game_architecture_refoundation/GAME_IMPLEMENTATION_READINESS_CLOSEOUT_20260909.md` — classified "working research record; non-operative. No architecture, governance, feature, or cross-project contract is adopted by this record", and at line 44: "**The challenge closeout's bounded pull/`Busy`/ack result flow** and related late-input, transaction-unit, retention, and durable-snapshot choices **remain proposals**. … **Neither side's proposal is adopted by this record.**"
2. `…/2026-09-09_architecture_reconciliation/sources/ASTRA_GAME_FOUNDATION_STAGE3_EXPERIMENTAL_HANDOFF_20260909.md` — "STAGE 3 EXPERIMENTAL HANDOFF — NON-OPERATIVE UNTIL SEPARATELY AUTHORIZED", line 81: "Additional new work receives pre-admission `Busy`".

The origin is S.P.A.R.K.'s own `research/fable-architecture-challenge-2026-09-09`
(`CLOSEOUT_2026-09-09.md` lines 42–43, 93–94): "one outstanding request, bounded result
chunks, `Busy` refusal, and **retention until acknowledgment**". **The contract's design
drops retention-until-acknowledgment** — §9.2 states "There is no queue inside the device"
— and accepts the consequence honestly in §9.3 row 3 ("the host must have durably recorded
the batch before acknowledging, or must treat the batch as lost"). That is a defensible
engineering choice forced by the engine's real behaviour, but the divergence from a named
existing proposal is nowhere flagged. So: the mission's instruction to reuse the design "if
verified in current GAME records" was correctly resolved as *not verified*; the recovery
step was under-reported (F-05).

The §11.2 table is otherwise exemplary — it marks the transaction unit and stale-input
refusal **Settled** (and §4.3's citation to the V3-F01 acceptance freeze checks out: the
freeze's §"Serialized request addendum" row retains the serialized-request-boundary addendum
"exactly by FINAL §19.1–19.2 and Revision-2 §11"), marks acknowledgment identity,
at-most-once, atomic application, the `BehaviorIntent` projection and the closed
vocabularies **Proposed**, marks embedded/service/transport/serialization **Reserved**, and
marks the persistent boundary **Blocked**. §11.3 correctly assigns the three decisions the
main engineer may not take.

**Does anything silently freeze a reserved choice? No.** §10.2 addresses all four reserved
choices by name and declines each: embedded vs service ("Not selected. §9.4 records a real
asymmetry between them; that is a *consequence to weigh*, not a decision taken here"),
transport ("Not selected"), serialization ("Not selected" — with the sound reason that
canonical digests are computed by `CanonicalEncoder` over logical content, "so a
serialization change cannot change a digest"), and concurrency beyond one outstanding
request ("Not proposed"). The prototype's `Cargo.toml` reinforces this: it is detached from
the workspace and carries no transport or serialization dependency at all. Nothing in §4.4
or §9.2 forecloses embedded vs service — it constrains the request/response shape, which is
common to both.

**§9.4's durability gap: accurate, arguably understated, and not papered over.** I checked
it at the pinned crate tree. `crates/spark-engine/src/engine.rs:195–199` documents
`EngineSnapshot` as "An **in-memory value** — no persistence backend and no crash-recovery
claim", and every field is `pub(crate)`. There is no `serde`, no `Serialize`, no
`std::fs` anywhere in `spark-core` or `spark-engine`. So an external consumer cannot even
serialize the snapshot by hand — a *stronger* limitation than §9.4 states. §9.4's three
consequences (embedded loses state with the host; a separate process can lose state while
the host survives; `replay.v1` reconstructs completed boundaries only, never paused
progress) are all correct, and §10.1 correctly records that the completed-only scoping is
"a controlling correction of the frozen architecture, not an oversight". §3.3 correspondingly
forbids advertising `persist.v1`, and E-1 asserts that. The contract states the gap as a
limit and refuses to resolve it, exactly as the mission required. **Not overstated, not
understated in the direction that matters.**

**Protocol §3 authority boundary: not violated anywhere.** §2 restates it unchanged; §6.1
declines to add a `BehaviorIntent` type to the engine; §6.3 makes advisory status
structural — "the contract gives the host **no message that means 'apply this'**"; §8's
"Feedback is re-entry, not application" keeps G.A.M.E. the sole author of world truth; §5.3
expresses the boundary in the type system (a host observation writes only
`Authority::HostOwned`). The contract does impose obligations on G.A.M.E. (§7.2 at-most-once,
§7.3 atomic application, §4.4 concurrency), but each is explicitly labelled **Proposed** in
§11.2 and §11.3 states "this contract cannot grant it" — S.P.A.R.K. proposes, G.A.M.E.
decides. That is the correct handling, not a boundary breach. The prototype code agrees: the
device only projects intents; `fake_host.rs` alone decides legality and mutates the world.

**Scoping note (favourable):** the contract is *more* conservative than the mission
permits. The mission states "Phase-3 connection work is now authorized within this
objective"; the contract nonetheless declines to freeze, stands on protocol §6 working
material, and blocks itself on Gate C2. That is a defensible reading of "subject to
existing predecessor gates" and errs in the safe direction.

### C. Does the prototype prove what it claims?

**Build and test — run by me, in my own worktree and target dir:**

```
$ cd /tmp/handoff-review-20260912/wt/engineering/phase3/spark_game_contract_prototype_2026-09-12
$ CARGO_TARGET_DIR=/tmp/handoff-review-20260912/cargo CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --offline
   Compiling spark-core v0.1.0 (/tmp/handoff-review-20260912/wt/crates/spark-core)
   Compiling spark-engine v0.1.0 (/tmp/handoff-review-20260912/wt/crates/spark-engine)
   Compiling spark-game-contract-prototype v0.0.0 (…/spark_game_contract_prototype_2026-09-12)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 13.66s

running 12 tests
test e11_re_presenting_a_completed_command_is_refused_not_re_executed ... ok
test e2_canonical_first_proof_runs_end_to_end_across_the_seam ... ok
test e1_session_negotiation_is_fail_closed_on_major_and_downgrades_minor ... ok
test e3_a_refused_intent_changes_nothing_in_the_world ... ok
test e3b_unknown_subject_and_unsupported_channel_are_named_rejections ... ok
test e6_a_second_request_while_one_is_active_is_refused_busy ... ok
test e5_stale_horizon_is_refused_with_the_frontier_named ... ok
test e4_duplicate_batch_delivery_applies_at_most_once ... ok
test e10_snapshot_restore_mid_sequence_reproduces_the_uninterrupted_run ... ok
test e7_oversized_observation_list_is_refused_whole ... ok
test e8_the_whole_proof_is_deterministic_across_independent_runs ... ok
test e9_completed_history_replays_to_the_same_digests ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
exit 0
```

I also independently reproduced the fmt and clippy claims (§6 below): both exit 0.
The prototype depends on `spark-engine` **without** the `test-support` feature, so it
exercises the surface a real consumer sees — a genuine strength, not a claim I had to take
on trust.

**Changes nothing under `crates/`: confirmed.**

```
$ git show --stat d8ab37e     → 29 files changed, 5076 insertions(+), 0 deletions(-)
                                 every path under engineering/phase2/ or engineering/phase3/
$ git diff 67b877192cc78b75c6fbe60c69b5594dc10befe8 d8ab37e -- crates/
(no output)   exit 0
$ git rev-parse 67b87719:crates → 7907f4d729104fd5dbfd4adad46e66cf09aa13dd
$ git rev-parse d8ab37e:crates  → 7907f4d729104fd5dbfd4adad46e66cf09aa13dd
```

The crate trees are byte-identical, which also substantiates disposition §6's
validation-reuse argument. The commit is purely additive.

**Workspace members: confirmed exactly three.**

```
$ cargo metadata --format-version 1 --offline   (at the repository root)
members: 3
  path+file:///…/crates/spark-core#0.1.0
  path+file:///…/crates/spark-engine#0.1.0
  path+file:///…/crates/spark-testkit#0.1.0
exit 0
```

The prototype detaches itself with an empty `[workspace]` table and a documented comment
explaining why.

**Are the E-1…E-11 assertions strong enough for the claims their names and printed messages
make? Attacked hard; they hold.** This is where I expected to find the "prints PASS: X,
asserts weaker-than-X" failure, and the prototype largely does not commit it:

- **E-1** asserts the *negative* that matters (`persist.v1` must not be advertised) rather
  than only the positives; refuses major 2 with the exact `Rejected` value; checks minor
  downgrade on both the descriptor and the device; and checks `ProfileMismatch` against a
  deliberately wrong hash. Message matches.
- **E-2** is a real end-to-end causal chain with hard numeric assertions at every link
  (affordability `3*30−1*20 = 70`, exposure 70, stress 70, exactly one intent on channel
  `intent.hunt` with value 1, host executed once, wildlife 100→90, neighbour pressure
  `100−90 = 10`, feedback reaching the neighbouring actor at 10) and it asserts the *absence*
  of a second intent below threshold. This is materially stronger than its message.
- **E-3** is genuinely discriminating: with the actor forbidden, it asserts zero executions,
  an unchanged world, **and** that S.P.A.R.K. still advised — then asserts neighbour pressure
  is `Some(0)`, i.e. the causal loop reflects the *observed* world, not the advice.
- **E-4** counts four deliveries and asserts the world moved exactly once (`wildlife == 90`,
  `executed_hunts == 1`) and that every redelivery returned an identical report.
- **E-5** matches the exact `StaleHorizon { frontier: 20, horizon: 5 }` *and* asserts the
  canonical state digest is byte-identical afterwards.
- **E-6** asserts `Busy` names the active horizon, and then that the *original* request
  completes on re-presentation — which is the actual §4.4 pull semantics, not just a refusal.
- **E-7** asserts the exact `(limit, observed) = (1, 2)` and an unchanged digest.
- **E-9** reconstructs payloads from the fixture and **asserts the reconstruction hashes to
  the recorded `canonical_payload_hash`** before replaying, then matches both digests. The
  dependency is disclosed in the README's "Known limits".
- **E-10** asserts the post-restore batch is byte-identical to the uninterrupted reference
  batch and that engine digests converge — and its printed message carries an explicit
  "LIMIT: … this is **NOT** durable process-recovery evidence".
- **E-11** asserts the typed refusal and an unchanged digest, and prints the adverse
  CONSEQUENCE rather than only the pass.

Two qualifications, neither a defect: **E-8**'s "independent runs" are in-process (F-08),
and **E-4/E-7** exercise contract-layer rules implemented by the prototype itself — they
demonstrate the rules are implementable and self-consistent, not that a real G.A.M.E. host
obeys them. The file header states exactly that limit in its first paragraph. What is
genuinely *not* exercised is affiliation identity, the `Deferred` disposition, and two of
the three declared bounds (F-02).

**Fake host labelling: consistently and repeatedly correct.** `tests/end_to_end.rs` lines
1–8: "a passing run here is preparatory evidence about the S.P.A.R.K. side of the contract.
It is **not** proof of G.A.M.E. integration, and **no test in this file may be cited as
one**." `src/fake_host.rs:1`: "A **fake** G.A.M.E. host." README: "**Does not:** prove
G.A.M.E. integration. `src/fake_host.rs` is a fake. Nothing here was authored, reviewed or
accepted by G.A.M.E." E-2's own printed PASS line ends "(fake host: preparatory evidence
only)". The commit message repeats it. I found no place where it is presented as
integration proof.

**Compile-only labelling: consistently correct, no runtime or parity claim.** Each of the
five blocks in `evidence/proto-cross-targets.txt` is headed
"(COMPILE-ONLY, no runtime execution)"; the README table marks all five "PASS —
**COMPILE-ONLY**" and adds "Linux is the only platform where anything was **executed**. …
no Windows or Android runtime, replay or digest-parity evidence exists, and no
three-platform equivalence is claimed." Disposition §7.2 item 4 and §8 repeat the
disclaimer. This satisfies mission §3 ("compile-only checks must remain labeled
compile-only") without exception.

### D. Anything overstated anywhere?

The four artifacts are, as a body, unusually disciplined — the nonclaims sections are real,
the limitations travel with the recommendation, and several claims are *understated*
(§9.4's durability gap is worse than described; E-2 asserts more than it advertises). The
overstatements I can substantiate are:

1. **Disposition §4:** "D-10 confirms there is no observable canonical consequence." It does
   not; it confirms cross-run reproducibility. **(F-01, MAJOR)**
2. **Disposition §7.1 item 4:** "twelve diagnostics … each of which would have come out
   differently had the defect explanation been the true one." False for D-10. **(F-01)**
3. **Disposition §3 preamble:** "Each disposition below therefore rests on a **discriminating
   test**…" — with the same exception. **(F-01)**
4. **`diagnostics.rs` D-10 printed message:** "RN02 repeated settlement has **no observable
   canonical effect**" while asserting only run-to-run agreement. This is precisely the
   prints-X/asserts-less-than-X pattern. **(F-01)**
5. **Prototype README:** "supports **every** logical surface Gate C3 enumerates". **(F-02)**
6. **Disposition §1 table:** SONNET "Novel cases: 30", in a section titled "What the
   receipts actually show". **(F-03)**
7. **Disposition §7.1 item 3:** "by **three parties**". **(F-04)**
8. **Contract §4.4:** "a search of `origin/main` … finds no G.A.M.E. record adopting…" —
   literally true, incomplete as a provenance recovery. **(F-05)**
9. **Disposition §3 row T-2/T-3:** "both fail `rule_set` activation with **typed errors**" —
   the T-3 assertion does not establish which typed error. **(F-06)**

Claims I specifically tried to break and **could not**: the elapsed-time table; the
"zero reproducible defects" framing; the RN01/RN02 carry-forward; the "no Operator
acceptance exists" statement; the "nothing under `crates/` is modified" statement; the
three-workspace-member statement; the fmt/clippy/test evidence; the compile-only labelling;
the fake-host labelling; the durability-gap statement; the reserved-choice statements; and
every §0 pin including the two byte-identical protocol copies.

## 4. Exact commands run, exit codes, and real output

Everything below was executed by me. Anything marked *(read via `git show`)* did not check
out the branch.

| # | Command | Exit | Result |
|---|---|---|---|
| 1 | `git -C /home/chromikey/Projects/SPARK ls-remote --heads origin` | 0 | `d8ab37ed…7bd9  refs/heads/candidate/spark-game-safe-handoff-20260912` present, plus all four predecessor pins (§1 above) |
| 2 | `df -h /` (before) | 0 | `117G 102G 9.2G 92% /` |
| 3 | `git worktree add --detach /tmp/handoff-review-20260912/wt d8ab37e…` | 0 | `HEAD is now at d8ab37e Dispose the Gate C2 campaign and prepare the SPARK-GAME contract` |
| 4 | `git show --stat d8ab37e` | 0 | 29 files changed, 5076 insertions(+), 0 deletions(-); all under `engineering/phase2/` or `engineering/phase3/` |
| 5 | `git diff 67b877192cc78b75c6fbe60c69b5594dc10befe8 d8ab37e -- crates/` | 0 | **no output** — nothing under `crates/` changed |
| 6 | `git rev-parse 67b87719:crates` / `git rev-parse d8ab37e:crates` | 0 | both `7907f4d729104fd5dbfd4adad46e66cf09aa13dd` |
| 7 | `cargo test --offline` in the prototype (target dir `/tmp/handoff-review-20260912/cargo`, `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=2`) | 0 | **12 passed; 0 failed; 0 ignored** — full list in §3C |
| 8 | `cargo metadata --format-version 1 --offline` at repo root | 0 | **3** workspace members: `spark-core`, `spark-engine`, `spark-testkit` |
| 9 | `cargo clippy --offline --all-targets -- -D warnings` in the prototype | 0 | `Finished dev profile … in 2.34s`; no warnings |
| 10 | `cargo fmt --all -- --check` in the prototype | 0 | no output |
| 11 | `sha256sum -c SHA256SUMS.txt` in `gate_c2_campaign_disposition_evidence_2026-09-12/` | 0 | all **12** files `OK` |
| 12 | `sha256sum -c SHA256SUMS.txt` in `spark_game_contract_prototype_2026-09-12/evidence/` | 0 | all **4** files `OK` |
| 13 | `git show dfabd058:…/luna_adversarial_campaign_2026-09-12/{README,handoff,findings}.md` *(read via `git show`)* | 0 | start `11:30:30Z`, closeout `11:34:00Z` "elapsed about 4 minutes"; "15 tests total (13 inherited … plus 1 … and 1 …)" |
| 14 | `git show 827a135:…/adversarial_sonnet_campaign_2026-09-12/{HANDOFF,FINDINGS_INDEX,SPARK_ADVERSARIAL_SONNET_CAMPAIGN_2026-09-12}.md` *(read via `git show`)* | 0 | started `11:30:40Z`, handoff `~11:40Z`; 30 cases; "Reproducible bugs found: 0"; five declared coverage gaps |
| 15 | `git -C /home/chromikey/Projects/GAME_PROJECT ls-remote --heads origin main` | 0 | `a73fbac743adc1a00679ebc63b91027386718d71` — matches the contract pin exactly |
| 16 | `git grep -i -n "outstanding request\|one outstanding\|single outstanding" origin/main` (GAME) | 0 | 2 hits, both research records (§3B) |
| 17 | `git grep -i -n "\bBusy\b" origin/main` (GAME) | 0 | hits in telemetry/console code plus the two research records quoted in F-05 |
| 18 | `git grep -i -n "pull.result\|pull_result\|poll.result\|pull-results" origin/main` (GAME) | 1 | **no matches** |
| 19 | `git show origin/main:project_records/foundation/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md \| sha256sum` (GAME) vs `sha256sum` of the SPARK copy | 0 | both `611f21ff5e39575f414e245b8409a2b6f7811a5f2e164bd81a3c69e7cbf0a06a` |
| 20 | `git ls-tree -r --name-only <every ref> \| grep -iE "OPERATOR_ACCEPT\|ACCEPTANCE_FREEZE\|GATE_C2.*ACCEPT\|ACCEPT.*GATE_C2"` | 0 | only `SPARK_PHASE_2_V3_F01_OPERATOR_ACCEPTANCE_FREEZE_2026-09-10.md` (Gate C1) and its governance-snapshot copy — **no Gate C2 acceptance** |
| 21 | `git grep -l -iE "operator (has )?(hereby )?accept\|gate c2 (is )?accepted\|acceptance of gate c2 …" <all heads> -- 'engineering/*'` | 0 | every hit is a V3-F01 acceptance or a review/mission document; **no Gate C2 acceptance** |
| 22 | `grep -n -B8 "pub struct EngineSnapshot" crates/spark-engine/src/engine.rs` | 0 | "An in-memory value — no persistence backend and no crash-recovery claim"; all fields `pub(crate)` |
| 23 | `grep -rn "serde\|serialize\|std::fs" crates/spark-engine/src crates/spark-core/src` | 0 | no serialization or filesystem surface (only `into_bytes` helpers) |
| 24 | `grep -rn -i "affiliation" src/ tests/` (prototype) | 1 | **no matches** (F-02) |
| 25 | `grep -rn "Defer" src/ tests/` (prototype) | 0 | defined in `src/lib.rs:239,256`, produced in `src/fake_host.rs:131`; **no test occurrence** (F-02) |
| 26 | `grep -n "max_intents_per_batch\|max_observations_per_command" src/lib.rs` | 0 | only `max_observations_per_command` is enforced (line 424); `max_intents_per_batch` unused (F-02) |
| 27 | `df -h /` (after all builds) | 0 | `117G 104G 7.6G 94% /`; my cargo dir `du -sh` = **112M** |

Files I wrote: this review only, under `/tmp/handoff-review-20260912/`. My worktree and
cargo directory were removed after writing it. No other worktree, branch, cache or file was
touched, reset or deleted.

## 5. Verdict

The four artifacts are, on the whole, careful and honest work. The elapsed-time accounting
is exact and self-incriminating where it could have been flattering; the acceptance
recommendation is cleanly separated from an acceptance that genuinely does not exist; the
KEEP review's two qualifications are carried forward without a word of softening; nothing
under `crates/` moved; the reserved architectural choices are all still reserved; the
durability gap is stated more harshly than it had to be; and the compile-only and fake-host
labels are applied everywhere they belong. Eleven of the twelve diagnostics are properly
discriminating, and several of the prototype's tests assert materially more than their
names claim.

But one diagnostic does not do the job the disposition says all twelve do, and the
disposition leans on "all twelve" as a pillar of its Gate C2 acceptance recommendation to
the Operator. D-10 compares the implementation against itself — exactly the argument-from-
implementation the mission forbade — while printing that repeated settlement "has no
observable canonical effect". That is a bounded, enumerable defect in a document that will
be read as support for an Operator decision, and it should be corrected before the artifacts
are relied on as published. The remaining findings are small and specific.

Corrections required (and sufficient):

1. **F-01** — either strengthen D-10 into a genuinely discriminating test (assert the
   committed value against a single-walk reference, e.g. `clamp(0+100, 0, 60) = 60`, or
   compare the composed body against an equivalent non-composed path), or restate
   disposition §3, §4 and §7.1 item 4 so they claim only what D-10 establishes —
   reproducibility — and say plainly that RN02's read-only character rests on the KEEP
   review's source analysis rather than on a discriminating test.
2. **F-02** — narrow the prototype README's "every logical surface Gate C3 enumerates", or
   list affiliation identity, the `Deferred` disposition and the two unenforced bounds under
   "Known limits".
3. **F-03** — source or qualify the SONNET "30 novel" figure.
4. **F-04** — state the party count only as far as the evidence supports it.
5. **F-05** — cite the two G.A.M.E. research records and the S.P.A.R.K. Fable closeout in
   §4.4/§11.2, and note that the contract's design deliberately drops the proposal's
   retention-until-acknowledgment element.
6. **F-06** — assert the specific zero-budget `RuleSetError` variant, or soften "typed
   errors" for the T-3 half.
7. **F-07** — disambiguate the D-1b / D-1f labels between the disposition table and the
   diagnostic source.

**Verdict: `HANDOFF_REVIEW_BOUNDED_REVISION_REQUIRED`**

Nothing structural is wrong: the gate sequencing, the authority boundary, the
settled/proposed separation, the isolation of the prototype and the absence of an invented
acceptance are all sound, so a foundational review is not warranted. Equally, the MAJOR is
not a detail — it is a claim made to the Operator that its own evidence does not carry — so
KEEP is not available either.

## 6. Addendum — local commits beyond the reviewed commit (observed read-only)

Recorded for completeness, because it is material to how this verdict should be used.

The reviewed commit `d8ab37ed6489843380d27e381c716021a18b7fd9` **is** the live head of
`candidate/spark-game-safe-handoff-20260912` on `origin` (verified in §1). However, the
local worktree `<WORKTREE>` sits two commits ahead of it
on the same branch, and those two commits are **not published to GitHub**:

```
$ git log --oneline d8ab37e..beebac3
beebac3 Bring the engineering status index up to date
747e44a Correct the contract after recovering prior GAME integration records
$ git merge-base --is-ancestor d8ab37e beebac3  →  YES (fast-forward descendant)
```

From their subject lines alone, `747e44a` appears directed at the same issue as **F-05**
(contract provenance after recovering prior G.A.M.E. integration records) and `beebac3` at
**F-11** (the stale `engineering/PHASE_STATUS.md`).

I did **not** inspect, check out, build or review either commit: my instruction named
`d8ab37e` as the exact commit to review, and that is the commit this verdict applies to.
I touched nothing in that worktree. Every finding above stands against `d8ab37e` as
published. F-05 and F-11 may already be addressed in unpublished local work, in which case
they should be closed by inspecting those commits rather than by new edits — but that is a
determination for whoever holds the writer role, and it requires its own review of the
published result. **F-01, the MAJOR, concerns the campaign disposition's diagnostic D-10
and is not plausibly addressed by either subject line.**
