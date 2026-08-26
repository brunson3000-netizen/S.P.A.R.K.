//! Profile validation skeleton (ADR-0004 "Validator requirements"),
//! corrected per the Phase-1 correction brief (B-04).
//!
//! Phase 1 implements only enough of the validator to support Phase-1
//! fixtures cleanly, as authorized by the implementation brief: reject
//! duplicate definition IDs within one manifest, reject definitions with
//! no valid scope, reject a definition whose `profile_id` disagrees with
//! its manifest, and — the load-bearing check for Phase 1 — reject any
//! attempt to declare a `(profile_id, definition_id)` with a different
//! full identity fingerprint than it was already declared with. Richer
//! structural checks (cycles, dead definitions, excessive fan-out) are
//! Phase 2 rule-runtime concerns once there is a rule graph to check.
//!
//! Validation is **pure with respect to the live registry until it
//! succeeds**: every definition in the manifest is checked against a
//! read-only view of the registry, and the registry is mutated only once,
//! atomically, after every definition has passed every check. A manifest
//! that fails partway through therefore leaves the registry exactly as it
//! was before validation started — the Phase-1 writer pass's bug (a later
//! duplicate/no-scope error returning failure after earlier declarations
//! in the same manifest remained installed) cannot recur, and a corrected
//! retry of a fixed manifest can always succeed cleanly.

use crate::identity::{DefinitionIdentityConflict, DefinitionIdentityRegistry};
use crate::manifest::ProfileManifest;
use spark_core::id::{DefinitionId, ProfileId};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    DuplicateDefinitionId(DefinitionId),
    NoValidScope(DefinitionId),
    /// A definition's `profile_id` does not match the manifest it was
    /// loaded in.
    ProfileMismatch {
        definition_id: DefinitionId,
        manifest_profile_id: ProfileId,
        definition_profile_id: ProfileId,
    },
    DefinitionIdentityConflict(DefinitionIdentityConflict),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::DuplicateDefinitionId(id) => {
                write!(
                    f,
                    "definition id '{id}' appears more than once in this manifest"
                )
            }
            ValidationError::NoValidScope(id) => {
                write!(f, "definition '{id}' declares no valid scope")
            }
            ValidationError::ProfileMismatch {
                definition_id,
                manifest_profile_id,
                definition_profile_id,
            } => write!(
                f,
                "definition '{definition_id}' declares profile '{definition_profile_id}' but was loaded in manifest for profile '{manifest_profile_id}'"
            ),
            ValidationError::DefinitionIdentityConflict(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validates a [`ProfileManifest`] against a [`DefinitionIdentityRegistry`]
/// shared across profile loads, so identity is enforced not just within
/// one manifest but across repeated/structural reloads over time. On
/// success, every definition's fingerprint is committed to `registry`
/// atomically. On failure, `registry` is left completely unchanged.
pub fn validate(
    manifest: &ProfileManifest,
    registry: &mut DefinitionIdentityRegistry,
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    let mut seen_in_this_manifest = BTreeSet::new();
    let mut candidate: Vec<((ProfileId, DefinitionId), spark_core::hash::Digest)> = Vec::new();

    for definition in &manifest.definitions {
        if !seen_in_this_manifest.insert(definition.id.clone()) {
            errors.push(ValidationError::DuplicateDefinitionId(
                definition.id.clone(),
            ));
            continue;
        }

        if definition.profile_id != manifest.profile_id {
            errors.push(ValidationError::ProfileMismatch {
                definition_id: definition.id.clone(),
                manifest_profile_id: manifest.profile_id.clone(),
                definition_profile_id: definition.profile_id.clone(),
            });
            continue;
        }

        if definition.valid_scopes.is_empty() {
            errors.push(ValidationError::NoValidScope(definition.id.clone()));
        }

        let fingerprint = definition.definition_fingerprint();
        match registry.fingerprint_of(&definition.profile_id, &definition.id) {
            Some(existing) if *existing != fingerprint => {
                errors.push(ValidationError::DefinitionIdentityConflict(
                    DefinitionIdentityConflict {
                        profile_id: definition.profile_id.clone(),
                        definition_id: definition.id.clone(),
                        previous_fingerprint: existing.clone(),
                        attempted_fingerprint: fingerprint,
                    },
                ));
            }
            _ => {
                candidate.push((
                    (definition.profile_id.clone(), definition.id.clone()),
                    fingerprint,
                ));
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    registry.commit(candidate);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::{DefinitionKind, DefinitionSpec};
    use spark_core::authority::Authority;
    use spark_core::id::{DefinitionId, ProfileId};
    use spark_core::scope::ScopeKind;
    use spark_core::value::ValueConstraint;
    use std::collections::BTreeSet;

    fn def(id: &str, authority: Authority, scopes: BTreeSet<ScopeKind>) -> DefinitionSpec {
        DefinitionSpec {
            profile_id: ProfileId::new("game-world").unwrap(),
            id: DefinitionId::new(id).unwrap(),
            kind: DefinitionKind::Trait,
            domain: None,
            layer: None,
            value_constraint: ValueConstraint::Fixed {
                min: spark_core::value::FixedPoint::ZERO,
                max: spark_core::value::FixedPoint::from_integer(1).unwrap(),
            },
            authority,
            valid_scopes: scopes,
            enabled: true,
            version: 1,
            description: "test definition".to_string(),
            behavioral_leverage: None,
        }
    }

    fn manifest(defs: Vec<DefinitionSpec>) -> ProfileManifest {
        ProfileManifest {
            profile_id: ProfileId::new("game-world").unwrap(),
            version_label: "1.0.0".to_string(),
            definitions: defs,
        }
    }

    #[test]
    fn valid_manifest_passes() {
        let m = manifest(vec![def(
            "trait.curiosity",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        )]);
        let mut registry = DefinitionIdentityRegistry::new();
        assert!(validate(&m, &mut registry).is_ok());
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
        assert!(matches!(errors[0], ValidationError::NoValidScope(_)));
    }

    /// Authority/identity change under the same definition ID must be
    /// rejected even when it is attempted through an ordinary structural
    /// profile reload (a second, later manifest validated against the
    /// same registry), not only within a single manifest.
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
            ValidationError::DefinitionIdentityConflict(_)
        ));
    }

    /// B-04: a changed `ValueType`/constraint under the same ID and
    /// authority is also an identity change and must reject.
    #[test]
    fn changed_value_type_under_same_id_and_authority_is_rejected() {
        let mut registry = DefinitionIdentityRegistry::new();
        let mut original_def = def(
            "trait.curiosity",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        );
        original_def.value_constraint = ValueConstraint::Fixed {
            min: spark_core::value::FixedPoint::ZERO,
            max: spark_core::value::FixedPoint::from_integer(1).unwrap(),
        };
        validate(&manifest(vec![original_def]), &mut registry).unwrap();

        let mut retyped_def = def(
            "trait.curiosity",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        );
        retyped_def.value_constraint = ValueConstraint::Bool;
        let errors = validate(&manifest(vec![retyped_def]), &mut registry).unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::DefinitionIdentityConflict(_)
        ));
    }

    /// B-04: changed valid scopes under the same ID/authority/type is an
    /// identity change and must reject.
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
            ValidationError::DefinitionIdentityConflict(_)
        ));
    }

    /// B-04: a manifest with one valid definition and one invalid
    /// definition must leave the live registry completely unchanged, and
    /// a corrected retry must then succeed cleanly.
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
        assert!(matches!(errors[0], ValidationError::NoValidScope(_)));

        // The registry must not have installed `trait.curiosity` even
        // though it individually passed every check.
        assert!(registry
            .fingerprint_of(&good.profile_id, &good.id)
            .is_none());

        // A corrected retry (fixing the scope) succeeds cleanly.
        let fixed = def(
            "trait.skepticism",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        );
        assert!(validate(&manifest(vec![good, fixed]), &mut registry).is_ok());
    }

    /// B-04: the same `DefinitionId` in two independent profiles must not
    /// conflict with each other.
    #[test]
    fn same_definition_id_in_independent_profiles_does_not_conflict() {
        let mut registry = DefinitionIdentityRegistry::new();

        let mut in_a = def(
            "trait.curiosity",
            Authority::SparkOwned,
            BTreeSet::from([ScopeKind::Actor]),
        );
        in_a.profile_id = ProfileId::new("game-world").unwrap();
        let manifest_a = ProfileManifest {
            profile_id: in_a.profile_id.clone(),
            version_label: "1.0.0".to_string(),
            definitions: vec![in_a],
        };
        validate(&manifest_a, &mut registry).unwrap();

        // A different profile can declare the same ID with a different
        // authority/scope without touching profile A's identity.
        let mut in_b = def(
            "trait.curiosity",
            Authority::HostOwned,
            BTreeSet::from([ScopeKind::Household]),
        );
        in_b.profile_id = ProfileId::new("mci-social").unwrap();
        let manifest_b = ProfileManifest {
            profile_id: in_b.profile_id.clone(),
            version_label: "1.0.0".to_string(),
            definitions: vec![in_b],
        };
        assert!(validate(&manifest_b, &mut registry).is_ok());
    }

    /// Demonstrates the extensibility acceptance criterion: a new
    /// tendency (`trait.curiosity`) and a new supernatural trigger
    /// (`trigger.supernatural.blood_moon`) are addable purely as profile
    /// data through the existing grammar, with no Rust/engine change
    /// (`CONTROLLING_BLUEPRINT_v0.2.md` §30.4, ADR-0004 verification).
    #[test]
    fn new_trait_and_supernatural_trigger_are_addable_as_pure_profile_data() {
        let m = manifest(vec![
            def(
                "trait.curiosity",
                Authority::SparkOwned,
                BTreeSet::from([ScopeKind::Actor]),
            ),
            def(
                "trigger.supernatural.blood_moon",
                Authority::SparkOwned,
                BTreeSet::from([ScopeKind::Region, ScopeKind::World]),
            ),
        ]);
        let mut registry = DefinitionIdentityRegistry::new();
        assert!(validate(&m, &mut registry).is_ok());
    }
}
