# RADAR V1 battery freeze

This is the first concrete offline freeze of the RADAR V1 capability battery.
It materializes the correction authorized by remote commit
`4246eda4d73daac61df3e8742caf1b4a962c1b87`, which states that the previously
described battery had not yet been materialized or hash-frozen.

## Frozen contract

- Battery: `RADAR-V1.0.0`
- Scorer: `RADAR-V1-MECHANICAL-1`
- Dimensions: 8 fixed dimensions
- Fixtures: 5 compact synthetic fixtures per dimension, 40 total
- CHEAP: fixture slots 0 and 1 per dimension (16 total)
- FULL: all 5 fixture slots per dimension (40 total)
- Scoring: exact mechanical/reference scoring, raw range 0..1 and normalized
  range 0..5; operational failures and unknowns are excluded from capability
  scores and remain explicit
- Oracle boundary: `RADAR_V1_SUBJECT_FIXTURES.json` is the subject-facing pack;
  oracle material exists only in the scorer-custody fixture pack
- No live provider or capability calls were made for this freeze.

## Validation basis

The offline test suite covers exact 40-fixture shape, every fixture scorer,
deterministic pass/fail scoring, high/low/mixed profiles, CHEAP-to-FULL
aggregation, conflicting duplicate observations, checkpoint/resume without
duplicating valid fixtures, oracle non-exposure, and transport-failure
separation. `validate_materialized()` also validates the generated pack
boundary and version binding.

The manifest contains the exact SHA-256 values for the private fixture pack,
subject pack, scorer contract, and canonical manifest payload. The file-level
hashes are recorded separately in the operator handoff.

Independent review is required before any live capability call.
