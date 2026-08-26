//! `ConfigRevision`: a minimal, profile-qualified, content-addressed
//! logical configuration revision (Phase-1 correction brief M-01).
//!
//! The Phase-1 writer pass implemented `ProfileManifest::manifest_content_hash`
//! but omitted the separately required hot-tunable *config* identity
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §12.4 "hot-tunable parameter changes"
//! and the implementation brief's requirement #13). A manifest hash and a
//! config-revision hash answer different questions — "did the structural
//! profile change?" versus "did a hot-tuned parameter change?" — and
//! reusing one for the other would conflate two different change classes
//! that the blueprint deliberately keeps distinct. This module does not
//! implement Phase-3's config-mutation API; it only supplies the logical
//! representation and content hash that API will need to reuse.

use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{DefinitionId, ProfileId};
use spark_core::value::CanonicalValue;

/// One hot-tunable configuration key/value pair. `key` reuses
/// `DefinitionId`'s namespaced-identifier syntax (e.g.
/// `trigger.weather.drought.base_rate`) since config keys are themselves
/// stable, namespaced identifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigEntry {
    pub key: DefinitionId,
    pub value: CanonicalValue,
}

/// A profile-qualified, content-addressed logical configuration revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigRevision {
    pub profile_id: ProfileId,
    /// Human-readable revision label. Never used as behavioral identity;
    /// see [`ConfigRevision::config_revision_hash`], mirroring
    /// `ProfileManifest::version_label`'s treatment.
    pub revision_label: String,
    pub entries: Vec<ConfigEntry>,
}

impl ConfigRevision {
    /// Deterministic, order-independent content hash. Entries are sorted
    /// by key before hashing so construction/insertion order never
    /// affects the result, and `revision_label` is deliberately excluded
    /// so a reused human label with different content still changes the
    /// hash.
    pub fn config_revision_hash(&self) -> Digest {
        let mut entries: Vec<&ConfigEntry> = self.entries.iter().collect();
        entries.sort_by(|a, b| a.key.as_str().cmp(b.key.as_str()));

        let mut enc = CanonicalEncoder::new();
        enc.push_str("config_revision_content");
        self.profile_id.canonicalize(&mut enc);
        enc.push_u64(entries.len() as u64);
        for entry in entries {
            let mut inner = CanonicalEncoder::new();
            entry.key.canonicalize(&mut inner);
            entry.value.canonicalize(&mut inner);
            enc.push_block(&inner);
        }
        enc.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spark_core::value::FixedPoint;

    fn revision(label: &str, entries: Vec<ConfigEntry>) -> ConfigRevision {
        ConfigRevision {
            profile_id: ProfileId::new("game-world").unwrap(),
            revision_label: label.to_string(),
            entries,
        }
    }

    fn entry(key: &str, value: CanonicalValue) -> ConfigEntry {
        ConfigEntry {
            key: DefinitionId::new(key).unwrap(),
            value,
        }
    }

    #[test]
    fn changed_content_changes_hash() {
        let a = revision(
            "1.0.0",
            vec![entry(
                "trigger.weather.drought.base_rate",
                CanonicalValue::Fixed(FixedPoint::from_integer(1).unwrap()),
            )],
        );
        let b = revision(
            "1.0.0",
            vec![entry(
                "trigger.weather.drought.base_rate",
                CanonicalValue::Fixed(FixedPoint::from_integer(2).unwrap()),
            )],
        );
        assert_ne!(a.config_revision_hash(), b.config_revision_hash());
    }

    #[test]
    fn reused_label_with_different_content_hashes_differently() {
        let a = revision("1.0.0", vec![entry("k", CanonicalValue::Bool(true))]);
        let b = revision("1.0.0", vec![entry("k", CanonicalValue::Bool(false))]);
        assert_eq!(a.revision_label, b.revision_label);
        assert_ne!(a.config_revision_hash(), b.config_revision_hash());
    }

    #[test]
    fn entry_order_does_not_affect_hash() {
        let forward = revision(
            "1.0.0",
            vec![
                entry("a.key", CanonicalValue::Bool(true)),
                entry("b.key", CanonicalValue::Bool(false)),
            ],
        );
        let reversed = revision(
            "1.0.0",
            vec![
                entry("b.key", CanonicalValue::Bool(false)),
                entry("a.key", CanonicalValue::Bool(true)),
            ],
        );
        assert_eq!(
            forward.config_revision_hash(),
            reversed.config_revision_hash()
        );
    }

    #[test]
    fn different_profile_changes_hash() {
        let mut a = revision("1.0.0", vec![entry("k", CanonicalValue::Bool(true))]);
        let mut b = a.clone();
        a.profile_id = ProfileId::new("game-world").unwrap();
        b.profile_id = ProfileId::new("mci-social").unwrap();
        assert_ne!(a.config_revision_hash(), b.config_revision_hash());
    }
}
