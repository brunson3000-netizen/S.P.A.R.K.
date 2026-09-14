# Paperclip — governance and primary coordination

Retrieved: 2026-09-14 UTC
Status: INTAKE — source study admitted, implementation not validated
Primary disposition: ARCHIVE_REFERENCE

## Admission decision
Add to MCI source-study intake. Strong governance fit and substantial accumulated repository attention. This is a candidate for later pattern extraction, not an adoption decision.

## Community signal
The repository has 80,603 stars at retrieval. This establishes accumulated attention, not current growth. Search surfaced Paperclip comparisons in a newer [company-OS discussion](https://news.ycombinator.com/item?id=49630606), but direct retrieval returned HTTP 429; do not assign that thread a verified date, score, or consensus. Independent fresh discussion evidence remains weaker than repository attention.

## Framing from pinned README
- PROBLEM / USER: coordinating multiple agents for an operator with delegated work and spending limits.
- INPUT / OUTPUT: goals, assignments, agent adapters and budgets become scheduled runs, task updates, cost events and audit records.
- AUTHORITY / TRUST: an operator/board can approve, pause and terminate agents; runtime adapters and injected secrets cross the execution boundary.
- STATE: documented durable wakeup queue, sessions, task state and configuration revisions.
- FAILURE: README describes orphan recovery and budget stops; crash/race behavior is unverified.

## MCI value and next traces
1. Assignment → checkout → heartbeat → adapter → durable completion. Inspect ownership races, duplicate wakeups, orphan recovery and cancellation tests.
2. Budget reservation → concurrent execution → cost reconciliation → stop. Determine whether a stop prevents new work, interrupts work, or merely records overspend.
3. Approval → versioned configuration → execution. Test stale approvals, delegated authority and revocation.

DETERMINIZATION_CANDIDATE: leases, budget arithmetic, deadline checks, policy-version checks and audit recording.
Potential location: MCI control plane, outside model reasoning. Finding class: IDEA. A UI approval or org chart is not evidence that every underlying operation enforces authority.

## Source record
### paperclipai/paperclip
- Upstream: https://github.com/paperclipai/paperclip
- Exact inspected commit: `13368c518303e886a5c9445fbc69afcbdf0a9228`
- Primary source: https://github.com/paperclipai/paperclip/blob/13368c518303e886a5c9445fbc69afcbdf0a9228/README.md
- Language: TypeScript; detailed runtime/dependencies not yet traced.
- License: MIT (GitHub metadata only; file-level review required before reuse).
- GitHub snapshot: 80603 stars; not a growth rate.

## Evidence limits and handoff
Pinned README and repository metadata inspected. No code paths or protecting tests traced; no runtime executed. Documentation claims are hypotheses. These refs are intake pins, not canonical harvest-lock/gitlink additions. No increase to the previously recorded 48 harvested sources is claimed. Continue under RESEARCH_PROTOCOL.md; preserve the other thread's active trace queue.
