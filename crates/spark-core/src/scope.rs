//! Scope identifiers.
//!
//! A [`StateCell`](crate::state::StateCell) is always attached to a scope:
//! an actor, a household, a settlement, a directed relationship endpoint
//! pair, and so on (`CONTROLLING_BLUEPRINT_v0.2.md` §12.2, §13.4). Phase 1
//! only needs a typed scope identity sufficient to key state and validate
//! `valid_scopes` compatibility; the exhaustive taxonomy of scope kinds is
//! profile vocabulary, not a hard-coded Rust enum, so `Custom` carries a
//! profile-defined tag for kinds beyond this provisional baseline.

use crate::hash::CanonicalEncoder;
use crate::id::{StableId, StableIdError};
use std::fmt;

/// Provisional baseline scope-kind taxonomy
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §13.4). Additional kinds are
/// expressible without a Rust change via `Custom`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    Custom,
}

impl ScopeKind {
    pub fn tag(&self) -> &'static str {
        match self {
            ScopeKind::Actor => "actor",
            ScopeKind::Household => "household",
            ScopeKind::Workplace => "workplace",
            ScopeKind::Settlement => "settlement",
            ScopeKind::Watershed => "watershed",
            ScopeKind::Region => "region",
            ScopeKind::Faction => "faction",
            ScopeKind::Religion => "religion",
            ScopeKind::Denomination => "denomination",
            ScopeKind::Kingdom => "kingdom",
            ScopeKind::World => "world",
            ScopeKind::Custom => "custom",
        }
    }
}

impl fmt::Display for ScopeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag())
    }
}

/// A typed scope identity: a [`ScopeKind`] plus a namespaced instance ID
/// (e.g. `Settlement` + `settlement.pontafique`).
///
/// A directed relationship scope is represented as an ordinary `Actor`
/// scope whose ID encodes both endpoints (e.g.
/// `actor.bron->player.main`); Phase 1 does not need a dedicated pair
/// type, and inventing one before Phase 4's relationship store exists
/// would be scope creep.
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

    pub fn kind(&self) -> ScopeKind {
        self.kind
    }

    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str(self.kind.tag());
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
mod tests {
    use super::*;

    #[test]
    fn distinct_kinds_with_same_instance_id_are_distinct_scopes() {
        let a = ScopeId::new(ScopeKind::Actor, "bron").unwrap();
        let b = ScopeId::new(ScopeKind::Household, "bron").unwrap();
        assert_ne!(a, b);
    }
}
