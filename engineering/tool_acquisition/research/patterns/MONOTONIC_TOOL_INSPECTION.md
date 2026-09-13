# Pattern Card — Monotonic Tool Inspection

PATTERN: Independent Safety Inspectors May Tighten, Never Broaden, Host Permission
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- Goose @ `50666ae0b9a51e260b52b7efbab2e4e020346e94`
- `crates/goose/src/tool_inspection.rs`
- `permission/permission_inspector.rs`
- security/egress/adversary inspectors
- convergence: grok-build ordered call-time permission pipeline.

## Problem

Security checks differ in reliability: some are deterministic policy, some pattern heuristics, some model judgments, some telemetry-only. Mixing them without explicit precedence can let a weak “allow” accidentally override a strong denial or make a failed optional detector look like authorization.

## Mechanism

1. Compute a host-owned baseline permission decision for each concrete tool call.
2. Run independent inspectors that emit attributed results:
   - Allow
   - RequireApproval
   - Deny
   plus reason/confidence/finding identity.
3. Fold results monotonically:
   - Deny may tighten any weaker result
   - RequireApproval may tighten Allow
   - Allow never loosens Deny/RequireApproval.
4. Label every inspector with an assurance/failure class, e.g.:
   - authoritative/fail-closed
   - authoritative/approval-on-error
   - defense-in-depth/fail-open
   - observation-only.
5. Preserve each finding separately in evidence/telemetry rather than collapsing to one unexplained score.

## Benefits

- adding a detector cannot silently widen authority
- multiple security techniques can coexist without pretending equal assurance
- optional model/ML detectors can add defense without becoming single points of failure
- decision evidence remains attributable
- deterministic host policy stays authoritative.

## Risks / failure modes

- permissive baseline (e.g. auto mode) becomes execution when all optional inspectors fail
- inspector outage/failure class is not surfaced
- all inspectors share the same parser blind spot
- remote annotations are trusted as hard evidence
- result aggregation ignores missing authoritative results.

## Boundaries crossed

Model/tool proposal → baseline authority.
Proposal/context → independent inspectors.
Inspector results → final admission partition.
Admission → executor.

## Determinization relevance

CRITICAL for aggregation/precedence. Model-based inspectors may exist, but their role and failure behavior are host-declared.

## Likely architectural location

SPARK Authority Broker / pre-execution inspection pipeline.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. No secondary Allow can override a stronger baseline or other inspector result.
2. Missing/failed authoritative inspector cannot be silently treated as Allow.
3. Every inspector has a declared failure policy and assurance class.
4. Final admission records the exact contributing decisions/reasons.
5. Model/ML confidence is never converted directly into wider hard authority.
6. Remote/provider annotations are advisory unless independently attested/validated by host policy.
