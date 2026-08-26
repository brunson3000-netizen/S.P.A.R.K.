# S.P.A.R.K. Phase 1 — Re-Foundation v2 Implementation Report

**Date:** 2026-08-26
**Role:** Implementation writer (Claude Code, Opus, HIGH effort)
**Controlling architecture:** `PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md`
**Controlling brief:** `PHASE_1_REFOUNDATION_V2_IMPLEMENTATION_BRIEF.md`
**Repository:** `/home/chromikey/Projects/SPARK`
**Branch:** `phase1-refoundation-v2` (created from `244391d`, no worktree; history preserved, nothing rewritten)
**Phase 2:** NOT AUTHORIZED

---

## 1. Summary

The three surviving Phase-1 defect classes are closed by the structure the
architecture review specified, not by another layer of convention:

- **B-02 trusted activation** — the complete ceremony and every
  intermediate mint now live in one crate, `spark-engine`, behind one
  public door. There is no partial-trust step left to call, because the
  raw declaration type, the kind-tag mint, and every artifact constructor
  are `pub(crate)`. This is proved from outside by compiling real external
  consumer crates, not by asserting that an error was returned.
- **B-03 scheduler conflict** — first-arrival-wins is abolished. A second
  distinct payload for one complete `WorkKey` poisons the key, carrying
  order-independent bounded evidence; neither payload executes. The
  defining assertions are canonical-state-digest equality and drained-work
  equality across every arrival permutation.
- **M-03 panic-free canonical API** — both executed panic paths are gone,
  and the policy is enforced by lints rather than by audit. The lint gate
  and the external compile probes each caught a defect during this pass
  that a grep-style audit would have missed (§6.4, §8.3).

No frozen Phase-0 contract was reopened. No causal runtime primitive was
added. No Phase-2 code was written.

---

## 2. Commits

| Commit | Contents |
| --- | --- |
| `562ff20` | **Test-first baseline.** The mandatory AT-A…AT-F assertions encoded against the inherited tree, plus the recorded evidence. |
| `33ad640` | **The v2 implementation.** Crate reshape, trusted activation, scheduler conflict poisoning, panic-free policy, reconstruction restriction, ported corpora, external compile probes. |
| `d25856e` | **Conformance completion.** Poison-evidence inspection, the Unicode contract on both text types, the single-registry composition convention. |
| *(this report)* | Canonical report and phase-status update. |

Parent of the branch: `244391d` ("Authorize S.P.A.R.K. Phase 1
Re-Foundation v2 from Fable architecture"). The failed implementation, the
correction, the v1 re-foundation, every review, and the Fable architecture
report are untouched.

---

## 3. Step 1 — test-first baseline (brief §10)

Before any implementation change, the mandatory acceptance assertions were
written against the **inherited** tree from an external crate vantage
point and run. Recorded at
`engineering/phase1/refoundation-v2-baseline/`:

- `BASELINE_V2_ADVERSARIAL_FAILURES.txt` — raw output;
- `BASELINE_V2_UNEXPRESSIBLE_FINDINGS.md` — requirements whose baseline is
  "required API absent".

**Result: 11 of 12 runtime-expressible assertions failed.**

| Baseline assertion | Result | Class |
| --- | --- | --- |
| `baseline_valid_definition_spec_must_not_self_activate` | FAILED | B-02 (AT-A3) |
| `baseline_raw_declaration_must_not_reach_activation` | FAILED | B-02 (AT-A1) |
| `baseline_builtin_kind_bit_must_not_be_caller_assertable` | FAILED | B-02 (AT-A7) |
| `baseline_same_key_conflict_state_must_be_arrival_order_independent` | FAILED | B-03 (AT-B1) |
| `baseline_neither_payload_survives_a_conflict` | FAILED | B-03 (AT-B3) |
| `baseline_three_way_conflict_must_converge_in_all_orders` | FAILED (3 distinct states) | B-03 (AT-B2) |
| `baseline_conflict_evidence_cap_must_be_order_independent` | FAILED | B-03 (AT-B6) |
| `baseline_invalid_static_tag_must_not_panic` | FAILED (runtime panic caught) | M-03 (AT-C1) |
| `baseline_reversed_clamp_bounds_must_not_panic` | FAILED (runtime panic caught) | M-03 (AT-C3) |
| `baseline_resume_at_frontier_must_be_absent_from_production_surface` | FAILED | AT-D1 |
| `baseline_poison_evidence_must_distinguish_competing_pairs` | FAILED (identical digests) | AT-F1 |
| `baseline_poison_evidence_is_arrival_order_independent` | **passed** | preserved property |

The twelfth already held and was recorded as a property that must not
regress; it still holds.

Per brief §10, no test was faked with an unrelated malformed input. The
two v1 tests independent review found materially insufficient are restated
in their strengthened form here — `baseline_valid_definition_spec_must_not_self_activate`
uses a **fully valid** spec (the v1 test used one with an empty scope set,
so activation failed for an unrelated reason), and
`baseline_same_key_conflict_state_must_be_arrival_order_independent`
compares state digests and drained work rather than error shape and length.
Both fail against the inherited tree, which is precisely the evidence the
v1 corpus could not produce.

---

## 4. Step 2 — crate reshape (Fable §3.2, §8)

```text
spark-core     deterministic kernel; no trust ceremony, no profile types
                 id (+ canonical_tag!), hash, value (+ FixedRange), scope,
                 authority (vocabulary only), clock, random, scheduler, timeline
                 features = { test-support = [] }

spark-engine   THE Phase-1 trust boundary
                 profile::{definition, manifest, config, text}
                 activation::{ActivationRegistry, ActivatedProfile,
                              ActivatedDefinition, DefinitionKindTag, fingerprint}
                 state::{StateCell, StateStore}
                 fixture   [test-support only]
                 features = { test-support = ["spark-core/test-support"] }

spark-testkit  fixtures, scenario harness, composition convention,
               adversarial corpora, dependency/feature-policy tests
```

Resolved dependency direction, asserted mechanically from `cargo metadata`:

```text
spark-core    -> blake3 only
spark-engine  -> spark-core
spark-testkit -> spark-core + spark-engine
```

Files were relocated with `git mv`, so `spark-profile/src/{definition,
manifest,config,text}.rs` and `spark-core/src/state.rs` retain their
history under their new paths. `spark-profile` is retired as a crate name;
Fable §3.2 reserves it for a Phase-3 `spark-profile-io` parsing/authoring
layer above the engine. `spark-core::activation` and `spark-core::state`
no longer exist — a consumer holding only `spark-core` cannot name an
activated artifact, a state store, or any constructor for either, and two
compile probes prove it.

`spark-profile/src/validate.rs` was **absorbed, not ported**: its checks
are now steps of the single door, and `ValidatedManifest` is superseded by
`ActivatedProfile`, which additionally carries the manifest content hash.

---

## 5. Step 3–4 — B-02 trusted activation

### 5.1 The door

```rust
impl ActivationRegistry {
    pub fn new() -> Self;
    pub fn activate(&mut self, manifest: &ProfileManifest)
        -> Result<ActivatedProfile, Vec<ValidationError>>;
    pub fn fingerprint_of(&self, &ProfileId, &DefinitionId) -> Option<&Digest>;
    pub fn lineage_digest(&self) -> Digest;
    pub fn lineage_len(&self) -> usize;
}
```

`activate` performs the entire ceremony atomically: manifest checks
(duplicate IDs, profile qualification) → per-definition checks (non-empty
valid scopes; bounds and typing are already guaranteed by the validated
field types) → fingerprints **computed** from the validated fields →
immutable-identity comparison against the registry's committed lineage →
atomic commit → mint. A manifest that produces any error commits nothing
and reports *every* error, not just the first.

### 5.2 Surfaces removed or restricted

| Surface | Disposition | Proof |
| --- | --- | --- |
| `DefinitionDeclaration` (raw mint input) | crate-private | compile probe |
| `spark_core::activation` module | deleted | compile probe |
| `spark_core::state` module | deleted | compile probe |
| `DefinitionSpec::to_declaration` | removed | compile probe |
| `DefinitionKind::kind_tag` | `pub(crate)` | compile probe |
| `DefinitionKindTag::{builtin, custom}` | `pub(crate)` | compile probe |
| `StateStore::{new, from_activated_schema}` | do not exist | compile probe ×2 |
| `StateStore::from_activation` | `pub(crate)` | compile probe |
| `ActivatedProfile` / `ActivatedDefinition` constructors | none public | compile probe ×2 + doc-tests |
| `StateStore::{observe_host_owned, apply_spark_effect, commit_derived}` | `pub(crate)` | compile probe ×3 |
| `spark_engine::fixture` | `cfg(any(test, feature = "test-support"))` | compile probe |

`DefinitionSpec` keeps its public fields, per the architecture ruling:
authority facts legitimately *originate* as operator-authored profile
data. What was wrong was accepting them at a trusted mint without the full
ceremony, and that mint no longer exists.

`definition_fingerprint(&DefinitionSpec)` stays public as a pure
predictor — computing a fingerprint grants nothing, and a reviewer needs
to be able to recompute one independently. `valid_spec_activates_only_through_the_door`
asserts the door's computed value equals the predictor's.

### 5.3 Registry lifecycle across instances

The report makes no claim Rust cannot back. A second
`ActivationRegistry::new()` is possible, and pretending otherwise is what
kept this defect alive for three passes. The invariant is content
addressing plus binding:

- every `ActivatedProfile` and every `StateStore` carries **both**
  `manifest_content_hash` and `activation_hash`;
- `StateStore::canonical_state_digest` hashes both, so divergence reaches
  every downstream digest;
- `ActivationRegistry::lineage_digest` is a deduplicated append-only
  digest of the whole activation history.

`divergent_activations_are_never_interchangeable` (AT-A5) proves both
halves: two independent registries fed identical manifests agree
byte-for-byte on activation hash, manifest hash, lineage digest and store
digest; two fed divergent authority facts disagree on all four.
`registry_lineage_is_append_only_and_deterministic` (AT-A8) proves the
lineage is replay-stable, order-sensitive when contents differ, and
idempotent under repeated identical activation.

The composition rule — one process, one registry — is encoded as
`spark_testkit::composition::SingleRegistryRuntime` with tests stating the
honest guarantee: a second runtime is either byte-identical (harmless) or
visibly divergent (detectable). Phase 3 makes it structural.

### 5.4 Test-support hygiene

`test-support` is off by default on both canonical crates and enabled only
by `spark-testkit`'s **dev**-dependency edges.
`test_support_feature_is_never_enabled_through_a_production_edge` parses
`cargo metadata` and asserts that (a) neither crate lists it in `default`,
and (b) no normal or build dependency declaration anywhere in the
workspace enables it. It also asserts that the dev edge *does* enable it,
so the test cannot become vacuous by the seams going untested.

---

## 6. Step 5 — B-03 scheduler conflict

### 6.1 Semantics

```text
Empty            + P  ->  Scheduled(P)
Scheduled(P)     + P  ->  AlreadyScheduledIdempotent, no state change
Scheduled(P)     + Q  ->  Conflicted{h(P), h(Q)}   (P is NOT retained)
Conflicted{S}    + R  ->  Conflicted{S + h(R)}     (set insert; idempotent)
drain of a due conflicted key -> reported in DrainOutcome::conflicted,
                                 removed, never executed
```

`schedule` is total and returns `ScheduleDisposition` — a conflict is a
legitimate deterministic outcome, not an `Err`, exactly like a poisoned
timeline slot. `drain_due` returns `DrainOutcome { due, conflicted }`,
both in stable ascending `WorkKey` order. After a conflicted key is
drained the key is free, so a producer that resolved the ambiguity may
reschedule; no cancellation API was added, because it could race
submission and thereby "choose history", which ADR-0003 §11 forbids. The
Phase-2 batch-API requirement (AT-B7) is recorded in the scheduler
documentation as a contract, with no implementation.

### 6.2 Conflict evidence

`WorkKeyConflict` has private fields and no public constructor, and
carries the key, the retained competing payload hashes, an omitted count,
and a truncation flag. The state digest encodes scheduled and conflicted
slots under distinct domain tags, with the full evidence for conflicted
ones.

### 6.3 One deliberate refinement of the Fable proposal

Fable §4.2 specified "retain a deterministic smallest-hash set up to a
declared cap and count omitted distinct claims". Implemented literally as
*count evictions*, that is **not** order-independent: a duplicate of an
already-omitted claim increments the count in one arrival order and not in
another, so `omitted_distinct` — and therefore the canonical state
digest — would still have depended on arrival order past the cap. That is
the precise defect B-03 exists to eliminate, so implementing the letter of
the proposal would have reintroduced it.

The implementation therefore adds one bounded tracking cap and one flag:

- `MAX_CONFLICT_EVIDENCE = 16` — retained, exposed evidence;
- `MAX_CONFLICT_TRACKED_CLAIMS = 256` — distinct claims tracked, so the
  omitted count is **exact** up to that bound;
- `evidence_truncated` — set when the claim set exceeds the tracking cap,
  at which point the count saturates at `256 − 16`.

All three values are smallest-first functions of the claim set, so all
three are arrival-order independent at any claim-set size, and memory per
conflicted key is bounded at 256 digests. This is recorded as a deviation
in §11 and is the only place the implementation departs from the letter of
the controlling architecture; it departs in order to satisfy that
architecture's own stated requirement.

### 6.4 Timeline symmetry

`SlotState::Poisoned` gains the identical evidence representation
(`MAX_POISON_EVIDENCE`, `MAX_POISON_TRACKED_CLAIMS`), answering Codex
counterexample #12: two slots poisoned by *different* competing envelope
pairs are now distinguishable in `canonical_state_digest`, while the same
pair in either order still agrees. A further claim on an already-poisoned
slot is still refused but is now *recorded*; a set insert is idempotent
and commutative, so this cannot make state arrival-order sensitive. Every
closed B-01 hash/acknowledgement/fence property is untouched, and the
per-call `SlotPoisonRecord` keeps its per-call fields, because call
*results* may legitimately differ per call while canonical *state* may
not. `TimelineIngress::poison_evidence` exposes the evidence read-only, so
a poisoned slot is explainable and not merely reported.

---

## 7. Step 6 — M-03 panic-free canonical API

### 7.1 `CanonicalTag`

`from_static` is gone. It conflated compile-time literal validation with
runtime construction from a `&'static str`, and because a `const fn` only
evaluates at compile time when *called* in a const context, an ordinary
call site panicked at run time — the path independent review executed.

Replaced by:

- `CanonicalTag::try_from_static(&'static str) -> Result<Self, StaticTagError>`,
  total in every context;
- `CanonicalTag::validate_static(&'static str) -> Result<ValidatedStaticTag, StaticTagError>`;
- `CanonicalTag::from_validated(ValidatedStaticTag) -> Self`, total and
  infallible because the proof type has a private field and only
  `validate_static` produces it;
- `canonical_tag!("trigger")`, which binds a `const` item and therefore
  forces const evaluation.

`ValidatedStaticTag` is `Copy` deliberately: a value with a destructor
cannot be dropped during const evaluation, so `Result<CanonicalTag, _>` is
not const-usable while `Result<ValidatedStaticTag, _>` is. That is what
lets the macro force compile-time validation without any function that can
panic at run time. The macro's single const-evaluated `panic!` is the one
sanctioned exception to the `clippy::panic` gate, and the `allow` is scoped
to the macro's own `const` item — not to any call site.

### 7.2 `FixedPoint`

`clamp` (which delegated to `i64::clamp` and panicked on reversed bounds)
is replaced by:

- `FixedRange` — coherent by construction, private fields, only
  constructor validates;
- `FixedPoint::clamp(&FixedRange)` and `clamp_to(&FixedRange)` — total;
- `FixedPoint::checked_clamp(min, max) -> Result<_, IncoherentClampBounds>`
  — the ad-hoc form;
- `checked_add` / `checked_sub -> Result<_, FixedPointArithmeticError>`.

`ValueConstraint::fixed` reuses `FixedRange`, so a declared constraint and
a runtime clamp cannot disagree about what a coherent range is.

**A defect the compile probes caught.** Deleting the inherent `clamp` did
*not* remove the reversed-bounds panic: `FixedPoint` implements `Ord`, and
`Ord` supplies a provided `clamp` method that panics on `min > max`, which
silently took over. The external compile probe for AT-C3 compiled
successfully and failed the suite. The inherent `clamp` therefore
deliberately **shadows** the standard-library method with a
validated-domain signature, so the reversed-bounds call is now a *compile*
error. This is exactly the failure mode the architecture review predicted
for a grep-audited panic policy, and it was caught by mechanism rather
than by inspection.

### 7.3 Scope of the policy — stated, not claimed away

The rule enforced is: *no public function this project defines in a
canonical crate may panic, wrap, or saturate on caller input.*

`core::cmp::Ord::clamp` remains reachable through fully-qualified syntax on
every `Ord` type in Rust — `u64`, `String`, and every canonical newtype
whose `Ord` implementation ADR-0003's stable total ordering depends on.
Forbidding it would mean forbidding `Ord`, which would forbid the
`BTreeMap`/`BTreeSet` determinism the kernel is built on; it is in any case
already reachable through the public `u64` fields of types like `Ordinal`.
Clamping is a meaningful domain operation on exactly one canonical type,
`FixedPoint`, and that one is closed. This boundary is documented in
`spark_core::value` rather than left implicit.

### 7.4 Mechanical enforcement

Both canonical crates deny, crate-wide via `[lints.clippy]`:

```text
unwrap_used  expect_used  panic  indexing_slicing  arithmetic_side_effects
```

Allowances are scoped to `#[cfg(test)]` modules, to test targets, to the
one const-evaluated macro panic, and to one `const fn` byte scan whose
indices are proved in bounds immediately above each use.

The gate is live, not decorative. Enabling it surfaced two real unchecked
arithmetic sites in production code (`hash::hex` capacity, `random::
derive_fixed_fraction` modulo) and one unchecked subtraction in
`timeline::window_end`; all three are now checked. A deliberate probe
inserting `v + 1` and `Option::unwrap` into `spark-core` was rejected with
two errors, and the probe was reverted.

---

## 8. Step 7 — reconstruction restriction

`TimelineIngress::resume_at_frontier` is behind
`#[cfg(any(test, feature = "test-support"))]`. `TimelineIngress::new`
(frontier 0, genesis anchor) is the only production constructor. The
compile probe for AT-D1 proves the symbol is not nameable from an external
crate with default features, and the feature-hygiene test proves no
production edge can turn it on.

The Phase-3 evidence-consuming constructor is **documented as a contract
and not implemented**, listing all ten required fields: profile, timeline
epoch, finalized frontier, real last finalized fence hash, canonical
history digest, manifest content hash, config revision hash, behavior
epoch, sequencer grant, epoch reset chain — with the explicit rule that it
must never synthesize a genesis hash for a nonzero frontier.

`arbitrary_frontier_cannot_masquerade_as_a_validated_continuation` (AT-D3)
proves that a synthesized-frontier ingress can reproduce neither a real
finalized history digest nor a real fence hash.

---

## 9. Step 8 — closed-item preservation

Everything independently closed was ported, not rewritten, and nothing was
weakened.

| Item | Status | Evidence |
| --- | --- | --- |
| **B-01** semantic/admission separation, structured `StageAcknowledgement`/`FinalizationResult` | preserved unchanged | 24 timeline integration tests; ported adversarial finality tests; 2 compile probes; AT-F3 per-field `covers` mutation sweep over all nine semantic fields |
| **B-04** full fingerprint + atomic validation | ported verbatim into the door | 10 activation unit tests: identity-field sweep, duplicate rejection in either order, atomicity + corrected retry, foreign profile, profile independence, builtin/custom domain separation, declaration-order independence |
| **M-01** `ConfigRevision` | preserved unchanged, relocated | config suite + AT-E2a: three duplicate keys, three arrival orders, identical sorted diagnostics, no partial revision |
| **M-02** history/state/outcome-transcript digest model | preserved; transcript renamed `outcome_transcript_digest` | scenario suite incl. 50-repeat bit-identical replay, AT-F2 pairwise distinctness, AT-F4 context-via-state-digest |
| **M-04** profile/artifact-qualified `RandomAddress` | preserved unchanged | random suite + AT-E5 items 15–16 |
| **m-01** dependency direction | mechanism preserved, graph updated, extended with feature hygiene | 3 metadata policy tests |
| **Unicode contract** | decided and documented: deliberate byte distinction, no normalization | AT-C6 + doc contract on `CategoricalValue` and `BoundedText` |

**AT-E5** — the original Phase-1 minimum corpus (implementation brief §5,
items 1–20) remains represented. Items 1–10: `spark-core/tests/timeline_admission.rs`.
Items 11–13: `spark-engine` state/activation suites plus
`write_paths_still_enforce_every_validation_class`. Item 14:
`spark-engine` manifest suite. Items 15–18: asserted from the external
vantage point in `original_minimum_corpus_items_15_to_18_remain_proved`.
Item 19: scenario harness. Item 20:
`workspace_dependency_direction.rs`.

Two v1 tests were **replaced**, both because independent review found them
unable to catch the defect they named. `definition_spec_cannot_self_activate`
became AT-A3 plus four compile probes; `payload_conflict_rejects_in_either_order`
became AT-B1 plus AT-B2/B3/B4/B5/B6. Neither weak form remains in the tree.

---

## 10. Validation results

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | **PASS** |
| `cargo clippy --workspace --all-targets -- -D warnings` | **PASS**, zero warnings |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | **PASS**, zero warnings |
| Strict canonical lint policy (`unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `arithmetic_side_effects` on `spark-core` and `spark-engine`) | **PASS**; verified live by a deliberate reverted violation |
| `cargo test --workspace` | **PASS**, 213 passed / 0 failed |
| `cargo metadata --format-version 1` | **PASS** |
| Git working tree after the writer commits | clean |

### Test breakdown (213, up from 172)

| Target | Count |
| --- | --- |
| `spark-core` unit | 61 |
| `spark-core` timeline integration | 24 |
| `spark-engine` unit | 38 |
| `spark-testkit` unit (scenario + composition) | 11 |
| `refoundation_adversarial` (v1 corpus, ported) | 27 |
| `refoundation_v2_adversarial` (AT-A…AT-F) | 25 |
| `workspace_dependency_direction` (incl. feature hygiene) | 3 |
| `external_compile_probes` | 1 |
| doc-tests (`spark-core` 15, `spark-engine` 8) | 23 |

Of the doc-tests, **16 are `compile_fail`**. The single
`external_compile_probes` test performs **23 external crate
compilations**: one positive control that must succeed, and **22 forbidden
composition paths** that must each fail *and* whose compiler output must
mention the symbol under test, so a failure cannot be an unrelated typo.
The positive control exists because without it every negative probe would
pass vacuously.

### Cross-platform static checks

`cargo check --workspace` and `cargo check --workspace --all-targets`
both **PASS** on all five installed targets:

```text
x86_64-pc-windows-gnu        PASS
x86_64-pc-windows-msvc       PASS
aarch64-linux-android        PASS
x86_64-linux-android         PASS
armv7-linux-androideabi      PASS
```

**These are static checks only.** No Windows or Android link, execution,
fixture replay, or cross-platform digest comparison was performed, and
none is claimed. The portability debt is unchanged from the previous
report: link and execute the canonical fixture corpus on a declared
Windows build and on declared Android build(s)/ABI(s), compare canonical
fixture digests against the Linux reference, and establish the repeatable
CI/device/emulator gate.

---

## 11. Deviations, judgment calls, and residuals

Stated plainly, including the ones a reviewer would otherwise have to find.

1. **Conflict-evidence tracking cap and truncation flag (§6.3).** The one
   departure from the letter of the controlling architecture, made in
   order to satisfy its own stated order-independence requirement.
   Fable's proposal, implemented literally, would have left
   `omitted_distinct` arrival-order sensitive.

2. **`Ord::clamp` remains reachable by fully-qualified syntax (§7.3).** A
   named residual, not a closure. The rationale is stated rather than
   argued away: eliminating it means eliminating `Ord`, which the
   determinism constitution depends on.

3. **`Scheduler::contains` changed meaning** — it now reports "this key
   occupies a slot, scheduled *or* conflicted". `slot_status` and
   `conflict_of` give the distinction. Documented on the method.

4. **`ValidationError` is flat.** The v1 pair
   (`ValidationError` wrapping `SchemaActivationError`) collapsed into one
   enum, because manifest-level and activation-level checks are now steps
   of one ceremony rather than two layers. `DuplicateDefinitionId` and
   `ProfileMismatch` are the surviving names; no check was dropped.

5. **An empty manifest activates successfully**, producing an
   `ActivatedProfile` with zero definitions and an empty store. This is
   benign — no authority is granted and no state is writable — and no
   contract requires rejecting it. Flagged because it is a reviewer-visible
   choice, not an oversight.

6. **`ActivatedProfile` and `StateStore` derive `Clone`.** Cloning grants
   nothing: a clone is byte-identical, carries the same artifact hashes,
   and confers no capability the original did not already have.

7. **The compile-probe harness shells out to `cargo`.** It is the only way
   to prove absence from the *default-feature* surface, because during
   `cargo test --workspace` the testkit's dev edge enables `test-support`
   on the canonical crates, so a `compile_fail` doc-test could not honestly
   make that claim. It uses a temporary crate outside the workspace and a
   separate target directory; it adds roughly 5 seconds to the suite and
   no dependency.

8. **No new dependency was added.** The workspace still resolves
   `blake3` (canonical crates) and dev-only `serde_json` (policy test).

**No Phase-0 contract conflict was encountered. No new causal primitive
was required. No paid, cloud, or credentialed action was required. No
Phase-2 scope was necessary or written.**

---

## 12. Phase-2 scope audit

Nothing outside the Phase-1 boundary was implemented. There is no
propagation/effect evaluator, no delayed-obligation execution, no
persistence backend, no service or network transport, no actor behavior,
no MCI/game adapter, no dialogue or voice, no broad catalogs, no
scripting/plugin runtime, and no runtime LLM.

Two Phase-2 landing zones are *named* in documentation — the engine crate
notes that the rule/effect evaluator will live there (which is why the
store's write paths are `pub(crate)` rather than public anywhere), and the
scheduler records the atomicity requirement any future batch API must
satisfy. Naming a landing zone is not authorization to implement it, and
neither was implemented.

---

## 13. Completion

- Branch: `phase1-refoundation-v2`
- Implementation commits: `562ff20`, `33ad640`, `d25856e`
- Canonical report: `engineering/phase1/PHASE_1_REFOUNDATION_V2_IMPLEMENTATION_REPORT_2026-08-26.md`
- Transfer copy: `~/Downloads/PHASE_1_REFOUNDATION_V2_IMPLEMENTATION_REPORT_2026-08-26.md`

Writer completion does not close Phase 1. This pass must receive
independent Codex review at HIGH effort, with section-by-section
conformance against the Fable architecture review in addition to free-form
falsification.

`PHASE_2_AUTHORIZATION: NO`
