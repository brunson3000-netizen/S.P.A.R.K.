//! Phase-1 test corpus item 20: an automated check of ADR-0001's
//! workspace dependency direction policy.
//!
//! ADR-0001 requires the dependency graph to be "enforced mechanically
//! in CI rather than inferred from a diagram." This test parses every
//! workspace member's `Cargo.toml`, extracts the path-dependency crate
//! names listed under `[dependencies]` (deliberately ignoring
//! `[dev-dependencies]`, since a dev-only cycle such as `spark-core`'s
//! dev-dependency on `spark-testkit` for its own tests is explicitly
//! allowed and does not affect the crate's normal build dependency
//! direction), and asserts each crate's normal dependencies are a subset
//! of what ADR-0001 authorizes for it.
//!
//! This intentionally does not pull in a TOML-parsing crate: the
//! workspace's `Cargo.toml` files are simple enough that a small,
//! explicit line scan is more legible here than a new dependency, and it
//! keeps this check exercisable with zero additional crates.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

/// The ADR-0001-authorized set of normal (non-dev) path-dependencies for
/// each Phase-1 crate. Adding a new workspace member requires adding its
/// policy here, which is the point: an unrecognized crate fails loudly
/// instead of silently passing.
fn allowed_normal_dependencies() -> HashMap<&'static str, BTreeSet<&'static str>> {
    let mut allowed = HashMap::new();
    // spark-core: canonical kernel. May not depend on any other
    // S.P.A.R.K. product-layer crate (ADR-0001 invariant 1).
    allowed.insert("spark-core", BTreeSet::new());
    // spark-profile: may depend on spark-core only (ADR-0001 "spark-profile
    // may depend on spark-core").
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

/// Extracts `(crate_name -> path_dependency_crate_names)` for the
/// `[dependencies]` section only.
fn parse_normal_path_dependencies(cargo_toml: &str) -> BTreeSet<String> {
    let mut deps = BTreeSet::new();
    let mut in_dependencies_section = false;

    for raw_line in cargo_toml.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') {
            in_dependencies_section = line == "[dependencies]";
            continue;
        }
        if !in_dependencies_section || line.is_empty() || line.starts_with('#') {
            continue;
        }
        // Lines look like: `spark-core = { path = "../spark-core" }`
        if let Some((name, rest)) = line.split_once('=') {
            if rest.contains("path") && rest.contains("../") {
                deps.insert(name.trim().to_string());
            }
        }
    }

    deps
}

fn crate_name_from_cargo_toml(cargo_toml: &str) -> String {
    let mut in_package_section = false;
    for raw_line in cargo_toml.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') {
            in_package_section = line == "[package]";
            continue;
        }
        if in_package_section {
            if let Some(rest) = line.strip_prefix("name") {
                let rest = rest.trim_start();
                if let Some(rest) = rest.strip_prefix('=') {
                    return rest.trim().trim_matches('"').to_string();
                }
            }
        }
    }
    panic!("could not find [package] name in Cargo.toml:\n{cargo_toml}");
}

#[test]
fn every_crate_normal_dependencies_respect_adr_0001_direction() {
    let root = workspace_root();
    let crates_dir = root.join("crates");
    let allowed = allowed_normal_dependencies();

    let mut checked = 0usize;
    let entries = std::fs::read_dir(&crates_dir)
        .unwrap_or_else(|e| panic!("reading {}: {e}", crates_dir.display()));

    for entry in entries {
        let entry = entry.expect("dir entry");
        let cargo_toml_path = entry.path().join("Cargo.toml");
        if !cargo_toml_path.is_file() {
            continue;
        }
        let content = std::fs::read_to_string(&cargo_toml_path)
            .unwrap_or_else(|e| panic!("reading {}: {e}", cargo_toml_path.display()));

        let crate_name = crate_name_from_cargo_toml(&content);
        let normal_deps = parse_normal_path_dependencies(&content);

        let allowed_for_crate = allowed.get(crate_name.as_str()).unwrap_or_else(|| {
            panic!(
                "crate '{crate_name}' has no ADR-0001 dependency-direction policy recorded in \
                 this test; add one in `allowed_normal_dependencies` before merging"
            )
        });

        for dep in &normal_deps {
            assert!(
                allowed_for_crate.contains(dep.as_str()),
                "crate '{crate_name}' declares a normal dependency on '{dep}', which ADR-0001 \
                 does not authorize (allowed: {allowed_for_crate:?})"
            );
        }
        checked += 1;
    }

    assert_eq!(
        checked, 3,
        "expected to check exactly the 3 Phase-1 workspace crates (spark-core, spark-profile, \
         spark-testkit); this count should be updated deliberately if the workspace grows"
    );
}

#[test]
fn spark_core_declares_no_normal_dependency_on_any_product_layer_crate() {
    let root = workspace_root();
    let content = std::fs::read_to_string(root.join("crates/spark-core/Cargo.toml")).unwrap();
    let normal_deps = parse_normal_path_dependencies(&content);
    assert!(
        normal_deps.is_empty(),
        "spark-core must have zero normal path-dependencies on other S.P.A.R.K. crates \
         (ADR-0001 invariant 1); found: {normal_deps:?}"
    );
}
