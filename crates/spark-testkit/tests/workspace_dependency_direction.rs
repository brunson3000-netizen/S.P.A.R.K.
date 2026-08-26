//! Phase-1 test corpus item 20: an automated check of ADR-0001's
//! workspace dependency direction policy.
//!
//! Corrected per the Phase-1 correction brief (m-01): the writer pass's
//! version of this test parsed only literal `[dependencies]` entries in
//! each crate's `Cargo.toml`, so it could not see `spark-core`'s unused
//! `dev-dependency` on `spark-testkit` — a real edge `cargo metadata`
//! reports but a hand-written TOML line-scanner, deliberately restricted
//! to one section, cannot. That unused edge has since been removed from
//! `crates/spark-core/Cargo.toml`; this test now invokes and parses
//! `cargo metadata --format-version 1` (the same command the correction
//! brief's completion gate runs) so it inspects the actual resolved
//! dependency graph — normal, dev, build, and target-specific edges alike
//! — instead of one hand-scanned section, and would catch a regression of
//! that edge (or any other unauthorized cross-crate edge, of any kind)
//! immediately.

use serde_json::Value;
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

/// The ADR-0001-authorized set of path-dependencies for each Phase-1
/// crate, covering every dependency kind (normal, dev, build,
/// target-specific) alike. Adding a new workspace member requires adding
/// its policy here, which is the point: an unrecognized crate fails
/// loudly instead of silently passing.
fn allowed_dependencies() -> HashMap<&'static str, BTreeSet<&'static str>> {
    let mut allowed = HashMap::new();
    // spark-core: canonical kernel. May not depend on any other
    // S.P.A.R.K. product-layer crate, in any dependency kind
    // (ADR-0001 invariant 1).
    allowed.insert("spark-core", BTreeSet::new());
    // spark-profile: may depend on spark-core only.
    allowed.insert("spark-profile", BTreeSet::from(["spark-core"]));
    // spark-testkit: fixtures/scenario harness; consumes spark-core and
    // spark-profile canonical types.
    allowed.insert(
        "spark-testkit",
        BTreeSet::from(["spark-core", "spark-profile"]),
    );
    allowed
}

fn workspace_root() -> PathBuf {
    // crates/spark-testkit -> crates -> workspace root.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates directory")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

/// Runs `cargo metadata --format-version 1` and parses it as JSON.
fn cargo_metadata() -> Value {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .current_dir(workspace_root())
        .output()
        .expect("failed to invoke `cargo metadata`");
    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("cargo metadata did not produce valid JSON")
}

/// `(workspace-member-crate-name -> the set of spark-* crate names it
/// depends on, across every dependency kind)`, derived from
/// `resolve.nodes`, which reports the actual resolved dependency graph
/// (not merely what one `[dependencies]`-style section declares).
fn resolved_spark_dependencies(metadata: &Value) -> HashMap<String, BTreeSet<String>> {
    let packages = metadata["packages"]
        .as_array()
        .expect("metadata.packages is an array");
    let workspace_members: BTreeSet<&str> = metadata["workspace_members"]
        .as_array()
        .expect("metadata.workspace_members is an array")
        .iter()
        .map(|v| v.as_str().expect("workspace_members entries are strings"))
        .collect();

    let mut id_to_name: HashMap<&str, &str> = HashMap::new();
    for package in packages {
        let id = package["id"].as_str().expect("package.id is a string");
        let name = package["name"].as_str().expect("package.name is a string");
        id_to_name.insert(id, name);
    }

    let known_spark_crates: BTreeSet<&str> = allowed_dependencies().keys().copied().collect();

    let nodes = metadata["resolve"]["nodes"]
        .as_array()
        .expect("metadata.resolve.nodes is an array");

    let mut result: HashMap<String, BTreeSet<String>> = HashMap::new();
    for node in nodes {
        let node_id = node["id"].as_str().expect("node.id is a string");
        if !workspace_members.contains(node_id) {
            continue;
        }
        let node_name = id_to_name[node_id].to_string();

        let deps = node["deps"].as_array().expect("node.deps is an array");
        let mut spark_deps = BTreeSet::new();
        for dep in deps {
            let dep_name = dep["name"].as_str().expect("dep.name is a string");
            if known_spark_crates.contains(dep_name) && dep_name != node_name {
                spark_deps.insert(dep_name.to_string());
            }
        }
        result.insert(node_name, spark_deps);
    }
    result
}

#[test]
fn every_crate_resolved_dependencies_respect_adr_0001_direction() {
    let metadata = cargo_metadata();
    let resolved = resolved_spark_dependencies(&metadata);
    let allowed = allowed_dependencies();

    for (crate_name, allowed_for_crate) in &allowed {
        let actual = resolved.get(*crate_name).unwrap_or_else(|| {
            panic!("workspace crate '{crate_name}' was not found in `cargo metadata` output")
        });
        for dep in actual {
            assert!(
                allowed_for_crate.contains(dep.as_str()),
                "crate '{crate_name}' resolves a dependency on '{dep}' (of some kind: normal, \
                 dev, build, or target-specific), which ADR-0001 does not authorize \
                 (allowed: {allowed_for_crate:?}, actual: {actual:?})"
            );
        }
    }

    assert_eq!(
        resolved.len(),
        3,
        "expected to check exactly the 3 Phase-1 workspace crates (spark-core, spark-profile, \
         spark-testkit); this count should be updated deliberately if the workspace grows"
    );
}

#[test]
fn spark_core_resolves_zero_dependencies_on_any_product_layer_crate() {
    let metadata = cargo_metadata();
    let resolved = resolved_spark_dependencies(&metadata);
    let spark_core_deps = resolved
        .get("spark-core")
        .expect("spark-core present in resolved graph");
    assert!(
        spark_core_deps.is_empty(),
        "spark-core must resolve zero dependencies (of any kind) on other S.P.A.R.K. crates \
         (ADR-0001 invariant 1); found: {spark_core_deps:?}"
    );
}
