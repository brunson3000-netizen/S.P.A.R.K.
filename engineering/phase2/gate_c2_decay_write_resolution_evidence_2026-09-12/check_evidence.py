"""Validate the captured results of the Gate C2 decay-write resolution pass, keeping
capture status apart from semantic outcome. Exit status is non-zero on any mismatch.
Writes evidence-summary.json."""
import hashlib, json, re, subprocess
from pathlib import Path
out = Path(__file__).resolve().parent
root = out.parents[2]
P = root / 'engineering/phase2'
BASE = 'a3227eb423e75851b05b9992dfde6424a7d8932d'  # mission checkpoint
def read(name): return (out / name).read_text()
def data(name): return json.loads(read(name))
def totals(text):
    return tuple(sum(int(m[i]) for m in re.findall(
        r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored', text)) for i in range(3))

checks = data('checks.json')
tree = checks['tree']
assert len(checks['checks']) == 15 and all(c['exit_code'] == 0 for c in checks['checks']), checks['checks']
workspace = totals(read('workspace-tests.txt'))
assert workspace == (415, 0, 0), workspace
resolution = re.search(
    r'Running tests/phase2_decay_write_resolution\.rs.*?test result: ok\. (\d+) passed; 0 failed; 0 ignored',
    read('workspace-tests.txt'), re.S)
assert resolution and int(resolution[1]) == 15, resolution
adjudication = re.search(
    r'Running tests/phase2_decay_adjudication\.rs.*?test result: ok\. (\d+) passed; 0 failed; 0 ignored',
    read('workspace-tests.txt'), re.S)
assert adjudication and int(adjudication[1]) == 10, adjudication

# Historical review corpora. The controlling review's own probes run BYTE-UNCHANGED and
# must fail at exactly the one superseded observation; the adapted copy passes 9/9.
OWN = P / 'gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/own_probes.rs'
ADAPTED = out / 'own_probes_resolution_adapted.rs'
PRIOR = P / 'gate_c2_decay_adjudication_evidence_2026-09-11/independent_probes_decision_adapted.rs'
corpora = {'controlling-review-own-probes': (OWN, (8, 1)),
           'controlling-review-own-probes-resolution-adapted': (ADAPTED, (9, 0)),
           'oracle-revision-review-probes-decision-adapted': (PRIOR, (9, 0))}
for log, (source, expected) in corpora.items():
    text = read(log + '.txt')
    assert hashlib.sha256(source.read_bytes()).hexdigest() in text, log
    assert 'Running unittests' in text, log
    assert f'{expected[0]} passed; {expected[1]} failed; 0 ignored' in text, (log, totals(text))
# With --nocapture, panic output interleaves with the progress line, so failing names
# are read from libtest's final `failures:` list.
def failed(log):
    tail = read(log + '.txt').rsplit('failures:', 1)[-1].split('test result:')[0]
    return sorted(re.findall(r'^    (\w+)$', tail, re.M))
assert failed('controlling-review-own-probes') == ['own_open_lost_prewrite_step'], \
    failed('controlling-review-own-probes')

# The adapted copy differs from the review's probe by exactly one expected-value literal.
original, adapted = OWN.read_text(), ADAPTED.read_text()
old, new = '[(vec![20],95),(vec![10,20],85)]', '[(vec![20],85),(vec![10,20],85)]'
assert original.count(old) == 1 and original.replace(old, new) == adapted
changed = [(a, b) for a, b in zip(original.splitlines(), adapted.splitlines()) if a != b]
assert len(changed) == 1 and len(original.splitlines()) == len(adapted.splitlines()), changed

m = data('mutation-controls.json')
assert m['baseline_all_pass'] and m['all_killed_as_claimed']
assert len(m['mutants']) == 17, len(m['mutants'])
resolution_mutants = [x for x in m['mutants'] if x['id'].startswith('RES-')]
assert len(resolution_mutants) == 7, len(resolution_mutants)
for x in m['mutants']:
    assert all(k['compiled'] and k['assertion_failure'] for k in x['kills']), x['id']
    assert all(s['passed'] for s in x['survives']), x['id']
kills = sum(len(x['kills']) for x in m['mutants'])
survivals = sum(len(x['survives']) for x in m['mutants'])
assert survivals == 2, survivals

pres = data('preservation.json')
assert not pres['failures'] and pres['contract_unchanged_since_first_commit']
assert set(pres['crate_changes_vs_base']) == {
    'M\tcrates/spark-engine/src/engine.rs', 'M\tcrates/spark-engine/src/rules.rs',
    'M\tcrates/spark-testkit/tests/phase2_decay_adjudication.rs',
    'A\tcrates/spark-testkit/tests/phase2_decay_write_resolution.rs'}
assert len(pres['production_removed_lines']) == 1
assert subprocess.check_output(
    ['git', 'diff', '--name-only', BASE, 'HEAD', '--', 'Cargo.toml', 'Cargo.lock', 'crates/spark-core'],
    cwd=root, text=True) == ''

summary = {
    'tree': tree, 'base': BASE, 'required_checks_passed': 15,
    'workspace': dict(zip(['passed', 'failed', 'ignored'], workspace)),
    'new_resolution_tests': 15, 'adjudication_tests': 10,
    'corpora': {k: {'passed': v[1][0], 'failed': v[1][1]} for k, v in corpora.items()},
    'superseded_review_assertion': failed('controlling-review-own-probes'),
    'mutants': {'total': len(m['mutants']), 'resolution_mutants': len(resolution_mutants),
                'killed': len(m['mutants']), 'kill_assertions': kills,
                'prior_test_survivals': survivals, 'baseline_tests': len(m['baseline'])},
    'preservation': {'failures': 0,
                     'inline_modules': len(pres['inline_test_modules_identical']),
                     'preexisting_engineering_files': pres['preexisting_engineering_files'],
                     'production_lines_added': pres['production_diff_numstat'],
                     'production_lines_removed': pres['production_removed_lines']},
    'environment': checks['environment'],
}
(out / 'evidence-summary.json').write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2))
