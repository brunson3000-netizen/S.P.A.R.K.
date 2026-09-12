from pathlib import Path
import json,re,subprocess,hashlib
p=Path(__file__).resolve().parent;r=p.parents[2]
j=json.loads((p/'checks.json').read_text());m=json.loads((p/'mutation-controls.json').read_text()); own=json.loads((p/'own-mutations.json').read_text())['controls']+json.loads((p/'own-atomic-mutations.json').read_text())['controls']; audit=json.loads((p/'custody-audit.json').read_text())
assert j['tree']==m['tree']=='cc3dc70182b428f7cbc953ae722f85b39d407dfa'
assert len(j['checks'])==15 and all(x['exit_code']==0 for x in j['checks'])
assert m['baseline_all_pass'] and m['all_killed_as_claimed'] and len(m['mutants'])==17
assert len(own)==11 and all(x['baseline_pass'] and x['killed'] for x in own)
assert audit['documentation_evidence_only'] and audit['original_probe_preserved'] and audit['one_literal_adaptation']
assert all('/'+x['id']+'/crates/spark-engine)' in (p/('mutant-'+x['id']+'.txt')).read_text() and '(target/debug/deps/' in (p/('mutant-'+x['id']+'.txt')).read_text() for x in m['mutants'])
wt=subprocess.check_output(['git','worktree','list','--porcelain'],cwd='/home/chromikey/Projects/SPARK',text=True)
assert wt==audit['original_worktrees']
assert subprocess.check_output(['git','status','--porcelain=v1'],cwd='/home/chromikey/Projects/SPARK',text=True)==audit['original_workspace_status']
summary=dict(candidate=j['tree'],required_checks=15,required_checks_passed=True,workspace_passed=sum(map(int,re.findall(r'test result: ok\. (\d+) passed;', (p/'workspace-tests.txt').read_text()))),writer_baseline_tests=len(m['baseline']),writer_mutants_killed=len(m['mutants']),writer_assertion_kill_runs=sum(len(x['kills']) for x in m['mutants']),writer_expected_survivals=sum(len(x['survives']) for x in m['mutants']),reviewer_positive_baselines=11,reviewer_mutants_killed=11,independent_probe_passes=8,independent_current_blocking_failures=2,historical_probe_passes=8,historical_superseded_failures=1,resolution_adapted_passes=9,decision_adapted_passes=9,original_worktrees_unchanged=True,workload_rows=(p/'workload.txt').read_text().split('history_len,')[-1].splitlines()[1:])
assert summary['workspace_passed']==415
(p/'validation-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
