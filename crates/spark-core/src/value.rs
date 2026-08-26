//! Canonical value types.
//!
//! The blueprint requires more than booleans: bounded integers/fixed-point
//! intensities, categorical affiliations, and typed references
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §3.2, §12.1 `value_type`). Determinism
//! constitution item 4 requires integer/fixed-point canonical
//! representations rather than floating point
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §28). Phase 1 defines only the value
//! shapes needed by the skeleton; richer value kinds (sets, timed
//! decay/recovery metadata) are Phase 2+ rule-runtime concerns.

use crate::hash::CanonicalEncoder;
use crate::id::DefinitionId;

/// Fixed-point scale shared by every `Fixed` value: a `Fixed(1_000_000)`
/// represents `1.0`. Using one fixed scale system-wide keeps arithmetic
/// between canonical values deterministic and comparable without a
/// floating-point conversion step.
pub const FIXED_SCALE: i64 = 1_000_000;

/// A fixed-point number stored as an integer numerator over
/// [`FIXED_SCALE`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedPoint(pub i64);

impl FixedPoint {
    pub const ZERO: FixedPoint = FixedPoint(0);

    pub fn from_integer(v: i64) -> Self {
        FixedPoint(v * FIXED_SCALE)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_point_integer_round_trip() {
        assert_eq!(FixedPoint::from_integer(2).0, 2 * FIXED_SCALE);
    }

    #[test]
    fn type_tag_matches_declared_value_type() {
        assert_eq!(CanonicalValue::Bool(true).type_tag(), ValueType::Bool);
        assert_eq!(
            CanonicalValue::Fixed(FixedPoint::ZERO).type_tag(),
            ValueType::Fixed
        );
    }
}
