# agentgateway — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [agentgateway/agentgateway](https://github.com/agentgateway/agentgateway/tree/5e5633bffe6dc5fdd29256648987197f366e462a). Exact pin: `5e5633bffe6dc5fdd29256648987197f366e462a`. Runtime: Rust gateway with wider controller ecosystem. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Gateway operators need MCP authorization; resource/method and CEL rule sets yield route admission decisions.

Configured authorization sets bind request attributes. An absent rule set is permissive in the inspected evaluator; a gateway route is not a grant from SWARM.

## Mechanism and failure semantics

McpAuthorizationSet::validate allows an empty set and binds method/resource into CEL evaluation. RuleSets::validate evaluates denies, requires and allows; matching deny wins, requires must all pass, allow matches admit, and a deny-only set with no match permits. Empty policy is not default-deny.

## Evidence and tests

Inline evaluator tests inspected, including empty-rule behavior; full authenticated request-to-upstream path and live reload/revocation not qualified.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [crates/agentgateway/src/mcp/rbac.rs](https://github.com/agentgateway/agentgateway/blob/5e5633bffe6dc5fdd29256648987197f366e462a/crates/agentgateway/src/mcp/rbac.rs)
- [crates/agentgateway/src/http/authorization.rs](https://github.com/agentgateway/agentgateway/blob/5e5633bffe6dc5fdd29256648987197f366e462a/crates/agentgateway/src/http/authorization.rs)
- [LICENSE](https://github.com/agentgateway/agentgateway/blob/5e5633bffe6dc5fdd29256648987197f366e462a/LICENSE)

## MCI transfer

Borrow deterministic policy composition and method/resource binding. MCI needs an explicit configured admission policy, identity provenance and call-time epoch checks. Rust is a useful reference; controller/network framework dependencies should not become Core dependencies accidentally.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: PROTO; cross-references: GOV, SEC, AUDIT. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
