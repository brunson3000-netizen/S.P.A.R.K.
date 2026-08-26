//! Profile validation and activation (ADR-0004 "Validator requirements"),
//! re-founded per `PHASE_1_REFOUNDATION_BRIEF_v0.1.md` §3.
//!
//! Phase 1 implements only enough of the validator to support Phase-1
//! fixtures cleanly, as the implementation brief authorizes: reject
//! duplicate definition IDs within a manifest, reject a definition whose
//! `profile_id` disagrees with its manifest, reject a definition with no
//! valid scope, and — the load-bearing check — reject any attempt to
//! declare a `(profile_id, definition_id)` with a different immutable
//! identity than it already has. Richer structural checks (cycles, dead
//! definitions, excessive fan-out) are Phase-2 rule-runtime concerns once
//! there is a rule graph to check.
//!
//! # Validation and activation are one ceremony
//!
//! The previous design let a validated-looking `DefinitionSpec` convert
//! itself into a state schema, and let a `StateStore` be built from raw
//! schema entries — so validation was advisory. Here, validation *is* the
//! activation path: [`validate`] hands the manifest's declarations to
//! [`DefinitionIdentityRegistry::activate`], which computes each
//! fingerprint, enforces immutable identity, and mints the
//! [`ActivatedSchema`]. There is no other way to obtain one, so a
//! [`ValidatedManifest`] is itself the evidence that the ceremony ran.
//!
//! Validation remains **pure with respect to the registry until it
//! succeeds**: every check runs against a read-only view and the registry
//! is mutated once, atomically, only after the whole manifest passes. A
//! manifest that fails partway leaves the registry exactly as it was, and
//! a corrected retry succeeds cleanly.

use crate::manifest::ProfileManifest;
use spark_core::activation::{ActivatedSchema, DefinitionIdentityRegistry, SchemaActivationError};
use spark_core::hash::Digest;
use spark_core::id::{DefinitionId, ProfileId};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    DuplicateDefinitionId(DefinitionId),
    /// A definition's `profile_id` does not match the manifest it was
    /// loaded in.
    ProfileMismatch {
        definition_id: DefinitionId,
        manifest_profile_id: ProfileId,
        definition_profile_id: ProfileId,
    },
    /// The canonical activation ceremony rejected a declaration.
    Activation(SchemaActivationError),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::DuplicateDefinitionId(id) => write!(
                f,
                "definition id '{id}' appears more than once in this manifest"
            ),
            ValidationError::ProfileMismatch {
                definition_id,
                manifest_profile_id,
                definition_profile_id,
            } => write!(
                f,
                "definition '{definition_id}' declares profile '{definition_profile_id}' but was loaded in manifest for profile '{manifest_profile_id}'"
            ),
            ValidationError::Activation(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// A manifest that passed validation and whose definitions were activated
/// into the canonical identity registry.
///
/// Fields are private and there is no public constructor, so this type is
/// itself proof that [`validate`] ran. It carries the manifest's exact
/// content hash (ADR-0004 content identity) alongside the
/// [`ActivatedSchema`] a [`spark_core::state::StateStore`] is built from,
/// so persisted state can be bound to the exact profile artifact that
/// produced it (ADR-0006).
///
/// ```compile_fail
/// use spark_profile::validate::ValidatedManifest;
/// use spark_core::hash::Digest;
/// let forged = ValidatedManifest { manifest_content_hash: Digest::ZERO };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedManifest {
    profile_id: ProfileId,
    manifest_content_hash: Digest,
    activated_schema: ActivatedSchema,
}

impl ValidatedManifest {
    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    /// The exact content hash of the manifest that was validated. A reused
    /// human version label with different content produces a different
    /// value here (ADR-0004).
    pub fn manifest_content_hash(&self) -> &Digest {
        &self.manifest_content_hash
    }

    /// The activated definition set. Consume it with
    /// [`spark_core::state::StateStore::from_activated_schema`].
    pub fn activated_schema(&self) -> &ActivatedSchema {
        &self.activated_schema
    }

    /// Takes ownership of the activated schema, for handing to a
    /// `StateStore`.
    pub fn into_activated_schema(self) -> ActivatedSchema {
        self.activated_schema
    }
}

/// Validates a [`ProfileManifest`] and activates its definitions into
/// `registry`.
///
/// On success every definition's registry-computed fingerprint is
/// committed atomically and the resulting [`ValidatedManifest`] carries
/// the activated schema. On failure `registry` is left completely
/// unchanged and every error found is reported, not just the first.
pub fn validate(
    manifest: &ProfileManifest,
    registry: &mut DefinitionIdentityRegistry,
) -> Result<ValidatedManifest, Vec<ValidationError>> {
    let mut errors: Vec<ValidationError> = Vec::new();
    let mut seen_in_this_manifest: BTreeSet<DefinitionId> = BTreeSet::new();

    for definition in manifest.definitions() {
        if !seen_in_this_manifest.insert(definition.id.clone()) {
            errors.push(ValidationError::DuplicateDefinitionId(
                definition.id.clone(),
            ));
            continue;
        }
        if definition.profile_id != *manifest.profile_id() {
            errors.push(ValidationError::ProfileMismatch {
                definition_id: definition.id.clone(),
                manifest_profile_id: manifest.profile_id().clone(),
                definition_profile_id: definition.profile_id.clone(),
            });
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let declarations: Vec<_> = manifest
        .definitions()
        .iter()
        .map(|d| d.to_declaration())
        .collect();

    match registry.activate(manifest.profile_id(), &declarations) {
        Ok(activated_schema) => Ok(ValidatedManifest {
            profile_id: manifest.profile_id().clone(),
            manifest_content_hash: manifest.manifest_content_hash(),
            activated_schema,
        }),
        Err(activation_errors) => Err(activation_errors
            .into_iter()
            .map(ValidationError::Activation)
            .collect()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::{DefinitionKind, DefinitionSpec};
    use crate::text::BoundedText;
    use spark_core::authority::Authority;
    use spark_core::scope::ScopeKind;
    use spark_core::state::StateStore;
    use spark_core::value::{FixedPoint, ValueConstraint};
    use std::collections::BTreeSet;

    fn def(id: &str, authority: Authority, scopes: BTreeSet<ScopeKind>) -> DefinitionSpec {
        DefinitionSpec {
            profile_id: ProfileId::new("game-world").unwrap(),
            id: DefinitionId::new(id).unwrap(),
            kind: DefinitionKind::Trait,
            domain: None,
            layer: None,
            value_constraint: ValueConstraint::fixed(
                FixedPoint::ZERO,
                FixedPoint::from_integer(1).unwrap(),
            )
            .unwrap(),
            authority,
            valid_scopes: scopes,
            enabled: true,
            version: 1,
            description: BoundedText::new("test definition").unwrap(),
            behavioral_leverage: None,
        }
    }

    fn manifest(defs: Vec<DefinitionSpec>) -> ProfileManifest {
        ProfileManifest::new(
            ProfileId::new("game-world").unwrap(),
            BoundedText::new("1.0.0").unwrap(),
            defs,
        )
    }

    #[test]
    fn valid_manifest_passes_and_yields_an_activated_schema() {
        let m = manifest(vec![def(
            "trait.curiosity",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        )]);
        let mut registry = DefinitionIdentityRegistry::new();
        let validated = validate(&m, &mut registry).unwrap();
        assert_eq!(
            validated.manifest_content_hash(),
            &m.manifest_content_hash()
        );
        assert_eq!(validated.activated_schema().len(), 1);

        // The activated schema is what a StateStore is built from.
        let store = StateStore::from_activated_schema(validated.into_activated_schema());
        assert!(store
            .schema_of(
                &ProfileId::new("game-world").unwrap(),
                &DefinitionId::new("trait.curiosity").unwrap()
            )
            .is_some());
    }

    #[test]
    fn duplicate_id_within_manifest_is_rejected() {
        let m = manifest(vec![
            def(
                "trait.curiosity",
                Authority::SparkOwned,
                BTreeSet::from([ScopeKind::Actor]),
            ),
            def(
                "trait.curiosity",
                Authority::SparkOwned,
                BTreeSet::from([ScopeKind::Actor]),
            ),
        ]);
        let mut registry = DefinitionIdentityRegistry::new();
        let errors = validate(&m, &mut registry).unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::DuplicateDefinitionId(_)
        ));
        assert!(registry.is_empty());
    }

    #[test]
    fn missing_valid_scope_is_rejected() {
        let m = manifest(vec![def(
            "trait.curiosity",
            Authority::SparkOwned,
            BTreeSet::new(),
        )]);
        let mut registry = DefinitionIdentityRegistry::new();
        let errors = validate(&m, &mut registry).unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::Activation(SchemaActivationError::NoValidScope { .. })
        ));
    }

    /// Authority/identity change under the same definition ID must be
    /// rejected even through an ordinary structural profile reload.
    #[test]
    fn authority_change_across_structural_reload_is_rejected() {
        let mut registry = DefinitionIdentityRegistry::new();

        let original = manifest(vec![def(
            "state.resource.food_availability",
            Authority::HostOwned,
            BTreeSet::from([ScopeKind::Settlement]),
        )]);
        validate(&original, &mut registry).unwrap();

        let reload_attempt = manifest(vec![def(
            "state.resource.food_availability",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Settlement]),
        )]);
        let errors = validate(&reload_attempt, &mut registry).unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::Activation(SchemaActivationError::IdentityConflict { .. })
        ));
    }

    /// B-04 regression: a changed value constraint under the same ID and
    /// authority is also an identity change.
    #[test]
    fn changed_value_type_under_same_id_and_authority_is_rejected() {
        let mut registry = DefinitionIdentityRegistry::new();
        let original_def = def(
            "trait.curiosity",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        );
        validate(&manifest(vec![original_def]), &mut registry).unwrap();

        let mut retyped_def = def(
            "trait.curiosity",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        );
        retyped_def.value_constraint = ValueConstraint::boolean();
        let errors = validate(&manifest(vec![retyped_def]), &mut registry).unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::Activation(SchemaActivationError::IdentityConflict { .. })
        ));
    }

    /// B-04 regression: changed valid scopes under the same
    /// ID/authority/type is an identity change.
    #[test]
    fn changed_valid_scopes_is_rejected() {
        let mut registry = DefinitionIdentityRegistry::new();
        validate(
            &manifest(vec![def(
                "trait.curiosity",
                Authority::SparkOwned,
                BTreeSet::from([ScopeKind::Actor]),
            )]),
            &mut registry,
        )
        .unwrap();

        let errors = validate(
            &manifest(vec![def(
                "trait.curiosity",
                Authority::SparkOwned,
                BTreeSet::from([ScopeKind::Actor, ScopeKind::Household]),
            )]),
            &mut registry,
        )
        .unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::Activation(SchemaActivationError::IdentityConflict { .. })
        ));
    }

    /// B-04 regression: a manifest with one valid and one invalid
    /// definition leaves the registry completely unchanged, and a
    /// corrected retry succeeds cleanly.
    #[test]
    fn failed_multi_definition_validation_leaves_registry_unchanged_and_retry_succeeds() {
        let mut registry = DefinitionIdentityRegistry::new();

        let good = def(
            "trait.curiosity",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        );
        let bad = def("trait.skepticism", Authority::SparkOwned, BTreeSet::new());
        let errors = validate(&manifest(vec![good.clone(), bad]), &mut registry).unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::Activation(SchemaActivationError::NoValidScope { .. })
        ));

        assert!(registry
            .fingerprint_of(&good.profile_id, &good.id)
            .is_none());

        let fixed = def(
            "trait.skepticism",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        );
        assert!(validate(&manifest(vec![good, fixed]), &mut registry).is_ok());
    }

    /// B-04 regression: the same `DefinitionId` in two independent
    /// profiles must not conflict.
    #[test]
    fn same_definition_id_in_independent_profiles_does_not_conflict() {
        let mut registry = DefinitionIdentityRegistry::new();

        let mut in_a = def(
            "trait.curiosity",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        );
        in_a.profile_id = ProfileId::new("game-world").unwrap();
        let manifest_a = ProfileManifest::new(
            in_a.profile_id.clone(),
            BoundedText::new("1.0.0").unwrap(),
            vec![in_a],
        );
        validate(&manifest_a, &mut registry).unwrap();

        let mut in_b = def(
            "trait.curiosity",
            Authority::HostOwned,
            BTreeSet::from([ScopeKind::Household]),
        );
        in_b.profile_id = ProfileId::new("mci-social").unwrap();
        let manifest_b = ProfileManifest::new(
            in_b.profile_id.clone(),
            BoundedText::new("1.0.0").unwrap(),
            vec![in_b],
        );
        assert!(validate(&manifest_b, &mut registry).is_ok());
    }

    #[test]
    fn definition_declaring_a_foreign_profile_is_rejected() {
        let mut foreign = def(
            "trait.curiosity",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        );
        foreign.profile_id = ProfileId::new("mci-social").unwrap();
        let mut registry = DefinitionIdentityRegistry::new();
        let errors = validate(&manifest(vec![foreign]), &mut registry).unwrap_err();
        assert!(matches!(errors[0], ValidationError::ProfileMismatch { .. }));
        assert!(registry.is_empty());
    }

    /// Extensibility acceptance criterion: a new tendency and a new
    /// supernatural trigger are addable purely as profile data through
    /// the existing grammar, with no Rust/engine change
    /// (`CONTROLLING_BLUEPRINT_v0.2.md` §30.4, ADR-0004 verification).
    #[test]
    fn new_trait_and_supernatural_trigger_are_addable_as_pure_profile_data() {
        let mut blood_moon = def(
            "trigger.supernatural.blood_moon",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Region, ScopeKind::World]),
        );
        blood_moon.kind = DefinitionKind::Trigger;

        let m = manifest(vec![
            def(
                "trait.curiosity",
                Authority::SparkOwned,
                BTreeSet::from([ScopeKind::Actor]),
            ),
            blood_moon,
        ]);
        let mut registry = DefinitionIdentityRegistry::new();
        assert!(validate(&m, &mut registry).is_ok());
    }

    /// A profile may introduce a wholly new definition kind as profile
    /// vocabulary, still without a Rust change (ADR-0004 taxonomy rule).
    #[test]
    fn a_new_custom_definition_kind_is_addable_as_profile_data() {
        let mut custom = def(
            "watershed.aquifer_stress",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Watershed]),
        );
        custom.kind = DefinitionKind::custom("hydrological_pressure").unwrap();
        let mut registry = DefinitionIdentityRegistry::new();
        assert!(validate(&manifest(vec![custom]), &mut registry).is_ok());
    }
}
