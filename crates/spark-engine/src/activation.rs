//! The single trusted activation door, the unforgeable artifacts it
//! mints, and the profile-qualified identity lineage it maintains.
//!
//! # One door, one ceremony
//!
//! [`ActivationRegistry::activate`] is the **only** public entry point in
//! S.P.A.R.K. that turns operator-authored profile data into activated
//! authority, and it performs the complete ceremony atomically:
//!
//! ```text
//! ProfileManifest (untrusted authoring input)
//!   -> manifest checks          duplicate definition IDs, profile qualification
//!   -> per-definition checks    non-empty valid scopes (bounds/typing are
//!                               already guaranteed by the validated field types)
//!   -> fingerprints COMPUTED    never accepted; there is no field to put one in
//!   -> immutable-identity check against this registry's committed lineage
//!   -> atomic commit            a batch that fails any check commits nothing
//!   -> ActivatedProfile         private fields, crate-only mint
//! ```
//!
//! Every intermediate step is crate-private. [`DefinitionDeclaration`] —
//! the raw mint input an external reviewer previously executed against —
//! is not part of the public API; `DefinitionSpec` has no
//! `to_declaration`; [`DefinitionKindTag`]'s constructors are
//! `pub(crate)`, so an external caller cannot assert the built-in/custom
//! identity bit; and neither [`ActivatedDefinition`] nor
//! [`ActivatedProfile`] has a public constructor. Possession of either is
//! therefore proof that the ceremony ran — which was *not* true while a
//! public mint accepted caller-authored declarations.
//!
//! ```compile_fail
//! use spark_engine::activation::ActivatedDefinition;
//! use spark_core::authority::Authority;
//! use spark_core::hash::Digest;
//! // Private fields, no constructor: activated authority cannot be
//! // fabricated by an external crate.
//! let forged = ActivatedDefinition {
//!     authority: Authority::Derived,
//!     fingerprint: Digest::ZERO,
//! };
//! ```
//!
//! ```compile_fail
//! use spark_engine::activation::ActivatedProfile;
//! use spark_core::id::ProfileId;
//! let forged = ActivatedProfile {
//!     profile_id: ProfileId::new("game-world").unwrap(),
//! };
//! ```
//!
//! # Registry lifecycle across instances
//!
//! Rust cannot prevent a second [`ActivationRegistry::new`] in one
//! process, and claiming otherwise is exactly the kind of convention that
//! kept this defect alive. The invariant is made real by content
//! addressing: identical manifests activated in two independent registries
//! produce byte-identical `manifest_content_hash` and `activation_hash`
//! (so the two artifacts are interchangeable *because* they are
//! deterministically identical), while divergent manifests or divergent
//! authority facts produce different hashes on every downstream artifact
//! including the [`crate::state::StateStore`]'s canonical state digest.
//! ADR-0006 continuation refuses a hash mismatch, so divergence cannot be
//! silent. The production composition root (Phase 3's embedded runtime or
//! service) owns exactly one registry; Phase 1 states that rule here and
//! tests the content-addressing property that makes a violation visible.

use crate::profile::definition::DefinitionSpec;
use crate::profile::manifest::ProfileManifest;
use crate::state::StateStore;
use spark_core::authority::Authority;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{CanonicalTag, DefinitionId, ProfileId};
use spark_core::scope::ScopeKind;
use spark_core::value::ValueConstraint;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// A definition's `kind` discriminator
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1), as activated identity sees it.
///
/// The kind taxonomy is profile vocabulary (ADR-0004's taxonomy rule), so
/// nothing here hard-codes a catalog — but a kind *is* immutable
/// definition identity, so it must be bounded and domain-separated. A
/// profile-declared custom kind literally named `trigger` can never
/// collide with the baseline `trigger`, because the built-in/custom bit is
/// an explicit domain separator rather than a string prefix.
///
/// Both constructors are `pub(crate)`. An external caller chooses a kind
/// through the [`crate::profile::definition::DefinitionKind`] enum (whose
/// `custom` constructor validates the tag); it cannot assert that a
/// profile-invented tag belongs to the engine's baseline vocabulary.
///
/// ```compile_fail
/// use spark_engine::activation::DefinitionKindTag;
/// use spark_core::id::CanonicalTag;
/// // The built-in/custom identity bit is not caller-assertable.
/// let asserted = DefinitionKindTag::builtin(CanonicalTag::new("weather").unwrap());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefinitionKindTag {
    custom: bool,
    tag: CanonicalTag,
}

impl DefinitionKindTag {
    /// A kind drawn from the engine's own baseline vocabulary. Crate-only:
    /// only [`crate::profile::definition::DefinitionKind`]'s baseline
    /// variants may claim it.
    pub(crate) fn builtin(tag: CanonicalTag) -> Self {
        Self { custom: false, tag }
    }

    /// A kind declared as profile vocabulary. Crate-only; reached from
    /// outside through
    /// [`crate::profile::definition::DefinitionKind::custom`], which
    /// validates and bounds the tag.
    pub(crate) fn custom(tag: CanonicalTag) -> Self {
        Self { custom: true, tag }
    }

    pub fn is_custom(&self) -> bool {
        self.custom
    }

    pub fn tag(&self) -> &CanonicalTag {
        &self.tag
    }

    pub(crate) fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str(if self.custom {
            "definition_kind.custom"
        } else {
            "definition_kind.builtin"
        });
        self.tag.canonicalize(enc);
    }
}

impl fmt::Display for DefinitionKindTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.custom {
            write!(f, "custom:{}", self.tag)
        } else {
            write!(f, "{}", self.tag)
        }
    }
}

/// The crate-private identity-bearing projection of one
/// [`DefinitionSpec`]: exactly the fields that constitute immutable
/// definition identity, and nothing else.
///
/// This type is deliberately **not** public. In the failed passes its
/// public equivalent was the mint input, which let any caller author every
/// canonical authority fact and then call a public `activate`. Here the
/// projection happens inside the door, from a spec the door has already
/// checked.
///
/// Note what is absent: there is no fingerprint field, so a caller cannot
/// assert an identity it did not earn even if this type were reachable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DefinitionDeclaration {
    pub(crate) profile_id: ProfileId,
    pub(crate) definition_id: DefinitionId,
    pub(crate) kind: DefinitionKindTag,
    pub(crate) authority: Authority,
    pub(crate) value_constraint: ValueConstraint,
    pub(crate) valid_scopes: BTreeSet<ScopeKind>,
}

pub(crate) fn declaration_of(spec: &DefinitionSpec) -> DefinitionDeclaration {
    DefinitionDeclaration {
        profile_id: spec.profile_id.clone(),
        definition_id: spec.id.clone(),
        kind: spec.kind.kind_tag(),
        authority: spec.authority,
        value_constraint: spec.value_constraint.clone(),
        valid_scopes: spec.valid_scopes.clone(),
    }
}

pub(crate) fn declaration_fingerprint(declaration: &DefinitionDeclaration) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("definition_fingerprint");
    declaration.profile_id.canonicalize(&mut enc);
    declaration.definition_id.canonicalize(&mut enc);
    declaration.kind.canonicalize(&mut enc);
    declaration.value_constraint.canonicalize(&mut enc);
    declaration.authority.canonicalize(&mut enc);
    enc.push_str(declaration.authority.implied_write_class().tag());
    // `valid_scopes` is a BTreeSet, so iteration is already in a stable
    // total order; the canonical tags are pushed length-prefixed, so two
    // different scope sets can never alias.
    enc.push_u64(declaration.valid_scopes.len() as u64);
    for scope in &declaration.valid_scopes {
        enc.push_str(&scope.canonical_tag());
    }
    enc.finish()
}

/// The immutable identity fingerprint of one definition (ADR-0004
/// "Content identity"): profile ID, definition ID, kind (domain-separated
/// built-in/custom), value constraint (which implies value type),
/// authority mode, the write class that authority implies, and the sorted
/// valid scopes.
///
/// This is a pure function of already-validated fields, and it is public
/// so a reviewer or a future authoring tool can recompute an identity
/// independently. Being able to *compute* a fingerprint grants nothing:
/// only [`ActivationRegistry::activate`] can mint an
/// [`ActivatedDefinition`], and it always recomputes the fingerprint
/// itself from the spec it validated.
pub fn definition_fingerprint(spec: &DefinitionSpec) -> Digest {
    declaration_fingerprint(&declaration_of(spec))
}

/// Rejects one definition offered to the activation door.
///
/// Every error found in a manifest is reported, not just the first, so an
/// operator fixing a profile sees the whole problem at once — and a batch
/// that produces any error commits nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// The same definition ID appears more than once in one manifest.
    /// Insertion order must never pick a winner, so the whole manifest is
    /// rejected.
    DuplicateDefinitionId {
        profile_id: ProfileId,
        definition_id: DefinitionId,
    },
    /// A definition's `profile_id` does not match the manifest it was
    /// offered in (ADR-0004 profile/trust-domain isolation).
    ProfileMismatch {
        definition_id: DefinitionId,
        manifest_profile_id: ProfileId,
        definition_profile_id: ProfileId,
    },
    /// A definition with no valid scope can never be written, so it is a
    /// malformed declaration rather than an inert one.
    NoValidScope {
        profile_id: ProfileId,
        definition_id: DefinitionId,
    },
    /// The `(profile_id, definition_id)` was previously activated in this
    /// registry with a different immutable identity. Per ADR-0002/ADR-0004
    /// an authority, value-type, kind, or scope-compatibility change
    /// requires a new definition ID, an explicit migration, an authority
    /// ADR, and operator approval — never a redeclaration under the same
    /// ID.
    IdentityConflict {
        profile_id: ProfileId,
        definition_id: DefinitionId,
        previous_fingerprint: Digest,
        attempted_fingerprint: Digest,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::DuplicateDefinitionId {
                profile_id,
                definition_id,
            } => write!(
                f,
                "definition '{definition_id}' is declared more than once in profile '{profile_id}''s manifest"
            ),
            ValidationError::ProfileMismatch {
                definition_id,
                manifest_profile_id,
                definition_profile_id,
            } => write!(
                f,
                "definition '{definition_id}' declares profile '{definition_profile_id}' but was offered for activation in profile '{manifest_profile_id}'"
            ),
            ValidationError::NoValidScope {
                profile_id,
                definition_id,
            } => write!(
                f,
                "definition '{definition_id}' in profile '{profile_id}' declares no valid scope"
            ),
            ValidationError::IdentityConflict {
                profile_id,
                definition_id,
                previous_fingerprint,
                attempted_fingerprint,
            } => write!(
                f,
                "definition '{definition_id}' in profile '{profile_id}' was activated with fingerprint {previous_fingerprint} and cannot be reactivated with fingerprint {attempted_fingerprint} under the same ID; an identity change (kind, value constraint, authority, or valid scopes) requires a new definition ID, an explicit migration, and an ADR"
            ),
        }
    }
}

impl std::error::Error for ValidationError {}

/// One definition whose identity was computed and committed by
/// [`ActivationRegistry::activate`].
///
/// Every field is private and there is no public constructor, so a value
/// of this type is itself proof that the activation ceremony ran.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedDefinition {
    profile_id: ProfileId,
    definition_id: DefinitionId,
    kind: DefinitionKindTag,
    authority: Authority,
    value_constraint: ValueConstraint,
    valid_scopes: BTreeSet<ScopeKind>,
    fingerprint: Digest,
}

impl ActivatedDefinition {
    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    pub fn definition_id(&self) -> &DefinitionId {
        &self.definition_id
    }

    pub fn kind(&self) -> &DefinitionKindTag {
        &self.kind
    }

    pub fn authority(&self) -> Authority {
        self.authority
    }

    pub fn value_constraint(&self) -> &ValueConstraint {
        &self.value_constraint
    }

    pub fn valid_scopes(&self) -> &BTreeSet<ScopeKind> {
        &self.valid_scopes
    }

    /// The immutable identity fingerprint the door computed for this
    /// definition. It was never accepted from a caller.
    pub fn fingerprint(&self) -> &Digest {
        &self.fingerprint
    }
}

/// The unforgeable activation artifact: the complete, immutable,
/// profile-qualified activated definition set, plus the exact-artifact
/// binding a Phase-3 save will need.
///
/// Private fields, no public constructor, no mutation method. It is minted
/// only inside [`ActivationRegistry::activate`], and it is the only value
/// from which a [`StateStore`] can exist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedProfile {
    profile_id: ProfileId,
    manifest_content_hash: Digest,
    activation_hash: Digest,
    definitions: BTreeMap<DefinitionId, ActivatedDefinition>,
}

impl ActivatedProfile {
    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    /// The exact content hash of the manifest that was activated
    /// (ADR-0004 content identity, ADR-0006 exact-artifact binding). A
    /// reused human version label over different content produces a
    /// different value here.
    pub fn manifest_content_hash(&self) -> &Digest {
        &self.manifest_content_hash
    }

    /// A deterministic content hash over the whole activated set: the
    /// profile plus every definition's door-computed fingerprint, in
    /// stable order. Two activations of the same definitions produce the
    /// same activation hash regardless of declaration order, and two
    /// divergent activations never can.
    pub fn activation_hash(&self) -> &Digest {
        &self.activation_hash
    }

    pub fn definition(&self, definition_id: &DefinitionId) -> Option<&ActivatedDefinition> {
        self.definitions.get(definition_id)
    }

    /// Every activated definition, in stable ascending definition-ID
    /// order. Shared references only: an activated profile cannot be
    /// edited after activation.
    pub fn definitions(&self) -> impl Iterator<Item = &ActivatedDefinition> {
        self.definitions.values()
    }

    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// The **only** path to a [`StateStore`]. The store's constructor is
    /// `pub(crate)`, so possession of an activated profile is the sole
    /// way canonical state can come into existence.
    pub fn into_state_store(self) -> StateStore {
        StateStore::from_activation(self)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ProfileId,
        Digest,
        Digest,
        BTreeMap<DefinitionId, ActivatedDefinition>,
    ) {
        (
            self.profile_id,
            self.manifest_content_hash,
            self.activation_hash,
            self.definitions,
        )
    }
}

/// One committed activation, in the order it was committed.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ActivationRecord {
    profile_id: ProfileId,
    manifest_content_hash: Digest,
    activation_hash: Digest,
}

impl ActivationRecord {
    fn record_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("activation_record");
        self.profile_id.canonicalize(&mut enc);
        enc.push_digest(&self.manifest_content_hash);
        enc.push_digest(&self.activation_hash);
        enc.finish()
    }
}

/// Owns every profile's canonical identity lineage within this runtime,
/// and holds the one public activation door.
///
/// The production composition root (Phase 3's embedded runtime or service
/// wrapper) holds exactly one of these per deployment. Content addressing
/// (see the module documentation) makes any rogue second instance either
/// behaviorally identical or visibly divergent on every artifact it
/// produces, so the composition rule is enforceable rather than merely
/// hoped for.
#[derive(Debug, Default, Clone)]
pub struct ActivationRegistry {
    /// Immutable profile-qualified definition identity: the B-04
    /// invariant, preserved verbatim from the independently closed work.
    identities: BTreeMap<(ProfileId, DefinitionId), Digest>,
    /// Append-only, deduplicated activation lineage. A repeated identical
    /// activation is idempotent and does not grow the lineage.
    lineage: Vec<ActivationRecord>,
    committed_records: BTreeSet<Digest>,
    /// Phase-2 rule-set lineage: append-only, deduplicated content hashes.
    rule_set_lineage: Vec<Digest>,
    committed_rule_sets: BTreeSet<Digest>,
}

impl ActivationRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// **THE door.** Validates a whole manifest and, only if every check
    /// passes, computes each definition's fingerprint, commits the
    /// identities, extends the lineage, and mints the
    /// [`ActivatedProfile`].
    ///
    /// The ceremony is atomic with respect to this registry: candidate
    /// fingerprints are compared against a read-only view and the registry
    /// is mutated exactly once, after every check has passed. A manifest
    /// that fails partway leaves the registry byte-for-byte unchanged, so
    /// a corrected retry always succeeds cleanly. Every error is reported,
    /// not just the first.
    ///
    /// Re-activating an identical manifest is idempotent: the identities
    /// already match, the lineage does not grow, and the resulting
    /// artifact's hashes are byte-identical to the first activation's.
    pub fn activate(
        &mut self,
        manifest: &ProfileManifest,
    ) -> Result<ActivatedProfile, Vec<ValidationError>> {
        let profile_id = manifest.profile_id();
        let mut errors: Vec<ValidationError> = Vec::new();
        let mut seen: BTreeSet<DefinitionId> = BTreeSet::new();
        let mut candidates: BTreeMap<DefinitionId, ActivatedDefinition> = BTreeMap::new();

        for spec in manifest.definitions() {
            if !seen.insert(spec.id.clone()) {
                errors.push(ValidationError::DuplicateDefinitionId {
                    profile_id: profile_id.clone(),
                    definition_id: spec.id.clone(),
                });
                continue;
            }

            if &spec.profile_id != profile_id {
                errors.push(ValidationError::ProfileMismatch {
                    definition_id: spec.id.clone(),
                    manifest_profile_id: profile_id.clone(),
                    definition_profile_id: spec.profile_id.clone(),
                });
                continue;
            }

            if spec.valid_scopes.is_empty() {
                errors.push(ValidationError::NoValidScope {
                    profile_id: profile_id.clone(),
                    definition_id: spec.id.clone(),
                });
                continue;
            }

            let declaration = declaration_of(spec);
            let fingerprint = declaration_fingerprint(&declaration);
            if let Some(previous) = self.identities.get(&(
                declaration.profile_id.clone(),
                declaration.definition_id.clone(),
            )) {
                if previous != &fingerprint {
                    errors.push(ValidationError::IdentityConflict {
                        profile_id: declaration.profile_id.clone(),
                        definition_id: declaration.definition_id.clone(),
                        previous_fingerprint: previous.clone(),
                        attempted_fingerprint: fingerprint,
                    });
                    continue;
                }
            }

            candidates.insert(
                declaration.definition_id.clone(),
                ActivatedDefinition {
                    profile_id: declaration.profile_id,
                    definition_id: declaration.definition_id,
                    kind: declaration.kind,
                    authority: declaration.authority,
                    value_constraint: declaration.value_constraint,
                    valid_scopes: declaration.valid_scopes,
                    fingerprint,
                },
            );
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        let activation_hash = compute_activation_hash(profile_id, &candidates);
        let manifest_content_hash = manifest.manifest_content_hash();

        // Every check has passed: commit atomically.
        for definition in candidates.values() {
            self.identities.insert(
                (
                    definition.profile_id.clone(),
                    definition.definition_id.clone(),
                ),
                definition.fingerprint.clone(),
            );
        }

        let record = ActivationRecord {
            profile_id: profile_id.clone(),
            manifest_content_hash: manifest_content_hash.clone(),
            activation_hash: activation_hash.clone(),
        };
        if self.committed_records.insert(record.record_digest()) {
            self.lineage.push(record);
        }

        Ok(ActivatedProfile {
            profile_id: profile_id.clone(),
            manifest_content_hash,
            activation_hash,
            definitions: candidates,
        })
    }

    /// The committed immutable identity of one `(profile, definition)`, if
    /// it has ever been activated through this registry.
    pub fn fingerprint_of(
        &self,
        profile_id: &ProfileId,
        definition_id: &DefinitionId,
    ) -> Option<&Digest> {
        self.identities
            .get(&(profile_id.clone(), definition_id.clone()))
    }

    /// A deterministic digest of the full activation lineage, so Phase-3
    /// persistence can bind a save to the exact registry history that
    /// produced it (ADR-0006).
    ///
    /// The lineage is append-only and deduplicated: replaying the same
    /// sequence of activations reproduces the digest exactly, repeating an
    /// identical activation does not change it, and two registries whose
    /// activation contents differ never agree.
    pub fn lineage_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("activation_lineage");
        enc.push_u64(self.lineage.len() as u64);
        for record in &self.lineage {
            enc.push_digest(&record.record_digest());
        }
        enc.finish()
    }

    /// The number of distinct activations committed to this registry's
    /// lineage.
    pub fn lineage_len(&self) -> usize {
        self.lineage.len()
    }

    /// **THE rule-set door** (Phase-2 freeze v1 Q1). Validates a whole rule set
    /// against an activated profile minted by this registry and, only if every
    /// check passes, mints the [`ActivatedRuleSet`] and records it in the
    /// rule-set lineage. A failure commits nothing and reports every error.
    ///
    /// The rule-set lineage is kept separate from the definition lineage, so
    /// [`lineage_digest`](Self::lineage_digest) is byte-identical for every
    /// registry that never activates a rule set (Phase-1 digest stability).
    pub fn activate_rule_set(
        &mut self,
        profile: &ActivatedProfile,
        spec: &crate::rules::RuleSetSpec,
    ) -> Result<crate::rules::ActivatedRuleSet, Vec<crate::rules::RuleSetError>> {
        let minted_here = profile.definitions().all(|d| {
            self.identities
                .get(&(d.profile_id().clone(), d.definition_id().clone()))
                == Some(d.fingerprint())
        });
        if !minted_here {
            return Err(vec![crate::rules::RuleSetError::ProfileNotActivatedHere {
                profile_id: profile.profile_id().clone(),
            }]);
        }
        let activated = crate::rules::validate_and_mint(profile, spec, true)?;
        let hash = activated.content_hash().clone();
        if self.committed_rule_sets.insert(hash.clone()) {
            self.rule_set_lineage.push(hash);
        }
        Ok(activated)
    }

    /// Test-support only: the rule-set door with the static zero-delay depth
    /// bound disabled (every other check, including cycle rejection, still
    /// runs). Sound static validation makes runtime depth overflow unreachable;
    /// this seam exists solely so the defense-in-depth conversion to strictly
    /// later work (v1 Q4, v2 §11; AT-I21) is executable.
    #[cfg(any(test, feature = "test-support"))]
    pub fn activate_rule_set_without_depth_bound(
        &mut self,
        profile: &ActivatedProfile,
        spec: &crate::rules::RuleSetSpec,
    ) -> Result<crate::rules::ActivatedRuleSet, Vec<crate::rules::RuleSetError>> {
        let activated = crate::rules::validate_and_mint(profile, spec, false)?;
        let hash = activated.content_hash().clone();
        if self.committed_rule_sets.insert(hash.clone()) {
            self.rule_set_lineage.push(hash);
        }
        Ok(activated)
    }

    /// A deterministic digest of the append-only, deduplicated rule-set
    /// activation lineage.
    pub fn rule_set_lineage_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("rule_set_lineage");
        enc.push_u64(self.rule_set_lineage.len() as u64);
        for h in &self.rule_set_lineage {
            enc.push_digest(h);
        }
        enc.finish()
    }

    pub fn rule_set_lineage_len(&self) -> usize {
        self.rule_set_lineage.len()
    }

    pub fn len(&self) -> usize {
        self.identities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.identities.is_empty()
    }
}

fn compute_activation_hash(
    profile_id: &ProfileId,
    definitions: &BTreeMap<DefinitionId, ActivatedDefinition>,
) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("activated_profile");
    profile_id.canonicalize(&mut enc);
    enc.push_u64(definitions.len() as u64);
    for definition in definitions.values() {
        let mut inner = CanonicalEncoder::new();
        definition.definition_id.canonicalize(&mut inner);
        inner.push_digest(&definition.fingerprint);
        enc.push_block(&inner);
    }
    enc.finish()
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
mod tests {
    use super::*;
    use crate::profile::definition::DefinitionKind;
    use crate::profile::text::BoundedText;
    use spark_core::value::FixedPoint;

    fn profile() -> ProfileId {
        ProfileId::new("game-world").unwrap()
    }

    fn spec(id: &str, authority: Authority) -> DefinitionSpec {
        DefinitionSpec {
            profile_id: profile(),
            id: DefinitionId::new(id).unwrap(),
            kind: DefinitionKind::Trait,
            domain: None,
            layer: None,
            value_constraint: ValueConstraint::fixed(
                FixedPoint::ZERO,
                FixedPoint::from_integer(1).unwrap(),
            )
            .unwrap(),
            authority,
            valid_scopes: BTreeSet::from([ScopeKind::Actor]),
            enabled: true,
            version: 1,
            description: BoundedText::new("test definition").unwrap(),
            behavioral_leverage: None,
        }
    }

    fn manifest(specs: Vec<DefinitionSpec>) -> ProfileManifest {
        ProfileManifest::new(profile(), BoundedText::new("1.0.0").unwrap(), specs)
    }

    /// B-04 (ported): the door computes the fingerprint itself and never
    /// accepts one.
    #[test]
    fn activation_computes_the_fingerprint_itself() {
        let mut registry = ActivationRegistry::new();
        let s = spec("trait.curiosity", Authority::SparkOwned);
        let activated = registry.activate(&manifest(vec![s.clone()])).unwrap();
        let definition = activated
            .definition(&DefinitionId::new("trait.curiosity").unwrap())
            .unwrap();
        assert_eq!(definition.fingerprint(), &definition_fingerprint(&s));
        assert_ne!(definition.fingerprint(), &Digest::ZERO);
    }

    /// B-04 (ported): duplicate definition IDs reject the whole manifest,
    /// in either arrival order, committing nothing.
    #[test]
    fn duplicate_definition_ids_reject_the_whole_manifest_in_either_order() {
        let a = spec("trait.curiosity", Authority::SparkOwned);
        let b = spec("trait.curiosity", Authority::HostOwned);

        let mut forward = ActivationRegistry::new();
        let errors = forward
            .activate(&manifest(vec![a.clone(), b.clone()]))
            .unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::DuplicateDefinitionId { .. }
        ));

        let mut reversed = ActivationRegistry::new();
        assert!(reversed.activate(&manifest(vec![b, a])).is_err());

        assert!(forward.is_empty(), "a failed manifest must commit nothing");
        assert!(reversed.is_empty());
        assert_eq!(forward.lineage_len(), 0);
    }

    /// B-04 (ported): an identity change under the same definition ID is
    /// rejected.
    #[test]
    fn identity_change_under_the_same_id_rejects() {
        let mut registry = ActivationRegistry::new();
        registry
            .activate(&manifest(vec![spec(
                "state.food.availability",
                Authority::HostOwned,
            )]))
            .unwrap();
        let errors = registry
            .activate(&manifest(vec![spec(
                "state.food.availability",
                Authority::SparkOwned,
            )]))
            .unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::IdentityConflict { .. }
        ));
    }

    /// B-04 (ported): re-activating an identical manifest is idempotent
    /// and produces byte-identical hashes.
    #[test]
    fn reactivating_the_same_identity_is_idempotent() {
        let mut registry = ActivationRegistry::new();
        let m = manifest(vec![spec("trait.curiosity", Authority::SparkOwned)]);
        let first = registry.activate(&m).unwrap();
        let lineage_after_first = registry.lineage_digest();
        let second = registry.activate(&m).unwrap();
        assert_eq!(first.activation_hash(), second.activation_hash());
        assert_eq!(
            first.manifest_content_hash(),
            second.manifest_content_hash()
        );
        assert_eq!(
            lineage_after_first,
            registry.lineage_digest(),
            "a repeated identical activation must not grow the lineage"
        );
        assert_eq!(registry.lineage_len(), 1);
    }

    /// B-04 (ported): a manifest that fails partway leaves the registry
    /// unchanged and a corrected retry succeeds cleanly.
    #[test]
    fn failed_manifest_leaves_registry_unchanged_and_retry_succeeds() {
        let mut registry = ActivationRegistry::new();
        let good = spec("trait.curiosity", Authority::SparkOwned);
        let mut bad = spec("trait.skepticism", Authority::SparkOwned);
        bad.valid_scopes = BTreeSet::new();

        let errors = registry
            .activate(&manifest(vec![good.clone(), bad]))
            .unwrap_err();
        assert!(matches!(errors[0], ValidationError::NoValidScope { .. }));
        assert!(registry.fingerprint_of(&profile(), &good.id).is_none());
        assert_eq!(registry.lineage_len(), 0);

        let fixed = spec("trait.skepticism", Authority::SparkOwned);
        assert!(registry.activate(&manifest(vec![good, fixed])).is_ok());
        assert_eq!(registry.len(), 2);
    }

    /// B-04 (ported): a definition naming a foreign profile is rejected.
    #[test]
    fn foreign_profile_definition_rejects() {
        let mut registry = ActivationRegistry::new();
        let mut foreign = spec("trait.curiosity", Authority::SparkOwned);
        foreign.profile_id = ProfileId::new("mci-social").unwrap();
        let errors = registry.activate(&manifest(vec![foreign])).unwrap_err();
        assert!(matches!(errors[0], ValidationError::ProfileMismatch { .. }));
        assert!(registry.is_empty());
    }

    /// B-04 (ported): the same definition ID in two independent profiles
    /// does not conflict.
    #[test]
    fn same_definition_id_in_independent_profiles_does_not_conflict() {
        let mut registry = ActivationRegistry::new();
        registry
            .activate(&manifest(vec![spec(
                "trait.curiosity",
                Authority::SparkOwned,
            )]))
            .unwrap();

        let other = ProfileId::new("mci-social").unwrap();
        let mut foreign = spec("trait.curiosity", Authority::HostOwned);
        foreign.profile_id = other.clone();
        let other_manifest =
            ProfileManifest::new(other, BoundedText::new("1.0.0").unwrap(), vec![foreign]);
        assert!(registry.activate(&other_manifest).is_ok());
    }

    /// B-04 (ported): activation identity does not depend on declaration
    /// order.
    #[test]
    fn activation_hash_is_declaration_order_independent() {
        let a = spec("trait.curiosity", Authority::SparkOwned);
        let b = spec("trait.skepticism", Authority::SparkOwned);

        let mut forward = ActivationRegistry::new();
        let mut reversed = ActivationRegistry::new();
        let f = forward
            .activate(&manifest(vec![a.clone(), b.clone()]))
            .unwrap();
        let r = reversed.activate(&manifest(vec![b, a])).unwrap();
        assert_eq!(f.activation_hash(), r.activation_hash());
        assert_eq!(f.manifest_content_hash(), r.manifest_content_hash());
    }

    /// B-04 (ported): every immutable identity field reaches the
    /// fingerprint.
    #[test]
    fn every_immutable_identity_field_changes_the_fingerprint() {
        let base = spec("trait.curiosity", Authority::SparkOwned);
        let baseline = definition_fingerprint(&base);

        let mut other_authority = base.clone();
        other_authority.authority = Authority::HostOwned;
        assert_ne!(definition_fingerprint(&other_authority), baseline);

        let mut other_kind = base.clone();
        other_kind.kind = DefinitionKind::Goal;
        assert_ne!(definition_fingerprint(&other_kind), baseline);

        let mut other_constraint = base.clone();
        other_constraint.value_constraint = ValueConstraint::boolean();
        assert_ne!(definition_fingerprint(&other_constraint), baseline);

        let mut other_scopes = base.clone();
        other_scopes.valid_scopes = BTreeSet::from([ScopeKind::Actor, ScopeKind::Household]);
        assert_ne!(definition_fingerprint(&other_scopes), baseline);

        let mut other_profile = base.clone();
        other_profile.profile_id = ProfileId::new("mci-social").unwrap();
        assert_ne!(definition_fingerprint(&other_profile), baseline);

        let mut other_id = base.clone();
        other_id.id = DefinitionId::new("trait.skepticism").unwrap();
        assert_ne!(definition_fingerprint(&other_id), baseline);
    }

    /// B-04 (ported): a built-in kind and a custom kind spelled the same
    /// way are different identities.
    #[test]
    fn builtin_and_custom_kinds_with_the_same_spelling_are_distinct() {
        let mut builtin = spec("trigger.drought", Authority::SparkOwned);
        builtin.kind = DefinitionKind::Trigger;
        let mut custom = spec("trigger.drought", Authority::SparkOwned);
        custom.kind = DefinitionKind::custom("trigger").unwrap();
        assert_ne!(
            definition_fingerprint(&builtin),
            definition_fingerprint(&custom)
        );
    }

    /// AT-A8: the lineage is deterministic, append-only, and sensitive to
    /// activation content but idempotent under replay.
    #[test]
    fn lineage_is_append_only_and_deterministic() {
        let m1 = manifest(vec![spec("trait.curiosity", Authority::SparkOwned)]);
        let m2 = manifest(vec![spec("trait.skepticism", Authority::SparkOwned)]);

        let mut forward = ActivationRegistry::new();
        forward.activate(&m1).unwrap();
        forward.activate(&m2).unwrap();

        let mut replay = ActivationRegistry::new();
        replay.activate(&m1).unwrap();
        replay.activate(&m2).unwrap();
        assert_eq!(forward.lineage_digest(), replay.lineage_digest());

        let mut reversed = ActivationRegistry::new();
        reversed.activate(&m2).unwrap();
        reversed.activate(&m1).unwrap();
        assert_ne!(forward.lineage_digest(), reversed.lineage_digest());

        // Distinct manifests grow the lineage; a replay of one does not.
        assert_eq!(forward.lineage_len(), 2);
        forward.activate(&m1).unwrap();
        assert_eq!(forward.lineage_len(), 2);
    }

    /// An empty registry has a stable, non-zero lineage digest, so
    /// "no activations yet" is itself a canonical fact.
    #[test]
    fn empty_lineage_digest_is_stable_and_distinct() {
        let empty = ActivationRegistry::new();
        assert_eq!(
            empty.lineage_digest(),
            ActivationRegistry::new().lineage_digest()
        );
        let mut one = ActivationRegistry::new();
        one.activate(&manifest(vec![spec(
            "trait.curiosity",
            Authority::SparkOwned,
        )]))
        .unwrap();
        assert_ne!(empty.lineage_digest(), one.lineage_digest());
    }
}
