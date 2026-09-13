# Failure Ledger

Status: ACTIVE

Use one entry per durable failure lesson.

---

## Template

FAILURE MODE: <name>
OBSERVED IN: <project/module/test/commit>
CAUSE:
PROJECT RESPONSE:
DID IT WORK?:
BOUNDARY AFFECTED:
S.P.A.R.K. LESSON:
TRANSFER CONFIDENCE: <HIGH | MEDIUM | LOW>
DISPOSITION: <decision bucket>
EVIDENCE:

---

## F-001 — Tool catalog overload

OBSERVED IN: progressive MCP Guardian and current on-demand tool-discovery ecosystem.
CAUSE: eagerly injecting many complete tool schemas into model context consumes large context and increases selection burden before work starts.
PROJECT RESPONSE: replace the full catalog with a small discovery/schema/execution surface; retrieve details only when needed.
DID IT WORK?: upstream benchmark claims are promising but require independent SPARK reproduction.
BOUNDARY AFFECTED: agent ↔ tool catalog.
S.P.A.R.K. LESSON: discovery metadata and executable capability detail should be separable; catalog membership must not imply authority.
TRANSFER CONFIDENCE: MEDIUM pending controlled experiment.
DISPOSITION: EXPERIMENT_NOW.
EVIDENCE: `sources/PROGRESSIVE_MCP_GUARDIAN.md`; pinned middle-layer source.

## F-002 — Compression can hide load-bearing evidence

OBSERVED IN: general output-compression problem; RTK qualification risk; context-compress/ctx-zip comparison lane.
CAUSE: reducing agent-visible output can discard the exact evidence needed for diagnosis or later review.
PROJECT RESPONSE: preserve original/raw material outside the prompt and expose compact summaries/references/search/retrieval.
DID IT WORK?: project-specific mechanisms require independent verification for fidelity and recoverability.
BOUNDARY AFFECTED: deterministic execution ↔ agent-visible evidence.
S.P.A.R.K. LESSON: compact presentation must never become evidence destruction. Preferred doctrine: small context, full evidence.
TRANSFER CONFIDENCE: HIGH as a safety requirement; implementation choice unproven.
DISPOSITION: EXPERIMENT_NOW.
EVIDENCE: forthcoming P0 context-compression trace and recovery tests.

## F-003 — Discovery index doubles as execution authority

OBSERVED IN: `S1LV3RJ1NX/mcp-guardian` commit `4c6a045...`, `proxy.py::execute_tool` + `index.py`.
CAUSE: the filtered in-memory index is both the discovery catalog and the execution admission mechanism; execution checks only whether `tool_name` is present in `index.entries` before forwarding.
PROJECT RESPONSE: scope filtering excludes blocked tools from index; tests and security benchmark verify blocked names are absent from search/schema/execute.
DID IT WORK?: it enforces the project’s static scope model, but it does not provide a separate current grant/authority decision at call time.
BOUNDARY AFFECTED: discovery ↔ execution authority.
S.P.A.R.K. LESSON: borrow scope-filtered discovery, reject authority coupling. Discovery is advisory; execution must reauthorize independently at call time.
TRANSFER CONFIDENCE: HIGH.
DISPOSITION: BORROW_PATTERN for discovery only; authority mechanism rejected.
EVIDENCE: `sources/PROGRESSIVE_MCP_GUARDIAN.md`; upstream `tests/test_proxy.py`, `tests/test_index.py`, security benchmark.

## F-004 — Auth helper and active call path are disconnected

OBSERVED IN: same pinned MCP Guardian source.
CAUSE: `config.py` accepts `static_header` and `token_passthrough`; `auth.py::get_auth_headers` implements and tests them. The active `UpstreamManager._resolve_auth` path does not call that helper and handles only bearer/OAuth-specific cases; `Guardian._get_client_headers()` returns an empty dict.
PROJECT RESPONSE: none found in this pinned execution path.
DID IT WORK?: not demonstrated for the advertised static-header/token-passthrough path in the traced execution flow.
BOUNDARY AFFECTED: client identity/credential ↔ upstream tool.
S.P.A.R.K. LESSON: test a feature through its real call path, not just a helper in isolation. Security-sensitive helpers can exist while being bypassed by integration code.
TRANSFER CONFIDENCE: HIGH for the pinned commit.
DISPOSITION: ARCHIVE_REFERENCE as a failure lesson.
EVIDENCE: upstream `config.py`, `auth.py`, `upstream.py`, `proxy.py`, `tests/test_auth.py`.

## F-005 — Stored schema is not locally enforced before forwarding

OBSERVED IN: same pinned MCP Guardian source, `proxy.py::execute_tool`.
CAUSE: full schemas are stored for learning, but `params` are passed to `UpstreamManager.call_tool` without explicit validation against the stored schema in the proxy path.
PROJECT RESPONSE: downstream MCP/FastMCP may reject invalid arguments; no proxy-local validation was found in first trace.
DID IT WORK?: sufficient for a transparent proxy, insufficient as evidence of a host-owned typed execution boundary.
BOUNDARY AFFECTED: agent input ↔ executable tool.
S.P.A.R.K. LESSON: schema retrieval and input enforcement are different states. A SPARK capability broker should validate call input at the host boundary before side effects.
TRANSFER CONFIDENCE: HIGH for absence in this path; downstream behavior remains separate.
DISPOSITION: BORROW_IDEA negative lesson / add acceptance invariant.
EVIDENCE: upstream `proxy.py` and `index.py`.
