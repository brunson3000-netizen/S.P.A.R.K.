# Pattern Card — Declared vs Applied Confinement

PATTERN: Verify Runtime Authority Against Declared Authority
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- ToolHive @ `e532cf07d45fa99f3e4e63819396a3e9c9fd763f`
- RunConfig permission profiles and Docker/network implementation
- architecture guidance explicitly treating declared/applied gaps as security issues
- grok-build memory/permission work independently supports host state rather than prompt claims.

## Problem

A configuration saying “read-only”, “no network”, “isolated”, or “not privileged” is not evidence that the running process actually has those properties. Backend bugs, unsupported modes and restart paths can create drift between declared and applied authority.

## Mechanism

Treat the security contract and runtime observation as separate evidence:

1. resolve a declarative authority envelope
2. launch the workload
3. inspect actual runtime mounts/network/privilege/process/sandbox state
4. compare applied state against the resolved envelope
5. stop/quarantine on unauthorized widening
6. record both the requested/resolved state and applied evidence.

Runtime-specific incompatibilities must be surfaced before or during launch; they must not be represented as successfully applied confinement.

## Benefits

- detects backend enforcement defects
- makes security assertions testable
- catches restart/config migration regressions
- permits multiple runtime backends under one authority model
- creates concrete evidence for qualification and incident review.

## Risks / failure modes

- runtime inspection API itself is incomplete or stale
- race between inspection and mutation
- unsupported backend reports success without enforcement
- comparison ignores an authority dimension
- warnings are mistaken for enforcement.

## Boundaries crossed

Policy/contract → runtime.
Runtime → evidence/inspection.
Evidence → health/quarantine supervisor.

## Determinization relevance

CRITICAL. This should be a deterministic acceptance gate, not model judgment.

## Likely architectural location

Sandbox/adapter supervisor and qualification service.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

EXPERIMENT_NOW

## Required invariants

1. Applied authority can never exceed resolved authority without a hard failure/explicit new grant.
2. “Unsupported” is not translated into “enforced.”
3. Security-relevant runtime state is captured after launch and after restart/recovery.
4. Any deliberate degradation is explicit and attributable.
5. Warnings do not count as passing evidence for a required boundary.
