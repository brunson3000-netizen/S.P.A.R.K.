# LUNA-C Findings Index — 2026-09-12

**Reproducible bugs found: 0.**

No finding IDs are issued because no reproducible defect was found in this
bounded continuation.

## Tests run (all passed, all novel vs. the original LUNA campaign and the
existing 427-test workspace suite)

| Test | Gap addressed | Result |
|---|---|---|
| `random_walk_snapshot_restore_and_reconstruct_agree_seed_0xc0ffee1234` | generated/seeded sequences, snapshot/restore, replay equivalence | PASS |
| `activate_epoch_is_refused_while_a_different_request_is_paused` | epoch activation interleaved with a paused request | PASS |
| `max_effects_per_wave_breach_surfaces_typed_semantic_cap_report` | semantic cap / budget overload typed reporting | PASS |

## Triaged non-findings

2 — both were my own test-construction errors (same-target effect
aggregation coalescing as designed; single-slice pacing admits fully by
design), corrected in the final test source. Full detail and settling
source locations are in LUNA_C_CAMPAIGN_2026-09-12.md.

## Coverage gaps left open (not tested this session)

- Independent decay/recovery-across-epoch-boundary vector (D-3...D-5 family)
- Update::Aggregate / derived-authority composed-body targets, removal and restoration
- Timeline ordinal-window edges through the command door: staging exhaustion, reset_timeline_epoch, replay across a reset
- Exhaustive/property-based state-space search, fuzzing, production mutation testing (all explicitly out of scope)

See LUNA_C_CAMPAIGN_2026-09-12.md for full detail.
