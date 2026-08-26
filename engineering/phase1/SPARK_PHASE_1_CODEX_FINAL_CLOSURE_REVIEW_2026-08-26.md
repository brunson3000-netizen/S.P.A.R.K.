# S.P.A.R.K. Phase 1 — Codex Final Closure Review

**Date:** 2026-08-26
**Reviewer:** Codex, independent final-closure audit
**Branch:** `phase1-refoundation-v2`
**Exact HEAD reviewed:** `20a1c66244646dddb926d08fb9beb78d5cd9be13`
**Pre-production reference reproduced:** `87da5be`
**AT-H baseline commit reproduced:** `ee2e850`
**Production repair commit reviewed:** `b8b7414`

### 1. VERDICT

`PHASE_1_CLOSED`

The final B-01 repair conforms to the controlling admission architecture. The complete
retained Phase-1 state is either committed to canonical state identity or is an exact
derived index of committed state. Independent permutation and same-next-input probes found
no equal-digest/hidden-behavior divergence. The previously closed Phase-1 items remain
closed, the authorized m-02/S2/S3 changes preserve their required semantics, and no finding
at MAJOR or BLOCKER severity remains.

### 2. HISTORICAL DEFECT REPRODUCTION

I reproduced the inherited defect at detached commit `87da5be`, the exact last commit before
the final writer changed Rust production code. Three temporary in-crate probes passed by
demonstrating the defects themselves:

- opposite A/B command-ID contest orders reached equal state digests, then returned different
  whole `Result<StageDisposition, StageError>` values for the same later semantic reuse;
- the same equal-digest/later-divergence counterexample reproduced through
  `(source_id, source_sequence)`;
- after fence promotion, the command and source-sequence identities remained simultaneously
  present in the staged registries and the finalized registries.

I then checked out `ee2e850`, which contains the AT-H tests but not the production repair.
The independently rerun baseline matched the writer's recorded raw totals exactly:

| Target | Green | Red | Red assertions |
|---|---:|---:|---|
| `spark-testkit --test final_closure` | 4 | 5 | H1, H2, H4, H8, H11 |
| `spark-core final_closure_registry_invariant` | 0 | 2 | H3, H6 |

H5, H7, H9, and the valid-manifest companion pin remained green. The failures were the
specified staged-identity defect, staged-registry cleanup defect, or m-02 defect—not an
unrelated failure.

The matrix's summary line predicted H6 and H8 green, but each test's controlling detailed
specification requires the missing repair: H6(b) directly requires promoted staged entries
to be absent, while H8 extends H1/H4's contested-identity reuse invariant to all three-way
orders. Their inherited red results therefore confirm the architecture. This is a harmless
prediction-summary inconsistency, not a material contradiction and not an improperly
overridden stop condition. No test was weakened or adapted to make the writer proceed.

All temporary worktrees and probes were removed after evidence capture.

### 3. FINAL B-01 CLOSURE

Source inspection of `TimelineIngress::stage`, `submit_fence`, and `reset_epoch` confirms
the complete architecture transition table:

| Transition | Current behavior | Result |
|---|---|---|
| Empty slot, screening succeeds | Inserts the slot and both staged identity entries | Conformant |
| Empty slot, screening fails | Returns before touching the slot or either registry | Conformant |
| Exact staged duplicate | Idempotent acknowledgement; no registry mutation | Conformant |
| `Staged(A)` plus distinct `B` | Removes A's command and source-sequence entries, then installs canonical poison evidence | Conformant |
| Further poisoned-slot claim | Unconditionally inserts its semantic hash; no identity screening or registration | Conformant |
| Fence promotion | Removes each promoted staged entry while inserting its permanent finalized entries | Conformant |
| Epoch reset | Clears slots and both staged registries; preserves finalized identity and history | Conformant |

The poisoning removal keys come from the envelope already held in the slot, never from the
incoming claimant. Empty-slot identity screening ensures at most one semantically distinct
positively staged envelope owns an identity key. An equal semantic hash includes the input
ordinal and denotes the same envelope, so removing the slot occupant's entry cannot evict a
foreign registration. This safe-removal argument continues to hold across a window slide.

The staged registries now satisfy the exact bidirectional invariant:

```text
staged command map == command-ID/hash projection of current Staged slots
staged source map  == (source-ID, sequence)/hash projection of current Staged slots
```

The in-crate H3 test recomputes both maps after every scripted prefix containing stages,
contests, pile-ons, fences, and reset. H6 proves promotion moves rather than copies claims.
Finalized uniqueness remains permanent, including after reset.

Independent public-surface probes added further coverage beyond AT-H:

- all 2-way, 3-way, and 4-way single-slot permutations;
- all 24 global interleavings of two independent two-way contests over two ordinals;
- opposite first claimants separated by a partial-prefix fence/window slide;
- whole-result and digest comparison after reuse of every contested command ID and every
  contested source-sequence pair, plus fresh identities;
- poisoned fence attempts, epoch reset, and post-reset resubmission;
- finalized command-ID and source-sequence reuse after epoch reset.

All six temporary current-head adversarial tests passed. No arrival order left an
authoritative first claimant.

Diff inspection found no unauthorized digest/error/signature/fence-order change. The only
change to the new AT-H external file in the production commit was the clippy-equivalent
replacement of a one-element cloned slice with `std::slice::from_ref`; no assertion or
behavior changed. No pre-existing integration test file changed.

### 4. HIDDEN-STATE / STATE-EQUIVALENCE AUDIT

| Component | Retained future-affecting state | Commitment result |
|---|---|---|
| `TimelineIngress` | Profile, epoch, sequencer, width, frontier, fence anchor, staged/poisoned slots, full tracked evidence, finalized commands/fences, reset chain | Canonically committed |
| `TimelineIngress` | Staged command and source-sequence registries | Exact functions of committed `Staged` slots after every transition |
| `TimelineIngress` | Finalized identity maps and last finalized source sequence | Exact functions of committed finalized envelopes/history |
| `Scheduler` | Scheduled payloads or full bounded conflict claim set for every work key | Canonically committed; no executable conflict winner |
| `ActivationRegistry` | Activated identities, lineage records, committed-record set | Identities/set derive from committed lineage; lineage digest commits its records |
| `StateStore` | Activated artifact binding, definitions, cells, revisions/provenance | Immutable activation-derived structure plus canonically committed cells |
| Config/manifest values | Profile-qualified canonical content | Immutable value state, canonically hashed |

The audit rechecked every retained field in those types, not only the repaired timeline
maps. No uncommitted mutable cache or first-arrival winner was found.

The fresh interleaving sweep compared equal post-contest digests and then applied identical
future inputs to every state. All whole disposition values and every resulting digest
matched. It also reproduced the architecture's honest mid-contest residual: moving a
cross-ordinal reuse from before poisoning to after poisoning can change whether it stages,
but the resulting states have different canonical digests because one visibly retains the
extra staged slot. This is ordered-input behavior, not hidden divergence.

AT-G independently remains green for both scheduler and timeline evidence: hidden tracked
claims affect canonical identity and the reaction to the next claim, while claims genuinely
forgotten beyond the 256-claim tracking cap deliberately collapse. The 16-claim exposed
projection never substitutes for canonical retained evidence.

No equal-digest/different-behavior pair was found in any current-head probe or retained-state
analysis.

### 5. M-02 / S2 / S3 CLOSURE

**m-02 — closed.** `manifest_content_hash` canonically encodes every definition and sorts by
`(definition ID, full canonical definition bytes)`. It is therefore a multiset function
even for invalid duplicate-ID input, including multiplicity. Independent tests confirmed
opposite duplicate-ID orders hash equally, different multiplicities remain distinguishable,
and `ActivationRegistry::activate` still rejects each duplicate-ID manifest atomically.
For valid manifests, unique IDs mean the new tie-break never changes ordering.

**S2 — closed.** The shared `pub(crate)` `BoundedClaimSet` implements the same bounded,
commutative smallest-first claim tracking for scheduler conflicts and timeline poison
evidence. Both consumers retain their 16 exposed/256 tracked policy constants and unchanged
public accessors. The type adds no public API and contains no consumer-specific scheduling
or admission behavior.

I independently compiled and ran one identical canonical-value probe at `87da5be` and at
the reviewed HEAD. Every representative value was byte-identical before and after:

| Value | 2 claims | 17 claims | 300 claims |
|---|---|---|---|
| Scheduler state | `beca43a9...421c` | `1170249f...5c3` | `6ea0cf4d...44d` |
| Timeline state | `f474567a...b1ac` | `951afbd0...0839` | `a8a86695...ed25` |

The one- and two-definition valid manifest hashes also remained exactly
`8fefaaa9...1874` and `9bdc65bf...c12c`. These checks cross the exposed bound and the
tracking cap; the full AT-G suite covers all mandated sizes and is unchanged.

**S3 — closed.** Retaining only slots with `ordinal >= new_frontier` is equivalent to the
old lower-and-upper-bound predicate. A slot can only be inserted inside the current fixed-
width window, and the frontier/window end move monotonically upward, so no retained slot can
exceed the new window end. Independent probes confirmed partial-prefix retention, full-tail
finalization, a frontier beginning at `u64::MAX - 2`, preservation of the high staged tail,
and atomic typed rejection when finalizing at exhausted `u64::MAX`.

### 6. CLOSED-ITEM REGRESSION

| Item | Independent closure evidence | Status |
|---|---|---|
| B-01 admission/finality | AT-H, H3/H6 registry introspection, fresh 2–4-way/two-slot/slide sweeps | Closed |
| B-02 trusted activation door | External positive/negative compile probes and activation ceremony tests | Closed |
| B-03 scheduler | Full retained evidence in digest; conflicts have no executable winner; scheduler corpus green | Closed |
| B-04 immutable identity/fingerprint | All immutable identity fields remain activation-computed and discriminating | Closed |
| M-01 config identity | Profile/content hashing, order independence, and duplicate rejection remain green | Closed |
| M-02 manifest digest | Full canonical multiset hash plus valid-hash stability and activation rejection | Closed |
| M-03 panic-capable project API | Strict canonical lint gate and boundary/overflow corpus green | Closed |
| M-04 random addressing | Profile and behavior-artifact separation plus call-order independence green | Closed |
| m-01 dependency/features | Metadata policy tests green; `test-support` exists only on dev edges | Closed |
| Reconstruction restriction (AT-D) | Unvalidated continuation remains unavailable externally | Closed |
| Fable crate boundaries | `spark-core <- spark-engine <- spark-testkit` direction preserved; no product reverse edge | Closed |

`Cargo.toml`, `Cargo.lock`, and all crate manifests are unchanged from pre-production
reference `87da5be`. No dependency, public API, frozen contract, or Phase-2 implementation
was added. Static source inspection found no wall-clock, nondeterministic collection,
floating-point, OS-random, or project-owned unsafe primitive in canonical production code.

### 7. TEST-QUALITY RESULT

The 232/232 result is supported by test-content review rather than accepted as a count:

- AT-H compares canonical digest values and whole result values, covers both identity maps,
  proves the private derived-index invariant, pins unconditional evidence insertion, crosses
  reset/finalization, extends to all three-way orders, and pins m-02 plus valid hashes.
- The inherited red baseline was reproduced before judging the green result.
- AT-A through AT-G remain present and green, including full-semantic identity, activation,
  dependency/visibility, order-independence, bounded evidence, and hidden-state tests.
- The original Phase-1 minimum corpus 1–20 remains represented and green.
- The external compile harness passed its positive control and all 22 forbidden-composition
  probes, so privacy and construction restrictions are demonstrated from a real consumer.
- Historically insufficient tests are supplemented by AT-G/H and the fresh independent
  probes; same-error checks are not used as a substitute for same canonical state.
- No `#[ignore]` or `#[should_panic]` escape is present, no existing test was deleted or
  weakened, and the full count is 232 passed / 0 failed / 0 ignored.

The temporary review corpus contributed six additional current-head adversarial tests and
one identical pre/post encoding probe; all passed and were removed rather than becoming an
unreviewed permanent surface.

### 8. NEW FINDINGS

No new production, contract, or architecture defect was found.

**MINOR — AT-H baseline prediction summary inconsistency.** The matrix summary groups H6
and H8 among expected-green inherited checks, while their detailed specifications necessarily
require the missing repair and therefore predict red. The writer disclosed both deviations,
preserved the failing output, followed the controlling detailed assertions, and changed no
test to proceed. This documentation inconsistency does not affect the implementation or
closure verdict.

No MAJOR or BLOCKER finding remains.

### 9. TEST / TOOL RESULTS

Environment: `rustc 1.98.0 (88d9e12ae 2026-08-18)` and
`cargo 1.98.0 (797e8a9bc 2026-08-05)` on `x86_64-unknown-linux-gnu`.

| Command/check | Result |
|---|---|
| `git status --short` before report | Clean |
| `git branch --show-current` | `phase1-refoundation-v2` |
| `git log --oneline --decorate -15` | Expected linear history; reviewed HEAD `20a1c66` |
| `git diff --check 87da5be..b8b7414` | Pass |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Pass |
| strict core/engine clippy gate denying warnings, unwrap, expect, panic, indexing, arithmetic side effects | Pass |
| `cargo test --workspace` | 232 passed / 0 failed / 0 ignored |
| `cargo metadata --format-version 1` | Pass |
| workspace dependency-direction and production-feature policy tests | 3 passed |
| five Windows/Android `cargo check --workspace --target ...` checks | Pass |
| five Windows/Android `cargo check --workspace --all-targets --target ...` checks | Pass |
| exact historical defect probe at `87da5be` | 3 reproduced / 0 probe failures |
| exact AT-H baseline at `ee2e850` | Expected 7 red assertions, exact raw split reproduced |
| independent current-head closure probe | 6 passed / 0 failed |
| independent pre/post canonical-value probe | Identical outputs / 0 differences |

The installed static targets checked were `x86_64-pc-windows-gnu`,
`x86_64-pc-windows-msvc`, `aarch64-linux-android`, `x86_64-linux-android`, and
`armv7-linux-androideabi`.

### 10. PORTABILITY DEBT

Windows and Android were compile-checked only. Executable cross-platform determinism remains
unproven and is explicitly outside this Phase-1 gate; no runtime parity is claimed.

The documented fully qualified standard-library `Ord::clamp` reversed-bounds panic boundary
also carries forward. Project-owned clamp operations are total/fallible, canonical code does
not call the standard-library panic path, and removing `Ord` would harm deterministic ordered
collection contracts.

The admission architecture's mid-contest cross-ordinal timing residual carries forward as
ordered-input semantics. This review independently confirmed that any such difference is
visible in the state digest. No hidden divergence is accepted.

No Phase-2 evaluator, rule/effect runtime, persistence backend, service/network layer, or
platform adapter exists in the Rust workspace.

### 11. PHASE-1 CLOSURE

YES

### 12. PHASE-2 AUTHORIZATION RECOMMENDATION

NO

Phase 1 is technically closed, but Phase 2 remains outside this mission and requires a
separate explicit operator authorization against the readiness blueprint.
