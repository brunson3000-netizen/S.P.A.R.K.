"""Validate the captured results of the Gate C2 decay adjudication pass, keeping
capture status apart from semantic outcome. Exit status is non-zero on any mismatch.
Writes evidence-summary.json."""
import hashlib, json, re, subprocess
from pathlib import Path
out = Path(__file__).resolve().parent
root = out.parents[2]
P = root / 'engineering/phase2'
CHECKPOINT1 = '244c54b66a9c5bdfe8dd49e28b05cc0f5c255637'  # code-and-test tree validated
def read(name): return (out / name).read_text()
def data(name): return json.loads(read(name))
def totals(text):
    return tuple(sum(int(m[i]) for m in re.findall(r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored', text)) for i in range(3))

checks = data('checks.json')
assert checks['tree'] == CHECKPOINT1, checks['tree']
assert len(checks['checks']) == 14 and all(c['exit_code'] == 0 for c in checks['checks']), checks['checks']
workspace = totals(read('workspace-tests.txt'))
assert workspace == (400, 0, 0), workspace
new_file = re.search(r'Running tests/phase2_decay_adjudication\.rs.*?test result: ok\. (\d+) passed; 0 failed; 0 ignored',
                     read('workspace-tests.txt'), re.S)
assert new_file and int(new_file[1]) == 10

R1 = P / 'gate_c2_independent_review_evidence_2026-09-11/independent_counterexamples.rs'
R2A = P / 'gate_c2_revision_independent_review_evidence_2026-09-11/independent_counterexamples_adapted.rs'
R2 = P / 'gate_c2_revision_independent_review_evidence_2026-09-11/independent_revision_probes.rs'
R3T = P / 'gate_c2_decay_oracle_revision_independent_review_evidence_2026-09-11/prior_revision_time_assertion_only.rs'
R3 = P / 'gate_c2_decay_oracle_revision_independent_review_evidence_2026-09-11/independent_probes.rs'
ADAPTED = out / 'independent_probes_decision_adapted.rs'
corpora = {'original-corpus': (R1, None), 'adapted-corpus': (R2A, (9, 0)), 'prior-revision-probes': (R2, (13, 1)),
           'prior-revision-time-assertion-only': (R3T, (14, 0)), 'independent-probes': (R3, (7, 2)),
           'independent-probes-decision-adapted': (ADAPTED, (9, 0))}
for log, (source, expected) in corpora.items():
    text = read(log + '.txt')
    assert hashlib.sha256(source.read_bytes()).hexdigest() in text, log
    if expected:
        assert 'Running unittests' in text, log
        assert f'{expected[0]} passed; {expected[1]} failed; 0 ignored' in text, (log, totals(text))
errors = sorted(set(re.findall(r'^error\[(E\d+)\]', read('original-corpus.txt'), re.M)))
assert errors == ['E0308', 'E0559', 'E0603', 'E0609'], errors
# With --nocapture, panic output interleaves with the progress line, so the failing
# names are read from libtest's final `failures:` list (the block before the result).
failed = lambda log: sorted(re.findall(r'^    (\w+)$', read(log + '.txt').rsplit('failures:', 1)[-1].split('test result:')[0], re.M))
assert failed('independent-probes') == ['r2prime_frozen_q7_full_cadence_after_fresh_assignment',
                                        'r2prime_frozen_q7_full_cadence_after_separate_shock'], failed('independent-probes')
assert failed('prior-revision-probes') == ['c2_05_recovery_and_unsaturated_remainders_positive'], failed('prior-revision-probes')
# The decision-adapted probe copy differs from the review's by exactly two expected tuples.
original, adapted = R3.read_text(), ADAPTED.read_text()
swaps = [('assert_eq!(state(&e),(100,LogicalTime(10)),', 'assert_eq!(state(&e),(90,LogicalTime(10)),'),
         ('assert_eq!(state(&e),(75,LogicalTime(40)),', 'assert_eq!(state(&e),(65,LogicalTime(40)),')]
rebuilt = original
for old, new in swaps:
    assert original.count(old) == 1
    rebuilt = rebuilt.replace(old, new)
assert rebuilt == adapted
changed_lines = [(a, b) for a, b in zip(original.splitlines(), adapted.splitlines()) if a != b]
assert len(changed_lines) == 2 and len(original.splitlines()) == len(adapted.splitlines())

m = data('mutation-controls.json')
assert m['tree'] == CHECKPOINT1 and m['baseline_all_pass'] and m['all_killed_as_claimed']
assert len(m['mutants']) == 10 and sum(len(x['survives']) for x in m['mutants']) == 2
for x in m['mutants']:
    assert all(k['compiled'] and k['assertion_failure'] for k in x['kills']), x['id']
    assert all(s['passed'] for s in x['survives']), x['id']
kills = sum(len(x['kills']) for x in m['mutants'])
r = data('reviewer-mutations.json')
assert r['tree'] == CHECKPOINT1 and r['baseline_all_pass'] and r['all_killed'] and len(r['mutants']) == 4
assert r['probe_sha256'] == hashlib.sha256(R3.read_bytes()).hexdigest()
surfaces = data('surface-probes.json')
assert len(surfaces) == 17 and all(s['pass_'] for s in surfaces)
pres = data('preservation.json')
assert not pres['failures'] and pres['adjudication_unchanged_since_first_commit']
assert all(v['comment_only'] for v in pres['comment_only_files'].values())
assert subprocess.check_output(['git', 'diff', '--name-only', CHECKPOINT1, 'HEAD', '--', 'crates', 'Cargo.toml', 'Cargo.lock'],
                               cwd=root, text=True) == ''
summary = {
    'code_and_test_tree': CHECKPOINT1, 'required_checks_passed': 14,
    'workspace': dict(zip(['passed', 'failed', 'ignored'], workspace)), 'new_adjudication_tests': 10,
    'corpora': {k: (v[1] if v[1] else 'expected compile failure: ' + ','.join(errors)) for k, v in corpora.items()},
    'superseded_review_assertions': failed('independent-probes'),
    'mutants': {'total': len(m['mutants']), 'killed': len(m['mutants']), 'kill_assertions': kills,
                'prior_test_survivals': 2, 'baseline_tests': len(m['baseline'])},
    'reviewer_mutants_killed': 4, 'default_feature_surface_controls': 17,
    'preservation': {'failures': 0, 'comment_only_files': len(pres['comment_only_files']),
                     'inline_modules': len(pres['inline_test_modules_identical'])},
    'environment': checks['environment'],
}
(out / 'evidence-summary.json').write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2))
