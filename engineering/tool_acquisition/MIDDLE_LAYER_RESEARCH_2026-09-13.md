# Agent–Tool Middle-Layer Harvest Research — 2026-09-13

Status: `HARVESTED_FOR_QUALIFICATION`

## Why this lane exists

The current external-tool program exposed a distinct layer between an agent and an executable tool. The useful problem is not merely how to add more tools; it is how to let an agent discover, learn, invoke, and inspect a large capability universe without loading that universe into its working context or weakening authority boundaries.

Working doctrine for study:

> Small context, full evidence.

> An agent should have a small stable doorway to a large governed capability space.

These are research hypotheses, not adopted S.P.A.R.K. or consumer architecture.

## Strong findings

### 1. Progressive disclosure is now a concrete pattern

`S1LV3RJ1NX/mcp-guardian` is the intended progressive-disclosure project. It exposes three meta-tools — search, schema retrieval, and execution — instead of publishing every upstream tool schema to the model. Its README reports benchmarked startup-context reduction on large MCP catalogs and combines discovery with scope enforcement and audit.

This is directly relevant to a candidate primitive surface such as:

- `find_capability()`
- `learn_capability()`
- `execute_capability()`
- `report_result()`

The exact verbs remain undecided; the important pattern is staged disclosure.

A different project with the same name, `eqtylab/mcp-guardian`, is a real MCP security/control project but is not the progressive-disclosure implementation discussed in this mission. Do not conflate them.

### 2. ARD supplies a standards-level discovery layer

`ards-project/ard-spec` defines Agentic Resource Discovery: a federated discovery layer for resources including MCP servers, A2A agents, skills, APIs and workflows. Its architectural position is before invocation. It therefore complements rather than replaces MCP, A2A, Agent Skills, or an authorization system.

This is the strongest newly discovered candidate for the *discovery vocabulary and federation shape* behind `find_capability()`.

### 3. The registry/gateway problem has at least two strong references

`agentic-community/mcp-gateway-registry` is a governed control-plane reference. It registers MCP servers, agents, skills and custom entities; supports semantic/lexical discovery, access controls, audit, federation, virtual MCP servers, egress authentication, and A2A discovery. Its default A2A pattern is especially relevant: the control plane can govern discovery/auth/access and then permit direct peer-to-peer agent communication rather than becoming a mandatory data-path bottleneck.

`agentgateway/agentgateway` answers a related but different question: how to govern and observe MCP, A2A and LLM traffic through one proxy/gateway surface, including budget controls, routing/failover, authentication, RBAC, rate limiting and OpenTelemetry.

Neither project should be allowed to become S.P.A.R.K. authority by import. They are implementation and boundary references.

### 4. Evidence-preserving context reduction is different from summarization

The intended project is `Open330/context-compress`. It focuses on agent/tool output and supports indexed searchable retention so the agent can see a compact result while detailed material remains retrievable. This is materially different from merely shortening instructions or deleting verbose output.

`vidanov/context-compress` is a separate, useful project for compressing agent instruction files. It is not the evidence-retention project described in this mission and is not pinned in this lane.

`karthikscale3/ctx-zip` is an additional comparison candidate that combines on-demand tool exposure with compacted outputs that remain addressable. Its current source is older than the strongest candidates, so it is reference evidence rather than a preferred foundation.

The key design test for this lane is:

> Can the agent receive a compact, deterministic view while the complete original evidence remains addressable, integrity-checkable, and available for later inspection?

### 5. Agent Skills is a real interoperability surface

`agentskills/agentskills` is the open Agent Skills format. A skill is centered on `SKILL.md`, with optional scripts, references and assets, and is intended for on-demand procedural knowledge loading.

This is highly relevant to the distinction between:

- capability identity — what exists;
- capability schema — how it can be called;
- skill — how an agent should use it well;
- specialist — an agent configured to perform a bounded role.

S.P.A.R.K. should qualify this format before inventing a competing skill-package format.

### 6. Disposable runtime prior art is strong but heavyweight

`e2b-dev/runtime` is an open runtime using Firecracker microVMs for isolated agent execution with snapshot-oriented lifecycle mechanics. It is close enough to disposable sandbox-cell concepts to deserve source-level study, especially around lifecycle, snapshot/resume, network/filesystem boundaries, and untrusted code execution.

It is not a near-term automatic dependency. The first question is what boundary patterns can be reused or reimplemented more narrowly.

### 7. Flight recorder and audit are separate observability concerns

`Klepsiphron/agenttrace` is a trajectory/tree reference: parent-child agent relationships, tokens, tool calls, latency and cost.

`KryptosAI/agent-observability` is more directly a local tool-call/audit reference: calls, failures, duration, cost and audit information.

The study should keep both concepts separate:

- trajectory observability explains *how work unfolded*;
- execution/audit evidence explains *what consequential calls actually occurred*.

### 8. Zero-trust tool access has useful concrete prior art

`openziti/mcp-gateway` applies OpenZiti-style zero-trust connectivity to MCP access. Its relevance is not that S.P.A.R.K. should adopt OpenZiti wholesale; it is a source-level reference for cryptographic identity, authenticated private connectivity, per-client isolation and tool-level filtering.

## External validation not harvested as source

GitHub Copilot's current tool-search behavior independently validates progressive disclosure: large tool catalogs are held back and tool definitions are loaded on demand instead of always occupying the model context. GitHub's current agent/resource work also points toward ARD-style resource discovery.

This is product evidence, not S.P.A.R.K. authority.

## Current best synthesis

The candidate middle layer is not one product. It is a separation of responsibilities:

1. **Discover** — locate candidate capabilities without exposing every schema.
2. **Learn** — load only the selected capability contract and relevant skill material.
3. **Authorize** — apply host-owned grants/policy before effects are possible.
4. **Execute** — invoke a narrow native/MCP/A2A/application adapter.
5. **Reduce context** — return a compact result while preserving complete evidence externally.
6. **Observe** — record trajectory and consequential-call evidence independently.
7. **Isolate** — run dangerous or untrusted work behind an appropriate containment boundary.

Transport, registry metadata, skill text, or a gateway's local permissions must never silently become a consuming system's authority.

## Harvest dispositions

| Candidate | Current disposition |
|---|---|
| ARD spec | `HIGH_PRIORITY_STANDARD_REFERENCE` |
| progressive mcp-guardian | `HIGH_PRIORITY_PATTERN_AND_PROTOTYPE_REFERENCE` |
| MCP Gateway & Registry | `HIGH_PRIORITY_CONTROL_PLANE_REFERENCE` |
| context-compress | `HIGH_PRIORITY_EVIDENCE_REDUCTION_REFERENCE` |
| Agent Skills | `HIGH_PRIORITY_STANDARD_REFERENCE` |
| agentgateway | `STRONG_GATEWAY_REFERENCE` |
| AgentTrace | `STRONG_TRAJECTORY_OBSERVABILITY_REFERENCE` |
| OpenZiti MCP Gateway | `STRONG_ZERO_TRUST_REFERENCE` |
| E2B Runtime | `STRONG_SANDBOX_PRIOR_ART` |
| agent-observability | `TOOL_AUDIT_COMPARISON_REFERENCE` |
| ctx-zip | `COMPARISON_REFERENCE__REQUALIFY_MATURITY` |

No entry in this table is adopted, installed, activated, or granted authority.
