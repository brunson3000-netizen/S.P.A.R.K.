# acp-spec — MCI source screen

Status: DOCUMENTATION SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [agentclientprotocol/agent-client-protocol](https://github.com/agentclientprotocol/agent-client-protocol/tree/ada6b108389a63a2625298f2be3eacde33a1d8c5). Exact pin: `ada6b108389a63a2625298f2be3eacde33a1d8c5`. Runtime: Protocol specification. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Client-agent implementers need a common session/permission/cancellation vocabulary; JSON-RPC requests and notifications carry progress and results.

Protocol sessions and cancellation IDs correlate activity. They cannot confer authority or prove host-process termination; state and enforcement belong to implementers.

## Mechanism and failure semantics

At the studied pin, protocol v1 is separate from schema/package release versions and v2 is draft material. Generic $/cancel_request is optional; supporting implementations MAY cancel the corresponding and nested activity, while answering the original request. Feature-level turn cancellation is a distinct contract.

## Evidence and tests

Specification screen, not implementation qualification. An example cascading-cancellation diagram is illustrative, not executed evidence.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [README.md](https://github.com/agentclientprotocol/agent-client-protocol/blob/ada6b108389a63a2625298f2be3eacde33a1d8c5/README.md)
- [docs/protocol/v1/cancellation.mdx](https://github.com/agentclientprotocol/agent-client-protocol/blob/ada6b108389a63a2625298f2be3eacde33a1d8c5/docs/protocol/v1/cancellation.mdx)
- [LICENSE](https://github.com/agentclientprotocol/agent-client-protocol/blob/ada6b108389a63a2625298f2be3eacde33a1d8c5/LICENSE)

## MCI transfer

Borrow typed lifecycle vocabulary and explicit cancellation outcomes for the MCI adapter boundary. Require an independent stop/cleanup receipt before reporting work terminated. Preserve version negotiation and avoid mixing v1 requirements with draft v2.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: PROTO; cross-references: GOV, COORD. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
