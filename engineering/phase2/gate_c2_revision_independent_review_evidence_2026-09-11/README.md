# Gate C2 corrected candidate — independent evidence

Exact reviewed candidate: `5b7fcf50161a65dc83b4806b513f01c0a4943ea5`.
Reviewer: Codex, separate from the writer. The parent review document controls dispositions.
No candidate source, manifest, lockfile or workspace test is changed by this review.

## Reproduction

Run these scripts from the isolated review worktree:

- `python3 engineering/phase2/gate_c2_revision_independent_review_evidence_2026-09-11/run_validation.py`
  performs all prescribed workspace/lint/metadata/diff/static-target/workload checks and
  original/adapted corpus runs. This script and the original corpus capture/preservation
  helpers are copied from writer evidence and inspected; their outputs here are fresh.
- `run_original_corpus.py <crate-root> <log-name> [<source.rs> --test-support]` creates a
  disposable external crate with path dependencies and the candidate lockfile. The original
  source remains in prior-review evidence. Original pre-correction run used the preserved
  `/tmp/spark-gate-c2-review` at `00d647e` (unchanged original-candidate code).
- `run_surface_probes.py` runs 14 independently authored default-feature external cases,
  including construction attempts for every refusal variant and a matching positive control.
- `run_mutation_controls.py` archives the exact pinned candidate to a disposable directory,
  first passes both new test files unmodified, then applies one explicit wrong implementation
  at a time. All six mutants compile and are killed by actual test assertion failures.
  It restores only its own disposable source files between cases and deletes that archive.
- `capture_custody.py` records local/tracking/live pins and all registered worktree states.
- `check_evidence.py` checks recorded counts, diagnostic surfaces and probe-source hash,
  producing `independent-results.json`. It does not turn candidate failures into success.

The reviewer runtime probe command was:

```sh
python3 engineering/phase2/gate_c2_revision_independent_review_evidence_2026-09-11/run_original_corpus.py /tmp/spark-gate-c2-revision-review independent-probes engineering/phase2/gate_c2_revision_independent_review_evidence_2026-09-11/independent_revision_probes.rs --test-support
```

`independent_revision_probes.rs` contains 14 new tests: 11 passing controls and three
failing assertions (canonical decay write time, shortened-cadence epoch rates, saturated
cell-time chunk equality). Their cargo exit is 101. A preliminary probe had an i64/u64
helper-call typo; corrected before the recorded semantic run. Another preliminary
materialized-identity comparison was corrected to account for the changed later epoch
hash (activation binds the preceding fence); the final probe independently recomputes the
correct creator-fingerprint formula instead. The surface harness also corrected its diagnostic spelling for unit variants
(`non_exhaustive`, versus the struct diagnostic `non-exhaustive`) and reran all cases.
None of these harness issues is a candidate finding. The preserved final probe log exactly matches the recorded source hash.

## Results and interpretation

374 workspace tests pass (including the 232 inherited tests); all prescribed compiler,
lint, whitespace, metadata and five static target checks pass. The workload passes.
The byte-unchanged original corpus freshly reproduces 3 passes/7 failures before correction;
it fails compilation on the corrected tree's three changed API surfaces. The inspected
A1–A4 variant passes 9/9. Six compiled wrong implementations are killed. Independent
surface expectations pass 14/14. Candidate contract probes fail 3/14, so this is a
bounded-revision verdict, not acceptance.

`checks.json` records command-runner exits and exact commands. In particular,
`run_original_corpus.py` returns 0 after **capturing** cargo output; the actual cargo
failures and test counts are in each corpus log and explicitly classified in
`independent-results.json`. No compiler failure is counted as a runtime mutant kill.

Logs are whitespace-normalized after capture (trailing whitespace and final blank lines
removed); semantic text and diagnostics are otherwise retained. `SHA256SUMS` covers this
review evidence except itself and the later publication receipt. The external receipt
avoids a self-referential review-commit hash and records live synchronization after push.

Limits: static Windows/Android coverage only; finite source/tests/mutation corpus;
one-machine performance observations; no production promotion, crash recovery, durable
mailbox/storage, GAME integration or Phase 3. Timeline-index injection, maximal-depth and
cross-profile oracle gaps remain as recorded in the review.
