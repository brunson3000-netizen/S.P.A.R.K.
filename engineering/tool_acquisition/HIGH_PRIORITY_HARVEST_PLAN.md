# High-Priority Harvest Plan

The immediate harvest is deliberately **source/reference acquisition rather than installation**. This avoids contaminating the S.P.A.R.K. runtime or Cargo graph while preserving exact upstream code and documentation for rapid qualification.

| # | Candidate | Pin | Why now | Immediate state |
|---:|---|---|---|---|
| 1 | `cli-anything` | `810c18b0d1ab` | agent-native application harness methodology; registry; skills; preview protocol | pinned source submodule harvested; qualification next |
| 2 | `rmcp` | `3075dc9152d4` | official Rust MCP transport/schema implementation | pinned source submodule harvested; qualification next |
| 3 | `toolhive` | `e532cf07d45f` | MCP runtime isolation; registry; gateway; permission profiles | pinned source submodule harvested; qualification next |
| 4 | `goose` | `50666ae0b9a5` | Rust agent/tool runtime and layered security inspectors | pinned source submodule harvested; qualification next |
| 5 | `mcp-inspector` | `795b1bb30ac8` | independent MCP qualification/inspection | pinned source submodule harvested; qualification next |
| 6 | `wasmtime` | `817c58787f43` | Rust/Wasm sandbox substrate and WASI capability boundary | pinned source submodule harvested; qualification next |
| 7 | `extism` | `d5da29759bba` | higher-level Wasm plugin boundary, path/fuel controls | pinned source submodule harvested; qualification next |
| 8 | `ast-grep` | `45b5eb6705b4` | structured syntax search/rewrite and context slicing | pinned source submodule harvested; qualification next |
| 9 | `tree-sitter` | `1b8407d1e718` | incremental parsing substrate | pinned source submodule harvested; qualification next |
| 10 | `toxiproxy` | `40f7fd31bee5` | network fault injection for provider/tool paths | pinned source submodule harvested; qualification next |
| 11 | `wiremock-rs` | `6b193047bf2c` | Rust HTTP mocking/provider cassette substrate | pinned source submodule harvested; qualification next |
| 12 | `cargo-nextest` | `8527c325bf9f` | fast structured Rust test execution | pinned source submodule harvested; qualification next |
| 13 | `cargo-mutants` | `fe82f1832778` | Rust mutation testing/falsification | pinned source submodule harvested; qualification next |

## Harvest acceptance

For every candidate:
1. pin the exact upstream Git commit as a quarantined submodule;
2. record repository URL and commit in `UPSTREAM_LOCK.json`;
3. keep the upstream source outside S.P.A.R.K. runtime/dependency manifests;
4. do not execute upstream installer scripts as part of harvest;
5. do not write to S.W.A.R.M. or infer consumer adoption;
6. verify the local checkout commit against the lock file after sync;
7. review upstream license/reuse terms before copying code into S.P.A.R.K. implementation;
8. mark activation `NONE` until a separate qualification/adoption decision.

## Priority interpretation

`HIGH_PRIORITY` means acquire evidence/source now. It does not mean integrate first. Qualification can demote, reject, or split a candidate into reusable subcomponents.
