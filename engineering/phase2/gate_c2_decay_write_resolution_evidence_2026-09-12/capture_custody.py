"""Capture local/tracking/live pins for every controlling ref and this candidate
branch, plus all worktree states. It asserts equality of the pinned refs; the
candidate branch row records whatever the three refs are at capture time. The
final publication receipt is recorded separately, because a commit cannot
contain its own hash."""
import datetime, json, subprocess
from pathlib import Path
out = Path(__file__).resolve().parent
root = out.parents[2]
def git(*args, cwd=root):
    return subprocess.check_output(['git', *args], cwd=cwd, text=True).strip()
BRANCH = 'candidate/phase2-gate-c2-decay-write-resolution-20260912'
pinned = {
    'review/phase2-gate-c2-decay-adjudication-independent-20260911': 'e00f248e25f6d34f1041e219f74085649714c19d',
    'candidate/phase2-gate-c2-decay-adjudication-20260911': '544c5f6f99d8dabf9855f8ac68f5666286aa9741',
    'review/phase2-gate-c2-decay-oracle-revision-independent-20260911': '6b167d84f21fb60f374a2ab45c62d5c6a040790a',
    'candidate/phase2-gate-c2-decay-oracle-revision-20260911': '6968a4af917c9a32761be371c3e12ec12ba0f1bd',
    'review/phase2-gate-c2-revision-independent-20260911': '3a0b51463e1874ad8b02dd3a3261933fcd2e22f4',
    'candidate/phase2-gate-c2-bounded-revision-20260911': '5b7fcf50161a65dc83b4806b513f01c0a4943ea5',
    'phase1-refoundation-v2': '7e3a0aae069a8bf840e4dfb74221cdf2687b1db5',
    'candidate/phase2-gate-c2-implementation-20260911': '053d1dc1131ec47be94b60513fad9ea8389cde0c',
    'review/phase2-gate-c2-independent-20260911': '00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0',
    'research/fable-architecture-challenge-2026-09-09': '03d36dd649641b28b232f4650b773413db634611',
}
subprocess.run(['git', 'fetch', '--prune', 'origin'], cwd=root, check=True, capture_output=True)
live = {r.removeprefix('refs/heads/'): h for h, r in (l.split() for l in git('ls-remote', '--heads', 'origin').splitlines())}
data = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'refs': {}, 'worktrees': [],
        'branches': git('for-each-ref', '--format=%(refname) %(objectname)', 'refs/heads').splitlines()}
for name, expected in pinned.items():
    row = dict(expected=expected, local=git('rev-parse', 'refs/heads/' + name),
               tracking=git('rev-parse', 'refs/remotes/origin/' + name), live=live[name])
    row['equal'] = all(row[k] == expected for k in ['local', 'tracking', 'live'])
    assert row['equal'], row
    data['refs'][name] = row
row = dict(local=git('rev-parse', 'refs/heads/' + BRANCH), tracking=git('rev-parse', 'refs/remotes/origin/' + BRANCH),
           live=live.get(BRANCH))
row['equal'] = row['local'] == row['tracking'] == row['live']
data['candidate_branch'] = {BRANCH: row}
for block in git('worktree', 'list', '--porcelain').split('\n\n'):
    lines = block.splitlines(); path = lines[0].removeprefix('worktree ')
    data['worktrees'].append({'path': path, 'registration': lines[1:],
                              'status': git('status', '--porcelain=v1', cwd=path).splitlines()})
(out / 'custody.json').write_text(json.dumps(data, indent=2) + '\n')
print(json.dumps({'pinned_equal': True, 'candidate_branch': row}))
