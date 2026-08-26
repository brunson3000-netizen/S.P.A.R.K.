//! Authority modes, implied write classes, and the immutable-identity
//! catalog that gates every canonical write.
//!
//! ADR-0002 requires that every state definition declare exactly one
//! authority mode, that the mode imply a fixed write class, and that
//! authority/write-class become immutable identity for any definition ID
//! once it has been declared: no hot tune, structural reload, alias,
//! restore, or ordinary migration may change it. This module is the
//! single place that enforces that invariant, and [`crate::state`] is
//! built so its write API cannot bypass it: there is deliberately no
//! generic "set arbitrary state" function anywhere in this crate.

use crate::hash::CanonicalEncoder;
use crate::id::DefinitionId;
use std::collections::BTreeMap;

/// Who is canonical for a definition's value
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §7, ADR-0002).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Authority {
    /// The host is canonical; S.P.A.R.K. may only observe/ingest values
    /// the host reports.
    HostOwned,
    /// S.P.A.R.K. is canonical; only validated engine effects (or an
    /// explicitly authorized migration) may change the value.
    SparkOwned,
    /// Deterministically computed from declared authoritative inputs;
    /// only the evaluator may write it.
    Derived,
}

/// The write class an [`Authority`] implies. This mapping is fixed by
/// ADR-0002 and is not configurable per definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WriteClass {
    HostIngressOnly,
    SparkEffectOrExplicitMigration,
    EvaluatorOnly,
}

impl Authority {
    pub fn implied_write_class(&self) -> WriteClass {
        match self {
            Authority::HostOwned => WriteClass::HostIngressOnly,
            Authority::SparkOwned => WriteClass::SparkEffectOrExplicitMigration,
            Authority::Derived => WriteClass::EvaluatorOnly,
        }
    }

    fn tag(&self) -> &'static str {
        match self {
            Authority::HostOwned => "host_owned",
            Authority::SparkOwned => "spark_owned",
            Authority::Derived => "derived",
        }
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str(self.tag());
    }
}

impl WriteClass {
    pub fn tag(&self) -> &'static str {
        match self {
            WriteClass::HostIngressOnly => "host_ingress_only",
            WriteClass::SparkEffectOrExplicitMigration => "spark_effect_or_explicit_migration",
            WriteClass::EvaluatorOnly => "evaluator_only",
        }
    }
}

/// Rejects an attempt to change the authority of an already-declared
/// definition ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityIdentityConflict {
    pub definition_id: DefinitionId,
    pub previously_declared: Authority,
    pub attempted: Authority,
}

impl std::fmt::Display for AuthorityIdentityConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "definition '{}' was declared {} and cannot be redeclared {} under the same ID; \
             an authority change requires a new definition ID, an explicit migration, and an authority ADR",
            self.definition_id,
            self.previously_declared.tag(),
            self.attempted.tag()
        )
    }
}

impl std::error::Error for AuthorityIdentityConflict {}

/// The immutable authority identity of every definition ID that has ever
/// been declared in this process. This is the mechanism, not a policy
/// suggestion: [`crate::state::StateStore`] refuses to construct a write
/// path for a definition unless its authority is declared here first, and
/// [`AuthorityCatalog::declare`] refuses to change it once set.
#[derive(Debug, Default, Clone)]
pub struct AuthorityCatalog {
    declared: BTreeMap<DefinitionId, Authority>,
}

impl AuthorityCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    /// Declares (or re-declares identically) a definition's authority.
    ///
    /// Declaring the same ID with the same authority again is a no-op
    /// success (idempotent structural reload / repeated profile load).
    /// Declaring the same ID with a *different* authority is rejected:
    /// this is the mechanical enforcement of ADR-0002's immutable
    /// authority identity.
    pub fn declare(
        &mut self,
        definition_id: DefinitionId,
        authority: Authority,
    ) -> Result<(), AuthorityIdentityConflict> {
        match self.declared.get(&definition_id) {
            None => {
                self.declared.insert(definition_id, authority);
                Ok(())
            }
            Some(existing) if *existing == authority => Ok(()),
            Some(existing) => Err(AuthorityIdentityConflict {
                definition_id: definition_id.clone(),
                previously_declared: *existing,
                attempted: authority,
            }),
        }
    }

    pub fn authority_of(&self, definition_id: &DefinitionId) -> Option<Authority> {
        self.declared.get(definition_id).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authority_implies_fixed_write_class() {
        assert_eq!(
            Authority::HostOwned.implied_write_class(),
            WriteClass::HostIngressOnly
        );
        assert_eq!(
            Authority::SparkOwned.implied_write_class(),
            WriteClass::SparkEffectOrExplicitMigration
        );
        assert_eq!(
            Authority::Derived.implied_write_class(),
            WriteClass::EvaluatorOnly
        );
    }

    #[test]
    fn redeclaring_same_authority_is_idempotent() {
        let mut catalog = AuthorityCatalog::new();
        let id = DefinitionId::new("trait.curiosity").unwrap();
        catalog.declare(id.clone(), Authority::SparkOwned).unwrap();
        assert!(catalog.declare(id, Authority::SparkOwned).is_ok());
    }

    #[test]
    fn changing_authority_under_same_id_is_rejected() {
        let mut catalog = AuthorityCatalog::new();
        let id = DefinitionId::new("state.resource.food_availability").unwrap();
        catalog.declare(id.clone(), Authority::HostOwned).unwrap();
        let err = catalog
            .declare(id.clone(), Authority::SparkOwned)
            .unwrap_err();
        assert_eq!(err.definition_id, id);
        assert_eq!(err.previously_declared, Authority::HostOwned);
        assert_eq!(err.attempted, Authority::SparkOwned);
    }
}
