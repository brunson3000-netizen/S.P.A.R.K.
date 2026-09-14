# cli-anything — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [HKUDS/CLI-Anything](https://github.com/HKUDS/CLI-Anything/tree/810c18b0d1ab9b234bc996c9fd999318523a3ef0). Exact pin: `810c18b0d1ab9b234bc996c9fd999318523a3ef0`. Runtime: Python application harness. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Application operations need compact, structured commands; agent users supply scene commands and paths and receive JSON scene state or generated scripts.

Authority is the harness process and its filesystem access. Scene dictionaries and undo stacks are local editing state; trust in generated adapters does not establish application correctness.

## Mechanism and failure semantics

Session.snapshot deep-copies before edits; undo/redo restore snapshots. save_session updates metadata then calls _locked_save_json, which opens an existing file r+, truncates, dumps and flushes. File-lock import/acquisition errors are ignored. No atomic rename or fsync appears in that helper, despite its atomic-write docstring.

## Evidence and tests

The inspected test module explicitly does not require actual Blender. JSON scene and script assertions are useful adapter tests, but do not prove real Blender execution. Tests were read, not run.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [blender/agent-harness/cli_anything/blender/core/session.py](https://github.com/HKUDS/CLI-Anything/blob/810c18b0d1ab9b234bc996c9fd999318523a3ef0/blender/agent-harness/cli_anything/blender/core/session.py)
- [blender/agent-harness/cli_anything/blender/tests/test_full_e2e.py](https://github.com/HKUDS/CLI-Anything/blob/810c18b0d1ab9b234bc996c9fd999318523a3ef0/blender/agent-harness/cli_anything/blender/tests/test_full_e2e.py)
- [LICENSE](https://github.com/HKUDS/CLI-Anything/blob/810c18b0d1ab9b234bc996c9fd999318523a3ef0/LICENSE)

## MCI transfer

Use bounded JSON command/result contracts and reversible editor-state operations in TOOLS. Build the planned Blender shadow adapter separately; require real application fixtures and crash-safe evidence writes. Native Python/runtime/application dependencies are external to Rust Core. Do not import the save helper as canonical persistence.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: TOOLS; cross-references: PROTO, TEST. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
