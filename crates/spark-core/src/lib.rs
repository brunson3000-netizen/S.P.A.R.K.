//! `spark-core` — the canonical deterministic S.P.A.R.K. kernel.
//!
//! This crate implements only what Phase 1 authorizes
//! (`engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md` §2): immutable
//! namespaced IDs, scope identifiers, canonical value types, the
//! authority model and its write-safe `StateStore`, a monotonic logical
//! clock, a due-work scheduler skeleton, a semantic random-address
//! service, and the canonical timeline ingress (sequencer authority,
//! admission window, staging, fencing, epoch reset).
//!
//! Per ADR-0001, `spark-core` may depend only on approved deterministic
//! utility libraries and must never depend on another S.P.A.R.K.
//! product-layer crate (service transport, host adapters, expression,
//! voice, ...). `#![forbid(unsafe_code)]` enforces the Phase-1
//! constraint that no `unsafe` appears in the canonical kernel.

#![forbid(unsafe_code)]

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
