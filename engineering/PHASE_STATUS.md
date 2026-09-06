# S.P.A.R.K. Engineering Status

**Updated:** 2026-09-06  
**Branch:** `phase1-refoundation-v2`  
**Status basis:** repository history through pre-update HEAD `d88806938eab74a6b468c12eef92cedeca32a666`

This file is a current-status summary. Detailed historical evidence remains in the phase reports, reviews, ADRs, Git history, and thread closeout records.

## Project operating state

**PARKED BY OPERATOR.**

Normal S.P.A.R.K. production/Phase-2 engineering is on HOLD to conserve scarce premium-model capacity until shared NVIDIA NIM development resources are better able to absorb engineering load.

Parking does not change any frozen architecture or phase result. In particular, it does not resolve the remaining Phase-2 architecture blocker and does not authorize implementation.

While parked, S.P.A.R.K. work may proceed only when it independently advances the shared NIM sub-project goals of:

1. improving reusable tools for practical NIM engineering utilization; and/or
2. developing evidence-backed architecture for amplifying heterogeneous NIM work in ways reusable by S.W.A.R.M. and the MCI.

Such work must not be used as a pretext to continue ordinary S.P.A.R.K. Phase-2 implementation. S.P.A.R.K. may serve as a real workload laboratory for ontology/semantic tooling, invariant-to-adversarial-test tooling, complexity pruning, engineering challenge corpora, independent panel architecture, and marginal-worker-value measurement.

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

**ARCHITECTURE V3 REQUIRES ONE BOUNDED REVISION. IMPLEMENTATION NOT AUTHORIZED. PROJECT PARKED.**

Phase-2 architecture went through the original freeze, Codex adversarial review, v2 correction, Codex v2 rereview, v3 correction, provenance correction, and final bounded Codex confirmation.

The latest controlling independent result is:

`PHASE_2_ARCHITECTURE_V3_REVISE`

recorded in:

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

**The correction is not active while the project is parked. Phase-2 implementation writer remains unreleased.**

## Phase 3

**NOT AUTHORIZED.**

## Cross-platform acceptance frontier

The production architecture remains one Rust canonical engine with platform deployment/adapters around it, not three platform-specific S.P.A.R.K. designs.

Windows/Android static compilation has been demonstrated in the existing evidence. Executable cross-platform canonical fixture replay and digest parity remain required before the project may claim full Windows/Linux/Android runtime equivalence.

## Durable operating constraints

These workflow rules do not change product architecture or phase authority:

- Important new programmer, auditor, research, review, or handoff artifacts use the `SPARK_` project prefix in their filenames. Historical artifacts are not renamed merely for cosmetic consistency if doing so would break references.
- Repository copies are canonical project evidence. `~/Downloads` copies are disposable transfer copies for handoff and may be deleted after upload/review.
- Active external-agent mission text is treated as static/canonical for that mission. Do not casually regenerate an "equivalent" prompt. Any change must be explicitly labeled as a revision.
- Operator handoff steps are given in execution order: open Terminal; `cd` to the repository; launch plain `claude` or `codex`; paste the canonical mission; select the stated model/effort if needed; return the requested artifacts. Do not default to giant combined terminal commands or extra launch flags.
- Preserve writer/reviewer separation. Use Fable sparingly and only for genuinely critical foundational architecture moments or non-converging foundational defect classes. Prefer Opus HIGH for difficult bounded architecture/implementation corrections, Sonnet for mature/routine implementation, and Codex HIGH for independent adversarial review.
- NVIDIA NIM is development/research compute only and gains no canonical runtime or project authority from availability. Credentials/API keys are not repository content.
- Within bounded concurrency and provider-health limits, useful NIM work should maximize diversity of task, role, model, and failure hypothesis rather than redundant repetition. Independent workers do not see each other's results before synthesis; raw responses and model/task/provenance metadata are preserved; telemetry should measure disagreement, unique and duplicate findings, useful defects/tests, latency, failures/rate limits, and whether additional workers changed the primary conclusion.
- Do not rerun the completed NIM benchmark study merely to increase panel size.

The final thread handoff is recorded at `engineering/SPARK_THREAD_FINAL_CLOSEOUT_2026-09-06.md`.
