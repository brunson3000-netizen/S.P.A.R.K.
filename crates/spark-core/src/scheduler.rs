//! Due-work scheduler skeleton.
//!
//! S.P.A.R.K. has a deterministic scheduler rather than a universal
//! polling loop: the host advances the logical clock and S.P.A.R.K.
//! evaluates only work that is due (`CONTROLLING_BLUEPRINT_v0.2.md`
//! §18.1). The Phase-1 correction brief (B-03) requires that canonical
//! occurrence identity never be manufactured from call/insertion order:
//! `occurrence_index` must be an explicit, persistable input the producer
//! supplies (e.g. recovered from a prior save), not an auto-increment
//! `schedule()` assigns at the moment it happens to be called. This module
//! therefore keys every due-work slot by the caller-supplied
//! `(due_time, occurrence_index)` pair and gives it timeline-style
//! idempotent-duplicate/conflicting-duplicate semantics: submitting the
//! identical item twice is a no-op, but two different items claiming the
//! same slot is a rejected conflict — insertion order can never decide
//! which one "wins".

use crate::clock::LogicalTime;
use crate::hash::CanonicalEncoder;
use crate::id::{DefinitionId, ProfileId};
use crate::scope::ScopeId;
use std::collections::BTreeMap;

/// Identifies one scheduled occurrence for replay-stable obligation
/// identity (`CONTROLLING_BLUEPRINT_v0.2.md` §28 item 2: "scheduled
/// occurrence identity derives from persisted logical occurrence
/// indexes, never batching/worker/catch-up chunks"). The producer (a
/// trigger/rule evaluator, or a recurrence resumer reading a prior save)
/// is responsible for supplying a unique, persistable value — the
/// scheduler itself never manufactures one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OccurrenceIndex(pub u64);

impl OccurrenceIndex {
    /// Rejects overflow when a caller derives the next occurrence index in
    /// a persisted sequence (e.g. resuming recurrence after a save),
    /// rather than silently wrapping (Phase-1 correction brief M-03).
    pub fn checked_next(&self) -> Option<OccurrenceIndex> {
        self.0.checked_add(1).map(OccurrenceIndex)
    }
}

/// One unit of due work: the rule/trigger that created it, the profile and
/// scope it applies to, and a domain-separated `work_kind` discriminator.
/// Phase 1 does not evaluate work; it only proves stable scheduling order.
/// Rule/effect semantics are Phase 2 scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DueWorkItem {
    pub due_time: LogicalTime,
    pub profile_id: ProfileId,
    pub producer_definition_id: DefinitionId,
    pub scope_id: ScopeId,
    pub occurrence_index: OccurrenceIndex,
    pub work_kind: String,
}

impl DueWorkItem {
    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.due_time.canonicalize(enc);
        self.profile_id.canonicalize(enc);
        self.producer_definition_id.canonicalize(enc);
        self.scope_id.canonicalize(enc);
        enc.push_u64(self.occurrence_index.0);
        enc.push_str(&self.work_kind);
    }
}

/// The logical result of one `schedule` attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleOutcome {
    /// The `(due_time, occurrence_index)` slot was empty and is now
    /// occupied by this item.
    Scheduled,
    /// An identical item was already scheduled for this slot; idempotent,
    /// no state change.
    AlreadyScheduledIdempotent,
}

/// A conflicting item already occupies this `(due_time, occurrence_index)`
/// slot; insertion order cannot decide a winner, so both submissions are
/// rejected until the producer supplies a distinct occurrence index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictingDuplicateWorkKey {
    pub due_time: LogicalTime,
    pub occurrence_index: OccurrenceIndex,
}

/// A deterministic due-work queue with stable total ordering for
/// equal-time items, keyed by the caller-supplied
/// `(due_time, occurrence_index)` pair rather than schedule-call order.
#[derive(Debug, Default)]
pub struct Scheduler {
    items: BTreeMap<(LogicalTime, OccurrenceIndex), DueWorkItem>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Schedules one work item under its own `(due_time, occurrence_index)`
    /// identity. Two calls with a byte-identical item are idempotent
    /// regardless of call order; two calls that disagree on any other
    /// field while claiming the same `(due_time, occurrence_index)` are
    /// rejected as a conflict.
    pub fn schedule(
        &mut self,
        item: DueWorkItem,
    ) -> Result<ScheduleOutcome, ConflictingDuplicateWorkKey> {
        let key = (item.due_time, item.occurrence_index);
        match self.items.get(&key) {
            None => {
                self.items.insert(key, item);
                Ok(ScheduleOutcome::Scheduled)
            }
            Some(existing) if *existing == item => Ok(ScheduleOutcome::AlreadyScheduledIdempotent),
            Some(_) => Err(ConflictingDuplicateWorkKey {
                due_time: item.due_time,
                occurrence_index: item.occurrence_index,
            }),
        }
    }

    /// Removes and returns every item with `due_time <= now`, in stable
    /// ascending `(due_time, occurrence_index)` order.
    pub fn drain_due(&mut self, now: LogicalTime) -> Vec<DueWorkItem> {
        let due_keys: Vec<(LogicalTime, OccurrenceIndex)> = self
            .items
            .range(..)
            .filter(|(k, _)| k.0 <= now)
            .map(|(k, _)| *k)
            .collect();
        due_keys
            .into_iter()
            .map(|k| self.items.remove(&k).expect("key came from this map"))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scope::ScopeKind;

    fn def(name: &str) -> DefinitionId {
        DefinitionId::new(name).unwrap()
    }

    fn scope(name: &str) -> ScopeId {
        ScopeId::new(ScopeKind::Actor, name).unwrap()
    }

    fn profile() -> ProfileId {
        ProfileId::new("game-world").unwrap()
    }

    fn item(due: u64, occurrence: u64, creator: &str) -> DueWorkItem {
        DueWorkItem {
            due_time: LogicalTime(due),
            profile_id: profile(),
            producer_definition_id: def(creator),
            scope_id: scope("x"),
            occurrence_index: OccurrenceIndex(occurrence),
            work_kind: "trigger.evaluate".to_string(),
        }
    }

    #[test]
    fn equal_due_time_drains_in_semantic_occurrence_order() {
        let mut sched = Scheduler::new();
        sched.schedule(item(5, 0, "trigger.a")).unwrap();
        sched.schedule(item(5, 1, "trigger.b")).unwrap();
        sched.schedule(item(5, 2, "trigger.c")).unwrap();

        let due = sched.drain_due(LogicalTime(5));
        let ids: Vec<&str> = due
            .iter()
            .map(|w| w.producer_definition_id.as_str())
            .collect();
        assert_eq!(ids, vec!["trigger.a", "trigger.b", "trigger.c"]);
    }

    /// Item: equal-time work inserted A/B versus B/A drains identically —
    /// canonical order comes from the caller-supplied occurrence index,
    /// never from which `schedule()` call happened first.
    #[test]
    fn reversed_schedule_call_order_drains_identically() {
        let mut ab = Scheduler::new();
        ab.schedule(item(5, 0, "trigger.a")).unwrap();
        ab.schedule(item(5, 1, "trigger.b")).unwrap();

        let mut ba = Scheduler::new();
        ba.schedule(item(5, 1, "trigger.b")).unwrap();
        ba.schedule(item(5, 0, "trigger.a")).unwrap();

        assert_eq!(ab.drain_due(LogicalTime(5)), ba.drain_due(LogicalTime(5)));
    }

    #[test]
    fn exact_duplicate_schedule_is_idempotent() {
        let mut sched = Scheduler::new();
        assert_eq!(
            sched.schedule(item(5, 0, "trigger.a")).unwrap(),
            ScheduleOutcome::Scheduled
        );
        assert_eq!(
            sched.schedule(item(5, 0, "trigger.a")).unwrap(),
            ScheduleOutcome::AlreadyScheduledIdempotent
        );
        assert_eq!(sched.len(), 1);
    }

    #[test]
    fn conflicting_duplicate_work_key_rejects() {
        let mut sched = Scheduler::new();
        sched.schedule(item(5, 0, "trigger.a")).unwrap();
        let err = sched.schedule(item(5, 0, "trigger.b")).unwrap_err();
        assert_eq!(
            err,
            ConflictingDuplicateWorkKey {
                due_time: LogicalTime(5),
                occurrence_index: OccurrenceIndex(0),
            }
        );
        // The conflicting item was not installed; the original remains.
        assert_eq!(sched.len(), 1);
        let due = sched.drain_due(LogicalTime(5));
        assert_eq!(due[0].producer_definition_id.as_str(), "trigger.a");
    }

    #[test]
    fn occurrence_index_overflow_rejects() {
        assert_eq!(OccurrenceIndex(u64::MAX).checked_next(), None);
        assert_eq!(OccurrenceIndex(0).checked_next(), Some(OccurrenceIndex(1)));
    }

    #[test]
    fn drain_only_returns_due_items_and_leaves_the_rest_scheduled() {
        let mut sched = Scheduler::new();
        sched.schedule(item(10, 0, "trigger.later")).unwrap();
        sched.schedule(item(1, 0, "trigger.now")).unwrap();

        let due = sched.drain_due(LogicalTime(1));
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].producer_definition_id.as_str(), "trigger.now");
        assert_eq!(sched.len(), 1);
    }
}
