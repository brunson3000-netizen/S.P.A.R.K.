# ctx-zip — MCI source screen

Status: NARROW CODE SCREEN COMPLETE; ADVERSE REPRODUCTION EXECUTED. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [karthikscale3/ctx-zip](https://github.com/karthikscale3/ctx-zip/tree/76580f7ba1555c891928702743ac74412c7fac60). Exact pin: `76580f7ba1555c891928702743ac74412c7fac60`. Runtime: TypeScript compactor and file adapters. License evidence: MIT; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Agents need less prompt context with later result retrieval; message arrays and storage adapters produce compact references.

Compactor writes artifacts and mutates nested messages. Caller-provided names and adapter behavior determine storage identity; compact views must not become canonical evidence.

## Mechanism and failure semantics

writeToolResultsToFileStrategy shallow-copies the outer array but replaces nested output objects. Same-name calls use the same toolName.json key and LocalFileAdapter overwrites it. Supplied serializer is bypassed by direct JSON.stringify. Reader tools can become source-path references without preserving returned bytes.

## Evidence and tests

Exact-pin local reproduction using actual strategy and LocalFileAdapter confirmed shared references, earlier-result overwrite, caller mutation and unused serializer. Upstream boundary tests use an append-only memory adapter, not overwrite semantics; two tests also pass all while asserting different windows. Upstream suite not run.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [src/tool-results-compactor/compact.ts](https://github.com/karthikscale3/ctx-zip/blob/76580f7ba1555c891928702743ac74412c7fac60/src/tool-results-compactor/compact.ts)
- [src/tool-results-compactor/strategies/index.ts](https://github.com/karthikscale3/ctx-zip/blob/76580f7ba1555c891928702743ac74412c7fac60/src/tool-results-compactor/strategies/index.ts)
- [src/sandbox-code-generator/file-adapter.ts](https://github.com/karthikscale3/ctx-zip/blob/76580f7ba1555c891928702743ac74412c7fac60/src/sandbox-code-generator/file-adapter.ts)
- [tests/boundary.spec.ts](https://github.com/karthikscale3/ctx-zip/blob/76580f7ba1555c891928702743ac74412c7fac60/tests/boundary.spec.ts)
- [LICENSE.md](https://github.com/karthikscale3/ctx-zip/blob/76580f7ba1555c891928702743ac74412c7fac60/LICENSE.md)

## MCI transfer

Retain the compact-reference idea only. Rule out direct use for canonical evidence at this pin; use immutable per-call digest-addressed artifacts, pure projections and byte-exact retrieval. See the committed experiment and failure record.

Finding classes: IDEA; TEST/INVARIANT; negative implementation evidence. Filing home: CONTEXT; cross-references: TOOLS, AUDIT, TEST. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_IDEA
