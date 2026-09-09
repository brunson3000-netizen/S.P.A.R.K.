//! RESEARCH / PROPOSED DESIGN — NOT ADOPTED.
//!
//! Executable architectural example for the Fable S.P.A.R.K. architecture
//! challenge (2026-09-09). It is **not** an engine, not an accepted
//! implementation, and not a performance measurement.
//!
//! What it models, using the real Phase-1 `spark-core` kernel where possible:
//!
//! * a bounded mailbox feeding one consumer;
//! * one active request until its horizon completes; budget exhaustion pauses;
//! * cohort-at-a-time extraction with the unmodified Phase-1 `Scheduler`
//!   (`drain_due(least_due)`), each cohort evaluated at `now = due_time`;
//! * a whole-cohort transaction on an overlay map (waves inside);
//! * strictly-later delayed work; a typed failure path that rejects a cohort;
//! * the engine clock as the only request frontier;
//! * canonical per-cohort records and digests that are invariant under budget
//!   and call partition, plus noncanonical pacing diagnostics that are not;
//! * snapshot at a pause boundary and resume from the snapshot;
//! * two deliberately wrong variants that the checks must detect.

#![forbid(unsafe_code)]

use spark_core::clock::{LogicalClock, LogicalTime};
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{CanonicalTag, DefinitionId, ProfileId};
use spark_core::scheduler::{
    DueWorkItem, OccurrenceIndex, ScheduleDisposition, Scheduler, WorkKey, WorkKind,
    WorkPayload,
};
use spark_core::scope::{ScopeId, ScopeKind};
use std::collections::{BTreeMap, VecDeque};

// ---------------------------------------------------------------------------
// Vocabulary (tiny declarative rule model; integer fixed-point only)
// ---------------------------------------------------------------------------

/// A cell is addressed by definition and scope, exactly as Phase 1's `StateCell`.
pub type CellKey = (DefinitionId, ScopeId);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub value: i64,
    pub updated_at: LogicalTime,
}

/// One declared effect. `ScheduleWork` with `delay == 0` is a rule defect:
/// feedback must cross a scheduled boundary (blueprint §17/§19.5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    AddDelta { target: CellKey, delta: i64 },
    ScheduleWork { producer: DefinitionId, scope: ScopeId, delay: u64, body: WorkBody },
}

/// The content behind a scheduled `WorkPayload` hash. Content-addressed: the
/// scheduler digest already commits to it through the payload hash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkBody {
    pub effects: Vec<Effect>,
}

impl WorkBody {
    pub fn digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("fable_example_work_body_v0");
        enc.push_u64(self.effects.len() as u64);
        for e in &self.effects {
            e.canonicalize(&mut enc);
        }
        enc.finish()
    }
}

impl Effect {
    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            Effect::AddDelta { target, delta } => {
                enc.push_str("add_delta");
                target.0.canonicalize(enc);
                target.1.canonicalize(enc);
                enc.push_i64(*delta);
            }
            Effect::ScheduleWork { producer, scope, delay, body } => {
                enc.push_str("schedule_work");
                producer.canonicalize(enc);
                scope.canonicalize(enc);
                enc.push_u64(*delay);
                enc.push_digest(&body.digest());
            }
        }
    }
}

/// A threshold rule: when `watch` rises to at least `at_least` during a
/// cohort, emit `emit` in the next wave. Bounded by `max_wave_depth`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThresholdRule {
    pub id: DefinitionId,
    pub watch: CellKey,
    pub at_least: i64,
    pub emit: Effect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ruleset {
    /// Linear decay per logical tick, by definition (closed-form on read).
    pub decay_per_tick: BTreeMap<DefinitionId, i64>,
    pub thresholds: Vec<ThresholdRule>,
    pub max_wave_depth: u32,
}

// ---------------------------------------------------------------------------
// Requests, outcomes, records
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    /// "Simulate everything due up to and including `to`."
    Advance { to: LogicalTime },
    /// A host input effective at `at`, executed as its own cohort after
    /// catch-up to `at`. `command_id` gives duplicate protection.
    Input { command_id: String, at: LogicalTime, effects: Vec<Effect> },
}

impl Request {
    pub fn horizon(&self) -> LogicalTime {
        match self {
            Request::Advance { to } => *to,
            Request::Input { at, .. } => *at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectReason {
    /// `horizon < clock.now()`: the host asked to simulate backwards.
    HorizonBehindClock { horizon: LogicalTime, clock: LogicalTime },
    /// Same `command_id` already finalized: redelivery, idempotently refused.
    DuplicateCommand { command_id: String },
}

/// Noncanonical pacing diagnostics: deterministic, partition-dependent,
/// and never part of any digest.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CallDiagnostics {
    pub admitted_cohorts: u32,
    pub deferred_within_horizon: usize,
    pub paused: bool,
}

/// The request-result contract: exactly the records this request produced,
/// in order, plus the clock and boundary digest at completion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionReport {
    pub records: Vec<CohortRecord>,
    pub clock: LogicalTime,
    pub boundary_digest: Digest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Completed { report: CompletionReport, diag: CallDiagnostics },
    Paused { diag: CallDiagnostics },
    Rejected(RejectReason),
}

/// Why a cohort transaction was rejected as a whole (typed, canonical).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CohortRejection {
    ZeroDelayFeedback { producer: DefinitionId },
    WaveDepthExceeded { depth: u32 },
    /// A due item's body is absent from the body store: scheduled work
    /// disappeared. Never silently treated as executed.
    MissingBody { key: WorkKey, payload_hash: Digest },
    /// The stored body does not hash to the advertised payload digest.
    BodyDigestMismatch { key: WorkKey, advertised: Digest, actual: Digest },
    /// `now + delay` or an occurrence counter would overflow.
    TimeOverflow { producer: DefinitionId, delay: u64 },
    OccurrenceOverflow { producer: DefinitionId },
}

/// The canonical per-cohort record. Two runs over the same input history
/// must produce identical sequences of these regardless of budget or call
/// partition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CohortRecord {
    pub at: LogicalTime,
    pub cohort_identity: Digest,
    pub pre_engine_digest: Digest,
    pub batch_digest: Digest,
    pub committed: Vec<(CellKey, i64)>,
    /// Scheduled work with its payload digest (pass 2: payload included).
    pub scheduled: Vec<(WorkKey, Digest)>,
    /// Keys this transaction contested (committed poison) or drained as
    /// already-poisoned slices; never executed.
    pub conflicts: Vec<WorkKey>,
    pub rejection: Option<CohortRejection>,
}

/// Which variant of the loop to run. The wrong variants exist so the checks
/// can prove they detect the defect; they are not alternatives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Correct,
    /// The inherited V3-F01 shape: drain the whole prefix `<= horizon`
    /// first, then evaluate the buffered cohorts.
    WrongDrainWholePrefix,
    /// Evaluate every cohort at `now = horizon` instead of its due time.
    WrongEvalAtHorizon,
}

// ---------------------------------------------------------------------------
// Engine
// ---------------------------------------------------------------------------

/// When the engine clock advances. `CompletionOnly` is the serialized-request
/// addendum's rule (`F` unchanged by pause); `PerCohort` is the self-challenge
/// correction (clock = simulated-through time, advanced as cohorts commit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockPolicy {
    CompletionOnly,
    PerCohort,
}

/// The transaction unit (self-challenge A1 / Operator decision D1).
/// `TimeSlice` is the frozen v3 cohort; `PerWorkKey` commits or rejects
/// each due item separately, in `WorkKey` order, within the slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxnUnit {
    TimeSlice,
    PerWorkKey,
}

/// The complete retained engine state. Everything a snapshot must carry is a
/// field here; nothing else exists between calls.
#[derive(Debug, Clone)]
pub struct Engine {
    pub profile: ProfileId,
    pub rules: Ruleset,
    pub mode: Mode,
    pub clock_policy: ClockPolicy,
    pub txn_unit: TxnUnit,
    // --- canonical retained state ---
    pub cells: BTreeMap<CellKey, Cell>,
    pub scheduler: Scheduler,
    pub bodies: BTreeMap<Digest, WorkBody>,
    pub occurrences: BTreeMap<(DefinitionId, ScopeId), u64>,
    pub finalized: BTreeMap<String, Digest>,
    pub clock: LogicalClock,
    // --- delivery state (engine-owned, snapshot-carried): records committed
    //     since the last completed request, awaiting the CompletionReport ---
    pub undelivered: Vec<CohortRecord>,
    // --- canonical output log (append-only; partition-invariant; test oracle) ---
    pub trace: Vec<CohortRecord>,
    // --- noncanonical ---
    pub diagnostics: Vec<CallDiagnostics>,
}

/// Overlay transaction for one cohort: reads fall through to the engine,
/// writes land here until commit. An ordinary `BTreeMap` is the whole
/// "wave/transaction protocol".
struct Txn {
    now: LogicalTime,
    writes: BTreeMap<CellKey, i64>,
    scheduled: Vec<DueWorkItem>,
    new_bodies: BTreeMap<Digest, WorkBody>,
    occurrence_bumps: BTreeMap<(DefinitionId, ScopeId), u64>,
}

/// Pass-2 regression switch: `true` reproduces the pass-1 batch digest that
/// omitted scheduled payload hashes (captured failing in results_before_fixes.txt).
pub const HASH_SCHEDULED_PAYLOADS_DISABLED_FOR_REGRESSION: bool = false;

fn work_kind() -> WorkKind {
    // The example uses one work kind; a validated static literal would be
    // used in production. Fallible construction is fine here.
    WorkKind::new(CanonicalTag::new("trigger.evaluate").unwrap_or_else(|_| unreachable!()))
}

impl Engine {
    pub fn new(profile: ProfileId, rules: Ruleset, mode: Mode) -> Self {
        Engine {
            profile,
            rules,
            mode,
            clock_policy: ClockPolicy::PerCohort,
            txn_unit: TxnUnit::TimeSlice,
            cells: BTreeMap::new(),
            scheduler: Scheduler::new(),
            bodies: BTreeMap::new(),
            occurrences: BTreeMap::new(),
            finalized: BTreeMap::new(),
            clock: LogicalClock::new(LogicalTime::ZERO),
            undelivered: Vec::new(),
            trace: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    // ---- reads --------------------------------------------------------

    /// Closed-form decayed value at `now`. Elapsed is never negative because
    /// `now >= updated_at` holds by construction (time-ordered loop).
    pub fn read(&self, key: &CellKey, now: LogicalTime, overlay: Option<&BTreeMap<CellKey, i64>>) -> i64 {
        if let Some(v) = overlay.and_then(|o| o.get(key)) {
            return *v;
        }
        match self.cells.get(key) {
            None => 0,
            Some(c) => {
                let decay = self.rules.decay_per_tick.get(&key.0).copied().unwrap_or(0);
                let elapsed = now.0.saturating_sub(c.updated_at.0) as i64;
                (c.value - decay.saturating_mul(elapsed)).max(0)
            }
        }
    }

    // ---- digests ------------------------------------------------------

    /// Commits to cells, scheduler slots (hence bodies, by content address),
    /// occurrence counters, and finalized commands. Excludes the clock.
    pub fn engine_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("fable_example_engine_v0");
        enc.push_u64(self.cells.len() as u64);
        for ((d, s), c) in &self.cells {
            d.canonicalize(&mut enc);
            s.canonicalize(&mut enc);
            enc.push_i64(c.value);
            c.updated_at.canonicalize(&mut enc);
        }
        enc.push_digest(&self.scheduler.canonical_state_digest());
        enc.push_u64(self.occurrences.len() as u64);
        for ((d, s), n) in &self.occurrences {
            d.canonicalize(&mut enc);
            s.canonicalize(&mut enc);
            enc.push_u64(*n);
        }
        enc.push_u64(self.finalized.len() as u64);
        for (id, dg) in &self.finalized {
            enc.push_str(id);
            enc.push_digest(dg);
        }
        enc.finish()
    }

    /// Snapshot commitment: engine digest plus the clock (the request
    /// frontier). Equal only at equal completed horizons — by design.
    pub fn boundary_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("fable_example_boundary_v0");
        enc.push_digest(&self.engine_digest());
        self.clock.now().canonicalize(&mut enc);
        enc.finish()
    }

    // ---- the request loop --------------------------------------------

    /// One `process` call. `budget` counts whole cohorts (>= 1). Returns
    /// `Paused` with nothing retained about the request; the consumer
    /// re-presents the same request to resume.
    pub fn process(&mut self, req: &Request, budget: u32) -> Outcome {
        let h = req.horizon();
        let now = self.clock.now();
        if h < now {
            return Outcome::Rejected(RejectReason::HorizonBehindClock { horizon: h, clock: now });
        }
        if let Request::Input { command_id, .. } = req {
            if self.finalized.contains_key(command_id) {
                return Outcome::Rejected(RejectReason::DuplicateCommand { command_id: command_id.clone() });
            }
        }
        let budget = budget.max(1);
        let mut diag = CallDiagnostics::default();

        match self.mode {
            Mode::Correct | Mode::WrongEvalAtHorizon if self.txn_unit == TxnUnit::PerWorkKey => {
                // Real brakes: take exactly one resident slot per unit of budget.
                // Nothing is extracted before the budget check, so a pause leaves
                // every unfinished item resident (never in a hidden buffer).
                while let Some(due) = self.scheduler.next_due_time() {
                    if due > h {
                        break;
                    }
                    if diag.admitted_cohorts >= budget {
                        diag.paused = true;
                        diag.deferred_within_horizon = self.count_due_within(h);
                        self.diagnostics.push(diag.clone());
                        return Outcome::Paused { diag };
                    }
                    let one = self.scheduler.take_next();
                    let eval_at = if self.mode == Mode::WrongEvalAtHorizon { h } else { due };
                    if one.due.is_empty() {
                        // A poisoned slot: reported, never executed, consumes one unit.
                        self.run_poisoned(due, one.conflicted.into_iter().map(|c| c.key().clone()).collect());
                    } else {
                        self.run_cohort(due, eval_at, one.due);
                    }
                    diag.admitted_cohorts += 1;
                }
            }
            Mode::Correct | Mode::WrongEvalAtHorizon => {
                while let Some(due) = self.scheduler.next_due_time() {
                    if due > h {
                        break;
                    }
                    if diag.admitted_cohorts >= budget {
                        diag.paused = true;
                        diag.deferred_within_horizon = self.count_due_within(h);
                        self.diagnostics.push(diag.clone());
                        return Outcome::Paused { diag };
                    }
                    // Exactly the slice at `due`; later slots stay resident.
                    let drained = self.scheduler.drain_due(due);
                    let eval_at = if self.mode == Mode::WrongEvalAtHorizon { h } else { due };
                    match self.txn_unit {
                        TxnUnit::TimeSlice => {
                            if drained.due.is_empty() {
                                self.run_poisoned(due, drained.conflicted.into_iter().map(|c| c.key().clone()).collect());
                            } else {
                                self.run_cohort_with_conflicts(due, eval_at, drained.due, drained.conflicted.into_iter().map(|c| c.key().clone()).collect());
                            }
                            diag.admitted_cohorts += 1;
                        }
                        TxnUnit::PerWorkKey => unreachable!("handled by the per-item loop"),
                    }
                }
            }
            Mode::WrongDrainWholePrefix => {
                // Inherited shape: everything <= h leaves the scheduler at once.
                let drained = self.scheduler.drain_due(h);
                let mut by_time: BTreeMap<LogicalTime, Vec<DueWorkItem>> = BTreeMap::new();
                for item in drained.due {
                    by_time.entry(item.key.due_time).or_default().push(item);
                }
                for (due, items) in by_time {
                    self.run_cohort(due, due, items);
                    diag.admitted_cohorts += 1;
                }
            }
        }

        if let Request::Input { command_id, at, effects } = req {
            if diag.admitted_cohorts >= budget {
                diag.paused = true;
                self.diagnostics.push(diag.clone());
                return Outcome::Paused { diag };
            }
            // Finalize + execute atomically at completion, as its own cohort.
            let body = WorkBody { effects: effects.clone() };
            self.run_input_cohort(*at, command_id, body);
            diag.admitted_cohorts += 1;
        }

        // Completion: advance the clock, then hand over exactly this request's records.
        let _ = self.clock.advance_to(h); // never backwards: h >= now checked above
        self.diagnostics.push(diag.clone());
        let records = std::mem::take(&mut self.undelivered);
        let report = CompletionReport { records, clock: self.clock.now(), boundary_digest: self.boundary_digest() };
        Outcome::Completed { report, diag }
    }

    fn count_due_within(&self, h: LogicalTime) -> usize {
        // Diagnostics only: count resident slots with due <= h via a clone probe.
        let mut probe = self.scheduler.clone();
        probe.drain_due(h).due.len()
    }

    // ---- cohort evaluation ---------------------------------------------

    fn cohort_identity(at: LogicalTime, keys: &[WorkKey]) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("fable_example_cohort_v0");
        at.canonicalize(&mut enc);
        enc.push_u64(keys.len() as u64);
        for k in keys {
            enc.push_digest(&k.identity_digest());
        }
        enc.finish()
    }

    fn run_cohort(&mut self, at: LogicalTime, eval_at: LogicalTime, items: Vec<DueWorkItem>) {
        self.run_cohort_with_conflicts(at, eval_at, items, Vec::new());
    }

    /// A slice (or item) whose only content is poisoned keys: a canonical
    /// record with `conflicts`, no evaluation, no writes.
    fn run_poisoned(&mut self, at: LogicalTime, conflicts: Vec<WorkKey>) {
        let identity = Self::cohort_identity(at, &[]);
        let pre = self.engine_digest();
        let txn = Txn { now: at, writes: BTreeMap::new(), scheduled: Vec::new(), new_bodies: BTreeMap::new(), occurrence_bumps: BTreeMap::new() };
        self.finish_cohort(at, identity, pre, Ok(txn), conflicts);
    }

    fn run_cohort_with_conflicts(&mut self, at: LogicalTime, eval_at: LogicalTime, mut items: Vec<DueWorkItem>, drained_conflicts: Vec<WorkKey>) {
        items.sort_by(|a, b| a.key.cmp(&b.key)); // already sorted; explicit
        let keys: Vec<WorkKey> = items.iter().map(|i| i.key.clone()).collect();
        let identity = Self::cohort_identity(at, &keys);
        let pre = self.engine_digest();
        // Wave 0: the due work's own effects, in key order. A missing or
        // corrupted body is a typed rejection, never a silent no-op.
        let mut wave0 = Vec::new();
        let mut body_error = None;
        for item in &items {
            match self.bodies.get(&item.payload.canonical_payload_hash) {
                None => {
                    body_error = Some(CohortRejection::MissingBody { key: item.key.clone(), payload_hash: item.payload.canonical_payload_hash.clone() });
                    break;
                }
                Some(body) => {
                    let actual = body.digest();
                    if actual != item.payload.canonical_payload_hash {
                        body_error = Some(CohortRejection::BodyDigestMismatch { key: item.key.clone(), advertised: item.payload.canonical_payload_hash.clone(), actual });
                        break;
                    }
                    wave0.extend(body.effects.iter().cloned());
                }
            }
        }
        let outcome = match body_error {
            Some(e) => Err(e),
            None => self.evaluate_waves(eval_at, wave0),
        };
        self.finish_cohort(at, identity, pre, outcome, drained_conflicts);
    }

    fn run_input_cohort(&mut self, at: LogicalTime, command_id: &str, body: WorkBody) {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("fable_example_input_cohort_v0");
        enc.push_str(command_id);
        at.canonicalize(&mut enc);
        enc.push_digest(&body.digest());
        let identity = enc.finish();
        let pre = self.engine_digest();
        let outcome = self.evaluate_waves(at, body.effects.clone());
        self.finalized.insert(command_id.to_string(), body.digest());
        self.finish_cohort(at, identity, pre, outcome, Vec::new());
    }

    /// All waves run on one overlay. Rejection anywhere discards everything:
    /// whole-cohort atomicity by construction.
    fn evaluate_waves(&self, now: LogicalTime, wave0: Vec<Effect>) -> Result<Txn, CohortRejection> {
        let mut txn = Txn {
            now,
            writes: BTreeMap::new(),
            scheduled: Vec::new(),
            new_bodies: BTreeMap::new(),
            occurrence_bumps: BTreeMap::new(),
        };
        let mut pending = wave0;
        let mut depth = 0u32;
        while !pending.is_empty() {
            if depth > self.rules.max_wave_depth {
                return Err(CohortRejection::WaveDepthExceeded { depth });
            }
            // Deterministic order within a wave: canonical encoding of the
            // effect, independent of emission order.
            pending.sort_by_key(|e| {
                let mut enc = CanonicalEncoder::new();
                e.canonicalize(&mut enc);
                enc.finish()
            });
            let before: BTreeMap<CellKey, i64> = self
                .rules
                .thresholds
                .iter()
                .map(|r| (r.watch.clone(), self.read(&r.watch, now, Some(&txn.writes))))
                .collect();
            for e in pending.drain(..) {
                self.apply(&mut txn, e)?;
            }
            // Threshold emissions for the next wave: rising crossings only.
            let mut next = Vec::new();
            for r in &self.rules.thresholds {
                let b = before.get(&r.watch).copied().unwrap_or(0);
                let a = self.read(&r.watch, now, Some(&txn.writes));
                if b < r.at_least && a >= r.at_least {
                    next.push(r.emit.clone());
                }
            }
            pending = next;
            depth = depth.saturating_add(1);
        }
        Ok(txn)
    }

    fn apply(&self, txn: &mut Txn, e: Effect) -> Result<(), CohortRejection> {
        match e {
            Effect::AddDelta { target, delta } => {
                let cur = self.read(&target, txn.now, Some(&txn.writes));
                txn.writes.insert(target, cur.saturating_add(delta).max(0));
            }
            Effect::ScheduleWork { producer, scope, delay, body } => {
                if delay == 0 {
                    return Err(CohortRejection::ZeroDelayFeedback { producer });
                }
                let due_time = match txn.now.0.checked_add(delay) {
                    Some(t) => LogicalTime(t),
                    None => return Err(CohortRejection::TimeOverflow { producer, delay }),
                };
                let k = (producer.clone(), scope.clone());
                let base = self.occurrences.get(&k).copied().unwrap_or(0);
                let bump = txn.occurrence_bumps.entry(k).or_insert(0);
                // The counter after commit is base + bump + 1; it must fit, or the
                // transaction rejects here with nothing written (found by P4b: the
                // pass-1 commit path overflowed in debug and wrapped in release).
                let next_bump = match bump.checked_add(1) {
                    Some(b) => b,
                    None => return Err(CohortRejection::OccurrenceOverflow { producer }),
                };
                if base.checked_add(next_bump).is_none() {
                    return Err(CohortRejection::OccurrenceOverflow { producer });
                }
                let occurrence = base.saturating_add(*bump); // fits: checked above
                *bump = next_bump;
                let key = WorkKey {
                    due_time,
                    profile_id: self.profile.clone(),
                    producer_definition_id: producer,
                    scope_id: scope,
                    occurrence_index: OccurrenceIndex(occurrence),
                    work_kind: work_kind(),
                };
                let digest = body.digest();
                txn.new_bodies.insert(digest.clone(), body);
                txn.scheduled.push(DueWorkItem { key, payload: WorkPayload::new(digest) });
            }
        }
        Ok(())
    }

    fn finish_cohort(&mut self, at: LogicalTime, identity: Digest, pre: Digest, outcome: Result<Txn, CohortRejection>, drained_conflicts: Vec<WorkKey>) {
        let mut record = CohortRecord {
            at,
            cohort_identity: identity,
            pre_engine_digest: pre,
            batch_digest: Digest::ZERO,
            committed: Vec::new(),
            scheduled: Vec::new(),
            conflicts: drained_conflicts,
            rejection: None,
        };
        match outcome {
            Err(rej) => {
                record.rejection = Some(rej);
            }
            Ok(txn) => {
                for (k, v) in txn.writes {
                    self.cells.insert(k.clone(), Cell { value: v, updated_at: at });
                    record.committed.push((k, v));
                }
                for (k, n) in txn.occurrence_bumps {
                    let slot = self.occurrences.entry(k).or_insert(0);
                    *slot = slot.checked_add(n).unwrap_or(u64::MAX); // apply() guarantees it fits
                }
                self.bodies.extend(txn.new_bodies);
                for item in txn.scheduled {
                    match self.scheduler.schedule(item.clone()) {
                        ScheduleDisposition::Scheduled | ScheduleDisposition::AlreadyScheduledIdempotent => {
                            record.scheduled.push((item.key, item.payload.canonical_payload_hash));
                        }
                        ScheduleDisposition::Conflicted(_) => {
                            // Committed poison (ADR-0003 rule): the cell writes stand, the
                            // contested key is poisoned, and the record says so. Rolling the
                            // newcomer back would be arrival-order authority.
                            record.conflicts.push(item.key);
                        }
                    }
                }
                record.conflicts.sort();
            }
        }
        let mut enc = CanonicalEncoder::new();
        enc.push_str("fable_example_batch_v0");
        enc.push_digest(&record.cohort_identity);
        enc.push_digest(&record.pre_engine_digest);
        enc.push_u64(record.committed.len() as u64);
        for ((d, s), v) in &record.committed {
            d.canonicalize(&mut enc);
            s.canonicalize(&mut enc);
            enc.push_i64(*v);
        }
        enc.push_u64(record.scheduled.len() as u64);
        for (k, payload) in &record.scheduled {
            enc.push_digest(&k.identity_digest());
            if !HASH_SCHEDULED_PAYLOADS_DISABLED_FOR_REGRESSION {
                enc.push_digest(payload);
            }
        }
        enc.push_u64(record.conflicts.len() as u64);
        for k in &record.conflicts {
            enc.push_digest(&k.identity_digest());
        }
        enc.push_bool(record.rejection.is_some());
        if let Some(r) = &record.rejection {
            enc.push_str(&format!("{:?}", r));
        }
        record.batch_digest = enc.finish();
        self.undelivered.push(record.clone());
        self.trace.push(record);
        if self.clock_policy == ClockPolicy::PerCohort {
            // Simulated-through time: a pause after this cohort leaves the
            // clock here, so no later request may target an earlier time.
            let _ = self.clock.advance_to(at);
        }
    }

    /// Host-side seeding of initial work (a fixture convenience; in the real
    /// system this arrives as `Input` requests).
    pub fn seed_work(&mut self, due: LogicalTime, producer: &str, scope: &ScopeId, body: WorkBody) -> WorkKey {
        let producer = DefinitionId::new(producer).unwrap_or_else(|_| unreachable!());
        let k = (producer.clone(), scope.clone());
        let occ = self.occurrences.entry(k).or_insert(0);
        let key = WorkKey {
            due_time: due,
            profile_id: self.profile.clone(),
            producer_definition_id: producer,
            scope_id: scope.clone(),
            occurrence_index: OccurrenceIndex(*occ),
            work_kind: work_kind(),
        };
        *occ += 1;
        let d = body.digest();
        self.bodies.insert(d.clone(), body);
        let _ = self.scheduler.schedule(DueWorkItem { key: key.clone(), payload: WorkPayload::new(d) });
        key
    }
}

// ---------------------------------------------------------------------------
// Mailbox + consumer (the whole host-side contract)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offer {
    Accepted,
    MailboxFull,
}

#[derive(Debug, Clone)]
pub struct Mailbox {
    pub capacity: usize,
    pub queue: VecDeque<Request>,
}

impl Mailbox {
    pub fn new(capacity: usize) -> Self {
        Mailbox { capacity, queue: VecDeque::new() }
    }
    pub fn offer(&mut self, r: Request) -> Offer {
        if self.queue.len() >= self.capacity {
            return Offer::MailboxFull;
        }
        self.queue.push_back(r);
        Offer::Accepted
    }
}

/// One consumer step: present the head; pop it only on completion or
/// rejection. Returns what the engine said.
pub fn consumer_step(mailbox: &mut Mailbox, engine: &mut Engine, budget: u32) -> Option<Outcome> {
    let head = mailbox.queue.front()?.clone();
    let outcome = engine.process(&head, budget);
    match outcome {
        Outcome::Paused { .. } => {}
        _ => {
            mailbox.queue.pop_front();
        }
    }
    Some(outcome)
}

/// Drive the mailbox until empty.
pub fn drain_mailbox(mailbox: &mut Mailbox, engine: &mut Engine, budget: u32) -> Vec<Outcome> {
    let mut log = Vec::new();
    while let Some(o) = consumer_step(mailbox, engine, budget) {
        log.push(o);
    }
    log
}

// ---------------------------------------------------------------------------
// Fixture helpers
// ---------------------------------------------------------------------------

pub fn def(s: &str) -> DefinitionId {
    DefinitionId::new(s).unwrap_or_else(|_| unreachable!())
}
pub fn scope(kind: ScopeKind, s: &str) -> ScopeId {
    ScopeId::new(kind, s).unwrap_or_else(|_| unreachable!())
}
pub fn profile() -> ProfileId {
    ProfileId::new("game-world").unwrap_or_else(|_| unreachable!())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The V3-F01 mechanism claim, on the real Phase-1 scheduler: with one
    /// accessor, `drain_due(next_due_time())` extracts exactly the least
    /// time-slice and leaves every later slot resident and digest-identical.
    #[test]
    fn real_scheduler_extracts_one_time_slice_and_leaves_the_rest_resident() {
        let mut s = Scheduler::new();
        let sc = scope(ScopeKind::Actor, "a1");
        let mk = |due: u64, occ: u64| DueWorkItem {
            key: WorkKey {
                due_time: LogicalTime(due),
                profile_id: profile(),
                producer_definition_id: def("trigger.x"),
                scope_id: sc.clone(),
                occurrence_index: OccurrenceIndex(occ),
                work_kind: work_kind(),
            },
            payload: WorkPayload::new(WorkBody { effects: vec![] }.digest()),
        };
        for (d, o) in [(10, 0), (10, 1), (20, 0), (35, 0)] {
            assert!(matches!(s.schedule(mk(d, o)), ScheduleDisposition::Scheduled));
        }
        let mut expected_rest = Scheduler::new();
        for (d, o) in [(20, 0), (35, 0)] {
            expected_rest.schedule(mk(d, o));
        }
        assert_eq!(s.next_due_time(), Some(LogicalTime(10)));
        let out = s.drain_due(LogicalTime(10));
        assert_eq!(out.due.len(), 2);
        assert!(out.due.iter().all(|i| i.key.due_time == LogicalTime(10)));
        assert_eq!(s.canonical_state_digest(), expected_rest.canonical_state_digest());
        assert_eq!(s.next_due_time(), Some(LogicalTime(20)));
    }

    #[test]
    fn checks_binary_logic_is_budget_invariant_smoke() {
        let rules = Ruleset { decay_per_tick: BTreeMap::new(), thresholds: vec![], max_wave_depth: 2 };
        let mk = |mode| {
            let mut e = Engine::new(profile(), rules.clone(), mode);
            let sc = scope(ScopeKind::Settlement, "s");
            e.seed_work(LogicalTime(5), "t.a", &sc, WorkBody { effects: vec![Effect::AddDelta { target: (def("p.x"), sc.clone()), delta: 3 }] });
            e.seed_work(LogicalTime(9), "t.b", &sc, WorkBody { effects: vec![Effect::AddDelta { target: (def("p.x"), sc.clone()), delta: 4 }] });
            e
        };
        let mut a = mk(Mode::Correct);
        let mut b = mk(Mode::Correct);
        let req = Request::Advance { to: LogicalTime(10) };
        while matches!(a.process(&req, 1), Outcome::Paused { .. }) {}
        assert!(matches!(b.process(&req, 99), Outcome::Completed { .. }));
        assert_eq!(a.trace, b.trace);
        assert_eq!(a.boundary_digest(), b.boundary_digest());
        assert_ne!(a.diagnostics, b.diagnostics);
    }
}
