"""Read-only custody/preservation capture against the live origin."""
import datetime,hashlib,json,subprocess
from pathlib import Path
out=Path(__file__).resolve().parent;root=out.parents[2]
def git(*args,cwd=root):return subprocess.check_output(['git',*args],cwd=cwd,text=True).strip()
pins={'candidate/phase2-gate-c2-bounded-revision-20260911':'5b7fcf50161a65dc83b4806b513f01c0a4943ea5','candidate/phase2-gate-c2-implementation-20260911':'053d1dc1131ec47be94b60513fad9ea8389cde0c','review/phase2-gate-c2-independent-20260911':'00d647e6c3581d1dfbaeccf31c6b8d1966f8bfd0','phase1-refoundation-v2':'7e3a0aae069a8bf840e4dfb74221cdf2687b1db5','research/fable-architecture-challenge-2026-09-09':'03d36dd649641b28b232f4650b773413db634611'}
refs={ref.removeprefix('refs/heads/'):sha for sha,ref in (l.split() for l in git('ls-remote','--heads','origin').splitlines())}
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'review_head':git('rev-parse','HEAD'),'remote_url':git('remote','get-url','origin'),'pins':{},'worktrees':[]}
for name,sha in pins.items():
 state={'expected':sha,'local':git('rev-parse','refs/heads/'+name),'tracking':git('rev-parse','refs/remotes/origin/'+name),'live':refs[name]};state['pass']=all(state[k]==sha for k in ['local','tracking','live']);result['pins'][name]=state
for block in git('worktree','list','--porcelain').split('\n\n'):
 data=dict(line.split(' ',1) for line in block.splitlines() if ' ' in line)
 if 'worktree' in data: data['status']=git('status','--porcelain',cwd=data['worktree']);result['worktrees'].append(data)
result['final_checkpoint_code_diff']=git('diff','--name-only','19930f3','5b7fcf5','--','crates','Cargo.toml','Cargo.lock')
result['candidate_source_diff']=git('diff','--name-only','5b7fcf5','--','crates','Cargo.toml','Cargo.lock')
result['instructions']=git('ls-tree','-r','--name-only','HEAD').splitlines()
result['instructions']=[p for p in result['instructions'] if Path(p).name in ('AGENTS.md','CLAUDE.md','CODEX.md')]
(out/'custody.json').write_text(json.dumps(result,indent=2)+'\n')
assert all(x['pass'] for x in result['pins'].values());assert result['final_checkpoint_code_diff']==result['candidate_source_diff']==''
print('All pinned local/tracking/live refs match; candidate sources unchanged; custody.json records all worktree states.')
