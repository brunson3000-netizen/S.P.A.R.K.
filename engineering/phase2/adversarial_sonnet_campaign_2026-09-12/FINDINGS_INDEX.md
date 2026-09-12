# SONNET findings index — 2026-09-12

**Target:** `67b877192cc78b75c6fbe60c69b5594dc10befe8`.

**Reproducible bugs found: 0.**

No SONNET-NNN finding is open. Six initial discrepancies were investigated within the
campaign window and each was source-confirmed as intentional/documented behavior, not a
defect; they are recorded as triaged non-findings T-1 … T-6 in
`SPARK_ADVERSARIAL_SONNET_CAMPAIGN_2026-09-12.md` §"Findings", together with the exact
source location that confirms each disposition, so a reviewer does not need to re-derive
them.

This is a negative result for the tested surface within the campaign window, not a
certification: see the report's "Coverage gaps" section for what was not exercised in time
(bidirectional-invariant obligation-drop mutation, a genuinely distinct second profile,
staged-conflicting-WorkKey, concurrent access, Windows/Android runtime).
