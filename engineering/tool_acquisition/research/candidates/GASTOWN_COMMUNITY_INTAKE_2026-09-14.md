# Gas Town — coordinator, lifecycle supervisor and merge authority

Retrieved: 2026-09-14 UTC
Status: INTAKE — source study admitted, implementation not validated
Primary disposition: ARCHIVE_REFERENCE

## Admission decision
Add to MCI source-study intake for its explicit role decomposition. Its attention is substantial but historical; the inspected tip was dated July 23, not September.

## Community signal
[Welcome to Gas Town on Hacker News](https://news.ycombinator.com/item?id=46458936) shows 354 points and 224 comments, labelled eight months old at retrieval. Discussion praises separation of coordination, operations and merging while questioning complexity, reliability, cost and how much human intervention remains. This is a useful contested architecture example, not evidence of present-week momentum.

## Framing from pinned README
- PROBLEM / USER: developers coordinating coding agents whose sessions can restart.
- INPUT / OUTPUT: work assignments become worker changes, tracked work and merged outputs.
- AUTHORITY: Mayor coordinates; Witness manages per-project worker lifecycles; Deacon supervises across projects; Refinery handles merging.
- TRUST: model decisions, local runtime processes and repository changes cross distinct boundaries.
- STATE: documented git-backed work hooks and persistent worker identities; sessions are ephemeral.
- FAILURE: documented patrol, stuck-worker recovery and escalation need implementation/test verification.

## MCI value and next traces
1. Mayor dispatch → durable assignment → worker restart → reclaim/resume. Inspect duplicate ownership and lost work.
2. Witness detection → Deacon escalation → restart/cancel. Inspect bounded retries and supervisor failure.
3. Worker completion → Refinery checks → merge. Determine who can change tests, bypass gates or incorrectly declare success.

DETERMINIZATION_CANDIDATE: liveness detection, leases, retry limits, capacity limits and merge prerequisites.
Potential location: MCI primary coordinator plus deterministic lifecycle controller. Finding class: IDEA.
Extract separation of roles rather than adopting the full organizational metaphor. The implementation burden and supervisor token cost are explicit study questions.

## Source record
### gastownhall/gastown
- Upstream: https://github.com/gastownhall/gastown
- Exact inspected commit: `649b832b7672bc7a2dbef26f5983aba6198b819b`
- Primary source: https://github.com/gastownhall/gastown/blob/649b832b7672bc7a2dbef26f5983aba6198b819b/README.md
- Language: Go; detailed runtime/dependencies not yet traced.
- License: MIT (GitHub metadata only; file-level review required before reuse).
- GitHub snapshot: 18045 stars; not a growth rate.

## Evidence limits and handoff
Pinned README and repository metadata inspected. No code paths or protecting tests traced; no runtime executed. Documentation claims are hypotheses. These refs are intake pins, not canonical harvest-lock/gitlink additions. No increase to the previously recorded 48 harvested sources is claimed. Continue under RESEARCH_PROTOCOL.md; preserve the other thread's active trace queue.
