# gemini-cli — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [google-gemini/gemini-cli](https://github.com/google-gemini/gemini-cli/tree/9c1b0a610534d6f8120964cf2672c07807d8fc90). Exact pin: `9c1b0a610534d6f8120964cf2672c07807d8fc90`. Runtime: TypeScript agent CLI. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Agent tool calls need configurable policy and shell analysis; prioritized rules and call arguments yield allow/ask/deny.

Policy configuration and checkers drive local admission. Host execution and current workspace trust remain separate dependencies.

## Mechanism and failure semantics

Policy engine sorts rules by priority, evaluates matching calls and performs shell subcommand checks. Default behavior is DENY for noninteractive and ASK otherwise unless configured. Checker denials and errors are handled in the policy path; exact rule precedence must be compared to the consumer's policy rather than assumed identical.

## Evidence and tests

Regression test bodies inspect chained commands (&&, ||, ;, &, pipes), nested substitution and redirection. These tests assert unauthorized components are not admitted under the fixture rules; not executed.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [packages/core/src/policy/policy-engine.ts](https://github.com/google-gemini/gemini-cli/blob/9c1b0a610534d6f8120964cf2672c07807d8fc90/packages/core/src/policy/policy-engine.ts)
- [packages/core/src/policy/shell-safety-regression.test.ts](https://github.com/google-gemini/gemini-cli/blob/9c1b0a610534d6f8120964cf2672c07807d8fc90/packages/core/src/policy/shell-safety-regression.test.ts)
- [LICENSE](https://github.com/google-gemini/gemini-cli/blob/9c1b0a610534d6f8120964cf2672c07807d8fc90/LICENSE)

## MCI transfer

Borrow policy explanation and compound-command test corpus in GOV/TOOLS. Never authorize a shell string from its first token alone. Rust integration would require a compatible parser and explicit policy contract; no TypeScript policy engine adoption selected.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: COORD; cross-references: GOV, SEC, TOOLS. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
