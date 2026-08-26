//! Canonical value types and validated declarative value constraints.
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
//! Re-founded per `PHASE_1_REFOUNDATION_BRIEF_v0.1.md` "Canonical
//! construction/bounds" requirements 2 and 3. Two properties are now
//! **structural** rather than merely documented:
//!
//! 1. **A [`ValueConstraint`] cannot be incoherent.** Its representation is
//!    private and every constructor that takes a range validates it, so
//!    `Int { min: 10, max: 0 }` — a bound that accepts nothing and would
//!    silently make a definition unwritable — is not expressible. The
//!    previous public-variant enum let a profile validator "accept" such a
//!    constraint because there was nothing to reject at construction.
//! 2. **A categorical canonical value cannot be unbounded.** Categorical
//!    values reach canonical state, canonical hashes, and persistence, so
//!    [`CategoricalValue`] enforces a length bound and rejects control
//!    characters at construction. `CanonicalValue::Categorical` therefore
//!    carries a validated value, not a raw `String`.

use crate::hash::CanonicalEncoder;
use crate::id::DefinitionId;
use std::fmt;

/// Fixed-point scale shared by every `Fixed` value: a raw value of
/// `FIXED_SCALE` represents `1.0`. Using one fixed scale system-wide keeps
/// arithmetic between canonical values deterministic and comparable
/// without a floating-point conversion step.
pub const FIXED_SCALE: i64 = 1_000_000;

/// Maximum length, in characters, of a [`CategoricalValue`] and therefore
/// the hard ceiling any declared categorical bound may name.
pub const MAX_CATEGORICAL_VALUE_LEN: usize = 256;

/// Rejects a [`FixedPoint`] construction that would overflow `i64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedPointOverflow {
    pub attempted_integer: i64,
}

impl fmt::Display for FixedPointOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fixed-point construction from integer {} overflows i64 at scale {FIXED_SCALE}",
            self.attempted_integer
        )
    }
}

impl std::error::Error for FixedPointOverflow {}

/// A fixed-point number stored as an integer numerator over
/// [`FIXED_SCALE`]. The raw numerator is intentionally private so that
/// checked construction stays the only sanctioned path for
/// integer-derived values and the type cannot be casually reconstructed
/// from an unchecked arithmetic result elsewhere.
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

/// Rejects a malformed categorical canonical value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CategoricalValueError {
    Empty,
    TooLong { len: usize, max: usize },
    ControlCharacter,
}

impl fmt::Display for CategoricalValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CategoricalValueError::Empty => write!(f, "categorical value must not be empty"),
            CategoricalValueError::TooLong { len, max } => write!(
                f,
                "categorical value length {len} exceeds maximum {max} characters"
            ),
            CategoricalValueError::ControlCharacter => {
                write!(f, "categorical value must not contain control characters")
            }
        }
    }
}

impl std::error::Error for CategoricalValueError {}

/// A bounded, validated categorical canonical value (e.g.
/// `religion.asterian`, `denomination.asterian.reform`).
///
/// Categorical values are profile vocabulary rather than engine
/// vocabulary, so unlike [`crate::id::CanonicalTag`] they permit ordinary
/// Unicode text — but they are still canonical hash input and persisted
/// canonical state, so they are length-bounded and control-character-free
/// at construction. The inner `String` is private, so an unbounded
/// categorical value is not constructible.
///
/// ```compile_fail
/// use spark_core::value::CategoricalValue;
/// let forged = CategoricalValue("x".repeat(100_000));
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CategoricalValue(String);

impl CategoricalValue {
    pub fn new(value: impl Into<String>) -> Result<Self, CategoricalValueError> {
        let value = value.into();
        if value.is_empty() {
            return Err(CategoricalValueError::Empty);
        }
        let len = value.chars().count();
        if len > MAX_CATEGORICAL_VALUE_LEN {
            return Err(CategoricalValueError::TooLong {
                len,
                max: MAX_CATEGORICAL_VALUE_LEN,
            });
        }
        if value.chars().any(|c| c.is_control()) {
            return Err(CategoricalValueError::ControlCharacter);
        }
        Ok(CategoricalValue(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Length in characters, used by [`ValueConstraint`] bound checks.
    pub fn char_len(&self) -> usize {
        self.0.chars().count()
    }
}

impl fmt::Debug for CategoricalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CategoricalValue({})", self.0)
    }
}

impl fmt::Display for CategoricalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
    Categorical(CategoricalValue),
    Reference(DefinitionId),
}

impl CanonicalValue {
    /// Convenience constructor for a validated categorical value.
    pub fn categorical(value: impl Into<String>) -> Result<Self, CategoricalValueError> {
        Ok(CanonicalValue::Categorical(CategoricalValue::new(value)?))
    }

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
                enc.push_i64(f.raw());
            }
            CanonicalValue::Categorical(s) => {
                enc.push_str("categorical");
                enc.push_str(s.as_str());
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

/// Rejects an incoherent or unbounded declared value constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueConstraintError {
    /// `min > max`: the range accepts no value at all, which silently
    /// makes the declaring definition unwritable rather than failing
    /// loudly at activation.
    IncoherentIntRange { min: i64, max: i64 },
    /// `min > max` for a fixed-point range.
    IncoherentFixedRange { min: i64, max: i64 },
    /// A declared categorical bound of zero admits nothing.
    ZeroCategoricalBound,
    /// A declared categorical bound above the hard canonical ceiling
    /// ([`MAX_CATEGORICAL_VALUE_LEN`]) would claim to permit values that
    /// [`CategoricalValue`] can never represent.
    CategoricalBoundExceedsCeiling { max_len: usize, ceiling: usize },
}

impl fmt::Display for ValueConstraintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueConstraintError::IncoherentIntRange { min, max } => write!(
                f,
                "integer constraint is incoherent: min {min} exceeds max {max}, so it accepts no value"
            ),
            ValueConstraintError::IncoherentFixedRange { min, max } => write!(
                f,
                "fixed-point constraint is incoherent: raw min {min} exceeds raw max {max}, so it accepts no value"
            ),
            ValueConstraintError::ZeroCategoricalBound => {
                write!(f, "categorical constraint with max_len 0 accepts no value")
            }
            ValueConstraintError::CategoricalBoundExceedsCeiling { max_len, ceiling } => write!(
                f,
                "categorical constraint max_len {max_len} exceeds the canonical ceiling {ceiling}"
            ),
        }
    }
}

impl std::error::Error for ValueConstraintError {}

/// The private representation behind [`ValueConstraint`]. Kept out of the
/// public API so a constraint can only exist if it passed validation.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ConstraintRepr {
    Bool,
    Int { min: i64, max: i64 },
    Fixed { min: FixedPoint, max: FixedPoint },
    Categorical { max_len: usize },
    Reference,
}

/// A validated declarative bound on a definition's canonical value.
///
/// Every constructor that takes a range validates coherence, and the
/// representation is private, so an incoherent constraint is not
/// expressible anywhere in the system:
///
/// ```
/// use spark_core::value::{ValueConstraint, ValueConstraintError};
/// assert!(ValueConstraint::int(0, 10).is_ok());
/// assert_eq!(
///     ValueConstraint::int(10, 0),
///     Err(ValueConstraintError::IncoherentIntRange { min: 10, max: 0 })
/// );
/// ```
///
/// ```compile_fail
/// use spark_core::value::ValueConstraint;
/// // The old public-variant form is gone: a caller cannot bypass the
/// // coherence check by naming the variant directly.
/// let forged = ValueConstraint::Int { min: 10, max: 0 };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueConstraint(ConstraintRepr);

impl ValueConstraint {
    pub fn boolean() -> Self {
        ValueConstraint(ConstraintRepr::Bool)
    }

    pub fn reference() -> Self {
        ValueConstraint(ConstraintRepr::Reference)
    }

    /// A closed integer range. Rejects `min > max`.
    pub fn int(min: i64, max: i64) -> Result<Self, ValueConstraintError> {
        if min > max {
            return Err(ValueConstraintError::IncoherentIntRange { min, max });
        }
        Ok(ValueConstraint(ConstraintRepr::Int { min, max }))
    }

    /// A closed fixed-point range. Rejects `min > max`.
    pub fn fixed(min: FixedPoint, max: FixedPoint) -> Result<Self, ValueConstraintError> {
        if min > max {
            return Err(ValueConstraintError::IncoherentFixedRange {
                min: min.raw(),
                max: max.raw(),
            });
        }
        Ok(ValueConstraint(ConstraintRepr::Fixed { min, max }))
    }

    /// A categorical value bounded to at most `max_len` characters.
    /// Rejects a zero bound and any bound above the canonical ceiling
    /// [`MAX_CATEGORICAL_VALUE_LEN`], so a constraint can never claim to
    /// permit a value the canonical value type cannot hold.
    pub fn categorical(max_len: usize) -> Result<Self, ValueConstraintError> {
        if max_len == 0 {
            return Err(ValueConstraintError::ZeroCategoricalBound);
        }
        if max_len > MAX_CATEGORICAL_VALUE_LEN {
            return Err(ValueConstraintError::CategoricalBoundExceedsCeiling {
                max_len,
                ceiling: MAX_CATEGORICAL_VALUE_LEN,
            });
        }
        Ok(ValueConstraint(ConstraintRepr::Categorical { max_len }))
    }

    pub fn value_type(&self) -> ValueType {
        match self.0 {
            ConstraintRepr::Bool => ValueType::Bool,
            ConstraintRepr::Int { .. } => ValueType::Int,
            ConstraintRepr::Fixed { .. } => ValueType::Fixed,
            ConstraintRepr::Categorical { .. } => ValueType::Categorical,
            ConstraintRepr::Reference => ValueType::Reference,
        }
    }

    /// Whether `value` both matches this constraint's declared type and
    /// falls within its declared bounds.
    pub fn accepts(&self, value: &CanonicalValue) -> bool {
        match (&self.0, value) {
            (ConstraintRepr::Bool, CanonicalValue::Bool(_)) => true,
            (ConstraintRepr::Int { min, max }, CanonicalValue::Int(v)) => v >= min && v <= max,
            (ConstraintRepr::Fixed { min, max }, CanonicalValue::Fixed(v)) => v >= min && v <= max,
            (ConstraintRepr::Categorical { max_len }, CanonicalValue::Categorical(s)) => {
                s.char_len() <= *max_len
            }
            (ConstraintRepr::Reference, CanonicalValue::Reference(_)) => true,
            _ => false,
        }
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match &self.0 {
            ConstraintRepr::Bool => {
                enc.push_str("constraint.bool");
            }
            ConstraintRepr::Int { min, max } => {
                enc.push_str("constraint.int");
                enc.push_i64(*min);
                enc.push_i64(*max);
            }
            ConstraintRepr::Fixed { min, max } => {
                enc.push_str("constraint.fixed");
                enc.push_i64(min.raw());
                enc.push_i64(max.raw());
            }
            ConstraintRepr::Categorical { max_len } => {
                enc.push_str("constraint.categorical");
                enc.push_u64(*max_len as u64);
            }
            ConstraintRepr::Reference => {
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
        let c = ValueConstraint::int(0, 10).unwrap();
        assert!(c.accepts(&CanonicalValue::Int(5)));
        assert!(!c.accepts(&CanonicalValue::Int(11)));
        assert!(!c.accepts(&CanonicalValue::Int(-1)));
    }

    #[test]
    fn constraint_rejects_wrong_type() {
        let c = ValueConstraint::boolean();
        assert!(!c.accepts(&CanonicalValue::Int(0)));
    }

    #[test]
    fn categorical_constraint_bounds_length() {
        let c = ValueConstraint::categorical(3).unwrap();
        assert!(c.accepts(&CanonicalValue::categorical("abc").unwrap()));
        assert!(!c.accepts(&CanonicalValue::categorical("abcd").unwrap()));
    }

    /// Re-foundation bounds requirement 2: an incoherent bound is not
    /// merely rejected by a validator, it is inexpressible.
    #[test]
    fn incoherent_ranges_are_rejected_at_construction() {
        assert_eq!(
            ValueConstraint::int(10, 0),
            Err(ValueConstraintError::IncoherentIntRange { min: 10, max: 0 })
        );
        assert!(ValueConstraint::int(0, 0).is_ok());
        assert_eq!(
            ValueConstraint::fixed(
                FixedPoint::from_integer(1).unwrap(),
                FixedPoint::from_integer(0).unwrap()
            ),
            Err(ValueConstraintError::IncoherentFixedRange {
                min: FIXED_SCALE,
                max: 0
            })
        );
    }

    #[test]
    fn categorical_constraint_bound_is_itself_bounded() {
        assert_eq!(
            ValueConstraint::categorical(0),
            Err(ValueConstraintError::ZeroCategoricalBound)
        );
        assert_eq!(
            ValueConstraint::categorical(usize::MAX),
            Err(ValueConstraintError::CategoricalBoundExceedsCeiling {
                max_len: usize::MAX,
                ceiling: MAX_CATEGORICAL_VALUE_LEN
            })
        );
        assert!(ValueConstraint::categorical(MAX_CATEGORICAL_VALUE_LEN).is_ok());
    }

    /// Re-foundation bounds requirement 3/4: a canonical categorical value
    /// is bounded at construction, so no unbounded string can reach a
    /// canonical hash.
    #[test]
    fn oversized_categorical_value_is_rejected_at_construction() {
        let oversized = "v".repeat(MAX_CATEGORICAL_VALUE_LEN + 1);
        assert_eq!(
            CategoricalValue::new(oversized),
            Err(CategoricalValueError::TooLong {
                len: MAX_CATEGORICAL_VALUE_LEN + 1,
                max: MAX_CATEGORICAL_VALUE_LEN
            })
        );
        assert!(CategoricalValue::new("v".repeat(MAX_CATEGORICAL_VALUE_LEN)).is_ok());
        assert_eq!(CategoricalValue::new(""), Err(CategoricalValueError::Empty));
        assert_eq!(
            CategoricalValue::new("bad\u{0}value"),
            Err(CategoricalValueError::ControlCharacter)
        );
    }

    #[test]
    fn distinct_categorical_values_canonicalize_distinctly() {
        let mut a = CanonicalEncoder::new();
        CanonicalValue::categorical("religion.asterian")
            .unwrap()
            .canonicalize(&mut a);
        let mut b = CanonicalEncoder::new();
        CanonicalValue::categorical("religion.solar")
            .unwrap()
            .canonicalize(&mut b);
        assert_ne!(a.finish(), b.finish());
    }
}
