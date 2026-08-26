//! `spark-testkit` — deterministic fixtures and a scenario replay
//! harness for S.P.A.R.K. Phase-1 canonical behaviors
//! (`CONTROLLING_BLUEPRINT_v0.2.md` §8.3).
//!
//! Per ADR-0001 this crate may depend on `spark-core` and `spark-engine`
//! (it consumes their canonical types to build fixtures; it is not itself
//! part of the causal-evaluation dependency direction, so nothing in
//! `spark-core`/`spark-engine` may depend back on it as a normal
//! dependency - only as a `dev-dependency` for their own tests).
//!
//! It is also the crate that dev-enables the `test-support` feature on
//! both canonical crates, which is what gives its tests access to the
//! sanctioned fixture seams (`spark_engine::fixture`, the restricted
//! nonzero-frontier timeline constructor) without those seams existing on
//! any production surface. `tests/workspace_dependency_direction.rs`
//! asserts that hygiene mechanically.

#![forbid(unsafe_code)]

pub mod scenario;
