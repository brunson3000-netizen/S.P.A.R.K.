//! Canonical, digest-committed ledgers (freeze v1 §6, v2 §9): the
//! `OccurrenceLedger` and the `CooldownLedger`. Mutated only in the commit
//! phase of a preflighted wave.

use spark_core::clock::LogicalTime;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{DefinitionId, ProfileId};
use spark_core::scheduler::WorkKind;
use spark_core::scope::ScopeId;
use std::collections::BTreeMap;

/// `(profile, producer definition, scope, work kind)`: one occurrence sequence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OccurrenceLedgerKey {
    pub profile_id: ProfileId,
    pub producer: DefinitionId,
    pub scope_id: ScopeId,
    pub work_kind: WorkKind,
}

impl OccurrenceLedgerKey {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.profile_id.canonicalize(enc);
        self.producer.canonicalize(enc);
        self.scope_id.canonicalize(enc);
        self.work_kind.canonicalize(enc);
    }
}

/// `key -> next OccurrenceIndex`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OccurrenceLedger {
    next: BTreeMap<OccurrenceLedgerKey, u64>,
}

impl OccurrenceLedger {
    pub fn next_for(&self, key: &OccurrenceLedgerKey) -> u64 {
        self.next.get(key).copied().unwrap_or(0)
    }

    pub(crate) fn set_next(&mut self, key: OccurrenceLedgerKey, next: u64) {
        self.next.insert(key, next);
    }

    pub fn len(&self) -> usize {
        self.next.len()
    }

    pub fn is_empty(&self) -> bool {
        self.next.is_empty()
    }

    pub fn canonical_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("occurrence_ledger_v1");
        enc.push_u64(self.next.len() as u64);
        for (k, v) in &self.next {
            let mut inner = CanonicalEncoder::new();
            k.canonicalize(&mut inner);
            inner.push_u64(*v);
            enc.push_block(&inner);
        }
        enc.finish()
    }
}

/// `(rule, scope) -> expiry`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CooldownLedger {
    expiry: BTreeMap<(DefinitionId, ScopeId), LogicalTime>,
}

impl CooldownLedger {
    pub fn expiry_of(&self, rule: &DefinitionId, scope: &ScopeId) -> Option<LogicalTime> {
        self.expiry.get(&(rule.clone(), scope.clone())).copied()
    }

    pub(crate) fn set(&mut self, rule: DefinitionId, scope: ScopeId, expiry: LogicalTime) {
        self.expiry.insert((rule, scope), expiry);
    }

    pub(crate) fn remove(&mut self, rule: &DefinitionId, scope: &ScopeId) {
        self.expiry.remove(&(rule.clone(), scope.clone()));
    }

    pub fn len(&self) -> usize {
        self.expiry.len()
    }

    pub fn is_empty(&self) -> bool {
        self.expiry.is_empty()
    }

    pub fn canonical_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("cooldown_ledger_v1");
        enc.push_u64(self.expiry.len() as u64);
        for ((rule, scope), t) in &self.expiry {
            let mut inner = CanonicalEncoder::new();
            rule.canonicalize(&mut inner);
            scope.canonicalize(&mut inner);
            t.canonicalize(&mut inner);
            enc.push_block(&inner);
        }
        enc.finish()
    }
}
