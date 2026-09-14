# Source Study — agent-observability

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: agent-observability (`agent-obs`)
- Upstream: `https://github.com/KryptosAI/agent-observability`
- Commit: `2658eef467225f376e2e92dc1465839eda2bc113`
- Package version: `1.0.7` (MCP server constant still reports `1.0.0`)
- License: MIT
- Runtime: Node.js, better-sqlite3, Express; self-reporting MCP server plus MCP stdio proxy fallback
- Primary evidence: `mcp-server.js`, `proxy.js`, `database.js`, `server.js`, `test.js`, `scripts/postinstall.mjs`, `README.md`, `package.json`.

## Problem framing

PROBLEM: Existing coding agents provide little structured information about tool calls, token/cost use, errors and intermediate decisions.

USER: local coding-agent users/operators using Claude Code, Cursor, opencode or other MCP clients.

INPUT: primarily model/agent self-reports through observability MCP tools; alternatively intercepted MCP JSON-RPC traffic through a stdio proxy.

OUTPUT: local SQLite sessions/tool calls/decision points/audit rows/health checks, aggregate grades/cost estimates, and an unauthenticated local dashboard API.

AUTHORITY: intended as observation only. It does not own execution permission. However, its installation/setup path mutates agent configuration and therefore crosses a behavior-loading boundary even though runtime records are observational.

TRUST: in default self-reporting mode it trusts the observed agent to call the observability server after every action and to accurately report tool identity/status/duration/input/output/tokens. Proxy mode trusts JSON-RPC interception/correlation. Database records do not independently prove the claimed action occurred.

STATE: SQLite/WAL database under `~/.agent-observability/sessions.db`; session/tool/skill/decision/audit/health records.

FAILURE: self-reporting can omit or fabricate events; proxy correlation is broken at this pin for ordinary tool responses; dashboard/API exposes stored trace data without authentication; raw payload storage is effectively unbounded in the traced database path.

## Trace A — self-reporting mode

Agent starts task
→ model is instructed by MCP server `initialize`/`tools/list` to call `start_session`
→ after every real tool call, **the model must remember to call** `log_tool_call`
→ observability server trusts submitted `sessionId`, `toolName`, input/output, duration and status
→ database inserts a tool-call row
→ server immediately inserts a `decision_points` row derived from that tool call
→ agent eventually calls `end_session`
→ server queries tool-call rows, computes an error-rate grade and stores session token totals/cost.

### What this proves

It is easy to deploy and gives the model a tiny reporting interface, but it is not an independent recorder. The subject being observed is also the reporter.

The server does not intercept the real external tool execution in this mode, so:
- missed reporting is indistinguishable from no action
- false tool name/status/duration/input/output can be recorded
- sequence is inferred from the current count of recorded calls
- token totals at end are caller-supplied.

### Decision fidelity issue

`log_tool_call` automatically calls `database.logDecision` with:
- `chosenAction = toolName`
- `rationale = outputSummary || toolName`.

That is not evidence of the agent’s decision process. It is a reconstructed annotation after an action. The README’s claim that decision points explain “why” is stronger than this implementation path.

At this pin there is no separate MCP `obs_record_decision` or `obs_record_token_usage` tool matching the README API table. The implemented tools are `start_session`, `log_tool_call`, `end_session`, `get_last_session`, `check_session`, `get_session_stats`.

## Trace B — proxy mode

`startProxy(targetCommand, targetArgs)`
→ creates session
→ spawns target using `child_process.spawn(..., shell: true)`
→ line-parses JSON-RPC from agent stdin
→ on `tools/call`, extracts name/input and assigns private properties directly to the **request object**:
  - `_obs_callStart`
  - `_obs_stepNumber`
  - `_obs_toolName`
  - `_obs_input`
→ JSON serializes request to MCP server
→ separately line-parses the server response
→ response handler attempts to read those same `_obs_*` fields **from the response object**
→ records tool call.

### Correlation defect

Ordinary MCP responses echo the JSON-RPC `id`, not arbitrary private request fields. No pending-request map keyed by JSON-RPC ID exists in this proxy path.

Therefore, for normal tool responses:
- tool name falls back to `unknown`
- input falls back to `{}`
- call start falls back to response time, producing near-zero duration
- step attribution falls back to current global `stepNumber`, which is unsafe under overlapping calls.

The proxy test does not exercise a `tools/call` round trip. It only sends `tools/list` and asserts forwarding/session creation, so the broken tool correlation is not caught.

This is a strong research lesson: an interception proxy needs stable request/response correlation owned by the proxy, not metadata attached to one side of the wire.

## Trace C — persistence model

SQLite/WAL schema includes:
- `sessions`
- `tool_calls`
- `skill_loads`
- `decision_points`
- `audit_entries`
- `tool_health_checks`.

Rows use UUIDs and session foreign keys. Useful vocabulary includes actor/resource/permission-scope/outcome fields for audit rows and step numbers for tool/decision order.

However, writes are individual statements; there is no event-envelope transaction binding a tool call to its corresponding decision/audit/session aggregate state.

### Payload bound mismatch

README claims tool output is “truncated at 64KB for storage.” The traced `logToolCall` path performs:
- `JSON.stringify(input || {})`
- `JSON.stringify(output || {})`
without a size check/truncation.

Only `output_summary` is truncated to 500 characters.

Thus this pin can duplicate arbitrarily large/sensitive tool payloads into SQLite, subject only to process/database limits.

## Trace D — cost/grade semantics

`estimateCost(totalTokens)` is a fixed rough estimate of **$3 per million total tokens**, independent of input/output split/model, despite README tables showing richer model pricing.

`computeGrade` is based only on tool-call error rate:
- zero errors → A
- <10% → B
- <25% → C
- <50% → D
- otherwise F.

It does not evaluate task correctness, wasted tokens, redundant tools or whether the stated goal was achieved, despite stronger README language.

These are useful operator-facing heuristics only if labeled as estimates/derived scores. They are not semantic or billing truth.

## Trace E — dashboard/data boundary

`server.js` exposes session details including raw tool input/output through Express routes. No authentication/authorization middleware is present in the traced server.

`startServer` calls `app.listen(port)` without an explicit loopback host. Node may bind on an unspecified address rather than guaranteeing loopback-only exposure. README describes the dashboard as localhost, but the code does not make that a hard security boundary.

SPARK implication: a flight recorder containing sensitive evidence must not inherit “local” as a security property. Bind/identity/access policy must be explicit.

## Trace F — installation/acquisition behavior

Package `postinstall` executes automatically after installation and:
- edits discovered global opencode configuration
- edits discovered global Claude MCP configuration
- otherwise may create `~/.mcp.json`
- inserts commands that invoke `npx -y agent-obs@latest server`.

This is behavior-changing installation, not passive observability. It combines install time with activation/config mutation and selects a moving `@latest` dependency at future execution.

This directly reinforces prior SPARK findings:
- installed ≠ activated
- source acquisition/provenance ≠ runtime grant
- tooling must not silently modify another agent’s behavior configuration.

## Tests/invariants extracted

Tests cover:
- basic database CRUD
- dashboard REST endpoints
- dashboard static assets
- basic CLI start/log/stop/inspect
- proxy process starts, forwards a `tools/list`, and creates a session
- MCP server initialize/list/basic start_session path.

Important omissions for architecture assurance:
- proxy `tools/call` request/response correlation
- concurrent MCP calls
- unreported self-reporting actions
- forged/inaccurate self-reports
- output-size bound promised by README
- dashboard authentication/bind scope
- transactional consistency between tool/audit/decision rows.

## Boundary findings

### Observed agent ↔ recorder
Self-reporting is low-assurance. It is appropriate as optional annotation, not canonical execution evidence.

### Interception proxy ↔ protocol causality
A proxy can provide stronger evidence, but only if it correlates actual request/response IDs and records before/after states independently of the model.

### Observation ↔ explanation
Recorded action and inferred “decision rationale” are different epistemic states. Never synthesize reasoning after the fact and present it as an observed decision.

### Installation ↔ activation
The package’s automatic config mutation is exactly the state conflation SPARK should avoid.

## Determinization candidates

- request ID → pending call correlation
- monotonic per-session sequence assignment
- start/end timestamps/duration
- actual intercepted method/tool/result/error capture
- size/redaction/artifact-reference policy
- session lifecycle/recovery
- aggregate statistics
- estimator provenance/labels.

## Material extracted

IDEA: an independent proxy recorder can complement in-process instrumentation for externally visible tool calls.

PATTERN: dual capture modes can be useful, but their assurance level must be explicit (`SELF_REPORTED` versus `INTERCEPTED`).

NEGATIVE LESSON: self-reported “full audit trail” is not independently verified evidence.

NEGATIVE LESSON: request/response correlation is load-bearing for a proxy flight recorder.

NEGATIVE LESSON: inferred labels/grades/reasoning must never be confused with observed facts.

## Primary disposition

ARCHIVE_REFERENCE / BORROW_IDEA.

The project has useful vocabulary and reinforces why SPARK needs an independent host-owned flight recorder, but its current recording fidelity is too weak to borrow as an implementation foundation.
