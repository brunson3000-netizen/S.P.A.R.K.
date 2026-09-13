# Pattern Card — Continuation Handles Are References, Not Grants

PATTERN: Cancellable Correlated Continuation Handle
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- rmcp @ `3075dc9152d4678775f20634fcb467a7b995dbab`
- `service.rs::RequestHandle`
- SEP-2663 `TaskManager`/task models and end-to-end tests
- subscription request IDs
- convergence: grok-build bot `await_turn` contract.

## Problem

Long-running/async work needs a way to continue waiting, poll, supply input or cancel without re-sending the original operation. If the returned handle is treated as authorization, anyone possessing a correlation token can potentially control work they do not own.

## Mechanism

Represent continuation as a typed reference containing enough identity for lifecycle correlation:
- request/task/worker identifier
- optional generation/version
- timeout/progress state
- operation status/location.

Each continuation action (`get`, `await`, `update`, `cancel`) resolves the reference, then independently rechecks current subject/resource authority before acting.

Cancellation is cooperative at the protocol/runtime level; acknowledgement is not proof the underlying side effect has stopped until terminal state confirms it.

## Benefits

- prevents duplicate work after timeouts
- supports explicit async lifecycle
- model can resume instead of polling arbitrary output
- authority can be revoked while work continues
- stale/foreign handles can be rejected deterministically.

## Risks / failure modes

- task ID used as bearer secret/grant
- no generation binding, so stale ID aliases newer work
- cancellation ack mistaken for completed cancellation
- handle outlives retained task/evidence
- task state held only in process memory and lost after crash.

## Boundaries crossed

Agent → continuation API.
Continuation reference → durable work registry.
Work registry → current authority decision.
Cancellation request → running operation → terminal evidence.

## Determinization relevance

VERY HIGH. Handle resolution, freshness, ownership, lifecycle and cancellation are host state-machine responsibilities.

## Likely architectural location

Worker/job broker and federation/mailbox layer.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. Possessing a handle does not grant inspect/update/cancel authority.
2. Every continuation operation reauthorizes current subject/resource state.
3. Handle binds stable job/worker identity plus freshness/generation where reuse is possible.
4. Timeout never causes automatic duplicate execution.
5. Cancellation acknowledgement and terminal cancellation are separate states.
6. Canonical work state survives the process lifetime when the mission requires recovery.
