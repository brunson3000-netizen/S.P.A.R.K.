"""Compiled wrong-implementation controls for the Gate C2 decay-write resolution pass.

Each mutant is a disposable `git archive HEAD` copy of the candidate with one named
wrong implementation patched in (every patch must match exactly once).

The first ten are the preceding adjudication pass's mutants with byte-identical
patches, re-verified against this tree: the fixed-grid walk is reused unchanged by
settlement, so every D-1 ... D-7 discriminator must still kill them. The superseded
elapsed-since-write mutant (`C2-05-relative-steps`) is additionally required to be
killed by the new resolution vectors.

The last seven are new, and are exactly the wrong implementations of the Operator's
2026-09-12 resolution that the report claims are ruled out: forfeiting the pre-write
debt (the superseded reading), letting earlier decay reduce an explicit replacement,
comparing a watcher's threshold against the settled intermediate, implicitly settling
a composed rule body or a `Scale`, resolving the settlement operation from the current
epoch only, and excluding the grid point that ends exactly at the write.

For each mutant the runner executes, one test at a time:
- `kills`: tests that must FAIL on the mutant by an assertion (the mutant compiles and a
  test panics). This is a real wrong-implementation kill, not a relabelled compile failure;
- `survives`: prior tests that PASS on the mutant, where the review identified such a gap.

Every listed test is first run on the unmutated archive and must pass. Results go to
mutation-controls.json; each mutant's output to mutant-<id>.txt (whitespace-normalized).
Nothing in the repository is modified.
"""
import json, os, subprocess, tempfile
from pathlib import Path

out = Path(__file__).resolve().parent
root = out.parents[2]
ENGINE = 'crates/spark-engine/src/engine.rs'
A = 'phase2_decay_adjudication'
D = 'phase2_decay_oracle_revision'
B = 'phase2_bounded_revision'
O = 'phase2_oracle_completion'
FRESH = (A, 'd2_fresh_assignment_keeps_the_fixed_grid')
SHOCK = (A, 'd2_post_shock_decay_keeps_the_fixed_grid')
BOUNDARY = (A, 'd3_d4_step_ending_at_activation_belongs_to_the_closing_segment')
RESIDUAL = (A, 'd3_residual_interval_is_discarded_at_a_parameter_change')
SHORT = (A, 'd3_d5_shortened_cadence_starts_a_new_grid_at_the_barrier')
LONG = (A, 'd3_d5_lengthened_cadence_starts_a_new_grid_at_the_barrier')
SATURATION = (A, 'd6_d7_saturation_and_unmoved_evaluations_commit_at_canonical_time')
CHUNK = (A, 'd1_chunking_across_unaligned_writes_and_barriers')
SWEEP = (A, 'd_generated_timelines_with_writes_match_the_reference_model')

MUTANTS = [
    dict(id='C2R-01-backdated-commit',
         wrong='commit at the end of the last whole step (instantiated for the cadence-10 fixtures by flooring the write time to a multiple of 10)',
         patches=[(ENGINE,
                   "        let epoch = self.behavior_epoch();\n        // Every committed effect is written",
                   "        let epoch = self.behavior_epoch();\n        let now = LogicalTime(now.0 - now.0 % 10);\n        // Every committed effect is written")],
         kills=[(D, 'c2r_01_decay_reapplied_at_canonical_time_reproduces_the_state'), FRESH, SATURATION],
         survives=[]),
    dict(id='C2-05-relative-steps',
         wrong='the superseded elapsed-since-write count: whole steps (now - updated_at) / cadence per segment, committing at now',
         patches=[(ENGINE,
                   "            let steps = index(end)\n                .zip(index(from))\n                .and_then(|(e, f)| e.checked_sub(f))\n                .ok_or(arithmetic(\"decay\"))?;",
                   "            let _ = index;\n            let steps = (end - from) / cadence;")],
         kills=[(D, 'c2r_02_03_grid_matches_the_reference_model_under_every_partition'),
                (B, 'c2_05_decay_preserves_elapsed_cadence_remainder_converted'),
                FRESH, SHOCK, SHORT, CHUNK, SWEEP,
                ('phase2_decay_write_resolution', 'settlement_counts_exactly_the_grid_points_that_have_ended'),
                ('phase2_decay_write_resolution', 'removal_suspends_accrual_and_restoration_starts_a_fresh_grid')],
         survives=[]),
    dict(id='C2R-02-absolute-grid',
         wrong='one grid from logical time 0 for every segment: a new rate/cadence step may contain pre-barrier time',
         patches=[(ENGINE,
                   "                (Some((rate, cadence)), _) => Some((rate, cadence, begin)),",
                   "                (Some((rate, cadence)), _) => Some((rate, cadence, 0)),")],
         kills=[(D, 'c2r_02_lengthened_cadence_and_residual_interval'),
                (D, 'c2r_02_rate_only_change_at_an_unaligned_barrier'),
                RESIDUAL, SHORT, LONG],
         survives=[]),
    dict(id='C2R-02-reset-every-activation',
         wrong='restart the grid at every activation, even one leaving the parameters unchanged',
         patches=[(ENGINE,
                   "if (rate, cadence) == (r, c) => {\n                    Some((rate, cadence, origin))",
                   "if (rate, cadence) == (r, c) => {\n                    let _ = origin;\n                    Some((rate, cadence, begin))")],
         kills=[(D, 'c2r_02_unchanged_parameters_do_not_move_the_grid'), LONG],
         survives=[]),
    dict(id='C2R-03-skip-unmoved',
         wrong='emit no decay candidate when the value does not move (saturated, rate 0, or no whole step)',
         patches=[(ENGINE,
                   "                    Some(Intent::Transform {\n                        family: crate::rules::TransformFamily::Decay,",
                   "                    if moved == i128::from(value) {\n                        return Ok(None);\n                    }\n                    Some(Intent::Transform {\n                        family: crate::rules::TransformFamily::Decay,")],
         kills=[(D, 'c2r_03_saturation_preserves_value_and_commit_time_chunk_invariance'),
                (D, 'c2r_03_every_evaluation_commits_at_its_canonical_time'),
                FRESH, SATURATION],
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
    # New: the rejected alternatives to D-3 and D-4.
    dict(id='ADJ-carried-residual-origin',
         wrong='carry the closed segment\'s phase into the new grid (new origin = last old grid point), so the first new-parameter step contains pre-barrier residual time',
         patches=[(ENGINE,
                   "                (Some((rate, cadence)), _) => Some((rate, cadence, begin)),",
                   "                (Some((rate, cadence)), previous) => Some((rate, cadence, previous.map_or(begin, |(_, c, o)| o + (begin - o) / c * c))),")],
         kills=[RESIDUAL, LONG],
         survives=[]),
    dict(id='ADJ-residual-billed-at-old-rate',
         wrong='bill the unfinished residual of every closed epoch as one whole old-rate step (round the closing endpoint up to the next old grid point)',
         patches=[(ENGINE,
                   "            let steps = index(end)\n                .zip(index(from))",
                   "            let closing = epochs.peek().and_then(|n| n.activated_at).is_some_and(|a| a.0 <= now.0);\n"
                   "            let end = if closing { end + cadence - 1 } else { end };\n"
                   "            let steps = index(end)\n                .zip(index(from))")],
         kills=[RESIDUAL, LONG],
         survives=[]),
    dict(id='ADJ-drop-closing-endpoint',
         wrong='exclude the step ending exactly at a parameter-changing activation from the closing segment (the reviewer\'s own mutant, retargeted)',
         patches=[(ENGINE,
                   ".map_or(now.0, |a| a.0.min(now.0));",
                   ".map_or(now.0, |a| a.0.saturating_sub(1).min(now.0));")],
         kills=[BOUNDARY],
         survives=[]),
]

W = 'phase2_decay_write_resolution'
EFFECTS = 'crates/spark-engine/src/effects.rs'
SETTLE = (W, 'an_additive_write_settles_overdue_decay_before_applying_its_delta')
NEGATIVE = (W, 'additive_settlement_kills_the_former_lost_debt_reading')
REPLACEMENT = (W, 'an_explicit_replacement_is_never_reduced_by_earlier_decay')
ENDED = (W, 'settlement_counts_exactly_the_grid_points_that_have_ended')
REPEATED = (W, 'repeated_and_co_firing_additions_never_charge_a_grid_point_twice')
SATURATE = (W, 'settlement_saturates_at_the_baseline_and_is_inert_at_rate_zero')
REMOVE = (W, 'removal_suspends_accrual_and_restoration_starts_a_fresh_grid')
SAMETIME = (W, 'same_time_activations_are_processed_in_committed_order')
BARRIER_ONCE = (W, 'settlement_bills_the_barrier_endpoint_once_at_the_old_rate')
ABSENT = (W, 'a_decay_evaluation_never_creates_an_absent_cell')
BODY = (W, 'a_composed_body_folds_its_declared_stages_in_order')
SCALE = (W, 'scale_keeps_its_existing_semantics')
WATCHER = (W, 'a_watcher_sees_one_committed_transition_and_the_reported_effect_matches')
ATOMIC = (W, 'a_refused_wave_settles_nothing')

RESOLUTION_MUTANTS = [
    dict(id='RES-lost-debt',
         wrong='the superseded reading: an additive write folds its delta onto the raw committed value, forfeiting every grid step that ended before it',
         patches=[(ENGINE,
                   "        let reduced = reduce(&canonical, &|d, s| settled.get(&(d, s)).copied())?;",
                   "        let _ = &settled;\n        let reduced = reduce(&canonical, &|d, s| view.cell(d, s))?;")],
         kills=[SETTLE, NEGATIVE, ENDED, REPEATED, SATURATE, REMOVE, BARRIER_ONCE, SWEEP],
         survives=[]),
    dict(id='RES-decay-reduces-replacement',
         wrong='earlier decay reduces an explicit replacement: the reduced RESULT is capped by the settled pre-wave value',
         patches=[(EFFECTS,
                   "                    if values.len() == 1 {\n                        *value",
                   "                    if values.len() == 1 {\n                        (*value).min(pre(&definition, &scope).unwrap_or(*value))")],
         kills=[REPLACEMENT],
         survives=[]),
    dict(id='RES-watcher-sees-settled-intermediate',
         wrong="a crossing watcher compares its threshold against the settled intermediate instead of the committed pre-wave value",
         patches=[(ENGINE,
                   "                        let old = view.cell(&r.definition, &r.scope).unwrap_or(0);",
                   "                        let old = settled.get(&(&r.definition, &r.scope)).copied().unwrap_or(0);")],
         kills=[WATCHER],
         survives=[]),
    dict(id='RES-settle-composed-body',
         wrong="a composed rule body's starting value is implicitly settled, so its declared decay stage has nothing left to apply",
         patches=[(ENGINE,
                   "        let mut v = i128::from(current.unwrap_or(0));",
                   "        let mut v = i128::from(self.settled(target, scope, ctx.now)?.unwrap_or(0));")],
         kills=[BODY],
         survives=[]),
    dict(id='RES-settle-scale',
         wrong='`Scale` resolves from the settled value instead of the committed one',
         patches=[(ENGINE,
                   "                    resolved: scale(current.unwrap_or(0), f.raw())?,",
                   "                    resolved: scale(self.settled(target, scope, ctx.now)?.unwrap_or(0), f.raw())?,")],
         kills=[SCALE],
         survives=[]),
    dict(id='RES-current-epoch-only',
         wrong='the settlement operation is resolved from the current epoch only, so an additive write lands unsettled while the operation is removed',
         patches=[(ENGINE,
                   "        self.lineage\n            .iter()\n            .rev()\n            .find_map(|e| e.rule_set.decay_operation(target))",
                   "        self.rule_set.decay_operation(target)")],
         kills=[REMOVE],
         survives=[]),
    dict(id='RES-exclude-endpoint-at-the-write',
         wrong='settlement excludes the grid point ending exactly at the write time',
         patches=[(ENGINE,
                   "        let moved = self.decay_walk(&op, i128::from(value), cell.updated_at, baseline, now)?;",
                   "        let moved = self.decay_walk(&op, i128::from(value), cell.updated_at, baseline, LogicalTime(now.0.saturating_sub(1)))?;")],
         kills=[ENDED, BARRIER_ONCE],
         survives=[]),
]
MUTANTS = MUTANTS + RESOLUTION_MUTANTS

def normalize(text):
    return '\n'.join(line.rstrip() for line in text.rstrip().splitlines()) + '\n'

head = subprocess.run(['git', 'rev-parse', 'HEAD'], cwd=root, capture_output=True, text=True, check=True).stdout.strip()
results = dict(tree=head, baseline=[], mutants=[])
with tempfile.TemporaryDirectory(prefix='spark-c2-resolution-mutants-') as tmp:
    tmp = Path(tmp)
    env = {**os.environ, 'CARGO_TARGET_DIR': str(tmp / 'target'), 'CARGO_PROFILE_DEV_DEBUG': '0',
           'CARGO_PROFILE_TEST_DEBUG': '0', 'CARGO_INCREMENTAL': '0', 'CARGO_BUILD_JOBS': '2'}

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
        print(m['id'], entry['verdict'], [k['test'] for k in entry['kills'] if not k['killed']], flush=True)
        subprocess.run(['rm', '-rf', str(tree)])
results['baseline_all_pass'] = all(b['passed'] for b in results['baseline'])
results['all_killed_as_claimed'] = all(m['verdict'] == 'KILLED' for m in results['mutants'])
(out / 'mutation-controls.json').write_text(json.dumps(results, indent=2) + '\n')
print(json.dumps({'baseline_all_pass': results['baseline_all_pass'],
                  'all_killed_as_claimed': results['all_killed_as_claimed']}))
