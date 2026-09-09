# First-principles sketch (written before re-reading the correction history)

**RESEARCH / PROPOSED DESIGN — NOT ADOPTED.** Written 2026-09-09 05:13Z after reading the
product intent, blueprint §3/§11/§18/§19, the Phase-1 kernel, and the serialized-request
addendum, and before reading the V3-F01 correction candidates in detail. Kept unedited so
the later design can be compared against it.

## What Spark computes

Spark is a **deterministic discrete-event simulator** over a typed, scoped key-value
state, driven by declarative rules:

1. A priority queue of timed work keyed by semantic identity (already exists: `Scheduler`).
2. Each unit of work evaluates rules **at its own due time** against committed state,
   produces a batch of effects, commits the batch, and may schedule strictly-later work.
3. External inputs (host observations, commands) are timed events too; they enter the same
   time-ordered stream.
4. Outputs are advisory intents derived from committed state; the host executes or not and
   reports confirmed outcomes as new inputs.

Everything else — waves, cohorts, emission identity — exists only to make **same-time
batches commit deterministically** regardless of iteration order.

## The minimum engine loop

```
process(request, budget):
  reject if request.horizon < clock.now()          // never simulate backwards
  while let Some(t) = scheduler.least_due_time():
     if t > horizon: break                          // beyond horizon: leave resident
     if budget exhausted: return Paused             // nothing to remember
     cohort = scheduler.drain_due(t)                // exactly the work at time t
     transaction = evaluate(cohort, now = t)        // all waves on an overlay
     commit-or-reject(transaction)                  // whole cohort atomic
  if request is a command: execute it at now = horizon (one more unit)
  clock.advance_to(horizon); return Completed
```

Pause needs no state: a paused request is re-presented and the loop restarts from live
state. The only "frontier" is the existing logical clock.

## What is hard for real

- Same-time determinism: sort effects by semantic identity, fold exact duplicates, poison
  contested slots. (Phase 1 already does this for scheduler keys and timeline ordinals.)
- Time semantics of late-arriving host inputs: the host's clock is authoritative, so a
  request dated before the engine clock is a host error, rejected non-canonically.
- Bounded work per host call without changing results: budget counts cohorts, never splits
  one, and evaluation time is the cohort's own due time, not the horizon.
- Integer-only, canonically encoded state so three platforms hash identically.
- Authority: host-owned cells written only by host observations; Spark-owned cells only by
  effects; derived cells only by the evaluator. (Phase 1 already enforces this.)

## What looks harder than it needs to be

- Multi-profile cohort sequencing inside one engine: one engine instance per profile
  removes the `(due_time, profile_id)` cohort question entirely.
- A dedicated cohort-extraction API: `drain_due(least_due_time)` already extracts exactly
  one time-slice and leaves later slots resident.
- Per-emission causal identity with parent-set digests: needed for explanation and
  duplicate folding, but the minimum version can fold by `(rule, target)` and record the
  cohort's committed effect list; the parent-set hash is an explanation feature.
- Three-level digest algebra before any evaluator exists.

## Retained state (minimum)

| State | Read by | Reconstructed how |
|---|---|---|
| cells (value, updated_at) | every rule read; closed-form decay uses updated_at | snapshot |
| scheduler slots | the loop; delayed work | snapshot |
| clock (= last completed horizon) | request admission; decay elapsed | snapshot |
| finalized command log (id -> digest) | duplicate-command rejection; replay | snapshot |
| occurrence counters per (producer, scope) | new WorkKey allocation | snapshot |

Nothing else. Per-request progress, pacing diagnostics, and mailbox contents are not
engine state.
