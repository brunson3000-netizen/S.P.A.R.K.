# Gate C2 decay adjudication independent review evidence

Exact candidate: `544c5f6f99d8dabf9855f8ac68f5666286aa9741`.
Review mission dated 2026-09-11; execution continued into 2026-09-12 (America/Chicago).
All candidate validation ran while this isolated review worktree's HEAD was the exact
candidate. Review-only files were untracked additions under this directory; candidate
source, tests, manifests and historical evidence were never edited.

- `initial-custody.json`: fetched/live references, worktrees, exact lineage and final delta.
- `independent-custody-audit.json`: independent parent/tree/history/checksum audit.
- `supersession-source-lines.txt`: S-1–S-13 source passages at the exact controlling base.
- `checks.json` and named logs: 14 fresh checks, including the release workload.
- `preservation.json`: protected functions, 17 inline modules, inherited tests, manifests,
  lockfile, comment-only changes, decision immutability and historical preservation.
- `mutation-controls.json`: writer's ten compiled mutants; 21 positive tests, 28 kills,
  and the two deliberately surviving earlier oracle tests.
- `reviewer-mutations.json`: four controlling-review mutants, unchanged probe source.
- `surface-probes.json`: 17 default-feature external compile expectations and diagnostics.
- `own_probes.rs`, `own-probes.txt`: nine independent hand-valued public-door tests.
  No writer model or engine arithmetic is copied. Shared fixture builders provide
  admitted profiles, rules and public requests. Includes all D rules and four open matters.
- `run_own_mutants.py`, `own-mutations.json`: nine isolated compiled wrong implementations,
  positive baselines followed by assertion kills. D4 endpoint exclusion and D6 next-tick
  repeat useful prior wrong implementations with new probes; other patches target new
  discriminators. The zero-length-epoch mutant characterizes an undecided behavior.
- `check_evidence.py`, `check_own_evidence.py`: executable result and custody checks.
- `SHA256SUMS`: all bounded evidence files except the manifest itself.

The fresh validation/corpus/writer-mutant/prior-review-mutant/surface/preservation runners
are copied from the published candidate, inspected, and run from this separate directory.
`check_evidence.py` changes its expected pin to the final commit. No earlier evidence is
rewritten. `run_retained.py` supplies the explicit corpus paths. Wrapper exit zero is
capture success only; the child cargo result and actual assertions control dispositions.

Expected historical failures: original corpus refuses at the same revised surfaces;
second review has exactly its superseded timestamp failure (13/1), and controlling review
has exactly two superseded Q7 failures (7/2). Their existing adapted copies pass 9/9 and
14/14; the decision-adapted copy passes 9/9 and differs by exactly two expected literals.

Run from a fresh checkout pinned to the candidate, preserving this evidence directory:

```sh
python3 engineering/phase2/gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/run_validation.py
python3 engineering/phase2/gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/run_mutation_controls.py
python3 engineering/phase2/gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/run_retained.py
python3 engineering/phase2/gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/run_own_mutants.py
python3 engineering/phase2/gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/run_corpus.py . own-probes engineering/phase2/gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/own_probes.rs --test-support
python3 engineering/phase2/gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/check_evidence.py
python3 engineering/phase2/gate_c2_decay_adjudication_independent_review_evidence_2026-09-11/check_own_evidence.py
```

Finite fixtures and controls support the review; they are not proofs, production acceptance,
a durable-storage test, platform runtime parity, or a performance guarantee. Build settings
are debug-info-free, nonincremental, two jobs, as recorded by the validation runner.
