# S.P.A.R.K. V3-F01 Independent Review — Supplementary NIM Evidence Summary

**Date:** 2026-09-06
**Authority:** supplementary development-compute evidence only
**Gateway:** established MCI development-tool ingress to the bounded NVIDIA NIM gateway
**Final adjudicator:** Codex independent reviewer

## Panel 1 — full evidence packet

- MCI request: `mci-dev-e9c9b5ca9492`
- Bundle: `dcb-d92644ddfeda`
- Tasks: code review, falsification, architecture review, test generation
- Fixed packet: candidate, candidate matrix, inherited v2/v3 architecture, controlling
  Codex V3-F01 finding, and Phase-1 scheduler source
- Independence: workers received the same packet and no other worker response

| Task/role | Model | Outcome |
|---|---|---|
| code review / deep reviewer | `moonshotai/kimi-k3` | HTTP 429, failed; raw failure preserved |
| falsification / judge | `openai/gpt-oss-20b` | HTTP 200, but hit the 8,192-token output limit in internal reasoning and returned no final response |
| architecture review / judge | `moonshotai/kimi-k3` | HTTP 429, failed; raw failure preserved |
| test generation / test generator | `moonshotai/kimi-k3` | network timeout after bounded retry; raw failure preserved |

The `gpt-oss-20b` internal reasoning was repetitive and did not deliver an adjudicable
final response. It was retained in `bundle.json` and was not silently treated as a
finding.

## Panel 2 — concise fallback packet

- MCI request: `mci-dev-680afb02ca2c`
- Job: `dcj-5bc33106384f`
- Task: code review panel
- Fixed packet: concise, exact contract facts and the requested failure hypotheses
- Independence: three roles were seated concurrently and received no worker output

| Role | Model | Outcome |
|---|---|---|
| fast reviewer | `openai/gpt-oss-20b` | completed, 224.65 s |
| deep reviewer | `moonshotai/kimi-k3` | HTTP 429 after bounded retry, failed |
| judge | `nvidia/nemotron-3-ultra-550b-a55b` | HTTP 200, 283.68 s; response ended at model length limit |

## Finding influence

The usable responses independently reinforced four findings already reached by Codex:

1. missing `now` makes `CohortNotDue` incoherent/unimplementable;
2. the owned token is replayable and the oracle omits replay/slot-state transitions;
3. boundary-`now + 1` delayed enqueues defeat the claimed catch-up theorem unless
   logical-time/barrier semantics are adjudicated; and
4. counting conflicted keys conflicts with the inherited `DrainOutcome::due` budget.

Codex rejected or narrowed several NIM suggestions after direct source/probe review:

- the claimed inherent Rust borrow failure was disproved by a bounded Rust 1.98 compile
  probe; non-lexical lifetimes permit the owned-selection call shape;
- returning an owned due-work plan or using `RefCell` would weaken the candidate's
  transience/safety objective and was not adopted;
- splitting an oversized cohort violates the inherited indivisible-cohort contract and
  was not adopted;
- speculative concurrent `drain_due` races are outside the single-`&mut Scheduler`
  safe-Rust surface and were not accepted without a concrete composition path;
- rollback-on-extraction-error is unnecessary if validation precedes any removal; the
  required contract is a byte-identical no-op failure, not rollback after partial work.

NIM did not change the final verdict; it increased confidence in the independently found
logical-time, token, missing-`now`, conflict-budget, and oracle defects.

## Evidence inventory

The root directory preserves Panel 1's `mci_request.json`, `bundle.json`, individual raw
worker files, and `telemetry.jsonl`. `fallback_panel/` preserves Panel 2's
`mci_request.json`, `job.json`, individual raw worker files, and telemetry. Request JSON
contains the exact fixed prompts, task/role/model selection provenance, failures, token
usage where returned, and the gateway's non-authoritative notice. No credential value is
stored; only the gateway's credential-source description appears.
