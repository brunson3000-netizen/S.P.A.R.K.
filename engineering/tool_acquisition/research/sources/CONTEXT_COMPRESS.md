# Source Study — context-compress

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: context-compress
- Upstream: `https://github.com/Open330/context-compress`
- Commit: `59fae35a7b383876a34f84090f6da978e230795a`
- Retrieval/study date: 2026-09-13
- License: MIT
- Language/runtime: TypeScript / Node.js MCP server; SQLite FTS5 (`better-sqlite3`); multi-language subprocess execution
- Primary docs: `README.md`, `SECURITY.md`, benchmark/reduction reports

## Problem framing

PROBLEM: Large command/tool/document outputs consume agent context even when only a few relevant lines or sections matter.

USER: coding/engineering agents using MCP.

INPUT: executed code/commands, files, fetched documents, raw text/markdown and search questions/intents.

OUTPUT: bounded agent-facing summaries/snippets plus a searchable local FTS5 corpus.

AUTHORITY: `execute`, `execute_file` and batch execution intentionally run arbitrary code with the MCP server user’s privileges; indexing/search affect only the local knowledge store. The project trusts the LLM as an authorized operator.

TRUST: LLM is trusted relative to local execution. External/fetched/indexed content is untrusted and receives heuristic prompt-injection warnings. There is explicitly no OS-level isolation of the executor.

STATE: SQLite source/chunk/vocabulary index; ephemeral by default or persistent if user enables it; optional cumulative stats.

FAILURE: output/time/resource bounds kill commands or truncate responses; indexing/search may fall back or return empty; persistent database creation failure falls back to an in-memory store.

## Trace A — command execution → compact response → searchable spillover

`execute` MCP tool
→ `SubprocessExecutor.execute`
→ runtime-specific preprocess/build/execute
→ `spawnAndCapture`
→ capture stdout/stderr under hard byte cap and timeout
→ normalize ANSI; `indexableStdout` is created before presentation filters
→ presentation copy may receive command-specific filter, format filter, progress removal, deduplication, error grouping and smart truncation
→ if caller supplied `intent` and the indexable corpus exceeds threshold:
  - `createIntentFilter.applyIntentFilter(indexableOutput, intent, sourceLabel)`
  - `ContentStore.index`
  - source-scoped FTS search against only this call’s `sourceId`
  - render top-ranked snippets under intent byte budget
  - return compact summary + searchable terms + instruction to use `search`
→ status/footer assembled under response budget.

Important separation: the agent-visible response can be aggressively compressed while a less-compressed `indexableOutput` is separately indexed.

However, `indexableOutput` is not equivalent to immutable raw process bytes:
- ANSI is stripped before indexing
- capture stops at the hard cap; process is killed when exceeded
- stderr is appended to the indexed corpus only when exit code is nonzero
- no raw byte artifact is written by this path.

Default execution bounds from the pinned config:
- intent threshold: 5,000 bytes
- intent inline budget: 1,800 bytes
- normal max visible execution output: 102,400 bytes
- hard capture cap: 16 MiB
- max concurrent executions: 8
- timeout request cap: 600,000 ms.

Primary files:
- `src/tools/execute.ts`
- `src/executor.ts`
- `src/util/intent-filter.ts`
- `src/config.ts`

## Trace B — content/indexing → chunk store

`index(content|path)`
→ XOR validation: exactly one content source
→ project-bound path check for file input
→ 50 MiB input ceiling
→ `ContentStore.index(text, label)`
→ classify markdown/plain text
→ chunk content
→ hard-split any chunk above 8,192 characters
→ detect injection patterns once per source
→ insert source metadata
→ insert each chunk into FTS5 `chunks(title, content, source_id, content_type)`
→ optionally mirror into lazily created trigram FTS table
→ update bounded vocabulary
→ prune oldest sources if retention limit exceeded.

Storage is searchable uncompressed text chunks, but chunking is not byte-preserving:
- markdown chunker `.trim()`s chunks
- horizontal separators can be consumed as chunk boundaries
- plain-text section chunks are trimmed
- fixed-line plain-text chunks overlap by default (20 lines, overlap 2)
- large chunks are sliced by character count.

Therefore the indexed corpus is useful evidence/search material but is not a byte-identical archive of the source.

Primary files:
- `src/tools/index-content.ts`
- `src/store.ts`

## Trace C — search/retrieval

`search(queries, source?, limit?)`
→ per-server rate window + query count max 16
→ effective result limit clamped to configured search limit (default 3, then reduced under repeated calls)
→ `ContentStore.search`
→ FTS5 Porter search
→ lazy trigram fallback
→ bounded fuzzy correction fallback
→ BM25-ranked rows
→ `extractSnippet(highlighted)`
→ agent-facing result blocks assembled under whole-response byte budget.

Default whole search response budget: 40,960 bytes.

Critical finding: search does not expose a full-row/raw-content retrieval primitive. `extractSnippet` caps one hit at 1,500 characters by default and can return only windows around matches. Documentation strings such as “retrieve full content of any section” therefore overstate what the actual search path guarantees for sections larger than the snippet bound.

## Persistence and retention semantics

Defaults:
- `persistDb = false`
- ephemeral store is placed in a private random temp directory and deleted on close
- `maxIndexedSources = 500`
- oldest sources are pruned after bounded overshoot
- `0` disables pruning, but retention configuration is user-scope-only because persistent indexed output can contain sensitive material.

Persistent mode stores `store.db` under a user-selected directory or project `.context-compress/` and survives close/reopen.

The project deliberately forbids project-local untrusted config from selecting persistence location, credential passthrough, hard output cap or retention controls.

## Tests/invariants extracted

`tests/unit/round-trip.test.ts` proves:
- setup/uninstall configuration round-trip
- persistent database returns byte-identical *search hit objects* after close/reopen, including injection-warning metadata
- persistent database survives close while ephemeral database is removed.

It does **not** prove byte-identical reconstruction of original indexed input.

`tests/unit/store.test.ts` protects:
- markdown/chunk counts
- search and fuzzy search
- persistent trigram reopen behavior
- private ephemeral DB directories
- exact source-ID scoping
- oversized chunk bounds
- diff-shaped data does not become one giant FTS row
- snippets remain bounded rather than returning whole documents.

Executor/tests and comments protect additional boundedness properties: process tree kill, timeout, hard capture cap, stderr diagnostics and memory-bound concerns.

## Boundary findings

### Agent ↔ visible result
Strong byte-budget discipline. Search/batch/execution responses have configurable ceilings and rate controls.

### Visible result ↔ indexed corpus
Useful separation: presentation compression happens separately from the searchable corpus. But the corpus is transformed and retained under lifecycle limits, so it must not be treated as canonical evidence.

### Indexed corpus ↔ untrusted content
Injection patterns are detected at index time and surfaced on retrieval; attribution-line injection is neutralized in rendered search hits. This is defense-in-depth, not an authority boundary.

### LLM ↔ executor / OS
The project explicitly trusts the LLM. Despite the user-facing phrase “sandboxed subprocess,” `SECURITY.md` states there is no OS-level sandbox; subprocesses have the MCP server user’s privileges and unrestricted outbound networking. Safe env allowlisting, timeouts, output bounds and process-tree cleanup are resource/security hygiene, not containment.

## Determinization candidates

- output byte budgeting
- capture hard caps/timeouts
- ANSI/progress normalization
- command-specific filters
- duplicate/error grouping
- section chunking
- source-ID scoping
- BM25/FTS retrieval
- retention pruning
- untrusted-content labeling
- response assembly

All are appropriate deterministic middleware candidates.

## Material extracted

IDEA: keep large material outside the model context and let the agent retrieve only relevant fragments.

PATTERN: searchable spillover store — compact visible result + externally indexed corpus + follow-up search.

ALGORITHM: source-scoped FTS5/BM25 retrieval with bounded trigram/fuzzy fallback; byte-budgeted rendering; retention pruning.

CODE: no direct reuse decision. Rust-native reimplementation would fit SPARK better if experiments validate the mechanism.

TEST/INVARIANT: presentation compression must never be confused with canonical evidence preservation; retrieval scope must not silently widen; every response/capture path needs hard bounds.

## Primary disposition

BORROW_PATTERN

Reason: the searchable-spillover mechanism is directly useful and substantially more evidence-aware than destructive summarization. But this implementation does **not** satisfy a strict “small context, full evidence” requirement by itself. A SPARK-grade design would preserve an immutable/raw evidence artifact separately, then build search/chunk indexes and compact views over that artifact.
