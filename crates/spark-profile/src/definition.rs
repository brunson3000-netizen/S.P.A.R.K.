//! `DefinitionSpec`: the logical shape of one profile definition
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1). The exact on-disk syntax is
//! provisional (ADR-0004); this struct is the deterministic,
//! format-independent logical representation used for validation and
//! content hashing.

use spark_core::authority::Authority;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{DefinitionId, ProfileId};
use spark_core::scope::ScopeKind;
use spark_core::state::DefinitionSchema;
use spark_core::value::ValueConstraint;
use std::collections::BTreeSet;

/// `kind` in the common definition schema (`CONTROLLING_BLUEPRINT_v0.2.md`
/// §12.1: "trigger/state/trait/appraisal/goal/behavior/etc"). The
/// baseline variants mirror the frozen causal grammar's primitive roles;
/// `Custom` keeps the taxonomy open as profile vocabulary per ADR-0004's
/// taxonomy rule, without requiring a Rust change to add a new kind tag.
///
/// Canonical encoding is domain-separated (Phase-1 correction brief
/// B-04): a built-in variant encodes as its literal tag, while `Custom`
/// always encodes with a `"custom:"` prefix, so a built-in `Trigger` can
/// never collide with `Custom("trigger")` even though both display the
/// same human-readable word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionKind {
    Trigger,
    StateDefinition,
    Trait,
    Appraisal,
    Goal,
    Behavior,
    Relationship,
    Custom(String),
}

impl DefinitionKind {
    fn canonical_tag(&self) -> String {
        match self {
            DefinitionKind::Trigger => "trigger".to_string(),
            DefinitionKind::StateDefinition => "state_definition".to_string(),
            DefinitionKind::Trait => "trait".to_string(),
            DefinitionKind::Appraisal => "appraisal".to_string(),
            DefinitionKind::Goal => "goal".to_string(),
            DefinitionKind::Behavior => "behavior".to_string(),
            DefinitionKind::Relationship => "relationship".to_string(),
            DefinitionKind::Custom(tag) => format!("custom:{tag}"),
        }
    }

    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str(&self.canonical_tag());
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
    fn tag(&self) -> &'static str {
        match self {
            BehavioralLeverage::High => "high",
            BehavioralLeverage::Medium => "medium",
            BehavioralLeverage::Low => "low",
        }
    }
}

/// The logical, format-independent shape of one profile definition
/// (common fields per `CONTROLLING_BLUEPRINT_v0.2.md` §12.1). `profile_id`
/// is carried explicitly so identity, fingerprinting, and the resulting
/// `StateStore` schema are all profile-qualified end to end (Phase-1
/// correction brief B-02/B-04).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionSpec {
    pub profile_id: ProfileId,
    pub id: DefinitionId,
    pub kind: DefinitionKind,
    pub domain: Option<String>,
    pub layer: Option<String>,
    pub value_constraint: ValueConstraint,
    pub authority: Authority,
    pub valid_scopes: BTreeSet<ScopeKind>,
    pub enabled: bool,
    pub version: u32,
    pub description: String,
    pub behavioral_leverage: Option<BehavioralLeverage>,
}

impl DefinitionSpec {
    /// Full-content canonical encoding, used by
    /// [`crate::manifest::ProfileManifest::manifest_content_hash`]. This
    /// covers every field, including ones that are not part of immutable
    /// identity (description, enabled, version label), so that *any*
    /// content change - not only an identity change - produces a
    /// different manifest hash.
    pub(crate) fn canonicalize_full(&self, enc: &mut CanonicalEncoder) {
        self.profile_id.canonicalize(enc);
        self.id.canonicalize(enc);
        self.kind.canonicalize(enc);
        enc.push_bool(self.domain.is_some());
        enc.push_str(self.domain.as_deref().unwrap_or(""));
        enc.push_bool(self.layer.is_some());
        enc.push_str(self.layer.as_deref().unwrap_or(""));
        self.value_constraint.canonicalize(enc);
        self.authority.canonicalize(enc);
        let mut scopes: Vec<String> = self
            .valid_scopes
            .iter()
            .map(|s| s.canonical_tag())
            .collect();
        scopes.sort_unstable();
        enc.push_u64(scopes.len() as u64);
        for s in &scopes {
            enc.push_str(s);
        }
        enc.push_bool(self.enabled);
        enc.push_u32(self.version);
        enc.push_str(&self.description);
        enc.push_bool(self.behavioral_leverage.is_some());
        enc.push_str(self.behavioral_leverage.map(|l| l.tag()).unwrap_or(""));
    }

    /// The immutable-identity subset of this definition's content, hashed
    /// independently of mutable/descriptive fields (ADR-0004 "Content
    /// identity"): profile ID, definition ID, kind, value constraint
    /// (which implies value type), authority mode, implied write class,
    /// and sorted valid scopes. Two definitions with the same fingerprint
    /// are the same identity for save-continuation purposes even if their
    /// description or enabled/disabled status later changes; two
    /// definitions that differ in *any* of these fields are different
    /// identities even under the same `DefinitionId` (Phase-1 correction
    /// brief B-04).
    pub fn definition_fingerprint(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("definition_fingerprint");
        self.profile_id.canonicalize(&mut enc);
        self.id.canonicalize(&mut enc);
        self.kind.canonicalize(&mut enc);
        self.value_constraint.canonicalize(&mut enc);
        self.authority.canonicalize(&mut enc);
        enc.push_str(self.authority.implied_write_class().tag());
        let mut scopes: Vec<String> = self
            .valid_scopes
            .iter()
            .map(|s| s.canonical_tag())
            .collect();
        scopes.sort_unstable();
        enc.push_u64(scopes.len() as u64);
        for s in &scopes {
            enc.push_str(s);
        }
        enc.finish()
    }

    /// Converts this validated definition into the immutable
    /// [`DefinitionSchema`] entry `spark_core::state::StateStore` is built
    /// from. Only meaningful to call after this definition has passed
    /// [`crate::identity::validate`].
    pub fn to_definition_schema(&self) -> DefinitionSchema {
        DefinitionSchema {
            profile_id: self.profile_id.clone(),
            definition_id: self.id.clone(),
            fingerprint: self.definition_fingerprint(),
            authority: self.authority,
            value_constraint: self.value_constraint.clone(),
            valid_scopes: self.valid_scopes.clone(),
        }
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
            value_constraint: ValueConstraint::Fixed {
                min: FixedPoint::ZERO,
                max: FixedPoint::from_integer(1).unwrap(),
            },
            authority: Authority::SparkOwned,
            valid_scopes: BTreeSet::from([ScopeKind::Actor]),
            enabled: true,
            version: 1,
            description: "test".to_string(),
            behavioral_leverage: None,
        }
    }

    /// B-04: a built-in `Trigger` kind must never fingerprint identically
    /// to `Custom("trigger")`, even though both display as "trigger".
    #[test]
    fn builtin_trigger_fingerprint_differs_from_custom_trigger() {
        let builtin = base(DefinitionKind::Trigger);
        let custom = base(DefinitionKind::Custom("trigger".to_string()));
        assert_ne!(
            builtin.definition_fingerprint(),
            custom.definition_fingerprint()
        );
    }
}
