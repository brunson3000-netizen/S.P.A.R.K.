# n8n — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [n8n-io/n8n](https://github.com/n8n-io/n8n/tree/4169b55bf3b3e6c255d7361642bc5243bd04345a). Exact pin: `4169b55bf3b3e6c255d7361642bc5243bd04345a`. Runtime: TypeScript workflow platform. License evidence: Sustainable Use License with exclusions; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Workflow users need many integrations and namespaced MCP tools; server/tool descriptors become callable workflow tools.

Handlers execute external integrations using configured credentials. Workflow state and displayed tool aliases do not establish SWARM authority.

## Mechanism and failure semantics

Resolver keeps original server/tool identity separately from normalized display names, resolves collisions, and forwards original tool calls with abortSignal. This is useful identity/adapter prior art; abort propagation is cooperative.

## Evidence and tests

Selected resolver screened; enterprise authorization, credential storage, workflow retries and production deployment not qualified.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [packages/@n8n/agents/src/runtime/mcp/mcp-tool-resolver.ts](https://github.com/n8n-io/n8n/blob/4169b55bf3b3e6c255d7361642bc5243bd04345a/packages/@n8n/agents/src/runtime/mcp/mcp-tool-resolver.ts)
- [LICENSE.md](https://github.com/n8n-io/n8n/blob/4169b55bf3b3e6c255d7361642bc5243bd04345a/LICENSE.md)

## MCI transfer

Archive as integration and operator-interface reference. Root license has Sustainable Use terms and enterprise exclusions, so public code availability must not be labeled unrestricted open-source reuse. Prefer an independently specified small adapter interface over importing the workflow platform.

Finding classes: reference/qualification assessment. Filing home: COORD; cross-references: GOV, TOOLS, AUDIT. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

ARCHIVE_REFERENCE
