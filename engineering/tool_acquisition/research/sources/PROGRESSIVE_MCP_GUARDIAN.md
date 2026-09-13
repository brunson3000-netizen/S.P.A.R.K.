# Source Study — Progressive MCP Guardian

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: mcp-guardian — progressive tool-disclosure proxy
- Upstream: `https://github.com/S1LV3RJ1NX/mcp-guardian`
- Commit: `4c6a04537b9bc548744b4146168b8fa9896069cb`
- Retrieval/study date: 2026-09-13
- License: MIT
- Language/runtime: Python; FastMCP client/server
- Primary docs: `README.md`, `docs/ARCHITECTURE.md`, `docs/WRITING_SCOPE_YAML.md`
- Evidence suites: `tests/`, `benchmarks/experiment-1-*` through `experiment-5-*`

## Problem framing

PROBLEM: Large MCP tool catalogs eagerly inject many complete schemas into model context, consuming context and increasing selection burden.

USER: MCP-capable AI clients/agents.

INPUT: Upstream MCP server definitions, active scope configuration, search/schema/execution calls.

OUTPUT: A three-tool agent surface plus proxied upstream results and audit records.

AUTHORITY: Can invoke upstream MCP tools that exist in the active in-memory index; scope filtering determines index membership.

TRUST: Trusts configured upstreams, local scope configuration, auth material and FastMCP transport behavior. It also assumes index membership is sufficient execution permission.

STATE: In-memory tool index; deferred server list; cached OAuth clients/tokens; key store; append-style audit log.

FAILURE: Unknown/out-of-scope tools return structured errors; upstream failures become `UPSTREAM_ERROR`; OAuth failures can become `AUTH_REQUIRED`; startup-list failures can defer indexing.

## Execution trace A — startup / catalog construction

`Guardian.__init__`
→ loads scope config
→ constructs `UpstreamManager`, `ToolIndex`, `AuditLogger`
→ registers exactly three FastMCP meta-tools.

`Guardian.startup`
→ `ToolIndex.build(config, upstream)`
→ for each scoped server, `UpstreamManager.list_tools(server, interactive=False)`
→ upstream MCP `list_tools`
→ `_is_tool_allowed(name, allowed, blocked)`
→ allowed tool stored as `ToolEntry` with full schema + short brief
→ blocked/non-allowed tool omitted from index.

Primary files:
- `src/mcp_guardian/proxy.py`
- `src/mcp_guardian/index.py`
- `src/mcp_guardian/upstream.py`
- `src/mcp_guardian/config.py`

Important property: filtering is deterministic and happens before the agent sees the catalog.

## Execution trace B — discover / learn

Agent `search_tools(query)`
→ `Guardian` meta-tool handler
→ `ToolIndex.search`
→ configured `SearchStrategy` (default `KeywordSearch`)
→ compact `{name, server, brief}` results.

Agent `get_schema(tool_name)`
→ `ToolIndex.get_schema`
→ exact indexed `ToolEntry.full_schema`
→ full schema only for requested tool.

Not-in-index names return an error/hint and do not reveal a schema.

## Execution trace C — execute / audit

Agent `execute_tool(tool_name, params)`
→ direct membership check: `self.index.entries.get(tool_name)`
→ if absent: `TOOL_NOT_IN_SCOPE`
→ `AuditLogger.log_call`
→ `UpstreamManager.call_tool(server, tool_name, params, client_headers=...)`
→ FastMCP upstream client call
→ `AuditLogger.log_result`
→ return upstream result.

Upstream exceptions are converted to `AUTH_REQUIRED` or `UPSTREAM_ERROR` result envelopes.

## Tests/invariants extracted

`tests/test_proxy.py` protects:
- exactly three exposed meta-tools
- search result behavior
- schema retrieval only for indexed tools
- blocked execution returns `TOOL_NOT_IN_SCOPE`
- upstream failure envelope
- OAuth-required envelope
- call/result audit entries.

`tests/test_index.py` protects:
- explicit allow-list filtering
- wildcard + block-list filtering
- excluded schema token accounting
- search behavior
- schema lookup behavior
- brief truncation.

`benchmarks/experiment-5-security/bench_security.py` exercises blocked tools across three exposure vectors: search, schema, execute. Its intended invariant is total invisibility/unreachability for blocked names.

## Boundary findings

### Agent ↔ discovery middleware
Typed: partially; FastMCP supplies schemas for the three meta-tools.
Validated: tool-name membership is checked; search query is free text.
Output bounded: search results are compact, but no explicit hard result-count/byte cap was found in this first trace.
Observable: execution is audited; search/schema calls are not equivalently recorded in the traced path.

### Discovery ↔ execution authority
Critical finding: the same filtered index serves both discovery and execution admission. `execute_tool` authorizes by index membership. There is no separate call-time grant/authority evaluator in this path.

S.P.A.R.K. implication: borrow progressive disclosure, not this authority coupling. Discovery must remain advisory; execution requires independent host-owned call-time authorization.

### Agent params ↔ upstream tool
The proxy stores the full upstream schema but `execute_tool` does not validate `params` against it before forwarding. Validation is therefore delegated downstream/FastMCP rather than enforced at this boundary.

### Client auth ↔ upstream
Config accepts `none`, `static_header`, `bearer_env`, `token_passthrough`, and `oauth`. `auth.py::get_auth_headers` implements/tests those header modes. However, the active `UpstreamManager._resolve_auth` path handles `bearer_env` and OAuth but does not call `get_auth_headers`; `Guardian._get_client_headers()` currently returns `{}`. This creates an integration disconnect for `static_header` and token passthrough at the pinned commit.

## Agent-facing surface

The model sees only:
- `search_tools(query)`
- `get_schema(tool_name)`
- `execute_tool(tool_name, params)`

Server instructions tell the model to search, then inspect schema, then execute. This is an unusually clean agent-facing contract and is the primary reusable lesson.

## Determinization candidates

- capability catalog filtering
- catalog search/ranking
- schema retrieval
- blocked-tool admission
- token-cost accounting
- audit bookkeeping
- known auth/deferred-index state

These do not inherently require model judgment.

## Material extracted

IDEA: tiny agent-facing doorway to a large capability universe.

PATTERN: progressive disclosure — compact search metadata first, exact schema on demand.

ALGORITHM: simple deterministic allow/block filtering and keyword ranking.

CODE: no direct reuse recommendation yet.

TEST/INVARIANT: blocked capability must be absent from search, schema retrieval and execution.

## Primary disposition

EXPERIMENT_NOW

Reason: the progressive-disclosure mechanism is strongly aligned and has direct benchmark/test prior art, but must be tested independently with SPARK-style call-time authority separation and bounded outputs before any implementation choice.

## Experiment requirement

Compare identical worker tasks under:
A. eager full catalog
B. three-meta-tool progressive disclosure
C. ARD-style resource discovery

Measure success, context/tokens, turns, latency, wrong-tool selection, missed capability, and authority failures.
