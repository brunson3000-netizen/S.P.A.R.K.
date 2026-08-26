//! `DefinitionSpec`: the logical shape of one profile definition
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1). The exact on-disk syntax is
//! provisional (ADR-0004); this struct is the deterministic,
//! format-independent logical representation used for validation and
//! content hashing.
//!
//! # Untrusted by design
//!
//! Every field here is public, and that is correct: authority facts
//! legitimately *originate* as operator-authored profile data, so the
//! authoring input type is the right place to carry them. What is gone is
//! the stepping stone. There is no `to_declaration`, no way to reach an
//! intermediate mint, and no way to assert the built-in/custom kind bit:
//! the only thing a caller can do with a spec is put it in a
//! [`crate::profile::manifest::ProfileManifest`] and offer that manifest
//! to [`crate::activation::ActivationRegistry::activate`], which runs the
//! complete ceremony or rejects the whole manifest.
//!
//! Every canonical string here is a bounded validated type: a custom
//! definition kind is a [`CanonicalTag`], `domain`/`layer` are canonical
//! tags, and `description` is [`BoundedText`]. An unbounded custom kind
//! used to be accepted and became immutable definition identity.

use crate::activation::DefinitionKindTag;
use crate::profile::text::BoundedText;
use spark_core::authority::Authority;
use spark_core::canonical_tag;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{CanonicalTag, DefinitionId, ProfileId, StableIdError};
use spark_core::scope::ScopeKind;
use spark_core::value::ValueConstraint;
use std::collections::BTreeSet;

/// `kind` in the common definition schema (`CONTROLLING_BLUEPRINT_v0.2.md`
/// §12.1: "trigger/state/trait/appraisal/goal/behavior/etc"). The
/// baseline variants mirror the frozen causal grammar's primitive roles;
/// `Custom` keeps the taxonomy open as profile vocabulary per ADR-0004's
/// taxonomy rule, without requiring a Rust change to add a new kind tag.
///
/// This enum is the **entire** public kind vocabulary. `Custom` carries a
/// bounded [`CanonicalTag`], so an oversized or malformed custom kind
/// cannot be constructed and therefore cannot become immutable definition
/// identity; and because
/// [`DefinitionKindTag`]'s constructors are crate-private, an external
/// caller cannot claim that a profile-invented tag is engine baseline
/// vocabulary. `Trigger` and `Custom("trigger")` remain different
/// identities even though they read the same.
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
    pub fn custom(tag: impl Into<String>) -> Result<Self, StableIdError> {
        Ok(DefinitionKind::Custom(CanonicalTag::new(tag)?))
    }

    /// The domain-separated representation activated identity sees.
    ///
    /// The baseline vocabulary is expressed with the
    /// [`spark_core::canonical_tag!`] macro, whose literals are validated
    /// by *const evaluation*: an invalid baseline tag is a compile error,
    /// and — unlike the panic-capable `from_static` this replaces — there
    /// is no runtime path through which an invalid literal could panic
    /// instead. This function is therefore total.
    ///
    /// Crate-private: minting a kind tag is part of the activation
    /// ceremony's identity projection, not a caller capability.
    pub(crate) fn kind_tag(&self) -> DefinitionKindTag {
        match self {
            DefinitionKind::Trigger => DefinitionKindTag::builtin(canonical_tag!("trigger")),
            DefinitionKind::StateDefinition => {
                DefinitionKindTag::builtin(canonical_tag!("state_definition"))
            }
            DefinitionKind::Trait => DefinitionKindTag::builtin(canonical_tag!("trait")),
            DefinitionKind::Appraisal => DefinitionKindTag::builtin(canonical_tag!("appraisal")),
            DefinitionKind::Goal => DefinitionKindTag::builtin(canonical_tag!("goal")),
            DefinitionKind::Behavior => DefinitionKindTag::builtin(canonical_tag!("behavior")),
            DefinitionKind::Relationship => {
                DefinitionKindTag::builtin(canonical_tag!("relationship"))
            }
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
/// resulting activated profile are all profile-qualified end to end.
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
    /// [`crate::profile::manifest::ProfileManifest::manifest_content_hash`].
    /// This covers every field, including ones that are not part of
    /// immutable identity (description, enabled, version label), so that
    /// *any* content change — not only an identity change — produces a
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

    /// The immutable-identity fingerprint this definition *would* be given
    /// (ADR-0004 "Content identity").
    ///
    /// This is a pure function; computing it grants nothing. It exists so
    /// a reviewer or a future authoring tool can predict an identity
    /// independently, and it delegates to
    /// [`crate::activation::definition_fingerprint`] so the profile layer
    /// can never compute a fingerprint that disagrees with the one the
    /// door enforces.
    pub fn definition_fingerprint(&self) -> Digest {
        crate::activation::definition_fingerprint(self)
    }
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

    /// Every built-in variant must map to a distinct, non-custom, valid
    /// canonical tag, so no two baseline kinds can alias and no baseline
    /// kind can be mistaken for profile vocabulary.
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

    /// An oversized or malformed custom kind is rejected at construction,
    /// so it can never become identity.
    #[test]
    fn oversized_custom_definition_kind_is_rejected_at_construction() {
        assert!(DefinitionKind::custom("k".repeat(100_000)).is_err());
        assert!(DefinitionKind::custom("weather_system").is_ok());
        assert!(DefinitionKind::custom("Weather System").is_err());
    }

    /// The spec's predicted fingerprint must agree with the door's, since
    /// the door's is the one actually enforced.
    #[test]
    fn spec_fingerprint_agrees_with_the_activation_door() {
        let spec = base(DefinitionKind::Trait);
        assert_eq!(
            spec.definition_fingerprint(),
            crate::activation::definition_fingerprint(&spec)
        );
    }
}
