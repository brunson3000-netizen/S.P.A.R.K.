//! `ProfileManifest` and its canonical content hash.
//!
//! ADR-0004: "The exact on-disk syntax remains provisional. The canonical
//! logical representation used for validation/content hashing must be
//! deterministic and format-independent," and "A version/display label may
//! aid humans but is never sufficient by itself to identify behavior."
//! Accordingly [`ProfileManifest::manifest_content_hash`] hashes the
//! profile ID and every definition's full content, sorted by definition ID
//! so construction order can never matter, and deliberately does **not**
//! include `version_label` — a manifest that reuses a human version string
//! but changes real content always produces a different hash.

use crate::definition::DefinitionSpec;
use crate::text::BoundedText;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::ProfileId;

/// A versioned logical profile manifest
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1, ADR-0004).
///
/// Fields are private with accessors, so the human `version_label` cannot
/// be mistaken for a mutable identity handle: identity is
/// [`manifest_content_hash`](Self::manifest_content_hash), and a manifest
/// that has been validated is represented by
/// [`crate::validate::ValidatedManifest`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileManifest {
    profile_id: ProfileId,
    version_label: BoundedText,
    definitions: Vec<DefinitionSpec>,
}

impl ProfileManifest {
    pub fn new(
        profile_id: ProfileId,
        version_label: BoundedText,
        definitions: Vec<DefinitionSpec>,
    ) -> Self {
        Self {
            profile_id,
            version_label,
            definitions,
        }
    }

    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    /// Human-readable version label. Never behavioral identity; see
    /// [`ProfileManifest::manifest_content_hash`].
    pub fn version_label(&self) -> &BoundedText {
        &self.version_label
    }

    pub fn definitions(&self) -> &[DefinitionSpec] {
        &self.definitions
    }

    /// Deterministic, format-independent content hash. Definitions are
    /// sorted by ID before hashing so that construction order never
    /// affects the result.
    pub fn manifest_content_hash(&self) -> Digest {
        let mut defs: Vec<&DefinitionSpec> = self.definitions.iter().collect();
        defs.sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));

        let mut enc = CanonicalEncoder::new();
        enc.push_str("profile_manifest_content");
        self.profile_id.canonicalize(&mut enc);
        enc.push_u64(defs.len() as u64);
        for d in defs {
            let mut inner = CanonicalEncoder::new();
            d.canonicalize_full(&mut inner);
            enc.push_block(&inner);
        }
        enc.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::{BehavioralLeverage, DefinitionKind};
    use spark_core::authority::Authority;
    use spark_core::id::{CanonicalTag, DefinitionId};
    use spark_core::scope::ScopeKind;
    use spark_core::value::{FixedPoint, ValueConstraint};
    use std::collections::BTreeSet;

    fn base_definition(description: &str) -> DefinitionSpec {
        DefinitionSpec {
            profile_id: ProfileId::new("game-world").unwrap(),
            id: DefinitionId::new("trait.curiosity").unwrap(),
            kind: DefinitionKind::Trait,
            domain: None,
            layer: Some(CanonicalTag::new("actor_attribute").unwrap()),
            value_constraint: ValueConstraint::fixed(
                FixedPoint::ZERO,
                FixedPoint::from_integer(1).unwrap(),
            )
            .unwrap(),
            authority: Authority::SparkOwned,
            valid_scopes: BTreeSet::from([ScopeKind::Actor]),
            enabled: true,
            version: 1,
            description: BoundedText::new(description).unwrap(),
            behavioral_leverage: Some(BehavioralLeverage::High),
        }
    }

    fn manifest(version_label: &str, definitions: Vec<DefinitionSpec>) -> ProfileManifest {
        ProfileManifest::new(
            ProfileId::new("game-world").unwrap(),
            BoundedText::new(version_label).unwrap(),
            definitions,
        )
    }

    /// Phase-1 test corpus item 14: a manifest version label reused with
    /// different content produces a different content hash.
    #[test]
    fn reused_version_label_with_different_content_hashes_differently() {
        let manifest_v1 = manifest(
            "1.0.0",
            vec![base_definition("Curiosity drives approach/investigation.")],
        );
        let manifest_v2 = manifest(
            "1.0.0",
            vec![base_definition(
                "Curiosity drives exploration and social contact.",
            )],
        );

        assert_eq!(manifest_v1.version_label(), manifest_v2.version_label());
        assert_ne!(
            manifest_v1.manifest_content_hash(),
            manifest_v2.manifest_content_hash()
        );
    }

    /// ...and the converse: a changed human label over identical content
    /// does not change identity.
    #[test]
    fn changed_version_label_over_identical_content_does_not_change_the_hash() {
        let a = manifest("1.0.0", vec![base_definition("Curiosity.")]);
        let b = manifest("2.0.0-rc1", vec![base_definition("Curiosity.")]);
        assert_ne!(a.version_label(), b.version_label());
        assert_eq!(a.manifest_content_hash(), b.manifest_content_hash());
    }

    #[test]
    fn identical_content_hashes_identically_regardless_of_definition_order() {
        let curiosity = base_definition("Curiosity trait.");
        let mut trust = base_definition("Trust relationship dimension.");
        trust.id = DefinitionId::new("relationship.trust").unwrap();
        trust.kind = DefinitionKind::Relationship;

        let forward = manifest("1.0.0", vec![curiosity.clone(), trust.clone()]);
        let reversed = manifest("1.0.0", vec![trust, curiosity]);

        assert_eq!(
            forward.manifest_content_hash(),
            reversed.manifest_content_hash()
        );
    }
}
