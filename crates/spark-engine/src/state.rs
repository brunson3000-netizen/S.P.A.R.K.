//! `StateCell` and the activation-bound canonical `StateStore`.
//!
//! # Provenance is structural, not conventional
//!
//! A `StateStore` can only be produced by
//! [`crate::activation::ActivatedProfile::into_state_store`], because
//! [`StateStore::from_activation`] is `pub(crate)` and
//! `ActivatedProfile` has no public constructor. Since the activation
//! ceremony is the only producer of an `ActivatedProfile`, canonical state
//! cannot exist without having passed the whole ceremony — and there is no
//! method that alters the activated definitions afterwards.
//!
//! ```compile_fail
//! use spark_engine::state::StateStore;
//! // There is no public constructor of any shape: not from raw entries,
//! // not from an activated schema, not a struct literal.
//! let store = StateStore::new(std::iter::empty());
//! ```
//!
//! ```compile_fail
//! use spark_engine::state::StateStore;
//! use spark_core::id::ProfileId;
//! let forged = StateStore {
//!     profile_id: ProfileId::new("game-world").unwrap(),
//! };
//! ```
//!
//! # Writes
//!
//! The three write paths (`observe_host_owned`, `apply_spark_effect`,
//! `commit_derived`) are `pub(crate)`. Phase 1 authorizes no host or
//! evaluator facade, so a caller outside this crate can read canonical
//! state but cannot write it at all; Phase 2's evaluator and Phase 3's
//! host facade will live in this crate and mediate them. Each path is
//! hard-coded to exactly one authority and validates profile, authority,
//! value type, declared bounds, and scope compatibility against the
//! activated definitions before writing anything.
//!
//! ```compile_fail
//! use spark_engine::state::StateStore;
//! fn write(store: &mut StateStore) {
//!     // Crate-internal: no public write path exists.
//!     store.apply_spark_effect(todo!(), todo!(), todo!(), todo!(), todo!(), 1).unwrap();
//! }
//! ```

use crate::activation::{ActivatedDefinition, ActivatedProfile};
use spark_core::authority::Authority;
use spark_core::clock::LogicalTime;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{DefinitionId, ProfileId};
use spark_core::scope::{ScopeId, ScopeKind};
use spark_core::value::{CanonicalValue, ValueType};
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

impl std::fmt::Display for SourceRefsBoundExceeded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a state cell may retain at most {} source references",
            self.limit
        )
    }
}

impl std::error::Error for SourceRefsBoundExceeded {}

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

/// Rejects a write that violates the activated definition set.
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

/// Holds canonical [`StateCell`] instances for exactly one profile, keyed
/// by `(definition_id, scope_id)`, over an immutable activated definition
/// set.
///
/// The store carries **both** exact-artifact hashes — the activation hash
/// and the manifest content hash — so persisted state can be bound to the
/// precise profile artifact that produced it (ADR-0006 exact-artifact
/// continuation), and so two divergent activation histories are visibly
/// different on every downstream digest rather than silently
/// interchangeable.
#[derive(Debug, Clone)]
pub struct StateStore {
    profile_id: ProfileId,
    manifest_content_hash: Digest,
    activation_hash: Digest,
    definitions: BTreeMap<DefinitionId, ActivatedDefinition>,
    cells: BTreeMap<(DefinitionId, ScopeId), StateCell>,
}

impl StateStore {
    /// The only constructor, reachable solely through
    /// [`ActivatedProfile::into_state_store`].
    pub(crate) fn from_activation(activated: ActivatedProfile) -> Self {
        let (profile_id, manifest_content_hash, activation_hash, definitions) =
            activated.into_parts();
        Self {
            profile_id,
            manifest_content_hash,
            activation_hash,
            definitions,
            cells: BTreeMap::new(),
        }
    }

    /// The profile this store was activated for.
    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    /// The activation hash of the definition set this store was built
    /// from.
    pub fn activation_hash(&self) -> &Digest {
        &self.activation_hash
    }

    /// The content hash of the exact manifest artifact that was activated.
    pub fn manifest_content_hash(&self) -> &Digest {
        &self.manifest_content_hash
    }

    pub fn schema_of(
        &self,
        profile_id: &ProfileId,
        definition_id: &DefinitionId,
    ) -> Option<&ActivatedDefinition> {
        if profile_id != &self.profile_id {
            return None;
        }
        self.definitions.get(definition_id)
    }

    /// Every activated definition backing this store, in stable ascending
    /// definition-ID order. Shared references only.
    pub fn definitions(&self) -> impl Iterator<Item = &ActivatedDefinition> {
        self.definitions.values()
    }

    pub fn get(
        &self,
        profile_id: &ProfileId,
        definition_id: &DefinitionId,
        scope_id: &ScopeId,
    ) -> Option<&StateCell> {
        if profile_id != &self.profile_id {
            return None;
        }
        self.cells.get(&(definition_id.clone(), scope_id.clone()))
    }

    /// A deterministic canonical digest of this store's exact-artifact
    /// binding and every cell, in stable key order.
    pub fn canonical_state_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("state_store");
        self.profile_id.canonicalize(&mut enc);
        enc.push_digest(&self.manifest_content_hash);
        enc.push_digest(&self.activation_hash);
        enc.push_u64(self.cells.len() as u64);
        for cell in self.cells.values() {
            let mut inner = CanonicalEncoder::new();
            cell.canonicalize(&mut inner);
            enc.push_block(&inner);
        }
        enc.finish()
    }

    // ---------------- Phase-2 crate-internal read-only helpers ----------------

    /// Every cell of one definition, in stable ascending scope order (the
    /// aggregation input; v1 §7).
    pub(crate) fn cells_of<'a>(
        &'a self,
        definition_id: &'a DefinitionId,
    ) -> impl Iterator<Item = &'a StateCell> + 'a {
        self.cells
            .iter()
            .filter(move |((d, _), _)| d == definition_id)
            .map(|(_, c)| c)
    }

    /// Phase-2 decay/recovery (v1 Q7): materialize the rule set's declared
    /// baseline into the **existing** `StateCell.baseline` field of one cell
    /// the engine has just written, if the cell has none. It never overwrites
    /// a baseline and never creates a cell; the `StateCell` encoding is
    /// unchanged (it always carried the optional baseline).
    pub(crate) fn materialize_baseline(
        &mut self,
        definition_id: &DefinitionId,
        scope_id: &ScopeId,
        raw: i64,
    ) {
        let fixed = self
            .schema_of(&self.profile_id, definition_id)
            .map(|d| d.value_constraint().value_type() == spark_core::value::ValueType::Fixed)
            .unwrap_or(false);
        if let Some(cell) = self
            .cells
            .get_mut(&(definition_id.clone(), scope_id.clone()))
        {
            if cell.baseline.is_none() {
                cell.baseline = Some(if fixed {
                    CanonicalValue::Fixed(spark_core::value::FixedPoint::from_raw(raw))
                } else {
                    CanonicalValue::Int(raw)
                });
            }
        }
    }

    /// Validates an evaluator effect without writing (the preflight half of
    /// validate-all-then-apply-all; v1 Q3). `validate_write` reads only the
    /// activated schema, never cell contents.
    pub(crate) fn validate_effect(
        &self,
        definition_id: &DefinitionId,
        scope_id: &ScopeId,
        value: &CanonicalValue,
        path: crate::report::WritePath,
    ) -> Result<(), StateWriteError> {
        let (required, name) = match path {
            crate::report::WritePath::SparkEffect => (Authority::SparkOwned, "apply_spark_effect"),
            crate::report::WritePath::CommitDerived => (Authority::Derived, "commit_derived"),
        };
        self.validate_write(
            &self.profile_id,
            definition_id,
            scope_id,
            value,
            required,
            name,
        )
    }

    /// Validates a host observation without writing.
    pub(crate) fn validate_observation(
        &self,
        definition_id: &DefinitionId,
        scope_id: &ScopeId,
        value: &CanonicalValue,
    ) -> Result<(), StateWriteError> {
        self.validate_write(
            &self.profile_id,
            definition_id,
            scope_id,
            value,
            Authority::HostOwned,
            "observe_host_owned",
        )
    }

    // ---------------- crate-internal write paths ----------------
    //
    // Phase 1 has no non-test caller for these: the host/evaluator facade
    // is Phase 2/3 scope and will live in this crate. The
    // `cfg_attr(not(any(test, feature = "test-support")), allow(dead_code))`
    // silences the resulting lint precisely where there is genuinely no
    // caller, without masking dead code once a real one exists.

    #[cfg_attr(not(any(test, feature = "test-support")), allow(dead_code))]
    fn validate_write(
        &self,
        profile_id: &ProfileId,
        definition_id: &DefinitionId,
        scope_id: &ScopeId,
        value: &CanonicalValue,
        required: Authority,
        attempted_path: &'static str,
    ) -> Result<(), StateWriteError> {
        if profile_id != &self.profile_id {
            return Err(StateWriteError::ForeignProfile {
                store_profile: self.profile_id.clone(),
                attempted_profile: profile_id.clone(),
            });
        }

        let definition = self.definitions.get(definition_id).ok_or_else(|| {
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
    #[cfg_attr(not(any(test, feature = "test-support")), allow(dead_code))]
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
    /// engine effect (Phase 2+).
    #[cfg_attr(not(any(test, feature = "test-support")), allow(dead_code))]
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
    #[cfg_attr(not(any(test, feature = "test-support")), allow(dead_code))]
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

    #[cfg_attr(not(any(test, feature = "test-support")), allow(dead_code))]
    fn upsert(
        &mut self,
        definition_id: DefinitionId,
        scope_id: ScopeId,
        value: CanonicalValue,
        at: LogicalTime,
        behavior_epoch: u64,
    ) {
        let profile_id = self.profile_id.clone();
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
    use crate::activation::{definition_fingerprint, ActivationRegistry};
    use crate::profile::definition::{DefinitionKind, DefinitionSpec};
    use crate::profile::manifest::ProfileManifest;
    use crate::profile::text::BoundedText;
    use spark_core::value::ValueConstraint;
    use std::collections::BTreeSet;

    fn profile() -> ProfileId {
        ProfileId::new("game-world").unwrap()
    }

    fn def(id: &str) -> DefinitionId {
        DefinitionId::new(id).unwrap()
    }

    fn spec(id: &str, authority: Authority) -> DefinitionSpec {
        DefinitionSpec {
            profile_id: profile(),
            id: def(id),
            kind: DefinitionKind::StateDefinition,
            domain: None,
            layer: None,
            value_constraint: ValueConstraint::int(0, 10).unwrap(),
            authority,
            valid_scopes: BTreeSet::from([ScopeKind::Actor]),
            enabled: true,
            version: 1,
            description: BoundedText::new("state definition").unwrap(),
            behavioral_leverage: None,
        }
    }

    fn store_with(specs: Vec<DefinitionSpec>) -> StateStore {
        let manifest = ProfileManifest::new(profile(), BoundedText::new("1.0.0").unwrap(), specs);
        let mut registry = ActivationRegistry::new();
        registry.activate(&manifest).unwrap().into_state_store()
    }

    fn actor() -> ScopeId {
        ScopeId::new(ScopeKind::Actor, "bron").unwrap()
    }

    /// Phase-1 test corpus item 11: host-owned state cannot be mutated by
    /// the S.P.A.R.K.-owned write path.
    #[test]
    fn host_owned_state_rejects_the_spark_effect_path() {
        let mut store = store_with(vec![spec("state.food.availability", Authority::HostOwned)]);
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
        let mut store = store_with(vec![spec("state.pressure.derived", Authority::Derived)]);
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
        let mut store = store_with(vec![
            spec("state.host.value", Authority::HostOwned),
            spec("state.spark.value", Authority::SparkOwned),
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
        let mut store = store_with(vec![spec("state.spark.value", Authority::SparkOwned)]);
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
        let mut store = store_with(vec![spec("state.spark.value", Authority::SparkOwned)]);
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
        let mut store = store_with(vec![spec("state.spark.value", Authority::SparkOwned)]);
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
        let mut store = store_with(vec![spec("state.spark.value", Authority::SparkOwned)]);
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

    /// The store's definitions carry the door-computed fingerprint, not
    /// anything a caller supplied.
    #[test]
    fn activated_definitions_carry_door_computed_identity() {
        let s = spec("state.spark.value", Authority::SparkOwned);
        let store = store_with(vec![s.clone()]);
        let activated = store
            .schema_of(&profile(), &def("state.spark.value"))
            .unwrap();
        assert_eq!(activated.fingerprint(), &definition_fingerprint(&s));
        assert_ne!(activated.fingerprint(), &Digest::ZERO);
    }

    /// AT-A4: the store carries the full exact-artifact binding, and
    /// neither hash is the zero digest.
    #[test]
    fn store_carries_full_artifact_binding() {
        let manifest = ProfileManifest::new(
            profile(),
            BoundedText::new("1.0.0").unwrap(),
            vec![spec("state.spark.value", Authority::SparkOwned)],
        );
        let mut registry = ActivationRegistry::new();
        let activated = registry.activate(&manifest).unwrap();
        let expected_activation = activated.activation_hash().clone();
        let store = activated.into_state_store();

        assert_eq!(store.activation_hash(), &expected_activation);
        assert_eq!(
            store.manifest_content_hash(),
            &manifest.manifest_content_hash()
        );
        assert_ne!(store.activation_hash(), &Digest::ZERO);
        assert_ne!(store.manifest_content_hash(), &Digest::ZERO);
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
        let mut store = store_with(vec![spec("state.spark.value", Authority::SparkOwned)]);
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
