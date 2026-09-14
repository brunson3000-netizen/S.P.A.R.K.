# google-adk-python — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [google/adk-python](https://github.com/google/adk-python/tree/460715b6c62c8e9ab00931c502381ee0364e39b6). Exact pin: `460715b6c62c8e9ab00931c502381ee0364e39b6`. Runtime: Python agent SDK. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Applications need safe confirmation resumption; session call history and confirmation responses resolve the operation to execute.

Registered tools and current agent-authored call records are checked. History content and a user author label alone are not proof of authenticated human identity.

## Mechanism and failure semantics

_resolve_confirmation_targets resolves original function-call IDs from history, checks current-agent provenance, registered tool and confirmation requirement, and rejects mismatched name/arguments. Consumed confirmation handling prevents routine repeated processing of already-resolved confirmations.

## Evidence and tests

Inspected rejection fixture, successful/declined responses and consumed-confirmation tests. No framework runtime tests or external tools executed.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [src/google/adk/flows/llm_flows/request_confirmation.py](https://github.com/google/adk-python/blob/460715b6c62c8e9ab00931c502381ee0364e39b6/src/google/adk/flows/llm_flows/request_confirmation.py)
- [tests/unittests/flows/llm_flows/test_request_confirmation.py](https://github.com/google/adk-python/blob/460715b6c62c8e9ab00931c502381ee0364e39b6/tests/unittests/flows/llm_flows/test_request_confirmation.py)
- [LICENSE](https://github.com/google/adk-python/blob/460715b6c62c8e9ab00931c502381ee0364e39b6/LICENSE)

## MCI transfer

Borrow exact call/name/argument matching and consumed-confirmation invariants. Add authenticated submitter, immutable tool version, policy epoch and resource revision to the MCI host envelope; current history matching does not establish those stronger bindings.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: COORD; cross-references: GOV, TOOLS, AUDIT. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
