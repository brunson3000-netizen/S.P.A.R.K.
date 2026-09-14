# temporal — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [temporalio/temporal](https://github.com/temporalio/temporal/tree/9ab3a9f770da20df7d94bcc0030f28eec7b0b947). Exact pin: `9ab3a9f770da20df7d94bcc0030f28eec7b0b947`. Runtime: Go workflow service with durable backend. License evidence: MIT; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Workflow operators need restartable long-running work; cancellation requests update durable workflow history and schedule follow-up processing.

Service namespaces, workflow/run IDs and persisted history determine workflow state. A cancellation request is not itself termination of an external activity process.

## Mechanism and failure semantics

RequestCancelWorkflowExecution validates namespace and runs through GetAndUpdateWorkflowWithNew. Already-completed execution returns success without another cancellation effect; active execution checks first-run identity and records cancellation state/event with a new workflow task when needed.

## Evidence and tests

Focused handler screen; worker activity cancellation, acknowledgement loss, external side-effect fencing and server persistence tests not run.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [service/history/api/requestcancelworkflow/api.go](https://github.com/temporalio/temporal/blob/9ab3a9f770da20df7d94bcc0030f28eec7b0b947/service/history/api/requestcancelworkflow/api.go)
- [LICENSE](https://github.com/temporalio/temporal/blob/9ab3a9f770da20df7d94bcc0030f28eec7b0b947/LICENSE)

## MCI transfer

Borrow durable intent/event and run-identity checks in COORD; do not embed the distributed Temporal service merely to supervise local agents. Rust consumer should preserve its own authoritative work IDs and epochs. Test late worker completion and duplicate external effects before using retries.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: COORD; cross-references: AUDIT, TEST. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
