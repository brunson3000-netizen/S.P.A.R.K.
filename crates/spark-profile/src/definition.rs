//! `DefinitionSpec`: the logical shape of one profile definition
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1). The exact on-disk syntax is
//! provisional (ADR-0004); this struct is the deterministic,
//! format-independent logical representation used for validation and
//! content hashing.

use spark_core::authority::Authority;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::DefinitionId;
use spark_core::scope::ScopeKind;
use spark_core::value::ValueType;

/// `kind` in the common definition schema (`CONTROLLING_BLUEPRINT_v0.2.md`
/// §12.1: "trigger/state/trait/appraisal/goal/behavior/etc"). The
/// baseline variants mirror the frozen causal grammar's primitive roles;
/// `Custom` keeps the taxonomy open as profile vocabulary per ADR-0004's
/// taxonomy rule, without requiring a Rust change to add a new kind tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionKind {
    Trigger,
    StateDefinition,
    Trait,
    Appraisal,
    Goal,
    Behavior,
    Relationship,
    Custom(String),
}

impl DefinitionKind {
    fn tag(&self) -> &str {
        match self {
            DefinitionKind::Trigger => "trigger",
            DefinitionKind::StateDefinition => "state_definition",
            DefinitionKind::Trait => "trait",
            DefinitionKind::Appraisal => "appraisal",
            DefinitionKind::Goal => "goal",
            DefinitionKind::Behavior => "behavior",
            DefinitionKind::Relationship => "relationship",
            DefinitionKind::Custom(tag) => tag,
        }
    }

    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str(self.tag());
    }
}

/// `behavioral_leverage` metadata (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehavioralLeverage {
    High,
    Medium,
    Low,
}

impl BehavioralLeverage {
    fn tag(&self) -> &'static str {
        match self {
            BehavioralLeverage::High => "high",
            BehavioralLeverage::Medium => "medium",
            BehavioralLeverage::Low => "low",
        }
    }
}

/// The logical, format-independent shape of one profile definition
/// (common fields per `CONTROLLING_BLUEPRINT_v0.2.md` §12.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionSpec {
    pub id: DefinitionId,
    pub kind: DefinitionKind,
    pub domain: Option<String>,
    pub layer: Option<String>,
    pub value_type: ValueType,
    pub authority: Authority,
    pub valid_scopes: Vec<ScopeKind>,
    pub enabled: bool,
    pub version: u32,
    pub description: String,
    pub behavioral_leverage: Option<BehavioralLeverage>,
}

impl DefinitionSpec {
    /// Full-content canonical encoding, used by
    /// [`crate::manifest::ProfileManifest::manifest_content_hash`]. This
    /// covers every field, including ones that are not part of immutable
    /// identity (description, enabled, version label), so that *any*
    /// content change - not only an identity change - produces a
    /// different manifest hash.
    pub(crate) fn canonicalize_full(&self, enc: &mut CanonicalEncoder) {
        self.id.canonicalize(enc);
        self.kind.canonicalize(enc);
        enc.push_bool(self.domain.is_some());
        enc.push_str(self.domain.as_deref().unwrap_or(""));
        enc.push_bool(self.layer.is_some());
        enc.push_str(self.layer.as_deref().unwrap_or(""));
        self.value_type.canonicalize(enc);
        self.authority.canonicalize(enc);
        let mut scopes: Vec<&'static str> = self.valid_scopes.iter().map(|s| s.tag()).collect();
        scopes.sort_unstable();
        enc.push_u64(scopes.len() as u64);
        for s in &scopes {
            enc.push_str(s);
        }
        enc.push_bool(self.enabled);
        enc.push_u32(self.version);
        enc.push_str(&self.description);
        enc.push_bool(self.behavioral_leverage.is_some());
        enc.push_str(self.behavioral_leverage.map(|l| l.tag()).unwrap_or(""));
    }

    /// The immutable-identity subset of this definition's content,
    /// hashed independently of mutable/descriptive fields
    /// (ADR-0004 "Content identity"): definition ID, kind, value type,
    /// authority mode, implied write class, and sorted valid scopes.
    /// Two definitions with the same fingerprint are the same identity
    /// for save-continuation purposes even if their description or
    /// enabled/disabled status later changes.
    pub fn definition_fingerprint(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("definition_fingerprint");
        self.id.canonicalize(&mut enc);
        self.kind.canonicalize(&mut enc);
        self.value_type.canonicalize(&mut enc);
        self.authority.canonicalize(&mut enc);
        enc.push_str(self.authority.implied_write_class().tag());
        let mut scopes: Vec<&'static str> = self.valid_scopes.iter().map(|s| s.tag()).collect();
        scopes.sort_unstable();
        enc.push_u64(scopes.len() as u64);
        for s in &scopes {
            enc.push_str(s);
        }
        enc.finish()
    }
}
