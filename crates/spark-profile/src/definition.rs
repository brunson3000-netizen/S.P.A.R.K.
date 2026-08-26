//! `DefinitionSpec`: the logical shape of one profile definition
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1). The exact on-disk syntax is
//! provisional (ADR-0004); this struct is the deterministic,
//! format-independent logical representation used for validation and
//! content hashing.
//!
//! Re-founded per `PHASE_1_REFOUNDATION_BRIEF_v0.1.md`:
//!
//! - Every canonical string here is a bounded validated type. A custom
//!   definition kind is a [`CanonicalTag`], not an arbitrary `String`;
//!   `domain`/`layer` are canonical tags; `description` is
//!   [`BoundedText`]. An unbounded custom kind used to be accepted and
//!   became immutable definition identity.
//! - There is no `to_definition_schema`. A spec cannot promote itself into
//!   activated authority; it can only produce a
//!   [`DefinitionDeclaration`], which carries no fingerprint and no trust
//!   and must still pass
//!   [`spark_core::activation::DefinitionIdentityRegistry::activate`].

use crate::text::BoundedText;
use spark_core::activation::{DefinitionDeclaration, DefinitionKindTag};
use spark_core::authority::Authority;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{CanonicalTag, DefinitionId, ProfileId};
use spark_core::scope::ScopeKind;
use spark_core::value::ValueConstraint;
use std::collections::BTreeSet;

/// `kind` in the common definition schema (`CONTROLLING_BLUEPRINT_v0.2.md`
/// §12.1: "trigger/state/trait/appraisal/goal/behavior/etc"). The
/// baseline variants mirror the frozen causal grammar's primitive roles;
/// `Custom` keeps the taxonomy open as profile vocabulary per ADR-0004's
/// taxonomy rule, without requiring a Rust change to add a new kind tag.
///
/// `Custom` carries a bounded [`CanonicalTag`], so an oversized or
/// malformed custom kind cannot be constructed and therefore cannot become
/// immutable definition identity. Built-in and custom kinds are
/// domain-separated by [`DefinitionKindTag`], so `Trigger` and
/// `Custom("trigger")` are different identities even though they read the
/// same.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionKind {
    Trigger,
    StateDefinition,
    Trait,
    Appraisal,
    Goal,
    Behavior,
    Relationship,
    Custom(CanonicalTag),
}

impl DefinitionKind {
    /// Builds a `Custom` kind from a raw string, rejecting anything that
    /// is not a valid bounded canonical tag.
    pub fn custom(tag: impl Into<String>) -> Result<Self, spark_core::id::StableIdError> {
        Ok(DefinitionKind::Custom(CanonicalTag::new(tag)?))
    }

    /// The kernel-side, domain-separated representation of this kind.
    ///
    /// The baseline vocabulary is expressed as compile-time constants
    /// built with [`CanonicalTag::from_static`], so each literal is
    /// validated by the compiler. This function therefore has no fallible
    /// or panicking path at all: an invalid built-in tag would be a build
    /// failure, not a runtime surprise.
    pub fn kind_tag(&self) -> DefinitionKindTag {
        const TRIGGER: CanonicalTag = CanonicalTag::from_static("trigger");
        const STATE_DEFINITION: CanonicalTag = CanonicalTag::from_static("state_definition");
        const TRAIT: CanonicalTag = CanonicalTag::from_static("trait");
        const APPRAISAL: CanonicalTag = CanonicalTag::from_static("appraisal");
        const GOAL: CanonicalTag = CanonicalTag::from_static("goal");
        const BEHAVIOR: CanonicalTag = CanonicalTag::from_static("behavior");
        const RELATIONSHIP: CanonicalTag = CanonicalTag::from_static("relationship");

        match self {
            DefinitionKind::Trigger => DefinitionKindTag::builtin(TRIGGER),
            DefinitionKind::StateDefinition => DefinitionKindTag::builtin(STATE_DEFINITION),
            DefinitionKind::Trait => DefinitionKindTag::builtin(TRAIT),
            DefinitionKind::Appraisal => DefinitionKindTag::builtin(APPRAISAL),
            DefinitionKind::Goal => DefinitionKindTag::builtin(GOAL),
            DefinitionKind::Behavior => DefinitionKindTag::builtin(BEHAVIOR),
            DefinitionKind::Relationship => DefinitionKindTag::builtin(RELATIONSHIP),
            DefinitionKind::Custom(tag) => DefinitionKindTag::custom(tag.clone()),
        }
    }
}

/// `behavioral_leverage` metadata (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehavioralLeverage {
    High,
    Medium,
    Low,
}

impl BehavioralLeverage {
    pub fn tag(&self) -> &'static str {
        match self {
            BehavioralLeverage::High => "high",
            BehavioralLeverage::Medium => "medium",
            BehavioralLeverage::Low => "low",
        }
    }
}

/// The logical, format-independent shape of one profile definition
/// (common fields per `CONTROLLING_BLUEPRINT_v0.2.md` §12.1).
/// `profile_id` is carried explicitly so identity, fingerprinting, and the
/// resulting activated schema are all profile-qualified end to end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionSpec {
    pub profile_id: ProfileId,
    pub id: DefinitionId,
    pub kind: DefinitionKind,
    pub domain: Option<CanonicalTag>,
    pub layer: Option<CanonicalTag>,
    pub value_constraint: ValueConstraint,
    pub authority: Authority,
    pub valid_scopes: BTreeSet<ScopeKind>,
    pub enabled: bool,
    pub version: u32,
    pub description: BoundedText,
    pub behavioral_leverage: Option<BehavioralLeverage>,
}

impl DefinitionSpec {
    /// Full-content canonical encoding, used by
    /// [`crate::manifest::ProfileManifest::manifest_content_hash`]. This
    /// covers every field, including ones that are not part of immutable
    /// identity (description, enabled, version label), so that *any*
    /// content change — not only an identity change — produces a
    /// different manifest hash.
    pub(crate) fn canonicalize_full(&self, enc: &mut CanonicalEncoder) {
        self.profile_id.canonicalize(enc);
        self.id.canonicalize(enc);
        self.kind.kind_tag().canonicalize(enc);
        enc.push_bool(self.domain.is_some());
        enc.push_str(self.domain.as_ref().map(|t| t.as_str()).unwrap_or(""));
        enc.push_bool(self.layer.is_some());
        enc.push_str(self.layer.as_ref().map(|t| t.as_str()).unwrap_or(""));
        self.value_constraint.canonicalize(enc);
        self.authority.canonicalize(enc);
        // `valid_scopes` is a BTreeSet, so this iteration is already in a
        // stable total order.
        enc.push_u64(self.valid_scopes.len() as u64);
        for scope in &self.valid_scopes {
            enc.push_str(&scope.canonical_tag());
        }
        enc.push_bool(self.enabled);
        enc.push_u32(self.version);
        enc.push_str(self.description.as_str());
        enc.push_bool(self.behavioral_leverage.is_some());
        enc.push_str(self.behavioral_leverage.map(|l| l.tag()).unwrap_or(""));
    }

    /// The untrusted declaration this spec offers to the canonical
    /// identity registry.
    ///
    /// Note what this does **not** do: it does not produce activated
    /// authority, and it does not assert a fingerprint. Only
    /// [`spark_core::activation::DefinitionIdentityRegistry::activate`]
    /// can turn a declaration into an
    /// [`spark_core::activation::ActivatedDefinition`], and it computes the
    /// fingerprint itself.
    pub fn to_declaration(&self) -> DefinitionDeclaration {
        DefinitionDeclaration {
            profile_id: self.profile_id.clone(),
            definition_id: self.id.clone(),
            kind: self.kind.kind_tag(),
            authority: self.authority,
            value_constraint: self.value_constraint.clone(),
            valid_scopes: self.valid_scopes.clone(),
        }
    }

    /// The immutable-identity fingerprint of this definition
    /// (ADR-0004 "Content identity").
    ///
    /// This delegates to the canonical kernel so there is exactly one
    /// definition of what a definition's identity is: the profile layer
    /// cannot compute a fingerprint that disagrees with the one the
    /// registry will enforce.
    pub fn definition_fingerprint(&self) -> Digest {
        spark_core::activation::definition_fingerprint(&self.to_declaration())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spark_core::value::FixedPoint;

    fn base(kind: DefinitionKind) -> DefinitionSpec {
        DefinitionSpec {
            profile_id: ProfileId::new("game-world").unwrap(),
            id: DefinitionId::new("some.definition").unwrap(),
            kind,
            domain: None,
            layer: None,
            value_constraint: ValueConstraint::fixed(
                FixedPoint::ZERO,
                FixedPoint::from_integer(1).unwrap(),
            )
            .unwrap(),
            authority: Authority::SparkOwned,
            valid_scopes: BTreeSet::from([ScopeKind::Actor]),
            enabled: true,
            version: 1,
            description: BoundedText::new("test").unwrap(),
            behavioral_leverage: None,
        }
    }

    /// B-04 regression: a built-in `Trigger` kind must never fingerprint
    /// identically to `Custom("trigger")`, even though both read as
    /// "trigger".
    #[test]
    fn builtin_trigger_fingerprint_differs_from_custom_trigger() {
        let builtin = base(DefinitionKind::Trigger);
        let custom = base(DefinitionKind::custom("trigger").unwrap());
        assert_ne!(
            builtin.definition_fingerprint(),
            custom.definition_fingerprint()
        );
    }

    /// Every built-in variant must map to a distinct, non-custom,
    /// valid canonical tag, so no two baseline kinds can alias and no
    /// baseline kind can be mistaken for profile vocabulary.
    #[test]
    fn builtin_kind_tags_are_distinct_valid_and_non_custom() {
        let variants = [
            DefinitionKind::Trigger,
            DefinitionKind::StateDefinition,
            DefinitionKind::Trait,
            DefinitionKind::Appraisal,
            DefinitionKind::Goal,
            DefinitionKind::Behavior,
            DefinitionKind::Relationship,
        ];
        let mut seen = BTreeSet::new();
        for variant in &variants {
            let kind_tag = variant.kind_tag();
            assert!(
                !kind_tag.is_custom(),
                "{variant:?} is not profile vocabulary"
            );
            assert!(CanonicalTag::new(kind_tag.tag().as_str()).is_ok());
            assert!(
                seen.insert(kind_tag.tag().as_str().to_string()),
                "{variant:?} reuses another baseline kind tag"
            );
        }
        assert_eq!(seen.len(), variants.len());
    }

    /// Re-foundation bounds requirement 3: an oversized custom kind is
    /// rejected at construction, so it can never become identity.
    #[test]
    fn oversized_custom_definition_kind_is_rejected_at_construction() {
        assert!(DefinitionKind::custom("k".repeat(100_000)).is_err());
        assert!(DefinitionKind::custom("weather_system").is_ok());
        assert!(DefinitionKind::custom("Weather System").is_err());
    }

    /// The profile layer's fingerprint must agree with the kernel's, since
    /// the kernel's is the one actually enforced.
    #[test]
    fn profile_fingerprint_agrees_with_the_kernel_registry() {
        let spec = base(DefinitionKind::Trait);
        assert_eq!(
            spec.definition_fingerprint(),
            spark_core::activation::definition_fingerprint(&spec.to_declaration())
        );
    }
}
