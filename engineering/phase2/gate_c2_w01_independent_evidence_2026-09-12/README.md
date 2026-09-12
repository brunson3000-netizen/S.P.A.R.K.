# Independent C2W-01 correction review evidence

Exact candidate: `67b877192cc78b75c6fbe60c69b5594dc10befe8`.
Validation ran in a fresh clone/main worktree, detached at this exact candidate until
review publication. Candidate source was not repaired.

The mission requires a third reviewer if available. `/root/independent_c2w01` is the
separate substantive reviewer, independent of the writer and the original C2W-01 finding
author. `/root` (the earlier finding author) coordinates custody, standard validation,
writer-mutant/corpus reruns and publication. The report identifies this division explicitly;
coordinator checks do not substitute for the third agent's independent judgment.

- `audit.py`, `custody-audit.json`: independent Git-object comparisons, live heads, full
  lineage, checkpoint/final crate-tree equality, 782 immutable inherited engineering blobs,
  contract-first chronology, and 16 unchanged mutant patch tuples.
- `original-custody.json`, `live-heads-before.txt`: existing worktree/ref/status and live
  remote heads captured before validation. Publication verifies preservation again.
- `run_validation.py`, `checks.json`, named logs: all required checks, exact commands and
  actual exits. The writer runner was copied with metadata stdout retained rather than
  redirected. `check_preservation.py` was rerun unchanged in this evidence directory.
- `run_prior_corpora.py`, `corpora-results.json`, corpus logs: historical sources are read
  byte-unchanged into disposable external crates; source SHA-256 heads each log. Wrapper
  status is distinct from the embedded cargo result. Historical adjudication's one lost-debt
  assertion remains a superseded failure; it is not counted as a normative pass.
- `run_mutation_controls.py`, baseline/mutant logs and `mutation-controls.json`: writer
  patch definitions retained, runner changed only to save full baseline output and use a
  target directory per mutant archive. Exact commands are defined in run_test and printed
  in baseline logs. Every kill requires a compiled test and assertion panic; expected
  prior-test survivals are recorded separately.
- `own_*`: the third reviewer's authored probes, mutant controls, logs and assessment.
  These are review evidence, not product repairs or an adversarial campaign.

Mutation naming caveat: `W01-settle-every-body` retains the no-Decay guard. It is not alone
equivalent to the old unconditional `RES-settle-composed-body`. The separate
`W01-settle-decay-body` supplies the explicit-decay double-charge control. The review
assesses their combined coverage rather than silently accepting a one-for-one equivalence.

The final publication receipt provides the review commit and local/tracking/live equality.
No Gate C2 acceptance, production promotion or Phase 3 is performed. A KEEP verdict leads
to the separately recorded adversarial campaign; that campaign is not part of this review.

Published .log output is whitespace-normalized (trailing spaces/blank lines removed);
commands, diagnostics and test outcomes are otherwise retained. Initial fixture-authoring
logs are retained but are not counted as candidate validation or mutation kills.
