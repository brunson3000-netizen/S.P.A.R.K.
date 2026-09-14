# Failure Lessons — agent-observability

Source pin: `2658eef467225f376e2e92dc1465839eda2bc113`
Status: FIRST TRACE COMPLETE

## AO-001 — Self-reporting is not independent audit evidence

FAILURE MODE: an observability database appears complete while actions are missing, falsified or misdescribed by the agent being observed.

CAUSE: default mode asks the model to call `log_tool_call` after every real tool operation. The recorder does not independently witness those operations.

SPARK LESSON: label source assurance explicitly. `SELF_REPORTED` annotations can enrich a flight record but can never replace host/interceptor-owned canonical execution events.

CONFIDENCE: HIGH.

DISPOSITION: BORROW_IDEA with strict assurance labeling.

## AO-002 — Proxy loses request/response causality

FAILURE MODE: intercepted tool results are attributed to `unknown`, wrong inputs, wrong duration or wrong step.

CAUSE: proxy writes `_obs_*` metadata onto the request object, serializes it downstream, then looks for those properties on the independent response object. There is no pending map keyed by JSON-RPC request ID. Global `stepNumber` is used as fallback.

TEST GAP: proxy end-to-end test exercises `tools/list`, not `tools/call` correlation.

SPARK LESSON: every intercepted request gets a host-owned event/call ID and protocol correlation key before forwarding; response matching must be deterministic and concurrency-safe.

CONFIDENCE: HIGH.

DISPOSITION: ARCHIVE_REFERENCE + acceptance invariant.

## AO-003 — Reconstructed rationale is presented as a decision record

FAILURE MODE: an audit interface implies it captured why an agent chose an action when the “reasoning” was synthesized after the tool call.

CAUSE: `log_tool_call` automatically writes a decision row whose chosen action is the tool name and whose rationale is the tool output summary (or tool name). No independent decision event is involved.

SPARK LESSON: distinguish `OBSERVED_ACTION`, `AGENT_REPORTED_RATIONALE`, `DERIVED_CLASSIFICATION` and `AUTHORITY_DECISION`. Never relabel one as another.

CONFIDENCE: HIGH.

DISPOSITION: ARCHIVE_REFERENCE + schema invariant.

## AO-004 — Promised payload bound is not enforced in storage path

FAILURE MODE: large or sensitive tool output is copied wholesale into observability storage despite documentation claiming a storage cap.

CAUSE: `database.logToolCall` serializes full input/output JSON with no byte limit. Only the human-readable summary is truncated to 500 characters. No 64KB storage bound was found.

SPARK LESSON: payload bounds/redaction/artifact-reference conversion must be enforced in deterministic code at the recorder boundary, not documented as convention.

CONFIDENCE: HIGH.

DISPOSITION: ARCHIVE_REFERENCE + acceptance invariant.

## AO-005 — Heuristic grade/cost fields overclaim semantics

FAILURE MODE: dashboard “grade” or “cost” is consumed as correctness/economic truth.

CAUSE: grade is purely tool-error-rate thresholds; cost is a fixed $3/million-total-token estimate at this pin. README describes richer correctness/efficiency and model-pricing semantics.

SPARK LESSON: every derived metric must carry method/version/provenance. Estimated scores are observations, never semantic correctness or billing authority.

CONFIDENCE: HIGH.

DISPOSITION: BORROW_IDEA with provenance labeling.

## AO-006 — Local dashboard has no explicit access boundary

FAILURE MODE: raw agent/tool data becomes reachable from another interface/network context because “local” was assumed rather than enforced.

CAUSE: Express API has no authentication middleware, and `app.listen(port)` does not explicitly bind loopback. Session-detail routes return stored tool inputs/outputs.

SPARK LESSON: local storage and local transport are separate properties. Evidence/UI access requires explicit bind + identity/access policy.

CONFIDENCE: HIGH for absent auth/explicit bind in traced source.

DISPOSITION: ARCHIVE_REFERENCE.

## AO-007 — Observability install silently mutates agent configuration

FAILURE MODE: installing a recorder changes what an agent loads/executes and introduces moving-code activation.

CAUSE: npm `postinstall` edits global opencode/Claude MCP configs or creates `~/.mcp.json`; inserted command executes `npx -y agent-obs@latest server`.

SPARK LESSON: acquisition, installation, activation and configuration mutation are distinct gated states. Observation tooling must not silently self-install into another agent’s authority surface, and runtime code should be pinned rather than `@latest`.

CONFIDENCE: HIGH.

DISPOSITION: ARCHIVE_REFERENCE / negative supply-chain invariant.
