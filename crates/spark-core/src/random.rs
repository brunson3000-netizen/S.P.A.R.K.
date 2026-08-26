//! Semantic random-address derivation.
//!
//! `CONTROLLING_BLUEPRINT_v0.2.md` §19.1 prohibits one shared mutable RNG
//! stream: independent random addresses must be derived from `root seed +
//! profile/behavior epoch + rule or trigger ID + scope/actor ID + logical
//! occurrence/index`, so that "concurrency, batching, and iteration order
//! must not alter unrelated outcomes." The Phase-1 correction brief (M-04)
//! further requires the address to carry *profile-qualified* behavior
//! context: the same private IDs and epoch number reused across two
//! different profiles (or two different accepted behavior artifacts within
//! one profile) must resolve to different addresses, so this module also
//! hashes an explicit [`ProfileId`] and a `behavior_artifact_hash` — a
//! deterministic digest identifying the active manifest/config behavior
//! context (e.g. `ProfileManifest::manifest_content_hash`) — into every
//! address. This module derives every random value as a pure function of
//! an explicit [`RandomAddress`] — there is no global generator, no
//! counter, and no mutable state to thread through call order, so
//! isolation between unrelated addresses and call-order independence both
//! fall out of the design rather than needing to be separately guarded.

use crate::clock::LogicalTime;
use crate::hash::{CanonicalEncoder, Digest};
use crate::id::{DefinitionId, ProfileId};

// `RandomAddress` needs a scope-or-actor identity; reuse `ScopeId` since
// an actor is itself represented as an `Actor`-kind scope
// (`CONTROLLING_BLUEPRINT_v0.2.md` §13.4).
pub use crate::scope::ScopeId as AddressScope;

/// The complete, explicit identity of one random draw. Two addresses that
/// differ in any field are expected (not merely likely) to be
/// independent, because the derivation hashes every field.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RandomAddress {
    pub root_seed: u64,
    pub profile_id: ProfileId,
    pub behavior_epoch: u64,
    /// A deterministic digest of the active behavior artifact (e.g. the
    /// accepted profile manifest/config content hash) the draw was made
    /// under. Two behavior epochs that happen to share a numeric value in
    /// different manifests/configs must not collide.
    pub behavior_artifact_hash: Digest,
    pub rule_or_trigger_id: DefinitionId,
    pub scope_id: AddressScope,
    pub occurrence_index: u64,
}

impl RandomAddress {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_u64(self.root_seed);
        self.profile_id.canonicalize(enc);
        enc.push_u64(self.behavior_epoch);
        enc.push_digest(&self.behavior_artifact_hash);
        self.rule_or_trigger_id.canonicalize(enc);
        self.scope_id.canonicalize(enc);
        enc.push_u64(self.occurrence_index);
    }
}

/// Stateless deterministic random-address service. It holds no mutable
/// generator state; every method is a pure function of its
/// [`RandomAddress`] argument.
#[derive(Debug, Default, Clone, Copy)]
pub struct RandomAddressService;

impl RandomAddressService {
    pub fn new() -> Self {
        Self
    }

    /// Derives a uniformly distributed `u64` for the given address.
    pub fn derive_u64(&self, address: &RandomAddress) -> u64 {
        let mut enc = CanonicalEncoder::new();
        address.canonicalize(&mut enc);
        let digest = enc.finish();
        u64::from_le_bytes(digest.as_bytes()[0..8].try_into().unwrap())
    }

    /// Derives a fixed-point value in `[0, FIXED_SCALE)`
    /// (see [`crate::value::FIXED_SCALE`]), suitable for probability
    /// gates over canonical fixed-point rates.
    pub fn derive_fixed_fraction(&self, address: &RandomAddress) -> i64 {
        let raw = self.derive_u64(address);
        (raw % crate::value::FIXED_SCALE as u64) as i64
    }
}

/// Helper for callers building a `RandomAddress` at a specific logical
/// time; logical time is not itself part of the address identity (the
/// occurrence index already captures scheduling order deterministically),
/// but this keeps call sites honest that a random draw always happens at
/// some due-work evaluation point rather than ad hoc.
#[allow(clippy::too_many_arguments)]
pub fn address_at(
    root_seed: u64,
    profile_id: ProfileId,
    behavior_epoch: u64,
    behavior_artifact_hash: Digest,
    rule_or_trigger_id: DefinitionId,
    scope_id: AddressScope,
    occurrence_index: u64,
    _at: LogicalTime,
) -> RandomAddress {
    RandomAddress {
        root_seed,
        profile_id,
        behavior_epoch,
        behavior_artifact_hash,
        rule_or_trigger_id,
        scope_id,
        occurrence_index,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::hash_bytes;
    use crate::scope::{ScopeId, ScopeKind};

    fn artifact(tag: &str) -> Digest {
        hash_bytes(tag.as_bytes())
    }

    fn addr(occurrence_index: u64) -> RandomAddress {
        RandomAddress {
            root_seed: 42,
            profile_id: ProfileId::new("game-world").unwrap(),
            behavior_epoch: 1,
            behavior_artifact_hash: artifact("manifest.v1"),
            rule_or_trigger_id: DefinitionId::new("trigger.weather.drought").unwrap(),
            scope_id: ScopeId::new(ScopeKind::Region, "northwood").unwrap(),
            occurrence_index,
        }
    }

    #[test]
    fn same_address_produces_identical_result() {
        let service = RandomAddressService::new();
        let a = addr(7);
        let b = addr(7);
        assert_eq!(service.derive_u64(&a), service.derive_u64(&b));
    }

    #[test]
    fn different_occurrence_index_is_independent() {
        let service = RandomAddressService::new();
        let a = service.derive_u64(&addr(1));
        let b = service.derive_u64(&addr(2));
        assert_ne!(a, b);
    }

    #[test]
    fn call_order_does_not_affect_result_for_unrelated_addresses() {
        let service = RandomAddressService::new();

        // Draw in one order.
        let first_order_a = service.derive_u64(&addr(1));
        let first_order_b = service.derive_u64(&addr(2));

        // Draw the same two addresses in the reverse order.
        let second_order_b = service.derive_u64(&addr(2));
        let second_order_a = service.derive_u64(&addr(1));

        assert_eq!(first_order_a, second_order_a);
        assert_eq!(first_order_b, second_order_b);
    }

    #[test]
    fn fixed_fraction_stays_in_bounds() {
        let service = RandomAddressService::new();
        let f = service.derive_fixed_fraction(&addr(3));
        assert!((0..crate::value::FIXED_SCALE).contains(&f));
    }

    /// M-04: identical private IDs/epoch in two different profiles must
    /// resolve to different addresses.
    #[test]
    fn same_ids_and_epoch_in_different_profiles_are_independent() {
        let service = RandomAddressService::new();
        let mut a = addr(1);
        let mut b = addr(1);
        a.profile_id = ProfileId::new("game-world").unwrap();
        b.profile_id = ProfileId::new("mci-social").unwrap();
        assert_ne!(service.derive_u64(&a), service.derive_u64(&b));
    }

    /// M-04: same profile but a different accepted behavior-artifact hash
    /// (e.g. a different manifest/config revision under the same numeric
    /// epoch) must resolve to a different address.
    #[test]
    fn same_profile_different_behavior_artifact_hash_is_independent() {
        let service = RandomAddressService::new();
        let mut a = addr(1);
        let mut b = addr(1);
        a.behavior_artifact_hash = artifact("manifest.v1");
        b.behavior_artifact_hash = artifact("manifest.v2");
        assert_ne!(service.derive_u64(&a), service.derive_u64(&b));
    }
}
