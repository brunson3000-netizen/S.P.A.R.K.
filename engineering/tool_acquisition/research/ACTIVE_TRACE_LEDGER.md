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

## Latest completed trace — provider/federation failure replay

Pattern: `patterns/SEMANTIC_PLUS_TRANSPORT_FAULT_REPLAY.md`
Experiment: `experiments/PROVIDER_FAILURE_REPLAY.md`

Confirmed:
- wiremock-rs supplies isolated HTTP mock servers, request matchers, static/dynamic responses, call-count expectations and received-request recording
- Toxiproxy inserts a real TCP proxy controlled over HTTP with dynamic upstream/downstream toxic chains
- semantic provider errors and transport faults should be exercised independently and jointly
- acceptance fixtures should use fixed toxics (`toxicity=1`, jitter 0, explicit timeout/reset/limit values); randomized chaos is a separate stress lane
- wiremock records requests by default, so real credentials must not enter qualification evidence
- Toxiproxy harness failure is infrastructure failure, not proof the application handled the intended fault.

Likely reuse after experiment:
- wiremock-rs → Rust dev dependency/provider fixture substrate
- Toxiproxy → isolated pinned fault-injection sidecar.

## Active trace

### T-RTK-01 — RTK vs context-compress
State: IN_PROGRESS
Pin: RTK `d0c2985155568d1d76fca03bc65d5098f136bbcd`
Comparison source: context-compress `59fae35a7b383876a34f84090f6da978e230795a`

Goal: determine what RTK contributes beyond searchable spillover, whether its command-specific compression preserves load-bearing evidence, and whether automatic command rewriting/hooking is suitable for SPARK.

Priority traces:
1. command interception/routing path and whether RTK executes commands itself or filters supplied output
2. command-specific parsers/compressors and fallback behavior
3. exit code/stderr/error preservation and maximum-output policies
4. token-reduction accounting and test evidence
5. hook/auto-rewrite mechanism and trust/authority implications
6. controlled comparison: raw output vs RTK vs searchable-spillover + raw artifact.

Standing requirement: compression cannot be canonical evidence custody. Any promoted pattern must retain a separate immutable raw artifact or reproducible source operation.

## Next durable update

Complete RTK comparison, then qualify containment/security candidates Goose, Wasmtime and Extism.
