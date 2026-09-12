#!/usr/bin/env python3
import json
from own_run_mutants import run,E,mutants
r=run(('OWN-settle-transform-only-v2',mutants['OWN-settle-transform-only']),'own_third_probes_v2.rs')
(E/'own_refined_mutation_summary.json').write_text(json.dumps(r,indent=2)+'\n')
print(json.dumps(r,indent=2))
