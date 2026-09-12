#!/usr/bin/env python3
import json
from own_run_mutants import run,E
controls={
'OWN-current-epoch-only': [('self.lineage\n            .iter()\n            .rev()\n            .find_map(|e| e.rule_set.decay_operation(target))','self.lineage.last().and_then(|e| e.rule_set.decay_operation(target))')],
'OWN-drop-barrier-endpoint': [('.map_or(now.0, |a| a.0.min(now.0));','.map_or(now.0, |a| a.0.saturating_sub(1).min(now.0));')],
}
records=[run(x) for x in controls.items()]
(E/'own_extra_mutation_summary.json').write_text(json.dumps(records,indent=2)+'\n')
print(json.dumps([{k:r[k] for k in ['name','exit_code','compiled','assertion_failures','killed']} for r in records],indent=2))
