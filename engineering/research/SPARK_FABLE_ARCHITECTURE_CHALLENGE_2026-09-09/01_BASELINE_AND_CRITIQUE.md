# Baseline and evidence-based critique

**RESEARCH / PROPOSED DESIGN — NOT ADOPTED.** 2026-09-09.

## 1. Inspected baseline (live, not assumed)

| Item | Value |
|---|---|
| Repository | `/home/chromikey/Projects/SPARK` |
| Branch | `phase1-refoundation-v2` (ahead of origin by 9; `master`, `phase1-refoundation` also exist) |
| HEAD at mission start | `47729bb719d8ae95ae00c4e195fa64d0e27bfdb7` "Reconcile S.P.A.R.K. headless compute evidence" |
| Worktree | clean at start; untouched by this mission |
| Toolchain | cargo 1.98.0, rustc 1.98.0 |
| Baseline tests | `cargo test --workspace --all-features`: **232 passed, 0 failed** across 13 test binaries/doc-test sets (matches the Phase-1 closure claim) |
| Rust source | 9,309 lines in `crates/spark-core`, `crates/spark-engine`, `crates/spark-testkit`; 4,999 lines of tests |
| Specification | phase0 4,768 + phase1 7,771 + phase2 9,665 markdown lines |
| Evaluator, rules, host facade, persistence codec | **none implemented** (Phase 2 unauthorized) |
| Isolated research location | worktree `/home/chromikey/Projects/SPARK-fable-challenge-worktree`, branch `research/fable-architecture-challenge-2026-09-09`, created from `47729bb` |

## 2. What the archive actually contains, sorted

**Implemented and tested (Phase 1, closed).**
- `spark-core`: bounded ids/tags, scopes, canonical values, `LogicalClock`, `Scheduler` keyed by the full semantic `WorkKey` with order-independent conflict poisoning, random-address derivation, `TimelineIngress` (sequencer authority, admission window, staging acks, digest-linked fences, epoch reset), `CanonicalEncoder`/blake3 digests. Panic-freedom enforced by clippy deny lints. No `unsafe`.
- `spark-engine`: the single activation door, unforgeable `ActivatedProfile`, `StateStore` with three authority-specific `pub(crate)` write paths and canonical digest.
- `spark-testkit`: scenario replay harness, adversarial corpora, workspace dependency-direction and feature-hygiene tests.
- Windows/Android: `cargo check` only (static). No executable cross-platform parity evidence exists.

**Adopted requirements (frozen, operative).** Blueprint v0.2, ADR-0001…0006, Phase-2 freeze v3 (cohort = `(profile, due_time)` slice; pacing selector; canonical-vs-diagnostic separation; emission identity with parent-set digests; `effect_batch_digest` composition). Convergence Protocol V1 (Gate C1–C5).

**Unaccepted proposals (all still open).** V3-F01 correction candidate (F-01…F-07 returned), foundational time adjudication (horizon expansion, per-barrier `now`), candidate V2 (compare-and-take, extraction slices), command-time adjudication (unified pending-command loop, cursor `X`, frontier `Φ`), serialized-request addendum (`F`, `stable_boundary_digest`, finalize-at-completion). Only the last one has an executable model, and it is Python, not the kernel.

**Demonstrated defects.**
- V3-F01 itself: a spec/mechanism mismatch. The freeze requires per-cohort extraction before each pre-wave digest; `drain_due(now)` removes the whole due prefix. Both statements are true. **It is not a runtime defect** — no runtime exists — and it is closed by calling `drain_due` with the least due time (example W1 reproduces the wrong shape and C2 shows the fix).
- Review-found oracle errors (V2-03, V2-04, V2-07): false expected values in unimplemented tests. These expose mistaken specifications, not runtime behavior.
- Review-found spec gaps (V2-02 `Φ` undefined; V2-05 extraction not one operation; V2-06 wave-versus-cohort atomicity): real, but all three vanish under whole-cohort transactions plus the clock-as-frontier (design §5–§6).

**Unsupported assumptions (carried since Phase 0, never measured).**
- That requests are sparse and bubble-concentrated with timers elsewhere: plausible; no fixture measures it. The example's telemetry channel is the shape of the measurement, not the measurement.
- That embedded and standalone forms will share the same request sequence: true by construction once the mailbox is the only entry, but no standalone form exists.
- That three-platform digest parity follows from integer math and canonical encoding: likely, but only Linux has executed anything.
- That multi-profile sequencing inside one engine is a product need: nothing in the game intent requires it.

**Persistence gap (new finding).** ADR-0006 defines persistence/versioning, but Phase 1 shipped no snapshot surface at all: `Scheduler` has no iteration or encode method, `StateStore` has `get` but no cell enumeration, and no crate has a codec. A save/load-capable game module cannot be built on the current surface without an addition to `spark-core` and `spark-engine`. This gap was invisible to the V3-F01 process because that process never needed to persist anything.

## 3. Diagnosis

**What is Spark's essential computation?** A deterministic discrete-event simulation over a
typed, scoped integer state, driven by declarative rules, in which every unit of work runs
at its own logical time, commits as a batch, and may schedule strictly-later work; outputs
are advisory intents derived from committed state. The Phase-1 kernel already contains the
queue, the clock, the store, and the hashing. What is missing is the loop and the rules.

**What is genuinely difficult.** Same-time determinism (solved in Phase 1 by semantic keys
and poison rules); time semantics of host inputs that arrive after the engine has already
simulated past their timestamp; bounding work per host call without changing results;
integer-only canonical encoding for three platforms; and keeping the authority boundary
structural. These are all one system: ordering, logical time, retained state, error
behavior, and the game boundary meet in the request loop, which is why repairing sentences
in the freeze one at a time kept producing incompatible neighbors.

**What has become difficult because of our design or process.**
- **Spec-before-loop.** Phase 2 has 9,665 lines of architecture and zero lines of evaluator.
  Digest algebra (`engine_state_digest`, `effect_batch_digest`, `stable_boundary_digest`,
  emission identity, parent-set digests, cohort identity) was specified in prose and reviewed
  in prose across seven passes. Every V2 review finding about "false assertions" is a
  consequence of writing acceptance tests for code that does not exist.
- **Mechanism inflation around V3-F01.** The candidate lineage introduced an owned selection
  token, then a least-due compare-and-take with full slot fingerprints, extraction slices,
  a frontier `Φ`, an execution cursor `X`, a finalized ceiling `C`, and amendments A-1…A-3,
  most of which the next pass withdrew. The fix that survives all of them is a loop that
  passes a different argument to an existing function.
- **Generality beyond the game.** Multi-profile cohorts in one engine; a general
  multi-sequencer timeline with admission windows and epoch handoff used, in practice, by one
  in-process host that fences one command at a time; per-emission causal identity with
  transitive parent-set hashing before any rule has ever emitted anything.
- **Process weight.** Writer/reviewer separation is valuable; eleven documents per finding
  is not. The addendum's Python model was the first executable artifact in Phase 2 and it
  resolved more in one afternoon than the six prose passes before it.

**Which components already work and deserve preservation.** All of Phase 1: the scheduler
(the example runs on it unmodified except one read-only accessor), the clock, hashing and
canonical encoding, ids/values/scopes, the activation door and `StateStore` authority
paths, the timeline ingress as the eventual multi-sequencer surface, the testkit and its
hygiene tests. The example needed nothing else from the kernel.

**Which abstractions earn their complexity.** `WorkKey` as full semantic identity (yes:
it is what makes catch-up and snapshot correct). Poison-on-conflict (yes: cheap and it
removes arrival-order authority). The activation ceremony as a crate boundary (yes: it
ended B-02). Canonical-versus-diagnostic separation (yes). **Can be removed, merged, or
deferred:** per-`(profile, due_time)` cohort sequencing (one engine per profile);
compare-and-take/extraction slices (replaced by `drain_due(least)`); `Φ`, `X`, `C`
(replaced by the existing clock); `stable_boundary_digest` as a separately named concept
(it is simply `H(engine_digest ‖ clock)`, kept, but with nothing new to specify); emission
identity and parent-set digests (deferred to the explanation feature); the multi-sequencer
admission-window usage (kept in the kernel, used degenerately).

**Which review findings expose actual runtime defects?** None — there is no runtime. The
findings that would have become runtime defects are: drain-whole-prefix (W1), evaluate-at-
horizon (W2, implied by the foundational adjudication's rejection of horizon-time
evaluation), and unbounded zero-delay feedback (C5). The rest expose mistaken specification
or oracle text.

**Which unresolved questions really block a working module?** Three, all decisions rather
than research: (1) the transaction unit (time slice vs. per-`WorkKey`, see
`04_SELF_CHALLENGE.md`); (2) late-dated host inputs: reject or re-date at the adapter;
(3) the snapshot encoding and its home crate. Everything else in the V3-F01 lineage is
either resolved by the loop shape or deferrable.

**Where have we demanded more generality than the game needs?** Multi-profile engines,
multi-sequencer timelines, per-emission causal identity, and a three-digest algebra defined
before a single rule ran. The game needs one profile, one host, a batch record per cohort,
and a save file.

## 4. On the existing freezes

The freezes remain operative. This document records the following as **proposals** only:
that the v3 cohort rule be satisfied by `drain_due(least_due)` rather than by a new
extraction API; that `F` be recognized as the existing `LogicalClock`; that emission
identity be deferred to a later explanation feature; that one engine serve one profile; and
that the Phase-2 acceptance matrix be replaced by executable fixtures against a real loop.
