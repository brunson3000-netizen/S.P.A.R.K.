//! Profile validation skeleton (ADR-0004 "Validator requirements").
//!
//! Phase 1 implements only enough of the validator to support Phase-1
//! fixtures cleanly, as authorized by the implementation brief: reject
//! duplicate definition IDs within one manifest, reject definitions with
//! no valid scope, and - the load-bearing check for Phase 1 - reject any
//! attempt to declare a definition ID with a different authority than it
//! was already declared with, by delegating to
//! [`spark_core::authority::AuthorityCatalog`]. Richer structural checks
//! (cycles, dead definitions, excessive fan-out) are Phase 2 rule-runtime
//! concerns once there is a rule graph to check.

use crate::manifest::ProfileManifest;
use spark_core::authority::{AuthorityCatalog, AuthorityIdentityConflict};
use spark_core::id::DefinitionId;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    DuplicateDefinitionId(DefinitionId),
    NoValidScope(DefinitionId),
    AuthorityIdentityConflict(AuthorityIdentityConflict),
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
            ValidationError::AuthorityIdentityConflict(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validates a [`ProfileManifest`] against an [`AuthorityCatalog`] shared
/// across profile loads, so authority identity is enforced not just
/// within one manifest but across repeated/structural reloads over time.
pub fn validate(
    manifest: &ProfileManifest,
    catalog: &mut AuthorityCatalog,
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    let mut seen_in_this_manifest = BTreeSet::new();

    for definition in &manifest.definitions {
        if !seen_in_this_manifest.insert(definition.id.clone()) {
            errors.push(ValidationError::DuplicateDefinitionId(
                definition.id.clone(),
            ));
            continue;
        }

        if definition.valid_scopes.is_empty() {
            errors.push(ValidationError::NoValidScope(definition.id.clone()));
        }

        if let Err(conflict) = catalog.declare(definition.id.clone(), definition.authority) {
            errors.push(ValidationError::AuthorityIdentityConflict(conflict));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::{DefinitionKind, DefinitionSpec};
    use spark_core::authority::Authority;
    use spark_core::id::{DefinitionId, ProfileId};
    use spark_core::scope::ScopeKind;
    use spark_core::value::ValueType;

    fn def(id: &str, authority: Authority, scopes: Vec<ScopeKind>) -> DefinitionSpec {
        DefinitionSpec {
            id: DefinitionId::new(id).unwrap(),
            kind: DefinitionKind::Trait,
            domain: None,
            layer: None,
            value_type: ValueType::Fixed,
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
            vec![ScopeKind::Actor],
        )]);
        let mut catalog = AuthorityCatalog::new();
        assert!(validate(&m, &mut catalog).is_ok());
    }

    #[test]
    fn duplicate_id_within_manifest_is_rejected() {
        let m = manifest(vec![
            def(
                "trait.curiosity",
                Authority::SparkOwned,
                vec![ScopeKind::Actor],
            ),
            def(
                "trait.curiosity",
                Authority::SparkOwned,
                vec![ScopeKind::Actor],
            ),
        ]);
        let mut catalog = AuthorityCatalog::new();
        let errors = validate(&m, &mut catalog).unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::DuplicateDefinitionId(_)
        ));
    }

    #[test]
    fn missing_valid_scope_is_rejected() {
        let m = manifest(vec![def("trait.curiosity", Authority::SparkOwned, vec![])]);
        let mut catalog = AuthorityCatalog::new();
        let errors = validate(&m, &mut catalog).unwrap_err();
        assert!(matches!(errors[0], ValidationError::NoValidScope(_)));
    }

    /// Authority/write-class change under the same definition ID must be
    /// rejected even when it is attempted through an ordinary structural
    /// profile reload (a second, later manifest validated against the
    /// same catalog), not only within a single manifest.
    #[test]
    fn authority_change_across_structural_reload_is_rejected() {
        let mut catalog = AuthorityCatalog::new();

        let original = manifest(vec![def(
            "state.resource.food_availability",
            Authority::HostOwned,
            vec![ScopeKind::Settlement],
        )]);
        validate(&original, &mut catalog).unwrap();

        let reload_attempt = manifest(vec![def(
            "state.resource.food_availability",
            Authority::SparkOwned,
            vec![ScopeKind::Settlement],
        )]);
        let errors = validate(&reload_attempt, &mut catalog).unwrap_err();
        assert!(matches!(
            errors[0],
            ValidationError::AuthorityIdentityConflict(_)
        ));
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
                vec![ScopeKind::Actor],
            ),
            def(
                "trigger.supernatural.blood_moon",
                Authority::SparkOwned,
                vec![ScopeKind::Region, ScopeKind::World],
            ),
        ]);
        let mut catalog = AuthorityCatalog::new();
        assert!(validate(&m, &mut catalog).is_ok());
    }
}
