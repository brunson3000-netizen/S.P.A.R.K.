# openai-codex — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [openai/codex](https://github.com/openai/codex/tree/516f2780fd227a80cd9fe89488f5039245090b71). Exact pin: `516f2780fd227a80cd9fe89488f5039245090b71`. Runtime: Rust agent core and platform execution backends. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Tool runtimes need unified admission and sandbox handling; a typed request becomes an approval decision, sandbox attempt and result/rejection.

Session/configuration and sandbox permission profiles drive host execution. ToolRuntime supplies approval action and execution behavior; the generic orchestrator cannot prove every implementation's side effects.

## Mechanism and failure semantics

Orchestrator constructs an approval action and call context, rejects Forbidden, waits for NeedsApproval, selects the initial sandbox and runs the attempt. Eligible denied attempts follow explicit retry/escalation policy; attachment-owned network policy cannot be bypassed by escalation.

## Evidence and tests

Inspected tests cover granular approval controls, deny-read blocking escalation/policy bypass and Windows backend preservation/rejection semantics. Tests not run; no cross-platform sandbox assurance claimed.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [codex-rs/core/src/tools/orchestrator.rs](https://github.com/openai/codex/blob/516f2780fd227a80cd9fe89488f5039245090b71/codex-rs/core/src/tools/orchestrator.rs)
- [codex-rs/core/src/tools/sandboxing_tests.rs](https://github.com/openai/codex/blob/516f2780fd227a80cd9fe89488f5039245090b71/codex-rs/core/src/tools/sandboxing_tests.rs)
- [LICENSE](https://github.com/openai/codex/blob/516f2780fd227a80cd9fe89488f5039245090b71/LICENSE)

## MCI transfer

Borrow a single host call pipeline with typed ToolRuntime/approval/sandbox seams. Strong Rust reference but coupled to core session/config/execution crates; extraction is not a drop-in dependency. Retry only after current policy evaluation and bind resulting evidence to the actual attempt.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: COORD; cross-references: GOV, SEC, PROTO. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
