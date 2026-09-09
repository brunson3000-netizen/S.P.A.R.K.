# Fable's version — a concrete alternative design for the Spark module

**RESEARCH / PROPOSED DESIGN — NOT ADOPTED.** 2026-09-09. Written after the first-principles
sketch (`00_FIRST_PRINCIPLES_SKETCH.md`) and after comparing it with the Phase-2 freeze v3,
the V3-F01 candidates, both independent reviews, and the serialized-request addendum. It
preserves the product intent verbatim and challenges inherited mechanisms only. Nothing here
enacts, freezes, or authorizes anything.

## 0. The thesis in one paragraph

Spark is a deterministic discrete-event simulator: a semantic-keyed priority queue of timed
work (the Phase-1 `Scheduler`, unchanged), a typed/scoped state store with authority
enforcement (the Phase-1 `StateStore`, unchanged), and a loop that pops the least due time
slice, evaluates it at its own time on an overlay transaction, commits or rejects it whole,
and schedules strictly-later work. Host requests are horizons. Pause needs no retained
state. The engine's logical clock is the only request frontier. Three digests exist because
three different things are being committed to, and no more. The V3-F01 "cohort extraction"
defect is closed by calling the existing `drain_due` with the least due time instead of the
horizon, plus one read-only accessor; no compare-and-take protocol, slot fingerprint, or
extraction slice is needed. The executable example in `example/` demonstrates every claim in
this document on the real Phase-1 kernel and detects two deliberately wrong variants.

## 1. Module boundaries and responsibilities

```
crates/
  spark-core      (KEEP, unchanged except +1 accessor)  ids, values, clock, hash, scheduler,
                                                        random addresses, timeline ingress
  spark-engine    (KEEP + EXTEND)                       activation door, StateStore, and NEW:
                                                        rules/, evaluate/, request/  (Phase 2)
  spark-host      (NEW, thin)                           Mailbox, Consumer, Snapshot codec,
                                                        adapter-facing DTOs (Phase 3)
  spark-testkit   (KEEP)                                fixtures, replay harness, acceptance
```

| Module | Owns | Never does |
|---|---|---|
| `spark_core::scheduler` | resident timed work by `WorkKey`; conflict poisoning | evaluate anything; know about horizons |
| `spark_core::clock` | `LogicalClock` = the request frontier `F` | advance backwards |
| `spark_engine::state` | cells + authority validation (`host_owned`/`spark_owned`/`derived`) | be written from outside the crate |
| `spark_engine::rules` (new) | validated declarative rule artifact (thresholds, decay, delayed emission, clamps) bound to the activation hash | run arbitrary code |
| `spark_engine::evaluate` (new) | one cohort → one `Txn` (overlay) → commit-or-reject; wave loop; emission ordering | touch the mailbox, the clock, or diagnostics |
| `spark_engine::request` (new) | `process(&Request, Budget) -> Outcome`; the loop of §4 | retain anything about a paused request |
| `spark_host::mailbox` (new) | bounded FIFO, head-until-complete, backpressure, telemetry | reorder, reinterpret, or alter requests |
| `spark_host::snapshot` (new) | canonical byte encoding of §6's state; restore with activation-hash check | encode diagnostics or mailbox contents |
| G.A.M.E. adapter (outside Spark) | mapping ids/time/observations/intents; executing or refusing intents | write Spark cells directly |

Dependency direction is unchanged: `core ← engine ← host ← adapter`; `testkit` is dev-only.

## 2. Principal Rust types (interface sketch)

```rust
// ---- spark_engine::request -------------------------------------------------
pub enum Request {
    Advance { to: LogicalTime },
    Input   { command: SemanticCommandEnvelope, body: InputBody },   // effective_time = horizon
}
pub struct Budget { pub cohorts: NonZeroU32 }                        // pacing only

pub enum Outcome {
    Completed { report: CompletionReport, diag: PacingDiagnostics },
    Paused    { diag: PacingDiagnostics },
    Rejected  { reason: RejectReason },                               // nothing mutated
}
pub enum RejectReason { HorizonBehindClock { horizon, clock }, DuplicateCommand { command_id },
                        MalformedInput(ValidationError), ProfileMismatch { .. } }

pub struct CompletionReport {                                        // canonical
    pub cohorts: Vec<CohortRecord>,                                  // this request's records
    pub intents: Vec<BehaviorIntent>,                                // advisory, derived from state
    pub clock: LogicalTime,
    pub boundary_digest: Digest,
}
pub struct CohortRecord {                                            // canonical, partition-invariant
    pub at: LogicalTime,
    pub cohort_identity: Digest,                                     // H(due_time ‖ sorted WorkKey identity digests) | H(command identity)
    pub pre_engine_digest: Digest,
    pub batch_digest: Digest,
    pub committed: Vec<CommittedEffect>,                             // (definition, scope, value), sorted
    pub scheduled: Vec<WorkKey>,
    pub conflicts: Vec<WorkKeyConflict>,                             // Phase-1 poison reports
    pub rejection: Option<CohortRejection>,                          // typed; whole cohort
}
pub struct PacingDiagnostics { admitted_cohorts: u32, deferred_within_horizon: u32, paused: bool,
                               processing: Duration /* noncanonical */ }

impl Engine {
    pub fn process(&mut self, req: &Request, budget: Budget) -> Outcome;
    pub fn engine_digest(&self) -> Digest;          // cells ‖ scheduler ‖ bodies (by content) ‖ occurrences ‖ finalized
    pub fn boundary_digest(&self) -> Digest;        // H(engine_digest ‖ clock)
    pub fn snapshot(&self) -> Snapshot;             // §6
    pub fn restore(activated: ActivatedProfile, s: Snapshot) -> Result<Engine, RestoreError>;
}

// ---- spark_engine::evaluate ------------------------------------------------
pub(crate) struct Txn { now: LogicalTime, writes: BTreeMap<CellKey, CanonicalValue>,
                        scheduled: Vec<DueWorkItem>, bodies: BTreeMap<Digest, WorkBody>,
                        occurrence_bumps: BTreeMap<(DefinitionId, ScopeId), u64> }
pub(crate) fn evaluate_cohort(store: &StateStore, rules: &RuleArtifact, now: LogicalTime,
                              wave0: Vec<Effect>) -> Result<Txn, CohortRejection>;

// ---- spark_host --------------------------------------------------------------
pub struct Mailbox { capacity: usize, queue: VecDeque<Request> }     // head stays until Completed/Rejected
pub enum Offer { Accepted, MailboxFull }
pub fn consumer_step(mb: &mut Mailbox, e: &mut Engine, b: Budget) -> Option<Outcome>;
```

`SemanticCommandEnvelope`, `WorkKey`, `DueWorkItem`, `Scheduler`, `LogicalClock`, `StateStore`,
`CanonicalEncoder`, `Digest` are the existing Phase-1 types, used as they are.

## 3. Ownership of authoritative state

| State | Owner | Writer | Reader |
|---|---|---|---|
| physical/economic world truth, entity identity, game time | G.A.M.E. | G.A.M.E. | Spark via `host_owned` observations only |
| `host_owned` cells | Spark store (mirror) | `observe_host_owned` from an `Input` | rules |
| `spark_owned` cells (pressures, relationships, memories, goals) | Spark | `apply_spark_effect` from committed `Txn` | rules, intent derivation |
| `derived` cells | Spark | evaluator recomputation | rules, intents |
| resident timed work | Spark scheduler | committed `Txn`, host `Input` | the loop |
| logical clock | Spark, mirroring the host's last completed horizon | `process` completion | admission, decay |
| finalized command log | Spark timeline | `Input` completion | duplicate refusal, replay |
| `BehaviorIntent`s | produced by Spark, owned by nobody | derived at completion | G.A.M.E. (may execute, refuse, defer) |
| mailbox contents, budgets, diagnostics, telemetry | host | host | tuning; never rules |

## 4. Input → mailbox → consumer → evaluation → result

```
 G.A.M.E. adapter                 spark_host                       spark_engine
 ┌──────────────┐  offer(req)   ┌───────────────┐  process(head,B) ┌───────────────────────────────┐
 │ observations │ ────────────► │ Mailbox (cap) │ ───────────────► │ admit: h ≥ clock, not dup     │
 │ advance(T)   │ ◄──────────── │ head-until-   │                  │ loop: t = sched.next_due_time │
 │ confirmed    │  MailboxFull  │ complete      │ ◄─────────────── │   t ≤ h ? drain_due(t) : done │
 │ outcomes     │               │ telemetry     │  Completed/      │   budget ? evaluate@t : Paused│
 └──────────────┘               └───────────────┘  Paused/Rejected │   Txn overlay → commit|reject │
        ▲                                                          │ Input? execute@h as cohort    │
        │  CompletionReport { records, intents, boundary_digest }  │ clock.advance_to(h)           │
        └──────────────────────────────────────────────────────────┴───────────────────────────────┘
```

Step by step:

1. **Offer.** The adapter converts a game event into `Request::Input` (typed, validated
   against the activated profile) or `Request::Advance`. Full mailbox → `MailboxFull` to the
   producer. No engine state is touched.
2. **Present.** The consumer calls `process(head, budget)`. Requests are never reordered.
3. **Admit.** `h < clock.now()` → `Rejected::HorizonBehindClock`; known `command_id` →
   `Rejected::DuplicateCommand`. Both non-canonical, nothing mutated.
4. **Loop.** While the least due time `t ≤ h`: if the budget is spent, return `Paused`;
   otherwise `drain_due(t)` extracts exactly the slice at `t` (all later slots stay resident
   and byte-identical), the cohort evaluates at `now = t`, commits or rejects as one
   transaction, and its `CohortRecord` is appended.
5. **Input cohort.** If the request is an `Input`, after catch-up to `h` it is staged, fenced
   as a one-command range, and executed as its own cohort at `now = h` (one budget unit).
6. **Complete.** `clock.advance_to(h)` (the clock already stands at the last committed cohort); `CompletionReport` carries this request's records, the
   intents derived from committed state for the scopes touched, and `boundary_digest`.
7. **Pop.** Only after `Completed` or `Rejected` does the consumer pop the head.

## 5. Request lifecycle

| Transition | Rule | Retained state changed |
|---|---|---|
| start | `h ≥ clock`; not duplicate | none |
| pause | budget exhausted with due work `≤ h` remaining, or `Input` not yet executed | cohorts already committed; **clock = time of the last committed cohort** (see note) |
| resume | consumer re-presents the same head; engine starts again from live state | none carried over |
| complete | no resident slot `≤ h`; `Input` executed | `clock := h`; finalized log for an `Input` |

**Note (self-challenge correction, `04_SELF_CHALLENGE.md` A3).** The serialized-request
addendum keeps `F` unchanged by a pause. That is unsafe: if the mailbox is lost after a pause
that committed cohorts through `t = 30`, a host may offer `Advance(12)` and `Input@12`,
both admitted because `12 ≥ F = 0`, and the input rewrites a cell's `updated_at` from 15 to
12. The example's S1 check reproduces this and shows the correction: the clock advances to
each cohort's `due_time` as it commits (simulated-through time) and to `h` at completion.
A pause then leaves the clock at the last committed time, and the history after a lost
mailbox is exactly the history `Advance(30); Advance(12)→rejected`, indistinguishable from an
uninterrupted run. Partition and budget equivalence are unaffected (C1, C2, C7 pass).
| reject | at start only | none |

There is deliberately **no** active-request record inside the engine. A paused `Advance` is
just time-ordered progress; a paused `Input` has not been staged, so losing the mailbox loses a
host input at the host, never engine consistency.

## 6. Scheduled work, ordering, waves, transactions, retained effects

- **Unit of evaluation** (minimum version): the equal-`due_time` slice of one profile.
  One engine instance serves one profile; multi-profile cohort sequencing is removed from
  the engine and becomes "two engines". (Self-challenge §4 examines per-`WorkKey` units.)
- **Ordering**: `WorkKey` total order (Phase 1) orders items in a slice; effects within a
  wave are ordered by their canonical encoding, never by emission order; exact duplicates
  fold; a contested `WorkKey` is poisoned by the Phase-1 scheduler and reported in
  `CohortRecord::conflicts`, never executed.
- **Time**: every cohort evaluates at its own `due_time`; decay/recovery are closed-form on
  read from `updated_at`; `now ≥ updated_at` holds by construction. Created work must have
  `due_time > now` (zero-delay feedback is a typed rule defect → cohort rejection).
- **Waves**: wave 0 = the due items' own effects; wave *n+1* = threshold emissions caused by
  wave *n* writes, bounded by `max_wave_depth`; all waves write to one overlay `Txn`.
- **Transaction boundary = the cohort.** Commit is: apply writes with `updated_at = now`,
  bump occurrence counters, insert bodies, schedule created work. Rejection is: discard the
  overlay entirely, consume the cohort's `WorkKey`s, append a typed rejection record.
  **Retained effects after failure: none from the failed cohort; every earlier cohort stays
  committed; every later slot stays resident.** The example's C5 exercises this.

## 7. Snapshot / replay — the exact state, why each field exists

| Field | Read by (future behavior) | Recorded / reconstructed |
|---|---|---|
| `cells: (definition, scope) → (value, updated_at)` | every rule read; closed-form decay | snapshot; `updated_at` is required because decay is computed on read |
| `scheduler` slots (`WorkKey → Scheduled(payload hash) | Conflicted(evidence)`) | the loop; conflict reports | snapshot (needs a **new iteration/encode surface** on `Scheduler`; Phase 1 has none) |
| `bodies: payload hash → WorkBody` | wave 0 of a due item | snapshot; content-addressed, so the scheduler digest already commits to it; unreferenced bodies may be dropped at snapshot |
| `occurrences: (producer, scope) → next index` | allocating a fresh `WorkKey` that cannot collide with a resident or past one | snapshot; cannot be derived from resident slots because consumed occurrences must never be reused |
| `finalized: command_id → semantic hash` (the Phase-1 timeline) | duplicate refusal; replay input stream | snapshot / append-only log |
| `clock` | request admission; elapsed time on read | snapshot; advanced at every cohort commit and at completion; **cannot** be derived from cells (a no-effect advance leaves no trace) |
| activation hash + manifest hash | restore refuses a mismatch (ADR-0006) | snapshot header |

Not retained: request progress, pacing diagnostics, telemetry, mailbox, parent sets, wave
buffers, intents (re-derivable from state).

**Replay** = restore genesis snapshot; re-present the recorded request sequence (`Advance`
horizons and `Input`s) in order; compare `CohortRecord`s and `boundary_digest`. Partition
equivalence (example C2) means recorded intermediate `Advance` horizons may be collapsed.

## 8. Digests — what each commits to and why

| Digest | Commits to | Needed for |
|---|---|---|
| `engine_digest` | cells, scheduler slots (hence bodies), occurrence counters, finalized log; **not** the clock | the pre-cohort binding inside `batch_digest`; equal across partitions and budgets at every cohort position (C1, C2) |
| `batch_digest` | cohort identity ‖ `engine_digest` before it ‖ sorted committed effects ‖ scheduled keys ‖ rejection flag | a verifiable causal record per cohort; fixture replay compares this sequence |
| `boundary_digest` | `engine_digest` ‖ clock | snapshot validation and equal-state tests; equal only at equal completed horizons, because engines at different clocks answer the same past-dated request differently |

No emission-level identity digest is required in the minimum version: duplicate folding
within a wave uses the effect's canonical encoding, and explanation records the cohort's
committed effect list. Parent-set digests are a later "why" feature (§10).

## 9. Game-adapter surface (minimum)

```rust
trait SparkDevice {                                    // implemented by embedded and standalone forms
    fn offer(&mut self, req: Request) -> Offer;         // bounded; backpressure
    fn step(&mut self, budget: Budget) -> Option<Outcome>;
    fn read(&self, q: Query) -> QueryResult;            // cells, intents, health, clock
    fn snapshot(&self) -> Snapshot;
    fn restore(&mut self, s: Snapshot) -> Result<(), RestoreError>;
}
```

Adapter obligations (the whole contract): stable external ids mapped to `ScopeId`s; monotonic
integer logical time; offer every `Input` with `effective_time ≤ T` **before** `Advance(T)`;
treat `BehaviorIntent` as advice; return confirmed outcomes as `Input`s; persist the mailbox
head alongside the Spark snapshot if crash-resume of a paused request is required.

## 10. Minimum version versus later capabilities

| Minimum (vertical slice) | Later |
|---|---|
| one profile per engine; one sequencer; in-process embedding | standalone service; multi-profile deployments as multiple engines |
| rule ops: add/scale/clamp, linear decay, thresholds, delayed emission, cooldown | curves, weighted sums, probability gates via random addresses, aggregation |
| cohort = time slice; whole-cohort transaction | per-`WorkKey` transactions if C8's tradeoff is decided that way |
| `batch_digest` per cohort | emission identity + parent-set digests for explanations |
| snapshot = full state clone with canonical encoding | incremental/streamed snapshots |
| single-command fences via the existing ingress | multi-sequencer epochs, admission windows in anger |
| intents derived by a fixed scoring table | choice-context scoring, dialogue, screenplay |

## 11. Worked execution trace (from the example, budget = 1)

Fixture: drought trigger due at 10 (writes drought +50, schedules food-scarcity +30 at 15);
threshold scarcity ≥ 30 → hunger +20 (wave 1); threshold hunger ≥ 20 → schedule actor choice
at now+20 (wave 2); the actor choice writes intent.hunting +1 and schedules a hunting result
at now+100. Rain trigger due at 30 (drought −10). Mailbox capacity 2.

```
offer Advance(50)                              -> Accepted            queue=[A50]
step 1: process(A50, B=1)
   admit: 50 ≥ clock 0
   t=10 ≤ 50, budget ok: drain_due(10) = {drought@10}; evaluate now=10
      wave0: drought 0→50; schedule followup@15 (occ 0)
      commit; record(10)                                       created work INSIDE horizon
   t=15 ≤ 50, budget spent                     -> Paused        queue=[A50]  (head stays)
offer Input(cmd-1 @60)                         -> Accepted            queue=[A50, I60]
offer Advance(70)                              -> MailboxFull   backpressure to producer
step 2: process(A50, B=1)   (same head re-presented; nothing carried over)
   t=15: drain_due(15) = {followup@15}; now=15
      wave0: scarcity 0→30
      wave1: scarcity crossed 30 → hunger 0→20
      wave2: hunger crossed 20 → schedule actor_choice@35 (occ 0)
      commit; record(15)                                   -> Paused        queue=[A50, I60]
step 3: process(A50, B=1)
   t=30: rain; drought reads 50−1·(30−10)=30 → 20; commit; record(30)  -> Paused
step 4: process(A50, B=1)
   t=35: actor_choice; intent.hunting 0→1; schedule hunting_result@135 (occ 0)
      commit; record(35)
   t=135 > 50: stop.  no Input. clock := 50    -> Completed     pop -> queue=[I60]
step 5: process(I60, B=1)
   admit: 60 ≥ 50; cmd-1 unknown
   t=135 > 60: no scheduled work; Input cohort at now=60: drought 20−1·25→0, +5 → 5
   finalized[cmd-1]; record(60); clock := 60   -> Completed     pop -> queue=[]
resident afterwards: hunting_result@135 (beyond horizon; consumed by a later Advance ≥ 135)
```

Unbudgeted, the same fixture completes `Advance(50)` in one call with the identical four
`CohortRecord`s and the identical `boundary_digest` (example C1); ten `Advance(+5)` calls give
the same again (C2); a snapshot taken at the pause after step 1 and resumed gives the same
again (C7).
