import hashlib,json,re,subprocess
from pathlib import Path
p=Path(__file__).resolve().parent;root=p.parents[2];final='67b877192cc78b75c6fbe60c69b5594dc10befe8'
def data(name):return json.loads((p/name).read_text())
def totals(name):return tuple(sum(int(x[i]) for x in re.findall(r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored', (p/name).read_text())) for i in range(3))
c=data('checks.json');m=data('mutation-controls.json');pres=data('preservation.json');audit=data('custody-audit.json')
assert c['tree']==m['tree']==final
assert len(c['checks'])==15 and all(x['exit_code']==0 for x in c['checks'])
assert totals('workspace-tests.txt')==(427,0,0)
assert m['baseline_all_pass'] and m['all_killed_as_claimed'] and len(m['mutants'])==20
assert len(m['baseline'])==42
assert not pres['failures'] and pres['preexisting_engineering_files']==782
assert audit['final_documentation_evidence_only'] and audit['inherited_engineering_changed']==[]
expected={'reviewer-own-probes':(9,0,0),'reviewer-materialization-probe':(1,0,0),'historical-own-probes':(8,1,0),'resolution-adapted':(9,0,0),'decision-adapted':(9,0,0)}
for name,want in expected.items():assert totals(name+'.txt')==want
for item in data('corpora-results.json'):
 result=json.loads(item['output']);assert result['exit_code']==(101 if result['log']=='historical-own-probes' else 0)
hist=(p/'historical-own-probes.txt').read_text();assert re.findall(r'^    (\w+)$',hist.rsplit('failures:',1)[-1].split('test result:')[0],re.M)==['own_open_lost_prewrite_step']
for x in m['mutants']:
 log=(p/('mutant-'+x['id']+'.txt')).read_text();assert '/'+x['id']+'/crates/spark-engine)' in log and '(target/debug/deps/' in log
original=data('original-custody.json')
for k,args in {'worktrees':['worktree','list','--porcelain'],'refs':['show-ref'],'status':['status','--porcelain=v1']}.items():assert subprocess.check_output(['git',*args],cwd='/home/chromikey/Projects/SPARK',text=True)==original[k]
summary=dict(candidate=final,required_checks_passed=15,workspace_passed=427,corpora={k:dict(zip(['passed','failed','ignored'],v)) for k,v in expected.items()},writer_baseline_tests=len(m['baseline']),writer_mutants_killed=len(m['mutants']),writer_assertion_kills=sum(len(x['kills']) for x in m['mutants']),writer_expected_survivals=sum(len(x['survives']) for x in m['mutants']),isolated_mutant_targets=True,preservation_passed=True,original_refs_worktrees_and_status_unchanged=True,checkpoint=audit['checkpoint'],final_added_documentation_evidence_files=len(audit['final_difference']),workload_rows=(p/'workload.txt').read_text().split('history_len,')[-1].splitlines()[1:])
(p/'validation-summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
