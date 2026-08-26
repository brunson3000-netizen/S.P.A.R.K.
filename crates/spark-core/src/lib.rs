//! `spark-core` — the canonical deterministic S.P.A.R.K. kernel.
//!
//! This crate implements only what Phase 1 authorizes
//! (`engineering/phase1/PHASE_1_IMPLEMENTATION_BRIEF.md` §2) *and* only
//! what belongs below the trust boundary: immutable namespaced IDs and
//! bounded canonical tags, scope identifiers, canonical value types, the
//! authority vocabulary, a monotonic logical clock, a semantic-key due-work
//! scheduler, a semantic random-address service, and the canonical
//! timeline ingress (sequencer authority, admission window, staging,
//! fencing, epoch reset).
//!
//! # Re-Foundation v2: the kernel holds no trust ceremony
//!
//! Phase 1 failed independent review three times on the same defect
//! expressed in three places: trust was written as a convention layered on
//! a public surface instead of being made coextensive with a crate
//! boundary. Each fix privatized the artifact one step downstream of a
//! public door, and the forgery moved to the door.
//!
//! Per `PHASE_1_FABLE_ARCHITECTURE_PROCESS_REVIEW_2026-08-26.md` §3, the
//! profile/activation/state machinery therefore no longer lives here at
//! all. It lives in `spark-engine`, which is one crate containing the
//! complete activation ceremony, every intermediate mint (all
//! `pub(crate)`), and the state runtime those mints produce. `spark-core`
//! retains the deterministic vocabulary and machines that carry no trust:
//!
//! - [`authority`] keeps the `Authority`/`WriteClass` vocabulary, but no
//!   enforcement point;
//! - there is no `activation` module and no `state` module — a caller
//!   holding only `spark-core` cannot name an activated schema, a state
//!   store, or any constructor for either.
//!
//! # The invariants this crate does own
//!
//! - [`timeline`] separates [`timeline::SemanticCommandEnvelope`] from
//!   [`timeline::AdmissionTicket`], so ephemeral admission timing cannot
//!   reach canonical identity or replay digests, and a contested ordinal
//!   slot is poisoned with order-independent bounded evidence rather than
//!   resolved by arrival.
//! - [`scheduler`] keys work by its complete semantic
//!   [`scheduler::WorkKey`], and a second distinct payload for one key
//!   poisons that key with an order-independent evidence set — no arrival
//!   order picks a winner (ADR-0003's own rule, applied to the scheduler).
//! - [`id`] and [`value`] make canonical construction total: bounded
//!   validated types, fallible runtime constructors, and compile-time
//!   literal macros. No public function in this crate panics, wraps, or
//!   saturates on caller input; the crate's `clippy` gate
//!   (`unwrap_used`, `expect_used`, `panic`, `indexing_slicing`,
//!   `arithmetic_side_effects`) enforces that mechanically rather than by
//!   audit.
//!
//! Per ADR-0001, `spark-core` may depend only on approved deterministic
//! utility libraries and must never depend on another S.P.A.R.K.
//! product-layer crate. `#![forbid(unsafe_code)]` enforces the Phase-1
//! constraint that no `unsafe` appears in the canonical kernel.

#![forbid(unsafe_code)]

pub mod authority;
pub mod clock;
pub mod hash;
pub mod id;
pub mod random;
pub mod scheduler;
pub mod scope;
pub mod timeline;
pub mod value;
