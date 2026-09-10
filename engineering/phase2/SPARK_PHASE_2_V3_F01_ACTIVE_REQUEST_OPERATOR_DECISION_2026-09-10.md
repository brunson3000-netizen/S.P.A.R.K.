# S.P.A.R.K. V3-F01 — ActiveRequest Operator Decision

**Date:** 2026-09-10

**Authority:** Operator

**Decision:** `ACTIVE_REQUEST_AUTHORIZED`

The Operator authorizes the recommended minimal engine-owned `ActiveRequest` mechanism
identified by the independent serialized-boundary review.

The Phase-2 v1 section 8 correction writer shall add:

- `HorizonFrontier F`, a canonical request-boundary scalar committed in the
  `stable_boundary_digest`, snapshot-carried, and covered by AT-G
  discrimination/equivalence from its first implementation commit; and
- `ActiveRequest`, a canonical request-boundary `Option<RequestDiscriminator>` present
  while a request is in progress or paused, committed in the `stable_boundary_digest`,
  snapshot-carried, and covered by AT-G discrimination/equivalence from its first
  implementation commit.

The request discriminator binds at least the request kind, request identity or complete
canonical payload digest, and logical horizon. Starting a request records the
discriminator before any cohort mutation. Only an identical request may resume it. A
different request receives a typed, non-mutating refusal. Completion atomically clears
`ActiveRequest` and sets `F` to the completed request horizon. A start-time refusal
mutates neither value.

`engine_state_digest` remains the per-cohort pre-wave digest and excludes request-boundary
protocol state. Snapshots, restore validation, and equal-state tests use:

```text
stable_boundary_digest = H("stable_boundary_v1"
                           || engine_state_digest
                           || F
                           || canonicalize(ActiveRequest))
```

This decision authorizes the separated final V3-F01 architecture correction writer pass
and its independent review. It does not by itself declare V3-F01 closed, accept the
writer's future text, authorize Phase-2 production Rust, authorize Phase 3, or change
G.A.M.E.
