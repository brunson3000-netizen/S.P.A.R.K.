"""Reviewer-authored compiled mutants, isolated from the candidate and writer controls."""
import hashlib, json, os, subprocess, tarfile, tempfile
from pathlib import Path
out = Path(__file__).resolve().parent
root = out.parents[2]
PIN = '6968a4af917c9a32761be371c3e12ec12ba0f1bd'
ENV = {**os.environ, 'CARGO_PROFILE_DEV_DEBUG':'0', 'CARGO_PROFILE_TEST_DEBUG':'0',
       'CARGO_INCREMENTAL':'0', 'CARGO_BUILD_JOBS':'2'}
engine = 'crates/spark-engine/src/engine.rs'
cases = [
    ('own-C2R-01-next-tick', engine,
     '        let epoch = self.behavior_epoch();\n        // Every committed effect is written',
     '        let epoch = self.behavior_epoch();\n        let now = LogicalTime(now.0 + 1);\n        // Every committed effect is written',
     'r01_recovery_reapplication_and_timestamp'),
    ('own-C2R-02-drop-closing-endpoint', engine,
     '.map_or(now.0, |a| a.0.min(now.0));',
     '.map_or(now.0, |a| a.0.saturating_sub(1).min(now.0));',
     'r02_activation_endpoint_is_owned_by_closing_segment'),
    ('own-C2R-03-drop-zero-rate', engine,
     '                    Some(Intent::Transform {\n                        family: crate::rules::TransformFamily::Decay,',
     '                    if matches!(&only.update, Update::Decay { rate: Param::Literal(0), .. }) { return Ok(None); }\n                    Some(Intent::Transform {\n                        family: crate::rules::TransformFamily::Decay,',
     'r03_unmoved_writes_fire_watchers_and_reject_shocks'),
    ('own-C2R-04-drop-later-wave-records', engine,
     '            self.obligations.insert(r.clone());\n        }\n        let batch',
     '            if wave == 0 { self.obligations.insert(r.clone()); }\n        }\n        let batch',
     'r04_per_wave_store_and_index_discriminator'),
]

def normalized(s): return '\n'.join(l.rstrip() for l in s.rstrip().splitlines())+'\n'
results = {'tree':PIN,'environment':{k:v for k,v in ENV.items() if k.startswith('CARGO_')},'baseline':[],'mutants':[]}
with tempfile.TemporaryDirectory(prefix='spark-c2-own-mutants-') as temp:
    temp = Path(temp)
    archive = temp/'source.tar'
    with archive.open('wb') as f: subprocess.run(['git','archive',PIN],cwd=root,stdout=f,check=True)
    tree = temp/'tree'; tree.mkdir()
    with tarfile.open(archive) as t: t.extractall(tree,filter='data')
    crate = temp/'probe'; (crate/'src').mkdir(parents=True)
    source = (out/'independent_probes.rs').read_bytes()
    (crate/'src/lib.rs').write_bytes(source)
    results['probe_sha256'] = hashlib.sha256(source).hexdigest()
    (crate/'Cargo.lock').write_bytes((root/'Cargo.lock').read_bytes())
    (crate/'Cargo.toml').write_text('[package]\nname="reviewer-decay-probes"\nversion="0.0.0"\nedition="2021"\n[workspace]\n[dependencies]\n'+''.join(
        f'{n} = {{ path="{tree}/crates/{n}"'+(', features=["test-support"]' if n=='spark-engine' else '')+' }\n'
        for n in ['spark-core','spark-engine','spark-testkit']))
    def run(name,label):
        cmd=['cargo','test','--offline','--', '--exact',name,'--nocapture']
        p=subprocess.run(cmd,cwd=crate,env=ENV,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
        log=p.stdout
        compiled='Running unittests' in log
        result={'name':name,'exit_code':p.returncode,'compiled':compiled,
                'passed':p.returncode==0 and '1 passed' in log,
                'assertion_failure':p.returncode==101 and compiled and 'panicked at' in log}
        (out/(label+'.txt')).write_text('# tree: '+PIN+'\n$ '+' '.join(cmd)+'\n'+normalized(log))
        return result
    for label,file,old,new,name in cases:
        result=run(name,'baseline-'+label); results['baseline'].append(result)
        assert result['passed'],result
    for label,file,old,new,name in cases:
        path=tree/file; original=path.read_text(); assert original.count(old)==1,(label,original.count(old))
        path.write_text(original.replace(old,new))
        result=run(name,label); results['mutants'].append(dict(id=label,patch_file=file,old=old,new=new,**result))
        path.write_text(original)
        print(label,result,flush=True)
results['all_killed']=all(x['assertion_failure'] for x in results['mutants'])
(out/'independent-mutations.json').write_text(json.dumps(results,indent=2)+'\n')
assert results['all_killed']
