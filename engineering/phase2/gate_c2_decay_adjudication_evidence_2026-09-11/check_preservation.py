"""Preservation checks for the Gate C2 decay adjudication pass (exit 1 on any violation).

Beyond the inherited checks (inherited tests, manifests, lockfile, inline Phase-1
test modules and protected functions byte-identical), this pass claims three things:
- no behavioral code change: every changed line of existing source and test files is
  a comment line;
- the adjudication record is immutable: it was introduced alone, before any
  implementation adjustment, and is byte-unchanged since;
- no historical record is rewritten: relative to the controlling review, every
  engineering/ change is an added file."""
import json, re, subprocess, sys
from pathlib import Path

out = Path(__file__).resolve().parent
root = out.parents[2]
PROD = '7e3a0aae069a8bf840e4dfb74221cdf2687b1db5'
REVIEW = '6b167d84f21fb60f374a2ab45c62d5c6a040790a'        # controlling review, direct base
PRIOR_REVIEW = '3a0b51463e1874ad8b02dd3a3261933fcd2e22f4'  # base of the reviewed correction series
CANDIDATE = '6968a4af917c9a32761be371c3e12ec12ba0f1bd'     # reviewed candidate
ADJUDICATION = 'engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md'
NEW_TEST = 'crates/spark-testkit/tests/phase2_decay_adjudication.rs'
def git(*a):
    return subprocess.run(['git', *a], cwd=root, capture_output=True, text=True, check=True).stdout
failures, report = [], {}
inherited = ['crates/spark-core/tests/timeline_admission.rs'] + [f'crates/spark-testkit/tests/{n}.rs' for n in
    ['external_compile_probes', 'final_closure', 'final_digest_correction', 'refoundation_adversarial',
     'refoundation_v2_adversarial', 'workspace_dependency_direction']]
unchanged = inherited + ['Cargo.toml', 'Cargo.lock', 'crates/spark-core/Cargo.toml',
                         'crates/spark-engine/Cargo.toml', 'crates/spark-testkit/Cargo.toml']
report['byte_identical_to_production'] = {}
for f in unchanged:
    same = git('diff', '--name-only', PROD, 'HEAD', '--', f).strip() == ''
    report['byte_identical_to_production'][f] = same
    if not same: failures.append(f'changed: {f}')
report['inline_test_modules_identical'] = {}
for f in git('ls-tree', '-r', '--name-only', PROD, 'crates').split():
    if not (f.endswith('.rs') and '/src/' in f): continue
    old = git('show', f'{PROD}:{f}')
    new = (root / f).read_text()
    starts = [m.start() for m in re.finditer(r'(?m)^#\[cfg\(test\)\]', old)]
    if not starts: continue
    i = starts[-1]
    same = new.endswith(old[i:]) and old[i:] in new
    report['inline_test_modules_identical'][f] = same
    if not same: failures.append(f'inline tests changed: {f}')
def body(text, sig):
    i = text.find(sig)
    if i < 0: return None
    depth, j = 0, text.index('{', i)
    for k in range(j, len(text)):
        depth += {'{': 1, '}': -1}.get(text[k], 0)
        if depth == 0: return text[i:k + 1]
report['protected_functions_identical'] = {}
# Four Phase-1 functions against production; the derived-index recomputation (added by
# accepted Gate C2 work, absent from production) against the controlling review.
for base, f, sig in [(PROD, 'crates/spark-core/src/scheduler.rs', 'pub fn schedule(&mut self'),
                     (PROD, 'crates/spark-core/src/scheduler.rs', 'pub fn drain_due('),
                     (PROD, 'crates/spark-core/src/timeline.rs', 'pub fn stage('),
                     (PROD, 'crates/spark-core/src/timeline.rs', 'pub fn submit_fence('),
                     (REVIEW, 'crates/spark-core/src/timeline.rs', 'pub fn derived_indexes_consistent(')]:
    old_body = body(git('show', f'{base}:{f}'), sig)
    same = old_body is not None and old_body == body((root / f).read_text(), sig)
    report['protected_functions_identical'][f'{f}::{sig} (vs {base[:7]})'] = same
    if not same: failures.append(f'protected function changed: {f} {sig}')
# spark-core: untouched by this pass; relative to the prior correction base it remains
# purely additive and confined to the two test-support-gated items.
report['spark_core_changed_vs_review'] = git('diff', '--name-only', REVIEW, 'HEAD', '--', 'crates/spark-core').split()
if report['spark_core_changed_vs_review']: failures.append('spark-core changed by this pass')
numstat = [l for l in git('diff', '--numstat', PRIOR_REVIEW, 'HEAD', '--', 'crates/spark-core').split('\n') if l]
report['spark_core_numstat_vs_prior_review'] = numstat
for l in numstat:
    if l.split()[1] != '0': failures.append(f'spark-core deletion: {l}')
added = [l[1:] for l in git('diff', '-U0', PRIOR_REVIEW, 'HEAD', '--', 'crates/spark-core').splitlines()
         if l.startswith('+') and not l.startswith('+++')]
gates = sum(1 for l in added if l.strip() == '#[cfg(any(test, feature = "test-support"))]')
report['spark_core_added_test_support_gates_vs_prior_review'] = gates
if gates != 2: failures.append(f'expected two test-support gates in spark-core, found {gates}')
# No behavioral change: every changed line of a pre-existing crate file is a comment.
status = [l.split('\t') for l in git('diff', '--name-status', REVIEW, 'HEAD', '--', 'crates').splitlines()]
report['crate_changes_vs_review'] = ['\t'.join(s) for s in status]
report['comment_only_files'] = {}
for s in status:
    kind, path = s[0], s[-1]
    if kind == 'A':
        if path != NEW_TEST: failures.append(f'unexpected new crate file: {path}')
        continue
    if kind != 'M': failures.append(f'unexpected crate change {kind}: {path}'); continue
    lines = [l[1:] for l in git('diff', '-U0', REVIEW, 'HEAD', '--', path).splitlines()
             if l[:1] in '+-' and not l.startswith(('+++', '---'))]
    ok = all(l.strip().startswith('//') for l in lines)
    report['comment_only_files'][path] = {'changed_lines': len(lines), 'comment_only': ok}
    if not ok: failures.append(f'non-comment change: {path}')
# The adjudication record: introduced alone in the first commit after the review, unchanged since.
commits = git('rev-list', '--reverse', f'{REVIEW}..HEAD').split()
first = commits[0] if commits else None
report['first_commit_after_review'] = first
report['first_commit_parent_is_review'] = bool(first) and git('rev-parse', f'{first}^').strip() == REVIEW
if not report['first_commit_parent_is_review']: failures.append('branch does not start directly at the review commit')
first_files = git('show', '--name-status', '--format=', first).split('\n') if first else []
first_files = [l for l in first_files if l]
report['first_commit_files'] = first_files
if not first or f'A\t{ADJUDICATION}' not in first_files or any(not l.startswith('A\tengineering/') for l in first_files):
    failures.append('adjudication not recorded alone (documents only) before implementation')
report['adjudication_unchanged_since_first_commit'] = bool(first) and git('diff', '--name-only', first, 'HEAD', '--', ADJUDICATION).strip() == ''
if not report['adjudication_unchanged_since_first_commit']: failures.append('adjudication record modified')
# Historical records: only additions under engineering/.
eng = [l.split('\t') for l in git('diff', '--name-status', REVIEW, 'HEAD', '--', 'engineering').splitlines()]
report['engineering_non_additions'] = ['\t'.join(s) for s in eng if s[0] != 'A']
if report['engineering_non_additions']: failures.append('a historical engineering record changed')
for a in [PROD, CANDIDATE, PRIOR_REVIEW, REVIEW]:
    ok = subprocess.run(['git', 'merge-base', '--is-ancestor', a, 'HEAD'], cwd=root).returncode == 0
    report[f'ancestor_{a[:7]}'] = ok
    if not ok: failures.append(f'not an ancestor: {a}')
report['worktrees'] = git('worktree', 'list').splitlines()
report['failures'] = failures
(out / 'preservation.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps({'failures': failures, 'inline_modules_checked': len(report['inline_test_modules_identical']),
                  'comment_only_files': len(report['comment_only_files'])}))
sys.exit(1 if failures else 0)
