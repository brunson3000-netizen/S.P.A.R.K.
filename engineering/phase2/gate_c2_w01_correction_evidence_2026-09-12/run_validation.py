"""Fresh validation for the Gate C2 C2W-01 correction pass. It reruns every
prescribed check on the current tree, records exact commands, exit codes and
elapsed seconds in checks.json, and each command's full output
(whitespace-normalized) in <name>.txt beside it.

Builds are storage-bounded, as in the controlling review: no debug info, no
incremental cache, two build jobs. These settings change no source, assertion or
optimization level; the release workload is unaffected. Corpus and mutant runs
are recorded by their own scripts."""
import json, os, subprocess, time
from pathlib import Path

out = Path(__file__).resolve().parent
root = out.parents[2]
PROD = '7e3a0aae069a8bf840e4dfb74221cdf2687b1db5'   # production / acceptance freeze
REVIEW = 'e39690dc51b63fa09fe7c3f30ebd48aa2c16686f' # controlling review, direct base
BASE = 'cc3dc70182b428f7cbc953ae722f85b39d407dfa'   # the reviewed candidate
ENV = {'CARGO_PROFILE_DEV_DEBUG': '0', 'CARGO_PROFILE_TEST_DEBUG': '0',
       'CARGO_INCREMENTAL': '0', 'CARGO_BUILD_JOBS': '2'}
env = {**os.environ, **ENV}
checks = [
    ('fmt', 'cargo fmt --all --check'),
    ('workspace-tests', 'cargo test --workspace --all-features --no-fail-fast'),
    ('clippy', 'cargo clippy --workspace --all-targets --all-features -- -D warnings'),
    ('strict', 'cargo clippy -p spark-core -p spark-engine --lib --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::indexing_slicing -D clippy::arithmetic_side_effects'),
    ('metadata', 'cargo metadata --format-version 1 > /dev/null'),
    ('whitespace-candidate-range', f'git diff --check {REVIEW} HEAD'),
    ('whitespace-production-range', f'git diff --check {PROD} HEAD'),
    ('whitespace-reviewed-candidate-range', f'git diff --check {BASE} HEAD'),
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
    r = subprocess.run(cmd, shell=True, cwd=root, env=env, stdout=subprocess.PIPE,
                       stderr=subprocess.STDOUT, text=True)
    text = '\n'.join(line.rstrip() for line in r.stdout.rstrip().splitlines())
    # An empty output adds no blank line (a trailing blank line fails `git diff --check`).
    body = f'{text}\n' if text else ''
    (out / (name + '.txt')).write_text(f'# tree: {head}\n# whitespace-normalized\n$ {cmd}\n{body}')
    results.append(dict(name=name, command=cmd, exit_code=r.returncode, seconds=round(time.time() - start, 3)))
    (out / 'checks.json').write_text(json.dumps({'tree': head, 'environment': ENV, 'checks': results}, indent=2) + '\n')
    print(results[-1], flush=True)
