# Tool Architecture Research Protocol

Status: ACTIVE
Adopted: 2026-09-13
Owner: S.P.A.R.K. tool-acquisition research
Scope: external tool/agent/runtime research only; no automatic adoption or activation

## Research doctrine

Preserve full source; extract small truths.

Do not begin with “what can we copy?” Begin with:
1. What problem did the project discover?
2. What mechanism did it build?
3. What evidence/tests protect that mechanism?
4. What boundaries does it cross?
5. What failed or remains weak?
6. Where, if anywhere, would the mechanism belong in S.P.A.R.K./a future consumer?

Do not read repositories front-to-back by default. Reverse-engineer concrete execution paths and invariants.

## 1. Freeze source before study

Every source record must include:
- project
- upstream URL
- exact commit/tag
- retrieval date
- license
- language/runtime
- official docs
- relevant papers/blogs/design docs

Canonical pins live in the applicable harvest lock and gitlink. External source remains quarantined under `external/harvest/`; no external project is a runtime dependency merely because it is harvested.

## 2. Project framing

Before deep code reading, answer:

- PROBLEM — pain being solved
- USER — consumer
- INPUT — accepted input
- OUTPUT — produced output
- AUTHORITY — what it can affect
- TRUST — assumptions about trusted actors/data
- STATE — durable/ephemeral state retained
- FAILURE — behavior when components fail

## 3. Trace real paths end-to-end

For each source, trace 2–3 useful operations through actual modules/functions. Record every arrow and implementation location.

Examples:
- agent request → discovery → schema → validation → execution → verification → result
- command → interception → execution → raw output → compression → compact result → evidence retrieval
- agent → lookup → identity → permission → routing → tool → response → audit

## 4. Treat tests as architecture evidence

Extract:
- invariants
- failure semantics
- edge cases
- security assumptions
- validation rules
- state transitions
- retry/cancellation behavior
- adversarial boundary coverage

Marketing claims without protecting tests are weak evidence.

## 5. Extract mechanisms, not products

Every reusable finding becomes a Pattern Card with:
- source/project/module/commit
- problem
- mechanism
- dependencies
- benefits
- risks
- likely architectural location
- implementation choice
- evidence
- status/confidence

## 6. Separate finding classes

Classify each finding as one or more of:
- IDEA
- PATTERN
- ALGORITHM
- CODE
- TEST/INVARIANT

Zero copied code can still be a strong result if the source yields useful invariants/tests.

## 7. Boundary analysis is mandatory

Inspect crossings such as:
- LLM ↔ deterministic code
- agent ↔ tool
- tool ↔ OS
- agent ↔ agent
- process ↔ process
- host ↔ sandbox
- local ↔ remote
- trusted ↔ untrusted
- read ↔ write
- proposal ↔ authority

At each crossing record: typing, validation, identity, authority check, output bound, mutation ability, escape risk, cancellation, observability, containment.

## 8. Determinization candidates

Flag `DETERMINIZATION_CANDIDATE` whenever model reasoning can plausibly be replaced by deterministic machinery, especially for:
- state tracking
- log filtering
- obvious tool selection
- known retries
- deadlines
- deterministic routing
- invariant/evidence checks

## 9. Agent-facing surface

Capture exactly what the agent sees:
- tool/system instructions
- command names
- schemas
- examples
- errors
- result envelopes
- context/token burden

Research question: what is the minimum information a cheap worker needs to use the capability reliably?

## 10. Failure ledger

Failures and limitations receive their own durable record with:
- failure mode
- source
- cause
- project response
- whether it worked
- transferable lesson
- confidence

## 11. Convergence

After source-specific traces, compare projects by problem/mechanism rather than brand. Repeated independent solutions are stronger evidence than a single project’s preference.

## 12. Mandatory decision bucket

Every completed finding ends in exactly one primary disposition:
- IGNORE
- ARCHIVE_REFERENCE
- BORROW_IDEA
- BORROW_PATTERN
- REUSE_CODE
- REIMPLEMENT_IN_RUST
- EXPERIMENT_NOW
- DEFER

Disposition requires evidence and justification.

## 13. Experiments

Strong claims become controlled comparisons. Keep model/task/toolset fixed where possible and vary the mechanism. Measure success, tokens/context, turns, latency, missed evidence, failures, and recovery.

## Repository memory rule

Load-bearing research state must be written to the repository as it is established. Conversation text is not authoritative project memory.
