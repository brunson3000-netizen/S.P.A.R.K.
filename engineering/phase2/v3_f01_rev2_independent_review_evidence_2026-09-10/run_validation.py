import subprocess,pathlib,json,sys
root=pathlib.Path(__file__).resolve().parents[3]
out=pathlib.Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=True)
e='engineering/phase2/v3_f01_rev2_bounded_correction_evidence_2026-09-10/'
checks=[('model',['python3',e+'model.py',str(out/'model-results.json')]),('probe',['python3',e+'run_rev2_probe.py']),('historical-red',['python3','engineering/phase2/v3_f01_final_independent_review_evidence_2026-09-10/run_regressions.py']),('workspace-tests',['cargo','test','--workspace','--offline']),('fmt',['cargo','fmt','--all','--check']),('clippy',['cargo','clippy','--workspace','--all-targets','--all-features','--offline','--','-D','warnings']),('strict',['cargo','clippy','-p','spark-core','-p','spark-engine','--lib','--all-features','--offline','--','-D','warnings','-D','clippy::unwrap_used','-D','clippy::expect_used','-D','clippy::panic','-D','clippy::indexing_slicing','-D','clippy::arithmetic_side_effects']),('metadata',['cargo','metadata','--format-version','1','--offline']),('whitespace',['git','diff','--check','e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8'])]
for t in ['x86_64-pc-windows-gnu','x86_64-pc-windows-msvc','aarch64-linux-android','armv7-linux-androideabi','x86_64-linux-android']:
 checks.append((t,['cargo','check','--workspace','--all-targets','--offline','--target',t]))
res=[]
for name,cmd in checks:
 p=subprocess.run(cmd,cwd=root,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
 (out/(name+'.txt')).write_text('\n'.join(x.rstrip() for x in p.stdout.splitlines()).rstrip()+'\n' if p.stdout else '')
 res.append({'check':name,'command':cmd,'exit':p.returncode});print(name,p.returncode,flush=True)
(out/'checks.json').write_text(json.dumps(res,indent=2)+'\n')
assert all(x['exit']==0 for x in res),res
