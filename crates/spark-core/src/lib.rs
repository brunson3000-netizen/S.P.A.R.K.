//! `spark-core` — the canonical deterministic S.P.A.R.K. kernel.
//!
//! This crate implements only what Phase 1 authorizes
//! (`engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md` §2): immutable
//! namespaced IDs, scope identifiers, canonical value types, the
//! authority model, the trusted definition-identity/schema activation
//! ceremony and the activation-bound `StateStore`, a monotonic logical
//! clock, a due-work scheduler skeleton, a semantic random-address
//! service, and the canonical timeline ingress (sequencer authority,
//! admission window, staging, fencing, epoch reset).
//!
//! Phase 1 was re-founded per
//! `engineering/phase1/PHASE_1_REFOUNDATION_BRIEF_v0.1.md` after two
//! independent reviews found the same three foundational defect classes
//! surviving a bounded correction. The re-foundation moved three
//! invariants from convention into the type system:
//!
//! - [`timeline`] separates [`timeline::SemanticCommandEnvelope`] from
//!   [`timeline::AdmissionTicket`], so ephemeral admission timing cannot
//!   reach canonical identity or replay digests;
//! - [`activation`] is the sole producer of an
//!   [`activation::ActivatedSchema`], and computes definition
//!   fingerprints itself, so authority metadata cannot be fabricated by
//!   external code;
//! - [`scheduler`] keys work by its complete semantic
//!   [`scheduler::WorkKey`], so independent producers and scopes keep
//!   independent occurrence sequences.
//!
//! Per ADR-0001, `spark-core` may depend only on approved deterministic
//! utility libraries and must never depend on another S.P.A.R.K.
//! product-layer crate (service transport, host adapters, expression,
//! voice, ...). `#![forbid(unsafe_code)]` enforces the Phase-1
//! constraint that no `unsafe` appears in the canonical kernel.

#![forbid(unsafe_code)]

pub mod activation;
pub mod authority;
pub mod clock;
pub mod hash;
pub mod id;
pub mod random;
pub mod scheduler;
pub mod scope;
pub mod state;
pub mod timeline;
pub mod value;
