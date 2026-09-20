Review the RADAR release candidate as an independent battery reviewer.

Inputs:
- Historical source commit: 60da771a192fc2e71be041a0d1727ffd93bfcfbc
- Toolkit repair head: 0e640573699faf16e944c89abd84fed9c57d620c
- Historical battery tests: 9 pass; broader compute-qualification tests: 29 pass
- Independent finding: RADAR-V1-DEF-01 says every fixture omits its named task
  material and its oracle is derivable from the visible ordinal.
- Proposed repair: preserve V1 unchanged; create a versioned fixture pack with
  genuine code/diffs/schemas/tool definitions/constraints/source records/context,
  plus guessability tests; retain exact mechanical scoring and UNKNOWN handling.

Return only JSON with keys: verdict, blocking_findings, required_repair_scope,
independent_review_gate, and paid_execution_recommendation. Do not provide
hidden chain-of-thought. Assess whether this is sufficient to release paid
testing and whether the current OpenRouter/NIM-only runner is sufficient for
the requested tool-interaction battery.
