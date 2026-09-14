# openai-agents-python — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [openai/openai-agents-python](https://github.com/openai/openai-agents-python/tree/fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292). Exact pin: `fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292`. Runtime: Python agent SDK. License evidence: MIT; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Agent applications need input guardrails and tool approval objects; agent input, guardrail results and approval items control workflow progression.

Guardrail classification and approval item plumbing are not universal pre-effect authorization. Framework/model clients remain separate from MCI host policy.

## Mechanism and failure semantics

InputGuardrail defaults run_in_parallel to true; a blocking pre-agent mode is configurable. Approval helper filters ToolApprovalItem objects and distinguishes interruptions/errors. These narrow helpers do not establish the complete execution gate.

## Evidence and tests

Source screen only; no end-to-end guardrail/tool execution or tripwire race test run.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [src/agents/guardrail.py](https://github.com/openai/openai-agents-python/blob/fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292/src/agents/guardrail.py)
- [src/agents/run_internal/approvals.py](https://github.com/openai/openai-agents-python/blob/fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292/src/agents/run_internal/approvals.py)
- [LICENSE](https://github.com/openai/openai-agents-python/blob/fbd2dbcaaf74a2c447c6d3fa9d5645d83fd7e292/LICENSE)

## MCI transfer

Borrow the separation of guardrail feedback from tool approval UX. For hard governance admission, use deterministic host checks before dispatch, independently of a parallel classifier. Full framework adoption would add Python/model SDK dependencies without resolving canonical authority.

Finding classes: IDEA. Filing home: COORD; cross-references: GOV, TOOLS, AUDIT. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_IDEA
