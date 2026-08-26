//! Authority modes and their implied write classes.
//!
//! ADR-0002 requires that every state definition declare exactly one
//! authority mode and that the mode imply a fixed write class. This module
//! owns the two small fixed vocabularies (`Authority`, `WriteClass`) and
//! **nothing else** — deliberately, because it sits below the trust
//! boundary.
//!
//! Enforcement lives in exactly one place, one level up: `spark-engine`'s
//! activation door hashes authority *and* its implied write class into
//! every definition fingerprint (so a definition ID's authority can never
//! change once activated), and `spark-engine`'s state store hard-codes one
//! authority per write path. A single overlapping "authority catalog"
//! duplicating identity enforcement in two places was exactly the kind of
//! structural confusion the Phase-1 independent reviews flagged, so there
//! is one enforcement point per invariant and the kernel keeps none.

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
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
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
