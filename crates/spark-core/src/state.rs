//! `StateCell`, the immutable profile-qualified [`DefinitionSchema`], and
//! the authority-safe `StateStore` write API.
//!
//! The Phase-1 correction brief (B-02) requires that Phase 1 expose **no
//! public raw state mutation API** that lets an arbitrary external crate
//! declare authority on demand, invoke host-ingress/evaluator writes
//! directly, or write a value that violates a validated definition's
//! type/bounds/scope/profile. This module therefore has exactly one way
//! to build a `StateStore`: from an already-validated, immutable
//! [`DefinitionSchema`] table supplied at construction. There is no
//! runtime schema-mutation method, and the three write paths
//! (`observe_host_owned`, `apply_spark_effect`, `commit_derived`) are
//! `pub(crate)` rather than public — the only "later higher-level
//! host/evaluator facade" Phase 1 has is its own internal tests, which the
//! correction brief explicitly sanctions ("tests may access internal
//! paths from inside the crate; do not make unsafe runtime APIs public
//! merely for test convenience"). A caller outside this crate can read
//! state (`get`) but cannot write it at all in Phase 1: that capability is
//! deliberately deferred to the Phase-2/3 rule runtime and host/evaluator
//! facades that do not exist yet.

use crate::authority::Authority;
use crate::clock::LogicalTime;
use crate::hash::{CanonicalEncoder, Digest};
use crate::id::{DefinitionId, ProfileId};
use crate::scope::ScopeId;
use crate::scope::ScopeKind;
use crate::value::{CanonicalValue, ValueConstraint, ValueType};
use std::collections::{BTreeMap, BTreeSet};

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

/// The immutable, profile-qualified schema entry a `StateStore` is built
/// from (Phase-1 correction brief B-02): the definition's full identity
/// fingerprint (opaque to this crate — computed and owned by
/// `spark-profile`), its authority, its declared value constraint, and its
/// valid scopes. `StateStore` has no way to construct or alter this table
/// after activation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionSchema {
    pub profile_id: ProfileId,
    pub definition_id: DefinitionId,
    pub fingerprint: Digest,
    pub authority: Authority,
    pub value_constraint: ValueConstraint,
    pub valid_scopes: BTreeSet<ScopeKind>,
}

/// A typed value attached to a scope
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §12.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateCell {
    pub profile_id: ProfileId,
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
        self.profile_id.canonicalize(enc);
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

/// Rejects a write that violates the definition's declared schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateWriteError {
    /// No schema entry exists for `(profile_id, definition_id)`; declare
    /// it in the immutable schema supplied at `StateStore` construction.
    UndeclaredDefinition {
        profile_id: ProfileId,
        definition_id: DefinitionId,
    },
    /// The definition's declared authority does not match the entry
    /// point used to write it.
    WrongAuthorityForWritePath {
        profile_id: ProfileId,
        definition_id: DefinitionId,
        declared: Authority,
        attempted_path: &'static str,
    },
    /// The value's runtime type does not match the definition's declared
    /// value constraint.
    WrongValueType {
        profile_id: ProfileId,
        definition_id: DefinitionId,
        expected: ValueType,
        actual: ValueType,
    },
    /// The value's runtime type matches, but its content falls outside
    /// the definition's declared bounds.
    OutOfBounds {
        profile_id: ProfileId,
        definition_id: DefinitionId,
    },
    /// `scope_id`'s kind is not among the definition's declared
    /// `valid_scopes`.
    DisallowedScope {
        profile_id: ProfileId,
        definition_id: DefinitionId,
        scope_kind: ScopeKind,
    },
}

impl std::fmt::Display for StateWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateWriteError::UndeclaredDefinition {
                profile_id,
                definition_id,
            } => write!(
                f,
                "definition '{definition_id}' has no declared schema in profile '{profile_id}'; it cannot be written"
            ),
            StateWriteError::WrongAuthorityForWritePath {
                profile_id,
                definition_id,
                declared,
                attempted_path,
            } => write!(
                f,
                "definition '{definition_id}' in profile '{profile_id}' is {declared:?} and cannot be written through the '{attempted_path}' path"
            ),
            StateWriteError::WrongValueType {
                profile_id,
                definition_id,
                expected,
                actual,
            } => write!(
                f,
                "definition '{definition_id}' in profile '{profile_id}' expects value type {expected:?}, got {actual:?}"
            ),
            StateWriteError::OutOfBounds {
                profile_id,
                definition_id,
            } => write!(
                f,
                "value for definition '{definition_id}' in profile '{profile_id}' is out of its declared bounds"
            ),
            StateWriteError::DisallowedScope {
                profile_id,
                definition_id,
                scope_kind,
            } => write!(
                f,
                "scope kind '{scope_kind}' is not a valid scope for definition '{definition_id}' in profile '{profile_id}'"
            ),
        }
    }
}

impl std::error::Error for StateWriteError {}

/// Holds canonical `StateCell` instances, keyed by `(profile_id,
/// definition_id, scope_id)`, built once from an immutable, already
/// profile-qualified [`DefinitionSchema`] table and exposing only
/// authority-specific, crate-internal write paths.
///
/// There is intentionally no public `set`/`write`/`declare_authority`
/// method: that shape would let a bug (or a future careless caller)
/// commit `host_owned` truth from inside S.P.A.R.K., let an external
/// client write `derived` truth directly, or redeclare a definition's
/// schema after activation. Each of the three `pub(crate)` write methods
/// below is hard-coded to check one specific authority against the
/// immutable schema before writing anything.
#[derive(Debug)]
pub struct StateStore {
    schema: BTreeMap<(ProfileId, DefinitionId), DefinitionSchema>,
    cells: BTreeMap<(ProfileId, DefinitionId, ScopeId), StateCell>,
}

fn cell_key(
    profile_id: &ProfileId,
    definition_id: &DefinitionId,
    scope_id: &ScopeId,
) -> (ProfileId, DefinitionId, ScopeId) {
    (profile_id.clone(), definition_id.clone(), scope_id.clone())
}

impl StateStore {
    /// Builds a `StateStore` from an immutable, already-validated schema
    /// table. There is no method to add, remove, or alter a schema entry
    /// afterward.
    pub fn new(schema: impl IntoIterator<Item = DefinitionSchema>) -> Self {
        let mut map = BTreeMap::new();
        for entry in schema {
            map.insert(
                (entry.profile_id.clone(), entry.definition_id.clone()),
                entry,
            );
        }
        Self {
            schema: map,
            cells: BTreeMap::new(),
        }
    }

    pub fn schema_of(
        &self,
        profile_id: &ProfileId,
        definition_id: &DefinitionId,
    ) -> Option<&DefinitionSchema> {
        self.schema
            .get(&(profile_id.clone(), definition_id.clone()))
    }

    // Phase 1 has no non-test caller for the write paths below: the
    // "later higher-level host/evaluator facade" the correction brief
    // anticipates is Phase 2/3 scope. `cfg_attr(not(test), allow(dead_code))`
    // silences the resulting lint precisely in non-test builds without
    // masking genuine dead code once a real caller exists (at which point
    // the allow becomes inert, not misleading).
    #[cfg_attr(not(test), allow(dead_code))]
    fn validate_write(
        &self,
        profile_id: &ProfileId,
        definition_id: &DefinitionId,
        scope_id: &ScopeId,
        value: &CanonicalValue,
        required: Authority,
        attempted_path: &'static str,
    ) -> Result<(), StateWriteError> {
        let schema = self
            .schema
            .get(&(profile_id.clone(), definition_id.clone()))
            .ok_or_else(|| StateWriteError::UndeclaredDefinition {
                profile_id: profile_id.clone(),
                definition_id: definition_id.clone(),
            })?;

        if !same_authority(schema.authority, required) {
            return Err(StateWriteError::WrongAuthorityForWritePath {
                profile_id: profile_id.clone(),
                definition_id: definition_id.clone(),
                declared: schema.authority,
                attempted_path,
            });
        }

        if value.type_tag() != schema.value_constraint.value_type() {
            return Err(StateWriteError::WrongValueType {
                profile_id: profile_id.clone(),
                definition_id: definition_id.clone(),
                expected: schema.value_constraint.value_type(),
                actual: value.type_tag(),
            });
        }

        if !schema.value_constraint.accepts(value) {
            return Err(StateWriteError::OutOfBounds {
                profile_id: profile_id.clone(),
                definition_id: definition_id.clone(),
            });
        }

        if !schema.valid_scopes.contains(scope_id.kind()) {
            return Err(StateWriteError::DisallowedScope {
                profile_id: profile_id.clone(),
                definition_id: definition_id.clone(),
                scope_kind: scope_id.kind().clone(),
            });
        }

        Ok(())
    }

    /// The only path that may write a `host_owned` cell: a validated
    /// observation reported by the host. S.P.A.R.K. never invents
    /// `host_owned` truth; it only records what the host said.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn observe_host_owned(
        &mut self,
        profile_id: ProfileId,
        definition_id: DefinitionId,
        scope_id: ScopeId,
        value: CanonicalValue,
        at: LogicalTime,
        behavior_epoch: u64,
    ) -> Result<(), StateWriteError> {
        self.validate_write(
            &profile_id,
            &definition_id,
            &scope_id,
            &value,
            Authority::HostOwned,
            "observe_host_owned",
        )?;
        self.upsert(
            profile_id,
            definition_id,
            scope_id,
            value,
            at,
            behavior_epoch,
        );
        Ok(())
    }

    /// The only path that may write a `spark_owned` cell: a validated
    /// engine effect (Phase 2+) or, for Phase 1's skeleton, a direct
    /// authorized write used by this crate's own tests standing in for
    /// that effect runtime.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn apply_spark_effect(
        &mut self,
        profile_id: ProfileId,
        definition_id: DefinitionId,
        scope_id: ScopeId,
        value: CanonicalValue,
        at: LogicalTime,
        behavior_epoch: u64,
    ) -> Result<(), StateWriteError> {
        self.validate_write(
            &profile_id,
            &definition_id,
            &scope_id,
            &value,
            Authority::SparkOwned,
            "apply_spark_effect",
        )?;
        self.upsert(
            profile_id,
            definition_id,
            scope_id,
            value,
            at,
            behavior_epoch,
        );
        Ok(())
    }

    /// The only path that may write a `derived` cell: the deterministic
    /// evaluator (Phase 2+) recomputing it from declared authoritative
    /// inputs. No client-facing API may call this directly.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn commit_derived(
        &mut self,
        profile_id: ProfileId,
        definition_id: DefinitionId,
        scope_id: ScopeId,
        value: CanonicalValue,
        at: LogicalTime,
        behavior_epoch: u64,
    ) -> Result<(), StateWriteError> {
        self.validate_write(
            &profile_id,
            &definition_id,
            &scope_id,
            &value,
            Authority::Derived,
            "commit_derived",
        )?;
        self.upsert(
            profile_id,
            definition_id,
            scope_id,
            value,
            at,
            behavior_epoch,
        );
        Ok(())
    }

    #[cfg_attr(not(test), allow(dead_code))]
    fn upsert(
        &mut self,
        profile_id: ProfileId,
        definition_id: DefinitionId,
        scope_id: ScopeId,
        value: CanonicalValue,
        at: LogicalTime,
        behavior_epoch: u64,
    ) {
        let k = cell_key(&profile_id, &definition_id, &scope_id);
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
                        profile_id,
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

    pub fn get(
        &self,
        profile_id: &ProfileId,
        definition_id: &DefinitionId,
        scope_id: &ScopeId,
    ) -> Option<&StateCell> {
        self.cells
            .get(&cell_key(profile_id, definition_id, scope_id))
    }
}

#[cfg_attr(not(test), allow(dead_code))]
fn same_authority(a: Authority, b: Authority) -> bool {
    std::mem::discriminant(&a) == std::mem::discriminant(&b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::hash_bytes;
    use crate::scope::ScopeKind;
    use crate::value::FixedPoint;

    fn actor_scope(name: &str) -> ScopeId {
        ScopeId::new(ScopeKind::Actor, name).unwrap()
    }

    fn profile_a() -> ProfileId {
        ProfileId::new("game-world").unwrap()
    }

    fn profile_b() -> ProfileId {
        ProfileId::new("mci-social").unwrap()
    }

    fn schema_entry(
        profile_id: ProfileId,
        definition_id: &str,
        authority: Authority,
        value_constraint: ValueConstraint,
        valid_scopes: BTreeSet<ScopeKind>,
    ) -> DefinitionSchema {
        DefinitionSchema {
            profile_id,
            definition_id: DefinitionId::new(definition_id).unwrap(),
            fingerprint: hash_bytes(definition_id.as_bytes()),
            authority,
            value_constraint,
            valid_scopes,
        }
    }

    #[test]
    fn host_owned_cannot_be_mutated_by_spark_effect_write_path() {
        let id = DefinitionId::new("state.resource.food_availability").unwrap();
        let mut store = StateStore::new([schema_entry(
            profile_a(),
            id.as_str(),
            Authority::HostOwned,
            ValueConstraint::Int { min: 0, max: 1000 },
            BTreeSet::from([ScopeKind::Actor]),
        )]);

        let err = store
            .apply_spark_effect(
                profile_a(),
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
        assert!(store.get(&profile_a(), &id, &actor_scope("bron")).is_none());
    }

    #[test]
    fn derived_cannot_be_written_by_client_facing_effect_path() {
        let id = DefinitionId::new("state.derived.food_insecurity").unwrap();
        let mut store = StateStore::new([schema_entry(
            profile_a(),
            id.as_str(),
            Authority::Derived,
            ValueConstraint::Bool,
            BTreeSet::from([ScopeKind::Household]),
        )]);

        let err = store
            .apply_spark_effect(
                profile_a(),
                id.clone(),
                ScopeId::new(ScopeKind::Household, "household.1").unwrap(),
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
                profile_a(),
                id.clone(),
                ScopeId::new(ScopeKind::Household, "household.1").unwrap(),
                CanonicalValue::Bool(true),
                LogicalTime(1),
                0,
            )
            .unwrap();
        assert!(store
            .get(
                &profile_a(),
                &id,
                &ScopeId::new(ScopeKind::Household, "household.1").unwrap()
            )
            .is_some());
    }

    #[test]
    fn undeclared_definition_cannot_be_written_through_any_path() {
        let mut store = StateStore::new([]);
        let id = DefinitionId::new("trait.curiosity").unwrap();
        let err = store
            .apply_spark_effect(
                profile_a(),
                id,
                actor_scope("bron"),
                CanonicalValue::Fixed(FixedPoint::ZERO),
                LogicalTime(0),
                0,
            )
            .unwrap_err();
        assert!(matches!(err, StateWriteError::UndeclaredDefinition { .. }));
    }

    #[test]
    fn wrong_value_type_rejects() {
        let id = DefinitionId::new("trait.curiosity").unwrap();
        let mut store = StateStore::new([schema_entry(
            profile_a(),
            id.as_str(),
            Authority::SparkOwned,
            ValueConstraint::Fixed {
                min: FixedPoint::ZERO,
                max: FixedPoint::from_integer(1).unwrap(),
            },
            BTreeSet::from([ScopeKind::Actor]),
        )]);
        let err = store
            .apply_spark_effect(
                profile_a(),
                id,
                actor_scope("bron"),
                CanonicalValue::Bool(true),
                LogicalTime(0),
                0,
            )
            .unwrap_err();
        assert!(matches!(err, StateWriteError::WrongValueType { .. }));
    }

    #[test]
    fn out_of_bounds_value_rejects() {
        let id = DefinitionId::new("state.resource.food_availability").unwrap();
        let mut store = StateStore::new([schema_entry(
            profile_a(),
            id.as_str(),
            Authority::HostOwned,
            ValueConstraint::Int { min: 0, max: 10 },
            BTreeSet::from([ScopeKind::Settlement]),
        )]);
        let err = store
            .observe_host_owned(
                profile_a(),
                id,
                ScopeId::new(ScopeKind::Settlement, "pontafique").unwrap(),
                CanonicalValue::Int(11),
                LogicalTime(0),
                0,
            )
            .unwrap_err();
        assert!(matches!(err, StateWriteError::OutOfBounds { .. }));
    }

    #[test]
    fn disallowed_scope_rejects() {
        let id = DefinitionId::new("trait.curiosity").unwrap();
        let mut store = StateStore::new([schema_entry(
            profile_a(),
            id.as_str(),
            Authority::SparkOwned,
            ValueConstraint::Bool,
            BTreeSet::from([ScopeKind::Actor]),
        )]);
        let err = store
            .apply_spark_effect(
                profile_a(),
                id,
                ScopeId::new(ScopeKind::Settlement, "pontafique").unwrap(),
                CanonicalValue::Bool(true),
                LogicalTime(0),
                0,
            )
            .unwrap_err();
        assert!(matches!(err, StateWriteError::DisallowedScope { .. }));
    }

    #[test]
    fn wrong_profile_rejects() {
        let id = DefinitionId::new("trait.curiosity").unwrap();
        let mut store = StateStore::new([schema_entry(
            profile_a(),
            id.as_str(),
            Authority::SparkOwned,
            ValueConstraint::Bool,
            BTreeSet::from([ScopeKind::Actor]),
        )]);
        let err = store
            .apply_spark_effect(
                profile_b(),
                id,
                actor_scope("bron"),
                CanonicalValue::Bool(true),
                LogicalTime(0),
                0,
            )
            .unwrap_err();
        assert!(matches!(err, StateWriteError::UndeclaredDefinition { .. }));
    }

    /// Same `DefinitionId` declared independently in two different
    /// profiles must not let a write to one profile become visible under
    /// the other (Phase-1 correction brief B-02: "profile A cannot
    /// write/read private state as profile B through ID collision").
    #[test]
    fn same_definition_id_in_two_profiles_does_not_cross_contaminate() {
        let id = DefinitionId::new("trait.curiosity").unwrap();
        let mut store = StateStore::new([
            schema_entry(
                profile_a(),
                id.as_str(),
                Authority::SparkOwned,
                ValueConstraint::Bool,
                BTreeSet::from([ScopeKind::Actor]),
            ),
            schema_entry(
                profile_b(),
                id.as_str(),
                Authority::SparkOwned,
                ValueConstraint::Bool,
                BTreeSet::from([ScopeKind::Actor]),
            ),
        ]);
        store
            .apply_spark_effect(
                profile_a(),
                id.clone(),
                actor_scope("bron"),
                CanonicalValue::Bool(true),
                LogicalTime(0),
                0,
            )
            .unwrap();
        assert!(store.get(&profile_a(), &id, &actor_scope("bron")).is_some());
        assert!(store.get(&profile_b(), &id, &actor_scope("bron")).is_none());
    }
}
