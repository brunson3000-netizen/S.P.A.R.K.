# Discovery Comparison Matrix

Status: ACTIVE — expands as source traces complete
Updated: 2026-09-13

| Dimension | Progressive MCP Guardian | ARD v0.91 | Emerging SPARK implication |
|---|---|---|---|
| Primary problem | MCP schema/context overload | ecosystem-scale resource discovery | capability discovery must scale outside worker context |
| Resource scope | MCP tools from configured servers | artifact-agnostic: MCP, A2A, skills, registries, extensions | catalog should not be transport-specific |
| Agent/client doorway | 3 MCP meta-tools | mandatory REST `/search`; optional MCP/A2A wrappers | expose a tiny host-neutral discovery primitive |
| Discovery metadata | name, server, brief | stable ID, type, selected metadata, score, source/referrals | return compact selection metadata + stable ID |
| Full descriptor | `get_schema(tool_name)` from local index | artifact via `url`/`data`; full entry retrieval by identifier not normatively defined | SPARK needs an explicit deterministic `learn_capability(id)` path |
| Stable identity | bare tool name key | globally unique domain-anchored URN | stable capability ID is required; display names are aliases only |
| Search/ranking | default keyword strategy | semantic text + structured filters; implementation flexible | ranking can be pluggable; security cannot depend on rank |
| Result bound | no hard result-count/byte bound found in first trace | pageSize default 10, max 100 | hard item and byte limits required |
| Scope/security | allow/block filter before indexing | trust metadata/filtering separated from discovery score | discovery filtering is not execution authority |
| Execution | proxy executes selected tool | explicitly delegated to native artifact protocol | independent call-time authority + executor required |
| Auth | proxy owns upstream auth integration | delegated to artifact protocol | host broker owns current auth/grants; discovery stays advisory |
| Trust | implied by configured scope/upstream | optional trust manifest; relevance explicitly not trust | trust evidence and relevance must be separate fields/decisions |
| Federation | multiple configured upstream servers | `none` / `referrals` / `auto` registry federation | referrals are useful prior art; no adoption yet |
| Audit | call/result audit in proxy | not execution layer | common host evidence/telemetry should surround execution |
| Conformance evidence | unit tests + token/search/security benchmarks | official manifest/registry conformance CLI | reuse external invariants as acceptance tests where applicable |
| Main weakness found | discovery index doubles as admission; bare-name collisions; auth integration disconnect | no normative full-entry lookup by ID; proposal status; ranking intentionally underspecified | combine small discovery surface with stable ID, explicit learn step, host-owned reauthorization |

## Current convergence

Both independent sources support moving the large capability universe out of the worker prompt.

ARD provides the stronger identity/separation model. MCP Guardian provides the clearer minimal agent interaction loop.

Current hypothesis to test:

`find_capability → learn_capability → execute_capability → report_result/evidence`

with independent host authority between learn and execute.
