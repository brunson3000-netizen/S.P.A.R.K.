//! Authority modes and their implied write classes.
//!
//! ADR-0002 requires that every state definition declare exactly one
//! authority mode and that the mode imply a fixed write class. Immutable
//! *identity* enforcement (that a definition ID's authority can never
//! change once declared) is owned by the profile-qualified definition
//! identity registry in `spark-profile` (which hashes authority as part of
//! the full definition fingerprint) and, at the state layer, by the
//! immutable schema [`crate::state::StateStore`] is built from. This
//! module intentionally owns only the two small fixed vocabularies
//! (`Authority`, `WriteClass`); a single overlapping "authority catalog"
//! duplicating identity enforcement in two places was exactly the kind of
//! structural confusion the Phase-1 independent review flagged, so Phase 1
//! keeps exactly one enforcement point per invariant instead.

use crate::hash::CanonicalEncoder;

/// Who is canonical for a definition's value
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §7, ADR-0002).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Authority {
    /// The host is canonical; S.P.A.R.K. may only observe/ingest values
    /// the host reports.
    HostOwned,
    /// S.P.A.R.K. is canonical; only validated engine effects (or an
    /// explicitly authorized migration) may change the value.
    SparkOwned,
    /// Deterministically computed from declared authoritative inputs;
    /// only the evaluator may write it.
    Derived,
}

/// The write class an [`Authority`] implies. This mapping is fixed by
/// ADR-0002 and is not configurable per definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WriteClass {
    HostIngressOnly,
    SparkEffectOrExplicitMigration,
    EvaluatorOnly,
}

impl Authority {
    pub fn implied_write_class(&self) -> WriteClass {
        match self {
            Authority::HostOwned => WriteClass::HostIngressOnly,
            Authority::SparkOwned => WriteClass::SparkEffectOrExplicitMigration,
            Authority::Derived => WriteClass::EvaluatorOnly,
        }
    }

    pub fn tag(&self) -> &'static str {
        match self {
            Authority::HostOwned => "host_owned",
            Authority::SparkOwned => "spark_owned",
            Authority::Derived => "derived",
        }
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str(self.tag());
    }
}

impl WriteClass {
    pub fn tag(&self) -> &'static str {
        match self {
            WriteClass::HostIngressOnly => "host_ingress_only",
            WriteClass::SparkEffectOrExplicitMigration => "spark_effect_or_explicit_migration",
            WriteClass::EvaluatorOnly => "evaluator_only",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authority_implies_fixed_write_class() {
        assert_eq!(
            Authority::HostOwned.implied_write_class(),
            WriteClass::HostIngressOnly
        );
        assert_eq!(
            Authority::SparkOwned.implied_write_class(),
            WriteClass::SparkEffectOrExplicitMigration
        );
        assert_eq!(
            Authority::Derived.implied_write_class(),
            WriteClass::EvaluatorOnly
        );
    }
}
