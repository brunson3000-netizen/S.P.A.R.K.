#!/usr/bin/env python3
"""Run the Revision-2 disposable finalization probe against the UNCHANGED Phase-1 library.

Builds a temporary external Cargo package (no workspace crate, manifest, or test changes),
runs the probe, then requires the --negative-control run to FAIL with the OLD-composition
message. Run from anywhere; requires cached Cargo dependencies (--offline).
"""
import json
import subprocess
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
with tempfile.TemporaryDirectory(prefix="spark-rev2-probe-") as tmp:
    tmp = Path(tmp)
    (tmp / "src").mkdir()
    (tmp / "src/main.rs").write_bytes((HERE / "rev2_finalization_probe.rs").read_bytes())
    (tmp / "Cargo.toml").write_text(
        '[package]\nname="spark-rev2-finalization-probe"\nversion="0.0.0"\nedition="2021"\n'
        "[dependencies]\nspark-core={path=" + json.dumps(str(ROOT / "crates/spark-core"))
        + ',features=["test-support"]}\n')
    # Retain the repository dependency versions in this disposable package.
    (tmp / "Cargo.lock").write_bytes((ROOT / "Cargo.lock").read_bytes())
    cmd = ["cargo", "run", "--offline", "--quiet", "--manifest-path", str(tmp / "Cargo.toml")]
    subprocess.run(cmd, check=True)
    red = subprocess.run(cmd + ["--", "--negative-control"], capture_output=True, text=True)
    assert red.returncode != 0 and "OLD composition refusal atomicity is false" in red.stderr, red.stderr
    print("CONFIRMED: OLD stage-then-fence composition fails the byte-identity requirement (expected red control)")
print("REV2 PROBE RUN COMPLETE")
