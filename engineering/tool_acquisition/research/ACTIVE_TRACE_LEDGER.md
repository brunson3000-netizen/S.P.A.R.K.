# Active Trace Ledger

Status: ACTIVE
Updated: 2026-09-13

Repository records, not conversation history, carry research state. Research method: `../RESEARCH_PROTOCOL.md`.

## Completed first traces

| ID | Source | Pin | Record | Primary disposition |
|---|---|---|---|---|
| T-ARD-01 | Agentic Resource Discovery | `b76f235a8f461876ad4f1e77abd0eb0eb302b48d` | `sources/ARD.md` | BORROW_PATTERN |
| T-MCPG-01 | progressive MCP Guardian | `4c6a04537b9bc548744b4146168b8fa9896069cb` | `sources/PROGRESSIVE_MCP_GUARDIAN.md` | EXPERIMENT_NOW |
| T-MCPREG-01 | MCP Gateway & Registry | `7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73` | `sources/MCP_GATEWAY_REGISTRY.md` | BORROW_PATTERN |
| T-CTX-01 | context-compress | `59fae35a7b383876a34f84090f6da978e230795a` | `sources/CONTEXT_COMPRESS.md` | BORROW_PATTERN |
| T-SKILL-01 | Agent Skills | `69ef37e9424c0a7ea9dd2293b559e43ec8176379` | `sources/AGENT_SKILLS.md` | BORROW_PATTERN |
| T-GROKBUILD-01 | xAI grok-build | `37949780c144e37df692e3d669051a21fec24f20` | `sources/GROK_BUILD.md` | BORROW_PATTERN |
| T-TOOLHIVE-01 | ToolHive | `e532cf07d45fa99f3e4e63819396a3e9c9fd763f` | `sources/TOOLHIVE.md` | BORROW_PATTERN |
| T-RMCP-01 | official Rust MCP SDK | `3075dc9152d4678775f20634fcb467a7b995dbab` | `sources/RMCP.md` | REUSE_CODE |
| T-SLICE-01 | ast-grep + Tree-sitter | `45b5eb67...` / `1b8407d1...` | `sources/AST_GREP_TREE_SITTER.md` | EXPERIMENT_NOW |
| T-TESTUTIL-01 | cargo-nextest + cargo-mutants | `8527c325...` / `fe82f183...` | `sources/CARGO_NEXTEST_MUTANTS.md` | EXPERIMENT_NOW |
| T-FAULT-01 | wiremock-rs + Toxiproxy | `6b193047...` / `40f7fd31...` | `sources/WIREMOCK_TOXIPROXY.md` | EXPERIMENT_NOW |
| T-RTK-01 | RTK + context-compress comparison | `d0c29851...` / `59fae35a...` | `sources/RTK.md` | EXPERIMENT_NOW |

## Latest completed trace — output reduction

Comparison: `comparison/OUTPUT_REDUCTION_MATRIX.md`
Experiment: `experiments/OUTPUT_REDUCTION_BENCHMARK.md`
Failures: `failures/RTK.md`

Confirmed:
- RTK is an executing CLI proxy, not merely a formatter; command modules run the underlying command and preserve its exit code
- hook/plugin integration can automatically rewrite agent shell commands, but this execution-affecting middleware should not become SPARK canonical authority
- large deterministic command-specific filter library is the strongest reusable asset
- current recall layer stores byte-faithful compressed raw bytes for sufficiently large failures and filter-declared successful truncations
- default recall bounds are 10 MiB/entry, 200 entries, 30 days; ordinary successful compressed output is not universally captured
- recall uses a 12-hex-character SHA-256 prefix as local primary/display ID, unsuitable for canonical evidence identity
- filter/recovery integration depends on command modules calling shared recovery helpers
- context-compress contributes generic source-scoped search but not byte-exact evidence
- strongest combined shape is `authorized operation → immutable raw full-digest artifact → RTK-like reducer → searchable derived index → compact view`.

## Active trace

### T-CONTAIN-01 — Goose + Wasmtime + Extism
State: IN_PROGRESS
Pins:
- Goose `50666ae0b9a51e260b52b7efbab2e4e020346e94`
- Wasmtime `817c58787f432bcdbbb87679011f72c5bc80dbda`
- Extism `d5da29759bba88645f886d9e12d3f4e4376df7b3`

Goal: determine the strongest reusable containment/security mechanisms for running external or generated capability code beneath SPARK’s host-owned authority.

Priority traces:
1. Goose: tool-call security inspectors, permission/repetition/egress/adversary handling; which checks are deterministic versus model-assisted
2. Wasmtime/WASI: filesystem/network capability grants, preopens, resource limits, fuel/epoch interruption, memory/table/instance bounds and host-function boundary
3. Extism: manifest/path/host grants, WASI enablement, filesystem permission model, init/call fuel/resource limits and host functions
4. compare raw Wasmtime versus Extism as a plugin runtime for narrow deterministic adapters
5. extract explicit failure lessons: sandbox substrate is not authority; host functions widen capability; resource limits are not all equivalent to wall-clock containment; filesystem capability semantics need adversarial tests.

Standing doctrine: containment can restrict what admitted code can do, but only SPARK’s host broker can decide whether the code should run and with which grants.

## Next durable update

Close Goose first, then Wasmtime and Extism with a containment comparison matrix and a bounded plugin-boundary experiment. After containment, continue remaining harvested middle-layer/observability candidates.

## Recovery checkpoint — 2026-09-14

Resumed through the GitHub connector from commit `795be8f2887911e91678e5a8760a991401e647fe` on `phase1-refoundation-v2`. Operator reiterated repository logging as work proceeds. The earlier exported handoff is superseded for current state: harvesting and multiple source traces are already recorded in this repository. Its obsolete harvest script finding is not a finding against the current repository scripts.

`GOOSE_SECURITY.md` and `WASMTIME_WASI.md` both record FIRST TRACE COMPLETE. The active-trace prose above lags those source records. Exact next unfinished work: Extism pinned-source trace, containment comparison, and bounded plugin-boundary experiment specification. No new implementation or runtime adoption follows from resuming this research. Runtime experiments remain unexecuted in this session.
