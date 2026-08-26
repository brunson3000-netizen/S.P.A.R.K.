//! The one bounded, order-independent claim set behind every contested
//! canonical slot.
//!
//! Two machines in this kernel refuse to let arrival order pick a winner
//! and retain evidence of the contest instead: [`crate::scheduler`]'s
//! conflicted work keys and [`crate::timeline`]'s poisoned staging slots.
//! Both need exactly the same object — a set of competing content hashes
//! that is
//!
//! - **idempotent and commutative** under insertion, so the value is a
//!   function of the claim set and never of arrival order;
//! - **bounded**, so a contested slot's memory is finite; and
//! - **honest about what it forgot**, so the exact count of distinct
//!   claims outside the exposed window is reportable up to a tracking cap,
//!   and flagged as a lower bound beyond it.
//!
//! Until this unification the two machines carried line-for-line
//! identical private implementations of that object, kept from drifting
//! apart by doc comments on both sides pointing at each other. That is a
//! convention standing where a type belongs — the same class of defect
//! the Phase-1 re-foundation removed elsewhere — and it was about to
//! matter, because a third consumer of contested state would have had to
//! copy it a third time. There is now one implementation and one
//! canonical encoding.
//!
//! The caps stay with the consumers as const parameters rather than being
//! fixed here: they are policy about how much a scheduler slot or a
//! timeline slot may remember, and the two are free to diverge without
//! either implementation forking. The public wrapper types
//! ([`crate::timeline::SlotPoisonEvidence`],
//! [`crate::scheduler::WorkKeyConflict`]) and their accessors, constants,
//! and canonical encodings are unchanged by the unification.

use crate::hash::{CanonicalEncoder, Digest};
use std::collections::BTreeSet;

/// A bounded set of competing claim hashes.
///
/// `EXPOSED` is how many hashes the owning type *presents*; `TRACKED` is
/// how many distinct claims it *remembers* in order to report an exact
/// omitted count. Both projections are smallest-first functions of the
/// claim set, so all of them — retained set, omitted count, truncation
/// flag — remain arrival-order independent past either cap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BoundedClaimSet<const EXPOSED: usize, const TRACKED: usize> {
    tracked: BTreeSet<Digest>,
    truncated: bool,
}

impl<const EXPOSED: usize, const TRACKED: usize> BoundedClaimSet<EXPOSED, TRACKED> {
    /// The two claims that made a slot contested in the first place.
    pub(crate) fn from_pair(a: Digest, b: Digest) -> Self {
        let mut claims = BoundedClaimSet {
            tracked: BTreeSet::new(),
            truncated: false,
        };
        claims.insert(a);
        claims.insert(b);
        claims
    }

    /// Inserts one claim, retaining the numerically smallest hashes. This
    /// is idempotent and commutative, which is exactly what makes the
    /// resulting state a function of the claim set rather than of arrival
    /// order.
    pub(crate) fn insert(&mut self, hash: Digest) {
        if self.tracked.contains(&hash) {
            return;
        }
        if self.tracked.len() < TRACKED {
            self.tracked.insert(hash);
            return;
        }
        // The tracking set is full. Keep the smallest hashes: if this
        // claim is smaller than the current largest tracked hash it
        // displaces it, otherwise it is itself dropped. Either way one
        // distinct claim is now untracked.
        self.truncated = true;
        let largest = match self.tracked.iter().next_back() {
            Some(largest) => largest.clone(),
            // Unreachable: the branch above proved the set is at its
            // non-zero cap. Reported rather than asserted so this
            // function has no panic path.
            None => return,
        };
        if hash < largest {
            self.tracked.remove(&largest);
            self.tracked.insert(hash);
        }
    }

    /// The exposed projection: the `EXPOSED` smallest tracked hashes.
    pub(crate) fn retained(&self) -> BTreeSet<Digest> {
        self.tracked.iter().take(EXPOSED).cloned().collect()
    }

    /// How many further distinct claims were tracked without being
    /// exposed.
    pub(crate) fn omitted_distinct(&self) -> u64 {
        self.tracked.len().saturating_sub(EXPOSED) as u64
    }

    /// Whether the claim set exceeded `TRACKED`, in which case
    /// [`omitted_distinct`](Self::omitted_distinct) is a saturated lower
    /// bound rather than an exact count.
    pub(crate) fn truncated(&self) -> bool {
        self.truncated
    }

    /// Commits to **every** tracked claim, not to the `EXPOSED`
    /// presentation projection.
    ///
    /// This distinction is the whole point of the type. The exposed
    /// projection — the smallest `EXPOSED` hashes plus a count and a flag
    /// — is a *presentation*, and hashing that presentation is what made
    /// two genuinely different states collide: sets sharing their smallest
    /// `EXPOSED` hashes but differing in tracked claims beyond them
    /// produced one digest, and then reacted differently to the same next
    /// claim, because those hidden claims decide whether it is a duplicate
    /// or a new contestant. Canonical state identity must commit to every
    /// retained value that can change future canonical behavior, so it
    /// commits to `tracked` in full.
    ///
    /// The exposed count and omitted count are deliberately *not* pushed:
    /// both are total functions of `tracked`, so pushing them would add no
    /// discrimination. `truncated` is pushed because it is independent
    /// retained state — it records that claims beyond the cap were
    /// forgotten, which no longer follows from `tracked` alone once the
    /// set is back at or below the cap. Claims dropped past the cap are
    /// genuinely not retained and are therefore genuinely collapsed: the
    /// digest commits to what the machine remembers, never to what it
    /// deliberately forgot.
    pub(crate) fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_u64(self.tracked.len() as u64);
        for hash in &self.tracked {
            enc.push_digest(hash);
        }
        enc.push_bool(self.truncated);
    }
}
