# Fable — comprehensive MCI source research

Status: READY TO HAND OFF. Research assignment; no claim that Fable has started or completed it.

## Assignment

Analyze the prepared source collection against S.W.A.R.M.'s MCI needs. Deliver governance-related study material first and keep all useful findings organized for later MCI functions. Finish with a disposition and coverage record for every source, not a list of interesting repositories.

Use [MCI_RESEARCH_MAP.md](MCI_RESEARCH_MAP.md), its eight category packets, and [MCI_RESEARCH_CATALOG.json](MCI_RESEARCH_CATALOG.json). Begin with [governance](research/categories/GOV.md). Follow [RESEARCH_PROTOCOL.md](RESEARCH_PROTOCOL.md); this brief adds routing and completion criteria without replacing it.

## Starting state and custody

Baseline inspected: phase1-refoundation-v2 at 7ee1f7f17dc1df1765e73d870a23ba4d5f90df89. Refresh branch state and read the active ledger before assigning work; other threads may advance it. Fourteen first-trace groups are recorded. E2B is active. Existing source notes, patterns, failure records and experiments remain valid at their recorded pins unless a relevant change invalidates them.

Read [FABLE_SOURCE_REVIEW.md](FABLE_SOURCE_REVIEW.md) and run its preparation/verification command in your checkout. There are 54 top-level pins; 51 plus five nested repositories passed the previous preparation verification. Three large sources require explicit selection or pinned remote review. Grok Build has prior study but no entry in this custody set; measure and record its availability before relying on local paths. Include Cloudways as documentation-only and historical consolidation as historical evidence. Do not let these supplements inflate the harvested count.

## Analysis method

1. Establish the current MCI consumer requirements from its authoritative records. Cite repository/path/revision for each criterion. Use the acquisition lane's Rust-first, deterministic-interface and consumer-authority boundaries for source screening; mark unverified MCI policy details as dependencies rather than deciding them.
2. Screen every source using the protocol's problem/user/input/output/authority/trust/state/failure framing. Record language/runtime, exact source, license evidence, dependencies and source completeness. Reuse existing study where applicable.
3. For retained candidates, trace 2–3 useful operations end to end with exact modules/functions and pin-bound links. Identify actual side effects and admission/enforcement points. Inspect protecting tests and negative paths. For irrelevant/duplicate/reference-only sources, a justified documented screen may end the study without unnecessary deep tracing.
4. Extract findings as IDEA, PATTERN, ALGORITHM, CODE or TEST/INVARIANT. Separate observed behavior, documentation claims, prior research and new hypotheses. A CODE candidate requires a named symbol/file and dependency/reuse assessment; a vague product recommendation does not qualify.
5. Compare by MCI function: what gap is addressed, where the mechanism would sit, what authoritative state it owns or must not own, Rust integration effort, platform support, external services/credentials, resource footprint, reversibility, failure behavior and evidence strength. State verified/documented/unknown for platform claims.
6. Keep source studies in research/sources, mechanisms in research/patterns, negative lessons in research/failures, comparisons in research/comparison and experiments in research/experiments. Link governance findings into GOV.md and relevant other categories. Keep full upstream trees intact and avoid duplicated evidence.
7. End every completed finding with exactly one primary disposition from RESEARCH_PROTOCOL.md: IGNORE, ARCHIVE_REFERENCE, BORROW_IDEA, BORROW_PATTERN, REUSE_CODE, REIMPLEMENT_IN_RUST, EXPERIMENT_NOW or DEFER. Include evidence and a reason; an experiment disposition proposes work, it does not authorize or prove execution.

## Qualification cases to look for

| Boundary | Required questions / negative cases |
|---|---|
| Authority | deny/allow conflict; evaluator error; stale approval; changed target; revoked permission; untrusted instructions |
| Delegation and budget | child escalation; concurrent reservation; uncertain cost; duplicate assignment; recursion and worker-count limits |
| Lifecycle | crash before/after effect; lost acknowledgement; retry duplication; cancellation; orphaned worker; stale lease/snapshot/handle |
| Tools and protocols | malformed arguments; identity mismatch; uninspectable call; partial stream; oversized output; false success exit status |
| Containment | host-side loading before sandbox; denied file/network access; resource exhaustion; host callback limits; cleanup failure |
| Context and evidence | lost critical detail; unretrievable raw result; stale/concurrent memory edit; missing telemetry; correlation race; export failure |

List which upstream tests protect these cases and whether you actually ran them. Reuse valid evidence. Run only safe local qualification permitted by the governing rules and resource grant; record unmet runtime/platform requirements. No paid inference, external deployment, consumer integration or change to governing policy is granted by this research brief.

## Deliverables and completion

- Updated per-source coverage: all 54 inventory IDs, Grok Build supplement and Cloudways reference have explicit status, evidence pointers, gaps and disposition where screening/study is complete.
- Governance packet: complete mechanism comparison, ranked shortlist, code/module candidates, failure tests, dependencies and consumer decisions still required.
- Seven other category packets: source findings, comparisons and future-use candidates; cross-links reuse governance evidence.
- A concise final recommendation separating useful now, useful later, reference only and rejected, with reasons and cost/fit tradeoffs. Do not invent numerical certainty scores.
- A gap ledger: missing custody, original historical evidence, unavailable backend, license uncertainty, unrun tests, platform limitations and consumer-property dependencies. Explicitly justify deferral; no silent skips.

Organization complete, first trace complete, qualification complete, test executed and consumer accepted are separate claims. Full study closeout requires each source screened and each retained candidate sufficiently traced; unresolved qualification remains visible even if the screening sweep is complete.

Log source/category progress in the repository at coherent checkpoints. Preserve other researchers' changes and active ownership. Formal independent review must follow the governing resource/review channels; a researcher cannot independently review their own work. Commit/publish only within existing authorization. Report repository, branch, commit, evidence paths, completed versus unfinished work and next authorized action.
