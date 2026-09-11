import subprocess, json, time
from pathlib import Path
root=Path(__file__).resolve().parents[3]
out=Path(__file__).resolve().parent
checks=[('fmt','cargo fmt --all --check'),('workspace-tests','cargo test --workspace --all-features'),('clippy','cargo clippy --workspace --all-targets --all-features -- -D warnings'),('strict','cargo clippy -p spark-core -p spark-engine --lib --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::indexing_slicing -D clippy::arithmetic_side_effects'),('metadata','cargo metadata --format-version 1'),('whitespace','git diff --check 7e3a0aae069a8bf840e4dfb74221cdf2687b1db5 053d1dc1131ec47be94b60513fad9ea8389cde0c')]
checks += [(t,'cargo check --workspace --all-targets --target '+t) for t in ['x86_64-pc-windows-gnu','x86_64-pc-windows-msvc','aarch64-linux-android','armv7-linux-androideabi','x86_64-linux-android']]
checks += [('workload','cargo run --release --offline -p spark-testkit --example gate_c2_workload')]
results=[]
for name, cmd in checks:
    start=time.time()
    with (out/(name+'.txt')).open('w') as f:
        f.write('$ '+cmd+'\n'); f.flush()
        r=subprocess.run(cmd,shell=True,cwd=root,stdout=f,stderr=subprocess.STDOUT)
    results.append(dict(name=name,command=cmd,exit_code=r.returncode,seconds=round(time.time()-start,3)))
    (out/'checks.json').write_text(json.dumps(results,indent=2)+'\n')
    print(results[-1],flush=True)
