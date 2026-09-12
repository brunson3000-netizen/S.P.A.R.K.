//! A **fake** G.A.M.E. host.
//!
//! **This is preparatory evidence only. It is not proof of G.A.M.E.
//! integration.** The handoff mission is explicit on the point, and so is this
//! module: nothing here is a G.A.M.E. artifact, nothing here was reviewed by
//! G.A.M.E., and a passing test against this host says only that the S.P.A.R.K.
//! side of the contract behaves as the contract says. A real adapter, built in
//! the G.A.M.E. repository against a pinned accepted S.P.A.R.K. artifact, is
//! Gate C4 and is not authorized yet.
//!
//! What the fake host *does* usefully demonstrate is the authority boundary:
//! it, not S.P.A.R.K., decides whether an advised action is legal, it executes
//! the action against its own world numbers, and its confirmed outcome returns
//! only as a typed observation.

use std::collections::BTreeMap;

use spark_core::hash::Digest;
use spark_core::id::DefinitionId;
use spark_core::scope::ScopeId;

use crate::fixture;
use crate::{
    ApplicationDecision, ApplicationLedger, DeferReason, ExternalAffiliationRef, ExternalEntityRef,
    IntentBatch, IntentDisposition, IntentSubject, OutcomeReport, RejectionReason,
};

/// The host's own world truth. S.P.A.R.K. never writes any of this.
#[derive(Debug, Clone)]
pub struct FakeWorld {
    pub wildlife: i64,
    /// Affiliations the host recognises. An intent addressed to an affiliation
    /// the host does not know is an `UnknownSubject` rejection just as an
    /// unknown actor is.
    pub known_affiliations: Vec<ExternalAffiliationRef>,
    /// Entities the host refuses to let act, whatever S.P.A.R.K. advises.
    pub forbidden: Vec<ExternalEntityRef>,
    /// Entities the host defers rather than refuses.
    pub busy: Vec<ExternalEntityRef>,
    pub known: Vec<ExternalEntityRef>,
    pub supported_channels: Vec<DefinitionId>,
    /// Every executed hunt, for at-most-once assertions.
    pub executed_hunts: u32,
    /// The settlement's food supply, and every executed ration order.
    pub food_supply: i64,
    pub executed_rations: u32,
}

impl Default for FakeWorld {
    fn default() -> Self {
        Self {
            wildlife: 100,
            known_affiliations: vec![
                ExternalAffiliationRef("game:settlement/1".to_string()),
                ExternalAffiliationRef("game:settlement/2".to_string()),
                ExternalAffiliationRef("game:region/1".to_string()),
            ],
            forbidden: vec![],
            busy: vec![],
            food_supply: 20,
            executed_rations: 0,
            known: vec![
                ExternalEntityRef("game:actor/1".to_string()),
                ExternalEntityRef("game:actor/2".to_string()),
            ],
            supported_channels: fixture::intent_channels(),
            executed_hunts: 0,
        }
    }
}

/// The host adapter: a world, an at-most-once ledger, and the mapping back to
/// S.P.A.R.K. scopes for the observations it returns.
pub struct FakeHost {
    pub world: FakeWorld,
    ledger: ApplicationLedger,
    affiliation_scope_of: BTreeMap<ExternalAffiliationRef, ScopeId>,
    /// Contract §7.2: the acknowledgment a batch identity earned the first
    /// time. A redelivery replays this record rather than re-deciding against
    /// a world the first application already moved — otherwise the second
    /// acknowledgment would contradict the first.
    reports: BTreeMap<(Digest, Digest, Digest), OutcomeReport>,
}

impl FakeHost {
    pub fn new(world: FakeWorld) -> Self {
        let map = fixture::entity_map();
        let mut affiliation_scope_of = BTreeMap::new();
        for external in ["game:settlement/1", "game:settlement/2", "game:region/1"] {
            let a = ExternalAffiliationRef(external.to_string());
            if let Some(s) = map.affiliation_scope_of(&a) {
                affiliation_scope_of.insert(a, s.clone());
            }
        }
        Self {
            world,
            ledger: ApplicationLedger::default(),
            affiliation_scope_of,
            reports: BTreeMap::new(),
        }
    }

    pub fn deliveries(&self, batch: &IntentBatch) -> usize {
        self.ledger.deliveries(batch)
    }

    /// G.A.M.E. owns identity, so "do I know this subject?" is a host question,
    /// answered separately for entities and affiliations.
    fn knows(&self, subject: &IntentSubject) -> bool {
        match subject {
            IntentSubject::Entity(e) => self.world.known.contains(e),
            IntentSubject::Affiliation(a) => self.world.known_affiliations.contains(a),
        }
    }

    /// Contract §7.2 and §7.3: admit the batch at most once per
    /// `(correlation, batch_digest)`, then apply it **atomically** — the whole
    /// accepted set is staged and committed together, or nothing is.
    pub fn deliver(&mut self, batch: &IntentBatch) -> OutcomeReport {
        let decision = self.ledger.admit(batch);
        let key = ApplicationLedger::key_of(batch);
        if decision == ApplicationDecision::AlreadyApplied {
            if let Some(stored) = self.reports.get(&key) {
                return stored.clone();
            }
        }

        // Decide every intent first. Deciding never mutates the world.
        let mut dispositions = Vec::with_capacity(batch.intents.len());
        let mut staged_wildlife_loss: i64 = 0;
        let mut staged_hunts: u32 = 0;
        let mut staged_rations: u32 = 0;
        for intent in &batch.intents {
            let d = if !self.world.supported_channels.contains(&intent.channel) {
                IntentDisposition::Rejected {
                    reason: RejectionReason::UnsupportedChannel,
                }
            } else if !self.knows(&intent.subject) {
                IntentDisposition::Rejected {
                    reason: RejectionReason::UnknownSubject,
                }
            } else if matches!(&intent.subject, IntentSubject::Entity(e)
                if self.world.forbidden.contains(e))
            {
                // G.A.M.E. alone decides legality. S.P.A.R.K. advised; the
                // host declines, and nothing in the world moves.
                IntentDisposition::Rejected {
                    reason: RejectionReason::IllegalAction,
                }
            } else if matches!(&intent.subject, IntentSubject::Entity(e)
                if self.world.busy.contains(e))
            {
                IntentDisposition::Deferred {
                    reason: DeferReason::WorldBusy,
                    not_before: None,
                }
            } else if intent.channel == fixture::def("intent.ration") {
                // A settlement-level advisory. G.A.M.E. executes it in its own
                // terms: rationing changes the settlement's food supply and
                // touches no wildlife. Different channels are different
                // actions; a host that treated every channel the same would be
                // executing an action nobody advised.
                staged_rations = staged_rations.saturating_add(1);
                let confirmations = match self
                    .affiliation_scope_of
                    .get(&ExternalAffiliationRef("game:settlement/1".to_string()))
                {
                    Some(sc) => vec![(
                        fixture::def("world.food_supply"),
                        sc.clone(),
                        self.world
                            .food_supply
                            .saturating_add(5i64.saturating_mul(i64::from(staged_rations))),
                    )],
                    None => vec![],
                };
                IntentDisposition::Executed { confirmations }
            } else if self.world.wildlife.saturating_sub(staged_wildlife_loss) <= 0 {
                IntentDisposition::Rejected {
                    reason: RejectionReason::InsufficientResource,
                }
            } else {
                staged_wildlife_loss = staged_wildlife_loss.saturating_add(10);
                staged_hunts = staged_hunts.saturating_add(1);
                // The confirmation is host truth about the host's own world,
                // reported at the region scope that owns wildlife.
                let confirmations = match self
                    .affiliation_scope_of
                    .get(&ExternalAffiliationRef("game:region/1".to_string()))
                {
                    Some(r) => vec![(
                        fixture::def("world.wildlife"),
                        r.clone(),
                        self.world.wildlife.saturating_sub(staged_wildlife_loss),
                    )],
                    None => vec![],
                };
                IntentDisposition::Executed { confirmations }
            };
            dispositions.push(d);
        }

        // Contract §7.3: commit atomically, and only on first delivery.
        if decision == ApplicationDecision::Apply {
            self.world.wildlife = self.world.wildlife.saturating_sub(staged_wildlife_loss);
            self.world.executed_hunts = self.world.executed_hunts.saturating_add(staged_hunts);
            self.world.food_supply = self
                .world
                .food_supply
                .saturating_add(5i64.saturating_mul(i64::from(staged_rations)));
            self.world.executed_rations =
                self.world.executed_rations.saturating_add(staged_rations);
        }

        let report = OutcomeReport {
            correlation: batch.correlation.clone(),
            batch_digest: batch.batch_digest.clone(),
            per_intent: dispositions,
        };
        self.reports.entry(key).or_insert_with(|| report.clone());
        report
    }
}
