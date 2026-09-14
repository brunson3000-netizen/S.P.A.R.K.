# acp-rust-sdk — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [agentclientprotocol/rust-sdk](https://github.com/agentclientprotocol/rust-sdk/tree/3a6d0ae88dbaa09fcb74e26761e3643a75b2015e). Exact pin: `3a6d0ae88dbaa09fcb74e26761e3643a75b2015e`. Runtime: Rust ACP session SDK. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Rust client-agent adapters need typed sessions and dispatch; connection/session builders route messages and results.

Session handles and resumed session state are protocol references. Agent/client implementations supply actual tool authority and persistence.

## Mechanism and failure semantics

Session module exposes typed SessionId-based sessions, connection/session construction, handler routing and resume facilities. README distinguishes released protocol from evolving/draft features and gates unstable functionality.

## Evidence and tests

Narrow session/API screen only; dispatch through every permission/terminal handler and cancellation-to-process teardown is unqualified.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [README.md](https://github.com/agentclientprotocol/rust-sdk/blob/3a6d0ae88dbaa09fcb74e26761e3643a75b2015e/README.md)
- [src/agent-client-protocol/src/session.rs](https://github.com/agentclientprotocol/rust-sdk/blob/3a6d0ae88dbaa09fcb74e26761e3643a75b2015e/src/agent-client-protocol/src/session.rs)
- [LICENSE](https://github.com/agentclientprotocol/rust-sdk/blob/3a6d0ae88dbaa09fcb74e26761e3643a75b2015e/LICENSE)

## MCI transfer

Rust fit is good for a future ACP adapter, but demand and negotiated protocol version should precede dependency selection. Borrow adapter boundaries now; compare current API compatibility and add host authorization, durable session mapping and stop receipts before considering code reuse.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: PROTO; cross-references: GOV, COORD, TEST. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
