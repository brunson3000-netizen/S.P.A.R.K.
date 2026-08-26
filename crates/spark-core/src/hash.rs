//! Deterministic, format-independent content hashing.
//!
//! Every canonical hash in S.P.A.R.K. is produced by hashing an explicit,
//! caller-constructed byte sequence with a fixed field order. Callers must
//! never hash a `HashMap`/`HashSet` iteration order or any wall-clock/OS
//! value; see `CanonicalEncoder` for the length-prefixed encoding used to
//! avoid field-boundary ambiguity (e.g. "ab"+"c" colliding with "a"+"bc").

use std::fmt;

/// A 256-bit deterministic content digest.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Digest(pub [u8; 32]);

impl Digest {
    pub const ZERO: Digest = Digest([0u8; 32]);

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Digest({})", hex(&self.0))
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex(&self.0))
    }
}

fn hex(bytes: &[u8]) -> String {
    // Two hex characters per byte; saturating so the capacity hint
    // contains no unchecked arithmetic (it is only a hint either way).
    let mut s = String::with_capacity(bytes.len().saturating_mul(2));
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// Hash an already-canonical byte buffer.
///
/// This is the single low-level hashing primitive used throughout the
/// canonical kernel so that every content hash (timeline fences, admission
/// window tokens, profile manifests, definition fingerprints) shares one
/// deterministic, cross-platform algorithm.
pub fn hash_bytes(bytes: &[u8]) -> Digest {
    Digest(*blake3::hash(bytes).as_bytes())
}

/// Builds a canonical, length-prefixed byte buffer for hashing.
///
/// Length-prefixing every variable-length field prevents ambiguous
/// concatenation (`"ab"` + `"c"` must not hash identically to `"a"` +
/// `"bc"`). All multi-byte integers are encoded little-endian, which is
/// deterministic regardless of host endianness because the encoding always
/// produces the same byte order irrespective of the platform's native
/// representation.
#[derive(Default)]
pub struct CanonicalEncoder {
    buf: Vec<u8>,
}

impl CanonicalEncoder {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.buf
            .extend_from_slice(&(bytes.len() as u64).to_le_bytes());
        self.buf.extend_from_slice(bytes);
        self
    }

    pub fn push_str(&mut self, s: &str) -> &mut Self {
        self.push_bytes(s.as_bytes())
    }

    pub fn push_u64(&mut self, v: u64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn push_i64(&mut self, v: i64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn push_u32(&mut self, v: u32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn push_bool(&mut self, v: bool) -> &mut Self {
        self.buf.push(v as u8);
        self
    }

    pub fn push_digest(&mut self, d: &Digest) -> &mut Self {
        self.buf.extend_from_slice(&d.0);
        self
    }

    /// Push a nested canonical sub-encoding as a length-prefixed block, so
    /// composed structures cannot alias across their own boundaries either.
    pub fn push_block(&mut self, inner: &CanonicalEncoder) -> &mut Self {
        self.push_bytes(&inner.buf)
    }

    pub fn finish(&self) -> Digest {
        hash_bytes(&self.buf)
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
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
    fn length_prefixing_prevents_field_boundary_collision() {
        let mut a = CanonicalEncoder::new();
        a.push_str("ab").push_str("c");

        let mut b = CanonicalEncoder::new();
        b.push_str("a").push_str("bc");

        assert_ne!(a.finish(), b.finish());
    }

    #[test]
    fn same_input_produces_same_digest() {
        let mut a = CanonicalEncoder::new();
        a.push_str("x").push_u64(42);
        let mut b = CanonicalEncoder::new();
        b.push_str("x").push_u64(42);
        assert_eq!(a.finish(), b.finish());
    }
}
