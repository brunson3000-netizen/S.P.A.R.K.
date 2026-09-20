# RADAR shared release decision — 2026-09-13

## Decision

The shared package is prepared but not executable for paid capability scoring.
The historical `RADAR-V1.0.0` pack is preserved byte-for-byte. It is not
silently replaced, expanded, or scored as if it tested the named abilities.

## Governing source and clearance

The independent review at
`/home/chromikey/SWARM/worktrees/openrouter-probation-lab-v7/project_records/research/harness_calibration/2026-09-08_radar_v1_battery_independent_review/RADAR_V1_BATTERY_DEFECT_1_COMPLETE_2026-09-08.md`
marks `RADAR-V1-DEF-01` **BLOCKING**: all 40 subjects omit the actual code,
diff, schema, tools, constraints, source records, or long context and their
oracles are derivable from the visible ordinal. A 40/40 non-capability guess
reproducer is included there.

The design-level review at
`.../RADAR_V1_BATTERY_DESIGN_LEVEL_REEXAMINATION_2026-09-08.md` requires a
bounded redesign before live scoring: `ProfileDepth`, immutable
`ExecutionScope`, typed `FixtureResult` and closed status, profile-derived
fixture membership, and distinct CHEAP/FULL confidence classes. It says the
independent re-review is required before any live capability call.

Therefore redesign clearance means: new fixture content and hashes, the typed
evidence-model repair, offline regression/hash validation, and independent
review of that exact candidate returning no blocking findings. Accepted release
means the coordinator records those artifacts and reissues the shared hash
manifest; it does not mean model acceptance or routing activation.

## Recommendation

Authorize the already-scoped bounded redesign/build and independent re-review
as the next action. Do not waive the blocking defect and do not spend against
V1.0.0. Slots 14 and 15 also require explicit route replacement because their
observed routes lack one of the required full-battery capabilities.

## Evidence reused

B's preflight hashes are reused from
`/home/chromikey/SWARM/research/radar_admin_b_preflight_20260913/`.
C's reported verification is accepted as corroborating evidence only; the
shared release copies are hash-checked locally and the absolute paths are in
`RADAR_SHARED_RELEASE_MANIFEST.json`.
