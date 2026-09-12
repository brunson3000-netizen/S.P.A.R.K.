# SONNET-C Findings Index

Target: `67b877192cc78b75c6fbe60c69b5594dc10befe8` (verified against
`origin`'s `refs/heads/candidate/phase2-gate-c2-w01-correction-20260912`).

**Reproducible bugs found: 0.**

No finding entries. All 7 new discriminating test cases (SC-1 through SC-7)
passed against the pinned commit; see `full-run.txt` for the complete
captured run (exit code 0) and `SONNET_C_CAMPAIGN_2026-09-12.md` for what
each case attacked and why it was judged a pass rather than a defect.

## Triaged non-findings (not bugs; see campaign report for detail)

- `reset_timeline_epoch` running while `ActiveRequest` is `Some` -- declared
  intentional behavior (doc comment at the function), and its consequence
  (stale-epoch finalization refusal) verified correct by SC-2.
- `RefusedActiveRequestMismatch` outranking `RefusedHorizonBehindFrontier`
  when both conditions could apply -- settled by direct code reading
  (`engine.rs` `Engine::process`, P0), no controlling text found requiring
  the opposite order.
- `fixture::snapshot_stage_raw` / `fixture::stage_raw` reporting success
  (`is_ok()`) for the retryable `NotInAdmissionWindow` disposition --
  pre-recorded by the mission brief, not new; worked around in SC-4 using
  the real frontier ordinal.

## Coverage gaps intentionally left open

See "Coverage gaps that remain" in `SONNET_C_CAMPAIGN_2026-09-12.md`:
epoch-reset-while-paused resumed with a different or non-Command request;
`reconstruct_completed` with mixed refused-commands/epoch-activations in one
history; deep wave-chain depth and obligation/scheduled-work store-pressure
caps; `FinalizationEntailmentViolated` cross-request-type stickiness; the
full existing 427-test workspace suite (out of scope, already passing); and
any fuzzing/property/concurrency/performance testing.
