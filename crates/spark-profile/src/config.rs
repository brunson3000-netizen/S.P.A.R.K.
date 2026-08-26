//! `ConfigRevision`: a validated, profile-qualified, content-addressed
//! logical configuration revision.
//!
//! A manifest hash and a config-revision hash answer different questions —
//! "did the structural profile change?" versus "did a hot-tuned parameter
//! change?" (`CONTROLLING_BLUEPRINT_v0.2.md` §12.4) — so they are kept
//! distinct. This module supplies the logical representation and content
//! hash that Phase 3's config-mutation API will reuse; it does not
//! implement that API.
//!
//! # The re-foundation: a config revision is a validated object
//!
//! The previous implementation was a freely-constructible struct with a
//! public `Vec<ConfigEntry>`. Independent review found that it admitted
//! two different values under the same key, and that because equal-key
//! order was preserved by the sort, reversing those two entries changed
//! `config_revision_hash`. Ambiguous content was therefore neither
//! rejected nor even order-independent: the same logical input hashed
//! differently depending on arrival order, which is exactly the
//! arrival-order authority the determinism constitution forbids.
//!
//! [`ConfigRevision::build`] is now the only constructor. It validates
//! before anything is hashable:
//!
//! 1. duplicate keys are rejected outright — there is no last-writer-wins
//!    and no order-sensitive tie-break;
//! 2. every value is already a bounded canonical value
//!    ([`spark_core::value::CanonicalValue`] cannot hold an unbounded
//!    categorical string), and the revision label is [`BoundedText`];
//! 3. the accepted entries are stored in a `BTreeMap`, so a unique-key
//!    representation is the *only* representation — insertion order is
//!    not merely ignored, it is not retained.
//!
//! Duplicate detection is itself order-independent: the reported duplicate
//! keys are sorted, so the same malformed input is rejected identically
//! regardless of the order it arrived in.

use crate::text::BoundedText;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{DefinitionId, ProfileId};
use spark_core::value::CanonicalValue;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// One hot-tunable configuration key/value pair. `key` reuses
/// [`DefinitionId`]'s namespaced-identifier syntax (e.g.
/// `trigger.weather.drought.base_rate`) since config keys are themselves
/// stable, namespaced identifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigEntry {
    pub key: DefinitionId,
    pub value: CanonicalValue,
}

/// Rejects a malformed configuration revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigRevisionError {
    /// One or more keys appeared more than once. A config that says two
    /// things about the same parameter is ambiguous, and no arrival order
    /// may resolve it. The reported keys are sorted, so the same input is
    /// rejected identically in any order.
    DuplicateKeys { keys: Vec<DefinitionId> },
}

impl fmt::Display for ConfigRevisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigRevisionError::DuplicateKeys { keys } => {
                let rendered: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
                write!(
                    f,
                    "configuration revision declares duplicate keys: {}",
                    rendered.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for ConfigRevisionError {}

/// A validated, profile-qualified, content-addressed logical configuration
/// revision.
///
/// Fields are private and [`ConfigRevision::build`] is the only
/// constructor, so an ambiguous or unbounded revision does not exist:
///
/// ```compile_fail
/// use spark_profile::config::ConfigRevision;
/// use spark_core::id::ProfileId;
/// let forged = ConfigRevision {
///     profile_id: ProfileId::new("game-world").unwrap(),
///     entries: Vec::new(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigRevision {
    profile_id: ProfileId,
    /// Human-readable revision label. Never behavioral identity; see
    /// [`ConfigRevision::config_revision_hash`].
    revision_label: BoundedText,
    /// Unique-key representation. Insertion order is not retained.
    entries: BTreeMap<DefinitionId, CanonicalValue>,
}

impl ConfigRevision {
    /// Validates and canonicalizes a configuration revision.
    ///
    /// Rejects duplicate keys regardless of the order they appear in. On
    /// success the entries are stored uniquely and in key order, so the
    /// resulting revision has exactly one representation for a given
    /// logical content.
    pub fn build(
        profile_id: ProfileId,
        revision_label: BoundedText,
        entries: impl IntoIterator<Item = ConfigEntry>,
    ) -> Result<Self, ConfigRevisionError> {
        let mut accepted: BTreeMap<DefinitionId, CanonicalValue> = BTreeMap::new();
        let mut duplicates: BTreeSet<DefinitionId> = BTreeSet::new();

        for entry in entries {
            if accepted.insert(entry.key.clone(), entry.value).is_some() {
                duplicates.insert(entry.key);
            }
        }

        if !duplicates.is_empty() {
            return Err(ConfigRevisionError::DuplicateKeys {
                keys: duplicates.into_iter().collect(),
            });
        }

        Ok(Self {
            profile_id,
            revision_label,
            entries: accepted,
        })
    }

    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    pub fn revision_label(&self) -> &BoundedText {
        &self.revision_label
    }

    pub fn get(&self, key: &DefinitionId) -> Option<&CanonicalValue> {
        self.entries.get(key)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Every entry, in stable ascending key order.
    pub fn entries(&self) -> impl Iterator<Item = (&DefinitionId, &CanonicalValue)> {
        self.entries.iter()
    }

    /// Deterministic content hash.
    ///
    /// Entries are already stored uniquely in key order, so insertion
    /// order cannot affect the result. `revision_label` is deliberately
    /// excluded, so a reused human label with different content still
    /// changes the hash; `profile_id` is included, so the same parameters
    /// in two trust domains are different configuration identities.
    pub fn config_revision_hash(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("config_revision_content");
        self.profile_id.canonicalize(&mut enc);
        enc.push_u64(self.entries.len() as u64);
        for (key, value) in &self.entries {
            let mut inner = CanonicalEncoder::new();
            key.canonicalize(&mut inner);
            value.canonicalize(&mut inner);
            enc.push_block(&inner);
        }
        enc.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spark_core::value::{CategoricalValue, FixedPoint, MAX_CATEGORICAL_VALUE_LEN};

    fn profile() -> ProfileId {
        ProfileId::new("game-world").unwrap()
    }

    fn label(s: &str) -> BoundedText {
        BoundedText::new(s).unwrap()
    }

    fn entry(key: &str, value: CanonicalValue) -> ConfigEntry {
        ConfigEntry {
            key: DefinitionId::new(key).unwrap(),
            value,
        }
    }

    fn revision(entries: Vec<ConfigEntry>) -> ConfigRevision {
        ConfigRevision::build(profile(), label("1.0.0"), entries).unwrap()
    }

    /// Re-foundation config test 1: duplicate keys reject.
    #[test]
    fn duplicate_keys_are_rejected() {
        let err = ConfigRevision::build(
            profile(),
            label("1.0.0"),
            vec![
                entry("trigger.a.base_rate", CanonicalValue::Int(1)),
                entry("trigger.a.base_rate", CanonicalValue::Int(2)),
            ],
        )
        .unwrap_err();
        assert_eq!(
            err,
            ConfigRevisionError::DuplicateKeys {
                keys: vec![DefinitionId::new("trigger.a.base_rate").unwrap()]
            }
        );
    }

    /// Re-foundation config test 4: the same duplicate-key input is
    /// rejected identically regardless of input order.
    #[test]
    fn duplicate_keys_reject_identically_in_either_order() {
        let forward = ConfigRevision::build(
            profile(),
            label("1.0.0"),
            vec![
                entry("trigger.a.base_rate", CanonicalValue::Int(1)),
                entry("trigger.a.base_rate", CanonicalValue::Int(2)),
            ],
        )
        .unwrap_err();
        let reversed = ConfigRevision::build(
            profile(),
            label("1.0.0"),
            vec![
                entry("trigger.a.base_rate", CanonicalValue::Int(2)),
                entry("trigger.a.base_rate", CanonicalValue::Int(1)),
            ],
        )
        .unwrap_err();
        assert_eq!(forward, reversed);
    }

    /// Re-foundation config test 2: an oversized/malformed value is
    /// rejected before the revision is constructible at all — the
    /// canonical value type itself cannot hold one.
    #[test]
    fn oversized_config_value_cannot_be_constructed() {
        assert!(CategoricalValue::new("v".repeat(MAX_CATEGORICAL_VALUE_LEN + 1)).is_err());
        assert!(CanonicalValue::categorical("v".repeat(MAX_CATEGORICAL_VALUE_LEN + 1)).is_err());
        // Bounded content is accepted normally.
        let ok = revision(vec![entry(
            "trigger.a.label",
            CanonicalValue::categorical("drought").unwrap(),
        )]);
        assert_eq!(ok.len(), 1);
    }

    /// Re-foundation config test 3: insertion order of unique logical
    /// entries does not affect the hash.
    #[test]
    fn entry_order_does_not_affect_hash() {
        let forward = revision(vec![
            entry("trigger.a", CanonicalValue::Bool(true)),
            entry("trigger.b", CanonicalValue::Bool(false)),
        ]);
        let reversed = revision(vec![
            entry("trigger.b", CanonicalValue::Bool(false)),
            entry("trigger.a", CanonicalValue::Bool(true)),
        ]);
        assert_eq!(
            forward.config_revision_hash(),
            reversed.config_revision_hash()
        );
        assert_eq!(forward, reversed);
    }

    /// Re-foundation config test 5: profile context changes the hash.
    #[test]
    fn different_profile_changes_hash() {
        let a = revision(vec![entry("trigger.a", CanonicalValue::Bool(true))]);
        let b = ConfigRevision::build(
            ProfileId::new("mci-social").unwrap(),
            label("1.0.0"),
            vec![entry("trigger.a", CanonicalValue::Bool(true))],
        )
        .unwrap();
        assert_ne!(a.config_revision_hash(), b.config_revision_hash());
    }

    /// Re-foundation config test 6: the human label is not identity.
    #[test]
    fn reused_label_with_different_content_hashes_differently() {
        let a = revision(vec![entry("trigger.a", CanonicalValue::Bool(true))]);
        let b = revision(vec![entry("trigger.a", CanonicalValue::Bool(false))]);
        assert_eq!(a.revision_label(), b.revision_label());
        assert_ne!(a.config_revision_hash(), b.config_revision_hash());

        // ...and a changed label with identical content does not change
        // identity.
        let relabelled = ConfigRevision::build(
            profile(),
            label("2.0.0-hotfix"),
            vec![entry("trigger.a", CanonicalValue::Bool(true))],
        )
        .unwrap();
        assert_eq!(a.config_revision_hash(), relabelled.config_revision_hash());
    }

    #[test]
    fn changed_content_changes_hash() {
        let a = revision(vec![entry(
            "trigger.weather.drought.base_rate",
            CanonicalValue::Fixed(FixedPoint::from_integer(1).unwrap()),
        )]);
        let b = revision(vec![entry(
            "trigger.weather.drought.base_rate",
            CanonicalValue::Fixed(FixedPoint::from_integer(2).unwrap()),
        )]);
        assert_ne!(a.config_revision_hash(), b.config_revision_hash());
    }
}
