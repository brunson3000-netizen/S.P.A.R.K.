# MCI community intake sweep — 2026-09-14

## Outcome
Four admitted intake records covering six upstream projects. These are research additions, not adoption or completed source harvesting. The previously recorded 48-source harvest count is not incremented by this sweep. No runtimes were installed or executed.

| Candidate record | MCI question | Disposition |
|---|---|---|
| [OpenClaw / Hermes](candidates/OPENCLAW_HERMES_COMMUNITY_INTAKE_2026-09-14.md) | Persistent harnesses, routing, worker authority and supervisor composition | ARCHIVE_REFERENCE |
| [Paperclip](candidates/PAPERCLIP_COMMUNITY_INTAKE_2026-09-14.md) | Governance, approvals, budgets and primary coordination | ARCHIVE_REFERENCE |
| [Gas Town](candidates/GASTOWN_COMMUNITY_INTAKE_2026-09-14.md) | Coordinator, lifecycle supervisor, recovery and merge authority | ARCHIVE_REFERENCE |
| [OpenShell / NemoClaw](candidates/NVIDIA_OPENSHELL_NEMOCLAW_COMMUNITY_INTAKE_2026-09-14.md) | Deterministic runtime enforcement and integration | ARCHIVE_REFERENCE |

ARCHIVE_REFERENCE means admitted evidence and trace questions, awaiting code/test-backed extraction. It does not imply a proven reusable pattern.

## What the discussion is about
- Specialized workers versus the cost, context and human intervention required to coordinate them.
- Persistent assignment and recovery when agent sessions disappear.
- Who may approve, stop or override an agent, and whether those controls exist below the model.
- Why a sandbox alone does not settle authorization for powerful connected accounts.
These themes are supported by the linked community discussions in candidate records. Discussion is evidence of interest and concerns; current source is evidence of documented mechanisms.

## Attention assessment
Among the seven sampled repositories, OpenClaw (389,621 stars) and Hermes (245,163) have the largest accumulated GitHub audiences; Paperclip follows at 80,603. NemoClaw has 22,449, Gas Town 18,045, OpenShell 8,599 and AgentTeams 5,615. All counts are retrieval snapshots, not weekly growth or active-user counts.

The retrieved Gas Town HN thread has 354 points/224 comments and is labelled eight months old; NemoClaw has 385/261 and is labelled five months old. Hermes/OpenClaw comparisons retrieved are from May/June. Recency-filtered searches did not establish a reliable fresh weekly ranking. A newer Paperclip-adjacent HN page could not be opened (429). Therefore this sweep prioritizes substantial demonstrated attention and MCI fit without claiming to measure the entire community or this week's fastest growth.

Search scope: public web search, Hacker News, Reddit and official GitHub repositories. No private Discord/Slack or comprehensive social-stream access was used.

## Watchlist: AgentTeams
Primary disposition: DEFER.
Reason: directly relevant manager/worker architecture, but smaller repository attention and insufficient independently verified discussion volume to displace the admitted targets under the user's loudest-buzz constraint.
- Upstream: https://github.com/agentscope-ai/AgentTeams
- Exact README inspected: https://github.com/agentscope-ai/AgentTeams/blob/f65d6e1af268039507b413e3c16679bb84d21ac7/README.md
- Pin: `f65d6e1af268039507b413e3c16679bb84d21ac7`
- Retrieved: 2026-09-14 UTC
- Language: Go; runtime architecture documented as containers, Kubernetes-style resources and Matrix; dependencies not code-traced.
- License: Apache-2.0 from GitHub metadata, file-level review pending.
- PROBLEM / USER: operators coordinating mixed agent runtimes.
- INPUT / OUTPUT: room messages and team/worker configuration become assignments and visible collaboration.
- AUTHORITY / TRUST: manager delegates to workers; gateway credential mediation is a README claim requiring verification.
- STATE / FAILURE: controller-managed resources and shared storage; reconciliation and crash behavior unverified.
- Future trace: assignment identity → worker scope → gateway authorization → audited human intervention.
- IDEA only. “Workers cannot see real keys” would not alone prove that they cannot misuse gateway authority.

## Handoff
Continue the existing active source-trace queue; this sweep does not reset it. For the next community-focused pass, start with OpenClaw/Hermes for audience breadth and Paperclip for governance fit, then Gas Town supervision and OpenShell enforcement. Before full code study, perform the protocol's canonical source preservation and license review. No code reuse, implementation recommendation or performance claim is made.

## Deposit log
- PAPERCLIP: commit `91ebe0222a84e9f02a7fb44ff841527646380d79`; saved content verified at that commit.
- OPENCLAW_HERMES: commit `bb2d2ffd0800c28d4c434552268c8b3096520f16`; saved content verified at that commit.
- GASTOWN: commit `087da91251aa362c2773cbbb5d19a6a7964466cf`; saved content verified at that commit.
- NVIDIA_OPENSHELL_NEMOCLAW: commit `82ef15ab6a78046082c1c608dbd906e4124de65d`; saved content verified at that commit.
