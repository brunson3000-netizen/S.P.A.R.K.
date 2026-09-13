# Pending Action — Middle-Layer Qualification

Status: `PENDING_EXECUTION`
Activation state: `SOURCE_HARVEST_ONLY`

The source set is pinned. Qualification may inspect, build in disposable environments, benchmark, and write evidence. It must not activate gateways/hooks, introduce production dependencies, store live credentials, or change another project's authority.

## P0 — qualify first

### ARD spec
- Map ARD resource identity/discovery fields against S.P.A.R.K. capability needs.
- Determine what ARD can represent cleanly and what must remain host-owned metadata.
- Test federation, versioning, stale records, trust/provenance, duplicate identity, and discovery-without-authorization.

### Progressive mcp-guardian
- Reproduce the three-meta-tool surface against a controlled large catalog.
- Measure baseline context tokens, search hit rate, wrong-tool rate, schema-load cost and execution latency.
- Test denied/blocked tools, stale schemas, ambiguous aliases, search poisoning, and exact audit behavior.
- Compare its pattern with GitHub-style deferred tool loading and ARD discovery.

### MCP Gateway & Registry
- Extract control-plane/data-plane boundaries, capability discovery, agent/skill registration, access model, federation and peer-to-peer A2A mechanics.
- Identify which pieces are reusable patterns versus assumptions tied to nginx/FastAPI/Kubernetes/cloud identity stacks.
- Explicitly test the rule that registry metadata and local scopes do not become host authority.

### context-compress
- Verify exactly which output is lossless/index-backed versus genuinely summarized or discarded.
- Measure compression ratio, retrieval precision/recall, deterministic addressability, artifact lifetime, tamper detection and large-trace behavior.
- Require full-evidence retrieval after compact projection.

### Agent Skills spec
- Map `SKILL.md`, scripts, references and assets against the existing S.P.A.R.K. research/tool-teaching needs.
- Separate procedural teaching from executable contract and authorization.
- Test selective loading, versioning, conflict resolution, provenance and untrusted skill content.

## P1 — qualify after P0 shape is known

### agentgateway
Compare MCP/A2A/LLM routing, RBAC, budgets, rate limits and OTel against the P0 control-plane hypothesis.

### AgentTrace
Evaluate parent/child trajectory identity, event model, cost/latency accounting, local retention and replay/export usefulness.

### OpenZiti MCP Gateway
Extract zero-trust identity, encrypted connectivity, per-client isolation and tool-filtering mechanics. Determine what is protocol-specific and what generalizes.

## P2 — focused prior-art studies

### E2B Runtime
Study microVM lifecycle, Firecracker isolation, snapshot/resume, image provenance, network/filesystem boundaries and disposable-workspace economics. No infrastructure deployment in this qualification stage.

### agent-observability
Compare tool-call audit semantics with AgentTrace trajectory semantics; identify useful local evidence fields and limitations.

### ctx-zip
Study only as a combined progressive-disclosure/output-reference comparison. Requalify maturity before considering implementation reuse.

## Falsification gates

The middle-layer hypothesis is weakened or rejected if qualification shows any of the following cannot be controlled acceptably:

- discovery regularly selects the wrong capability under realistic catalogs;
- hidden schema loading recreates the same context cost elsewhere;
- compact output cannot reliably recover full evidence;
- registry search permits untrusted metadata to escalate execution authority;
- stale contracts or skills produce silent wrong execution;
- discovery/gateway latency outweighs context savings for normal workloads;
- audit records cannot tie a visible result to exact execution and source provenance;
- sandbox lifecycle leaks state or capabilities across disposable cells.

## First recommended experiment

Build a noncanonical lab with a synthetic 100–300 capability catalog and a tiny agent-facing facade. Compare:

1. all schemas loaded up front;
2. progressive search → schema → execute;
3. ARD-like resource discovery → selected contract/skill → execute;
4. compact result only;
5. compact result + retrievable full evidence.

Measure context tokens, discovery accuracy, total latency, wrong-tool rate, denied-effect handling, full-evidence recovery and audit completeness.

No result from this experiment constitutes adoption.
