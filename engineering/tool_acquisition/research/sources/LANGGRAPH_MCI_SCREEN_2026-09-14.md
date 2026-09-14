# langgraph — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [langchain-ai/langgraph](https://github.com/langchain-ai/langgraph/tree/e539ac122f4126f6dd850581c1494948cf620e31). Exact pin: `e539ac122f4126f6dd850581c1494948cf620e31`. Runtime: Python graph execution / checkpoint ecosystem. License evidence: MIT; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Workflow developers need resumable graphs and human interrupts; graph state, commands and checkpoint settings yield resumed computation.

Checkpointed workflow state and interrupt IDs drive execution order. They do not authenticate approval or fence external effects.

## Mechanism and failure semantics

interrupt resumes by re-running the node from its beginning and matches resume values by interrupt order. Durability exposes sync, async and exit modes. TimedAttemptScope guards writes with an active flag and lock; timed-out attempt writes can be ignored, which is distinct from killing its external work.

## Evidence and tests

Interrupt/retry implementation inspected; no checkpoint backend, process cancellation or model workload run.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [libs/langgraph/langgraph/types.py](https://github.com/langchain-ai/langgraph/blob/e539ac122f4126f6dd850581c1494948cf620e31/libs/langgraph/langgraph/types.py)
- [libs/langgraph/langgraph/pregel/_retry.py](https://github.com/langchain-ai/langgraph/blob/e539ac122f4126f6dd850581c1494948cf620e31/libs/langgraph/langgraph/pregel/_retry.py)
- [LICENSE](https://github.com/langchain-ai/langgraph/blob/e539ac122f4126f6dd850581c1494948cf620e31/LICENSE)

## MCI transfer

Borrow explicit resume/replay semantics for COORD and display interruption as a state transition in MCI. Make pre-interrupt effects idempotent or place them behind durable host admission. A suppressed late graph write does not prove a subprocess or network effect stopped.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: COORD; cross-references: GOV, CONTEXT, TEST. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
