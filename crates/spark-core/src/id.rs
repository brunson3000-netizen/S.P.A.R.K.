//! Immutable namespaced identifiers and bounded canonical tags.
//!
//! Per the controlling blueprint (`CONTROLLING_BLUEPRINT_v0.2.md` §12.3),
//! stable IDs such as `trigger.weather.drought` or `trait.curiosity` are
//! immutable identity, not display text. A display name may change; an ID
//! may be disabled, deprecated, or migrated, but never silently
//! repurposed. This module owns the syntax rules every canonical
//! identifier in the system must satisfy, and the typed wrappers that keep
//! IDs from different identity spaces (a definition vs. a scope vs. a
//! profile) from being accidentally interchanged by the type system.
//!
//! Two bound disciplines live here, both re-founded per
//! `PHASE_1_REFOUNDATION_BRIEF_v0.1.md` "Canonical construction/bounds":
//!
//! 1. **Namespacing.** §12.3's ID contract is explicitly *namespaced* —
//!    every example it gives has at least two dot-separated segments. A
//!    [`DefinitionId`] therefore requires at least two segments, so a bare
//!    `curiosity` is rejected where `trait.curiosity` is accepted. Identity
//!    spaces the blueprint does **not** define as namespaced vocabulary —
//!    a [`ProfileId`] naming a deployment profile, or the transient
//!    [`CommandId`]/[`FenceId`]/[`SourceId`] of one runtime artifact — keep
//!    the single-segment-permitted rule, because requiring a namespace
//!    there would be an invented constraint rather than an enforced one.
//! 2. **Boundedness.** Every canonical string that reaches a canonical
//!    hash is length-bounded at construction. [`CanonicalTag`] is the
//!    short bounded vocabulary type used wherever a canonical *kind*
//!    discriminator is needed (a command kind, a scheduler work kind, a
//!    custom definition kind); it exists so those fields can never be
//!    unbounded `String`s again.

use crate::hash::CanonicalEncoder;
use std::borrow::Cow;
use std::fmt;

/// Maximum byte length of one [`StableId`], consistent with the Phase-0
/// bounded-identifier budget (`CONTROLLING_BLUEPRINT_v0.2.md` §10 internal
/// bound discipline). This keeps namespaced identifiers from becoming an
/// unbounded canonical-hashing/storage vector.
pub const MAX_STABLE_ID_LEN: usize = 256;

/// Maximum byte length of one [`CanonicalTag`]. Kind discriminators are
/// short vocabulary words, not free text, so they carry a tighter bound
/// than a full namespaced identifier.
pub const MAX_CANONICAL_TAG_LEN: usize = 64;

/// Errors constructing a canonical identifier or tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableIdError {
    Empty,
    TooLong {
        len: usize,
        max: usize,
    },
    InvalidCharacter(char),
    LeadingOrTrailingSeparator,
    EmptySegment,
    /// The identifier has fewer dot-separated segments than its identity
    /// space requires (e.g. a bare `curiosity` where the blueprint's ID
    /// contract requires `trait.curiosity`).
    TooFewSegments {
        found: usize,
        required: usize,
    },
}

impl fmt::Display for StableIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StableIdError::Empty => write!(f, "canonical identifier must not be empty"),
            StableIdError::TooLong { len, max } => {
                write!(f, "canonical identifier length {len} exceeds maximum {max}")
            }
            StableIdError::InvalidCharacter(c) => {
                write!(f, "canonical identifier contains invalid character '{c}'")
            }
            StableIdError::LeadingOrTrailingSeparator => {
                write!(f, "canonical identifier must not start or end with '.'")
            }
            StableIdError::EmptySegment => {
                write!(f, "canonical identifier must not contain '..'")
            }
            StableIdError::TooFewSegments { found, required } => write!(
                f,
                "canonical identifier has {found} dot-separated segment(s) but its identity space requires at least {required} (e.g. 'trait.curiosity', not 'curiosity')"
            ),
        }
    }
}

impl std::error::Error for StableIdError {}

/// The one syntax rule shared by every canonical identifier: ASCII
/// lowercase alphanumerics, `_` and `-` within non-empty dot-separated
/// segments, bounded length, and at least `min_segments` segments.
fn validate_canonical_syntax(
    value: &str,
    max_len: usize,
    min_segments: usize,
) -> Result<(), StableIdError> {
    if value.is_empty() {
        return Err(StableIdError::Empty);
    }
    if value.len() > max_len {
        return Err(StableIdError::TooLong {
            len: value.len(),
            max: max_len,
        });
    }
    if value.starts_with('.') || value.ends_with('.') {
        return Err(StableIdError::LeadingOrTrailingSeparator);
    }
    let mut segments = 0usize;
    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(StableIdError::EmptySegment);
        }
        segments += 1;
    }
    for c in value.chars() {
        let ok = c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' || c == '.';
        if !ok {
            return Err(StableIdError::InvalidCharacter(c));
        }
    }
    if segments < min_segments {
        return Err(StableIdError::TooFewSegments {
            found: segments,
            required: min_segments,
        });
    }
    Ok(())
}

/// A validated, immutable, namespaced identifier: ASCII lowercase
/// alphanumerics, `_` and `-` within dot-separated segments
/// (e.g. `trigger.weather.drought`, `relationship.trust`).
///
/// This is the general single-segment-permitted form; each identity space
/// is given a distinct newtype (below) so the type system prevents mixing
/// them, and each newtype declares its own minimum segment count.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StableId(String);

impl StableId {
    pub fn new(value: impl Into<String>) -> Result<Self, StableIdError> {
        let value = value.into();
        validate_canonical_syntax(&value, MAX_STABLE_ID_LEN, 1)?;
        Ok(StableId(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The number of dot-separated segments (`trait.curiosity` is 2).
    pub fn segment_count(&self) -> usize {
        self.0.split('.').count()
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

/// A short, bounded, validated canonical vocabulary tag.
///
/// Wherever the canonical kernel needs a *kind* discriminator that reaches
/// a canonical hash — a [`crate::timeline::CommandKind`], a
/// [`crate::scheduler::WorkKind`], a custom definition kind — it uses this
/// type rather than a raw `String`. An unbounded canonical string is both
/// a hashing/storage vector and a silent way for two deployments to
/// disagree about what is canonical, so boundedness is enforced at
/// construction and cannot be bypassed: the inner `String` is private and
/// there is no `From<String>`.
///
/// A tag may be a single segment (`weather`) or namespaced
/// (`trigger.evaluate`); what it may not be is empty, oversized, or
/// non-canonical in syntax.
///
/// ```
/// use spark_core::id::{CanonicalTag, MAX_CANONICAL_TAG_LEN};
/// assert!(CanonicalTag::new("trigger.evaluate").is_ok());
/// assert!(CanonicalTag::new("x".repeat(MAX_CANONICAL_TAG_LEN + 1)).is_err());
/// ```
///
/// The bound is structural, not merely checked: there is no way to build a
/// `CanonicalTag` from an arbitrary `String`.
///
/// ```compile_fail
/// use spark_core::id::CanonicalTag;
/// // The inner field is private, so an unbounded canonical tag cannot be
/// // constructed even from inside a trusted-looking call site.
/// let forged = CanonicalTag("x".repeat(100_000));
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanonicalTag(Cow<'static, str>);

/// The same language [`validate_canonical_syntax`] accepts for a
/// single-segment tag, expressed as a `const fn` so a compile-time
/// constant tag can be validated by the compiler.
const fn is_valid_canonical_tag(bytes: &[u8]) -> bool {
    if bytes.is_empty() || bytes.len() > MAX_CANONICAL_TAG_LEN {
        return false;
    }
    if bytes[0] == b'.' || bytes[bytes.len() - 1] == b'.' {
        return false;
    }
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        let ok =
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_' || c == b'-' || c == b'.';
        if !ok {
            return false;
        }
        if c == b'.' && i + 1 < bytes.len() && bytes[i + 1] == b'.' {
            return false;
        }
        i += 1;
    }
    true
}

impl CanonicalTag {
    pub fn new(value: impl Into<String>) -> Result<Self, StableIdError> {
        let value = value.into();
        validate_canonical_syntax(&value, MAX_CANONICAL_TAG_LEN, 1)?;
        Ok(CanonicalTag(Cow::Owned(value)))
    }

    /// Builds a tag from a compile-time constant, validated by the
    /// compiler.
    ///
    /// In a `const` context an invalid literal is a **compile error**, not
    /// a runtime panic — which is what lets the engine's own baseline
    /// vocabulary (command kinds, work kinds, definition kinds) be
    /// expressed as constants without introducing a fallible or panicking
    /// path at every use site.
    ///
    /// This is the standard compile-time-assertion idiom, and it carries
    /// the standard caveat: the `assert!` is evaluated at compile time
    /// only when the call appears in a `const` context. Calling it from a
    /// non-`const` context with an invalid string (which requires
    /// deliberately manufacturing a `&'static str` at runtime, e.g. by
    /// leaking a `String`) would panic instead. Use [`CanonicalTag::new`]
    /// for any value that is not a literal; it validates and returns a
    /// typed error.
    ///
    /// ```
    /// use spark_core::id::CanonicalTag;
    /// const TRIGGER: CanonicalTag = CanonicalTag::from_static("trigger");
    /// assert_eq!(TRIGGER.as_str(), "trigger");
    /// ```
    ///
    /// ```compile_fail
    /// use spark_core::id::CanonicalTag;
    /// // Rejected at compile time: uppercase is not canonical syntax.
    /// const BAD: CanonicalTag = CanonicalTag::from_static("Trigger");
    /// ```
    pub const fn from_static(value: &'static str) -> Self {
        assert!(
            is_valid_canonical_tag(value.as_bytes()),
            "canonical tag literal is not valid canonical syntax"
        );
        CanonicalTag(Cow::Borrowed(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_str(&self.0);
    }
}

impl fmt::Debug for CanonicalTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CanonicalTag({})", self.0)
    }
}

impl fmt::Display for CanonicalTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Declares a typed newtype over [`StableId`] for one identity namespace,
/// with that namespace's own minimum segment count.
macro_rules! stable_id_newtype {
    ($name:ident, $min_segments:expr, $doc:expr) => {
        #[doc = $doc]
        #[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(StableId);

        impl $name {
            /// The minimum number of dot-separated segments this identity
            /// space requires.
            pub const MIN_SEGMENTS: usize = $min_segments;

            pub fn new(value: impl Into<String>) -> Result<Self, StableIdError> {
                let value = value.into();
                validate_canonical_syntax(&value, MAX_STABLE_ID_LEN, $min_segments)?;
                Ok(Self(StableId(value)))
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
    2,
    "Immutable **namespaced** ID of a profile definition (trigger, trait, state, behavior, ...).\n\n\
     `CONTROLLING_BLUEPRINT_v0.2.md` §12.3 fixes this identity space as namespaced — every ID it \
     lists (`trigger.weather.drought`, `trait.curiosity`, `relationship.trust`, \
     `behavior.acquire.steal`) carries a domain segment. A bare, unnamespaced `curiosity` is \
     therefore rejected at construction: an ID with no namespace cannot be deprecated, aliased, or \
     migrated without colliding across domains.\n\n\
     ```\n\
     use spark_core::id::DefinitionId;\n\
     assert!(DefinitionId::new(\"trait.curiosity\").is_ok());\n\
     assert!(DefinitionId::new(\"curiosity\").is_err());\n\
     ```"
);
stable_id_newtype!(
    ProfileId,
    1,
    "Immutable ID of an active profile (e.g. `game-world`, `mci-social`).\n\n\
     A profile names a deployment/trust domain rather than a member of the namespaced definition \
     vocabulary, so the blueprint's namespacing contract does not apply to it and a single-segment \
     profile name is valid."
);
stable_id_newtype!(
    SourceId,
    1,
    "Identity of a canonical command source (e.g. a granted timeline sequencer)."
);
stable_id_newtype!(CommandId, 1, "Identity of one canonical command envelope.");
stable_id_newtype!(
    FenceId,
    1,
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
        assert_eq!(CanonicalTag::new(""), Err(StableIdError::Empty));
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

    /// Re-foundation bounds requirement 1: the blueprint's ID contract is
    /// namespaced, so a bare definition ID must be rejected.
    #[test]
    fn definition_id_requires_a_namespace() {
        assert_eq!(
            DefinitionId::new("curiosity"),
            Err(StableIdError::TooFewSegments {
                found: 1,
                required: 2
            })
        );
        assert!(DefinitionId::new("trait.curiosity").is_ok());
        assert!(DefinitionId::new("trigger.weather.drought").is_ok());
    }

    /// Identity spaces the blueprint does not define as namespaced
    /// vocabulary keep the single-segment rule, so the namespacing bound
    /// is an enforced contract rather than a blanket invention.
    #[test]
    fn non_definition_identity_spaces_permit_a_single_segment() {
        assert!(ProfileId::new("game-world").is_ok());
        assert!(SourceId::new("sequencer").is_ok());
        assert!(CommandId::new("cmd-0").is_ok());
        assert!(FenceId::new("fence-1").is_ok());
    }

    /// Re-foundation bounds requirement 4: canonical kind discriminators
    /// are bounded validated tags, never unbounded strings.
    /// The `const fn` validator must accept and reject exactly the same
    /// language as the runtime constructor, or a constant could encode a
    /// tag the runtime path would refuse (or vice versa).
    #[test]
    fn const_and_runtime_tag_validation_agree() {
        let cases = [
            "trigger",
            "trigger.evaluate",
            "state_decay",
            "a-b",
            "",
            ".trigger",
            "trigger.",
            "trigger..evaluate",
            "Trigger",
            "trigger evaluate",
            "trigger/evaluate",
            "trigg\u{e9}r",
        ];
        for case in cases {
            assert_eq!(
                is_valid_canonical_tag(case.as_bytes()),
                CanonicalTag::new(case).is_ok(),
                "const and runtime validation disagree on {case:?}"
            );
        }
        let at_limit = "k".repeat(MAX_CANONICAL_TAG_LEN);
        let over_limit = "k".repeat(MAX_CANONICAL_TAG_LEN + 1);
        assert!(is_valid_canonical_tag(at_limit.as_bytes()));
        assert!(!is_valid_canonical_tag(over_limit.as_bytes()));
    }

    #[test]
    fn const_tag_and_runtime_tag_are_equal_values() {
        const TRIGGER: CanonicalTag = CanonicalTag::from_static("trigger");
        assert_eq!(TRIGGER, CanonicalTag::new("trigger").unwrap());
    }

    #[test]
    fn canonical_tag_is_bounded_and_validated() {
        assert!(CanonicalTag::new("trigger.evaluate").is_ok());
        assert!(CanonicalTag::new("weather").is_ok());
        let oversized = "k".repeat(MAX_CANONICAL_TAG_LEN + 1);
        assert_eq!(
            CanonicalTag::new(oversized.clone()),
            Err(StableIdError::TooLong {
                len: oversized.len(),
                max: MAX_CANONICAL_TAG_LEN
            })
        );
        assert!(CanonicalTag::new("Trigger").is_err());
        assert!(CanonicalTag::new("trigger evaluate").is_err());
    }

    #[test]
    fn distinct_newtypes_are_not_interchangeable_at_the_type_level() {
        // This is a compile-time property; the test documents intent and
        // exercises construction of both types from the same syntax.
        let _def = DefinitionId::new("trait.curiosity").unwrap();
        let _profile = ProfileId::new("game-world").unwrap();
    }

    #[test]
    fn segment_count_is_reported() {
        assert_eq!(StableId::new("a").unwrap().segment_count(), 1);
        assert_eq!(StableId::new("a.b.c").unwrap().segment_count(), 3);
    }
}
