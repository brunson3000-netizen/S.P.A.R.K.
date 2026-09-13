# Failure Ledger

Status: ACTIVE

Use one entry per durable failure lesson.

---

## Template

FAILURE MODE: <name>
OBSERVED IN: <project/module/test/commit>
CAUSE:
PROJECT RESPONSE:
DID IT WORK?:
BOUNDARY AFFECTED:
S.P.A.R.K. LESSON:
TRANSFER CONFIDENCE: <HIGH | MEDIUM | LOW>
DISPOSITION: <decision bucket>
EVIDENCE:

---

## F-001 — Tool catalog overload

OBSERVED IN: progressive MCP Guardian and current on-demand tool-discovery ecosystem.
CAUSE: eagerly injecting many complete tool schemas into model context consumes large context and increases selection burden before work starts.
PROJECT RESPONSE: replace the full catalog with a small discovery/schema/execution surface; retrieve details only when needed.
DID IT WORK?: upstream benchmark claims are promising but require independent SPARK reproduction.
BOUNDARY AFFECTED: agent ↔ tool catalog.
S.P.A.R.K. LESSON: discovery metadata and executable capability detail should be separable; catalog membership must not imply authority.
TRANSFER CONFIDENCE: MEDIUM pending controlled experiment.
DISPOSITION: EXPERIMENT_NOW.
EVIDENCE: pinned middle-layer sources and forthcoming P0 trace records.

## F-002 — Compression can hide load-bearing evidence

OBSERVED IN: general output-compression problem; RTK qualification risk; context-compress/ctx-zip comparison lane.
CAUSE: reducing agent-visible output can discard the exact evidence needed for diagnosis or later review.
PROJECT RESPONSE: preserve original/raw material outside the prompt and expose compact summaries/references/search/retrieval.
DID IT WORK?: project-specific mechanisms require independent verification for fidelity and recoverability.
BOUNDARY AFFECTED: deterministic execution ↔ agent-visible evidence.
S.P.A.R.K. LESSON: compact presentation must never become evidence destruction. Preferred doctrine: small context, full evidence.
TRANSFER CONFIDENCE: HIGH as a safety requirement; implementation choice unproven.
DISPOSITION: EXPERIMENT_NOW.
EVIDENCE: forthcoming P0 context-compression trace and recovery tests.
