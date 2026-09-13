# Discovery Comparison Matrix

Status: ACTIVE — expands as source traces complete
Updated: 2026-09-13

| Dimension | Progressive MCP Guardian | ARD v0.91 | MCP Gateway & Registry | Emerging SPARK implication |
|---|---|---|---|---|
| Primary problem | MCP schema/context overload | ecosystem-scale resource discovery | governed multi-asset inventory + access plane | capability discovery must scale outside worker context and stay governed |
| Resource scope | MCP tools from configured servers | artifact-agnostic: MCP, A2A, skills, registries, extensions | MCP servers/tools, A2A agents, skills, virtual/custom assets | catalog should not be transport-specific |
| Agent/client doorway | 3 MCP meta-tools | mandatory REST `/search`; optional MCP/A2A wrappers | REST/MCP-facing search APIs; broad platform API | SPARK should expose a much smaller host-neutral doorway than the backend control plane |
| Discovery metadata | name, server, brief | stable ID, type, selected metadata, score, source/referrals | per-asset result types with relevance, trust state, endpoint/projection data | return compact selection metadata + stable ID; keep control-plane richness out of worker context |
| Full descriptor | `get_schema(tool_name)` from local index | artifact via `url`/`data`; full entry retrieval by identifier not normatively defined | registry stores richer server/agent/skill records and tool schemas | explicit deterministic `learn_capability(id)` should resolve authoritative descriptor/version |
| Stable identity | bare tool name key | globally unique domain-anchored URN | server/agent paths and typed entity records; tool identity includes parent server context in search | canonical capability identity must be collision-resistant and independent of display name |
| Search/ranking | default keyword strategy | semantic text + structured filters; implementation flexible | hybrid semantic/lexical + tag/type filters | ranking can be pluggable; security cannot depend on rank |
| Result bound | no hard result-count/byte bound found in first trace | pageSize default 10, max 100 | query max 512; max_results 1..50 per collection | hard item, query and byte limits required |
| Discovery visibility | one static active scope filters index | discovery protocol itself; trust/filtering separate from score | authenticated per-user server/tool/agent/skill pruning; missing tool allowlist fails closed | discovery should be identity-aware without becoming execution authority |
| Execution authority | index membership admits execution | explicitly outside ARD | separate `/validate` path parses concrete method/tool and re-checks scopes | every execution needs current host-owned reauthorization |
| Uninspectable request | not a distinct admission case | outside spec | malformed body 400; uninspectable/spilled body 413; deny rather than guess | inability to identify intended side effect is a stop condition |
| Internal process trust | direct in-process proxy path | outside spec | short-lived signed audience-bound hop tokens after validation | use signed bounded assertions across process/adapter boundaries where needed |
| Auth | proxy owns upstream auth integration | delegated to artifact protocol | IdP/session/static validation + group/scope mapping + egress mechanisms | catalog discovery, caller auth and target credentials are distinct layers |
| Trust | implied by configured scope/upstream | optional trust manifest; relevance explicitly not trust | separate trust_verified state plus access policy | trust evidence and relevance must never collapse into one score/grant |
| A2A | — | can describe A2A artifacts | explicit default direct/P2P mode and opt-in gateway reverse-proxy mode | data-path choice is architectural and must be explicit per deployment/capability |
| Federation | multiple configured upstream servers | `none` / `referrals` / `auto` registry federation | peer/external registry federation + origin metadata | referrals/federation useful prior art; authority remains local/host-owned |
| Audit | call/result audit in proxy | not execution layer | protocol-level MCP audit + search/filter auditing; assurance levels differ | common evidence/telemetry should surround execution; critical audit must have explicit durability semantics |
| Conformance/test evidence | unit tests + token/search/security benchmarks | official manifest/registry conformance CLI | handler integration tests, auth-server security regressions, internal-token tests | reuse invariants as acceptance criteria; distinguish mocked handler tests from full edge-to-backend E2E |
| Main weakness found | discovery index doubles as admission; bare-name collisions; auth integration disconnect | no normative full-entry lookup by ID; proposal status; ranking underspecified | legacy MCP method→tools fallback blurs grant namespaces; broad/heavy infrastructure | combine tiny surface + stable ID + explicit learn step + independent reauth; keep protocol/tool grants structurally separate |

## Current convergence after three traces

Independent sources now converge on a stronger shape:

1. **MCP Guardian:** minimal worker interaction loop.
2. **ARD:** discovery/identity separation and ecosystem-scale descriptor model.
3. **MCP Gateway & Registry:** identity-aware discovery plus concrete call-time reauthorization and process-bound trust handoff.

Current experiment candidate:

`find_capability → learn_capability → [host reauthorization] → execute_capability → evidence/result`

with stable capability identity across every arrow, bounded worker-visible context, and full evidence outside the prompt.
