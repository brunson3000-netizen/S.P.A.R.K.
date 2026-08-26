# Re-Foundation Step 2 — findings not expressible against the inherited API

Recorded: 2026-08-26
Branch: `phase1-refoundation`
Inherited HEAD: `1ac3e3b`

Fifteen of the still-open counterexamples in
`PHASE_1_REFOUNDATION_BRIEF_v0.1.md` were encoded as executable tests and
all fifteen fail against the inherited implementation; see
`BASELINE_ADVERSARIAL_FAILURES.txt`.

The following remaining requirements could not be expressed as a *runtime*
assertion against the inherited API, because the API surface they require
does not exist at all. Their baseline is therefore "does not compile:
required API absent", which is a stronger failure than an assertion
failure, not a weaker one. Each is encoded as a permanent test against the
re-founded API.

| Brief requirement | Inherited API | Baseline |
| --- | --- | --- |
| Finality/admission 3 — structured `StageAcknowledgement` binding profile, epoch, ordinal, command ID, canonical semantic-envelope hash, slot state | `stage` returns the bare enum `StageOutcome`; no acknowledgement record type exists | does not compile: no `StageAcknowledgement` type |
| Finality/admission 4 — structured finalization result identifying the finalized fence/range and canonical result | `submit_fence` returns `Result<(), FenceError>` | does not compile: no finalization-result type |
| Schema provenance 3/6 — trusted activation artifact; post-activation schema mutation structurally unavailable | `DefinitionSchema` has all-public fields and `StateStore::new` accepts raw entries | expressed at runtime as forgery/duplicate-key tests (both fail); the *structural* claim is encoded as `compile_fail` doc-tests against the re-founded API |
| Bounds 5 — timeline window arithmetic cannot panic/wrap | `window_end()` panics via `expect`, but no constructor can place the frontier near `u64::MAX`, so the panic is unreachable from outside the crate | not reachable through the inherited API; the re-founded API adds `TimelineIngress::resume_at_frontier` so the property becomes falsifiable |
| Bounds 6 — fence finalization atomic if frontier advance would overflow | same: `submit_fence` promotes commands before checking `end_ordinal + 1`, but the state is unreachable from outside the crate | not reachable through the inherited API; falsifiable after `resume_at_frontier` |
| Canonical types 4 — command kind and scheduler work kind must be bounded canonical types, not raw `String` | both are `pub String` | runtime oversized-value tests fail (recorded); the *type-level* claim is encoded as `compile_fail` doc-tests |
