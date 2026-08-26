//! `spark-profile` — profile manifest logical types, a validation
//! skeleton, canonical content hashing/definition fingerprints, a
//! profile-qualified definition identity registry, and a minimal
//! content-addressed config revision type.
//!
//! Per ADR-0001, `spark-profile` may depend on `spark-core` only. The
//! exact on-disk profile file format remains provisional
//! (ADR-0004); this crate defines the deterministic, format-independent
//! logical representation used for validation and content identity, and
//! implements only enough loader/validation structure to support
//! Phase-1 fixtures, per the implementation brief §2 item 11-13 as
//! corrected by `PHASE_1_CORRECTION_BRIEF_v0.1.md`.

#![forbid(unsafe_code)]

pub mod config;
pub mod definition;
pub mod identity;
pub mod manifest;
pub mod validate;
