# LUNA campaign handoff

Target commit: `67b877192cc78b75c6fbe60c69b5594dc10befe8`.

UTC window: started `2026-09-12T11:30:30Z`, deadline `2026-09-12T12:00:30Z`.
Closeout: `2026-09-12T11:34:00Z` (elapsed about 4 minutes).

Executed 15 hostile public-interface cases: composed Add/Subtract with Scale/Clamp,
explicit Decay, Assign/Aggregate, derived targets, activation/removal/restoration,
watchers, refusal, replay/restore/pacing, extreme values, zero-rate, and four seeded
deterministic sequences. All 15 passed with cargo exit 0; no timeout or panic.

Finding LUNA-001 is an unclassified, reproducible observation: i64::MAX and i64::MIN
assignments to `state.pressure` are rejected OutOfBounds and leave the cell absent.
Expected semantics were not specified. LUNA-002 and LUNA-003 are passing observations,
not defects. Full output and exact reproduction are in `luna-cases-v4.txt`; source is
`luna_cases.rs`; findings index is `findings.md`.

Coverage gaps: no exhaustive generated state-space search, no production mutation, and
no adversarial campaign beyond the bounded public vectors above. Product files and the
existing checkout were unchanged.
