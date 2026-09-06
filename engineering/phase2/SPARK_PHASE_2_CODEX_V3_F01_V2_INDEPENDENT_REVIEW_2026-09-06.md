# S.P.A.R.K. Gate C1 — Independent V3-F01 V2 Acceptance Review

**Date:** 2026-09-06
**Reviewer:** Codex, independent adjudicating reviewer
**Repository:** `/home/chromikey/Projects/SPARK`
**Branch:** `phase1-refoundation-v2`
**Writer commit reviewed:** `983a01fd6807b8c627c97d2673a1f40c54cc059c`
**Writer parent:** `379f8dc355125e6bca09801e4a66b0b35ec9720c`
**Overall verdict:** `V3_F01_FOUNDATIONAL_REVIEW_REQUIRED`

## 1. Executive adjudication

The pinned V2 writer commit passes its lineage and changed-file gate. The core scheduled-
work correction also makes real progress: Fable's horizon expansion with cohort-local
evaluation time supplies a sound partition proof for command-free catch-up and for fully
drained catch-up segments; the least-live-slice compare-and-take is safe-Rust implementable;
its full slot fingerprint closes the previous key-only TOCTOU defect; scheduled and
conflicted entries are correctly separated into one operational extraction slice and an
executable-only cohort; and the proposed canonical report expansion has been withdrawn.

The combined candidate is not ready for Operator acceptance. It introduces SH-1/SH-2
command refusals using a canonical frontier `Phi`/`Φ` that has no exact representation,
update rule, digest commitment, or reconstruction rule. Gate C3's phrase "monotonic logical
execution point" is a future contract-freeze checklist that applies only after Phase 2 is
accepted and Phase 3 is separately opened; it is not present Gate C1 authority for a new
canonical command refusal. Worse, SH-2 makes accepted-command semantics depend on the
pacing budget: the same call/command history can accept a command after a large-budget
drain and refuse it after a small-budget incomplete drain. This violates the inherited
rule that quota/yield changes latency, not semantics. The command-time question left open
by Fable therefore remains foundational rather than being resolved by the V2 writer.

The acceptance oracle is independently insufficient. Four required fixtures state false
outcomes: budget one cannot admit both `S` and the following `T`; removing both `S` and
conflicted `X` (and their obligation records) before capture leaves no retained difference
in the pre-wave engine digest or batch; scheduling a different payload into a key that was
freed by extraction creates a fresh `Scheduled` slot, not a conflict; and command refusal
R-8 cannot uniformly have extracted WorkKeys or WorkKey retry semantics. The rejection
model also mistakes wave atomicity for whole-cohort atomicity when a later wave rejects
after earlier waves committed.

No implementation or freeze authority is released.

## 2. Lineage and writer-scope gate

The repository gate was checked against live Git before substantive review:

| Check | Result |
|---|---|
| Branch | `phase1-refoundation-v2` |
| Live HEAD at entry | `983a01fd6807b8c627c97d2673a1f40c54cc059c` |
| Pinned target | `983a01fd6807b8c627c97d2673a1f40c54cc059c` — exact HEAD |
| Sole parent | `379f8dc355125e6bca09801e4a66b0b35ec9720c` — exact expected parent |
| Worktree at entry | clean |
| Writer diff whitespace check | pass |
| Later commits requiring isolation | none |
| Rust/source/test changes | none |
| Historical candidate/review changes | none |

The exact target diff contains only:

1. `engineering/PHASE_STATUS.md` — one bounded V2 candidate pointer.
2. `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_V2_2026-09-06.md` — added.
3. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_V2_2026-09-06.md` — added.
4. `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_V2_2026-09-06.md` — added.

The scope gate therefore passes and review proceeds. The writer's `RESOLVED` labels are
treated only as claims.

## 3. Exact material reviewed

### 3.1 Target and immediate Gate C1 record, read in full

1. `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_V2_2026-09-06.md`
2. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_V2_2026-09-06.md`
3. `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_V2_2026-09-06.md`
4. the V2 subsection of `engineering/PHASE_STATUS.md`
5. `engineering/phase2/SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md`
6. `engineering/phase2/SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_REPORT_2026-09-06.md`
7. `engineering/phase2/SPARK_PHASE_2_CODEX_V3_F01_INDEPENDENT_REVIEW_2026-09-06.md`
8. the preserved first-pass V3-F01 candidate, matrix, and writer report

### 3.2 Governing and inherited architecture/oracles reconciled

1. `engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md`
2. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md`
3. `engineering/phase0/ADR-0001-rust-workspace-and-dependency-direction.md`
4. `engineering/phase0/ADR-0002-authority-and-host-acknowledgement.md`
5. `engineering/phase0/ADR-0003-deterministic-clock-rng-and-commit-order.md`
6. `engineering/phase0/ADR-0006-persistence-versioning-and-bounded-provenance.md`
7. `engineering/phase0/PHASE_0_PERFORMANCE_AND_SECURITY_BUDGET_v0.2.md`
8. `engineering/phase0/PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
9. `engineering/phase1/SPARK_PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_2026-08-26.md`
10. `engineering/phase2/PHASE_2_FABLE_READINESS_BLUEPRINT_2026-08-26.md`
11. `engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_2026-08-26.md`
12. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_2026-08-26.md`
13. `engineering/phase2/SPARK_PHASE_2_CODEX_ARCHITECTURE_ADVERSARIAL_REVIEW_2026-08-26.md`
14. `engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v2_2026-08-26.md`
15. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v2_2026-08-26.md`
16. `engineering/phase2/SPARK_PHASE_2_CODEX_ARCHITECTURE_V2_REREVIEW_2026-08-26.md`
17. `engineering/phase2/SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md`
18. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v3_2026-08-26.md`
19. `engineering/phase2/SPARK_PHASE_2_CODEX_FINAL_ARCHITECTURE_V3_CONFIRMATION_2026-08-26.md`

Relevant live Phase-1 source and workspace policy were inspected directly:

- `crates/spark-core/src/clock.rs`
- `crates/spark-core/src/evidence.rs`
- `crates/spark-core/src/scheduler.rs`
- `crates/spark-core/src/timeline.rs`
- `crates/spark-core/src/lib.rs`
- `crates/spark-engine/src/lib.rs`
- `crates/spark-engine/src/state.rs`
- all three crate manifests and
  `crates/spark-testkit/tests/workspace_dependency_direction.rs`

No repository-level `AGENTS.md`, `CLAUDE.md`, or `CODEX.md` exists in this repository; the
project's controlling instructions are the engineering artifacts above and the Operator
mission.

## 4. Findings

| ID | Severity | Finding | Required disposition |
|---|---|---|---|
| V2-01 | **BLOCKER / FOUNDATIONAL** | SH-1/SH-2 add canonical command-execution refusals without current authority. Gate C3 is future work, and Fable explicitly left the command-time admission question open. SH-2 also makes accepted-command semantics pacing-dependent. | A focused foundational command-time/history adjudication is required. Do not treat Gate C3 as already frozen. |
| V2-02 | **BLOCKER / FOUNDATIONAL** | `Φ` is behavior-relevant retained state but has no field, update law, stable-boundary encoding, digest treatment, or reconstruction function. AT-I29(d) is a future test, not a definition. | Define and commit the frontier completely, or remove every rule that reads it. Cover successful/no-effect/rejected/all-conflicted/command-refusal paths. |
| V2-03 | **MAJOR** | Candidate §6 and AT-I45(a) claim budget 1 admits `S` and then `T`; loop A5 leaves `r=0`, so A3 defers `T`. The same mistake appeared in the preceding Codex review's illustrative counterexample. | Correct the premise. Use budget 2 to discriminate next-cohort admission, or retain budget 1 and assert only count/overrun diagnostic differences. |
| V2-04 | **MAJOR** | Candidate §6 and AT-I40(e) require different post-extraction pre-wave digests/batches for `{S}` versus `{S,X_conflicted}`. Extraction removes `S`, `X`, and their obligation records before capture, leaving equal retained state. | Require equal cohort/emission/pre-wave/batch/final-state values; require only the conflict report/extraction outcome to differ. |
| V2-05 | **MAJOR** | The cross-store extraction is asserted atomic but not specified as one implementable engine operation. X-3 removes only scheduler entries and returns payload hashes, while the subsequent obligation-record removal can fail or discard records needed to evaluate scheduled obligations. | Define engine-level preflight, full record extraction, typed no-op failures, exclusive-borrow sequencing, and infallible apply across both stores. |
| V2-06 | **MAJOR** | Rejection rules conflate per-wave atomicity with whole-cohort atomicity. On a later-wave rejection, earlier wave commits remain canonical, so final digest cannot equal the original post-extraction digest. R-8 has no extracted slice or WorkKey retry. | Split wave-0, later-wave, conflict-only, obligation-refusal, semantic-cap, and command-refusal assertions. |
| V2-07 | **MAJOR** | AT-I42(d)(2) contradicts Phase-1 key-freeing semantics: after extraction, scheduling one different payload into the empty key yields `Scheduled`, not `Conflicted`. | Create the occupied-key mutation needed for conflict, or expect successful replay against the new scheduled state with a changed fingerprint. |
| V2-08 | **MAJOR** | The matrix overclaims structural visibility. X-1…X-4 are explicitly `pub` in the public `spark_core::scheduler` module, so a crate directly depending on `spark-core` can call them. A no-re-export convention cannot make a host compile probe fail. | Test the actual engine/host facade boundary, or introduce a structural engine-owned authority boundary; do not claim Rust privacy that the signature lacks. |
| V2-09 | **MAJOR** | Several `Kills` lines do not discriminate their named wrong implementation: extraction-attempt count need not expose tick iteration; a remaining-budget carry bug need not make prefix lengths agree; and feature hygiene proves feature gating, not that a generic digest value is causally unreadable after observation. | Give each wrong implementation a fixture/observable that necessarily differs; narrow claims where structural proof is unavailable. |
| V2-10 | **MINOR** | All-conflicted slices are declared not to be cohorts, yet X-4 counts operational slices to populate `deferred_cohort_count`; after an oversized exception A2 stops before later all-conflicted slices despite the claim they never stop progress. Mixed-slice report ordering is also underdefined. | Define executable-only diagnostics and deterministic mixed/conflict report order, including post-exception residual conflicts. |

## 5. Concrete counterexamples and falsification attempts

### 5.1 Budget one does not admit the following cohort

Candidate §7 is decisive without implementation:

```text
B = 1
slice 100/P = {S scheduled, X conflicted}; e = 1
slice 101/P = {T scheduled}; e = 1

A3 on 100/P: 1 <= 1, admit normally; exception_fired remains false
A5: r := r - e = 1 - 1 = 0
A3 on 101/P: 1 <= 0 is false and admitted_executable != 0
result: T is deferred
```

Candidate lines 385–390 and matrix AT-I45(a), lines 421–428, say the opposite. The prior
Codex review's §5.5 also incorrectly attributed `T` deferral uniquely to counting the
conflict. The underlying F-04 finding remains valid, but this fixture distinguishes only
the pacing count and overrun diagnostic: conflict-inclusive counting treats size two as an
oversized exception, while executable-only counting admits size one normally. For an
admission-boundary discriminator, set `B=2`: executable-only counting admits `S` then `T`;
conflict-inclusive counting consumes the full budget at the first slice and defers `T`.

### 5.2 Conflicted `X` leaves no retained pre-wave difference after extraction

Compare otherwise identical operational slices at `(t,P)`:

```text
A = {S scheduled}
B = {S scheduled, X conflicted}
```

The V2 loop removes the complete operational slice and every corresponding
`ObligationStore` record before defining the pre-wave digest (candidate lines 435–452).
After that step both schedulers contain the same residual slots and both obligation stores
contain the same residual records. No other retained store was changed by extracting `X`.
The inherited engine digest commits to retained stores, not returned reports. Therefore:

```text
scheduled_cohort_identity(A) == scheduled_cohort_identity(B)
pre_wave_engine_digest(A)    == pre_wave_engine_digest(B)
effect_batch_digest(A)       == effect_batch_digest(B)
scheduled evaluation/final state are equal
conflict report(A)           != conflict report(B)
```

Candidate lines 408–416 and AT-I40(e), lines 182–190, incorrectly move the
pre-extraction scheduler difference across extraction. The test would reject the correct
implementation and accept a wrong implementation that leaks `X` into pre-wave identity.

### 5.3 `Φ` cannot be reconstructed from the enumerated stable state

Construct two reachable engines with identical activated artifacts and no-effect scheduled
work. In run A consume the sole cohort at time 10; in run B consume the sole cohort at time
20. Advance both host clock frontiers to `T=100`. Let the cohorts create no effects,
obligations, ledger writes, or cooldowns. At return both have:

- empty scheduler and obligation store;
- identical state, epoch, occurrence, cooldown, and timeline stores;
- equal clock frontier 100; and
- equal frozen v1 §8 engine digests, because neither the clock nor `Φ` is a component.

Yet the proposed `Φ` values are 10 and 20. The same next command at `effective_time=15`
is accepted under SH-1 in A and refused in B. Thus `Φ` is not a function of the candidate's
listed stable state and violates the retained-state law. The ambiguity is worse for an
all-conflicted slice, which is consumed and reported but explicitly has no cohort: the
candidate never says whether that due time advances `Φ`.

The document also never states whether `Φ` advances after a no-effect cohort, a wave-0
rejection, a later-wave rejection, a successful command, an R-8 refusal, or an
all-conflicted operational step. Candidate §14 acknowledges reconstructibility as an open
risk; AT-I29(d) merely promises that a later implementation will fail if the architecture
is incomplete.

### 5.4 SH-2 makes pacing causal

Use one initial state with executable cohorts `A@10` and `B@20`, then the same input
history in both runs:

```text
advance(20)
execute finalized command at effective_time 21
```

With budget 1, the call consumes `A`, leaves `B@20`, and SH-2 refuses the command because
resident `20 < 21`. With budget 2, the call consumes both cohorts and the same command is
executed. The only changed parameter is `max_due_per_cycle`, which v3 §3.2 and ADR-0003
§15 classify as pacing. A typed deterministic refusal is still a semantic difference; it
does not become causally inert because it is fail-closed.

This is not hypothetical malformed input: AT-I43(f) deliberately constructs the small-
budget branch. Fable's P′-commands theorem avoids the contradiction only by requiring
complete drainage before the next command. It does not prove partition equivalence for an
incomplete paced segment followed by a command, which is the case at issue.

### 5.5 Gate C3 supplies no present SH authority

The convergence protocol says Gate C3 occurs **after Phase 2 is accepted and Phase 3 is
separately opened**, and directs that gate to freeze a future product integration contract.
Its bullet "monotonic logical execution point" is not labeled an already-frozen Phase-2
rule and does not define `Φ`, backlog disposition, staging, or command refusal. Gate C1
cannot cite that future task as authority for R-8. Fable's downstream constraint 9 is
explicit: the V3-F01 pass must not add an `effective_time` admission rule. Renaming the
same semantic restriction an execution-door refusal does not resolve the deferred
foundational choice.

ADR-0003 orders finalized commands by ordinal and carries `effective_time` in their
canonical envelopes, but it does not require monotonic effective times or a scheduler-
backlog check. Choosing staging rejection, execution refusal, an implicit drain, or a
different time model changes canonical behavior and requires its own adjudication.

### 5.6 Fresh scheduling into a freed key is not a conflict

Phase-1 source is exact:

- `Scheduler::drain_due` removes the slot;
- the scheduler contract says a drained conflicted key is free again; and
- `Scheduler::schedule` maps an empty slot plus one payload to `Scheduled`.

Therefore AT-I42(d)(2)'s sequence — extract key K, then schedule K once with a different
payload — yields `Scheduled(new_payload)`. A conflict requires at least two unequal claims
while the key is occupied, for example schedule `K/A`, observe, then schedule `K/B`, or
after extraction schedule `K/B` and then `K/C` before replay.

### 5.7 Later-wave rejection cannot restore the original post-extraction digest

Take a cohort whose wave 0 commits a cell update, then whose threshold emission makes wave
1 contain an unequal RESULT conflict. The frozen pipeline commits each validated wave and
chains the next wave to its post-commit engine digest. Wave 1 rejection applies none of
wave 1, but it does not erase wave 0. The final state therefore equals the state before
wave 1, not the cohort's original post-extraction/pre-wave-0 state.

Candidate §8's claim that extraction is the only mutation when "a wave" rejects is true
only for wave 0. AT-I44(a)'s uniform equality to the post-extraction digest would either
reject correct later-wave behavior or require unauthorized whole-cohort rollback.

R-8 is a separate category error: a command barrier has no scheduler extraction, no
extracted WorkKeys, no `ObligationStore` cleanup, and no producer retry under a WorkKey.
The command may need a command-specific retry/refinalization rule, but it cannot satisfy
AT-I44(a), (b), or (d) as written.

### 5.8 Cross-store extraction is not yet one total operation

X-3 alone can be implemented safely and atomically over `Scheduler`: scan and fingerprint
under `&mut self`, compare, then remove one contiguous live least slice. But candidate A4
then separately removes `ObligationStore` records and merely declares the two mutations
one atomic step. No engine-level signature or failure ordering is provided.

This omission is functional, not cosmetic. `SliceExtraction.due` contains only
`DueWorkItem { WorkKey, WorkPayload }`; the Phase-1 payload is only the obligation record's
hash. If the full scheduled obligation records are removed without being returned in an
engine-owned extraction, the evaluator no longer has the `MaterializedEffect` or
`RuleReEvaluation` record it must execute. If a record is missing or mismatched, removing
the scheduler first would violate the promised byte-identical failure.

A coherent safe-Rust operation must preflight the live slice against the complete
corresponding obligation claim sets, return/move the executable records needed by
evaluation, and then perform infallible removal of both stores while holding exclusive
engine ownership. A scheduler fingerprint alone does not bind an independently corrupted
or missing obligation-store record.

### 5.9 Static refinement and full scheduler fingerprint attacks failed

Two adversarial attacks did not find defects:

1. The immutable-handle then mutable-take borrow pattern is ordinary non-lexical-lifetime
   Rust once the handle's last use produces the owned fingerprint.
2. For a static scheduler with a null evaluator, repeatedly extracting the live least
   `(due_time,profile_id)` slice yields the same ordered scheduled and conflicted vectors,
   including bounded/truncated conflict evidence, and the same empty/residual digest as
   `drain_due(T)`. This holds for mixed, all-conflicted, oversized, and same-time
   multi-profile states.

The proposed fingerprint is appropriately stronger than the first pass: it commits to
key, slot tag, scheduled payload hash, the complete tracked conflict set, and truncation
flag using the existing canonical per-slot encoding. Replay of an identical fingerprint is
not authority because X-3 always selects the live least slice. These local properties are
accepted subject to the missing engine-level cross-store operation.

## 6. Horizon expansion and partition-equivalence ruling

### 6.1 What is proved

Fable's A4 rule and the V2 restatement correctly distinguish the host horizon `T` from
scheduled-cohort canonical time. When every scheduled cohort evaluates with
`now=due_time`, newly created work uses checked `due_time >= creator_time+1`, and the loop
recomputes the live least slice after each stable boundary:

- a created key is identical across one-call and stepwise horizons;
- created work inside `T` joins the correct later slice rather than a call-entry plan;
- same-time profile cohorts cannot be invaded by their own output because output is
  strictly later;
- delayed work beyond `T` remains resident identically;
- checked `u64` overflow rejects the same wave in every partition; and
- the prefix induction derives, rather than assumes, equal later keys and membership.

For command-free segments, or segments completely drained before the next command,
Theorem P′ and its completed-horizon composition are sound. The theorem is also properly
narrowed away from the static `drain_due` refinement.

### 6.2 What is not proved

The candidate does not prove semantic equivalence for a command after an incomplete paced
segment. SH-2 instead makes the two runs differ. PE-B/PE-C exclude that case, and saying
that a refused command replays deterministically is not partition equivalence.

Equal-prefix pacing is sound at cohort boundaries while both runs have reached the same
cohort, but operational all-conflict steps and the host clock/frontier need exact treatment
when comparing complete stable state. `PacingDiagnostics` may differ; command
acceptance/refusal may not become a pacing diagnostic.

The combined time adjudication is therefore a valid foundation for dynamic scheduled-work
expansion, but not a complete global time/command contract.

## 7. API, conflict, diagnostic, and visibility adjudication

### 7.1 Compare-and-take

`least_due_slice(horizon)` plus a full owned comparison fingerprint followed by
`take_least_due_slice(horizon, expected)` is implementable in safe Rust. It prevents an
argument from selecting a nonleast slice, detects membership/payload/status changes, and
has meaningful `NoSliceDue` and `SliceChanged` scheduler-level no-op results. The
fingerprint is intentionally replayable but non-authorizing; an identical re-created live
least slice may be taken because a fresh observation would permit exactly the same act.

The architecture must additionally define collision assumptions/domain encoding and the
engine-level obligation-store check described in §5.8. It must not claim that X-3 is the
only removal method on `Scheduler`, because inherited public `drain_due` remains.

### 7.2 Conflict disposition

The V2 correction is right that conflicted slots:

- occupy scheduler membership and affect the pre-extraction scheduler digest;
- are included in the atomic operational `(due_time,profile_id)` extraction slice;
- are reported with the complete Phase-1 conflict projection;
- do not enter executable scheduled-cohort identity;
- do not consume the executable pacing budget; and
- produce no wave or batch when the slice is all-conflicted.

Those are required clarifications of inherited Phase-1/v3 behavior, not unauthorized
semantic changes. The rejected D-3/D-4 were the unauthorized changes.

The candidate still needs a deterministic combined outcome for a mixed slice (conflict
disposition plus scheduled evaluation/rejection) and executable-only definitions for
deferred diagnostics. After an oversized exception, the loop stops before a later
all-conflicted slice, so the statement that an all-conflicted slice never stops progress
must be narrowed or the loop changed consistently with the frozen exception.

### 7.3 Visibility and observation

Withdrawing CE-8 is correct. A feature-gated test observation can expose post-extraction
pre-wave digests without adding retained state, changing canonical identity, or creating a
production report feedback path. The existing manifest hygiene test can prove that no
workspace production dependency enables `test-support` by default.

It cannot prove all of AT-I32/AT-I46's broader claims. `spark-core` publicly exports the
`scheduler` module and the candidate makes X-1…X-4 `pub`; the Phase-1 engine documentation
itself warns that Rust `pub` is public to everyone. A hypothetical host that directly
depends on `spark-core` can reach these methods. Likewise, after a test seam returns a
generic digest, causal unreadability must follow from evaluator input types, not from the
feature flag alone. Compile probes must target concrete dependency surfaces and types.

## 8. Acceptance-matrix sufficiency

**Oracle verdict:** `INSUFFICIENT_FOR_IMPLEMENTATION_OR_FREEZE`.

The revised AT-I39(A)/(B), AT-I40(a)–(d), AT-I42(a)–(c)/(e)–(k), and AT-I43(d)/(i)
substantially improve the oracle. They would fail the inherited whole-prefix path, directly
distinguish capture before versus after extraction, preserve deferred slots, cover dynamic
horizon expansion and same-time profile cuts, exercise full conflict evidence, and reject
key-only compare-and-take.

The matrix nevertheless cannot serve as the implementation oracle until these defects are
corrected:

1. AT-I45(a) must expect `T` deferred at budget 1, or use budget 2 for the claimed next-
   cohort distinction.
2. AT-I40(e) must expect equal post-extraction pre-wave and batch digests; only conflict
   extraction/reporting differs.
3. AT-I29(d) must be backed by a normative `Φ` representation/update/digest rule, not used
   to discover whether one exists.
4. AT-I42(d)(2) must reflect empty-key scheduling or add a second unequal occupied-key
   claim to create conflict.
5. AT-I44 must split command refusal from WorkKey extraction/retry and compare a rejected
   later wave with its own pre-wave state after prior committed waves.
6. AT-I32 must probe the actual exported engine/host boundary; it cannot require public
   `spark-core` methods to be unreachable.
7. AT-I39(C) needs a direct loop/selection-operation bound; two successful extraction
   attempts do not distinguish a million empty tick iterations.
8. AT-I39(D)'s carry-state negative control must name a concrete carry algorithm and a
   fixture that necessarily diverges; the current universal `Kills` statement is false.
9. AT-I45 must cover executable-only deferred counts and residual all-conflict slices
   after the oversized exception.
10. AT-I44 must cover a mixed slice whose scheduled portion rejects, and define the exact
    order/content of its conflict and rejection reports.
11. Cross-store tests need missing/mismatched/contested obligation-record cases that fail
    before either store mutates, plus proof that extracted executable records remain
    available to evaluation.
12. AT-I46 must separately prove feature absence, report-schema stability, and evaluator-
    type non-reachability; one feature-hygiene test proves only the first.

No test has been implemented or run for the candidate; these are architecture-level oracle
definitions. The word `Kills` does not itself supply discrimination.

## 9. Disposition of original findings F-01 through F-07

| Original finding | Independent V2 disposition | Reason |
|---|---|---|
| **F-01** time/catch-up partition | **PARTLY RESOLVED; FOUNDATIONAL REMAINDER** | A4 horizon expansion and cohort-local time close work-producing scheduled catch-up under their preconditions. Global command ordering after incomplete paced segments and the `Φ` state remain unresolved. |
| **F-02** owned removal capability | **RESOLVED at scheduler level** | The fingerprint is comparison data, not target-selection authority; X-3 always chooses the live least slice. Engine-level extraction still needs §5.8. |
| **F-03** no `now`/not-due source | **RESOLVED** | Explicit horizon makes `NoSliceDue` meaningful and constructible. |
| **F-04** conflicted identity/budget change | **RESOLVED architecturally; oracle premise wrong** | Executable-only identity and budget are restored. Budget-one does not, however, admit `T`; use the corrected fixture in §5.1. |
| **F-05** oracle gaps | **NOT RESOLVED** | Coverage expanded, but multiple required cases assert false outcomes or fail to discriminate. |
| **F-06** canonical report expansion | **RESOLVED in design** | CE-8 is withdrawn for a test-only observation seam, subject to exact structural type tests. |
| **F-07** borrow/lifetime overclaim | **RESOLVED** | The V2 wording correctly uses NLL and concedes that the owned fingerprint is retainable. |

Writer claims that all seven are `RESOLVED` are therefore not accepted.

## 10. Separate verdicts

### 10.1 Architectural verdict

`FOUNDATIONAL_REVIEW_REQUIRED`

The scheduled horizon-expansion and scheduler compare-and-take core is acceptable as a
basis for another correction. The command-time/unfinished-pacing policy and its canonical
frontier are not a bounded extraction detail; they decide which finalized commands execute
and what time stateful rule operations observe. They require authority and a complete R1
state model before a mechanical Phase-2 contract exists.

### 10.2 Acceptance-oracle verdict

`INSUFFICIENT_FOR_IMPLEMENTATION_OR_FREEZE`

The oracle has strong direct tests for the original whole-prefix defect, but its false
budget, digest, replay, rejection, and visibility assertions would reject conforming code
and permit or fail to localize several wrong implementations.

### 10.3 Overall verdict

`V3_F01_FOUNDATIONAL_REVIEW_REQUIRED`

## 11. Smallest coherent correction route

Preserve the current V2 documents unchanged as review history. Do not repair them in
place.

1. Conduct one narrowly scoped foundational adjudication of finalized-command effective
   time relative to scheduled work and pacing. Decide, under Phase-0 authority, whether
   monotonicity is a staging rule, an execution rule, an implicit-drain rule, or a declared
   host-history precondition; prove the choice for incomplete paced segments. Do not cite
   future Gate C3 wording as an already-frozen rule.
2. Either remove `Φ`, SH-1/SH-2, R-8, and the `due_time <= Φ` defense from V3-F01, or define
   one exact persisted frontier: type/owner, initialization, update after every successful,
   no-effect, rejected, command, refusal, mixed, and all-conflicted path, snapshot encoding,
   engine-digest inclusion, restore validation, and equal-digest/same-next-input tests.
3. Retain A4 horizon expansion, `now=scheduled due_time`, dynamic live-least recomputation,
   strictly later checked delayed work, and the completed-segment proof. Restate honestly
   which command histories are covered by the foundational decision.
4. Retain least-live-slice compare-and-take and the full per-slot fingerprint, but define an
   engine-owned cross-store extraction. Preflight scheduler and exact obligation claim sets;
   on any mismatch mutate neither; on success move the full executable records into the
   transient extraction and perform infallible removal under one exclusive engine borrow.
5. Retain executable-only identity and pacing plus whole operational conflict extraction.
   Specify mixed-report order, all-conflict progress after an oversized exception, and
   executable-only deferred diagnostics.
6. Correct AT-I40(e), AT-I42(d)(2), AT-I44, and AT-I45(a); narrow or replace the false
   `Kills`/compile claims listed in §8; add cross-store and later-wave fixtures.
7. Route that separated writer result to a new independent Gate C1 review. Only a contract
   with no unresolved command-time foundation and a discriminating red-first oracle may be
   recommended for Operator acceptance.

This is the smallest coherent route because it preserves the accepted extraction/time-
expansion work and reopens only the command-time state/authority question plus the bounded
oracle/API corrections demonstrated here.

## 12. Supplementary compute

No supplementary external/NIM compute was used in this pass. It is supplementary rather
than dispositive, the writer recorded a bounded gateway-availability failure, and the
material findings above are direct deductions from the pinned documents and live Phase-1
source. The completed benchmark study was not rerun.

## 13. Explicit nonclaims

This review does **not** claim that:

- the V2 candidate, Fable adjudication, or preceding Codex review is edited or replaced;
- horizon expansion or compare-and-take is inherently unsound;
- V3-F01 is closed, accepted, canonical, or frozen;
- Phase-2 architecture outside the bounded issues above is reopened;
- Phase-1 source or semantics should be changed;
- Phase-2 implementation or Phase 3 is authorized;
- Operator acceptance has occurred;
- any production Rust or acceptance test was implemented;
- any G.A.M.E. file, contract, state, or repository was modified;
- executable cross-platform parity is established; or
- supplementary compute has project authority.

## 14. Review deliverables and pre-commit accounting

The review change is bounded to this report and one truthful pointer in
`engineering/PHASE_STATUS.md`. The writer's files and every historical artifact remain
byte-identical. Validation includes the exact writer diff, exact review diff,
`git diff --check`, cited-repository-path existence, no-Rust/no-historical-change checks,
the existing baseline test suite, identical-copy hashing, commit parent/contents, and final
worktree accounting. The resulting review commit is returned to the Operator because a
document cannot include its own commit hash without changing it.

## 15. Operator recommendation

Do not accept or freeze V3-F01 and do not release the Phase-2 implementation writer.
Preserve commit `983a01fd6807b8c627c97d2673a1f40c54cc059c` and this review as additive
history. Authorize only the focused foundational command-time/frontier adjudication and
the bounded correction route in §11, followed by a separated independent review.
