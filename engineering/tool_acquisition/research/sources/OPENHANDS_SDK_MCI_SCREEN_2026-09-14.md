# openhands-sdk — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [OpenHands/software-agent-sdk](https://github.com/OpenHands/software-agent-sdk/tree/c37007429be8b4465a83487dc1fd0914df0ea734). Exact pin: `c37007429be8b4465a83487dc1fd0914df0ea734`. Runtime: Python agent SDK / optional agent server. License evidence: MIT; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Agent applications need confirmation policies; actions and risk annotations produce confirm/no-confirm decisions.

The selected policy gates workflow interaction. Risk classification and caller-selected modes are configuration inputs; they cannot independently authenticate an operator or supply OS confinement.

## Mechanism and failure semantics

AlwaysConfirm requires confirmation; NeverConfirm does not. ConfirmRisky compares a risk threshold and treats missing risk as requiring confirmation by default. These are deterministic policy outcomes over supplied/classified risk, not proof that classification was correct.

## Evidence and tests

Confirmation-policy tests inspected; no full agent/server tool dispatch or live isolation suite executed.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [openhands-sdk/openhands/sdk/security/confirmation_policy.py](https://github.com/OpenHands/software-agent-sdk/blob/c37007429be8b4465a83487dc1fd0914df0ea734/openhands-sdk/openhands/sdk/security/confirmation_policy.py)
- [tests/sdk/security/test_confirmation_policy.py](https://github.com/OpenHands/software-agent-sdk/blob/c37007429be8b4465a83487dc1fd0914df0ea734/tests/sdk/security/test_confirmation_policy.py)
- [LICENSE](https://github.com/OpenHands/software-agent-sdk/blob/c37007429be8b4465a83487dc1fd0914df0ea734/LICENSE)

## MCI transfer

Borrow the explicit confirmation-policy interface and conservative unknown-risk handling. Keep current permission checks independent of an LLM risk score and of UI confirmation mode. Python agent framework can remain an adapter/reference rather than owning MCI authority.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: COORD; cross-references: GOV, SEC, TOOLS. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
