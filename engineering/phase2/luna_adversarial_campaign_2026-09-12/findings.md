# Findings index

Campaign window: 2026-09-12T11:30:30Z–12:00:30Z UTC. Exact target:
`67b877192cc78b75c6fbe60c69b5594dc10befe8`.

No confirmed product failure observed.

LUNA-001 — unclassified extreme assignment refusal. At UTC 2026-09-12T11:33Z,
public `cmd.set` assignments of `i64::MAX` and `i64::MIN` to `state.pressure`
were rejected with `InvalidEffect/OutOfBounds`, exit 0 from the harness, and
left the cell absent. Expected behavior was not specified for these values, so
this is an observation only. Reproduction: `luna-cases-v4.txt`, test
`luna_extreme_assignments_and_zero_cadence_observation`, source
`luna_cases.rs`, seed none. One bounded rerun reproduced the same result.

LUNA-002 — no discrepancy: zero-rate evaluation committed canonical time with
cell `(2,1)`; this matched the retained D-6 behavior. Source/test are the same
as LUNA-001.

LUNA-003 — no discrepancy: four seeded sequences (seeds `1`, `17`, `0x5eed`,
`u64::MAX-1`), 12 mixed set/body events each, all replayed deterministically.
See `luna-cases-v4.txt`, test `luna_seeded_deterministic_sequences`.

The harness executed 15 tests total (13 inherited hostile public vectors plus 1
extreme/zero-rate and 1 seeded deterministic test). All passed;
there were no panics or timeouts. `luna-cases-v4.txt` contains the complete
command, source digest, cargo exit code, stdout and stderr classification.
