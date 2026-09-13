# Source Study — ast-grep + Tree-sitter

Status: FIRST TRACE COMPLETE / EXPERIMENT REQUIRED
Study date: 2026-09-13

## Frozen sources

### ast-grep
- Upstream: `https://github.com/ast-grep/ast-grep`
- Commit: `45b5eb6705b4c24e04746137d259874abf1087ad`
- Workspace version: `0.45.3`
- License: MIT
- Rust edition: 2024; Rust 1.88+
- Direct parser dependency at this pin: `tree-sitter = 0.27.0`

### Tree-sitter
- Upstream: `https://github.com/tree-sitter/tree-sitter`
- Commit: `1b8407d1e718f2a26e2886c03cc55622d8d1d7bd`
- Workspace version: `0.28.0`
- License: MIT
- Rust edition: 2024; Rust 1.90+

Important dependency fact: the separately harvested Tree-sitter head is one minor release ahead of ast-grep’s pinned runtime dependency. Treat the two pins as independent evidence; do not replace ast-grep’s Tree-sitter version without compatibility testing.

## Problem framing

PROBLEM: worker agents waste context on line-oriented grep windows that include unrelated text and miss structure such as complete expressions, functions, call sites, arguments and exact symbol boundaries.

USER: deterministic repo-inspection/search/refactoring tools and agents needing narrow code evidence.

INPUT: source bytes, language grammar and structural pattern/rule/query.

OUTPUT: syntax-node matches with exact source ranges, captured metavariables and optional bounded display context; optionally deterministic rewrites.

AUTHORITY: read-only search has repository-read authority only. ast-grep also supports mutation/rewrite, which is a materially different authority and should not be part of the first SPARK qualification.

TRUST: language grammar/parser, source bytes, structural query/rule and file-selection configuration. Parser output can contain recovery/error/missing nodes when source is incomplete or invalid.

STATE: parsing/search is primarily derived/ephemeral; incremental Tree-sitter trees can be updated but are not canonical repository truth.

FAILURE: invalid query pattern can fail parse/selection; source syntax errors can produce recoverable trees with `ERROR`/missing nodes; unsupported/incompatible grammar ABI rejects at language assignment; no matches is distinct from a malformed pattern.

## Trace A — source → Tree-sitter syntax tree

Tree-sitter:
→ create `Parser`
→ assign a language
→ `set_language` checks grammar ABI compatibility and parseability
→ parse source to a concrete syntax tree
→ tree nodes expose named/anonymous kind, byte/point ranges, fields, error/missing state and traversal.

Tree-sitter’s core property for SPARK is tolerant structural parsing: malformed/incomplete code does not necessarily make the entire file unavailable. The resulting tree records error/missing regions that the consumer can inspect.

The Rust binding exposes:
- parser/language ABI checks
- node kind/field identity
- exact byte/position ranges
- `Node::has_error`, `is_error`, `is_missing`
- parse state/lookahead for diagnostics
- incremental parsing APIs.

SPARK implication: parse-error state must accompany a structural slice when it can affect confidence. A successful structural match inside a recovered tree is evidence, but not equivalent to “file parsed cleanly.”

## Trace B — pattern source → structural matcher

ast-grep `PatternBuilder`:
→ parses pattern text with the target language
→ requires a meaningful single root node unless contextual selector is used
→ converts syntax nodes into `PatternNode::{Internal, Terminal, MetaVar}`
→ ignores parser-inserted missing nodes when converting pattern structure
→ supports metavariable capture (`$A`, `$$$ARGS`, etc.)
→ records matching strictness.

Pattern construction explicitly errors on:
- parse failure
- empty/no content
- multiple root nodes where one is required
- invalid root multi-metavariable
- invalid selector/kind.

`Pattern::has_error()` exposes when the pattern itself compiled around a Tree-sitter `ERROR` kind. CLI behavior distinguishes a malformed/error pattern with no matches from an ordinary no-match result.

## Trace C — search → exact structural evidence

Compiled pattern/rule
→ traverse syntax nodes
→ compare node structure/kinds/text under selected strictness
→ capture metavariables into match environment
→ produce `NodeMatch` referencing the original document.

Machine JSON output includes:
- matched text
- file
- language
- exact inclusive-start/exclusive-end byte range
- zero-based line/column start/end
- optional bounded leading/trailing context
- metavariable captures, each with text + exact range
- multi-capture arrays
- transformed capture values
- optional replacement/diff fields.

This output shape is unusually suitable for a worker-facing evidence slicer because the host can retain the whole file/artifact while returning only exact matched nodes plus deliberately selected surrounding definitions/context.

## Trace D — invalid/partial source

Tree-sitter can return trees containing `ERROR` and missing nodes. ast-grep exposes `Node::is_error` and its matcher supports the Tree-sitter `ERROR` kind explicitly; rules can search invalid syntax.

This is powerful but creates a confidence requirement:
- a match can be structurally useful in a file that also has parse errors
- a match that intersects/depends on an error recovery region should be flagged lower confidence
- absence of a match in a severely malformed tree should not automatically prove semantic absence.

The first SPARK implementation should therefore be **evidence extraction**, not a semantic oracle.

## Trace E — mutation boundary

ast-grep supports replacement offsets/diffs and rewrite rules. That is valuable for future deterministic refactoring, but it crosses from read-only evidence into source mutation.

For the first tool-fabric qualification:
- enable search/query only
- no in-place rewrite
- if a later mutation experiment is authorized, produce a proposed patch/diff artifact first and independently validate it.

## Tests / invariants extracted

### Parser/grammar
- incompatible Tree-sitter grammar ABI is rejected at `set_language`
- error/missing nodes are inspectable rather than silently erased.

### Pattern
- malformed/ambiguous structural patterns produce explicit errors
- root multi-capture is rejected
- contextual selector must exist
- language tests assert cross-language pattern behavior.

### Output
- machine JSON carries exact ranges and capture ranges
- JSON stream/compact modes support low-overhead machine consumption
- range end is explicitly exclusive.

### Tooling lesson
- syntax structure is deterministic derived evidence; it should never overwrite canonical source truth.

## Boundary findings

### Source bytes ↔ syntax tree
Tree is derived and grammar-dependent. Preserve source digest/version and parser/grammar version with any long-lived slice evidence.

### Parse recovery ↔ confidence
Tree-sitter recovery is a feature, not proof of correctness. Evidence envelope needs parse-health flags/error overlap.

### Pattern query ↔ result
Structural pattern is deterministic and inspectable. If the pattern itself has parse errors, fail the query rather than hand ambiguous evidence to the worker.

### Structural match ↔ semantic meaning
AST shape does not provide whole-program semantic resolution, type inference or dynamic dispatch truth. Do not label ast-grep results as semantic reference/call-graph facts without another layer.

### Search ↔ rewrite
Separate capabilities/authority. Read-only structural slicing should not imply mutation permission.

## Determinization candidates

Strong candidates:
- language detection/selection
- structural query compilation
- symbol/function/class boundary extraction
- call-pattern search
- exact byte-range slicing
- import/declaration extraction
- parse-health/error-region reporting
- duplicate/context-window reduction
- machine evidence envelopes.

## Material extracted

IDEA: give workers syntax-bounded evidence instead of arbitrary line windows.

PATTERN: canonical source + derived syntax index + exact evidence slice.

ALGORITHM: Tree-sitter parsing + ast-grep structural matching/metavariable capture.

CODE: both are strong Rust reuse candidates under MIT, but ast-grep should initially be consumed at its own compatible Tree-sitter dependency version rather than forcing the separately pinned Tree-sitter 0.28 head.

TEST/INVARIANT: every returned slice must bind to file/source digest + byte range + parser/language version + parse-health status; structural search must not be described as semantic proof.

## Primary disposition

EXPERIMENT_NOW

Likely implementation after a passing experiment: REUSE_CODE (`ast-grep-core`/language pieces and Tree-sitter through the dependency graph) for deterministic read-only context slicing.

## Controlled experiment

Compare identical repository questions under:

A. grep/ripgrep hit + fixed ±N-line windows
B. ast-grep structural match + nearest enclosing definition/import/declaration slices

Hold repository, questions and downstream worker/model fixed.

Measure:
- source bytes delivered
- estimated tokens delivered
- correct target recall
- inclusion of required enclosing definition/imports
- unrelated-context bytes
- number of follow-up searches
- task/debugging success
- search latency
- deterministic repeatability
- parse-error cases and false-confidence incidents.

Acceptance direction: materially lower context with no meaningful loss in answer/debugging success, plus explicit parse-health evidence.
