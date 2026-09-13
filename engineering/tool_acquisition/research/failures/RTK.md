# RTK Failure / Boundary Lessons

Source: `rtk-ai/rtk` @ `d0c2985155568d1d76fca03bc65d5098f136bbcd`

## RTK-F01 — Recall is not universal raw evidence custody

RTK SQLite recovery stores failures and filter-declared truncations, not every successful compressed command. Default retention is 200 entries/30 days and payloads above 10 MiB are stored truncated.

SPARK lesson: preserve canonical raw operation evidence before any reducer. Recall is a derived UX layer.

## RTK-F02 — Short recall hash is not a canonical evidence ID

The content digest is SHA-256 over command + NUL + full content, but only the first 12 hex characters are used as the stored/public primary key. A collision would hit `ON CONFLICT(hash)` and replace the row.

SPARK lesson: use full collision-resistant digest/identity for canonical artifacts. Short prefixes may be display handles only after uniqueness is checked.

## RTK-F03 — Filter correctness is distributed across command modules

Successful elision recovery depends on the module explicitly invoking `force_tee_hint`/`force_tee_tail_hint`; structured failure recovery similarly depends on the expected shared helper contract.

SPARK lesson: evidence capture should be centralized around execution, not entrusted to each presentation filter.

## RTK-F04 — Raw fallback preserves correctness but can flood context

Filter failure/passthrough deliberately favors raw command output. This protects operational semantics but can inject arbitrarily large output into an agent context.

SPARK lesson: preserve raw bytes externally and return a bounded filter-failure envelope with evidence reference rather than placing unbounded fallback output into worker context.

## RTK-F05 — Automatic hook rewrite is execution-affecting middleware

The hook changes the command before execution. It has integrity/permission safeguards and generally defers on uncertainty, but integration failure can fall through to the original command and agent support varies.

SPARK lesson: do not make an agent-specific hook the canonical authority/execution path. Keep broker authorization and operation identity explicit, then apply reduction to the resulting evidence.

## RTK-F06 — Wrapper compression and operation authority are coupled

RTK command modules run underlying commands themselves. A generic RTK capability would therefore transitively expose git/cloud/kubernetes/package/test authority.

SPARK lesson: reuse/adapt narrow reducer/parser components or explicit scoped adapters, not ambient `rtk <command>` execution authority.
