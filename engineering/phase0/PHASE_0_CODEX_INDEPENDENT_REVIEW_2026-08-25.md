### 1. VERDICT

`REVISE_PHASE_0`

The primitive grammar is viable, but three foundational contracts remain underspecified in ways that could silently change canonical behavior during Phase 1.

### 2. BLOCKERS

#### B-01 — Canonical input transaction boundaries are undefined

- **Affected artifact/section:** [ADR-0003](/home/chromikey/Downloads/spark_phase0/ADR-0003-deterministic-clock-rng-and-commit-order.md), “Logical time,” “Commit waves”; [ADR-0005](/home/chromikey/Downloads/spark_phase0/ADR-0005-service-and-embedded-semantic-equivalence.md), equivalence criterion; matrix R-003, R-038, R-041, R-044.
- **Defect:** Stable ordering inside a commit wave is specified, but the architecture does not define how concurrent messages, split batches, multiple sessions, or same-time commands become canonical waves. “Ordered logical host inputs” assumes away the service/embedded boundary being tested.
- **Concrete failure scenario:** At time 100, the host submits a food-price update and an actor evaluation. The service processes two HTTP requests sequentially, so the actor sees the new price. Embedded mode submits both in one batch evaluated against a stable pre-update snapshot, so the actor sees the old price. Both paths received the same logical commands but produce different behavior.
- **Minimum correction:** Define a canonical command envelope and transaction/barrier rule, including effective logical time, source sequence, duplicate/idempotency handling, same-time total order, and when a snapshot closes. Transport batches must be packaging only. Ambiguous conflicting commands must either be explicitly ordered or rejected.

#### B-02 — Profile administration can change canonical authority

- **Affected artifact/section:** [ADR-0002](/home/chromikey/Downloads/spark_phase0/ADR-0002-authority-and-host-acknowledgement.md); [ADR-0004](/home/chromikey/Downloads/spark_phase0/ADR-0004-profile-schema-validation-and-change-classes.md), “Three change classes”; matrix R-005, R-007, R-012–R-016, R-062.
- **Defect:** Authority is required on definitions, but authority assignment is not declared immutable for a persisted ID. Changing an existing definition from `host_owned` to `spark_owned` can currently be classified as an ordinary structural profile revision rather than an authority change.
- **Concrete failure scenario:** A profile administrator reloads `state.economy.money` with the same ID but changes its authority to `spark_owned`. A propagation rule can then commit money changes while all individual definitions still pass the “exactly one authority mode” validator.
- **Minimum correction:** Make authority/write class part of immutable definition identity. Changing it requires a new ID, explicit migration, an authority ADR, and operator approval. Configuration, reload, restore, alias, and migration paths must all be unable to fabricate or commit host-owned/derived truth.

#### B-03 — Old-save continuation is not bound to exact behavior artifacts

- **Affected artifact/section:** [ADR-0006](/home/chromikey/Downloads/spark_phase0/ADR-0006-persistence-versioning-and-bounded-provenance.md), “Versioning”; ADR-0004, “Stable IDs”; matrix R-012, R-015, R-055, R-060.
- **Defect:** Saves store versions and an epoch, but the package does not require immutable content-addressed manifest/config artifacts or bind delayed work to the rule version that created it. “Unsupported migration fails” is not by itself the blueprint’s requirement that old saves retain or reference what they need to continue or migrate.
- **Concrete failure scenario:** A drought rule schedules a six-month obligation. The profile is structurally reloaded and the old manifest is replaced. After loading the save, the obligation executes using the new rule topology or cannot resolve its original rule. Continuation is neither old behavior nor a declared migration.
- **Minimum correction:** Define the supported compatibility envelope and require immutable manifest/config hashes, retained or resolvable artifacts, epoch/version binding for scheduled obligations and relevant state, atomic migration/checkpoint activation, and explicit separation of mechanism replay from outcome replay.

### 3. MAJORS

#### M-01 — Global scope can become an O(total-world) fan-out

Matrix R-019, R-030, R-040, R-058, and R-070 claim coverage through semantic scopes and scheduling, but no invariant prevents a world-scoped blood moon from materializing fear or belief state for every dormant actor.

Require scoped/inherited exposure to remain aggregate until a target is due or activated, or an equivalent bounded mechanism. Add a scaling test proving a global trigger does not enumerate total population or addressable cells.

#### M-02 — Several frozen requirements are absent from the matrix

The matrix does not independently capture:

- availability, accessibility, and affordability remaining distinct;
- trigger polarity being descriptive rather than causal;
- prohibition on reconstructing unrecorded microhistory during activation;
- outcome replay versus mechanism replay;
- receipt of only permitted derived player telemetry;
- isolation of profile state, references, and capabilities between game and MCI profiles.

These need explicit implementation homes and falsifiable tests rather than being inferred from broader rows.

#### M-03 — Provisional concepts are accidentally hardened

- R-020 and R-021 mark exact taxonomy counts as `FROZEN`, although blueprint §34.1 calls them a provisional baseline taxonomy. What is frozen is their role as profile vocabulary and the currently required distinctions—not their permanence as an exhaustive catalog.
- R-024 turns “appraisals are normally ephemeral” and “not persisted by default” into an unconditional snapshot-exclusion rule. A profile should remain able to reify a behaviorally persistent interpretation as a declared `StateCell` without adding a primitive.

Correct the statuses and verification language.

#### M-04 — Host acceptance and execution are not adequately separated

ADR-0002 uses `accepted/executed` as one status expression. Acceptance into a host queue is not confirmation that the action occurred. There is also no required test for duplicate, stale, cross-profile, or out-of-order acknowledgements.

Define separate accepted and executed/outcome stages. An outcome must bind to the correct outstanding intent and be idempotent; only actual confirmed effects may enter causal feedback.

#### M-05 — MCI and inspection isolation depends too heavily on capability naming

Negative capability enumeration does not prove absence of reverse control if an MCI profile can emit a generic `BehaviorIntent` or `GameplayHookCandidate` that a shared real-work adapter consumes. Likewise, an inspection “staging” operation can mutate production if it shares state or configuration credentials.

Require:

- an MCI output-type allowlist and no routing to real-work adapters;
- rejection of cross-profile references and subscriptions;
- non-composable trust-domain capabilities;
- staging/dry-run execution on an isolated snapshot with no production commit path.

#### M-06 — Internal queue and schema-bomb limits are not closed

The budget bounds protocol collections and direct fan-out, but not total profile definitions, rule graph edges, alias/migration chains, generated delayed obligations, or sustained backlog. A rule under the direct cap can schedule hundreds of delayed effects every cycle indefinitely.

Specify bounded admission/backpressure or deterministic aggregation/rejection for internal queues. Add limits for nesting, graph size, expansion, string lengths, reference chains, and migration complexity.

#### M-07 — Service equivalence omits non-semantic transport/session influences

ADR-0005’s equivalence tuple omits granted capability set, effective config revision, and canonical input ordinal. It also does not prohibit session IDs, subscriber backpressure, JSON numeric representation, or transport metadata from influencing causal evaluation.

Require these fields either to be part of the explicit logical input or proven causally inert.

#### M-08 — Android determinism is downgraded to compilation compatibility

Matrix R-060 verifies hash parity only on Linux and Windows, with an Android-compatible check. The blueprint requires cross-platform determinism across declared Windows/Linux/Android support builds.

An executable Android fixture-replay path is required before the cross-platform acceptance criterion can pass, even if it lands after Phase 1.

### 4. MINORS

#### N-01 — Controlling source filename is wrong

The matrix names `SPARK_ENGINEERING_TAKEOVER_BLUEPRINT_v0.2.md`, while the reviewed controlling artifact is [CONTROLLING_BLUEPRINT_v0.2.md](/home/chromikey/Downloads/spark_phase0/CONTROLLING_BLUEPRINT_v0.2.md). This weakens automated traceability.

#### N-02 — Combined statuses obscure requirement classification

Rows such as R-018 and R-075 use `FROZEN/CALIBRATE` or `OUT/FROZEN`. Split the frozen mechanism/non-goal from its calibrated values so milestone reports cannot downgrade the invariant accidentally.

#### N-03 — The ADR-0001 dependency diagram is ambiguous

Arrow direction is difficult to reconcile consistently, although its written dependency rules are sound. Replace it with an unambiguous “A may depend on B” list or mechanically testable layer policy.

#### N-04 — Provenance truncation needs explicit coverage semantics

The 64-reference default is legitimately provisional, but pruning/summarization must be deterministic and report omitted-source coverage. A summary must not be presented as authoritative source evidence.

No actual reintroduction of Living Chronicle, runtime LLM authority, general scripting, universal plugins, distributed/graph infrastructure, or economy ownership was found.

### 5. MISSING TESTS / FALSIFICATION CASES

1. Submit identical commands as one batch, split batches, and concurrent service requests; require identical canonical results.
2. Same-time host observation and actor evaluation with both possible explicit orders; reject any implicit arrival-order semantics.
3. Advance ten days once versus one day ten times, including quota overflow and trigger occurrence addressing.
4. Randomized insertion and worker interleaving with colliding equal-time effects.
5. Duplicate, stale, forged, cross-session, and out-of-order behavior acknowledgements.
6. Accepted intent that is never executed must not produce physical, economic, or success feedback.
7. Attempt to change a persisted definition from `host_owned` or `derived` to `spark_owned` through reload, alias, migration, and snapshot restore.
8. Hot tune and structural reload arriving during a wave; activation must occur at a declared logical boundary or roll back atomically.
9. Save with a delayed obligation, replace the active profile, then prove continuation uses or explicitly migrates the originating epoch.
10. Load a save when only the exact content hash—not merely a reused version string—distinguishes manifests.
11. Separate outcome-replay and mechanism-replay fixtures, including explicit unsupported-mode failure.
12. Snapshot during heavy propagation; restored state must correspond to one complete commit boundary.
13. World-scoped trigger with ten times more dormant actors; ordinary trigger cost must not grow with total population.
14. Dormant-to-active promotion must not synthesize unrecorded individual memories, relationships, or actions.
15. Cross-profile state reference, output subscription, config patch, and acknowledgement attempts between game and MCI.
16. MCI emits every shared output family; real-work-consuming families must be structurally unavailable.
17. Inspection dry-run, profiling, and explanation abuse must not mutate production state or block canonical work indefinitely.
18. Service/embedded equivalence under different session IDs, batch limits, subscribers, capability negotiation, and serialization representations.
19. Android execution of the same canonical fixture hashes used on Linux and Windows.
20. Sustained delayed fan-out, profile graph bombs, deeply nested payloads, alias cycles, oversized IDs/strings, and migration-chain bombs.
21. Availability, accessibility, and affordability must not alias or overwrite one another.
22. Changing a trigger’s descriptive polarity alone must not change effects.
23. Provenance pruning must emit deterministic truncation/coverage warnings and never invent causes.
24. Profile isolation must include state stores, ID resolution, capabilities, config revisions, and output routing—not merely separate manifest names.

### 6. PRIMITIVE-SUFFICIENCY RESULT

The frozen primitive set is sufficient.

A delayed promise, for example, composes as a `MemoryRecord` plus `GoalInstance`, a scheduled `PropagationRule`, scoped `StateCell` conditions, a `BehaviorIntent`, and a host-confirmed `EffectBatch`. Directed relationships are pair-scoped `StateCell`s; affiliations and reputation are scoped state; appraisals and choice contexts remain ephemeral; voice identity can be composed from manifest definitions and persistent state.

No blueprint-required counterexample demands a tenth domain primitive. The blockers concern transaction, authority, scope, and version semantics around the primitives.

### 7. AUTHORITY / DETERMINISM RESULT

**Authority:** Not closed. The intent/outcome principle is correct, but authority metadata can be changed through profile lifecycle paths, acknowledgement stages are ambiguous, and MCI/inspection isolation lacks end-to-end routing constraints.

**Determinism:** Not closed. Random-address and stable-wave principles are sound, but canonical input framing, batch boundaries, reload effective time, snapshot boundaries, occurrence indexing, and exact manifest resolution remain underspecified.

### 8. PHASE-1 AUTHORIZATION

`NO`

Phase 1 should begin only after B-01 through B-03 are corrected and their falsification tests are added to the matrix. These corrections do not require redesigning the causal grammar or adding a primitive; they define the canonical transaction, immutable authority, and save/epoch contracts that the Phase 1 clock, scheduler, state model, random-address service, and manifest loader must embody. Implementing those components first would otherwise freeze accidental semantics that later phases could not repair without invalidating fixtures or saves.
