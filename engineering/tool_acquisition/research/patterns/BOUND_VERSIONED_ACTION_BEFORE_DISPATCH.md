# Bound, versioned action before dispatch

Status: PROPOSED TRANSFER PATTERN; NOT IMPLEMENTED OR CONSUMER-ACCEPTED.

## Problem

A remembered approval or a resumable agent task can outlive the specific tool, resource, policy or process that made it meaningful. Framework-level correlation is useful, but a same-name tool or replacement coordinator must not accidentally acquire a stale grant.

## Evidence and mechanism

[Cedar](../sources/CEDAR_MCI_REVIEW_2026-09-14.md) separates decision and diagnostics. [Codex](../sources/OPENAI_CODEX_MCI_SCREEN_2026-09-14.md) centralizes approval, sandbox selection and attempts. [ADK](../sources/GOOGLE_ADK_PYTHON_MCI_SCREEN_2026-09-14.md) matches recorded calls and arguments. [Microsoft](../sources/MICROSOFT_AGENT_FRAMEWORK_MCI_SCREEN_2026-09-14.md) supplies server/argument-scoped standing approvals, but intentionally permits same-name tool replacement on resume. [Paperclip](../sources/PAPERCLIP_MCI_REVIEW_2026-09-14.md) uses conditional state transitions and operational budget-stop wiring. [NemoClaw](../sources/NEMOCLAW_MCI_REVIEW_2026-09-14.md) distinguishes applied, rejected and ambiguous policy changes. Each source record binds the exact upstream revision and modules.

Proposed consumer-owned sequence:

1. Resolve authenticated subject and current task/attempt identity outside model input.
2. Resolve canonical capability, executable/schema revision, exact resource and settled typed arguments.
3. Read current policy/grants; evaluate decision and diagnostics according to the accepted consumer error contract.
4. Match any needed approval to those exact values, expiry and active policy epoch. Changed values require a fresh decision.
5. Reserve bounded resources atomically where the consumer requires hard caps; an estimated price or observed-spend counter is insufficient.
6. Dispatch through the required enforcement backend and record actual applied bounds and attempt identity.
7. Commit independent outcome/evidence; on unknown effect, retain an explicit reconciliation state rather than retrying blindly.

This sequence is a research proposal, not a new required authority hierarchy. Independent references support individual properties; no studied product was shown to implement this entire contract for MCI.

## Location and dependencies

Belongs in the consuming host's deterministic admission/execution boundary. MCI presents the request, reason, policy revision and observed status through provider-neutral contracts. The UI, worker, protocol SDK and telemetry exporter do not own the grant.

Dependencies: authenticated identity binding, canonical capability registry, immutable policy revisions, durable task/attempt state, optional reservation store, sandbox adapter and canonical evidence storage. A small Rust implementation may fit the acquisition lane; exact placement/language belongs to the consumer's accepted architecture.

## Benefit and risks

Benefit: explicit bindings make stale approval, confused identity and unknown execution states inspectable and falsifiable. Risks: incomplete ingress coverage, TOCTOU between check and effect, serialization ambiguity, mutable policy/entity data, reservation races and a false assumption that cancellation means physical stop. A single shared orchestration helper only helps if every consequential ingress actually uses it.

## Qualification

Use the [assessment test matrix](../comparison/MCI_COMPREHENSIVE_ASSESSMENT_2026-09-14.md#testing-and-qualification). Required observations include zero backend calls on denied/stale requests, same-name tool upgrade invalidation, child fencing, reservation races, crash reconciliation and independently retained evidence. Tests in this proposed combined contract are NOT RUN. Individual upstream tests were inspected only, except the separate ctx-zip adverse reproduction.

Finding classes: PATTERN; TEST/INVARIANT; DETERMINIZATION_CANDIDATE. Confidence: strong for the need to separate bindings and stages; integration effectiveness unproven.

## Primary disposition

BORROW_PATTERN
