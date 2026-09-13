# Pattern Card — Declarative Workload Security Envelope

PATTERN: Versioned Workload Contract With Runtime Confinement
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- ToolHive @ `e532cf07d45fa99f3e4e63819396a3e9c9fd763f`
- `pkg/api/v1/workload_service.go`
- `pkg/runner/config.go`
- `docs/arch/05-runconfig-and-permissions.md`
- container/network runtime implementation and tests
- convergence: CLI-Anything tool contracts, grok-build host-owned policy, planned Rust Tool Contract v0.

## Problem

A tool adapter is not safely defined by its command/schema alone. The host also needs an inspectable answer to: what executable runs, what filesystem/network/secret authority it receives, what transport it exposes, what middleware surrounds it, and how that exact configuration can be restarted or audited.

## Mechanism

Represent one runnable capability as a versioned declarative contract containing, at minimum:
- stable identity/version/provenance
- executable/image/adapter identity
- transport/endpoints
- arguments/environment
- filesystem read/write grants
- network mode/egress grants
- privileged/device grants
- secrets by reference, not value
- middleware/policy hooks
- tool exposure/alias rules
- evidence/telemetry configuration.

Before start:
1. normalize and validate the contract
2. reject contradictory confinement requests
3. apply host policy
4. persist the exact resolved contract
5. launch from that contract.

After start, inspect the actual runtime state and compare it with the contract.

## Benefits

- restartable deterministic configuration
- pre-execution review is possible
- authority is explicit rather than inferred from tool prose
- secrets can remain references
- runtime drift becomes measurable
- one contract can support container, process or remote-adapter backends.

## Risks / failure modes

- configuration claims confinement the backend did not actually apply
- permissive defaults silently widen a missing field
- runtime-specific escape hatch bypasses common policy
- mutable image/tag resolves to different executable later
- secret values leak into exported configuration
- tool visibility config is mistaken for execution authority.

## Boundaries crossed

Registry/source → resolved workload contract.
Contract → host policy.
Host policy → runtime adapter/container.
Runtime → applied-state evidence.

## Determinization relevance

CRITICAL. Resolution, validation, authority grants, launch and applied-state checks should be deterministic Rust host machinery.

## Likely architectural location

SPARK Tool Contract / Adapter RunSpec / sandbox-launch boundary.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

EXPERIMENT_NOW

## Required invariants

1. Missing fields have explicit fail-safe defaults.
2. Contradictory requested confinement cannot silently degrade.
3. Source/image identity is immutable or digest-bound at activation.
4. Secret material is not serialized into the portable contract.
5. Runtime start occurs only after host policy accepts the resolved contract.
6. Applied filesystem/network/privilege state is independently inspectable.
7. A restart uses the recorded resolved contract, not conversational reconstruction.
