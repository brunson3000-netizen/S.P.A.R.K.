//! `ProfileManifest` and its canonical content hash.
//!
//! ADR-0004: "The exact on-disk syntax remains provisional. The
//! canonical logical representation used for validation/content hashing
//! must be deterministic and format-independent," and "A version/display
//! label may aid humans but is never sufficient by itself to identify
//! behavior." Accordingly [`ProfileManifest::manifest_content_hash`]
//! hashes the profile ID and every definition's full content, sorted by
//! definition ID so hash-map/insertion order can never matter, and
//! deliberately does **not** include `version_label` - a manifest that
//! reuses a human version string but changes real content always
//! produces a different hash.

use crate::definition::DefinitionSpec;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::ProfileId;

/// A versioned logical profile manifest
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1, ADR-0004).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileManifest {
    pub profile_id: ProfileId,
    /// Human-readable version label. Never used as behavioral identity;
    /// see [`ProfileManifest::manifest_content_hash`].
    pub version_label: String,
    pub definitions: Vec<DefinitionSpec>,
}

impl ProfileManifest {
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
    use spark_core::id::DefinitionId;
    use spark_core::scope::ScopeKind;
    use spark_core::value::{FixedPoint, ValueConstraint};
    use std::collections::BTreeSet;

    fn base_definition(description: &str) -> DefinitionSpec {
        DefinitionSpec {
            profile_id: ProfileId::new("game-world").unwrap(),
            id: DefinitionId::new("trait.curiosity").unwrap(),
            kind: DefinitionKind::Trait,
            domain: None,
            layer: Some("actor_attribute".to_string()),
            value_constraint: ValueConstraint::Fixed {
                min: FixedPoint::ZERO,
                max: FixedPoint::from_integer(1).unwrap(),
            },
            authority: Authority::SparkOwned,
            valid_scopes: BTreeSet::from([ScopeKind::Actor]),
            enabled: true,
            version: 1,
            description: description.to_string(),
            behavioral_leverage: Some(BehavioralLeverage::High),
        }
    }

    /// Phase-1 test corpus item 14: a manifest version label reused with
    /// different content produces a different content hash.
    #[test]
    fn reused_version_label_with_different_content_hashes_differently() {
        let manifest_v1 = ProfileManifest {
            profile_id: ProfileId::new("game-world").unwrap(),
            version_label: "1.0.0".to_string(),
            definitions: vec![base_definition("Curiosity drives approach/investigation.")],
        };
        let manifest_v2 = ProfileManifest {
            profile_id: ProfileId::new("game-world").unwrap(),
            version_label: "1.0.0".to_string(),
            definitions: vec![base_definition(
                "Curiosity drives exploration and social contact.",
            )],
        };

        assert_eq!(manifest_v1.version_label, manifest_v2.version_label);
        assert_ne!(
            manifest_v1.manifest_content_hash(),
            manifest_v2.manifest_content_hash()
        );
    }

    #[test]
    fn identical_content_hashes_identically_regardless_of_definition_order() {
        let curiosity = base_definition("Curiosity trait.");
        let mut trust = base_definition("Trust relationship dimension.");
        trust.id = DefinitionId::new("relationship.trust").unwrap();
        trust.kind = DefinitionKind::Relationship;

        let forward = ProfileManifest {
            profile_id: ProfileId::new("game-world").unwrap(),
            version_label: "1.0.0".to_string(),
            definitions: vec![curiosity.clone(), trust.clone()],
        };
        let reversed = ProfileManifest {
            profile_id: ProfileId::new("game-world").unwrap(),
            version_label: "1.0.0".to_string(),
            definitions: vec![trust, curiosity],
        };

        assert_eq!(
            forward.manifest_content_hash(),
            reversed.manifest_content_hash()
        );
    }
}
