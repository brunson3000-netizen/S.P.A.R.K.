# ADR-0003 — Canonical Timeline Sequencer, Ordinal Credit Window, Finality Fences, Deterministic Clock, RNG, and Commit Order

**Status:** REVISED AFTER THIRD INDEPENDENT PHASE-0 REVIEW  
**Date:** 2026-08-25  
**Decision class:** Foundational / FROZEN

## Context

S.P.A.R.K. requires deterministic replay across declared Windows/Linux/Android support builds. Canonical history must not depend on network arrival order, concurrent delivery, worker interleaving, transport batching, or bounded-buffer accident.

Previous Phase-0 corrections established:

- one exclusive authoritative timeline sequencer per profile timeline epoch;
- canonical command envelopes;
- causally inert staging;
- unique sequencer-issued ordinals;
- digest/hash-linked finality fences;
- contiguous finalization;
- command barriers after finality;
- immutable authority and exact behavior-artifact continuation.

The third independent review found one remaining ingress hole: a generic bounded staging buffer that “rejects the new request when full” can crowd out the next canonical frontier ordinal if higher ordinals happen to arrive first.

## Decision

### 1. One authoritative timeline sequencer per profile timeline epoch

Every profile timeline epoch has exactly one active `TimelineSequencerAuthority`.

Only that authority may submit canonical state-changing/evaluation commands and finality fences for the profile epoch.

Multiple upstream sources are ordered by the host/integration sequencer before they become candidate canonical S.P.A.R.K. inputs.

### 2. Canonical command envelope

Every canonical state-changing/evaluation command contains at least:

```text
command_id
profile_id
timeline_epoch
effective_time
source_id
source_sequence
input_ordinal
admission_window_token
command_kind
canonical_payload_hash
```

`input_ordinal` is issued by the active sequencer.

Transport/session metadata is not canonical.

### 3. Deterministic ordinal credit window

At every unfinalized frontier, the engine exposes a deterministic `AdmissionWindow`:

```text
profile_id
timeline_epoch
frontier_ordinal
window_width
window_end = frontier_ordinal + window_width - 1
last_finalized_fence_hash
admission_window_token
```

`admission_window_token` is deterministically derived from the profile/timeline epoch, current frontier, configured window width, and last finalized fence hash.

The active window is therefore not chosen by request arrival.

`window_width` is finite, declared by the deployment/profile compatibility contract, and included in service/embedded conformance context.

### 4. One logical slot per ordinal

The staging structure is not a generic first-come queue.

It is a fixed ordinal-addressed slot set for the current admission window.

For every ordinal in:

```text
[frontier_ordinal, window_end]
```

there is exactly one logical staging slot.

Consequences:

- a higher ordinal can never consume the frontier ordinal's capacity;
- at most one distinct payload identity occupies a slot;
- identical duplicate for the same slot is idempotent;
- a different payload for the same slot poisons that ordinal;
- duplicates/conflicts do not consume additional staging capacity.

### 5. Admission rule

A state-changing/evaluation envelope is eligible for staging only when:

1. sequencer authority is valid;
2. profile/timeline epoch matches;
3. `admission_window_token` is valid for the window under which the sequencer submitted it;
4. `input_ordinal` lies within that token's ordinal range;
5. command/source/idempotency validation passes.

An ordinal outside the envelope's declared/tokenized window is never admitted merely because network delay causes it to arrive after the frontier later moves.

### 6. Out-of-window behavior: retry, never eviction

An otherwise valid command whose ordinal is outside its tokenized credit window receives a retryable logical result such as:

```text
NOT_IN_ADMISSION_WINDOW
```

It:

- does not consume a slot;
- does not mutate canonical state;
- does not poison another ordinal;
- is not evicted into a different slot;
- is not silently buffered in an unbounded overflow queue.

The sequencer may retry it only after obtaining the later admission-window token under which its ordinal is eligible.

There is **no arrival-based eviction policy**.

### 7. Stage acknowledgement before fence

The sequencer must not submit a finality fence for an ordinal range until it has received a positive logical `STAGED` acknowledgement for every ordinal in that range.

A positive staging acknowledgement identifies:

```text
profile_id
timeline_epoch
input_ordinal
command_id
canonical_payload_hash
staged_slot_state
```

This is a protocol precondition, not an advisory suggestion.

Therefore a compliant fence cannot race ahead of the commands it claims to finalize.

### 8. Timeline finality fence

A `TimelineFence` contains at least:

```text
profile_id
timeline_epoch
fence_id
start_ordinal
end_ordinal
previous_fence_hash
ordered_stream_digest
```

Fence rules:

1. only the active sequencer may submit it;
2. `start_ordinal` equals the current frontier;
3. `end_ordinal` is within the currently stageable/finalizable ordinal horizon;
4. every ordinal in the range is positively staged exactly once and unpoisoned;
5. the digest matches the ordered canonical envelopes/payloads;
6. `previous_fence_hash` matches the finalized chain;
7. the range is contiguous;
8. failure rejects the fence atomically and promotes nothing.

A successful fence promotes the range into canonical history, then commands execute in ascending ordinal order through individual stable command barriers.

### 9. Sliding window after prefix finalization

A fence may finalize a contiguous prefix of the current ordinal window.

After finalization:

- `frontier_ordinal` advances to `end_ordinal + 1`;
- a new deterministic admission-window token is issued;
- already-staged unfinalized higher ordinals that remain within the new window keep their dedicated slots;
- newly exposed higher ordinal slots become available;
- new submissions for newly exposed ordinals require the new token.

This preserves bounded capacity while ensuring the next frontier always has a reserved slot.

### 10. Reversed-delivery capacity invariant

For `window_width = 2` and frontier `n`, the only eligible ordinals are:

```text
n
n+1
```

So both delivery orders:

```text
n+1, n+2, n
n, n+1, n+2
```

produce the same logical staging result:

```text
n     -> STAGED
n+1   -> STAGED
n+2   -> NOT_IN_ADMISSION_WINDOW
```

A fence through `n+1` can then finalize identically in both cases.

After finalization, `n+2` may be retried with the new admission-window token.

### 11. Poison/cancellation/recovery policy

There is no arrival-based eviction or winner selection.

Within a timeline epoch:

- an unpoisoned staged exact duplicate is idempotent;
- a conflicting payload poisons the ordinal;
- a poisoned ordinal cannot be “fixed” by replacing one competing payload based on arrival time;
- there is no ordinary per-command cancellation that can race submission and choose history.

Recovery from a poisoned/unrecoverable staging state requires an explicit sequencer epoch handoff/reset at a stable canonical boundary:

```text
freeze current finalized frontier
-> discard all unfinalized staged state for old epoch
-> persist handoff/reset evidence
-> create new timeline_epoch
-> grant one sequencer
-> recompute admission window from the unchanged finalized frontier
-> resubmit intended unfinalized commands under the new epoch
```

No finalized history is altered.

### 12. Generic ingress/backpressure is outside canonical staging

Transport implementations may have ordinary bounded socket/request queues before logical staging.

Such transport backpressure may reject/drop a request before it becomes a logical staging attempt, but conformance/equivalence tests must operate within declared transport resource limits and compare the same logical submissions.

No transport queue may be treated as the canonical ordinal staging store.

### 13. Embedded convenience API

Embedded mode may combine:

```text
obtain current window
-> stage eligible commands
-> receive STAGED acknowledgements
-> submit fence
```

inside one convenience call.

That is syntactic sugar only. The same logical eligibility, slots, acknowledgements, digest, frontier, and finality semantics apply.

### 14. Command barrier after finality

For every finalized command:

```text
stable snapshot
-> direct canonical ingress
-> deterministic due/zero-delay evaluation
-> collect/sort/validate effects
-> atomic commit wave(s)
-> stable boundary
-> next finalized ordinal
```

### 15. Logical time, scheduled occurrences, snapshots, RNG, and numeric model

The existing frozen rules remain:

- wall-clock time is never canonical;
- host simulation time is monotonic integer time;
- quota/yield changes latency, not semantics;
- scheduled occurrence identity derives from persisted logical occurrence indexes, never batching/worker/catch-up chunks;
- profile/config activation is sequenced and barrier-bound;
- snapshots represent complete stable boundaries;
- randomness uses semantic random addresses rather than a mutable global RNG stream;
- canonical arithmetic/serialization is normalized and integer/fixed-point where practical.

## Why this closes the bounded-overflow defect

Capacity is allocated by ordinal position, not arrival.

Higher ordinals cannot fill or evict the frontier's slot.

Commands beyond the deterministic credit window are not opportunistically accepted depending on when they arrive; their envelope carries the window token under which the sequencer submitted them, and they must be explicitly retried under a later valid token.

## Required falsification tests

1. capacity two: `n+1,n+2,n` versus `n,n+1,n+2` yields identical stage statuses and identical finalization through `n+1`;
2. deliver `n+2` late after frontier advances but with the old token: still reject; retry with new token succeeds;
3. frontier slot remains available after all higher eligible slots are occupied;
4. opposite-order same-ordinal conflict poisons identically and consumes only one ordinal slot;
5. no arrival-based eviction exists;
6. fence cannot be issued successfully without positive STAGED acknowledgements for its full range;
7. partial-prefix fence slides the window deterministically and preserves still-valid staged higher ordinals;
8. poisoned-window recovery uses explicit epoch reset and cannot rewrite finalized history;
9. service, embedded, and concurrent-worker delivery paths produce identical logical stage/fence results;
10. transport backpressure is proven separate from logical canonical staging.
