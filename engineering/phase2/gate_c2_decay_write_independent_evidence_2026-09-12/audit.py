import subprocess,json,hashlib,difflib
from pathlib import Path
e=Path(__file__).resolve().parent;r=e.parents[2]
def git(*args):return subprocess.check_output(['git',*args],cwd=r,text=True)
h=git('rev-parse','HEAD').strip(); base='a3227eb423e75851b05b9992dfde6424a7d8932d'; checkpoint='b1608ddc4470d73b9c9eaf5ce26c61697000be38'
p='engineering/phase2/gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/own_probes.rs'; old=git('show','e00f248:'+p); original=(r/p).read_text(); adapted=(r/'engineering/phase2/gate_c2_decay_write_resolution_evidence_2026-09-12/own_probes_resolution_adapted.rs').read_text()
paths=git('diff','--name-status',checkpoint,h).splitlines()
res=dict(candidate=h,history=git('log','--reverse','--format=%H %P %s',base+'..'+h).splitlines(),checkpoint_final_diff=paths,documentation_evidence_only=all(x.startswith('A\tengineering/phase2/') for x in paths),checkpoint_crates_tree=git('rev-parse',checkpoint+':crates').strip(),candidate_crates_tree=git('rev-parse',h+':crates').strip(),original_probe_preserved=old==original,adaptation=list(difflib.unified_diff(original.splitlines(),adapted.splitlines(),lineterm='')),one_literal_adaptation=original.replace('(vec![20],95)','(vec![20],85)')==adapted,source_sha256={p:hashlib.sha256(original.encode()).hexdigest()},original_workspace_status=subprocess.check_output(['git','status','--porcelain=v1'],cwd='/home/chromikey/Projects/SPARK',text=True),original_worktrees=subprocess.check_output(['git','worktree','list','--porcelain'],cwd='/home/chromikey/Projects/SPARK',text=True))
(e/'custody-audit.json').write_text(json.dumps(res,indent=2)+'\n');print(json.dumps(res,indent=2))
