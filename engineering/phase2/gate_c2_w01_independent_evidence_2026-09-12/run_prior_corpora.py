import subprocess,json
from pathlib import Path
out=Path(__file__).resolve().parent;root=out.parents[2];p=root/'engineering/phase2'
inputs=[('reviewer-own-probes','gate_c2_decay_write_independent_evidence_2026-09-12/own_probes.rs'),('reviewer-materialization-probe','gate_c2_decay_write_independent_evidence_2026-09-12/own_materialized.rs'),('historical-own-probes','gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/own_probes.rs'),('resolution-adapted','gate_c2_decay_write_resolution_evidence_2026-09-12/own_probes_resolution_adapted.rs'),('decision-adapted','gate_c2_decay_adjudication_evidence_2026-09-11/independent_probes_decision_adapted.rs')]
res=[]
for name,path in inputs:
 cmd=['python3',str(out/'run_corpus.py'),str(root),name,str(p/path),'--test-support'];r=subprocess.run(cmd,capture_output=True,text=True)
 res.append(dict(command=cmd,wrapper_exit=r.returncode,output=r.stdout,stderr=r.stderr));(out/'corpora-results.json').write_text(json.dumps(res,indent=2)+'\n');print(name,r.stdout,flush=True)
