# Agent–Tool Middle-Layer Architecture Hypothesis

Status: `PROBATIONARY_RESEARCH_HYPOTHESIS`

This record captures the architectural shape suggested by the middle-layer harvest. It is not an adopted S.P.A.R.K. architecture and creates no authority for another project.

## Hypothesis

A specialist agent should not receive a large static toolbox. It should receive a small stable interface through which it can discover and use governed capabilities only when needed.

Candidate conceptual surface:

```text
find_capability(intent)
learn_capability(capability_id)
execute_capability(capability_id, request)
report_result(execution_id)
```

The names and number of calls are not frozen. The useful invariant is progressive disclosure: discovery metadata first, full contract only after selection, execution only after authorization.

## Candidate responsibility chain

```text
AGENT / SPECIALIST
      |
      | tiny stable surface
      v
DISCOVERY
  ARD / semantic or lexical tool search
      |
      v
LEARNING
  contract/schema + Agent Skill + provenance
      |
      v
HOST-OWNED AUTHORIZATION
  grants / effects / scope / budgets / credentials
      |
      v
EXECUTION ROUTING
  native tool | MCP | A2A | application adapter
      |
      v
CONTAINMENT WHERE REQUIRED
  process / Wasm / container / microVM / other bounded cell
      |
      v
ACTUAL SOFTWARE OR SERVICE
      |
      v
RESULT PROJECTION
  compact deterministic summary + evidence handles
      |
      +--> complete original artifacts / trace / logs remain addressable
      |
      v
OBSERVABILITY
  trajectory + consequential-call audit + cost/latency/resource evidence
```

## Required separations

1. **Discovery is not authorization.** Finding a capability cannot grant permission to execute it.
2. **A skill is not a tool contract.** Skill material teaches usage; the executable contract remains machine-checkable.
3. **Transport is not authority.** MCP, A2A, ACP, HTTP, CLI or native bindings carry requests; they do not decide project authority.
4. **Compression is not evidence destruction.** Any lossy projection used for agent context must retain a route to the complete evidence when the task requires it.
5. **Gateway policy is subordinate to host policy.** An imported gateway's RBAC/scopes cannot silently become canonical project authority.
6. **Observability is not truth by itself.** Recorded traces must identify source, parent/child relation, execution identity and artifact provenance.
7. **Sandboxing is not authorization.** Isolation reduces damage from an allowed execution; it does not make an otherwise unauthorized action permissible.

## Candidate capability record

A discoverable capability should eventually be testable against fields such as:

- stable capability ID and version;
- provider/implementation identity;
- purpose and semantic description;
- input/output contract reference;
- skill/reference material pointers;
- effect class;
- required scopes/capabilities;
- credential/network/filesystem/process needs;
- cost/resource budget class;
- provenance/source pin;
- qualification state;
- execution binding(s);
- evidence/result projection policy;
- removal/replacement path.

This is intentionally more than a tool name but less than a live authorization grant.

## Why this is potentially load-bearing

The same agent-facing interface can remain small even as the catalog grows from tens to hundreds or thousands of providers. Catalog growth then increases registry/search complexity rather than model-context size.

The architecture also permits specialization without permanently hard-wiring every worker to five tools or flooding every worker with fifty schemas. A specialist can begin with a narrow doorway and resolve only the capabilities relevant to the current work.

## Qualification question

The next work should try to falsify this architecture, not merely demonstrate it. The critical test is whether progressive discovery plus evidence-preserving result projection materially reduces context and tool-selection error without creating hidden authority, stale-schema, retrieval, latency, or audit failures.
