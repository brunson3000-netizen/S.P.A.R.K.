# S.P.A.R.K. Phase 1 — Fable Architecture / Process Review

**Date:** 2026-08-26
**Role:** Independent architecture specialist (Fable, HIGH effort)
**Mode:** Architecture only. No Rust was written; no implementation file was changed.
**Repository:** `/home/chromikey/Projects/SPARK`, branch `phase1-refoundation`
**Basis of record:** Controlling blueprint v0.2; ADR-0001…ADR-0006; Phase-0 closure review (PASS); Phase-1 implementation brief; Codex independent review, correction re-review, and re-foundation independent review; the re-foundation implementation report; and the current source of `spark-core`, `spark-profile`, and `spark-testkit` at HEAD `3883409`.
**Precedence honored:** repository behavior and independent falsification evidence outrank writer self-assessments.

---

### 1. VERDICT

`READY_FOR_IMPLEMENTATION_REFOUNDATION_V2`

The three surviving defect classes (B-02 authority provenance, B-03 scheduler conflict, M-03 panic paths) are all closable with ordinary Rust structure inside the frozen Phase-0 contracts. No frozen causal-grammar element, no Phase-0 product architecture, and no causal runtime primitive needs to change. No decision below alters product meaning or authority ownership, so no operator product decision is required (Section 14). What was missing was not a new invariant but a correct placement of the existing invariants relative to the one boundary Rust actually enforces — the crate boundary — plus one deterministic conflict rule and one mechanical panic policy. This document specifies exactly that.

---

### 2. ROOT-CAUSE ANALYSIS

Phase 1 did not fail three times for three unrelated reasons. It failed three times for one reason expressed in three places:

**Trust was repeatedly expressed as a convention layered on a public surface, instead of being made coextensive with a crate boundary.**

1. **B-02 across all three passes is the same defect walking upstream.** Pass 1: schema fields were public, so authority was forgeable at the schema. Pass 2: write methods were hidden, but `StateStore::new` accepted raw schemas, so authority was forgeable at store construction. Pass 3: `ActivatedSchema` became unforgeable as a *literal*, but the mint (`DefinitionIdentityRegistry::activate`) is public and consumes raw public `DefinitionDeclaration`s, so authority is forgeable at the mint. Each fix privatized the artifact one step downstream of a public door, and the forgery simply moved to the door. The underlying design conflict was never resolved: the project wanted (a) Core to own the fingerprint/schema invariants, (b) profile validation to sit above Core, and (c) the mint to be non-bypassable. In Rust, `pub` is `pub` for everyone — Core cannot distinguish `spark-profile` from an adversarial adapter crate. Trying to satisfy (a) and (b) with a cross-crate call therefore *forced* the mint to be public, which forfeits (c). The only structural resolution is to put the **entire ceremony behind one public door in one crate**, so that there is no partial-trust intermediate step left to call. The re-foundation writer saw the problem ("Making the write facades crate-internal did not help, because the schema those facades check against was itself caller-supplied") but repeated the same shape one level up.

2. **B-03 is order-independence applied to identity but not to resolution.** The `WorkKey` re-foundation was correct: independent producers, scopes, kinds, and profiles no longer collide. But the conflict rule "keep the installed payload, reject the newcomer" *feels* order-independent because both orders report a conflict — while the retained canonical state is precisely the arrival-order artifact the determinism constitution forbids. The repository's own test (`payload_conflict_rejects_in_either_order`) checked error/length and never compared state digests or drained work, so the gap was invisible to the corpus. Notably, ADR-0003 already contains the correct rule for exactly this situation — a conflicting payload for an ordinal slot **poisons** the slot; no arrival picks a winner — and the scheduler simply never adopted its own project's precedent.

3. **M-03's residue is a policy audited by grep instead of by mechanism.** The writer removed every literal `unwrap`/`expect`/`panic!` and concluded "no panic path remains," missing (a) a panic reachable through the standard library (`i64::clamp` with reversed bounds) and (b) the const-context asymmetry of `const fn from_static` (compile error in `const` position, runtime panic otherwise). A panic-freedom policy that depends on a human enumerating panic sources will keep leaking; it must be enforced by lints, by types (validated-range parameters that make operations total), and by macros that force const evaluation.

4. **Process observation.** The failure corpus was excellent, but two of the twenty-seven re-foundation tests were structured so they could not catch the defect they named (`definition_spec_cannot_self_activate` used an invalid spec; the scheduler conflict test omitted the state comparison). The v2 corpus below states the *assertion*, not just the scenario, for every blocker-class test, and every "X is impossible" claim must be either a `compile_fail` test or a state-digest equality — never an error-was-returned check alone.

---

### 3. TRUSTED ACTIVATION ARCHITECTURE

#### 3.1 The governing principle

**A trust ceremony is non-bypassable only if the ceremony and every intermediate mint live in one crate, with exactly one public door that performs the complete ceremony.** The Codex re-foundation review states the acceptance condition precisely: a public activation surface is safe "if the method itself performed the complete authorized activation ceremony." That is the design adopted here. Anything less re-creates B-02.

Consequences that fall out immediately:

- The registry mint, the `ActivatedSchema`-equivalent artifact, the `StateStore` constructor, and (in Phase 2) the evaluator's write access must all be `pub(crate)` in the **same crate**, because a `pub(crate)` item is callable only from inside its crate and the evaluator/facades must reach the write paths.
- Manifest-level validation must be in that same crate, because the door must run it and the door cannot call upward.
- Profile **parsing** (the provisional on-disk format, which Phase 1 does not implement at all) remains a separate concern above this crate, so ADR-0001/ADR-0004 layering is preserved: parsing sits above validation, validation sits above the deterministic kernel, and the kernel (`spark-core`) contains no profile or trust machinery whatsoever.

#### 3.2 Decision 1 — which crate/module owns activation

**A new crate, `spark-engine`, owns activation, and it is the Phase-1 trust boundary.** It absorbs the current `spark-profile` contents (definition, manifest, config, text, validate) plus the activation registry and the `StateStore`, which move up out of `spark-core`. In Phase 2 it also receives the rule/effect evaluator — which is exactly why the store's `pub(crate)` write paths belong there.

```text
spark-core     deterministic kernel vocabulary and machines, zero trust ceremony:
               id, hash, value, scope, authority (vocabulary), clock, random,
               scheduler, timeline
spark-engine   the trust boundary (one crate = one ceremony):
               profile::{definition, manifest, config, text}
               activation::{registry, activated artifacts, fingerprint}
               state::{StateCell store}          [Phase 2 adds: rules, effects, evaluator]
spark-testkit  fixtures, scenario harness, adversarial corpus, dependency-policy tests
```

Dependency direction: `spark-engine -> spark-core`; `spark-testkit -> spark-core + spark-engine`. This is consistent with ADR-0001's authoritative rules: `spark-core` still depends on nothing but BLAKE3; validation is still separated from host adapters; canonical causal evaluation will exist in exactly one engine implementation (`spark-engine`); future `spark-persistence`/`spark-service`/adapters depend downward on engine and core. ADR-0001 explicitly leaves the exact crate graph to the engineer; only the direction rules are frozen, and none is violated. (Renaming rather than reusing the name `spark-profile` is deliberate: a crate named "profile" that contains the canonical state runtime and evaluator would mislabel the trust boundary. A future `spark-profile-io` parsing/authoring crate sits above `spark-engine` in Phase 3.)

Why not the two rejected placements:

- *Ceremony in `spark-core`* would drag logical-manifest validation into the kernel, violating the required "profile parsing/validation remains above Core" and contradicting the Codex clarification that manifest validation is a profile/activation-layer responsibility.
- *Ceremony split across `spark-profile` (validate) and `spark-core` (mint)* is the current, twice-failed design: the cross-crate call forces the mint public.

#### 3.3 Decision 2/3/4 — public types, constructor visibility, and the opaque artifact

The single door and the artifacts around it:

```rust
// ─── spark-engine::activation ── all mints pub(crate); one public door ───

/// Owns every profile's canonical identity lineage within this runtime.
/// The runtime composition root holds exactly one of these per deployment;
/// content addressing (below) makes any rogue second instance either
/// behaviorally identical or visibly divergent.
pub struct ActivationRegistry {
    identities: BTreeMap<(ProfileId, DefinitionId), Digest>, // fingerprints
    lineage:    Vec<ActivationRecord>,                       // append-only
}

impl ActivationRegistry {
    pub fn new() -> Self;

    /// THE door. Performs the complete ceremony, atomically:
    ///   manifest checks (duplicate IDs, profile qualification)
    ///   -> per-definition checks (non-empty scopes, bounded/typed fields —
    ///      largely already guaranteed by the validated field types)
    ///   -> fingerprints COMPUTED here from validated fields
    ///   -> immutable-identity comparison against this registry's lineage
    ///   -> atomic commit (failure leaves the registry unchanged; all
    ///      errors reported, not just the first)
    ///   -> mint of the unforgeable artifact
    pub fn activate(&mut self, manifest: &ProfileManifest)
        -> Result<ActivatedProfile, Vec<ValidationError>>;

    pub fn fingerprint_of(&self, p: &ProfileId, d: &DefinitionId) -> Option<&Digest>;

    /// Deterministic digest of the full activation lineage, for Phase-3
    /// persistence to bind a save to the exact registry history.
    pub fn lineage_digest(&self) -> Digest;
}

/// The unforgeable activation artifact: private fields, no public
/// constructor, minted only inside `activate`. This is the only value
/// from which a StateStore can exist, and the value future host/evaluator
/// write facades are constructed from.
pub struct ActivatedProfile {
    profile_id:            ProfileId,
    manifest_content_hash: Digest,     // ADR-0004/0006 exact-artifact binding
    activation_hash:       Digest,     // profile + every computed fingerprint
    schema:                ActivatedSchema,   // crate-internal representation
}

impl ActivatedProfile {
    pub fn profile_id(&self) -> &ProfileId;
    pub fn manifest_content_hash(&self) -> &Digest;
    pub fn activation_hash(&self) -> &Digest;
    pub fn definition(&self, id: &DefinitionId) -> Option<&ActivatedDefinition>; // read-only
    pub fn definitions(&self) -> impl Iterator<Item = &ActivatedDefinition>;

    /// The ONLY path to a state store.
    pub fn into_state_store(self) -> StateStore;
}

// ─── spark-engine::state ───

pub struct StateStore { /* schema + cells, single-profile */ }

impl StateStore {
    pub(crate) fn from_activation(/* called only by ActivatedProfile */) -> Self;

    // Public read-only surface (unchanged semantics from the re-foundation):
    pub fn profile_id(&self) -> &ProfileId;
    pub fn activation_hash(&self) -> &Digest;
    pub fn manifest_content_hash(&self) -> &Digest;   // NEW: full artifact binding
    pub fn schema_of(&self, p: &ProfileId, d: &DefinitionId) -> Option<&ActivatedDefinition>;
    pub fn get(&self, p: &ProfileId, d: &DefinitionId, s: &ScopeId) -> Option<&StateCell>;
    pub fn canonical_state_digest(&self) -> Digest;

    // Write paths stay pub(crate); Phase-2 evaluator and Phase-3 host
    // facades live in this crate and mediate them:
    pub(crate) fn observe_host_owned(...) -> Result<(), StateWriteError>;
    pub(crate) fn apply_spark_effect(...) -> Result<(), StateWriteError>;
    pub(crate) fn commit_derived(...)    -> Result<(), StateWriteError>;
}
```

Visibility rulings, item by item:

| Item | Ruling | Reason |
|---|---|---|
| `ActivationRegistry::new` / `activate` | **public** | It is the complete ceremony; there is nothing to bypass. Divergence between two registry instances is handled by content addressing (§3.5), and "who may run activation" as a *permission* is a Phase-3 capability-scope concern, correctly outside Phase 1. |
| `DefinitionDeclaration` | **deleted from the public surface** (crate-private conversion detail, or removed entirely) | It was the raw-declaration mint input Codex executed against. `DefinitionSpec` → internal declaration happens inside the door. |
| `DefinitionSpec` (all fields public) | **public, untrusted authoring input** | Authority facts *originate* as operator-authored profile data; carrying them in an untrusted input struct is correct. What was wrong was accepting them at a trusted mint without the full ceremony. |
| `DefinitionSpec::to_declaration` | **removed** | The self-activation stepping stone. |
| `DefinitionSpec::definition_fingerprint` | **keep public** | Pure function; computing a fingerprint grants nothing, and reviewers need it. |
| `DefinitionKindTag::builtin` | **crate-private** | Codex counterexample #4: external callers must not assert the built-in/custom identity bit. The public vocabulary is the `DefinitionKind` enum plus `DefinitionKind::custom(tag)`. |
| `ActivatedProfile`, `ActivatedDefinition` | public types, **private fields, no public constructor** | Possession is proof of ceremony — now actually true, because the mint is `pub(crate)`. |
| `StateStore::from_activation` | **`pub(crate)`** | Reachable only via `ActivatedProfile::into_state_store`. The old `from_activated_schema` public constructor is gone with the crate move. |
| `spark-core::activation` module and `spark-core::state` module | **removed from core** | Core retains no trust machinery: `Authority`/`WriteClass` vocabulary stays; `StateCell` may stay in core as pure data vocabulary or move with the store (writer's choice; recommend moving it to keep core state-free). |

#### 3.4 Decision 5 — registry lifecycle and profile identity anchoring

The re-foundation's B-04 semantics (full-fingerprint immutable identity, profile-qualified keys, atomic batch commit, all-errors reporting) are preserved verbatim inside the door — they were independently closed twice and must be ported, not rewritten.

The new anchoring, addressing "duplicate/divergent registry histories cannot silently exist":

1. **Within one registry:** already impossible (B-04): a `(profile, definition)` re-activation with a different fingerprint rejects; identical re-activation is idempotent and produces an identical `activation_hash`.
2. **Across registry instances:** Rust cannot prevent a second in-process `ActivationRegistry::new()`, and pretending otherwise is what kept B-02 alive. The invariant is made real by **content addressing plus binding**: every `ActivatedProfile` carries `manifest_content_hash` and `activation_hash`; every `StateStore` carries both; the registry exposes `lineage_digest()`. Two independent registries fed identical manifests produce byte-identical hashes — interchangeable *because* deterministically identical, hence harmless. Two divergent registries produce different activation hashes on every downstream artifact, and Phase-3 persistence (ADR-0006 already mandates this) refuses continuation unless the exact hashes match. Divergence therefore cannot be *silent*; it is visible on every artifact it touches.
3. **Composition rule (documented + tested, not merely hoped):** the production composition root (embedded runtime / service, Phase 3) owns exactly one `ActivationRegistry`. Phase 1 encodes the rule now as a testkit convention plus a doc-level contract on `ActivationRegistry`, and Phase 3 makes it structural in the service/runtime wrapper.
4. **Behavior-artifact context:** `ActivatedProfile::activation_hash` (optionally combined with a `ConfigRevision` hash by the caller, exactly as the testkit does today) is the behavior-artifact digest for `RandomAddress`. Binding `ConfigRevision` *into* activation and pairing both into a behavior epoch at a canonical barrier is deliberately deferred to Phase 3 (ADR-0004 behavior-epoch machinery), and the seam — both hashes visible on the artifact — is already in place.

#### 3.5 Decision 6 — is a dedicated activation crate justified?

A dedicated activation-only crate is **not** justified, and would re-create the defect: an activation crate separate from the store/evaluator crate would need public store-construction and write seams in the crate below it, which is B-02 again. The justified unit is the **engine** crate: activation + activated artifacts + state runtime (+ Phase-2 evaluation) as one trust boundary, with `activation` as a module inside it. This is the smallest structure in which every trusted mint can be `pub(crate)`.

#### 3.6 Decision 7 — test access without opening production APIs

Three sanctioned tiers, and nothing else:

1. **In-crate unit tests** use `pub(crate)` paths directly (as today).
2. **Cross-crate tests go through the production door wherever possible** — building manifests from `DefinitionSpec`s and calling `activate` *is* the production path and is the preferred fixture style.
3. **A `test-support` cargo feature on `spark-engine` and `spark-core`** exposes a `fixture` module wrapping the `pub(crate)` write paths (and the restricted timeline constructor, §6). It is off by default, enabled only by `spark-testkit` as a dev-dependency and by `#[cfg(test)]` builds. The dependency-policy test is extended to assert that **no production dependency edge enables `test-support`** (it already parses `cargo metadata`; features are visible there). A feature-gated API is not part of the production surface, and the gate is mechanically checked — this replaces both "make it public for tests" and "leave it untestable," the two failure modes the record shows.

---

### 4. SCHEDULER CONFLICT ARCHITECTURE

#### 4.1 The chosen model: order-independent conflict state (key poisoning with evidence)

**Adopt ADR-0003's own rule for the scheduler: a second distinct payload under the same complete semantic `WorkKey` poisons the key.** Neither payload wins; the key transitions to a `Conflicted` state that retains an **order-independent sorted evidence set** of every distinct payload hash that ever claimed the key. First-installed-wins is abolished.

Rejected alternatives, for the record: *deterministic winner by smallest payload hash* is order-independent but silently discards a genuine ambiguity, violating "conflict is visible/explainable"; *retroactively unscheduling both* lets a late conflicting arrival cancel legitimately scheduled work and interacts non-deterministically with drains that already happened. Poisoning is deterministic, visible, consistent with the timeline's frozen semantics, and gives Phase 2 a stable base.

#### 4.2 API shape

```rust
// ─── spark-core::scheduler ───

pub enum ScheduleDisposition {
    Scheduled,
    AlreadyScheduledIdempotent,
    /// The key is now (or already was) conflicted. Carries the current
    /// order-independent evidence. Not an Err: a conflict is a legitimate,
    /// deterministic logical outcome, exactly like a poisoned timeline slot.
    Conflicted(WorkKeyConflict),
}

/// Order-independent conflict evidence: the key plus the sorted set of
/// every distinct payload hash that claimed it. Bounded by
/// MAX_CONFLICT_EVIDENCE (recommend 16) with a retain-smallest-hashes
/// eviction rule and an omitted-distinct count, so the retained set and
/// count are identical for every arrival order of the same claim set.
pub struct WorkKeyConflict {          // private fields + accessors
    key: WorkKey,
    competing_payload_hashes: BTreeSet<Digest>,
    omitted_distinct: u64,
}

enum SlotState {                      // internal
    Scheduled(WorkPayload),
    Conflicted { competing: BTreeSet<Digest>, omitted_distinct: u64 },
}

pub struct DrainOutcome {
    /// Executable due work, stable ascending WorkKey order.
    pub due: Vec<DueWorkItem>,
    /// Due-but-conflicted keys: removed from the queue and reported, never
    /// executed. Deterministic order (ascending WorkKey).
    pub conflicted: Vec<WorkKeyConflict>,
}

impl Scheduler {
    /// Total; no Err variant is needed in Phase 1 (occurrence overflow is
    /// rejected at key construction via OccurrenceIndex::checked_next).
    pub fn schedule(&mut self, item: DueWorkItem) -> ScheduleDisposition;
    pub fn drain_due(&mut self, now: LogicalTime) -> DrainOutcome;
    pub fn contains(&self, key: &WorkKey) -> bool;
    pub fn slot_status(&self, key: &WorkKey) -> WorkSlotStatus; // Empty|Scheduled|Conflicted
    pub fn canonical_state_digest(&self) -> Digest;             // encodes Conflicted slots
}
```

Semantics:

- **Empty key + payload P** → `Scheduled`, state `Scheduled(P)`.
- **`Scheduled(P)` + identical P** → `AlreadyScheduledIdempotent`, state unchanged.
- **`Scheduled(P)` + distinct Q** → state becomes `Conflicted{{h(P), h(Q)}}` (P is *not* retained as work — retaining it is exactly first-arrival privilege), disposition `Conflicted`.
- **`Conflicted{S}` + payload R** → `h(R)` set-inserted (idempotent if already present), disposition `Conflicted` with the updated evidence. No arrival order can matter because a sorted set of the same members is one value.
- **Drain:** conflicted keys with `due_time <= now` are removed and reported in `DrainOutcome::conflicted`, never in `due`. After removal the key is free; a producer that has resolved the ambiguity may reschedule (typically under the next occurrence index). This is the deterministic Phase-1 recovery; in Phase 2/3 any richer recovery flows through canonical timeline commands like every other mutation, so no separate cancellation API (which could race submission and "choose history", the exact thing ADR-0003 §11 forbids) is introduced.
- **Persistence/replay:** `SlotState` is fully canonical: the state digest encodes `Scheduled` slots as key+payload and `Conflicted` slots as key+sorted evidence+omitted count, with distinct domain tags. Two schedulers reaching the same logical claim set by any call order are digest-identical. `WorkKey::identity_digest()` is unchanged; ADR-0006 delayed-obligation records (Phase 3) wrap it together with creator fingerprint/epoch/artifact hashes — the artifact-binding fields deliberately do **not** enter `WorkKey` itself (Codex item #18): occurrence identity is producer/scope-local; artifact identity belongs to the obligation record that persists it.

Phase-2 safety: delayed rules schedule through the same API; a conflicted key deterministically blocks exactly that occurrence, is visible to inspection (blueprint §19.5 "visible truncation/defer warnings" family), and cannot corrupt neighboring occurrences because the key is the full semantic identity.

**Hidden-coupling note (timeline symmetry):** adopt the same evidence representation for timeline slot poisoning — `SlotState::Poisoned { competing: BTreeSet<Digest>, omitted_distinct: u64 }` — replacing the evidence-free unit variant. This answers Codex counterexample #12 (poisoned slots caused by different competing pairs remain distinguishable in `canonical_state_digest`) while staying arrival-order-independent, and it changes only the *state* representation; every closed B-01 hash/ack/fence property is preserved (the per-call `SlotPoisonRecord` may keep its per-call fields — call results may differ per call; canonical state may not).

#### 4.3 Acceptance tests

See AT-B in Section 10 for the exact corpus; the defining assertions are **state-digest equality and drained-work equality across all arrival permutations**, not error-shape checks.

---

### 5. PANIC-FREE CANONICAL API POLICY

One uniform project rule, mechanically enforced:

> **No public function in a canonical crate may panic, wrap, or saturate on any input. Fallibility is expressed as `Result` with a typed error; infallibility is earned by validated-domain parameter types; compile-time literals go through macros that force const evaluation.**

Concretely:

1. **`CanonicalTag::from_static` is replaced.** The runtime-reachable panic (Codex-executed) is eliminated by splitting the two roles it conflated:

   ```rust
   impl CanonicalTag {
       /// Total at runtime AND usable in const contexts.
       pub const fn try_from_static(s: &'static str) -> Result<Self, StaticTagError>;
   }

   /// Literal construction. The inline-const block forces compile-time
   /// evaluation, so an invalid literal is ALWAYS a compile error and the
   /// panic inside the const block is unreachable at runtime by construction.
   #[macro_export]
   macro_rules! canonical_tag {
       ($lit:literal) => {
           const {
               match $crate::id::CanonicalTag::try_from_static($lit) {
                   Ok(t) => t,
                   Err(_) => panic!("invalid canonical tag literal"),
               }
           }
       };
   }
   ```

   Baseline vocabulary (`DefinitionKind::kind_tag`, command/work kinds in fixtures) uses `canonical_tag!("trigger")`. `CanonicalTag::new` remains the runtime path. The existing const/runtime language-agreement test is retained against `try_from_static`. (Toolchain 1.98 supports inline const; no nightly feature involved.)

2. **`FixedPoint::clamp` is replaced by a total operation over a validated domain plus a checked ad-hoc form:**

   ```rust
   /// Coherent by construction (min <= max), like ValueConstraint's ranges.
   pub struct FixedRange { min: FixedPoint, max: FixedPoint }   // private fields
   impl FixedRange { pub fn new(min, max) -> Result<Self, ValueConstraintError>; }

   impl FixedPoint {
       pub fn clamp_to(self, range: &FixedRange) -> FixedPoint;              // total
       pub fn checked_clamp(self, min: FixedPoint, max: FixedPoint)
           -> Result<FixedPoint, IncoherentClampBounds>;                     // fallible
       pub fn checked_add(self, o: FixedPoint) -> Result<FixedPoint, FixedPointOverflow>;
       pub fn checked_sub(self, o: FixedPoint) -> Result<FixedPoint, FixedPointOverflow>;
       // Phase-2 scoring arithmetic follows the same pattern (checked_mul_int, ...).
   }
   ```

   `ValueConstraint::fixed` internally reuses `FixedRange` so there is one coherence definition.

3. **Incoherent bounds stay inexpressible** (`ValueConstraint` opaque constructors — closed, preserved), and the same "validated domain ⇒ total operation" pattern is the stated template for **all future canonical constructors**: a new canonical type gets (a) a fallible validating runtime constructor, (b) a literal macro only if literals are genuinely needed, (c) operations that are total over validated inputs, and (d) no public field or `From<String>`/`From<i64>` bypass.

4. **Timeline/scheduler/occurrence arithmetic:** already checked after the re-foundation (window end, fence frontier pre-check, `checked_next`) — preserved, with the frontier-at-`u64::MAX` idempotent-failure test (Codex #17) added.

5. **Mechanical enforcement, so this never regresses to a grep audit:** the canonical crates (`spark-core`, `spark-engine`) add, in addition to the existing `#![forbid(unsafe_code)]` and `-D warnings`:

   ```text
   -D clippy::unwrap_used  -D clippy::expect_used  -D clippy::panic
   -D clippy::indexing_slicing  -D clippy::arithmetic_side_effects
   ```

   (`arithmetic_side_effects` makes every unchecked `+`/`-`/`*` a CI failure in canonical code — this single lint would have caught both historical M-03 arithmetic findings.) Test modules may `allow` these locally. The `const {}` macro panic is compile-time-only and does not trip `clippy::panic` at runtime call sites; if the lint flags the macro body, a scoped `allow` with a comment stating the const-evaluation guarantee is the one sanctioned exception.

---

### 6. RECONSTRUCTION / RESUME POLICY

Codex ruling `RESUME_AT_FRONTIER: RESTRICT` is adopted and made structural:

1. **Nonzero-frontier construction does not exist in the Phase-1 production surface.** `TimelineIngress::resume_at_frontier` moves behind the `test-support` cargo feature (`#[cfg(any(test, feature = "test-support"))]`). Production builds cannot name it; the compile-fail/doc surface documents that. `TimelineIngress::new` (frontier 0, genesis hash) remains the only production constructor.
2. **Test support remains non-production mechanically:** the same feature gate and the same dependency-policy assertion as §3.6 — no production edge enables `test-support`. The ordinal-space-exhaustion properties stay falsifiable from `spark-testkit` (which dev-enables the feature) and from in-crate unit tests.
3. **The future persistence constructor (Phase 3, `spark-persistence`) is specified now** so the seam is unambiguous. It consumes evidence, not parameters:

   ```rust
   pub struct TimelineResumeEvidence {   // assembled and verified by spark-persistence
       profile_id: ProfileId,
       timeline_epoch: TimelineEpoch,
       finalized_frontier: Ordinal,
       last_finalized_fence_hash: Digest,     // real chain anchor, never synthesized
       canonical_history_digest: Digest,      // must re-verify against restored history
       manifest_content_hash: Digest,         // ADR-0006 exact-artifact binding
       config_revision_hash: Digest,
       behavior_epoch: u64,
       sequencer_grant: SourceId,
       epoch_reset_chain: Vec<EpochResetRecord>,
   }
   ```

   The constructor validates internal consistency (fence hash matches history digest; artifact hashes resolve per ADR-0006's compatibility envelope) and fails explicitly otherwise. Crucially it never *invents* a genesis hash for a nonzero frontier — the current `resume_at_frontier` synthesizing a fresh "genesis" anchor at an arbitrary ordinal is precisely what made it an authority surface, and the restriction plus this evidence contract prevents arbitrary canonical-timeline injection permanently (Codex #9/#10).
4. **No Phase-1 implementation of the evidence constructor** — defining it now is a contract, not scope creep; implementing it is Phase 3.

---

### 7. CLOSED-ITEM PRESERVATION

| Closed item | Ruling | Interaction with this design |
|---|---|---|
| Semantic/admission separation (B-01) | **Preserve unchanged.** | Untouched by the crate reshape (timeline stays in `spark-core`). The type boundary (`SemanticCommandEnvelope` / `AdmissionTicket` with no canonical encoding / transient `SubmittedCommand`) is the pattern §3 generalizes. |
| Structured `StageAcknowledgement` / `FinalizationResult` (B-01) | **Preserve unchanged**, including private fields, `covers`, and the note that the acknowledgement hash is a binding, not a wire signature (Phase-3 concern). | The scheduler's new `WorkKeyConflict`/`DrainOutcome` follow the same structured-evidence idiom. |
| History/state/transcript digest model (M-02) | **Preserve.** Two small hardenings: rename the testkit field to `outcome_transcript_digest` (Codex #13's documentation ruling), and extend the state digests to the new conflict-evidence slot representations with fresh domain-separation tags. | The scheduler gains `canonical_state_digest` coverage of `Conflicted` slots; the timeline `Poisoned` variant gains the evidence set (§4.2). Neither touches `canonical_history_digest` — finalized history semantics are unchanged. |
| Immutable full definition fingerprint + atomic validation (B-04) | **Preserve verbatim; relocate with the ceremony into `spark-engine`.** Port the complete test suite (identity-field sweep, atomic failed batch, builtin-vs-custom domain separation, profile independence, corrected retry). | The fingerprint remains computed by the ceremony, never accepted; relocation is a move, not a rewrite. |
| `ConfigRevision` model (M-01) | **Preserve unchanged**; moves into `spark-engine::profile`. | Phase-3 seam: activation may later bind a config hash into the behavior epoch; both hashes are already exposed. Duplicate-diagnostic determinism test extended per Codex #15 (three-plus duplicates, reordered, identical sorted diagnostics, no partial revision). |
| Profile/artifact-qualified random addresses (M-04) | **Preserve unchanged** in `spark-core`. | The behavior-artifact digest input is now naturally `ActivatedProfile::activation_hash` (+ config hash), which strengthens, not changes, the contract. |
| Dependency direction (m-01) | **Preserve the mechanism; update the expected graph** to `spark-engine -> spark-core`, `spark-testkit -> both`, and extend the metadata check to assert `test-support` is absent from production resolution. | — |
| Unicode contract (Codex #14) | **Decide now: deliberate byte distinction, no normalization.** `CategoricalValue`/`BoundedText` compare and hash exact scalar sequences; composed vs decomposed forms are distinct values. Rationale: normalization drags Unicode tables into canonical hashing and creates version-skew risk across the declared support envelope. Documented on both types; authoring-layer normalization warnings are a Phase-3+ tooling concern. | — |

No other hidden couplings were found: clock, hash, random, scope, id, value (minus §5 changes) are untouched by the B-02/B-03 designs.

---

### 8. MODULE / CRATE MAP

The smallest clean Phase-1 map (Phase 2 is deliberately not designed here; only its landing zones are named):

```text
spark/
  crates/
    spark-core/            # deterministic kernel — no trust ceremony, no profile types
      src/
        id.rs              # StableId, CanonicalTag (+ canonical_tag! macro), typed ID newtypes
        hash.rs            # CanonicalEncoder, Digest (BLAKE3)
        value.rs           # FixedPoint (+ FixedRange, checked ops), CanonicalValue,
                           #   CategoricalValue, ValueConstraint
        scope.rs           # ScopeKind, ScopeId
        authority.rs       # Authority, WriteClass vocabulary
        clock.rs           # LogicalTime, monotonic clock
        random.rs          # RandomAddress (profile/artifact-qualified)
        scheduler.rs       # WorkKey, WorkKind, ScheduleDisposition, WorkKeyConflict,
                           #   DrainOutcome, Scheduler (conflict-poisoning model)
        timeline.rs        # SemanticCommandEnvelope, AdmissionTicket, StageAcknowledgement,
                           #   FinalizationResult, TimelineIngress
                           #   [resume_at_frontier: cfg(test)/feature = "test-support" only]
      Cargo.toml           # features = { test-support = [] }

    spark-engine/          # THE trust boundary (formerly spark-profile, absorbing state)
      src/
        profile/
          definition.rs    # DefinitionSpec (untrusted authoring input), DefinitionKind
          manifest.rs      # ProfileManifest + manifest_content_hash
          config.rs        # ConfigRevision (validated, content-addressed)
          text.rs          # BoundedText
        activation.rs      # ActivationRegistry (the single door), ActivatedProfile,
                           #   ActivatedDefinition, fingerprint fn, ValidationError
        state.rs           # StateCell, StateStore (pub(crate) mint + write paths)
        fixture.rs         # [feature = "test-support" only] sanctioned test write seams
        # Phase 2 lands here: rules.rs, effects.rs, evaluator.rs (uses pub(crate) writes)
      Cargo.toml           # deps: spark-core; features = { test-support = ["spark-core/test-support"] }

    spark-testkit/         # fixtures + scenario harness + adversarial corpus
      src/scenario.rs      # three digests (history / ingress state / outcome transcript)
      tests/               # refoundation + v2 corpus, dependency & feature policy tests
      Cargo.toml           # deps: spark-core, spark-engine (dev: with test-support)
```

Future crates (named for direction only): `spark-profile-io` (parsing/authoring, above engine), `spark-persistence`, `spark-protocol`, `spark-service`, adapters — all depend downward, exactly per ADR-0001.

---

### 9. PUBLIC API CONTRACT

**Important public surface after Phase 1** (types/functions a consumer legitimately uses):

- `spark-core::id` — `StableId`, `CanonicalTag::{new, try_from_static}`, `canonical_tag!`, `DefinitionId`, `ProfileId`, `SourceId`, `CommandId`, `FenceId`, `StableIdError`.
- `spark-core::hash` — `CanonicalEncoder`, `Digest`.
- `spark-core::value` — `FixedPoint` (`from_raw`, `raw`, `from_integer`, `checked_add/sub`, `checked_clamp`, `clamp_to`), `FixedRange`, `CanonicalValue`, `CategoricalValue`, `ValueType`, `ValueConstraint` (opaque, validating constructors), error types.
- `spark-core::scope` — `ScopeKind` (incl. `Custom(StableId)`), `ScopeId`.
- `spark-core::authority` — `Authority`, `WriteClass`, `implied_write_class`.
- `spark-core::clock` — `LogicalTime`, monotonic clock with backward-advance rejection.
- `spark-core::random` — `RandomAddress` derivation (root seed, profile, behavior epoch, behavior-artifact digest, producer, scope, occurrence).
- `spark-core::scheduler` — `WorkKey`, `WorkKind`, `OccurrenceIndex::checked_next`, `WorkPayload`, `DueWorkItem`, `ScheduleDisposition`, `WorkKeyConflict`, `DrainOutcome`, `Scheduler` (`schedule`, `drain_due`, `contains`, `slot_status`, `canonical_state_digest`), `WorkKey::identity_digest`.
- `spark-core::timeline` — `Ordinal`, `TimelineEpoch`, `CommandKind`, `SemanticCommandEnvelope` (`semantic_hash`, `submit_with`), `AdmissionWindow`/`AdmissionTicket`, `SubmittedCommand`, `StageDisposition`, `StageAcknowledgement` (read-only + `covers`), `SlotPoisonRecord`, `AdmissionRetryAdvice`, `StageError`, `TimelineFence`, `FenceError`, `FinalizationResult` (read-only), `EpochResetRecord`/`EpochResetError`, `SlotStatus`, `TimelineIngress` (`new`, `current_admission_window`, `stage`, `submit_fence`, `reset_epoch`, `slot_status`, `is_positively_staged`, digests, read accessors), `compute_ordered_stream_digest`.
- `spark-engine::profile` — `DefinitionSpec` (public fields; untrusted), `DefinitionKind` (+ `custom`), `BehavioralLeverage`, `ProfileManifest` (+ `manifest_content_hash`), `ConfigRevision::build` (+ `config_revision_hash`), `ConfigEntry`, `BoundedText`, error types.
- `spark-engine::activation` — `ActivationRegistry` (`new`, `activate`, `fingerprint_of`, `lineage_digest`), `ActivatedProfile` (read accessors + `into_state_store`), `ActivatedDefinition` (read accessors), `ValidationError`, `definition_fingerprint` (pure).
- `spark-engine::state` — `StateCell` (read), `StateStore` (read-only surface + digests), `StateWriteError`, `SourceRefs`/`MAX_SOURCE_REFS`.
- `spark-testkit` — scenario harness, three digests, corpus.

**Dangerous items that must NOT be public** (each was, or would become, a falsified surface):

| Must not be public | Why |
|---|---|
| Any `StateStore` constructor from raw schema data, and the three write paths | B-02 passes 1–3. Mint: `pub(crate)` via `ActivatedProfile`; writes: `pub(crate)` for Phase-2 evaluator/facades. |
| `DefinitionDeclaration` (or any raw-declaration mint input) and `DefinitionIdentityRegistry::{new, activate}` as a freestanding core API | The executed B-02 bypass. Replaced by the engine door. |
| `DefinitionSpec::to_declaration`, `DefinitionKindTag::builtin` | Self-activation stepping stone; caller-asserted built-in identity bit (Codex #4). |
| Constructors of `ActivatedProfile`, `ActivatedDefinition`, `StageAcknowledgement`, `FinalizationResult`, `WorkKeyConflict`, `ValidatedManifest`-equivalents | Possession must remain proof of ceremony/evidence. |
| `AdmissionTicket::canonicalize` or any admission-data canonical encoding | Would reopen B-01/M-02. |
| `TimelineIngress::resume_at_frontier` in production builds | §6; feature-gated only. |
| Panic-capable canonical operations (`from_static` as-is, unchecked `clamp`, unchecked arithmetic) | §5; lint-enforced. |
| Unbounded `String`/`i64`-field bypasses into canonical types (`From` impls, public fields on `CanonicalTag`/`CategoricalValue`/`FixedPoint`/`ValueConstraint`) | M-03 class. |
| Any public epoch-reset/handoff without active-sequencer authentication and strict epoch increase | Closed in refoundation; must not regress. |

---

### 10. INVARIANT / TEST MATRIX

Trust-boundary diagrams first, then the invariant table, then the acceptance corpus.

**Activation / state:**

```text
profile data (DefinitionSpec / ProfileManifest / ConfigRevision — untrusted authoring input)
-> validation                    [inside ActivationRegistry::activate — the single door]
-> activation authority          [fingerprints computed, lineage-checked, atomically committed]
-> immutable activated schema    [ActivatedProfile: private fields, crate-only mint]
-> state runtime                 [StateStore via into_state_store; pub(crate) writes behind
                                  future host/evaluator facades; public read-only inspection]
```

**Scheduler:**

```text
producer semantic identity (WorkKey) + payload hash
-> schedule                      [Scheduled | Idempotent | Conflicted(evidence set)]
-> conflict state                [order-independent sorted evidence; no arrival winner]
-> drain_due                     [due work in stable key order; conflicted keys reported,
                                  removed, never executed]
```

**Timeline / finality (unchanged, for completeness):**

```text
SemanticCommandEnvelope (identity) + AdmissionTicket (ephemeral, no canonical encoding)
-> stage (sequencer-authenticated, ordinal-slot, token-checked-then-dropped)
-> StageAcknowledgement | Poisoned(evidence set) | NotInAdmissionWindow(retry)
-> fence (full positive acks, contiguous, digest- and chain-verified, atomic)
-> FinalizationResult -> canonical_history_digest
```

**Invariant table:**

| Invariant | Authoritative owner | Structural enforcement | Falsification/test |
|---|---|---|---|
| No activated authority/schema exists that did not pass the complete ceremony | `spark-engine::activation` | `ActivatedProfile`/`ActivatedDefinition` private fields; mint `pub(crate)`; `DefinitionDeclaration` not public; one public door runs the whole ceremony | AT-A1, AT-A2 (compile-fail), AT-A3 |
| `StateStore` construction cannot accept caller-authored authority/type/scope/fingerprint facts | `spark-engine::state` | Only constructor is `pub(crate)`, reachable solely via `ActivatedProfile::into_state_store` | AT-A2, AT-A4 |
| Fingerprints are computed, never accepted; full-fingerprint immutable identity; atomic batch | `spark-engine::activation` | No fingerprint field on any input; B-04 algorithm ported verbatim | AT-E1 (ported B-04 suite) |
| Divergent registry histories cannot silently coexist | `spark-engine::activation` + content addressing | `activation_hash`/`manifest_content_hash` on every artifact; `lineage_digest`; ADR-0006 continuation refuses mismatched hashes | AT-A5 |
| Host-owned/derived/spark-owned write classes are unbypassable from outside the engine crate | `spark-engine::state` | Write paths `pub(crate)`; no public write method exists | AT-A6 (compile-fail), ported state suite |
| Profile/trust-domain isolation of schema, state, registry keys | engine (`(ProfileId, DefinitionId)` keys, single-profile store) | Ported refoundation checks (`ForeignProfile`, profile-qualified registry) | ported suite + AT-A5 |
| Admission data can never reach canonical identity/replay | `spark-core::timeline` | `AdmissionTicket` has no canonical encoding and is never stored | ported B-01 suite (unchanged) |
| Positive staging acknowledgement binds its exact envelope; finalization returns structured evidence | `spark-core::timeline` | Private-field evidence types; `covers` | ported suite + AT-F3 (per-field mutation sweep, Codex #11) |
| Fence chain contiguity, digest verification, per-source monotone sequence, atomic rejection | `spark-core::timeline` | Pre-mutation validation order in `submit_fence` | ported suite + AT-C4 |
| Same-key scheduler conflict state is arrival-order-independent and visible | `spark-core::scheduler` | `Conflicted` sorted evidence set; no retained first payload | AT-B1…AT-B6 |
| Scheduler order and occurrence identity are semantic, persistence-friendly | `spark-core::scheduler` | Full `WorkKey`; `identity_digest` | ported refoundation scheduler suite |
| No public canonical operation can panic/wrap/saturate | all canonical crates | `try_from_static` + `canonical_tag!` const block; `FixedRange`/`checked_*`; clippy panic/arithmetic lints in CI | AT-C1…AT-C5 + lint gate |
| Nonzero-frontier timeline construction absent from production surface | `spark-core::timeline` | `test-support` feature gate + dependency/feature policy test | AT-D1 (compile-fail), AT-D2 |
| Dependency direction & feature hygiene | workspace | metadata-based policy test incl. feature assertion | AT-E3 |
| Canonical strings/IDs bounded and validated; Unicode byte-distinct | core/engine value & text types | closed M-03 constructors + documented normalization contract | ported suite + AT-C6 |
| No unsafe, no wall clock, no ambient RNG, no float, no unordered canonical iteration | all canonical crates | `#![forbid(unsafe_code)]`; audit + existing tests | ported audits |

---

### 10a. ACCEPTANCE-TEST CORPUS (deliverable 5)

Every test below is mandatory for the v2 writer. "compile_fail" means a doc-test or `trybuild`-style case compiled as an external consumer. All current 172 tests are retained (ported across the crate move without weakened assertions) except where a test below explicitly *replaces* one that Codex found insufficient.

**AT-A — trusted activation (B-02):**

- **AT-A1 `raw_declarations_cannot_reach_activation`** (compile_fail): external crate references `DefinitionDeclaration` / calls a registry mint with caller-built declarations — the type/path does not exist publicly.
- **AT-A2 `state_store_is_unconstructible_without_activation`** (compile_fail ×3): `StateStore { .. }` literal; any `StateStore::new/from_*` public constructor; `ActivatedProfile { .. }` literal.
- **AT-A3 `valid_spec_cannot_self_activate`** — replaces the insufficient `definition_spec_cannot_self_activate`: a **fully valid** `DefinitionSpec` (non-empty scopes, coherent constraint) has no public path to an `ActivatedProfile` except a `ProfileManifest` through `ActivationRegistry::activate`; expressed as compile-fail on the removed `to_declaration`/mint path plus a positive test that the door path succeeds and yields the same fingerprint `definition_fingerprint` predicts.
- **AT-A4 `store_carries_full_artifact_binding`**: a store built through the door exposes `activation_hash` and `manifest_content_hash` equal to the registry/manifest values; `Digest::ZERO` appears nowhere.
- **AT-A5 `divergent_activations_are_never_interchangeable`** (Codex #3): two fresh registries activate the same `(profile, definition)` with different authority; both succeed locally, but every downstream hash differs (`activation_hash`, store digests), and identical manifests in two registries produce byte-identical hashes; `lineage_digest` differs iff histories differ.
- **AT-A6 `no_public_write_path_exists`** (compile_fail): external call to `observe_host_owned` / `apply_spark_effect` / `commit_derived` does not compile; plus feature-gated fixture writes prove the schema validation (authority/type/bounds/scope/profile) still rejects each violation class (ported suite).
- **AT-A7 `builtin_kind_bit_is_not_caller_assertable`** (compile_fail, Codex #4): `DefinitionKindTag::builtin(...)` is not nameable externally; `DefinitionKind::custom("trigger")` still fingerprints distinctly from builtin `Trigger` (ported).
- **AT-A8 `registry_lineage_is_append_only_and_deterministic`** (Codex #16): repeated identical activations do not grow divergence; `lineage_digest` after (m1, m2) equals any replay of (m1, m2) and differs from (m2, m1) only if the activation contents differ.

**AT-B — scheduler conflict (B-03):**

- **AT-B1 `same_key_conflict_state_is_arrival_order_independent`** — replaces the insufficient `payload_conflict_rejects_in_either_order`: payloads A,B under one full `WorkKey`, both orders: assert `canonical_state_digest` equality, `slot_status == Conflicted`, equal evidence sets, and **equal `drain_due` results** (empty `due`, identical `conflicted` reports).
- **AT-B2 `three_way_conflict_converges_in_all_six_orders`**: A,B,C permutations all yield one digest and evidence {h(A),h(B),h(C)}.
- **AT-B3 `neither_payload_survives_a_conflict`**: after A-then-B, drained `due` contains neither A nor B; the conflict report carries both hashes.
- **AT-B4 `exact_duplicate_into_conflicted_slot_is_idempotent`**: re-submitting A into `Conflicted{A,B}` changes nothing (digest-equal), disposition `Conflicted`.
- **AT-B5 `conflict_report_and_requeue_are_deterministic`**: drain removes+reports the conflicted key; a corrected reschedule under the same key then succeeds; replaying the whole sequence is digest-identical.
- **AT-B6 `evidence_cap_is_order_independent`**: > MAX_CONFLICT_EVIDENCE distinct payloads in shuffled orders retain the identical smallest-hash set and identical omitted count.
- **AT-B7 (batch seam, Codex #6):** documented-only for Phase 1 (no batch API exists); the invariant "batch conflict handling must be atomic and order-independent" is recorded in the scheduler docs for the Phase-2 writer.

**AT-C — panic-free construction (M-03):**

- **AT-C1 `try_from_static_is_total_at_runtime`**: invalid static string returns `Err`, never panics (this replaces the Codex-executed panic).
- **AT-C2 `canonical_tag_macro_rejects_invalid_literals_at_compile_time`** (compile_fail): `canonical_tag!("Trigger")` fails to build; valid literal equals `CanonicalTag::new` value; const/runtime language-agreement test retained.
- **AT-C3 `clamp_is_total_or_fallible_never_panicking`**: `checked_clamp(min>max)` returns typed `Err`; `clamp_to(FixedRange)` is total; `FixedRange::new(min>max)` rejects; the Codex reversed-bounds panic case now returns `Err`.
- **AT-C4 `frontier_at_max_is_idempotently_reportable`** (Codex #17): ingress at `u64::MAX` frontier, width 1: repeated stage/fence/query in varied orders return the exhaustion/rejection results with zero state mutation and zero panics.
- **AT-C5 `checked_fixed_point_arithmetic`**: `checked_add/sub` overflow returns `Err`; lint gate (`clippy::arithmetic_side_effects` etc.) active in CI for canonical crates.
- **AT-C6 `unicode_is_byte_distinct`** (Codex #14): composed vs decomposed categorical/text values are distinct values with distinct hashes; documented contract asserted.

**AT-D — reconstruction restriction:**

- **AT-D1 `resume_at_frontier_absent_from_production_surface`** (compile_fail without `test-support`): the symbol is not nameable in a default-feature build.
- **AT-D2 `test_support_feature_is_dev_only`**: metadata policy test asserts no production dependency edge enables `test-support` on core or engine.
- **AT-D3 `arbitrary_frontier_cannot_masquerade_as_continuation`** (Codex #10, under `test-support`): two nonzero-frontier ingresses carry synthesized genesis anchors that match no real fence chain; assert their history digests cannot equal any digest produced by an actual finalization sequence over the same envelopes from frontier 0.

**AT-E — closed-item regression (must all stay green):**

- **AT-E1** full B-04 suite (identity sweep, atomicity, retry, domain separation, profile independence) ported into `spark-engine`.
- **AT-E2** M-04 random-address suite; M-01 config suite + AT-E2a duplicate-diagnostic determinism with ≥3 reordered duplicates and no partial revision (Codex #15).
- **AT-E3** dependency-direction test updated to the new graph + feature assertion.
- **AT-E4** complete B-01 timeline suite (admission timing digest-equivalence, ack binding, fence chain, epoch handoff, partial-prefix slide, reversed delivery, out-of-window non-disturbance, poisoned non-finalizability) ported unchanged; plus **AT-F1** poisoned-slot evidence: opposite-order distinct pairs produce identical state digests, and *different* competing pairs produce different state digests (Codex #12).
- **AT-E5** original implementation-brief corpus items 1–20 remain proved.

**AT-F — evidence/digest hardening:**

- **AT-F2** `empty/staged/poisoned/conflicted` scenarios all pairwise digest-distinct; 50-repeat bit-identical replay retained.
- **AT-F3** per-field envelope mutation sweep: for each semantic field, `StageAcknowledgement::covers` fails after mutation (Codex #11).
- **AT-F4** transcript renamed `outcome_transcript_digest`; transcripts across different profiles/epochs/window widths distinguished through the state digest, documented (Codex #13).

---

### 11. REUSE / REPLACE MAP

| Current code | Classification | Notes |
|---|---|---|
| `spark-core/src/hash.rs`, `clock.rs`, `random.rs`, `authority.rs`, `scope.rs` | **REUSE AS-IS** | — |
| `spark-core/src/id.rs` | **REUSE AFTER SMALL CHANGE** | `from_static` → `try_from_static` + `canonical_tag!` macro; everything else unchanged. |
| `spark-core/src/value.rs` | **REUSE AFTER SMALL CHANGE** | `clamp` → `checked_clamp`/`clamp_to(FixedRange)`; add checked add/sub; rest unchanged. |
| `spark-core/src/timeline.rs` | **REUSE AFTER SMALL CHANGE** | Feature-gate `resume_at_frontier`; `Poisoned` gains order-independent evidence set; all B-01 semantics untouched. |
| `spark-core/src/scheduler.rs` | **REUSE AFTER SMALL CHANGE** | Keep `WorkKey`/`WorkKind`/ordering/digest machinery; replace the conflict arm and `drain_due` return type with §4's model. |
| `spark-core/src/activation.rs` | **REPLACE** | The public raw-declaration mint is the open blocker. Its *algorithms* (fingerprint, atomic batch, error taxonomy) move verbatim into `spark-engine::activation`; the public surface (`DefinitionDeclaration`, public registry) is retired. |
| `spark-core/src/state.rs` | **REUSE AFTER SMALL CHANGE (relocated)** | Validation/write logic and tests are sound; constructor becomes `pub(crate)` in `spark-engine`; adds `manifest_content_hash` binding. |
| `spark-profile/src/{definition,manifest,config,text}.rs` | **REUSE AS-IS (relocated)** into `spark-engine::profile` | Minus `to_declaration` (REMOVE) and public `kind_tag`→`DefinitionKindTag::builtin` exposure (RESTRICT to crate). |
| `spark-profile/src/validate.rs` | **REPLACE (absorbed)** | Its checks fold into the single door; `ValidatedManifest` is superseded by `ActivatedProfile` (which now also carries the manifest hash). |
| `spark-testkit` scenario + corpus | **REUSE AFTER SMALL CHANGE** | Port to new crate paths and dispositions; rename transcript digest; add the v2 corpus; strengthen the two insufficient tests. |
| `TimelineIngress::resume_at_frontier` | **REMOVE/RESTRICT** | Behind `test-support`; superseded in Phase 3 by the evidence constructor (§6). |
| Persistence resume constructor, config→behavior-epoch binding, host/evaluator write facades, richer validator (cycles/dead definitions/fan-out), conflicted-key canonical recovery commands, Windows/Android *execution* gate | **DEFER TO LATER PHASE** | Seams specified here; no Phase-1 implementation. |

---

### 12. IMPLEMENTATION SEQUENCE

Bounded writer sequence (tests first, one commit per numbered step preferred):

```text
1. tests first        Encode AT-A…AT-F (including every compile-fail) against the CURRENT
                      tree; record which fail/refuse-to-compile, mirroring the refoundation
                      baseline discipline. Strengthened replacements land here.
2. crate reshape      Create spark-engine; git-mv spark-profile modules + state.rs into it;
                      retire spark-core::{activation,state}; update testkit paths and the
                      dependency-policy expectations. No semantic change in this step.
3. trusted activation Implement the single door (§3): ActivationRegistry, ActivatedProfile,
                      pub(crate) mints, removed public surfaces, lineage digest,
                      manifest-hash binding on the store. AT-A + AT-E1 green.
4. scheduler conflict Implement §4 (Conflicted evidence model, DrainOutcome, digest
                      coverage; timeline Poisoned evidence for symmetry). AT-B + AT-F1 green.
5. constructor policy Implement §5 (try_from_static + canonical_tag!, FixedRange/checked
                      clamp/arithmetic, clippy lint gate in CI config). AT-C green.
6. reconstruction     Feature-gate resume_at_frontier; add test-support plumbing and the
                      feature-hygiene policy test. AT-D green.
7. closed-item pass   Port/verify every closed suite (B-01, B-04, M-01, M-02, M-04, m-01),
                      transcript rename, Unicode contract docs. AT-E/AT-F green.
8. full regression    cargo fmt --check; clippy (workspace, all targets, -D warnings + the
                      new panic/arithmetic lints); cargo test --workspace; cargo metadata;
                      installed Windows/Android cargo check (static only — no execution
                      claims). Completion report with per-finding disposition;
                      PHASE_2_AUTHORIZATION: NO pending independent review.
```

Hard stops for the writer, unchanged from the convergence decision: if any acceptance test appears to conflict with a frozen Phase-0 contract, stop and report the exact contradiction; do not redesign silently. Do not weaken or delete any listed test to make an API convenient.

---

### 13. WRITER / REVIEWER RECOMMENDATION

- **Writer:** Claude Code — **Opus, HIGH effort**, strictly bound to this document as the controlling architecture. The re-foundation showed Opus/HIGH executes a well-specified boundary correctly where one exists (B-01, M-01, M-02 all passed); the failures were architecture decisions this document now makes, not implementation quality. The writer's latitude is limited to naming, file organization within the map, and test phrasing — not to boundary shape, visibility rulings, or conflict semantics.
- **Independent reviewer:** Codex — **HIGH effort**, unchanged, preserving writer/reviewer separation. The review prompt should require section-by-section conformance to this document (especially §3 visibility rulings and the AT corpus assertions) in addition to free-form falsification.
- **Role separation:** architect (Fable, this document) ≠ writer (Opus) ≠ reviewer (Codex) keeps three independent perspectives. If the v2 pass fails on the same classes again, escalate back to architecture review (Fable) rather than another writer loop — but note that B-02's recurrence risk is now structurally low: after §3 there is no public intermediate mint left to move to.

---

### 14. OPERATOR DECISIONS REQUIRED

**None are required to proceed.** Every decision in this document is ordinary Rust realization within blueprint §35.2 engineer authority (crate/module organization, internal types, API visibility, deterministic algorithms). Specifically:

- The `spark-engine` crate introduction/rename is within "crate/module organization"; ADR-0001's frozen direction rules are preserved and the mechanical check is updated, not weakened.
- The scheduler conflict model is a deterministic realization of the already-frozen no-arrival-authority constitution, using ADR-0003's own poisoning precedent; it changes no product meaning.
- The panic policy and reconstruction restriction change no authority, cost, privacy, or compatibility surface.

No new causal primitive, no authority-model change, no paid/external resource, and no scope expansion is proposed. (For the operator's awareness only, not approval: the crate rename retires the name `spark-profile` until Phase 3 revives it as the parsing/authoring layer above `spark-engine`.)

---

### 15. IMPLEMENTATION AUTHORIZATION RECOMMENDATION

**Authorize a Re-Foundation v2 writer pass immediately**, scoped exactly to Sections 3–12 of this document, on the `phase1-refoundation` line (new branch `phase1-refoundation-v2` from current HEAD, preserving all evidence unchanged), followed by independent Codex review at HIGH effort with a conformance requirement against this document.

**Phase 2 remains unauthorized** until that independent review passes B-02, B-03, and M-03 with no new equivalent-severity finding and confirms no regression of B-01, B-04, M-01, M-02, M-04, and m-01.

---

*End of architecture/process review. No implementation files were modified; this artifact is the only repository change from this review.*
