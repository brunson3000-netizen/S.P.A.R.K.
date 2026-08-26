//! `StateCell` and the authority-safe `StateStore` write API.
//!
//! `CONTROLLING_BLUEPRINT_v0.2.md` §7 and the Phase-1 implementation
//! brief both require that the Phase-1 API "not contain a generic state
//! mutation path" that lets S.P.A.R.K. commit `host_owned` truth or lets
//! a caller directly write `derived` truth. This module has no `set`
//! method at all: there are three narrow entry points
//! (`observe_host_owned`, `apply_spark_effect`, `commit_derived`), each
//! hard-wired to exactly one [`Authority`], and each checks the cell's
//! declared authority in the [`AuthorityCatalog`] before writing anything.
//! A caller cannot reach a `host_owned` cell through `apply_spark_effect`
//! even if they wanted to: the function checks the catalog and returns
//! `Err` instead of writing.

use crate::authority::{Authority, AuthorityCatalog};
use crate::clock::LogicalTime;
use crate::hash::CanonicalEncoder;
use crate::id::DefinitionId;
use crate::scope::ScopeId;
use crate::value::CanonicalValue;
use std::collections::BTreeMap;

/// Maximum number of bounded source references retained per cell
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §12.2 `source_refs (bounded)`).
/// Phase 1 has no evaluator producing real provenance chains yet; the
/// bound exists so the shape is falsifiable now rather than discovered
/// unbounded in Phase 2.
pub const MAX_SOURCE_REFS: usize = 8;

/// A bounded, order-preserving list of source references.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceRefs(Vec<DefinitionId>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRefsBoundExceeded {
    pub limit: usize,
}

impl SourceRefs {
    pub fn push(&mut self, source: DefinitionId) -> Result<(), SourceRefsBoundExceeded> {
        if self.0.len() >= MAX_SOURCE_REFS {
            return Err(SourceRefsBoundExceeded {
                limit: MAX_SOURCE_REFS,
            });
        }
        self.0.push(source);
        Ok(())
    }

    pub fn as_slice(&self) -> &[DefinitionId] {
        &self.0
    }
}

/// A typed value attached to a scope
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §12.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateCell {
    pub definition_id: DefinitionId,
    pub scope_id: ScopeId,
    pub value: CanonicalValue,
    pub baseline: Option<CanonicalValue>,
    pub created_at: LogicalTime,
    pub updated_at: LogicalTime,
    pub salience: i64,
    pub source_refs: SourceRefs,
    pub behavior_epoch: u64,
}

impl StateCell {
    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.definition_id.canonicalize(enc);
        self.scope_id.canonicalize(enc);
        self.value.canonicalize(enc);
        match &self.baseline {
            Some(b) => {
                enc.push_bool(true);
                b.canonicalize(enc);
            }
            None => {
                enc.push_bool(false);
            }
        }
        self.created_at.canonicalize(enc);
        self.updated_at.canonicalize(enc);
        enc.push_i64(self.salience);
        enc.push_u64(self.source_refs.as_slice().len() as u64);
        for r in self.source_refs.as_slice() {
            r.canonicalize(enc);
        }
        enc.push_u64(self.behavior_epoch);
    }
}

/// Rejects a write attempted through the wrong authority-specific entry
/// point, or against a definition ID with no declared authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateWriteError {
    /// The definition ID has no declared authority; declare it in the
    /// [`AuthorityCatalog`] first (normally via profile load/validation).
    UndeclaredAuthority { definition_id: DefinitionId },
    /// The definition's declared authority does not match the entry
    /// point used to write it.
    WrongAuthorityForWritePath {
        definition_id: DefinitionId,
        declared: Authority,
        attempted_path: &'static str,
    },
}

impl std::fmt::Display for StateWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateWriteError::UndeclaredAuthority { definition_id } => write!(
                f,
                "definition '{definition_id}' has no declared authority; it cannot be written"
            ),
            StateWriteError::WrongAuthorityForWritePath {
                definition_id,
                declared,
                attempted_path,
            } => write!(
                f,
                "definition '{definition_id}' is {declared:?} and cannot be written through the '{attempted_path}' path"
            ),
        }
    }
}

impl std::error::Error for StateWriteError {}

/// Holds canonical `StateCell` instances, keyed by `(definition_id,
/// scope_id)`, and exposes only authority-specific write paths.
///
/// There is intentionally no `set`/`write` method taking an arbitrary
/// [`Authority`] parameter: that shape would let a bug (or a future
/// careless caller) commit `host_owned` truth from inside S.P.A.R.K., or
/// let an external client write `derived` truth directly. Each of the
/// three methods below is hard-coded to check one specific authority.
#[derive(Debug, Default)]
pub struct StateStore {
    catalog: AuthorityCatalog,
    cells: BTreeMap<(DefinitionId, ScopeId), StateCell>,
}

fn key(definition_id: &DefinitionId, scope_id: &ScopeId) -> (DefinitionId, ScopeId) {
    (definition_id.clone(), scope_id.clone())
}

impl StateStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Declares a definition's authority. Must be called (normally by
    /// profile validation) before any cell under that ID can be written.
    /// Delegates immutable-identity enforcement to [`AuthorityCatalog`].
    pub fn declare_authority(
        &mut self,
        definition_id: DefinitionId,
        authority: Authority,
    ) -> Result<(), crate::authority::AuthorityIdentityConflict> {
        self.catalog.declare(definition_id, authority)
    }

    pub fn authority_of(&self, definition_id: &DefinitionId) -> Option<Authority> {
        self.catalog.authority_of(definition_id)
    }

    fn require_authority(
        &self,
        definition_id: &DefinitionId,
        required: Authority,
        attempted_path: &'static str,
    ) -> Result<(), StateWriteError> {
        match self.catalog.authority_of(definition_id) {
            None => Err(StateWriteError::UndeclaredAuthority {
                definition_id: definition_id.clone(),
            }),
            Some(declared) if same_authority(declared, required) => Ok(()),
            Some(declared) => Err(StateWriteError::WrongAuthorityForWritePath {
                definition_id: definition_id.clone(),
                declared,
                attempted_path,
            }),
        }
    }

    /// The only path that may write a `host_owned` cell: a validated
    /// observation reported by the host. S.P.A.R.K. never invents
    /// `host_owned` truth; it only records what the host said.
    pub fn observe_host_owned(
        &mut self,
        definition_id: DefinitionId,
        scope_id: ScopeId,
        value: CanonicalValue,
        at: LogicalTime,
        behavior_epoch: u64,
    ) -> Result<(), StateWriteError> {
        self.require_authority(&definition_id, Authority::HostOwned, "observe_host_owned")?;
        self.upsert(definition_id, scope_id, value, at, behavior_epoch);
        Ok(())
    }

    /// The only path that may write a `spark_owned` cell: a validated
    /// engine effect (Phase 2+) or, for Phase 1's skeleton, a direct
    /// authorized write used by fixtures/tests standing in for that
    /// effect runtime.
    pub fn apply_spark_effect(
        &mut self,
        definition_id: DefinitionId,
        scope_id: ScopeId,
        value: CanonicalValue,
        at: LogicalTime,
        behavior_epoch: u64,
    ) -> Result<(), StateWriteError> {
        self.require_authority(&definition_id, Authority::SparkOwned, "apply_spark_effect")?;
        self.upsert(definition_id, scope_id, value, at, behavior_epoch);
        Ok(())
    }

    /// The only path that may write a `derived` cell: the deterministic
    /// evaluator (Phase 2+) recomputing it from declared authoritative
    /// inputs. No client-facing API may call this directly.
    pub fn commit_derived(
        &mut self,
        definition_id: DefinitionId,
        scope_id: ScopeId,
        value: CanonicalValue,
        at: LogicalTime,
        behavior_epoch: u64,
    ) -> Result<(), StateWriteError> {
        self.require_authority(&definition_id, Authority::Derived, "commit_derived")?;
        self.upsert(definition_id, scope_id, value, at, behavior_epoch);
        Ok(())
    }

    fn upsert(
        &mut self,
        definition_id: DefinitionId,
        scope_id: ScopeId,
        value: CanonicalValue,
        at: LogicalTime,
        behavior_epoch: u64,
    ) {
        let k = key(&definition_id, &scope_id);
        match self.cells.get_mut(&k) {
            Some(existing) => {
                existing.value = value;
                existing.updated_at = at;
                existing.behavior_epoch = behavior_epoch;
            }
            None => {
                self.cells.insert(
                    k,
                    StateCell {
                        definition_id,
                        scope_id,
                        value,
                        baseline: None,
                        created_at: at,
                        updated_at: at,
                        salience: 0,
                        source_refs: SourceRefs::default(),
                        behavior_epoch,
                    },
                );
            }
        }
    }

    pub fn get(&self, definition_id: &DefinitionId, scope_id: &ScopeId) -> Option<&StateCell> {
        self.cells.get(&key(definition_id, scope_id))
    }
}

fn same_authority(a: Authority, b: Authority) -> bool {
    std::mem::discriminant(&a) == std::mem::discriminant(&b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scope::ScopeKind;

    fn actor_scope(name: &str) -> ScopeId {
        ScopeId::new(ScopeKind::Actor, name).unwrap()
    }

    #[test]
    fn host_owned_cannot_be_mutated_by_spark_effect_write_path() {
        let mut store = StateStore::new();
        let id = DefinitionId::new("state.resource.food_availability").unwrap();
        store
            .declare_authority(id.clone(), Authority::HostOwned)
            .unwrap();

        let err = store
            .apply_spark_effect(
                id.clone(),
                actor_scope("bron"),
                CanonicalValue::Int(1),
                LogicalTime(1),
                0,
            )
            .unwrap_err();
        assert!(matches!(
            err,
            StateWriteError::WrongAuthorityForWritePath { .. }
        ));
        assert!(store.get(&id, &actor_scope("bron")).is_none());
    }

    #[test]
    fn derived_cannot_be_written_by_client_facing_effect_path() {
        let mut store = StateStore::new();
        let id = DefinitionId::new("state.derived.food_insecurity").unwrap();
        store
            .declare_authority(id.clone(), Authority::Derived)
            .unwrap();

        let err = store
            .apply_spark_effect(
                id.clone(),
                actor_scope("household.1"),
                CanonicalValue::Bool(true),
                LogicalTime(1),
                0,
            )
            .unwrap_err();
        assert!(matches!(
            err,
            StateWriteError::WrongAuthorityForWritePath { .. }
        ));

        // Only the evaluator-only path succeeds.
        store
            .commit_derived(
                id.clone(),
                actor_scope("household.1"),
                CanonicalValue::Bool(true),
                LogicalTime(1),
                0,
            )
            .unwrap();
        assert!(store.get(&id, &actor_scope("household.1")).is_some());
    }

    #[test]
    fn undeclared_definition_cannot_be_written_through_any_path() {
        let mut store = StateStore::new();
        let id = DefinitionId::new("trait.curiosity").unwrap();
        let err = store
            .apply_spark_effect(
                id,
                actor_scope("bron"),
                CanonicalValue::Fixed(crate::value::FixedPoint::ZERO),
                LogicalTime(0),
                0,
            )
            .unwrap_err();
        assert!(matches!(err, StateWriteError::UndeclaredAuthority { .. }));
    }
}
