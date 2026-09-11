# S.P.A.R.K. Phase 2 — Acceptance Test Oracle, V3-F01 Revision 2 (AT-I)

**Date:** 2026-09-10
**Agent:** Claude Code (Opus 5, `claude-opus-5`), separated V3-F01 bounded correction writer.
**Status:** **CANDIDATE — REVISION 2 PROPOSAL — AWAITING INDEPENDENT REVIEW.** Not accepted,
not frozen, not canonical, and authorizing no implementation.
**Companion to:** `SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_REV2_2026-09-10.md`
(unqualified "§" references are to that document).
**Amends, as a delta:** `SPARK_PHASE_2_ACCEPTANCE_TEST_ORACLE_V3_F01_FINAL_2026-09-10.md`
(the "FINAL oracle", commit `74d044ddd2b59c3c2eff558beeeeb515390dbe0f`), preserved unchanged
as history. Entries not named in §1 apply verbatim from the FINAL oracle.
**Model and probe references:** "Model …" names a check in
`engineering/phase2/v3_f01_rev2_bounded_correction_evidence_2026-09-10/model.py`; "Probe"
names `rev2_finalization_probe.rs` in the same directory.

**Discipline (unchanged).** Every AT-I test is encoded red-first against the inherited tree
with its baseline recorded verbatim; "inexpressible against the inherited tree" is recorded,
never skipped. Every "X is impossible" claim is a digest or value equality or a
compile-fail, never an error-was-returned check alone. No existing test is weakened,
deleted, or skipped; the Phase-1 suite (232 tests, re-run fresh in this pass) runs
unmodified. Every oracle carries a **Kills:** line naming a concrete wrong implementation
and the observable that necessarily differs. **No test here has been executed**; no
Phase-2 implementation exists. The model and probe are architecture evidence.

**Full-state comparison (used throughout).** "Complete timeline state" means equality of
`canonical_state_digest`, `canonical_history_digest`, **and** the complete derived `Debug`
rendering of `TimelineIngress`, which exposes every private field (slots, poison evidence,
staged and finalized identity indexes, `last_finalized_source_sequence`, finalized
commands and fences, epoch, sequencer, window width, epoch-reset records, frontier, last
fence hash). An implementation may replace the `Debug` comparison with a `test-support`
accessor returning the same fields; digests alone are insufficient because
`canonical_state_digest` does not hash the identity indexes.

---

## 1. Exact delta

| Entry | Disposition |
|---|---|
| **AT-I48** `single_command_finalization_is_atomic` | **NEW** — §2 (FINAL-01) |
| **AT-I49** `consumer_dequeue_is_request_bound` | **NEW** — §3 (FINAL-02) |
| **AT-I50** `replay_scope_is_completed_boundaries_only` | **NEW** — §4 (FINAL-03) |
| **AT-I43** (a), (f), (h), (i) | **REVISED** — §5 |
| **AT-I47** (a), (d) | **REVISED** — §6 |
| **AT-I35** | **EXTENDED** — §7 (encoding pins) |
| **AT-I40** (e) | **REVISED** — §8 |
| **AT-I39** (C) | **REVISED** — §9 |
| **AT-I32** | **EXTENDED** — §10 (timeline facade probes) |
| FINAL oracle §2 conformance row | **EXTENDED**: "and **no public timeline staging or fencing path** lands on the `spark-engine` facade" |
| FINAL oracle §14 traceability | **EXTENDED** by §11 |
| Every other entry | **UNCHANGED** |

---

## 2. AT-I48 — NEW: `single_command_finalization_is_atomic`

- **(a) Historical red evidence, preserved.** The review's probe
  `engineering/phase2/v3_f01_final_independent_review_evidence_2026-09-10/source_sequence_regression.rs`
  remains a record that the FINAL recipe (live `stage`, then live `submit_fence`) leaves
  ordinal 1 positively staged after `SourceSequenceNotIncreasing` on unchanged Phase-1
  code. It is not edited, and production Rust is not changed to make it pass.
- **(b) Source-sequence regression through `process`.** Finalize a command from source
  `s` with `source_sequence = 10` at ordinal 0. With no scheduled work in the horizon,
  present a distinct command from `s` with `source_sequence = 9`: result
  `COMPLETED_COMMAND_NOT_FINALIZED { SourceSequenceNotIncreasing { previously_finalized:
  10, attempted: 9 } }`; complete timeline state equal before and after;
  `slot_status(Ordinal(1)) = Empty`; `frontier_ordinal = 1`; `engine_state_digest` equal
  to its pre-request value; `F = effective_time`; `ActiveRequest = None`. The same
  fixture through the old composition is the recorded red baseline.
  **Kills:** the FINAL recipe (slot 1 `Staged`, state digest and indexes differ); a
  preflight without row P-9. Model F1, F1b; Probe route 1.
- **(c) Every refusal route.** One single-fault fixture per row of §4.4 — P-1 wrong
  profile, P-2 wrong epoch, P-3 sequencer grant not active (through a `test-support`
  seam; unreachable otherwise), P-4 window arithmetic exhausted and P-5 frontier advance
  exhausted (both through `TimelineIngress::resume_at_frontier` under `test-support`:
  frontier `u64::MAX` with widths 2 and 1), P-6 four unexpected-staging states
  (poisoned frontier slot; different envelope staged at the frontier; identical envelope
  staged at the frontier; staged tail above an empty frontier, each injected through the
  seam), P-7 command-id reuse, P-8 source-sequence pair reuse, P-9 source-sequence
  regression. For each: the typed refusal row, and complete timeline state equal before
  and after.
  **Kills:** the FINAL recipe, which mutates the live timeline on six of these routes
  (P-9, P-5, and all four P-6 states: staged residue, poison-evidence insertion, slot
  poisoning, or an unexpected finalization); any preflight missing P-5, P-6, or P-9.
  Model F2, F2b; Probe routes 1–11.
- **(d) Success sequence.** Through the `test-support` seam, observe the operation log of
  one finalization: exactly one `stage` returning `Acknowledged(NewlyStaged)` whose
  acknowledgement `covers` the envelope, **then** exactly one `submit_fence` with
  `start = end =` the entry frontier, `finalized_command_count = 1`, and new frontier
  `+1`; `fence_id = "fence." ‖ decimal(n)`. The resulting complete timeline state equals the
  state produced by running the unchanged `stage` and `submit_fence` on an isolated clone.
  **Kills:** fencing without a prior positive acknowledgement; a multi-ordinal fence; a
  non-deterministic fence identity. Model F3; Probe "SUCCESS".
- **(e) Differential completeness of the preflight.** For every fixture of (c) and a
  generated corpus of clean-staging timelines (random prior finalized streams across at
  least two sources and one epoch reset, crossed with requests varying profile, epoch,
  command id, and source sequence below, equal to, and above the last finalized value),
  compare the preflight verdict with the verdict of the unchanged `stage` then
  `submit_fence` run on a `clone()` of the live timeline: equal on every clean-staging
  input; on single-fault inputs the refusal reason also corresponds; the live timeline is
  never touched by the reference. On unclean states the preflight refuses by P-6 even
  where the reference succeeds (the identical-envelope and staged-tail states); this is
  asserted, not treated as divergence.
  **Kills:** a preflight missing any row (it passes where the reference refuses, which
  in production would reach the entailment violation of §4.5). Model F2c; Probe.
- **(f) Follow-on.** After (b), a distinct command from `s` with `source_sequence = 11`
  finalizes at ordinal 1. **Kills:** residue that lets the next command poison ordinal 1
  (the FINAL recipe: `stage` returns `Poisoned`, and no fence can finalize ordinal 1).
  Model F4; Probe "FOLLOW-ON".
- **(g) Clean staging.** At every stable boundary of every AT-I43, AT-I47, and AT-I49
  fixture, every ordinal of the current window is `Empty`. A snapshot edited through the
  seam to carry a staged or poisoned slot is rejected with
  `RESTORE_REJECTED_TIMELINE_STAGING_PRESENT`, nothing live. Model F5.
- **(h) Horizon completion is not command success, and nothing is rolled back.** A
  command request whose horizon admits a scheduled cohort, then refuses at P-7: the
  scheduled cohort's committed effects remain; `F = effective_time`; `ActiveRequest =
  None`; the complete timeline state is unchanged; no earlier request's cohorts or waves
  change. **Kills:** whole-request rollback; leaving `F` behind (D-2). Model F6.
- **(i) Deferred command.** With the budget exhausted before the command, `PAUSED`
  leaves the complete timeline state unchanged; the next call finalizes. (Carries FINAL
  AT-I43(h)'s paused-boundary assertion with full-state comparison.) Model S19.
- **(j) Entailment-violation handling.** Through a `test-support` fault-injection seam
  that disables one preflight row (test builds only), drive (b): the call returns
  `FINALIZATION_ENTAILMENT_VIOLATED`, **not** `COMPLETED_COMMAND_NOT_FINALIZED`; no stable
  boundary or snapshot is published; every later `process` call returns the same error;
  restoring the last committed snapshot yields a clean-staging engine that then refuses
  the request correctly. **Kills:** mapping a preflight defect to an ordinary refusal and
  publishing the residue as a stable boundary. Specified; not modelled.

---

## 3. AT-I49 — NEW: `consumer_dequeue_is_request_bound`

- **(a) Faulty presentation with the genuine active head queued.** Mailbox
  `[Advance(20), Command@21]`, budget 1, work `A@10` (whose rule creates `D@15`) and `B@20`.
  Present the head: `PAUSED`. A faulty consumer presents `Command@12`:
  `REFUSED_ACTIVE_REQUEST_MISMATCH { active: d(Advance(20)), presented: d(Command@12) }`.
  Assert: the result is not dequeue-eligible; the mailbox contents, their order, and the
  head's identity are unchanged; the engine snapshot and `stable_boundary_digest` are
  unchanged. Then drive normally: `PAUSED`, `COMPLETED`, `COMPLETED`, and the canonical
  result equals the undisturbed run.
  **Kills:** the FINAL rule that pops on mismatch — the head `Advance(20)` disappears
  while `ActiveRequest` still binds it and the next head is `Command@21`. Model Q1, Q1b,
  Q1c.
- **(b) Request-bound removal.** With no request active, a faulty consumer presents a
  request `X` that is not the head; `X` completes with `presented = d(X)`. The consumer
  does not remove the head, because `presented ≠ d(head)`. **Kills:** a consumer that pops
  on any terminal result regardless of which request it names. Specified; not modelled.
- **(c) Restore/mailbox disagreement.** Snapshot at a paused `Advance(20)`; restore it
  with a durable mailbox whose head is `Command@21`. The consumer halts with
  `CONSUMER_HALTED_ACTIVE_REQUEST_NOT_AT_HEAD`, makes no `process` call, removes nothing;
  engine snapshot and mailbox unchanged. **Kills:** a consumer that pops, skips, or
  presents the non-matching head. Model Q2.
- **(d) Durability ordering.** Completion durable before the dequeue: re-presenting a
  completed `Advance(20)` yields `COMPLETED` with no cohort and a byte-identical stable
  boundary; re-presenting a finalized `Command@21` yields
  `COMPLETED_COMMAND_NOT_FINALIZED { CommandIdentityConflict }` with the complete
  timeline state and stable boundary byte-identical. Dequeue durable first: the restored
  engine has `ActiveRequest = Some(d(Advance(20)))`, the head is `Command@21`, and the
  consumer halts. **Kills:** an ordering that silently loses the active request. Model Q3,
  Q3b. Deep-copy evidence only; no crash test is claimed.
- **(e) Engine-level no-mutation.** FINAL AT-I43(e), (f), and (k) are retained as the
  engine half; this entry is the consumer half.

---

## 4. AT-I50 — NEW: `replay_scope_is_completed_boundaries_only`

- **(a) History plus `F` cannot reconstruct a pause.** From one genesis with `A@10`,
  `B@20`, budget 1: engine P after `process(Advance(20)) = PAUSED`; engine Q after
  `process(Advance(0)) = COMPLETED`. Assert equal complete timeline state and `F = 0`, and
  **unequal** `engine_state_digest` and `stable_boundary_digest`. **Kills:** any claim or
  implementation that restores a paused boundary from finalized history and `F`. Model R1.
- **(b) Snapshot plus exact request.** Restore P's snapshot; present `Command@20`:
  refused, byte-identical; present `Advance(20)` until `COMPLETED`; the result equals P
  continued without restart. **Kills:** restart without `ActiveRequest`. Model R2.
- **(c) Completed-boundary reconstruction.** Original run at budget 1 over
  `Advance(12)`, `Command@20`, `Advance(21)`, a `Command@22` reusing the first command's
  id (refused by P-7), `Command@24`, an epoch reset to epoch 1 with a new sequencer,
  `Command@30` (epoch 1), `Advance(33)`. Reconstruct per §6.1 at budget 9 from genesis,
  timeline metadata, the finalized envelopes **with payloads**, and `F`: the
  `stable_boundary_digest` and complete timeline state are equal. **Kills:** a
  reconstruction that depends on pacing or on refused requests. Model R3.
- **(d) Preconditions are necessary.** The same reconstruction with a different window
  width: equal `canonical_history_digest`, different stable boundary. Without the reset
  record: the epoch-1 command cannot finalize. Without a payload: the command cohort
  cannot be evaluated (the envelope carries only `canonical_payload_hash`). **Kills:**
  envelope-only replay claims. Model R3b (width and reset); payload case specified.
- **(e) Non-claim.** No test here is a process-crash or durable-storage test.

---

## 5. AT-I43 — REVISED entries

- **(a) Lifecycle, spelled out.** Fixture: `A@10` and `B@20` resident, budget 1, and the
  **work-producing rule** that `A`'s wave 0 enqueues `D` at `due_time = 10 + 5 = 15`.
  Successive calls presenting `Advance(20)`: call 1 → `PAUSED` after cohort 10 (`F = 0`,
  `ActiveRequest = Some(Advance, 20)`); call 2 → `PAUSED` after cohort 15 (unchanged `F`
  and `ActiveRequest`); call 3 → `COMPLETED` after cohort 20 (`F = 20`, `ActiveRequest =
  None`). A `Command@21` offered to the mailbox after call 1 is presented only after call 3;
  call 4 → `COMPLETED` (`F = 21`). **Kills:** starting a later request while one is paused;
  a call-entry plan (would not consume `D@15` inside the same request). Model P-e, S1.
- **(f) Same-kind, same-horizon substitution.** Budget 1, work `A@10`, `B@20`; present
  `Command@20(p1)`: `PAUSED`. Present `Command@20(p2)` — same kind, same horizon, different
  command id and payload: `REFUSED_ACTIVE_REQUEST_MISMATCH`, engine snapshot and stable
  boundary byte-identical. The FINAL fixture (a `Command@20` while `Advance(20)` is paused) is
  retained with its **Kills** narrowed to "a horizon-only discriminator".
  **Kills:** a discriminator over kind and horizon only (it resumes and returns `PAUSED`
  for the substitute). Model P-a.
- **(h) Command finalization at completion.** As the FINAL entry, with "no
  staged-unfinalized slot at any stable boundary" asserted as `slot_status = Empty` over the
  whole window (AT-I48(g)), and the positive acknowledgement before the fence asserted per
  AT-I48(d).
- **(i) Finalization refusal.** As the FINAL entry, with "timeline unchanged" replaced by
  "complete timeline state equal" and the refusal row asserted; see AT-I48(b), (c).

---

## 6. AT-I47 — REVISED entries

- **(a) Discrimination — stated exactly.** Five engines with equal `engine_state_digest`
  and equal `F`: `None`; `Some(Advance, 20)`; `Some(Command, 20, id₁)`; `Some(Advance, 21)`;
  `Some(Command, 20, id₂)`. Asserted: five pairwise-distinct `stable_boundary_digest`s and
  equal `engine_state_digest`s; plus the two-engine `F` case. **What this kills:** omission
  of `F`; omission of `ActiveRequest`; a colliding `None`/`Some` encoding; omission of
  `identity` (the `id₁`/`id₂` pair collides); inclusion of either value in
  `engine_state_digest`. **What it does not kill:** omission of `kind` or of `horizon`
  from the encoding — every pair differing in kind or horizon also differs in `identity`,
  whose preimage binds both, so the digests stay distinct (model P-b: five distinct digests
  under either mutant). Those omissions are killed only by the AT-I35 encoding pins.
- **(d) Snapshot and restore.** As the FINAL entry, plus: restoring with a mailbox whose
  head does not match is AT-I49(c), not an engine refusal test.

---

## 7. AT-I35 — EXTENDED: encoding pins

In addition to the FINAL pins: golden byte vectors, produced by an independent reference
encoder written from the candidate's text (FINAL §6.2), for `canonicalize(ActiveRequest)`
over `None`, `Some(Advance, 20)`, and `Some(Command, 20, id)` — field order
`push_str("active_request.some") ‖ push_str(kind tag) ‖ horizon.canonicalize ‖
push_digest(identity)` — and for `stable_boundary_digest` over each. Mutants omitting
`kind`, omitting `horizon`, or reordering fields fail the byte comparison.
**Kills:** every omission or reordering, including the two that discrimination cannot
detect. Model P-b (pins distinguish all three omission mutants).

---

## 8. AT-I40 — REVISED (e)

For `A = {S}` and `B = {S, X_conflicted}` at one `(t, P)`: **equal** cohort identity,
emission identities, pre-wave engine digests, `effect_batch_digest`s, **resulting
obligation stores**, and final `engine_state_digest`; **different** pre-extraction scheduler
digests, canonical conflict reports, and **removed sets** — `B`'s extraction additionally
removes `X`'s complete contested claim set, asserted as the exact set difference.
**Kills:** as FINAL, plus an extraction that leaves part of `X`'s contested set (the
residual stores would then differ). Model P-c, S15.

---

## 9. AT-I39 — REVISED (C)

Cohorts at `due_time` 1 and 1 000 000, nothing between. `Advance(1_000_000)` produces exactly
two cohorts equal to the stepwise partition. Through the `test-support` seam, count
**outer-loop selections only** — the X-1 call at step A2 of FINAL §9, instrumented at that
seam: the count equals `slices consumed + 1` (the terminal `None`), here 3. Extraction
preflight (FINAL §11 step 1) and X-3 also locate the least slice and are **not** counted;
an unconditional count of all such uses is 7 here and is not the asserted quantity.
**Kills:** a tick-iterating outer loop (~10⁶ outer selections). It bounds the outer loop
only; it does not measure time. Model P-d.

---

## 10. AT-I32 — EXTENDED compile probes

Each must fail to compile: obtaining `&mut TimelineIngress` from any public `spark_engine`
item; calling `stage`, `submit_fence`, or `reset_epoch` on the engine's timeline through
any public `spark_engine` path other than the engine's own documented epoch-reset
operation; invoking the engine's finalization other than through `process`; constructing a
`FinalizationRefusal` or `FinalizationRecord` outside `spark_engine`.
**Kills:** a host path that stages or fences the engine's timeline and so breaks I-CS.

---

## 11. Traceability to the final independent review

| Review item | Test |
|---|---|
| FINAL-01 atomic finalization, every refusal byte-identical | AT-I48(b)(c)(e)(g)(j) |
| FINAL-01 positive acknowledgement before fence; one command barrier | AT-I48(d), AT-I43(h) |
| FINAL-01 empty-frontier and unexpected staged/poisoned state | AT-I48(c) P-6, (g) |
| FINAL-01 ordinal/window exhaustion | AT-I48(c) P-4, P-5 |
| FINAL-01 no rollback; D-2 | AT-I48(h) |
| FINAL-01 executable regression, old recipe red | AT-I48(a)(b)(f) |
| FINAL-02 mismatch never consumes the head; consumer regression | AT-I49(a)(b) |
| FINAL-02 fail-closed restore/mailbox mismatch; durable exact request | AT-I49(c)(d) |
| FINAL-03 same history and `F`, different pause | AT-I50(a) |
| FINAL-03 snapshot plus exact request; substitution refused | AT-I50(b), AT-I47(d) |
| FINAL-03 completed-boundary scope and preconditions | AT-I50(c)(d) |
| Oracle: AT-I43(f) same-kind/same-horizon | AT-I43(f) |
| Oracle: AT-I47(a) honest claims and pins | AT-I47(a), AT-I35 |
| Oracle: AT-I40(e) stores versus removed sets | AT-I40(e) |
| Oracle: AT-I39(C) outer-loop seam | AT-I39(C) |
| Oracle: AT-I43(a) work-producing rule and successive results | AT-I43(a) |

---

## 12. Out of scope (unchanged)

No persistence, service, protocol, crash, or mailbox-implementation tests; no executable
Windows/Android runs; no G.A.M.E. integration. **Phase-2 implementation is not authorized
by this oracle; Phase 3 is not authorized.** This is a proposed oracle awaiting independent
review; no test in it has been executed.
