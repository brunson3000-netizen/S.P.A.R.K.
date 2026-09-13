# Pattern Card — Semantic + Transport Fault Replay

PATTERN: Separate Provider Semantics From Network Failure
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- wiremock-rs @ `6b193047bf2c5626da5dc5f3a23b58ab9bd3f130`
- Toxiproxy @ `40f7fd31bee529d824116bd2a11a9e3425e904ec`

## Problem

Retry/recovery code often conflates a valid provider error response with network failure. Tests that simulate only one layer cannot prove the application classifies and reacts correctly across both.

## Mechanism

Route the application through two controlled layers:

`application → network fault proxy → semantic fake upstream`

Semantic layer controls:
- request matching
- exact status/headers/body
- response sequence
- expected invocation count
- request recording.

Transport layer controls:
- fixed latency
- timeout/down/reset
- bandwidth/data truncation/slow close
- upstream/downstream direction
- explicit enable/disable transition.

Store one immutable fixture describing both layers and expected application outcome.

## Benefits

- deterministic differentiation of provider vs transport errors
- reproducible retry/backoff tests
- easy verification of retry count/idempotency
- failure transitions can be replayed without paid/live provider calls
- complements rather than replaces sparse authorized live acceptance.

## Risks / failure modes

- fixtures drift from provider reality
- recorded mock requests expose secrets
- Toxiproxy/random jitter makes supposedly deterministic tests stochastic
- fault proxy itself fails
- application accidentally bypasses proxy
- semantic mock and transport schedule do not represent a real reachable state.

## Boundaries crossed

Application → test proxy.
Proxy → fake provider.
Fixture → application behavior assertion.
Recorded traffic → evidence/redaction layer.

## Determinization relevance

VERY HIGH. Fault schedules, request expectations and result classification should be deterministic test machinery.

## Likely architectural location

Provider backend cassette/fault test harness and federation/network recovery qualification.

## Finding class

PATTERN + TOOL/CODE REUSE + TEST/INVARIANT

## Primary disposition

EXPERIMENT_NOW

## Required invariants

1. Fixture records source commit/tool versions/semantic response/fault schedule.
2. Harness proves traffic actually traversed the fault proxy.
3. Harness failure is distinct from application failure.
4. Acceptance fixtures use fixed toxics; randomness is a separately labeled stress lane.
5. Real secrets are never required or retained in mock request evidence.
6. Retry count, final result and side-effect count are all asserted.
7. Mock fixtures carry provider-contract provenance/validity date when based on external behavior.
