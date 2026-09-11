"""Executed negative controls in a disposable archive, never the reviewed worktree.
Each mutation must compile and cause an assertion failure in the named candidate tests.
"""
import io,json,os,subprocess,tarfile,tempfile
from pathlib import Path
out=Path(__file__).resolve().parent;root=out.parents[2]
engine='crates/spark-engine/src/engine.rs';rules='crates/spark-engine/src/rules.rs';effects='crates/spark-engine/src/effects.rs'
mutations=[
 ('C2-01-watched-only',engine,'for (definition, scope) in watcher.spec().condition_cell_reads() {','for (definition, scope) in watcher.spec().condition_cell_reads().into_iter().take(0) {','phase2_bounded_revision','c2_01_derived_identity'),
 ('C2-02-definition-only',rules,'let targets: BTreeSet<(&DefinitionId, &ScopeRef)> =\n        rule.emits.iter().map(|e| (&e.target, &e.scope)).collect();','let targets: BTreeSet<&DefinitionId> =\n        rule.emits.iter().map(|e| &e.target).collect();','phase2_bounded_revision','c2_02_fanout'),
 ('C2-03-unclaimed-nested',rules,'                    claim(&e.sub_id, errors);\n                    check_emit(profile, baselines, rule_id, e, true, errors);','                    check_emit(profile, baselines, rule_id, e, true, errors);','phase2_bounded_revision','c2_03_materialized_subids'),
 ('C2-04-narrow-before-weight',effects,'    let weighted = acc\n        .checked_mul(i128::from(weight_raw))','    let acc = i128::from(to_i64(acc, "aggregate")?);\n    let weighted = acc\n        .checked_mul(i128::from(weight_raw))','phase2_bounded_revision','c2_04_aggregate_keeps'),
 ('C2-06-payload-only',engine,'CommandPayload::ActivateEpoch { .. } if !reserved => {','CommandPayload::ActivateEpoch { .. } if false && !reserved => {','phase2_bounded_revision','c2_06_epoch_activation'),
 ('R9-shape-only',engine,'self.scheduler.slot_commitment(&r.key) == Some(s.expected_commitment())','(status == WorkSlotStatus::Scheduled && !s.is_contested()) || (status == WorkSlotStatus::Conflicted && s.is_contested())','phase2_oracle_completion','at_i8_enqueue_preflight'),
]
results=[]
with tempfile.TemporaryDirectory(prefix='spark-c2-review-mutants-') as tmp:
 p=Path(tmp)
 archive=subprocess.check_output(['git','archive','5b7fcf50161a65dc83b4806b513f01c0a4943ea5','Cargo.toml','Cargo.lock','crates'],cwd=root)
 with tarfile.open(fileobj=io.BytesIO(archive)) as t:t.extractall(p,filter='data')
 # Establish the disposable unmodified suite's relevant files pass first.
 for file in ['phase2_bounded_revision','phase2_oracle_completion']:
  cmd=['cargo','test','--offline','-p','spark-testkit','--all-features','--test',file]
  r=subprocess.run(cmd,cwd=p,capture_output=True,text=True)
  (out/f'mutation-baseline-{file}.txt').write_text('$ '+' '.join(cmd)+'\n'+'\n'.join(x.rstrip() for x in (r.stdout+r.stderr).rstrip().splitlines())+'\n')
  assert r.returncode==0
 for name,path,before,after,test,needle in mutations:
  f=p/path;original=f.read_text();assert original.count(before)==1,(name,original.count(before))
  f.write_text(original.replace(before,after))
  cmd=['cargo','test','--offline','-p','spark-testkit','--all-features','--test',test,needle,'--','--nocapture']
  r=subprocess.run(cmd,cwd=p,capture_output=True,text=True);log=r.stdout+r.stderr
  f.write_text(original)
  killed=r.returncode==101 and 'test result: FAILED.' in log and 'panicked at' in log and 'could not compile' not in log
  (out/f'mutant-{name}.txt').write_text('# disposable mutation: '+before+'\n# replaced by: '+after+'\n$ '+' '.join(cmd)+'\n'+'\n'.join(x.rstrip() for x in log.rstrip().splitlines())+'\n')
  results.append(dict(name=name,file=path,before=before,after=after,command=cmd,exit_code=r.returncode,killed=killed))
  (out/'mutation-controls.json').write_text(json.dumps(results,indent=2)+'\n');print(name,killed,flush=True)
assert all(r['killed'] for r in results)
