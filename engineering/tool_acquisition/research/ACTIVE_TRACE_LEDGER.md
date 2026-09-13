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

## Latest completed trace — Rust validation/falsification

Pattern: `patterns/FAST_TEST_PLUS_BOUNDED_FALSIFICATION.md`
Failures: `failures/CARGO_NEXTEST_MUTANTS.md`
Experiment: `experiments/RUST_VALIDATION_FALSIFICATION_LANE.md`

Confirmed:
- nextest builds/lists tests, then executes each individual test in a separate process with structured per-attempt status
- partitioning/archiving/JUnit/record-replay-rerun are useful deterministic evidence surfaces
- retry defaults can turn fail→pass into overall success; assurance profiles should retain first failure and normally fail flaky results
- timeout/process-tree handling is explicit on Unix/Windows
- nextest does not run doctests and cannot silently replace repository canonical test policy
- cargo-mutants parses production Rust, generates likely-valid wrong AST mutations, copies source tree, proves baseline, then tests mutants in per-worker scratch build dirs
- outcomes distinguish caught/missed/unviable/timeout; machine evidence includes mutants/outcomes/diffs/logs
- baseline identity is load-bearing; mutation evidence without a passing current/equivalent baseline is meaningless
- cargo-mutants supports nextest natively, but inherits nextest omissions
- source-copy isolation protects canonical files but does not sandbox test side effects
- mutation work must be explicitly bounded by files/packages/mutants/time/concurrency/shards.

Likely operational outcome after experiment:
- nextest → reusable compatible-Rust executor
- cargo-mutants → periodic/on-demand bounded falsification, not every-edit gate.

## Active trace

### T-FAULT-01 — wiremock-rs + Toxiproxy
State: IN_PROGRESS
Pins:
- wiremock-rs `6b193047bf2c5626da5dc5f3a23b58ab9bd3f130`
- Toxiproxy `40f7fd31bee529d824116bd2a11a9e3425e904ec`

Goal: qualify a deterministic provider/federation failure-replay pair that separates application/HTTP semantics from transport/network faults.

Priority traces:
1. wiremock request matching, response templating, verification/counting, server lifecycle and recorded requests
2. deterministic provider cassette/replay fit and limits versus hand-written fake services
3. Toxiproxy proxy/toxic lifecycle, latency/bandwidth/timeout/reset/slow-close/limit-data/failure modes
4. whether faults can be enabled/disabled and composed reproducibly at runtime
5. experiment contract pairing semantic response fixtures with transport fault schedules and immutable evidence.

## Next durable update

Complete wiremock-rs/Toxiproxy trace, then compare RTK against context-compress, followed by containment candidates Goose/Wasmtime/Extism.
