# ctx-zip evidence overwrite

Status: REPRODUCED LOCALLY AT EXACT SOURCE BYTES. This is an adverse result, not a qualification pass.

Upstream: Langtrace ctx-zip at `76580f7ba1555c891928702743ac74412c7fac60`. See [source screen](../sources/CTX_ZIP_MCI_SCREEN_2026-09-14.md) for exact upstream paths and license evidence.

## Observed failure

Two calls named lookup, with different call IDs and FIRST/SECOND payloads, generate the same lookup.json reference. The actual LocalFileAdapter overwrites the first payload with the second. Both compacted references consequently retrieve the second result. The strategy also mutates the caller's nested message objects, and the supplied serializer is never called.

Cause: filename identity uses tool name rather than call ID/content digest; the strategy copies only the outer message array and invokes JSON.stringify directly. The adapter performs ordinary overwriting file writes. The upstream boundary fixture records writes in an append-only array, which does not model the retrieval semantics of a filesystem adapter. No fix was applied upstream.

Separate static limitation: named file-reader results can be replaced with references to current source files without archiving the returned bytes. A later read need not reproduce the earlier evidence. This separate case was not runtime-tested here.

## Reproduction evidence

- [Runnable harness](../experiments/ctx_zip_repro.mjs)
- [Observed result and source hashes](../experiments/CTX_ZIP_REPRO_RESULT_2026-09-14.json)
- Runtime: Node v24.19.0 native TypeScript stripping. No models, network calls, package installation or changes outside disposable scratch files.
- Both upstream files were staged byte-for-byte as .mts copies to permit native module loading without installing the upstream package. Git blob hashes are asserted before importing. The final rerun exited 0 because all four adverse-behavior assertions held.

To repeat, make exact byte copies of pinned `src/tool-results-compactor/strategies/index.ts` as `ctx-zip-strategies.mts` and `src/sandbox-code-generator/file-adapter.ts` as `ctx-zip-file-adapter.mts`, then run:

```sh
node ctx_zip_repro.mjs ctx-zip-strategies.mts ctx-zip-file-adapter.mts
```

Use Node 24.19.0 for the recorded environment. No production evidence files should be passed to the harness. It creates and removes its own temporary storage directory. This is a focused reproduction, not execution of the upstream full suite.

## MCI consequence

Canonical evidence must have immutable per-call identity, content integrity and retrievable original bytes. Compression/search are pure derived views. Require repeated-tool, concurrent-write, serializer, input-immutability and source-file-change tests for any future compactor.

Transfer confidence: HIGH for the four reproduced behaviors at this pin; static-only for the reader-reference limitation. Primary disposition: BORROW_IDEA. The implementation is not selected for direct canonical-evidence reuse.
