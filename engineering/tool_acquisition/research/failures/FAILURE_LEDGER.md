# Failure Ledger

Status: ACTIVE

## F-001 — Tool catalog overload

OBSERVED IN: progressive MCP Guardian and search-first discovery ecosystem.
CAUSE: eager full-schema injection consumes context and increases selection burden.
PROJECT RESPONSE: small discovery surface; details on demand.
DID IT WORK?: promising upstream evidence; independent SPARK experiment pending.
BOUNDARY AFFECTED: agent ↔ catalog.
S.P.A.R.K. LESSON: separate compact discovery metadata from executable detail.
TRANSFER CONFIDENCE: MEDIUM pending experiment.
DISPOSITION: EXPERIMENT_NOW.
EVIDENCE: `sources/PROGRESSIVE_MCP_GUARDIAN.md`.

## F-002 — Compression can hide load-bearing evidence

OBSERVED IN: RTK/context-compression research lane.
CAUSE: reducing visible output can destroy evidence needed later.
PROJECT RESPONSE: preserve originals outside prompt and expose references/search/retrieval.
DID IT WORK?: project-specific fidelity still to verify.
BOUNDARY AFFECTED: deterministic execution ↔ agent-visible evidence.
S.P.A.R.K. LESSON: small context, full evidence.
TRANSFER CONFIDENCE: HIGH as requirement.
DISPOSITION: EXPERIMENT_NOW.

## F-003 — Discovery index doubles as execution authority

OBSERVED IN: progressive MCP Guardian commit `4c6a045...`, `proxy.py::execute_tool` + `index.py`.
CAUSE: filtered index is both discovery catalog and execution admission; execution checks index membership before forwarding.
PROJECT RESPONSE: allow/block filtering; tests verify blocked names are absent from search/schema/execute.
DID IT WORK?: enforces the project’s static scope model, but no independent current grant check exists at call time.
BOUNDARY AFFECTED: discovery ↔ execution authority.
S.P.A.R.K. LESSON: discovery is advisory; execution reauthorizes independently every call.
TRANSFER CONFIDENCE: HIGH.
DISPOSITION: BORROW_PATTERN for discovery only; authority coupling rejected.

## F-004 — Auth helper and active call path are disconnected

OBSERVED IN: same MCP Guardian pin.
CAUSE: config accepts `static_header` and `token_passthrough`; `auth.py::get_auth_headers` implements/tests them, but active `UpstreamManager._resolve_auth` does not use that helper and `Guardian._get_client_headers()` returns `{}`.
PROJECT RESPONSE: none found in pinned active path.
DID IT WORK?: not demonstrated end-to-end for those advertised modes.
BOUNDARY AFFECTED: client credential ↔ upstream tool.
S.P.A.R.K. LESSON: security features must be tested through the real call path, not isolated helpers.
TRANSFER CONFIDENCE: HIGH for pinned commit.
DISPOSITION: ARCHIVE_REFERENCE.

## F-005 — Stored schema is not locally enforced before forwarding

OBSERVED IN: MCP Guardian `proxy.py::execute_tool`.
CAUSE: full schemas are stored for learning, but `params` are forwarded without explicit proxy-local schema validation.
PROJECT RESPONSE: downstream protocol/library may reject invalid arguments.
DID IT WORK?: acceptable for a transparent proxy; insufficient as a host-owned typed execution boundary.
BOUNDARY AFFECTED: agent input ↔ executable tool.
S.P.A.R.K. LESSON: learning a schema and enforcing it are different states; host validates before side effects.
TRANSFER CONFIDENCE: HIGH for absence in traced path.
DISPOSITION: BORROW_IDEA negative lesson.

## F-006 — Bare tool names collide across upstream servers

OBSERVED IN: MCP Guardian `ToolIndex.entries: dict[str, ToolEntry]` keyed by `tool.name`; each server writes `self.entries[tool.name]`.
CAUSE: tool identity is not namespaced by server/publisher/stable resource ID.
PROJECT RESPONSE: none found in first trace; existing tests use non-colliding names and do not cover duplicate names across servers.
DID IT WORK?: ambiguous by construction; later indexed server can replace an earlier entry with the same name.
BOUNDARY AFFECTED: catalog identity ↔ routing/execution.
S.P.A.R.K. LESSON: display/tool names cannot be canonical IDs. Use stable collision-resistant capability identity; record aliases separately.
TRANSFER CONFIDENCE: HIGH.
DISPOSITION: REIMPLEMENT_IN_RUST as a required identity property, not reuse of the reference keying model.
EVIDENCE: MCP Guardian `index.py`; contrast ARD domain-anchored identifiers.

## F-007 — ARD search can return a stable ID without a normative full-entry lookup

OBSERVED IN: ARD v0.91 §5.3.2.
CAUSE: search results may omit every term except `identifier`; normative retrieval of the complete ARD entry by identifier is explicitly out of scope in this draft.
PROJECT RESPONSE: client obtains the full entry/artifact from the source that published it when source information is available.
DID IT WORK?: sufficient for a discovery specification, but leaves a gap for a generic `learn_capability(id)` API.
BOUNDARY AFFECTED: discovery result ↔ complete capability descriptor.
S.P.A.R.K. LESSON: the middle layer needs an explicit deterministic descriptor-resolution step bound to stable ID/version/source.
TRANSFER CONFIDENCE: HIGH.
DISPOSITION: BORROW_PATTERN with local completion of the learn/resolve step.
EVIDENCE: `sources/ARD.md` and ARD spec.

## F-008 — Legacy method/tool fallback blurs authorization namespaces

OBSERVED IN: MCP Gateway & Registry `auth_server/server.py::validate_server_tool_access` @ `7c353798...`.
CAUSE: for non-HTTP MCP methods not admitted by the `methods` list, compatibility logic also checks the `tools` list. A tools wildcard (`*`/`all`) can therefore admit a non-`tools/call` MCP method through the legacy fallback.
PROJECT RESPONSE: HTTP verbs were explicitly hardened so they never fall back to tools; MCP compatibility fallback remains.
DID IT WORK?: preserves legacy scope behavior, but weakens the conceptual separation between protocol-method authority and specific-tool authority.
BOUNDARY AFFECTED: protocol method ↔ tool capability authorization.
S.P.A.R.K. LESSON: do not share wildcard/value namespaces between protocol operations and executable capabilities. Model them as structurally separate grant types.
TRANSFER CONFIDENCE: HIGH for the pinned source path.
DISPOSITION: ARCHIVE_REFERENCE / negative acceptance criterion.
EVIDENCE: `sources/MCP_GATEWAY_REGISTRY.md`; upstream `auth_server/server.py`.
