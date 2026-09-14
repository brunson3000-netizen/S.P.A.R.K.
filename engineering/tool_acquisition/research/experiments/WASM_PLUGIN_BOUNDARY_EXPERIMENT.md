# Experiment — Wasm Plugin Boundary: Raw Wasmtime vs Extism

Status: READY_FOR_IMPLEMENTATION — NOT RUN
Created: 2026-09-13

## Question

Can Extism materially reduce implementation/maintenance cost versus raw Wasmtime while preserving the same host-owned authority, evidence and deterministic bounded-execution contract?

This experiment is for runtime qualification only. It does not authorize adoption.

## Fixed test subject

Use one tiny deterministic Wasm plugin/component with functions that can:
- pure compute
- read/write a provided guest path
- attempt path escape
- loop forever during initialization
- loop forever during a normal call
- call one explicitly linked host function
- attempt a nonexistent/unlinked host function
- make HTTP requests when that capability is supplied.

Use identical plugin bytes/digest and identical logical grants for both variants wherever substrate semantics permit.

## Variant A — raw Wasmtime/WASI

Host constructs:
- verified module/component bytes only
- explicit WASI context
- explicit filesystem preopens/perms
- explicit network policy
- explicit host-function linker
- explicit StoreLimits/ResourceLimiter
- finite fuel
- wall deadline/cancel.

## Variant B — Extism

Host requirements:
- build with `default-features = false`
- do **not** enable `register-http` or `register-filesystem`
- supply already verified `Wasm::Data`
- enable WASI only when required
- host-generate manifest `allowed_paths`/HTTP policy
- finite initialization fuel + per-call fuel
- timeout/cancel
- explicit host-function set.

If the required capability cannot be represented at equal strength through Extism, record that as a result rather than weakening Variant A.

## Mandatory tests

### Source/provenance
1. Full SHA-256 (or stronger project-standard digest) of exact plugin bytes recorded.
2. Untrusted manifest file/URL source cannot cause host acquisition in the SPARK wrapper.
3. Wrong digest fails before instantiation.

### Zero-capability baseline
4. No host filesystem visibility.
5. No network.
6. No custom host functions.
7. Bounded memory/resources.

### Filesystem
8. Explicit RO preopen reads expected file.
9. RO preopen cannot write/truncate/create/link.
10. RW preopen writes only inside granted subtree.
11. `..`, symlink and alias escape attempts fail.

### Network
12. No network grant → request fails.
13. One explicit allowed destination succeeds.
14. Different host fails.
15. Different port/scheme is tested separately.
16. Allowed host redirecting to disallowed host is tested.
17. DNS resolution to loopback/private/link-local address is tested against intended policy.

Extism hostname filtering does not receive credit for cases it cannot express; an outer host/network control may be required.

### Host functions
18. Only explicitly granted host function is linked.
19. Missing import fails cleanly.
20. Host function attempt outside its grant is denied by host broker, not trusted because it came through Wasm.

### Compute/resource bounds
21. Infinite initialization stops at initialization-fuel ceiling.
22. Normal infinite call stops at per-call fuel ceiling.
23. A second ordinary call receives a fresh call budget.
24. Wall timeout/cancel stops a long-running call independently of fuel.
25. Memory growth above configured bound fails.
26. Repeated instantiation/calls remain within intended resource envelope.

### Evidence
27. Result bundle records:
   - plugin full digest + origin/provenance
   - runtime/library version
   - linked host-function set
   - preopens/perms
   - network policy
   - memory/count limits
   - init fuel + call fuel
   - deadline/cancel cause
   - exact trap/error/result class.
28. Applied runtime state is compared to resolved Tool Contract; no required authority dimension is accepted solely from declarative config.

## Measurements

For both variants record:
- host integration LOC and number of runtime-specific concepts
- compile/dependency footprint
- startup/compile time
- first-call and steady-call latency
- memory footprint where practical
- quality of deterministic tests
- clarity of applied-authority evidence
- number of hidden/default behaviors requiring override
- upgrade/security-maintenance burden estimate.

## Decision rule

Prefer raw Wasmtime unless Extism shows a meaningful implementation/maintenance reduction **and** passes the same authority/evidence requirements without hidden widening.

Extism convenience is not enough to win if SPARK must reach through the wrapper for most security-critical configuration anyway.

## Expected outputs

- experiment results
- failing/passing invariant table
- dependency/maintenance comparison
- runtime recommendation: raw Wasmtime / Extism / neither
- any new failure lessons or Tool Contract requirements.
