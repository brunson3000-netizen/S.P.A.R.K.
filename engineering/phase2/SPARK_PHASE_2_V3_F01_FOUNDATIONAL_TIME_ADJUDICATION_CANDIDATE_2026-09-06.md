# S.P.A.R.K. Phase 2 — V3-F01 Foundational Logical-Time Adjudication Candidate

**Date:** 2026-09-06
**Agent:** Claude Code (Fable 5.1), Gate C1 foundational architecture adjudicator
**Status:** **CANDIDATE — AWAITING A NEW V3-F01 WRITER PASS AND INDEPENDENT REVIEW.**
This document adjudicates one foundational question. It is **not** accepted, **not**
frozen, **not** canonical, and authorizes **no** implementation. It does not repair,
edit, or overwrite the rejected V3-F01 correction candidate.
**Form:** bounded additive adjudication over the inherited v1/v2/v3 Phase-2 record and
the Phase-0 ADRs, feeding a separated Opus V3-F01 correction pass and a later
independent Codex review. No historical artifact is rewritten or deleted.
**Companion report:**
`SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_REPORT_2026-09-06.md`

**Nonclaims (stated first because they bound everything below):** this adjudication
does not claim that V3-F01 is closed, that the rejected candidate is repaired, that
Phase-2 architecture is frozen or accepted, that the Phase-2 implementation writer is
released, that Phase 3 is authorized, that Operator acceptance has occurred, that any
cross-platform runtime parity exists, or that G.A.M.E. convergence state
`ARCHITECTURE_READY` has been reached. No Rust and no test was written.

---

## 1. Exact baseline and governing authority

### 1.1 Repository state verified before writing

| Item | Verified value |
|---|---|
| Repository | `/home/chromikey/Projects/SPARK` |
| Branch | `phase1-refoundation-v2` |
| HEAD at entry | `513c3982d5b9cd14f86ec07369662f3a178f1d95` (Record independent V3-F01 foundational review) |
| Convergence baseline | `5fd556bfad958bda4439560cbfc5f4e537ce375a` — verified ancestor of HEAD |
| Rejected writer candidate | `03daa82032cecc6ed84407b440bc9eab06cbcd69` — verified ancestor of HEAD; sole parent `5fd556b` |
| Lineage | `5fd556b` → `03daa82` → `513c398` (HEAD) |
| Worktree at entry | clean (`git status --porcelain` empty) |
| Phase 0 | CLOSED / PASS — not reopened |
| Phase 1 | CLOSED (`PHASE_1_CLOSED` at reviewed HEAD `20a1c66`) — not reopened |
| Phase 2 | architecture-only; controlling verdict `PHASE_2_ARCHITECTURE_V3_REVISE`; latest Gate C1 verdict `V3_F01_FOUNDATIONAL_REVIEW_REQUIRED` |
| Phase 3 | NOT AUTHORIZED |
| Gate | C1 (`engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` §5), Fable-reserved foundational path |

No material contradiction between repository state and the mission was found.

### 1.2 Governing authority, in precedence order

1. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md` §18.1, §18.3, §19.4, §28.
2. `engineering/phase0/ADR-0003-deterministic-clock-rng-and-commit-order.md` §2, §8, §14,
   §15.
3. `engineering/phase0/PHASE_0_REQUIREMENT_COVERAGE_MATRIX.md` rows R-037, R-038, R-041,
   R-044 (all FROZEN).
4. `engineering/phase0/PHASE_0_PERFORMANCE_AND_SECURITY_BUDGET_v0.2.md` catch-up row
   ("scale with scheduled aggregate obligations, not reconstructed microhistory").
5. Phase-1 closed source, consumed unmodified: `crates/spark-core/src/clock.rs`
   (`LogicalTime`, `LogicalClock::advance_to`), `crates/spark-core/src/scheduler.rs`
   (`WorkKey`, `Scheduler::schedule`, `Scheduler::drain_due`, `canonical_state_digest`),
   `crates/spark-core/src/timeline.rs` (`SemanticCommandEnvelope.effective_time`).
6. `engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_2026-08-26.md` (v1) Q3,
   Q4, Q7, §5, §6, §8.
7. `engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v2_2026-08-26.md` (v2) §5.1,
   §6, §11, §12; matrix v2 AT-I20, AT-I21, AT-I22.
8. `engineering/phase2/SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md` (v3) §3,
   §4.1, §4.2, §4.5, §4.6, §4.7, §7; matrix v3 AT-I20, AT-I21, AT-I39, AT-I40, AT-I41.
9. `engineering/phase2/SPARK_PHASE_2_CODEX_FINAL_ARCHITECTURE_V3_CONFIRMATION_2026-08-26.md`
   §7 (V3-F01).
10. `engineering/phase2/SPARK_PHASE_2_CODEX_V3_F01_INDEPENDENT_REVIEW_2026-09-06.md`
    (F-01, §5.1, §6 Q1–Q3, §9 item 1) — the review that requires this adjudication.
11. The rejected candidate
    `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_2026-09-06.md`
    and its matrix — read as rejected history, not as authority.

Where a lower item conflicts with a higher one, the higher governs. The adjudication
below is entirely a consequence of items 1–5; items 6–8 are retained, clarified, or
superseded as stated in §9.

---

## 2. The conflict being adjudicated

### 2.1 The three inherited statements

1. **Catch-up partition equivalence (Phase 0, FROZEN).** R-038 requires
   "ten-days-once versus one-day-ten-times" equality. ADR-0003 §15 requires that
   scheduled occurrence identity derive from persisted logical occurrence indexes,
   "never batching/worker/catch-up chunks". v2 §6.1, v3 §4.5, and v3 §4.7 restate this
   as canonical byte-identity across drain-call partitions.
2. **Strictly-later delayed work (v2 §11, first bullet).** Every obligation created or
   converted by a wave carries `due_time ≥ current logical time + 1`. Matrix AT-I21
   (v2, carried into v3) concretizes this as `due_time ≥ now + 1`.
3. **One drain per boundary (v2 §11, second bullet).** "A canonical boundary drains due
   work at most once; nothing scheduled during a barrier's execution is drained within
   that same barrier, by construction." AT-I21 adds "the barrier's drain set is pinned".

### 2.2 The undefined term

None of the three statements defines **whose logical time** "current logical time" or
`now` is, nor whether a **canonical boundary** is the host's catch-up call or a
logical-time position inside it. Phase 1 supplies `LogicalTime` and a host-advanced
`LogicalClock` but retains no time inside the scheduler; `Scheduler::drain_due(now)`
takes `now` as an argument and treats it as an inclusive **horizon** (every slot with
`due_time <= now`). The rejected candidate read `now` as the host call's argument
(§6.3, Lemma V) and acknowledged the alternative reading in its §14 item 3.

### 2.3 The mandatory Codex counterexample, reproduced

One scheduled cohort `C@100`. Evaluating `C` creates delayed work `D` under statement 2.
Both runs are legal non-decreasing host histories that reach logical time 101 with no
command between them.

```text
Reading H ("current logical time" = host call argument):
  U: advance(101)            C evaluated with now=101, D.due >= 102, D resident at 101
  R: advance(100); advance(101)
                             C evaluated with now=100, D.due >= 101; D evaluated at 101
  -> D's WorkKey (due_time is a key field), scheduler digest, cohort sequence,
     reports, and final engine state differ.  Statement 1 is violated.

Reading C ("current logical time" = C.due_time), with "boundary" = host call:
  U: advance(101)            C evaluated with now=100, D.due >= 101;
                             statement 3 keeps D out of the already-pinned call
  R: advance(100); advance(101)
                             D is drained at the second call
  -> D consumed in R, resident in U.  Statement 1 is violated.
```

Under either inherited reading the three statements are jointly unsatisfiable for a
work-producing cohort. This is the F-01 blocker. It is independent of the extraction
API: no cohort-granular removal surface can repair it, because the disagreement is
about which `WorkKey` `D` receives and when `D` is eligible.

### 2.4 What is settled and must not move

- The cohort is the equal-`(profile_id, due_time)` slice (v3 §3.1); no budget splits it.
- Cohorts are consumed in ascending `(due_time, profile_id)` order (v3 §3.1–§3.2).
- `max_due_per_cycle` is pacing and semantics-neutral; deferred cohorts stay resident
  and untouched (v2 §6.2, v3 §3.2(d), v3 §3.4).
- Semantic caps are epoch-bound; over-cap disposition is v2 §6.3.
- Command-barrier rule evaluation forms its own cohort with the finalized command
  barrier identity (v2 §6.1, v3 §4.2).
- The v1 heartbeat descriptor is retired from canonical use (v3 §4.5).
- The pre-wave engine digest of a cohort must be captured with that cohort extracted
  and every later or deferred cohort resident and byte-identical (Codex V3-F01 §7.4).
- Phase-1 `WorkKey`, `schedule`, `drain_due`, conflict, encoding, and digest semantics
  are consumed unmodified.
- No zero-delay same-barrier recursion (R-037, v2 §11).

---

## 3. Alternatives considered

| ID | Rule | Disposition | Decisive reason |
|---|---|---|---|
| **A1** | One host catch-up call is one canonical barrier at the final horizon `T`; every cohort in it evaluates with `now = T`; delayed work gets `due ≥ T + 1`. | **REJECTED** | The `WorkKey.due_time` of every obligation created during catch-up becomes a function of the host's chunking. That is a direct violation of ADR-0003 §15 ("never … catch-up chunks") and R-038, both FROZEN at Phase 0. It also makes v1 Q3's effect `at` time, v1 Q7 decay evaluation time, and v1 §6 cooldown expiry checks chunk-dependent. Not correctable inside Phase 2. |
| **A2** | Per-cohort time (`now = C.due_time`), but the host call is one barrier whose drain set is pinned at call entry; created work with `due ≤ T` waits for the next host call. | **REJECTED** | Created work's eligibility becomes a function of how many host calls occur, so U retains what R consumed (§2.3 second branch). Violates R-038. Also leaves "past-due" resident work whose eligibility depends on call count rather than logical time. |
| **A3** | Per-cohort time; one host call; the due view is recomputed after every cohort and admits any resident slot with `due ≤ T`, including work created during the call. | **NOT ADOPTED AS STATED** | Mechanically this produces the same canonical results as A4. As a statement it contradicts the literal v2 §11 second bullet (created work is drained "within the same barrier" if barrier means host call), leaves "barrier" and `now` undefined, and therefore cannot carry the partition-equivalence proof. It is A4 without the vocabulary A4 needs. |
| **A4** | **Horizon expansion with per-barrier canonical time.** One host catch-up call `advance(T)` is a horizon, not a time. It deterministically expands into the ascending sequence of canonical logical-time barriers at every resident due time `τ ≤ T`, including due times created during the call. Each scheduled cohort evaluates with `now = its due_time`; each command cohort with `now = its effective_time`. | **CHOSEN** | The only alternative under which statements 1, 2, and 3 of §2.1 are simultaneously true, once "canonical boundary" is read as a logical-time barrier or command barrier rather than a host call. Requires no Phase-1 change, no new retained store, no new causal primitive, and no change to cohort identity, pacing semantics, conflict semantics, or host authority. |
| **A5** | Forbid multi-tick catch-up; the host must advance one logical unit per call. | **REJECTED** | Contradicts blueprint §18.3 (host may select deterministic catch-up), the budget's catch-up row, and R-038's own test, which presupposes a ten-day single call. Also makes cost proportional to elapsed ticks rather than scheduled obligations. |
| **A6** | Execute created work whose due time lies inside the horizon immediately, inside the creating cohort's barrier. | **REJECTED** | Zero-delay recursion by another name; violates R-037 and v2 §11 first bullet. |

Two sub-decisions inside A4 were also weighed:

| Sub-decision | Chosen | Alternative rejected | Reason |
|---|---|---|---|
| Pacing budget scope under expansion | one `max_due_per_cycle` budget **per host call**, spent across the expanded barrier sequence in order | one budget per expanded barrier | The budget exists to bound work per host call (latency). Per-barrier budgets would let a ten-day call do ten budgets of work. Both are semantics-neutral (pacing is causally inert), so this is a clarification of "drain cycle", not a semantic choice. Matches v3 §3.2's "for one drain cycle". |
| Canonical time of a command cohort | the command's finalized **`effective_time`** (an existing canonical envelope field, ADR-0003 §2, in the finalized-history digest) | the engine `LogicalClock` frontier | The frontier is classified in Phase 1 as a "monotonic host value" and is outside the v1 §8 engine digest. Reading it during canonical evaluation would make it R1 retained state and force a v1 §8 digest change — a larger correction. `effective_time` is already finalized history; the Phase-0 rereview framed commands as "processed by `(effective_time, input_ordinal)`". |

---

## 4. Chosen canonical rule (A4), stated normatively

### 4.1 Vocabulary

- **Horizon.** The `LogicalTime` argument `T` of a host catch-up call. It is an inclusive
  bound on which scheduled work may be consumed by that call. It is **not** the canonical
  time of anything evaluated during the call.
- **Canonical time of a cohort.** For a scheduled cohort, its `due_time` (the first
  `WorkKey` field, shared by every member). For a command cohort, the command's
  finalized `effective_time`. This value is what every inherited use of "current logical
  time", `now`, or `at` **means** during that cohort's evaluation.
- **Canonical logical-time barrier.** The maximal run of consecutive scheduled cohorts
  sharing one `due_time` in the global cohort sequence. It is a grouping label with **no
  canonical identity of its own**; every canonical value (cohort identity, pre-wave
  digest, wave index, batch digest, emission identity) is per cohort. A barrier may span
  more than one host call under pacing without any canonical consequence.
- **Command barrier.** Unchanged (ADR-0003 §8, §14; v3 §4.2): one finalized command,
  one command cohort, executed at its position in canonical input history.
- **Canonical input history.** The ordered sequence of host operations, each either a
  finalized command execution or a catch-up call `advance(T)` (indicative name; the
  Phase-2 entry that advances the host horizon and consumes due work). Non-decreasing
  `T` is enforced by the frozen Phase-1 `LogicalClock::advance_to`; a backward call is
  rejected without canonical mutation and is not history.
- **Catch-up segment.** A maximal run of consecutive `advance` items between two
  command executions (or history ends).

### 4.2 The rule

1. **Expansion.** One host catch-up call `advance(T)` consumes, in ascending
   `(due_time, profile_id)` order, every resident cohort whose `due_time ≤ T` — including
   cohorts that become resident because earlier cohorts in the same call created them —
   subject only to the pacing budget. The call therefore expands deterministically into
   as many canonical logical-time barriers as there are distinct resident due times
   `≤ T` reached, and into none at due times where nothing is resident. Iteration is
   over resident due times, never over integer ticks.
2. **Canonical time.** Every scheduled cohort is evaluated with `now = cohort.due_time`.
   Every command cohort is evaluated with `now = command.effective_time`. The horizon
   `T` and the `LogicalClock` frontier are never supplied to rule evaluation, decay
   closed-form catch-up, cooldown expiry checks, effect `at` stamping, occurrence
   allocation, random addressing, or delayed-work construction.
3. **Delayed work.** Every obligation created or converted by cohort `C` carries
   `due_time ≥ C.canonical_time + 1`, computed with checked integer arithmetic; overflow
   at `u64::MAX` is a typed atomic wave rejection (the AT-I8 exhaustion pattern). The
   obligation's `WorkKey` is therefore a function of `C` and the rule set only.
4. **Eligibility.** Work created during a catch-up call with `due_time = δ` is eligible
   exactly when the global cohort sequence reaches barrier `δ`. If `δ ≤ T` and the
   pacing budget permits, that is inside the same host call; otherwise it is resident
   and consumed by the first later call whose horizon is `≥ δ` and whose budget reaches
   it. Eligibility is a function of logical time and budget consumption only — never of
   the number or partition of host calls.
5. **Barrier pinning (the corrected v2 §11 second bullet).** The membership of barrier
   `τ` is fixed at the moment the sequence first reaches `τ`: it is every resident slot
   with `due_time = τ`. Because every cohort at `τ` creates work at `≥ τ + 1`, nothing
   created during barrier `τ` can join barrier `τ`. Nothing is drained twice; nothing
   created at `τ` is drained at `τ`. This is the statement v2 §11 intended, now with
   "barrier" meaning a logical-time barrier rather than a host call.
6. **Pacing.** One `max_due_per_cycle` budget per host call, applied incrementally by the
   v3 §3.2 selector to the least resident cohort with `due_time ≤ T` after every stable
   boundary. Admission stops at the first cohort that does not fit (after at least one
   admission) or when nothing `≤ T` is resident. The oversized-earliest exception
   (§3.2(c)) is evaluated against the call's first cohort only. Every unadmitted cohort,
   whether pre-existing or created during the call, is left resident and byte-identical.
   `PacingDiagnostics` is emitted per host call with its frozen fields; deferred counts
   range over resident cohorts with `due_time ≤ T` at call end.
7. **Clock frontier.** On completion (including completion with deferred work) the
   `LogicalClock` frontier is `T`. A later call with the same `T` is legal and resumes
   deferred work. No canonical evaluation reads the frontier; it is host-facing
   admission state for backward-call rejection only.
8. **Commands.** A command cohort executes at its position in canonical input history,
   with canonical time `effective_time`. Command barriers never interleave inside a
   catch-up call's expansion; a call is atomic with respect to history ordering even when
   it expands into many barriers. A command barrier does not itself consume scheduled
   work (see §6).

### 4.3 The mandatory counterexample under the rule

```text
State: {C@100}.  C's rule creates D at delay 1.

U: advance(101)
   barrier 100: extract C; pre-wave digest (scheduler = ∅); evaluate C with now=100;
                commit; D scheduled with due_time = 101 (WorkKey fixed by C and rule)
   barrier 101: D is resident and 101 <= 101; extract D; pre-wave digest (scheduler = ∅);
                evaluate D with now=101; commit; anything D creates has due >= 102
   frontier := 101

R: advance(100)
   barrier 100: identical to U's barrier 100
   frontier := 100
   advance(101)
   barrier 101: identical to U's barrier 101
   frontier := 101
```

Same `WorkKey` for `D`, same cohort sequence, same pre-wave digests, same reports, same
final state. If `D`'s delay were 5, `D@105` would be resident and untouched at 101 in
both runs. If `C` and `D`'s rules created work at 101 for two profiles, barrier 101
would hold two cohorts `(101, P1)`, `(101, P2)` in both runs, in that order.

---

## 5. Deterministic event and barrier sequence

For one catch-up call `advance(T)` on engine state `S`, with epoch-bound budget
`B = max_due_per_cycle ≥ 1`:

```text
A0  if T < frontier: reject (typed, no canonical mutation); the call is not history.
A1  frontier := T;  r := B;  admitted := 0
A2  c := least resident cohort by (due_time, profile_id) with c.due_time <= T
    if none: goto A6
A3  admit c  iff  c.work_key_count <= r  or  admitted == 0        (v3 §3.2(b)(c))
    if not admitted: goto A6                                       (v3 §3.2(d): defer)
A4  extract exactly c (atomic, scheduled and conflicted slots of the slice)
    capture pre-wave engine digest                                  (V3-F01 §7.4)
    evaluate c with now = c.due_time; waves 0..n; commit or reject atomically
    (v2 §5.1, §6.3, §7); every created obligation has due_time >= c.due_time + 1
A5  STABLE BOUNDARY.  admitted += 1;  r := (exception fired ? 0 : r - c.work_key_count)
    goto A2
A6  emit PacingDiagnostics for this call; return.
```

Properties that follow directly:

- **Finite.** Each iteration consumes at least one resident slot; created work is at a
  strictly later due time and bounded by `max_enqueue_per_wave`; due times are bounded by
  `T`. The loop terminates.
- **Cost model.** Barriers exist only at resident due times, so a call over an idle
  span costs nothing per tick (budget catch-up row honored).
- **No retained continuation.** At A6 the only state is resident slots plus the frontier.
  A2 recomputes from canonical state; nothing from a prior iteration survives.
- **Same shape as the rejected candidate's S0–S5 loop**, with two corrections: the view
  is bounded by the horizon `T` (supplying the `now` Codex F-03 found missing), and the
  evaluation `now` is `c.due_time`, never `T`.

The **global cohort sequence** of a canonical input history is obtained by concatenating,
in history order, one command cohort per finalized command and the A2–A5 admissions of
each catch-up call. It is a deterministic function of (initial state, history, activated
artifact) and of nothing else.

---

## 6. Command-barrier treatment

1. A finalized command executes at its own barrier at its history position (ADR-0003
   §8 item "ascending ordinal order through individual stable command barriers").
   Its cohort identity is v3 §4.2; its canonical time is `effective_time`.
2. Delayed work created by a command cohort carries `due_time ≥ effective_time + 1`.
3. A command barrier evaluates its own cohort only. It does **not** drain scheduled
   work. ADR-0003 §14's "deterministic due/zero-delay evaluation" is read as the
   command's own zero-delay evaluation within its barrier; consumption of scheduled
   work is the exclusive business of catch-up calls. If a later, separately authorized
   decision ever attaches an implicit scheduled drain to a command barrier, the only
   rule-compatible form is an implicit `advance(effective_time)` executed **before** the
   command cohort, so that §4.2 applies unchanged; nothing here authorizes that.
4. Command barriers and catch-up calls are ordered by canonical input history alone. The
   engine never reorders a command against scheduled cohorts by comparing
   `effective_time` with due times, and never splits a catch-up call to interleave a
   command inside its expansion.
5. Consequently, a command that lands **between** two catch-up calls of a paced or
   stepwise run is a **different canonical input history** from a run in which the same
   command lands after a single call reaching the same horizon (v2 §6.2: "new canonical
   input arriving between drains is new history, not a quota artifact"). Partition
   equivalence (§8) is asserted within catch-up segments and across histories with the
   same command subsequence and the same per-segment maximal horizon.
6. `effective_time` monotonicity relative to the frontier is a host-history property.
   The engine does not enforce it in this adjudication (§11, U-1). A past-dated command
   is deterministic: its created work is dated from its own `effective_time` and becomes
   eligible at the next catch-up call, in ascending order with everything else resident.

---

## 7. Delayed-work eligibility rule (consolidated)

| Situation | Eligibility |
|---|---|
| Work created by scheduled cohort at `τ`, `due = δ ≥ τ + 1`, `δ ≤ T` (current call's horizon), budget not exhausted before barrier `δ` | Consumed in the **same host call**, at barrier `δ`, in ascending order with every other resident cohort at `δ` (same profile → same cohort; other profile → later cohort at `δ`). |
| Same, but the call's budget is exhausted before barrier `δ` | Resident, untouched, byte-identical; consumed by a later call at horizon `≥ δ` when the budget reaches it. |
| `δ > T` | Resident; consumed by the first call with horizon `≥ δ` whose budget reaches it. |
| Work created by a command cohort at `effective_time = e`, `due = δ ≥ e + 1` | Resident; consumed by the first later catch-up call with horizon `≥ δ` whose budget reaches it. Never consumed by a command barrier. |
| Work created at barrier `τ` targeting a due time at which a cohort was **deferred** by pacing | Impossible for the same `τ` (created work is `≥ τ + 1`). For a later deferred `(τ', P)` with `τ' ≥ τ + 1`, the created key joins that slice; membership is fixed when the sequence reaches `τ'`, identically in every partition. |
| Created key collides with a resident key (same full `WorkKey`, different payload) | Phase-1 `schedule` poisons the slot order-independently; the conflicted slot is extracted with its slice and reported per Phase-1/v2 §6.3 semantics. Identical in every partition because creation precedes barrier `δ` in every partition. |

---

## 8. Proof sketch — partition equivalence for work-producing cohorts

Fix an activated artifact and epoch-bound caps. Let `Run(S, T)` denote the unbudgeted
A1–A6 procedure on state `S` with horizon `T` (budget large enough that A3 always
admits). Let `Run_B` denote the budgeted procedure.

**Lemma 1 (least-cohort invariance).** Let the cohort sequence of `Run(S, T2)` be
`c_1, …, c_k`. For every `i`, `c_i` is the least resident cohort with `due ≤ T2` in the
state reached after `c_1 … c_{i-1}` have been processed, and `c_i.due_time` is
non-decreasing in `i`.
*Proof.* A2 selects the least resident cohort; created work is at strictly greater
`due_time` than its creator (§4.2 item 3) and so cannot sort before the current cohort;
deferred and untouched cohorts keep their keys. ∎

**Lemma 2 (prefix property).** For `T1 ≤ T2`, the cohorts of `Run(S, T2)` with
`due ≤ T1` form a prefix `c_1 … c_j`, and `Run(S, T1)` processes exactly that prefix,
producing the same state after each cohort.
*Proof.* By induction on `i ≤ j`. The states before `c_i` agree (hypothesis). Each
cohort is evaluated with `now = c_i.due_time` (§4.2 item 2) — a property of the cohort,
not of the horizon — so the evaluation, its commit or rejection, and every created
`WorkKey` agree. Hence the resident sets after `c_i` agree. In `Run(S, T1)`, A2 selects
the least resident cohort with `due ≤ T1`; since `c_{i+1}` is the least with `due ≤ T2`
and `c_{i+1}.due ≤ T1`, it is also the least with `due ≤ T1`. When `Run(S, T1)` stops,
every resident cohort has `due > T1`, so it has processed exactly `c_1 … c_j`. ∎

**Theorem P′ (catch-up composition).** For `T1 ≤ T2`,
`Run(Run(S, T1), T2) = Run(S, T2)`, cohort by cohort, including every pre-wave engine
digest, cohort identity, wave index, candidate-set digest, committed effect block (with
its `at` time), obligation record, occurrence allocation, cooldown write, canonical
semantic report, and the final engine digest.
*Proof.* By Lemma 2, `Run(S, T1)` reaches the state after `c_j`. Applying Lemma 1 to the
remaining sequence, `Run(·, T2)` from that state selects `c_{j+1}, …, c_k` in order with
identical states. Per-cohort canonical values are functions of (cohort members, state
after the previous cohort, `now = due_time`, artifact) — v3 §4.1, §4.5, §7 CE-2 — and
the pre-wave engine digest is captured with exactly `c_i` extracted and everything else
resident (V3-F01 §7.4, to be realized by the next writer's extraction surface). ∎

**Corollary P′-n (ten-days-once ≡ one-day-ten-times, R-038).** Iterating Theorem P′
over any non-decreasing horizon sequence ending at `T` gives the same result as one
call at `T`. This holds for arbitrary work-producing cohorts, because Lemma 2 never
assumes that created keys or cohort membership are equal a priori — it derives them
from equal states and equal `now`. This is the induction premise Codex F-01 found
missing in the rejected candidate's Theorem P.

**Theorem P′-pacing (deferred-then-completed ≡ unbudgeted, v2 §6.2).** `Run_B` with any
budget, repeated at the same or later horizons until nothing `≤ T` is resident, yields
the same cohort sequence and per-cohort values as `Run(S, T)`.
*Proof.* A3 admits a prefix of the A2 selections and never reorders; deferred cohorts
are untouched and remain least (Lemma 1); a resumed call starts at A2 from the same
state `Run` would be in. The oversized exception only changes *how many* cohorts a call
admits, never *which is next*. `PacingDiagnostics` differ by construction and are in no
canonical value (v3 §3.4). ∎

**Theorem P′-commands.** Two canonical input histories with the same finalized command
subsequence and, for each catch-up segment, the same maximal horizon and complete
drainage before the next command, produce identical global cohort sequences and
canonical values.
*Proof.* Segment by segment: Theorem P′ and P′-pacing give equal segment results; a
command cohort is a function of (finalized command, state at its position,
`now = effective_time`), all equal. ∎

**Same-time multi-profile check.** Cohorts `(τ, P1)`, `(τ, P2)` in one barrier: `P1`'s
created work is `≥ τ + 1`, so barrier `τ`'s membership is fixed at entry in every
partition; a pacing cut between them followed by resumption leaves `(τ, P2)` least and
untouched. Equal under all partitions.

**What the proof does not cover (by design).** Histories that differ in command
placement relative to catch-up calls (different history, §6 item 5); different epochs
or caps (different declared behavior, v2 C-11); `PacingDiagnostics` (noncanonical).

---

## 9. Supersession and retention map

| Source statement | Disposition | Effect |
|---|---|---|
| Blueprint §18.1 "host advances a monotonic integer simulation clock; S.P.A.R.K. evaluates only work that is due" | **RETAINED** | The host advance is the horizon; "due" is `due_time ≤ T`. |
| Blueprint §18.3 deterministic catch-up; budget catch-up row | **RETAINED** | Expansion iterates resident due times, not ticks. |
| ADR-0003 §15 "scheduled occurrence identity … never batching/worker/catch-up chunks"; "quota/yield changes latency, not semantics" | **RETAINED (governing)** | The reason A1 and A2 are rejected. |
| ADR-0003 §8/§14 command barriers in ascending ordinal order | **RETAINED**; "due/zero-delay evaluation" **CLARIFIED** | Command barrier evaluates its own cohort; no implicit scheduled drain (§6 item 3). |
| ADR-0003 §2 `effective_time` | **RETAINED; CLARIFIED** | Canonical time of a command cohort. |
| R-037, R-038, R-041, R-044 | **RETAINED** | R-038's test is now precisely defined (§8 Corollary P′-n). |
| Phase-1 `LogicalClock::advance_to` (equal or later accepted; backward rejected) | **RETAINED** | Frontier semantics of §4.2 item 7; equal-horizon calls resume deferred work. |
| Phase-1 `Scheduler::drain_due(now)` | **RETAINED, unmodified** | Its `now` was always a horizon. Rejected-candidate Lemma D (static drain refinement) remains valid in its stated scope. The Phase-2 evaluator does not call it (unchanged from Codex V3-F01 §7.4). |
| Phase-1 `LogicalClock` classified "monotonic host value", outside the v1 §8 engine digest | **RETAINED** | The rule makes no canonical evaluation read it. See U-2. |
| v1 Q3 effect record `at` logical time | **CLARIFIED** | `at` = the cohort's canonical time; therefore partition-independent and safe inside the effect hash. |
| v1 Q3 heartbeat barrier identity | **RETAINED AS RETIRED** (by v3 §4.5) | The "heartbeat drain" concept is replaced by the catch-up call; a per-call label may live only in `PacingDiagnostics`. |
| v1 Q4 "Deferred work is simply due at the next drain" | **RETAINED** | "Next drain" = next catch-up call (horizon is already `≥` the deferred due time). |
| v1 Q4 depth overflow "converts … into scheduled work at the next due boundary" | **CLARIFIED** by v2 §11 | Strictly later canonical time. |
| v1 Q6 no batch-scheduling API | **RETAINED** | Nothing here adds one. |
| v1 Q7 closed-form decay "(between `updated_at` and now)" and AT-I22 chunk invariance | **CLARIFIED** | `now` = cohort canonical time. Chunk invariance is the formula property; evaluation time is fixed by this rule. |
| v1 §6 cooldown expiry "removed deterministically when next consulted during evaluation" | **CLARIFIED** | Consulted against the cohort's canonical time. |
| v2 §6.1 cohort per due time; "distinct due times within one drain are successive cohorts" | **RETAINED** | "Drain" = host call; the sequence may include created due times. |
| v2 §6.2 pacing; "new canonical input arriving between drains is new history" | **RETAINED; "drain cycle" CLARIFIED** | One budget per host call across the expansion (§3 sub-decision). |
| v2 §6.3 over-cap disposition | **RETAINED** | Unchanged. |
| v2 §11 first bullet "due_time ≥ current logical time + 1" | **RETAINED; CLARIFIED** | "Current logical time" = the creating cohort's canonical time. |
| v2 §11 second bullet "a canonical boundary drains due work at most once; nothing scheduled during a barrier's execution is drained within that same barrier" | **RETAINED under the barrier reading; SUPERSEDED under the host-call reading** | "Canonical boundary/barrier" = a logical-time barrier or command barrier (§4.1). Reading it as the host call is inconsistent with R-038 and is withdrawn. |
| v2/v3 AT-I21 "`due_time ≥ now + 1`"; "the barrier's drain set is pinned and the converted key is absent from it"; "provably not executed within the same barrier" | **RETAINED; CLARIFIED** | `now` = barrier canonical time; "pinned" = §4.2 item 5; "same barrier" = same logical-time barrier. Any reading as "not executed within the same host call" is **SUPERSEDED**. |
| v2 C-11 (cross-depth-budget equality only in isolation) | **RETAINED** | Unaffected. |
| v3 §3.1 "the cohort sequence of a boundary is its cohorts in ascending order" | **RETAINED; CLARIFIED** | The sequence of a catch-up call is determined incrementally (A2) and includes created cohorts `≤ T`. |
| v3 §3.2 selector (a)–(d) | **RETAINED** | Applied incrementally, one budget per call. |
| v3 §3.3 progress | **RETAINED** | Created work is strictly later; every admitted cohort is consumed; unchanged derivation. |
| v3 §3.4 `PacingDiagnostics` fields and binding rules | **RETAINED** | Per host call; deferred counts over resident cohorts `≤ T` at call end. |
| v3 §3.5, §4.1, §4.2, §4.3, §4.4, §4.6, §5, §7 CE-1…CE-6 | **RETAINED** | Unchanged. |
| v3 §4.5 partition-independence claim | **RETAINED**; derivation **SUPPLIED** | Given this rule plus a correct cohort-granular extraction, §8 derives it. |
| v3 §4.7 row "drain-call partition / catch-up chunking" | **RETAINED** | Justification is now §8. |
| Rejected candidate §6.3 / Lemma V ("`now` = boundary's host argument; intra-boundary enqueues are `> now`") | **SUPERSEDED — inconsistent** | This is reading H of §2.3. |
| Rejected candidate §14 item 3 (per-cohort `due_time` reading, "D-2 must be revisited") | **ADOPTED IN SUBSTANCE** | D-2's recomputation is retained and is now *required* because created work `≤ T` must be admitted at its barrier. |
| Rejected candidate §6.1 S0–S5 loop shape | **COMPATIBLE**, not accepted | §5 A1–A6 is the corrected shape; all other rejected-candidate content (C-1…C-3, D-3…D-6, CE-8, CE-9, Theorem P, matrix) remains rejected per Codex F-02…F-07 and is not repaired here. |
| Codex V3-F01 review F-01, §5.1, §6 Q1–Q3, §9 item 1 | **CONFIRMED AND ANSWERED** | Q1 of the mission: expansion. Q2: cohort canonical time. Q3: at barrier `δ`, same call if `δ ≤ T` and budget permits. |
| Codex V3-F01 review F-02…F-07 | **NOT ADJUDICATED** | Remain open for the next writer. |

No inherited statement was found inconsistent **after** clarification except the
host-call reading of v2 §11's second bullet and the rejected candidate's Lemma V, both
withdrawn above.

---

## 10. Downstream constraints for the next V3-F01 writer

These are binding on the correction candidate that follows, so that the foundational
decision is implementable. They constrain; they do not design the API.

1. **Horizon is explicit.** Any cohort-selection or extraction operation takes the
   horizon `T` as an explicit argument. "Not due" means "the least resident cohort has
   `due_time > T`". This supplies the information source Codex F-03 found missing, and
   is the only `now` the scheduler surface ever sees.
2. **Evaluation `now` is the cohort's.** The evaluator passes `now = cohort.due_time`
   (scheduled) or `now = effective_time` (command) to rule evaluation, decay, cooldowns,
   effect `at`, and delayed-work construction. It must be structurally impossible to
   pass `T`: the horizon must not be reachable from the evaluation context.
3. **Recompute after every stable boundary.** The next cohort is always the live least
   resident cohort with `due ≤ T`. No call-entry snapshot of the due set, no pinned
   plan, no continuation cursor. This retains the rejected candidate's D-2 with its
   justification corrected.
4. **Pre-wave capture with exactly the current cohort extracted** (Codex V3-F01 §7.4,
   unchanged). Deferred and created cohorts are resident and byte-identical at capture.
5. **Least-currently-due compare-and-take** (Codex §9 item 3) is the extraction shape
   to pursue; the expected-value guard binds key identity and slot state; it never
   grants target selection. This adjudication adds only that the operation is bounded
   by `T`.
6. **Iterate resident due times, not ticks.** A horizon over an idle span is `O(1)`
   in barriers.
7. **Checked delayed-time arithmetic.** `due_time = canonical_time + delay` uses checked
   `u64` addition; overflow rejects the wave atomically with a typed error.
8. **Pacing per host call.** One budget spans the expansion; the oversized exception is
   judged on the call's first cohort; `PacingDiagnostics` is per call. Do not add fields.
9. **Command cohorts.** `now = effective_time`; no scheduled drain at a command barrier;
   no interleaving inside a call. Do not add an `effective_time` admission rule (U-1) in
   the V3-F01 pass.
10. **No clock read.** Neither the scheduler surface nor the evaluator reads
    `LogicalClock`; the frontier is set to `T` at call completion and consulted only for
    backward-call rejection.
11. **Restate the proofs in the §8 form.** Replace the rejected candidate's Lemma V /
    Lemma A / Theorem P with Lemma 1, Lemma 2, Theorem P′, P′-pacing, and P′-commands,
    over the global cohort sequence as a function of history. Do not assume equal
    created keys; derive them.
12. **Matrix obligations (minimum).** AT-I39 case (iv) must include at least one
    work-producing cohort whose created work lands inside the final horizon (the §4.3
    fixture, plus a delay-5 variant that lands outside), asserting equality of every
    per-cohort value across one-call, per-due-time, and paced partitions. Add: a pacing
    cut inside a multi-profile barrier followed by resumption after created work joins
    a later slice; effect `at` / decay / cooldown evaluation-time equality across
    partitions; a command placed after equal-horizon segments (equal) versus between
    stepwise calls (documented as distinct history, determinism asserted); a
    `CohortNotDue` case constructed with an explicit horizon; and the u64 overflow
    rejection. These are in addition to the Codex §7 list, which stands.

---

## 11. Unresolved risks and explicit nonclaims

### 11.1 Unresolved risks

- **U-1 — `effective_time` versus frontier.** The engine does not require a finalized
  command's `effective_time` to be `≥` the current frontier or `≥` the previous command's
  `effective_time`. Past-dated commands are deterministic under §6 item 6 but create
  past-due obligations. Whether to add an ingress admission rule is an ADR-0003/timeline
  question outside this bounded adjudication and must not be silently assumed either way
  by the next writer.
- **U-2 — Clock frontier and R1.** Two engines with equal engine digests but different
  frontiers respond differently to a backward `advance` (one rejects, one does not).
  This is pre-existing Phase-1/v1 §8 classification ("monotonic host value"), not
  introduced here; the chosen rule keeps every canonical evaluation independent of the
  frontier. Snapshot/restore must carry the frontier as ADR-0003 §15 already requires
  ("snapshots represent complete stable boundaries"). Whether the frontier should enter
  the engine digest is flagged for the reviewer, not decided.
- **U-3 — Long horizons under pacing.** A very long catch-up with dense self-
  rescheduling work may need many equal-horizon host calls. Cost is bounded by created
  work, not ticks; semantics are unaffected. Host-side call policy is a G.A.M.E.
  integration concern (Gate C3/C4), not a canonical one.
- **U-4 — ADR-0003 §14 reading.** §6 item 3 reads "due/zero-delay evaluation" at a
  command barrier as the command's own evaluation. If the Operator or reviewer holds the
  other reading, the only rule-compatible form is stated in §6 item 3; adopting it
  would be a separate decision.
- **U-5 — F-02…F-07 remain open.** This adjudication supplies the foundation; the
  extraction authority shape, TOCTOU binding, conflicted-slot identity/pacing (D-3/D-4),
  CE-8, CE-9, and the oracle gaps are for the next writer and reviewer.
- **U-6 — No supplementary compute was used.** The counterexample analysis and the
  proof sketch are closed-form derivations from the frozen record; an external panel was
  judged unlikely to change the derivation within the mission's bounded deadline. The
  independent Codex review remains the adversarial check.

### 11.2 Explicit nonclaims

This adjudication does **not** claim that:

- V3-F01 is closed, or that the rejected candidate is repaired or accepted;
- Phase-2 architecture is frozen, accepted, or re-verdicted (the controlling verdicts
  remain `PHASE_2_ARCHITECTURE_V3_REVISE` and `V3_F01_FOUNDATIONAL_REVIEW_REQUIRED`);
- Phase-2 implementation or Phase 3 is authorized;
- Operator acceptance or architectural freeze has occurred;
- any Phase-1 source, semantic, encoding, conflict, authority, or determinism invariant
  is reopened or changed;
- any pacing budget value, executable-cohort identity, conflict semantic, or host
  authority boundary has changed;
- the v1 §8 engine digest composition has changed;
- any production Rust or test was written;
- cross-platform runtime or digest parity is established;
- G.A.M.E. has been modified or that any G.A.M.E. state is asserted.

---

## 12. Verdict of this pass

`FOUNDATIONAL_TIME_ADJUDICATION_CANDIDATE_READY`

- Foundational question: **answered by the A4 rule (§4)**, subject to the next writer
  pass and independent review.
- V3-F01 status: **OPEN**.
- Phase-2 architecture: **NOT frozen**, **NOT accepted**.
- PHASE_2_IMPLEMENTATION_AUTHORIZATION: **NO**
- PHASE_3_AUTHORIZATION: **NO**
- Phase 1: **CLOSED and not reopened.**
