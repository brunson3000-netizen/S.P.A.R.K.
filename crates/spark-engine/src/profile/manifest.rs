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

use crate::profile::definition::DefinitionSpec;
use crate::profile::text::BoundedText;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::ProfileId;

/// A versioned logical profile manifest
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1, ADR-0004).
///
/// Fields are private with accessors, so the human `version_label` cannot
/// be mistaken for a mutable identity handle: identity is
/// [`manifest_content_hash`](Self::manifest_content_hash). A manifest is
/// untrusted authoring input; a manifest that has *passed* the activation
/// ceremony is represented by
/// [`crate::activation::ActivatedProfile`], which carries this hash
/// forward as its exact-artifact binding.
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
    /// sorted before hashing so that construction order never affects the
    /// result — on **every** input, valid or not.
    ///
    /// The sort key is `(definition ID, full canonical encoding)`, not the
    /// ID alone. A manifest is untrusted authoring input, so it may carry
    /// two *different* definitions under one ID; sorting by ID alone left
    /// their relative order decided by construction order, and the hash
    /// with it (finding m-02). Such a manifest is invalid and can never
    /// pass the activation ceremony, so no activated artifact, save, or
    /// epoch has ever referenced an affected hash — but ADR-0004 requires
    /// the canonical logical representation used for content hashing to be
    /// deterministic, and a canonical hash function that is order-sensitive
    /// on any input is a trap for later tooling that hashes unvalidated
    /// manifests (authoring diffs, artifact stores). Adding the encoded
    /// block as the tie-break makes the hash a true multiset function.
    ///
    /// The ID stays the primary key so the hash of every manifest that can
    /// actually activate — IDs unique, hence no tie to break — is
    /// bit-for-bit what it was before this correction.
    pub fn manifest_content_hash(&self) -> Digest {
        let mut blocks: Vec<(&str, Vec<u8>)> = self
            .definitions
            .iter()
            .map(|d| {
                let mut inner = CanonicalEncoder::new();
                d.canonicalize_full(&mut inner);
                (d.id.as_str(), inner.into_bytes())
            })
            .collect();
        blocks.sort();

        let mut enc = CanonicalEncoder::new();
        enc.push_str("profile_manifest_content");
        self.profile_id.canonicalize(&mut enc);
        enc.push_u64(blocks.len() as u64);
        for (_, block) in &blocks {
            enc.push_bytes(block);
        }
        enc.finish()
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
    use crate::profile::definition::{BehavioralLeverage, DefinitionKind};
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
