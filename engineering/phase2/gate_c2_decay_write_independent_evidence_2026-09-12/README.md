# Independent review evidence

Exact candidate: cc3dc70182b428f7cbc953ae722f85b39d407dfa. All validation preceded
creation of the review branch; candidate source was never edited.

- run_validation.py / checks.json / named logs: required checks on exact HEAD.
  metadata-full.json additionally preserves stdout of `cargo metadata --format-version 1`
  (exit 0); the copied validation script redirects this command's stdout.
- check_preservation.py / preservation.json: original writer preservation checker rerun;
  audit.py / custody-audit.json independently verifies lineage, code-tree equality,
  the original probe blob and exact one-literal adaptation, plus original worktrees.
- run_corpus.py: external disposable crate, SHA-256 source identity in each log.
  corpora-results.json distinguishes capture-wrapper exit from actual cargo exit.
- own_probes.rs: reviewer-authored tests using the writer's public-door Rig fixture,
  with new identity-transform command rules. Test assertions/vectors are independent.
  own-probes.txt: 7 passes and 2 current blocking assertion failures, cargo 101.
- own_materialized.rs: same fixture with a public materializing schedule; 1 pass,
  both signs, cargo 0. See own-materialized.txt.
- run_mutation_controls.py: writer's 17 patch definitions unchanged; final rerun uses
  isolated build targets per mutant, records baseline output as well as mutant output.
  mutation-controls.json records actual cargo results. Baseline logs name exact commands;
  mutant commands are the runner's `cargo test --offline -p spark-testkit --all-features
  --test <file> -- --exact <name>` for each named test. No compile failures count as kills.
- run_own_mutants.py / own-mutations.json: ten controls using reviewer vectors, with
  isolated targets and full baseline/mutant commands and output. Useful writer patch
  shapes are reused explicitly; Clamp and absent-cell controls are additional patches.
- run_own_atomic_mutant.py / own-atomic-mutations.json: an additional cross-family
  rejection bypass control. It must compile and fail the independent atomic-refusal test.
- validation-summary.json: checked final totals. SHA256SUMS covers published evidence
  except itself. Publication receipt is supplied after commit/push in the final response.

External corpora were run with the exact-candidate repository root, via:
`python3 E/run_corpus.py . <log-name> <source-path> --test-support`.
The log/source pairs are historical-own → the controlling review's own_probes.rs;
resolution-adapted → writer's own_probes_resolution_adapted.rs; decision-adapted →
controlling review's independent_probes_decision_adapted.rs; own-probes → E/own_probes.rs;
own-materialized → E/own_materialized.rs. Source paths and cargo results for prior corpora
are also recorded in corpora-results.json. E is this directory.

The unmodified historical corpus's cargo 101 failure is the superseded assertion, not a
normative pass. The two independent composition failures are current blockers. Standalone
Scale/Clamp observations are scoped characterization tests, not authority to exempt
additive stages. Initial shared-target reviewer mutation results were discarded after
baseline contamination was detected; final counts require every isolated baseline to pass.
