# Pattern Card — Searchable Spillover

PATTERN: Searchable Spillover Outside Agent Context
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH for the mechanism; separate raw-evidence requirement remains

## Source evidence
- Open330/context-compress @ `59fae35a7b383876a34f84090f6da978e230795a`
- `src/executor.ts`
- `src/util/intent-filter.ts`
- `src/store.ts`
- `src/tools/search.ts`
- `tests/unit/store.test.ts`
- `tests/unit/round-trip.test.ts`

## Problem

Large deterministic outputs contain useful evidence but are too expensive to inject wholesale into a worker’s context.

## Mechanism

1. Capture a bounded corpus outside model context.
2. Build a searchable section/chunk index over that corpus.
3. Return a compact result containing status, top query-conditioned snippets, source identity and retrieval hints.
4. Let the agent issue bounded follow-up searches scoped to the relevant source.
5. Keep indexing/search/retention deterministic.

## Dependencies

- bounded capture
- stable source/evidence identity
- chunk/index layer
- source-scoped search
- output byte budgets
- retention/lifecycle policy
- untrusted-content labeling

## Benefits

- context scales with relevance, not raw output size
- agent can revisit evidence rather than relying on one irreversible summary
- deterministic indexing/search can replace model-based log filtering for many tasks
- source scoping prevents cross-command attribution mistakes

## Risks / failure modes

- indexed representation may not be byte-identical to source
- retention/ephemeral lifecycle may remove older material
- hard capture caps can mean later bytes never exist in the store
- search ranking/snippet windows can hide relevant evidence
- “full content” language can overstate actual retrieval guarantees
- indexed hostile text can become prompt-injection material when replayed

## Boundaries crossed

Execution → evidence capture: must be hard bounded.
Evidence capture → search index: derived representation; not canonical evidence.
Search index → agent: byte-bounded, source-attributed, untrusted-content labeled.
Agent → retrieval: source scope must be explicit and never silently widen.

## Agent-facing surface

A compact result should state:
- operation status
- compact findings
- evidence/artifact identity
- what remains searchable/retrievable
- bounded hints/terms

Follow-up search should accept source/evidence identity plus query; the worker should not receive the entire store catalog.

## Determinization relevance

VERY HIGH. Capture, indexing, ranking, scoping, response budgets, retention and error-line extraction can all sit beneath the model.

## Likely architectural location

Evidence/Artifact layer immediately below tool execution results.

## Finding class

PATTERN + ALGORITHM + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## SPARK strengthening required

Do **not** make the searchable index the canonical evidence object.

Preferred stronger shape:

`immutable raw artifact → derived search index → compact agent view`

The raw artifact should have an immutable digest/identity and explicit retention. Search/chunk material is rebuildable derived state.

## Required invariants

1. Compact view cannot delete canonical evidence.
2. Search index is explicitly derived/non-authoritative.
3. Agent-visible search results carry evidence/source identity.
4. Search scope cannot silently widen.
5. Capture/search/result paths have hard byte/time/item bounds.
6. Retention deletion is explicit and observable.
7. Untrusted retrieved content is labeled as data.
