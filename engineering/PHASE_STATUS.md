# S.P.A.R.K. Engineering Status

**Updated:** 2026-09-12
**Branch:** `phase1-refoundation-v2`  
**Status basis:** Operator acceptance of candidate `5ecc95c033bf3a7744bb7862bf959066e6561670`, independently reviewed at `e55b1da1c9049b1de58fcb06c65eea59939f2a57`

This file is a current-status summary. Detailed historical evidence remains in the phase reports, reviews, ADRs, Git history, and thread closeout records.

## Project operating state

**REACTIVATED BY OPERATOR FOR G.A.M.E. CONVERGENCE.**

The 2026-09-06 parking decision remains historical operating evidence, but the later Operator direction reactivates S.P.A.R.K. on the earliest safe path to a G.A.M.E.-usable S.P.A.R.K. device. Speed is a priority; deterministic correctness, writer/reviewer separation, project authority boundaries, and phase gates remain controlling.

The active cross-project operating record is:

`engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md`

The Operator has now accepted and frozen the independently reviewed V3-F01 Revision-2 correction. V3-F01 is CLOSED. S.P.A.R.K. Gate C1 is satisfied. The Operator authorized Gate C2 (Phase-2 implementation) on 2026-09-11; the Gate C2 candidate series ran through bounded revisions, a decay adjudication and write resolution, and the C2W-01 contract correction, which an independent review returned `GATE_C2_W01_CORRECTION_KEEP` on 2026-09-12. An adversarial campaign then ran against that exact pinned candidate and found no reproducible defect.

**Gate C2 acceptance is the Operator's decision and has not been given.** Nothing is promoted to production.

On 2026-09-12 the Operator authorized Phase-3 connection work within the safe-handoff objective, subject to existing predecessor gates (`engineering/SPARK_GAME_SAFE_HANDOFF_MISSION_2026-09-12.md`). Because Gate C2 acceptance is the controlling predecessor, only the working material protocol §6 permits has been produced: the integration-contract candidate and its test fixtures and prototype. No production device surface, no G.A.M.E. adapter, no contract freeze, no joint integration readiness and no G.A.M.E. state change is claimed.

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

No Phase-1 semantic identity, encoding, conflict, authority, or determinism invariant is reopened by the accepted Phase-2 correction.

## Phase 2

**ARCHITECTURE CORRECTION ACCEPTED AND FROZEN. V3-F01 CLOSED. GATE C2 CANDIDATE INDEPENDENTLY KEPT AND ADVERSARIALLY TESTED; ACCEPTANCE IS THE OPERATOR'S DECISION AND HAS NOT BEEN GIVEN.**

The controlling Operator acceptance is
`engineering/phase2/SPARK_PHASE_2_V3_F01_OPERATOR_ACCEPTANCE_FREEZE_2026-09-10.md`.
The controlling independent review is
`engineering/phase2/SPARK_PHASE_2_CODEX_V3_F01_REV2_INDEPENDENT_REVIEW_2026-09-10.md`
at `e55b1da1c9049b1de58fcb06c65eea59939f2a57`, verdict
`V3_F01_REV2_ACCEPTABLE_FOR_OPERATOR_ACCEPTANCE_AND_FREEZE`.
Candidate `5ecc95c033bf3a7744bb7862bf959066e6561670` and its complete supersession
stack are accepted under the review's two controlling pins:

- Replay epoch resets sharing a frozen frontier in ascending `reset_index`.
- Production P-7 through P-9 use bounded indexed lookup; prefer PX-1's three read-only
  accessors or an equivalent bounded private seam, without encoding, authority, identity
  or admission-semantic changes.

The frozen correction retains horizon expansion, cohort-local time, atomic least-slice
scheduler/obligation extraction, exact ActiveRequest resume, atomic finalization refusal,
request-bound dequeue and completed-versus-paused replay. All production Rust remains
unchanged; Phase 0 and Phase 1 remain closed. Fable architecture-challenge research remains
noncanonical and unmerged.

Fresh independent validation passes: 58 model checks, disposable Rust probes and negative
controls, 2,496 additional differential cases, 232 workspace tests, formatting, strict
lint, metadata, whitespace and five static Windows/Android target builds. Crash recovery,
durable snapshots/mailboxes, GAME integration, performance evidence and executable
Windows/Android parity remain future gates.

### Gate C2 — Phase-2 implementation candidate (writer pass, 2026-09-11)

The Operator authorized the Phase-2 implementation writer pass
(`engineering/phase2/SPARK_PHASE_2_GATE_C2_IMPLEMENTATION_MISSION_2026-09-11.md`). The
candidate is on `candidate/phase2-gate-c2-implementation-20260911`, based on production
`7e3a0aae069a8bf840e4dfb74221cdf2687b1db5`; its last code-and-test checkpoint is
`4997d56220000edef3a2b4cd987feff7900d2488`. Writer verdict:
`GATE_C2_CANDIDATE_READY_FOR_INDEPENDENT_REVIEW`.

- Report: `engineering/phase2/SPARK_PHASE_2_GATE_C2_IMPLEMENTATION_REPORT_2026-09-11.md`
- Evidence: `engineering/phase2/gate_c2_evidence_2026-09-11/`
- Independent review mission:
  `engineering/phase2/SPARK_PHASE_2_GATE_C2_INDEPENDENT_REVIEW_MISSION_2026-09-11.md`

It implements the frozen rule/effect runtime, cohort-granular scheduled processing, the
serialized request boundary with `F` and `ActiveRequest`, atomic P-1 … P-9 finalization
with both acceptance pins, and snapshot/restore/replay validation, with additive `spark-core`
surfaces only. Workspace: 320 tests passed, 0 failed, 0 ignored (232 inherited unmodified);
formatting, all-target clippy, strict core/engine lint, metadata, and five static
Windows/Android checks pass. The report lists partial oracle sub-cases, writer
interpretations for review, and one inherited performance limit (whole finalization is
`O(history)` through Phase-1 `submit_fence`). Nothing is accepted or promoted; production is
unchanged. Durable storage, crash recovery, GAME integration, performance evidence, and
executable Windows/Android parity remain later gates. Phase 3 remains unauthorized.

### Gate C2 — independent review and bounded revision (2026-09-11)

Independent Codex review of the writer candidate (`053d1dc`) at
`00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0` returned `GATE_C2_BOUNDED_REVISION_REQUIRED`
(C2-01 … C2-08; D-C2-3/5/7/11/13 revisions). The separated correction writer's bounded
revision is on `candidate/phase2-gate-c2-bounded-revision-20260911`, based exactly on that
review commit (code-and-test checkpoint `19930f3`; exact candidate hash in the publication
receipt). Writer verdict: `GATE_C2_BOUNDED_REVISION_READY_FOR_INDEPENDENT_REVIEW`.

- Mission: `engineering/phase2/SPARK_PHASE_2_GATE_C2_BOUNDED_REVISION_MISSION_2026-09-11.md`
- Report: `engineering/phase2/SPARK_PHASE_2_GATE_C2_BOUNDED_REVISION_REPORT_2026-09-11.md`
- Evidence: `engineering/phase2/gate_c2_bounded_revision_evidence_2026-09-11/`
- Independent review mission:
  `engineering/phase2/SPARK_PHASE_2_GATE_C2_REVISION_INDEPENDENT_REVIEW_MISSION_2026-09-11.md`

Every finding is corrected, the review's counterexamples are repository tests, and the
partial oracle rows are completed with negative controls (one further defect found and
fixed: obligation emissions now canonicalize by emission identity). Workspace: 374 tests
passed, 0 failed, 0 ignored (232 inherited unmodified); formatting, all-target clippy,
strict lint, metadata, five static targets and 45 external compile probes pass. Nine
representation choices are flagged for review. Nothing is accepted or promoted;
production is unchanged; Phase 3 remains unauthorized.

### Gate C2 — C2W-01 correction, independent KEEP and adversarial campaign (2026-09-12)

The pinned tested candidate is `67b877192cc78b75c6fbe60c69b5594dc10befe8` on
`candidate/phase2-gate-c2-w01-correction-20260912`, crate tree
`7907f4d729104fd5dbfd4adad46e66cf09aa13dd`. Independent review
`21a4fec666ca493f6ac4d5194ec6e9c5380ce40c` returned `GATE_C2_W01_CORRECTION_KEEP`; C2W-01 is
closed. Fresh validation on that exact tree: **427 workspace tests passed, 0 failed, 0
ignored**, with formatting, all-target/all-feature clippy, strict core/engine lint,
`cargo metadata`, whitespace and five static Windows/Android target builds passing. Two
nonblocking report-accuracy qualifications, C2W-RN01 and C2W-RN02, are recorded and
deliberately uncorrected.

The Operator's next authorized step
(`engineering/phase2/SPARK_PHASE_2_GATE_C2_OPERATOR_NEXT_AUTHORIZED_STEP_2026-09-12.md`) was
an adversarial campaign against that pinned build by independent testers who may not repair
product code. Four passes ran — the original LUNA and SONNET campaigns and two bounded
collection-only continuations of their unused window time — for roughly 32 minutes of a
nominal 60, finding **zero reproducible defects**.

The consolidated disposition is
`engineering/phase2/SPARK_PHASE_2_GATE_C2_CAMPAIGN_DISPOSITION_2026-09-12.md`, with evidence
in `engineering/phase2/gate_c2_campaign_disposition_evidence_2026-09-12/`. It disposes every
recorded observation against twelve discriminating diagnostics, closes three declared
coverage gaps, records one new nonblocking observation (H-OBS-01, a `test-support` seam that
reports success for a stage it did not perform), and **recommends** Gate C2 acceptance with
four recorded limitations. It accepts nothing.

### Historical Gate C1 chronology — superseded status statements

The dated subsections below preserve the writer/reviewer history. Their OPEN, pending,
not-accepted and not-frozen statements describe those earlier passes only. The current
acceptance and freeze above control; withdrawn proposals remain withdrawn.

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

#### V3-F01 serialized-boundary independent review (2026-09-10)

Independent Codex review at production baseline `e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8`
returned `V3_F01_BOUNDED_REVISION_REQUIRED`:

`engineering/phase2/SPARK_PHASE_2_CODEX_V3_F01_SERIALIZED_BOUNDARY_INDEPENDENT_REVIEW_2026-09-10.md`

The addendum's horizon expansion, cohort-local evaluation, least-slice extraction, `F`,
and two-digest structure remain the viable basis. One blocker remains: after a paced
pause, `F` is intentionally unchanged and the engine retains no active-request identity,
so substituting a different request can satisfy `h >= F` and evaluate backward relative
to already-processed work. The next writer must mechanically bind exact-request resume
across pause/restart (recommended: a minimal digest-committed `ActiveRequest`
discriminator), correct the carried oracle, and incorporate the Operator's section 8
amendment decision. V3-F01 remains open; no implementation or later phase is authorized.

#### V3-F01 ActiveRequest Operator decision (2026-09-10)

The Operator authorized the independent review's recommended minimal engine-owned
`ActiveRequest` discriminator and the associated Phase-2 v1 section 8 retained-state
amendment. The exact decision is recorded in:

`engineering/phase2/SPARK_PHASE_2_V3_F01_ACTIVE_REQUEST_OPERATOR_DECISION_2026-09-10.md`

The separated final correction writer mission is:

`engineering/phase2/SPARK_PHASE_2_V3_F01_FINAL_CORRECTION_WRITER_MISSION_2026-09-10.md`

This decision releases the correction writer pass and its independent review. V3-F01
remains open until that exact candidate is independently accepted and frozen. Phase-2
production Rust and Phase 3 remain unauthorized.

#### V3-F01 final correction candidate (separated writer pass, 2026-09-10)

The consolidated final correction package now exists on
`candidate/v3-f01-final-correction-20260910` and **awaits independent Codex review**,
with writer verdict `CANDIDATE_READY_FOR_INDEPENDENT_REVIEW`:

- `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_FINAL_2026-09-10.md`
- `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_ORACLE_V3_F01_FINAL_2026-09-10.md`
- `engineering/phase2/SPARK_PHASE_2_V3_F01_FINAL_CORRECTION_WRITER_REPORT_2026-09-10.md`
- `engineering/phase2/v3_f01_final_correction_model_2026-09-10/` (disposable model, 34 checks passing)

It defines the engine-owned `ActiveRequest` discriminator completely, commits it with `F`
in the stable-boundary digest, corrects V2-03 through V2-10 and the false oracle
assertions named by the independent reviews, and supersedes the earlier V3-F01 candidates
as a proposal while preserving them unchanged. **V3-F01 remains OPEN.** Nothing is
accepted, frozen, or canonical; Phase-2 production Rust and Phase 3 remain unauthorized;
Phase 1 remains closed; no Rust or Rust test was written or run by this pass.

#### V3-F01 final independent review (2026-09-10)

Independent Codex review of candidate `74d044ddd2b59c3c2eff558beeeeb515390dbe0f`
(parent `772e38da0d130d7a1225ba2eb60a75b3999336fe`) returned
`V3_F01_BOUNDED_REVISION_REQUIRED`:

`engineering/phase2/SPARK_PHASE_2_CODEX_V3_F01_FINAL_INDEPENDENT_REVIEW_2026-09-10.md`

Confirmed: stage-then-fence can leave a staged slot after source-sequence regression
refusal; ActiveRequest mismatch must not pop the actual active FIFO head; finalized
history plus `F` cannot reconstruct paused progress. The authorized ActiveRequest design
and earlier V2 corrections remain the basis for a bounded correction. Fresh validation:
34 model checks and 232 Rust tests passed; isolated probes confirmed the defects;
formatting, strict lint, and five Windows/Android static target checks passed.

The canonical separated-writer mission is
`engineering/phase2/SPARK_PHASE_2_V3_F01_BOUNDED_CORRECTION_WRITER_MISSION_REV2_2026-09-10.md`.
**No new Operator decision is required merely to run this bounded correction writer
pass.** V3-F01 remains OPEN, not accepted or frozen; Phase 1 remains closed; Phase-2
production implementation and Phase 3 remain unauthorized. The review branch is
`review/v3-f01-final-20260910`; production is not merged or advanced.

#### V3-F01 bounded correction candidate, Revision 2 (separated writer pass, 2026-09-10)

A Revision-2 correction package based on review commit
`5b19d7b5aa935b65fcfad1d8bda9a210d5a5684d` now exists on
`candidate/v3-f01-bounded-correction-rev2-20260910` and **awaits independent Codex HIGH
review**, with writer verdict `CANDIDATE_READY_FOR_INDEPENDENT_REVIEW`:

- `engineering/phase2/SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_REV2_2026-09-10.md`
- `engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_ORACLE_V3_F01_REV2_2026-09-10.md`
- `engineering/phase2/SPARK_PHASE_2_V3_F01_BOUNDED_CORRECTION_WRITER_REPORT_REV2_2026-09-10.md`
- `engineering/phase2/v3_f01_rev2_bounded_correction_evidence_2026-09-10/` (58-check model, disposable Phase-1 probe, validation logs)

It corrects FINAL-01 with an engine-internal single-command finalization (complete
read-only preflight, then an entailed stage-then-fence) whose every refusal leaves the whole
timeline byte-identical; FINAL-02 with request-bound dequeue and a fail-closed consumer
halt; FINAL-03 by scoping history-only replay to completed boundaries; and the five oracle
precision items. The FINAL candidate and its oracle are superseded as proposals and kept
unchanged. **V3-F01 remains OPEN.** Nothing is accepted, frozen, or canonical; Phase-2
production Rust and Phase 3 remain unauthorized; Phase 1 remains closed; no production Rust
was written.

## Phase 3

**CONNECTION WORK AUTHORIZED WITHIN THE SAFE-HANDOFF OBJECTIVE; BLOCKED ON GATE C2
ACCEPTANCE FOR EVERYTHING BEYOND PROTOCOL §6 WORKING MATERIAL.**

Produced on `candidate/spark-game-safe-handoff-20260912`:

- `engineering/phase3/SPARK_GAME_INTEGRATION_CONTRACT_V1_2026-09-12.md` — the Gate C3
  contract **candidate**, covering every surface the protocol enumerates. It recovers and
  cites G.A.M.E.'s own accepted direction (`GAME_PRODUCT_DESIGN_FOUNDATION_V1.md` §11.3,
  CANONICAL/Operator-approved), distinguishes settled decisions from engineering proposals,
  leaves embedded-versus-service, transport and serialization unselected, and states the
  durability gap rather than resolving it. **Not frozen.**
- `engineering/phase3/spark_game_contract_prototype_2026-09-12/` — a contained, reversible
  prototype permitted by protocol §6. It runs the protocol's canonical first causal sequence
  end to end across the real S.P.A.R.K. surface against a **fake** host: 17 tests pass, fmt
  and clippy with warnings denied pass, five cross-target checks pass **compile-only**. A
  fake host is preparatory evidence only, never proof of G.A.M.E. integration.
- `engineering/phase3/SPARK_GAME_CONTRACT_V1_SELF_CORRECTION_REPORT_2026-09-12.md` — six
  author-found corrections, including a blocking prototype defect that dropped every intent
  committed before a pause in a paced request.
- `engineering/phase3/handoff_independent_review_2026-09-12/` and
  `engineering/phase3/SPARK_GAME_HANDOFF_REVIEW_RESPONSE_2026-09-12.md` — a fresh independent
  adversarial review of the handoff artifacts (verdict
  `HANDOFF_REVIEW_BOUNDED_REVISION_REQUIRED`; 0 blockers, 1 major, 6 minor, 4 notes) and the
  writer's disposition of every finding. The major finding — a diagnostic that proved
  reproducibility while claiming to prove idempotence — is corrected and the failure is
  recorded rather than quietly replaced.

Blocked until Gate C2 acceptance: freezing the contract (Gate C3), implementing the
production device surface, implementing the G.A.M.E. adapter (Gate C4), and building the
minimal persistent boundary a restart-safe handoff needs.

## Cross-platform acceptance frontier

The production architecture remains one Rust canonical engine with platform deployment/adapters around it, not three platform-specific S.P.A.R.K. designs.

Windows/Android static compilation has been demonstrated in the existing evidence. Executable cross-platform canonical fixture replay and digest parity remain required before the project may claim full Windows/Linux/Android runtime equivalence.

## Durable operating constraints

These workflow rules do not change product architecture or phase authority:

- Important new programmer, auditor, research, review, or handoff artifacts use the `SPARK_` project prefix in their filenames. Historical artifacts are not renamed merely for cosmetic consistency if doing so would break references.
- Repository copies are canonical project evidence. `~/Downloads` copies are disposable transfer copies for handoff and may be deleted after upload/review.
- The canonical project-wide duty to preserve conversation material worth future reliance before it can be lost is recorded in `engineering/SPARK_DURABLE_CONVERSATION_CAPTURE_PRINCIPLE.md`.
- The September 9 Fable architecture research closeout is discoverable through `engineering/SPARK_FABLE_RESEARCH_POINTER_2026-09-09.md`; the research branch and proposals remain non-adopted.
- Active external-agent mission text is treated as static/canonical for that mission. Do not casually regenerate an "equivalent" prompt. Any change must be explicitly labeled as a revision.
- Operator handoff steps are given in execution order: open Terminal; `cd` to the repository; launch plain `claude` or `codex`; paste the canonical mission; select the stated model/effort if needed; return the requested artifacts. Do not default to giant combined terminal commands or extra launch flags.
- Preserve writer/reviewer separation. Use Fable sparingly and only for genuinely critical foundational architecture moments or non-converging foundational defect classes. Prefer Opus HIGH for difficult bounded architecture/implementation corrections, Sonnet for mature/routine implementation, and Codex HIGH for independent adversarial review.
- NVIDIA NIM is development/research compute only and gains no canonical runtime or project authority from availability. Credentials/API keys are not repository content.
- Within bounded concurrency and provider-health limits, useful NIM work should maximize diversity of task, role, model, and failure hypothesis rather than redundant repetition. Independent workers do not see each other's results before synthesis; raw responses and model/task/provenance metadata are preserved; telemetry should measure disagreement, unique and duplicate findings, useful defects/tests, latency, failures/rate limits, and whether additional workers changed the primary conclusion.
- Do not rerun the completed NIM benchmark study merely to increase panel size.

The final thread handoff is recorded at `engineering/SPARK_THREAD_FINAL_CLOSEOUT_2026-09-06.md`.
