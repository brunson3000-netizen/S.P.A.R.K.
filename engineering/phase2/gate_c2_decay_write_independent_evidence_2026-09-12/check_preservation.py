"""Preservation checks for the Gate C2 decay-write resolution pass (exit 1 on any
violation).

Unlike the preceding adjudication pass, this one deliberately changes production
logic, so it claims a bounded change rather than a comment-only one:
- the inherited checks: inherited test files, manifests, lockfile, inline Phase-1
  test modules and protected functions byte-identical;
- `spark-core` is untouched by this pass;
- exactly four crate paths change relative to the mission checkpoint, and the two
  production files change only by the named additions plus the one replaced
  reducer read;
- the implementation contract was recorded alone, before any implementation
  change, and is byte-unchanged since;
- the Operator mission and adjudication records are byte-unchanged, and every
  pre-existing engineering/ file at the checkpoint is byte-identical, so no
  historical record or original review probe is rewritten."""
import json, re, subprocess, sys
from pathlib import Path

out = Path(__file__).resolve().parent
root = out.parents[2]
PROD = '7e3a0aae069a8bf840e4dfb74221cdf2687b1db5'   # production / acceptance freeze
REVIEW = 'e00f248e25f6d34f1041e219f74085649714c19d' # controlling independent review
BASE = 'a3227eb423e75851b05b9992dfde6424a7d8932d'   # mission checkpoint, direct base
CANDIDATE = '544c5f6f99d8dabf9855f8ac68f5666286aa9741'  # reviewed candidate
CONTRACT = 'engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_WRITE_IMPLEMENTATION_CONTRACT_2026-09-12.md'
MISSION = 'engineering/phase2/SPARK_GATE_C2_DECAY_WRITE_RESOLUTION_MISSION_2026-09-12.md'
ADJUDICATION = 'engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md'
NEW_TEST = 'crates/spark-testkit/tests/phase2_decay_write_resolution.rs'
EXPECTED_CRATE_CHANGES = {
    'M\tcrates/spark-engine/src/engine.rs',
    'M\tcrates/spark-engine/src/rules.rs',
    'M\tcrates/spark-testkit/tests/phase2_decay_adjudication.rs',
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
                     (BASE, 'crates/spark-core/src/timeline.rs', 'pub fn derived_indexes_consistent('),
                     # The fixed-grid walk itself is reused unchanged by settlement.
                     (BASE, 'crates/spark-engine/src/engine.rs', 'fn decay_walk('),
                     (BASE, 'crates/spark-engine/src/engine.rs', 'fn baseline_of(')]:
    old_body = body(git('show', f'{base}:{f}'), sig)
    same = old_body is not None and old_body == body((root / f).read_text(), sig)
    report['protected_functions_identical'][f'{f}::{sig} (vs {base[:7]})'] = same
    if not same: failures.append(f'protected function changed: {f} {sig}')

report['spark_core_changed_vs_base'] = git('diff', '--name-only', BASE, 'HEAD', '--', 'crates/spark-core').split()
if report['spark_core_changed_vs_base']: failures.append('spark-core changed by this pass')

# Exactly the four expected crate paths change, and nothing is deleted.
status = [l for l in git('diff', '--name-status', BASE, 'HEAD', '--', 'crates').splitlines() if l]
report['crate_changes_vs_base'] = status
if set(status) != EXPECTED_CRATE_CHANGES:
    failures.append(f'unexpected crate change set: {sorted(set(status) ^ EXPECTED_CRATE_CHANGES)}')
report['production_diff_numstat'] = [l for l in git(
    '--no-pager', 'diff', '--numstat', BASE, 'HEAD', '--', 'crates/spark-engine/src').splitlines() if l]
removed = [l[1:] for l in git('diff', '-U0', BASE, 'HEAD', '--', 'crates/spark-engine/src').splitlines()
           if l.startswith('-') and not l.startswith('---')]
report['production_removed_lines'] = removed
# One production line is replaced (the reducer's pre-wave read); everything else is added.
if len(removed) != 1 or 'reduce(&canonical' not in removed[0]:
    failures.append(f'unexpected production removals: {removed}')

# The implementation contract: recorded alone, before implementation, unchanged since.
commits = git('rev-list', '--reverse', f'{BASE}..HEAD').split()
first = commits[0] if commits else None
report['first_commit_after_checkpoint'] = first
report['first_commit_parent_is_checkpoint'] = bool(first) and git('rev-parse', f'{first}^').strip() == BASE
if not report['first_commit_parent_is_checkpoint']:
    failures.append('branch does not continue directly from the mission checkpoint')
first_files = [l for l in (git('show', '--name-status', '--format=', first).split('\n') if first else []) if l]
report['first_commit_files'] = first_files
if not first or first_files != [f'A\t{CONTRACT}']:
    failures.append('implementation contract not recorded alone before implementation')
report['contract_unchanged_since_first_commit'] = bool(first) and git(
    'diff', '--name-only', first, 'HEAD', '--', CONTRACT).strip() == ''
if not report['contract_unchanged_since_first_commit']: failures.append('implementation contract modified')

# Immutable records and every pre-existing engineering/ file.
report['immutable_records_unchanged'] = {}
for f in [MISSION, ADJUDICATION]:
    same = git('diff', '--name-only', BASE, 'HEAD', '--', f).strip() == ''
    report['immutable_records_unchanged'][f] = same
    if not same: failures.append(f'immutable record changed: {f}')
eng = [l.split('\t') for l in git('diff', '--name-status', BASE, 'HEAD', '--', 'engineering').splitlines() if l]
report['engineering_non_additions'] = ['\t'.join(s) for s in eng if s[0] != 'A']
if report['engineering_non_additions']: failures.append('a historical engineering record changed')
report['preexisting_engineering_files'] = len(git('ls-tree', '-r', '--name-only', BASE, 'engineering').split())

for a in [PROD, CANDIDATE, REVIEW, BASE]:
    ok = subprocess.run(['git', 'merge-base', '--is-ancestor', a, 'HEAD'], cwd=root).returncode == 0
    report[f'ancestor_{a[:7]}'] = ok
    if not ok: failures.append(f'not an ancestor: {a}')
report['worktrees'] = git('worktree', 'list').splitlines()
report['failures'] = failures
(out / 'preservation.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps({'failures': failures,
                  'inline_modules_checked': len(report['inline_test_modules_identical']),
                  'preexisting_engineering_files': report['preexisting_engineering_files']}))
sys.exit(1 if failures else 0)
