"""Preservation checks for the second Gate C2 correction (exit 1 on any violation)."""
import json, re, subprocess, sys
from pathlib import Path

out = Path(__file__).resolve().parent
root = out.parents[2]
PROD = '7e3a0aae069a8bf840e4dfb74221cdf2687b1db5'
REVIEW = '3a0b51463e1874ad8b02dd3a3261933fcd2e22f4'
CANDIDATE = '5b7fcf50161a65dc83b4806b513f01c0a4943ea5'
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
# Inline #[cfg(test)] modules of every source file that exists in production.
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
# The four Phase-1 functions are compared with production. The derived-index
# recomputation that AT-I28 fault injection exercises was added by accepted Gate
# C2 work (aad4633) and does not exist in production, so it is compared with the
# controlling review commit: this pass must not alter the check it tests.
for base, f, sig in [(PROD, 'crates/spark-core/src/scheduler.rs', 'pub fn schedule(&mut self'),
                     (PROD, 'crates/spark-core/src/scheduler.rs', 'pub fn drain_due('),
                     (PROD, 'crates/spark-core/src/timeline.rs', 'pub fn stage('),
                     (PROD, 'crates/spark-core/src/timeline.rs', 'pub fn submit_fence('),
                     (REVIEW, 'crates/spark-core/src/timeline.rs', 'pub fn derived_indexes_consistent(')]:
    old_body = body(git('show', f'{base}:{f}'), sig)
    same = old_body is not None and old_body == body((root / f).read_text(), sig)
    report['protected_functions_identical'][f'{f}::{sig} (vs {base[:7]})'] = same
    if not same: failures.append(f'protected function changed: {f} {sig}')
# spark-core: this pass is purely additive relative to the controlling review,
# and every added line is inside the test-support-gated fault seam.
numstat = [l for l in git('diff', '--numstat', REVIEW, 'HEAD', '--', 'crates/spark-core').split('\n') if l]
report['spark_core_numstat_vs_review'] = numstat
for l in numstat:
    if l.split()[1] != '0': failures.append(f'spark-core deletion: {l}')
added = [l[1:] for l in git('diff', '-U0', REVIEW, 'HEAD', '--', 'crates/spark-core').splitlines()
         if l.startswith('+') and not l.startswith('+++')]
gates = sum(1 for l in added if l.strip() == '#[cfg(any(test, feature = "test-support"))]')
report['spark_core_added_lines'] = len(added)
report['spark_core_added_test_support_gates'] = gates
if gates != 2: failures.append(f'expected the fault enum and injector to be test-support gated, found {gates} gates')
# Lineage.
for a in [PROD, CANDIDATE, REVIEW]:
    ok = subprocess.run(['git', 'merge-base', '--is-ancestor', a, 'HEAD'], cwd=root).returncode == 0
    report[f'ancestor_{a[:7]}'] = ok
    if not ok: failures.append(f'not an ancestor: {a}')
parent = git('rev-list', '--max-parents=1', '--reverse', f'{REVIEW}..HEAD').split()
report['first_commit_after_review'] = parent[0] if parent else None
report['first_commit_parent_is_review'] = bool(parent) and git('rev-parse', f'{parent[0]}^').strip() == REVIEW
if not report['first_commit_parent_is_review']: failures.append('branch does not start directly at the review commit')
report['worktrees'] = git('worktree', 'list').splitlines()
report['failures'] = failures
(out / 'preservation.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps({'failures': failures, 'inline_modules_checked': len(report['inline_test_modules_identical'])}))
sys.exit(1 if failures else 0)
