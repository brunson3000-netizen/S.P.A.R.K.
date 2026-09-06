# S.P.A.R.K. Engineering Status

**Updated:** 2026-09-06  
**Branch:** `phase1-refoundation-v2`  
**Status basis:** repository history through pre-convergence HEAD `5d45b29b1664018d44ca7f1130f703f95906969d`

This file is a current-status summary. Detailed historical evidence remains in the phase reports, reviews, ADRs, Git history, and thread closeout records.

## Project operating state

**REACTIVATED BY OPERATOR FOR G.A.M.E. CONVERGENCE.**

The 2026-09-06 parking decision remains historical operating evidence, but the later Operator direction reactivates S.P.A.R.K. on the earliest safe path to a G.A.M.E.-usable S.P.A.R.K. device. Speed is a priority; deterministic correctness, writer/reviewer separation, project authority boundaries, and phase gates remain controlling.

The active cross-project operating record is:

`engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md`

Reactivation does not resolve V3-F01, authorize Phase-2 implementation, or authorize Phase 3. The immediate active action is the bounded V3-F01 architecture correction and independent acceptance/freeze. G.A.M.E. integration requirements and test fixtures may be prepared without prematurely implementing Phase-3 product protocol/service code.

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

**ARCHITECTURE V3 REQUIRES ONE BOUNDED REVISION. IMPLEMENTATION NOT AUTHORIZED. CONVERGENCE ACTIVE.**

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

**The correction is now the active convergence frontier. Phase-2 implementation writer remains unreleased until the correction is frozen and independently accepted.**

#### V3-F01 correction candidate (Gate C1 writer pass, 2026-09-06)

A bounded architecture-correction candidate and a corrected acceptance-test-oracle
candidate now exist and **await independent Codex review**:

- `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_2026-09-06.md`
- `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_2026-09-06.md`
- `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_2026-09-06.md`

**V3-F01 remains OPEN.** The candidate is not accepted, not frozen, and not canonical;
the controlling independent verdict is still `PHASE_2_ARCHITECTURE_V3_REVISE`. Phase-2
implementation remains unauthorized, Phase 3 remains unauthorized, Phase 1 remains
closed, and no production Rust was written or changed by the writer pass.

#### V3-F01 independent Gate C1 review (2026-09-06)

Independent Codex review of writer commit
`03daa82032cecc6ed84407b440bc9eab06cbcd69` returned
`V3_F01_FOUNDATIONAL_REVIEW_REQUIRED`:

`engineering/phase2/SPARK_PHASE_2_CODEX_V3_F01_INDEPENDENT_REVIEW_2026-09-06.md`

The candidate is **not accepted or frozen**. V3-F01 remains open; Phase-2 implementation
and Phase 3 remain unauthorized. The next permitted step is the bounded foundational
logical-time/catch-up adjudication and a separated correction writer/reviewer cycle
specified by the independent review. Phase 1 remains closed.

#### V3-F01 foundational logical-time adjudication candidate (Gate C1 Fable pass, 2026-09-06)

The bounded foundational adjudication required by the independent review now exists as a
**candidate** and returned `FOUNDATIONAL_TIME_ADJUDICATION_CANDIDATE_READY`:

- `engineering/phase2/SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md`
- `engineering/phase2/SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_REPORT_2026-09-06.md`

It chooses horizon expansion with per-barrier canonical time: one host catch-up call
expands into one canonical logical-time barrier per resident due time inside the
horizon, every scheduled cohort evaluates with `now` equal to its own `due_time`, and
created work is eligible at its own barrier. **V3-F01 remains OPEN.** The adjudication is
not accepted, not frozen, and not canonical; the controlling verdicts are unchanged. The
next permitted step is a separated Opus V3-F01 correction writer pass under the
candidate's downstream constraints, then independent Codex review. Phase-2
implementation and Phase 3 remain unauthorized; Phase 1 remains closed.

The adjudication is recorded at commit `379f8dc355125e6bca09801e4a66b0b35ec9720c`
(parent `513c3982d5b9cd14f86ec07369662f3a178f1d95`); its own report omitted that hash.

#### V3-F01 correction candidate V2 (Gate C1 second writer pass, 2026-09-06)

A separated second-pass correction candidate and acceptance matrix now exist and **await
independent Codex review**, with writer verdict
`CANDIDATE_READY_FOR_INDEPENDENT_REVIEW`:

- `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_V2_2026-09-06.md`
- `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_V3_F01_CORRECTION_CANDIDATE_V2_2026-09-06.md`
- `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_WRITER_REPORT_V2_2026-09-06.md`

It builds on the foundational time adjudication as a **proposed** basis and resolves
review findings F-01 through F-07 together: a least-due compare-and-take with an explicit
eligibility horizon replaces the owned selection token; conflicted slots are extracted
with their operational slice but belong to neither the executable cohort identity nor the
pacing budget; the new canonical report fields are withdrawn in favour of the existing
`test-support` observation seam; partition equivalence is stated with preconditions,
separating completed-horizon from equal-prefix equivalence; and command effective-time
ordering is bounded by two supported-history conditions with a fail-closed refusal, while
the ADR-0003 staging question is left open.

**V3-F01 remains OPEN.** The first-pass candidate, its matrix, the independent review, and
the adjudication are preserved unchanged. Nothing here is accepted, frozen, or canonical;
the controlling verdicts are unchanged. Phase-2 implementation and Phase 3 remain
unauthorized; Phase 1 remains closed; no production Rust or test was written or executed.

#### V3-F01 correction candidate V2 independent Gate C1 review (2026-09-06)

Independent Codex review of writer commit
`983a01fd6807b8c627c97d2673a1f40c54cc059c` returned:

`V3_F01_FOUNDATIONAL_REVIEW_REQUIRED`

`engineering/phase2/SPARK_PHASE_2_CODEX_V3_F01_V2_INDEPENDENT_REVIEW_2026-09-06.md`

The scheduled horizon-expansion and least-slice compare-and-take core remain a viable
basis, but the V2 candidate is **not accepted or frozen**. The command-time frontier and
SH-1/SH-2 authority require focused foundational adjudication, and the acceptance oracle
contains false budget, post-extraction digest, replay, and rejection assertions. V3-F01
remains open; Phase-2 implementation and Phase 3 remain unauthorized; Phase 1 remains
closed. No Operator acceptance or G.A.M.E. change is claimed.

#### V3-F01 command-time adjudication candidate (Gate C1 Fable pass, 2026-09-06)

The focused command-time adjudication required by the V2 review now exists as a
**candidate** and returned `COMMAND_TIME_ADJUDICATION_CANDIDATE_READY`:

- `engineering/phase2/SPARK_PHASE_2_V3_F01_COMMAND_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md`
- `engineering/phase2/SPARK_PHASE_2_V3_F01_COMMAND_TIME_ADJUDICATION_REPORT_2026-09-06.md`

It withdraws the V2 frontier `Φ`, SH-1/SH-2, and R-8; adopts unified time-ordered
processing in which finalized commands are pending inputs consumed by the catch-up call
after every earlier-time scheduled slice; gates command admission on the host horizon
and the finalized time ceiling at staging and fence, non-canonically and retryably; and
defines exactly two added state items, the horizon frontier and a per-timeline execution
cursor. Three Operator amendments (engine-digest inclusion of both, and an ADR-0003
admission addendum) are **proposed, not enacted**. **V3-F01 remains OPEN.** Nothing is
accepted, frozen, or canonical; controlling verdicts are unchanged; Phase-2
implementation and Phase 3 remain unauthorized; Phase 1 remains closed.

#### V3-F01 serialized request boundary addendum (Gate C1 Fable pass, 2026-09-06)

Reconciles the command-time candidate with the Operator's serialized-request direction
and returned `SERIALIZED_REQUEST_BOUNDARY_ADDENDUM_READY`:

- `engineering/phase2/SPARK_PHASE_2_V3_F01_SERIALIZED_REQUEST_BOUNDARY_ADDENDUM_2026-09-06.md`
- `engineering/phase2/SPARK_PHASE_2_V3_F01_SERIALIZED_REQUEST_BOUNDARY_REPORT_2026-09-06.md`
- `engineering/phase2/serialized_request_model_2026-09-06/` (disposable executable model, all checks passing)

A bounded mailbox feeds one consumer; one request is active until its defined horizon
completes; budget exhaustion pauses it; commands finalize and execute atomically at
completion; a request starts only if its horizon is at least the last completed horizon
`F`. The unified pending-command loop, cursor `X`, and amendments A-1…A-3 are withdrawn;
`F` is the only added state, committed in a stable-boundary digest and excluded from the
per-cohort engine digest. One v1 §8 amendment is **proposed, not enacted**. **V3-F01
remains OPEN.** Nothing is accepted, frozen, or canonical; Phase-2 implementation and
Phase 3 remain unauthorized; Phase 1 remains closed.

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
