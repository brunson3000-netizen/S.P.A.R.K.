"""Check fresh evidence pins, own compiled assertion controls and immutable history."""
import hashlib,json,subprocess
from pathlib import Path
out=Path(__file__).resolve().parent
root=out.parents[2]
PIN='544c5f6f99d8dabf9855f8ac68f5666286aa9741'
BASE='6b167d84f21fb60f374a2ab45c62d5c6a040790a'
CP='244c54b66a9c5bdfe8dd49e28b05cc0f5c255637'
ZERO='0c27ad89833d0c88162803ab41dec177424b49bb'
def git(*a):return subprocess.check_output(['git',*a],cwd=root,text=True)
assert [git('rev-parse',p+'^').strip() for p in [PIN,CP,ZERO]]==[CP,ZERO,BASE]
assert not git('diff',CP,PIN,'--','crates','Cargo.toml','Cargo.lock')
files=git('diff','--name-only',CP,PIN).splitlines()
assert len(files)==75 and all(f.startswith('engineering/phase2/') for f in files)
history=git('ls-tree','-r','--name-only',BASE,'engineering').splitlines()
assert all(git('rev-parse',BASE+':'+f)==git('rev-parse',PIN+':'+f) for f in history)
record='engineering/phase2/SPARK_PHASE_2_GATE_C2_DECAY_OPERATOR_ADJUDICATION_2026-09-11.md'
assert git('rev-parse',ZERO+':'+record)==git('rev-parse',PIN+':'+record)
writer=root/'engineering/phase2/gate_c2_decay_adjudication_evidence_2026-09-11'
for line in (writer/'SHA256SUMS').read_text().splitlines():
 h,f=line.split(maxsplit=1)
 assert hashlib.sha256((writer/f.strip()).read_bytes()).hexdigest()==h
own=json.loads((out/'own-mutations.json').read_text())
assert own['tree']==PIN and own['baseline_all_pass'] and own['all_killed']
assert len(own['mutants'])==9 and len(own['baseline'])==9
assert all(m['compiled'] and m['assertion_failure'] for m in own['mutants'])
sha=hashlib.sha256((out/'own_probes.rs').read_bytes()).hexdigest()
assert own['probe_sha256']==sha
log=(out/'own-probes.txt').read_text()
assert sha in log and '9 passed; 0 failed; 0 ignored' in log and 'warning:' not in log
validation=json.loads((out/'checks.json').read_text())
assert validation['tree']==PIN and len(validation['checks'])==14
assert all(c['exit_code']==0 for c in validation['checks'])
summary={'candidate':PIN,'own_probes':9,'own_mutants_compiled_and_killed':9,'own_positive_baselines':9,'own_probe_sha256':sha,'historical_engineering_files_unchanged':len(history),'writer_checksum_manifest_valid':True}
(out/'own-evidence-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
