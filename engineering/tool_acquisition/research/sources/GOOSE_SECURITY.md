# Source Study — Goose Security / Tool Inspection

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: Goose
- Upstream: `https://github.com/aaif-goose/goose`
- Commit: `50666ae0b9a51e260b52b7efbab2e4e020346e94`
- License: Apache-2.0
- Primary runtime: Rust agent runtime with MCP/tool extensions and layered tool inspection.
- Primary evidence: `crates/goose/src/tool_inspection.rs`, permission/security inspectors, `security/mod.rs`, `egress_inspector.rs`, `adversary_inspector.rs`, agent construction/tests.

## Problem framing

PROBLEM: A coding agent may propose tool calls that are user-disallowed, unexpectedly mutating, repetitive, prompt-injected, exfiltrating, or malicious even when the protocol/tool itself is legitimate.

USER: Goose agent runtime and interactive/automated users.

INPUT: proposed tool calls, tool annotations, current Goose mode, user permissions, conversation/user task, configured security/adversary rules.

OUTPUT: per-tool inspection findings (`Allow`, `Deny`, `RequireApproval`) plus security/egress telemetry that is folded into the final permission partition.

AUTHORITY: the final permission result decides which proposed tool calls are approved/denied/require approval. Individual inspectors vary: some are authoritative inputs, some only log, and some are model-assisted advisory checks.

TRUST: user permission configuration, tool annotations, configured security thresholds/classifiers, agent/provider used by model-assisted judges and the accuracy/coverage of command/url pattern extractors.

STATE: persistent/user permission manager state, per-agent read-only annotation cache, SmartApprove negative cache, config-loaded adversary policy, session/conversation history and security telemetry.

FAILURE: each inspector has different failure behavior. The manager logs and continues when an inspector errors. Missing permission results default to approval-required during permission mixing, but non-permission inspector failure simply removes that extra defense. The LLM adversary reviewer explicitly fails open.

## Trace A — proposed calls → inspector fan-out → permission result

Agent constructs default `ToolInspectionManager`
→ registers multiple inspectors in a fixed order (security, egress, adversary, permission, repetition at the pinned source)
→ `inspect_tools` calls each enabled inspector sequentially over the same proposed requests
→ results are collected by request ID
→ permission inspector results form the baseline partition
→ non-permission findings apply monotonic overrides:
  - `Deny` removes from approved/needs-approval and adds to denied
  - `RequireApproval` can demote approved to approval-required
  - `Allow` never overrides an existing denial/approval requirement.

This is a strong composition rule: secondary safety inspectors may only make a permission outcome stricter, not broaden it.

Inspector **execution order** still matters for cost/telemetry, but permission-mixing semantics are intentionally monotonic by severity.

## Trace B — PermissionInspector

For each valid tool call:

1. `GooseMode::Auto` baseline allows.
2. Approve/SmartApprove checks explicit user permission first:
   - AlwaysAllow → allow
   - NeverAllow → deny
   - AskBefore → approval required.
3. SmartApprove may accept a tool annotated `read_only_hint=true`.
4. Extension management is forced to approval.
5. Unknown SmartApprove candidates may be sent to an LLM read-only detector.
6. Unknown/default paths require approval.

Important defensive details:
- explicit user permission outranks smart-approval caches
- legacy cached allow is re-judged rather than blindly trusted
- only **negative** LLM read-only determinations are cached name-wide as `AskBefore`; positive model judgment is not persisted as an ambient future allow
- when no permission result exists during final mixing, the call defaults to `needs_approval`.

Important limit:
- Auto mode intentionally approves all at the permission baseline. Its safety therefore relies on later/harder mechanisms and is a product mode, not a suitable SPARK hard-authority default.

## Trace C — pattern/ML SecurityInspector

`SecurityInspector`
→ `SecurityManager.analyze_tool_requests`
→ optional prompt-injection detection (disabled by default unless config enables it)
→ lazy scanner initialization
→ pattern-only or optional ML classifier
→ analyze each tool call with conversation context
→ findings below threshold are logged
→ malicious finding above configured threshold becomes `SecurityResult.should_ask_user=true`
→ inspector converts that to `RequireApproval`, not hard deny.

If requested ML initialization fails, Goose explicitly falls back to pattern-only scanning rather than losing the entire security scan.

This is a useful degradation pattern: optional probabilistic/ML enrichment can fail while deterministic scanning remains available.

But the product’s security scan is approval escalation/defense-in-depth, not an absolute host invariant.

## Trace D — deterministic egress observation

`EgressInspector` examines recognized shell/web tool arguments.

It deterministically extracts destinations for patterns such as:
- HTTP/FTP URLs
- git SSH remotes
- S3/GCS buckets
- scp/rsync/ssh targets
- Docker registry push/login
- generic networking commands
- npm/cargo publish.

It separately heuristically classifies direction as inbound/outbound/unknown for git, curl/wget uploads/downloads, package publishing, Docker, scp/rsync, etc.

Result behavior:
- emits structured security/network telemetry
- returns an `Allow` inspection result with destination reason
- does **not** enforce an egress allowlist or deny by itself.

This is excellent prior art for deterministic **effect extraction** but not egress containment. A SPARK broker can reuse the idea to turn a shell/web request into normalized intended network effects, then compare those effects with real host grants/firewall capability.

## Trace E — LLM AdversaryInspector

Optional `adversary.md` selects reviewed tools and supplies rules.

For each selected tool:
→ formats complete tool call/arguments
→ includes original task + last few user messages
→ sends a separate model call with an adversarial security-review prompt
→ expects first line ALLOW/BLOCK
→ BLOCK becomes hard `Deny`
→ ALLOW becomes allow-only finding (cannot broaden a stricter permission result)
→ model/provider/config failure produces **Allow** with `confidence=0` and explicit fail-open telemetry.

The default rules themselves say to err on the side of allow and block only clearly dangerous actions.

This makes AdversaryInspector useful as a second opinion/novel-threat detector, but explicitly unsuitable as a sole authoritative control.

## Trace F — inspector failure semantics

`ToolInspectionManager.inspect_tools` catches any inspector error, logs it and continues.

Consequences depend on the inspector:
- PermissionInspector failure → final mixer finds no permission result, so calls default to approval-required: relatively fail safe.
- Security/Egress/Repetition/Adversary inspector failure → that extra finding disappears; baseline permission remains.
- In Auto mode, baseline permission is Allow, so loss of secondary safety inspection can result in execution unless another host boundary stops it.

This distinction is load-bearing: **a composable inspector framework does not imply every inspector is a gate**. The host must label each check’s assurance/failure semantics.

## Tests/invariants extracted

### Permission
- explicit AlwaysAllow/AskBefore/NeverAllow map deterministically
- explicit user permission outranks legacy smart cache
- legacy cached allow is re-judged and can downgrade to AskBefore
- positive LLM read-only verdict is not cached name-wide; negative verdict is cached as AskBefore
- unknown tool defaults to approval in non-auto modes.

### Inspector composition
- Deny and RequireApproval can only tighten the permission partition
- Allow findings do not override denial/approval requirements.

### Egress
- tests cover destination extraction/direction and avoid simple quoted false positives for package-publish phrases.

### Adversary
- rule/tool-frontmatter parsing is deterministic
- complete argument object is formatted, including sibling fields rather than inspecting only one `command` key
- failure path is explicitly tested/documented as fail open.

## Boundary findings

### Model proposal ↔ host permission
Strong direction: tool calls are inspected after model proposal and before execution. However, Auto mode is intentionally permissive and is not SPARK’s authority doctrine.

### Tool annotation ↔ read-only authority
`read_only_hint` can enable SmartApprove. Remote/tool-provided annotations are useful metadata but should not be sufficient proof of non-mutating behavior in SPARK; host capability contracts should own effect classification.

### Pattern/ML scan ↔ hard authority
Pattern/ML prompt-injection detection can escalate to approval, but scanner configuration and ML failure cannot be allowed to define canonical authority.

### Intended egress ↔ actual egress
Regex extraction tells what a command appears to target. It does not enforce network reachability. Runtime network confinement remains a separate layer.

### LLM reviewer ↔ hard gate
Goose correctly treats its adversary review as optional/fail-open. SPARK can use model review for novelty/second opinion only, never as the sole enforcement of a hard boundary.

## Determinization candidates

- typed inspection-result envelope
- monotonic safety-result composition
- explicit user permission precedence
- negative-only risk caching
- deterministic egress/effect extraction
- scanner fallback from optional ML to deterministic patterns
- structured security telemetry.

## Material extracted

IDEA: tool safety is a stack of independently attributable inspectors around a host-owned baseline permission decision.

PATTERN: monotonic safety inspectors may tighten but never broaden an existing permission result.

PATTERN: deterministic intended-effect extraction (egress destinations/direction) can feed real capability checks.

PATTERN: probabilistic/model reviewers are defense-in-depth; their outage cannot become a hidden authorization state.

CODE: isolated Rust inspection/result-composition and pattern-extraction ideas are strong references; direct reuse would require dependency/API qualification and SPARK-specific authority semantics.

TEST/INVARIANT: every inspector needs an explicit assurance class and failure mode (`fail_closed`, `approval_on_error`, `advisory/fail_open`).

## Primary disposition

BORROW_PATTERN

The Goose runtime as a whole is not selected as SPARK’s security layer. The strongest transfer is the inspector composition model, with stricter SPARK labeling of which checks are authoritative.
