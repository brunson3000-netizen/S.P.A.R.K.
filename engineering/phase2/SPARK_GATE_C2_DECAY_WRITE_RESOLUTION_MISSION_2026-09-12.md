# Gate C2 — Operator decay resolution and implementation mission

## Authority and status

Recorded 2026-09-12 by the thread coordinator. The Operator stated: "I agree and authorize implementation of your suggestions. Go ahead and resolve."

This accepts the immediately preceding recommendation distinguishing additive updates from explicit replacements, within the active task to resolve all four open decay matters before Gate C2 acceptance. It does NOT accept Gate C2, promote production, authorize Phase 3, GAME changes, or paid external inference. Routine bounded implementation, tests, candidate commits, normal pushes and independent-review preparation are authorized. Preserve writer/reviewer separation.

Pinned prior candidate: 544c5f6f99d8dabf9855f8ac68f5666286aa9741.
Independent review/base: e00f248e25f6d34f1041e219f74085649714c19d.
Review: engineering/phase2/SPARK_PHASE_2_CODEX_GATE_C2_DECAY_ADJUDICATION_INDEPENDENT_REVIEW_2026-09-12.md.
Writer branch: candidate/phase2-gate-c2-decay-write-resolution-20260912.

This document records authority before implementation. Preserve it as history; any later clarification belongs in a new record. The earlier D-1–D-7 decision remains immutable.

## Approved behavior

1. Additive change: settle applicable overdue decay/recovery against the existing value first, then apply the additive change. An additive event must not erase unapplied earlier grid steps.
2. Explicit replacement: set the declared replacement value; earlier decay must not reduce that replacement. Neither replacement nor addition restarts the fixed grid.
3. Removal/restoration: no decay accrues while the operation is absent; restoration starts a fresh grid. Closing-segment whole steps remain applicable unless superseded by an explicit replacement; unfinished residual is discarded. This is the previously recommended remove/restore behavior, subject to the new additive settlement rule.
4. Same-time activations: process each in committed order; a parameter change and reversal still restart the grid. Unchanged parameters preserve it.
5. A single decay operation on an absent cell creates neither cell nor effect. This does not prohibit a distinct operation in a composed rule body from initializing a value.

Illustrative discriminator: baseline 0, value 100 at time 0, rate 10/cadence 10, additive +5 at 15: result at 15 is 95, and decay at 20 gives 85. Explicitly scheduling decay at 10 before the addition must give the same numeric outcome. Replacing with 100 at 15 instead gives 100 at 15 and 90 at 20. Recovery must be mirrored. These vectors do not promise full-history digest equality between different event histories.

The coordinator disclosed that settlement can interact with watchers and ordering. The Operator authorized implementing the recommendation, not a particular invented watcher protocol. Derive handling from retained frozen atomic-wave, canonical-time, effect reporting and causality rules. If these cannot determine a compatible solution, report the exact conflict and a concrete recommendation; do not silently amend additional architecture.

## Writer execution

Use the separated local writer (Opus). Fetch and verify the named base and branch; inspect applicable repository instructions and the complete controlling architecture stack. Preserve all worktrees and unrelated changes. Create an isolated worktree on the new branch; do not reset any existing checkout. This mission is already committed before implementation.

Read the review's four open-matter probes and reuse its established evidence: 400 workspace tests, all required gates and independent controls passed on the pinned candidate. Non-decay findings are already closed. Do not reopen them without changed dependencies.

Implement the approved semantics end to end. Before changing production logic, record a concise implementation contract mapping actual additive/replacement paths, direct command writes, scheduled effects and composed bodies, with ordering, activation ownership, bounds, failure atomicity, and reporting. Do not equate every RESULT or TRANSFORM with explicit replacement without examining its contract. Preserve unrelated update-family semantics. Resolve ordinary implementation choices autonomously.

Use existing fixed-grid calculation and activation lineage where sufficient. Preserve canonical commit times, no-new-cell-encoding, baseline/bounds, checked arithmetic, family-conflict refusal, report reapplication, provenance, deterministic replay and pacing. Do not introduce implicit backdated commits, unreported state mutation, a second charge at the same endpoint, or partial settlement on refused updates. Do not force an additive-plus-decay effect mixture through the existing forbidden family path or weaken its atomic rejection test.

Explicitly map any superseded interpretation/oracle to the new authority. The old lost-prewrite observation becomes historical behavior, not a passing normative requirement. Record new normative tests for all four formerly open matters. Preserve historical records and original review probes; any adapted probe must be a separate copy with its delta explained.

Meaningful tests must distinguish additive settlement from replacement; aligned/unaligned writes; repeated additions; no overdue step; both signs/recovery; saturation and zero rate; same-time ordering and no double charge; parameter changes and removal/restoration; absent cells; composed-body behavior; refusal atomicity; watchers/thresholds and effect reapplication; replay/restore/pacing on identical histories. Include a negative control demonstrating that the former lost-debt implementation fails the new additive test. Use bounded regression effort appropriate to the change, plus all required Gate C2 final validation gates.

Run workspace tests, fmt, all-target/all-feature clippy with warnings denied, strict core/engine lint, metadata, both relevant whitespace ranges, preservation, five static Windows/Android targets, release workload, affected historical/adapted probes and meaningful mutation controls. Retain performance and platform limitations honestly. Do not claim durable recovery, definition migration or multi-profile runtime.

Publish coherent candidate commits, a report and evidence under engineering/phase2/, and an exact independent-review mission. Verify local HEAD = tracking = live remote, and that production and unrelated refs remain preserved. Independent review must be performed by an agent other than this writer; do not label writer checks independent.

Finish with candidate branch/full SHA, decision dispositions, changed behavior, validation totals, remaining blockers, synchronization and review mission path. Gate C2 acceptance remains pending until resolution and independent review. No production promotion or Phase 3.

## Coordinator execution limitation

At recording time this web workspace has no Cargo executable. No Rust implementation or test execution is claimed by this record. The local writer must perform implementation and validation; the coordinator will retrieve its published evidence directly from GitHub.
