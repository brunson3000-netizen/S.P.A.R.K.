# Experiment — Provider Semantic + Transport Failure Replay

Status: READY_TO_RUN

## Goal

Prove one provider adapter can correctly distinguish and recover from deterministic HTTP/provider failures and deterministic transport failures without live/paid calls.

## Fixture topology

`adapter under test → Toxiproxy → wiremock upstream`

Use only synthetic credentials/data.

## Required cases

1. HTTP 200 valid response.
2. HTTP 429 + retry headers; verify retry/backoff count.
3. HTTP 500 then 200; verify bounded retry and eventual success.
4. malformed JSON / schema mismatch; verify terminal protocol classification.
5. fixed 1s downstream latency with client deadline below/above bound.
6. connection down before request.
7. connection reset during transfer.
8. limit-data/partial response if supported by target client path.
9. recovery transition: fault active → disabled → subsequent request succeeds without corrupting session state.
10. idempotent side-effect operation: assert fake upstream invocation count under retry.

## Determinism

For acceptance fixtures:
- toxicity 1.0
- jitter 0
- no random packet loss
- fixed explicit durations/byte limits
- frozen wiremock status/body/header sequence.

Randomized chaos is a separate optional lane and cannot supply sole acceptance evidence.

## Evidence

Preserve:
- source commit
- adapter config/version
- wiremock fixture
- Toxiproxy proxy/toxic JSON before/during/after case
- received requests after secret redaction
- retry/attempt timeline
- final classified error/result
- timeout/cancel evidence
- harness health evidence.

## Acceptance

Each case must show:
1. intended fault was successfully installed
2. application traffic passed through the harness
3. observed result matches expected classification
4. retries/side effects stayed within bounds
5. harness itself remained healthy or case is marked infrastructure failure.
