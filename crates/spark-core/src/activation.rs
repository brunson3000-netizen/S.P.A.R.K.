//! Trusted definition-identity registry and schema activation.
//!
//! This module is the single authority-declaration ceremony in S.P.A.R.K.
//! and the only source of an [`ActivatedSchema`]. It exists because the
//! independent Phase-1 re-review found the previous boundary structurally
//! forgeable: `DefinitionSchema` had all-public fields and
//! `StateStore::new` accepted arbitrary raw entries, so external code
//! could fabricate a `Derived` authority schema with a zero fingerprint
//! and install it as activated authority. Making the *write* facades
//! crate-internal did not help, because the schema those facades check
//! against was itself caller-supplied.
//!
//! The re-founded boundary inverts that. A caller supplies a
//! [`DefinitionDeclaration`], which is deliberately *untrusted* and
//! carries **no fingerprint field at all**. The registry, not the caller,
//! decides what a declaration's identity is:
//!
//! ```text
//! DefinitionDeclaration (untrusted, no fingerprint)
//!   -> DefinitionIdentityRegistry::activate  (validate whole batch, atomically)
//!        - profile qualification
//!        - non-empty valid scopes
//!        - duplicate (profile, definition) key rejection
//!        - fingerprint COMPUTED from the validated fields
//!        - immutable-identity check against every prior activation
//!   -> ActivatedSchema (private fields, no public constructor)
//!   -> StateStore::from_activated_schema
//! ```
//!
//! Three properties are structural rather than documented:
//!
//! 1. **Provenance.** [`ActivatedDefinition`] and [`ActivatedSchema`] have
//!    private fields and no public constructor, so no value of either type
//!    can exist that did not come out of [`DefinitionIdentityRegistry::activate`].
//!    `spark_core` is a separate crate from every consumer, so this is
//!    enforced by the compiler, not by convention.
//! 2. **Unforgeable identity.** A fingerprint is never accepted from a
//!    caller; it is derived by [`definition_fingerprint`] from the
//!    validated profile, ID, kind, value constraint, authority, implied
//!    write class, and sorted valid scopes. `Digest::ZERO` cannot be
//!    smuggled in because there is no field to put it in.
//! 3. **Immutability after activation.** [`ActivatedSchema`] exposes only
//!    shared references to its entries and has no mutation method, and
//!    [`crate::state::StateStore`] has no schema-mutation method either.
//!
//! Atomic candidate validation is preserved from the independently closed
//! B-04 work: a batch that fails any check leaves the registry byte-for-byte
//! unchanged and a corrected retry succeeds cleanly.

use crate::authority::Authority;
use crate::hash::{CanonicalEncoder, Digest};
use crate::id::{CanonicalTag, DefinitionId, ProfileId};
use crate::scope::ScopeKind;
use crate::value::ValueConstraint;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// A definition's `kind` discriminator (`CONTROLLING_BLUEPRINT_v0.2.md`
/// §12.1), as the canonical kernel sees it.
///
/// The kind taxonomy itself is profile vocabulary (ADR-0004's taxonomy
/// rule), so the kernel does not hard-code a catalog — but a kind *is*
/// immutable definition identity, so it must be bounded and
/// domain-separated. Built-in and custom kinds are separated by an
/// explicit flag rather than by a string prefix, so a profile-declared
/// custom kind literally named `trigger` can never collide with the
/// built-in `trigger`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefinitionKindTag {
    custom: bool,
    tag: CanonicalTag,
}

impl DefinitionKindTag {
    /// A kind drawn from the engine's own baseline vocabulary.
    pub fn builtin(tag: CanonicalTag) -> Self {
        Self { custom: false, tag }
    }

    /// A kind declared as profile vocabulary. Bounded by
    /// [`CanonicalTag`]'s construction rules, so an oversized custom kind
    /// is rejected before it can become canonical identity.
    pub fn custom(tag: CanonicalTag) -> Self {
        Self { custom: true, tag }
    }

    pub fn is_custom(&self) -> bool {
        self.custom
    }

    pub fn tag(&self) -> &CanonicalTag {
        &self.tag
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str(if self.custom {
            "definition_kind.custom"
        } else {
            "definition_kind.builtin"
        });
        self.tag.canonicalize(enc);
    }
}

impl fmt::Display for DefinitionKindTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.custom {
            write!(f, "custom:{}", self.tag)
        } else {
            write!(f, "{}", self.tag)
        }
    }
}

/// The untrusted, caller-supplied description of one definition offered
/// for activation.
///
/// Note what is **absent**: there is no fingerprint field. Identity is
/// computed by the registry from the fields below, so a caller cannot
/// assert an identity it did not earn. Every field here is already a
/// validated bounded type ([`DefinitionId`] is namespaced,
/// [`DefinitionKindTag`] is bounded, [`ValueConstraint`] is coherent), so
/// there is no unbounded or incoherent value for the registry to have to
/// re-check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionDeclaration {
    pub profile_id: ProfileId,
    pub definition_id: DefinitionId,
    pub kind: DefinitionKindTag,
    pub authority: Authority,
    pub value_constraint: ValueConstraint,
    pub valid_scopes: BTreeSet<ScopeKind>,
}

/// The immutable identity fingerprint of a declaration (ADR-0004 "Content
/// identity"): profile ID, definition ID, kind, value constraint (which
/// implies value type), authority mode, the write class that authority
/// implies, and the sorted valid scopes.
///
/// This is a pure function of already-validated fields and is public so a
/// reviewer or a profile layer can recompute it independently. Being able
/// to *compute* a fingerprint grants no authority: only
/// [`DefinitionIdentityRegistry::activate`] can mint an
/// [`ActivatedDefinition`], and it always recomputes the fingerprint
/// itself.
pub fn definition_fingerprint(declaration: &DefinitionDeclaration) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("definition_fingerprint");
    declaration.profile_id.canonicalize(&mut enc);
    declaration.definition_id.canonicalize(&mut enc);
    declaration.kind.canonicalize(&mut enc);
    declaration.value_constraint.canonicalize(&mut enc);
    declaration.authority.canonicalize(&mut enc);
    enc.push_str(declaration.authority.implied_write_class().tag());
    // `valid_scopes` is a BTreeSet, so iteration is already in a stable
    // total order; the canonical tags are pushed length-prefixed, so two
    // different scope sets can never alias.
    enc.push_u64(declaration.valid_scopes.len() as u64);
    for scope in &declaration.valid_scopes {
        enc.push_str(&scope.canonical_tag());
    }
    enc.finish()
}

/// Rejects one declaration in an activation batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaActivationError {
    /// The declaration names a profile other than the one being activated.
    ForeignProfile {
        definition_id: DefinitionId,
        activating_profile: ProfileId,
        declared_profile: ProfileId,
    },
    /// A definition with no valid scope can never be written and is
    /// therefore a malformed declaration, not an inert one.
    NoValidScope { definition_id: DefinitionId },
    /// Two declarations in the same batch claim the same
    /// `(profile_id, definition_id)`. Insertion order must never pick a
    /// winner, so the whole batch is rejected.
    DuplicateDefinitionKey {
        profile_id: ProfileId,
        definition_id: DefinitionId,
    },
    /// The `(profile_id, definition_id)` was previously activated with a
    /// different immutable identity. Per ADR-0002/ADR-0004 an authority,
    /// value-type, or scope-compatibility change requires a new
    /// definition ID, an explicit migration, an authority ADR, and
    /// operator approval — never a redeclaration under the same ID.
    IdentityConflict {
        profile_id: ProfileId,
        definition_id: DefinitionId,
        previous_fingerprint: Digest,
        attempted_fingerprint: Digest,
    },
}

impl fmt::Display for SchemaActivationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemaActivationError::ForeignProfile {
                definition_id,
                activating_profile,
                declared_profile,
            } => write!(
                f,
                "definition '{definition_id}' declares profile '{declared_profile}' but was offered for activation in profile '{activating_profile}'"
            ),
            SchemaActivationError::NoValidScope { definition_id } => {
                write!(f, "definition '{definition_id}' declares no valid scope")
            }
            SchemaActivationError::DuplicateDefinitionKey {
                profile_id,
                definition_id,
            } => write!(
                f,
                "definition '{definition_id}' is declared more than once in profile '{profile_id}''s activation batch"
            ),
            SchemaActivationError::IdentityConflict {
                profile_id,
                definition_id,
                previous_fingerprint,
                attempted_fingerprint,
            } => write!(
                f,
                "definition '{definition_id}' in profile '{profile_id}' was activated with fingerprint {previous_fingerprint} and cannot be reactivated with fingerprint {attempted_fingerprint} under the same ID; an identity change (kind, value constraint, authority, or valid scopes) requires a new definition ID, an explicit migration, and an ADR"
            ),
        }
    }
}

impl std::error::Error for SchemaActivationError {}

/// One definition whose identity was computed and accepted by
/// [`DefinitionIdentityRegistry::activate`].
///
/// Every field is private and there is no public constructor, so a value
/// of this type is itself the proof that the activation ceremony ran.
///
/// ```compile_fail
/// use spark_core::activation::ActivatedDefinition;
/// use spark_core::authority::Authority;
/// use spark_core::hash::Digest;
/// // External code cannot fabricate activated authority: the fields are
/// // private and there is no constructor.
/// let forged = ActivatedDefinition {
///     authority: Authority::Derived,
///     fingerprint: Digest::ZERO,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedDefinition {
    profile_id: ProfileId,
    definition_id: DefinitionId,
    kind: DefinitionKindTag,
    authority: Authority,
    value_constraint: ValueConstraint,
    valid_scopes: BTreeSet<ScopeKind>,
    fingerprint: Digest,
}

impl ActivatedDefinition {
    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    pub fn definition_id(&self) -> &DefinitionId {
        &self.definition_id
    }

    pub fn kind(&self) -> &DefinitionKindTag {
        &self.kind
    }

    pub fn authority(&self) -> Authority {
        self.authority
    }

    pub fn value_constraint(&self) -> &ValueConstraint {
        &self.value_constraint
    }

    pub fn valid_scopes(&self) -> &BTreeSet<ScopeKind> {
        &self.valid_scopes
    }

    /// The immutable identity fingerprint the registry computed for this
    /// definition. It was never accepted from a caller.
    pub fn fingerprint(&self) -> &Digest {
        &self.fingerprint
    }
}

/// A complete, immutable, profile-qualified activated definition set: the
/// only thing a [`crate::state::StateStore`] can be built from.
///
/// Private fields, no public constructor, and no mutation method — the
/// activation ceremony is the sole producer, and nothing can alter the
/// result afterward.
///
/// ```compile_fail
/// use spark_core::activation::ActivatedSchema;
/// use spark_core::id::ProfileId;
/// let forged = ActivatedSchema {
///     profile_id: ProfileId::new("game-world").unwrap(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedSchema {
    profile_id: ProfileId,
    entries: BTreeMap<DefinitionId, ActivatedDefinition>,
    activation_hash: Digest,
}

impl ActivatedSchema {
    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    pub fn get(&self, definition_id: &DefinitionId) -> Option<&ActivatedDefinition> {
        self.entries.get(definition_id)
    }

    /// Every activated definition, in stable ascending definition-ID
    /// order. Shared references only: an activated schema cannot be
    /// edited after activation.
    pub fn definitions(&self) -> impl Iterator<Item = &ActivatedDefinition> {
        self.entries.values()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// A deterministic content hash over the whole activated set: the
    /// profile plus every definition's registry-computed fingerprint, in
    /// stable order. Two activations of the same validated definitions
    /// produce the same activation hash regardless of declaration order.
    pub fn activation_hash(&self) -> &Digest {
        &self.activation_hash
    }
}

/// The immutable, profile-qualified identity of every
/// `(profile_id, definition_id)` ever activated through this registry.
///
/// Identity is the definition's full content fingerprint — kind, value
/// constraint, authority *and its implied write class*, and valid scopes —
/// not authority alone, and the registry is profile-qualified so the same
/// definition ID in two trust domains cannot contaminate the other's
/// history (ADR-0004 profile isolation).
#[derive(Debug, Default, Clone)]
pub struct DefinitionIdentityRegistry {
    fingerprints: BTreeMap<(ProfileId, DefinitionId), Digest>,
}

impl DefinitionIdentityRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fingerprint_of(
        &self,
        profile_id: &ProfileId,
        definition_id: &DefinitionId,
    ) -> Option<&Digest> {
        self.fingerprints
            .get(&(profile_id.clone(), definition_id.clone()))
    }

    pub fn len(&self) -> usize {
        self.fingerprints.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fingerprints.is_empty()
    }

    /// Validates a whole activation batch and, only if every declaration
    /// passes, commits its identities and mints the [`ActivatedSchema`].
    ///
    /// Validation is **pure with respect to this registry until it
    /// succeeds**: candidate fingerprints are compared against a
    /// read-only view, and the registry is mutated exactly once, after
    /// every check has passed. A batch that fails partway leaves the
    /// registry unchanged, so a corrected retry always succeeds cleanly.
    ///
    /// Every error found is reported, not just the first, so an operator
    /// fixing a profile sees the whole problem at once.
    pub fn activate(
        &mut self,
        profile_id: &ProfileId,
        declarations: &[DefinitionDeclaration],
    ) -> Result<ActivatedSchema, Vec<SchemaActivationError>> {
        let mut errors: Vec<SchemaActivationError> = Vec::new();
        let mut entries: BTreeMap<DefinitionId, ActivatedDefinition> = BTreeMap::new();
        let mut seen: BTreeSet<DefinitionId> = BTreeSet::new();

        for declaration in declarations {
            if !seen.insert(declaration.definition_id.clone()) {
                errors.push(SchemaActivationError::DuplicateDefinitionKey {
                    profile_id: declaration.profile_id.clone(),
                    definition_id: declaration.definition_id.clone(),
                });
                continue;
            }

            if &declaration.profile_id != profile_id {
                errors.push(SchemaActivationError::ForeignProfile {
                    definition_id: declaration.definition_id.clone(),
                    activating_profile: profile_id.clone(),
                    declared_profile: declaration.profile_id.clone(),
                });
                continue;
            }

            if declaration.valid_scopes.is_empty() {
                errors.push(SchemaActivationError::NoValidScope {
                    definition_id: declaration.definition_id.clone(),
                });
                continue;
            }

            let fingerprint = definition_fingerprint(declaration);
            if let Some(previous) =
                self.fingerprint_of(&declaration.profile_id, &declaration.definition_id)
            {
                if previous != &fingerprint {
                    errors.push(SchemaActivationError::IdentityConflict {
                        profile_id: declaration.profile_id.clone(),
                        definition_id: declaration.definition_id.clone(),
                        previous_fingerprint: previous.clone(),
                        attempted_fingerprint: fingerprint,
                    });
                    continue;
                }
            }

            entries.insert(
                declaration.definition_id.clone(),
                ActivatedDefinition {
                    profile_id: declaration.profile_id.clone(),
                    definition_id: declaration.definition_id.clone(),
                    kind: declaration.kind.clone(),
                    authority: declaration.authority,
                    value_constraint: declaration.value_constraint.clone(),
                    valid_scopes: declaration.valid_scopes.clone(),
                    fingerprint,
                },
            );
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        let mut enc = CanonicalEncoder::new();
        enc.push_str("activated_schema");
        profile_id.canonicalize(&mut enc);
        enc.push_u64(entries.len() as u64);
        for definition in entries.values() {
            let mut inner = CanonicalEncoder::new();
            definition.definition_id.canonicalize(&mut inner);
            inner.push_digest(&definition.fingerprint);
            enc.push_block(&inner);
        }
        let activation_hash = enc.finish();

        for definition in entries.values() {
            self.fingerprints.insert(
                (
                    definition.profile_id.clone(),
                    definition.definition_id.clone(),
                ),
                definition.fingerprint.clone(),
            );
        }

        Ok(ActivatedSchema {
            profile_id: profile_id.clone(),
            entries,
            activation_hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::FixedPoint;

    fn profile() -> ProfileId {
        ProfileId::new("game-world").unwrap()
    }

    fn tag(s: &str) -> CanonicalTag {
        CanonicalTag::new(s).unwrap()
    }

    fn declaration(id: &str, authority: Authority) -> DefinitionDeclaration {
        DefinitionDeclaration {
            profile_id: profile(),
            definition_id: DefinitionId::new(id).unwrap(),
            kind: DefinitionKindTag::builtin(tag("trait")),
            authority,
            value_constraint: ValueConstraint::fixed(
                FixedPoint::ZERO,
                FixedPoint::from_integer(1).unwrap(),
            )
            .unwrap(),
            valid_scopes: BTreeSet::from([ScopeKind::Actor]),
        }
    }

    #[test]
    fn activation_computes_the_fingerprint_itself() {
        let mut registry = DefinitionIdentityRegistry::new();
        let decl = declaration("trait.curiosity", Authority::SparkOwned);
        let schema = registry
            .activate(&profile(), std::slice::from_ref(&decl))
            .unwrap();
        let activated = schema
            .get(&DefinitionId::new("trait.curiosity").unwrap())
            .unwrap();
        assert_eq!(activated.fingerprint(), &definition_fingerprint(&decl));
        assert_ne!(activated.fingerprint(), &Digest::ZERO);
    }

    #[test]
    fn duplicate_definition_keys_reject_the_whole_batch() {
        let mut registry = DefinitionIdentityRegistry::new();
        let errors = registry
            .activate(
                &profile(),
                &[
                    declaration("trait.curiosity", Authority::SparkOwned),
                    declaration("trait.curiosity", Authority::HostOwned),
                ],
            )
            .unwrap_err();
        assert!(matches!(
            errors[0],
            SchemaActivationError::DuplicateDefinitionKey { .. }
        ));
        assert!(registry.is_empty(), "a failed batch must commit nothing");
    }

    /// Duplicate rejection must not depend on which declaration arrives
    /// first.
    #[test]
    fn duplicate_definition_keys_reject_in_either_order() {
        let mut forward = DefinitionIdentityRegistry::new();
        let mut reversed = DefinitionIdentityRegistry::new();
        let a = declaration("trait.curiosity", Authority::SparkOwned);
        let b = declaration("trait.curiosity", Authority::HostOwned);
        assert!(forward
            .activate(&profile(), &[a.clone(), b.clone()])
            .is_err());
        assert!(reversed.activate(&profile(), &[b, a]).is_err());
        assert!(forward.is_empty());
        assert!(reversed.is_empty());
    }

    #[test]
    fn identity_change_under_the_same_id_rejects() {
        let mut registry = DefinitionIdentityRegistry::new();
        registry
            .activate(
                &profile(),
                &[declaration("state.food.availability", Authority::HostOwned)],
            )
            .unwrap();
        let errors = registry
            .activate(
                &profile(),
                &[declaration(
                    "state.food.availability",
                    Authority::SparkOwned,
                )],
            )
            .unwrap_err();
        assert!(matches!(
            errors[0],
            SchemaActivationError::IdentityConflict { .. }
        ));
    }

    #[test]
    fn reactivating_the_same_identity_is_accepted() {
        let mut registry = DefinitionIdentityRegistry::new();
        let decl = declaration("trait.curiosity", Authority::SparkOwned);
        let first = registry
            .activate(&profile(), std::slice::from_ref(&decl))
            .unwrap();
        let second = registry.activate(&profile(), &[decl]).unwrap();
        assert_eq!(first.activation_hash(), second.activation_hash());
    }

    #[test]
    fn failed_batch_leaves_registry_unchanged_and_retry_succeeds() {
        let mut registry = DefinitionIdentityRegistry::new();
        let good = declaration("trait.curiosity", Authority::SparkOwned);
        let mut bad = declaration("trait.skepticism", Authority::SparkOwned);
        bad.valid_scopes = BTreeSet::new();

        let errors = registry
            .activate(&profile(), &[good.clone(), bad])
            .unwrap_err();
        assert!(matches!(
            errors[0],
            SchemaActivationError::NoValidScope { .. }
        ));
        assert!(registry
            .fingerprint_of(&profile(), &good.definition_id)
            .is_none());

        let fixed = declaration("trait.skepticism", Authority::SparkOwned);
        assert!(registry.activate(&profile(), &[good, fixed]).is_ok());
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn foreign_profile_declaration_rejects() {
        let mut registry = DefinitionIdentityRegistry::new();
        let mut decl = declaration("trait.curiosity", Authority::SparkOwned);
        decl.profile_id = ProfileId::new("mci-social").unwrap();
        let errors = registry.activate(&profile(), &[decl]).unwrap_err();
        assert!(matches!(
            errors[0],
            SchemaActivationError::ForeignProfile { .. }
        ));
    }

    #[test]
    fn same_definition_id_in_independent_profiles_does_not_conflict() {
        let mut registry = DefinitionIdentityRegistry::new();
        registry
            .activate(
                &profile(),
                &[declaration("trait.curiosity", Authority::SparkOwned)],
            )
            .unwrap();

        let other = ProfileId::new("mci-social").unwrap();
        let mut decl = declaration("trait.curiosity", Authority::HostOwned);
        decl.profile_id = other.clone();
        assert!(registry.activate(&other, &[decl]).is_ok());
    }

    /// B-04 regression: a built-in kind and a custom kind spelled the same
    /// way are different identities.
    #[test]
    fn builtin_and_custom_kinds_with_the_same_spelling_are_distinct() {
        let mut builtin = declaration("trigger.drought", Authority::SparkOwned);
        builtin.kind = DefinitionKindTag::builtin(tag("trigger"));
        let mut custom = declaration("trigger.drought", Authority::SparkOwned);
        custom.kind = DefinitionKindTag::custom(tag("trigger"));
        assert_ne!(
            definition_fingerprint(&builtin),
            definition_fingerprint(&custom)
        );
    }

    /// Activation identity must not depend on declaration order.
    #[test]
    fn activation_hash_is_declaration_order_independent() {
        let a = declaration("trait.curiosity", Authority::SparkOwned);
        let b = declaration("trait.skepticism", Authority::SparkOwned);

        let mut forward = DefinitionIdentityRegistry::new();
        let mut reversed = DefinitionIdentityRegistry::new();
        let f = forward
            .activate(&profile(), &[a.clone(), b.clone()])
            .unwrap();
        let r = reversed.activate(&profile(), &[b, a]).unwrap();
        assert_eq!(f.activation_hash(), r.activation_hash());
    }

    /// Every immutable identity field must reach the fingerprint.
    #[test]
    fn every_immutable_identity_field_changes_the_fingerprint() {
        let base = declaration("trait.curiosity", Authority::SparkOwned);
        let baseline = definition_fingerprint(&base);

        let mut other_authority = base.clone();
        other_authority.authority = Authority::HostOwned;
        assert_ne!(definition_fingerprint(&other_authority), baseline);

        let mut other_kind = base.clone();
        other_kind.kind = DefinitionKindTag::builtin(tag("goal"));
        assert_ne!(definition_fingerprint(&other_kind), baseline);

        let mut other_constraint = base.clone();
        other_constraint.value_constraint = ValueConstraint::boolean();
        assert_ne!(definition_fingerprint(&other_constraint), baseline);

        let mut other_scopes = base.clone();
        other_scopes.valid_scopes = BTreeSet::from([ScopeKind::Actor, ScopeKind::Household]);
        assert_ne!(definition_fingerprint(&other_scopes), baseline);

        let mut other_profile = base.clone();
        other_profile.profile_id = ProfileId::new("mci-social").unwrap();
        assert_ne!(definition_fingerprint(&other_profile), baseline);

        let mut other_id = base.clone();
        other_id.definition_id = DefinitionId::new("trait.skepticism").unwrap();
        assert_ne!(definition_fingerprint(&other_id), baseline);
    }
}
