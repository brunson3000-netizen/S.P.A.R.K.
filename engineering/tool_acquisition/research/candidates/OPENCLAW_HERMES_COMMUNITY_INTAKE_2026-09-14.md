# OpenClaw and Hermes — agent harnesses and supervisor composition

Retrieved: 2026-09-14 UTC
Status: INTAKE — source study admitted, implementation not validated
Primary disposition: ARCHIVE_REFERENCE

## Admission decision
Add both to MCI source-study intake as the largest accumulated-attention pair in this sampled set. Compare concrete mechanisms; do not select a winner from popularity.

## Community signal
A [June 30 side-by-side user report](https://www.reddit.com/r/hermesagent/comments/1ujucjo/hermes_vs_openclaw_a_full_day_sidebyside_test/) describes specialized scout/planning/verification/coding/audit roles. A [May 30 comparison](https://www.reddit.com/r/Agent_AI/comments/1ts0pmm/hermes_vs_openclaw_gatewayfirst_multiagent/) discusses routing versus persistent skills/memory. These are historical user accounts, not controlled benchmarks or proof of a current weekly trend. Current GitHub snapshots show large accumulated audiences.

## Framing from pinned READMEs
- PROBLEM / USER: persistent tool-using assistants for operators, with external channels and execution backends.
- INPUT / OUTPUT: inbound messages, tools, context and skills produce actions, answers and retained state.
- AUTHORITY / TRUST: channel pairing, tool approvals and host/container execution must be distinguished. OpenClaw explicitly states main-session tools run on the host unless sandboxing is configured.
- STATE: OpenClaw documents local state/memory/credentials; Hermes documents persistent skills and multiple execution backends.
- FAILURE: restart, duplicate-message, stale-memory and child-cancellation behavior needs source tracing.

## MCI value and next traces
1. External message → identity/pairing → routing → worker session → tool authorization → result.
2. Supervisor delegation → child authority → cancellation → durable completion; first establish whether each advertised composition is implemented by the harness or only by user prompts.
3. Skill/memory write → validation → later retrieval. Inspect trust labels and whether a worker can change its own governing instructions.

DETERMINIZATION_CANDIDATE: role routing, identity checks, task ownership, cancellation propagation and completion evidence checks.
Potential location: MCI agent adapters and supervisor/worker contracts. Finding class: IDEA.
A second LLM reviewing the first does not itself create independent enforcement. Compare same task/model/toolset before claiming quality or cost gains.
OpenClaw license metadata is NOASSERTION; resolve actual license files before any reuse.

## Source record
### openclaw/openclaw
- Upstream: https://github.com/openclaw/openclaw
- Exact inspected commit: `b0638604941dda7eb80095481576898f875caf08`
- Primary source: https://github.com/openclaw/openclaw/blob/b0638604941dda7eb80095481576898f875caf08/README.md
- Language: TypeScript; detailed runtime/dependencies not yet traced.
- License: NOASSERTION (GitHub metadata only; file-level review required before reuse).
- GitHub snapshot: 389621 stars; not a growth rate.

### NousResearch/hermes-agent
- Upstream: https://github.com/NousResearch/hermes-agent
- Exact inspected commit: `ee4452991d17534aa561f31ee55596d082aa94e7`
- Primary source: https://github.com/NousResearch/hermes-agent/blob/ee4452991d17534aa561f31ee55596d082aa94e7/README.md
- Language: Python; detailed runtime/dependencies not yet traced.
- License: MIT (GitHub metadata only; file-level review required before reuse).
- GitHub snapshot: 245163 stars; not a growth rate.

## Evidence limits and handoff
Pinned README and repository metadata inspected. No code paths or protecting tests traced; no runtime executed. Documentation claims are hypotheses. These refs are intake pins, not canonical harvest-lock/gitlink additions. No increase to the previously recorded 48 harvested sources is claimed. Continue under RESEARCH_PROTOCOL.md; preserve the other thread's active trace queue.
