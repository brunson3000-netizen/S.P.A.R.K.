# autogen — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [microsoft/autogen](https://github.com/microsoft/autogen/tree/027ecf0a379bcc1d09956d46d12d44a3ad9cee14). Exact pin: `027ecf0a379bcc1d09956d46d12d44a3ad9cee14`. Runtime: Python multi-agent framework. License evidence: MIT code; CC BY 4.0 other material; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Agent teams need stop conditions; messages, counters and elapsed time yield termination signals.

Team termination conditions operate within the agent loop. They do not by themselves revoke tool capabilities or terminate a process tree.

## Mechanism and failure semantics

MaxMessageTermination counts messages; TimeoutTermination observes monotonic elapsed time when evaluated; external termination is a condition mechanism. README marks maintenance mode and points new work to Microsoft Agent Framework.

## Evidence and tests

Selected termination classes screened; no team runtime or process-stop test executed. Code license is MIT in LICENSE-CODE; root LICENSE is CC BY 4.0 for non-code material, so one root-file label would be misleading.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [README.md](https://github.com/microsoft/autogen/blob/027ecf0a379bcc1d09956d46d12d44a3ad9cee14/README.md)
- [python/packages/autogen-agentchat/src/autogen_agentchat/conditions/_terminations.py](https://github.com/microsoft/autogen/blob/027ecf0a379bcc1d09956d46d12d44a3ad9cee14/python/packages/autogen-agentchat/src/autogen_agentchat/conditions/_terminations.py)
- [LICENSE-CODE](https://github.com/microsoft/autogen/blob/027ecf0a379bcc1d09956d46d12d44a3ad9cee14/LICENSE-CODE)
- [LICENSE](https://github.com/microsoft/autogen/blob/027ecf0a379bcc1d09956d46d12d44a3ad9cee14/LICENSE)

## MCI transfer

Retain historical coordination and termination vocabulary. Maintenance status and duplicated framework scope make this a weaker new foundation. Use host-owned deadlines and independently confirmed stop state instead of equating loop termination with confinement.

Finding classes: reference/qualification assessment. Filing home: COORD; cross-references: GOV, TEST. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

ARCHIVE_REFERENCE
