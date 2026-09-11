"""Verify recorded review results and distinguish process, compiler and contract outcomes."""
import hashlib,json,re
from pathlib import Path
out=Path(__file__).resolve().parent
checks=json.loads((out/'checks.json').read_text());assert checks['tree']=='5b7fcf50161a65dc83b4806b513f01c0a4943ea5'
assert all(c['exit_code']==0 for c in checks['checks'])
expected={'original_corpus_pre_correction':(3,7),'adapted_corpus_post_correction':(9,0),'independent-probes':(11,3)}
result={'candidate':checks['tree'],'captured_runs':{},'candidate_contract_failures':['c2_05_effect_reapplication_at_frozen_cohort_time_must_reproduce_state','c2_05_saturation_must_preserve_claimed_value_and_commit_time_chunk_invariance','c2_05_shorter_cadence_must_not_bill_steps_ending_before_activation']}
for name,(passed,failed) in expected.items():
 s=(out/(name+'.txt')).read_text();assert f'{passed} passed; {failed} failed; 0 ignored;' in s
 result['captured_runs'][name]={'cargo_exit_code':101 if failed else 0,'passed':passed,'failed':failed}
for test in result['candidate_contract_failures']:assert test in (out/'independent-probes.txt').read_text().split('failures:\n\nfailures:')[1]
original=(out/'original_corpus_post_correction.txt').read_text()
assert all(x in original for x in ['WrongProfile','cohort_identity','toward','mismatched types'])
result['captured_runs']['original_corpus_post_correction']={'cargo_exit_code':101,'compilation_only':True,'changed_surfaces':['FinalizationRefusal construction','CohortReport.cohort_identity','Update::Decay shape and Param types']}
surface=json.loads((out/'surface-probes.json').read_text());assert len(surface)==14 and all(x['pass_'] for x in surface)
mutants=json.loads((out/'mutation-controls.json').read_text());assert len(mutants)==6 and all(x['killed'] for x in mutants)
result['surface_expectations_passed']=len(surface);result['compiled_mutants_killed']=len(mutants)
counts=[tuple(map(int,m)) for m in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', (out/'workspace-tests.txt').read_text())]
assert tuple(map(sum,zip(*counts)))==(374,0,0);result['workspace']={'passed':374,'failed':0,'ignored':0}
assert not json.loads((out/'preservation.json').read_text())['failures']
result['probe_source_sha256']=hashlib.sha256((out/'independent_revision_probes.rs').read_bytes()).hexdigest()
assert result['probe_source_sha256'] in (out/'independent-probes.txt').read_text()
(out/'independent-results.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
