//! Profile-qualified immutable definition identity registry
//! (Phase-1 correction brief B-04).
//!
//! The Phase-1 writer pass enforced only *authority* immutability, and
//! did so through a single global (not profile-qualified) catalog that
//! [`crate::validate::validate`] mutated while it walked a manifest —
//! so a later error in the same manifest left earlier, already-declared
//! definitions installed even though validation as a whole failed. This
//! module replaces that mechanism: identity is the definition's full
//! content [fingerprint](spark_core::hash::Digest) (kind, value
//! constraint, authority/write class, and valid scopes — not authority
//! alone), the registry is keyed by `(ProfileId, DefinitionId)` so the
//! same ID in two different profiles cannot contaminate each other's
//! history, and the only way to change it is
//! [`DefinitionIdentityRegistry::commit`], which [`crate::validate::validate`]
//! calls only after an entire manifest has passed every check —
//! validation is pure with respect to the registry until it is proven to
//! succeed.

use spark_core::hash::Digest;
use spark_core::id::{DefinitionId, ProfileId};
use std::collections::BTreeMap;

/// Rejects an attempt to change the immutable full identity fingerprint of
/// an already-declared `(profile_id, definition_id)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionIdentityConflict {
    pub profile_id: ProfileId,
    pub definition_id: DefinitionId,
    pub previous_fingerprint: Digest,
    pub attempted_fingerprint: Digest,
}

impl std::fmt::Display for DefinitionIdentityConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "definition '{}' in profile '{}' was declared with fingerprint {} and cannot be \
             redeclared with fingerprint {} under the same ID; an identity change (kind, value \
             constraint, authority, or valid scopes) requires a new definition ID, an explicit \
             migration, and an ADR",
            self.definition_id,
            self.profile_id,
            self.previous_fingerprint,
            self.attempted_fingerprint
        )
    }
}

impl std::error::Error for DefinitionIdentityConflict {}

/// The immutable, profile-qualified full-identity fingerprint of every
/// `(profile_id, definition_id)` pair ever successfully validated in this
/// process.
#[derive(Debug, Default, Clone)]
pub struct DefinitionIdentityRegistry {
    fingerprints: BTreeMap<(ProfileId, DefinitionId), Digest>,
}

impl DefinitionIdentityRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fingerprint_of(
        &self,
        profile_id: &ProfileId,
        definition_id: &DefinitionId,
    ) -> Option<&Digest> {
        self.fingerprints
            .get(&(profile_id.clone(), definition_id.clone()))
    }

    /// Commits a batch of `(profile_id, definition_id) -> fingerprint`
    /// entries atomically. Only [`crate::validate::validate`] should call
    /// this, and only after every definition in a manifest has passed
    /// every check (including identity-conflict checks against the
    /// pre-commit state of this registry) — this method itself performs
    /// no validation, so callers must not use it to bypass
    /// [`crate::validate::validate`].
    pub(crate) fn commit(&mut self, entries: Vec<((ProfileId, DefinitionId), Digest)>) {
        for (key, fingerprint) in entries {
            self.fingerprints.insert(key, fingerprint);
        }
    }
}
