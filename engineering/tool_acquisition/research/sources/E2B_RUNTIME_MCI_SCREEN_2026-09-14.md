# e2b-runtime — MCI source screen

Status: DOCUMENTATION SCREEN COMPLETE; T-SANDBOX-02 REMAINS IN PROGRESS. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [e2b-dev/runtime](https://github.com/e2b-dev/runtime/tree/cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497). Exact pin: `cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497`. Runtime: Go service stack / Linux Firecracker microVMs. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Sandbox operators need disposable machines with pause/resume; API requests and snapshot artifacts drive node-level VM lifecycle.

Architecture documents separate control-plane placement/state from node-level execution. Snapshot state is not a new MCI authority grant.

## Mechanism and failure semantics

Documented flow is control API to per-node orchestrator to Firecracker and in-guest envd; memory/disk/VM snapshots support resume. Documentation names Postgres/Redis routing and object storage, plus optional deployment services.

## Evidence and tests

Documentation and source-tree screen only in this pass. Existing T-SANDBOX-02 retains IN PROGRESS ownership and its required code/negative-test traces; no microVM started.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [README.md](https://github.com/e2b-dev/runtime/blob/cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497/README.md)
- [docs/ARCHITECTURE.md](https://github.com/e2b-dev/runtime/blob/cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497/docs/ARCHITECTURE.md)
- [LICENSE](https://github.com/e2b-dev/runtime/blob/cf6961c4b9c8fb919a5b372f0d8c4cd3216aa497/LICENSE)

## MCI transfer

Retain sandbox lifecycle prior art while deferring disposition on implementation reuse to the active trace. Linux/KVM and service-stack needs make direct local portability uncertain. Require fresh authority on resume, kill/cleanup receipts and evidence export before teardown.

Finding classes: reference/qualification assessment. Filing home: SEC; cross-references: GOV, COORD, AUDIT. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

DEFER
