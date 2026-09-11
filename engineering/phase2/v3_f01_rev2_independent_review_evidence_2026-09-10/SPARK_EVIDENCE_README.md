# S.P.A.R.K. Revision-2 independent review evidence

Fresh independent Codex runs against candidate 5ecc95c033bf3a7744bb7862bf959066e6561670.
Linux x86_64; rustc/cargo 1.98.0; Python 3.12.3. No production changes.

`checks.json` records exact commands and exit codes. `run_validation.py OUTPUT_DIRECTORY`
reruns them using the containing repository. Logs have trailing whitespace trimmed.
The 58 model checks all pass and parsed results equal the writer evidence. Workspace test
summaries total 232 passed, zero failed/ignored, including doctests.

`run_independent_probe.py` augments the preserved writer Rust probe in a disposable
external Cargo package: 2,496 clean-state differential cases over two sources, prior
sequences 1–8, two same-frontier epoch resets, profile/epoch/identity/sequence variations.
It compares success to unchanged stage/fence on a clone and refusal to the complete entry
state (Debug plus both digests). Eight reset-order fixtures compare full state and reject
reversed reset order. `independent-probe.txt` is the fresh passing output.

The original eleven-route probe also passes; the missing inactive-grant Rust fixture is
covered by the Python model and direct source predicate, not falsely counted as a twelfth
original Rust route. Historical regression negatives remain red as intended.

These are architecture evidence. Generated corpus coverage is finite, not exhaustive.
The bounded indexed seam and production fail-stop/consumer fault injection are future
implementation obligations. Python deep copies do not prove durable crash recovery;
static cross-target compilation does not prove executable platform parity.
