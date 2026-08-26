//! `spark-profile` — profile manifest logical types, a validation
//! skeleton, and canonical content hashing/definition fingerprints.
//!
//! Per ADR-0001, `spark-profile` may depend on `spark-core` only. The
//! exact on-disk profile file format remains provisional
//! (ADR-0004); this crate defines the deterministic, format-independent
//! logical representation used for validation and content identity, and
//! implements only enough loader/validation structure to support
//! Phase-1 fixtures, per the implementation brief §2 item 11-13.

#![forbid(unsafe_code)]

pub mod definition;
pub mod manifest;
pub mod validate;
