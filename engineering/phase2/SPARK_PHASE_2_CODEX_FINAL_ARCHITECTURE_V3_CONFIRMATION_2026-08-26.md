# S.P.A.R.K. Phase 2 — Codex Final Bounded Architecture v3 Confirmation

**Date:** 2026-08-26
**Reviewer:** Codex, independent adjudicating reviewer
**Repository:** `/home/chromikey/Projects/SPARK`
**Branch:** `phase1-refoundation-v2`
**Starting HEAD:** `3fce3c624755170ec65480cd96e1e38c5ccea1c1`
**Reviewed provenance-correction commit:**
`3fce3c624755170ec65480cd96e1e38c5ccea1c1`
**Scope:** Phase-2 architecture and acceptance-test oracle only; no production Rust

## 1. Verdict

`PHASE_2_ARCHITECTURE_V3_REVISE`

Architecture Freeze v3 and Acceptance Test Matrix v3 close the two defects explicitly
returned by the v2 rereview: B01's oversized-earliest-cohort starvation and B02's
single-parent/drain-partition causal identity. The previously repaired v2 areas also
remain closed.

One concrete blocker remains at the boundary between the new cohort semantics and the
closed Phase-1 scheduler. The v3 partition-independence proof assumes that each cohort
is removed from canonical scheduler state immediately before that cohort's pre-wave
digest is taken, while later cohorts remain scheduled. The only inherited removal API,
`Scheduler::drain_due(now)`, instead removes the entire due-time prefix. Since the
scheduler digest participates in `engine_state_digest`, and that full pre-wave engine
digest participates in `effect_batch_v3`, catch-up/drain partition can still change a
canonical batch digest. The v3 matrix simultaneously says the scheduler is consumed
unchanged and requires those batch digests to be equal.

This is a bounded Phase-2 extraction-boundary defect. It does not reopen any Phase-1
semantic identity, encoding, conflict, authority, or determinism invariant. It does not
authorize production implementation or Phase 3.

The Phase-2 implementation writer is **not yet releasable** from v3.

## 2. Repository lineage and provenance correction

### 2.1 Additive canonical lineage

The relevant history is linear, additive, and contains no merge in the Phase-2 segment:

```text
f6665aa  independent Phase-1 final closure review
  |
87ba1c2  original Fable Phase-2 freeze + matrix v1
  |
905880e  first Codex adversarial architecture review
  |
9ff9d9b  Fable architecture freeze v2 + matrix v2
  |
d07ed38  Codex architecture v2 adversarial rereview
  |
ce3597f  original mis-attributed Fable v3 freeze + matrix v3
  |
3fce3c6  correctly attributed Opus v3 canonical replacement + matrix pointer correction
```

Every commit above, plus the Phase-1 closure commit `20a1c66`, is an ancestor of the
starting HEAD. No rebase, reset, replacement, deletion, or history rewrite was found.

The prior committed artifacts remain blob-identical at the starting HEAD:

| Artifact | Historical/current blob |
|---|---|
| original Fable freeze v1 | `4fb1ca41ed9acc081ecc2eeafeba8ac17e34a7b8` |
| acceptance matrix v1 | `43c1d4873888030e231087a2c24bb1d8ca5b4505` |
| first Codex review | `2e138279548f5668421a7ddfdc6d6af741d876a9` |
| Fable freeze v2 | `ce8a61691f1e188cdc80238b64cd96a99ed0c4e1` |
| acceptance matrix v2 | `4d6a0d9fe7a055af1eeb374027e53f208246f329` |
| Codex v2 rereview | `70c5645df8260d1e4b0a6270305b7bacb8cf1a16` |
| mis-attributed Fable v3 freeze | `23fc0e4bc12a23f841278cc2727bbd05db3fd50f` |

### 2.2 Provenance-correction content audit

Commit `3fce3c6`:

1. adds
   `engineering/phase2/SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md`;
2. changes the matrix's companion pointer from the mis-attributed Fable filename to the
   canonical Opus filename; and
3. adds explicit metadata-only correction notes.

The complete architecture body from the first horizontal rule through the v3 verdict is
byte-identical between the retained Fable-attributed file and the canonical Opus file.
Their only differences are the title, Agent line, and the additive provenance note. The
matrix diff changes only its companion pointer and adds the matching provenance note.
No test case, assertion, traceability row, scope rule, architecture decision, acceptance
requirement, or verdict changed. `git diff-tree --check` passes for both `ce3597f` and
`3fce3c6`.

**Provenance ruling:** the correction changed metadata/reference information only. It
preserved both Git history and the v3 architecture/test semantics.

## 3. Controlling artifacts reviewed

The review read and reconciled:

1. the Phase-2 Fable readiness blueprint;
2. the original Fable architecture freeze and acceptance matrix v1;
3. the first Codex adversarial architecture review;
4. the Fable architecture freeze v2 and acceptance matrix v2;
5. the Codex architecture v2 rereview;
6. the retained mis-attributed Fable v3 freeze;
7. the correctly attributed canonical Opus v3 freeze;
8. acceptance matrix v3;
9. the controlling Phase-0 blueprint, ADR-0003/0006, requirement matrix, and
   performance/security budget where cited by the reviews; and
10. the inherited Phase-1 `WorkKey`, `Scheduler::drain_due`, and scheduler digest seams.

Phase 1 remains closed. No production Phase-2 implementation exists.

## 4. Previously repaired areas reconfirmed

No cleared decision was reopened without a counterexample. The following contracts
remain sufficiently frozen, subject only to the extraction-boundary blocker in §7:

| Area | Controlling closure | Confirmation |
|---|---|---|
| Causal identity distinct from numeric result | v2 §3.2; v3 §4 | identity excludes payload and binds cause context |
| Additive same-target composition | v2 §4.2 | all distinct deltas fold in checked `i128`, applied once to the stable snapshot |
| Exact redelivery idempotency | v2 §3.2; AT-I6a/I6c(d) | same identity+payload folds once; contested payload rejects atomically |
| Incompatible update families | v2 §4.1–§4.3; AT-I7c/d/I37 | typed closed-family rejection, never sort-order semantics |
| One committed result per target | v2 §4.2/§5.1 | complete target-local reduction precedes `CommittedEffect` creation |
| Stable snapshot | v1 Q2; v2 §5.1; AT-I1/I2 | evaluation reads the one pre-wave snapshot; commit re-executes nothing |
| Threshold after full reduction | v2 §5.1; v3 §4.3; AT-I25 | committed-old to fully-reduced-new only; no partial crossing |
| Whole-transition preflight | v2 §7; AT-I7e/I8 | cells, occurrences, cooldowns, obligations, queues, and cross-store invariant preflight before mutation |
| Obligation collisions | v2 §8; AT-I12/I15 | bounded order-independent claim set represents all contested records |
| Same-ledger occurrence allocation | v2 §9; v3 CE-6; AT-I16 | complete emission identities sort a checked consecutive range |
| Bounded provenance | v2 §10; AT-I38 | coverage status/counts/pruning identity; cap bounds explanation only, never arithmetic or identity parents |
| Delayed-work boundary | v2 §11; AT-I21 | due time is strictly later and one barrier drains once |
| Canonical/hidden-state equivalence | v1 §8; v2 §12; v3 §4.6; AT-I26–I29 | retained stores are enumerated and digest-committed; parent/reduction buffers are transient |

## 5. B01 pacing-progress attack

### 5.1 Architecture result

The selector in v3 §3.2 closes the v2 starvation counterexample:

- cohorts are equal-`(profile_id, due_time)` and indivisible;
- normal admission is the unique maximal ascending whole-cohort prefix;
- an empty prefix admits exactly the earliest cohort whole, even when its `WorkKey`
  count exceeds `max_due_per_cycle`;
- the exception fires at most once per cycle and admits no later cohort;
- `max_due_per_cycle == 0` is rejected atomically at activation;
- admitted work leaves the head through commit or v2 §6.3 terminal semantic-cap
  rejection; and
- pacing diagnostics are structurally separate, noncanonical, deterministic for a
  given drain-call sequence, and unreadable by evaluation.

The progress proof is valid for a fixed finite due-work set: the number of cohorts ahead
of any cohort strictly decreases every cycle. Repeated oversized legal cohorts cannot
livelock. Semantic caps remain authoritative and terminal; the exception is not an
overload waiver.

### 5.2 AT-I20c boundary falsification audit

| Required boundary | Oracle strength | Result |
|---|---|---|
| `N == budget` | normal admission, overrun false, later cohort deferred | sufficient |
| `N == budget + 1` | whole earliest cohort exactly once, overrun true, later cohorts scheduler-digest-checked | sufficient |
| `N == budget - 1` and far above budget | pins greedy prefix and forbids repacking/lookahead | sufficient |
| three successive oversized cohorts | exact cycle order, once each, no skip/duplicate | sufficient |
| zero budget | registry/epoch digest equality plus compile probe, not error-only | sufficient |
| co-target additions and opposing threshold inputs | exact values, one target write, complete cohort membership, no partial crossing | sufficient |
| no starvation/livelock | positive admission and strictly decreasing due-key count every due cycle | sufficient |
| paced/unbudgeted equivalence | exhaustive canonical state/report/identity/digest equality | strong, but blocked by §7's extraction contradiction |
| semantic-cap authority | atomic digest preservation plus terminal key consumption | sufficient |

**B01 ruling:** the pacing-progress correction itself closes B01. The surviving blocker
is not a failure of the progress exception; it is the scheduler extraction/digest seam
needed to realize its partition-equivalence claim.

## 6. B02 later-wave identity attack

### 6.1 Architecture result

The v3 identity construction closes the v2 multi-parent and heartbeat defects:

- scheduled cohort identity is a domain-separated function of profile, due time, count,
  and the complete ascending `WorkKey` identity set;
- command cohort identity remains the finalized command barrier identity and is domain
  separated from scheduled cohorts;
- wave-0 scheduled and command leaves have distinct domain tags;
- every wave `N >= 1` derived emission uses the complete sorted set of immediate parent
  emission identities over every target group read to become eligible;
- exact duplicates are canonicalized before the parent set, so one parent appears once;
- no parent selector or order-based winner exists;
- different parent sets remain identity-distinct even when their numeric reductions
  match;
- parent sets compose transitively by embedding prior emission identities;
- occurrence allocation, obligation hashes, candidate-set digest, batch inputs,
  threshold emission, and mechanism replay consume the same total identity;
- heartbeat/drain-call labels and pacing diagnostics are excluded from causal identity;
  and
- parent sets are recomputable transient buffers, never retained continuation state.

The parent-set definition is sufficiently total for multiple WorkKeys, multi-target
reads, enumeration permutations, equal-result/different-parent cases, exact replay, and
fresh reconstruction. Background state that contributed no candidates correctly stays
out of emission identity and is discriminated at the batch/snapshot layer.

### 6.2 Acceptance-oracle audit

| Requirement | v3 oracle | Result |
|---|---|---|
| later-wave multi-parent identity | AT-I6c(a/b/e/f) pins hand-computed sets, every permutation, depth transitivity, and empty-set atomicity | sufficient |
| equal numeric result, different parent set | AT-I6c(c) requires identity/obligation/candidate-digest inequality with equal cell values | sufficient |
| exact derived redelivery | AT-I6c(d) pins once-only apply and contested-payload digest atomicity | sufficient |
| no selected parent | AT-I6d combines content/order controls with an unforgeable parent-set compile probe | sufficient |
| paced/unbudgeted identity | AT-I20/I39 compare all cohort/emission/obligation/occurrence/report/digest outputs | strong, but blocked by §7 |
| threshold parent coverage | AT-I25 recomputes the full digest and asserts inequality against every proper subset | sufficient |
| no hidden identity state | AT-I29 reconstructs at stable boundaries and compares next identities, hashes, mappings, reports, and digests | sufficient once §7 freezes the stable extraction boundary |

**B02 ruling:** the causal emission-identity formula closes B02. The surviving blocker is
the broader batch snapshot input/extraction boundary, not parent-set identity.

## 7. Remaining blocker V3-F01 — cohort extraction and pre-wave digest are inconsistent

### 7.1 Concrete counterexample

Start both runs from the same canonical state. The scheduler already contains two valid
scheduled items for one profile:

```text
C1 = one WorkKey at due_time 100
C2 = one WorkKey at due_time 101
```

No rule reads or mutates the scheduler, and both cohorts remain below every semantic
cap. Compare the catch-up partitions explicitly required by R-038 and AT-I39:

```text
Run U (one catch-up drain): Scheduler::drain_due(101)
  before C1 evaluation, inherited API removed C1 and C2
  scheduler digest in pre-wave engine digest = digest(empty)

Run R (one due time per drain): Scheduler::drain_due(100)
  before C1 evaluation, inherited API removed C1 but retained future C2
  scheduler digest in pre-wave engine digest = digest({C2})
```

The two scheduler digests differ by the Phase-1 R1/R2 contract. The engine digest
includes the scheduler digest (v1 §8). `effect_batch_v3` includes the full pre-wave
engine digest (v3 §4.5). Therefore C1's `effect_batch_digest` differs between Run U and
Run R solely because catch-up partition changed.

This falsifies v3 §4.5's claim that the pre-wave engine digest is necessarily identical,
v3 §3.4 canonical equivalence, R-038 catch-up equivalence, R-044 split-batch
equivalence, and AT-I20(h)/AT-I39's required batch-digest equality.

The same defect appears inside one due time with profiles P1/P2: the pacing selector may
admit `(time, P1)` and defer `(time, P2)`, but `drain_due(time)` removes both profiles.
The matrix nevertheless requires P2 to remain scheduled and untouched and says the
Phase-1 scheduler is consumed unmodified.

### 7.2 Why the apparent workarounds are not conforming

- **Drain everything and hold later cohorts in a buffer.** This makes the catch-up
  pre-wave scheduler digest depend on how much was due at the call. It also contradicts
  v3 §3.2(d)'s explicit requirement that deferred cohorts remain scheduled, untouched,
  and not copied into a buffer.
- **Drain everything, then reinsert later cohorts before C1.** Run U then hashes C2 in
  the pre-wave scheduler while an implementation that does not reinsert until after C1
  does not; the timing remains architecturally unspecified. Removal/reinsertion is also
  not “untouched,” and its atomicity relative to whole-transition preflight is not
  frozen.
- **Ignore the mismatch because final engine state converges.** AT-I39 correctly
  requires every per-cohort batch digest, not only the final state. Mechanism replay and
  batch identity are controlling semantics.
- **Exclude the scheduler difference only in tests.** That weakens the required oracle
  and would conceal the architecture defect.

### 7.3 Violated controlling invariant and impact

**Invariant:** runtime batching/catch-up/pacing may change latency and telemetry, never
canonical causal or batch identity; stable boundaries must commit the same semantic
state for the same cohort history (ADR-0003 §15, R-038, R-044).

**Impact:** `effect_batch_digest` and any mechanism replay keyed by it can diverge under
two expressly equivalent drain histories. The writer has no single conforming answer
because the freeze simultaneously mandates the old whole-prefix drain surface, later
cohorts untouched in scheduler state, full pre-wave engine commitment, and cross-partition
batch equality.

### 7.4 Smallest safe bounded correction

Revise only the Phase-2 scheduler-consumption boundary and its oracle:

1. Freeze an additive, deterministic **cohort-granular due-work extraction** operation
   (exact API name is not important) that:
   - obtains a read-only canonical view of due cohorts grouped by
     `(due_time, profile_id)` for the §3.2 selector;
   - atomically removes exactly the admitted current cohort immediately before that
     cohort's pre-wave engine digest is captured;
   - leaves every later/deferred scheduler slot byte-identical and resident; and
   - handles scheduled and conflicted slots without a retained continuation buffer.
2. Preserve every Phase-1 `WorkKey`, slot, conflict, ordering, canonical encoding, and
   digest semantic. This is an additive Phase-2 internal extraction surface, not a
   change to `Scheduler::schedule` or `drain_due` semantics.
3. Correct the v3 matrix's “Scheduler consumed, not modified” row to permit this one
   additive extraction surface while continuing to forbid a batch-scheduling API.
4. Strengthen AT-I39/AT-I40 with all future cohorts pre-scheduled before both runs and
   assert, immediately before every cohort batch, equality of the scheduler digest and
   full pre-wave engine digest across:
   - one-call catch-up versus one-due-time-per-call;
   - paced versus unbudgeted admission; and
   - a pacing cut between profiles at the same due time.
   Assert that deferred slots never leave the scheduler and that no transient due-work
   plan survives a stable boundary.

This correction is local, does not alter the v3 parent-set or pacing selector semantics,
does not reopen Phase 1, and adds no causal primitive or retained store.

## 8. NIM supplementary panel

The established shared NIM development-compute gateway was available and used. The
completed benchmark study was not rerun. Workers ran concurrently from one fixed
evidence packet and received no other worker's output.

| Role | Model | Result/use in adjudication |
|---|---|---|
| architecture consistency | `nvidia/nemotron-3-ultra-550b-a55b` | HTTP 404/not-deployed; raw failure preserved |
| architecture consistency fallback | `nvidia/nemotron-3-super-120b-a12b` | completed; no blocker returned |
| determinism/failure mode | `moonshotai/kimi-k3` | completed; identified the exact drain/reinsert seam but classified it as implementation choice; Codex rejects that classification because of the explicit pre-wave digest trace in §7.1 |
| adversarial identity | `openai/gpt-oss-20b` | completed; no B02 parent-set blocker returned |
| test oracle | `nvidia/nemotron-3-nano-30b-a3b` | completed; proposed a rule-fingerprint permutation blocker; rejected because v1 AT-I3 already requires a multiset `ruleset_content_hash` and per-rule content addressing, so no order-sensitive fingerprint counterexample was established |

Raw responses, returned reasoning fields, tasks, models, timestamps, token usage,
latencies, HTTP outcomes, input hashes, telemetry, and gateway provenance are preserved
under:

`engineering/phase2/codex_final_architecture_v3_confirmation_nim_evidence_2026-08-26/`

The fixed evidence-packet SHA-256 is
`fee3460689fa56b38b18e978f1cfd4caabba715c6182f1e96381b990a167e47e`.

NIM remained supplementary. Codex independently adjudicated the final result.

## 9. Acceptance-matrix overall ruling

The v3 matrix generally uses the right falsification discipline: exact values, complete
result/report equality, independently recomputed hashes, digest discrimination,
permutation corpora, fresh reconstruction, and compile-fail boundaries. It does not
substitute “an error occurred” where atomic state/digest equality is required.

The new B01 and B02 cases are materially strong. AT-I20(c/d), AT-I6(c/d), AT-I25, and
AT-I29 are sufficient for their local invariants. AT-I39 is also strong enough to expose
V3-F01, but its demanded equality lacks a conforming frozen scheduler transition and
conflicts with the matrix's unchanged-scheduler row. The acceptance-test oracle is
therefore not yet sufficiently frozen to release the implementation writer.

## 10. Checks performed

- branch, starting HEAD, worktree, commit subjects, and parent chain;
- `git merge-base --is-ancestor` for Phase-1 closure and every Phase-2 evidence commit;
- no-merge check across the Phase-2 segment;
- blob-level preservation of v1/v2/prior-review/mis-attributed-v3 artifacts;
- semantic/body diff of retained Fable v3 versus canonical Opus v3;
- exact correction-commit diff and `git diff-tree --check`;
- full Phase-2 blueprint/freeze/review/matrix reconciliation;
- focused Phase-1 source inspection of `WorkKey`, `Scheduler::drain_due`, and
  `Scheduler::canonical_state_digest`;
- explicit B01 boundary enumeration from AT-I20c;
- explicit B02 multi-parent, equal-result/different-parent, replay, pacing, and
  reconstruction falsification;
- every v3 acceptance group audited for value/digest/error-oracle sufficiency;
- bounded independent NIM panel with raw evidence preservation; and
- `git diff --check`: PASS;
- `cargo fmt --check`: PASS;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS;
- `cargo test --workspace`: PASS, 232 passed / 0 failed / 0 ignored; and
- installed-target static `cargo check --workspace` and `--all-targets`: PASS for
  `x86_64-pc-windows-gnu`, `x86_64-pc-windows-msvc`, `aarch64-linux-android`,
  `x86_64-linux-android`, and `armv7-linux-androideabi`. These are compile checks only;
  no cross-platform execution or digest-parity claim is made.

## 11. Boundary result

- Phase 1 remains **closed**.
- No production Rust was written or authorized.
- Phase 2 remains architecture-only and the implementation writer is not released.
- The only unresolved issue is V3-F01 and the bounded correction in §7.4.
- Phase 3 remains **unauthorized**.

`PHASE_2_ARCHITECTURE_V3_REVISE`
