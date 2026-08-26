# ADR-0003 — Canonical Timeline Sequencer, Finality Fences, Deterministic Clock, RNG, and Commit Order

**Status:** REVISED AFTER SECOND INDEPENDENT PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN

## Context

S.P.A.R.K. requires deterministic replay across declared Windows/Linux/Android support builds. The host advances simulation time, but service request arrival order, concurrent clients, thread interleaving, transport batch partitioning, and delayed delivery must never construct canonical history accidentally.

The first Phase-0 review found that canonical input transaction boundaries were undefined. The v0.2 correction added an explicit command envelope, total-order ordinal, idempotency, command barriers, and transport-batch independence.

The second independent review confirmed those evaluation-side corrections but found one remaining ingress defect: the architecture named `input_ordinal` without defining who has exclusive authority to issue it or when an ordinal stream becomes final. A first-arriving conflicting command could therefore still win before a later collision was observed.

## Decision

### 1. One authoritative timeline sequencer per profile timeline epoch

Every canonical profile timeline has exactly one active `TimelineSequencerAuthority` for a given `timeline_epoch`.

The sequencer is an integration authority, not a causal-state primitive. It owns only the ordering/finality of external canonical inputs for that profile epoch.

The sequencer is normally the authoritative host adapter for the game profile and the sanitized MCI bridge for the MCI-social profile.

Rules:

1. only the active sequencer principal may stage state-changing/evaluation commands for canonical admission;
2. the sequencer capability is exclusive and non-composable within a profile epoch;
3. read-only queries do not require timeline ordinals and do not enter canonical history;
4. ordinary clients/actors/NPCs do not independently issue canonical ordinals;
5. multiple upstream sources must be ordered by the host/integration sequencer before they become canonical S.P.A.R.K. commands;
6. changing the sequencer requires an explicit epoch handoff/fence at a stable boundary; two sequencers cannot be active for the same profile epoch.

### 2. Canonical command envelope

Every command that can affect canonical state or canonical evaluation is staged using a `CanonicalCommandEnvelope` containing at least:

```text
command_id
profile_id
timeline_epoch
effective_time
source_id
source_sequence
input_ordinal
command_kind
canonical_payload_hash
```

`source_id/source_sequence` preserve provenance and source-local ordering validation. `input_ordinal` is issued by the active timeline sequencer and is the unique total-order position in that profile epoch.

Transport/session metadata is causally inert.

### 3. Staging is not canonical admission

Receiving a command does **not** make it canonical.

Commands first enter an isolated bounded staging buffer keyed by:

```text
(profile_id, timeline_epoch, input_ordinal)
```

Staging rules:

- identical duplicate envelope/payload is idempotent;
- two different payloads for the same staged ordinal poison that ordinal and prevent finalization;
- higher ordinals may arrive before lower ordinals and remain non-canonical staged data;
- no staged command may mutate canonical state, advance canonical time, consume canonical RNG occurrence identity, or emit canonical outputs.

This removes first-arrival authority.

### 4. Timeline finality fence

Only the active sequencer can submit a `TimelineFence`.

A fence contains at least:

```text
profile_id
timeline_epoch
fence_id
start_ordinal
end_ordinal
previous_fence_hash
ordered_stream_digest
```

The digest commits to the complete ordered canonical representation of every envelope/payload in the fenced ordinal range.

A fence is accepted only when:

1. it comes from the active sequencer authority;
2. its epoch matches the active profile timeline epoch;
3. `start_ordinal` equals the next unfinalized ordinal;
4. every ordinal through `end_ordinal` is present exactly once and unpoisoned;
5. the ordered-stream digest matches;
6. `previous_fence_hash` matches the last finalized fence;
7. no canonical command in the range has previously been finalized differently.

If any condition fails, the entire fence is rejected and **none** of its staged commands become canonical.

### 5. Finalized stream

A successfully validated fence atomically promotes its contiguous command range into the canonical profile input stream.

Only after promotion are commands processed, in ascending `input_ordinal`, through their individual stable command barriers.

Therefore:

```text
arrival order != canonical order
staging != admission
fence validation == finality decision
```

A later conflicting arrival cannot replace a finalized command. Before finalization, a collision blocks the fence rather than allowing whichever command arrived first to win.

### 6. Gap and late-arrival rules

- A fence cannot skip an ordinal.
- An unfenced higher ordinal may wait in the bounded staging buffer.
- A command below the next unfinalized ordinal that exactly matches its finalized canonical hash is an idempotent late duplicate.
- A command below the next unfinalized ordinal with a different hash is rejected as a finalized-history conflict.
- Staging-buffer limits are governed by the Phase-0 performance/security budget. Overflow rejects new staging atomically; it cannot force canonical reordering.
- Wall-clock timeout is never used to decide canonical order or fill a gap.

### 7. Sequencer epoch handoff

Changing sequencer ownership or resetting ordinal space requires:

```text
complete stable canonical boundary
-> final fence for old epoch
-> explicit epoch-handoff record
-> new timeline_epoch
-> new exclusive sequencer grant
-> ordinal space starts according to the declared epoch contract
```

The handoff is itself persisted and auditable. An old-epoch sequencer cannot stage/finalize commands in the new epoch.

### 8. Embedded convenience API

Embedded integrations may expose a convenience call that stages one or more commands and immediately supplies a matching fence.

That convenience API is only syntactic sugar. It must execute the same logical stage/fence/finalize rules as service mode.

### 9. Command barrier after finality

For each finalized command:

```text
current stable snapshot
-> apply/validate direct canonical ingress
-> evaluate all permitted zero-delay work
-> deterministic effect collection/sort/validation
-> atomic commit wave(s)
-> stable boundary
-> next finalized command
```

The next canonical command never observes an incomplete prior barrier.

### 10. Logical time and `advance_time`

- wall-clock time is never canonical;
- host simulation time is monotonic integer time;
- finalized commands are processed by `input_ordinal`, and their declared `effective_time` must satisfy the timeline/time policy;
- `advance_time(target)` processes due work deterministically before the committed clock moves beyond it;
- runtime quota/yield may change latency only, not occurrence identity, ordering, or hashes;
- one ten-day advance versus ten one-day advances with no intervening canonical inputs must converge to the same final canonical state/output hashes.

### 11. Scheduled occurrence identity

Scheduled/recurring work derives occurrence identity from persisted schedule/obligation identity and logical occurrence index, never from:

- arrival order;
- worker count;
- transport batch;
- catch-up chunk count;
- runtime quota resume count.

### 12. Profile/config activation

Any profile/config activation that can alter canonical outcomes is itself a sequenced, fenced canonical command and activates only at its declared stable barrier.

### 13. Snapshots

Persistence snapshots represent complete stable canonical commit boundaries only.

### 14. Randomness

Canonical stochastic decisions derive semantic random addresses from:

```text
root_seed
+ exact behavior epoch/artifact context
+ rule_or_trigger_id
+ scope_or_actor_id
+ persisted logical occurrence/index
```

No mutable global RNG stream exists.

### 15. Numeric model

Canonical probability/score arithmetic uses integer/fixed-point representation where practical. Accepted wire/config representations normalize before hashing/evaluation; ambiguous representations reject.

## Why a fence instead of arrival-time sequencing

A single service thread would still make network arrival order canonical. Rejecting `n+1` merely because it arrived before `n` would make the accepted command set depend on delivery order.

Staging plus a sequencer-authored digest fence allows commands to arrive in any order while making finality depend only on the sequencer's explicit ordered commitment.

## Consequences

- Neither of two conflicting same-ordinal commands can become canonical merely by arriving first.
- `(n+1, n)` and `(n, n+1)` delivery produce the same finalized stream when the same valid fence is supplied.
- A gap cannot be silently skipped.
- Service and embedded modes share one finality contract.
- Timeline sequencing is explicit integration authority and does not add a causal runtime primitive.

## Required falsification tests

1. Stage ordinals `(n+1, n)` and `(n, n+1)` with the same valid fence; both finalize identical commands/hashes.
2. Two different payloads claim ordinal `n` before finalization; the slot is poisoned and the fence rejects atomically regardless of arrival order.
3. A non-sequencer source claims a valid-looking ordinal; staging rejects before canonical mutation.
4. Submit `n+1` without `n`; a fence through `n+1` rejects for a gap and neither command becomes canonical.
5. Late exact duplicate below the finalized frontier is idempotent; late differing payload rejects as history conflict.
6. Same staged commands with different fence digest reject without partial finalization.
7. Replayed/branched fence with wrong `previous_fence_hash` rejects.
8. Old-epoch sequencer attempts staging/finalization after handoff; reject.
9. Service delivery and embedded convenience API with reversed arrival produce identical accepted/finalized streams and canonical hashes.
10. Staging-buffer overflow never advances canonical order or state.
11. Ten-day once vs one-day ten-times remains hash-equivalent with quota yielding.
12. Concurrent delivery/worker interleaving cannot change a finalized fenced stream.
