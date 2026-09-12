#!/usr/bin/env python3
"""Independent third-review mutations; only extracted archives change. All logs retained."""
import concurrent.futures, hashlib, json, os, pathlib, re, subprocess, tarfile, tempfile
ROOT=pathlib.Path('/tmp/spark-c2w01-independent-review-20260912')
E=ROOT/'engineering/phase2/gate_c2_w01_independent_evidence_2026-09-12'
SHA='67b877192cc78b75c6fbe60c69b5594dc10befe8'
SRC='crates/spark-engine/src/engine.rs'
mutants={
 'OWN-subtract-not-additive': [('let is_additive_composition = !declares_decay\n            && ops\n                .iter()\n                .any(|o| matches!(o.update, Update::Add(_) | Update::Subtract(_)));','let is_additive_composition = !declares_decay\n            && ops\n                .iter()\n                .any(|o| matches!(o.update, Update::Add(_)));')],
 'OWN-only-additive-first': [('let start = if is_additive_composition {','let start = if is_additive_composition && matches!(ops[0].update, Update::Add(_) | Update::Subtract(_)) {')],
 'OWN-settle-after-fold': [('let mut v = i128::from(start.unwrap_or(0));','let mut v = i128::from(current.unwrap_or(0));'),('resolved: to_i64(v, "rule_body")?,','resolved: to_i64(v - i128::from(current.unwrap_or(0)) + i128::from(start.unwrap_or(0)), "rule_body")?,')],
 'OWN-charge-start-twice': [('let mut v = i128::from(start.unwrap_or(0));','let mut v = i128::from(start.unwrap_or(0)) * 2 - i128::from(current.unwrap_or(0));')],
 'OWN-settle-declared-decay': [('let is_additive_composition = !declares_decay','let is_additive_composition = true /* disregard declares_decay */')],
 'OWN-settle-transform-only': [('let start = if is_additive_composition {','let start = if !declares_decay {')],
 'OWN-aggregate-inherits-debt': [('Update::Aggregate { source, weight } => self.aggregate(source, weight)?,','Update::Aggregate { source, weight } => self.aggregate(source, weight)? - i128::from(current.unwrap_or(0)) + i128::from(start.unwrap_or(0)),')],
 'OWN-watcher-intermediate': [('let old = view.cell(&r.definition, &r.scope).unwrap_or(0);','let old = view.settled(&r.definition, &r.scope, now)?.unwrap_or(0);')],
 'OWN-lineage-first': [('self.lineage\n            .iter()\n            .rev()\n            .find_map','self.lineage\n            .iter()\n            .find_map')],
 'OWN-derived-time-lag': [('WritePath::CommitDerived => self.store.commit_derived(\n                    self.profile_id.clone(),\n                    e.definition.clone(),\n                    e.scope.clone(),\n                    e.value.clone(),\n                    now,','WritePath::CommitDerived => self.store.commit_derived(\n                    self.profile_id.clone(),\n                    e.definition.clone(),\n                    e.scope.clone(),\n                    e.value.clone(),\n                    LogicalTime(now.0.saturating_sub(1)),')],
 'OWN-refusal-not-rejected': [('self.store\n                .validate_effect(&r.definition, &r.scope, &value, r.write_path)\n                .map_err(|error| WaveRejection::InvalidEffect {\n                    definition: r.definition.clone(),\n                    scope: r.scope.clone(),\n                    error: Box::new(error),\n                })?;','let _ = self.store.validate_effect(&r.definition, &r.scope, &value, r.write_path);')],
}
# OWN-lineage-first is deliberately a plausibly surviving control: all epochs in these vectors
# retain rule/sub-ID identity, so oldest and latest operation resolution may coincide. Report honestly.

def run(item, probe_name="own_third_probes.rs"):
 name,patches=item
 td=pathlib.Path(tempfile.mkdtemp(prefix='spark-c2w01-third-'+name+'-'))
 archive=td/'candidate.tar'
 with archive.open('wb') as f: subprocess.run(['git','archive',SHA],cwd=ROOT,stdout=f,check=True)
 tree=td/'candidate';tree.mkdir()
 with tarfile.open(archive) as tf:tf.extractall(tree,filter='data')
 p=tree/SRC;s=p.read_text()
 for before,after in patches:
  assert s.count(before)==1,(name,s.count(before))
  s=s.replace(before,after)
 p.write_text(s)
 probe=td/'probe';(probe/'src').mkdir(parents=True)
 (probe/'src/lib.rs').write_bytes((E/probe_name).read_bytes())
 (probe/'Cargo.toml').write_text('[package]\nname="own-third-mutant"\nversion="0.0.0"\nedition="2021"\n[dependencies]\n'+''.join(f'{pkg} = {{path="{tree}/crates/{pkg}"'+(', features=["test-support"]' if pkg!='spark-testkit' else '')+'}\n' for pkg in ['spark-core','spark-engine','spark-testkit']))
 cmd=['cargo','test','--manifest-path',str(probe/'Cargo.toml'),'--offline','--target-dir',str(td/'target'),'--','--nocapture']
 env=os.environ.copy();env['CARGO_BUILD_JOBS']='2'
 r=subprocess.run(cmd,cwd=ROOT,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,env=env)
 out=r.stdout.decode();(E/(name+'.log')).write_text(out)
 failures=re.findall(r'^test (\S+) \.\.\. FAILED$',out,re.M)
 compiled='Running unittests' in out
 record={'name':name,'command':cmd,'exit_code':r.returncode,'compiled':compiled,'assertion_failures':failures,'killed':r.returncode==101 and compiled and bool(failures),'patches':patches,'probe_source':probe_name,'probe_sha256':hashlib.sha256((probe/'src/lib.rs').read_bytes()).hexdigest(),'target_dir':str(td/'target')}
 (E/(name+'.json')).write_text(json.dumps(record,indent=2)+'\n')
 return record
if __name__=='__main__':
 with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
  records=list(pool.map(run,mutants.items()))
 (E/'own_mutation_summary.json').write_text(json.dumps(records,indent=2)+'\n')
 print(json.dumps([{k:r[k] for k in ['name','exit_code','compiled','assertion_failures','killed']} for r in records],indent=2))
