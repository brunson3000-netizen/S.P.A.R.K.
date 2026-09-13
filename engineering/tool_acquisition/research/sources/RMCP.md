# Source Study — Official Rust MCP SDK (`rmcp`)

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: official Rust SDK for Model Context Protocol
- Upstream: `https://github.com/modelcontextprotocol/rust-sdk`
- Commit: `3075dc9152d4678775f20634fcb467a7b995dbab`
- Retrieval/study date: 2026-09-13
- Workspace crate version: `3.3.0`
- License: Apache-2.0
- Rust edition: 2024; minimum Rust: 1.88
- Primary crates: `rmcp`, `rmcp-macros`
- Evidence: core models/service/transports, task manager, tool router/macros, conformance client/server, CI conformance workflow and tests.

## Problem framing

PROBLEM: MCP implementations need interoperable typed JSON-RPC models, lifecycle/version negotiation, transports, request correlation, tool/resource/prompt/task routing, cancellation/progress/subscriptions and protocol conformance without reimplementing wire mechanics.

USER: Rust MCP clients, servers, gateways and embedded hosts.

INPUT: typed client/server handlers, negotiated protocol versions/capabilities, JSON-RPC messages over stdio/HTTP/in-process transports.

OUTPUT: typed protocol requests/results/notifications, handler dispatch, transport/session lifecycle, cancellation/progress/subscription primitives and MCP task wire state.

AUTHORITY: The SDK transports and dispatches protocol operations; it does not define SPARK’s application authority. A server handler may invoke arbitrary business logic, so permission/identity/resource policy must be supplied by the host around or inside handlers.

TRUST: peer-declared capabilities/protocol metadata, transport/auth configuration and application handlers. Protocol session IDs, task IDs, tool names and negotiated capabilities are correlation/feature state, not security grants.

STATE: connection/peer info, request/progress correlation, optional response cache, optional legacy HTTP session state, subscriptions and in-memory server task state. Durable application state is not supplied by MCP itself.

FAILURE: typed initialization/transport/service errors; request timeout sends cancellation; unknown/disabled tools error; unsupported/missing extension capabilities error; transport/session close is surfaced. Some compatibility/cache behaviors intentionally prioritize interoperability over strict freshness or security and therefore need host policy.

## Trace A — connect → protocol negotiation → running service

Client/server transport is supplied to `.serve(...)`.

### Client

Client initialization:
→ create initialize/discover request with client implementation + capabilities + preferred versions
→ send request with unique request ID
→ require correlated response ID
→ negotiate compatible protocol version
→ store server peer info
→ enter `RunningService` loop.

Pre-handshake handling is intentionally interoperable: logging notifications can be handled, pings ignored, other unexpected messages warned/ignored.

### Server

Server receives initialize/discover
→ constructs `RequestContext` with request ID, cancellation token, metadata/extensions and `Peer`
→ `ServerHandler::initialize` / `discover`
→ `negotiate_protocol_version` against server-supported versions
→ store negotiated peer info
→ send typed result
→ enter service loop.

At the pinned revision, the server deliberately does **not** wait for the legacy `initialized` notification before processing the main loop; the code explains that Streamable HTTP POST ordering is not guaranteed and the MCP text uses SHOULD NOT rather than MUST NOT. Therefore “initialized notification received” is not a security or authority gate.

For protocol 2026-07-28+, inline/per-request lifecycle metadata is supported and validated where required. Server-to-client requests such as elicitation/legacy roots/sampling are request-associated under SEP-2260; unassociated restricted requests are rejected under the strict protocol revision.

Reusable mechanics:
- typed lifecycle models
- version negotiation
- peer/request context
- request association
- request-ID correlation
- explicit service close/cancel/wait lifecycle.

## Trace B — list tools / call tool

Tool definitions can be produced manually or via `#[tool]`, `#[tool_router]` and `#[tool_handler]` macros.

`ToolRouter`:
- maps name → `ToolRoute`
- carries generated JSON Schema and annotations
- lists enabled tools sorted by name
- supports disable/enable + `tools/list_changed` notifier
- rejects missing/disabled tool as `INVALID_PARAMS`
- wraps typed handler argument deserialization.

Typed parameter wrappers deserialize arguments before the business handler runs. Argument-deserialization failure becomes a tool-level error result rather than invoking the tool.

Server dispatch for `tools/call`:
→ protocol/capability/lifecycle validation
→ handler `call_tool(params, RequestContext)`
→ handler/router performs application logic
→ SDK validates task-extension compatibility if handler returns a task result
→ typed result/error serialized to peer.

Important boundary: `ToolRouter` enable/disable and registration are local server routing state. They are **not** a substitute for host-owned caller identity/current grants. The router does not know SPARK authority, cost limits, project scope or governance state.

Additional identity caution: router storage is keyed by tool name; adding a duplicate name inserts/replaces the previous route. SPARK canonical capability identity must stay outside this local protocol name map.

## Trace C — cancellable requests / progress / subscriptions

`Peer::send_request_with_option` creates a `RequestHandle` containing:
- request ID
- progress token
- response receiver
- timeout/progress options
- peer handle.

`RequestHandle` supports:
- response wait
- idle timeout
- optional timeout reset on matching progress
- maximum total timeout
- explicit cancellation
- timeout-triggered `notifications/cancelled`.

`RequestContext` contains a child cancellation token cancelled when peer cancellation arrives. This is reusable cooperative cancellation plumbing.

Subscriptions are request-scoped:
- requested/accepted filters are intersected with advertised capabilities
- subscription notifications carry the originating subscription request ID
- channels are bounded
- an undrained channel ends as `Lagged`
- stream closure/cancel/graceful result are distinct states
- subscriptions are not resumable after abrupt transport close; caller must listen again.

Task-status notifications are explicitly not yet routable through `subscriptions/listen` at this pin; task state is observed by `tasks/get` polling.

## Trace D — MCP tasks / continuations

The SEP-2663 tasks extension is capability-gated.

Tool call may return either:
- a normal complete `CallToolResult`, or
- `CreateTaskResult` carrying a task ID/status/poll interval.

SDK dispatch rejects a task result if the client did not declare the tasks extension, even if a misbehaving application handler returns one.

`tasks/get`, `tasks/update` and `tasks/cancel` are also extension-capability gated.

`TaskManager` supplies a convenient server-side implementation:
- creates UUID task IDs
- inserts state before `spawn` returns, so `tasks/get` can immediately resolve
- tracks working/input-required/terminal payloads
- accepts keyed input responses
- cooperative cancellation
- TTL expiration and retention
- polling interval hints.

Tests cover create → poll → complete, cancel acknowledgment/terminal settling, missing capability, unknown task, legacy task fields and metadata preservation.

Critical terminology correction: despite its comments saying it owns “durable state,” this `TaskManager` stores tasks in an in-process `Arc<Mutex<HashMap<...>>>`. It is durable only in the protocol-observability sense while that manager/process survives. It is **not crash-durable** and is not suitable as SPARK canonical work state without an external durable adapter.

Task IDs similarly identify protocol task state; possession of a task ID must never become authorization to inspect/update/cancel a SPARK job. Host identity/grants still apply to each operation.

## Trace E — HTTP session / stateless transport

The pinned SDK supports both legacy stateful Streamable HTTP sessions and the 2026-07-28 stateless design.

For 2026-07-28, rmcp automatically operates statelessly:
- no `Mcp-Session-Id`
- no standalone GET/DELETE stream
- no Last-Event-ID resumption.

Legacy session mode can issue a random `Mcp-Session-Id`. `SessionManager` abstracts create/initialize/has/close/request stream/standalone stream/resume and optional external-store restoration.

The default local session manager is in-memory. Custom managers can use a database/Redis.

Important architectural warning from the SDK itself: `OriginatingRequestId` association is a non-serialized extension used by the local session manager. A cross-process session implementation that serializes messages loses it and can violate SEP-2260 unless it implements its own association mechanism.

Therefore:
- MCP session ID = transport/protocol correlation
- SPARK canonical subject/session/work identity = host-owned and separately authenticated.

## Trace F — client response cache

`ClientCacheConfig` defaults:
- enabled
- max 512 entries
- server TTL bounded to 24h
- `serve_stale_on_error = true`.

Cache invalidation is tied to list-changed/resource-updated notifications and a generation counter prevents an older in-flight response from repopulating invalidated state.

Private responses can be partitioned by an opaque authorization-context identifier.

However, stale-on-error intentionally returns an expired cached response as `Ok(...)` when refresh fails. This is useful MCP client behavior but **unsafe for authority-sensitive discovery or policy inputs** unless explicitly disabled. SPARK must not let stale catalog/tool/resource data silently stand in for current authorization truth.

## Conformance evidence

The repo contains an official conformance client/server and a CI workflow running:
- full 2025-11-25 server suite
- full 2026-07-28 server suite
- full 2025-11-25 client suite
- full 2026-07-28 client suite
- extension suites with a strict expected-failure baseline.

At this pin, extension baseline says all listed server Tasks scenarios pass and were removed from expected failures; `tasks-status-notifications` remains upstream-skipped pending the subscription rewrite. Client informational extension expected failures remain for enterprise managed authorization, DPoP, DPoP nonce and WIF JWT bearer.

An older 2026-02-25 audit stored in the repo reports much weaker 0.16.0-era conformance. It is historical and **invalidated for current pin capability claims** by the 3.3.0 source, new ROADMAP/VERSIONING files and current conformance workflow. Do not quote the old percentages as current quality.

The commit-status REST endpoint contains no classic status contexts for this exact pin, so this trace does not claim that an observed hosted CI run passed; it claims only what the pinned tests/workflow/baseline establish.

## Boundary findings

### MCP feature capability ↔ authority
Capabilities state what a peer can parse/participate in. They do not authorize business side effects.

### Protocol tool name ↔ canonical capability identity
rmcp needs a local string name for wire routing. SPARK needs a stable canonical ID/provenance/version mapping outside that name.

### Task/session/request handle ↔ grant
All are correlation/lifecycle state. Each host-side read/update/cancel/execute operation needs independent authorization.

### MCP lifecycle ↔ adapter lifecycle
Protocol negotiation/connection/session state should remain adapter-local. SPARK supervisor health/activation truth is separate.

### Cache ↔ current truth
Client cache may intentionally serve stale success. Never use that default for current authority, trust or health decisions.

## Determinization candidates

Direct reuse is strongly plausible for:
- model types and schemas
- protocol version negotiation
- request/response/notification enums
- request/progress correlation
- cancellation/timeouts
- stdio and Streamable HTTP transports
- typed tool handler/router plumbing
- subscription protocol handling
- MCP task wire models
- conformance harness inputs.

Host-owned SPARK machinery remains required for:
- canonical capability identity/provenance
- source trust
- current caller/resource grants
- economic/governance authority
- sandbox/confinement
- durable work/task state
- evidence custody
- health/quarantine
- cross-process request-association continuity when needed.

## Material extracted

IDEA: use the official SDK as a protocol adapter rather than rebuilding MCP wire semantics.

PATTERN: protocol substrate below an independent host authority layer.

PATTERN: cancellable correlated request handles are lifecycle references, not grants.

PATTERN: negotiated capability gates prevent peers from receiving wire shapes they did not declare.

CODE: `rmcp` is a strong direct-reuse candidate under Apache-2.0 for MCP protocol mechanics, subject to dependency/API qualification and pinning.

TEST/INVARIANT: protocol success never implies host authorization; stale cached discovery never substitutes for current authority; cross-process association state must survive the serialization boundary explicitly.

## Primary disposition

REUSE_CODE

Scope of reuse: official MCP wire models, lifecycle, transports, typed router/helper machinery and compatible conformance tests.

Explicitly excluded from authority: tool registration visibility, MCP capability declarations, session IDs, request/task handles, cache state and SDK task storage.

## Experiment requirement

Build a small SPARK-side Rust MCP adapter around the pinned SDK where:
1. rmcp handles transport/schema/lifecycle only
2. external SPARK canonical capability ID maps to MCP server/name/version
3. every `tools/call` crosses host call-time authority before rmcp handler execution
4. task/session IDs are scoped correlation data only
5. stale-on-error cache is disabled for discovery used by authorization
6. adapter emits canonical immutable execution evidence outside MCP.
