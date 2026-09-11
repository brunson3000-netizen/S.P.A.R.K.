"""Run a review corpus BYTE-UNCHANGED against a chosen crate root, in a disposable
external crate.

Usage: python3 run_corpus.py <crate-root> <log-name> <source.rs> [--test-support]

The source file is read, never modified; its sha256 heads the log. `--test-support`
enables spark-engine's test-support feature in the disposable crate exactly as
spark-testkit's dev-dependency edge does (the second review's probes call
`spark_engine::fixture`, so they need it; the first review's original corpus is run
without it, as that review ran it).

The captured log is whitespace-normalized for Git hygiene (trailing spaces on libtest
progress lines and trailing blank lines are removed); the header says so. Command
output is otherwise exactly what cargo printed. The runner's own exit status is the
cargo exit status, recorded in the JSON line it prints; a failing corpus is evidence,
not a runner failure, so the caller records both.
"""
import json, subprocess, sys, tempfile, hashlib
from pathlib import Path

out = Path(__file__).resolve().parent
root = Path(sys.argv[1]).resolve()
log_name = sys.argv[2]
source = Path(sys.argv[3]).resolve()
test_support = '--test-support' in sys.argv[4:]
text = source.read_text()
with tempfile.TemporaryDirectory(prefix='spark-c2-rev2-corpus-') as temp:
    p = Path(temp); (p / 'src').mkdir()
    (p / 'Cargo.lock').write_text((root / 'Cargo.lock').read_text())
    (p / 'src/lib.rs').write_text(text)
    deps = ''.join(
        f'{n} = {{ path = "{root}/crates/{n}"'
        + (', features = ["test-support"]' if test_support and n == 'spark-engine' else '')
        + ' }\n'
        for n in ['spark-core', 'spark-engine', 'spark-testkit'])
    (p / 'Cargo.toml').write_text('[package]\nname="spark-c2-counterexamples"\nversion="0.0.0"\n'
                                  'edition="2021"\n[workspace]\n[dependencies]\n' + deps)
    cmd = ['cargo', 'test', '--offline', '--', '--nocapture', '--test-threads=1']
    with (out / (log_name + '.txt')).open('w') as log:
        log.write(f'# source: {source.name} sha256={hashlib.sha256(text.encode()).hexdigest()}\n'
                  f'# crate root: {root}\n# spark-engine test-support: {test_support}\n'
                  '# whitespace-normalized: trailing spaces and trailing blank lines removed\n'
                  '$ ' + ' '.join(cmd) + '\n')
        log.flush()
        r = subprocess.run(cmd, cwd=p, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        log.write('\n'.join(line.rstrip() for line in r.stdout.rstrip().splitlines()) + '\n')
print(json.dumps({'log': log_name, 'source': source.name, 'root': str(root),
                  'test_support': test_support, 'exit_code': r.returncode}))
sys.exit(0)
