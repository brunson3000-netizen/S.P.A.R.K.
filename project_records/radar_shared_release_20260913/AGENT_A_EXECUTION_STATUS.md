# Agent A execution status — 2026-09-13

## Result

No paid calls were dispatched. The assignment is blocked before execution by
the existing runner contract, not by budget or missing model assignment.

## Exact assignment

Slots 01–05 from the shared manifest:

1. `google/gemini-3.8-flash` — `google-ai-studio/flex`
2. `mistralai/devstral-2512` — `mistral/eu`
3. `deepseek/deepseek-v4-flash-0731` — `baidu/fp8`
4. `nex-agi/nex-n2.5-pro:free` — `nex-agi/fp8`
5. `nex-agi/nex-n2.5-mini:free` — `nex-agi/bf16`

Planned count: 5 × 40 × 3 = 600 case executions.

## Verified runner boundary

From the actual toolkit worktree, `tools/compute_qualification/__main__.py`
exposes only `live-canary`, whose implementation constructs `NimHarnessBroker`
and accepts a single NIM model. The historical `live_canary.py` caps a canary
at three calls and uses the fixed prompt `Return exactly JSON: {"status":"ok"}`.
It does not load RADAR fixtures, iterate 40 cases, perform tool exchanges, or
pin arbitrary OpenRouter routes. `InferenceRequest` has no tool-call field.

Therefore the existing runner cannot administer the requested 600 executions.
Launching it would test the wrong workload and would not satisfy the manifest.
No replacement runner was invented, per the explicit instruction not to
redesign the harness.

## Offline checks

Executed from `/home/chromikey/SWARM/worktrees/compute-qualification-toolkit-v1`
at commit `0e640573699faf16e944c89abd84fed9c57d620c`:

```text
PYTHONPATH=. /home/chromikey/AI/.venv/bin/python -m unittest \
  tests.tools.test_radar_battery_v1 tests.tools.test_compute_qualification
38 tests passed.
```

These tests validate scorer/checkpoint plumbing only. The independent review
still identifies V1 fixture-content Defect 1: all 40 fixtures are ordinal
pattern-guessable and omit their claimed task material. V1 remains preserved;
no live call used it.

## Cost reservation

Manifest bound for Agent A: `$1.659` maximum, below the `$4.00` allocation.
Actual spend: `$0.00`. No reservation was consumed.

## Exact decision required

Authorize one bounded, lane-local RADAR/OpenRouter runner that calls the
existing exact-route OpenRouter adapter through the existing broker, supports
the frozen fixture/tool contract, and writes append-only checkpoints and
reservations. Separately authorize the versioned fixture-content repair and
independent review required to clear Defect 1. Preserve V1 unchanged.
