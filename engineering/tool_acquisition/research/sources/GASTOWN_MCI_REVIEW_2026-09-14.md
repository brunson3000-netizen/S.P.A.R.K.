# GASTOWN — Disposable supervisory roles and liveness distinction

Status: SCOPED CODE STUDY COMPLETE; runtime qualification not executed.
Study date: 2026-09-14. Exact pin: `649b832b7672bc7a2dbef26f5983aba6198b819b`.
Upstream: https://github.com/gastownhall/gastown
Language/license at inspected root: Go / MIT. Individual dependency/reuse terms still need qualification.
Finding class: PATTERN / TEST-INVARIANT unless the disposition says idea/reference.
Primary disposition: BORROW_IDEA.

## Finding

Deacon heartbeat stores timestamp/action counts in a JSON file; reading missing, unreadable or corrupt data returns nil, which is considered very stale. Pause is a separate file; IsPaused returns false plus an error on invalid data. Callers must not discard that error and treat it as confirmed permission to continue.

## Traced paths

1. WriteHeartbeat → create directory → write heartbeat JSON → best-effort legacy liveness marker. ReadHeartbeat → decode or nil → Age/IsFresh/IsVeryStale. Timestamp freshness establishes reported liveness, not current authority, lease ownership or completed work.
2. Pause → write paused.json → IsPaused decode/state/error → Resume removes file. These routines do not themselves kill workers or enforce every consequential operation. Supervisor roles and patrol semantics remain separate from a deterministic permission boundary.

## Evidence

- [internal/deacon/heartbeat.go](https://github.com/gastownhall/gastown/blob/649b832b7672bc7a2dbef26f5983aba6198b819b/internal/deacon/heartbeat.go)
- [internal/deacon/pause.go](https://github.com/gastownhall/gastown/blob/649b832b7672bc7a2dbef26f5983aba6198b819b/internal/deacon/pause.go)
- [internal/deacon/pause_test.go](https://github.com/gastownhall/gastown/blob/649b832b7672bc7a2dbef26f5983aba6198b819b/internal/deacon/pause_test.go)
- [internal/daemon/daemon.go](https://github.com/gastownhall/gastown/blob/649b832b7672bc7a2dbef26f5983aba6198b819b/internal/daemon/daemon.go)

Pause tests assert missing-file behavior, corrupt/empty-file errors, read error, creation and resume. Test assertions inspected; no runtime execution. Daemon entry was inspected selectively; complete patrol-consumer enforcement is not established.

## MCI assessment

Keep role vocabulary and recovery cases as ideas for MCI's replaceable agents. Borrow neither file timestamps as authority nor a whole multi-agent hierarchy. Rust-owned durable task/attempt state remains the relevant consumer boundary.

## Limits and follow-up

Need generation-fenced ownership, atomic durable writes/reconciliation, malicious/future timestamps, pause-error consumers and process stop evidence before any supervisory code transfer.

These conclusions are research recommendations. No copied implementation, installed dependency, adopted policy or independent assurance is claimed.
