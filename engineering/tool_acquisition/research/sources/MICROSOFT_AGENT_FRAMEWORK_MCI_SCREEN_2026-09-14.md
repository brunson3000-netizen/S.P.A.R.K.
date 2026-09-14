# microsoft-agent-framework — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [microsoft/agent-framework](https://github.com/microsoft/agent-framework/tree/1cd06c5a2058a172eebadf5d9d7c3fa45c519520). Exact pin: `1cd06c5a2058a172eebadf5d9d7c3fa45c519520`. Runtime: Python/.NET agent framework; Python harness studied. License evidence: MIT; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Applications need queued and standing tool approvals; function calls, approval responses and session state produce resumed calls or more prompts.

Middleware stores approval rules/queues in AgentSession. Tool name, optional hosted-server label and argument matching identify standing approvals; session data is not independent operator authentication.

## Mechanism and failure semantics

_matches_rule checks tool name and server label, then accepts tool-wide rules or exact serialized argument matches. Middleware persists queues and collected responses through AgentSession. Source warns auto-approval callbacks can accidentally approve unrelated same-name local tools.

## Evidence and tests

Inspected tests cover forged standing approvals, server boundaries, argument-scoped rules and changed hidden snapshots. test_approval_resume_allows_same_name_tool_upgrade explicitly expects the replacement same-name implementation to run (old calls 0, new calls 1). Tests not executed.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [python/packages/core/agent_framework/_harness/_tool_approval.py](https://github.com/microsoft/agent-framework/blob/1cd06c5a2058a172eebadf5d9d7c3fa45c519520/python/packages/core/agent_framework/_harness/_tool_approval.py)
- [python/packages/core/tests/core/test_harness_tool_approval.py](https://github.com/microsoft/agent-framework/blob/1cd06c5a2058a172eebadf5d9d7c3fa45c519520/python/packages/core/tests/core/test_harness_tool_approval.py)
- [LICENSE](https://github.com/microsoft/agent-framework/blob/1cd06c5a2058a172eebadf5d9d7c3fa45c519520/LICENSE)

## MCI transfer

Borrow approval queue/result and server+argument binding patterns. MCI should additionally bind capability version/digest, active policy epoch and exact resource; invalidate pending approvals on those changes. Do not adopt same-name replacement semantics for high-impact grants.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: COORD; cross-references: GOV, PROTO, TEST. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
