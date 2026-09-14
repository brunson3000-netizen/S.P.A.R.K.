# mcp-inspector — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [modelcontextprotocol/inspector](https://github.com/modelcontextprotocol/inspector/tree/795b1bb30ac845b7baa7cb3df8ec0b693882ca1d). Exact pin: `795b1bb30ac845b7baa7cb3df8ec0b693882ca1d`. Runtime: TypeScript/Node diagnostic clients. License evidence: MIT declared in package.json; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

MCP developers need reproducible protocol probes; method arguments and connection settings produce tool results and diagnostics.

The client can invoke real remote tools and, for some display options, perform additional resource requests. Connection credentials and server replies are trusted inputs; diagnostic state has no MCI authority.

## Mechanism and failure semantics

Tool-method handling resolves a tool and invokes callTool or callToolStream. Missing results are turned into an isError result. JSON/app-info display can trigger a resources/read probe, so one visible CLI invocation need not equal one RPC.

## Evidence and tests

Handler source screened; full inspector suite and live server behavior not run. No claim that a diagnostic success validates authorization or sandbox safety.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [clients/cli/src/handlers/run-method.ts](https://github.com/modelcontextprotocol/inspector/blob/795b1bb30ac845b7baa7cb3df8ec0b693882ca1d/clients/cli/src/handlers/run-method.ts)
- [package.json](https://github.com/modelcontextprotocol/inspector/blob/795b1bb30ac845b7baa7cb3df8ec0b693882ca1d/package.json)

## MCI transfer

Retain as a qualification tool against disposable mock servers. Record every RPC, including automatic probes, and separate protocol conformance from host policy acceptance. Node and UI dependencies stay in the test lane; no production dependency selected.

Finding classes: reference/qualification assessment. Filing home: TEST; cross-references: PROTO, SEC. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

EXPERIMENT_NOW
