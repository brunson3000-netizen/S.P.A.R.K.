# hermes-agent — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent/tree/ee4452991d17534aa561f31ee55596d082aa94e7). Exact pin: `ee4452991d17534aa561f31ee55596d082aa94e7`. Runtime: Python agent runtime. License evidence: MIT; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Agent users need command approval floors across execution modes; command/environment/configuration produce deny/confirm/allow decisions.

User deny configuration and execution environment classification influence the floor. Approval behavior is not itself process confinement.

## Mechanism and failure semantics

check_all_command_guards applies user-deny floors before yolo/mode-off bypasses. Container shortcuts still consult user-deny rules and distinguish host-mounted Docker. However, _match_user_deny_rule catches configuration exceptions and returns no match; unattended mode can be configured permissively.

## Evidence and tests

Focused floor and caller paths inspected; no live command, configuration-failure or container run executed.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [tools/approval_floors.py](https://github.com/NousResearch/hermes-agent/blob/ee4452991d17534aa561f31ee55596d082aa94e7/tools/approval_floors.py)
- [tools/approval.py](https://github.com/NousResearch/hermes-agent/blob/ee4452991d17534aa561f31ee55596d082aa94e7/tools/approval.py)
- [LICENSE](https://github.com/NousResearch/hermes-agent/blob/ee4452991d17534aa561f31ee55596d082aa94e7/LICENSE)

## MCI transfer

Borrow the ordering of non-bypassable floors before convenience modes, with stricter explicit error semantics. MCI must not silently interpret an unreadable denial policy as no denial. Python runtime and execution backends remain outside the Rust authoritative core.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: COORD; cross-references: GOV, SEC, CONTEXT. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
