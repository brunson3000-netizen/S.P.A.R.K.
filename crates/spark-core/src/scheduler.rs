//! Due-work scheduler skeleton.
//!
//! S.P.A.R.K. has a deterministic scheduler rather than a universal
//! polling loop: the host advances the logical clock and S.P.A.R.K.
//! evaluates only work that is due (`CONTROLLING_BLUEPRINT_v0.2.md`
//! §18.1).
//!
//! # The re-foundation: the work key is the whole semantic identity
//!
//! `PHASE_1_REFOUNDATION_BRIEF_v0.1.md` §4 requires a complete semantic
//! work key. The previous implementation keyed work by
//! `(due_time, occurrence_index)` alone. That conflated independent
//! occurrence domains: two different triggers, each legitimately
//! scheduling *its own* occurrence 0 for the same due time, collided —
//! and the first one to arrive was retained while the second was rejected,
//! so reversing insertion order reversed which logical work survived. That
//! is precisely the arrival-order authority the determinism constitution
//! forbids, and it made per-producer/per-scope occurrence sequences (the
//! kind a save actually persists) unusable.
//!
//! [`WorkKey`] is now the full semantic identity the blueprint describes:
//!
//! ```text
//! due_time
//! profile_id
//! producer_definition_id
//! scope_id
//! occurrence_index
//! work_kind
//! ```
//!
//! Occurrence indexes are therefore producer- and scope-local, which is
//! what makes them persistence-friendly: a save records "trigger
//! `trigger.weather.drought`, scope `settlement.pontafique`, occurrence
//! 7", and resuming it cannot collide with an unrelated producer's
//! occurrence 7. The key's field order is also its `Ord` order, so drain
//! order is a stable total order over semantic identity and never over
//! call order.
//!
//! # Re-Foundation v2: conflict resolution is order-independent too
//!
//! The complete key made *identity* order-independent, but the conflict
//! rule did not follow: "keep the installed payload, reject the newcomer"
//! reports a conflict in either order while retaining precisely the
//! arrival-order artifact the determinism constitution forbids — reverse
//! the two calls and the other payload survives, with a different
//! canonical state digest and different drained work.
//!
//! ADR-0003 already contains the correct rule for a contested canonical
//! slot: a conflicting claim **poisons** it, and no arrival picks a
//! winner. [`Scheduler`] now applies that same rule to a contested
//! [`WorkKey`]. Payload is still separated from identity — same key plus
//! equal payload is idempotent — but same key plus a *distinct* payload
//! transitions the key to a conflicted state carrying an order-independent
//! sorted evidence set of every distinct payload hash that claimed it.
//! Neither payload remains executable, and a conflicted key is reported
//! through [`DrainOutcome::conflicted`] rather than executed.

use crate::clock::LogicalTime;
use crate::evidence::BoundedClaimSet;
use crate::hash::{CanonicalEncoder, Digest};
use crate::id::{CanonicalTag, DefinitionId, ProfileId};
use crate::scope::ScopeId;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Identifies one scheduled occurrence within its producer's and scope's
/// own sequence (`CONTROLLING_BLUEPRINT_v0.2.md` §28 item 2: "scheduled
/// occurrence identity derives from persisted logical occurrence indexes,
/// never batching/worker/catch-up chunks").
///
/// The producer — a trigger/rule evaluator, or a recurrence resumer
/// reading a prior save — supplies this value; the scheduler never
/// manufactures one from call order. Because [`WorkKey`] carries the
/// producer and scope alongside it, this index only has to be unique
/// *within* that producer/scope, not globally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OccurrenceIndex(pub u64);

impl OccurrenceIndex {
    /// Rejects overflow when a caller derives the next occurrence index in
    /// a persisted sequence (e.g. resuming recurrence after a save),
    /// rather than silently wrapping.
    pub fn checked_next(&self) -> Option<OccurrenceIndex> {
        self.0.checked_add(1).map(OccurrenceIndex)
    }
}

/// The kind of scheduled work. Canonical identity, so it is a bounded
/// validated [`CanonicalTag`] rather than an unbounded `String` — the same
/// discipline [`crate::timeline::CommandKind`] applies to command kinds.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkKind(CanonicalTag);

impl WorkKind {
    pub fn new(tag: CanonicalTag) -> Self {
        WorkKind(tag)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.0.canonicalize(enc);
    }
}

impl fmt::Display for WorkKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The complete semantic identity of one scheduled unit of work.
///
/// Field order is deliberate and load-bearing: the derived [`Ord`] makes
/// `due_time` primary (work is drained in time order) and then breaks ties
/// by profile, producer, scope, occurrence, and kind — a stable total
/// order over semantic identity, never over call order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkKey {
    pub due_time: LogicalTime,
    pub profile_id: ProfileId,
    pub producer_definition_id: DefinitionId,
    pub scope_id: ScopeId,
    pub occurrence_index: OccurrenceIndex,
    pub work_kind: WorkKind,
}

impl WorkKey {
    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.due_time.canonicalize(enc);
        self.profile_id.canonicalize(enc);
        self.producer_definition_id.canonicalize(enc);
        self.scope_id.canonicalize(enc);
        enc.push_u64(self.occurrence_index.0);
        self.work_kind.canonicalize(enc);
    }

    /// The canonical identity digest of this work key, suitable for
    /// binding a persisted delayed obligation to its exact logical
    /// identity (ADR-0006 delayed-obligation binding).
    pub fn identity_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("work_key_identity");
        self.canonicalize(&mut enc);
        enc.finish()
    }
}

/// The scheduled payload attached to a [`WorkKey`].
///
/// Phase 1 does not evaluate work and has no rule/effect language, so the
/// payload is an opaque, already-hashed blob — exactly as the timeline
/// treats a command payload. Keeping it separate from [`WorkKey`] is what
/// makes "same identity, different content" a detectable conflict rather
/// than a silent overwrite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkPayload {
    pub canonical_payload_hash: Digest,
}

impl WorkPayload {
    pub fn new(canonical_payload_hash: Digest) -> Self {
        Self {
            canonical_payload_hash,
        }
    }

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_digest(&self.canonical_payload_hash);
    }
}

/// One unit of due work: its complete semantic identity plus its payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DueWorkItem {
    pub key: WorkKey,
    pub payload: WorkPayload,
}

impl DueWorkItem {
    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.key.canonicalize(enc);
        self.payload.canonicalize(enc);
    }
}

/// The maximum number of distinct competing payload hashes a conflicted
/// key retains as *exposed* evidence.
///
/// The retained set is always the numerically smallest hashes of the whole
/// claim set, so it is a function of the claim set rather than of arrival
/// order.
pub const MAX_CONFLICT_EVIDENCE: usize = 16;

/// The maximum number of distinct claims one conflicted key *tracks*
/// internally in order to report an exact `omitted_distinct` count.
///
/// Counting distinct claims exactly requires remembering them, so the
/// tracking set is capped to keep a conflicted key's memory finite. Up to
/// this many distinct claims the omitted count is exact; beyond it the
/// key reports [`WorkKeyConflict::evidence_truncated`] and the count
/// saturates at `MAX_CONFLICT_TRACKED_CLAIMS - MAX_CONFLICT_EVIDENCE`.
/// Every one of those three values (retained set, count, truncation flag)
/// is the smallest-first function of the claim set, so all three remain
/// arrival-order independent past the cap — which is the property that
/// matters, and which a plain "count evictions" scheme would lose the
/// moment a duplicate of an already-omitted claim arrived.
pub const MAX_CONFLICT_TRACKED_CLAIMS: usize = 256;

/// Order-independent evidence that one complete semantic [`WorkKey`] was
/// claimed by more than one distinct payload.
///
/// A conflict is a legitimate, deterministic logical outcome — not an
/// error — exactly like a poisoned timeline slot. The evidence is a sorted
/// set, so the same claim set produces one value in every arrival order.
/// Fields are private: like a [`crate::timeline::StageAcknowledgement`],
/// this is evidence the scheduler issued, not a value a caller asserts.
///
/// ```compile_fail
/// use spark_core::scheduler::WorkKeyConflict;
/// use std::collections::BTreeSet;
/// let forged = WorkKeyConflict {
///     competing_payload_hashes: BTreeSet::new(),
///     omitted_distinct: 0,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkKeyConflict {
    key: WorkKey,
    competing_payload_hashes: BTreeSet<Digest>,
    omitted_distinct: u64,
    evidence_truncated: bool,
}

impl WorkKeyConflict {
    /// The complete semantic identity that was ambiguously claimed.
    pub fn key(&self) -> &WorkKey {
        &self.key
    }

    /// The retained competing payload hashes, ascending. Bounded by
    /// [`MAX_CONFLICT_EVIDENCE`]; always the smallest hashes of the whole
    /// claim set.
    pub fn competing_payload_hashes(&self) -> &BTreeSet<Digest> {
        &self.competing_payload_hashes
    }

    /// How many further distinct payload hashes claimed this key without
    /// being retained as evidence.
    pub fn omitted_distinct(&self) -> u64 {
        self.omitted_distinct
    }

    /// Whether the claim set exceeded [`MAX_CONFLICT_TRACKED_CLAIMS`], in
    /// which case [`omitted_distinct`](Self::omitted_distinct) is a
    /// saturated lower bound rather than an exact count.
    pub fn evidence_truncated(&self) -> bool {
        self.evidence_truncated
    }
}

impl fmt::Display for WorkKeyConflict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "work key (t={}, profile={}, producer={}, scope={}, occurrence={}, kind={}) is conflicted: {} distinct competing payload(s) retained, {} omitted{}",
            self.key.due_time.0,
            self.key.profile_id,
            self.key.producer_definition_id,
            self.key.scope_id,
            self.key.occurrence_index.0,
            self.key.work_kind,
            self.competing_payload_hashes.len(),
            self.omitted_distinct,
            if self.evidence_truncated {
                " (at least)"
            } else {
                ""
            }
        )
    }
}

/// The internal claim record behind a conflicted key: the smallest
/// distinct hashes seen, capped for finiteness.
///
/// This is [`BoundedClaimSet`], the one implementation this kernel has of
/// a bounded order-independent contested-claim set, instantiated at the
/// scheduler's caps; [`crate::timeline::SlotPoisonEvidence`] is the same
/// machinery at the timeline's. The two used to be line-for-line
/// duplicates kept in step by doc comments, which is a convention
/// standing where a type belongs.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ConflictEvidence(BoundedClaimSet<MAX_CONFLICT_EVIDENCE, MAX_CONFLICT_TRACKED_CLAIMS>);

impl ConflictEvidence {
    fn from_pair(a: Digest, b: Digest) -> Self {
        ConflictEvidence(BoundedClaimSet::from_pair(a, b))
    }

    /// Inserts one claim, retaining the numerically smallest hashes. This
    /// is idempotent and commutative, which is exactly what makes the
    /// resulting state a function of the claim set rather than of arrival
    /// order.
    fn insert(&mut self, hash: Digest) {
        self.0.insert(hash);
    }

    fn to_conflict(&self, key: &WorkKey) -> WorkKeyConflict {
        WorkKeyConflict {
            key: key.clone(),
            competing_payload_hashes: self.0.retained(),
            omitted_distinct: self.0.omitted_distinct(),
            evidence_truncated: self.0.truncated(),
        }
    }

    /// Commits to **every** tracked claim, not to the exposed
    /// [`MAX_CONFLICT_EVIDENCE`] projection — see
    /// [`BoundedClaimSet::canonicalize`] for why that distinction is the
    /// whole point of the type.
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.0.canonicalize(enc);
    }
}

/// The internal state of one occupied work slot.
#[derive(Debug, Clone, PartialEq, Eq)]
enum SlotState {
    Scheduled(WorkPayload),
    /// Two or more distinct payloads claimed this key. **No payload is
    /// retained**: retaining the first one is precisely the arrival-order
    /// privilege the determinism constitution forbids.
    Conflicted(ConflictEvidence),
}

/// The externally observable status of one work key's slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkSlotStatus {
    Empty,
    Scheduled,
    Conflicted,
}

/// The logical result of one [`Scheduler::schedule`] attempt.
///
/// Note that a conflict is **not** an `Err`. `schedule` is total: the only
/// way to fail to produce a slot is an occurrence-index overflow, which is
/// rejected earlier at key construction by
/// [`OccurrenceIndex::checked_next`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleDisposition {
    /// The semantic work key was unoccupied and is now scheduled.
    Scheduled,
    /// The identical item was already scheduled under this exact semantic
    /// key; idempotent, no state change.
    AlreadyScheduledIdempotent,
    /// The key is now — or already was — conflicted, and carries the
    /// current order-independent evidence. Neither the previously
    /// installed payload nor this one is executable.
    Conflicted(WorkKeyConflict),
}

impl ScheduleDisposition {
    /// A stable tag for deterministic transcripts.
    pub fn tag(&self) -> &'static str {
        match self {
            ScheduleDisposition::Scheduled => "scheduled",
            ScheduleDisposition::AlreadyScheduledIdempotent => "already_scheduled_idempotent",
            ScheduleDisposition::Conflicted(_) => "conflicted",
        }
    }

    pub fn conflict(&self) -> Option<&WorkKeyConflict> {
        match self {
            ScheduleDisposition::Conflicted(conflict) => Some(conflict),
            _ => None,
        }
    }
}

/// The result of one [`Scheduler::drain_due`] call.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DrainOutcome {
    /// Executable due work, in stable ascending [`WorkKey`] order.
    pub due: Vec<DueWorkItem>,
    /// Due-but-conflicted keys: removed from the queue and reported,
    /// never executed. Also in stable ascending [`WorkKey`] order.
    pub conflicted: Vec<WorkKeyConflict>,
}

/// A deterministic due-work queue with a stable total order over complete
/// semantic work identity, and an order-independent conflict rule.
///
/// # Conflict semantics
///
/// ADR-0003 already fixes the correct rule for a contested canonical slot:
/// a conflicting claim **poisons** the slot, and no arrival picks a
/// winner. The scheduler applies that same rule to a contested
/// [`WorkKey`]:
///
/// ```text
/// Empty              + P  ->  Scheduled(P)
/// Scheduled(P)       + P  ->  idempotent, no state change
/// Scheduled(P)       + Q  ->  Conflicted{h(P), h(Q)}   (P is NOT retained)
/// Conflicted{S}      + R  ->  Conflicted{S + h(R)}     (set insert; idempotent)
/// drain of a due conflicted key -> reported, removed, never executed
/// ```
///
/// Because the evidence is a sorted set of the same members in every
/// order, two schedulers that received the same logical claim set by any
/// call sequence are digest-identical and drain identically. After a
/// conflicted key is drained the key is free again, so a producer that has
/// resolved the ambiguity may reschedule it — typically under the next
/// occurrence index. That is the deterministic Phase-1 recovery; a
/// separate cancellation API is deliberately **not** introduced, because
/// it could race submission and thereby "choose history", which
/// ADR-0003 §11 forbids. In Phase 2/3 richer recovery flows through
/// canonical timeline commands like every other mutation.
///
/// # Seam noted for Phase 2 (no batch API exists in Phase 1)
///
/// If a batch scheduling API is ever added, its conflict handling must be
/// **atomic and order-independent**: a batch containing two conflicting
/// claims for one key must reach the same canonical state as the same
/// claims submitted individually in any order. Recording that requirement
/// here is a contract for the Phase-2 writer, not authorization to
/// implement the API.
#[derive(Debug, Default, Clone)]
pub struct Scheduler {
    slots: BTreeMap<WorkKey, SlotState>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Schedules one work item under its complete semantic identity.
    ///
    /// Two calls with an identical item are idempotent regardless of call
    /// order. Two calls that agree on the full semantic key but disagree
    /// on payload poison that key. Two calls that differ in *any* key
    /// field — including producer, scope, profile, or work kind — are
    /// independent work and both are retained.
    pub fn schedule(&mut self, item: DueWorkItem) -> ScheduleDisposition {
        let incoming = item.payload.canonical_payload_hash.clone();
        match self.slots.get_mut(&item.key) {
            None => {
                self.slots
                    .insert(item.key, SlotState::Scheduled(item.payload));
                ScheduleDisposition::Scheduled
            }
            Some(SlotState::Scheduled(installed)) => {
                if installed.canonical_payload_hash == incoming {
                    return ScheduleDisposition::AlreadyScheduledIdempotent;
                }
                let evidence =
                    ConflictEvidence::from_pair(installed.canonical_payload_hash.clone(), incoming);
                let conflict = evidence.to_conflict(&item.key);
                self.slots.insert(item.key, SlotState::Conflicted(evidence));
                ScheduleDisposition::Conflicted(conflict)
            }
            Some(SlotState::Conflicted(evidence)) => {
                evidence.insert(incoming);
                ScheduleDisposition::Conflicted(evidence.to_conflict(&item.key))
            }
        }
    }

    /// Removes every slot with `due_time <= now`, returning executable
    /// work and conflicted keys separately, each in stable ascending
    /// [`WorkKey`] order.
    ///
    /// Because `due_time` is the primary component of [`WorkKey`]'s
    /// ordering, the due slots are exactly a prefix of the map, so this
    /// walks entries off the front and stops at the first future one. No
    /// key is looked up a second time, so there is no fallible re-lookup
    /// and no panic path.
    pub fn drain_due(&mut self, now: LogicalTime) -> DrainOutcome {
        let mut outcome = DrainOutcome::default();
        while let Some(entry) = self.slots.first_entry() {
            if entry.key().due_time > now {
                break;
            }
            let (key, slot) = entry.remove_entry();
            match slot {
                SlotState::Scheduled(payload) => {
                    outcome.due.push(DueWorkItem { key, payload });
                }
                SlotState::Conflicted(evidence) => {
                    outcome.conflicted.push(evidence.to_conflict(&key));
                }
            }
        }
        outcome
    }

    /// RESEARCH-ONLY ADDITION (Fable architecture challenge, 2026-09-09;
    /// not adopted): the least due time of any resident slot, scheduled
    /// or conflicted. Together with `drain_due(least)` this yields exactly
    /// one time-slice per call while every later slot stays resident and
    /// byte-identical - the cohort-granular extraction V3-F01 asks for,
    /// without a new extraction protocol.
    pub fn next_due_time(&self) -> Option<LogicalTime> {
        self.slots.first_key_value().map(|(key, _)| key.due_time)
    }

    /// RESEARCH-ONLY ADDITION (Fable architecture challenge pass 2,
    /// 2026-09-09; not adopted): removes and returns exactly the first
    /// resident slot in stable `WorkKey` order, scheduled or conflicted,
    /// as a one-entry `DrainOutcome`. Nothing is buffered: an item is
    /// either resident or returned to the caller.
    pub fn take_next(&mut self) -> DrainOutcome {
        let mut outcome = DrainOutcome::default();
        if let Some(entry) = self.slots.first_entry() {
            let (key, slot) = entry.remove_entry();
            match slot {
                SlotState::Scheduled(payload) => outcome.due.push(DueWorkItem { key, payload }),
                SlotState::Conflicted(evidence) => outcome.conflicted.push(evidence.to_conflict(&key)),
            }
        }
        outcome
    }

    /// Whether the given complete semantic work key currently occupies a
    /// slot, scheduled **or** conflicted. Use
    /// [`slot_status`](Self::slot_status) when the distinction matters.
    pub fn contains(&self, key: &WorkKey) -> bool {
        self.slots.contains_key(key)
    }

    /// The observable status of one key's slot.
    pub fn slot_status(&self, key: &WorkKey) -> WorkSlotStatus {
        match self.slots.get(key) {
            None => WorkSlotStatus::Empty,
            Some(SlotState::Scheduled(_)) => WorkSlotStatus::Scheduled,
            Some(SlotState::Conflicted(_)) => WorkSlotStatus::Conflicted,
        }
    }

    /// The current conflict evidence for a key, if it is conflicted.
    pub fn conflict_of(&self, key: &WorkKey) -> Option<WorkKeyConflict> {
        match self.slots.get(key) {
            Some(SlotState::Conflicted(evidence)) => Some(evidence.to_conflict(key)),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// A deterministic digest of every occupied slot in stable key order,
    /// so two schedulers built by different call sequences can be compared
    /// for canonical equality.
    ///
    /// Scheduled and conflicted slots carry distinct domain tags, and a
    /// conflicted slot hashes **every** claim hash it still tracks — all
    /// of them, not the [`MAX_CONFLICT_EVIDENCE`] presentation projection
    /// — plus its truncation flag. Two keys contested by *different*
    /// claim sets therefore never collide even when their exposed
    /// evidence is identical, while the same claim set in any arrival
    /// order always agrees.
    pub fn canonical_state_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("scheduler_state");
        enc.push_u64(self.slots.len() as u64);
        for (key, slot) in &self.slots {
            let mut inner = CanonicalEncoder::new();
            key.canonicalize(&mut inner);
            match slot {
                SlotState::Scheduled(payload) => {
                    inner.push_str("slot.scheduled");
                    payload.canonicalize(&mut inner);
                }
                SlotState::Conflicted(evidence) => {
                    inner.push_str("slot.conflicted");
                    evidence.canonicalize(&mut inner);
                }
            }
            enc.push_block(&inner);
        }
        enc.finish()
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
mod tests {
    use super::*;
    use crate::hash::hash_bytes;
    use crate::scope::ScopeKind;

    fn profile() -> ProfileId {
        ProfileId::new("game-world").unwrap()
    }

    fn key(due: u64, producer: &str, scope: &str, occurrence: u64, kind: &str) -> WorkKey {
        WorkKey {
            due_time: LogicalTime(due),
            profile_id: profile(),
            producer_definition_id: DefinitionId::new(producer).unwrap(),
            scope_id: ScopeId::new(ScopeKind::Actor, scope).unwrap(),
            occurrence_index: OccurrenceIndex(occurrence),
            work_kind: WorkKind::new(CanonicalTag::new(kind).unwrap()),
        }
    }

    fn item(due: u64, producer: &str, scope: &str, occurrence: u64) -> DueWorkItem {
        DueWorkItem {
            key: key(due, producer, scope, occurrence, "trigger.evaluate"),
            payload: WorkPayload::new(hash_bytes(b"payload")),
        }
    }

    fn claim(payload: &str) -> DueWorkItem {
        DueWorkItem {
            key: key(5, "trigger.a", "bron", 0, "trigger.evaluate"),
            payload: WorkPayload::new(hash_bytes(payload.as_bytes())),
        }
    }

    /// Two producers may both schedule occurrence 0 at the same time
    /// without collision.
    #[test]
    fn independent_producers_may_share_an_occurrence_index() {
        let mut sched = Scheduler::new();
        assert_eq!(
            sched.schedule(item(5, "trigger.a", "bron", 0)),
            ScheduleDisposition::Scheduled
        );
        assert_eq!(
            sched.schedule(item(5, "trigger.b", "bron", 0)),
            ScheduleDisposition::Scheduled
        );
        assert_eq!(sched.len(), 2);
        assert!(sched.contains(&key(5, "trigger.a", "bron", 0, "trigger.evaluate")));
        assert!(sched.contains(&key(5, "trigger.b", "bron", 0, "trigger.evaluate")));
    }

    /// Occurrence identity is scope-local as well as producer-local.
    #[test]
    fn independent_scopes_may_share_an_occurrence_index() {
        let mut sched = Scheduler::new();
        sched.schedule(item(5, "trigger.a", "bron", 0));
        sched.schedule(item(5, "trigger.a", "mira", 0));
        assert_eq!(sched.len(), 2);
    }

    /// ...and work-kind-local.
    #[test]
    fn independent_work_kinds_may_share_an_occurrence_index() {
        let mut sched = Scheduler::new();
        let mut evaluate = item(5, "trigger.a", "bron", 0);
        let mut decay = evaluate.clone();
        decay.key.work_kind = WorkKind::new(CanonicalTag::new("state.decay").unwrap());
        evaluate.key.work_kind = WorkKind::new(CanonicalTag::new("trigger.evaluate").unwrap());
        sched.schedule(evaluate);
        sched.schedule(decay);
        assert_eq!(sched.len(), 2);
    }

    /// Independent profiles keep independent occurrence domains, so an
    /// MCI profile cannot displace a game profile's scheduled work.
    #[test]
    fn independent_profiles_may_share_an_occurrence_index() {
        let mut sched = Scheduler::new();
        let a = item(5, "trigger.a", "bron", 0);
        let mut b = a.clone();
        b.key.profile_id = ProfileId::new("mci-social").unwrap();
        sched.schedule(a);
        sched.schedule(b);
        assert_eq!(sched.len(), 2);
    }

    /// Reversed insertion of two independent items drains identically.
    #[test]
    fn reversed_insertion_of_independent_producers_drains_identically() {
        let mut forward = Scheduler::new();
        forward.schedule(item(5, "trigger.a", "bron", 0));
        forward.schedule(item(5, "trigger.b", "bron", 0));

        let mut reversed = Scheduler::new();
        reversed.schedule(item(5, "trigger.b", "bron", 0));
        reversed.schedule(item(5, "trigger.a", "bron", 0));

        assert_eq!(
            forward.canonical_state_digest(),
            reversed.canonical_state_digest()
        );
        assert_eq!(
            forward.drain_due(LogicalTime(5)),
            reversed.drain_due(LogicalTime(5))
        );
    }

    /// Same semantic key + same payload is idempotent.
    #[test]
    fn exact_duplicate_schedule_is_idempotent() {
        let mut sched = Scheduler::new();
        assert_eq!(
            sched.schedule(item(5, "trigger.a", "bron", 0)),
            ScheduleDisposition::Scheduled
        );
        let after_first = sched.canonical_state_digest();
        assert_eq!(
            sched.schedule(item(5, "trigger.a", "bron", 0)),
            ScheduleDisposition::AlreadyScheduledIdempotent
        );
        assert_eq!(sched.len(), 1);
        assert_eq!(after_first, sched.canonical_state_digest());
    }

    /// AT-B1 (replaces the insufficient
    /// `payload_conflict_rejects_in_either_order`, which compared only
    /// error shape and length): a same-key conflict reaches **one**
    /// canonical state and one drain result in either arrival order.
    #[test]
    fn same_key_conflict_state_is_arrival_order_independent() {
        let a = claim("payload.a");
        let b = claim("payload.b");
        let k = a.key.clone();

        let mut forward = Scheduler::new();
        forward.schedule(a.clone());
        let forward_disposition = forward.schedule(b.clone());

        let mut reversed = Scheduler::new();
        reversed.schedule(b);
        let reversed_disposition = reversed.schedule(a);

        assert_eq!(
            forward.canonical_state_digest(),
            reversed.canonical_state_digest()
        );
        assert_eq!(forward.slot_status(&k), WorkSlotStatus::Conflicted);
        assert_eq!(reversed.slot_status(&k), WorkSlotStatus::Conflicted);
        assert_eq!(forward_disposition, reversed_disposition);
        assert_eq!(
            forward.conflict_of(&k).unwrap().competing_payload_hashes(),
            reversed.conflict_of(&k).unwrap().competing_payload_hashes()
        );
        assert_eq!(
            forward
                .conflict_of(&k)
                .unwrap()
                .competing_payload_hashes()
                .len(),
            2
        );

        let forward_drain = forward.drain_due(LogicalTime(5));
        let reversed_drain = reversed.drain_due(LogicalTime(5));
        assert_eq!(forward_drain, reversed_drain);
        assert!(forward_drain.due.is_empty());
        assert_eq!(forward_drain.conflicted.len(), 1);
    }

    /// AT-B3: after a conflict, neither competing payload is executable.
    #[test]
    fn neither_payload_survives_a_conflict() {
        let a = claim("payload.a");
        let b = claim("payload.b");
        let mut sched = Scheduler::new();
        sched.schedule(a.clone());
        sched.schedule(b.clone());

        let outcome = sched.drain_due(LogicalTime(5));
        assert!(outcome.due.is_empty(), "no payload may survive a conflict");
        assert_eq!(outcome.conflicted.len(), 1);
        let evidence = outcome.conflicted[0].competing_payload_hashes();
        assert!(evidence.contains(&a.payload.canonical_payload_hash));
        assert!(evidence.contains(&b.payload.canonical_payload_hash));
        assert_eq!(outcome.conflicted[0].omitted_distinct(), 0);
        assert!(!outcome.conflicted[0].evidence_truncated());
    }

    /// AT-B2: a three-way conflict converges in all six permutations.
    #[test]
    fn three_way_conflict_converges_in_all_six_orders() {
        let claims = [claim("payload.a"), claim("payload.b"), claim("payload.c")];
        let permutations = [
            [0usize, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];

        let mut digests: BTreeSet<Digest> = BTreeSet::new();
        // `WorkKeyConflict` is a presentation value rather than a
        // canonical one — it deliberately has no `canonicalize` — so the
        // reports are compared as values, which is strictly stronger than
        // comparing a digest of them.
        let mut drains: Vec<WorkKeyConflict> = Vec::new();
        for permutation in permutations {
            let mut sched = Scheduler::new();
            for index in permutation {
                sched.schedule(claims[index].clone());
            }
            let conflict = sched.conflict_of(&claims[0].key).unwrap();
            assert_eq!(conflict.competing_payload_hashes().len(), 3);
            for c in &claims {
                assert!(conflict
                    .competing_payload_hashes()
                    .contains(&c.payload.canonical_payload_hash));
            }
            digests.insert(sched.canonical_state_digest());

            let outcome = sched.drain_due(LogicalTime(5));
            assert!(outcome.due.is_empty());
            drains.push(outcome.conflicted[0].clone());
        }
        assert_eq!(digests.len(), 1, "all six orders must converge");
        for drain in &drains {
            assert_eq!(drain, &drains[0], "all six orders must drain identically");
        }
    }

    /// AT-B4: re-submitting a payload already present in the evidence set
    /// changes nothing.
    #[test]
    fn exact_duplicate_into_conflicted_slot_is_idempotent() {
        let a = claim("payload.a");
        let b = claim("payload.b");
        let mut sched = Scheduler::new();
        sched.schedule(a.clone());
        sched.schedule(b);
        let after_conflict = sched.canonical_state_digest();

        let disposition = sched.schedule(a);
        assert!(matches!(disposition, ScheduleDisposition::Conflicted(_)));
        assert_eq!(after_conflict, sched.canonical_state_digest());
    }

    /// AT-B5: a drained conflict frees the key, a corrected reschedule
    /// then succeeds, and replaying the whole sequence is digest-identical.
    #[test]
    fn conflict_report_and_requeue_are_deterministic() {
        fn run() -> (Digest, Digest, usize) {
            let mut sched = Scheduler::new();
            sched.schedule(claim("payload.a"));
            sched.schedule(claim("payload.b"));
            let outcome = sched.drain_due(LogicalTime(5));
            let after_drain = sched.canonical_state_digest();
            // The key is free again; a producer that resolved the
            // ambiguity may reschedule it.
            assert_eq!(
                sched.schedule(claim("payload.resolved")),
                ScheduleDisposition::Scheduled
            );
            (
                after_drain,
                sched.canonical_state_digest(),
                outcome.conflicted.len(),
            )
        }
        let first = run();
        let second = run();
        assert_eq!(first, second);
        assert_eq!(first.2, 1);
    }

    /// AT-B6: more distinct claims than the evidence cap still converge —
    /// the retained set is the smallest hashes of the claim set and the
    /// omitted count is exact, in any arrival order.
    #[test]
    fn evidence_cap_is_order_independent() {
        let claims: Vec<DueWorkItem> = (0..(MAX_CONFLICT_EVIDENCE * 3))
            .map(|i| claim(&format!("payload.{i}")))
            .collect();

        let mut forward = Scheduler::new();
        for c in &claims {
            forward.schedule(c.clone());
        }
        let mut reversed = Scheduler::new();
        for c in claims.iter().rev() {
            reversed.schedule(c.clone());
        }
        // A third, interleaved order for good measure.
        let mut interleaved = Scheduler::new();
        for c in claims.iter().step_by(2) {
            interleaved.schedule(c.clone());
        }
        for c in claims.iter().skip(1).step_by(2) {
            interleaved.schedule(c.clone());
        }

        assert_eq!(
            forward.canonical_state_digest(),
            reversed.canonical_state_digest()
        );
        assert_eq!(
            forward.canonical_state_digest(),
            interleaved.canonical_state_digest()
        );

        let conflict = forward.conflict_of(&claims[0].key).unwrap();
        assert_eq!(
            conflict.competing_payload_hashes().len(),
            MAX_CONFLICT_EVIDENCE
        );
        assert_eq!(
            conflict.omitted_distinct() as usize,
            claims.len() - MAX_CONFLICT_EVIDENCE
        );
        assert!(!conflict.evidence_truncated());

        // The retained set really is the smallest hashes of the whole
        // claim set, which is what makes it order-independent.
        let mut all: Vec<Digest> = claims
            .iter()
            .map(|c| c.payload.canonical_payload_hash.clone())
            .collect();
        all.sort();
        let expected: BTreeSet<Digest> = all.into_iter().take(MAX_CONFLICT_EVIDENCE).collect();
        assert_eq!(conflict.competing_payload_hashes(), &expected);
    }

    /// Beyond the tracking cap the evidence is still an order-independent
    /// function of the claim set: same retained set, same saturated count,
    /// same truncation flag.
    #[test]
    fn evidence_beyond_the_tracking_cap_is_still_order_independent() {
        let claims: Vec<DueWorkItem> = (0..(MAX_CONFLICT_TRACKED_CLAIMS + 40))
            .map(|i| claim(&format!("payload.{i}")))
            .collect();

        let mut forward = Scheduler::new();
        for c in &claims {
            forward.schedule(c.clone());
        }
        let mut reversed = Scheduler::new();
        for c in claims.iter().rev() {
            reversed.schedule(c.clone());
        }

        assert_eq!(
            forward.canonical_state_digest(),
            reversed.canonical_state_digest()
        );
        let conflict = forward.conflict_of(&claims[0].key).unwrap();
        assert!(conflict.evidence_truncated());
        assert_eq!(
            conflict.omitted_distinct() as usize,
            MAX_CONFLICT_TRACKED_CLAIMS - MAX_CONFLICT_EVIDENCE
        );

        let mut all: Vec<Digest> = claims
            .iter()
            .map(|c| c.payload.canonical_payload_hash.clone())
            .collect();
        all.sort();
        let expected: BTreeSet<Digest> = all.into_iter().take(MAX_CONFLICT_EVIDENCE).collect();
        assert_eq!(conflict.competing_payload_hashes(), &expected);
    }

    /// Two keys poisoned by *different* competing payload sets must not
    /// share a canonical state digest.
    #[test]
    fn different_conflict_evidence_produces_different_state_digests() {
        let mut one = Scheduler::new();
        one.schedule(claim("payload.a"));
        one.schedule(claim("payload.b"));

        let mut two = Scheduler::new();
        two.schedule(claim("payload.c"));
        two.schedule(claim("payload.d"));

        assert_ne!(one.canonical_state_digest(), two.canonical_state_digest());
    }

    /// A conflicted slot must not collide with a scheduled slot.
    #[test]
    fn scheduled_and_conflicted_slots_are_digest_distinct() {
        let mut scheduled = Scheduler::new();
        scheduled.schedule(claim("payload.a"));

        let mut conflicted = Scheduler::new();
        conflicted.schedule(claim("payload.a"));
        conflicted.schedule(claim("payload.b"));

        assert_ne!(
            scheduled.canonical_state_digest(),
            conflicted.canonical_state_digest()
        );
        assert_ne!(
            Scheduler::new().canonical_state_digest(),
            scheduled.canonical_state_digest()
        );
    }

    /// Drain order comes from semantic identity, not call order.
    #[test]
    fn equal_due_time_drains_in_stable_semantic_order() {
        let mut sched = Scheduler::new();
        sched.schedule(item(5, "trigger.c", "bron", 0));
        sched.schedule(item(5, "trigger.a", "bron", 0));
        sched.schedule(item(5, "trigger.b", "bron", 0));

        let outcome = sched.drain_due(LogicalTime(5));
        let ids: Vec<&str> = outcome
            .due
            .iter()
            .map(|w| w.key.producer_definition_id.as_str())
            .collect();
        assert_eq!(ids, vec!["trigger.a", "trigger.b", "trigger.c"]);
        assert!(outcome.conflicted.is_empty());
    }

    /// Occurrence identity is persistence-friendly: the same
    /// producer/scope keeps its own monotone sequence, and the key digest
    /// is a stable identity a saved obligation can be bound to.
    #[test]
    fn occurrence_identity_is_producer_and_scope_local_and_digestible() {
        let a0 = key(5, "trigger.a", "bron", 0, "trigger.evaluate");
        let a1 = key(5, "trigger.a", "bron", 1, "trigger.evaluate");
        let b0 = key(5, "trigger.b", "bron", 0, "trigger.evaluate");

        assert_ne!(a0.identity_digest(), a1.identity_digest());
        assert_ne!(a0.identity_digest(), b0.identity_digest());
        assert_eq!(
            a0.identity_digest(),
            key(5, "trigger.a", "bron", 0, "trigger.evaluate").identity_digest()
        );
    }

    #[test]
    fn occurrence_index_overflow_rejects() {
        assert_eq!(OccurrenceIndex(u64::MAX).checked_next(), None);
        assert_eq!(OccurrenceIndex(0).checked_next(), Some(OccurrenceIndex(1)));
    }

    #[test]
    fn drain_only_returns_due_items_and_leaves_the_rest_scheduled() {
        let mut sched = Scheduler::new();
        sched.schedule(item(10, "trigger.later", "bron", 0));
        sched.schedule(item(1, "trigger.now", "bron", 0));

        let outcome = sched.drain_due(LogicalTime(1));
        assert_eq!(outcome.due.len(), 1);
        assert_eq!(
            outcome.due[0].key.producer_definition_id.as_str(),
            "trigger.now"
        );
        assert_eq!(sched.len(), 1);
    }

    /// A conflicted key that is not yet due stays in the queue.
    #[test]
    fn conflicted_key_is_only_reported_once_due() {
        let mut sched = Scheduler::new();
        sched.schedule(claim("payload.a"));
        sched.schedule(claim("payload.b"));

        let early = sched.drain_due(LogicalTime(4));
        assert!(early.due.is_empty() && early.conflicted.is_empty());
        assert_eq!(sched.len(), 1);

        let due = sched.drain_due(LogicalTime(5));
        assert_eq!(due.conflicted.len(), 1);
        assert!(sched.is_empty());
    }
}
