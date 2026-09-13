# External Tool Acquisition Mission Plan V0

## Objective

Turn scattered cross-project external-tool research into one S.P.A.R.K.-owned evidence base and acquire the highest-value upstream source/reference surfaces immediately, without introducing production dependencies or assuming authority over a consuming project.

## Phase H0 — Consolidate now

Complete when:
- all recovered NIM/S.W.A.R.M./G.A.M.E./S.P.A.R.K. external-tool findings are represented in the research archive;
- duplicate findings are normalized instead of copied as competing conclusions;
- provenance names the originating artifact/project;
- conclusions that depended on S.W.A.R.M. authority are rewritten as consumer-boundary constraints rather than S.P.A.R.K. decisions.

H0 is consolidated in this online repository record.

## Phase H1 — High-priority source harvest complete

The high-priority upstreams are pinned as quarantined git submodules under `external/harvest/high_priority/`. `UPSTREAM_LOCK.json` is the expected-source ledger. Local synchronization and verification leave S.P.A.R.K.'s dependency manifests/runtime untouched.

Priority group A — tool fabric and containment:
1. CLI-Anything
2. official MCP Rust SDK (`rmcp`)
3. ToolHive
4. Goose
5. MCP Inspector
6. Wasmtime
7. Extism

Priority group B — deterministic engineering utilities:
8. ast-grep
9. tree-sitter
10. Toxiproxy
11. wiremock-rs
12. cargo-nextest
13. cargo-mutants

## Phase H2 — Qualification packets — NEXT

For each high-priority candidate, produce one short packet containing:
- exact problem/capability;
- candidate surface actually needed;
- build/runtime/dependency cost;
- license/reuse status;
- attack/failure surface;
- deterministic qualification test;
- token/context reduction opportunity;
- integration shape: executable, library, sidecar, adapter, or reference only;
- replacement/removal path.

Do not perform implementation adoption in H2.

## Phase H3 — Capability map

Normalize candidates against capability IDs, not product names. Initial IDs:
- `tool.protocol.mcp`
- `tool.discovery.registry`
- `tool.teaching.skill`
- `tool.runtime.isolated`
- `tool.security.inspect`
- `repo.syntax.query`
- `repo.symbol.slice`
- `test.execute.fast`
- `test.falsify.mutation`
- `provider.http.mock`
- `provider.network.fault`
- `artifact.preview.bundle`
- `evidence.trace.export`
- `external_app.adapter`

A product may serve multiple capabilities; a capability may have multiple providers.

## Phase H4 — Mission-plan selection

Only after H1-H3, select bounded implementation/experiment missions. Selection criteria:
- frequency and engineering value;
- removal of silent-wrongness;
- degree of deterministic offload;
- fit with Rust-first implementation;
- reversibility;
- security boundary clarity;
- existing project demand.

The first implementation should be the smallest experiment that proves a reusable contract, not the largest platform installation.

## Hard boundary

S.P.A.R.K. may prepare a tool for a future S.W.A.R.M. consumer. S.P.A.R.K. does not decide that S.W.A.R.M. adopts it, alter S.W.A.R.M. authority, or write to S.W.A.R.M. canonical state.
