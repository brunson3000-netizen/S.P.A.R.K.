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
//! Payload is separated from identity: same key + equal payload is
//! idempotent, same key + different payload is a rejected conflict.
//! Insertion order can never decide a winner.

use crate::clock::LogicalTime;
use crate::hash::{CanonicalEncoder, Digest};
use crate::id::{CanonicalTag, DefinitionId, ProfileId};
use crate::scope::ScopeId;
use std::collections::BTreeMap;
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

/// The logical result of one `schedule` attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleOutcome {
    /// The semantic work key was unoccupied and is now scheduled.
    Scheduled,
    /// The identical item was already scheduled under this exact semantic
    /// key; idempotent, no state change.
    AlreadyScheduledIdempotent,
}

/// Two different payloads claim the same complete semantic work key.
///
/// Insertion order must never decide a winner, so the conflicting
/// submission is rejected and the installed item is left untouched. Note
/// that this can only happen for genuinely identical semantic identity —
/// two independent producers, or two scopes, or two work kinds, never
/// collide, because all of those are part of the key.
///
/// It is returned boxed ([`ScheduleConflict`]) because a complete semantic
/// work key is a large value and the success path is the common one; the
/// error carries the full key deliberately, since reporting *which*
/// identity was ambiguous is the whole point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictingWorkPayload {
    pub key: WorkKey,
    pub installed_payload: WorkPayload,
    pub rejected_payload: WorkPayload,
}

impl fmt::Display for ConflictingWorkPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "work key (t={}, profile={}, producer={}, scope={}, occurrence={}, kind={}) is already scheduled with a different payload",
            self.key.due_time.0,
            self.key.profile_id,
            self.key.producer_definition_id,
            self.key.scope_id,
            self.key.occurrence_index.0,
            self.key.work_kind
        )
    }
}

impl std::error::Error for ConflictingWorkPayload {}

/// The boxed conflict returned by [`Scheduler::schedule`].
pub type ScheduleConflict = Box<ConflictingWorkPayload>;

/// A deterministic due-work queue with a stable total order over complete
/// semantic work identity.
#[derive(Debug, Default, Clone)]
pub struct Scheduler {
    items: BTreeMap<WorkKey, WorkPayload>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Schedules one work item under its complete semantic identity.
    ///
    /// Two calls with an identical item are idempotent regardless of call
    /// order. Two calls that agree on the full semantic key but disagree
    /// on payload are a rejected conflict. Two calls that differ in *any*
    /// key field — including producer, scope, or work kind — are
    /// independent work and both are retained.
    pub fn schedule(&mut self, item: DueWorkItem) -> Result<ScheduleOutcome, ScheduleConflict> {
        match self.items.get(&item.key) {
            None => {
                self.items.insert(item.key, item.payload);
                Ok(ScheduleOutcome::Scheduled)
            }
            Some(installed) if *installed == item.payload => {
                Ok(ScheduleOutcome::AlreadyScheduledIdempotent)
            }
            Some(installed) => Err(Box::new(ConflictingWorkPayload {
                key: item.key,
                installed_payload: installed.clone(),
                rejected_payload: item.payload,
            })),
        }
    }

    /// Removes and returns every item with `due_time <= now`, in stable
    /// ascending [`WorkKey`] order.
    ///
    /// Because `due_time` is the primary component of [`WorkKey`]'s
    /// ordering, the due items are exactly a prefix of the map, so this
    /// walks entries off the front and stops at the first future one. No
    /// key is looked up a second time, so there is no fallible re-lookup
    /// and no panic path.
    pub fn drain_due(&mut self, now: LogicalTime) -> Vec<DueWorkItem> {
        let mut due = Vec::new();
        while let Some(entry) = self.items.first_entry() {
            if entry.key().due_time > now {
                break;
            }
            let (key, payload) = entry.remove_entry();
            due.push(DueWorkItem { key, payload });
        }
        due
    }

    /// Whether the given complete semantic work key is currently
    /// scheduled.
    pub fn contains(&self, key: &WorkKey) -> bool {
        self.items.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// A deterministic digest of every scheduled item in stable key
    /// order, so two schedulers built by different call sequences can be
    /// compared for canonical equality.
    pub fn canonical_state_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("scheduler_state");
        enc.push_u64(self.items.len() as u64);
        for (key, payload) in &self.items {
            let mut inner = CanonicalEncoder::new();
            key.canonicalize(&mut inner);
            payload.canonicalize(&mut inner);
            enc.push_block(&inner);
        }
        enc.finish()
    }
}

#[cfg(test)]
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

    /// Re-foundation scheduler test 1: two producers may both schedule
    /// occurrence 0 at the same time without collision.
    #[test]
    fn independent_producers_may_share_an_occurrence_index() {
        let mut sched = Scheduler::new();
        sched.schedule(item(5, "trigger.a", "bron", 0)).unwrap();
        sched.schedule(item(5, "trigger.b", "bron", 0)).unwrap();
        assert_eq!(sched.len(), 2);
        assert!(sched.contains(&key(5, "trigger.a", "bron", 0, "trigger.evaluate")));
        assert!(sched.contains(&key(5, "trigger.b", "bron", 0, "trigger.evaluate")));
    }

    /// Occurrence identity is scope-local as well as producer-local.
    #[test]
    fn independent_scopes_may_share_an_occurrence_index() {
        let mut sched = Scheduler::new();
        sched.schedule(item(5, "trigger.a", "bron", 0)).unwrap();
        sched.schedule(item(5, "trigger.a", "mira", 0)).unwrap();
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
        sched.schedule(evaluate).unwrap();
        sched.schedule(decay).unwrap();
        assert_eq!(sched.len(), 2);
    }

    /// Re-foundation scheduler test 2: reversed insertion of two
    /// independent items drains identically.
    #[test]
    fn reversed_insertion_of_independent_producers_drains_identically() {
        let mut forward = Scheduler::new();
        forward.schedule(item(5, "trigger.a", "bron", 0)).unwrap();
        forward.schedule(item(5, "trigger.b", "bron", 0)).unwrap();

        let mut reversed = Scheduler::new();
        reversed.schedule(item(5, "trigger.b", "bron", 0)).unwrap();
        reversed.schedule(item(5, "trigger.a", "bron", 0)).unwrap();

        assert_eq!(
            forward.canonical_state_digest(),
            reversed.canonical_state_digest()
        );
        assert_eq!(
            forward.drain_due(LogicalTime(5)),
            reversed.drain_due(LogicalTime(5))
        );
    }

    /// Re-foundation scheduler test 3: same semantic key + same payload is
    /// idempotent.
    #[test]
    fn exact_duplicate_schedule_is_idempotent() {
        let mut sched = Scheduler::new();
        assert_eq!(
            sched.schedule(item(5, "trigger.a", "bron", 0)).unwrap(),
            ScheduleOutcome::Scheduled
        );
        assert_eq!(
            sched.schedule(item(5, "trigger.a", "bron", 0)).unwrap(),
            ScheduleOutcome::AlreadyScheduledIdempotent
        );
        assert_eq!(sched.len(), 1);
    }

    /// Re-foundation scheduler test 4: same semantic key + different
    /// payload is a conflict, and the installed item is untouched.
    #[test]
    fn same_key_with_different_payload_conflicts() {
        let mut sched = Scheduler::new();
        let original = item(5, "trigger.a", "bron", 0);
        sched.schedule(original.clone()).unwrap();

        let mut conflicting = original.clone();
        conflicting.payload = WorkPayload::new(hash_bytes(b"different"));
        let err = sched.schedule(conflicting.clone()).unwrap_err();
        assert_eq!(err.key, original.key);
        assert_eq!(err.installed_payload, original.payload);
        assert_eq!(err.rejected_payload, conflicting.payload);

        assert_eq!(sched.len(), 1);
        let due = sched.drain_due(LogicalTime(5));
        assert_eq!(due[0], original);
    }

    /// The conflict must resolve the same way in either arrival order:
    /// both orders reject, and neither lets first arrival pick a winner
    /// between two *independent* items (there are none here — this is a
    /// genuinely ambiguous single identity).
    #[test]
    fn payload_conflict_rejects_in_either_order() {
        let a = item(5, "trigger.a", "bron", 0);
        let mut b = a.clone();
        b.payload = WorkPayload::new(hash_bytes(b"different"));

        let mut forward = Scheduler::new();
        forward.schedule(a.clone()).unwrap();
        assert!(forward.schedule(b.clone()).is_err());

        let mut reversed = Scheduler::new();
        reversed.schedule(b).unwrap();
        assert!(reversed.schedule(a).is_err());

        assert_eq!(forward.len(), 1);
        assert_eq!(reversed.len(), 1);
    }

    /// Re-foundation scheduler test 5: drain order comes from semantic
    /// identity, not call order.
    #[test]
    fn equal_due_time_drains_in_stable_semantic_order() {
        let mut sched = Scheduler::new();
        sched.schedule(item(5, "trigger.c", "bron", 0)).unwrap();
        sched.schedule(item(5, "trigger.a", "bron", 0)).unwrap();
        sched.schedule(item(5, "trigger.b", "bron", 0)).unwrap();

        let due = sched.drain_due(LogicalTime(5));
        let ids: Vec<&str> = due
            .iter()
            .map(|w| w.key.producer_definition_id.as_str())
            .collect();
        assert_eq!(ids, vec!["trigger.a", "trigger.b", "trigger.c"]);
    }

    /// Re-foundation scheduler test 6: occurrence identity is
    /// persistence-friendly — the same producer/scope keeps its own
    /// monotone sequence, and the key digest is a stable identity a saved
    /// obligation can be bound to.
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
        sched
            .schedule(item(10, "trigger.later", "bron", 0))
            .unwrap();
        sched.schedule(item(1, "trigger.now", "bron", 0)).unwrap();

        let due = sched.drain_due(LogicalTime(1));
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].key.producer_definition_id.as_str(), "trigger.now");
        assert_eq!(sched.len(), 1);
    }

    /// Independent profiles keep independent occurrence domains, so an
    /// MCI profile cannot displace a game profile's scheduled work.
    #[test]
    fn independent_profiles_may_share_an_occurrence_index() {
        let mut sched = Scheduler::new();
        let a = item(5, "trigger.a", "bron", 0);
        let mut b = a.clone();
        b.key.profile_id = ProfileId::new("mci-social").unwrap();
        sched.schedule(a).unwrap();
        sched.schedule(b).unwrap();
        assert_eq!(sched.len(), 2);
    }
}
