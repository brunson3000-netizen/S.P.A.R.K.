# Source Study — MCP Gateway & Registry

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: MCP Gateway & Registry
- Upstream: `https://github.com/agentic-community/mcp-gateway-registry`
- Commit: `7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73`
- Retrieval/study date: 2026-09-13
- License: Apache-2.0
- Primary runtime: Python/FastAPI registry + auth server, nginx gateway/Lua, DocumentDB/search, JavaScript/TypeScript UI; multiple deployment integrations
- Primary docs: `README.md`, `docs/design/theory-of-the-system.md`, `docs/design/a2a-protocol-integration.md`, `docs/scopes.md`, `docs/auth.md`

## Problem framing

PROBLEM: Organizations accumulate MCP servers, agents, skills and other AI assets with fragmented discovery, credentials, access rules and audit trails.

USER: agents/coding assistants, platform/security operators, service identities and human users.

INPUT: asset registrations; authenticated search/list requests; MCP/A2A/generic proxy traffic; scope rules and identity-provider claims.

OUTPUT: filtered discovery results, governed gateway routes, proxied execution, agent cards/skills, and attributable audit/telemetry.

AUTHORITY: The registry can maintain/enable asset inventory. The gateway/auth server can admit or reject calls and route admitted requests to backends. Scope and token rules can materially determine access to external side effects.

TRUST: identity-provider validation, server-controlled nginx markers, scope repository, registered target metadata, signing secrets, registry/auth internal trust, and guarded upstream resolution.

STATE: asset inventory, scope/group mappings, search index/embeddings, sessions/tokens, egress credentials, audit records and runtime health/configuration.

FAILURE: security-sensitive paths generally fail closed: missing/invalid scopes deny; malformed/uninspectable request bodies deny; scope repository errors deny; internal hop tokens are audience/expiry bound. Search service failure returns 503 rather than widening results.

## Trace A — authenticated capability search

`POST /api/search/semantic`
→ `nginx_proxied_auth` supplies authenticated `user_context`
→ `SemanticSearchRequest` validates query/filter shape (`query` max 512; `max_results` 1..50)
→ `SearchRepositoryBase.search` / `search_by_tags`
→ raw cross-asset results
→ per-result visibility checks (`_user_can_access_server`, agent/skill/custom equivalents)
→ per-tool pruning with `filter_tools_for_user` / `tool_allowed_for_user`
→ backend URL redaction for callers that must not see internal endpoints
→ shaped server/tool/agent/skill/virtual/custom result collections
→ audit action recorded.

Important properties:
- discovery is identity-aware
- missing/empty per-server tool allowlist fails closed to zero visible tools
- admin and explicit wildcard grants are separate paths
- filtered tool counts are recomputed after authorization pruning
- result cardinality is bounded per collection
- discovery may expose trust status and gateway endpoint without leaking raw backend URL.

Primary files:
- `registry/api/search_routes.py`
- `registry/auth/tool_filter.py`
- `registry/services/visibility.py`
- `registry/auth/access_resolver.py`

Tests:
- `tests/integration/test_tool_level_access.py`
- search route unit tests

## Trace B — MCP `tools/call` authorization and execution handoff

Ingress request
→ nginx captures original URL and request body for auth subrequest
→ auth server `/validate`
→ validates caller credential/session
→ maps groups to scopes
→ derives registered server name from proxy path
→ parses JSON-RPC body and extracts method
→ for `tools/call`, extracts `params.name` as the actual tool name
→ refuses uninspectable body (`413`) or non-empty unparseable body (`400`) rather than guessing an unprivileged method
→ `validate_server_tool_access(server, method, actual_tool_name, user_scopes)`
→ re-reads scope data and checks server, method and specific tool
→ deny on no match or repository error
→ admitted request receives a short-lived signed internal hop token
→ nginx/backend hop verifies audience/expiry/signature and uses signed identity/scopes/destination claims instead of caller-forgeable forwarding headers
→ backend/proxy reaches registered upstream.

Key separation from the progressive Guardian reference:
- discovery visibility is resolved into `user_context` for search/list shaping
- execution authorization is performed again on the concrete call, against scope data, at `/validate`
- therefore finding/seeing a tool is not the sole execution grant.

Primary files:
- `auth_server/server.py::validate_request`
- `auth_server/server.py::validate_server_tool_access`
- `auth_server/server.py::filter_tools_list_response`
- `auth_server/internal_request_token.py`
- registry-side proxied-token verification

Tests/invariants:
- allowed/denied server access
- tool-specific `tools/call`
- repository exception denies
- missing scope denies
- uninspectable body denies
- malformed body denies
- tool list remains filtered even on cached fallback
- unauthorized server returns 403
- internal-token audience/use/expiry contracts have dedicated tests.

## Trace C — A2A discovery and invocation

The project has two explicit modes.

### Registry-only/default mode
Agent authenticates to registry
→ discovers permitted agent card/URL
→ registry leaves data path
→ agent communicates directly with target URL/native A2A security.

### Reverse-proxy mode
Agent discovery returns gateway-rewritten `/agent/{path}` URL
→ caller caches discovered agent + gateway token
→ `RemoteAgentClient` validates registry-supplied URL with an SSRF guard before fetch
→ constructs two credential channels:
  - `X-Authorization`: gateway/ingress credential
  - `Authorization`: target-agent/delegation credential
→ redirects disabled; 30s outbound timeout
→ request hits gateway `/validate`
→ `validate_a2a_agent_access` requires per-agent `invoke_agent`
→ gateway credential is stripped before backend; target credential remains end-to-end
→ A2A message sent to target.

Tests specifically reject:
- missing gateway credential on an agent path
- duplicated gateway token in `Authorization`
- duplicate token disguised with different Bearer formatting
- caller without per-agent invoke access.

Primary files:
- `docs/design/a2a-protocol-integration.md`
- `agents/a2a/src/travel-assistant-agent/registry_discovery_client.py`
- `agents/a2a/src/travel-assistant-agent/agent.py`
- `agents/a2a/src/travel-assistant-agent/remote_agent_client.py`
- `auth_server/server.py`

## Tests/invariants extracted

### Access
- no scope / repository failure → deny
- missing or empty tool allowlist → no visible tools
- tools/call requires specific tool or explicit wildcard
- server visibility and tool visibility are distinct
- cached tool-list fallback is filtered too

### Inspectability
- if the gateway cannot inspect the request body needed to determine the privileged operation, deny rather than substitute a benign default.

### Internal trust
- caller-supplied forwarding headers are not authoritative after `/validate`
- short-lived signed tokens bind identity/scopes/destination with distinct audiences for different internal hops
- extra claims cannot override reserved issuer/audience/subject/expiry claims.

### A2A credential separation
- gateway credential and target-agent credential occupy different headers and must not be duplicated.

### Audit
- protocol-level audit parses method/tool/resource from JSON-RPC and records identity, server, request, response, duration, session/transport and errors.
- tool-list pruning audit is best-effort, while startup contains stronger durable-audit sink checks for critical MCP/token audit when enabled. Do not conflate these assurance levels.

## Boundary findings

### Agent ↔ discovery
Typed request; authenticated; filtered by current resolved identity context; count/query bounds exist.

### Discovery ↔ execution
Meaningfully separated. Search visibility does not replace `/validate` call-time authorization.

### Edge ↔ auth server
Server-set original URL/body markers are security-critical inputs. If body capture/parse fails, authorization fails closed.

### Auth server ↔ backend proxy
Strong pattern: translate externally presented credentials/state into a short-lived signed internal assertion bound to the intended hop. Backend ignores forgeable caller headers.

### Agent ↔ remote A2A agent
Two supported trust/data-path modes must remain explicit: direct peer traffic versus governed reverse-proxy traffic. They have different security/cost/latency properties.

## Important risk / compatibility finding

`validate_server_tool_access` preserves a legacy fallback: for non-HTTP MCP methods not found in the `methods` list, it also checks the `tools` list; a wildcard tools allowlist can therefore authorize non-`tools/call` MCP methods. HTTP verbs are explicitly protected from this fallback, but the MCP method/tool namespaces remain partially conflated for compatibility.

S.P.A.R.K. should not copy this compatibility behavior. Method/protocol authority and capability/tool authority should be structurally distinct grants.

## Determinization candidates

- group/scope resolution
- discovery visibility filtering
- tool allowlist pruning
- call-time method/tool authorization
- request inspectability checks
- internal claim binding
- backend URL redaction
- rate/concurrency/output limits
- audit record generation
- SSRF/redirect checks

These belong under model reasoning, not in prompts.

## Agent-facing surface

This project is not minimal at the full registry API level. Its value is the machinery behind a potential tiny doorway: a host can expose a small `find/learn/execute` interface while the registry handles asset type, identity, trust, transport and policy out of context.

## Material extracted

IDEA: unified governed inventory for tools, agents, skills and custom resources.

PATTERN: identity-aware discovery pruning plus independent call-time authorization.

PATTERN: signed short-lived internal hop assertions after edge validation.

PATTERN: explicit direct-vs-proxied A2A modes.

ALGORITHM: fail-closed allowlist resolution, path normalization, request inspectability, bounded search and URL redaction.

CODE: no direct reuse decision yet; project is broad Python/nginx infrastructure and should be compared against SPARK’s Rust-core constraints.

TEST/INVARIANT: inability to inspect the real privileged request must deny, never guess; discovery visibility does not grant execution.

## Primary disposition

BORROW_PATTERN

Reason: several mechanisms are directly relevant and better separated than the simpler proxy references. The full product is infrastructure-heavy and not a candidate for wholesale adoption into SPARK’s core.
