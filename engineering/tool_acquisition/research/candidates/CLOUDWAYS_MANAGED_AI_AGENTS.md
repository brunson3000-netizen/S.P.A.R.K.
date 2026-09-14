# Intake assessment — Cloudways Managed AI Agents

Date: 2026-09-14
Status: DOCUMENTATION_INTAKE_COMPLETE; no source harvest or runtime trial.
Primary disposition: ARCHIVE_REFERENCE.
Recommendation: include as a secondary managed-operations comparison; prioritize underlying agent repositories for any code-level study. This is not Cloudflare Agents, which is already a separate harvested source.

## What it contributes

Cloudways offers managed deployments of OpenClaw and Hermes. Its public product documentation describes per-customer dedicated infrastructure, managed patching, validated agent updates, backups, terminal access and customer-supplied model credentials. These are vendor-described features, not independently verified security properties. [1,2]

Its MCP guide documents connect, status sync, credential update and disconnect. The demonstrated connector lets an agent view and manage Cloudways servers/applications. This is a useful concrete operator workflow to compare with an MCI connection panel. It does not establish generic connector support or least-privilege enforcement for every action. [3]

Setup explicitly separates hosting from provider API usage and includes OpenRouter among model-provider options. Subscription compute is not bundled merely by hosting an agent. [4]

## Study questions with incremental value

1. Managed lifecycle: how does the operator see provisioned, running, updating, failed, restored and removed states? What is preserved through updates and restore?
2. Capability connection: distinguish configured credentials, successful authentication, discovered tools, permitted actions and verified revocation. A Connected label alone is insufficient evidence.
3. Runtime version versus saved memory/skills: can restore reintroduce revoked grants or stale credentials? Public evidence here does not answer this.
4. Isolation versus authority: separate customer-instance isolation from restrictions on an admitted agent's tools, callbacks and destinations.
5. Portable exit: determine what configuration, memory, skills, evidence and version information can be exported before treating the service as an operational dependency.

These questions add an operator-facing managed-service reference to the existing ToolHive, sandbox, gateway and observability studies. They do not require buying hosting.

## Harvestability and stronger source targets

No public source repository or reuse license for Cloudways' managed-agent control plane was identified in this bounded lookup. This is not proof that none exists. Treat Cloudways material as documentation/product evidence, not a pinned source-code acquisition.

The official README surfaces for OpenClaw and Nous Research's Hermes Agent were inspected. OpenClaw describes a gateway/control-plane architecture with channel, tool and provider integrations. Hermes describes persistent memory, reusable skills, scheduling, delegation and selectable execution backends. These are promising separate code-study candidates; README claims do not establish implementation correctness.

- https://github.com/openclaw/openclaw — observed README blob 02d592da6f9715de203af9d0dc41988ed0524b14
- https://github.com/NousResearch/hermes-agent — observed README blob c05112266746ff99a3326a62c38c33fbc08ecd23

Blob identifiers bind the inspected README surfaces only. No repository commit was frozen and neither project was added to source locks in this assessment. Before tracing either, resolve exact commit/license and inspect protecting tests. Hermes is particularly relevant to skill reuse and cross-session continuity; OpenClaw to gateway and multi-agent lifecycle comparison.

## Evidence limits and decision

Cloudways provides useful operational examples, but currently offers less inspectable mechanism evidence than already-pinned open-source runtimes. Keep it secondary. Do not infer a supervisor/coordinator implementation from managed hosting or marketing claims.

The product FAQ still mentions public preview while the GA announcement says preview ended; pricing/trial eligibility is not relied on here. No account signup, deployment, API-key entry, purchase, model call, installation or adoption performed. Existing harvest counts are unchanged. Active research queue remains governed by ACTIVE_TRACE_LEDGER.md.

## Primary sources inspected

[1] https://www.cloudways.com/en/managed-ai-agents.php
[2] https://www.cloudways.com/blog/cloudways-managed-ai-agents-is-generally-available/
[3] https://support.cloudways.com/en/articles/16522430-how-to-connect-mcp-server-to-your-managed-ai-agent-on-cloudways
[4] https://support.cloudways.com/en/articles/15799213-how-to-get-started-with-managed-ai-agents-on-cloudways

All four are rolling vendor pages retrieved 2026-09-14; no deployed Cloudways version was verified.
