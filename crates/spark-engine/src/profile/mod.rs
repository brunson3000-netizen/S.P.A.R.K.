//! Profile manifest logical types: the **untrusted authoring input** to
//! the activation ceremony.
//!
//! ADR-0004 leaves the exact on-disk profile syntax provisional; these
//! types are the deterministic, format-independent logical representation
//! used for validation and content identity. Authority facts legitimately
//! *originate* as operator-authored profile data, so carrying them in a
//! public, freely constructible input struct is correct. What was wrong in
//! the failed passes was accepting those facts at a trusted mint without
//! the full ceremony — see [`crate::activation`], which is the only place
//! a definition's identity is ever computed or committed.

pub mod config;
pub mod definition;
pub mod manifest;
pub mod text;
