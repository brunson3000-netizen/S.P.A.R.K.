# S.P.A.R.K. Gate C2 — inherited-tree baseline (red-first record)

Recorded before any production change, on the unmodified production baseline
`phase1-refoundation-v2` at `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5`, in the isolated
worktree `/home/chromikey/Projects/SPARK-gate-c2` on branch
`candidate/phase2-gate-c2-implementation-20260911`.

## Inherited suite

`cargo test --workspace --all-features --offline` — **232 passed, 0 failed, 0 ignored**
(`inherited-workspace-tests.txt`, trailing whitespace trimmed). This is the AT-I34
regression floor: every one of these tests must still pass, unmodified, at every Gate C2
checkpoint.

## Red-first status of the Phase-2 acceptance oracle against the inherited tree

Every AT-I entry of the frozen oracle stack (matrix v1 → v2 → v3, V3-F01 FINAL oracle,
Revision-2 oracle) names types or operations that do not exist in the inherited tree:
`RuleSetSpec` / `ActivatedRuleSet`, the effect/candidate/reduction model, `ObligationStore`,
`OccurrenceLedger`, `CooldownLedger`, `EpochRegistry`, the Phase-2 `Engine` with `process`,
`F`, `ActiveRequest`, `stable_boundary_digest`, the scheduler's least-due compare-and-take
surface X-1 … X-4, the timeline's PX-1 read-only accessors, snapshot/restore/replay, and
`PacingDiagnostics`.

Each such test is therefore recorded here as **inexpressible against the inherited tree**
(it cannot compile before its types exist), per the matrix discipline — the recording is
made, not skipped. The two exceptions that are expressible against the inherited tree are
already red evidence in the repository:

- the FINAL-01 source-sequence regression (old stage-then-fence recipe leaves a staged
  slot) — `engineering/phase2/v3_f01_final_independent_review_evidence_2026-09-10/`;
- AT-I35 Phase-1 digest stability — the inherited pinned-hash tests are green here and are
  the fixed point every later checkpoint is compared against.

Each Gate C2 test lands with the first commit that makes it expressible; its checkpoint
records the red/green transition in the implementation report.
