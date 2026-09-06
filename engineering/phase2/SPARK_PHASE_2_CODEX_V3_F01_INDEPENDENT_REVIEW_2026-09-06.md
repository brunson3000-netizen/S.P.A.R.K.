# S.P.A.R.K. Gate C1 — Independent V3-F01 Acceptance Review

**Date:** 2026-09-06
**Reviewer:** Codex, independent adjudicating reviewer
**Repository:** `/home/chromikey/Projects/SPARK`
**Branch:** `phase1-refoundation-v2`
**Writer commit reviewed:** `03daa82032cecc6ed84407b440bc9eab06cbcd69`
**Writer parent:** `5fd556bfad958bda4439560cbfc5f4e537ce375a`
**Verdict:** `V3_F01_FOUNDATIONAL_REVIEW_REQUIRED`

## 1. Executive adjudication

The candidate correctly identifies the original whole-prefix extraction defect and gives
a sound local ordering rule for a **static, pre-scheduled due set**: remove one cohort,
then capture its pre-wave digest while every later cohort remains resident. It is not
ready for Operator acceptance or architecture freeze.

The review found one foundational blocker and several major contract/API/oracle defects.
Most importantly, Theorem P assumes a partition-invariant cohort sequence that Lemma V
does not prove. The inherited record simultaneously requires catch-up partition
equivalence, binds wave-created delayed work to `now + 1`, and permits only one due drain
per canonical barrier. A cohort that creates later work produces different due times and
different cohort membership under one catch-up call versus one-time-step-at-a-time calls.
Resolving that conflict requires an explicit adjudication of logical boundary time and
catch-up barrier expansion across inherited v1/v2/v3 provisions; it cannot be repaired by
editing the extraction API alone.

Separately, the proposed owned selection token is retainable and replayable, can select
any exposed due cohort rather than only the admitted least cohort, and commits only to key
membership rather than slot state. `take_due_cohort` has no `now` input or scheduler clock
from which its promised `CohortNotDue` result can be computed. D-3/D-4 also convert
conflicted slots into evaluation-cohort identity and pacing-budget inputs despite the
inherited v1 budget being over `DrainOutcome::due`; that is a semantic change, not a
required clarification.

No implementation or freeze authority is released.

## 2. Lineage and writer-scope gate

The repository gate passed before substantive review:

| Check | Result |
|---|---|
| Branch | `phase1-refoundation-v2` |
| Current writer HEAD at entry | `03daa82032cecc6ed84407b440bc9eab06cbcd69` |
| Sole parent | `5fd556bfad958bda4439560cbfc5f4e537ce375a` |
| Worktree at entry | clean |
| Writer commit count over baseline | exactly one |
| Writer changed files | exactly the four files listed below |
| Rust/source/test changes | none |
| Phase-1 contract changes | none |
| Historical architecture/review changes | none |
| Writer diff whitespace check | pass |

Exact writer-commit surface:

1. `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_2026-09-06.md` — added.
2. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_2026-09-06.md` — added.
3. `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_2026-09-06.md` — added.
4. `engineering/PHASE_STATUS.md` — fourteen-line candidate pointer added.

Because lineage and scope were materially true, the mission did not stop at the baseline
gate.

## 3. Exact material reviewed

### 3.1 Candidate/control surface reviewed in full

1. `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_2026-09-06.md`
2. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_2026-09-06.md`
3. `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_2026-09-06.md`
4. the candidate subsection added to `engineering/PHASE_STATUS.md`

### 3.2 Governing and inherited architecture/oracles reconciled

1. `engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md`
2. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md`
3. `engineering/phase0/ADR-0002-authority-and-host-acknowledgement.md`
4. `engineering/phase0/ADR-0003-deterministic-clock-rng-and-commit-order.md`
5. `engineering/phase0/ADR-0006-persistence-versioning-and-bounded-provenance.md`
6. `engineering/phase0/PHASE_0_PERFORMANCE_AND_SECURITY_BUDGET_v0.2.md`
7. `engineering/phase0/PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md`
8. `engineering/phase1/SPARK_PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_2026-08-26.md`
9. `engineering/phase2/PHASE_2_FABLE_READINESS_BLUEPRINT_2026-08-26.md`
10. `engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_2026-08-26.md`
11. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_2026-08-26.md`
12. `engineering/phase2/SPARK_PHASE_2_CODEX_ARCHITECTURE_ADVERSARIAL_REVIEW_2026-08-26.md`
13. `engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v2_2026-08-26.md`
14. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v2_2026-08-26.md`
15. `engineering/phase2/SPARK_PHASE_2_CODEX_ARCHITECTURE_V2_REREVIEW_2026-08-26.md`
16. `engineering/phase2/SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md`
17. `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v3_2026-08-26.md`
18. `engineering/phase2/SPARK_PHASE_2_CODEX_FINAL_ARCHITECTURE_V3_CONFIRMATION_2026-08-26.md`

The relevant Phase-1 implementation was inspected directly at
`crates/spark-core/src/scheduler.rs`, including `WorkKey`, conflict evidence,
`Scheduler::schedule`, `Scheduler::drain_due`, slot observation, and
`canonical_state_digest`.

## 4. Findings

| ID | Severity | Finding | Disposition |
|---|---|---|---|
| F-01 | **BLOCKER / FOUNDATIONAL** | Lemma V and Theorem P assume, but do not prove, a partition-invariant cohort sequence when evaluated work enqueues delayed work. Literal v2 §11/AT-I21 `now + 1` semantics produce different due times and membership across catch-up partitions. | Requires inherited logical-time/catch-up boundary adjudication before a new correction candidate. |
| F-02 | **MAJOR** | `DueCohortSelection` is owned, retainable, replayable, key-only, and mintable for every due view entry. Privacy prevents construction but does not prevent replay, later-cohort removal, or same-key slot-state TOCTOU. | Replace with a least-currently-due compare-and-take operation; do not use an owned removal capability as authority. |
| F-03 | **MAJOR** | `take_due_cohort(selection)` has neither `now` nor clock/boundary state, so `CohortNotDue` is not implementable as specified. A valid token minted only from a due view also makes that branch unconstructible for the public oracle. | Pass `now` explicitly and check the live least cohort, or remove the impossible error and redesign the surface coherently. |
| F-04 | **MAJOR** | D-3/D-4 silently change inherited semantics: conflicted slots are made members of scheduled-cohort identity and consume a pacing budget originally defined over `DrainOutcome::due`. | Separate operational slice extraction from executable evaluation-cohort membership; any change to budget/identity needs explicit higher-level adjudication. |
| F-05 | **MAJOR** | The corrected oracle misses the successful counterexamples: token replay, selecting a nonleast due cohort, scheduled-to-conflicted same-key mutation, delayed-enqueue catch-up membership, command-barrier placement, and ordinary rejected-wave post-extraction state. | Add direct red-first cases and correct the false CE-9 traceability row. |
| F-06 | **MINOR** | CE-8 is deterministic and bounded as written, but is not necessary to implement the fix and expands a canonical host-visible report with internal state digests. “Causally inert” is enforced only inside the evaluator; a host can use any returned field to form later input. | Prefer test-only/internal observation or a clearly noncanonical diagnostic; otherwise explicitly version and justify the report surface. |
| F-07 | **MINOR** | §6 says view/selection/extraction are dropped at S6 while §8 relies on the immutable view borrow ending before S2. Rust NLL makes the intended borrow pattern possible, but the normative sequence is internally imprecise. | End the view/entry borrow and materialize only a non-authorizing comparison value before mutable extraction. |

## 5. Counterexamples and falsification attempts

### 5.1 Successful: delayed enqueue breaks the claimed cohort sequence

Start with one scheduled cohort `C@100`. Evaluating C creates delayed work D according to
the frozen rule `D.due_time >= current logical time + 1`, concretized by AT-I21 as
`due_time >= now + 1`. Compare legal non-decreasing histories reaching logical time 101:

```text
U: one catch-up boundary at now=101
   extract/evaluate C@100
   D is created at >=102 and is not due

R: boundary now=100, then boundary now=101
   evaluate C@100; D is created at >=101
   at the next boundary D is due and is consumed/evaluated
```

At time 101 U retains D at a different key/time while R may already have consumed D.
Scheduler state, cohort sequence, reports, batch inputs, and final engine state therefore
differ. This is not repaired by recomputing the view. If “current logical time” instead
means `C.due_time`, D is due at 101 in both runs, but v2's one-drain-per-barrier rule keeps
it out of U's already-pinned drain set while R admits it in the next barrier; the
architecture must define whether one host catch-up call expands into multiple canonical
logical-time barriers. The candidate does not do so.

This counterexample also defeats the induction in Theorem P: Lemma A proves ordering of
whatever cohorts happen to become visible, not equality of cohort membership across
partitions. The proof's scheduler equation silently assumes equal earlier enqueues,
including equal `WorkKey.due_time`, which is the disputed proposition.

### 5.2 Successful: token replay removes newly created work

Mint token T for slice `(100,P)` with key set K; extract K successfully. Phase-1
semantics free drained keys, so K may be scheduled again (the source says typically under
a later occurrence, not exclusively). Recreate the same K with new payload state and
call `take_due_cohort(T)`. T's coordinate and key-only digest still match, so the old
token removes newly scheduled work. Private fields do not prevent replay.

Even without re-creation, a caller can mint tokens for the first and second entries of
one view and invoke the second token first. C-3 validates membership, not that the token
names the selector-admitted least cohort. This is an arbitrary due-cohort removal surface
relative to §6's mandatory order.

### 5.3 Successful: key-only check misses a slot-state transition

Mint T while K is `Scheduled(payload A)`. End the view borrow, call the public
`schedule(K, payload B)`, which changes the same key to `Conflicted{A,B}`, then call
`take_due_cohort(T)`. Count and key digest are unchanged, so
`CohortMembershipChanged` does not fire. Extraction returns a different disposition and
content from that observed at selection. The conforming happy path promises no mutation,
but the API claims typed fail-closed handling and the mission explicitly requires the
TOCTOU gap to be closed; it is not.

### 5.4 Successful: `CohortNotDue` has no information source

The scheduler does not retain logical time. C-3 accepts only `&mut self` and the token.
The token does not contain the boundary `now`; even if it did, a token minted from
`due_cohort_view(now)` necessarily satisfies its own captured cutoff. There is no value
against which `selection.due_time > now` can be tested and no conforming external path to
construct AT-I42(d)'s not-due case.

### 5.5 Successful: conflicted-slot budget changes admission boundaries

Let `max_due_per_cycle=1`. Earliest slice `(100,P)` contains one scheduled key S and one
conflicted key X; `(101,P)` contains one scheduled key T. Under inherited v1 Q4, the
budget counts `DrainOutcome::due`, so the executable cohort size is one. Under D-4 it is
two and triggers the oversized-earliest exception, forces remaining budget to zero, and
defers T. The latency/diagnostic schedule changes. That is observable pacing behavior
and is not an “unchanged” selector. D-3 likewise changes
`scheduled_cohort_identity(S)` when a non-executable conflict X is added, even though
Phase-1 reports X through `DrainOutcome::conflicted` rather than `due`.

Conflicted slots must be included in the **atomic operational extraction slice** so they
do not pollute later scheduler digests. That does not entail including them in the
executable scheduled-cohort identity or its pacing count. An all-conflicted slice can be
consumed and reported as Phase-1 conflict disposition without inventing an evaluation
cohort.

### 5.6 Failed attack: immutable-view/mutable-extraction borrowing

A bounded standalone Rust probe modeled the candidate call shape. Rust 1.98 non-lexical
lifetimes/two-phase borrowing accepted a view entry producing an owned selection followed
by `&mut self` extraction once the immutable references had no later use. There is no
inherent borrow-checker blocker. The surface still fails for authority/replay/TOCTOU
reasons above.

### 5.7 Failed attack: static drain-refinement ordering

For a scheduler that receives no mutation while it is drained, the due prefix is exactly
the concatenation of equal-`(due_time,profile_id)` BTreeMap ranges. Reusing the same
per-slot disposition logic yields the same scheduled vector, conflicted vector (including
truncated evidence), and residual digest as `drain_due(now)`. Lemma D is sound for its
stated null-evaluator/static-state experiment across mixed, all-conflicted, oversized,
and same-time multi-profile states. It does not prove integration refinement when waves
enqueue work, does not authorize D-3/D-4, and does not cure C-3's API defects.

### 5.8 Failed attack: digest captured after extraction for a static due set

For wholly pre-scheduled cohorts and no boundary-changing enqueue, S2 then S3 does close
the original V3-F01 counterexample. AT-I39(a), (b), (d), and AT-I42(e) would fail an
inherited whole-prefix `drain_due` evaluator and would distinguish capture-before from
capture-after extraction. This local success is insufficient for the candidate's
universal theorem and freeze claim.

## 6. Adjudication of the eleven primary questions

### Q1 — Theorem P

**No.** It covers neither membership equality under delayed enqueues and differing
`now` histories nor command-cohort interleavings. Oversized-first and fixed-boundary
pacing are locally ordered correctly, but do not repair the missing induction premise.
“Fixed canonical input history” is not enough because the candidate itself defines a
partition as different non-decreasing `now` call sequences and delayed `WorkKey`s are
derived from those times.

### Q2 — Lemma V and v2 §11

Within one already-defined canonical barrier, v2 §11 plus AT-I21 require wave-created
work to be strictly after that barrier's `now`, and the pinned drain set excludes it.
It is not legitimate to silently weaken this to “after the currently evaluated cohort's
due time.” The candidate's intra-barrier view-stability conclusion is therefore sound
for a fixed `now`; its use of that conclusion across catch-up partitions is not. The
inherited architecture must decide whether one catch-up host call contains one canonical
barrier at final `now` or a deterministic sequence of logical-time barriers. That is the
foundational review required.

### Q3 — view recomputation

For a fixed barrier `now`, conforming wave enqueues are `> now`, so recomputation returns
the residual boundary-entry due sequence. Across legal partitions with different `now`
values, newly scheduled work can change due keys, visibility, membership, and processing
order as §5.1 shows. The candidate overgeneralizes the fixed-boundary lemma.

### Q4 — safe-Rust implementability

The borrow shape is implementable. The authority shape is not acceptable: the owned
token is retainable/replayable, any due entry can mint one, and key-only commitment leaves
a slot-state TOCTOU. A retained token is exactly a small due-work plan/capability even if
it carries no payload.

### Q5 — `take_due_cohort`

Range removal can be total, atomic, bounded by the finite queue, and exactly one
coordinate slice after a preflight scan. The proposed public contract as a whole is not:
`CohortNotDue` lacks `now`, selection need not be the least/admitted live cohort, and
membership validation ignores live slot state. The three claimed no-op failures cannot
therefore establish the required boundary.

### Q6 — conflicted slots

They must participate in the atomic **operational range extraction**, progress, residual
scheduler digest, and their existing conflict report. The inherited record does not
authorize them as executable scheduled-cohort identity members or pacing-budget units.
D-3/D-4 are semantic changes. All-conflicted ranges should be consumed/reported without
a wave or batch; whether they have a new “cohort identity” requires explicit authority,
not clarification by this bounded correction.

### Q7 — Lemma D

The static scheduler refinement is genuine for all Phase-1 slot states, conflict
truncation, oversized ranges, and multiple profiles. Its scope is narrower than the
candidate uses: it proves removal equivalence only under a null evaluator with no
enqueues. It does not prove Theorem P, pacing semantics, or conflicted identity.

### Q8 — CE-8

The two fixed-size digests and one existing cohort digest are deterministic and bounded.
The candidate excludes them from retained stores and internal causal identities. The
surface is nevertheless unnecessary for production conformance: internal/test-only S3
observation can support the oracle. Adding internal state digests to a canonical returned
report changes its schema and gives the host a new observable it may use when creating
later canonical input. CE-8 should be removed, made explicitly noncanonical/test-only, or
versioned and justified through the appropriate interface authority.

### Q9 — CE-9

Treating input extraction as preceding the wave and restoring a failed wave to its
post-extraction S3 snapshot is compatible with inherited drain-before-evaluation behavior.
It yields terminal consumption on commit, semantic-cap rejection, and ordinary wave
rejection; a later reschedule is new canonical input and replay remains deterministic.
The candidate must say this for every rejection class and pin scheduler/obligation-store
cleanup and same-key/next-occurrence retry. The current matrix does not.

### Q10 — AT-I39/AT-I40 and related tests

They are sufficient to kill the inherited whole-prefix drain for static pre-scheduled
cohorts and to distinguish S3 before/after extraction. They strongly cover deferred
residency, scheduled/mixed/all-conflicted extraction, conflict evidence, same-time profile
cuts, and static drain equivalence. They are insufficient for delayed-enqueue catch-up
membership, command barriers, nonleast token use, replay, same-key status TOCTOU,
constructible not-due failure, and CE-9 across ordinary rejection paths. AT-I29's view
compile probe cannot prove that the owned selection is not retained.

### Q11 — supersession map

**No.** The map calls D-3/D-4 “unchanged/clarified” despite their effect on identity and
pacing, claims a non-retained surface while C-2 is an owned retainable plan/capability,
and leaves `CohortNotDue` without a source of `now`. Combined with the unadjudicated
catch-up/boundary-time conflict, it does not yield one mechanical contract.

## 7. Acceptance-matrix sufficiency ruling

`INSUFFICIENT_FOR_IMPLEMENTATION_OR_FREEZE`.

The matrix has a strong direct oracle for the original static extraction/digest bug, but
its claimed coverage exceeds what it can construct or observe. The next matrix must add:

1. one-call catch-up versus stepwise calls where an early cohort schedules work inside
   the final target horizon;
2. explicit canonical-barrier and command-barrier placements;
3. an attempt to extract a nonleast due cohort;
4. replay after successful extraction and exact-key re-creation;
5. same-key scheduled-to-conflicted change between observation and extraction;
6. a constructible and meaningful not-due comparison against explicit `now`;
7. post-extraction scheduler and ObligationStore state for every ordinary wave-rejection
   class, plus retry under the same and next occurrence identities;
8. inherited-versus-proposed pacing counts for scheduled, mixed, and all-conflicted
   slices; and
9. a structural proof that neither a view nor any selection/continuation capability is
   retainable at a stable boundary.

## 8. Supplementary NIM evidence

The established free-authorized MCI/NIM gateway was operational and was used. The
completed benchmark study was not rerun. The first four-task bundle used one fixed full
evidence packet and independent workers. Provider/model health produced one HTTP-200
`openai/gpt-oss-20b` falsification seat, two `moonshotai/kimi-k3` HTTP-429 failures, and
one Kimi timeout. The successful seat exhausted its 8,192-token output budget in internal
reasoning and returned no final response; that raw reasoning is preserved rather than
silently promoted to a finding.

Because the first bundle yielded no usable final response, one bounded fallback panel
was launched with a shorter fixed packet. It seated three independent roles on three
models: `openai/gpt-oss-20b` completed (224.65 s), `moonshotai/kimi-k3` failed with HTTP
429 after its bounded retry, and `nvidia/nemotron-3-ultra-550b-a55b` completed (283.68 s)
but ended at its output length limit. The usable responses independently reinforced the
missing-`now`, replay/oracle, delayed-enqueue catch-up, and conflicted-budget findings.
They did not change the independently reached verdict.

Codex rejected or narrowed unsupported NIM suggestions: a direct Rust 1.98 compile probe
disproved the claimed inherent borrow failure; an owned view/`RefCell` workaround would
weaken the contract; splitting an oversized cohort violates indivisibility; speculative
concurrent drains have no demonstrated safe-Rust composition path; and atomic extraction
should preflight before removal rather than depend on rollback.

All raw responses/reasoning, prompts inside request envelopes, model selection and
fallback provenance, HTTP/failure details, token usage, latency, and telemetry are under:

`engineering/phase2/codex_v3_f01_independent_review_nim_evidence_2026-09-06/`

The evidence summary is
`SPARK_V3_F01_NIM_EVIDENCE_SUMMARY_2026-09-06.md` in that directory. Panel request IDs
are `mci-dev-e9c9b5ca9492` and `mci-dev-680afb02ca2c`.

NIM is supplementary and does not decide this verdict.

## 9. Precise correction instruction

Do not repair the current candidate in place. Preserve it as rejected review history and
open a separated foundational architecture adjudication before another V3-F01 writer
pass:

1. Freeze the meaning of canonical logical boundary time during catch-up. Decide whether
   one host catch-up call deterministically expands into per-logical-time barriers, which
   `now` feeds delayed due-time construction, and exactly when newly scheduled work inside
   the catch-up horizon becomes eligible. Re-prove R-038 for work-producing cohorts and
   command barriers, not only static pre-scheduled cohorts.
2. Once that is settled, state the boundary-entry due set and cohort sequence as exact
   functions of canonical history. Rebuild Lemma V, Lemma A, and Theorem P without
   assuming equal enqueue keys or equal membership.
3. Replace the owned removal capability with a least-currently-due compare-and-take API,
   for example a crate-private operation taking explicit `now` and an expected full live
   slice fingerprint. It must always target the live least due `(due_time,profile_id)`;
   the expected value may guard atomicity but must not grant target-selection authority.
   Bind the comparison to key identities **and slot state/content**, scan before mutation,
   and return typed byte-identical failures.
4. Keep scheduled and conflicted entries in one atomic operational extraction range, but
   retain inherited executable-cohort identity and `DrainOutcome::due` pacing semantics
   unless a separately authorized review explicitly changes them. Specify mixed and
   all-conflicted disposition without inventing an effect batch.
5. Remove CE-8 from the production canonical report in favor of internal/test-only
   capture, or route any report-schema addition through explicit interface/versioning
   authority. Retain CE-9 only after its full rejection/retry oracle is added.
6. Add the matrix cases in §7 and require them red against both the inherited whole-prefix
   path and plausible wrong cohort-granular implementations.

Because item 1 adjudicates an inherited cross-version logical-time contract, the next
step is the Fable-reserved non-converging/foundational review path in Gate C1, not Phase-2
implementation.

## 10. Explicit nonclaims

This review does **not** claim that:

- the writer candidate is accepted, canonical, or frozen;
- V3-F01 is closed;
- all Phase-2 architecture outside this correction is reopened;
- Phase-1 source or semantics should be edited;
- Phase-2 implementation or Phase 3 is authorized;
- Operator acceptance has occurred;
- any production Rust or test was implemented;
- CE-9 is inherently incompatible with drain-before-evaluation semantics;
- Lemma D is false in its stated static/null-evaluator scope;
- NIM evidence has project authority;
- executable Windows/Android or cross-platform digest parity is established.

## 11. Verdict and Operator recommendation

`V3_F01_FOUNDATIONAL_REVIEW_REQUIRED`

**Operator recommendation:** do not freeze or accept this candidate and do not release
the Phase-2 implementation writer. Preserve the candidate and this review as additive
history. Authorize one bounded foundational adjudication of catch-up logical boundary
time and delayed-work eligibility, then a new separated V3-F01 correction writer pass
covering the API and oracle defects above, followed by another independent review.
