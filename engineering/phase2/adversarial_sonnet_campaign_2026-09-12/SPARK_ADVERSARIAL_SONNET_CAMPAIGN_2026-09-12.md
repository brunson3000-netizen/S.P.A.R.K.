# S.P.A.R.K. Gate C2 — Adversarial test campaign (SONNET)

**Role:** Independent adversarial tester (Claude Sonnet 5, `claude-sonnet-5`). Not the main
engineer, not an independent reviewer, and not a repairer. This campaign records
reproducible bugs and evidence only. No product file was edited.

**Authority:** `SPARK_PHASE_2_GATE_C2_OPERATOR_NEXT_AUTHORIZED_STEP_2026-09-12.md` — the
adversarial campaign against the pinned build, authorized to run after independent KEEP.

**Exact target commit:** `67b877192cc78b75c6fbe60c69b5594dc10befe8`
(`candidate/phase2-gate-c2-w01-correction-20260912`).
**Independent KEEP review:** `21a4fec666ca493f6ac4d5194ec6e9c5380ce40c`
(`review/phase2-gate-c2-w01-correction-independent-20260912`).
Both verified against live GitHub (`git ls-remote --heads origin`) before testing began.

**Campaign window (UTC):** start `2026-09-12T11:30:40Z`, deadline `2026-09-12T12:00:40Z`.
**Test workspace:** an isolated detached worktree at `/tmp/spark-adversarial-sonnet/target`
pinned to the exact target commit, plus a disposable external crate at
`/tmp/spark-adversarial-sonnet/harness` depending on it by local path (spark-engine with the
`test-support` feature, matching `spark-testkit`'s own dev-dependency edge). No existing
checkout, worktree or branch was touched. This evidence is published from its own detached
worktree at `/tmp/spark-adversarial-sonnet/evidence-branch`, never from Luna's or any other
agent's workspace.

## What was exercised

30 hostile test cases through the public door (`Engine::process`, `Engine::genesis`,
`Engine::snapshot`/`restore`, `Engine::reconstruct_completed`) plus the sanctioned
`fixture.rs` snapshot-tampering seams (`test-support` feature) for malformed-snapshot
rejection:

| Group | Cases | Focus |
|---|---|---|
| A | 4 | Duplicate submission, conflicting payload under one command id, `ActiveRequest` mismatch mid-pause, horizon behind frontier |
| B | 3 | Same-time scheduled-work volume (500 items, one identity), scheduled-work/command ordering at one time, zero-delay self-reschedule |
| C | 5 | Malformed-snapshot restore rejection: active-behind-frontier, truncated lineage, tampered activation time, obligation-drop baseline, mismatched profile instance |
| D | 1 | Replay equivalence of a 10-command history |
| E | 2 | Atomic refusal leaving state unchanged; a decay rule declared on a `HostOwned` target |
| F | 2 | Long bounded sequences: 40 steps mixing activation/commands with replay equivalence; 30 steps with periodic snapshot/restore |
| G | 5 | C2W-01 composed-body stress: 15-round removal/restoration/snapshot interleave; dangling initial-work rule reference at genesis; large values through composed settlement; isolating the declared-bounds check; the per-cohort report for a bounds refusal |
| H | 1 | `restore` rejecting a digest mismatch on an unsealed tamper (no `snapshot_reseal`) |
| I | 5 | Zero pacing budget, duplicate command id with different time/payload, snapshot mid-pause, an exact clamp-boundary composed settlement, three distinct-identity additive rules on one target |
| J | 2 | Snapshot/restore immediately after an atomic refusal; double-restore of the same snapshot |

Full source: `adversarial_probes.rs`. Full captured output: `full-run.txt`
(`SHA256SUMS.txt` fixes the probe file's hash). **Result: 30/30 pass** against the pinned
target — no reproducible defect was found in the tested surface within the campaign window.

## Findings

**No SONNET-numbered finding is recorded.** Every initial discrepancy this campaign
produced was traced, within the campaign, to a source-confirmed intentional behavior rather
than a defect, and the probe was corrected to assert that behavior instead of the tester's
original (wrong) assumption. Each is logged below as a **triaged non-finding**, per "do not
invent a requirement" — none of these is a bug and none should be re-investigated as one.

| # | Initial observation | Source-confirmed disposition |
|---|---|---|
| T-1 | Resubmitting the exact same completed command returned `CompletedCommandNotFinalized(CommandIdentityConflict)` instead of the tester's assumed idempotent `Completed` | `request.rs:252`/`engine.rs:2635`: `CommandIdentityConflict` is a named finalization refusal (D-2). State was confirmed unchanged by the refused resubmission. Not a bug. |
| T-2 | A rule that reschedules itself with zero delay failed at `rule_set` construction rather than at runtime | `rules.rs:938,1347`: `RuleSetError::ZeroDelay` is a static admission check — exactly the anti-infinite-loop guard one would want, firing before the rule can ever run. Not a bug. |
| T-3 | `Engine::genesis` with `max_due_per_cycle=0` failed before reaching the runtime loop | `rules.rs`: `ZeroPacingBudget` is rejected at rule-set construction, so a zero-progress budget can never reach `process()` at all. Not a hang; the guard prevents the hang class structurally. Not a bug. |
| T-4 | With pacing budget 1 and 20–50 same-time work items, the request completed in one call instead of pausing | `engine.rs` P1/A3: `admit = executable <= remaining \|\| admitted_executable == 0` — the first slice is always admitted whole even if it exceeds the budget, guaranteeing progress rather than starving on an oversized atomic slice. Confirmed by re-reading the pacing loop; not a bug. |
| T-5 | An `Assign` far outside a target's declared bounds returned request-level `Outcome::Completed` with the cell left absent | Confirmed via `reports()`: the per-cohort report carries `CohortOutcome::Rejected { rejection: InvalidEffect { error: OutOfBounds } }`. Request-level `Completed` means the request boundary was fully processed; refusal is reported at the cohort level, which the tester had not inspected in the first probe. State was unchanged. Not a bug — a gap in the tester's own initial probe, corrected in `sonnet_g5`. |
| T-6 | Restoring against a freshly re-activated "different" profile instance succeeded (`None` error) | The profile is content-addressed; re-activating the identical manifest in a fresh registry yields the same activation hash, so it is not actually a distinct profile from the engine's perspective. Consistent with content-addressed identity, not a gap — recorded because this campaign could not construct a *genuinely* different profile within its time budget, so this path has less coverage than the others in group C. |

No panic, timeout, or unbounded process occurred anywhere in the 30-case run. No timeout
was recorded; every case returned within the bounded loops used (either `drive()`'s standard
10,000-call cap, or an explicit 50-call bounded loop in the pacing-edge cases).

## Coverage gaps (not findings)

- **Obligation-drop bidirectional-invariant restore test (group C)** was not completed to a
  targeted mutation: the public `snapshot_drop_obligation` seam needs an internal `WorkKey`
  this campaign did not construct in time. Only the unmodified-reseal baseline was checked.
- **A genuinely distinct second profile** (different manifest content) for the
  mismatched-profile restore case was not constructed; see T-6.
- **Duplicate/conflicting `WorkKey` staged directly via `snapshot_stage_raw`** was not
  exercised.
- Every case ran on one machine, one process, sequentially or with `--test-threads=4`; no
  concurrent-access or multi-process contention was tested (single-engine-instance API, so
  likely out of scope regardless).
- Windows/Android runtime behavior was not exercised (out of scope for this workstation;
  the existing evidence already discloses static-only coverage there).

## Reproduction

```
git worktree add --detach /tmp/repro-sonnet 67b877192cc78b75c6fbe60c69b5594dc10befe8
cd /tmp/repro-sonnet
# build a disposable external crate per harness_Cargo.toml.txt, pointing its
# path dependencies at /tmp/repro-sonnet/crates/{spark-core,spark-engine,spark-testkit}
# with spark-engine feature = ["test-support"], then:
cargo test --offline --test adversarial -- --nocapture
```

No random seed was used; every vector is deterministic (fixed logical times, fixed literal
parameters). `--test-threads=1` reproduces `full-run.txt` in call order exactly.
