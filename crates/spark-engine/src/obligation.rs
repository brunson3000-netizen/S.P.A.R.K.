//! Delayed obligations and the canonical `ObligationStore` (freeze v1 §5,
//! v2 §8).
//!
//! An obligation is scheduled through the Phase-1 `Scheduler` under its complete
//! `WorkKey`; its full record lives here, keyed by `WorkKey::identity_digest()`,
//! as a bounded, order-independent claim set of content-addressed records — the
//! same insertion discipline and caps as the scheduler's conflict evidence, so a
//! contested key's record set and the scheduler's tracked claim set agree by
//! construction. The scheduled `WorkPayload.canonical_payload_hash` is the
//! record hash.

use crate::rules::ResultFamily;
use spark_core::clock::LogicalTime;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{CanonicalTag, DefinitionId};
use spark_core::scheduler::{SlotCommitment, WorkKey, WorkPayload, MAX_CONFLICT_TRACKED_CLAIMS};
use spark_core::scope::ScopeId;
use std::collections::{BTreeMap, BTreeSet};

/// An effect resolved at scheduling time and frozen in the record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterializedEffect {
    pub sub_id: CanonicalTag,
    pub target: DefinitionId,
    pub scope: ScopeId,
    pub intent: FrozenIntent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrozenIntent {
    AddDelta(i64),
    Result { value: i64, family: ResultFamily },
}

impl MaterializedEffect {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        self.sub_id.canonicalize(enc);
        self.target.canonicalize(enc);
        self.scope.canonicalize(enc);
        match &self.intent {
            FrozenIntent::AddDelta(d) => {
                enc.push_str("frozen.add_delta");
                enc.push_i64(*d);
            }
            FrozenIntent::Result { value, family } => {
                enc.push_str("frozen.result");
                enc.push_i64(*value);
                enc.push_str(family.tag());
            }
        }
    }
}

/// How an obligation executes. Modes never switch (ADR-0006).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObligationMode {
    /// Apply frozen effects; refuses if **any** target's current definition
    /// fingerprint differs from the recorded set (v2 §8 multi-target binding).
    MaterializedEffect {
        effects: Vec<MaterializedEffect>,
        target_fingerprints: BTreeMap<DefinitionId, Digest>,
    },
    /// Re-evaluate the exact rule, bound by fingerprint and ruleset hash.
    RuleReEvaluation {
        rule_id: DefinitionId,
        rule_fingerprint: Digest,
        ruleset_content_hash: Digest,
    },
}

/// One content-addressed obligation record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObligationRecord {
    pub(crate) key: WorkKey,
    pub(crate) creator_rule_id: DefinitionId,
    /// The creating rule's fingerprint (v1 §5 `creator_definition_fingerprint`).
    /// A materialized record's later emissions use it as their rule-fingerprint
    /// component, so their identity never depends on the frozen payload
    /// (v3 §4.4, D-C2-13).
    pub(crate) creator_rule_fingerprint: Digest,
    pub(crate) creator_behavior_epoch: u64,
    pub(crate) creator_behavior_artifact_hash: Digest,
    /// The creating emission's complete identity (a function of its parent
    /// set at wave `N >= 1`), so two records with coinciding payloads but
    /// different causes stay discriminated (AT-I15 v3).
    pub(crate) creator_emission_identity: Digest,
    pub(crate) mode: ObligationMode,
}

impl ObligationRecord {
    pub fn key(&self) -> &WorkKey {
        &self.key
    }

    pub fn due_time(&self) -> LogicalTime {
        self.key.due_time
    }

    pub fn mode(&self) -> &ObligationMode {
        &self.mode
    }

    pub fn creator_rule_id(&self) -> &DefinitionId {
        &self.creator_rule_id
    }

    pub fn creator_emission_identity(&self) -> &Digest {
        &self.creator_emission_identity
    }

    pub fn creator_rule_fingerprint(&self) -> &Digest {
        &self.creator_rule_fingerprint
    }

    pub fn creator_behavior_artifact_hash(&self) -> &Digest {
        &self.creator_behavior_artifact_hash
    }

    /// The canonical record hash (the scheduled payload hash).
    pub fn record_hash(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("obligation_record_v1");
        self.key.canonicalize(&mut enc);
        self.creator_rule_id.canonicalize(&mut enc);
        enc.push_digest(&self.creator_rule_fingerprint);
        enc.push_u64(self.creator_behavior_epoch);
        enc.push_digest(&self.creator_behavior_artifact_hash);
        enc.push_digest(&self.creator_emission_identity);
        match &self.mode {
            ObligationMode::MaterializedEffect {
                effects,
                target_fingerprints,
            } => {
                enc.push_str("mode.materialized_effect");
                enc.push_u64(effects.len() as u64);
                for e in effects {
                    let mut inner = CanonicalEncoder::new();
                    e.canonicalize(&mut inner);
                    enc.push_block(&inner);
                }
                enc.push_u64(target_fingerprints.len() as u64);
                for (d, fp) in target_fingerprints {
                    d.canonicalize(&mut enc);
                    enc.push_digest(fp);
                }
            }
            ObligationMode::RuleReEvaluation {
                rule_id,
                rule_fingerprint,
                ruleset_content_hash,
            } => {
                enc.push_str("mode.rule_reevaluation");
                rule_id.canonicalize(&mut enc);
                enc.push_digest(rule_fingerprint);
                enc.push_digest(ruleset_content_hash);
            }
        }
        enc.finish()
    }

    pub fn payload(&self) -> WorkPayload {
        WorkPayload::new(self.record_hash())
    }
}

/// The claim set under one work identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimSet {
    key: WorkKey,
    records: BTreeMap<Digest, ObligationRecord>,
    truncated: bool,
}

impl ClaimSet {
    pub fn key(&self) -> &WorkKey {
        &self.key
    }

    pub fn records(&self) -> impl Iterator<Item = &ObligationRecord> {
        self.records.values()
    }

    pub fn is_contested(&self) -> bool {
        self.records.len() > 1 || self.truncated
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    /// The single record of an uncontested claim set.
    pub fn sole_record(&self) -> Option<&ObligationRecord> {
        if self.is_contested() {
            None
        } else {
            self.records.values().next()
        }
    }

    /// The `BoundedClaimSet` insertion discipline, over record hashes.
    fn insert(&mut self, record: ObligationRecord) {
        let hash = record.record_hash();
        if self.records.contains_key(&hash) {
            return;
        }
        if self.records.len() < MAX_CONFLICT_TRACKED_CLAIMS {
            self.records.insert(hash, record);
            return;
        }
        self.truncated = true;
        let largest = match self.records.keys().next_back() {
            Some(l) => l.clone(),
            None => return,
        };
        if hash < largest {
            self.records.remove(&largest);
            self.records.insert(hash, record);
        }
    }

    /// The slot commitment the scheduler must hold for this claim set.
    pub fn expected_commitment(&self) -> SlotCommitment {
        match self.sole_record() {
            Some(r) => SlotCommitment::scheduled(&self.key, &r.payload()),
            None => {
                let tracked: BTreeSet<Digest> = self.records.keys().cloned().collect();
                SlotCommitment::conflicted(&self.key, &tracked, self.truncated)
            }
        }
    }
}

/// The canonical, digest-committed obligation store.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObligationStore {
    claims: BTreeMap<Digest, ClaimSet>,
}

impl ObligationStore {
    pub fn get(&self, key: &WorkKey) -> Option<&ClaimSet> {
        self.claims.get(&key.identity_digest())
    }

    pub fn len(&self) -> usize {
        self.claims.len()
    }

    pub fn is_empty(&self) -> bool {
        self.claims.is_empty()
    }

    /// Commutative, idempotent insert (mirrors `Scheduler::schedule`).
    pub(crate) fn insert(&mut self, record: ObligationRecord) {
        let id = record.key.identity_digest();
        match self.claims.get_mut(&id) {
            Some(set) => set.insert(record),
            None => {
                let mut set = ClaimSet {
                    key: record.key.clone(),
                    records: BTreeMap::new(),
                    truncated: false,
                };
                set.insert(record);
                self.claims.insert(id, set);
            }
        }
    }

    pub(crate) fn remove(&mut self, key: &WorkKey) -> Option<ClaimSet> {
        self.claims.remove(&key.identity_digest())
    }

    /// Every work key the store holds, ascending by identity digest.
    pub fn keys(&self) -> impl Iterator<Item = &WorkKey> {
        self.claims.values().map(|c| &c.key)
    }

    pub fn canonical_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("obligation_store_v1");
        enc.push_u64(self.claims.len() as u64);
        for (id, set) in &self.claims {
            let mut inner = CanonicalEncoder::new();
            inner.push_digest(id);
            set.key.canonicalize(&mut inner);
            inner.push_u64(set.records.len() as u64);
            for h in set.records.keys() {
                inner.push_digest(h);
            }
            inner.push_bool(set.truncated);
            enc.push_block(&inner);
        }
        enc.finish()
    }

    /// Test-support: replace or delete the claim set under `key`, so the
    /// cross-store preflight refusals can be exercised.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn tamper(&mut self, key: &WorkKey, records: Option<Vec<ObligationRecord>>) {
        let id = key.identity_digest();
        self.claims.remove(&id);
        if let Some(records) = records {
            for r in records {
                let mut r = r;
                r.key = key.clone();
                self.insert(r);
            }
        }
    }
}
