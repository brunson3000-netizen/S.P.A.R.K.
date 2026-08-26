# S.P.A.R.K. Phase 1 — Fable Final Admission Architecture (B-01 Staged-Identity Repair)

**Date:** 2026-08-26
**Role:** Independent architecture specialist (Fable, HIGH effort), overnight autonomous mission
**Mode:** Architecture only. No production Rust was written; no implementation file was changed.
**Repository:** `/home/chromikey/Projects/SPARK`, branch `phase1-refoundation-v2`, HEAD `87da5be`
**Basis of record:** Controlling blueprint v0.2; ADR-0001…ADR-0006; the Fable architecture/process review of 2026-08-26; the Re-Foundation v2 implementation report and its Codex independent review; the final digest correction report; the out-of-scope staged-identity probe; and the current source of `spark-core`, `spark-engine`, and `spark-testkit` at HEAD, which outranks every report.
**Independent evidence:** `engineering/phase1/fable-overnight-baseline/FABLE_OVERNIGHT_PROBE_EVIDENCE_2026-08-26.txt` (probes P1–P7, reproduced against a verified 221/221 green tree).

---

## 1. The defect, independently reproduced and extended

`TimelineIngress` retains two in-flight identity registries for the current epoch:

```rust
staged_command_identity:         BTreeMap<CommandId, Digest>,          // timeline.rs:968
staged_source_sequence_identity: BTreeMap<(SourceId, u64), Digest>,    // timeline.rs:969
```

They exist to enforce module invariant 3 — command IDs and `(source_id, source_sequence)` pairs are unique within finalized-and-staged history — and they are populated in exactly one place: the **empty-slot** branch of `stage` (timeline.rs:1318–1321). Three structural facts follow, each confirmed by probe:

1. **Only the first arrival at a contested ordinal registers its identities.** When a second, semantically different envelope poisons the slot (timeline.rs:1361–1371), the poisoner's identities are never registered — and the *formerly staged* envelope's registrations are never removed, even though that envelope no longer occupies any slot and can never be finalized in this epoch. The registries therefore retain a first-arrival artifact.
2. **The registries are not committed to `canonical_state_digest`** (timeline.rs:1676–1706 commits history, epoch/sequencer/window, and slots only), and — unlike every other retained map in the kernel — they are **not derivable** from what the digest commits to: the poison evidence retains only semantic hashes, not the command IDs or source-sequence pairs of the contestants, and not which contestant arrived first.
3. **The identity checks are entirely skipped when the target slot is occupied**: a claim on a staged or poisoned ordinal goes down the occupied branches, where the registries are never consulted (probe P3).

Probe results (full source and output in the evidence file):

| Probe | Result |
|---|---|
| **P1** | Two ingresses fed the same two contesting envelopes in opposite orders reach **equal canonical state digests**, then react divergently to the same later reuse of the first claimant's command ID: `Err(CommandIdentityConflict)` vs `Ok(newly_staged)`, after which the digests diverge. Independent reproduction of the reported finding. |
| **P2** | **New:** the identical defect exists through the `(source_id, source_sequence)` registry — `Err(SourceSequenceConflict)` vs `Ok(newly_staged)` from equal digests. The original out-of-scope probe demonstrated only the command-ID half. Any repair that treats only `staged_command_identity` is half a repair. |
| **P3** | **New facet:** an envelope whose command ID conflicts with a *different* staged command is refused with `CommandIdentityConflict` when it targets an **empty** ordinal, but is accepted as a poisoning claim (evidence recorded, `Ok(poisoned)`) when it targets an **occupied** ordinal. See §4.3 for why this asymmetry is load-bearing and must be kept. |
| **P4** | An ADR-0003 §11 epoch reset clears slots and both staged registries, after which the two divergent ingresses fully re-converge (equal digests, equal subsequent behavior). The blast radius is bounded to the current epoch's unfinalized state. |
| **P5** | The crisp behavioral statement: after A stages and B poisons, reusing **A's** command ID is rejected while reusing **B's** is accepted — first arrival decides which contested identity stays reserved. That is arrival-order authority over canonical admission behavior, which the determinism constitution (blueprint §28) and ADR-0003 §11 ("no arrival-based winner selection") forbid. |

Severity calibration, unchanged from the correction report and re-verified: this cannot restore arrival-order authority over which payload *executes*, cannot make a poisoned slot finalizable, and cannot alter finalized history (`canonical_history_digest` is untouched — a poisoned slot never finalizes). It affects only whether a later command reusing a contested identity is admitted. It is nevertheless a genuine violation of both governing rules Phase 1 just finished enforcing everywhere else:

> **R1 (state commitment).** Every retained internal state value that can change future canonical behavior participates in canonical state identity.
> **R2 (order independence).** Canonical state reached from the same claim set is identical in every arrival order.

The registries currently violate both at once, and — as the correction report correctly analyzed — merely adding them to the digest would satisfy R1 by *breaking* R2 (the digests would become order-dependent, regressing B-01/AT-F1). The repair must change what the registries *contain*, not what the digest *sees*.

---

## 2. The governing principle

**Identity registration must be a pure function of digest-committed state.**

Everywhere else in the kernel this is already true: the scheduler's slots are the whole state; the timeline's finalized registries (`finalized_command_identity`, `finalized_source_sequence_identity`, `last_finalized_source_sequence`) are pure functions of `finalized_commands`, which the history digest commits to. The staged registries are the single exception, and the fix is to end the exception rather than to enlarge the digest:

> **The staged identity registries are a derived index over the positively staged slots — nothing more.** An envelope's identity claims are registered exactly while that envelope occupies a `Staged` slot, and at no other time.

Once that invariant holds, R1 and R2 are satisfied **structurally, with no digest change at all**: the digest already commits to every staged envelope in full (timeline.rs:1689–1695), so two ingresses with equal digests have equal slots and equal finalized history, hence equal registries, hence equal future admission behavior. This is the same shape as the Fable process review's B-02 resolution — don't audit the invariant, make the state incapable of violating it.

---

## 3. The chosen model: registration coextensive with positive staging

### 3.1 Semantics, transition by transition

Let `R` = the pair of staged registries. The complete rule set:

| Transition | Current behavior | **Required behavior** |
|---|---|---|
| Stage into empty slot (after identity screening passes) | register both entries | **unchanged** — register both entries |
| Empty-slot attempt fails identity screening | no trace | **unchanged** — no trace, no slot touched |
| Idempotent restage of a staged envelope | no registry action | **unchanged** |
| `Staged(A)` + distinct `B` → `Poisoned{h(A),h(B)}` | A's entries linger; B never registered | **remove A's two entries**; B still never registered |
| Further claim on a poisoned slot | evidence insert only | **unchanged** — evidence insert only, no registry action (§4.3) |
| Fence promotes `Staged` slots to finalized history | staged entries linger as duplicates of the new finalized entries | **remove each promoted envelope's staged entries** (the finalized registries take over, exactly as they already do) |
| Epoch reset | clear slots + both staged registries | **unchanged** |

After these changes the derived-index invariant holds at every observable point:

```text
staged_command_identity          == { env.command_id → env.semantic_hash          | Staged(env) ∈ slots }
staged_source_sequence_identity  == { (env.source_id, env.source_sequence) → env.semantic_hash | Staged(env) ∈ slots }
```

### 3.2 The safe-removal lemma

Removal on poisoning cannot evict another envelope's registration. Proof: an entry under key `command_id = c` was created by the envelope that staged into this slot, and registration screening guarantees at most one *semantically distinct* staged envelope per command ID. Could a semantically **identical** envelope be staged elsewhere and share the entry? No: equal semantic hash implies equal envelope (the hash covers every field, timeline.rs:260–270), which implies equal `input_ordinal`, which is this slot. So the entry belongs to exactly the envelope being deregistered. The same argument covers the source-sequence key. This lemma should appear as a comment on the removal site and as a derived-index invariant test (AT-H3 in the closure test matrix).

### 3.3 Resulting semantics for contested identities

A command ID or source-sequence pair whose only claim was swallowed by a contest becomes **unclaimed**: after the contest, a semantically different envelope may reuse it at another ordinal, identically in every arrival order (probes P1/P2/P5 flip from divergence to convergence). This is the coherent reading of the frozen poison rule — *no arrival picks a winner* means **neither** contestant's claim is honored, including its identity reservation. The contest itself remains fully explainable through the retained poison evidence, and recovery remains the explicit epoch reset. The alternative reading (both contestants' identities stay permanently reserved) is analyzed and rejected in §5.2.

Boundary cases, stated so the writer does not have to infer them:

- **Within-epoch resubmission of a contested envelope itself** is unaffected: its ordinal is poisoned, so it lands in the poisoned branch (evidence insert, `Poisoned` disposition), never in a fresh slot — an envelope's ordinal is part of its identity.
- **After finalization**, reuse of a *finalized* command's identity by a different envelope is still rejected via the permanent registries — unchanged, and existing tests `reused_command_id_for_different_envelope_at_different_ordinal_conflicts` (timeline_admission.rs:500) and `reused_source_sequence_for_different_command_conflicts` (timeline_admission.rs:520) keep passing without edits, because they contest against a positively staged, uncontested envelope.
- **After epoch reset**, everything unfinalized is void, as today (P4).
- The empty-slot screening's tolerance for `existing == semantic_hash` (timeline.rs:1291, 1303) becomes provably unreachable for the staged registries once the invariant holds (an identical envelope would target its own occupied ordinal), but it should be **kept**: it is correct, defensive, and still reachable through the finalized registries' `.or_else` chain for out-of-window retries that race a frontier advance.

### 3.4 What deliberately does not change

- **No digest change.** `canonical_state_digest` and `canonical_history_digest` encodings are byte-for-byte untouched. (Digest *values* for sequences involving contests naturally stay as they are; only future *behavior* converges.)
- **No public API change.** No signature, type, or error variant is added or removed. `StageError::CommandIdentityConflict` / `SourceSequenceConflict` keep their exact meaning — they now simply fire identically in every arrival order.
- **No frozen-contract change.** ADR-0003 §5's admission-time "command/source/idempotency validation" still runs, at the same place, with the same errors. §4 slot semantics, §6 retry, §7 acknowledgements, §8 fences, §11 poison/no-winner and epoch-reset recovery: all untouched.
- **No scheduler change.** The scheduler has no identity registries; its audit is clean (§6).

---

## 4. Design rationale

### 4.1 Why the registries must not enter the digest

The registries are arrival-order-sensitive today. Hashing them would make equal claim sets produce unequal digests by arrival order — a direct regression of AT-F1 ("opposite-order distinct pairs produce identical state digests") and of the B-01 closure. R1 must never be satisfied by committing state that violates R2; the correction report's refusal to "fix" this with a digest change was correct and is ratified here.

### 4.2 Why registration-on-poison (the union model) is rejected

The superficially conservative alternative — register **every** contestant's identities so contested IDs stay reserved — fails on four grounds:

1. **It registers identity claims for envelopes that were never admitted to anything**, inverting ADR-0003's admission discipline (out-of-window claims leave no trace; why would refused contest claims reserve global identity?).
2. **It needs its own conflict rule recursively.** If contestant B's command ID is already registered to a different staged envelope C, registering B's claim conflicts with C's — the model needs a rule for conflicts *between registrations*, which is the same problem one level down.
3. **It creates unbounded (or newly capped) retained state**: per-contest identity sets would need the same 256-cap discipline as the evidence sets, plus canonicalization, plus commitment to the digest — a strictly larger and more fragile change.
4. **It still isn't a function of committed state**, so the digest would have to grow anyway, and interleaved-reuse scenarios (a reuse arriving mid-contest) still produce order-dependent registration outcomes (whichever registration lands first wins the recursive conflict).

### 4.3 Why the occupied-slot identity bypass (P3) is kept, and must be documented

Probe P3 looks like a bug — identity screening applies at empty ordinals but not occupied ones — but the asymmetry is load-bearing: **poison evidence must be an unconditional, unscreened set-insert of the claimant's semantic hash**, because that is exactly what makes the evidence a commutative, idempotent function of the claim set (the property AT-B/AT-F1/AT-G all rest on). Screening the poison branch against the registries would make the evidence *set* depend on registry contents, i.e. on arrival order — reintroducing the defect inside its own repair. The rule to state in the module docs:

> Identity screening guards **admission into positive staging** only. A claim recorded as contest evidence is recorded by semantic hash, unconditionally. Evidence is a record of contest, not an admission.

This is the one place a well-meaning closure writer could silently break AT-F1 while "completing" the identity checks; the test matrix pins it (AT-H5).

### 4.4 Why not defer identity checks to fence time

Moving uniqueness enforcement out of `stage` into `submit_fence` would make registration trivially order-independent (only finalization registers), but ADR-0003 §5 freezes "command/source/idempotency validation passes" as an **admission rule**, and §7's positive acknowledgement would become a weaker promise (an acknowledged envelope could still die at the fence for an identity conflict acknowledged long before). This is a frozen-contract change with no compensating benefit; rejected.

### 4.5 The residual, honestly stated

Full arrival-order independence over *arbitrary interleavings that include identity-conflicting cross-ordinal claims* is not achievable while admission-time screening exists, under any model: a reuse claim arriving **mid-contest** (after the first claimant staged, before the poisoner arrived) is screened against the then-current registry and may be rejected in one interleaving and admitted in another. This is not hidden divergence — the resulting states differ **visibly** in the digest (a staged slot exists in one and not the other) — and it is not transport nondeterminism: `stage` calls are an ordered input sequence issued by the single active sequencer, and the determinism constitution requires identical outcomes for identical *ordered* inputs, which holds. ADR-0005's reversed-delivery requirement is about slot occupancy and finalization within a window for the *same* logical submissions, and is preserved. The invariant this architecture restores and the closure suite must assert is precisely: **per-slot contested state, including its effect on all future admission decisions, is a function of the claim set** — R1 and R2 together, which probes P1/P2/P5 falsify today and AT-H1/H2/H4 will hold green.

---

## 5. Secondary findings from the overnight audit

### 5.1 State-commitment audit (mission goal 2) — result table

| Retained state | Committed to a canonical digest? | Verdict |
|---|---|---|
| `Scheduler::slots` (scheduled payloads + full tracked conflict evidence + truncation flags) | yes, in full (scheduler.rs:603–623) | **SOUND** |
| Timeline finalized history: commands, fences, epoch resets, frontier, fence-chain anchor | yes (`canonical_history_digest`) | **SOUND** |
| Timeline live config (epoch, sequencer, window width) + staged/poisoned slots incl. full tracked poison evidence | yes (`canonical_state_digest`) | **SOUND** |
| Timeline `finalized_command_identity`, `finalized_source_sequence_identity`, `last_finalized_source_sequence` | derivable functions of `finalized_commands` (committed) | **SOUND** |
| Timeline `staged_command_identity`, `staged_source_sequence_identity` | **neither committed nor derivable** | **DEFECT — this document's subject** |
| `ActivationRegistry::identities` | derivable from the committed lineage (`lineage_digest`; records carry manifest + activation hashes, which fix all fingerprints) | **SOUND** |
| `ActivationRegistry::lineage` / `committed_records` | committed (`lineage_digest`); dedup set derivable | **SOUND** |
| `StateStore` definitions | fixed by the committed `activation_hash` (content addressing) | **SOUND** |
| `StateStore::cells` | yes, in full | **SOUND** |
| `ConfigRevision` | value object with unique representation; content hash total | **SOUND** |
| `LogicalClock`, `RandomAddressService` | monotonic host value / stateless pure function | **SOUND** |
| Testkit scenario harness | three digests, separated by role | **SOUND** |

### 5.2 Order-independence audit (mission goal 3) — result table

| Surface | Result |
|---|---|
| Scheduler conflict state, incl. at and beyond both caps | order-independent (verified corpus + AT-G) |
| Timeline poison evidence, incl. caps | order-independent (verified corpus + AT-G) |
| Timeline staged-identity registries | **order-dependent — the defect** |
| Activation: manifest content hash, activation hash, lineage under replay | declaration-order-independent; lineage is deliberately sequence-sensitive (activation *order* is real history) |
| Config revision: entries, duplicate diagnostics | order-independent |
| Epoch-reset evidence and post-reset convergence | order-independent (P4) |
| Drain/report/finalize orderings | stable total orders over semantic identity |

### 5.3 New minor finding m-02: duplicate-ID manifest content hash is construction-order-sensitive

`ProfileManifest::manifest_content_hash` sorts definitions by ID alone with a stable sort (manifest.rs:66–67), so a manifest containing two *different* definitions under one ID — invalid, and always rejected by activation — hashes differently depending on construction order (probe P6). No activated artifact, save, or epoch can ever reference such a hash, so this is cosmetic today; but ADR-0004 requires the canonical hashing representation to be deterministic and format-independent, and a hash function that is order-sensitive on any input is a latent trap for Phase-3 tooling that may hash unvalidated manifests (authoring diffs, artifact stores). **Recommended fix (one line):** sort by the full canonical encoding of each definition (or by `(id, encoded-block)`), making the hash a true multiset function. Classified MINOR; bundle into the closure pass (operator's option, §7).

### 5.4 Confirmed non-findings worth recording

- The `*ord <= new_window_end` arm of the post-fence slot retention (timeline.rs:1545–1547) is provably always true for surviving slots (any staged ordinal ≤ old window end < new window end); it is defensive dead logic, not a defect. Keep or simplify at the writer's discretion (S3 in §6).
- The `Ord::clamp` standard-library boundary and the Windows/Android static-only portability debt carry forward unchanged, as accepted by Codex.
- No `HashMap`/`HashSet`, wall clock, ambient RNG, float, or unordered canonical iteration anywhere in canonical code (re-verified by sweep).

---

## 6. Simplifications (mission goal 4)

| # | Simplification | Ruling |
|---|---|---|
| **S1** | **Staged-registry cleanup at finalization** (fold of the derived-index invariant): today promoted commands' entries linger in the staged registries as exact duplicates of the new finalized entries until epoch reset. Behaviorally inert (equal values behind an `.or_else`), but it is precisely the kind of redundant authoritative state this mission exists to remove, and the derived-index invariant is only provable once it's gone. | **Mandatory; part of the B-01 repair** (§3.1). |
| **S2** | **Unify `ConflictEvidence` (scheduler.rs:289) and `SlotPoisonEvidence` (timeline.rs:779)** into one shared bounded-claim-set type in `spark-core` (e.g. `evidence::BoundedClaimSet` with const-generic or constant caps). The two implementations are line-for-line structurally identical (insert, caps, truncation, full-set canonicalization) and their non-drift is currently maintained by doc comments on both sides — a convention where a type could stand. | **Recommended for the closure pass**; small, mechanical, removes a real drift risk before Phase 2 adds a third contested-state consumer. Public surfaces (`WorkKeyConflict`, `SlotPoisonEvidence` accessors) stay put; only the internal representation unifies. |
| **S3** | Simplify the always-true retention bound after fence promotion (§5.4) to `*ord >= new_frontier`, with a comment carrying the proof. | **Optional**; zero behavior change; writer's discretion. |
| **S4** | Drop the unreachable staged-registry `existing == semantic_hash` tolerance. | **Rejected** — keep the defensive check (§3.3); removing it buys nothing and costs the finalized-registry path its symmetry. |

No other redundant authoritative state was found: the finalized registries earn their keep as O(log n) indexes over genuinely committed history, and every digest encodes only non-derivable state (the AT-G correction already stripped the derivable counts).

---

## 7. Operator decisions required

1. **Authorize the Phase-1 closure implementation pass** implementing §3 (both registries), S1, and the AT-H corpus — the authorization the final digest correction report explicitly reserved for the operator. The contested-identity semantics ("contested identities become unclaimed", §3.3) is the recommended and, per §5.2's rejection analysis, the only coherent bounded option; it is called out here because it changes observable admission behavior rather than only internal representation. *Without this authorization Phase 1 cannot close*, because the tree currently violates R1/R2 in a place both Codex and the correction writer have flagged.
2. **Option:** include the m-02 manifest-hash fix (§5.3) and S2/S3 in the same pass (recommended — all are small, none touches frozen contracts) or defer them with m-02 recorded as accepted debt.

No other decision is required. Everything else in this document is engineer-authority realization within blueprint §35.2.

---

## 8. Verdict

**OPERATOR_DECISION_REQUIRED** — the closure architecture is complete and no frozen Phase-0 contract needs to change, but the implementation pass it specifies requires the operator authorization reserved by the previous pass (§7.1). Upon that authorization the plan in `PHASE_1_OPUS_FINAL_IMPLEMENTATION_PLAN_2026-08-26.md` is ready to execute immediately, with acceptance defined by `PHASE_1_FINAL_CLOSURE_TEST_MATRIX_2026-08-26.md` and independent review defined by `PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_PLAN_2026-08-26.md`.

PHASE_2_IMPLEMENTATION_AUTHORIZATION: NO

*End of admission architecture. No implementation files were modified by this mission.*
