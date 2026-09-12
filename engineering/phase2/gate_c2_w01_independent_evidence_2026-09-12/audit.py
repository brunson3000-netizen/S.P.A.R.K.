import subprocess,json,hashlib,ast
from pathlib import Path
out=Path(__file__).resolve().parent;root=out.parents[2];base='e39690dc51b63fa09fe7c3f30ebd48aa2c16686f';final='67b877192cc78b75c6fbe60c69b5594dc10befe8';checkpoint='d1d9f8e55c33ecb5dddccc0e4799cd8f9aaccb40'
def git(*a):return subprocess.check_output(['git',*a],cwd=root,text=True)
assert git('rev-parse','HEAD').strip()==final
live={line.split()[1]:line.split()[0] for line in (out/'live-heads-before.txt').read_text().splitlines()}
assert live['refs/heads/candidate/phase2-gate-c2-w01-correction-20260912']==final
commits=git('rev-list','--reverse',base+'..'+final).split();contract='engineering/phase2/SPARK_PHASE_2_GATE_C2_W01_CONTRACT_CORRECTION_2026-09-12.md'
assert git('show','--name-status','--format=',commits[0]).strip()=='A\t'+contract
assert git('diff','--name-only',commits[0],final,'--',contract)==''
files=git('diff','--name-status',checkpoint,final).splitlines();assert all(x.startswith('A\tengineering/phase2/') for x in files)
assert git('rev-parse',checkpoint+':crates')==git('rev-parse',final+':crates')
ancestors=['7e3a0aae069a8bf840e4dfb74221cdf2687b1db5','cc3dc70182b428f7cbc953ae722f85b39d407dfa',base,checkpoint]
for a in ancestors:subprocess.run(['git','merge-base','--is-ancestor',a,final],cwd=root,check=True)
# Check every inherited engineering blob by Git object identity, independently of writer's checker.
def blobs(ref):return dict((line.split('\t',1)[1],line.split()[2]) for line in git('ls-tree','-r',ref,'engineering').splitlines())
old,new=blobs(base),blobs(final); changed=[p for p,h in old.items() if new.get(p)!=h];assert not changed
# Read only mutant definitions (stop before runner), compare actual patch tuples.
def mutants(path):
 ns={'__file__':str(path)};exec(path.read_text().split('head = subprocess.run')[0],ns);return {x['id']:x for x in ns['MUTANTS']}
prior=mutants(root/'engineering/phase2/gate_c2_decay_write_resolution_evidence_2026-09-12/run_mutation_controls.py');current=mutants(out/'run_mutation_controls.py')
common=sorted(set(prior)&set(current));same={k:prior[k]['patches']==current[k]['patches'] for k in common};assert len(same)==16 and all(same.values())
summary=dict(candidate=final,live_heads=live,lineage=git('log','--reverse','--format=%H %P %s',base+'..'+final).splitlines(),contract_first_and_immutable=True,final_difference=files,final_documentation_evidence_only=True,crates_tree=git('rev-parse',final+':crates').strip(),checkpoint=checkpoint,ancestors=ancestors,inherited_engineering_files=len(old),inherited_engineering_changed=changed,unchanged_mutant_patches=same,removed_mutants=sorted(set(prior)-set(current)),added_mutants=sorted(set(current)-set(prior)),mutation_scope_note='W01-settle-every-body retains !declares_decay; it is not individually equivalent to unconditional RES-settle-composed-body. W01-settle-decay-body separately covers that old mutant\'s explicit-decay error; assess their union.',candidate_status=git('status','--porcelain'))
(out/'custody-audit.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({k:v for k,v in summary.items() if k not in ['live_heads','final_difference','candidate_status']},indent=2))
