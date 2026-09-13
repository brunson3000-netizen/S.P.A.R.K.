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
PROJECT RESPONSE: preserve richer material outside prompt and expose references/search/retrieval.
DID IT WORK?: context-compress improves recoverability but does not preserve an immutable byte-exact raw artifact; RTK source trace remains pending.
BOUNDARY AFFECTED: deterministic execution ↔ agent-visible evidence.
S.P.A.R.K. LESSON: small context, full evidence requires a canonical raw evidence object separate from derived compression/search state.
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
CAUSE: for non-HTTP MCP methods not admitted by the `methods` list, compatibility logic also checks the `tools` list. A tools wildcard can therefore admit a non-`tools/call` MCP method through the legacy fallback.
PROJECT RESPONSE: HTTP verbs were explicitly hardened so they never fall back to tools; MCP compatibility fallback remains.
DID IT WORK?: preserves legacy scope behavior, but weakens separation between protocol-method authority and specific-tool authority.
BOUNDARY AFFECTED: protocol method ↔ tool capability authorization.
S.P.A.R.K. LESSON: do not share wildcard/value namespaces between protocol operations and executable capabilities. Model them as structurally separate grant types.
TRANSFER CONFIDENCE: HIGH for the pinned source path.
DISPOSITION: ARCHIVE_REFERENCE / negative acceptance criterion.
EVIDENCE: `sources/MCP_GATEWAY_REGISTRY.md`; upstream `auth_server/server.py`.

## F-009 — Searchable spillover is not canonical/full evidence
OBSERVED IN: Open330/context-compress @ `59fae35a...`.
CAUSE: executor strips ANSI before `indexableStdout`; capture has a hard cap; content indexer trims/chunks/drops some separator structure and may overlap lines; search emits bounded snippets, not a raw-content read; store is ephemeral by default and retains only a bounded number of sources by default.
PROJECT RESPONSE: keep a richer uncompressed searchable corpus outside the agent-visible response and allow source-scoped follow-up search.
DID IT WORK?: strong context-reduction/recoverability mechanism, but it does not prove or provide immutable byte-exact reconstruction of the original operation output.
BOUNDARY AFFECTED: execution result ↔ evidence store ↔ agent view.
S.P.A.R.K. LESSON: searchable/chunked material is derived evidence state. Canonical raw evidence must be stored separately with its own immutable identity/digest and retention policy.
TRANSFER CONFIDENCE: HIGH.
DISPOSITION: BORROW_PATTERN with required strengthening.
EVIDENCE: `sources/CONTEXT_COMPRESS.md`; upstream `executor.ts`, `store.ts`, `search.ts`, `round-trip.test.ts`.

## F-010 — “Sandboxed subprocess” is not containment
OBSERVED IN: Open330/context-compress execute tool and `SECURITY.md`.
CAUSE: execution uses a temp working directory, safe environment allowlist, timeouts, process-tree kill and output caps, but no seccomp/AppArmor/container/microVM boundary; subprocesses run with the MCP server user’s privileges and unrestricted outbound networking.
PROJECT RESPONSE: documentation explicitly lists no OS-level sandbox as a known limitation and recommends external container/gVisor/network controls for high-security use.
DID IT WORK?: resource hygiene and secret-minimization are useful; isolation claim must not be inferred.
BOUNDARY AFFECTED: LLM/tool executor ↔ host OS/network.
S.P.A.R.K. LESSON: reserve “sandbox/containment” for an actual enforced authority boundary. A subprocess with limits is an executor, not a security sandbox.
TRANSFER CONFIDENCE: HIGH.
DISPOSITION: ARCHIVE_REFERENCE / naming and acceptance invariant.
EVIDENCE: `sources/CONTEXT_COMPRESS.md`; upstream `SECURITY.md`, `executor.ts`, `execute.ts`.

## F-011 — Portable skill names and scope shadowing are not provenance identity
OBSERVED IN: Agent Skills @ `69ef37e...`.
CAUSE: `name` is a local package name; client guidance uses deterministic project-over-user shadowing and permits multiple discovery roots. A repository-provided skill can therefore replace the human-visible name of a user skill in the active catalog if the host treats name alone as identity.
PROJECT RESPONSE: client guide recommends deterministic collision handling and separately warns that project skills may be untrusted.
DID IT WORK?: good interoperability convention, insufficient as governed canonical identity.
BOUNDARY AFFECTED: skill discovery/source ↔ model-facing teaching identity.
S.P.A.R.K. LESSON: preserve the interoperable skill name as an alias, but bind active teaching content to host-owned source/provenance/version/digest identity before loading it.
TRANSFER CONFIDENCE: HIGH.
DISPOSITION: BORROW_PATTERN with provenance strengthening.
EVIDENCE: `sources/AGENT_SKILLS.md`; `patterns/SKILLS_TEACH_AUTHORITY_ACTS.md`.

## F-012 — Teaching-package allowlists must not become execution grants
OBSERVED IN: Agent Skills optional `allowed-tools` plus client guidance to allowlist skill directories for frictionless resource reads.
CAUSE: instructional metadata and resource readability can be mistaken for host execution authority, especially when skills bundle executable scripts.
PROJECT RESPONSE: the spec labels `allowed-tools` experimental and leaves permission enforcement to the client/harness.
DID IT WORK?: portable teaching works; authority semantics are intentionally not standardized.
BOUNDARY AFFECTED: instructions/resources ↔ executable tools/scripts.
S.P.A.R.K. LESSON: loading a skill can teach and make its read-only resources addressable; every actual script/tool side effect still passes current host authorization.
TRANSFER CONFIDENCE: HIGH.
DISPOSITION: BORROW_PATTERN with hard separation invariant.
EVIDENCE: `sources/AGENT_SKILLS.md`; Agent Skills specification/client/script guides.

## F-013 — “Nothing risky found” can become a stale trust grant
OBSERVED IN: grok-build folder-trust design explicitly prevents this failure.
CAUSE: if a repository scan finds no behavior-bearing config and that allow is cached as durable trust, a later pull/write can add MCP/LSP/policy/instruction/skill configuration that loads without a fresh trust decision.
PROJECT RESPONSE: grok-build treats the no-config allow as provisional/non-cached and re-scans on a later resolve.
DID IT WORK?: source contract is explicit and the cache design preserves the distinction.
BOUNDARY AFFECTED: repository state change ↔ project-behavior loader trust.
S.P.A.R.K. LESSON: absence of a risky artifact is an observation, not authorization for future artifacts. Cache durable trust decisions, not transient absence.
TRANSFER CONFIDENCE: HIGH.
DISPOSITION: BORROW_PATTERN.
EVIDENCE: `sources/GROK_BUILD.md`; `patterns/REPOSITORY_TRUST_GATE.md`.

## F-014 — Durable memory writes can silently clobber newer truth
OBSERVED IN: grok-build memory v2 explicitly defends against this failure.
CAUSE: an agent can read a durable note, spend time reasoning, then overwrite a file that another worker/user changed meanwhile.
PROJECT RESPONSE: existing-file replacement requires a recorded read snapshot; immediately before atomic replacement the current bytes are hashed and compared. Mismatch returns stale and preserves the newer file.
DID IT WORK?: dedicated tests mutate a file after the recorded read and assert the later agent write is rejected without changing the external update.
BOUNDARY AFFECTED: historical/model-visible memory ↔ durable mutable state.
S.P.A.R.K. LESSON: editable durable state needs optimistic concurrency semantics; “I read it earlier” is not enough.
TRANSFER CONFIDENCE: HIGH.
DISPOSITION: BORROW_PATTERN.
EVIDENCE: `sources/GROK_BUILD.md`; `patterns/DURABLE_CAPTURE_RECONCILIATION.md`.
