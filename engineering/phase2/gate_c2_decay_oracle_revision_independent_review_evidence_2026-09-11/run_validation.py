"""Fresh validation for the second Gate C2 correction. Reruns every prescribed
check on the current tree and records exact commands, exit codes, and elapsed
seconds in checks.json, with each command's full output (whitespace-normalized)
in <name>.txt beside it. Corpus runs are recorded by run_corpus.py separately;
their cargo exit codes are evidence (a failing corpus is not a runner failure)."""
import json, subprocess, time
from pathlib import Path
import os
# Storage-bounded review builds; source, assertions and optimization level unchanged.
for key, val in {'CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'2'}.items():
    os.environ.setdefault(key, val)

out = Path(__file__).resolve().parent
root = out.parents[2]
PROD = '7e3a0aae069a8bf840e4dfb74221cdf2687b1db5'
REVIEW = '3a0b51463e1874ad8b02dd3a3261933fcd2e22f4'
checks = [
    ('fmt', 'cargo fmt --all --check'),
    ('workspace-tests', 'cargo test --workspace --all-features --no-fail-fast'),
    ('clippy', 'cargo clippy --workspace --all-targets --all-features -- -D warnings'),
    ('strict', 'cargo clippy -p spark-core -p spark-engine --lib --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::indexing_slicing -D clippy::arithmetic_side_effects'),
    ('metadata', 'cargo metadata --format-version 1 > /dev/null'),
    ('whitespace-candidate-range', f'git diff --check {REVIEW} HEAD'),
    ('whitespace-production-range', f'git diff --check {PROD} HEAD'),
]
checks += [(t, 'cargo check --workspace --all-targets --target ' + t) for t in
           ['x86_64-pc-windows-gnu', 'x86_64-pc-windows-msvc', 'aarch64-linux-android',
            'armv7-linux-androideabi', 'x86_64-linux-android']]
checks += [
    ('preservation', f'python3 {out}/check_preservation.py'),
    ('workload', 'cargo run --release --offline -p spark-testkit --example gate_c2_workload'),
]
head = subprocess.run(['git', 'rev-parse', 'HEAD'], cwd=root, capture_output=True, text=True).stdout.strip()
results = []
for name, cmd in checks:
    start = time.time()
    r = subprocess.run(cmd, shell=True, cwd=root, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    text = '\n'.join(line.rstrip() for line in r.stdout.rstrip().splitlines())
    # An empty output adds no blank line (a trailing blank line fails `git diff --check`).
    body = f'{text}\n' if text else ''
    (out / (name + '.txt')).write_text(f'# tree: {head}\n# whitespace-normalized\n$ {cmd}\n{body}')
    results.append(dict(name=name, command=cmd, exit_code=r.returncode, seconds=round(time.time() - start, 3)))
    (out / 'checks.json').write_text(json.dumps({'tree': head, 'checks': results}, indent=2) + '\n')
    print(results[-1], flush=True)
