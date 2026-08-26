# S.P.A.R.K. Phase 2 — Codex Architecture Freeze v2 Adversarial Rereview

**Date:** 2026-08-26  
**Reviewer:** Codex, independent bounded architecture reviewer (HIGH effort)  
**Repository:** `/home/chromikey/Projects/SPARK`  
**Branch:** `phase1-refoundation-v2`  
**Reviewed starting HEAD:** `9ff9d9b644408d7f324486c5caaa820eab4e3485`  
**Scope:** Phase-2 architecture and acceptance-test oracle only; no production Rust  
**Primary subjects:** corrected same-target effect semantics, indivisible-cohort pacing
progress, and later-wave causal emission identity

## 1. Verdict

`PHASE_2_ARCHITECTURE_V2_REVISE`

Architecture Freeze v2 genuinely closes the original `(target, resolved value)` defect
and its supporting gaps. It now preserves causal identity separately from numeric
payload, composes all distinct additive emissions, keeps exact redelivery idempotent,
rejects unordered update-family mixtures, reduces to one committed result per target,
evaluates thresholds only after complete reduction, preflights the complete transition,
represents contested obligations, allocates same-ledger occurrences canonically, reports
honest provenance coverage, and makes delayed work cross a strictly later boundary.

The v2 architecture is not yet safe to hand to the Phase-2 implementation writer. Two
newly attacked cases remain underfrozen:

1. a valid earliest equal-due-time cohort larger than `max_due_per_cycle` can be
   deferred forever; and
2. the frozen emission-context formula has no unambiguous parent for wave N+1 or later
   emissions derived from a reduction with multiple causal parents. Its inherited
   heartbeat-barrier component can also vary with pacing partition.

Both findings are local to the new Phase-2 architecture. Neither reopens Phase 1,
requires a new causal primitive, authorizes production implementation, or authorizes
Phase 3.

## 2. Lineage and canonical-evidence verification

### 2.1 Repository state at review entry

- Branch: `phase1-refoundation-v2`.
- Starting HEAD: `9ff9d9b644408d7f324486c5caaa820eab4e3485`, commit
  `Freeze the corrected Phase 2 architecture (v2) after Codex adjudication`.
- Entry worktree: clean.
- No repository `AGENTS.md` was present under the workspace.
- The Phase-1 closure commit `20a1c66244646dddb926d08fb9beb78d5cd9be13`, the
  original Phase-2 freeze commit `87ba1c22b4b1634e5205fb14bb6806a886df3f0b`, and
  the first Codex architecture-review commit
  `905880ed138b79f40b94d7fbaefa643406905030` are all ancestors of the starting HEAD.

### 2.2 Linear preservation of architecture history

The relevant history is additive and linear:

```text
87ba1c2  original Fable freeze + v1 matrix
  |
905880e  original Codex adversarial review (REVISE)
  |
9ff9d9b  Fable v2 freeze + v2 matrix
```

`9ff9d9b` creates exactly these two files:

- `engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v2_2026-08-26.md`
- `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v2_2026-08-26.md`

It does not edit the v1 freeze, v1 matrix, prior Codex review, Phase-1 evidence, or
production Rust. Blob comparison between the historical commits and the starting HEAD
confirmed that the v1 Phase-2 artifacts remain byte-identical:

- v1 matrix blob: `43c1d4873888030e231087a2c24bb1d8ca5b4505`;
- v1 freeze blob: `4fb1ca41ed9acc081ecc2eeafeba8ac17e34a7b8`;
- prior Codex review blob: `2e138279548f5668421a7ddfdc6d6af741d876a9`.

The v2 artifacts are therefore canonical committed repository evidence, and the prior
architecture/review history was not rewritten.

## 3. Controlling record reviewed

The review applied the blueprint precedence rule and read/reconciled:

1. `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md`, especially the authority,
   causal-grammar, lifecycle, feedback, deterministic-wave, boundedness,
   determinism/versioning, testing, and Phase-2 scope contracts;
2. all six Phase-0 ADRs, with particular weight on ADR-0002's immutable write classes,
   ADR-0003 §§14–15's command barriers and quota neutrality, ADR-0004's closed
   declarative semantics/change classes, and ADR-0006's obligation/replay/provenance
   requirements;
3. the Phase-0 requirement matrix and performance/security budget, especially R-038,
   R-044, R-055/R-056, R-086, R-096/R-098, and budget §3;
4. the closed Phase-1 final-closure authorization, implementation report, AT-H matrix,
   and independent Codex closure review returning `PHASE_1_CLOSED`;
5. `engineering/phase2/PHASE_2_FABLE_READINESS_BLUEPRINT_2026-08-26.md`;
6. the original Fable freeze and acceptance matrix;
7. `engineering/phase2/SPARK_PHASE_2_CODEX_ARCHITECTURE_ADVERSARIAL_REVIEW_2026-08-26.md`;
8. the v2 Fable freeze and v2 acceptance matrix.

The Rust tree was inspected only to verify inherited seams. In particular,
`WorkKey`'s total order is due-time first and `Scheduler::drain_due` removes the whole
due prefix. No Phase-2 production implementation exists.

## 4. Closure of the original blocking defect

The original review's primary counterexample was pre-wave `stress = 20` with distinct
causes `A: +10` and `B: +15`. V1 incorrectly converted them to absolute results `30`
and `35`, then rejected them as conflicting. The equal variant `+10/+10` incorrectly
deduplicated two causes to one `30`.

V2 closes that defect structurally:

| Required property | V2 contract | Result |
|---|---|---|
| Causal identity is not numeric result | Freeze v2 §3.2 hashes emission context, rule fingerprint, operation sub-ID, scope, target, and behavior artifact; payload is excluded | CLOSED, subject to the later-wave context defect in §7 |
| Distinct additive causes compose | §4.2 folds every distinct `AddDelta` in checked `i128`, then applies once to the pre-wave value | CLOSED (`20 + 10 + 15 = 45`) |
| Equal distinct causes are not deduplicated | Canonicalization is by emission identity, not value | CLOSED (`20 + 10 + 10 = 40`) |
| Exact duplicate emission is idempotent | Same identity + same payload folds once; same identity + different payload rejects atomically | CLOSED |
| Ordering does not invent mixed semantics | Cross-family groups and multiple transforms reject; sorting controls groups/reports only | CLOSED |
| One committed result follows typed reduction | Candidate intent survives evaluation; target-local resolution precedes `CommittedEffect` creation | CLOSED |
| Threshold sees only the complete result | §5.1 step 4 compares committed-old with fully-reduced-new and emits only into N+1/later work | CLOSED |
| Full atomic preflight | §7 covers effects, occurrences, cooldowns, obligations, scheduler admission, and the bidirectional invariant before mutation | CLOSED |
| Obligation collision representation | §8 uses a bounded order-independent content-addressed claim set per work identity | CLOSED |
| Same-ledger allocation order | §9 sorts by complete emission identity and preflights a checked consecutive range | CLOSED, once emission identity is completed per §7 |
| Provenance cannot alter arithmetic | §10 folds all candidates and caps explanation only, with ADR-0006 coverage metadata | CLOSED |
| No same-barrier delayed recursion | §11 requires `due_time >= current logical time + 1` and one due drain per barrier | CLOSED |
| Quota cannot split semantic causes | §6 makes an equal-due-time cohort indivisible | CLOSED for semantics, but progress is open per §6 |
| New retained state obeys R1/R2 | §8/§12 commit contested claims; buffers are transient; AT-I26–I29 apply discrimination/equivalence/recomputation tests | CLOSED for enumerated stores |

The AddDelta/RESULT/TRANSFORM classification is the smallest safe contract. It does not
invent a universal meaning for scale ordering, clamp intersection, decay-plus-shock,
or assignment-plus-delta. Equal RESULT coalescence is correctly limited to one declared
result family, and equal resolved transforms remain multiplicity-significant and reject.

## 5. Mandatory acceptance-test correction audit

Every mandatory correction in the original Codex review §9 is represented strongly
enough to falsify its intended invariant:

| Original mandate | V2 matrix evidence | Ruling |
|---|---|---|
| Replace value-based duplicate/conflict oracle | AT-I6a/I6b and AT-I7a–I7e assert exact values, write counts, provenance cause counts, whole-result permutation equality, and pre-wave digest preservation | SUFFICIENT |
| Stable snapshot for overlapping additions | AT-I1 pins `pre + Σdeltas`, clone equality, and absence of re-read/partial-fold behavior | SUFFICIENT |
| Ordering sorts results, not meaning | AT-I3/I5 permute rule declarations/IDs and compare ruleset hashes, engine digests, and reports | SUFFICIENT |
| Threshold only after complete reduction | AT-I25 uses opposing contributions that would create both false-positive and false-negative partial crossings | SUFFICIENT |
| Pin operation-pair classes | AT-I37 covers every mandated pair and digest-atomic rejection | SUFFICIENT |
| Deferral cannot split co-target causes | AT-I20 includes co-target additions, threshold inputs, scheduler/ledger/store/report equality, and false-crossing obligations | SUFFICIENT for no-split semantics; it lacks the progress case in §6 |
| Later/depth boundary and interference | AT-I21 proves strictly later time, single-drain exclusion, continuation priority, and deterministic boundary identity | SUFFICIENT for the adjudicated depth-budget contract; it lacks multi-parent emission identity in §7 |
| Decay/recovery interaction and exact math | AT-I22 pins formula/rounding and decay+shock rejection plus both sanctioned composition paths | SUFFICIENT |
| Aggregation multiplicity and ownership | AT-I24 pins equal child multiplicity, partition equivalence, and single reducer activation | SUFFICIENT |
| Late preflight failures | AT-I7e/I8 cover arithmetic, occurrence range, obligation cap, queue cap, and scheduler/store invariant failures with digest atomicity | SUFFICIENT |
| Contested obligations and multi-target artifact binding | AT-I12/I13/I15 cover claim-set invariants, second-target drift, permutations, cap edges, discrimination, removal, and recovery | SUFFICIENT |
| Same-key occurrence multi-allocation | AT-I16 pins payload-to-occurrence mapping across candidate permutations and mid-range exhaustion | SUFFICIENT once §7 supplies complete identities |
| Provenance coverage and hidden behavior | AT-I38 pins full arithmetic, exact coverage counts/status, pruning identity, and equal-digest/same-next-input behavior | SUFFICIENT |
| Extend hidden-state tests | AT-I26–I29 cover contested claims, cap edges, transient candidate/reduction/preflight buffers, and reconstruction | SUFFICIENT |

The matrix does not merely check that an error was returned. Its important claims use
exact arithmetic, complete result equality, engine/batch/store digests, canonical
reports, permutation corpora, compile-fail boundaries, and fresh-state reconstruction.

## 6. Blocking finding V2-B01 — an oversized valid first cohort can starve forever

**Classification:** local Phase-2 pacing defect; blocking; not foundational.  
**Affected contracts:** freeze v2 §§6.1–6.2 and matrix AT-I20.  
**Controlling invariant:** ADR-0003 §15 and performance/security budget §3 require a
runtime quota to change latency, not accepted-command semantics, and safely deferrable
due work must resume in deterministic order.

### 6.1 Concrete falsifying example

Let:

```text
max_due_per_cycle        = 4 WorkKeys
earliest due_time        = 100
WorkKeys at due_time 100 = K0..K5 (6 keys, one equal-time cohort)
next cohort              = due_time 101
```

Assume `K0..K5` jointly emit only six candidates/effects, all within
`max_cohort_candidates`, `max_effects_per_wave`, enqueue, queue, fan-out, and depth caps.
The cohort is therefore legal semantic work, not overload.

Freeze v2 requires the six-key cohort never to be split. It also says
`max_due_per_cycle` defers whole cohorts. If admission is `cohort_size <= remaining
pacing budget`, the first cohort is deferred. On the next cycle it is still the first
cohort and still size six, so it is deferred again. No newer cohort may pass it because
due-time order gives continuation priority. The legal queue makes no progress forever.

Section 6.3 does not help: its terminal consumption rule applies only when a semantic
cohort/wave cap is exceeded, which this example explicitly does not exceed. AT-I20 also
does not falsify this case; it assumes a deferred cohort later completes but never makes
the first cohort larger than the entire pacing budget.

There is a second pacing-oracle contradiction in the same local area. V2 requires a
deferral to be visibly reported, then requires paced and unbudgeted execution to have
identical returned reports. A run that reports a deferral cannot have the same complete
presentation value as a run that had no deferral. Quota neutrality applies to canonical
causal semantics, not to an honest latency diagnostic.

### 6.2 Smallest safe correction

Keep cohorts indivisible and keep semantic caps authoritative. Freeze this deterministic
pacing selector:

1. validate at activation that `max_due_per_cycle >= 1`;
2. select the maximal ascending prefix of whole equal-due-time cohorts whose total
   WorkKey count fits the pacing budget;
3. if that prefix is empty solely because the earliest legal cohort itself exceeds the
   pacing budget, admit exactly that one earliest cohort whole for this cycle;
4. report the deterministic pacing overrun (`admitted_work_keys`, declared budget, and
   cohort identity); and
5. defer every later cohort untouched.

If earlier cohorts consume part of a cycle's budget and the next cohort does not fit,
the next cohort defers. On the following cycle it is first and receives the same
one-cohort progress exception if still oversized. Semantic cohort/wave caps continue to
bound actual work and continue to use §6.3 terminal rejection; pacing never splits the
cohort or converts legal work into overload.

Also distinguish the canonical semantic cohort/wave report from noncanonical pacing
diagnostics. The former must match unbudgeted execution. The latter must honestly report
deferral/overrun and therefore is expected to differ. Neither diagnostic participates
in engine state identity or causal emission identity.

### 6.3 Exact test change

Add **AT-I20c `oversized_first_cohort_makes_progress_without_split`**:

- construct a first equal-due-time cohort with `N > max_due_per_cycle` but candidate,
  effect, enqueue, queue, depth, and fan-out volumes at or below every semantic cap;
- include co-target equal/unequal additions and opposing threshold inputs;
- assert that one cycle admits the entire first cohort exactly once, admits no later
  cohort, reports the deterministic pacing overrun, and leaves later cohorts untouched;
- compare cells, threshold emissions, obligations, scheduler, occurrence/cooldown
  ledgers, semantic cohort/wave reports, canonical candidate sets, and final engine
  digest with unbudgeted execution over the identical canonical input history;
- assert separately that pacing diagnostics truthfully differ (the paced run reports
  deferral/overrun and the unbudgeted run does not) and are causally inert; and
- include the boundary cases `cohort_size == budget`, `budget + 1`, multiple successive
  oversized cohorts, and activation rejection of zero.

This test must fail an implementation that loops by repeatedly returning a deferral
report without consuming the legal earliest cohort.

## 7. Blocking finding V2-B02 — later-wave emission identity has no stable multi-parent context

**Classification:** local but load-bearing Phase-2 identity defect; blocking; not a
Phase-1 or causal-grammar foundation change.  
**Affected contracts:** freeze v2 §§3.2, 5.1–5.2, 6.1–6.2, 9 and matrix AT-I5/I6,
AT-I16, AT-I20, AT-I25, AT-I29.  
**Controlling invariant:** semantic identity must be replay-stable and independent of
evaluation order, batching, pacing, and numeric coincidence; R1/R2 forbid hidden
future-affecting context.

### 7.1 Concrete falsifying example: threshold emission in wave N+1

At one scheduled equal-due-time cohort:

```text
WorkKey A -> AddDelta(+10) to stress
WorkKey B -> AddDelta(+15) to stress
pre-wave stress = 20
fully reduced stress = 45
threshold 40 crossing -> emit obligation O in wave 1
```

The wave-1 crossing is caused by the reduced set `{emission A, emission B}`. Freeze v2
§3.2 nevertheless requires the singular context:

```text
barrier identity || wave_index || triggering WorkKey identity digest
```

There is no truthful single triggering WorkKey. Choosing A or B by evaluation order is
nondeterministic. Choosing the lexical minimum is deterministic but arbitrary, collapses
the multi-parent cause to one parent, and changes if another lower-sorting contributing
WorkKey is added. Reusing one parent's identity can also collapse distinct reduced cause
sets. The `"command"` alternative does not apply to scheduled work and would not encode
the scheduled parent set.

The ambiguity propagates into occurrence allocation (§9), obligation record identity,
candidate-set/batch digests, exact-duplicate canonicalization, provenance, and mechanism
replay. AT-I25 proves when the threshold fires, but not that its emission has a unique
identity derived from all causal parents.

### 7.2 Concrete falsifying example: pacing-sensitive heartbeat context

V1's unchanged heartbeat barrier identity is:

```text
H("heartbeat" || logical time || digest of drained WorkKeys)
```

V2 keeps wave chaining to that barrier identity while allowing pacing to defer whole
cohorts. An unbudgeted drain can contain cohorts at due times 100 and 101; a paced run
can process them across two drain calls. The set of drained WorkKeys—and therefore the
barrier component of otherwise identical scheduled emissions—can differ solely because
of pacing partition. If a continuation instead retains the original drain identity
across calls, the architecture has not enumerated or digest-committed that retained
continuation state and cannot reconstruct it after a stable boundary.

Thus §6.2's claim of digest-identical paced/unbudgeted execution is not derivable from
the frozen identity formula. AT-I20 demands the equality but supplies no construction
that can satisfy it without redefining the context or adding hidden retained state.

### 7.3 Smallest safe correction

Do not add a new causal primitive and do not select a parent. Complete the existing
emission context using canonical identities already required by v2:

1. Define **scheduled cohort identity** as
   `H("scheduled_cohort_v1" || profile || due_time || count || ascending complete
   WorkKey identity digests)`. It names the entire equal-due-time cohort and is
   independent of which pacing cycle admits it. A command cohort uses its existing
   finalized command barrier identity.
2. Define an emission's **immediate parent-set digest** from the ascending set of
   complete parent emission identities that made that operation eligible:
   - scheduled wave 0: the singleton triggering WorkKey identity is the leaf context;
   - command wave 0: the finalized command identity/tag is the leaf context;
   - threshold/propagation wave N+1 and later: hash the complete sorted parent emission
     set of the fully reduced predecessor target group(s), never a selected WorkKey and
     never a numeric result.
3. Freeze the context as
   `cohort identity || wave_index || parent-set digest`, followed by the already frozen
   rule fingerprint, operation sub-ID, producer scope, target, and behavior artifact.
4. Require parent sets to be transient/recomputable from the canonicalized candidate
   and reduction groups already present in the wave pipeline. They must not become a
   new retained store. The batch candidate-set digest commits to the resulting complete
   identities as already required by §5.2.
5. Replace the drain-sensitive heartbeat component with scheduled cohort identity for
   the scheduled batch/emission hash chain. A heartbeat/drain-call partition and its
   deferral diagnostics are pacing metadata, not causal identity. This makes the same
   cohort's outer batch identity, idempotency, occurrence allocation, provenance, and
   obligation identity independent of how many drain calls were needed.

This uses the already frozen WorkKey identity, equal-time cohort, emission identities,
candidate canonicalization, and reducer parent set. It introduces no generalized
causal object, no arbitrary reducer order, and no hidden retained state.

### 7.4 Exact test changes

Add **AT-I6c `later_wave_multi_parent_emission_identity_is_replay_stable`**:

- two or more WorkKeys contribute to one fully reduced target and trigger one wave-1
  threshold/propagation emission;
- all WorkKey insertion, rule, candidate, reduction-group, and parent enumeration
  permutations produce the same complete later-wave identity, occurrence mapping,
  obligation record hash, wave report, batch digest, and final engine digest;
- cause sets with the same numeric reduced result but different complete parent
  emissions produce different later-wave identities;
- exact replay/redelivery of one derived emission is idempotent, while two distinct
  derived cause sets are not deduplicated; and
- adding or renaming a lexically lower parent cannot make it the arbitrary identity
  winner.

Strengthen **AT-I20/AT-I20c** so paced and unbudgeted execution compare complete wave-0
and later-wave emission identities, canonical candidate-set and committed-effect
blocks, semantic cohort/wave reports, outer scheduled-cohort batch digests, and final
engine state—not only final threshold outputs. Use at least two due-time cohorts so the
number of drain calls differs, while comparing noncanonical pacing diagnostics
separately as intentionally different.

Strengthen **AT-I25** to assert that the threshold emission's parent-set digest covers
every distinct canonicalized contribution used by the full reduction, while an exact
duplicate parent appears once.

Strengthen **AT-I29** with fresh reconstruction after paced deferral and before later
wave evaluation, proving that no uncommitted continuation/barrier-parent state is
needed to reproduce emission identities and next-barrier behavior.

## 8. General adversarial pass

| Revised decision attacked | Counterexample attempted | Ruling |
|---|---|---|
| Equal-due-time cohort semantics | Split equal-time additive/threshold parents; merge distinct due times and break catch-up | Correct cohort boundary; KEEP after B01 progress rule |
| Command vs scheduled cohorts | Collision across command and scheduled triggers | Finalized command barrier vs scheduled cohort domain separation is sufficient after B02 completes scheduled context |
| ADDITIVE/RESULT/TRANSFORM families | Equal adds, unequal adds, equal results across families, multiple equal transforms, add+scale, decay+shock | Closed typed reducer rejects or composes each without order semantics |
| RESULT equality/coalescence | Numeric equality across families/transforms | Correctly limited to same RESULT family; provenance retains distinct causes |
| Transform singleton | Two same-value scales and scale+delta | Rejects; no numeric-coincidence laundering |
| Decay/recovery | Coincident additive shock; two reducers; catch-up partitions | One reducer, explicit mixed rejection, exact closed-form vectors and chunk tests are sufficient |
| Aggregation ownership | Equal child values; second reducer; dynamic overlap | Children remain reducer inputs; activation owns one reducer; runtime result conflict remains defensive |
| Complete preflight | Late occurrence/store/queue/invariant failure | Full transition is checked before apply; tests compare every retained digest |
| Semantic-cap terminal rejection | Valid work mistaken for overload; repeated true overload | Semantic cap is epoch-bound and terminal; B01 separates legal pacing oversize from semantic overload |
| Contested ObligationStore | 3+ claims, insertion permutations, cap edges, poisoned drain | Claim-set representation plus R1/R2 tests is sufficient |
| Multi-target fingerprint binding | Drift only the second target | Canonical target/fingerprint set catches it atomically |
| Occurrence allocation | Multiple same-key claims in reverse enumeration | Sorted complete emission identities suffice after B02 defines later-wave identities |
| Provenance coverage | Same retained prefix, different omitted causes | Exact counts/status and full arithmetic prevent truncation from hiding retained behavior |
| Batch/candidate digests | Same committed numeric result, different cause set | Candidate-set digest discriminates mechanisms; B02 must first make every identity total |
| Stable snapshot/permutation | Re-read after partial fold; order-dependent transform | Borrow-split evaluation and closed reducer are sufficient |
| Hidden-state/equal-digest | Contested claims, wave buffers, pacing continuation | Enumerated stores pass in design; B02 forbids hidden continuation identity state and adds reconstruction test |
| Delayed recursion | Schedule at `now`; drain twice in one barrier | Strictly later logical time plus one drain structurally closes zero-time recursion |
| Panic-free totality | arithmetic/occurrence/cap extremes and forged surfaces | Typed errors, strict lint/compile probes, and AT-I33 are adequate architecture oracles |

No other concrete counterexample against a controlling invariant was found. Alternative
design preferences were not treated as defects.

## 9. Authorization and boundary result

- Phase 1 remains closed.
- No production Rust was written or authorized by this review.
- The Phase-2 implementation writer is **not authorized** from v2 as currently frozen.
- The smallest next step is an architecture v2 correction limited to B01/B02 and the
  named AT-I changes, followed by independent confirmation.
- Phase 3 remains unauthorized.

## 10. Review checks

The review performed:

- branch/HEAD/worktree and ancestry verification;
- `git show` inspection of the original freeze, prior review, and v2 commits;
- blob-level preservation checks for the v1 freeze/matrix and prior review;
- `git diff --name-status` and `git diff --check` over the correction commit;
- controlling-document and acceptance-oracle reconciliation;
- focused source inspection of `WorkKey`, `Scheduler`, `DrainOutcome`, and
  `Scheduler::drain_due` solely to verify frozen Phase-1 seams;
- traceability of every original mandatory test correction to v2 AT-I; and
- explicit counterexamples for pacing progress, pacing partition identity, and
  multi-parent later-wave identity.

Post-draft inherited gates on the reviewed tree:

- `cargo fmt --check`: PASS;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS; and
- `cargo test --workspace`: PASS, 232 passed / 0 failed / 0 ignored.

The review artifact is documentation-only; no production Rust or existing architecture
artifact was changed.
