//! Test targets scope the canonical crates' strict panic/arithmetic gate
//! locally.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
//! **External compile probes: the "must be impossible" proofs.**
//!
//! Every claim of the form "an external caller cannot do X" is proved here
//! by actually compiling a real external consumer crate — one that is not
//! a workspace member and enables **only default features** — and
//! asserting that the compile fails, for the right reason.
//!
//! # Why this exists rather than only `compile_fail` doc-tests
//!
//! Doc-tests are compiled against whatever feature set the crate happens
//! to be built with in the current `cargo` invocation. During
//! `cargo test --workspace`, `spark-testkit`'s dev-dependency edge enables
//! `test-support` on `spark-core` and `spark-engine`, so a doc-test could
//! not honestly prove that a `test-support`-gated symbol is absent from
//! the *production* surface. These probes build a separate crate with a
//! separate target directory and default features, which is exactly the
//! vantage point an integrator has.
//!
//! Independent review noted that the v1 corpus's `compile_fail` doc-tests
//! "validly prove their narrow claims" but "do not prove that equivalent
//! public composition paths are absent" — inability to write an
//! `ActivatedSchema { .. }` literal did not stop a public `activate` from
//! minting one. The probes below therefore target *composition paths*, not
//! just struct literals, and the suite opens with a **positive control**:
//! a probe that must compile. Without it, a typo in the harness would make
//! every negative probe pass vacuously.
//!
//! All probes run inside one `#[test]` so they share one temporary crate
//! and one warm target directory; the first build pays for the
//! dependencies and the rest are incremental.

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

/// A scratch crate outside the workspace that depends on the canonical
/// crates with **default features only**.
struct Probe {
    dir: PathBuf,
}

impl Probe {
    fn new() -> Self {
        let root = workspace_root();
        let dir = std::env::temp_dir().join(format!(
            "spark-external-probe-{}-{}",
            std::process::id(),
            env!("CARGO_PKG_VERSION").replace('.', "_")
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("src")).expect("create probe crate directory");

        let core = root.join("crates/spark-core");
        let engine = root.join("crates/spark-engine");
        fs::write(
            dir.join("Cargo.toml"),
            format!(
                r#"[package]
name = "spark-external-probe"
version = "0.0.0"
edition = "2021"
publish = false

[dependencies]
spark-core = {{ path = "{core}" }}
spark-engine = {{ path = "{engine}" }}

# Standalone: not a member of the S.P.A.R.K. workspace, so it resolves
# features exactly as an outside integrator would.
[workspace]
"#,
                core = core.display(),
                engine = engine.display(),
            ),
        )
        .expect("write probe manifest");

        Probe { dir }
    }

    /// Compiles `source` as the probe crate's `main.rs`, returning
    /// `(succeeded, combined_output)`.
    fn compile(&self, source: &str) -> (bool, String) {
        fs::write(self.dir.join("src/main.rs"), source).expect("write probe source");
        let output = Command::new("cargo")
            .args(["build", "--offline", "--quiet"])
            .current_dir(&self.dir)
            .env("CARGO_TARGET_DIR", self.dir.join("target"))
            // A probe that fails to compile is the *expected* result, so
            // colour codes would only make the assertion messages harder
            // to read.
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

/// The positive control: real, production-path code that an external
/// consumer legitimately writes. If this ever fails to compile, every
/// negative probe below is meaningless and the suite says so loudly.
const POSITIVE_CONTROL: &str = r#"
use spark_core::authority::Authority;
use spark_core::id::{DefinitionId, ProfileId};
use spark_core::scope::ScopeKind;
use spark_core::value::{FixedPoint, ValueConstraint};
use spark_engine::activation::{definition_fingerprint, ActivationRegistry};
use spark_engine::profile::definition::{DefinitionKind, DefinitionSpec};
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::profile::text::BoundedText;
use std::collections::BTreeSet;

fn main() {
    let profile = ProfileId::new("game-world").unwrap();
    let spec = DefinitionSpec {
        profile_id: profile.clone(),
        id: DefinitionId::new("trait.curiosity").unwrap(),
        kind: DefinitionKind::Trait,
        domain: None,
        layer: None,
        value_constraint: ValueConstraint::fixed(
            FixedPoint::ZERO,
            FixedPoint::from_integer(1).unwrap(),
        )
        .unwrap(),
        authority: Authority::Derived,
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
        enabled: true,
        version: 1,
        description: BoundedText::new("valid").unwrap(),
        behavioral_leverage: None,
    };
    let predicted = definition_fingerprint(&spec);
    let manifest = ProfileManifest::new(
        profile,
        BoundedText::new("1.0.0").unwrap(),
        vec![spec],
    );
    let mut registry = ActivationRegistry::new();
    let activated = registry.activate(&manifest).unwrap();
    let store = activated.into_state_store();
    assert_ne!(store.activation_hash(), store.manifest_content_hash());
    let _ = (predicted, registry.lineage_digest(), store.canonical_state_digest());
}
"#;

/// One negative probe: a name, source that must **not** compile, and a
/// fragment the compiler output must contain so the failure is provably
/// about the intended symbol rather than an unrelated typo.
struct Forbidden {
    name: &'static str,
    source: &'static str,
    expected_fragment: &'static str,
}

fn forbidden_probes() -> Vec<Forbidden> {
    vec![
        // ---------------- AT-A1: raw declarations ----------------
        Forbidden {
            name: "AT-A1 raw declaration type is not nameable",
            source: r#"
use spark_engine::activation::DefinitionDeclaration;
fn main() { let _: Option<DefinitionDeclaration> = None; }
"#,
            expected_fragment: "DefinitionDeclaration",
        },
        Forbidden {
            name: "AT-A1 the old kernel activation module is gone",
            source: r#"
fn main() {
    let _ = spark_core::activation::DefinitionIdentityRegistry::new();
}
"#,
            expected_fragment: "activation",
        },
        // ---------------- AT-A3: no self-activation stepping stone ----
        Forbidden {
            name: "AT-A3 DefinitionSpec::to_declaration is gone",
            source: r#"
use spark_core::authority::Authority;
use spark_core::id::{DefinitionId, ProfileId};
use spark_core::scope::ScopeKind;
use spark_core::value::ValueConstraint;
use spark_engine::profile::definition::{DefinitionKind, DefinitionSpec};
use spark_engine::profile::text::BoundedText;
use std::collections::BTreeSet;

fn main() {
    let spec = DefinitionSpec {
        profile_id: ProfileId::new("game-world").unwrap(),
        id: DefinitionId::new("trait.curiosity").unwrap(),
        kind: DefinitionKind::Trait,
        domain: None,
        layer: None,
        value_constraint: ValueConstraint::boolean(),
        authority: Authority::Derived,
        valid_scopes: BTreeSet::from([ScopeKind::Actor]),
        enabled: true,
        version: 1,
        description: BoundedText::new("valid").unwrap(),
        behavioral_leverage: None,
    };
    // The intermediate mint input this used to produce is gone.
    let _ = spec.to_declaration();
}
"#,
            expected_fragment: "to_declaration",
        },
        Forbidden {
            name: "AT-A3 DefinitionKind::kind_tag is crate-private",
            source: r#"
use spark_engine::profile::definition::DefinitionKind;
fn main() { let _ = DefinitionKind::Trait.kind_tag(); }
"#,
            expected_fragment: "kind_tag",
        },
        // ---------------- AT-A2: no store construction ----------------
        Forbidden {
            name: "AT-A2 StateStore has no public constructor",
            source: r#"
use spark_engine::state::StateStore;
fn main() { let _ = StateStore::new(std::iter::empty()); }
"#,
            expected_fragment: "StateStore",
        },
        Forbidden {
            name: "AT-A2 StateStore has no from_activation constructor",
            source: r#"
use spark_engine::state::StateStore;
fn main() { let _ = StateStore::from_activation(todo!()); }
"#,
            expected_fragment: "from_activation",
        },
        Forbidden {
            name: "AT-A2 ActivatedProfile cannot be built as a literal",
            source: r#"
use spark_core::id::ProfileId;
use spark_engine::activation::ActivatedProfile;
fn main() {
    let _ = ActivatedProfile {
        profile_id: ProfileId::new("game-world").unwrap(),
    };
}
"#,
            expected_fragment: "ActivatedProfile",
        },
        Forbidden {
            name: "AT-A2 ActivatedDefinition cannot be built as a literal",
            source: r#"
use spark_core::authority::Authority;
use spark_core::hash::Digest;
use spark_engine::activation::ActivatedDefinition;
fn main() {
    let _ = ActivatedDefinition {
        authority: Authority::Derived,
        fingerprint: Digest::ZERO,
    };
}
"#,
            expected_fragment: "ActivatedDefinition",
        },
        Forbidden {
            name: "AT-A2 the old kernel state module is gone",
            source: r#"
fn main() { let _: Option<spark_core::state::StateStore> = None; }
"#,
            expected_fragment: "state",
        },
        // ---------------- AT-A6: no public write path ----------------
        Forbidden {
            name: "AT-A6 apply_spark_effect is not publicly callable",
            source: r#"
use spark_engine::state::StateStore;
fn write(store: &mut StateStore) {
    let _ = store.apply_spark_effect(todo!(), todo!(), todo!(), todo!(), todo!(), 1);
}
fn main() {}
"#,
            expected_fragment: "apply_spark_effect",
        },
        Forbidden {
            name: "AT-A6 observe_host_owned is not publicly callable",
            source: r#"
use spark_engine::state::StateStore;
fn write(store: &mut StateStore) {
    let _ = store.observe_host_owned(todo!(), todo!(), todo!(), todo!(), todo!(), 1);
}
fn main() {}
"#,
            expected_fragment: "observe_host_owned",
        },
        Forbidden {
            name: "AT-A6 commit_derived is not publicly callable",
            source: r#"
use spark_engine::state::StateStore;
fn write(store: &mut StateStore) {
    let _ = store.commit_derived(todo!(), todo!(), todo!(), todo!(), todo!(), 1);
}
fn main() {}
"#,
            expected_fragment: "commit_derived",
        },
        Forbidden {
            name: "AT-A6/AT-D2 the fixture seam does not exist without test-support",
            source: r#"
fn main() { let _ = spark_engine::fixture::apply_spark_effect; }
"#,
            expected_fragment: "fixture",
        },
        // ---------------- AT-A7: builtin kind bit ----------------
        Forbidden {
            name: "AT-A7 DefinitionKindTag::builtin is not caller-assertable",
            source: r#"
use spark_core::id::CanonicalTag;
use spark_engine::activation::DefinitionKindTag;
fn main() {
    let _ = DefinitionKindTag::builtin(CanonicalTag::new("weather").unwrap());
}
"#,
            expected_fragment: "builtin",
        },
        // ---------------- AT-D1: reconstruction restriction ----------
        Forbidden {
            name: "AT-D1 resume_at_frontier is absent from the production surface",
            source: r#"
use spark_core::id::{ProfileId, SourceId};
use spark_core::timeline::{Ordinal, TimelineEpoch, TimelineIngress};
fn main() {
    let _ = TimelineIngress::resume_at_frontier(
        ProfileId::new("game-world").unwrap(),
        TimelineEpoch(1),
        SourceId::new("sequencer.primary").unwrap(),
        2,
        Ordinal(1_000_000),
    );
}
"#,
            expected_fragment: "resume_at_frontier",
        },
        // ---------------- AT-C: panic-free canonical API -------------
        Forbidden {
            name: "AT-C1 the panic-capable from_static is gone",
            source: r#"
use spark_core::id::CanonicalTag;
fn main() { let _ = CanonicalTag::from_static("INVALID TAG"); }
"#,
            expected_fragment: "from_static",
        },
        Forbidden {
            name: "AT-C2 canonical_tag! rejects an invalid literal at compile time",
            source: r#"
use spark_core::canonical_tag;
fn main() { let _ = canonical_tag!("Trigger"); }
"#,
            expected_fragment: "canonical_tag",
        },
        Forbidden {
            name: "AT-C3 the panic-capable FixedPoint::clamp is gone",
            source: r#"
use spark_core::value::FixedPoint;
fn main() {
    let _ = FixedPoint::from_raw(1).clamp(FixedPoint::from_raw(9), FixedPoint::ZERO);
}
"#,
            expected_fragment: "clamp",
        },
        Forbidden {
            name: "AT-C3 FixedRange cannot be built as a literal",
            source: r#"
use spark_core::value::{FixedPoint, FixedRange};
fn main() {
    let _ = FixedRange { min: FixedPoint::ZERO, max: FixedPoint::ZERO };
}
"#,
            expected_fragment: "FixedRange",
        },
        // ---------------- B-01 preservation ----------------
        Forbidden {
            name: "B-01 AdmissionTicket has no canonical encoding",
            source: r#"
use spark_core::hash::CanonicalEncoder;
use spark_core::timeline::AdmissionTicket;
fn contaminate(ticket: &AdmissionTicket, enc: &mut CanonicalEncoder) {
    ticket.canonicalize(enc);
}
fn main() {}
"#,
            expected_fragment: "canonicalize",
        },
        Forbidden {
            name: "B-01 StageAcknowledgement cannot be built as a literal",
            source: r#"
use spark_core::hash::Digest;
use spark_core::timeline::{AcknowledgedSlotState, StageAcknowledgement};
fn main() {
    let _ = StageAcknowledgement {
        slot_state: AcknowledgedSlotState::NewlyStaged,
        semantic_envelope_hash: Digest::ZERO,
    };
}
"#,
            expected_fragment: "StageAcknowledgement",
        },
        // ---------------- B-03 evidence ----------------
        Forbidden {
            name: "B-03 WorkKeyConflict cannot be built as a literal",
            source: r#"
use spark_core::scheduler::WorkKeyConflict;
use std::collections::BTreeSet;
fn main() {
    let _ = WorkKeyConflict {
        competing_payload_hashes: BTreeSet::new(),
        omitted_distinct: 0,
    };
}
"#,
            expected_fragment: "WorkKeyConflict",
        },
    ]
}

/// Compiles a real external consumer crate against the default-feature
/// production surface and asserts that every forbidden composition path
/// fails to compile, for the right reason — after first proving the
/// harness can compile legitimate code at all.
#[test]
fn forbidden_surfaces_do_not_compile_for_an_external_consumer() {
    let probe = Probe::new();

    // Positive control first. A harness that cannot compile anything
    // would make every assertion below vacuously true.
    let (ok, output) = probe.compile(POSITIVE_CONTROL);
    assert!(
        ok,
        "the positive control must compile, otherwise every negative probe is vacuous.\n\
         Compiler output:\n{output}"
    );

    let mut failures: Vec<String> = Vec::new();
    for forbidden in forbidden_probes() {
        let (compiled, output) = probe.compile(forbidden.source);
        if compiled {
            failures.push(format!(
                "[{}] COMPILED, but must not: the forbidden surface is reachable from an \
                 external crate",
                forbidden.name
            ));
            continue;
        }
        if !output.contains(forbidden.expected_fragment) {
            failures.push(format!(
                "[{}] failed to compile, but the compiler output never mentions '{}', so the \
                 failure may be unrelated to the property under test.\nCompiler output:\n{}",
                forbidden.name, forbidden.expected_fragment, output
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "external compile probes failed:\n\n{}",
        failures.join("\n\n")
    );
}
