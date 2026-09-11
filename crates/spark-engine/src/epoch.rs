//! Behavior epochs (freeze v1 Q5): an explicit, hash-chained epoch record per
//! profile, minted only at its own canonical barrier, and the canonical
//! append-only `EpochRegistry`.
//!
//! `behavior_artifact_hash` — the digest every random address and every delayed
//! obligation carries — is the canonical hash of the epoch record, which binds
//! the manifest, config revision, and rule set as one identity.

use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::ProfileId;
use std::fmt;

/// The barrier that activated an epoch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivatingBarrier {
    /// The engine's genesis state (behavior epoch 1).
    Genesis,
    /// A finalized epoch-activation command (ADR-0003 §14 barrier identity).
    Command {
        timeline_epoch: u64,
        input_ordinal: u64,
        semantic_hash: Digest,
        fence_hash: Digest,
    },
}

impl ActivatingBarrier {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            ActivatingBarrier::Genesis => {
                enc.push_str("barrier.genesis");
            }
            ActivatingBarrier::Command {
                timeline_epoch,
                input_ordinal,
                semantic_hash,
                fence_hash,
            } => {
                enc.push_str("barrier.command");
                enc.push_u64(*timeline_epoch);
                enc.push_u64(*input_ordinal);
                enc.push_digest(semantic_hash);
                enc.push_digest(fence_hash);
            }
        }
    }
}

/// One behavior-epoch record. Private fields; minted only by the engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpochRecord {
    profile_id: ProfileId,
    behavior_epoch: u64,
    manifest_content_hash: Digest,
    config_revision_hash: Digest,
    ruleset_content_hash: Digest,
    activation_hash: Digest,
    previous_epoch_record_hash: Digest,
    activating_barrier: ActivatingBarrier,
}

impl EpochRecord {
    // One argument per frozen record field (v1 Q5); construction is crate-only.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        profile_id: ProfileId,
        behavior_epoch: u64,
        manifest_content_hash: Digest,
        config_revision_hash: Digest,
        ruleset_content_hash: Digest,
        activation_hash: Digest,
        previous_epoch_record_hash: Digest,
        activating_barrier: ActivatingBarrier,
    ) -> Self {
        Self {
            profile_id,
            behavior_epoch,
            manifest_content_hash,
            config_revision_hash,
            ruleset_content_hash,
            activation_hash,
            previous_epoch_record_hash,
            activating_barrier,
        }
    }

    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    pub fn behavior_epoch(&self) -> u64 {
        self.behavior_epoch
    }

    pub fn config_revision_hash(&self) -> &Digest {
        &self.config_revision_hash
    }

    pub fn ruleset_content_hash(&self) -> &Digest {
        &self.ruleset_content_hash
    }

    pub fn manifest_content_hash(&self) -> &Digest {
        &self.manifest_content_hash
    }

    pub fn activating_barrier(&self) -> &ActivatingBarrier {
        &self.activating_barrier
    }

    /// The canonical record hash: this epoch's `behavior_artifact_hash`.
    pub fn record_hash(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("behavior_epoch_record_v1");
        self.profile_id.canonicalize(&mut enc);
        enc.push_u64(self.behavior_epoch);
        enc.push_digest(&self.manifest_content_hash);
        enc.push_digest(&self.config_revision_hash);
        enc.push_digest(&self.ruleset_content_hash);
        enc.push_digest(&self.activation_hash);
        enc.push_digest(&self.previous_epoch_record_hash);
        self.activating_barrier.canonicalize(&mut enc);
        enc.finish()
    }
}

/// Rejects an out-of-chain epoch record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EpochChainError {
    ForeignProfile,
    EpochNotSuccessor { expected: u64, attempted: u64 },
    PreviousHashMismatch,
    EpochSpaceExhausted,
}

impl fmt::Display for EpochChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "epoch record does not extend the chain: {self:?}")
    }
}

impl std::error::Error for EpochChainError {}

/// The canonical, digest-committed, append-only, hash-chained epoch registry
/// of one profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpochRegistry {
    profile_id: ProfileId,
    records: Vec<EpochRecord>,
}

impl EpochRegistry {
    pub(crate) fn new(profile_id: ProfileId) -> Self {
        Self {
            profile_id,
            records: Vec::new(),
        }
    }

    pub fn current(&self) -> Option<&EpochRecord> {
        self.records.last()
    }

    pub fn records(&self) -> &[EpochRecord] {
        &self.records
    }

    /// The next record's epoch number and predecessor hash.
    pub(crate) fn successor(&self) -> Result<(u64, Digest), EpochChainError> {
        match self.records.last() {
            None => Ok((1, Digest::ZERO)),
            Some(r) => r
                .behavior_epoch
                .checked_add(1)
                .map(|n| (n, r.record_hash()))
                .ok_or(EpochChainError::EpochSpaceExhausted),
        }
    }

    /// Validates, without mutating, that `record` extends the chain.
    pub(crate) fn check_append(&self, record: &EpochRecord) -> Result<(), EpochChainError> {
        if record.profile_id != self.profile_id {
            return Err(EpochChainError::ForeignProfile);
        }
        let (expected, previous) = self.successor()?;
        if record.behavior_epoch != expected {
            return Err(EpochChainError::EpochNotSuccessor {
                expected,
                attempted: record.behavior_epoch,
            });
        }
        if record.previous_epoch_record_hash != previous {
            return Err(EpochChainError::PreviousHashMismatch);
        }
        Ok(())
    }

    pub(crate) fn append(&mut self, record: EpochRecord) -> Result<(), EpochChainError> {
        self.check_append(&record)?;
        self.records.push(record);
        Ok(())
    }

    /// Whether every record chains to its predecessor (restore validation).
    pub(crate) fn chain_is_valid(&self) -> bool {
        let mut probe = EpochRegistry::new(self.profile_id.clone());
        for r in &self.records {
            if probe.append(r.clone()).is_err() {
                return false;
            }
        }
        true
    }

    pub fn canonical_digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        enc.push_str("epoch_registry_v1");
        self.profile_id.canonicalize(&mut enc);
        enc.push_u64(self.records.len() as u64);
        for r in &self.records {
            enc.push_digest(&r.record_hash());
        }
        enc.finish()
    }
}
