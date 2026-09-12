# S.P.A.R.K.–G.A.M.E. Integration Contract V1 (candidate)

**Date:** 2026-09-12.
**Status:** **CANDIDATE. NOT FROZEN, NOT ACCEPTED.** Gate C3 of
`engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` freezes a contract only *after*
Phase 2 is accepted and Phase 3 is separately opened. Phase-2 acceptance has not been
given (see §11), so this document is prepared as the integration-requirement and
test-fixture material that protocol §6 permits before Phase 3, and as the exact text a
freeze would act on.
**Author:** the local main engineer (Claude Code, Opus 5, `claude-opus-5`), sole
integration writer for the 2026-09-12 safe-handoff mission.
**Assurance class:** canonical-target for the proposed handoff, pending required
independent review and Operator acceptance.

## 0. Pinned sources

| Pin | Exact value |
|---|---|
| Shared ACTIVE protocol | `engineering/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md`, sha256 `611f21ff5e39575f414e245b8409a2b6f7811a5f2e164bd81a3c69e7cbf0a06a` |
| G.A.M.E. mirror of that protocol | `project_records/foundation/SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md`, sha256 `611f21ff5e39575f414e245b8409a2b6f7811a5f2e164bd81a3c69e7cbf0a06a` — **byte-identical** |
| S.P.A.R.K. pinned candidate | `67b877192cc78b75c6fbe60c69b5594dc10befe8` (`candidate/phase2-gate-c2-w01-correction-20260912`) |
| S.P.A.R.K. crate tree of that candidate | `7907f4d729104fd5dbfd4adad46e66cf09aa13dd` |
| Independent KEEP of that candidate | `21a4fec666ca493f6ac4d5194ec6e9c5380ce40c` |
| G.A.M.E. live `origin/main` when this contract was written | `a73fbac743adc1a00679ebc63b91027386718d71` |
| G.A.M.E. local `main` at the same moment | `88cc098736a245c42a30c6178ab37b47bdd4d055` (behind live by 17 commits; the live hash is the pin) |
| G.A.M.E. governing set | version 2.9.6; all seven members verified against `OPERATIVE_SET.sha256`, exit 0 |
| Recovered G.A.M.E. product direction | `project_records/foundation/GAME_PRODUCT_DESIGN_FOUNDATION_V1.md` §11.2–§11.3, status **CANONICAL — OPERATOR-APPROVED PRODUCT/DESIGN DIRECTION** |
| Recovered G.A.M.E. working record | `project_records/research/game_architecture_refoundation/GAME_IMPLEMENTATION_READINESS_CLOSEOUT_20260909.md`, status **working research record; non-operative** |
| Recovered S.P.A.R.K. research | `engineering/research/SPARK_FABLE_ARCHITECTURE_CHALLENGE_2026-09-09/CLOSEOUT_2026-09-09.md` on `research/fable-architecture-challenge-2026-09-09`, **noncanonical and unmerged** |

This contract is written against the S.P.A.R.K. public surface of that exact crate tree.
Every S.P.A.R.K. type named below is a real public item of `spark-core`/`spark-engine` at
that tree, or is explicitly marked as a contract-layer type that does not exist in the
engine.

## 1. What this contract is, and what it is not

It defines the **smallest logical seam** through which G.A.M.E. can invoke the
S.P.A.R.K. device and consume its advisory output, with the surfaces protocol Gate C3
enumerates. It is defined independently of both projects' internal classes: the wire
shape, the transport, the serialization and the embedded-versus-service form are
**deliberately not settled here** (§10).

It does **not** claim that a device exists, that an adapter exists, that G.A.M.E. has
accepted anything, that Phase 2 is accepted, that Phase 3 is open, or that any
cross-platform runtime parity has been demonstrated.

## 1.2 Recovered G.A.M.E. decisions, and where this contract diverges

Gate C3 work must recover existing accepted G.A.M.E. integration decisions before designing
replacements. Three prior sources were found and read. Their status differs sharply and is
stated for each, because only the first carries G.A.M.E. authority.

### 1.2.1 Accepted: `GAME_PRODUCT_DESIGN_FOUNDATION_V1.md` §11.3

This record's status line is **CANONICAL — OPERATOR-APPROVED PRODUCT/DESIGN DIRECTION**.
Its §11.3 carries a **SETTLED DIRECTION** on the authority split that matches protocol §3
verbatim in substance — including the sentence "A S.P.A.R.K. `BehaviorIntent` or
gameplay-hook candidate is never an executable G.A.M.E. command" — and a **REQUIRED
CAPABILITY** paragraph for the seam. `BehaviorIntent` is therefore **G.A.M.E.'s own
vocabulary**, recovered rather than invented here.

Every required capability it names is satisfied by a numbered section of this contract:

| G.A.M.E. REQUIRED CAPABILITY (§11.3) | Where this contract provides it |
|---|---|
| stable entity identities | §4.1, with injectivity and stability as hard, checkable requirements |
| monotonic authoritative logical time | §4.2, enforced by the engine's own refusal, not by trusting the host |
| typed/scoped host observations | §5 |
| actor/affiliation context | §4.1 (`ExternalAffiliationRef`) |
| advisory outputs | §6 |
| explicit host-confirmed outcomes | §8, with closed rejection and defer vocabularies |
| "Exact transport, serialization, process boundary, and embedded-versus-service deployment remain open" | §10.2, which leaves all four unselected |

§11.2 of the same record states that G.A.M.E. "integrates a specific S.P.A.R.K.
artifact/version only under a later bounded implementation mission". That is G.A.M.E.'s own
statement of the Gate C4 precondition, and this contract does not attempt to satisfy it.

### 1.2.2 Not accepted: `GAME_IMPLEMENTATION_READINESS_CLOSEOUT_20260909.md`

This record is explicitly classified **working research; non-operative**, and states that
"neither side's proposal is adopted by this record". It nonetheless matters, because it is
the current G.A.M.E. record that **names** the seam questions:

- it records "the challenge closeout's bounded pull/`Busy`/ack result flow and related
  late-input, transaction-unit, retention, and durable-snapshot choices remain proposals";
- it records a **G.A.M.E.-side** proposal of "`BehaviorIntent` parent/child identity, staged
  child admission, durable outcome replay, and frontier-based release".

So the accurate provenance statement is **not** "G.A.M.E. has no record of this design". It
is: G.A.M.E. has a current record that names the design and classifies it as an unadopted
proposal, alongside a different G.A.M.E.-side proposal that this contract does not adopt
either. §11.2 states each item on that footing.

**Divergences from the G.A.M.E.-side proposal, stated rather than buried:**

| G.A.M.E.-side proposal (unadopted) | This contract | Why |
|---|---|---|
| `BehaviorIntent` parent/child identity | flat intents carrying the engine's own bounded `Provenance` | the pinned engine exposes provenance, not a parent/child intent tree; inventing one would put structure in the seam that no evidence backs |
| staged child admission | not present | it presupposes the parent/child model |
| durable outcome replay | **not present**, and §9.4 says why: there is no durable state to replay from | a capability the engine cannot support must not be advertised |
| frontier-based release | the completed request boundary **is** the release point (§4.3) | the same idea, expressed in the frozen architecture's own terms |

### 1.2.3 Noncanonical: the S.P.A.R.K. Fable architecture challenge closeout

Research on the S.P.A.R.K. side, unmerged and non-adopted, and the origin of the
pull/`Busy`/ack flow the G.A.M.E. record cites. Two of its open questions are now **closed
by later S.P.A.R.K. authority**, and this contract says so rather than reopening them:

- its "transaction unit: per-`WorkKey` versus equal-time slice" question is superseded by
  the 2026-09-10 V3-F01 acceptance and freeze, which makes the **request boundary** the
  transaction unit (§4.3);
- its "strict reject versus adapter re-date" late-input question is settled **on the device
  side** by the engine's typed `RefusedHorizonBehindFrontier` (§4.2). What the *adapter*
  does with that refusal — retry later, drop, escalate — remains an open host policy and is
  not decided here.

Its acknowledgment-gap analysis is adopted in substance at §7.2, with the difference
recorded there.

## 2. Authority boundary (restated, unchanged)

Protocol §3 controls and is not reopened. G.A.M.E. is authoritative for physical and
economic world truth — entity identity, logical game time, movement, inventory, money,
trade, combat, resources, **action legality, execution, and confirmed outcomes**.
S.P.A.R.K. owns its deterministic causal, social, psychological, relationship,
selected-memory, appraisal, goal and pressure state.

A `BehaviorIntent` (§6) is **never** an executable G.A.M.E. command. G.A.M.E. may
validate, execute, reject, defer or translate it. Only G.A.M.E.-confirmed outcomes
re-enter S.P.A.R.K., and they re-enter only as typed observations (§5).

No S.W.A.R.M. runtime component, and no model inference of any kind, is a dependency of
this seam.

## 3. Session establishment: version, profile and capability negotiation

### 3.1 Protocol version

`protocol_version` is the pair `(major, minor)`. This contract is `(1, 0)`.

- A host presenting a **different `major`** is refused, fail-closed, with
  `Rejected::UnsupportedProtocolVersion { supported_major: 1 }`. No session opens.
- A host presenting the same `major` and a **higher `minor`** is accepted at the
  device's own `minor`; the device reports its `minor` in the session descriptor and the
  host must not rely on anything above it.
- A host presenting a **lower `minor`** is accepted at the host's `minor`; the device
  must not emit a surface introduced after that `minor`.

`major` changes whenever any canonical encoding, identity rule, or refusal meaning
changes. Nothing in this section is advisory: an unsupported version never degrades
silently.

### 3.2 Profile binding

A session is bound to exactly one activated S.P.A.R.K. profile. The session descriptor
carries, verbatim, the two content-addressed identities the engine already publishes:

- `manifest_content_hash` — the full content of the profile manifest;
- `activation_hash` — the activation identity of the activated profile.

The host records both. Any later message that names a different pair is refused with
`Rejected::ProfileMismatch`. This is the seam's whole answer to "did both sides agree on
what S.P.A.R.K. is": it is content-addressed, so divergence cannot be silent
(`spark-engine` crate documentation, §"What this crate does not claim").

### 3.3 Capability negotiation

A capability is a named, versioned surface the device may or may not provide. The device
advertises a **set** of capability tags in the session descriptor; the host selects a
subset. A host that requires a tag the device does not advertise must fail closed, not
proceed degraded.

V1 defines exactly these capability tags, each of which maps to a real public surface of
the pinned S.P.A.R.K. tree:

| Tag | Meaning | Backing surface at the pinned tree |
|---|---|---|
| `observe.v1` | typed, scoped host observations may be carried on a command | `CommandPayload::Host { observations }` |
| `advance.v1` | the host may advance logical time without a command | `Request::Advance(LogicalTime)` |
| `intent.v1` | the device projects advisory intents from canonical reports | contract-layer projection, §6 |
| `replay.v1` | deterministic canonical replay from completed history | `Engine::reconstruct_completed`, `CompletedHistory` |
| `snapshot.v1` | in-memory stable-boundary snapshot and restore | `Engine::snapshot` / `Engine::restore` |
| `epoch.v1` | host-driven behavior-epoch activation | `CommandPayload::ActivateEpoch` |

**`persist.v1` is deliberately not defined and must not be advertised.** The engine's
snapshot is an in-memory value with no persistence backend and no crash-recovery claim
(`EngineSnapshot` documentation at the pinned tree). See §9.3.

## 4. Identity, time and the transaction unit

### 4.1 External entity and affiliation identity

G.A.M.E. owns identity. The contract requires a host-side **injective, stable** mapping

```
ExternalEntityRef  <->  spark_core::scope::ScopeId
ExternalAffiliationRef  <->  spark_core::scope::ScopeId (ScopeKind::Region or an
                             affiliation-kind scope the profile declares)
```

Two hard requirements, both because `ScopeId` is an input to every canonical digest the
engine commits:

1. **Injective.** Two distinct external references never map to one `ScopeId`. A
   violation silently merges two entities' causal state and is not detectable downstream.
2. **Stable for the life of the engine instance.** A reference's `ScopeId` never changes,
   including across host restarts, because committed cells, obligations and scheduler
   keys are addressed by it. Re-mapping is a new engine instance, not an edit.

The mapping lives in the host adapter. The contract does not prescribe its
representation; it prescribes that the adapter can prove both properties and that the
proof is part of the adapter's own acceptance evidence.

### 4.2 Monotonic G.A.M.E. logical time

G.A.M.E. owns logical time. It maps to `spark_core::clock::LogicalTime(u64)`. The
mapping must be **monotonic non-decreasing** over a session.

The device enforces the safety half of this itself and does not trust the host: a request
whose horizon is behind the completed-boundary frontier `F` is refused with the engine's
own typed `Outcome::RefusedHorizonBehindFrontier { frontier, horizon }`, which the
contract surfaces as `Rejected::StaleHorizon { frontier, horizon }`. A host that presents
time backwards gets a refusal, never a rewritten past.

### 4.3 The transaction unit

**The transaction unit is one request boundary**, and this is settled by existing
S.P.A.R.K. authority rather than chosen here. The frozen V3-F01 serialized request
boundary (accepted and frozen at
`engineering/phase2/SPARK_PHASE_2_V3_F01_OPERATOR_ACCEPTANCE_FREEZE_2026-09-10.md`, with
the Revision-2 correction) establishes that one request is active until its horizon
completes, that budget exhaustion pauses rather than ends it, that commands finalize and
execute atomically at completion, and that `F` — the last completed horizon — is the only
boundary at which a stable state is published.

Consequences the host adapter must honour:

- A **partial** result (`Outcome::Paused`) is **not** a transaction. It publishes no
  stable boundary, and the host must not apply anything from it to the world.
- Exactly one request is outstanding at a time (§4.4).
- The host's unit of retry, idempotency and acknowledgment is the request, identified by
  the engine's `RequestDiscriminator` identity digest.

### 4.4 One outstanding request, Busy, and pull-results

The serialized boundary gives the seam its concurrency model directly:

- The device holds at most one **active request**. While one is active the engine carries
  an engine-owned `ActiveRequest` discriminator (`Engine::active_request()`).
- Presenting a *different* request while one is active is refused with
  `Outcome::RefusedActiveRequestMismatch { active }`, surfaced as `Rejected::Busy`
  carrying the active request's `kind`, `horizon` and identity digest.
- Progress is made by **presenting the same request again** — the host pulls results by
  re-presenting, not by polling a side channel. The engine's own
  `ProcessResult::terminates(head)` is the exact dequeue-eligibility predicate: the
  request leaves the head of the host's queue only when a result for *that exact* request
  reports `Completed`, `CompletedCommandNotFinalized(_)` or `RefusedHorizonBehindFrontier`.
- A mismatch terminates nothing. A `Paused` result terminates nothing.

**Provenance, stated precisely.** This design is settled *in S.P.A.R.K.*, by the frozen
V3-F01 architecture and the reviewed Gate C2 implementation.

On the G.A.M.E. side it is **named but not adopted**. `GAME_IMPLEMENTATION_READINESS_CLOSEOUT_20260909.md`
— a record G.A.M.E. itself classifies as working research, non-operative — states that "the
challenge closeout's bounded pull/`Busy`/ack result flow and related late-input,
transaction-unit, retention, and durable-snapshot choices remain proposals" and that
"neither side's proposal is adopted by this record". No G.A.M.E. record **adopts** the
design. The G.A.M.E.-side obligation to adopt it is therefore an **engineering proposal of
this contract**, listed as such in §11.2. See §1.2.2 for the full recovery.

## 5. Typed, scoped host observations

Host-owned truth enters only through canonical ingress. One observation is

```
Observation { definition: DefinitionId, value: CanonicalValue }
```

carried on a `CommandPayload::Host { subject, observations, params }` at a single
`subject` scope. The contract adds no new observation type; it constrains use:

1. **Typed.** `value` must satisfy the activated definition's declared `ValueConstraint`.
   A violation is refused; it is not clamped, truncated or coerced. The refusal is
   reported at the cohort level as `CohortOutcome::IngressRejected { definition, error }`
   and nothing from that command is applied.
2. **Scoped.** The definition must be valid at the subject scope's `ScopeKind`.
3. **Authority-respecting.** A host observation writes only `Authority::HostOwned`
   definitions. `SparkOwned` and `Derived` state is not writable by the host — that is the
   authority boundary of §2 expressed in the type system.
4. **Declared bounds are the specification.** A value outside a definition's declared
   `ValueConstraint` is refused at the declared boundary, not at the machine integer
   boundary. This is not an unspecified edge: it is proved discriminatingly by diagnostic
   D-1 in `engineering/phase2/gate_c2_campaign_disposition_evidence_2026-09-12/`, where
   the declared maximum commits, the declared maximum plus one refuses, and `i64::MAX`
   produces the byte-identical refusal.

## 6. Advisory intent batches

### 6.1 There is no `BehaviorIntent` type in the engine, and V1 does not add one

At the pinned tree, Phase 2 produces canonical `CohortReport`s containing
`CommittedEffect`s over declared state, plus obligations and diagnostics. It has no
intent type. V1 therefore defines `BehaviorIntent` as a **contract-layer projection** of
canonical report content, computed outside the engine. This keeps the engine's frozen
production Rust untouched (which matters: Gate C2 is not accepted, §11) and keeps the
logical contract independent of both projects' internal classes, as Gate C3 requires.

### 6.2 The projection

A profile declares which of its definitions are **intent channels**, using the existing
`DefinitionSpec::domain` tag with the reserved value `intent`. No engine change is
required; `domain` is already part of the manifest content hash, so the intent-channel
set is content-addressed along with everything else.

For one completed request, the device emits one **intent batch**. "For one completed
request" is load-bearing: a paced request spreads its committed cohorts over several
`process` calls, and each `ProcessResult` carries only its own call's cohorts. The device
therefore **accumulates cohort reports across every call of the active request** and
projects the batch once, at the completed boundary. A device that projected only from the
completing result would silently drop every intent committed before the last pause — a
defect this contract's prototype actually contained and now carries a regression test for
(`tests/end_to_end.rs` E-12). An interleaved `Busy` refusal (§4.4) must not disturb the
active request's accumulation; a sticky fail-stop discards it, because no stable boundary
is published.

```
IntentBatch {
  correlation: CorrelationId,          // §7.1
  horizon: GameLogicalTime,            // the completed request horizon
  intents: [BehaviorIntent],           // canonical order, see below
  batch_digest: Digest,                // §7.2
}

BehaviorIntent {
  subject: ExternalEntityRef,          // mapped back from the effect's ScopeId
  channel: DefinitionId,               // the intent-channel definition
  value: CanonicalValue,               // the committed value
  leverage: Option<BehavioralLeverage>,// declared metadata, advisory only
  canonical_time: GameLogicalTime,     // the cohort's canonical time
  provenance: Provenance,              // the engine's own bounded provenance
}
```

One `BehaviorIntent` is produced for each `CommittedEffect` whose `definition` is an
intent channel, taken from the `waves` of each `CohortReport` of the completed request,
in the engine's own canonical order: cohort sequence order, then wave index, then the
canonical order the engine already fixes within a wave. **The projection introduces no
ordering of its own**, so two devices given the same history produce byte-identical
batches.

Effects on non-intent-channel definitions are internal S.P.A.R.K. state and are **not**
exported. Refused cohorts export nothing.

### 6.3 Advisory status is structural, not a convention

An `IntentBatch` carries no execution authority, and the contract gives the host no
message that means "apply this". The host validates each intent against its own rules and
then reports back what it actually did (§8). A host that executed an intent it had
rejected would be violating its own authority, not S.P.A.R.K.'s.

## 7. Correlation, idempotency and acknowledgment identity

### 7.1 Correlation identity

```
CorrelationId = RequestDiscriminator.identity()   // a Digest
```

The engine already computes this as the canonical hash of the entire request: for a
command, the full envelope minus the sequencer-issued ordinal, plus the canonical payload
hash; for an advance, the target time. It is therefore:

- **exact** — two requests with any semantic difference have different identities;
- **derivable by both sides** without a round trip, since the host constructed the
  request;
- **stable across pause and resume**, because resuming means re-presenting the same
  request.

The contract adds no separate request id. A host-side id, if any, is adapter bookkeeping
and never enters the canonical seam.

### 7.2 Acknowledgment identity and at-most-once application

The unit G.A.M.E. acknowledges is `(activation_hash, CorrelationId, batch_digest)`, where
`activation_hash` is the session's content-addressed profile activation identity (§3.2) and

```
batch_digest = H( "spark.intent_batch.v1"
                || CorrelationId
                || horizon
                || for each intent in canonical order:
                     subject_scope || channel || value || canonical_time )
```

computed with the engine's own `CanonicalEncoder`, so it is a function of canonical
content alone.

**At-most-once rule.** The host applies the effects of a batch at most once per
`(activation_hash, CorrelationId, batch_digest)`. A redelivered batch with an identity the
host has already applied is acknowledged again and **not** re-applied — and it is
acknowledged with the **recorded** disposition of the first application, not a freshly
recomputed one, since the world the first application moved would otherwise produce a
contradictory second answer.

**Why this key and not a sequence number.** The prior S.P.A.R.K. research
(§1.2.3) established that "`acked_through` or another persisted sequence number alone
cannot prove exactly-once application", and proposed
`(spark_instance_id, report_seq, report_digest)`. This contract adopts that conclusion and
differs in one way: every component of its key is **content-addressed**, so it needs no
instance counter and no issued sequence. `activation_hash` distinguishes engines activated
from different profile content; `CorrelationId` is the canonical hash of the whole request;
`batch_digest` is a function of the committed canonical content. Two engines that are
genuinely interchangeable — identical activation, identical history — produce the identical
key, and treating a redelivery from either as already applied is correct, not a collision.
Two engines that diverge produce different keys and cannot be confused.

**What the key does not solve.** It gives at-most-once. It does not by itself give
at-least-once: if the host loses a batch before recording it, §9.3 shows the batch cannot be
re-derived from the device. That is the durability gap of §9.4, not a defect of this key. This is what makes duplicate
delivery safe without SPARK-side state: re-presenting a completed request is refused by
the engine's own identity checks (`CommandIdentityConflict`, proved identity-scoped rather
than payload-scoped by diagnostic D-2), and re-delivering a batch is deduplicated by the
host against this key.

### 7.3 Atomic report application

The host applies a batch **atomically with respect to its own world**: either every intent
in the batch that the host accepted is applied and the acknowledgment is durable, or none
is applied and nothing is acknowledged. Partial application followed by an acknowledgment
is the one failure this rule exists to forbid, because S.P.A.R.K. cannot detect it.

The contract does not prescribe *how* G.A.M.E. achieves atomicity; that is inside
G.A.M.E.'s authority. It prescribes that the adapter's acceptance evidence demonstrates
it.

## 8. Confirmed outcomes, rejections and defers

A batch is answered by exactly one **outcome report**:

```
OutcomeReport {
  correlation: CorrelationId,
  batch_digest: Digest,
  per_intent: [IntentDisposition],     // same length and order as the batch
}

IntentDisposition =
  | Executed { confirmations: [Observation] }   // typed, scoped, HostOwned only
  | Rejected { reason: RejectionReason }
  | Deferred { reason: DeferReason, not_before: Option<GameLogicalTime> }
```

`RejectionReason` and `DeferReason` are **closed, named** vocabularies — a free-text
reason is not acceptable, because S.P.A.R.K. must be able to react deterministically:

| `RejectionReason` | Meaning |
|---|---|
| `IllegalAction` | the action is not legal in the world state |
| `UnknownSubject` | the external reference does not resolve |
| `InsufficientResource` | the world lacks a required resource |
| `AuthorityRefused` | the host declines on its own authority |
| `UnsupportedChannel` | the host does not implement that intent channel |

| `DeferReason` | Meaning |
|---|---|
| `WorldBusy` | the subject cannot act yet |
| `AwaitingPrecondition` | a world precondition is pending |
| `RateLimited` | host-side backpressure (§9.2) |

**Feedback is re-entry, not application.** Nothing in an `OutcomeReport` writes S.P.A.R.K.
state directly. Confirmed outcomes re-enter only as typed observations on a subsequent
command (§5), at a horizon at or after the batch's horizon. This is the closed loop the
protocol's first proof requires, and it keeps G.A.M.E. the sole author of world truth.

## 9. Boundedness, failure and recovery

### 9.1 Bounded message size

Every list in this contract is bounded, and the bound is declared in the session
descriptor: `max_observations_per_command`, `max_intents_per_batch`,
`max_message_bytes`.

**Inbound** bounds are enforced by refusal: a message exceeding a declared bound is refused
with `Rejected::MessageTooLarge { limit, observed }` and **nothing is partially processed**.

**Outbound, `max_intents_per_batch` is a capacity hint, not an enforceable refusal, and the
contract says so rather than pretending otherwise.** By the time a batch is projected its
work is already committed at a completed boundary; a device cannot un-commit it, and
truncating a batch would silently lose advisory output. So an oversized batch is delivered
**whole**, with a `capacity_exceeded` flag telling the host to size the bound up. The
alternative — chunked pull of one boundary's results, as the prior research proposed — is a
reasonable V2 surface and is deliberately **not** in the smallest V1.
S.P.A.R.K.'s own semantic caps (`max_effects_per_wave`, `max_wave_depth`, `max_fan_out`,
`max_obligations`, `max_cohort_candidates`) remain the engine's internal bounds and are
reported through the engine's typed `WaveRejection::SemanticCap { cap, observed, bound }`;
the seam does not duplicate or override them.

### 9.2 Backpressure

The seam has exactly one backpressure mechanism, and it is the one the serialized
boundary already gives: `Rejected::Busy` (§4.4) and `Outcome::Paused`. A host that
receives `Paused` re-presents the same request; a host that receives `Busy` completes the
active request first. There is no queue inside the device, so there is no unbounded
buffer to overflow. Host-side backpressure travels the other way as
`DeferReason::RateLimited`.

The pacing budget is the device's admission control, and the contract records its exact
declared behavior rather than an idealisation: the **first** executable slice at a
canonical time is admitted whole even when it exceeds the declared budget, which
guarantees forward progress instead of starving on an oversized atomic slice, and that
admission is reported as `PacingDiagnostics::pacing_overrun`. Later due times are still
deferred. This is proved discriminatingly by diagnostic D-4.

### 9.3 Interruption between request, result, application and acknowledgment

Four points can fail. The contract's required behavior at each:

| Failure point | Required behavior |
|---|---|
| After the host sends a request, before any result | The host re-presents the identical request. The engine is either not started on it (starts now) or has it active (resumes it). Nothing is applied. |
| After a `Paused` result, before completion | The host re-presents the identical request. A *different* request is refused `Busy`. No stable boundary was published, so there is nothing to apply or roll back. |
| After a completed result, before the host applied the batch | The host may re-derive the batch by re-presenting? **No.** Re-presenting a completed command is refused (`CommandIdentityConflict`). The host must have durably recorded the batch before acknowledging, or must treat the batch as lost (§9.4). |
| After the host applied the batch, before acknowledgment | The host re-acknowledges. The at-most-once key (§7.2) makes the second acknowledgment a no-op. |

**Device-side retention is deliberately absent from V1, and this is a divergence worth
naming.** The prior research design (§1.2.3) has the device *retain* an unacknowledged
report and redeliver it, which would close row 3 above. V1 has no retention because the
pinned engine has no durable state to retain it in: a retained report held only in memory
is lost by exactly the failure it exists to survive. Retention is therefore recorded as a
**candidate resolution of the §9.4 durability gap**, to be designed together with the
persistent boundary rather than bolted on before it.

### 9.4 The durability gap — stated, not papered over

**This is a real, unresolved limitation of V1 and the reason §3.3 forbids `persist.v1`.**

At the pinned tree the engine's snapshot is an in-memory value with no persistence
backend and no crash-recovery claim, and the Phase-2 restore tests are in-memory
round-trip tests, not durable process-recovery proof. Therefore:

- A device that **shares the host process** (embedded form) loses its state exactly when
  the host loses its own, which a host transaction can be made to cover.
- A device in a **separate process** (service form) can lose its state while the host
  survives, and V1 has **no mechanism to rebuild it beyond replaying completed history**
  (`replay.v1`), which reconstructs only completed boundaries, never paused progress.

The honest consequence, and this contract states it as a limit rather than resolving it:
**a safe handoff that survives independent device restart requires a minimal persistent
boundary that does not exist yet.** Implementing and testing that boundary is authorized
work under the handoff mission, but only *after* the required predecessor acceptance
(§11). It is not in V1.

### 9.5 Malformed, stale and unsupported input

All fail closed, with a typed refusal and no partial effect:

| Input | Result |
|---|---|
| Unsupported protocol major | `Rejected::UnsupportedProtocolVersion` (no session) |
| Wrong profile identity pair | `Rejected::ProfileMismatch` |
| Horizon behind `F` | `Rejected::StaleHorizon { frontier, horizon }` |
| A different request while one is active | `Rejected::Busy { active }` |
| Observation violating a declared constraint | cohort `IngressRejected`; nothing applied |
| Effect violating a declared constraint | cohort `Rejected { WaveRejection::InvalidEffect }`; the wave is atomic, earlier waves stay committed |
| Reused command identity | `CompletedCommandNotFinalized(CommandIdentityConflict)` |
| Non-increasing source sequence | `CompletedCommandNotFinalized(SourceSequenceNotIncreasing)` |
| Oversized message | `Rejected::MessageTooLarge` |
| Engine-internal invariant violation | sticky fail-stop; no snapshot published; the only exit is restoring a prior snapshot |

**Two-level reporting is part of the contract, not an accident.** A request can complete
(`Outcome::Completed`) while a cohort inside it was rejected. Both levels must be read: the
request level says the boundary was processed, the cohort level says what was refused and
names the exact definition. An adapter that reads only the request level will silently
believe a refused write succeeded. This is exactly the mistake an independent tester made
and self-corrected during the adversarial campaign (SONNET T-5), and it is proved
discriminatingly by diagnostic D-5.

## 10. Deterministic canonical replay, and what stays swappable

### 10.1 Replay

Given genesis inputs and the completed history — the finalized commands with their
payloads in ordinal order, every epoch-reset record, `F`, and the recorded digests —
reconstruction must reproduce the recorded history digest and stable-boundary digest
exactly, or refuse with `REPLAY_DIVERGED`. The engine provides this as
`Engine::reconstruct_completed` over `CompletedHistory`.

Replay is scoped to **completed** boundaries. Paused progress is not reconstructible from
history plus `F`; that scoping is a controlling correction of the frozen architecture, not
an oversight, and §9.4 is its integration-level consequence.

Replay is defined over canonical content only, so it is independent of transport and
serialization. Any transport that preserves canonical content preserves replay.

### 10.2 What V1 deliberately does not settle

Protocol §2 states that the embedded-versus-service, transport, serialization and
deployment choices are open, and this contract does not close them by implication:

- **Embedded versus service.** Not selected. §9.4 records a real asymmetry between them;
  that is a *consequence to weigh*, not a decision taken here.
- **Transport.** Not selected. The seam is request/response with one outstanding request;
  any transport preserving that and canonical content is admissible.
- **Serialization.** Not selected. The canonical digests are computed with the engine's
  own `CanonicalEncoder` over logical content, never over a wire encoding, so a
  serialization change cannot change a digest.
- **Concurrency beyond one outstanding request.** Not proposed.

A contained prototype may explore any of these reversibly. Freezing reliance on one
requires the assurance and the reserved decision named in §11.

## 11. Predecessor gates, open decisions, and what is blocked

### 11.1 The blocking gate

Protocol Gate C3 freezes a contract "after Phase 2 is accepted and Phase 3 is separately
opened". **Phase-2 (Gate C2) acceptance has not been given.** The controlling record,
`engineering/phase2/SPARK_PHASE_2_GATE_C2_OPERATOR_NEXT_AUTHORIZED_STEP_2026-09-12.md` §4,
states plainly: "Acceptance remains the Operator's decision, taken after the independent
review and informed by whatever the campaign finds." No Operator acceptance of Gate C2
exists in the repository, and none is invented here.

Consequently this contract is a candidate, the device surface and the G.A.M.E. adapter are
not implemented as production code, and the durable boundary of §9.4 is not built. What is
prepared instead is everything that does not depend on that decision: this contract, the
test fixtures, and the reversible prototype.

### 11.2 Engineering proposals that are not recovered decisions

Recorded honestly so a reviewer is not misled about provenance:

| Item | Status |
|---|---|
| Transaction unit = one request boundary | **Settled** by frozen S.P.A.R.K. V3-F01 authority (§4.3) |
| Late/stale input refused at `F` | **Settled** by the engine's typed refusal (§4.2) |
| The authority split and the seam's required capabilities | **Settled by G.A.M.E.**, recovered from `GAME_PRODUCT_DESIGN_FOUNDATION_V1.md` §11.3 (§1.2.1) |
| One outstanding request / `Busy` / pull-by-re-presenting | **Settled in S.P.A.R.K.**; **named but expressly unadopted** in current G.A.M.E. records, so **proposed** as a G.A.M.E.-side obligation (§1.2.2, §4.4) |
| Acknowledgment identity `(activation_hash, CorrelationId, batch_digest)` | **Proposed** by this contract, adopting the prior research's conclusion with a content-addressed key (§7.2) |
| Outbound batch capacity as a hint rather than a refusal; no chunked pull in V1 | **Proposed** by this contract (§9.1) |
| Device-side retention of unacknowledged reports | **Deferred**, tied to the persistent boundary (§9.3, §9.4) |
| G.A.M.E.-side `BehaviorIntent` parent/child identity, staged child admission, durable outcome replay | **Not adopted**; divergences recorded (§1.2.2) |
| At-most-once host application and atomic batch application | **Proposed** by this contract (§7.2, §7.3) |
| `BehaviorIntent` as a contract-layer projection over `domain = "intent"` | **Proposed** by this contract (§6) |
| Closed rejection/defer vocabularies | **Proposed** by this contract (§8) |
| Embedded vs service, transport, serialization | **Reserved**, not selected (§10.2) |
| Minimal persistent boundary | **Blocked** on §11.1; required for restart-safe handoff (§9.4) |

### 11.3 Decisions that require someone other than the main engineer

1. **Gate C2 acceptance** — the Operator's, and the gate that blocks Gate C3 freeze,
   device implementation and adapter implementation.
2. **G.A.M.E. adoption of the §4.4 concurrency obligation and the §8 vocabularies** —
   G.A.M.E.'s, through its own records; this contract cannot grant it.
3. **Embedded versus service** — reserved by protocol §2; it determines whether §9.4's
   durability gap must be closed before any restart-safe claim.

## 12. Nonclaims

This contract does not claim that Gate C2 is accepted, that Gate C3 is frozen, that a
deployable S.P.A.R.K. device exists, that a G.A.M.E. adapter exists, that G.A.M.E. has
adopted anything, that the durability gap is closed, that cross-platform runtime parity is
demonstrated, or that either project holds authority over the other.
