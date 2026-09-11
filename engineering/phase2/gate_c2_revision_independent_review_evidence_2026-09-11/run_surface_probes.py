"""Independent default-feature external surface probes; exact diagnostics retained."""
import json, subprocess, tempfile, shutil
from pathlib import Path
out=Path(__file__).resolve().parent
root=out.parents[2]
variants=['WrongProfile','WrongTimelineEpoch','NotActiveSequencer','OrdinalSpaceExhaustedWindow','OrdinalSpaceExhaustedFrontierAdvance','UnexpectedStagingState { ordinal: spark_core::timeline::Ordinal(0) }','CommandIdentityConflict { command_id: spark_core::id::CommandId::new("cmd.one").unwrap() }','SourceSequenceConflict { source_id: spark_core::id::SourceId::new("source.one").unwrap(), source_sequence: 1 }','SourceSequenceNotIncreasing { source_id: spark_core::id::SourceId::new("source.one").unwrap(), previously_finalized: 2, attempted: 1 }']
cases=[('positive-match',True,'use spark_engine::request::FinalizationRefusal; pub fn inspect(r: &FinalizationRefusal)->bool { matches!(r, FinalizationRefusal::WrongProfile { .. }) }','')]
cases += [(f'refusal-{i}',False,'pub fn probe() { let _ = spark_engine::request::FinalizationRefusal::'+v+'; }','non_exhaustive' if i < 5 else 'non-exhaustive') for i,v in enumerate(variants)]
cases += [('cohort-field',False,'pub fn probe(r: &spark_engine::report::CohortReport) { let _ = &r.cohort_identity; }','no field'),('diagnostics-field',False,'pub fn probe(r: &spark_engine::report::PacingDiagnostics) { let _ = r.command_deferred; }','no field'),('evaluator-view',False,'pub fn probe(_: spark_engine::engine::EvalView) {}','private'),('fixture-hidden',False,'pub fn probe(e: &spark_engine::engine::Engine) { let _ = spark_engine::fixture::observation(e); }','could not find')]
results=[]
with tempfile.TemporaryDirectory(prefix='spark-c2-independent-surfaces-') as temp:
 p=Path(temp);(p/'src').mkdir();shutil.copy(root/'Cargo.lock',p/'Cargo.lock')
 (p/'Cargo.toml').write_text('[package]\nname="independent-surfaces"\nversion="0.0.0"\nedition="2021"\n[workspace]\n[dependencies]\n'+''.join(f'{n} = {{ path = "{root}/crates/{n}" }}\n' for n in ['spark-core','spark-engine']))
 for name,expected,source,diagnostic in cases:
  (p/'src/lib.rs').write_text(source+'\n');r=subprocess.run(['cargo','check','--offline'],cwd=p,capture_output=True,text=True)
  log=r.stdout+r.stderr
  ok=(r.returncode==0)==expected and (expected or diagnostic in log)
  (out/f'surface-{name}.txt').write_text('# source:\n'+source+'\n$ cargo check --offline\n'+'\n'.join(x.rstrip() for x in log.rstrip().splitlines())+'\n')
  results.append(dict(name=name,source=source,expected_compile=expected,exit_code=r.returncode,expected_diagnostic=diagnostic,pass_=ok))
(out/'surface-probes.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps({'passed':sum(x['pass_'] for x in results),'total':len(results)}))
assert all(x['pass_'] for x in results)
