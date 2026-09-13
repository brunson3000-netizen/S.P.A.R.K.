# Experiment — Syntax-Bounded Context Slicing vs Line Windows

Status: READY_TO_RUN
No implementation/adoption authorized by this record.

## Hypothesis

Syntax-aware structural slicing can reduce agent-visible repository context materially without reducing correct debugging/research outcomes compared with grep/ripgrep hits plus fixed line windows.

## Candidates

A — baseline:
- ripgrep/grep query
- fixed configurable ±N line context

B — structural:
- ast-grep structural query
- Tree-sitter syntax tree
- exact match + bounded enclosing definition/import/declaration policy
- parse-health metadata.

## Controls

Hold constant:
- repository commit
- task/question set
- downstream model/worker
- answer token budget
- permitted follow-up count
- file-read authority
- no source mutation.

Use a mixed task corpus:
1. find function definition + relevant validation
2. find all call sites matching argument structure
3. locate config parsing for one option
4. trace one error enum to handling sites
5. identify all implementations of a repeated syntactic shape
6. repeat selected tasks in files containing intentional syntax errors/incomplete code.

## Measurements

Per task/method:
- files scanned
- search latency
- bytes returned initially
- estimated input tokens returned
- distinct structural units returned
- unrelated bytes
- required definition/import/call-site recall
- false matches
- follow-up search/read count
- total context bytes before answer
- task success / correct answer
- missed load-bearing evidence
- deterministic repeat agreement across repeated runs
- parse-health warnings produced
- confidence errors caused by malformed source.

## Evidence format

Every structural result must record:
- repository commit/source digest
- file path
- byte range
- line/column range
- query/rule identity
- language + parser/grammar version
- parse tree has-error flag
- whether match/enclosing slice intersects error/missing nodes
- raw source artifact/reference.

## Stop / reject conditions

Reject the mechanism for default worker slicing if:
- task success materially declines
- required evidence is systematically hidden
- malformed-source matches are presented without parse-health warnings
- context reduction is negligible relative to complexity/cost
- results are non-reproducible on identical source/query/parser pins.

## Promotion direction

If B preserves task success while substantially reducing bytes/tokens/follow-up reads, promote `ast-grep-core`-based read-only slicing to a Rust prototype behind a host-owned evidence envelope.

Do not enable ast-grep rewriting in this experiment.
