# Goose Security / Inspector Failure Lessons

Source: `aaif-goose/goose` @ `50666ae0b9a51e260b52b7efbab2e4e020346e94`

## GOOSE-F01 — Optional inspector failure is not a hard stop

`ToolInspectionManager` logs an inspector error and continues.

Effect depends on the inspector: PermissionInspector absence defaults unresolved calls to approval-required in final mixing; security/egress/repetition/adversary failure simply removes that extra defense. Under an allow-all baseline mode, this can leave the call allowed.

SPARK lesson: every pre-execution check must declare whether failure means deny, approval-required, or advisory loss. Never infer this from the fact that it is called a “security inspector.”

## GOOSE-F02 — Adversary LLM review explicitly fails open

AdversaryInspector returns `Allow` with confidence 0 when provider/model/config/review fails.

SPARK lesson: model review is second-opinion evidence only. It can tighten or flag a decision but cannot carry a hard invariant whose outage would widen authority.

## GOOSE-F03 — Egress inspector observes intent but does not enforce network capability

Regex/tool-argument inspection extracts network destinations/direction and logs them, then returns `Allow`.

SPARK lesson: intended-effect extraction is useful input to policy. Actual egress must be enforced by runtime network capabilities/firewall/proxy state and verified independently.

## GOOSE-F04 — Remote read-only annotation can influence SmartApprove

A tool with MCP `read_only_hint=true` can be allowed in SmartApprove mode absent stronger user policy.

SPARK lesson: provider-supplied/read-only annotations are descriptive metadata, not proof of effects. Canonical Tool Contract/effect class belongs to the host and must be independently qualified.

## GOOSE-F05 — Auto mode is intentionally broader than hard-policy doctrine

PermissionInspector baseline in `GooseMode::Auto` is `Allow` for all proposed tools; secondary inspectors can tighten it.

SPARK lesson: permissive product automation modes cannot substitute for host current-grant enforcement. SPARK’s authoritative baseline remains least privilege; convenience modes may only operate within an already bounded envelope.

## GOOSE-F06 — Security threshold escalates to approval, not containment

Pattern/optional ML scan above threshold produces `RequireApproval`, not a runtime containment guarantee. Below-threshold malicious findings can be logged only.

SPARK lesson: prompt-injection/malicious-command classifiers are useful detection evidence but not substitutes for deterministic authority, filesystem/network sandboxing or explicit user/operator grants.
