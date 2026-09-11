#!/usr/bin/env python3
"""Run isolated review probes without editing the candidate or production Rust."""
import contextlib
import copy
import io
import json
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
MODEL = HERE.parent / 'v3_f01_final_correction_model_2026-09-10/model.py'
with tempfile.TemporaryDirectory(prefix='spark-review-') as tmp:
    tmp = Path(tmp)
    (tmp / 'src').mkdir()
    (tmp / 'src/main.rs').write_bytes((HERE / 'source_sequence_regression.rs').read_bytes())
    (tmp / 'Cargo.toml').write_text(
        '[package]\nname="spark-review-probe"\nversion="0.0.0"\nedition="2021"\n'
        '[dependencies]\nspark-core={path=' + json.dumps(str(ROOT / 'crates/spark-core')) + '}\n')
    # Retain the repository dependency versions in this disposable package.
    (tmp / 'Cargo.lock').write_bytes((ROOT / 'Cargo.lock').read_bytes())
    cmd = ['cargo', 'run', '--offline', '--quiet', '--manifest-path', str(tmp / 'Cargo.toml')]
    subprocess.run(cmd, check=True)
    red = subprocess.run(cmd + ['--', '--assert-candidate'], capture_output=True, text=True)
    assert red.returncode != 0 and 'candidate refusal atomicity is false' in red.stderr, red.stderr
    print('CONFIRMED: candidate byte-identical timeline assertion fails (expected red control)')
    old_argv = sys.argv
    sys.argv = [str(MODEL), str(tmp / 'model.json')]
    with contextlib.redirect_stdout(io.StringIO()):
        m = runpy.run_path(str(MODEL))
    sys.argv = old_argv
    results = json.loads((tmp / 'model.json').read_text())['checks']
    assert len(results) == 34 and all(v.startswith('PASS') for v in results.values())
    print('PASS: candidate disposable model, 34/34 checks')

    # Faulty presentation of a different request while the real active head is retained.
    e = m['fresh'](1)
    active = m['Advance'](20)
    mb = m['Mailbox'](4)
    mb.offer(active)
    assert e.process(active) == 'PAUSED'
    before = copy.deepcopy(e.snapshot())
    result = e.process(m['Command'](12, 'substitute'))
    assert result == 'REFUSED_ACTIVE_REQUEST_MISMATCH' and e.snapshot() == before
    assert mb.head() is active
    if result in m['TERMINAL']:  # candidate Next row / consumer rule
        mb.pop()
    assert mb.head() is None and e.active == active.discriminator()
    print('CONFIRMED: candidate terminal-pop rule loses actual active FIFO head on mismatch')

    # Two reachable boundaries have the same history and F, but different progress.
    paused = m['fresh'](1)
    assert paused.process(active) == 'PAUSED'
    replay = m['fresh'](1)
    assert replay.process(m['Advance'](paused.F)) == 'COMPLETED'
    assert replay.timeline == paused.timeline and replay.F == paused.F == 0
    assert replay.engine_state_digest() != paused.engine_state_digest()
    assert replay.stable_boundary_digest() != paused.stable_boundary_digest()
    status, restored = m['Engine'].restore(m['rules'], 1, paused.snapshot())
    assert status == 'OK'
    assert restored.process(m['Command'](20, 'different')) == 'REFUSED_ACTIVE_REQUEST_MISMATCH'
    while paused.process(active) == 'PAUSED':
        pass
    while restored.process(active) == 'PAUSED':
        pass
    assert restored.snapshot() == paused.snapshot()
    print('CONFIRMED: history + F cannot reconstruct pause; snapshot + exact request resumes equivalently')
print('ALL REVIEW PROBES PASS (confirmed defects, not candidate acceptance)')
