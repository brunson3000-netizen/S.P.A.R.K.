# S.P.A.R.K. Engineering Status

**Updated:** 2026-09-06  
**Branch:** `phase1-refoundation-v2`  
**Status basis:** repository history through pre-update HEAD `8398324e7f8943a8e19789537b3d38d9ded8ff25`

This file is a current-status summary. Detailed historical evidence remains in the phase reports, reviews, ADRs, and Git history.

## Phase 0

**CLOSED / PASS.**

The controlling architecture baseline is `engineering/phase0/CONTROLLING_BLUEPRINT_v0.2.md` plus the approved Phase-0 ADRs.

Frozen points relevant to the current handoff include:

- Rust is the committed production language.
- One reusable canonical engine serves Windows, Linux, and Android.
- Standalone-service and embedded forms must preserve the same canonical semantics.
- Platform-specific facilities remain outside canonical causal evaluation.
- Deterministic clock/RNG/commit ordering, authority boundaries, profile validation, and persistence/versioning contracts are already defined by Phase-0 ADRs.

## Phase 1

**CLOSED.**

Independent Codex final closure review returned `PHASE_1_CLOSED` at reviewed HEAD `20a1c66` with no MAJOR or BLOCKER remaining.

The final closure lineage records 232 passing tests and passing fmt/clippy/strict-lint/metadata gates. Windows and Android target checks passed as static `cargo check` coverage. Those cross-platform checks are compile/static evidence only; they are not executable runtime or digest-parity proof.

Controlling evidence includes:

- `engineering/phase1/PHASE_1_FINAL_CLOSURE_IMPLEMENTATION_REPORT_2026-08-26.md`
- `engineering/phase1/SPARK_PHASE_1_CODEX_FINAL_CLOSURE_REVIEW_2026-08-26.md`

No Phase-1 semantic identity, encoding, conflict, authority, or determinism invariant is reopened by the current Phase-2 blocker.

## Phase 2

**ARCHITECTURE V3 REQUIRES ONE BOUNDED REVISION. IMPLEMENTATION NOT AUTHORIZED.**

Phase-2 architecture went through the original freeze, Codex adversarial review, v2 correction, Codex v2 rereview, v3 correction, provenance correction, and final bounded Codex confirmation.

The latest controlling independent result is:

`PHASE_2_ARCHITECTURE_V3_REVISE`

at pre-update HEAD `1aaa2fd65a3fdd328936c89c424ddfeb30bd5200`, recorded in:

`engineering/phase2/SPARK_PHASE_2_CODEX_FINAL_ARCHITECTURE_V3_CONFIRMATION_2026-08-26.md`

All previously repaired Phase-2 areas remain closed. Exactly one known blocker remains:

### V3-F01 — cohort extraction / pre-wave digest inconsistency

The v3 pacing and catch-up equivalence rules require each admitted `(due_time, profile_id)` cohort to leave canonical scheduler state immediately before that cohort's pre-wave engine digest is captured, while deferred cohorts remain resident and unchanged. The inherited Phase-1 `Scheduler::drain_due(now)` removes the whole due prefix instead. Therefore equivalent pacing/catch-up partitions can produce different scheduler, pre-wave engine, and `effect_batch` digests.

The independent review identifies the smallest safe bounded correction:

- add an internal deterministic cohort-granular due-work extraction surface;
- remove exactly the admitted current cohort immediately before its pre-wave digest;
- leave later/deferred scheduler slots resident and byte-identical;
- preserve existing Phase-1 `WorkKey`, slot, conflict, ordering, encoding, digest, `schedule`, and `drain_due` semantics;
- correct the v3 acceptance oracle to permit that additive extraction surface and prove paced/unbudgeted and catch-up partition equivalence at every cohort boundary.

This is an architecture correction only. It introduces no new causal primitive and does not authorize production Rust.

**Phase-2 implementation writer remains unreleased until V3-F01 is frozen and independently accepted.**

## Phase 3

**NOT AUTHORIZED.**

## Cross-platform acceptance frontier

The production architecture remains one Rust canonical engine with platform deployment/adapters around it, not three platform-specific S.P.A.R.K. designs.

Windows/Android static compilation has been demonstrated in the existing evidence. Executable cross-platform canonical fixture replay and digest parity remain required before the project may claim full Windows/Linux/Android runtime equivalence.

## Operational handoff constraints

These workflow rules do not change product architecture or phase authority, but they are durable operating constraints for successor engineering threads:

- Important new programmer, auditor, research, review, or handoff artifacts use the `SPARK_` project prefix in their filenames. Historical artifacts are not renamed merely for cosmetic consistency if doing so would break references.
- Repository copies are canonical project evidence. `~/Downloads` copies are disposable transfer copies for handoff and may be deleted after upload/review.
- External-agent handoffs should state the agent/model, effort level, launch location, exact instruction, expected output artifact, and what the operator must return. Prefer a single paste-ready terminal command when safe, and authorize routine in-scope reads/tests/local artifact work so the operator is not asked to babysit each step.
- Preserve writer/reviewer separation. The current successful pattern for difficult work is architecture specialization before implementation and independent adversarial review afterward; model-specific capability estimates are time-sensitive workflow choices, not frozen product architecture.
- NVIDIA NIM development compute is available to the project and has already been tested by the operator. Credentials/API keys are not repository content. A shared cross-project NIM integration/workflow is being developed through S.W.A.R.M.; S.P.A.R.K. should adopt or adapt that shared result rather than independently invent a conflicting permanent harness. NIM is development/research compute only and does not gain canonical runtime or project authority from availability.
- NIM integration is not a blocker for the current V3-F01 architecture correction. Continue the bounded Phase-2 architecture closure from the controlling repository evidence while the shared compute workflow matures separately.

The thread-level operational addendum is recorded at `engineering/SPARK_THREAD_CLOSEOUT_OPERATIONAL_ADDENDUM_2026-09-06.md`.