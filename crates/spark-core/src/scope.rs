//! Scope identifiers.
//!
//! A [`StateCell`](crate::state::StateCell) is always attached to a scope:
//! an actor, a household, a settlement, and so on
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §12.2, §13.4). Phase 1 only needs a
//! typed scope identity sufficient to key state and validate
//! `valid_scopes` compatibility; the exhaustive taxonomy of scope kinds is
//! profile vocabulary, not a hard-coded Rust enum, so `Custom` carries a
//! profile-defined, validated identifier for kinds beyond this
//! provisional baseline.
//!
//! Directed relationships (e.g. `actor.bron` trusts `player.main`) are a
//! Phase-4 relationship-store concern once that store exists
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §15.3); Phase 1 deliberately does not
//! invent a pair-scope type early. When it is added it must use
//! structured subject/target fields, not a single string such as
//! `"a->b"` — `StableId` rejects `->` as invalid syntax precisely to keep
//! that shortcut from being taken by accident (Phase-1 correction brief
//! M-03).

use crate::hash::CanonicalEncoder;
use crate::id::{StableId, StableIdError};
use std::fmt;

/// Provisional baseline scope-kind taxonomy
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §13.4). Additional kinds are
/// expressible without a Rust change via `Custom`, which carries its own
/// validated [`StableId`] rather than collapsing every custom kind to one
/// indistinguishable tag (Phase-1 correction brief M-03/B-04).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ScopeKind {
    Actor,
    Household,
    Workplace,
    Settlement,
    Watershed,
    Region,
    Faction,
    Religion,
    Denomination,
    Kingdom,
    World,
    Custom(StableId),
}

impl ScopeKind {
    /// A stable, domain-separated encoding used both for canonical
    /// hashing and for sorted-set ordering (e.g. a definition's
    /// `valid_scopes`). Built-in kinds encode as their literal tag;
    /// `Custom` encodes as `"custom:"` followed by its validated
    /// identifier, so a custom kind can never collide with a built-in tag
    /// (a built-in tag never starts with `"custom:"`) and two different
    /// custom kinds never collide with each other.
    pub fn canonical_tag(&self) -> String {
        match self {
            ScopeKind::Actor => "actor".to_string(),
            ScopeKind::Household => "household".to_string(),
            ScopeKind::Workplace => "workplace".to_string(),
            ScopeKind::Settlement => "settlement".to_string(),
            ScopeKind::Watershed => "watershed".to_string(),
            ScopeKind::Region => "region".to_string(),
            ScopeKind::Faction => "faction".to_string(),
            ScopeKind::Religion => "religion".to_string(),
            ScopeKind::Denomination => "denomination".to_string(),
            ScopeKind::Kingdom => "kingdom".to_string(),
            ScopeKind::World => "world".to_string(),
            ScopeKind::Custom(id) => format!("custom:{}", id.as_str()),
        }
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str(&self.canonical_tag());
    }
}

impl fmt::Display for ScopeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.canonical_tag())
    }
}

/// A typed scope identity: a [`ScopeKind`] plus a namespaced instance ID
/// (e.g. `Settlement` + `settlement.pontafique`).
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopeId {
    kind: ScopeKind,
    id: StableId,
}

impl ScopeId {
    pub fn new(kind: ScopeKind, id: impl Into<String>) -> Result<Self, StableIdError> {
        Ok(Self {
            kind,
            id: StableId::new(id)?,
        })
    }

    pub fn kind(&self) -> &ScopeKind {
        &self.kind
    }

    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.kind.canonicalize(enc);
        enc.push_str(self.id.as_str());
    }
}

impl fmt::Debug for ScopeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.kind, self.id)
    }
}

impl fmt::Display for ScopeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.kind, self.id)
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

    #[test]
    fn distinct_kinds_with_same_instance_id_are_distinct_scopes() {
        let a = ScopeId::new(ScopeKind::Actor, "bron").unwrap();
        let b = ScopeId::new(ScopeKind::Household, "bron").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn custom_scope_kind_carries_its_real_identifier_and_does_not_collide() {
        let a = ScopeKind::Custom(StableId::new("sensor_grid").unwrap());
        let b = ScopeKind::Custom(StableId::new("logistics_hub").unwrap());
        assert_ne!(a.canonical_tag(), b.canonical_tag());
        assert_ne!(a, b);
    }

    #[test]
    fn built_in_kind_never_collides_with_custom_domain_separated_tag() {
        let builtin = ScopeKind::Actor;
        let custom = ScopeKind::Custom(StableId::new("actor").unwrap());
        assert_ne!(builtin.canonical_tag(), custom.canonical_tag());
        assert_ne!(builtin, custom);
    }
}
