"""Reexecute retained corpora, reviewer mutants, and surfaces; inspect capture results separately."""
import subprocess
from pathlib import Path
out=Path(__file__).resolve().parent
root=out.parents[2]
p=root/'engineering/phase2'
corpora=[('original-corpus','gate_c2_independent_review_evidence_2026-09-11/independent_counterexamples.rs'),('adapted-corpus','gate_c2_revision_independent_review_evidence_2026-09-11/independent_counterexamples_adapted.rs'),('prior-revision-probes','gate_c2_revision_independent_review_evidence_2026-09-11/independent_revision_probes.rs'),('prior-revision-time-assertion-only','gate_c2_decay_oracle_revision_independent_review_evidence_2026-09-11/prior_revision_time_assertion_only.rs'),('independent-probes','gate_c2_decay_oracle_revision_independent_review_evidence_2026-09-11/independent_probes.rs'),('independent-probes-decision-adapted','gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/independent_probes_decision_adapted.rs')]
for name, source in corpora:
 subprocess.run(['python3',str(out/'run_corpus.py'),str(root),name,str(p/source),'--test-support'],check=True)
for script in ['run_reviewer_mutants.py','run_surface_probes.py']:
 subprocess.run(['python3',str(out/script)],check=True)
