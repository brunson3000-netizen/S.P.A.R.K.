"""Preservation checks for the bounded revision (exit 1 on any violation)."""
import json, re, subprocess, sys
from pathlib import Path

out = Path(__file__).resolve().parent
root = out.parents[2]
PROD = '7e3a0aae069a8bf840e4dfb74221cdf2687b1db5'
REVIEW = '00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0'
CANDIDATE = '053d1dc1131ec47be94b60513fad9ea8389cde0c'
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
    # An inline test module starts with `#[cfg(test)]` at the start of a line
    # (a `//!` doc mention, as in fixture.rs, is not a module).
    starts = [m.start() for m in re.finditer(r'(?m)^#\[cfg\(test\)\]', old)]
    if not starts: continue
    i = starts[-1]
    same = new.endswith(old[i:]) and old[i:] in new
    report['inline_test_modules_identical'][f] = same
    if not same: failures.append(f'inline tests changed: {f}')
# Protected Phase-1 functions, byte-identical to production.
def body(text, sig):
    i = text.find(sig)
    if i < 0: return None
    depth, j = 0, text.index('{', i)
    for k in range(j, len(text)):
        depth += {'{': 1, '}': -1}.get(text[k], 0)
        if depth == 0: return text[i:k + 1]
report['protected_functions_identical'] = {}
for f, sig in [('crates/spark-core/src/scheduler.rs', 'pub fn schedule(&mut self'),
               ('crates/spark-core/src/scheduler.rs', 'pub fn drain_due('),
               ('crates/spark-core/src/timeline.rs', 'pub fn stage('),
               ('crates/spark-core/src/timeline.rs', 'pub fn submit_fence(')]:
    old_body = body(git('show', f'{PROD}:{f}'), sig)
    same = old_body is not None and old_body == body((root / f).read_text(), sig)
    report['protected_functions_identical'][f'{f}::{sig}'] = same
    if not same: failures.append(f'protected function changed: {f} {sig}')
# spark-core: this revision is purely additive relative to the reviewed candidate.
numstat = git('diff', '--numstat', REVIEW, 'HEAD', '--', 'crates/spark-core').split('\n')
report['spark_core_numstat_vs_review'] = [l for l in numstat if l]
for l in numstat:
    if l and l.split()[1] != '0': failures.append(f'spark-core deletion: {l}')
# Lineage: the branch descends from the review commit, which descends from the candidate and production.
for a in [PROD, CANDIDATE, REVIEW]:
    ok = subprocess.run(['git', 'merge-base', '--is-ancestor', a, 'HEAD'], cwd=root).returncode == 0
    report[f'ancestor_{a[:7]}'] = ok
    if not ok: failures.append(f'not an ancestor: {a}')
report['worktrees'] = git('worktree', 'list').splitlines()
report['failures'] = failures
(out / 'preservation.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps({'failures': failures, 'inline_modules_checked': len(report['inline_test_modules_identical'])}))
sys.exit(1 if failures else 0)
