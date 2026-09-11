from pathlib import Path
import subprocess,json
out=Path(__file__).resolve().parent
root=out.parents[2]
base='7e3a0aae069a8bf840e4dfb74221cdf2687b1db5'
candidate='053d1dc1131ec47be94b60513fad9ea8389cde0c'
def git(*args):return subprocess.check_output(['git',*args],cwd=root,text=True)
def content(ref,path):return git('show',f'{ref}:{path}')
paths=git('ls-tree','-r','--name-only',base).splitlines()
unchanged_tests=[]; unchanged_inline=[]; failures=[]
for p in paths:
    if not p.startswith('crates/') or not p.endswith('.rs'):continue
    a=content(base,p); b=content(candidate,p)
    if '/tests/' in p:
        (unchanged_tests if a==b else failures).append(p)
    if '\n#[cfg(test)]' in a:
        (unchanged_inline if a[a.index('#[cfg(test)]'):]==b[b.index('#[cfg(test)]'):] else failures).append(p)
def function(s,name):
    start=s.index('    pub fn '+name+'('); brace=s.index('{',start); depth=1; end=brace+1
    while depth:
        depth += (s[end]=='{')-(s[end]=='}'); end+=1
    return s[start:end]
methods={}
for file,names in [('scheduler',['schedule','drain_due']),('timeline',['stage','submit_fence'])]:
    p=f'crates/spark-core/src/{file}.rs'
    for n in names:methods[n]=function(content(base,p),n)==function(content(candidate,p),n)
checkpoints=['4b02532','aad4633','b7d849f','5fe883f','0423d91','4997d56220000edef3a2b4cd987feff7900d2488',candidate]
previous=base; lineage=[]
for c in checkpoints:
    full=git('rev-parse',c).strip(); parent=git('rev-parse',full+'^').strip()
    lineage.append(dict(commit=full,parent=parent,expected_parent=previous,match=parent==previous)); previous=full
worktrees=[]
for line in git('worktree','list','--porcelain').splitlines():
    if line.startswith('worktree '):
        p=line[9:]; status=subprocess.check_output(['git','-C',p,'status','--porcelain'],text=True)
        worktrees.append(dict(path=p,status=status))
data=dict(candidate=candidate,baseline=base,unchanged_inherited_test_files=unchanged_tests,unchanged_inline_test_modules=unchanged_inline,failures=failures,unchanged_functions=methods,lineage=lineage,final_documentation_commit_paths=git('diff-tree','--no-commit-id','--name-only','-r',candidate).splitlines(),manifests_and_lock_unchanged=git('diff',base,candidate,'--','Cargo.toml','Cargo.lock','crates/*/Cargo.toml')=='',worktrees=worktrees,remote=git('ls-remote','origin','refs/heads/phase1-refoundation-v2','refs/heads/candidate/phase2-gate-c2-implementation-20260911'))
(out/'preservation.json').write_text(json.dumps(data,indent=2)+'\n')
print(json.dumps({k:v for k,v in data.items() if k not in ['unchanged_inherited_test_files','unchanged_inline_test_modules','worktrees']},indent=2))
assert not failures and all(methods.values()) and all(x['match'] for x in lineage)
