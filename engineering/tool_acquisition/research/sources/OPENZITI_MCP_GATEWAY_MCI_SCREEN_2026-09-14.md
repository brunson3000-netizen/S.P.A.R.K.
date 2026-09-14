# openziti-mcp-gateway — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [openziti/mcp-gateway](https://github.com/openziti/mcp-gateway/tree/8f99623d95d2f5223d2fa12b9f125688d8c80bf9). Exact pin: `8f99623d95d2f5223d2fa12b9f125688d8c80bf9`. Runtime: Go gateway; OpenZiti/zrok/Agora ecosystem. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Gateway operators need path-scoped tool admission; tool name and settled arguments are checked before backend forwarding.

Per-connection CallPolicy trusts configured roots and local filesystem canonicalization. This is local path admission, not OS containment.

## Mechanism and failure semantics

Prepare passes ungoverned tools unchanged; governed calls settle JSON once, reject ambiguous/malformed path inputs and resolve symlinks. Missing leaf resolution uses an existing parent; dangling symlinks fail. ValidateTools rejects configured rules for unadvertised tools.

## Evidence and tests

Tests inspected for duplicate keys, one-time serialization, symlinks, dangling links and denial before backend dispatch. Config test rejects path policy on a remote backend, correctly limiting local-path assumptions. No tests executed.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [aggregator/policy.go](https://github.com/openziti/mcp-gateway/blob/8f99623d95d2f5223d2fa12b9f125688d8c80bf9/aggregator/policy.go)
- [aggregator/policy_test.go](https://github.com/openziti/mcp-gateway/blob/8f99623d95d2f5223d2fa12b9f125688d8c80bf9/aggregator/policy_test.go)
- [LICENSE](https://github.com/openziti/mcp-gateway/blob/8f99623d95d2f5223d2fa12b9f125688d8c80bf9/LICENSE)

## MCI transfer

Borrow settled-argument identity and pre-dispatch path checks. Add handle-based filesystem enforcement or equivalent to close check/use races; do not interpret an earlier path check as a sandbox. Whole-network deployment is much heavier than a local Rust adapter.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: SEC; cross-references: GOV, PROTO. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
