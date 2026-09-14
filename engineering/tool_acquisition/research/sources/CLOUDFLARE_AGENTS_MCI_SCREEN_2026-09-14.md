# cloudflare-agents — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [cloudflare/agents](https://github.com/cloudflare/agents/tree/46760e635ce9599add0abbfe6c1a34af0d5d44f1). Exact pin: `46760e635ce9599add0abbfe6c1a34af0d5d44f1`. Runtime: TypeScript Cloudflare Workers/Durable Objects. License evidence: MIT; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Hosted workflow developers need durable waits; an approval event resumes a workflow step or raises rejection.

Cloudflare workflow/event and Durable Object state underpin continuity. An approved boolean in an event is not, alone, authenticated operator authority.

## Mechanism and failure semantics

waitForApproval calls step.waitForEvent with step name, event type and optional timeout. A rejected payload reports an error and throws WorkflowRejectedError; an approved payload returns metadata; the event resource is disposed in finally.

## Evidence and tests

Specific workflow wait helper inspected; event producer authentication and full platform lifecycle not traced or run.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [README.md](https://github.com/cloudflare/agents/blob/46760e635ce9599add0abbfe6c1a34af0d5d44f1/README.md)
- [packages/agents/src/workflows.ts](https://github.com/cloudflare/agents/blob/46760e635ce9599add0abbfe6c1a34af0d5d44f1/packages/agents/src/workflows.ts)
- [LICENSE](https://github.com/cloudflare/agents/blob/46760e635ce9599add0abbfe6c1a34af0d5d44f1/LICENSE)

## MCI transfer

Archive the durable waiting interface for MCI operator-state design. Cloudflare runtime/service dependencies make direct local Rust reuse a poor fit. Any independent implementation needs exact action binding, expiry, replay protection and host authorization of approval submission.

Finding classes: reference/qualification assessment. Filing home: COORD; cross-references: GOV, CONTEXT, PROTO. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

ARCHIVE_REFERENCE
