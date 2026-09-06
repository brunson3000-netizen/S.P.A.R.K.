# S.P.A.R.K. Thread Closeout — Operational Addendum

**Date:** 2026-09-06  
**Branch:** `phase1-refoundation-v2`  
**Supplements:** `engineering/SPARK_THREAD_CLOSEOUT_2026-09-06.md`  
**Purpose:** preserve operator workflow instructions and shared-compute handoff constraints that are operationally material but do not alter product architecture or phase authority.

## 1. Relationship to the primary closeout

The primary closeout remains controlling for the product/architecture reconciliation and current engineering frontier.

This addendum captures later operational instructions from the same long-running engineering thread so a successor coordinator does not have to recover them from chat.

It does **not** reopen Phase 0 or Phase 1, does **not** freeze Phase-2 architecture, does **not** authorize Phase-2 implementation, and does **not** authorize Phase 3.

## 2. Engineering artifact naming

Important new programmer, auditor, research, review, evidence, and handoff artifacts should include the project name and use the `SPARK_` prefix.

Examples:

- `SPARK_PHASE_2_FABLE_ARCHITECTURE_REVIEW_YYYY-MM-DD.md`
- `SPARK_PHASE_2_OPUS_IMPLEMENTATION_REPORT_YYYY-MM-DD.md`
- `SPARK_PHASE_2_CODEX_INDEPENDENT_REVIEW_YYYY-MM-DD.md`

Do not rename historical evidence solely for cosmetic consistency if that would break references or obscure provenance.

## 3. Repository and Downloads handling

Repository copies are canonical project evidence.

Downloads copies are disposable transfer artifacts used so the operator can easily hand a report/package to ChatGPT or another reviewer. They may be deleted after the handoff.

Future agent missions that create a material report should, where practical:

1. place the canonical copy in the repository/evidence chain;
2. commit it when appropriate;
3. place an identical transfer copy in `~/Downloads`;
4. never treat Downloads as project storage.

## 4. Operator-attention reduction

The operator should not have to babysit routine engineering actions.

When assigning work to Claude Code, Codex, Fable, Opus, Sonnet, or another external agent, the coordinator should automatically provide:

- WHO / model;
- effort level;
- launch location;
- exact instruction;
- expected output file;
- what the operator must return.

Prefer a single paste-ready terminal command for Codex or similar operations when safe. Routine in-scope repository reads, tests, local artifact generation, bounded refactors, and evidence packaging should be pre-authorized in the mission rather than prompting the operator step-by-step.

This is workflow discipline, not an expansion of agent authority.

## 5. Writer / reviewer separation and convergence rule

Preserve independent roles. The current successful pattern for difficult work is:

```text
architecture specialization
-> implementation writer
-> independent adversarial reviewer
```

The recent S.P.A.R.K. workflow commonly used Fable for architecture, Opus for deep implementation, and Codex for adversarial review. Those model choices are time-sensitive engineering tactics, not permanent product architecture.

When the same foundational defect class survives a bounded correction, stop ordinary patching and return to architecture/process review rather than starting an indefinite correction loop.

## 6. NVIDIA NIM development compute

The operator confirmed that NVIDIA NIM compute is available to S.P.A.R.K. through the terminal, the API key already exists, and basic access has been tested successfully.

Operational constraints:

- credentials/API keys are not repository content;
- NIM availability does not create canonical runtime or project authority;
- use is for development/research/engineering assistance unless a later explicit product decision changes that boundary;
- a shared cross-project NIM integration/workflow is being designed through the S.W.A.R.M. project;
- S.P.A.R.K. should adopt or adapt that shared result when available rather than independently creating a conflicting permanent harness;
- current S.P.A.R.K. architecture work should not block on that integration.

The earlier brainstorming idea of broad multi-model NIM adversarial panels remains non-operative until the shared S.W.A.R.M. integration path is available or the operator separately authorizes a S.P.A.R.K.-specific workflow.

## 7. Current truthful handoff frontier

The controlling repository status is `engineering/PHASE_STATUS.md`.

At this addendum:

- Phase 0: CLOSED / PASS.
- Phase 1: CLOSED.
- Phase 2 architecture v3: one bounded blocker remains, V3-F01 (cohort extraction / pre-wave digest inconsistency).
- Phase-2 implementation: NOT AUTHORIZED.
- Phase 3: NOT AUTHORIZED.
- Windows/Android executable canonical replay and digest parity: not yet proven; current cross-platform evidence is static compilation only.

The next engineering action is the bounded V3-F01 architecture correction and independent acceptance/freeze. Only after that gate may Phase-2 implementation authority be reconsidered.

## 8. Closeout status

This addendum closes the remaining operational handoff gaps from the long-running takeover/Phase-1 thread without changing the project's current architecture frontier.

**THREAD OPERATIONAL HANDOFF: COMPLETE**
