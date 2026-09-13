# Pattern Card — Provenance Pin Before Activation

PATTERN: Bind External Package Content and Signer Before Reuse/Activation
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH for the principle; exact trust-root design remains open

## Source evidence
- ToolHive @ `e532cf07d45fa99f3e4e63819396a3e9c9fd763f`
- `pkg/skills/skillsvc/verify.go`
- `pkg/skills/lockfile/`
- `docs/arch/12-skills-system.md`
- convergence: SPARK harvest exact git pins; grok-build repository trust; Agent Skills provenance gap.

## Problem

A human-readable package name, mutable tag or previously trusted location does not prove that newly retrieved bytes come from the same trusted source. External tools/skills can change ownership, tags, mirrors or signer identity.

## Mechanism

Before project-level activation:
1. resolve immutable content identity/digest
2. obtain expected provenance from a trusted catalog, existing lock or explicit external trust anchor
3. verify the downloaded artifact against that expectation
4. refuse silent trust-anchor changes
5. record exact source + digest + provenance decision in durable lock state
6. require future sync/upgrade to satisfy the recorded trust anchor or enter an explicit re-qualification path.

On true first use where no external expectation exists, trust-on-first-use can record observed signer identity, but this is weaker and must be labeled as such.

## Benefits

- prevents mutable-tag/signing-identity drift from becoming invisible activation
- cleanly separates human package identity from cryptographic/content identity
- future upgrades become comparable against an explicit previous trust decision
- provides durable provenance for evidence and incident review.

## Risks / failure modes

- TOFU can faithfully pin an attacker on first contact
- warn-only verification can be mistaken for a passing gate
- unsigned exceptions become sticky ambient policy
- catalog itself is compromised
- re-anchor workflow is too easy and normalizes signer churn
- signatures prove origin/key possession, not software safety.

## Boundaries crossed

Registry/catalog → trust expectation.
Downloaded artifact → verifier.
Verifier → project lock/provenance record.
Lock → later install/activation gate.

## Determinization relevance

HIGH. Digesting, signature verification, constraint matching and lock comparison should be host machinery.

## Likely architectural location

SPARK source qualification / acquisition lock / activation gate.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. Mutable name/tag never substitutes for immutable content identity.
2. Artifact cannot select its own trust policy.
3. Recorded trust-anchor change requires explicit re-qualification.
4. Verification failure or unavailable required verifier fails closed.
5. TOFU, catalog-anchored, key-pinned and unsigned states are distinguishable.
6. Signature/provenance success is necessary evidence where required, never sufficient evidence of safe behavior.
