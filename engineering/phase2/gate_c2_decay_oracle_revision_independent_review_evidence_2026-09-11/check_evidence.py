"""Validate captured results without conflating capture status and semantic outcomes."""
import hashlib, json, re, subprocess
from pathlib import Path
out=Path(__file__).resolve().parent
root=out.parents[2]
pin='6968a4af917c9a32761be371c3e12ec12ba0f1bd'
def read(name): return (out/name).read_text()
def data(name): return json.loads(read(name))
checks=data('checks.json')
assert checks['tree']==pin
assert len(checks['checks'])==14
assert all(c['exit_code']==0 for c in checks['checks'])
totals=tuple(sum(int(m[i]) for m in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored',read('workspace-tests.txt'))) for i in range(3))
assert totals==(390,0,0),totals
expected={'adapted-corpus.txt':(9,0),'prior-revision-probes.txt':(13,1),
          'prior-revision-time-assertion-only.txt':(14,0),'independent-probes.txt':(7,2)}
for name,(passed,failed) in expected.items():
    assert f'{passed} passed; {failed} failed; 0 ignored' in read(name),name
    assert 'Running unittests' in read(name),name
old=root/'engineering/phase2/gate_c2_revision_independent_review_evidence_2026-09-11/independent_revision_probes.rs'
assert read('prior_revision_time_assertion_only.rs')==old.read_text().replace('LogicalTime(40)','LogicalTime(42)')
source_pairs=[
 ('original-corpus.txt',root/'engineering/phase2/gate_c2_independent_review_evidence_2026-09-11/independent_counterexamples.rs'),
 ('adapted-corpus.txt',root/'engineering/phase2/gate_c2_revision_independent_review_evidence_2026-09-11/independent_counterexamples_adapted.rs'),
 ('prior-revision-probes.txt',old),('independent-probes.txt',out/'independent_probes.rs')]
for log,source in source_pairs:
    assert hashlib.sha256(source.read_bytes()).hexdigest() in read(log),log
errors=re.findall(r'^error\[(E\d+)\]',read('original-corpus.txt'),re.M)
assert sorted(set(errors))==['E0308','E0559','E0603','E0609'],errors
w=data('mutation-controls.json')
assert w['tree']==pin and len(w['baseline'])==12 and w['baseline_all_pass']
assert len(w['mutants'])==7 and w['all_killed_as_claimed']
assert sum(len(m['survives']) for m in w['mutants'])==2
for m in w['mutants']:
    assert all(k['compiled'] and k['assertion_failure'] for k in m['kills'])
    assert all(s['passed'] for s in m['survives'])
i=data('independent-mutations.json')
assert i['tree']==pin and len(i['baseline'])==4 and all(b['passed'] for b in i['baseline'])
assert len(i['mutants'])==4 and i['all_killed']
surfaces=data('surface-probes.json')
assert len(surfaces)==17 and all(s['pass_'] for s in surfaces)
assert not data('preservation.json')['failures']
assert subprocess.check_output(['git','diff',pin,'--name-only','--','crates','Cargo.toml','Cargo.lock'],cwd=root,text=True)==''
report=root/'engineering/phase2/SPARK_PHASE_2_CODEX_GATE_C2_DECAY_ORACLE_REVISION_INDEPENDENT_REVIEW_2026-09-11.md'
prior=root/'engineering/phase2/SPARK_PHASE_2_CODEX_GATE_C2_REVISION_INDEPENDENT_REVIEW_2026-09-11.md'
rows=lambda text:re.findall(r'^\| (AT-I\d+[a-z]?) \|',text,re.M)
assert set(rows(report.read_text()))==set(rows(prior.read_text()))
assert len(rows(report.read_text()))==len(set(rows(report.read_text())))
summary={'candidate':pin,'required_checks_passed':14,'workspace':{'passed':390,'failed':0,'ignored':0},
 'corpora':expected,'independent_contract_failures':[
 'r2prime_frozen_q7_full_cadence_after_fresh_assignment','r2prime_frozen_q7_full_cadence_after_separate_shock'],
 'writer_mutants_killed':7,'writer_old_test_survivors':2,'independent_mutants_killed':4,
 'default_feature_surface_controls':17,'oracle_rows':len(rows(report.read_text())),
 'oracle_R':['AT-I22'],'oracle_P':['AT-I13'],
 'oracle_qualified_S':['AT-I23 (nonretroactivity supported; residual/grid policy pending)','AT-I39 (per-profile)','AT-I40 (per-profile)'],
 'environment':{'CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'2'},
 'reviewer_corrections':['storage-exhaustion rerun','two expected diagnostic substrings corrected; negatives already hidden'],
 'preservation_passed':True}
(out/'evidence-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
