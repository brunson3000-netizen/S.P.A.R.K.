//! `spark-engine` — **the Phase-1 trust boundary.**
//!
//! One crate holds the complete activation ceremony, every intermediate
//! mint, the unforgeable activation artifacts, and the state runtime those
//! artifacts produce. That is the whole point of the crate: a trust
//! ceremony is non-bypassable only if the ceremony *and every intermediate
//! mint* live in one crate, with exactly one public door that performs the
//! complete ceremony
//! (`PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md` §3.1).
//!
//! # Why this crate exists
//!
//! Phase 1's authority-provenance defect (B-02) recurred three times
//! because Rust's `pub` is `pub` for everyone: a kernel crate cannot
//! distinguish a trusted validator crate from an adversarial adapter. As
//! long as validation lived above the kernel and the mint lived inside it,
//! the cross-crate call *forced* the mint to be public — and a public mint
//! that consumes caller-authored authority facts is the bypass. Moving the
//! artifact one step downstream each time simply moved the forgery to the
//! next door.
//!
//! The resolution is structural rather than conventional:
//!
//! ```text
//! profile data                     [untrusted authoring input:
//!   DefinitionSpec / ProfileManifest / ConfigRevision]
//!     -> ActivationRegistry::activate    THE door - the complete ceremony,
//!                                        atomically, all errors reported
//!     -> ActivatedProfile                private fields, pub(crate) mint
//!     -> ActivatedProfile::into_state_store
//!     -> StateStore                      public read-only surface;
//!                                        pub(crate) write paths for the
//!                                        Phase-2 evaluator/host facades
//!                                        that will live in this crate
//! ```
//!
//! There is no partial-trust intermediate step left to call: the raw
//! declaration type is crate-private, `DefinitionKindTag::builtin` is
//! crate-private, there is no `DefinitionSpec::to_declaration`, and
//! neither [`activation::ActivatedProfile`],
//! [`activation::ActivatedDefinition`], nor [`state::StateStore`] has any
//! public constructor.
//!
//! # What this crate does *not* claim
//!
//! Rust cannot prevent a second `ActivationRegistry::new()` in the same
//! process, and pretending otherwise is what kept B-02 alive. The
//! invariant is made real by content addressing instead: every
//! [`activation::ActivatedProfile`] and every [`state::StateStore`]
//! carries both `manifest_content_hash` and `activation_hash`, and the
//! registry exposes [`activation::ActivationRegistry::lineage_digest`].
//! Two registries fed identical manifests are byte-identical, hence
//! interchangeable; two divergent registries differ visibly on every
//! downstream artifact, and ADR-0006 continuation refuses a mismatch.
//! Divergence therefore cannot be *silent* — see §3.5 of the architecture
//! review and the `divergent_activations_are_never_interchangeable`
//! acceptance test.
//!
//! # Layering
//!
//! Per ADR-0001, `spark-engine` depends on `spark-core` only, and nothing
//! in `spark-core` depends back on it. Profile *parsing* (the provisional
//! on-disk format, which Phase 1 does not implement) remains a separate
//! concern above this crate, so ADR-0001/ADR-0004 layering is preserved:
//! parsing sits above validation, validation sits above the deterministic
//! kernel, and the kernel contains no profile or trust machinery.
//! `#![forbid(unsafe_code)]` and the same `clippy` panic/arithmetic gate
//! as `spark-core` apply here.
//!
//! Phase 2's rule/effect evaluator lands in this crate — which is exactly
//! why the store's write paths are `pub(crate)` here rather than public
//! anywhere. Naming that landing zone is not authorization to implement
//! it.

#![forbid(unsafe_code)]

pub mod activation;
pub mod profile;
pub mod state;

#[cfg(any(test, feature = "test-support"))]
pub mod fixture;
