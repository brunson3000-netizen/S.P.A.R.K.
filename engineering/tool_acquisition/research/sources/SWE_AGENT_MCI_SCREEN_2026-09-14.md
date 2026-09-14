# swe-agent — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [SWE-agent/SWE-agent](https://github.com/SWE-agent/SWE-agent/tree/3ea751c087f32b16e039a2233dd6eefecef325d5). Exact pin: `3ea751c087f32b16e039a2233dd6eefecef325d5`. Runtime: Python software engineering agent. License evidence: MIT; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Coding agents need a small action language; model output is parsed into executable action representations.

Parsing model text is an input-validation step; environment execution owns actual OS effects.

## Mechanism and failure semantics

Parsing module implements several action/function-call formats and parsing failures. README points to mini-swe-agent as the successor rather than treating this repository as the preferred new implementation.

## Evidence and tests

Selected parser screen; no benchmark or external environment run, and no successor source added to the locked inventory.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [README.md](https://github.com/SWE-agent/SWE-agent/blob/3ea751c087f32b16e039a2233dd6eefecef325d5/README.md)
- [sweagent/tools/parsing.py](https://github.com/SWE-agent/SWE-agent/blob/3ea751c087f32b16e039a2233dd6eefecef325d5/sweagent/tools/parsing.py)
- [LICENSE](https://github.com/SWE-agent/SWE-agent/blob/3ea751c087f32b16e039a2233dd6eefecef325d5/LICENSE)

## MCI transfer

Retain interface and evaluation lessons, but avoid importing the older harness as MCI coordination infrastructure. Schema parsing, environment confinement, permission checks and correctness verification remain separate.

Finding classes: reference/qualification assessment. Filing home: COORD; cross-references: SEC, TOOLS, TEST. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

ARCHIVE_REFERENCE
