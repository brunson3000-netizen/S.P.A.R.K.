//! `spark-profile` — profile manifest logical types, the validation and
//! activation path, canonical content hashing/definition fingerprints,
//! bounded profile text, and a validated content-addressed config
//! revision type.
//!
//! Re-founded per `PHASE_1_REFOUNDATION_BRIEF_v0.1.md`. The
//! profile-qualified definition identity registry now lives in
//! [`spark_core::activation`], because immutable authority identity is a
//! canonical kernel invariant (ADR-0002/ADR-0004) rather than a profile
//! file-format concern — and because keeping it there is what lets the
//! kernel refuse to build a `StateStore` from anything that did not pass
//! through it. This crate drives that ceremony via
//! [`validate::validate`], which is the only producer of a
//! [`validate::ValidatedManifest`].
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
pub mod manifest;
pub mod text;
pub mod validate;
