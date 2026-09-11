# S.P.A.R.K. Gate C1 — Final V3-F01 Independent Review

**Date:** 2026-09-10
**Reviewer:** Codex, independent reviewer, not the correction writer
**Verdict:** `V3_F01_BOUNDED_REVISION_REQUIRED`

## Baseline and preservation

- Repository: `https://github.com/brunson3000-netizen/S.P.A.R.K..git`.
- Production: `phase1-refoundation-v2` at `e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8`.
- Reviewed candidate: `candidate/v3-f01-final-correction-20260910` at
  `74d044ddd2b59c3c2eff558beeeeb515390dbe0f`.
- Its sole parent: `772e38da0d130d7a1225ba2eb60a75b3999336fe`.
- Review branch: `review/v3-f01-final-20260910`, created at that exact candidate.

State was reconstructed after reboot, without reliance on interrupted-session state.
Fetch succeeded with the filesystem escalation required for Git metadata; live
`ls-remote`, fetched refs, and local production/candidate tips agreed. The prior remote
serialized-boundary review tip was `772e38d`. No final-review branch existed locally or
remotely. Existing backup, reconciliation, research, candidate, and production branches
were preserved. The primary, writer, and research worktrees were clean when inspected.
The candidate changes seven documentation/model files relative to its parent; production
Rust, manifests, and tests are identical to the production baseline.

No applicable `AGENTS.md`, `CLAUDE.md`, or `CODEX.md` was found in the repository tree or
ancestor AGENTS locations; `.agents` and `.codex` were empty. Operating instructions
consulted include PHASE_STATUS, the convergence protocol, durable conversation capture
principle, development handoff, Operator ActiveRequest decision, and prior writer mission.
The current Operator instruction authorizes this review publication and the bounded next
writer pass; it does not authorize production implementation or acceptance.

## Review scope and disposition

Reviewed the complete final architecture candidate, acceptance oracle, writer report,
Python model and preserved results, candidate status delta, ActiveRequest decision, and
serialized-boundary review. Reconciled the relevant V2 findings and inspected Phase-1
clock, timeline stage/fence/digest paths, scheduler extraction/conflict/digest paths,
engine state and public surface, manifests, dependency hygiene tests, and ADR-0003.

The engine-owned exact-request discriminator, set before mutation, retained across pause,
checked before the horizon guard, and committed with `F` in the stable-boundary digest is
the correct bounded response to SB-01. Preserve it. Preserve horizon expansion,
cohort-local time, live least-slice selection, preflighted cross-store extraction,
executable-only identity/pacing, mixed-conflict ordering, and later-wave commit retention.
The candidate materially carries the V2-03 through V2-10 repairs; the findings below do
not reopen those foundations or Phase 1.

D-1 (ordinal at finalization) and D-2 (advance `F` after scheduled horizon completion even
if finalization refuses) remain viable only with the atomicity repair below. D-3
zero-cost conflict consumption, D-4 equal-horizon Advance identity, and D-5 restore
validation do not require a new Operator choice for this bounded pass. The writer's
Fable-versus-Opus provenance deviation is disclosed; it does not change the technical
verdict or justify manufacturing a new approval gate.

## FINAL-01 — BLOCKER: stage/fence composition is not atomic

Candidate §§6.3, 8.2–8.3 and CE-10 assert that finalization refusal leaves the timeline
unchanged and no staged-but-unfinalized command exists at a stable boundary. The concrete
recipe calls the existing `stage`, then existing `submit_fence`, and treats the latter's
no-mutation guarantee as if it covered both calls. It does not.

Direct source anchors in `crates/spark-core/src/timeline.rs`:

- `stage` starts at line 1226. Empty-slot checks at 1278–1309 reject reused identities
  and exact source/sequence pairs; they do not compare against the last finalized source
  sequence. Lines 1310–1327 insert the slot and staged identity indexes and acknowledge it.
- `submit_fence` starts at 1435. Lines 1516–1532 compare against
  `last_finalized_source_sequence` and return `SourceSequenceNotIncreasing` before any
  fence promotion. They do not undo prior staging.
- `canonical_state_digest` at 1712 includes staged slots, unlike finalized history alone.

Executable counterexample, with symbolic source S encoded as valid lowercase `s`:

1. Finalize a distinct command at ordinal 0 with source sequence 10.
2. Frontier 1 is empty; capture timeline state and history digests.
3. Present a distinct command at ordinal 1, source sequence 9, with a valid ticket,
   epoch, sequencer, and one-command fence digest.
4. `stage` returns `NewlyStaged`; the timeline state digest changes.
5. `submit_fence` returns `SourceSequenceNotIncreasing { previously_finalized: 10,
   attempted: 9 }`.
6. Frontier and finalized history remain unchanged, but ordinal 1 is still positively
   staged and the post-fence state equals the post-stage state, not the pre-stage state.

The isolated Rust probe executes the actual unchanged Phase-1 library, asserts these
facts, and separately demonstrates that the candidate's equality assertion fails.
Evidence includes before/staged/after digests. No Phase-2 engine exists, so the probe
establishes failure of the candidate's prescribed composition, not an executed Phase-2
`process` result. Following §8.3 would return a stable completed boundary with that residue.
A subsequent different command at the same frontier can poison it; ordinary retry does
not repair this transaction.

ADR-0003 §8 rule 8 guarantees that a rejected **fence** promotes nothing; §13 says a
convenience call preserves stage/fence semantics. Neither guarantees rollback of earlier
successful stage calls. This is a candidate composition defect, not a reopened Phase-1
bug. Nonpositive stage results can themselves record poison evidence on occupied slots,
so the next specification must cover every refusal, not just add a sequence check.

Required correction: specify one mechanically atomic single-command finalization
operation, with complete validation before live mutation or a fully specified isolated
transaction and commit point. Every refusal must preserve the entire timeline byte for
byte, including slots, poison evidence, staged/finalized identity indexes, frontier,
fences, and epoch metadata. Define preconditions, ownership, errors, and successful
publication. A prose assertion that both calls happen inside one `process` is insufficient.
Do not change inherited stage/fence semantics or implement production Rust in this pass.
Already committed scheduled work may remain and `F` may advance under D-2; timeline
refusal atomicity is not whole-request rollback or whole-cohort rollback.

The model's `Command` has no source sequence or staging store; `Engine.process` lines
275–285 checks duplicate command IDs then appends directly to finalized history. S17's
passing duplicate-ID case therefore cannot establish the general refusal claim. Extend
the model and oracle with the executable regression and a negative control that fails the
old composition.

## FINAL-02 — MAJOR: mismatch must not consume the real FIFO head

Candidate §5 Next explicitly includes `REFUSED_ACTIVE_REQUEST_MISMATCH` among results
that pop the head; §17 item 3 says to pop on any terminal result. The model includes
mismatch in `TERMINAL` and `drive` pops for those results. This contradicts retention of
the actual active request required by §§5–6 and the Operator decision.

The probe pauses real head `Advance(20)`, presents a different `Command@12`, observes the
correct non-mutating engine refusal, then applies the documented terminal-pop rule. The
actual head disappears while `ActiveRequest` still binds it. This models precisely the
faulty presentation the engine is intended to withstand; it is not the normal disciplined
`drive` loop, which always presents its head.

Make mismatch a refusal of the presented request, never acknowledgment/completion of the
actual active head. Preserve engine state, FIFO contents/order, and durable active request;
fail closed on a restore/mailbox disagreement and resume only the exact matching request.
Define terminal dequeue eligibility explicitly, tied to the request actually at the head.
Add a consumer-level regression in addition to engine no-mutation tests. No cancellation,
abandonment, replacement, or new mailbox implementation is authorized.

## FINAL-03 — MAJOR: history-only replay cannot restore a pause

Candidate §16 says history is finalized commands plus `F`, then proposes replaying those
commands and finally `Advance(F)`. It also says paused partial progress can be consumed by
any later equal-or-greater-horizon request. That latter statement conflicts with the exact
ActiveRequest guard and §6.5; §19.1 retains the earlier replay claim without narrowing it.

The model probe constructs two reachable boundaries from the same genesis, with empty
finalized command history and `F = 0`: one before any progress, one after a budgeted
`Advance(20)` has processed work at 10 and paused. Their engine and stable-boundary
digests differ. History plus `F` supplies no active request or progress position, and
replaying `Advance(0)` cannot reproduce the paused state. Restoring the committed snapshot
and presenting the exact request does reproduce continuation; a substitute is refused.

Scope history-only reconstruction to completed request boundaries, with fixed initial
state, activated artifacts/epochs/caps, and the complete required timeline metadata.
Do not imply envelope-only replay reproduces Phase-1 fence/reset digests without those
inputs. Paused recovery requires the committed snapshot containing stores, `F`, and
`ActiveRequest`, validated together, plus the exact durable mailbox request. Do not
claim that a discriminator reconstructs a command payload, or that losing a command is
safe recovery. No real process-crash persistence proof is claimed by a deepcopy model.

## Oracle precision to correct in the same bounded pass

The full oracle still contains assertions that should be narrowed rather than credited as
universal proofs:

- AT-I43(f) presents Commands while an **Advance** is active. A kind+horizon-only mutant
  rejects both; to kill omission of command payload identity, pause a Command and present
  a different Command with the same kind and horizon.
- AT-I47(a)'s five cases change identity along with kind/horizon. Because identity itself
  binds those fields, this does not independently kill omission of each redundant field.
  Keep canonical encoding pins and state exactly what the discrimination fixture proves.
- AT-I40(e) says obligation removals are equal for `{S}` versus `{S,X}`. Resulting stores
  can be equal; removed sets differ by X's contested records.
- AT-I39(C)'s selection count must identify the outer-loop seam: §11 also selects the live
  least slice during extraction. An unconditional count of all X-1 calls is not slices+1.
- AT-I43(a) must state the work-producing rule that creates the intermediate cohort and
  spell out successive call results; one call cannot return both PAUSED and COMPLETED.

These are bounded oracle corrections, not additional foundational decisions.

## Fresh validation

Environment: Linux x86_64, rustc 1.98.0 (88d9e12ae 2026-08-18), cargo 1.98.0
(797e8a9bc 2026-08-05), Python 3.12. Evidence directory:
`engineering/phase2/v3_f01_final_independent_review_evidence_2026-09-10/`.

| Check | Fresh result |
|---|---|
| Candidate disposable Python model | 34/34 PASS; stdout byte-identical and parsed JSON equal to candidate evidence |
| Isolated Phase-1 source-sequence probe | Counterexample confirmed; candidate equality assertion fails as expected |
| Consumer mismatch and paused replay probes | Both counterexamples confirmed; exact snapshot/request continuation agrees |
| `cargo test --workspace` | 232 passed, 0 failed, 0 ignored, including doctests |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| Strict core/engine library clippy: warnings, unwrap, expect, panic, indexing, arithmetic side effects denied | PASS |
| `cargo metadata --format-version 1` | PASS; dependency hygiene also covered by workspace tests |
| Windows GNU/MSVC and Android aarch64/armv7/x86_64 `cargo check --workspace --all-targets --target ...` | All five PASS; static coverage only |
| Candidate `git diff --check 772e38d 74d044d` | PASS |
| Review whitespace, changed-file and production-preservation checks | PASS after whitespace cleanup; recorded in validation evidence |

Unavailable here: Phase-2 production acceptance execution (implementation does not exist),
real process-crash/durable mailbox testing, Windows/Android executable replay/digest parity.
They are not inferred from Python or static target checks. No supplementary agents or NIM
panel were used. No production implementation or historical candidate rewrite occurred.

## Verdict, custody, and next step

`V3_F01_BOUNDED_REVISION_REQUIRED`. FINAL-01 blocks acceptance; FINAL-02 and FINAL-03
must be corrected in the same bounded writer pass. V3-F01 remains OPEN; no acceptance,
freeze, production merge, Phase-2 implementation, or Phase-3 authorization is claimed.

The canonical separated-writer mission, a new revision rather than an edit to the prior
mission, is:
`engineering/phase2/SPARK_PHASE_2_V3_F01_BOUNDED_CORRECTION_WRITER_MISSION_REV2_2026-09-10.md`.

**No new Operator decision is required merely to run this bounded correction writer
pass.** The ActiveRequest amendment is already authorized. Later acceptance/freeze and
implementation remain distinct gates. The Operator's next action is to hand that mission
to the separated architecture writer (Claude Code Opus, high reasoning), then return its
exact published candidate commit for independent Codex review.

This review package is committed and normally pushed on its review branch. Its receipt
must verify equality of local HEAD, tracking ref, and live remote branch tip after push;
the commit hash is provided in that receipt rather than self-referenced in this document.
Production is not merged or advanced by this review.
