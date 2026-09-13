# Source Study — ToolHive

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: ToolHive
- Upstream: `https://github.com/stacklok/toolhive`
- Commit: `e532cf07d45fa99f3e4e63819396a3e9c9fd763f`
- Retrieval/study date: 2026-09-13
- License: Apache-2.0
- Primary runtime: Go platform/CLI/proxy/operator; container runtime + MCP middleware; shared `toolhive-core` libraries
- Primary evidence: `docs/arch/02-core-concepts.md`, `docs/arch/05-runconfig-and-permissions.md`, `docs/middleware.md`, `docs/arch/12-skills-system.md`, `pkg/api/v1/workload_service.go`, `pkg/mcp/tool_filter.go`, `pkg/authz/middleware.go`, `pkg/skills/skillsvc/verify.go`, associated tests.

## Problem framing

PROBLEM: MCP servers and agent-facing packages need a managed execution plane: discovery, repeatable configuration, isolation, authentication/authorization, tool shaping, provenance, secrets and observability.

USER: MCP clients/agents and operators running local, remote or Kubernetes-hosted MCP services.

INPUT: registry entries or direct server references, versioned RunConfig, permission profiles, client MCP traffic, identity tokens, policies, skill/plugin artifacts.

OUTPUT: running isolated/proxied workloads, filtered/authorized MCP surfaces, audit/telemetry, installed signed packages and persisted restartable configuration.

AUTHORITY: ToolHive can start containers/processes, mount host paths, grant network access, inject secrets, proxy authenticated requests and invoke backend tools. It is therefore an execution/security boundary, not merely a catalog.

TRUST: configured registries, runtime/container engine, identity provider, policy configuration, signature roots/lock state and operator-selected middleware. Directly supplied or registry-resolved executable images can run with substantial authority depending on the permission profile.

STATE: persisted RunConfig, workload lifecycle, container/runtime state, middleware/session state, registry metadata, package lock/provenance state and telemetry/audit state.

FAILURE: many authority paths fail closed (unknown MCP methods under authz, malformed/non-JSON protected requests, explicit incompatible network-isolation topology, signer-pin mismatch); several convenience/filtering paths intentionally or currently fail open/warn and therefore cannot be treated as the sole authority boundary.

## Trace A — create workload → declarative boundary → runtime

API/CLI request
→ `WorkloadService.BuildFullRunConfig`
→ resolve registry metadata or direct image/URL
→ verify/inspect image according to image-verification mode
→ resolve transport, secrets, middleware, permission profile and network-isolation settings
→ `runner.EagerCheckCreateServer`
→ persist RunConfig
→ `RunWorkloadDetached`
→ container/runtime applies mounts, privilege mode, network topology and proxy sidecars.

Load-bearing properties:
- RunConfig is versioned/serializable and contains execution, network, security, middleware, secrets-by-reference, tool filters and grouping.
- API `network_isolation` defaults to enabled when omitted.
- Permission profiles describe read-only/read-write mounts, outbound constraints and privileged mode.
- Bridge-mode isolation uses an internal network plus DNS and Squid ingress/egress sidecars.
- Explicit `isolate-network=true` with a non-bridge mode fails instead of silently pretending confinement exists; a defaulted isolation request with host networking is dropped with a warning; network `none` needs no sidecar isolation.
- creation policy gate runs before state is saved or workload starts.
- desired configuration can be exported/reviewed and actual runtime mounts/network can be inspected; ToolHive docs explicitly call the declared-vs-applied gap a security signal.

Critical qualification:
- container-image provenance verification defaults to `warn`, not hard failure (`VerifyImageWarn`). A stronger `enabled` mode exists, but the default is not sufficient for a high-assurance activation boundary.

Primary implementation evidence:
- `pkg/api/v1/workload_service.go`
- `pkg/api/v1/workload_types.go`
- `pkg/runner/config.go`
- `pkg/container/docker/client.go`
- permission/networking packages and tests.

## Trace B — MCP request → surface shaping → call-time authorization

Client request
→ outer audit/authentication middleware
→ MCP parser creates structured method/resource/argument context
→ tool-call mapping/filtering (when configured)
→ optional webhook/middleware stages
→ Cedar authorization (when configured)
→ backend proxy/runtime.

Outgoing `tools/list` responses take the complementary path:
backend result
→ authorization response filtering (when configured)
→ tool-list mapping/filtering/renaming
→ client-visible catalog.

### Tool mapping

ToolHive deliberately uses two independent middleware instances with the same static mapping configuration:
- list filter/override shapes what the client sees
- call filter maps user-facing aliases back to actual names and blocks calls outside its configured list.

Filtered calls attempt to look like nonexistent tools rather than disclose hidden capability names.

### Authorization

`pkg/authz/middleware.go` is a stronger security boundary than the presentation filter:
- parser output drives typed feature/operation/resource authorization
- unknown MCP methods deny by default
- non-JSON POSTs are rejected before a JSON-RPC body can be smuggled beneath the parser
- malformed/missing parsed requests reject
- protected list responses are filtered per item where supported
- `tools/call` receives call-time authorization rather than relying on list visibility.

This independently confirms the cross-project rule already emerging from ARD/Gateway research: **visibility/discovery is not execution authority**.

## Trace C — project package install → provenance → lock

Project-scoped skill install
→ resolve plain-name registry entry / OCI / git source
→ determine existing lock trust anchor; on true first use prefer catalog-declared provenance when present
→ otherwise keyless/key-based signature verification; absent prior/catalog constraint falls back to trust-on-first-use identity recording
→ explicit unsigned exception only when requested/allowed
→ verify before artifact extraction/recording
→ validate paths/no symlinks and sanitize installed material
→ write client skill directory
→ record exact source/digest/version/provenance/signature bundle in project lock state.

Subsequent install/sync/upgrade enforces the recorded trust anchor rather than accepting whatever signer an artifact presents.

Useful mechanisms:
- trust anchor is chosen by lock/catalog/caller state, never by the downloaded artifact itself
- conflicting caller key vs recorded signer/key stops rather than choosing one silently
- project lock distinguishes content identity from mutable tag/location
- catalog can constrain first install so first use need not be blind.

Qualification limits:
- when neither a lock nor catalog expectation exists, keyless first use is still TOFU.
- user-scoped skills deliberately do not use the project lock-backed verification model.
- image verification for executable MCP container workloads is less strict by default than project skill verification.

Primary implementation evidence:
- `pkg/skills/skillsvc/verify.go`
- `pkg/skills/lockfile/`
- `docs/arch/12-skills-system.md`
- verifier/installation tests.

## Tests/invariants extracted

### Workload/runtime
- policy denial stops creation before `RunWorkloadDetached`
- network-isolation/network-mode contradictions have explicit behavior
- isolation defaults and update semantics are testable configuration state
- restart paths retain a runtime safety net.

### MCP surface/authority
- filtering and renaming are tested in both list and call directions
- malformed tool calls reject
- unknown authz methods deny
- list authorization can filter individual resources/tools
- non-JSON/malformed protected requests reject rather than reaching the backend unauthorised.

### Package provenance
- signer/key mismatch rejects
- first-use/catalog provenance constraints are enforced when supported
- lock state pins trust decisions and digest/source
- explicit unsigned exception is distinct from failed verification.

## Boundary findings

### Desired RunConfig ↔ applied confinement
Strong pattern. Declarative security is inspectable/restartable, but actual mounts/network/privilege remain the runtime truth. A consumer should verify both rather than treating configuration text as evidence of enforcement.

### Tool catalog ↔ tool execution
Separated in architecture. Tool filtering is presentation/routing policy; Cedar authorization is a separate optional authority layer.

### Middleware ordering ↔ security semantics
Order is load-bearing. Tool filtering happens before mutating webhooks. ToolHive documents that a mutating webhook can rename an allowed request into a tool excluded by `--tools`; a later independent authorizer may still stop it, but the filter itself no longer represents what is executed.

### Request read failure ↔ filtering
`NewToolCallMappingMiddleware` passes the request onward if `io.ReadAll(r.Body)` fails. Parser/body-limit/authz layers may independently reject the resulting request, but the tool-filter layer itself is fail-open on this error and therefore is not suitable as a hard authority boundary.

### Registry/catalog ↔ activation trust
Catalog metadata can seed an expected provenance constraint; lock state then binds subsequent artifacts. This is useful prior art for SPARK source qualification, but TOFU and explicit unsigned modes are weaker than a canonical high-assurance activation policy.

## Determinization candidates

- versioned execution/run specification
- profile resolution and mount/network validation
- desired-vs-applied runtime verification
- tool-list shaping and alias mapping
- typed MCP parsing
- per-call authorization
- middleware-order validation
- package digest/provenance checking
- lock-state update/reconciliation
- audit/telemetry generation.

## Material extracted

IDEA: a governed tool runtime should package execution, isolation, transport, policy, provenance and evidence as one inspectable workload contract.

PATTERN: declarative workload security envelope backed by runtime verification.

PATTERN: presentation/discovery filtering is separate from call-time authority.

PATTERN: external project packages can bind content digest + provenance to durable lock state before future activation.

ALGORITHM: deterministic list/call alias mapping, method classification, permission-profile resolution and trust-anchor selection.

CODE: no direct reuse recommendation. ToolHive is Go/infrastructure-heavy and broader than the SPARK Rust core. Individual algorithms/tests are strong prior art.

TEST/INVARIANT: declared confinement must be checked against applied runtime state; visibility cannot grant authority; provenance warnings cannot count as verified activation.

## Primary disposition

BORROW_PATTERN

## Secondary disposition

EXPERIMENT_NOW for:
1. a minimal Rust RunSpec/Tool Contract carrying execution + confinement + provenance identity;
2. declared-vs-applied verification against a disposable adapter runtime;
3. tool visibility vs independent call-time authority tests;
4. hard-fail provenance qualification before executable activation.
