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

## Load-bearing findings from latest completed traces

### rmcp
- use official Rust SDK for MCP wire models, lifecycle/version negotiation, typed tool routing, cancellation/progress, transports and conformance
- keep canonical identity, source trust, current grants, cost/governance, containment, durable work/evidence and health outside MCP
- task/session/request handles are correlation references, never grants
- bundled TaskManager is process-local, not crash-durable
- stale-on-error client cache must not satisfy current authority/trust inputs
- cross-process session adapters must preserve originating-request association explicitly.

Patterns: `PROTOCOL_SUBSTRATE_HOST_AUTHORITY`, `CONTINUATION_HANDLES_NOT_GRANTS`.
Failures: `failures/RMCP.md`.

### ast-grep + Tree-sitter
- ast-grep JSON returns exact match/capture byte + line/column ranges suitable for compact evidence envelopes
- Tree-sitter exposes parser ABI compatibility and error/missing-node recovery state
- syntax-bounded slices must carry parse-health/error-overlap confidence
- syntax matches are not semantic-reference proof
- read-only search and rewrite require separate authorities
- ast-grep 0.45.3 depends on Tree-sitter 0.27.0; independently harvested Tree-sitter 0.28.0 must not be force-substituted without compatibility testing.

Pattern: `patterns/SYNTAX_BOUNDED_CONTEXT_SLICING.md`.
Experiment: `experiments/CONTEXT_SLICING_BENCHMARK.md`.
Failures: `failures/AST_GREP_TREE_SITTER.md`.

## Active trace

### T-TESTUTIL-01 — cargo-nextest + cargo-mutants
State: IN_PROGRESS
Pins:
- cargo-nextest `8527c325bf9f0e5dcdcb26fa80e3ed7dc7f3e35e`
- cargo-mutants `fe82f1832778a591ab74248010fb40e699defafe`

Goal: determine whether SPARK should standardize a deterministic Rust validation lane combining fast isolated test execution with bounded mutation/falsification.

Priority traces:
1. nextest test enumeration, process isolation, retries/slow timeouts/partitioning and machine result formats
2. how nextest distinguishes test failure from runner/infrastructure failure
3. cargo-mutants mutation generation, baseline/build/test command path, timeout and shard/package selection
4. whether mutation results are reproducible/bounded enough for automated agent falsification
5. integration opportunities with existing repository test commands without replacing canonical project test policy.

## Cross-project hypotheses under test

H1 tiny doorway; H2 small context/full evidence; H3 discovery advisory; H4 skills teach/tools act; H5 deterministic middleware; H6 stable identity; H7 signed/bounded cross-process assertions; H8 indexes derived; H9 instructional provenance; H10 repository trust gate; H11 durable work/reconciliation; H12 bounded subagents; H13 declared-vs-applied confinement; H14 explicit provenance before activation; H15 protocol substrate below authority; H16 handles not grants; H17 syntax-bounded evidence can reduce context while preserving source attribution.

## Current convergence

`trust source → find → learn → host reauthorize → execute → compact result`

supported by:
- canonical raw evidence/durable work state
- derived searchable/syntax indexes
- bounded agent-visible views
- separately trusted teaching packages
- versioned RunSpec + applied-state verification
- protocol adapters below canonical identity/authority.

## Next durable update

Complete cargo-nextest/cargo-mutants trace, then qualify wiremock-rs + Toxiproxy, RTK against context-compress, and containment candidates Goose/Wasmtime/Extism.
