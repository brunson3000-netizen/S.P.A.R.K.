//! `StateCell` and the authority-safe, activation-bound `StateStore`.
//!
//! Re-founded per `PHASE_1_REFOUNDATION_BRIEF_v0.1.md` §3 (trusted schema
//! activation). The previous boundary failed independent review twice for
//! the same reason: a `StateStore` was built from a caller-supplied
//! `DefinitionSchema` whose fields — including its authority mode and its
//! identity fingerprint — were all public. Making the write facades
//! crate-internal did not repair that, because the schema those facades
//! validated against was itself forgeable, so external code could declare
//! `Derived` authority with a `Digest::ZERO` fingerprint and have it
//! installed as activated truth.
//!
//! There is now exactly one way to obtain a `StateStore`:
//! [`StateStore::from_activated_schema`], which consumes a
//! [`crate::activation::ActivatedSchema`] — a type with private fields and
//! no public constructor, obtainable only from
//! [`crate::activation::DefinitionIdentityRegistry::activate`]. Provenance
//! is therefore structural: a `StateStore`'s schema cannot exist without
//! having passed the activation ceremony, and there is no method that can
//! alter it afterwards.
//!
//! ```compile_fail
//! use spark_core::state::StateStore;
//! // The raw-schema constructor is gone; there is no way to install
//! // arbitrary authority metadata into a store.
//! let store = StateStore::new(std::iter::empty());
//! ```
//!
//! The three write paths (`observe_host_owned`, `apply_spark_effect`,
//! `commit_derived`) remain `pub(crate)`: Phase 1 authorizes no
//! host/evaluator facade, so a caller outside this crate can read state
//! but cannot write it at all. Each path is hard-coded to one authority
//! and validates profile, authority, value type, declared bounds, and
//! scope compatibility against the activated schema before writing
//! anything.

use crate::activation::{ActivatedDefinition, ActivatedSchema};
use crate::authority::Authority;
use crate::clock::LogicalTime;
use crate::hash::{CanonicalEncoder, Digest};
use crate::id::{DefinitionId, ProfileId};
use crate::scope::ScopeId;
use crate::scope::ScopeKind;
use crate::value::{CanonicalValue, ValueType};
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

/// Rejects a write that violates the activated schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateWriteError {
    /// The write names a profile other than the one this store was
    /// activated for (ADR-0004 profile/trust-domain isolation).
    ForeignProfile {
        store_profile: ProfileId,
        attempted_profile: ProfileId,
    },
    /// No activated definition exists for `definition_id` in this store.
    UndeclaredDefinition {
        profile_id: ProfileId,
        definition_id: DefinitionId,
    },
    /// The definition's activated authority does not match the entry
    /// point used to write it.
    WrongAuthorityForWritePath {
        profile_id: ProfileId,
        definition_id: DefinitionId,
        declared: Authority,
        attempted_path: &'static str,
    },
    /// The value's runtime type does not match the activated value
    /// constraint.
    WrongValueType {
        profile_id: ProfileId,
        definition_id: DefinitionId,
        expected: ValueType,
        actual: ValueType,
    },
    /// The value's runtime type matches, but its content falls outside
    /// the activated declared bounds.
    OutOfBounds {
        profile_id: ProfileId,
        definition_id: DefinitionId,
    },
    /// `scope_id`'s kind is not among the definition's activated
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
            StateWriteError::ForeignProfile {
                store_profile,
                attempted_profile,
            } => write!(
                f,
                "this state store was activated for profile '{store_profile}' and cannot hold state for profile '{attempted_profile}'"
            ),
            StateWriteError::UndeclaredDefinition {
                profile_id,
                definition_id,
            } => write!(
                f,
                "definition '{definition_id}' is not activated in profile '{profile_id}'; it cannot be written"
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

/// Holds canonical `StateCell` instances for exactly one profile, keyed by
/// `(definition_id, scope_id)`, over an immutable activated schema.
///
/// There is intentionally no public `new`, `set`, `write`, or
/// `declare_authority` method. The first would let arbitrary authority
/// metadata become activated truth; the rest would let a bug (or a future
/// careless caller) commit `host_owned` truth from inside S.P.A.R.K. or
/// write `derived` truth directly from a client.
#[derive(Debug)]
pub struct StateStore {
    schema: ActivatedSchema,
    cells: BTreeMap<(DefinitionId, ScopeId), StateCell>,
}

impl StateStore {
    /// The only constructor: build a store over an already-activated,
    /// immutable schema.
    pub fn from_activated_schema(schema: ActivatedSchema) -> Self {
        Self {
            schema,
            cells: BTreeMap::new(),
        }
    }

    /// The profile this store was activated for.
    pub fn profile_id(&self) -> &ProfileId {
        self.schema.profile_id()
    }

    /// The activated schema backing this store. Shared reference only —
    /// there is no path that mutates it after activation.
    pub fn activated_schema(&self) -> &ActivatedSchema {
        &self.schema
    }

    /// The activation hash of the schema this store was built from, so a
    /// caller can bind persisted state to the exact activated definition
    /// set that produced it (ADR-0006 exact-artifact continuation).
    pub fn activation_hash(&self) -> &Digest {
        self.schema.activation_hash()
    }

    pub fn schema_of(
        &self,
        profile_id: &ProfileId,
        definition_id: &DefinitionId,
    ) -> Option<&ActivatedDefinition> {
        if profile_id != self.schema.profile_id() {
            return None;
        }
        self.schema.get(definition_id)
    }

    // Phase 1 has no non-test caller for the write paths below: the
    // higher-level host/evaluator facade is Phase 2/3 scope. The
    // `cfg_attr(not(test), allow(dead_code))` silences the resulting lint
    // precisely in non-test builds without masking genuine dead code once
    // a real caller exists.
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
        if profile_id != self.schema.profile_id() {
            return Err(StateWriteError::ForeignProfile {
                store_profile: self.schema.profile_id().clone(),
                attempted_profile: profile_id.clone(),
            });
        }

        let definition = self.schema.get(definition_id).ok_or_else(|| {
            StateWriteError::UndeclaredDefinition {
                profile_id: profile_id.clone(),
                definition_id: definition_id.clone(),
            }
        })?;

        if definition.authority() != required {
            return Err(StateWriteError::WrongAuthorityForWritePath {
                profile_id: profile_id.clone(),
                definition_id: definition_id.clone(),
                declared: definition.authority(),
                attempted_path,
            });
        }

        let constraint = definition.value_constraint();
        if value.type_tag() != constraint.value_type() {
            return Err(StateWriteError::WrongValueType {
                profile_id: profile_id.clone(),
                definition_id: definition_id.clone(),
                expected: constraint.value_type(),
                actual: value.type_tag(),
            });
        }

        if !constraint.accepts(value) {
            return Err(StateWriteError::OutOfBounds {
                profile_id: profile_id.clone(),
                definition_id: definition_id.clone(),
            });
        }

        if !definition.valid_scopes().contains(scope_id.kind()) {
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
        self.upsert(definition_id, scope_id, value, at, behavior_epoch);
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
        self.upsert(definition_id, scope_id, value, at, behavior_epoch);
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
        self.upsert(definition_id, scope_id, value, at, behavior_epoch);
        Ok(())
    }

    #[cfg_attr(not(test), allow(dead_code))]
    fn upsert(
        &mut self,
        definition_id: DefinitionId,
        scope_id: ScopeId,
        value: CanonicalValue,
        at: LogicalTime,
        behavior_epoch: u64,
    ) {
        let profile_id = self.schema.profile_id().clone();
        let k = (definition_id.clone(), scope_id.clone());
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
        if profile_id != self.schema.profile_id() {
            return None;
        }
        self.cells.get(&(definition_id.clone(), scope_id.clone()))
    }

    /// A deterministic canonical digest of this store's activated schema
    /// and every cell, in stable key order.
    pub fn canonical_state_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("state_store");
        enc.push_digest(self.schema.activation_hash());
        enc.push_u64(self.cells.len() as u64);
        for cell in self.cells.values() {
            let mut inner = CanonicalEncoder::new();
            cell.canonicalize(&mut inner);
            enc.push_block(&inner);
        }
        enc.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activation::{DefinitionDeclaration, DefinitionIdentityRegistry, DefinitionKindTag};
    use crate::id::CanonicalTag;
    use crate::value::ValueConstraint;
    use std::collections::BTreeSet;

    fn profile() -> ProfileId {
        ProfileId::new("game-world").unwrap()
    }

    fn def(id: &str) -> DefinitionId {
        DefinitionId::new(id).unwrap()
    }

    fn declaration(id: &str, authority: Authority) -> DefinitionDeclaration {
        DefinitionDeclaration {
            profile_id: profile(),
            definition_id: def(id),
            kind: DefinitionKindTag::builtin(CanonicalTag::new("state_definition").unwrap()),
            authority,
            value_constraint: ValueConstraint::int(0, 10).unwrap(),
            valid_scopes: BTreeSet::from([ScopeKind::Actor]),
        }
    }

    fn store_with(declarations: &[DefinitionDeclaration]) -> StateStore {
        let mut registry = DefinitionIdentityRegistry::new();
        let schema = registry.activate(&profile(), declarations).unwrap();
        StateStore::from_activated_schema(schema)
    }

    fn actor() -> ScopeId {
        ScopeId::new(ScopeKind::Actor, "bron").unwrap()
    }

    /// Phase-1 test corpus item 11: host-owned state cannot be mutated by
    /// the S.P.A.R.K.-owned write path.
    #[test]
    fn host_owned_state_rejects_the_spark_effect_path() {
        let mut store = store_with(&[declaration("state.food.availability", Authority::HostOwned)]);
        let err = store
            .apply_spark_effect(
                profile(),
                def("state.food.availability"),
                actor(),
                CanonicalValue::Int(1),
                LogicalTime(0),
                1,
            )
            .unwrap_err();
        assert!(matches!(
            err,
            StateWriteError::WrongAuthorityForWritePath { .. }
        ));
        assert!(store
            .get(&profile(), &def("state.food.availability"), &actor())
            .is_none());
    }

    /// Phase-1 test corpus item 12: derived state cannot be independently
    /// written through host ingress or a spark effect.
    #[test]
    fn derived_state_rejects_host_ingress_and_spark_effect_paths() {
        let mut store = store_with(&[declaration("state.pressure.derived", Authority::Derived)]);
        assert!(store
            .observe_host_owned(
                profile(),
                def("state.pressure.derived"),
                actor(),
                CanonicalValue::Int(1),
                LogicalTime(0),
                1,
            )
            .is_err());
        assert!(store
            .apply_spark_effect(
                profile(),
                def("state.pressure.derived"),
                actor(),
                CanonicalValue::Int(1),
                LogicalTime(0),
                1,
            )
            .is_err());
        // Only the evaluator path succeeds.
        assert!(store
            .commit_derived(
                profile(),
                def("state.pressure.derived"),
                actor(),
                CanonicalValue::Int(1),
                LogicalTime(0),
                1,
            )
            .is_ok());
    }

    #[test]
    fn each_authority_accepts_only_its_own_path() {
        let mut store = store_with(&[
            declaration("state.host.value", Authority::HostOwned),
            declaration("state.spark.value", Authority::SparkOwned),
        ]);
        assert!(store
            .observe_host_owned(
                profile(),
                def("state.host.value"),
                actor(),
                CanonicalValue::Int(3),
                LogicalTime(0),
                1
            )
            .is_ok());
        assert!(store
            .apply_spark_effect(
                profile(),
                def("state.spark.value"),
                actor(),
                CanonicalValue::Int(4),
                LogicalTime(0),
                1
            )
            .is_ok());
        assert!(store
            .commit_derived(
                profile(),
                def("state.spark.value"),
                actor(),
                CanonicalValue::Int(4),
                LogicalTime(0),
                1
            )
            .is_err());
    }

    #[test]
    fn undeclared_definition_cannot_be_written() {
        let mut store = store_with(&[declaration("state.spark.value", Authority::SparkOwned)]);
        let err = store
            .apply_spark_effect(
                profile(),
                def("state.never.declared"),
                actor(),
                CanonicalValue::Int(1),
                LogicalTime(0),
                1,
            )
            .unwrap_err();
        assert!(matches!(err, StateWriteError::UndeclaredDefinition { .. }));
    }

    #[test]
    fn out_of_bounds_and_mistyped_values_are_rejected() {
        let mut store = store_with(&[declaration("state.spark.value", Authority::SparkOwned)]);
        assert!(matches!(
            store
                .apply_spark_effect(
                    profile(),
                    def("state.spark.value"),
                    actor(),
                    CanonicalValue::Int(11),
                    LogicalTime(0),
                    1
                )
                .unwrap_err(),
            StateWriteError::OutOfBounds { .. }
        ));
        assert!(matches!(
            store
                .apply_spark_effect(
                    profile(),
                    def("state.spark.value"),
                    actor(),
                    CanonicalValue::Bool(true),
                    LogicalTime(0),
                    1
                )
                .unwrap_err(),
            StateWriteError::WrongValueType { .. }
        ));
    }

    #[test]
    fn disallowed_scope_kind_is_rejected() {
        let mut store = store_with(&[declaration("state.spark.value", Authority::SparkOwned)]);
        let household = ScopeId::new(ScopeKind::Household, "bron").unwrap();
        assert!(matches!(
            store
                .apply_spark_effect(
                    profile(),
                    def("state.spark.value"),
                    household,
                    CanonicalValue::Int(1),
                    LogicalTime(0),
                    1
                )
                .unwrap_err(),
            StateWriteError::DisallowedScope { .. }
        ));
    }

    /// ADR-0004 profile isolation: a store activated for one profile
    /// cannot be used to hold or resolve another profile's state.
    #[test]
    fn foreign_profile_writes_and_reads_are_rejected() {
        let mut store = store_with(&[declaration("state.spark.value", Authority::SparkOwned)]);
        let other = ProfileId::new("mci-social").unwrap();
        assert!(matches!(
            store
                .apply_spark_effect(
                    other.clone(),
                    def("state.spark.value"),
                    actor(),
                    CanonicalValue::Int(1),
                    LogicalTime(0),
                    1
                )
                .unwrap_err(),
            StateWriteError::ForeignProfile { .. }
        ));
        assert!(store.schema_of(&other, &def("state.spark.value")).is_none());
    }

    /// The store's schema carries the registry-computed fingerprint, not
    /// anything a caller supplied.
    #[test]
    fn activated_schema_carries_registry_computed_identity() {
        let decl = declaration("state.spark.value", Authority::SparkOwned);
        let store = store_with(std::slice::from_ref(&decl));
        let activated = store
            .schema_of(&profile(), &def("state.spark.value"))
            .unwrap();
        assert_eq!(
            activated.fingerprint(),
            &crate::activation::definition_fingerprint(&decl)
        );
        assert_ne!(activated.fingerprint(), &Digest::ZERO);
    }

    #[test]
    fn source_refs_are_bounded() {
        let mut refs = SourceRefs::default();
        for i in 0..MAX_SOURCE_REFS {
            refs.push(def(&format!("trigger.source_{i}"))).unwrap();
        }
        assert_eq!(
            refs.push(def("trigger.overflow")),
            Err(SourceRefsBoundExceeded {
                limit: MAX_SOURCE_REFS
            })
        );
    }

    #[test]
    fn state_digest_reflects_written_cells() {
        let mut store = store_with(&[declaration("state.spark.value", Authority::SparkOwned)]);
        let empty = store.canonical_state_digest();
        store
            .apply_spark_effect(
                profile(),
                def("state.spark.value"),
                actor(),
                CanonicalValue::Int(1),
                LogicalTime(0),
                1,
            )
            .unwrap();
        assert_ne!(empty, store.canonical_state_digest());
    }
}
