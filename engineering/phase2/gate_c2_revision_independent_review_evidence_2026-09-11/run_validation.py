"""Fresh bounded-revision validation. Reruns every prescribed check on the
current tree and records exact commands, exit codes, and elapsed seconds in
checks.json, with each command's full output in <name>.txt beside it."""
import json, subprocess, time
from pathlib import Path

out = Path(__file__).resolve().parent
root = out.parents[2]
PROD = '7e3a0aae069a8bf840e4dfb74221cdf2687b1db5'
REVIEW = '00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0'
checks = [
    ('fmt', 'cargo fmt --all --check'),
    ('workspace-tests', 'cargo test --workspace --all-features --no-fail-fast'),
    ('clippy', 'cargo clippy --workspace --all-targets --all-features -- -D warnings'),
    ('strict', 'cargo clippy -p spark-core -p spark-engine --lib --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::indexing_slicing -D clippy::arithmetic_side_effects'),
    ('metadata', 'cargo metadata --format-version 1'),
    ('whitespace-candidate-range', f'git diff --check {REVIEW} HEAD'),
    ('whitespace-production-range', f'git diff --check {PROD} HEAD'),
]
checks += [(t, 'cargo check --workspace --all-targets --target ' + t) for t in
           ['x86_64-pc-windows-gnu', 'x86_64-pc-windows-msvc', 'aarch64-linux-android',
            'armv7-linux-androideabi', 'x86_64-linux-android']]
checks += [
    ('original-corpus-unchanged', f'python3 {out}/run_original_corpus.py {root} original_corpus_post_correction'),
    ('adapted-corpus', f'python3 {out}/run_original_corpus.py {root} adapted_corpus_post_correction {out}/independent_counterexamples_adapted.rs --test-support'),
    ('preservation', f'python3 {out}/check_preservation.py'),
    ('workload', 'cargo run --release --offline -p spark-testkit --example gate_c2_workload'),
]
head = subprocess.run(['git', 'rev-parse', 'HEAD'], cwd=root, capture_output=True, text=True).stdout.strip()
results = []
for name, cmd in checks:
    start = time.time()
    with (out / (name + '.txt')).open('w') as f:
        f.write(f'# tree: {head}\n$ ' + cmd + '\n'); f.flush()
        r = subprocess.run(cmd, shell=True, cwd=root, stdout=f, stderr=subprocess.STDOUT)
    results.append(dict(name=name, command=cmd, exit_code=r.returncode, seconds=round(time.time() - start, 3)))
    (out / 'checks.json').write_text(json.dumps({'tree': head, 'checks': results}, indent=2) + '\n')
    print(results[-1], flush=True)
