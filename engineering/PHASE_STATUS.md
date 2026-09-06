# S.P.A.R.K. Engineering Status

**Updated:** 2026-09-06  
**Branch:** `phase1-refoundation-v2`  
**Status basis:** repository history through pre-update HEAD `1aaa2fd65a3fdd328936c89c424ddfeb30bd5200`

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
