# S.P.A.R.K. V3-F01 — Operator acceptance and architecture freeze

**Date:** 2026-09-10
**Authority:** Explicit Operator instruction in the acceptance/publication session
**Status:** CANONICAL — ACCEPTED AND FROZEN — V3-F01 CLOSED
**Scope:** Architecture and acceptance oracle only

## Operator decision

The Operator explicitly authorized publication of the independent Revision-2 review,
acceptance of candidate `5ecc95c033bf3a7744bb7862bf959066e6561670` with the two precision
pins below, architecture freeze, V3-F01 closure, publication of this acceptance record,
and normal production advancement through this architecture-only lineage.

That exact candidate and its complete supersession stack are **ACCEPTED** as resolved by
the precedence rules below. The independent review at
`e55b1da1c9049b1de58fcb06c65eea59939f2a57` is controlling:

`SPARK_PHASE_2_CODEX_V3_F01_REV2_INDEPENDENT_REVIEW_2026-09-10.md`

Its independently confirmed verdict is
`V3_F01_REV2_ACCEPTABLE_FOR_OPERATOR_ACCEPTANCE_AND_FREEZE`.

- **V3-F01 is CLOSED.** Gate C1 is satisfied for S.P.A.R.K.
- **The Phase-2 architecture correction is ACCEPTED and FROZEN.**
- Phase 0 and Phase 1 remain CLOSED and unchanged.
- **Phase-2 production implementation remains UNAUTHORIZED.**
- **Phase 3 remains UNAUTHORIZED.**
- The September 9 Fable architecture-challenge research remains noncanonical and unmerged;
  no experimental runtime change is accepted or promoted.

This decision supersedes earlier candidate/review status statements that this correction
was open, unaccepted, unfrozen or awaiting review. Those dated records remain historical
and unchanged. It does not reverse any explicit withdrawal in the supersession maps.

## Complete accepted stack and precedence

Highest precedence: this Operator acceptance, the independent Revision-2 review and its two
pins; then Revision-2 architecture/oracle deltas; then the FINAL consolidated candidate/oracle
as amended; then the retained portions of its inherited stack. A superseded or withdrawn
proposal is accepted only in its recorded disposition, never revived as operative text.

All paths below are in `engineering/phase2/`.

| Layer | Controlling artifact / disposition |
|---|---|
| Original Phase-2 architecture and matrix | `SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_2026-08-26.md`; `SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_2026-08-26.md`, retained only as amended downstream. |
| Phase-2 v2 | `SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v2_2026-08-26.md`; matrix v2, retained only as amended downstream. |
| Phase-2 v3 | `SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md`; matrix v3. The Opus record is the corrected provenance record; the historical Fable-named v3 copy is not a competing authority. |
| First V3-F01 correction and matrix | `SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_2026-09-06.md`, `03daa82032cecc6ed84407b440bc9eab06cbcd69`: superseded by the downstream maps. |
| Foundational logical-time adjudication | `SPARK_PHASE_2_V3_F01_FOUNDATIONAL_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md`, `379f8dc355125e6bca09801e4a66b0b35ec9720c`: horizon expansion/per-barrier time adopted as carried. |
| V3-F01 V2 correction and matrix | `SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_V2_2026-09-06.md`, `983a01fd6807b8c627c97d2673a1f40c54cc059c`: retained only through FINAL §19.3–19.4 and Revision-2 §11. |
| Command-time adjudication | `SPARK_PHASE_2_V3_F01_COMMAND_TIME_ADJUDICATION_CANDIDATE_2026-09-06.md`, `53a8432b9f0f3bad99c273d4ff7b52f4c63b5340`: superseded via serialized request boundary; ALT-U, X, C and A-1…A-3 remain withdrawn. |
| Serialized request addendum | `SPARK_PHASE_2_V3_F01_SERIALIZED_REQUEST_BOUNDARY_ADDENDUM_2026-09-06.md`, `30626abe4ec790e5f6a0287c3cc0b6d85470a893`: retained/amended/withdrawn exactly by FINAL §19.1–19.2 and Revision-2 §11. |
| ActiveRequest authority | `SPARK_PHASE_2_V3_F01_ACTIVE_REQUEST_OPERATOR_DECISION_2026-09-10.md`, `772e38da0d130d7a1225ba2eb60a75b3999336fe`: retained; F and ActiveRequest belong in the stable-boundary digest, outside the engine digest. |
| FINAL consolidation and oracle | `SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_FINAL_2026-09-10.md`; `SPARK_PHASE_2_ACCEPTANCE_TEST_ORACLE_V3_F01_FINAL_2026-09-10.md`, `74d044ddd2b59c3c2eff558beeeeb515390dbe0f`: all unaffected sections retained, subject to Revision-2 deltas. |
| Revision-2 correction and oracle | `SPARK_PHASE_2_V3_F01_ARCHITECTURE_CORRECTION_CANDIDATE_REV2_2026-09-10.md`; `SPARK_PHASE_2_ACCEPTANCE_TEST_ORACLE_V3_F01_REV2_2026-09-10.md`, `5ecc95c033bf3a7744bb7862bf959066e6561670`: accepted with the controlling review pins. |

This incorporates the complete FINAL §19 and Revision-2 §11 supersession maps, the
Revision-2 oracle §1 delta, the retained prior matrices, and the corrected V2-03 through
V2-10 dispositions. Phi, SH-1/SH-2, R-8 and the live stage-then-fence-without-preflight
recipe remain withdrawn. Prior reviews/reports/models are evidence and provenance; their
historical pending/revise verdicts do not override this acceptance.

## Controlling precision pins

1. **Multiple epoch-reset records sharing one frozen frontier must replay in ascending
   `reset_index`.** This includes resets before the next command and trailing resets.
2. **Production P-7 through P-9 must use bounded indexed lookup rather than scanning
   ever-growing finalized history.** Prefer PX-1's three read-only accessors or an
   equivalent bounded private seam. This changes no encoding, authority, identity or
   admission semantics. The candidate's O(history) fallback is not an accepted production
   option. The seam is a frozen implementation constraint, not permission to implement now.

D-6 through D-10 are accepted as reviewed: complete preflight/entailed apply, clean staging,
sticky fail-stop on entailment violation with no boundary publication, request-bound
results/dequeue with fail-closed mismatch, and deterministic fence identity. Restore and
replay obey the review's completed-versus-paused distinction and durability ordering.

## Publication lineage and preservation

Verified initial production: `e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8`.
The architecture-only path is:

`e9e26e8` → `bb1ccbb` → `772e38d` → `74d044d` → `5b19d7b` →
`5ecc95c` → independent review `e55b1da` → this acceptance commit.

The acceptance branch is `accept/v3-f01-rev2-freeze-20260910`, based directly on the
published independent review. Publish it normally, then advance `phase1-refoundation-v2`
by a preservation-safe fast-forward through this accepted history and push normally.
Verify live refs again before promotion; reconcile movement without rewriting history.
The final receipt provides this record's commit and final production commit, avoiding a
self-referential hash. No reset, force-push, discarded work, experimental merge or production
Rust change is authorized by this record.

## Validation and remaining gates

Independent evidence is committed with review `e55b1da`: 58/58 model checks; disposable
Rust probe and historical negative controls; 2,496 additional differential cases and eight
same-frontier reset-order fixtures; all 232 workspace tests; formatting, all-target strict
clippy, stricter core/engine library lint, metadata, whitespace and five Windows/Android
static target builds pass. Production crates, manifests, lockfile and tests are unchanged.
After promotion the applicable validation and local/tracking/live equality must be rerun
and reported in the final receipt.

Future gates remain: separately authorized Phase-2 implementation and executable acceptance
oracles with independent review; real process-crash recovery and durable snapshot/mailbox
proof; GAME integration and its transaction/late-input/report-acknowledgment contract;
performance evidence; executable Windows/Android replay and digest parity. Static builds
and disposable models do not satisfy those gates. GAME authority and repository state are
unchanged; this is S.P.A.R.K.'s architecture readiness, not a joint integration promotion.

**Exact next authorization required:** explicit Operator authorization to begin Gate C2,
implementing the frozen Phase-2 rule/effect runtime and acceptance oracle, including the
V3-F01 correction and both pins, under writer/reviewer separation. Phase 3 requires its own
later authorization after Phase-2 acceptance.
