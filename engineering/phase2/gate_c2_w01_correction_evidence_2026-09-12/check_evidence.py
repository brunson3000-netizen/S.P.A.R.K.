"""Validate the captured results of the Gate C2 C2W-01 correction pass, keeping
capture status apart from semantic outcome. Exit status is non-zero on any mismatch.
Writes evidence-summary.json."""
import hashlib, json, re, subprocess
from pathlib import Path
out = Path(__file__).resolve().parent
root = out.parents[2]
P = root / 'engineering/phase2'
REVIEW = 'e39690dc51b63fa09fe7c3f30ebd48aa2c16686f'  # controlling review, direct base
def read(name): return (out / name).read_text()
def data(name): return json.loads(read(name))
def totals(text):
    return tuple(sum(int(m[i]) for m in re.findall(
        r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored', text)) for i in range(3))

checks = data('checks.json')
assert len(checks['checks']) == 15 and all(c['exit_code'] == 0 for c in checks['checks']), checks['checks']
workspace = totals(read('workspace-tests.txt'))
assert workspace == (427, 0, 0), workspace
for suite, count in [('phase2_decay_write_composition', 12),
                     ('phase2_decay_write_resolution', 15),
                     ('phase2_decay_adjudication', 10)]:
    m = re.search(rf'Running tests/{suite}\.rs.*?test result: ok\. (\d+) passed; 0 failed; 0 ignored',
                  read('workspace-tests.txt'), re.S)
    assert m and int(m[1]) == count, (suite, m)

IND = P / 'gate_c2_decay_write_independent_evidence_2026-09-12'
OWN = P / 'gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/own_probes.rs'
corpora = {
    # The controlling review's two blocking vectors now pass, byte-unchanged.
    'reviewer-own-probes': (IND / 'own_probes.rs', (9, 0)),
    'reviewer-materialization-probe': (IND / 'own_materialized.rs', (1, 0)),
    # Historical corpora keep their previously classified dispositions.
    'historical-own-probes': (OWN, (8, 1)),
    'resolution-adapted': (P / 'gate_c2_decay_write_resolution_evidence_2026-09-12/own_probes_resolution_adapted.rs', (9, 0)),
    'decision-adapted': (P / 'gate_c2_decay_adjudication_evidence_2026-09-11/independent_probes_decision_adapted.rs', (9, 0)),
}
for log, (source, expected) in corpora.items():
    text = read(log + '.txt')
    assert hashlib.sha256(source.read_bytes()).hexdigest() in text, log
    assert 'Running unittests' in text, log
    assert f'{expected[0]} passed; {expected[1]} failed; 0 ignored' in text, (log, totals(text))
def failed(log):
    tail = read(log + '.txt').rsplit('failures:', 1)[-1].split('test result:')[0]
    return sorted(re.findall(r'^    (\w+)$', tail, re.M))
# Exactly one expected historical failure remains, and it is the superseded observation.
assert failed('historical-own-probes') == ['own_open_lost_prewrite_step'], failed('historical-own-probes')
assert failed('reviewer-own-probes') == [], failed('reviewer-own-probes')

m = data('mutation-controls.json')
assert m['baseline_all_pass'] and m['all_killed_as_claimed']
assert len(m['mutants']) == 20, len(m['mutants'])
ids = [x['id'] for x in m['mutants']]
assert len([i for i in ids if i.startswith('W01-')]) == 4, ids
# The superseded prior mutant is replaced, not silently dropped.
assert 'RES-settle-composed-body' not in ids and 'W01-settle-every-body' in ids
for x in m['mutants']:
    assert all(k['compiled'] and k['assertion_failure'] for k in x['kills']), x['id']
    assert all(s['passed'] for s in x['survives']), x['id']
kills = sum(len(x['kills']) for x in m['mutants'])
survivals = sum(len(x['survives']) for x in m['mutants'])
assert survivals == 2, survivals

pres = data('preservation.json')
assert not pres['failures'] and pres['correction_unchanged_since_first_commit']
assert set(pres['crate_changes_vs_review']) == {
    'M\tcrates/spark-engine/src/engine.rs',
    'A\tcrates/spark-testkit/tests/phase2_decay_write_composition.rs'}
assert len(pres['production_removed_lines']) == 1
assert all(pres['evidence_directories_unchanged'].values())
assert all(pres['historical_records_unchanged'].values())
assert subprocess.check_output(
    ['git', 'diff', '--name-only', REVIEW, 'HEAD', '--', 'Cargo.toml', 'Cargo.lock', 'crates/spark-core'],
    cwd=root, text=True) == ''

summary = {
    'tree': checks['tree'], 'base': REVIEW, 'required_checks_passed': 15,
    'workspace': dict(zip(['passed', 'failed', 'ignored'], workspace)),
    'new_composition_tests': 12,
    'corpora': {k: {'passed': v[1][0], 'failed': v[1][1]} for k, v in corpora.items()},
    'c2w01_blocking_vectors': 'reviewer own_probes.rs passes 9/9 byte-unchanged',
    'expected_historical_failure': failed('historical-own-probes'),
    'mutants': {'total': len(m['mutants']), 'w01_mutants': 4, 'killed': len(m['mutants']),
                'kill_assertions': kills, 'prior_test_survivals': survivals,
                'baseline_tests': len(m['baseline']),
                'superseded_and_replaced': 'RES-settle-composed-body -> W01-settle-every-body'},
    'preservation': {'failures': 0,
                     'inline_modules': len(pres['inline_test_modules_identical']),
                     'preexisting_engineering_files': pres['preexisting_engineering_files'],
                     'production_numstat': pres['production_diff_numstat'],
                     'production_lines_removed': pres['production_removed_lines']},
    'environment': checks['environment'],
}
(out / 'evidence-summary.json').write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2))
