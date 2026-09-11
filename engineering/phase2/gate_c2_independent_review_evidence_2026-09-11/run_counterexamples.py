from pathlib import Path
import subprocess, tempfile, json
out=Path(__file__).resolve().parent
root=out.parents[2]
with tempfile.TemporaryDirectory(prefix='spark-c2-counterexamples-') as temp:
    p=Path(temp); (p/'src').mkdir()
    (p/'Cargo.lock').write_text((root/'Cargo.lock').read_text())
    (p/'src/lib.rs').write_text((out/'independent_counterexamples.rs').read_text())
    (p/'Cargo.toml').write_text('[package]\nname="spark-c2-counterexamples"\nversion="0.0.0"\nedition="2021"\n[workspace]\n[dependencies]\n'+''.join(f'{name} = {{ path = "{root}/crates/{name}" }}\n' for name in ['spark-core','spark-engine','spark-testkit']))
    cmd=['cargo','test','--offline','--','--nocapture']
    with (out/'counterexamples.txt').open('w') as log:
        log.write('$ '+' '.join(cmd)+'\n'); log.flush()
        result=subprocess.run(cmd,cwd=p,stdout=log,stderr=subprocess.STDOUT)
    print(json.dumps({'command':cmd,'exit_code':result.returncode,'expected':'seven contract assertions fail; external-construction and two positive controls pass'}))

    text=(out/'counterexamples.txt').read_text()
    assert result.returncode == 101 and '3 passed; 7 failed; 0 ignored' in text, text
    (out/'counterexamples.json').write_text(json.dumps({'exit_code':result.returncode,'contract_failures':7,'passing_probes':3,'candidate':'053d1dc1131ec47be94b60513fad9ea8389cde0c','candidate_source_modified':False},indent=2)+'\n')
