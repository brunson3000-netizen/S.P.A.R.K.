# Pattern Card — Syntax-Bounded Context Slicing

PATTERN: Exact Structural Evidence Slices Over Canonical Source
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH for mechanics; task-value experiment pending

## Source evidence
- ast-grep @ `45b5eb6705b4c24e04746137d259874abf1087ad`
- Tree-sitter @ `1b8407d1e718f2a26e2886c03cc55622d8d1d7bd`
- exact JSON match/capture ranges
- parser error/missing-node state
- convergence: context-compress searchable spillover; RTK context reduction hypothesis.

## Problem

Line-oriented search returns arbitrary context windows: too much unrelated text, incomplete structural units and repeated output. Cheap workers spend tokens reconstructing boundaries the parser already knows.

## Mechanism

1. Keep repository/source bytes canonical.
2. Parse with a pinned language grammar.
3. Compile a deterministic structural query.
4. Find exact syntax-node matches and metavariable captures.
5. Expand only by explicit structural policy, for example:
   - enclosing function/class/module
   - declaration/imports referenced by the task
   - caller/callee pattern hits
6. Return a bounded evidence envelope containing exact byte ranges and source digest.
7. Include parser/grammar version and parse-error overlap/confidence.

Derived syntax data is disposable/rebuildable.

## Benefits

- lower context volume
- complete syntactic units rather than arbitrary line fragments
- exact source attribution
- deterministic/repeatable retrieval
- language-aware structural patterns without model reasoning
- natural input to a searchable evidence store.

## Risks / failure modes

- parser recovery creates structurally plausible matches in invalid code
- grammar/version drift changes node kinds/ranges
- syntax match mistaken for semantic/reference truth
- contextual expansion policy becomes too broad
- missing language grammar falls back to text search without explicit downgrade
- rewrite capability accidentally enabled with read-only research authority.

## Boundaries crossed

Canonical source → derived syntax tree.
Syntax tree → exact match/range.
Match → agent-visible evidence.
Search capability → optional source mutation (must be separate authority).

## Determinization relevance

VERY HIGH. Parsing/query/range expansion/context budgeting are deterministic machinery.

## Likely architectural location

Repository inspection / evidence retrieval layer used by worker agents and Foreman-like inspection flows.

## Finding class

PATTERN + ALGORITHM + CODE + TEST/INVARIANT

## Primary disposition

EXPERIMENT_NOW

## Required invariants

1. Every slice binds to source path + immutable digest/version + exact byte range.
2. Parser/language/grammar version is attributable.
3. Parse errors/missing nodes are surfaced; error-overlapping evidence is never silently labeled clean.
4. No-match in malformed source is not semantic proof of absence.
5. Syntax match is not labeled semantic symbol/reference resolution without additional evidence.
6. Read-only slice permission never implies rewrite permission.
7. Structural expansion has hard byte/node limits.
