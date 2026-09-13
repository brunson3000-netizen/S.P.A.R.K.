# Secondary Wave 1 Source Ledger

Date: 2026-09-13
State: `PINNED_SOURCE_ONLY`
Activation: `NONE`

This ledger explains why each secondary source is in custody. Exact revisions are authoritative in `SECONDARY_HARVEST_LOCK.json`.

| Source | Lane | Harvest reason | Current posture |
|---|---|---|---|
| ACP spec | tool fabric | Standard agent↔client/session protocol shape | compare for MCI/window transport; no authority semantics imported |
| ACP Rust SDK | tool fabric | Rust implementation of ACP | direct-reuse candidate below SPARK authority after qualification |
| wasmCloud | capability runtime | WIT/component and host-mediated capabilities | mine runtime/capability patterns; do not adopt control plane wholesale |
| Nushell | structured CLI | Typed values, pipelines, plugin contracts | mine agent-native structured-output and discovery patterns |
| OTel Collector | evidence | Receiver→processor→exporter pipeline | candidate evidence-export sidecar/reference, never canonical truth |
| Temporal | orchestration | Durable workflow/replay/retry semantics | mine recovery/idempotency patterns; likely too heavy for core by default |
| OpenHands SDK | agent runtime | Agent server, workspaces, events, tool/UI boundaries | compare agent-service separation and workspace abstractions |
| LangGraph | orchestration | State graph/checkpoint workflow | compare graph semantics; reference unless a concrete runtime need wins |
| n8n | integration | Broad external workflow/connector model | mine connector/execution patterns; not a governance engine |
| Microsoft Agent Framework | agent runtime | Current Microsoft successor architecture | primary Microsoft multi-agent/supervision reference |
| AutoGen | historical agent architecture | AutoGen and Magentic-One design history | reference only by default; upstream maintenance mode |
| Cedar | policy evaluator | Deterministic authorization evaluator | evaluator candidate only; host owns live authority and fail-closed behavior |
| Cloudflare Agents | containment/agent runtime | Gatekeeper/Gadgets, isolates, durable agent state | mine isolation and mediation patterns |
| OPA | policy evaluator | Mature Rego evaluator/control-policy model | compare against Cedar; no imported authority |
| Claude Code | agent product mechanics | Teams/subagents/tasks/permissions/context mechanics | source-mine UX and coordination patterns |
| Gemini CLI | agent product mechanics | Sandbox/policy/context and filesystem isolation | source-mine containment; pinned head includes sandbox boundary hardening |
| Google ADK Python | agent runtime | Workflow agents and graph validation | compare deterministic workflow-agent contracts |
| OpenAI Codex | agent product mechanics | Sandbox/approval/workspace/session/subagent mechanics | source-mine trusted-workspace and approval patterns |
| OpenAI Agents Python | agent runtime | Handoffs, tools, MCP, tracing | compare lightweight runtime and tool/handoff contracts |
| SWE-agent | agent-computer interface | Purpose-built software-engineering interface | mine narrow ACI/tool-surface patterns |
| Cosign | provenance | Signing and verification mechanics | conditional distribution-hardening candidate |
| in-toto | provenance | Supply-chain layouts and attestations | conditional provenance model reference |
| SLSA Verifier | provenance | Provenance verification implementation | reference only by default; upstream maintenance status |

## Cross-source conclusions to test during qualification

1. **Transport is not authority.** MCP/ACP/session protocols may carry requests and observations; SPARK or a consumer must own grants, effects, and canonical state outside them.
2. **Structured state beats narrative state.** Nushell, workflow engines, and agent runtimes repeatedly expose typed/inspectable state rather than relying on chat history alone.
3. **Mediation should be a separate layer.** Cloudflare Gatekeeper-style patterns, Cedar/OPA, and sandboxed runtimes reinforce a separate admission/effect boundary rather than putting security in prompts.
4. **Durability is valuable but expensive.** Temporal/OpenHands/LangGraph/n8n solve useful persistence/recovery problems, but importing a whole runtime is not justified until a specific SPARK capability requires it.
5. **External telemetry is not truth.** OTel-style pipelines are useful export mechanisms; SPARK's own evidence/canonical state must remain authoritative.
6. **Corporate agent products are reference mines, not constitutions.** Claude Code, Gemini CLI, Codex, ADK, and Agents SDK expose useful mechanics, but their assumptions about users, permissions, and product boundaries are not automatically SPARK's.
7. **Provenance becomes load-bearing at distribution time.** Cosign/in-toto/SLSA matter most if SPARK builds, vendors, or distributes acquired tools; source pinning alone does not justify adopting the entire supply-chain stack.

## Explicit non-conclusions

Pinning does not mean a tool is approved, secure enough, license-cleared for code copying, buildable on every SPARK platform, or appropriate for a consumer. Those are H2 qualification questions.
