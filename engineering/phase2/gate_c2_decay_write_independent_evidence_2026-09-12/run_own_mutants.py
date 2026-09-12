"""Independent finite controls. Disposable archives only; candidate stays untouched.
Reuses writer runner's storage settings and named patch shapes where appropriate,
with reviewer-authored vectors. Additional clamp and refusal controls are local.
"""
from pathlib import Path
import subprocess, tempfile, os, json
out=Path(__file__).resolve().parent; root=out.parents[2]
ns={ '__file__':str(out/'run_mutation_controls.py') }
exec((out/'run_mutation_controls.py').read_text().split('head = subprocess.run')[0],ns)
by={m['id']:m for m in ns['MUTANTS']}; E=ns['ENGINE']
selected=[('RES-lost-debt','own_additive_and_replacement'),('RES-decay-reduces-replacement','own_additive_and_replacement'),('RES-current-epoch-only','own_removed_and_barrier'),('C2R-02-reset-every-activation','own_same_time_and_absent'),('RES-watcher-sees-settled-intermediate','own_watchers_and_atomic_refusal'),('RES-settle-composed-body','own_explicit_decay_body_once'),('RES-settle-scale','own_scale_clamp_observed_boundary')]
mutants=[dict(id=i,patches=by[i]['patches'],test=t) for i,t in selected]
mutants += [dict(id='OWN-clamp-settled',test='own_scale_clamp_observed_boundary',patches=[(E,'resolved: current.unwrap_or(0).max(*min).min(*max),','resolved: self.settled(target, scope, ctx.now)?.unwrap_or(0).max(*min).min(*max),')]),dict(id='OWN-absent-decay-creates',test='own_same_time_and_absent',patches=[(E,'let (Some(cell), Some(value)) = (cell, current) else {\n                        return Ok(None);','let (Some(cell), Some(value)) = (cell, current) else {\n                        return Ok(Some(Intent::AddDelta(0)));')]),dict(id='OWN-refusal-bills-endpoint',test='own_watchers_and_atomic_refusal',patches=[(E,'if let Some(v) = view.settled(&c.definition, &c.scope, now)? {','if let Some(v) = view.settled(&c.definition, &c.scope, LogicalTime(now.0.saturating_add(10)))? {')])]
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(); result=dict(tree=head,controls=[])
with tempfile.TemporaryDirectory(prefix='spark-c2-own-mutants-') as tmp:
 tmp=Path(tmp); env={**os.environ,'CARGO_TARGET_DIR':str(tmp/'target'),'CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'2'}
 for m in mutants:
  dst=tmp/m['id']; dst.mkdir(); archive=subprocess.check_output(['git','archive',head],cwd=root); subprocess.run(['tar','-x','-C',str(dst)],input=archive,check=True)
  env['CARGO_TARGET_DIR']=str(dst/'target')
  (dst/'crates/spark-testkit/tests/reviewer_own.rs').write_text((out/'own_probes.rs').read_text())
  cmd=['cargo','test','--offline','-p','spark-testkit','--all-features','--test','reviewer_own','--','--exact',m['test'],'--nocapture']
  def run():
   p=subprocess.run(cmd,cwd=dst,env=env,capture_output=True,text=True); return p,p.stdout+p.stderr
  b,blog=run()
  for f,old,new in m['patches']:
   p=dst/f; s=p.read_text(); assert s.count(old)==1,(m['id'],s.count(old)); p.write_text(s.replace(old,new))
  p,log=run(); entry=dict(id=m['id'],test=m['test'],command=cmd,patches=m['patches'],baseline_exit=b.returncode,mutant_exit=p.returncode,baseline_pass=b.returncode==0 and '1 passed' in blog,killed=p.returncode==101 and 'panicked at' in log and 'Running tests/' in log)
  result['controls'].append(entry)
  (out/('own-mutant-'+m['id']+'.txt')).write_text('\n'.join(x.rstrip() for x in (f'# candidate {head}\n$ '+ ' '.join(cmd)+'\nBASELINE\n'+blog+'\nMUTANT\n'+log).rstrip().splitlines())+'\n')
  (out/'own-mutations.json').write_text(json.dumps(result,indent=2)+'\n'); print(entry['id'],entry['baseline_pass'],entry['killed'],flush=True)
