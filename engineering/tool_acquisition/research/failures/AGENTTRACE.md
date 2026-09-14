# Failure Lessons — AgentTrace

Source pin: `9a10f9aae3bc508ddca83093b2d25aede5ad5bd0`
Status: FIRST TRACE COMPLETE

## AT-001 — Shared mutable active trace can misattribute concurrent async tool calls

FAILURE MODE: overlapping traces on one SDK instance can attach a tool call to another operation.

CAUSE: `AgentTrace` keeps one mutable `activeTraceContext`. `trace()` replaces it across an `await`, and `recordToolCall()` writes to whatever context is current when invoked. There is no task-local/AsyncLocalStorage boundary in the traced implementation.

EVIDENCE: `packages/sdk/src/index.ts`; first-trace tests cover simple collection/nesting but no overlapping async-context isolation was found.

SPARK LESSON: flight-recorder context must be explicit in the event/call handle or maintained by actor/task-local context keyed to the actual operation. Never use one process/instance-global “current trace” as causal truth under concurrency.

CONFIDENCE: HIGH by source-path analysis.

DISPOSITION: ARCHIVE_REFERENCE + acceptance invariant.

## AT-002 — OTLP export loses local causality/details

FAILURE MODE: exported telemetry can look complete while parent/child/tool/link semantics are missing.

CAUSE: current OTLP mapper emits one span per stored trace but does not export `parentId`, tool-call child records or arbitrary trace links. It derives trace and span IDs from the same local trace ID.

SPARK LESSON: OTel/export formats are projections of the canonical flight record. Export success does not prove semantic completeness.

CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN negative lesson.

## AT-003 — Raw payload copies create privacy/evidence duplication risk

FAILURE MODE: prompts, outputs and tool input/output containing secrets/PII are copied into the tracing database and can also be emitted into export attributes/previews.

CAUSE: local schema serializes caller-provided input/output; automatic PII redaction is explicitly absent. OTel export stringifies input/output up to a fixed preview length.

SPARK LESSON: canonical full payload belongs in the evidence/artifact layer. Flight-recorder events should carry digest/reference + bounded sanitized preview by default, with explicit higher-sensitivity capture policy when required.

CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN with strengthening.

## AT-004 — Derived cost can be mistaken for billing truth

FAILURE MODE: static/approximate local model pricing is used as if it were provider/account cost authority.

CAUSE: default calculator uses built-in approximate rates and a generic fallback for unknown models. Token counts are producer-supplied.

SPARK LESSON: cost telemetry must record pricing source/version and distinguish estimated cost from provider-billed/account truth. Economic authority cannot rely on an observability estimate.

CONFIDENCE: HIGH.

DISPOSITION: ARCHIVE_REFERENCE + schema requirement.

## AT-005 — Rate limiting intentionally drops observation

FAILURE MODE: absence of a trace is interpreted as absence of an operation.

CAUSE: if trace rate limit is exceeded, AgentTrace executes the wrapped function but skips recording the trace.

PROJECT INTENT: preserve application behavior under observability pressure.

SPARK LESSON: this is acceptable for secondary telemetry but unacceptable for required canonical evidence. Required execution evidence needs an independent durable path; telemetry may sample/drop only after canonical event/evidence capture.

CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN with canonical/telemetry separation.
