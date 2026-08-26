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
    if value.split('.').any(str::is_empty) {
        return Err(StableIdError::EmptySegment);
    }
    // `count()` performs its own arithmetic internally; doing it here
    // would be an unchecked `+` under this crate's arithmetic gate.
    let segments = value.split('.').count();
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

/// Rejects a compile-time or runtime `&'static str` that is not valid
/// canonical tag syntax.
///
/// This is a separate, `Copy`, allocation-free error type from
/// [`StableIdError`] because it is produced inside a `const fn`: it must
/// be constructible during const evaluation, which rules out anything
/// that allocates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticTagError {
    Empty,
    TooLong {
        len: usize,
        max: usize,
    },
    /// The offending byte. A `char` is not used because the const
    /// validator scans bytes; every rejected byte is by definition not a
    /// valid canonical character.
    InvalidByte {
        byte: u8,
    },
    LeadingOrTrailingSeparator,
    EmptySegment,
    /// The scan index would overflow `usize`. Unreachable for any real
    /// `&str` (its length is bounded by `isize::MAX`), but reported rather
    /// than assumed so the validator contains no unchecked arithmetic at
    /// all.
    ScanOverflow,
}

impl fmt::Display for StaticTagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StaticTagError::Empty => write!(f, "canonical tag must not be empty"),
            StaticTagError::TooLong { len, max } => {
                write!(f, "canonical tag length {len} exceeds maximum {max}")
            }
            StaticTagError::InvalidByte { byte } => {
                write!(f, "canonical tag contains invalid byte 0x{byte:02x}")
            }
            StaticTagError::LeadingOrTrailingSeparator => {
                write!(f, "canonical tag must not start or end with '.'")
            }
            StaticTagError::EmptySegment => write!(f, "canonical tag must not contain '..'"),
            StaticTagError::ScanOverflow => write!(f, "canonical tag scan index overflowed"),
        }
    }
}

impl std::error::Error for StaticTagError {}

/// The same language [`validate_canonical_syntax`] accepts for a
/// single-segment tag, expressed as a `const fn` so a compile-time
/// constant tag can be validated by the compiler — and, crucially, so the
/// *same* function is total when called at runtime.
///
/// The scan indexes `bytes` directly because a `const fn` cannot use
/// iterators or `slice::first`/`last`. Every index is guarded immediately
/// above its use (`i < bytes.len()`, and the empty case returns before the
/// first/last accesses), so the crate's `indexing_slicing` gate is allowed
/// exactly here and nowhere else.
#[allow(clippy::indexing_slicing)]
const fn validate_canonical_tag_bytes(bytes: &[u8]) -> Result<(), StaticTagError> {
    if bytes.is_empty() {
        return Err(StaticTagError::Empty);
    }
    if bytes.len() > MAX_CANONICAL_TAG_LEN {
        return Err(StaticTagError::TooLong {
            len: bytes.len(),
            max: MAX_CANONICAL_TAG_LEN,
        });
    }
    // `bytes` is non-empty here, so index 0 and `len - 1` are both in
    // bounds; `len - 1` is computed with `checked_sub` so the function
    // contains no unchecked arithmetic.
    let last_index = match bytes.len().checked_sub(1) {
        Some(index) => index,
        None => return Err(StaticTagError::ScanOverflow),
    };
    if bytes[0] == b'.' || bytes[last_index] == b'.' {
        return Err(StaticTagError::LeadingOrTrailingSeparator);
    }
    let mut i = 0usize;
    while i < bytes.len() {
        let c = bytes[i];
        let ok =
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_' || c == b'-' || c == b'.';
        if !ok {
            return Err(StaticTagError::InvalidByte { byte: c });
        }
        let next = match i.checked_add(1) {
            Some(next) => next,
            None => return Err(StaticTagError::ScanOverflow),
        };
        if c == b'.' && next < bytes.len() && bytes[next] == b'.' {
            return Err(StaticTagError::EmptySegment);
        }
        i = next;
    }
    Ok(())
}

/// A `&'static str` that has passed canonical-tag validation.
///
/// Its field is private and [`CanonicalTag::validate_static`] is its only
/// producer, so possession is proof of validation — which is what lets
/// [`CanonicalTag::from_validated`] be public, `const`, **and** total.
///
/// It is `Copy` and holds no owned allocation on purpose: a value with a
/// destructor cannot be dropped during const evaluation, so a
/// `Result<CanonicalTag, _>` is not const-usable while a
/// `Result<ValidatedStaticTag, _>` is. That is what makes
/// [`canonical_tag!`](crate::canonical_tag) able to force compile-time
/// validation without any function that panics at run time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidatedStaticTag(&'static str);

impl ValidatedStaticTag {
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

/// Builds a [`CanonicalTag`] from a literal, validated by **const
/// evaluation**.
///
/// This replaces the panic-capable `CanonicalTag::from_static`. That
/// function conflated two roles: compile-time literal validation (where an
/// invalid tag should be a build failure) and runtime construction from a
/// `&'static str` (where an invalid tag must be a typed error). Because a
/// `const fn` only evaluates at compile time when it is *called* in a
/// const context, an ordinary call site such as
/// `CanonicalTag::from_static("INVALID TAG")` compiled fine and panicked
/// at run time — an executed production panic path, and the one an
/// independent reviewer falsified.
///
/// The two roles are now separate and neither can panic at run time:
///
/// - [`CanonicalTag::try_from_static`] is total in every context;
/// - this macro forces const evaluation by binding a `const` item, so an
///   invalid literal is **always** a compile error and the `panic!` in the
///   expansion is unreachable at run time by construction. That single
///   const-evaluated panic is the one sanctioned exception to the crate's
///   `clippy::panic` gate, and the `allow` is scoped to the macro's own
///   `const` item rather than to any call site.
///
/// ```
/// use spark_core::{canonical_tag, id::CanonicalTag};
/// const TRIGGER: CanonicalTag = canonical_tag!("trigger");
/// assert_eq!(TRIGGER, CanonicalTag::new("trigger").unwrap());
/// ```
///
/// ```compile_fail
/// use spark_core::canonical_tag;
/// // Rejected during const evaluation: uppercase is not canonical syntax.
/// let bad = canonical_tag!("Trigger");
/// ```
#[macro_export]
macro_rules! canonical_tag {
    ($lit:literal) => {{
        // Binding a `const` item forces const evaluation, so an invalid
        // literal is a compile error in every context. `ValidatedStaticTag`
        // is `Copy`, so this `Result` has no destructor and is const-usable.
        #[allow(clippy::panic)]
        const VALIDATED: $crate::id::ValidatedStaticTag =
            match $crate::id::CanonicalTag::validate_static($lit) {
                Ok(validated) => validated,
                Err(_) => panic!("canonical_tag! literal is not valid canonical tag syntax"),
            };
        $crate::id::CanonicalTag::from_validated(VALIDATED)
    }};
}

impl CanonicalTag {
    pub fn new(value: impl Into<String>) -> Result<Self, StableIdError> {
        let value = value.into();
        validate_canonical_syntax(&value, MAX_CANONICAL_TAG_LEN, 1)?;
        Ok(CanonicalTag(Cow::Owned(value)))
    }

    /// Builds a tag from a `&'static str`, **totally**: an invalid value
    /// returns a typed error in every context, const or otherwise.
    ///
    /// For a literal, prefer [`canonical_tag!`](crate::canonical_tag),
    /// which forces const evaluation so an invalid literal is a compile
    /// error. For any value that is not a literal, this and
    /// [`CanonicalTag::new`] both validate and return a typed error;
    /// neither panics.
    ///
    /// ```
    /// use spark_core::id::{CanonicalTag, StaticTagError};
    /// assert!(CanonicalTag::try_from_static("trigger").is_ok());
    /// // The case an independent reviewer executed as a runtime panic:
    /// assert_eq!(
    ///     CanonicalTag::try_from_static("INVALID TAG"),
    ///     Err(StaticTagError::InvalidByte { byte: b'I' })
    /// );
    /// ```
    pub const fn try_from_static(value: &'static str) -> Result<Self, StaticTagError> {
        match Self::validate_static(value) {
            Ok(validated) => Ok(Self::from_validated(validated)),
            Err(e) => Err(e),
        }
    }

    /// Validates a `&'static str` as canonical tag syntax, yielding a
    /// [`ValidatedStaticTag`] proof on success.
    ///
    /// This is the const-evaluable half of the literal path: its result
    /// type is `Copy`, so it can be bound to a `const` item, which is what
    /// [`canonical_tag!`](crate::canonical_tag) does to turn an invalid
    /// literal into a compile error.
    pub const fn validate_static(
        value: &'static str,
    ) -> Result<ValidatedStaticTag, StaticTagError> {
        match validate_canonical_tag_bytes(value.as_bytes()) {
            Ok(()) => Ok(ValidatedStaticTag(value)),
            Err(e) => Err(e),
        }
    }

    /// Builds a tag from a validation proof. Total and infallible: a
    /// [`ValidatedStaticTag`] cannot exist unless it passed
    /// [`CanonicalTag::validate_static`], so there is no input for which
    /// this could fail — and therefore none for which it could panic.
    pub const fn from_validated(validated: ValidatedStaticTag) -> Self {
        CanonicalTag(Cow::Borrowed(validated.as_str()))
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
                validate_canonical_tag_bytes(case.as_bytes()).is_ok(),
                CanonicalTag::new(case).is_ok(),
                "const and runtime validation disagree on {case:?}"
            );
        }
        let at_limit = "k".repeat(MAX_CANONICAL_TAG_LEN);
        let over_limit = "k".repeat(MAX_CANONICAL_TAG_LEN + 1);
        assert!(validate_canonical_tag_bytes(at_limit.as_bytes()).is_ok());
        assert!(validate_canonical_tag_bytes(over_limit.as_bytes()).is_err());
    }

    #[test]
    fn const_tag_and_runtime_tag_are_equal_values() {
        const TRIGGER: CanonicalTag = crate::canonical_tag!("trigger");
        assert_eq!(TRIGGER, CanonicalTag::new("trigger").unwrap());
        assert_eq!(TRIGGER, CanonicalTag::try_from_static("trigger").unwrap());
    }

    /// AT-C1, the executed panic this replaces: an invalid `&'static str`
    /// reaching the static constructor outside a const context returns a
    /// typed error. There is no longer any input, static or otherwise,
    /// for which a public canonical tag constructor panics.
    #[test]
    fn try_from_static_is_total_at_runtime() {
        assert_eq!(
            CanonicalTag::try_from_static("INVALID TAG"),
            Err(StaticTagError::InvalidByte { byte: b'I' })
        );
        assert_eq!(
            CanonicalTag::try_from_static(""),
            Err(StaticTagError::Empty)
        );
        assert_eq!(
            CanonicalTag::try_from_static(".trigger"),
            Err(StaticTagError::LeadingOrTrailingSeparator)
        );
        assert_eq!(
            CanonicalTag::try_from_static("trigger."),
            Err(StaticTagError::LeadingOrTrailingSeparator)
        );
        assert_eq!(
            CanonicalTag::try_from_static("trigger..evaluate"),
            Err(StaticTagError::EmptySegment)
        );
        // A `&'static str` longer than the bound is reachable at runtime
        // (e.g. by leaking a `String`); it must report, not panic.
        let oversized: &'static str =
            Box::leak("k".repeat(MAX_CANONICAL_TAG_LEN + 1).into_boxed_str());
        assert_eq!(
            CanonicalTag::try_from_static(oversized),
            Err(StaticTagError::TooLong {
                len: MAX_CANONICAL_TAG_LEN + 1,
                max: MAX_CANONICAL_TAG_LEN
            })
        );
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
