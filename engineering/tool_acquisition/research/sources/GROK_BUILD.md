# Source Study — xAI grok-build

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: grok-build
- Upstream: `https://github.com/xai-org/grok-build`
- Commit: `37949780c144e37df692e3d669051a21fec24f20`
- Source revision recorded by the sync commit: `c4ea71cfdbcdb21e32e41bc25a0043d7d4836714`
- Retrieval/study date: 2026-09-13
- License: Apache-2.0
- Primary language/runtime: Rust workspace
- Relevant subsystems: `xai-grok-workspace` permissions/trust, `xai-grok-memory` v2, `xai-grok-shell` sessions/subagents, `xai-computer-hub-core` remote tool + Grok Bot contracts, prompt templates and user-guide security docs.

## Problem framing

PROBLEM: A coding agent needs broad local/external capability while remaining usable across interactive, headless, subagent and remote-tool modes. It also needs durable context without letting repository content or stale memory silently become authority/current truth.

USER: software-engineering agents, interactive users, automation/SDK clients, child agents and remote tool endpoints.

INPUT: user/model tool calls, repository/user/managed policy, repository-local config/instructions/skills, session turns, durable memory files, remote-tool registrations and subagent requests.

OUTPUT: authorized tool execution, bounded model-facing tool schemas, child-session work, durable observations/topics, generated memory manifests, audit/telemetry and structured remote-tool streams.

AUTHORITY: Host-owned permission and trust machinery can allow, ask or deny filesystem, shell, MCP, web and other operations. Subagents share parent backends but remain subject to inherited/clamped policy. Repository-local configuration is separately gated before it can affect tools, policy or instructions.

TRUST: Managed requirements/policy and host runtime outrank model preference. Repository content is potentially hostile until folder trust permits project scope. Memory is historical context rather than current truth. Remote transport identity/session binding is explicit but remote service internals are outside this checkout.

STATE: Session actors, permission state and remembered grants/denies, trusted-folder store/cache, subagent coordinator state, memory v2 SQLite state/indexes plus immutable observation files, remote-tool registration snapshots and transcripts/tool metadata.

FAILURE: Security-sensitive paths generally fail closed: deny outranks allow/mode; untrusted project scope is withheld; stale memory edits are rejected; unsafe/symlinked memory paths are rejected; stale capture leases are fenced; conflicting idempotent capture retries are rejected; remote transport failures become network/tool errors.

## Trace A — concrete tool call → current permission decision

Model requests a tool
→ host `PreToolUse` hooks may deny
→ merged permission rules evaluate with severity `deny > ask > allow`
→ remembered project grants/denies are consulted where applicable
→ built-in read-only auto-approvals may apply
→ mode prompt policy (`ask`, `auto`, always-approve and related compatibility modes) decides any remaining request
→ admitted tool executes through host backend.

The product documentation and manager source agree on the important ordering property: a deny is not merely a prompt preference. It is evaluated above broad auto/always-approve behavior. `mcp_pre_decision` likewise checks remembered per-tool deny before remembered server/tool grants.

The permission manager is actor-owned and carries:
- current always-approve/auto state
- managed-policy pin
- read-deny globs inherited by subagents
- in-flight accounting
- prompt notifications
- policy/classifier/hook state.

Permission rule classes include Bash, read/edit/write/search/MCP/web-fetch/skill-oriented decisions. The shell path performs substantial deterministic command decomposition and unsafe-flag checks before accepting a command as read-only/routine.

Primary evidence:
- `crates/codegen/xai-grok-workspace/src/permission/manager/mod.rs`
- permission policy/preflight/resolution modules
- `crates/codegen/xai-grok-pager/docs/user-guide/22-permissions-and-safety.md`

## Trace B — cloned repository → project-scope loading

Workspace opens
→ folder-trust scanner checks for repository-local behavior-bearing configuration
→ sources include project MCP/LSP configuration, permission policy, instructions (`AGENTS.md`/`CLAUDE.md`), skills and plugin/config paths
→ one trust decision is resolved before repository-local services/loaders are allowed
→ loaders call the cheap project-scope gate
→ untrusted project scope with relevant config stays blocked; trusted scope is allowed.

Important details:
- trust is separate from plugin trust
- only project-scoped content is gated; user/bundled/global machinery remains distinct
- a “no relevant repo config exists” allow is deliberately provisional and not cached as durable trust, so a later `git pull`/agent write introducing executable config is re-evaluated
- cached untrusted decisions can reconcile a later explicit trust grant
- unsafe broad roots are handled specially rather than becoming normal persisted grants
- trust resolution is intended to occur before repo-local MCP/LSP/policy/instruction/skill loading.

This is direct supply-chain defense against a clone silently installing behavior into the agent.

Primary evidence:
- `crates/codegen/xai-grok-shell/src/agent/folder_trust.rs`
- workspace folder-trust/trust-store modules and tests.

## Trace C — durable memory write and observation capture

### User/model-visible memory filesystem

Memory v2 initializes separate global/workspace scopes containing:
- `topics/`
- immutable/pending `observations/_inbox/`
- `archive/`
- generated bounded `MEMORY.md`
- durable `memory_state.sqlite`
- lexical `index.sqlite`.

The model-facing prompt tells the agent:
- `MEMORY.md` is a bounded generated discovery index and must not be edited
- topics contain durable reusable facts
- observations are pending material for later consolidation
- only designated memory roots exist
- existing memory files must be read before editing
- memory is historical context, not current truth; live tools must verify changing facts.

`V2MemoryAccessPolicy` enforces the filesystem side:
- absolute in-scope paths only
- canonical containment + symlink rejection
- writes only to direct `.md` children of `topics/` or `observations/_inbox/`
- generated manifests/archive/databases are protected
- write size caps
- existing file replacement requires a previously recorded content hash
- if the file changed since that read, the edit is rejected as stale
- writes are atomic; manifest refresh failure attempts rollback.

Tests cover protected/path aliases, symlink aliases, read-before-write, stale edit rejection, size bounds, nested-path rejection and manifest refresh behavior.

### Background durable observation capture

`V2CaptureStore`
→ deterministic session/range job ID
→ SQLite state is cross-process serialization point
→ worker claims a bounded lease
→ immutable observation files are prepared/published
→ outcome hash/state committed
→ search index and cursor advance
→ `reconcile()` repairs supported crash windows.

Key crash/idempotency design:
- files can exist before outcome row; files carry job identity/expected count so only a complete hash-consistent set can be adopted
- outcome commits before indexing/cursor advancement, allowing deterministic replay of the second crash window
- duplicate enqueue converges on one deterministic job
- retrying the same outcome bytes is idempotent
- retrying different bytes for the same committed lease conflicts
- expired leases can be reclaimed; the stale owner is fenced
- tests assert file/index/manifest/cursor convergence and recovery.

Primary evidence:
- `crates/codegen/xai-grok-memory/src/v2.rs`
- `v2_access.rs` + `v2_access_tests.rs`
- `v2_capture.rs` + `v2_capture_tests.rs`
- agent prompt template.

## Trace D — parent agent → child agent

Parent task/subagent request
→ shared task coordinator actor owns pending/active/completed state, waiters, deadlines and cancellation
→ shell adapter constructs child session
→ child shares parent filesystem, terminal, hunk tracker, environment and other host backends
→ parent hooks are inherited so child tool calls traverse the same pre-tool gate
→ child permission mode is resolved/clamped against parent/managed policy
→ lifecycle notifications/results flow back to parent.

Important authority properties:
- sharing a backend does not mean the child owns it
- parent/root process scope is retained so nested child processes can be reaped with the root session
- child question/user-interaction ability is deliberately not inherited (`ask_user_question_enabled` stays false)
- policy can downgrade a child-requested bypass to `Default`; plugin-agent policy is separately constrained
- concurrency, nesting depth, sampling limits and workflow-agent limits are host configuration, not model-selected unlimited recursion.

Primary evidence:
- `crates/codegen/xai-grok-shell/src/agent/subagent/mod.rs`
- `handle_request.rs`, task coordinator/admission types
- `agent/config.rs` + config/subagent tests.

## Trace E — remote/hub tool contract

`xai-computer-hub-core` defines one host-neutral remote-tool contract:
- local and remote registrations implement the same `ToolHandle`
- remote transport is bound to `(user_id, session_id)`
- progress subscription is installed before request send
- response and progress are merged into `Progress* → exactly one Terminal`
- transport failures surface as tool/network errors.

The same crate defines the model-facing Grok Bot harness tools. At this pin the shared public contract includes create/list/send prompt/transcript/await/search primitives. Notable interface properties:
- `bot_send_prompt` has explicit `fire_and_forget | blocking | async` modes
- busy behavior is an enum (`reject | queue | supersede`)
- blocking/async continuation returns a handle
- `bot_await_turn` is the wait primitive; transcript tools explicitly say not to poll to wait for completion
- schemas set `additionalProperties: false`
- the default bot-tool surface has a dedicated prompt-byte budget test.

Boundary on evidence: this checkout proves the Rust schemas/descriptions/gating/shared transport contract. Code search did not expose the authoritative backend implementation of the Grok Bot service itself, so server-side queue/supersede/handle semantics are not independently credited beyond the published contract in this first trace.

Primary evidence:
- `crates/common/xai-computer-hub-core/src/bot_tools.rs`
- `remote.rs`, resolver/transport/registry modules.

## Tests/invariants extracted

### Permission
- deny rules outrank ask/allow and broad permission mode
- project/global permission sources merge by severity, not by lower-layer override
- remembered deny wins over remembered grants in supported paths
- unsafe “read-only-looking” command flags are detected before routine auto-approval.

### Repository trust
- project behavior-bearing config is a trust-gated supply-chain input
- “no config yet” is not a durable trust grant
- trust for one subsystem (folder vs plugin) is not silently generalized.

### Memory
- manifest/generated/database/archive internals are protected
- read-before-write is required for replacing existing memory
- stale concurrent edit is rejected
- observation capture is deterministic/idempotent and lease-fenced
- recovery converges incomplete crash states
- raw durable files and derived search/index state are distinct.

### Subagents
- child lifecycle is coordinator-owned
- parent/root backends and gates are reused
- requested child permission mode can be clamped by parent/managed restrictions
- recursion/concurrency/sampling are bounded by host configuration.

### Agent-facing surface
- internal bot-tool schemas are closed and byte-budgeted
- waiting is modeled as an explicit continuation/await operation, not transcript polling.

## Boundary findings

### Model ↔ permission system
Strong separation. The model proposes tool calls; host state/policy decides. “Auto” and “always approve” are modes inside the host gate, not authority held by the model.

### Repository ↔ agent configuration
Explicit trust boundary. Repo content can contain commands, policy, instructions, MCP/LSP and skills, so loading it is treated as potential code/policy execution.

### Memory ↔ current project truth
Explicit epistemic boundary in the prompt: memory is historical context. Repository/live tool evidence is fresher authority for changeable claims.

### Memory file ↔ write
Optimistic concurrency boundary: a prior read snapshot is required and stale writes reject.

### Parent ↔ subagent
Shared implementation resources, bounded inherited authority. Child sessions are not separate roots of trust.

### Local ↔ remote tool
Transport/session identity and streaming protocol are separated from tool implementation. A remote registration is wrapped to look like the same host `ToolHandle`, which is useful for a unified tool fabric.

## Determinization candidates

Very high-value deterministic machinery:
- permission rule merge/evaluation
- remembered deny/grant resolution
- shell command parsing/risk checks
- project trust scan and cache invalidation
- subagent admission/depth/concurrency/lifecycle
- memory path classification/containment
- read-snapshot stale-write detection
- durable job IDs/leases/fencing/reconciliation
- bounded memory manifest generation
- remote progress/terminal correlation
- prompt-surface schema/budget checks.

## Material extracted

IDEA: coding-agent runtime as host-owned actors and typed backends rather than prompt-owned coordination.

PATTERN: ordered call-time permission pipeline where hard deny remains effective even under broad automation modes.

PATTERN: repository-trust gate before project-controlled servers/policy/instructions/skills can load.

PATTERN: durable capture as `immutable files + transactional outcome state + derived index + deterministic reconciliation`.

PATTERN: read-before-write memory editing using content digest as a stale-write guard.

PATTERN: child agents share parent backends/gates while host policy clamps authority and resource limits.

PATTERN: explicit continuation handles / await primitive instead of polling for asynchronous agent turns (contract proven; backend implementation not inspected here).

CODE: Rust source is unusually relevant to SPARK. No direct code reuse is approved by this research record; dependency/API boundaries and fit must be qualified per module before any reuse decision.

TEST/INVARIANT: repository content, historical memory, tool discovery and subagent definitions are inputs to host policy—not sources of authority.

## Primary disposition

BORROW_PATTERN

## Secondary disposition candidates

- `REIMPLEMENT_IN_RUST`: several mechanisms already exist in Rust upstream, but SPARK should reimplement only where its own authoritative core needs a smaller contract or different dependencies.
- `REUSE_CODE`: possible for narrowly isolated Apache-2.0 modules only after dependency/license/API qualification; not established by this trace.
- `EXPERIMENT_NOW`: permission pipeline, bounded memory manifest + immutable evidence, stale-write guard and subagent admission are strong candidates for controlled SPARK experiments.

## Net effect on the emerging SPARK shape

grok-build strengthens the case that the middle layer should be **host-owned Rust actors/state machines**, with the model operating on a small typed interface.

It also adds two important planes to the earlier `find → learn → reauthorize → execute → evidence` shape:

1. a **trust gate before acquisition/loading** of repository-provided behavior; and
2. a **durable state/evidence pipeline** whose canonical files/state can survive crashes while compact manifests/indexes remain derived views.
