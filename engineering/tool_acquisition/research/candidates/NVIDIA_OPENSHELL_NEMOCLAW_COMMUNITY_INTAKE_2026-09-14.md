# OpenShell and NemoClaw — enforcement outside the agent

Retrieved: 2026-09-14 UTC
Status: INTAKE — source study admitted, implementation not validated
Primary disposition: ARCHIVE_REFERENCE

## Admission decision
Add OpenShell as the main MCI enforcement study target; retain NemoClaw as its integration/reference-stack companion. Avoid treating the two as independent sandbox implementations.

## Community signal
[Nvidia NemoClaw on Hacker News](https://news.ycombinator.com/item?id=47427027) shows 385 points and 261 comments, labelled five months old at retrieval. The useful disagreement is whether sandboxing meaningfully constrains agents once they can access powerful external accounts. Concerns about credential scope and authorized destructive operations remain research questions, not confirmed current product defects.

## Framing from pinned READMEs
- PROBLEM / USER: operators need execution and network limits around tool-using agents.
- INPUT / OUTPUT: agent processes plus declarative policies become controlled execution, allow/deny decisions and lifecycle operations.
- AUTHORITY / TRUST: enforcement moves to the host/runtime/proxy boundary; authority to change policy remains critical.
- STATE: sandbox lifecycle, policies and integration configuration; exact durability needs tracing.
- FAILURE: fail-closed behavior, controller loss and interrupted policy updates are unverified.
OpenShell documents method/path-level network rules and a policy proxy. NemoClaw now describes multiple supported agent choices including Hermes; older OpenClaw-only impressions should not be repeated as current facts.

## MCI value and next traces
1. Tool network request → identity → proxy policy → allow/deny → audit. Inspect redirects, alternate network paths and policy-update races.
2. Credential provisioning → agent-visible substitute → upstream request → revocation. Establish current behavior in code rather than repeating old discussion allegations.
3. Stop/snapshot/restart → retained permissions → policy version. Inspect controller failure and recovery tests.

DETERMINIZATION_CANDIDATE: authorization, request scope, resource limits and revocation.
Potential location: MCI execution boundary, below coordinator/supervisor models. Finding class: IDEA.
Network permission does not establish business authorization: an allowed endpoint may still permit an unwanted action. Study method/path scope and account privileges together.

## Source record
### NVIDIA/OpenShell
- Upstream: https://github.com/NVIDIA/OpenShell
- Exact inspected commit: `5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0`
- Primary source: https://github.com/NVIDIA/OpenShell/blob/5b9daab9351b1e053f9a5e0ce4c899f5d3f674b0/README.md
- Language: Rust; detailed runtime/dependencies not yet traced.
- License: Apache-2.0 (GitHub metadata only; file-level review required before reuse).
- GitHub snapshot: 8599 stars; not a growth rate.

### NVIDIA/NemoClaw
- Upstream: https://github.com/NVIDIA/NemoClaw
- Exact inspected commit: `3ea2d5f9a515d4176e0745214743a48ff4868f4c`
- Primary source: https://github.com/NVIDIA/NemoClaw/blob/3ea2d5f9a515d4176e0745214743a48ff4868f4c/README.md
- Language: TypeScript; detailed runtime/dependencies not yet traced.
- License: Apache-2.0 (GitHub metadata only; file-level review required before reuse).
- GitHub snapshot: 22449 stars; not a growth rate.

## Evidence limits and handoff
Pinned README and repository metadata inspected. No code paths or protecting tests traced; no runtime executed. Documentation claims are hypotheses. These refs are intake pins, not canonical harvest-lock/gitlink additions. No increase to the previously recorded 48 harvested sources is claimed. Continue under RESEARCH_PROTOCOL.md; preserve the other thread's active trace queue.
