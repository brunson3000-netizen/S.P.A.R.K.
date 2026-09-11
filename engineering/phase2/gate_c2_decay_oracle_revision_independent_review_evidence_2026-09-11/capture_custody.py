"""Capture exact local/tracking/live pins and preserved worktree states."""
import datetime, json, subprocess
from pathlib import Path
out=Path(__file__).resolve().parent
root=out.parents[2]
def git(*args,cwd=root):
    return subprocess.check_output(['git',*args],cwd=cwd,text=True).strip()
candidate='6968a4af917c9a32761be371c3e12ec12ba0f1bd'
refs={
 'candidate/phase2-gate-c2-decay-oracle-revision-20260911':candidate,
 'review/phase2-gate-c2-revision-independent-20260911':'3a0b51463e1874ad8b02dd3a3261933fcd2e22f4',
 'candidate/phase2-gate-c2-bounded-revision-20260911':'5b7fcf50161a65dc83b4806b513f01c0a4943ea5',
 'phase1-refoundation-v2':'7e3a0aae069a8bf840e4dfb74221cdf2687b1db5',
 'candidate/phase2-gate-c2-implementation-20260911':'053d1dc1131ec47be94b60513fad9ea8389cde0c',
 'review/phase2-gate-c2-independent-20260911':'00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0',
 'research/fable-architecture-challenge-2026-09-09':'03d36dd649641b28b232f4650b773413db634611',
}
live={r.removeprefix('refs/heads/'):h for h,r in (l.split() for l in git('ls-remote','--heads','origin').splitlines())}
data={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'candidate':candidate,'refs':{},'worktrees':[],
      'branches':git('for-each-ref','--format=%(refname) %(objectname)','refs/heads').splitlines()}
for name,expected in refs.items():
    row=dict(expected=expected,local=git('rev-parse','refs/heads/'+name),tracking=git('rev-parse','refs/remotes/origin/'+name),live=live[name])
    row['equal']=all(row[k]==expected for k in ['local','tracking','live'])
    assert row['equal'],row
    data['refs'][name]=row
for block in git('worktree','list','--porcelain').split('\n\n'):
    lines=block.splitlines(); path=lines[0].removeprefix('worktree ')
    data['worktrees'].append({'path':path,'registration':lines[1:], 'status':git('status','--porcelain=v1',cwd=path).splitlines()})
data['code_test_tree_equal_checkpoint1']=git('diff','--name-only','5ae506adf13ddf1d610595c59202c52b7ddee81a',candidate,'--','crates','Cargo.toml','Cargo.lock')==''
data['candidate_tracked_tree_unchanged']=git('diff',candidate,'--name-only')==''
data['checkpoint1_is_ancestor']=subprocess.run(['git','merge-base','--is-ancestor','5ae506adf13ddf1d610595c59202c52b7ddee81a',candidate],cwd=root).returncode==0
data['history']=git('log','--format=%H %s','3a0b514..'+candidate).splitlines()
assert data['code_test_tree_equal_checkpoint1'] and data['candidate_tracked_tree_unchanged'] and data['checkpoint1_is_ancestor']
(out/'custody.json').write_text(json.dumps(data,indent=2)+'\n')
print('Pinned local/tracking/live refs equal; candidate tree unchanged; checkpoint tree equal; worktrees recorded.')
