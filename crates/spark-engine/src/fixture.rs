//! Sanctioned test-only seams over the crate-internal state-write paths.
//!
//! This module exists only under `#[cfg(test)]` or the `test-support`
//! cargo feature, which is off by default and enabled solely by
//! `spark-testkit`'s **dev**-dependency edge. It is therefore not part of
//! the production surface, and the workspace feature-hygiene test asserts
//! mechanically that no production dependency edge enables it
//! (`PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md` §3.6).
//!
//! It exists to replace the two failure modes the Phase-1 record shows:
//! "make it public for the tests" (which is what produced B-02 three
//! times) and "leave it untestable" (which is what let the schema
//! validation rules go unfalsified). Cross-crate tests still go through
//! the production door wherever possible — building a `ProfileManifest`
//! and calling [`crate::activation::ActivationRegistry::activate`] *is*
//! the production path and is the preferred fixture style. These seams
//! are only for the write paths, which have no Phase-1 production caller
//! at all because the host/evaluator facade is Phase 2/3 scope.
//!
//! The seams are thin forwarders. They add no capability the crate's own
//! evaluator would not have: every call still runs the full schema
//! validation (profile, authority, value type, declared bounds, scope
//! compatibility) inside [`crate::state::StateStore`].

use crate::state::{StateStore, StateWriteError};
use spark_core::clock::LogicalTime;
use spark_core::id::{DefinitionId, ProfileId};
use spark_core::scope::ScopeId;
use spark_core::value::CanonicalValue;

/// Test-only forwarder to the `host_owned` write path.
#[allow(clippy::too_many_arguments)]
pub fn observe_host_owned(
    store: &mut StateStore,
    profile_id: ProfileId,
    definition_id: DefinitionId,
    scope_id: ScopeId,
    value: CanonicalValue,
    at: LogicalTime,
    behavior_epoch: u64,
) -> Result<(), StateWriteError> {
    store.observe_host_owned(
        profile_id,
        definition_id,
        scope_id,
        value,
        at,
        behavior_epoch,
    )
}

/// Test-only forwarder to the `spark_owned` write path.
#[allow(clippy::too_many_arguments)]
pub fn apply_spark_effect(
    store: &mut StateStore,
    profile_id: ProfileId,
    definition_id: DefinitionId,
    scope_id: ScopeId,
    value: CanonicalValue,
    at: LogicalTime,
    behavior_epoch: u64,
) -> Result<(), StateWriteError> {
    store.apply_spark_effect(
        profile_id,
        definition_id,
        scope_id,
        value,
        at,
        behavior_epoch,
    )
}

/// Test-only forwarder to the `derived` write path.
#[allow(clippy::too_many_arguments)]
pub fn commit_derived(
    store: &mut StateStore,
    profile_id: ProfileId,
    definition_id: DefinitionId,
    scope_id: ScopeId,
    value: CanonicalValue,
    at: LogicalTime,
    behavior_epoch: u64,
) -> Result<(), StateWriteError> {
    store.commit_derived(
        profile_id,
        definition_id,
        scope_id,
        value,
        at,
        behavior_epoch,
    )
}
