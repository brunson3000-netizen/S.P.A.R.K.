//! Due-work scheduler skeleton.
//!
//! S.P.A.R.K. has a deterministic scheduler rather than a universal
//! polling loop: the host advances the logical clock and S.P.A.R.K.
//! evaluates only work that is due (`CONTROLLING_BLUEPRINT_v0.2.md`
//! §18.1). Determinism constitution item 2 requires a stable total
//! ordering for equal-time work (§28). This module gives every scheduled
//! item a monotonically assigned `occurrence_index` at schedule time and
//! uses `(due_time, occurrence_index)` as the sort key, so two items due
//! at the same logical time always drain in the order they were
//! scheduled - never in `BTreeMap`/`HashMap` iteration order, thread
//! interleaving, or arrival order.

use crate::clock::LogicalTime;
use crate::hash::CanonicalEncoder;
use crate::id::DefinitionId;
use crate::scope::ScopeId;
use std::collections::BTreeSet;

/// Identifies one scheduled occurrence for replay-stable obligation
/// identity (`CONTROLLING_BLUEPRINT_v0.2.md` §28 item 2: "scheduled
/// occurrence identity derives from persisted logical occurrence
/// indexes, never batching/worker/catch-up chunks").
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OccurrenceIndex(pub u64);

/// One unit of due work: the rule/trigger that created it and the scope
/// it applies to. Phase 1 does not evaluate work; it only proves stable
/// scheduling order. Rule/effect semantics are Phase 2 scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DueWorkItem {
    pub due_time: LogicalTime,
    pub occurrence_index: OccurrenceIndex,
    pub creator_id: DefinitionId,
    pub scope_id: ScopeId,
}

impl DueWorkItem {
    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.due_time.canonicalize(enc);
        enc.push_u64(self.occurrence_index.0);
        self.creator_id.canonicalize(enc);
        self.scope_id.canonicalize(enc);
    }
}

/// Total order key used to keep due-work draining deterministic
/// regardless of insertion structure: ties on `due_time` break on
/// `occurrence_index`, which is assigned monotonically at schedule time
/// and therefore reflects schedule order, not arrival/iteration order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct OrderKey(LogicalTime, OccurrenceIndex);

/// A deterministic due-work queue with stable total ordering for
/// equal-time items.
#[derive(Debug, Default)]
pub struct Scheduler {
    next_occurrence_index: u64,
    items: BTreeSet<OrderKeyedItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderKeyedItem {
    order: OrderKey,
    item: DueWorkItem,
}

impl PartialOrd for OrderKeyedItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderKeyedItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Schedules one work item, assigning it the next monotonic
    /// occurrence index. Returns the assigned index so callers can bind
    /// it into a delayed-obligation identity.
    pub fn schedule(
        &mut self,
        due_time: LogicalTime,
        creator_id: DefinitionId,
        scope_id: ScopeId,
    ) -> OccurrenceIndex {
        let occurrence_index = OccurrenceIndex(self.next_occurrence_index);
        self.next_occurrence_index += 1;
        let item = DueWorkItem {
            due_time,
            occurrence_index,
            creator_id,
            scope_id,
        };
        self.items.insert(OrderKeyedItem {
            order: OrderKey(due_time, occurrence_index),
            item,
        });
        occurrence_index
    }

    /// Removes and returns every item with `due_time <= now`, in stable
    /// ascending `(due_time, occurrence_index)` order.
    pub fn drain_due(&mut self, now: LogicalTime) -> Vec<DueWorkItem> {
        let due: Vec<OrderKeyedItem> = self
            .items
            .iter()
            .filter(|k| k.order.0 <= now)
            .cloned()
            .collect();
        for k in &due {
            self.items.remove(k);
        }
        due.into_iter().map(|k| k.item).collect()
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

    #[test]
    fn equal_due_time_drains_in_schedule_order() {
        let mut sched = Scheduler::new();
        sched.schedule(LogicalTime(5), def("trigger.a"), scope("x"));
        sched.schedule(LogicalTime(5), def("trigger.b"), scope("x"));
        sched.schedule(LogicalTime(5), def("trigger.c"), scope("x"));

        let due = sched.drain_due(LogicalTime(5));
        let ids: Vec<&str> = due.iter().map(|w| w.creator_id.as_str()).collect();
        assert_eq!(ids, vec!["trigger.a", "trigger.b", "trigger.c"]);
    }

    #[test]
    fn drain_only_returns_due_items_and_leaves_the_rest_scheduled() {
        let mut sched = Scheduler::new();
        sched.schedule(LogicalTime(10), def("trigger.later"), scope("x"));
        sched.schedule(LogicalTime(1), def("trigger.now"), scope("x"));

        let due = sched.drain_due(LogicalTime(1));
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].creator_id.as_str(), "trigger.now");
        assert_eq!(sched.len(), 1);
    }
}
