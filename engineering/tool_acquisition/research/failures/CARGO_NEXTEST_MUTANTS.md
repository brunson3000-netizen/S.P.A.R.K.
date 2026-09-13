# cargo-nextest / cargo-mutants Failure Lessons

Sources:
- nextest @ `8527c325bf9f0e5dcdcb26fa80e3ed7dc7f3e35e`
- cargo-mutants @ `fe82f1832778a591ab74248010fb40e699defafe`

## TESTUTIL-F01 — Retry can turn a real first failure into overall success

Nextest labels a fail-then-pass test flaky, but default overall result is success.

SPARK lesson: retries are evidence collection, not erasure. Acceptance profiles should retain the failed attempt and normally use `flaky-result=fail`.

## TESTUTIL-F02 — Faster executor can silently narrow canonical test coverage

Nextest currently does not run doctests and some custom harnesses require adaptation. cargo-mutants using `test_tool=nextest` inherits that omission.

SPARK lesson: preserve repository canonical test policy. A runner optimization may replace only a proven-compatible subset; omitted categories remain explicit separate gates.

## TESTUTIL-F03 — Source-copy isolation does not contain test side effects

cargo-mutants defaults to copied source/build trees, but mutated program/tests still have whatever filesystem/network/credential access the test environment provides.

SPARK lesson: mutation testing is arbitrary buggy-code execution. Run it in a disposable sandbox/VM/container appropriate to the project and remove production credentials.

## TESTUTIL-F04 — Mutation result without a valid baseline is meaningless

Skipping cargo-mutants baseline when tests are already failing can make almost every mutant appear “caught” for the wrong reason.

SPARK lesson: bind falsification evidence to a current passing baseline with identical source/test/toolchain/environment semantics, or to a prior baseline whose validity has not been invalidated.

## TESTUTIL-F05 — Timeout has ambiguous meaning

A mutant timeout may mean the injected defect caused a loop, the repository test is flaky/slow, or the timeout bound is too low. cargo-mutants intentionally gives timeout its own outcome/exit code.

SPARK lesson: never collapse timeout into caught/missed. Preserve it as an unresolved falsification outcome requiring separate analysis/retry policy.

## TESTUTIL-F06 — In-place mutation weakens the safety boundary

cargo-mutants offers `--in-place`, which modifies canonical checkout files and may leave a mutation behind after abrupt interruption.

SPARK lesson: default falsification lane forbids in-place mutation. Disposable copied/worktree state only, unless separately authorized.

## TESTUTIL-F07 — Unbounded mutation count is a compute sink

A full mutation set multiplies build/test cost by number of mutants; shards distribute elapsed time but do not remove total CPU cost.

SPARK lesson: automated mutation work requires explicit package/file/diff/sample count, wall-clock and concurrency envelopes. More shards do not reset mission budget.
