# Pattern Card — Subagents Share Backends, Not Root Authority

PATTERN: Subordinate Agents Share Host Backends Under Clamped Authority
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH for local subagents; remote Grok Bot backend semantics remain contract-only

## Source evidence
- xAI grok-build @ `37949780c144e37df692e3d669051a21fec24f20`
- `xai-grok-shell/src/agent/subagent/mod.rs`
- `subagent/handle_request.rs`
- `agent/config.rs` + config/subagent tests
- task coordinator/admission types
- `xai-computer-hub-core/src/bot_tools.rs` for the remote/harness continuation contract

## Problem

Child agents need the same filesystem, terminal, tools and project context as the parent, but giving each child independent runtime ownership or self-chosen permissions creates authority inflation, orphaned work and uncontrolled recursion.

## Mechanism

Host coordinator owns child lifecycle and resource limits.

Child session:
- reuses parent/root filesystem, terminal, process scope, hooks and selected shared services
- receives a resolved/clamped permission mode
- inherits only explicitly permitted capabilities/config
- cannot silently enable parent-disabled user-interaction/tool behavior
- is bounded by nesting depth, concurrency and sampling limits
- reports lifecycle/result state through coordinator-owned channels.

For asynchronous remote-agent interactions, expose a continuation handle and explicit await primitive instead of forcing the model to re-send or poll transcript state.

## Dependencies

- central child coordinator actor
- parent/root resource handles
- current permission policy
- explicit capability inheritance map
- cancellation/deadline support
- nesting/concurrency limits
- lifecycle/result channel
- stable continuation identity for async turns.

## Benefits

- same project truth/backends without duplicate state ownership
- parent hard limits remain effective
- child cleanup follows root lifetime
- bounded fan-out/recursion
- asynchronous work can be resumed without duplicate prompts/jobs
- child agents remain workers rather than autonomous security principals by accident.

## Risks / failure modes

- shared backend interpreted as shared unrestricted authority
- child definition requests a more permissive mode than parent/admin policy
- nested children orphan processes/tasks
- inherited hooks/config differ from actual parent gate
- unlimited depth/concurrency causes resource exhaustion
- timeout leads model to re-send work instead of continuing existing turn
- continuation handles lack generation/version binding, allowing stale-handle ambiguity.

## Boundaries crossed

Parent → child: context/config/capability inheritance.
Child → host backend: same call-time authority path as parent.
Child → root process/task scope: host-owned lifecycle.
Async child turn → parent/model: continuation handle rather than duplicate action.

## Agent-facing surface

Small lifecycle verbs are preferable:
- spawn/submit
- await/continue by handle
- inspect bounded result/transcript
- cancel/terminate when authorized.

Do not expose coordinator implementation state unless needed for a decision.

## Determinization relevance

VERY HIGH. Scheduling, admission, limits, cancellation, handle freshness and capability inheritance are host state-machine work.

## Likely architectural location

Supervisor/worker coordinator below the MCI-facing agent interface.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. Child cannot exceed parent/admin hard authority.
2. Capability inheritance is explicit, not “copy everything.”
3. Root/parent owns process and task cleanup.
4. Depth/concurrency/sampling are bounded by host state.
5. Child calls traverse the same host authorization gates as equivalent parent calls.
6. Async continuation never requires re-sending the original operation after a timeout.
7. Continuation/turn handles must be bound to stable child identity and freshness/generation before SPARK adopts that portion.
