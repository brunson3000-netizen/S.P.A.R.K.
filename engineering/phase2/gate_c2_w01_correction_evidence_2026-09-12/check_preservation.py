"""Preservation checks for the Gate C2 C2W-01 correction pass (exit 1 on any
violation).

This pass is a bounded correction of one blocking finding, so it claims:
- the inherited checks: inherited test files, manifests, lockfile, inline Phase-1
  test modules and protected functions byte-identical;
- `spark-core` untouched;
- exactly two crate paths change relative to the controlling review, and the one
  production file changes only by the named additions plus the one replaced
  starting-value line;
- the contract correction was recorded alone, before any production change, and
  is byte-unchanged since;
- every historical record is preserved: the immutable adjudication, the Operator
  resolution, the earlier contract and report, and every pre-existing
  engineering/ file at the controlling review, including all three published
  evidence directories."""
import json, re, subprocess, sys
from pathlib import Path

out = Path(__file__).resolve().parent
root = out.parents[2]
PROD = '7e3a0aae069a8bf840e4dfb74221cdf2687b1db5'   # production / acceptance freeze
REVIEW = 'e39690dc51b63fa09fe7c3f30ebd48aa2c16686f' # controlling review, direct base
BASE = 'cc3dc70182b428f7cbc953ae722f85b39d407dfa'   # the reviewed candidate
CORRECTION = 'engineering/phase2/SPARK_PHASE_2_GATE_C2_W01_CONTRACT_CORRECTION_2026-09-12.md'
NEW_TEST = 'crates/spark-testkit/tests/phase2_decay_write_composition.rs'
HISTORICAL = [
    'engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md',
    'engineering/phase2/SPARK_GATE_C2_DECAY_WRITE_RESOLUTION_MISSION_2026-09-12.md',
    'engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_WRITE_IMPLEMENTATION_CONTRACT_2026-09-12.md',
    'engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_WRITE_RESOLUTION_REPORT_2026-09-12.md',
    'engineering/phase2/SPARK_PHASE_2_CODEX_GATE_C2_DECAY_WRITE_RESOLUTION_INDEPENDENT_REVIEW_2026-09-12.md',
]
EXPECTED_CRATE_CHANGES = {
    'M\tcrates/spark-engine/src/engine.rs',
    f'A\t{NEW_TEST}',
}

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
for base, f, sig in [(PROD, 'crates/spark-core/src/scheduler.rs', 'pub fn schedule(&mut self'),
                     (PROD, 'crates/spark-core/src/scheduler.rs', 'pub fn drain_due('),
                     (PROD, 'crates/spark-core/src/timeline.rs', 'pub fn stage('),
                     (PROD, 'crates/spark-core/src/timeline.rs', 'pub fn submit_fence('),
                     (REVIEW, 'crates/spark-core/src/timeline.rs', 'pub fn derived_indexes_consistent('),
                     # Reused byte-unchanged by composed settlement.
                     (REVIEW, 'crates/spark-engine/src/engine.rs', 'fn decay_walk('),
                     (REVIEW, 'crates/spark-engine/src/engine.rs', 'fn baseline_of('),
                     (REVIEW, 'crates/spark-engine/src/engine.rs', 'fn settled('),
                     (REVIEW, 'crates/spark-engine/src/engine.rs', 'fn settlement_operation('),
                     (REVIEW, 'crates/spark-engine/src/effects.rs', 'pub(crate) fn reduce(')]:
    old_body = body(git('show', f'{base}:{f}'), sig)
    same = old_body is not None and old_body == body((root / f).read_text(), sig)
    report['protected_functions_identical'][f'{f}::{sig} (vs {base[:7]})'] = same
    if not same: failures.append(f'protected function changed: {f} {sig}')

report['spark_core_changed_vs_review'] = git('diff', '--name-only', REVIEW, 'HEAD', '--', 'crates/spark-core').split()
if report['spark_core_changed_vs_review']: failures.append('spark-core changed by this pass')

status = [l for l in git('diff', '--name-status', REVIEW, 'HEAD', '--', 'crates').splitlines() if l]
report['crate_changes_vs_review'] = status
if set(status) != EXPECTED_CRATE_CHANGES:
    failures.append(f'unexpected crate change set: {sorted(set(status) ^ EXPECTED_CRATE_CHANGES)}')
report['production_diff_numstat'] = [l for l in git(
    '--no-pager', 'diff', '--numstat', REVIEW, 'HEAD', '--', 'crates/spark-engine/src').splitlines() if l]
removed = [l[1:] for l in git('diff', '-U0', REVIEW, 'HEAD', '--', 'crates/spark-engine/src').splitlines()
           if l.startswith('-') and not l.startswith('---')]
report['production_removed_lines'] = removed
if len(removed) != 1 or 'let mut v = i128::from(current.unwrap_or(0));' not in removed[0]:
    failures.append(f'unexpected production removals: {removed}')

# The contract correction: recorded alone, before implementation, unchanged since.
commits = git('rev-list', '--reverse', f'{REVIEW}..HEAD').split()
first = commits[0] if commits else None
report['first_commit_after_review'] = first
report['first_commit_parent_is_review'] = bool(first) and git('rev-parse', f'{first}^').strip() == REVIEW
if not report['first_commit_parent_is_review']:
    failures.append('branch does not continue directly from the controlling review')
first_files = [l for l in (git('show', '--name-status', '--format=', first).split('\n') if first else []) if l]
report['first_commit_files'] = first_files
if not first or first_files != [f'A\t{CORRECTION}']:
    failures.append('contract correction not recorded alone before implementation')
report['correction_unchanged_since_first_commit'] = bool(first) and git(
    'diff', '--name-only', first, 'HEAD', '--', CORRECTION).strip() == ''
if not report['correction_unchanged_since_first_commit']: failures.append('contract correction modified')

report['historical_records_unchanged'] = {}
for f in HISTORICAL:
    same = git('diff', '--name-only', REVIEW, 'HEAD', '--', f).strip() == ''
    report['historical_records_unchanged'][f] = same
    if not same: failures.append(f'historical record changed: {f}')
eng = [l.split('\t') for l in git('diff', '--name-status', REVIEW, 'HEAD', '--', 'engineering').splitlines() if l]
report['engineering_non_additions'] = ['\t'.join(s) for s in eng if s[0] != 'A']
if report['engineering_non_additions']: failures.append('a historical engineering record changed')
report['preexisting_engineering_files'] = len(git('ls-tree', '-r', '--name-only', REVIEW, 'engineering').split())
# Every published evidence directory is intact.
report['evidence_directories_unchanged'] = {}
for d in ['gate_c2_decay_write_resolution_evidence_2026-09-12',
          'gate_c2_decay_write_independent_evidence_2026-09-12',
          'gate_c2_decay_adjudication_evidence_2026-09-11',
          'gate_c2_decay_adjudication_independent_review_evidence_2026-09-11']:
    same = git('diff', '--name-only', REVIEW, 'HEAD', '--', f'engineering/phase2/{d}').strip() == ''
    report['evidence_directories_unchanged'][d] = same
    if not same: failures.append(f'published evidence changed: {d}')

for anc in [PROD, BASE, REVIEW]:
    ok = subprocess.run(['git', 'merge-base', '--is-ancestor', anc, 'HEAD'], cwd=root).returncode == 0
    report[f'ancestor_{anc[:7]}'] = ok
    if not ok: failures.append(f'not an ancestor: {anc}')
report['worktrees'] = git('worktree', 'list').splitlines()
report['failures'] = failures
(out / 'preservation.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps({'failures': failures,
                  'inline_modules_checked': len(report['inline_test_modules_identical']),
                  'preexisting_engineering_files': report['preexisting_engineering_files']}))
sys.exit(1 if failures else 0)
