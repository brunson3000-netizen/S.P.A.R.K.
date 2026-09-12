"""Independent reviewer mutants, each isolated in an archive of the exact final commit.
Own hand-valued probes run positively before assertion-failure kills.
D4 and D6 repeat useful wrong implementations with the new probe corpus.
"""
import hashlib, json, os, subprocess, tarfile, tempfile
from pathlib import Path
out = Path(__file__).resolve().parent
root = out.parents[2]
PIN = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip()
PROBES = out / 'own_probes.rs'
ENV = {**os.environ, 'CARGO_PROFILE_DEV_DEBUG': '0', 'CARGO_PROFILE_TEST_DEBUG': '0',
       'CARGO_INCREMENTAL': '0', 'CARGO_BUILD_JOBS': '2'}
engine = 'crates/spark-engine/src/engine.rs'
cases = [('D1-double-anchor-endpoint', 'crates/spark-engine/src/engine.rs', 'let from = anchor.0.max(begin);', 'let from = anchor.0.saturating_sub(1).max(begin);', 'own_d1_d2_unaligned_write_and_partition'), ('D2-body-rephases', 'crates/spark-engine/src/engine.rs', 'let anchor = cell.map(|c| c.updated_at).unwrap_or(ctx.now);', 'let anchor = ctx.now;', 'own_d2_body_and_command_shock'), ('D3-ignore-cadence-only-change', 'crates/spark-engine/src/engine.rs', 'if (rate, cadence) == (r, c) => {', 'if rate == r && c > 0 => {', 'own_d3_d4_d5_barrier_endpoint_residual_and_unchanged'), ('D3-keep-old-origin', 'crates/spark-engine/src/engine.rs', '(Some((rate, cadence)), _) => Some((rate, cadence, begin)),', '(Some((rate, cadence)), prior) => Some((rate, cadence, prior.map_or(begin, |(_, _, o)| o))),', 'own_d3_d4_d5_barrier_endpoint_residual_and_unchanged'), ('D4-exclude-old-endpoint', 'crates/spark-engine/src/engine.rs', '.map_or(now.0, |a| a.0.min(now.0));', '.map_or(now.0, |a| a.0.saturating_sub(1).min(now.0));', 'own_d3_d4_d5_barrier_endpoint_residual_and_unchanged'), ('D5-charge-prebarrier', 'crates/spark-engine/src/engine.rs', '(Some((rate, cadence)), _) => Some((rate, cadence, begin)),', '(Some((rate, cadence)), _) => Some((rate, cadence, begin.saturating_sub(1))),', 'own_d3_d4_d5_barrier_endpoint_residual_and_unchanged'), ('D6-next-tick', 'crates/spark-engine/src/engine.rs', '        let epoch = self.behavior_epoch();\n        // Every committed effect is written', '        let epoch = self.behavior_epoch();\n        let now = LogicalTime(now.0 + 1);\n        // Every committed effect is written', 'own_d6_d7_nonzero_baseline_saturation_and_zero'), ('D7-overshoot-baseline', 'crates/spark-engine/src/effects.rs', 'let moved = distance.min(movement);', 'let _ = distance;\n    let moved = movement;', 'own_d6_d7_nonzero_baseline_saturation_and_zero'), ('open-skip-zero-length-epoch', 'crates/spark-engine/src/engine.rs', '            segment = match (parameters, segment) {', '            if begin == end { continue; }\n            segment = match (parameters, segment) {', 'own_open_same_time_activations')]

def normalized(s): return '\n'.join(l.rstrip() for l in s.rstrip().splitlines()) + '\n'
results = {'tree': PIN, 'probe_source': str(PROBES.relative_to(root)), 'baseline': [], 'mutants': []}
with tempfile.TemporaryDirectory(prefix='spark-c2-adjudication-reviewer-mutants-') as temp:
    temp = Path(temp)
    archive = temp / 'source.tar'
    with archive.open('wb') as f: subprocess.run(['git', 'archive', PIN], cwd=root, stdout=f, check=True)
    tree = temp / 'tree'; tree.mkdir()
    with tarfile.open(archive) as t: t.extractall(tree, filter='data')
    crate = temp / 'probe'; (crate / 'src').mkdir(parents=True)
    source = PROBES.read_bytes()
    (crate / 'src/lib.rs').write_bytes(source)
    results['probe_sha256'] = hashlib.sha256(source).hexdigest()
    (crate / 'Cargo.lock').write_bytes((root / 'Cargo.lock').read_bytes())
    (crate / 'Cargo.toml').write_text('[package]\nname="reviewer-decay-probes"\nversion="0.0.0"\nedition="2021"\n[workspace]\n[dependencies]\n' + ''.join(
        f'{n} = {{ path="{tree}/crates/{n}"' + (', features=["test-support"]' if n == 'spark-engine' else '') + ' }\n'
        for n in ['spark-core', 'spark-engine', 'spark-testkit']))
    def run(name, label):
        cmd = ['cargo', 'test', '--offline', '--', '--exact', name, '--nocapture']
        p = subprocess.run(cmd, cwd=crate, env=ENV, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        log = p.stdout
        compiled = 'Running unittests' in log
        result = {'name': name, 'exit_code': p.returncode, 'compiled': compiled,
                  'passed': p.returncode == 0 and '1 passed' in log,
                  'assertion_failure': p.returncode == 101 and compiled and 'panicked at' in log}
        (out / ('own-mutant-' + label + '.txt')).write_text('# tree: ' + PIN + '\n$ ' + ' '.join(cmd) + '\n' + normalized(log))
        return result
    for label, file, old, new, name in cases:
        result = run(name, 'baseline-' + label); results['baseline'].append(result)
    for label, file, old, new, name in cases:
        path = tree / file; original = path.read_text(); assert original.count(old) == 1, (label, original.count(old))
        path.write_text(original.replace(old, new))
        result = run(name, label); results['mutants'].append(dict(id=label, **result))
        path.write_text(original)
        print(label, result, flush=True)
results['baseline_all_pass'] = all(x['passed'] for x in results['baseline'])
results['all_killed'] = all(x['assertion_failure'] for x in results['mutants'])
(out / 'own-mutations.json').write_text(json.dumps(results, indent=2) + '\n')
print(json.dumps({'baseline_all_pass': results['baseline_all_pass'], 'all_killed': results['all_killed']}))
