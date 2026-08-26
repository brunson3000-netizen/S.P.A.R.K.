//! The single-registry composition convention, as a usable fixture.
//!
//! `PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md` §3.5 item 3
//! requires the production composition rule — *the composition root owns
//! exactly one [`ActivationRegistry`]* — to be "documented + tested, not
//! merely hoped". Phase 3 makes it structural inside the embedded
//! runtime/service wrapper; Phase 1 encodes it here, as the shape every
//! fixture uses, plus the content-addressing property that makes a
//! violation visible rather than silent.
//!
//! # Why a convention and not an enforcement
//!
//! Rust cannot prevent a second `ActivationRegistry::new()` in one
//! process, and claiming otherwise is what kept the authority-provenance
//! defect alive across three passes. What *is* enforced is that
//! divergence cannot hide: every artifact a registry mints carries its
//! `manifest_content_hash` and `activation_hash`, so two runtimes fed
//! identical manifests are byte-identical (harmlessly interchangeable)
//! and two fed divergent ones disagree on every downstream digest. See
//! `divergent_activations_are_never_interchangeable` in the acceptance
//! corpus.

use spark_core::hash::Digest;
use spark_core::id::{DefinitionId, ProfileId};
use spark_engine::activation::{ActivationRegistry, ValidationError};
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::state::StateStore;

/// A minimal stand-in for a production composition root: one process,
/// **one** activation registry, every profile activated through it.
#[derive(Debug, Default)]
pub struct SingleRegistryRuntime {
    registry: ActivationRegistry,
}

impl SingleRegistryRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    /// Activates a manifest through the one registry and hands back the
    /// state store it mints. This is the whole production path: there is
    /// no other way to obtain a store, here or anywhere else.
    pub fn activate(
        &mut self,
        manifest: &ProfileManifest,
    ) -> Result<StateStore, Vec<ValidationError>> {
        Ok(self.registry.activate(manifest)?.into_state_store())
    }

    /// The committed immutable identity of one definition, if activated.
    pub fn fingerprint_of(
        &self,
        profile_id: &ProfileId,
        definition_id: &DefinitionId,
    ) -> Option<&Digest> {
        self.registry.fingerprint_of(profile_id, definition_id)
    }

    /// The runtime's whole activation lineage, as a Phase-3 save would
    /// bind itself to.
    pub fn lineage_digest(&self) -> Digest {
        self.registry.lineage_digest()
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
    use spark_core::authority::Authority;
    use spark_core::scope::ScopeKind;
    use spark_core::value::ValueConstraint;
    use spark_engine::profile::definition::{DefinitionKind, DefinitionSpec};
    use spark_engine::profile::text::BoundedText;
    use std::collections::BTreeSet;

    fn profile() -> ProfileId {
        ProfileId::new("game-world").unwrap()
    }

    fn manifest(authority: Authority) -> ProfileManifest {
        ProfileManifest::new(
            profile(),
            BoundedText::new("1.0.0").unwrap(),
            vec![DefinitionSpec {
                profile_id: profile(),
                id: DefinitionId::new("trait.curiosity").unwrap(),
                kind: DefinitionKind::Trait,
                domain: None,
                layer: None,
                value_constraint: ValueConstraint::boolean(),
                authority,
                valid_scopes: BTreeSet::from([ScopeKind::Actor]),
                enabled: true,
                version: 1,
                description: BoundedText::new("curiosity").unwrap(),
                behavioral_leverage: None,
            }],
        )
    }

    /// One runtime, one registry: reactivating the same manifest is
    /// idempotent and the lineage does not grow.
    #[test]
    fn one_runtime_keeps_one_lineage() {
        let mut runtime = SingleRegistryRuntime::new();
        let m = manifest(Authority::SparkOwned);
        let first = runtime.activate(&m).unwrap();
        let lineage = runtime.lineage_digest();
        let second = runtime.activate(&m).unwrap();

        assert_eq!(first.activation_hash(), second.activation_hash());
        assert_eq!(lineage, runtime.lineage_digest());
        assert!(runtime
            .fingerprint_of(&profile(), &DefinitionId::new("trait.curiosity").unwrap())
            .is_some());
    }

    /// Two runtimes fed identical manifests are byte-identical, so a
    /// second instance is harmless; two fed divergent manifests disagree
    /// on every downstream digest, so a second instance is *visible*.
    #[test]
    fn a_second_runtime_is_either_identical_or_visibly_divergent() {
        let mut same_a = SingleRegistryRuntime::new();
        let mut same_b = SingleRegistryRuntime::new();
        let mut divergent = SingleRegistryRuntime::new();

        let a = same_a.activate(&manifest(Authority::SparkOwned)).unwrap();
        let b = same_b.activate(&manifest(Authority::SparkOwned)).unwrap();
        let c = divergent.activate(&manifest(Authority::HostOwned)).unwrap();

        assert_eq!(a.canonical_state_digest(), b.canonical_state_digest());
        assert_eq!(same_a.lineage_digest(), same_b.lineage_digest());

        assert_ne!(a.canonical_state_digest(), c.canonical_state_digest());
        assert_ne!(a.activation_hash(), c.activation_hash());
        assert_ne!(same_a.lineage_digest(), divergent.lineage_digest());
    }
}
