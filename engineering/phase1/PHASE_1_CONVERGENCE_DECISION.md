# S.P.A.R.K. Phase 1 — Convergence Decision and Re-Foundation Plan

**Date:** 2026-08-26  
**Decision:** STOP CORRECTION LOOP / RE-FOUND PHASE 1  
**Phase 2:** NOT AUTHORIZED

## Trigger

The independent correction re-review returned:

```text
REVISE_PHASE_1
B-01 OPEN
B-02 OPEN
B-03 OPEN
B-04 CLOSED
M-01 OPEN
M-02 OPEN
M-03 OPEN
M-04 CLOSED
m-01 CLOSED
PHASE-2 AUTHORIZATION: NO
```

The same foundational defect classes survived the bounded correction:

1. canonical identity/finality;
2. authority/schema provenance;
3. scheduler semantic identity.

Per the established convergence rule, no second correction pass is authorized.

## Decision

Preserve the existing implementation and evidence unchanged.

Create a fresh Phase-1 re-foundation line that reuses only independently closed/noncontroversial components and treats the reviewer failure corpus as executable acceptance criteria before implementation.

This is not a new architecture and does not reopen Phase 0.

## Rebuild boundary

Rewrite/re-found:

- canonical timeline ingress/finality representation;
- activation/schema provenance boundary;
- state-store construction and schema trust;
- scheduler semantic work key;
- config revision validation/canonical construction;
- replay/state digest semantics affected by admission metadata;
- canonical construction/bounds that remain open.

May reuse after explicit inspection:

- stable clock concepts;
- hashing primitive/encoder where unambiguous;
- B-04 definition identity registry/atomic validation;
- M-04 profile/artifact-qualified random address;
- closed dependency direction changes;
- closed tests that still prove valid properties.

Do not mechanically copy affected implementation from the failed line.

## Process change

The re-foundation is test-first:

1. encode every surviving Codex adversarial counterexample as a permanent regression test;
2. confirm those tests fail against the current implementation;
3. rewrite affected foundation modules;
4. make the adversarial corpus pass without weakening it;
5. run original Phase-1 corpus and full workspace validation;
6. independent review by a different agent.

## Writer / reviewer separation

**Writer:** Claude Code — Opus — HIGH effort  
**Independent reviewer:** Codex — HIGH effort

Sonnet is not used for this re-foundation writer pass because the same foundational implementation classes survived its bounded correction.

## Hard stop

If the re-foundation writer concludes that any acceptance test conflicts with a frozen Phase-0 contract, stop and escalate with the exact contradiction. Do not redesign silently.

If the independent re-review finds the same foundational classes again after this re-foundation, Phase 1 will be escalated to operator architecture/process review rather than another implementation attempt.
