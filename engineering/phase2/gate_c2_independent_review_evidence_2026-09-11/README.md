# Gate C2 independent review evidence

Candidate: `053d1dc1131ec47be94b60513fad9ea8389cde0c`.
Verdict: `GATE_C2_BOUNDED_REVISION_REQUIRED`.

- `checks.json`, `run_validation.py`, and named logs: fresh prescribed validation.
- `preservation.json`, `preservation.txt`, `check_preservation.py`: exact lineage,
  unchanged inherited test files/modules and four protected functions, manifests/lock,
  final documentation scope, preserved worktrees and live candidate/production refs.
- `independent_counterexamples.rs`, `run_counterexamples.py`, `counterexamples.txt`,
  `counterexamples.json`: external probes using unchanged candidate path dependencies
  and a copy of its lockfile. Seven assertions of required behavior fail. Three probes
  pass: two positive semantic/encoding controls and the external construction that the
  oracle says must not compile. Exit 101 is deliberately retained and checked, not hidden.

The Python probe runner exits successfully only if the expected 3-pass/7-fail result is
observed; the Rust failure output is the review evidence. The runner creates and removes
only its own disposable temporary crate. It does not modify candidate source or tests.

All paths in logs identify the isolated review checkout or the disposable crate. Timing
is one-machine evidence. Static targets are not executable platform parity. The review
contains the findings, per-entry oracle disposition and exact next action. Publication
hash and post-push live synchronization are supplied in the final receipt, since this
commit cannot record its own hash.

The workspace-test log has only its extra terminal blank line removed for Git whitespace hygiene; command output and results are otherwise preserved.
