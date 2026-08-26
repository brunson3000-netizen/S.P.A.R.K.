# S.P.A.R.K. Phase 1 — Re-Foundation v2 Implementation Brief

**Date:** 2026-08-26  
**Status:** IMPLEMENTATION AUTHORIZED WITHIN THIS BRIEF  
**Phase 2:** NOT AUTHORIZED  
**Controlling architecture:** `PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md`

## 1. Mission

Implement the Fable architecture review exactly enough to close the three remaining Phase-1 classes:

- B-02 trusted activation / authority provenance;
- B-03 order-independent scheduler same-key conflict semantics;
- M-03 uniformly panic-free canonical construction/reconstruction policy.

Do not reopen Phase 0.
Do not add a causal runtime primitive.
Do not begin Phase 2.

Preserve every independently closed item:

- B-01 semantic/admission separation and structured finality evidence;
- B-04 full immutable definition fingerprint + atomic validation;
- M-01 ConfigRevision identity;
- M-02 history/state/outcome-transcript digest model;
- M-04 profile/artifact-qualified random addresses;
- m-01 dependency direction.

## 2. Branch / evidence discipline

Create a new implementation line:

```text
phase1-refoundation-v2
```

from the current canonical HEAD after the Fable architecture report is preserved.

Do not rewrite the historical failed implementation, correction, re-foundation, reviews, or Fable architecture report.

## 3. Required crate boundary

Implement the Fable ruling:

```text
spark-core
    deterministic kernel only:
    id, hash, value, scope, authority vocabulary, clock,
    random, scheduler, timeline

spark-engine
    THE Phase-1 trust boundary:
    profile::{definition, manifest, config, text}
    activation::{registry, ActivatedProfile, ActivatedDefinition, fingerprint}
    state::{StateCell, StateStore}
    fixture [test-support only]

spark-testkit
    fixtures, scenario harness, adversarial corpus,
    dependency / feature-policy tests
```

Required production dependency direction:

```text
spark-engine  -> spark-core
spark-testkit -> spark-core + spark-engine
spark-core    -> no S.P.A.R.K. product-layer crate
```

The former `spark-profile` functionality moves into `spark-engine::profile`. Do not keep a second live validation/activation implementation merely to avoid moving code.

## 4. Trusted activation architecture — B-02

### Single public door

The complete activation ceremony lives in `spark-engine`.

A caller may supply untrusted `ProfileManifest` / `DefinitionSpec` content, but may not call any intermediate mint.

Required conceptual surface:

```rust
pub struct ActivationRegistry { /* private fields */ }

impl ActivationRegistry {
    pub fn new() -> Self;

    pub fn activate(
        &mut self,
        manifest: &ProfileManifest
    ) -> Result<ActivatedProfile, Vec<ValidationError>>;

    pub fn fingerprint_of(
        &self,
        profile: &ProfileId,
        definition: &DefinitionId
    ) -> Option<&Digest>;

    pub fn lineage_digest(&self) -> Digest;
}
```

`activate` must perform the entire ceremony atomically:

```text
manifest checks
-> definition validation
-> profile qualification
-> fingerprint computation
-> immutable identity comparison
-> atomic registry commit
-> ActivatedProfile mint
```

### Opaque activation artifact

`ActivatedProfile`:

- public type;
- private fields;
- no public constructor;
- minted only inside `ActivationRegistry::activate`;
- carries at least:
  - profile_id;
  - manifest_content_hash;
  - activation_hash;
  - immutable activated definitions/schema.

It exposes read-only inspection plus:

```rust
pub fn into_state_store(self) -> StateStore;
```

### StateStore

- moved to `spark-engine`;
- no public constructor from raw schema;
- internal constructor `pub(crate)` only;
- no public state-write methods;
- state-write paths remain `pub(crate)` for future engine-internal evaluator/host facades;
- public surface is read-only inspection/digest;
- store carries manifest and activation hashes.

### Remove/restrict dangerous surfaces

These must not remain public production paths:

- raw `DefinitionDeclaration` mint input;
- `DefinitionSpec::to_declaration`;
- external `DefinitionKindTag::builtin`;
- public `StateStore::new/from_raw/from_activated_schema`;
- public authority/write mutation seams;
- any public constructor for `ActivatedProfile` or `ActivatedDefinition`.

### Registry lifecycle / divergent histories

Within one registry, full fingerprint identity remains immutable.

Across independent registries:

- identical manifests must produce identical activation/artifact hashes;
- divergent manifests/authority facts produce distinct hashes;
- downstream store/artifact hashes carry that divergence visibly;
- no fake claim that Rust can prevent a second registry instance.

### Test support

Use a `test-support` feature only for sanctioned fixture seams.

Production dependency-policy tests must verify that no production edge enables `test-support`.

## 5. Scheduler conflict architecture — B-03

Keep the complete `WorkKey`.

Replace first-arrival-wins conflict behavior with deterministic conflict poisoning.

Conceptually:

```rust
enum SlotState {
    Scheduled(WorkPayload),
    Conflicted {
        competing_payload_hashes: BTreeSet<Digest>,
        omitted_distinct: u64,
    },
}

pub enum ScheduleDisposition {
    Scheduled,
    AlreadyScheduledIdempotent,
    Conflicted(WorkKeyConflict),
}

pub struct DrainOutcome {
    pub due: Vec<DueWorkItem>,
    pub conflicted: Vec<WorkKeyConflict>,
}
```

Required semantics:

- Empty + P -> Scheduled(P)
- Scheduled(P) + P -> idempotent
- Scheduled(P) + Q -> Conflicted({h(P), h(Q)}); neither payload remains executable
- Conflicted(S) + R -> deterministic set insert
- drain of due conflict -> conflict reported, no payload executed
- canonical scheduler state digest includes conflict state/evidence
- same logical claim set in any arrival order -> identical scheduler state digest and drain output.

Conflict evidence must be finitely bounded and order-independent. Follow the Fable proposal: retain a deterministic smallest-hash set up to a declared cap and count omitted distinct claims.

For symmetry, timeline poisoned slot state may also retain deterministic bounded competing semantic-envelope hash evidence, provided B-01 closed semantics do not regress.

## 6. Panic-free canonical API policy — M-03

Adopt this project-wide rule:

> No public function in a canonical production crate may panic, wrap, or saturate on any caller input. Fallibility is `Result`; infallibility is earned through validated-domain types.

### CanonicalTag

Replace panic-capable `from_static`.

Implement:

- a total `try_from_static(&'static str) -> Result<...>`;
- a compile-time literal macro such as `canonical_tag!("trigger")` that forces const evaluation for project literals;
- runtime construction remains fallible.

Invalid runtime static strings return `Err`, never panic.

### FixedPoint

Remove panic-capable arbitrary-bound clamp.

Add validated range type:

```rust
pub struct FixedRange { /* private */ }
impl FixedRange {
    pub fn new(min: FixedPoint, max: FixedPoint) -> Result<Self, ...>;
}

impl FixedPoint {
    pub fn clamp_to(self, range: &FixedRange) -> FixedPoint;
    pub fn checked_clamp(self, min: FixedPoint, max: FixedPoint)
        -> Result<FixedPoint, ...>;
    pub fn checked_add(self, other: FixedPoint)
        -> Result<FixedPoint, ...>;
    pub fn checked_sub(self, other: FixedPoint)
        -> Result<FixedPoint, ...>;
}
```

### Lint enforcement

Canonical crates must mechanically deny relevant panic/arithmetic patterns.

Use project-appropriate equivalents of:

```text
clippy::unwrap_used
clippy::expect_used
clippy::panic
clippy::indexing_slicing
clippy::arithmetic_side_effects
```

Scoped test-only allowances are acceptable.

If a compile-time macro legitimately contains a const-evaluated panic, document and scope the lint allowance to that macro only.

## 7. Reconstruction / resume restriction

`TimelineIngress::resume_at_frontier` must not exist on the default production surface.

Move it behind:

```text
#[cfg(any(test, feature = "test-support"))]
```

Production dependency-policy test must verify `test-support` is not enabled through production edges.

Do not implement Phase-3 persistence reconstruction.

Document only the future evidence requirements from the Fable architecture:

- exact profile;
- epoch;
- finalized frontier;
- real last fence hash;
- canonical history digest;
- manifest content hash;
- config revision hash;
- behavior epoch;
- sequencer grant;
- epoch reset chain.

Never synthesize a fresh genesis hash for a nonzero production frontier.

## 8. Preserve closed architecture

Port/reuse, do not casually rewrite:

- `SemanticCommandEnvelope` versus `AdmissionTicket`;
- structured `StageAcknowledgement` and `FinalizationResult`;
- history/state/outcome-transcript digest separation;
- B-04 full fingerprint/atomic validation algorithm;
- ConfigRevision unique-key validated model;
- profile/artifact-qualified RandomAddress;
- metadata-based dependency-policy test.

Rename transcript field to `outcome_transcript_digest` as recommended.

Unicode canonical text remains byte-distinct; do not introduce Unicode normalization into canonical hashing.

## 9. Mandatory acceptance tests

Implement every AT group from the Fable architecture report.

At minimum include these blocker-critical assertions.

### AT-A trusted activation

- external raw declaration cannot activate;
- external StateStore construction without ActivatedProfile does not compile;
- fully valid DefinitionSpec cannot self-activate through any intermediate mint;
- store carries activation_hash + manifest_content_hash;
- identical independent activations produce identical hashes;
- divergent independent activations visibly differ;
- no public write paths;
- external caller cannot assert builtin/custom identity bit;
- registry lineage is deterministic.

### AT-B scheduler

- same-key A/B vs B/A -> identical canonical scheduler state digest;
- same drain output;
- neither A nor B executes after conflict;
- three-way conflict all permutations converge;
- exact duplicate into conflict is idempotent;
- conflict drain/report/reschedule deterministic;
- evidence cap is order independent.

### AT-C panic-free

- invalid `try_from_static` -> Err, no panic;
- invalid literal macro -> compile_fail;
- reversed clamp bounds -> typed Err, no panic;
- FixedRange incoherent -> Err;
- u64::MAX frontier repeated public operations -> no panic/no mutation;
- checked fixed-point overflow -> Err;
- Unicode byte-distinct contract.

### AT-D reconstruction

- production build cannot name resume_at_frontier;
- test-support is dev/test only;
- arbitrary test frontier cannot masquerade as validated continuation.

### AT-E/F closed regressions

- full B-01 timeline suite;
- full B-04 fingerprint/atomicity suite;
- M-01 config suite incl. >=3 duplicate diagnostics under reordered input;
- M-02 digest/replay suite;
- M-04 random-address suite;
- dependency graph + feature hygiene;
- per-field StageAcknowledgement::covers mutation sweep;
- different poison evidence pairs -> different ingress state digests;
- equivalent poison pair arrival orders -> same state digest;
- original Phase-1 minimum corpus 1-20 remains represented.

## 10. Test-first proof

Before implementation changes, add/port the v2 acceptance tests against the current inherited tree and record which fail/compile-fail.

Do not fake an impossible test with an unrelated malformed input.

Every "must be impossible" property must be demonstrated via a genuine external compile-fail/property test.

## 11. Scope boundary

Still forbidden:

- Phase-2 propagation/effect runtime;
- delayed-obligation execution;
- service/network transport;
- persistence backend;
- actor behavior;
- MCI/game adapters;
- dialogue/voice;
- broad catalogs;
- scripting/plugin runtime;
- runtime LLM.

Naming where Phase-2 evaluator code will eventually live is not authorization to implement it.

## 12. Required validation

Before writer completion:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo metadata --format-version 1
```

Also run the stricter canonical lint policy required by Section 6.

Rerun already-installed Windows/Android target checks without operator prompting.

Do not claim Windows/Android execution.

## 13. Completion report

Create canonical:

```text
engineering/phase1/PHASE_1_REFOUNDATION_V2_IMPLEMENTATION_REPORT_2026-08-26.md
```

Copy identical file to:

```text
~/Downloads/PHASE_1_REFOUNDATION_V2_IMPLEMENTATION_REPORT_2026-08-26.md
```

Report:

- branch/worktree;
- exact commits;
- test-first baseline failures;
- crate reshape;
- every Fable architectural decision implemented;
- B-02/B-03/M-03 evidence;
- closed-item regressions;
- final test count;
- fmt/clippy/strict lint results;
- Windows/Android static checks;
- deviations/conflicts;
- `PHASE_2_AUTHORIZATION: NO`.
