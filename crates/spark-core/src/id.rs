//! Immutable namespaced identifiers.
//!
//! Per the controlling blueprint (`CONTROLLING_BLUEPRINT_v0.2.md` §12.3),
//! stable IDs such as `trigger.weather.drought` or `trait.curiosity` are
//! immutable identity, not display text. A display name may change; an ID
//! may be disabled, deprecated, or migrated, but never silently
//! repurposed. This module owns the one syntax rule every namespaced ID in
//! the system must satisfy, and the typed wrappers that keep IDs from
//! different identity spaces (a definition vs. a scope vs. a profile) from
//! being accidentally interchanged by the type system.

use crate::hash::CanonicalEncoder;
use std::fmt;

/// Maximum byte length of one [`StableId`], consistent with the Phase-0
/// bounded-identifier budget (`CONTROLLING_BLUEPRINT_v0.2.md` §10 internal
/// bound discipline). This keeps namespaced identifiers from becoming an
/// unbounded canonical-hashing/storage vector (Phase-1 correction brief
/// M-03).
pub const MAX_STABLE_ID_LEN: usize = 256;

/// Errors constructing a [`StableId`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableIdError {
    Empty,
    TooLong { len: usize, max: usize },
    InvalidCharacter(char),
    LeadingOrTrailingSeparator,
    EmptySegment,
}

impl fmt::Display for StableIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StableIdError::Empty => write!(f, "stable id must not be empty"),
            StableIdError::TooLong { len, max } => {
                write!(f, "stable id length {len} exceeds maximum {max}")
            }
            StableIdError::InvalidCharacter(c) => {
                write!(f, "stable id contains invalid character '{c}'")
            }
            StableIdError::LeadingOrTrailingSeparator => {
                write!(f, "stable id must not start or end with '.'")
            }
            StableIdError::EmptySegment => write!(f, "stable id must not contain '..'"),
        }
    }
}

impl std::error::Error for StableIdError {}

/// A validated, immutable, namespaced identifier: ASCII lowercase
/// alphanumerics, `_` and `-` within dot-separated segments
/// (e.g. `trigger.weather.drought`, `relationship.trust`).
///
/// This is the single syntax rule shared by every identity space
/// (`DefinitionId`, `ScopeId`, `ProfileId`, ...); each identity space is
/// then given a distinct newtype so the type system prevents mixing them.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StableId(String);

impl StableId {
    pub fn new(value: impl Into<String>) -> Result<Self, StableIdError> {
        let value = value.into();
        if value.is_empty() {
            return Err(StableIdError::Empty);
        }
        if value.len() > MAX_STABLE_ID_LEN {
            return Err(StableIdError::TooLong {
                len: value.len(),
                max: MAX_STABLE_ID_LEN,
            });
        }
        if value.starts_with('.') || value.ends_with('.') {
            return Err(StableIdError::LeadingOrTrailingSeparator);
        }
        for segment in value.split('.') {
            if segment.is_empty() {
                return Err(StableIdError::EmptySegment);
            }
        }
        for c in value.chars() {
            let ok =
                c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' || c == '.';
            if !ok {
                return Err(StableIdError::InvalidCharacter(c));
            }
        }
        Ok(StableId(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str(&self.0);
    }
}

impl fmt::Debug for StableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for StableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Declares a typed newtype over [`StableId`] for one identity namespace.
macro_rules! stable_id_newtype {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(StableId);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, StableIdError> {
                Ok(Self(StableId::new(value)?))
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
                self.0.canonicalize(enc);
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

stable_id_newtype!(
    DefinitionId,
    "Immutable namespaced ID of a profile definition (trigger, trait, state, behavior, ...)."
);
stable_id_newtype!(ProfileId, "Immutable namespaced ID of an active profile.");
stable_id_newtype!(
    SourceId,
    "Identity of a canonical command source (e.g. a granted timeline sequencer)."
);
stable_id_newtype!(CommandId, "Identity of one canonical command envelope.");
stable_id_newtype!(
    FenceId,
    "Identity of one canonical timeline finality fence."
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_namespaced_id() {
        assert!(StableId::new("trigger.weather.drought").is_ok());
        assert!(DefinitionId::new("trait.curiosity").is_ok());
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(StableId::new(""), Err(StableIdError::Empty));
    }

    #[test]
    fn rejects_invalid_character() {
        assert_eq!(
            StableId::new("Trigger.Weather"),
            Err(StableIdError::InvalidCharacter('T'))
        );
        assert!(StableId::new("trigger weather").is_err());
    }

    #[test]
    fn rejects_id_longer_than_max_length() {
        let too_long = "a".repeat(MAX_STABLE_ID_LEN + 1);
        assert_eq!(
            StableId::new(too_long.clone()),
            Err(StableIdError::TooLong {
                len: too_long.len(),
                max: MAX_STABLE_ID_LEN
            })
        );
        assert!(StableId::new("a".repeat(MAX_STABLE_ID_LEN)).is_ok());
    }

    #[test]
    fn rejects_leading_trailing_and_double_dot() {
        assert_eq!(
            StableId::new(".trigger"),
            Err(StableIdError::LeadingOrTrailingSeparator)
        );
        assert_eq!(
            StableId::new("trigger."),
            Err(StableIdError::LeadingOrTrailingSeparator)
        );
        assert_eq!(
            StableId::new("trigger..weather"),
            Err(StableIdError::EmptySegment)
        );
    }

    #[test]
    fn distinct_newtypes_are_not_interchangeable_at_the_type_level() {
        // This is a compile-time property; the test documents intent and
        // exercises construction of both types from the same syntax.
        let _def = DefinitionId::new("trait.curiosity").unwrap();
        let _profile = ProfileId::new("game-world").unwrap();
    }
}
