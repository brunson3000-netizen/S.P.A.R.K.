# S.P.A.R.K. Phase 2 — V3-F01 Bounded Correction Candidate, Revision 2

**Date:** 2026-09-10
**Agent:** Claude Code (Opus 5, `claude-opus-5`), separated V3-F01 bounded correction
writer. The independent reviewer is Codex and is not the writer.
**Status:** **CANDIDATE — REVISION 2 PROPOSAL — AWAITING INDEPENDENT REVIEW.** Not
accepted, not frozen, not canonical; authorizes no implementation. V3-F01 remains **OPEN**.
**Form:** a bounded delta to
`SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_FINAL_2026-09-10.md` (the "FINAL
candidate", commit `74d044ddd2b59c3c2eff558beeeeb515390dbe0f`). Every FINAL section named in
§11 is replaced or amended exactly as stated there; **every other FINAL section applies
verbatim**. The FINAL candidate, its oracle, writer report, and model are preserved
unchanged as history. Unqualified "§" references are to this document; "FINAL §…" to the
FINAL candidate.
**Controlling inputs:**
`SPARK_PHASE_2_CODEX_V3_F01_FINAL_INDEPENDENT_REVIEW_2026-09-10.md` (verdict
`V3_F01_BOUNDED_REVISION_REQUIRED`, findings FINAL-01, FINAL-02, FINAL-03, and five oracle
precision items) and the mission
`SPARK_PHASE_2_V3_F01_BOUNDED_CORRECTION_WRITER_MISSION_REV2_2026-09-10.md`, both at review
commit `5b19d7b5aa935b65fcfad1d8bda9a210d5a5684d`.
**Companion oracle:** `SPARK_PHASE_2_ACCEPTANCE_TEST_ORACLE_V3_F01_REV2_2026-09-10.md`.
**Evidence:** `engineering/phase2/v3_f01_rev2_bounded_correction_evidence_2026-09-10/`.

**Nonclaims, first.** This revision does not claim V3-F01 closure, acceptance of this or
any prior candidate, a Phase-2 architecture freeze, Phase-2 implementation or Phase-3
authorization, any Phase-1 change, crash-recovery or durable-mailbox evidence, executable
cross-platform parity, or any G.A.M.E. change. No production Rust was written. The Python
model and the disposable Rust probe are architecture evidence, not acceptance tests.

---

## 1. Baseline and lineage

| Item | Verified value |
|---|---|
| Repository / remote | `/home/chromikey/Projects/SPARK`; `https://github.com/brunson3000-netizen/S.P.A.R.K..git` |
| Production | `phase1-refoundation-v2` at `e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8` (local, tracking, and live remote agree) |
| Reviewed candidate | `candidate/v3-f01-final-correction-20260910` at `74d044ddd2b59c3c2eff558beeeeb515390dbe0f`, sole parent `772e38da0d130d7a1225ba2eb60a75b3999336fe` |
| Base of this revision | `review/v3-f01-final-20260910` at `5b19d7b5aa935b65fcfad1d8bda9a210d5a5684d`, sole parent `74d044d…`; contains the final independent review and the Revision-2 mission |
| This branch | `candidate/v3-f01-bounded-correction-rev2-20260910` (new; did not exist locally or remotely), worktree `/home/chromikey/Projects/SPARK-v3-f01-rev2-writer` |
| Phase 0 / Phase 1 | CLOSED, not reopened |
| Phase 3 | NOT AUTHORIZED |

---

## 2. What this revision changes

| Review item | Disposition | Where |
|---|---|---|
| FINAL-01 (BLOCKER) stage-then-fence is not atomic | **CORRECTED**: one engine-internal finalization operation with a complete read-only preflight and an entailed apply; every refusal leaves the whole timeline byte-identical | §4 |
| FINAL-01 executable regression | **ADDED**: the review's unchanged-Phase-1 probe is preserved red; a model with Phase-1-faithful staging/sequencing fails the old recipe and passes the new one; a disposable Rust probe does the same against the unchanged Phase-1 library | §4.10, §10 |
| FINAL-02 (MAJOR) mismatch pops the real FIFO head | **CORRECTED**: mismatch terminates nothing; request-bound dequeue eligibility; fail-closed restore/mailbox mismatch; durability ordering | §5 |
| FINAL-03 (MAJOR) history-only replay cannot restore a pause | **CORRECTED**: history reconstruction scoped to completed boundaries with explicit preconditions; paused recovery requires the committed snapshot plus the exact durable request; the "any equal-or-greater horizon" claim withdrawn | §6 |
| Oracle precision (five items) | **CORRECTED** in the companion oracle | §8 |

---

## 3. Preserved without reopening

Carried verbatim from the FINAL candidate: the authorized `ActiveRequest` owner, full
identity, encoding, set-before-mutation, exact resume, pause retention, snapshot
validation, and atomic completion with `F` (FINAL §6); `F` and `ActiveRequest` in
`stable_boundary_digest` and out of `engine_state_digest` (FINAL §14); horizon expansion,
cohort-local time, live least-slice full-fingerprint compare-and-take, atomic
scheduler/obligation extraction, executable-only identity and pacing, complete conflicted
extraction and report ordering, later-wave commit retention, the observation and facade
boundaries (FINAL §§4, 9–15); decisions D-1 … D-5 (FINAL §22); and the withdrawal of
`Phi`, `X`, `C`, SH-1/SH-2, R-8, and A-1 … A-3. No cancellation, abandonment,
transport, mailbox implementation, or broad scheduler surface is added. No Phase-1 code,
signature, semantics, encoding, tag, or digest input changes; `effect_batch_v3` and every
pinned value are unchanged.

---

## 4. FINAL-01 — atomic single-command finalization

### 4.1 Operation, owner, input

```text
Engine::finalize_command(&mut self, request: &CommandRequest)          (engine-internal; not host surface)
    -> Result<FinalizationRecord, FinalizationRefusal>
```

- **Owner.** The Phase-2 engine composition in `spark-engine`, which owns its
  `TimelineIngress` value and the timeline sequencer grant `G` (D-1: under serialized
  processing the engine is the active sequencer). No public `spark_engine` item returns
  `&mut TimelineIngress` or calls `stage`, `submit_fence`, or this operation except
  through `process` (oracle AT-I32).
- **Caller.** Only step A6 of the processing loop (FINAL §9, replaced in §4.8), when no
  resident slice has `due_time ≤ effective_time` and one budget unit is available.
- **Input.** The command request (FINAL §8.1: the `SemanticCommandEnvelope` fields minus
  `input_ordinal`), the live timeline, and `G`.
- **Output.** On success, the Phase-1 `StageAcknowledgement` and `FinalizationResult`
  for ordinal `n` (the frontier at entry); on refusal, one typed `FinalizationRefusal`
  row of §4.4.

### 4.2 Mechanism choice (decision D-6)

The mission permits either complete validation before live mutation or an isolated
transaction with infallible publication. This revision specifies **complete read-only
preflight followed by an entailed apply** on the live timeline. The isolated alternative
is expressible with the existing API (`TimelineIngress` derives `Clone`), and publication
by assignment would be infallible, but every command would copy the complete finalized
history (`finalized_commands`, `finalized_fences`, registries), which grows without bound
on a long-running device. It is therefore **not** the production mechanism; it is retained
as the **differential reference** of oracle AT-I48(e), which is what makes an incomplete
preflight detectable. The recipe of FINAL §8.2 (live `stage`, then live `submit_fence`,
no preflight) is **withdrawn**.

### 4.3 Clean-staging invariant (decision D-7)

**I-CS.** At every stable boundary the engine's timeline has no slot: `slot_status(o) =
Empty` for every ordinal `o` in `[frontier, window_end]` (Phase-1 slots can exist only in
that range: `submit_fence` retains `ord ≥ new_frontier` and `stage` inserts only inside the
current window, timeline.rs 1565–1583). By the derived-index invariant (timeline.rs
938–967), the staged command and source-sequence indexes are then empty.

I-CS holds by construction: the only `stage` call is the apply step A-3 below, which is
followed within the same exclusive borrow by a one-ordinal fence that promotes and drops
that slot; every refusal stages nothing; `reset_epoch` clears all slots
(timeline.rs 1631–1633); restore rejects any snapshot whose timeline carries a slot (§7);
and the engine's facade offers no other path to its timeline. A state violating I-CS is
therefore unreachable; if one is ever encountered, finalization **fails closed** (row
P-6), even where Phase-1 alone would accept (an identical envelope already staged at the
frontier, or a staged tail above it).

### 4.4 Complete preflight — validation and error table

Evaluated in row order, read-only, under `&mut self` (no mutation occurs before A-3). The
first failing row is the refusal. `n` is the frontier; `E` is the envelope formed from the
request with `input_ordinal = n`; `h(E)` is `E.semantic_hash()`.

| Row | Predicate (all must hold) | Phase-1 check it discharges (`crates/spark-core/src/timeline.rs`) | Refusal | Reachable by |
|---|---|---|---|---|
| P-1 | `request.profile_id = timeline.profile_id()` | `stage` 1233; `submit_fence` 1440 | `WrongProfile` | host input |
| P-2 | `request.timeline_epoch = timeline.timeline_epoch()` | `stage` 1236; `submit_fence` 1443 | `WrongTimelineEpoch` | host input (e.g., after an epoch reset) |
| P-3 | `G = timeline.active_sequencer()` | `stage` 1239; `submit_fence` 1446 | `NotActiveSequencer` | unreachable under engine ownership; still checked |
| P-4 | `current_admission_window()` is `Ok` (window end computable) | `stage` 1248–1250; `submit_fence` 1452–1454 | `OrdinalSpaceExhausted { window }` | ordinal space end |
| P-5 | `n + 1 ≤ u64::MAX` | `submit_fence` 1466–1476 | `OrdinalSpaceExhausted { frontier_advance }` | frontier `= u64::MAX` |
| P-6 | I-CS: every ordinal of the window is `Empty` | `stage` 1277–1391 (poison-evidence insert 1336–1338, poisoning 1359–1390, idempotent acknowledgement 1350–1358); `submit_fence` 1478–1498 | `UnexpectedStagingState { ordinal, status }` | unreachable (§4.3); fail closed |
| P-7 | `command_id` unclaimed in finalized history, or claimed by `h(E)` | `stage` 1284–1294 | `CommandIdentityConflict { command_id }` | host input (reuse, redelivery) |
| P-8 | `(source_id, source_sequence)` unclaimed, or claimed by `h(E)` | `stage` 1295–1307 | `SourceSequenceConflict { source_id, source_sequence }` | host input |
| P-9 | no finalized sequence for `source_id`, or `source_sequence >` the last finalized one | `submit_fence` 1516–1533 | `SourceSequenceNotIncreasing { source_id, previously_finalized, attempted }` | host input — the FINAL-01 counterexample |

**Excluded by construction** (the engine derives each value from the same unmutated
state within the same borrow, so the check cannot fail): ordinal inside the window
(`stage` 1252: `n` is the frontier and `window_end ≥ n` because `window_width ≥ 1`); the
admission ticket (1264–1273: minted from the current window); `StartNotAtFrontier`
(1449); `EndOutsideStageableHorizon` (1455: `start = end = n ≤ window_end`);
`PreviousFenceHashMismatch` (1458: read from `last_finalized_fence_hash()`);
`RangeNotFullyStaged` / `RangeContainsPoisoned` (after A-3 slot `n` is `Staged(E)`);
`DigestMismatch` (1500–1507: the engine computes `compute_ordered_stream_digest([(n,
h(E))])`, the same function).

**Evaluating P-7 … P-9 with today's API.** The finalized registries and
`last_finalized_source_sequence` are private. Their contents are exactly derivable from
the public `finalized_commands()`: the registries are populated only at promotion and are
never cleared, including across epoch resets (1535–1558, 1631–1633), and the last
finalized sequence of a source is that of its last promoted command. Under P-6 the staged
registries are empty. This derivation is exact but `O(|finalized history|)` per command.

**Proposal PX-1 (additive, read-only, not enacted).** Three accessors over the existing
private maps — `finalized_command_claim(&CommandId) -> Option<&Digest>`,
`finalized_source_sequence_claim(&SourceId, u64) -> Option<&Digest>`,
`last_finalized_source_sequence(&SourceId) -> Option<u64>` — would make P-7 … P-9
`O(log n)`. They change no semantics and no encoding. Adopting them is a separate
Phase-1-surface decision; this revision does not depend on them. Either evaluation must
pass the same oracle (AT-I48).

**Reason ordering.** On a single-fault input the reported row is the Phase-1 error the
unchanged calls would return. On a multi-fault input the preflight reports its first
failing row, which can differ from the error `stage`-then-`submit_fence` would report
(for example, P-5 precedes P-7 here, while Phase-1 detects the frontier overflow only at
the fence). The oracle therefore compares **verdicts** differentially, and **reasons** on
single-fault fixtures only.

### 4.5 Apply — entailed, with the positive acknowledgement before the fence

After every row passes, within the same borrow:

```text
A-1  window := current_admission_window()                     Ok by P-4; unchanged since preflight
A-2  E      := request fields ‖ input_ordinal = n
A-3  stage(G, E.submit_with(window.ticket()))
         = Acknowledged(StageAcknowledgement{ slot_state: NewlyStaged, … }) with ack.covers(E)
         effect: slot n := Staged(E); staged indexes gain E's command-id and sequence keys
A-4  fence_n := { profile, epoch, fence_id = "fence." ‖ decimal(n), start = end = n,
                  previous_fence_hash = last_finalized_fence_hash(),
                  ordered_stream_digest = compute_ordered_stream_digest([(n, h(E))]) }
     submit_fence(G, fence_n) = Ok(FinalizationResult{ start = end = n, count = 1,
                                                       new_frontier = n + 1 })
         effect: E promoted; identity claims moved to the permanent registries;
                 last finalized sequence of E.source_id := E.source_sequence;
                 fence appended; frontier := n + 1; slot n dropped  ⇒ I-CS restored
```

**Entailment lemma.** `stage` and `submit_fence` are deterministic total functions of the
timeline state and their arguments (timeline.rs 1226–1598). A-3's result follows from
P-1 … P-4 (profile, epoch, sequencer, window), the construction of the ordinal and ticket,
P-6 (slot `n` empty), and P-7/P-8 with I-CS (registries). A-3 changes only slot `n` and the
two staged indexes. A-4's result then follows from P-1 … P-5, the construction of every
fence field, A-3 (slot `n` is `Staged(E)`), and P-9 (`last_finalized_source_sequence` is not
touched by `stage`). No other result is possible. This is the same pattern the review
accepted for FINAL §11's cross-store extraction (preflight, then an apply that cannot
fail under the exclusive borrow).

**Fence identity (decision D-10).** FINAL §8.2 left `fence_id` unspecified. It is fixed
here as `"fence." ‖ decimal(n)` (valid for every `u64` under the Phase-1 identifier rules:
at most 26 bytes, lowercase digits), so fence hashes and `canonical_history_digest` are a
function of the finalized stream (required by §6.1).

**Entailment violation (decision D-8).** If an implementation ever observes a result at
A-3 or A-4 other than the entailed one, that is a missing preflight row — an
implementation defect, not a timeline refusal. It is surfaced as the typed internal error
`FINALIZATION_ENTAILMENT_VIOLATED`, is **never** mapped to
`COMPLETED_COMMAND_NOT_FINALIZED`, publishes no stable boundary and no snapshot, and
fail-stops the engine instance: every later `process` call returns the same error until
the host restores the last committed snapshot (which, by I-CS, contains no staging). No
new host operation is introduced; restore already exists. AT-I48(e) is the red test that
detects a missing row before this can ship; AT-I48(j) pins this handling.

### 4.6 Exclusive access, transaction boundary, publication point

- **Exclusive access.** One `&mut Engine` borrow spans the whole `process` call; the
  engine owns the timeline by value; the host has no mutable path to it; the consumer is
  serialized (FINAL §17 as replaced in §5.5).
- **Transaction boundary.** Opens at A6 when finalization is attempted. It contains the
  preflight (read-only), A-1 … A-4, the command cohort's evaluation (waves and rejection
  per FINAL §13), and the completion assignments. **No stable boundary exists inside it**
  (ADR-0003 §14 command barrier; FINAL §4 "stable boundary").
- **Publication point.** The single stable boundary returned by that `process` call:
  - **success** — the timeline after A-4, the stores after the command cohort's last
    committed wave (or its typed rejection, FINAL §13), `F := effective_time`,
    `ActiveRequest := None`, result `COMPLETED`;
  - **refusal** — the timeline **byte-identical** to its state when the request started
    (the timeline is mutated only by this operation and by the engine's epoch-reset path,
    and neither ran), the stores as committed by the request's scheduled cohorts,
    `F := effective_time`, `ActiveRequest := None`, result
    `COMPLETED_COMMAND_NOT_FINALIZED { refusal }`.

### 4.7 Refusal semantics (replaces FINAL §8.3)

For every row P-1 … P-9: the **entire timeline** is byte-identical before and after —
`canonical_state_digest`, `canonical_history_digest`, and every private field: slots and
poison evidence, staged and finalized command-id and source-sequence indexes,
`last_finalized_source_sequence`, finalized commands and fences, epoch, sequencer,
window width, epoch-reset records, frontier, and last fence hash. The command cohort is
not evaluated. `F := effective_time` and `ActiveRequest := None` (D-2 unchanged: the
frontier tracks the completed **horizon**, not command success). Scheduled cohorts
already committed by this request remain committed; nothing of any earlier request or wave
is rolled back; there is no whole-request or whole-cohort rollback.

Host remedies, all ordinary new requests subject to `h ≥ F`: P-2 re-present under the
current epoch; P-4/P-5 the Phase-1 remedy, an ADR-0003 §11 epoch handoff, then a new
request; P-7/P-8 a fresh identity (a redelivered, already-finalized command lands here
harmlessly, §5.4); P-9 a strictly greater source sequence. P-3 and P-6 are unreachable and
are reported as invariant findings.

### 4.8 Loop step A6 (replaces the FINAL §9 A6 body)

```text
A6  if R is a Command:
        if not (1 ≤ r or admitted_executable == 0):   goto A7   (command deferred; weighs one unit)
        match Engine::finalize_command(R):
            Ok(record)     → execute the command cohort at now = effective_time (FINAL §13 wave rule);
                             result := COMPLETED
            Err(refusal)   → timeline byte-identical (§4.7); result := COMPLETED_COMMAND_NOT_FINALIZED { refusal }
    F := h;  ActiveRequest := None   [atomic];  emit PacingDiagnostics;  return result
```

A deferred command (`PAUSED`) leaves the timeline unchanged, because no preflight row and
no apply step has run; the next call re-enters A6.

### 4.9 Inherited semantics preserved

ADR-0003 §7 (a positive `STAGED` acknowledgement covering the envelope precedes the fence),
§8 (the fence's own no-promotion-on-failure rule), §13 (a convenience call is sugar over
the same eligibility, slots, acknowledgements, digest, frontier, and finality), and §14
(finalization and execution within one command barrier) are used as written. The
operation calls only public Phase-1 items: `current_admission_window`, `profile_id`,
`timeline_epoch`, `active_sequencer`, `slot_status`, `finalized_commands`,
`last_finalized_fence_hash`, `stage`, `submit_fence`, and
`compute_ordered_stream_digest`. PX-1 is the only additive surface mentioned, and it is a
proposal.

### 4.10 Executable regression

- **Historical red evidence, preserved.** The review's probe
  `v3_f01_final_independent_review_evidence_2026-09-10/source_sequence_regression.rs`
  still demonstrates, against unchanged Phase-1 code, that the FINAL recipe leaves ordinal
  1 positively staged after `SourceSequenceNotIncreasing`. It was re-run in this pass and
  is still red (evidence `historical-red-probe-rerun.txt`). Production Rust is not changed
  to make it pass.
- **Model.** The Revision-2 model replaces the flat command list with a
  Phase-1-faithful timeline and compares the complete timeline state on every route
  (checks F1 … F6).
- **Disposable Rust probe.** `rev2_finalization_probe.rs` implements this preflight over
  the **unchanged Phase-1 library** in a temporary external package: on eleven routes the
  REV2 refusal leaves the complete `TimelineIngress` state (its derived `Debug` rendering,
  which exposes every private field, plus both digests) byte-identical; the old
  composition mutates the live timeline on six of them; the success path equals the
  working-copy reference byte-for-byte; and a `--negative-control` run of the old
  composition fails the byte-identity assertion.

---

## 5. FINAL-02 — mismatch never consumes the real FIFO head

### 5.1 Request-bound results

Every `process` result carries `presented: RequestDiscriminator`, the `d(R)` of the
request passed to that call (decision D-9; results are non-canonical host values, so no
canonical encoding or digest changes). `REFUSED_ACTIVE_REQUEST_MISMATCH` additionally
carries `active`.

| Result | Terminates | Dequeue-eligible |
|---|---|---|
| `PAUSED` | nothing; the presented request is active and incomplete | no |
| `COMPLETED`, `COMPLETED_COMMAND_NOT_FINALIZED` | the presented request (it was the active request and has completed) | yes, iff `presented = d(head)` |
| `REFUSED_HORIZON_BEHIND_FRONTIER` | the presented request (it never started, and cannot start later: `F` never decreases) | yes, iff `presented = d(head)` |
| `REFUSED_ACTIVE_REQUEST_MISMATCH` | **nothing**; the active request remains active and its durable request must remain queued | **never** |

### 5.2 Dequeue eligibility

The consumer removes the head `H` **iff** the call it just made presented `H`
(`result.presented = d(H)`) **and** the result is `COMPLETED`,
`COMPLETED_COMMAND_NOT_FINALIZED`, or `REFUSED_HORIZON_BEHIND_FRONTIER`. No other event
acknowledges, pops, drops, replaces, reorders, or loses `H`.

### 5.3 Mismatch handling

- **Faulty presentation** (`presented ≠ d(H)`, e.g. a consumer defect presenting a
  different request while `H` is the paused active request): the engine refuses the
  presented request without mutation (FINAL §6.3); `H` stays at the head with the queue
  contents and order unchanged; the consumer continues by presenting `H`, which resumes.
- **Restore/mailbox disagreement** (`ActiveRequest = Some(a)` and `a ≠ d(H)`, detected
  either by the read-only `ActiveRequest` observation of FINAL §6.6 before presenting, or
  by a mismatch returned for `H` itself): the consumer **halts fail-closed** with
  `CONSUMER_HALTED_ACTIVE_REQUEST_NOT_AT_HEAD { active: a, head: d(H) }`. It presents
  nothing further and pops nothing. Resolution requires the host to restore a matching
  pair — the committed snapshot and a durable mailbox whose head is the request `a` — which
  is an operational action outside the engine. No abandonment, cancellation, replacement,
  reordering, or search-and-promote path exists.

### 5.4 The exact durable request is kept until completion

The durable mailbox keeps `H`, byte-for-byte, until `H` is dequeue-eligible. **Durability
ordering (consumer-contract requirement, not a mailbox implementation):** the consumer
makes the dequeue of `H` durable only after the engine's completing stable boundary is
durably committed. Consequences (model, deep-copy only, not crash evidence):

- a restart between the two re-presents `H` against the completed boundary: an
  `Advance(h = F)` completes with no cohort and a byte-identical stable boundary; a
  `Command` that was finalized is refused by P-7 (`COMPLETED_COMMAND_NOT_FINALIZED
  { CommandIdentityConflict }`) with the timeline and stable boundary byte-identical
  (model Q3); the producer can distinguish this redelivery by the refusal reason;
- the reverse order (dequeue durable first) can restore `ActiveRequest = Some(d(H))` with
  `H` gone; the consumer then halts per §5.3 (model Q3b). That is a detected fail-closed
  state, **not** safe recovery.

### 5.5 Replacement text

**FINAL §5, Next row (replaced):** "*Next* | consumer | — | The consumer removes the
head only on a result that terminates the request it presented, per §5.2 of the Revision-2
candidate; `REFUSED_ACTIVE_REQUEST_MISMATCH` terminates nothing and never removes the
head."

**FINAL §6.5, last paragraph (replaced):** "After a restore with `Some(d)`, the engine
accepts exactly the request `d` and refuses every other request with
`REFUSED_ACTIVE_REQUEST_MISMATCH`. The consumer re-presents the durable head only if its
discriminator is `d`; otherwise it halts fail-closed (Revision-2 §5.3)."

**FINAL §17 (replaced in full):**

1. Offer requests to a bounded FIFO; on backpressure, wait or drop at the producer.
2. Present only the head, with `process(head)`; repeat while `PAUSED`.
3. Remove the head only when the result terminates the presented head (§5.2). Never remove
   it on `REFUSED_ACTIVE_REQUEST_MISMATCH`.
4. Never present a different request while the head is incomplete — and if a defect
   does, the engine refuses it without mutation and the head is unaffected (§5.3). The
   contract is a liveness convenience; safety is the engine's.
5. If `ActiveRequest` is `Some(a)` and the head's discriminator is not `a`, halt
   fail-closed (§5.3); do not present, pop, drop, reorder, or replace anything.
6. Keep the mailbox durable across restarts if paused requests must survive them, and make
   a dequeue durable only after the completing stable boundary is durable (§5.4).

---

## 6. FINAL-03 — replay scope

### 6.1 History-only reconstruction applies to completed boundaries only

A request boundary can be reconstructed without a snapshot **only** when all of these
hold:

- (a) the boundary is **completed**: `ActiveRequest = None`;
- (b) the same genesis: initial stores, activated artifact, behavior epoch, and cap set
  (PE-D);
- (c) the complete **timeline metadata**: profile, genesis timeline epoch, genesis
  sequencer grant, genesis frontier, window width, every `EpochResetRecord` (with its
  frozen frontier and new sequencer), and the fence-identity rule of D-10;
- (d) the **finalized command stream** in ordinal order, each envelope **together with
  its canonical payload** — the content whose hash is the envelope's
  `canonical_payload_hash`. A payload hash does not reconstruct a payload, and a command
  discriminator does not reconstruct a command;
- (e) `F`.

**Procedure.** Build the engine from (b) and (c). For each finalized command in ordinal
order: apply every reset record whose frozen frontier equals that ordinal; `process`
`Advance(effective_time)` to completion; `process` the command to completion, requiring
`COMPLETED`. Apply any remaining reset records; `process` `Advance(F)` to completion.
Verify the recomputed `canonical_history_digest` and `stable_boundary_digest` against the
recorded values; any difference is a typed, non-canonical `REPLAY_DIVERGED`.

**Claim.** Under (a)–(e) the reconstruction reproduces the `stable_boundary_digest` and the
complete timeline state. Requests whose finalization was refused, and all advance
horizons, are intermediate horizons that Theorem P′ makes irrelevant; pacing is
irrelevant by Theorem P′-pacing (model R3: original at budget 1 with pauses, a refused
finalization, and an epoch reset; replay at budget 9). **Negative controls:** a different
window width yields an equal `canonical_history_digest` but a different stable boundary;
omitting a reset record makes a later-epoch command unfinalizable (model R3b). Envelope-only
replay therefore does **not** reproduce Phase-1 fence, reset, or state digests without (c).

### 6.2 Paused recovery requires the snapshot and the exact request

Two reachable boundaries can share finalized history and `F` yet differ in paused
progress — the review's case: the same genesis, empty finalized history, `F = 0`, one
before any progress and one paused after a budgeted `Advance(20)` processed work at 10.
Their engine and stable-boundary digests differ, and history plus `F` carries no active
request or progress (model R1). Paused recovery therefore requires the **committed
snapshot** — stores, timeline, `F`, `ActiveRequest`, and the recorded
`stable_boundary_digest`, validated together (FINAL §6.5 as amended in §7) — **plus the
exact durable mailbox request** whose discriminator equals `ActiveRequest`. Restoring the
snapshot and presenting that request continues byte-identically to the uninterrupted run;
a substitute is refused (model R2, S8b).

### 6.3 Withdrawn and narrowed statements

- FINAL §16 "A paused request is not history; its partial progress is time-ordered work
  any later request with an equal-or-greater horizon consumes identically" — **WITHDRAWN.**
  While `ActiveRequest = Some(d)`, no request other than `d` can start, whatever its
  horizon (FINAL §6.3; model S7b). A paused request's progress is resumable only by `d`.
- FINAL §16 "Canonical history is the finalized command stream (timeline) plus `F`" —
  **NARROWED** to: canonical finalized history is the timeline's finalized record
  (commands, fences, epoch resets); reconstructing a completed boundary additionally
  requires §6.1(b)–(e).
- FINAL §19.1 row "§2 replay and restart — RETAINED" — **NARROWED**: replay per §6.1;
  restart per §6.2.
- FINAL §22 residual 1 — **RESTATED**: an `Advance` discriminator's explicit kind and
  horizon fields determine that request exactly (its identity is `H(T)`), so presenting
  `Advance(horizon)` is a presentation of the same request. That is an identity fact, not
  an endorsed mailbox-loss recovery procedure. A command discriminator carries only a
  digest; it cannot reconstruct the command. Losing a paused command's durable request is
  **not** safe recovery: the engine stays fail-closed and no abandonment path exists.

No real process-crash persistence is claimed; the model's snapshots are deep copies.

---

## 7. Restore validation (amends FINAL §6.5)

Inserted between FINAL §6.5 steps 2 and 3: **2b.** The snapshot's timeline satisfies
I-CS (no slot), else typed `RESTORE_REJECTED_TIMELINE_STAGING_PRESENT` with nothing live.
A snapshot is only ever taken at a stable boundary, where I-CS holds, so this rejects only
corrupted or foreign snapshots (model F5). Steps 1, 2, and 3 and their order are unchanged.

---

## 8. Oracle precision (companion oracle)

| Review item | Correction |
|---|---|
| AT-I43(f) did not pause a Command | Pause `Command@20(p1)`, present `Command@20(p2)` (same kind, same horizon): refused byte-identically. A kind+horizon-only discriminator accepts it (model P-a), so this kills that mutant; the old Command-while-Advance fixture does not |
| AT-I47(a) overclaimed field-wise kills | Five-engine discrimination kills omission of `ActiveRequest`, of `F`, and of `identity`; it **cannot** kill omission of `kind` or `horizon`, which `identity` binds (model P-b). Those are killed by the AT-I35 byte-level encoding pins |
| AT-I40(e) "obligation removals equal" | Resulting stores equal; **removed sets differ** by `X`'s complete contested claim set (model P-c) |
| AT-I39(C) selection count | Counted at the outer-loop seam A2 only: `slices consumed + 1`. Extraction preflight and X-3 also locate the least slice, so a count of all X-1 uses is not `slices + 1` (model P-d: 3 versus 7) |
| AT-I43(a) ambiguous lifecycle | `A@10`'s wave-0 rule enqueues `D` at 15; with budget 1 the successive calls of `Advance(20)` return `PAUSED` (after 10), `PAUSED` (after 15), `COMPLETED` (after 20, `F = 20`), then `Command@21` returns `COMPLETED` (model P-e) |

---

## 9. Consistency edits (complete list for this revision)

| # | Edit | Forced by |
|---|---|---|
| CE-10‴ | A command is finalized by the engine-internal operation of §4 — complete preflight, then entailed stage-then-fence — and executed in the same command barrier; any refusal leaves the whole timeline byte-identical and completes the horizon. Replaces CE-10″ | FINAL-01 |
| CE-16 | I-CS holds at every stable boundary; restore rejects staging | FINAL-01 |
| CE-17 | Results are request-bound; mismatch terminates nothing; the consumer halts on head/`ActiveRequest` disagreement; dequeue durability follows completion durability | FINAL-02 |
| CE-18 | History-only reconstruction is scoped to completed boundaries under §6.1(b)–(e); paused recovery requires snapshot plus exact request | FINAL-03 |

---

## 10. Model and probe summary

`engineering/phase2/v3_f01_rev2_bounded_correction_evidence_2026-09-10/model.py`
(standard library only) runs **58 checks, all passing**: the 34 checks of the FINAL model,
re-run under the new timeline and consumer, and 24 new checks.

| Group | Checks | What they show |
|---|---|---|
| Carried | S1 … S19 (34) | every FINAL claim still holds |
| FINAL-01 | F1, F1b, F2, F2b, F2c, F3, F4, F5, F6 | old recipe leaves staged residue (red control); REV2 byte-identical on 12 routes; old recipe mutates on 6; REV2 verdict equals the working-copy reference on clean states; success equals the reference; follow-on command finalizes instead of poisoning; I-CS at every boundary and on restore; D-2 retained |
| FINAL-02 | Q1, Q1b, Q1c, Q2, Q3, Q3b | faulty presentation leaves queue and engine unchanged and the head then resumes; the old terminal-pop rule loses the head (red control); restore/mailbox mismatch halts; durability ordering both ways |
| FINAL-03 | R1, R2, R3, R3b | same history and `F`, different pause; snapshot plus exact request; completed-boundary reconstruction; metadata negative controls |
| Oracle precision | P-a … P-e | §8 |

The disposable Rust probe (`rev2_finalization_probe.rs`, run by `run_rev2_probe.py`)
exercises the same mechanism against the unchanged Phase-1 library (§4.10). Neither the
model nor the probe is production code, a Phase-2 acceptance run, or crash evidence. The
model's hashes are truncated SHA-256 over JSON, not the production encoder.

---

## 11. Supersession map against the FINAL candidate

| FINAL section | Disposition |
|---|---|
| §5 lifecycle, Next row | **REPLACED** (§5.5) |
| §6.3 row "Completion with command finalization refused" | **AMENDED**: "nothing staged/finalized" now means the entire timeline byte-identical (§4.7) |
| §6.5 restore validation | **AMENDED** (step 2b, §7); last paragraph **REPLACED** (§5.5) |
| §8.1 command request; D-1 | **RETAINED** |
| §8.2 finalization recipe | **REPLACED** by §4.1 … §4.6 |
| §8.3 finalization refusal | **REPLACED** by §4.7; D-2 retained |
| §9 A6 | **REPLACED** (§4.8); P0 … A5, A7 and all properties retained |
| §13 last paragraph (command cohort rejected at wave `n`) | **RETAINED** |
| §16 Replay bullet | **REPLACED** by §6.1 … §6.3 |
| §17 consumer contract | **REPLACED** (§5.5) |
| §19.1 row "§2 replay and restart" | **NARROWED** (§6.3) |
| §20 CE-10″ | **REPLACED** by CE-10‴; CE-16 … CE-18 added (§9) |
| §21 model summary | **SUPERSEDED** by §10 |
| §22 decisions | D-1 … D-5 **RETAINED**; D-6 … D-10 added (§12) |
| §22 residual 1 | **RESTATED** (§6.3) |
| Every other section | **UNCHANGED** |

---

## 12. Decisions taken in this revision, and residual risks

**Decisions (each flagged for the reviewer):** D-6 complete preflight plus entailed apply,
with the working copy only as a differential reference (§4.2); D-7 the clean-staging
invariant, fail-closed refusal of unexpected staging even where Phase-1 would accept, and
the restore check (§4.3, §7); D-8 an entailment violation is a fail-stop internal error,
never a timeline refusal (§4.5); D-9 request-bound results and the consumer halt (§5); D-10
`fence_id = "fence." ‖ decimal(n)` (§4.5).

**Residual and unresolved:**

1. Without PX-1, preflight rows P-7 … P-9 scan finalized history (`O(n)` per command).
   PX-1 is a proposal requiring its own decision.
2. On multi-fault inputs the reported refusal reason follows preflight row order, not
   Phase-1 call order (§4.4).
3. No `ActiveRequest` abandonment path exists (unchanged). A lost durable command request
   leaves the consumer halted; recovery needs a matching snapshot and mailbox.
4. No real crash-recovery, durable-mailbox, or Windows/Android executable parity evidence
   exists; the Rust probe uses the Phase-1 `test-support` constructor only to reach the
   ordinal-space-exhaustion states.
5. Model hashing is not the production canonical encoding; the model omits the Phase-1
   poison-evidence caps.

---

## 13. Verdict of this pass

`V3_F01_BOUNDED_CORRECTION_CANDIDATE_REV2` — submitted for independent Codex HIGH review.

- V3-F01: **OPEN**. Phase-2 architecture: **NOT frozen, NOT accepted.**
- PHASE_2_IMPLEMENTATION_AUTHORIZATION: **NO**. PHASE_3_AUTHORIZATION: **NO**.
- Phase 1: **CLOSED and not reopened.** G.A.M.E.: **unchanged.**
