# OPENSHELL — Enforcement evidence and compatibility modes

Status: SCOPED CODE STUDY COMPLETE; runtime qualification not executed.
Study date: 2026-09-14. Exact pin: `5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0`.
Upstream: https://github.com/NVIDIA/OpenShell
Language/license at inspected root: Rust / Apache-2.0. Individual dependency/reuse terms still need qualification.
Finding class: PATTERN / TEST-INVARIANT unless the disposition says idea/reference.
Primary disposition: BORROW_PATTERN.

## Finding

Linux sandbox preparation opens Landlock path descriptors before privilege drop, then enforcement calls restrict_self after privilege drop and applies seccomp through the Linux sandbox module. HardRequirement rejects an empty path policy. BestEffort can log unavailable/failed Landlock and continue, so policy declaration does not alone prove confinement.

## Traced paths

1. Sandbox policy → linux::prepare / prepare_current_user → Landlock ruleset → linux::enforce → landlock::enforce → restrict_self; failure treatment depends explicitly on compatibility mode.
2. Policy service imports workspace authorization and atomic revision storage, and exposes policy status separately. Its broad service surface was mapped, not credited as a complete revision-to-all-processes revocation trace.

## Evidence

- [crates/openshell-supervisor-process/src/sandbox/linux/landlock.rs](https://github.com/NVIDIA/OpenShell/blob/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0/crates/openshell-supervisor-process/src/sandbox/linux/landlock.rs)
- [crates/openshell-supervisor-process/src/sandbox/linux/mod.rs](https://github.com/NVIDIA/OpenShell/blob/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0/crates/openshell-supervisor-process/src/sandbox/linux/mod.rs)
- [crates/openshell-server/src/grpc/policy.rs](https://github.com/NVIDIA/OpenShell/blob/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0/crates/openshell-server/src/grpc/policy.rs)
- [e2e/rust/tests/landlock.rs](https://github.com/NVIDIA/OpenShell/blob/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0/e2e/rust/tests/landlock.rs)

Inline path/compatibility tests inspected. hard_requirement_accepts_enriched_device_path E2E test reads allowed random bytes and writes an allowed temp path; it is not an escape-denial proof. No tests executed.

## MCI assessment

Borrow explicit prepare/enforce split and machine-visible enforcement outcome. Any MCI use requiring Landlock must select and verify the required enforcement behavior. Linux Landlock/seccomp cannot establish native Windows parity.

## Limits and follow-up

Remaining: network authorization/revocation, running-process policy change, cross-platform substrate, privileged setup cost, physical confinement probes and enforcement acknowledgement. No OpenShell deployment or integration.

These conclusions are research recommendations. No copied implementation, installed dependency, adopted policy or independent assurance is claimed.
