"""Run the review's ORIGINAL independent counterexample corpus, byte-unchanged,
against a chosen crate root, in a disposable external crate.

Usage: python3 run_original_corpus.py <crate-root> <log-name> [<variant.rs> [--test-support]]

The corpus file is read from the review evidence directory and never modified.
With a third argument, that variant file is compiled instead (used only for the
documented mechanically-adapted variant); the original is always recorded first.
`--test-support` enables spark-engine's test-support feature in the disposable
crate, exactly as spark-testkit's dev-dependency edge does; it is used only for
the adapted variant (adaptation A4) and never for the original run.
"""
import json, subprocess, sys, tempfile, hashlib
from pathlib import Path

out = Path(__file__).resolve().parent
review = out.parent / 'gate_c2_independent_review_evidence_2026-09-11'
root = Path(sys.argv[1]).resolve()
log_name = sys.argv[2]
source = Path(sys.argv[3]).resolve() if len(sys.argv) > 3 else review / 'independent_counterexamples.rs'
test_support = '--test-support' in sys.argv[4:]
text = source.read_text()
with tempfile.TemporaryDirectory(prefix='spark-c2-rev-corpus-') as temp:
    p = Path(temp); (p / 'src').mkdir()
    (p / 'Cargo.lock').write_text((root / 'Cargo.lock').read_text())
    (p / 'src/lib.rs').write_text(text)
    (p / 'Cargo.toml').write_text('[package]\nname="spark-c2-counterexamples"\nversion="0.0.0"\nedition="2021"\n[workspace]\n[dependencies]\n' + ''.join(f'{n} = {{ path = "{root}/crates/{n}"' + (', features = ["test-support"]' if test_support and n == 'spark-engine' else '') + ' }\n' for n in ['spark-core', 'spark-engine', 'spark-testkit']))
    cmd = ['cargo', 'test', '--offline', '--', '--nocapture', '--test-threads=1']
    with (out / (log_name + '.txt')).open('w') as log:
        log.write(f'# source: {source.name} sha256={hashlib.sha256(text.encode()).hexdigest()}\n# crate root: {root}\n# spark-engine test-support: {test_support}\n$ ' + ' '.join(cmd) + '\n'); log.flush()
        r = subprocess.run(cmd, cwd=p, stdout=log, stderr=subprocess.STDOUT)
print(json.dumps({'log': log_name, 'source': source.name, 'root': str(root), 'test_support': test_support, 'exit_code': r.returncode}))
