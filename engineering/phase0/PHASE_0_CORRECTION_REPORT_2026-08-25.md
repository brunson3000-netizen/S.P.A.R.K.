# S.P.A.R.K. Phase 0 Correction Report

**Date:** 2026-08-25  
**Correction package:** v0.2  
**Basis:** `PHASE_0_CODEX_INDEPENDENT_REVIEW_2026-08-25.md`  
**Scope:** Documentation/contract corrections only. No Rust implementation. No new runtime primitive. No controlling-blueprint change.

## Verdict being corrected

Independent review returned:

```text
REVISE_PHASE_0
PHASE-1 AUTHORIZATION: NO
```

It found the frozen primitive set sufficient but identified three blockers plus eight majors and four minors.

## Blocker closure map

### B-01 — Canonical input transaction boundaries

**Corrected in:**
- ADR-0003;
- ADR-0005;
- matrix R-003, R-038, R-041, R-044, R-081, R-082, R-084, R-097.

**Contract now requires:**
- canonical command envelope;
- effective logical time;
- stable logical source + source sequence;
- unique canonical input ordinal;
- command idempotency/payload hash;
- explicit command barriers;
- stable snapshot closure before next command;
- transport batches as packaging only;
- ambiguous same-time order rejected;
- quota/catch-up partition does not alter occurrence identity.

### B-02 — Profile administration can change canonical authority

**Corrected in:**
- ADR-0002;
- ADR-0004;
- matrix R-005, R-007, R-012–R-016, R-062, R-083, R-093.

**Contract now requires:**
- authority/write class included in immutable definition fingerprint;
- no authority conversion through hot tune, reload, alias, restore, or ordinary migration;
- authority change requires new ID, explicit migration, authority ADR, and operator approval;
- host-owned/derived truth cannot be fabricated by migration.

### B-03 — Old-save continuation not bound to exact behavior artifacts

**Corrected in:**
- ADR-0004;
- ADR-0006;
- matrix R-012, R-015, R-055, R-060, R-085–R-087.

**Contract now requires:**
- immutable content-addressed manifest/config/definition artifacts;
- exact artifact resolution for non-migrated continuation;
- delayed work bound to creator epoch/rule fingerprint/artifact hashes;
- explicit materialized-effect vs rule-re-evaluation obligation modes;
- atomic migration/checkpoint activation;
- separate mechanism-replay and outcome-replay contracts.

## Major finding disposition

| Finding | Disposition |
|---|---|
| M-01 global O(total-world) fan-out | Closed architecturally by aggregate/inherited exposure and lazy materialization; scaling test added. |
| M-02 frozen requirements missing from matrix | Added explicit rows for availability/accessibility/affordability, polarity metadata, no invented microhistory, replay distinction, permitted telemetry, and full profile isolation. |
| M-03 provisional concepts accidentally hardened | Taxonomy rows reclassified as PROFILE_DATA; role-as-profile-vocabulary separately frozen; appraisal rule allows persistent interpretation via StateCell. |
| M-04 acceptance vs execution | Separate disposition and outcome stages; outstanding-intent/idempotency/cross-profile checks added. |
| M-05 MCI/inspection isolation | Output allowlist, trust-domain routing exclusion, cross-profile rejection, isolated dry-run snapshot added. |
| M-06 internal queue/schema bombs | Finite queue/graph/nesting/string/alias/migration budgets and deterministic backpressure added in budget v0.2. |
| M-07 service equivalence tuple | Exact manifest/config, capability context, input ordinal added; session/subscriber/transport metadata explicitly inert. |
| M-08 Android determinism | Executable Android canonical fixture parity required before cross-platform acceptance. |

## Minor finding disposition

| Finding | Disposition |
|---|---|
| N-01 controlling filename | Matrix now names `CONTROLLING_BLUEPRINT_v0.2.md`. |
| N-02 combined statuses | Status legend normalized; combined status forms removed from corrected rows; calibration/profile data separated. |
| N-03 ambiguous dependency diagram | Replaced by explicit “may depend on” policy in ADR-0001. |
| N-04 provenance truncation | Deterministic coverage metadata and non-authoritative summary rule added to ADR-0006/matrix/budget. |

## Primitive sufficiency

No new primitive was added. Corrections strengthen contracts around the existing frozen primitive set.

## Phase-1 status

**NOT AUTHORIZED.**

The corrected writer package must receive independent re-review. Phase 1 may begin only after the re-review closes B-01 through B-03 and reports no new foundational blocker.
