# Source Study — RTK (Rust Token Killer)

Status: FIRST TRACE COMPLETE / COMPARATIVE EXPERIMENT REQUIRED
Study date: 2026-09-13

## Frozen source

- Project: RTK — Rust Token Killer
- Upstream: `https://github.com/rtk-ai/rtk`
- Commit: `d0c2985155568d1d76fca03bc65d5098f136bbcd`
- Version: 0.48.0
- License: Apache-2.0
- Rust edition: 2021; minimum Rust: 1.91
- Relevant source: hook/rewrite registry, command modules, core filter/tracking/retriever/tee, trust/integrity.

## Problem framing

PROBLEM: agent shell commands emit far more text than most coding tasks need; raw test/build/git/search/list/log output consumes context and hides actionable failures.

USER: coding agents and humans using CLI development tools.

INPUT: ordinary shell command (often rewritten automatically by an agent hook), command arguments and the underlying command’s stdout/stderr/exit status.

OUTPUT: command-specific compact representation, original exit status, estimated token-savings telemetry and — for failures or explicit truncation paths — a recall handle for elided raw output.

AUTHORITY: RTK is not merely a postprocessor. Most command modules execute the underlying command themselves. Wrapping `git push`, package tools, cloud tools, test runners, kubectl, AWS, etc. therefore carries the authority of those commands. Hook rewriting also changes the command that the agent/host executes.

TRUST: hook installation/integrity state, command-rewrite rules, permission/config sources, project TOML filter trust, executable found on PATH, command-specific parser/filter correctness and local recall store.

STATE: installed hook + integrity hash/config; command registry; tracking SQLite; recall SQLite/legacy tee files; analytics; project filter trust state.

FAILURE: rewrite miss/failure generally falls through to the original command; filter failure is designed to fall back to raw output; underlying exit codes propagate. Recovery storage is best-effort and can be unavailable/disabled/truncated.

## Trace A — agent command → rewrite decision

Agent issues Bash command
→ installed PreToolUse/plugin hook receives agent-specific JSON
→ thin hook invokes `rtk rewrite` / in-process hook processor
→ central decision layer evaluates command permission/rewrite state
→ lexer tokenizes compound shell command and preserves byte offsets/operators/redirections
→ supported segments classified by registry rules
→ qualifying command rewritten to explicit `rtk ...`
→ host agent still applies its own approval protocol where supported
→ rewritten command executes.

Important engineering details:
- compound commands are segmented rather than regex-replaced wholesale
- pipeline semantics deliberately keep unsafe/intermediate segments raw
- unsupported flags/shell constructs defer instead of forcing a rewrite
- `RTK_DISABLED=1` disables rewrite for a command
- permission precedence modeled as deny > ask > explicit allow > default ask for supported hosts
- hook artifacts have SHA-256 integrity checks
- hook parse/integration failure is deliberately non-blocking and can let the raw original command proceed.

SPARK implication: this is good prior art for deterministic command recognition, but automatic external hook rewriting should not become canonical Tool Fabric authority. SPARK should call an explicit adapter/filter after its own broker has already authorized the operation.

## Trace B — RTK wrapper → underlying command → filter

RTK CLI parses command
→ routes to command-specific Rust module or TOML DSL filter
→ module executes underlying command with `std::process::Command`
→ captures raw stdout/stderr and exit status
→ applies one of several deterministic compression strategies
→ on parser/filter failure, falls back toward raw output
→ prints compact result
→ preserves underlying nonzero exit code
→ tracks raw vs compact byte/token estimate.

Filtering strategies include:
- stats extraction
- failure-only test output
- grouping by file/rule/code
- deduplication
- structure-only JSON/code views
- progress removal
- state-machine parsing
- NDJSON aggregation
- list/tree compression
- command-specific confirmation summaries.

This is the strongest RTK contribution: hundreds of command-specific deterministic reduction rules encode “what is usually load-bearing” below the LLM.

## Trace C — recovery / recall path

RTK’s current recall subsystem materially improves on destructive-only compression.

For sufficiently large **failed** commands (default minimum 500 bytes), and for successful command paths whose filter explicitly marks a truncation/elision:
→ raw/unfiltered bytes are passed to `core::retriever::store`
→ content hash derives from command + full original bytes using SHA-256, then is shortened to 12 hex chars for the stored public ID
→ payload is gzip-compressed by default
→ SQLite row stores command/cwd/exit/time/line/count/size/truncated/codec/blob
→ compact output includes `rtk recall <hash>` hint
→ recall can return stored bytes, bounded line slices or byte-regex matching without rerunning the original command.

Tests include gzip round-trips over arbitrary binary bytes and store/fetch byte-faithful cases.

### Critical bounds

Defaults:
- max entry payload: 10 MiB
- max entries: 200
- retention: 30 days
- recovery mode: SQLite
- success runs are **not** automatically archived in SQLite merely because they were compressed
- successful output is stored only when the command/filter explicitly calls forced truncation recovery
- recovery can be disabled by config/environment
- over-cap content is deliberately stored truncated; the recall command warns that requested lines can be lost.

Therefore RTK’s recall store is **recoverable compacting evidence**, not a universal immutable evidence archive.

### Identity caveat

The primary key exposed/stored is only the first 12 hexadecimal characters (48 bits) of SHA-256(command || 0 || full-content). `ON CONFLICT(hash)` replaces the stored row. That is reasonable as a short-lived local UX handle, but SPARK canonical evidence IDs need a full cryptographic digest (or collision-safe full identity), not a 48-bit prefix.

## Trace D — recall vs context-compress

### RTK
Strengths:
- knows the command semantics and can produce much better summaries than generic retrieval
- preserves exit code
- deterministic command-specific parsers
- raw recall for failures/explicit truncations
- easy runnable handle in compact output
- byte-oriented recall can preserve arbitrary bytes under cap.

Limits:
- many successful compression paths do not persist raw output
- retention/count/cap mean recall is not permanent/full by default
- short hash ID is unsuitable as canonical evidence identity
- each filter must correctly remember to invoke recovery on elision
- direct wrapper executes the command and therefore mixes execution with presentation.

### context-compress
Strengths:
- generic searchable spillover across arbitrary indexed material
- FTS5/BM25/fuzzy retrieval and source-scoped search
- can query hidden material by content rather than knowing a recall ID/line.

Limits:
- chunked/indexed representation is transformed and not byte-identical
- no raw canonical artifact by itself
- snippets are bounded retrieval, not full content guarantee.

### Combined SPARK lesson

Best shape is stronger than either implementation:

`authorized command executes once`
→ **immutable raw evidence artifact with full digest**
→ deterministic command-specific reducer (RTK-like)
→ derived searchable/chunk index (context-compress-like)
→ compact agent result carrying artifact ID + retrieval/search handles.

The reducer and search index should be rebuildable views over canonical raw evidence, not owners of that evidence.

## Trace E — token analytics

RTK records raw/filtered byte counts and estimates tokens as bytes / 4. Upstream documentation is appropriately explicit that:
- savings percentages refer to Bash output only
- they are not equivalent to bill reduction
- no tokenizer ships
- absolute token counts are approximate.

SPARK should similarly distinguish exact byte reduction from estimated model-token reduction, and use the actual target tokenizer only when the cost/benefit justifies it.

## Boundary findings

### Hook rewrite ↔ host authority
Rewriting a command is execution-affecting behavior. Host approval after rewrite is good defense, but SPARK should avoid depending on third-party hook correctness for canonical admission.

### Wrapper execution ↔ compression
RTK executes underlying commands; its filter is not an inert formatter. A future SPARK integration should prefer postprocessing already-authorized captured output or narrowly scoped adapters, not expose a generic `rtk <anything>` capability as ambient authority.

### Compact result ↔ raw evidence
Recall handles are excellent agent UX but not canonical evidence under default cap/retention/storage rules. Central evidence custody should happen before the reducer.

### Filter fallback ↔ context budget
Failing open to raw output preserves correctness but can unexpectedly flood agent context. A host integration should bound raw fallback: preserve full raw artifact externally and return a bounded “filter failed; raw evidence available” envelope instead of injecting arbitrary output.

### Permission config ↔ actual host permission
RTK mirrors several agent permission surfaces, but those mappings are compatibility behavior. SPARK authority remains its own broker.

## Determinization candidates

High-value mechanisms to mine/reuse:
- command lexer/classifier/registry
- filter taxonomy and command-specific parsers
- failure/test/build summarizers
- list truncation with explicit recovery hint
- exit-code-preserving proxy mechanics
- content-addressed recall UX
- byte/token reduction telemetry
- hook integrity verification patterns.

## Material extracted

IDEA: command-specific deterministic reducers can replace a large amount of LLM log filtering.

PATTERN: compact result + explicit loss/recovery handle.

PATTERN: reducer failure should preserve correctness but not destroy evidence.

ALGORITHM/CODE: several Rust parsers/filter modules may be directly reusable or adaptable under Apache-2.0 after dependency/API qualification.

TEST/INVARIANT: compact output must disclose truncation/loss and provide a resolvable evidence reference; exit status remains unchanged.

## Primary disposition

EXPERIMENT_NOW

Likely split disposition after experiment:
- BORROW_PATTERN for reducer/evidence UX
- REUSE_CODE selectively for isolated Rust filter/parser modules
- do **not** adopt automatic hook rewriting or RTK’s recall DB as SPARK canonical authority/evidence layer.
