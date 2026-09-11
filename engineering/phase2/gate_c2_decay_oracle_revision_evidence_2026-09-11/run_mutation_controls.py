"""Compiled wrong-implementation controls for the second Gate C2 correction.

Each mutant is a disposable `git archive HEAD` copy of the candidate with one
named wrong implementation patched in (every patch must match exactly once).
For each mutant the runner executes, one test at a time:

- `kills`: tests that must FAIL on the mutant by an assertion (the mutant
  compiles and a test panics) — a real wrong-implementation kill, not a
  compiler failure relabelled;
- `survives`: prior tests that PASS on the mutant, showing the gap the new test
  closes (only where the review identified such a gap).

Every listed test is first run on the unmutated archive and must pass. Results
go to mutation-controls.json; each mutant's output to mutant-<id>.txt
(whitespace-normalized). Nothing in the repository is modified.
"""
import json, os, subprocess, tempfile
from pathlib import Path

out = Path(__file__).resolve().parent
root = out.parents[2]
ENGINE = 'crates/spark-engine/src/engine.rs'
D = 'phase2_decay_oracle_revision'
B = 'phase2_bounded_revision'
O = 'phase2_oracle_completion'

MUTANTS = [
    dict(id='C2R-01-backdated-commit',
         wrong='commit at the end of the last whole step (instantiated for the cadence-10 fixtures by flooring the write time to a multiple of 10)',
         patches=[(ENGINE,
                   "        let epoch = self.behavior_epoch();\n        // Every committed effect is written",
                   "        let epoch = self.behavior_epoch();\n        let now = LogicalTime(now.0 - now.0 % 10);\n        // Every committed effect is written")],
         kills=[(D, 'c2r_01_decay_reapplied_at_canonical_time_reproduces_the_state')],
         survives=[]),
    dict(id='C2-05-relative-steps',
         wrong='count whole steps from updated_at ((now - updated_at) / cadence) while committing at now: the original lost remainder',
         patches=[(ENGINE,
                   "            let steps = index(end)\n                .zip(index(from))\n                .and_then(|(e, f)| e.checked_sub(f))\n                .ok_or(arithmetic(\"decay\"))?;",
                   "            let _ = index;\n            let steps = (end - from) / cadence;")],
         kills=[(D, 'c2r_02_03_grid_matches_the_reference_model_under_every_partition'),
                (B, 'c2_05_decay_preserves_elapsed_cadence_remainder_converted')],
         survives=[]),
    dict(id='C2R-02-absolute-grid',
         wrong='one grid from logical time 0 for every segment: a new rate/cadence step may contain pre-barrier time',
         patches=[(ENGINE,
                   "                (Some((rate, cadence)), _) => Some((rate, cadence, begin)),",
                   "                (Some((rate, cadence)), _) => Some((rate, cadence, 0)),")],
         kills=[(D, 'c2r_02_lengthened_cadence_and_residual_interval'),
                (D, 'c2r_02_rate_only_change_at_an_unaligned_barrier')],
         survives=[]),
    dict(id='C2R-02-reset-every-activation',
         wrong='restart the grid at every activation, even one leaving the parameters unchanged',
         patches=[(ENGINE,
                   "if (rate, cadence) == (r, c) => {\n                    Some((rate, cadence, origin))",
                   "if (rate, cadence) == (r, c) => {\n                    let _ = origin;\n                    Some((rate, cadence, begin))")],
         kills=[(D, 'c2r_02_unchanged_parameters_do_not_move_the_grid')],
         survives=[]),
    dict(id='C2R-03-skip-unmoved',
         wrong='emit no decay candidate when the value does not move (saturated, rate 0, or no whole step)',
         patches=[(ENGINE,
                   "                    Some(Intent::Transform {\n                        family: crate::rules::TransformFamily::Decay,",
                   "                    if moved == i128::from(value) {\n                        return Ok(None);\n                    }\n                    Some(Intent::Transform {\n                        family: crate::rules::TransformFamily::Decay,")],
         kills=[(D, 'c2r_03_saturation_preserves_value_and_commit_time_chunk_invariance'),
                (D, 'c2r_03_every_evaluation_commits_at_its_canonical_time')],
         survives=[]),
    dict(id='AT-I8-deferred-obligation-insert',
         wrong='insert each wave\'s obligation records into the ObligationStore only at the end of the cohort',
         patches=[(ENGINE,
                   "        let mut wave: u32 = 0;\n        loop {\n            let prewave = self.engine_state_digest();",
                   "        let mut wave: u32 = 0;\n        let mut deferred: Vec<ObligationRecord> = Vec::new();\n        loop {\n            let prewave = self.engine_state_digest();"),
                  (ENGINE,
                   "                    let report = self.apply_wave(cohort, now, wave, &prewave, plan);",
                   "                    deferred.extend(plan.records.clone());\n                    let report = self.apply_wave(cohort, now, wave, &prewave, plan);"),
                  (ENGINE,
                   "                    if next.is_empty() || converted {\n                        return (waves, CohortOutcome::Committed);",
                   "                    if next.is_empty() || converted {\n                        for r in deferred.drain(..) {\n                            self.obligations.insert(r);\n                        }\n                        return (waves, CohortOutcome::Committed);"),
                  (ENGINE,
                   "            self.obligations.insert(r.clone());\n        }\n        let batch",
                   "        }\n        let batch")],
         kills=[(D, 'at_i8_invariant_holds_after_each_intermediate_committed_wave')],
         survives=[(O, 'at_i8_invariant_holds_after_every_committed_wave')]),
    dict(id='AT-I20b-tombstone-producer',
         wrong='after a semantic-cap rejection, exhaust the rejected producers\' occurrence sequences (the producer can never be rescheduled)',
         patches=[(ENGINE,
                   "        let (waves, outcome) = self.run_waves(&cohort, extraction.due_time, seeds);\n",
                   "        let (waves, outcome) = self.run_waves(&cohort, extraction.due_time, seeds);\n"
                   "        if matches!(outcome, CohortOutcome::Rejected { rejection: WaveRejection::SemanticCap { .. }, .. }) {\n"
                   "            for (item, _) in &extraction.executable {\n"
                   "                self.occurrences.set_next(OccurrenceLedgerKey { profile_id: item.key.profile_id.clone(), producer: item.key.producer_definition_id.clone(), scope_id: item.key.scope_id.clone(), work_kind: item.key.work_kind.clone() }, u64::MAX);\n"
                   "            }\n"
                   "        }\n")],
         kills=[(D, 'at_i20b_rejected_producer_retries_under_its_next_occurrence')],
         survives=[(O, 'at_i20b_every_semantic_cap_is_atomic_terminal_replayable_and_retryable')]),
]

def normalize(text):
    return '\n'.join(line.rstrip() for line in text.rstrip().splitlines()) + '\n'

head = subprocess.run(['git', 'rev-parse', 'HEAD'], cwd=root, capture_output=True, text=True, check=True).stdout.strip()
results = dict(tree=head, baseline=[], mutants=[])
with tempfile.TemporaryDirectory(prefix='spark-c2-rev2-mutants-') as tmp:
    tmp = Path(tmp)
    env = {**os.environ, 'CARGO_TARGET_DIR': str(tmp / 'target')}

    def archive(name):
        dst = tmp / name
        dst.mkdir()
        subprocess.run(f'git -C {root} archive HEAD | tar -x -C {dst}', shell=True, check=True)
        return dst

    def run_test(tree, test_file, name):
        cmd = ['cargo', 'test', '--offline', '-p', 'spark-testkit', '--all-features',
               '--test', test_file, '--', '--exact', name]
        r = subprocess.run(cmd, cwd=tree, env=env, capture_output=True, text=True)
        text = r.stdout + r.stderr
        compiled = 'Running tests/' in text
        panicked = 'panicked at' in text
        return dict(test=f'{test_file}::{name}', exit_code=r.returncode, compiled=compiled,
                    passed=r.returncode == 0 and compiled and 'test result: ok. 1 passed' in text,
                    assertion_failure=compiled and panicked and r.returncode != 0), text

    base = archive('baseline')
    tests = sorted({t for m in MUTANTS for t in m['kills'] + m['survives']})
    for t in tests:
        res, _ = run_test(base, *t)
        results['baseline'].append(res)
        print('baseline', res, flush=True)
    for m in MUTANTS:
        tree = archive(m['id'])
        for f, old, new in m['patches']:
            path = tree / f
            text = path.read_text()
            assert text.count(old) == 1, (m['id'], f, old[:60])
            path.write_text(text.replace(old, new))
        log = [f"# mutant {m['id']}: {m['wrong']}\n# tree: {head}\n"]
        entry = dict(id=m['id'], wrong=m['wrong'], kills=[], survives=[])
        for t in m['kills']:
            res, text = run_test(tree, *t)
            res['killed'] = res['assertion_failure']
            entry['kills'].append(res)
            log.append(f"$ kills {res['test']}\n{text}")
        for t in m['survives']:
            res, text = run_test(tree, *t)
            entry['survives'].append(res)
            log.append(f"$ survives {res['test']}\n{text}")
        entry['verdict'] = ('KILLED' if all(k['killed'] for k in entry['kills']) and
                            all(s['passed'] for s in entry['survives']) else 'NOT_AS_CLAIMED')
        (out / f"mutant-{m['id']}.txt").write_text(normalize('\n'.join(log)))
        results['mutants'].append(entry)
        print(m['id'], entry['verdict'], flush=True)
results['baseline_all_pass'] = all(b['passed'] for b in results['baseline'])
results['all_killed_as_claimed'] = all(m['verdict'] == 'KILLED' for m in results['mutants'])
(out / 'mutation-controls.json').write_text(json.dumps(results, indent=2) + '\n')
print(json.dumps({'baseline_all_pass': results['baseline_all_pass'],
                  'all_killed_as_claimed': results['all_killed_as_claimed']}))
