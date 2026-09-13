# Source Study — wiremock-rs + Toxiproxy

Status: FIRST TRACE COMPLETE / EXPERIMENT REQUIRED
Study date: 2026-09-13

## Frozen sources

### wiremock-rs
- Upstream: `https://github.com/LukeMathWalker/wiremock-rs`
- Commit: `6b193047bf2c5626da5dc5f3a23b58ab9bd3f130`
- Version: 0.6.5
- License: MIT OR Apache-2.0
- Runtime: Rust async HTTP mock server

### Toxiproxy
- Upstream: `https://github.com/Shopify/toxiproxy`
- Commit: `40f7fd31bee529d824116bd2a11a9e3425e904ec`
- License: MIT
- Runtime: Go TCP proxy + HTTP control API

## Problem framing

PROBLEM: Provider adapters, federation links and external-service tooling need repeatable tests for two different failure classes:
1. application/protocol semantics are wrong (HTTP status/body/header/order/request expectations)
2. the transport path is unhealthy (latency, timeout, reset, partial data, bandwidth, close behavior).

USER: adapter developers, provider backends, federation/network code and supervisor recovery tests.

INPUT: deterministic semantic fixtures, expected requests, upstream test service, proxy topology and toxic/fault schedule.

OUTPUT: controlled provider responses, recorded/verified requests, transport fault behavior and application recovery evidence.

AUTHORITY: both are test infrastructure. wiremock accepts local HTTP requests and records bodies/headers; Toxiproxy opens listening sockets and forwards/interrupts TCP traffic. Neither should receive production credentials or be exposed as an untrusted network service during normal qualification.

TRUST: test fixture definitions, local network binding, application-under-test routing, expected secret redaction, upstream service fixture, fault schedule.

STATE: wiremock mounted mocks/expectations/recorded requests; Toxiproxy proxy definitions, enabled state and toxic chains.

FAILURE: unmatched wiremock request returns 404 unless another mock handles it; unmet expectations fail verification/panic. Toxiproxy faults can be deterministic or intentionally randomized depending on toxic parameters/probability.

## Trace A — wiremock semantic fixture

Test creates `MockServer`
→ server binds isolated random local port
→ mount one or more `Mock`s
→ each mock combines request matchers + response template/dynamic responder + optional expected invocation count
→ application sends HTTP request
→ wiremock selects matching behavior or returns 404
→ records received requests by default
→ expectation verification occurs explicitly/at server shutdown.

Useful match/evidence mechanics:
- HTTP method/path/query/header/body matching
- custom `Match` implementations/closures
- static or request-dependent `Respond`
- deterministic status/header/body fixtures
- optional fixed response delay
- per-test isolated mock server
- received-request recording and inspection
- expected call counts to prove side effects/retries occurred or did not occur.

This is suitable for provider contracts such as:
- exact error envelope
- malformed/missing field
- pagination/token sequence
- rate-limit headers/status
- retryable vs terminal status
- duplicate/idempotency behavior
- authentication rejection
- response-schema evolution.

Limit: wiremock is HTTP-level. `ResponseTemplate::set_delay` can simulate a slow server response, but it does not reproduce lower-level socket failure/partial-stream behavior.

## Trace B — Toxiproxy topology and control

Application test endpoint is changed from direct upstream to a Toxiproxy listener.

Test/control client:
→ create/populate proxy with name/listen/upstream/enabled
→ application establishes ordinary TCP connection through proxy
→ HTTP control API adds/updates/removes toxics independently on upstream/downstream directions
→ toxic chain modifies data flow
→ application observes real socket/network behavior.

Built-in toxic/fault families at this pin/documentation include:
- fixed latency (+ optional jitter)
- connection down
- bandwidth limit
- slow close
- timeout
- peer reset
- slicer
- limit data
- packet loss.

Toxics are chained and can be dynamically added/removed while the proxy runs. The control API makes fault transitions testable without restarting the application.

## Determinism boundary

Toxiproxy advertises both deterministic tampering and randomized chaos.

For SPARK acceptance/replay fixtures:
- `toxicity = 1.0`
- fixed attributes
- latency jitter = 0
- avoid packet-loss/randomized schedules unless a seed/replay mechanism is explicitly owned by the test harness.

Concrete evidence: `LatencyToxic` uses `math/rand` whenever jitter > 0. Therefore “1000ms ± jitter” is not byte-for-byte deterministic timing evidence unless randomness is controlled externally.

Randomized chaos remains valuable as a separate exploratory/stress lane, not as the only acceptance reproduction.

## Trace C — paired semantic + transport replay

Recommended pair:

Application/provider adapter
→ Toxiproxy listener
→ wiremock upstream.

This gives orthogonal controls:

### wiremock owns semantic behavior
- exact request expectations
- provider HTTP responses
- response payload/status/header sequence
- request recording.

### Toxiproxy owns transport behavior
- fixed network delay
- connection refusal/down
- timeout/no data
- reset/partial data/slow close/bandwidth.

A test can therefore distinguish:
- “provider returned 429” from
- “TCP timed out before an HTTP response” from
- “provider returned valid HTTP too slowly” from
- “connection reset after partial transfer”.

That distinction matters for retry/quarantine/backoff policy.

## Boundary findings

### Mock provider ↔ real provider
A mock fixture proves local adapter behavior against the fixture, not that the current external provider still behaves exactly that way. Fixtures need provenance/version/source from captured docs/contracts or a separately authorized live observation.

### Recorded request ↔ secret custody
wiremock records requests by default. If Authorization headers/tokens are present, raw recorded evidence can contain secrets. SPARK must redact/avoid real credentials and use synthetic secrets in qualification.

### Semantic latency ↔ transport latency
wiremock delay occurs at HTTP responder level. Toxiproxy delay/reset/timeout operates on the transport. Keep these fault classes explicit rather than treating “slow” as one phenomenon.

### Fault schedule ↔ deterministic acceptance
Random toxicity/jitter/packet loss can create useful chaos but weak reproduction. Acceptance evidence should use explicit fixed schedules and preserve the schedule as an artifact.

### Proxy availability ↔ tested dependency availability
If Toxiproxy itself crashes or its control request fails, that is a harness/infrastructure failure, not evidence that the application handled the intended fault.

## Determinization candidates

- provider response cassettes/fixtures
- expected request schema/count verification
- retry sequence fixtures
- fixed transport fault schedules
- fault-state transitions
- recovery timing bounds
- evidence capture/redaction
- application error-class assertion.

## Material extracted

IDEA: provider semantics and transport health require separate fault injectors.

PATTERN: semantic fake service behind a controlled network-fault proxy.

ALGORITHM: deterministic request matching/response templating + ordered toxic chain.

CODE/TOOL: wiremock-rs is a strong direct Rust dev-dependency candidate; Toxiproxy is best treated as a pinned external sidecar/executable controlled through its HTTP API rather than reimplemented early.

TEST/INVARIANT: fault-injection infrastructure must prove the intended fault was active; harness failure is distinct from application failure; randomized chaos is not accepted as sole reproduction evidence.

## Primary disposition

EXPERIMENT_NOW

Likely after passing fixture experiment:
- wiremock-rs: REUSE_CODE/REUSE_TOOL in Rust adapter tests
- Toxiproxy: REUSE_TOOL as isolated qualification sidecar.
