"""Read-only post-push receipt. Write outside the checkout so its commit stays clean."""
import subprocess,json
from pathlib import Path
out=Path(__file__).resolve().parent;root=out.parents[2]
def git(*a):return subprocess.check_output(['git',*a],cwd=root,text=True).strip()
branch='review/phase2-gate-c2-w01-correction-independent-20260912';candidate='67b877192cc78b75c6fbe60c69b5594dc10befe8'
head=git('rev-parse','HEAD');tracking=git('rev-parse','@{upstream}');live={line.split()[1]:line.split()[0] for line in git('ls-remote','--heads','origin').splitlines()};before={line.split()[1]:line.split()[0] for line in (out/'live-heads-before.txt').read_text().splitlines()}
assert git('branch','--show-current')==branch
assert head==tracking==live['refs/heads/'+branch]
assert all(live[k]==v for k,v in before.items())
assert set(live)-set(before)=={'refs/heads/'+branch}
assert git('rev-parse',head+'^')==candidate
assert git('status','--porcelain')==''
assert all(line.startswith('A\tengineering/phase2/') for line in git('diff','--name-status',candidate,head).splitlines())
original=json.loads((out/'original-custody.json').read_text())
for k,args in {'worktrees':['worktree','list','--porcelain'],'refs':['show-ref'],'status':['status','--porcelain=v1']}.items():assert subprocess.check_output(['git',*args],cwd='/home/chromikey/Projects/SPARK',text=True)==original[k]
receipt=dict(review_branch=branch,review_commit=head,tracking=tracking,live=live['refs/heads/'+branch],synchronized=True,candidate=candidate,production=live['refs/heads/phase1-refoundation-v2'],all_preexisting_remote_heads_unchanged=True,original_refs_worktrees_status_unchanged=True,clean_review_tree=True,only_review_documentation_evidence_added=True)
Path('/tmp/spark-c2w01-review-publication-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt,indent=2))
