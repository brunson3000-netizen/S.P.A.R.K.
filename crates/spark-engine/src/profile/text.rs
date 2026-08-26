//! Bounded human-readable profile text.
//!
//! Descriptions and version labels are operator-facing text rather than
//! canonical vocabulary, so unlike [`spark_core::id::CanonicalTag`] they
//! permit ordinary Unicode. They are still bounded and
//! control-character-free at construction, because a profile description
//! reaches [`crate::profile::manifest::ProfileManifest::manifest_content_hash`] and
//! an unbounded canonical hash input is a storage/hashing vector
//! (`PHASE_1_REFOUNDATION_BRIEF_v0.1.md` "Canonical construction/bounds").

use std::fmt;

/// Maximum length, in characters, of one [`BoundedText`] value.
pub const MAX_BOUNDED_TEXT_LEN: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundedTextError {
    TooLong { len: usize, max: usize },
    ControlCharacter,
}

impl fmt::Display for BoundedTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoundedTextError::TooLong { len, max } => write!(
                f,
                "profile text length {len} exceeds maximum {max} characters"
            ),
            BoundedTextError::ControlCharacter => {
                write!(f, "profile text must not contain control characters")
            }
        }
    }
}

impl std::error::Error for BoundedTextError {}

/// Bounded, control-character-free human-readable text. Empty is
/// permitted (an undescribed definition is legal); unbounded is not.
///
/// The inner `String` is private, so an oversized value cannot be
/// constructed:
///
/// ```compile_fail
/// use spark_engine::profile::text::BoundedText;
/// let forged = BoundedText("x".repeat(100_000));
/// ```
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundedText(String);

impl BoundedText {
    pub fn new(value: impl Into<String>) -> Result<Self, BoundedTextError> {
        let value = value.into();
        let len = value.chars().count();
        if len > MAX_BOUNDED_TEXT_LEN {
            return Err(BoundedTextError::TooLong {
                len,
                max: MAX_BOUNDED_TEXT_LEN,
            });
        }
        if value.chars().any(|c| c.is_control()) {
            return Err(BoundedTextError::ControlCharacter);
        }
        Ok(BoundedText(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for BoundedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Display for BoundedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
    fn bounds_length_and_rejects_control_characters() {
        assert!(BoundedText::new("Curiosity drives investigation.").is_ok());
        assert!(BoundedText::new("").is_ok());
        assert_eq!(
            BoundedText::new("x".repeat(MAX_BOUNDED_TEXT_LEN + 1)),
            Err(BoundedTextError::TooLong {
                len: MAX_BOUNDED_TEXT_LEN + 1,
                max: MAX_BOUNDED_TEXT_LEN
            })
        );
        assert_eq!(
            BoundedText::new("bad\u{0}text"),
            Err(BoundedTextError::ControlCharacter)
        );
    }
}
