//! `spark-testkit` — deterministic fixtures and a scenario replay
//! harness for S.P.A.R.K. Phase-1 canonical behaviors
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §8.3).
//!
//! Per ADR-0001 this crate may depend on `spark-core` and `spark-profile`
//! (it consumes their canonical types to build fixtures; it is not
//! itself part of the causal-evaluation dependency direction, so nothing
//! in `spark-core`/`spark-profile` may depend back on it as a normal
//! dependency - only as a `dev-dependency` for their own tests).

#![forbid(unsafe_code)]

pub mod scenario;
