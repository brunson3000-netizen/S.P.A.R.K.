//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! **Phase-2 external compile probes** (AT-I32 extended, AT-I2, AT-I6d,
//! AT-I20d, AT-I29 boundary discipline, AT-I46(b)(d), FINAL oracle §6,
//! Revision-2 oracle §10).
//!
//! Same harness shape as `external_compile_probes.rs`: a real external crate,
//! outside the workspace, default features only, a positive control that must
//! compile, and one negative probe per must-not-compose surface, each required
//! to fail for the named symbol. The probes target the concrete engine/host
//! facade; they make **no** claim about `spark-core` privacy — a crate that
//! depends on `spark-core` can call X-1 … X-4 on a `Scheduler` it constructs
//! itself, and no probe claims otherwise.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates directory")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

struct Probe {
    dir: PathBuf,
}

impl Probe {
    fn new() -> Self {
        let root = workspace_root();
        let dir = std::env::temp_dir().join(format!(
            "spark-phase2-external-probe-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("src")).expect("create probe crate directory");
        fs::write(
            dir.join("Cargo.toml"),
            format!(
                r#"[package]
name = "spark-phase2-external-probe"
version = "0.0.0"
edition = "2021"
publish = false

[dependencies]
spark-core = {{ path = "{core}" }}
spark-engine = {{ path = "{engine}" }}

[workspace]
"#,
                core = root.join("crates/spark-core").display(),
                engine = root.join("crates/spark-engine").display(),
            ),
        )
        .expect("write probe manifest");
        let _ = fs::copy(root.join("Cargo.lock"), dir.join("Cargo.lock"));
        Probe { dir }
    }

    fn compile(&self, source: &str) -> (bool, String) {
        fs::write(self.dir.join("src/main.rs"), source).expect("write probe source");
        let output = Command::new("cargo")
            .args(["build", "--offline", "--quiet"])
            .current_dir(&self.dir)
            .env("CARGO_TARGET_DIR", self.dir.join("target"))
            .env("CARGO_TERM_COLOR", "never")
            .output()
            .expect("failed to invoke `cargo build` for the compile probe");
        let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
        combined.push_str(&String::from_utf8_lossy(&output.stderr));
        (output.status.success(), combined)
    }
}

impl Drop for Probe {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

/// Shared prelude: builds a real engine through the production doors.
const PRELUDE: &str = r#"
#![allow(unused)]
use spark_core::authority::Authority;
use spark_core::clock::LogicalTime;
use spark_core::id::{CanonicalTag, DefinitionId, ProfileId, SourceId};
use spark_core::scope::{ScopeId, ScopeKind};
use spark_core::scheduler::WorkKind;
use spark_core::timeline::TimelineEpoch;
use spark_core::value::ValueConstraint;
use spark_engine::activation::ActivationRegistry;
use spark_engine::engine::{Engine, EngineGenesis, InitialWork, TimelineGenesis};
use spark_engine::profile::config::ConfigRevision;
use spark_engine::profile::definition::{DefinitionKind, DefinitionSpec};
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::profile::text::BoundedText;
use spark_engine::request::Request;
use spark_engine::rules::{DeclaredBudgets, EmitOp, Expr, Input, RuleSetSpec, RuleSpec, ScopeRef, Trigger, Update};
use std::collections::BTreeSet;

fn build() -> Engine {
    let profile = ProfileId::new("game-world").unwrap();
    let spec = DefinitionSpec {
        profile_id: profile.clone(),
        id: DefinitionId::new("state.stress").unwrap(),
        kind: DefinitionKind::StateDefinition,
        domain: None,
        layer: None,
        value_constraint: ValueConstraint::int(-100, 100).unwrap(),
        authority: Authority::SparkOwned,
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
        enabled: true,
        version: 1,
        description: BoundedText::new("probe").unwrap(),
        behavioral_leverage: None,
    };
    let manifest = ProfileManifest::new(profile.clone(), BoundedText::new("1").unwrap(), vec![spec]);
    let mut registry = ActivationRegistry::new();
    let activated = registry.activate(&manifest).unwrap();
    let work = WorkKind::new(CanonicalTag::new("work.eval").unwrap());
    let rule = RuleSpec {
        rule_id: DefinitionId::new("rule.a").unwrap(),
        trigger: Trigger::Work(work.clone()),
        conditions: vec![],
        emits: vec![EmitOp {
            sub_id: CanonicalTag::new("x").unwrap(),
            target: DefinitionId::new("state.stress").unwrap(),
            scope: ScopeRef::Subject,
            update: Update::Add(Expr::Input(Input::Literal(1))),
        }],
        schedules: vec![],
        cooldown: None,
    };
    let budgets = DeclaredBudgets {
        max_due_per_cycle: 1, max_cohort_candidates: 10, max_effects_per_wave: 10,
        max_wave_depth: 2, max_enqueue_per_wave: 10, max_obligations: 10,
        max_scheduled_work: 10, max_fan_out: 4,
    };
    let rule_set = registry
        .activate_rule_set(&activated, &RuleSetSpec { profile_id: profile.clone(), budgets, rules: vec![rule] })
        .unwrap();
    let config = ConfigRevision::build(profile, BoundedText::new("c").unwrap(), vec![]).unwrap();
    Engine::genesis(EngineGenesis {
        profile: activated,
        rule_set,
        config,
        timeline: TimelineGenesis {
            timeline_epoch: TimelineEpoch(0),
            sequencer: SourceId::new("sequencer").unwrap(),
            window_width: 2,
        },
        start_time: LogicalTime(0),
        initial_work: vec![InitialWork {
            rule_id: DefinitionId::new("rule.a").unwrap(),
            scope: ScopeId::new(ScopeKind::Actor, "bron").unwrap(),
            due_time: LogicalTime(5),
            work_kind: work,
        }],
    })
    .unwrap()
}
"#;

fn with_prelude(body: &str) -> String {
    format!("{PRELUDE}\nfn main() {{\n{body}\n}}\n")
}

/// The positive control: the legitimate host surface compiles.
const POSITIVE_BODY: &str = r#"
    let mut engine = build();
    let result = engine.process(&Request::Advance(LogicalTime(5)));
    let _ = result.terminates(&Request::Advance(LogicalTime(5)).discriminator());
    let _ = (engine.frontier(), engine.active_request(), engine.stable_boundary_digest());
    let _ = (engine.engine_state_digest(), engine.scheduler_digest(), engine.state().canonical_state_digest());
    let snapshot = engine.snapshot().unwrap();
    let _ = snapshot.recorded_stable_boundary_digest();
    let _ = engine.reset_timeline_epoch(TimelineEpoch(1), SourceId::new("sequencer").unwrap());
"#;

struct Forbidden {
    name: &'static str,
    body: &'static str,
    expected_fragment: &'static str,
}

fn forbidden() -> Vec<Forbidden> {
    vec![
        Forbidden { name: "AT-I32 no &Scheduler from the engine", body: "let e = build(); let _ = &e.scheduler;", expected_fragment: "scheduler" },
        Forbidden { name: "AT-I32 no &mut Scheduler from the engine", body: "let mut e = build(); let _ = &mut e.scheduler;", expected_fragment: "scheduler" },
        Forbidden { name: "Rev2 §10 no &mut TimelineIngress", body: "let mut e = build(); let _ = &mut e.timeline;", expected_fragment: "timeline" },
        Forbidden { name: "Rev2 §10 no public staging on the engine timeline", body: "let mut e = build(); let _ = e.timeline.frontier_ordinal();", expected_fragment: "timeline" },
        Forbidden { name: "AT-I32 F is not host-settable", body: "let mut e = build(); e.frontier = LogicalTime(0);", expected_fragment: "frontier" },
        Forbidden { name: "AT-I32 ActiveRequest is not host-settable", body: "let mut e = build(); e.active = None;", expected_fragment: "active" },
        Forbidden { name: "AT-I32 no ActiveRequest abandonment path", body: "let mut e = build(); e.abandon_active_request();", expected_fragment: "abandon_active_request" },
        Forbidden { name: "AT-I32 no cancellation path", body: "let mut e = build(); e.cancel_request();", expected_fragment: "cancel_request" },
        Forbidden { name: "AT-I32 no arbitrary removal by WorkKey", body: "let mut e = build(); e.remove_work(todo!());", expected_fragment: "remove_work" },
        Forbidden { name: "AT-I32 RequestDiscriminator cannot be forged", body: "let _ = spark_engine::request::RequestDiscriminator { kind: spark_engine::request::RequestKind::Advance, horizon: LogicalTime(0), identity: spark_core::hash::Digest::ZERO };", expected_fragment: "RequestDiscriminator" },
        Forbidden { name: "AT-I32 ProcessResult cannot be forged (dequeue eligibility)", body: "let _ = spark_engine::request::ProcessResult { presented: todo!(), outcome: todo!(), reports: vec![], diagnostics: None };", expected_fragment: "ProcessResult" },
        Forbidden { name: "Rev2 §10 finalization is reachable only through process", body: "let mut e = build(); let _ = e.finalize_command(todo!());", expected_fragment: "finalize_command" },
        Forbidden { name: "AT-I32 the cross-store extraction is engine-internal", body: "let mut e = build(); let _ = e.extract_least_due_slice(LogicalTime(5));", expected_fragment: "extract_least_due_slice" },
        Forbidden { name: "AT-I29 mid-cohort inter-wave points are unreachable", body: "let mut e = build(); let _ = e.run_waves(todo!(), LogicalTime(0), vec![]);", expected_fragment: "run_waves" },
        Forbidden { name: "AT-I32 ActivatedRuleSet cannot be forged", body: "let _ = spark_engine::rules::ActivatedRuleSet { content_hash: spark_core::hash::Digest::ZERO };", expected_fragment: "ActivatedRuleSet" },
        Forbidden { name: "AT-I32 epoch records are engine-minted", body: "let _ = spark_engine::epoch::EpochRecord::new(todo!(), 1, todo!(), todo!(), todo!(), todo!(), todo!(), todo!());", expected_fragment: "new" },
        Forbidden { name: "AT-I32 obligation records cannot be forged", body: "let _ = spark_engine::obligation::ObligationRecord { key: todo!(), creator_rule_id: todo!(), creator_behavior_epoch: 1, creator_behavior_artifact_hash: todo!(), creator_emission_identity: todo!(), mode: todo!() };", expected_fragment: "ObligationRecord" },
        Forbidden { name: "AT-I32 no ledger forgery", body: "let mut l = spark_engine::ledger::OccurrenceLedger::default(); l.set_next(todo!(), 0);", expected_fragment: "set_next" },
        Forbidden { name: "AT-I6d parent sets are not constructible outside the evaluator", body: "let _ = spark_engine::effects::ParentContext::Emissions(Default::default());", expected_fragment: "ParentContext" },
        Forbidden { name: "AT-I32 emission identities cannot be forged", body: "let _ = spark_engine::effects::emission_identity;", expected_fragment: "emission_identity" },
        Forbidden { name: "AT-I20d the evaluation view is not reachable (no pacing/frontier read)", body: "let _: Option<spark_engine::engine::EvalView<'static>> = None;", expected_fragment: "EvalView" },
        Forbidden { name: "AT-I46(b) the observation seam is absent without test-support", body: "let e = build(); let _ = spark_engine::fixture::observation(&e);", expected_fragment: "fixture" },
        Forbidden { name: "AT-I46(d) observed digests are absent without test-support", body: "let _: Option<spark_engine::engine::ObservedDigest> = None;", expected_fragment: "ObservedDigest" },
        Forbidden { name: "AT-I21 the depth-bound seam is absent without test-support", body: "let mut r = ActivationRegistry::new(); let _ = r.activate_rule_set_without_depth_bound(todo!(), todo!());", expected_fragment: "activate_rule_set_without_depth_bound" },
        Forbidden { name: "AT-I32 no evaluator write path on StateStore", body: "let e = build(); let _ = e.state().validate_effect(todo!(), todo!(), todo!(), todo!());", expected_fragment: "validate_effect" },
        Forbidden { name: "AT-I32 snapshots expose no scheduler", body: "let e = build(); let s = e.snapshot().unwrap(); let _ = &s.scheduler;", expected_fragment: "scheduler" },
        Forbidden { name: "AT-I32 the engine cannot be built as a literal", body: "let _ = Engine { fail_stopped: false };", expected_fragment: "Engine" },
    ]
}

#[test]
fn phase2_forbidden_surfaces_do_not_compile_for_an_external_consumer() {
    let probe = Probe::new();
    let (ok, output) = probe.compile(&with_prelude(POSITIVE_BODY));
    assert!(
        ok,
        "the positive control must compile, otherwise every negative probe is vacuous.\n{output}"
    );
    let mut failures = Vec::new();
    for f in forbidden() {
        let (compiled, output) = probe.compile(&with_prelude(f.body));
        if compiled {
            failures.push(format!("[{}] COMPILED, but must not", f.name));
        } else if !output.contains(f.expected_fragment) {
            failures.push(format!(
                "[{}] failed to compile but never mentions '{}':\n{}",
                f.name, f.expected_fragment, output
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "phase-2 compile probes failed:\n\n{}",
        failures.join("\n\n")
    );
}
