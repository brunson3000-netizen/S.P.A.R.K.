//! Canonical value types and declarative value constraints.
//!
//! The blueprint requires more than booleans: bounded integers/fixed-point
//! intensities, categorical affiliations, and typed references
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §3.2, §12.1 `value_type`). Determinism
//! constitution item 4 requires integer/fixed-point canonical
//! representations rather than floating point
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §28). Phase 1 defines only the value
//! shapes needed by the skeleton; richer value kinds (sets, timed
//! decay/recovery metadata) are Phase 2+ rule-runtime concerns.
//!
//! Per the Phase-1 correction brief (M-03), `FixedPoint`'s raw integer is
//! not publicly forgeable and construction from an integer multiplier is a
//! checked operation that returns a typed error on overflow rather than
//! panicking (`debug_assert`/panic) or silently wrapping in release.
//! [`ValueConstraint`] gives every definition a declarative bound so
//! `StateStore` can reject out-of-bounds/mistyped writes before they ever
//! become canonical state.

use crate::hash::CanonicalEncoder;
use crate::id::DefinitionId;

/// Fixed-point scale shared by every `Fixed` value: a raw value of
/// `FIXED_SCALE` represents `1.0`. Using one fixed scale system-wide keeps
/// arithmetic between canonical values deterministic and comparable
/// without a floating-point conversion step.
pub const FIXED_SCALE: i64 = 1_000_000;

/// Rejects a [`FixedPoint`] construction that would overflow `i64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedPointOverflow {
    pub attempted_integer: i64,
}

impl std::fmt::Display for FixedPointOverflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fixed-point construction from integer {} overflows i64 at scale {FIXED_SCALE}",
            self.attempted_integer
        )
    }
}

impl std::error::Error for FixedPointOverflow {}

/// A fixed-point number stored as an integer numerator over
/// [`FIXED_SCALE`]. The raw numerator is intentionally private: Phase 1
/// has no invariant beyond "a valid `i64`", but hiding the field keeps the
/// type from being casually reconstructed from an unchecked arithmetic
/// result elsewhere and keeps checked construction the only sanctioned
/// path for integer-derived values (M-03).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedPoint(i64);

impl FixedPoint {
    pub const ZERO: FixedPoint = FixedPoint(0);

    /// Constructs a `FixedPoint` directly from an already-scaled raw
    /// numerator (e.g. a value decoded from canonical storage). No
    /// arithmetic is performed, so this cannot overflow.
    pub fn from_raw(raw: i64) -> Self {
        FixedPoint(raw)
    }

    /// The raw scaled numerator.
    pub fn raw(&self) -> i64 {
        self.0
    }

    /// Constructs a `FixedPoint` representing the integer `v` (i.e.
    /// `v * FIXED_SCALE`), rejecting overflow instead of panicking or
    /// wrapping.
    pub fn from_integer(v: i64) -> Result<Self, FixedPointOverflow> {
        v.checked_mul(FIXED_SCALE)
            .map(FixedPoint)
            .ok_or(FixedPointOverflow {
                attempted_integer: v,
            })
    }

    pub fn clamp(self, min: FixedPoint, max: FixedPoint) -> FixedPoint {
        FixedPoint(self.0.clamp(min.0, max.0))
    }
}

/// A canonical, deterministic value. `value_type` in a profile definition
/// declares which variant a `StateCell` may hold
/// (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CanonicalValue {
    Bool(bool),
    Int(i64),
    Fixed(FixedPoint),
    Categorical(String),
    Reference(DefinitionId),
}

impl CanonicalValue {
    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            CanonicalValue::Bool(b) => {
                enc.push_str("bool");
                enc.push_bool(*b);
            }
            CanonicalValue::Int(i) => {
                enc.push_str("int");
                enc.push_i64(*i);
            }
            CanonicalValue::Fixed(f) => {
                enc.push_str("fixed");
                enc.push_i64(f.0);
            }
            CanonicalValue::Categorical(s) => {
                enc.push_str("categorical");
                enc.push_str(s);
            }
            CanonicalValue::Reference(id) => {
                enc.push_str("reference");
                id.canonicalize(enc);
            }
        }
    }

    /// The `value_type` tag this value belongs to, used to validate a
    /// `StateCell` write against its definition's declared `value_type`.
    pub fn type_tag(&self) -> ValueType {
        match self {
            CanonicalValue::Bool(_) => ValueType::Bool,
            CanonicalValue::Int(_) => ValueType::Int,
            CanonicalValue::Fixed(_) => ValueType::Fixed,
            CanonicalValue::Categorical(_) => ValueType::Categorical,
            CanonicalValue::Reference(_) => ValueType::Reference,
        }
    }
}

/// The declared shape of a definition's value, independent of any
/// particular instance (`CONTROLLING_BLUEPRINT_v0.2.md` §12.1
/// `value_type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueType {
    Bool,
    Int,
    Fixed,
    Categorical,
    Reference,
}

impl ValueType {
    pub fn tag(&self) -> &'static str {
        match self {
            ValueType::Bool => "bool",
            ValueType::Int => "int",
            ValueType::Fixed => "fixed",
            ValueType::Categorical => "categorical",
            ValueType::Reference => "reference",
        }
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str(self.tag());
    }
}

/// A declarative bound on a definition's canonical value
/// (Phase-1 correction brief M-03: "Add explicit declarative
/// bounds/constraints sufficient for StateStore/schema write
/// validation"). Every variant corresponds to exactly one [`ValueType`],
/// so a constraint unambiguously implies its declared type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueConstraint {
    Bool,
    Int { min: i64, max: i64 },
    Fixed { min: FixedPoint, max: FixedPoint },
    Categorical { max_len: usize },
    Reference,
}

impl ValueConstraint {
    pub fn value_type(&self) -> ValueType {
        match self {
            ValueConstraint::Bool => ValueType::Bool,
            ValueConstraint::Int { .. } => ValueType::Int,
            ValueConstraint::Fixed { .. } => ValueType::Fixed,
            ValueConstraint::Categorical { .. } => ValueType::Categorical,
            ValueConstraint::Reference => ValueType::Reference,
        }
    }

    /// Whether `value` both matches this constraint's declared type and
    /// falls within its declared bounds.
    pub fn accepts(&self, value: &CanonicalValue) -> bool {
        match (self, value) {
            (ValueConstraint::Bool, CanonicalValue::Bool(_)) => true,
            (ValueConstraint::Int { min, max }, CanonicalValue::Int(v)) => v >= min && v <= max,
            (ValueConstraint::Fixed { min, max }, CanonicalValue::Fixed(v)) => v >= min && v <= max,
            (ValueConstraint::Categorical { max_len }, CanonicalValue::Categorical(s)) => {
                s.chars().count() <= *max_len
            }
            (ValueConstraint::Reference, CanonicalValue::Reference(_)) => true,
            _ => false,
        }
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            ValueConstraint::Bool => {
                enc.push_str("constraint.bool");
            }
            ValueConstraint::Int { min, max } => {
                enc.push_str("constraint.int");
                enc.push_i64(*min);
                enc.push_i64(*max);
            }
            ValueConstraint::Fixed { min, max } => {
                enc.push_str("constraint.fixed");
                enc.push_i64(min.raw());
                enc.push_i64(max.raw());
            }
            ValueConstraint::Categorical { max_len } => {
                enc.push_str("constraint.categorical");
                enc.push_u64(*max_len as u64);
            }
            ValueConstraint::Reference => {
                enc.push_str("constraint.reference");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_point_integer_round_trip() {
        assert_eq!(FixedPoint::from_integer(2).unwrap().raw(), 2 * FIXED_SCALE);
    }

    #[test]
    fn fixed_point_integer_overflow_is_rejected_not_wrapped() {
        let err = FixedPoint::from_integer(i64::MAX).unwrap_err();
        assert_eq!(
            err,
            FixedPointOverflow {
                attempted_integer: i64::MAX
            }
        );
    }

    #[test]
    fn type_tag_matches_declared_value_type() {
        assert_eq!(CanonicalValue::Bool(true).type_tag(), ValueType::Bool);
        assert_eq!(
            CanonicalValue::Fixed(FixedPoint::ZERO).type_tag(),
            ValueType::Fixed
        );
    }

    #[test]
    fn int_constraint_rejects_out_of_bounds() {
        let c = ValueConstraint::Int { min: 0, max: 10 };
        assert!(c.accepts(&CanonicalValue::Int(5)));
        assert!(!c.accepts(&CanonicalValue::Int(11)));
        assert!(!c.accepts(&CanonicalValue::Int(-1)));
    }

    #[test]
    fn constraint_rejects_wrong_type() {
        let c = ValueConstraint::Bool;
        assert!(!c.accepts(&CanonicalValue::Int(0)));
    }

    #[test]
    fn categorical_constraint_bounds_length() {
        let c = ValueConstraint::Categorical { max_len: 3 };
        assert!(c.accepts(&CanonicalValue::Categorical("abc".to_string())));
        assert!(!c.accepts(&CanonicalValue::Categorical("abcd".to_string())));
    }
}
