# S.P.A.R.K. Phase 2 — V3-F01 Command-Time Adjudication Candidate

**Date:** 2026-09-06
**Agent:** Claude Code (Fable 5.1), Gate C1 foundational architecture adjudicator
**Status:** **CANDIDATE — AWAITING A NEW V3-F01 WRITER PASS AND INDEPENDENT REVIEW.**
Not accepted, not frozen, not canonical; authorizes no implementation. V3-F01 remains
**OPEN**.
**Scope:** exactly the command-time question the independent V2 review (V2-01, V2-02,
§5.3, §5.4, §5.5, §6.2, §11 items 1–2) returned to the foundational path. Nothing else
is redesigned. The reviewed successes are retained verbatim (§2).
**Form:** bounded additive adjudication over the foundational time adjudication
(`379f8dc…`), the V2 candidate (`983a01f…`), and the independent V2 review
(`8d0dba9…`). No historical artifact is rewritten.
**Companion report:**
`SPARK_PHASE_2_V3_F01_COMMAND_TIME_ADJUDICATION_REPORT_2026-09-06.md`

**Nonclaims, first.** This document does not claim that V3-F01 is closed, that any prior
candidate is repaired or accepted, that Phase-2 architecture is frozen, that Phase-2
implementation or Phase 3 is authorized, that Operator acceptance has occurred, that the
amendments proposed in §9 are enacted, or that any cross-platform parity exists. No
Rust and no test was written.

---

## 1. Baseline and lineage

| Item | Verified value |
|---|---|
| Repository / branch | `/home/chromikey/Projects/SPARK` / `phase1-refoundation-v2` |
| HEAD at entry | `8d0dba92845d9a0dde08dca7c25ec161ca85822e` (independent V2 review) |
| Reviewed V2 candidate | `983a01fd6807b8c627c97d2673a1f40c54cc059c` — sole parent of HEAD |
| Foundational time adjudication | `379f8dc355125e6bca09801e4a66b0b35ec9720c` |
| Lineage | `5fd556b` → `03daa82` → `513c398` → `379f8dc` → `983a01f` → `8d0dba9` |
| Worktree at entry | clean |
| Phase 0 / 1 | CLOSED, not reopened |
| Phase 2 | architecture-only; `PHASE_2_ARCHITECTURE_V3_REVISE`; latest Gate C1 verdict `V3_F01_FOUNDATIONAL_REVIEW_REQUIRED` |
| Phase 3 | NOT AUTHORIZED |

Governing authority for this adjudication, in order: blueprint §18.1, §19.4, §28;
ADR-0003 §1, §2, §5, §6, §8, §11, §13, §14, §15; ADR-0005 §9; budget §3 items 2–4 and
§4; R-037/R-038/R-044; Phase-1 `crates/spark-core/src/clock.rs` and
`crates/spark-core/src/timeline.rs`; v1 §8 (R1/R2 law; derived-index row); v2 §6.2,
§11; v3 §3.2, §3.4, §4.2. The Gate C3 bullet "monotonic logical execution point" in
`engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` is **not** cited as authority
anywhere below; the V2 review was right that it is a future checklist item.

---

## 2. Retained without change (the reviewed successes)

Horizon expansion over resident due times; scheduled-cohort evaluation at
`now = due_time`; dynamically recomputed least-due processing after every stable
boundary; strictly later, checked delayed work; scheduler least-slice compare-and-take
with the full per-slot fingerprint; executable-only cohort identity and pacing; and
`test-support`-gated pre-wave digest observation. Every statement below composes with
these and changes none of them.

---

## 3. The question, sharpened by the review

The V2 candidate answered "what happens when a finalized command follows unfinished
scheduled processing" with a refusal keyed to a canonical time frontier `Φ`. The review
showed two things that any answer must now satisfy, and they are mutually reinforcing:

1. **Pacing neutrality (budget §3 item 2, ADR-0003 §15).** Whether a finalized command
   executes, what it observes, and where it sits in the canonical sequence must not
   depend on `max_due_per_cycle`. Review §5.4: after `advance(20)` with `A@10`, `B@20`
   resident, a command at 21 must be treated identically at budget 1 (B deferred) and
   budget 2 (B consumed).
2. **The retained-state law (v1 §8 R1).** Any value that decides future canonical
   behavior must be in the engine digest and reconstructible from committed state.
   Review §5.3: two engines with equal digests, equal host frontier 100, having consumed
   a no-effect cohort at 10 versus 20, must respond identically to a command at 15.

From (1): the value that gates a command may depend only on **host inputs** (horizons,
finalized envelopes), never on **progress** (which cohorts happened to be consumed).
From (2): whatever that value is, it is canonical state and must be committed. `Φ`
failed both: it was progress, and it was uncommitted.

There is a third constraint the V2 candidate identified and the review did not dispute:
stateful operations (closed-form decay, cooldown expiry) read `now` against per-cell
`updated_at`; if a cohort ever evaluates at a canonical time earlier than a time already
committed into the cells it reads, the frozen arithmetic extrapolates backwards. The
answer must make that unreachable by construction, not by a check.

---

## 4. Alternatives

| ID | Rule | Disposition | Decisive reason |
|---|---|---|---|
| **ALT-R** | V2's SH-1/SH-2: canonical refusal keyed to progress frontier `Φ` | **WITHDRAWN** | Pacing-causal (§5.4); `Φ` uncommitted and unreconstructible (§5.3); no present authority (§5.5). |
| **ALT-N** | Execute a finalized command immediately at its host-history position; scheduled backlog stays resident | **REJECTED** | The command observes budget-dependent state (budget 1: before `B`; budget 2: after `B`). Violates budget §3 item 2 directly. |
| **ALT-I** | A separate command-execution operation that first drains scheduled work `≤ e` **without budget**, then runs the command | **REJECTED** | Unbounded work in one host call; violates blueprint §19.4 "bound work per cycle and defer overflow visibly". |
| **ALT-D** | A separate command-execution operation that drains `≤ e` **under budget**, returns a non-canonical "backlog pending, retry" result when incomplete, and runs the command only when nothing `≤ e` remains | **VIABLE, NOT CHOSEN** | Correct and pacing-neutral, but needs two host operations, a deferral result type, and an interaction rule stopping `advance(T)` at the next pending command's time. ALT-U delivers the same canonical sequence with one operation and no deferral type. |
| **ALT-T** | Make host time advances themselves timeline commands so every time input is finalized history and every frontier is derivable | **REJECTED for this pass** | Elegant, but timelines are per profile epoch (ADR-0003 §1) while the record's drain horizon is global (v3 §3.1 orders cohorts across profiles at one due time). Reconciling per-profile time with a global clock is outside this bounded question. Recorded as a possible future direction, not as a defect. |
| **ALT-L** | Admit or canonically refuse each command at finalization; keep an explicit pending-command list store | **REJECTED** | Needs a list store rather than a cursor, and a refusal emitted at finalization is a command barrier occurring out of ordinal order (ADR-0003 §8). Larger than ALT-U with no counterexample demanding it. |
| **ALT-U** | **Unified time-ordered processing.** Finalized commands are pending inputs at their `effective_time`; the existing catch-up call processes scheduled slices and pending commands in one ascending canonical-time order under one budget. Admission of a command into the finalized stream requires `effective_time ≥ max(F, C)` where `F` is the host horizon and `C` the maximum effective time already finalized; failure is a **non-canonical, retryable** staging/fence result in the ADR-0003 §6 shape. | **CHOSEN** | Satisfies §5.4 (the command is reached only after every slice `≤ e`, in every budget), §5.3 (the gating value `F` is a host input, equal in both engines, and is committed), and the backwards-evaluation constraint by construction. Adds exactly two scalar state items, both forced by demonstrated counterexamples, and no canonical refusal class. |

Sub-decisions inside ALT-U:

| Sub-decision | Chosen | Alternative | Reason |
|---|---|---|---|
| Equal canonical time, same profile | scheduled slice at `t` before command at `t` | command first | "Time has reached `t`" means obligations due at `t` are done; matches the inclusive `due_time ≤ horizon` semantics of the frozen `drain_due`. Both are deterministic; this is a decision, recorded as such. |
| Cross-profile order at equal time | `(time, profile_id, kind)` with scheduled `<` command | `(time, kind, profile_id)` | Keeps each profile's own sequence contiguous, extending v3's `(due_time, profile_id)` primary order rather than cutting across it. |
| Budget weight of a command cohort | one executable unit | zero | Zero would permit unbounded commands per call (§19.4). Pacing is semantics-neutral, so the weight cannot change any canonical value. |
| Where effective-time admission is enforced | engine-level preflight at **staging** and at **fence**, both non-canonical and retryable | canonical refusal at execution | Execution-time checks against any frontier are either pacing-causal (progress) or refuse legitimately ordered commands (horizon raised by a later call before the loop reached them). Admission-time checks use only host inputs. |

---

## 5. Chosen rule (ALT-U), stated normatively

### 5.1 Canonical state added, defined completely

| | **`F` — horizon frontier** | **`X` — execution cursor** |
|---|---|---|
| Owner | engine; realized by the Phase-1 `LogicalClock` unchanged | engine; one per profile timeline epoch |
| Type | `LogicalTime` | `Ordinal` — the next finalized ordinal not yet executed |
| Initialization | the engine's declared genesis time (Phase-1 `LogicalClock::new(start)`; default `ZERO`) | the timeline's frontier ordinal at engine genesis (nothing pending) |
| `advance(T)` | `T < F` → typed non-canonical rejection (`BackwardClockAdvance`, Phase-1, unchanged); else `F := T` **at call entry**, before any processing, whether or not the call completes | unchanged |
| successful scheduled cohort | unchanged | unchanged |
| no-effect scheduled cohort | unchanged | unchanged |
| rejected wave (any class R-1…R-5, R-7) | unchanged | unchanged for scheduled cohorts; for a command cohort whose barrier completes with a rejection or an R-7 refusal, `X := X + 1` |
| mixed or all-conflicted slice | unchanged | unchanged |
| command cohort executed | unchanged | `X := X + 1` at that command's stable boundary |
| pacing deferral (call ends with work `≤ F` resident, or a pending command not reached) | unchanged (`F` is the horizon, not progress) | unchanged |
| staging / fence | unchanged | unchanged (admission does not execute) |
| epoch reset (ADR-0003 §11) | unchanged | unchanged; finalized-pending commands survive a reset because they are finalized |
| Digest | **added to `engine_state_digest`** (amendment A-1, §9) | **added to `engine_state_digest`** (amendment A-2, §9) |
| Snapshot / restore | included; restored verbatim | included; restore validates first-ordinal-of-epoch `≤ X ≤` finalized frontier `+ 1`, else typed restore failure |
| Equal-state / same-next-input | equal digests ⟹ equal `F` ⟹ identical acceptance of the next `advance`, identical staging/fence admission | equal digests ⟹ equal `X` and (via the timeline digest) equal pending commands ⟹ identical next input consumed |

**`C` — finalized time ceiling** is **not** state. For a profile timeline it is the
maximum `effective_time` over every finalized command in that timeline's canonical
history, a pure function of the timeline digest. An implementation may cache it as a
derived index under the v1 §8 derived-index row, with the AT-H3-style recomputation
invariant. Nothing else time-valued is retained anywhere.

### 5.2 Admission of a command into the finalized stream

At the engine level, before a command is handed to the Phase-1 timeline:

- **Staging preflight.** If `effective_time < max(F, C)` the engine returns the
  retryable, non-mutating logical result `EFFECTIVE_TIME_BEHIND_FRONTIER { F, C }`. No
  slot is consumed, nothing is staged, nothing is history — exactly the ADR-0003 §6
  `NOT_IN_ADMISSION_WINDOW` shape. Because the check precedes staging, the timeline
  never sees the envelope.
- **Fence preflight.** Before submitting a fence to the timeline, the engine checks every
  envelope in the range in ordinal order, updating a local ceiling `c := max(c, e)` as it
  goes, starting from `c = max(F, C)`. If any envelope has `e < c`, the whole fence is
  returned as `EFFECTIVE_TIME_BEHIND_FRONTIER` — atomic, nothing promoted, not history
  (ADR-0003 §8 rule 8 shape). Otherwise the fence is submitted unchanged.

Both checks read only host inputs (`F` from advance calls, `C` and the envelopes from
finalized/staged history). Neither depends on any budget, any cohort outcome, or any
progress value. This is what makes command admission **pacing-neutral by construction**.

**Consequence: a finalized command always satisfies `e ≥` every horizon requested
before its finalization and `e ≥` every earlier finalized effective time.** Within a
timeline, finalized effective times are non-decreasing in ordinal order, so ADR-0003
§8's ordinal execution order and canonical-time order coincide.

**When the precondition is unmet.** The engine is never corrupted and never needs the
host to repair anything for its own correctness; the command simply does not enter
history. The host must resubmit a corrected envelope. If the host staged a command and
then advanced `F` past its `effective_time` before fencing, the staged slot can no longer
be finalized; replacing that ordinal's payload would poison it (ADR-0003 §4), so the
frozen recovery is the ADR-0003 §11 sequencer epoch reset at a stable boundary. That is
the host contradicting its own clock, and ADR-0003 already prescribes the remedy. The
staging preflight exists to make this rare; it does not make it impossible, and this
candidate says so.

### 5.3 The unified processing loop (extends adjudication §5 / V2 §7 without changing scheduled semantics)

For one host call `advance(T)` with epoch-bound budget `B = max_due_per_cycle ≥ 1`:

```text
U0  if T < F: typed non-canonical rejection; not history.
U1  F := T;  r := B;  admitted := 0;  exception_fired := false
U2  if exception_fired: goto U6
    s := least resident scheduled slice with due_time ≤ T          (X-1, unchanged)
    k := for each profile timeline, the command at its cursor X, if finalized
         and effective_time ≤ T
    n := least of {s, k…} by key (time, profile_id, kind)  where kind: scheduled=0 < command=1
    if none: goto U6
U3  w := (n is scheduled) ? n.executable_count : 1
    admit iff  w ≤ r  or  admitted == 0   (v3 §3.2(b)(c) unchanged; a command is never oversized)
    otherwise goto U6                      (v3 §3.2(d): defer, untouched; a pending command stays pending)
U4  if n is scheduled: exactly V2 §7 A4 (extract with obligation records; conflicts reported;
                       cohort evaluated with now = due_time; commit or reject)
    if n is command:   command cohort at now = effective_time, identity v3 §4.2,
                       waves per v2 §5.1; commit, reject, or R-7 refuse;  X := X + 1
U5  STABLE BOUNDARY.  admitted += (w > 0);  r := exception_fired ? 0 : r − w;  goto U2
U6  due-slice summary (executable-only); PacingDiagnostics; return.
```

Properties:

- **A command is reached only after every scheduled slice with time `< e`, and after
  slices at time `= e` for its profile.** This is the ordering that makes §5.4 hold in
  every budget: the budget decides only in which call the command is reached.
- **Global canonical time is non-decreasing** along the entire cohort sequence with no
  frontier read: scheduled slices are least-first and created work is strictly later
  than its creator; a command's `e` is `≥ F` at admission `≥` every horizon under which
  scheduled work was consumed before it, and `≥ C ≥` every earlier command; nothing
  resident with time `< e` remains when it is reached. Hence `now ≥ updated_at` at every
  decay/cooldown evaluation, by construction. No defense check is needed and none is
  specified.
- **Temporary deferral versus canonical refusal, distinguished.** A pending command not
  reached in a call is *deferred*: nothing is emitted, nothing is decided, `X` is
  unchanged, and any later call with `T ≥ e` reaches it. There is **no canonical command
  refusal** in this contract. The only non-canonical rejections are backward `advance`
  and `EFFECTIVE_TIME_BEHIND_FRONTIER`; the only canonical refusal touching commands is
  the pre-existing v1 §5 obligation execution refusal R-7, which is an outcome of an
  executed barrier.
- **Bounded per call.** At most `B` executable units plus one oversized exception plus
  zero-cost conflicted slices bounded by residency.
- **State across calls is exactly `F`, `X`, and the canonical stores.** No deferral
  record, no partial-command state, no pending list: the pending set is derivable as
  the finalized ordinals `≥ X`.

### 5.4 Reading of ADR-0003 §14

"Stable snapshot → direct canonical ingress → deterministic due/zero-delay evaluation →
… → stable boundary → next finalized ordinal" is satisfied per finalized command: the
command's ingress is its finalization; the due work at times `≤ e` is evaluated as the
scheduled cohorts that precede it in the loop; the zero-delay evaluation is the command
cohort; each has its own stable boundary; the next finalized ordinal follows. Whether
one labels the preceding scheduled cohorts as "inside" the command barrier is
presentation; every canonical value is per cohort (v3 §4.5).

---

## 6. Traces for the mandatory counterexamples

### 6.1 Review §5.4 — `advance(20)`, then command at 21; `A@10`, `B@20`

Initial: `F = 0`, `X = k`, scheduler `{A@10, B@20}`, nothing finalized after `k−1`.

```text
Budget 1                                     Budget 2
advance(20): F := 20                         advance(20): F := 20
  U2 n = A@10  (w=1 ≤ 1) → evaluate A        U2 n = A@10 (w=1 ≤ 2) → evaluate A;  r = 1
  r = 0;  U2 n = B@20, w=1 > 0, admitted=1   U2 n = B@20 (w=1 ≤ 1) → evaluate B;  r = 0
  → defer; return  (B resident)              U2 none ≤ 20 → return
stage/fence cmd@21: 21 ≥ max(F=20, C)        stage/fence cmd@21: 21 ≥ max(20, C)
  → finalized; pending at ordinal k          → finalized; pending at ordinal k
advance(21): F := 21                         advance(21): F := 21
  U2 n = B@20 (time 20 < 21) → evaluate B    U2 n = cmd@21 → command cohort;  X := k+1
  r = 0;  U2 n = cmd@21, w=1 > 0 → defer     return
advance(21): F = 21
  U2 n = cmd@21 (admitted=0) → command;  X := k+1
```

Canonical sequence in both: `A, B, cmd@21`. Command admission: identical (decided by
`F=20`, `C`). Command effects: identical (evaluated against the state after `A` and `B`
in both). Canonical ordering: identical. Only the number of host calls and
`PacingDiagnostics` differ. **Pacing changed latency, not semantics.**

### 6.2 Review §5.3 — no-effect cohorts at 10 versus 20; equal stores; `F = 100`; command at 15

Engine A consumed a no-effect cohort at 10; engine B one at 20; both then reached
`F = 100`; every store, `X`, and `C` are equal; engine digests (now including `F`, `X`)
are equal.

```text
stage cmd@15 in A:  15 < max(F=100, C)  → EFFECTIVE_TIME_BEHIND_FRONTIER, not staged
stage cmd@15 in B:  15 < max(F=100, C)  → EFFECTIVE_TIME_BEHIND_FRONTIER, not staged
```

**Which stored state distinguishes the histories?** None, and none is needed. The only
value that decides the outcome is `F`, a host input equal in both, and it is committed
to the digest, so equal digests imply equal outcomes exactly as R1 demands. The "10
versus 20" difference is not retained anywhere because it is not behavior-relevant: had
the command been admitted, its evaluation would read only per-cell `updated_at` and
ledger entries, all equal in both engines. The V2 `Φ` was wrong precisely because it
retained a distinction that no future behavior depends on, and did so outside the
digest.

### 6.3 Two further checks the rule must pass

- **Command finalized before a horizon it lies within.** `F = 40`; fence `cmd@50`
  (50 ≥ 40 ✓); `advance(100)`. Loop: scheduled `≤ 50`, then `cmd@50`, then scheduled
  `51…100`. The horizon rising to 100 at U1 does not refuse the command, because
  admission was decided at fence time. An execution-time check would have refused it;
  that is why admission is at ingress.
- **Two commands out of time order.** `F = 40`; fence `[k@60, k+1@55]`: fence preflight
  computes `c = 60` after `k`, then `55 < 60` → whole fence rejected, nothing promoted.
  Ordinal order and time order can therefore never disagree among finalized commands.

---

## 7. Equivalence, restated

**Precondition PE-A′ (history skeleton).** Compared runs have the same initial state,
the same finalized command stream, the same `F` at each fence, and the same final
horizon. Within those constraints, the partition of `advance` calls and the budget are
free.

**Claim.** Under PE-A′, the global cohort sequence — scheduled slices and command
cohorts interleaved — and every per-cohort canonical value are identical, compared by
sequence position; and when both runs have drained everything `≤` the final horizon,
their engine states (including `F`, `X`) are byte-identical.

**Why PE-C (segment completion) is no longer required for commands.** The V2 review
correctly noted that the earlier theorem covered a command only after a fully drained
segment. Under ALT-U a command's position is fixed by time, not by how much of the
segment a particular call happened to finish, so an incomplete paced segment followed by
a fence yields the same sequence as a complete one. PE-C remains relevant only to the
equal-prefix statement about engine state at a given host-call index, exactly as V2
§10.3 stated it.

**What is still not claimed.** Different command streams or different `F` at a fence
are different histories (v2 §6.2, retained). Equal state at equal call index under
different budgets is not claimed. `PacingDiagnostics` differ by design.

---

## 8. Supersession map

| Source statement | Disposition |
|---|---|
| V2 §4.5 SH-1, SH-2, `Φ`, execution-door refusal, `due_time ≤ Φ` defense; V2 R-8; V2 CE-10′'s refusal clause | **WITHDRAWN** |
| V2 §4.4 "a command barrier does not drain scheduled work; commands never interleave inside a call's expansion" | **SUPERSEDED** by §5.3: commands are consumed inside `advance` calls in time order; scheduled slices `≤ e` precede them |
| V2 §10.1 PE-B (supported history) | **WITHDRAWN**; replaced by admission-time enforcement (§5.2), which is not a precondition on compared runs |
| V2 §10.1 PE-C for command equivalence | **NARROWED** to the equal-prefix state statement (§7) |
| V2 §10.3 equal-prefix versus completed-horizon, and the equal-call-index non-claim | **RETAINED** |
| V2 §5 X-1…X-4, §6, §7 A4 extraction, §8 R-1…R-7, §9, §11 | **RETAINED** (subject to the review's V2-03…V2-10 corrections, §10) |
| Adjudication §4.1 "no canonical evaluation reads the frontier" | **RETAINED for evaluation; CLARIFIED**: `F` gates admission and is therefore canonical, committed state |
| Adjudication §4.2 item 7 "frontier := T at completion" | **SUPERSEDED**: `F := T` at call entry |
| Adjudication §4.4 / §6 items 3–6 (command barrier drains nothing; never interleaves; past-dated commands allowed) | **SUPERSEDED** by §5 |
| Adjudication §8 P′-commands with PE-C | **STRENGTHENED**: PE-C dropped for commands (§7) |
| Adjudication §11 U-1 (`effective_time` monotonicity), U-2 (frontier and R1), U-4 (ADR-0003 §14 reading) | **RESOLVED** by §5.2, §5.1, §5.4 |
| v1 §8 engine digest composition | **AMENDMENT PROPOSED** (A-1, A-2), not enacted |
| v1 §8 derived-index row | **APPLIED** to `C` |
| ADR-0003 §5 item 5 (command validation), §8 fence rules | **AMENDMENT PROPOSED** (A-3), not enacted; timeline code unchanged |
| ADR-0003 §6 retry-never-eviction; §8 rule 8; §11 epoch reset | **RETAINED** and reused as the shapes of §5.2 |
| ADR-0003 §13 (convenience call excludes execution); ADR-0005 §9 | **RETAINED**; execution is host-driven through `advance` |
| ADR-0003 §14 | **READ** per §5.4 |
| ADR-0003 §15; budget §3 items 2–4; blueprint §19.4 | **RETAINED (governing)** |
| v3 §3.2 selector | **CLARIFIED**: a command cohort weighs one executable unit and is never oversized |
| v3 §3.4 `PacingDiagnostics` field list | **RETAINED**; no field added; pending commands are observable through the read-only cursor `X`, not through diagnostics |
| v2 §6.2 "new canonical input arriving between drains is new history" | **RETAINED and made exact**: "between drains" means at a different `F` |
| Gate C3 "monotonic logical execution point" | **NOT AUTHORITY**; noted only as consistent with §5.2 for the future contract |
| Phase-1 `LogicalClock` classification "monotonic host value" (Phase-1 admission architecture §5.1) | **SUPERSEDED for Phase 2**: it becomes a committed engine store; Phase-1 code unchanged |

---

## 9. Amendments requiring Operator decision (proposed, not enacted)

- **A-1 (v1 §8).** Add `horizon frontier F` to `engine_state_digest`:
  `H("engine_state" ‖ … ‖ cooldown-ledger digest ‖ F)`. Justification: R1; review §5.3;
  adjudication U-2. No pinned value moves (no implementation exists).
- **A-2 (v1 §8 table).** Add row `ExecutionCursor` — canonical; one `Ordinal` per profile
  timeline epoch; digest-committed; AT-G discrimination/equivalence obligations from
  first commit. Justification: ADR-0003 §8/§13 already separate finalization from
  execution; review V2-02 requires the state to be explicit.
- **A-3 (ADR-0003 addendum, Phase-2 layer).** Add to §5 item 5's validation and to §8 as
  rule 9: "at the engine boundary, an envelope with `effective_time < max(F, C)` is
  returned `EFFECTIVE_TIME_BEHIND_FRONTIER`, retryable and non-mutating; a fence
  containing such an envelope, evaluated in ordinal order with a running ceiling, is
  rejected atomically and promotes nothing." The Phase-1 timeline implementation is
  unchanged; the rule lives in the engine composition that already wraps it.

Each amendment addresses a demonstrated counterexample (A-1: §5.3; A-2: §5.4 needs a
pending command to survive across calls; A-3: §6.3's out-of-order fence and the
backwards-evaluation constraint). None weakens pacing neutrality or catch-up
equivalence; A-3 is what makes both hold for commands.

---

## 10. Downstream correction checklist for the next V3-F01 writer (V2-03 … V2-10)

Consequences of this adjudication only; the items themselves are not redesigned.

| Review item | What the writer must do | Effect of this adjudication |
|---|---|---|
| V2-03 budget-one fixture | Use budget 2 for the next-cohort admission discriminator; at budget 1 assert only count/overrun diagnostics | none additional |
| V2-04 `{S}` vs `{S, X_conflicted}` | Assert equal cohort identity, pre-wave digests, batch digests, final state; assert only the conflict report differs | none additional |
| V2-05 engine-owned cross-store extraction | Specify preflight over scheduler slice **and** exact obligation claim sets; typed no-op on any mismatch; move full executable records into the transient extraction; infallible removal under one exclusive engine borrow | the same operation is used unchanged for scheduled slices inside the unified loop; command cohorts perform no extraction |
| V2-06 rejection atomicity | Split wave-0 rejection (state = post-extraction) from later-wave rejection (state = after the last committed wave); separate conflict-only, R-7, and semantic-cap assertions | **delete R-8 and every command-refusal row**; add fixtures for the two non-canonical admission results (`EFFECTIVE_TIME_BEHIND_FRONTIER` at stage and at fence) asserting nothing mutated and nothing finalized; add "command cohort rejected at wave `n`" with `X` advanced |
| V2-07 replay sub-case (2) | Construct the conflict by two unequal claims on the occupied key, or expect a successful replay with a changed fingerprint | none additional |
| V2-08 visibility | Probe the actual exported engine/host boundary; drop claims that public `spark-core` methods are unreachable | `F` and `X` are engine stores, never host-settable; only `advance`, `stage`, `fence`, and read-only `X` are host-facing |
| V2-09 non-discriminating `Kills` | Give each named wrong implementation a fixture whose observable necessarily differs; narrow where no structural proof exists | add the §6.1 trace as a fixture whose observable is the identical command position under budgets 1 and 2 (kills ALT-N and ALT-R); add §6.3 first case (kills execution-time frontier checks) |
| V2-10 diagnostics and loop | Executable-only deferred counts; narrow the all-conflicted progress claim to "before an oversized exception"; define mixed-slice report order | pending commands are not counted in `deferred_cohort_count`; the read-only cursor is the observation point |
| AT-I29(d) | Replace the `Φ` reconstruction test with AT-G discrimination/equivalence tests for `F` and `X` under A-1/A-2 | forced by §5.1 |
| AT-I43 | Rewrite: (a) scheduled `≤ e` precede the command in every budget; (b) equal-time tie rule per profile; (c) admission at stage/fence with host-input-only dependence; (d) fence with out-of-order effective times rejected atomically; (e) command reached only when `T ≥ e`; (f) equal `F` engines with different consumed histories admit identically | forced by §5–§6 |

---

## 11. Unresolved decisions and residual risks

1. **Per-profile timelines versus one global horizon.** `F` is global (blueprint §18.1);
   `C` and `X` are per profile timeline (ADR-0003 §1). This adjudication defines the
   cross-profile total order (§4 sub-decision) but does not revisit whether time should
   itself be per profile. ALT-T records the alternative.
2. **Stuck staged command.** After a host advances `F` past a staged-unfinalized
   command's `effective_time`, the frozen remedy is an ADR-0003 §11 epoch reset. A reviewer
   may prefer a non-canonical guard on `advance` against staged effective times; it was
   rejected here to keep staging causally inert.
3. **Tie rule.** Scheduled-before-command at equal time is a decision with a symmetric
   alternative; the record does not force either.
4. **Amendments A-1…A-3 are proposals.** Until the Operator amends v1 §8 and ADR-0003,
   this contract is not implementable; that is the intended gate.
5. **Supplementary compute was not used.** Two bounded probes (25 s, then 20 s with
   stdin closed) of `~/.local/bin/swarm-mci-dev --help` did not return; the Operator
   reported that this binary launches the MCI desktop GUI, which is not the headless
   harness entry point and was never intended to be opened. This observation did not
   establish that the Development Compute gateway was unavailable. The prior review's
   evidence bundle records request identifiers and lifecycle but no CLI or API
   invocation; the engineer did not locate a documented headless entry point in the
   bounded S.P.A.R.K. search and correctly stopped troubleshooting rather than expanding
   scope. No request was submitted. S.W.A.R.M. now documents the recovered invocation as
   `.venv/bin/python -m tools.dev_compute.orchestrate ...` in its canonical
   `tools/dev_compute/README.md`; this follow-up creates no S.P.A.R.K. runtime dependency.
   This candidate rests on the frozen record and the three reviews.
6. **No adversarial review of this pass exists.**

---

## 12. Verdict of this pass

`COMMAND_TIME_ADJUDICATION_CANDIDATE_READY`

- Command-time question: **answered by ALT-U**, subject to a new V3-F01 writer pass and
  independent review, and to Operator decision on A-1…A-3.
- V3-F01: **OPEN**. Phase-2 architecture: **NOT frozen, NOT accepted.**
- PHASE_2_IMPLEMENTATION_AUTHORIZATION: **NO**. PHASE_3_AUTHORIZATION: **NO**.
- Phase 1: **CLOSED and not reopened.**
