# S.P.A.R.K. V3-F01 Revision-2 independent review

**Date:** 2026-09-10
**Reviewer:** Codex, independent of the separated correction writer
**Verdict:** `V3_F01_REV2_ACCEPTABLE_FOR_OPERATOR_ACCEPTANCE_AND_FREEZE`

## Verified custody and scope

Fetched origin and inspected live GitHub heads, local branches, tracking refs, ancestry,
and every registered worktree. Candidate branch
`candidate/v3-f01-bounded-correction-rev2-20260910` is exactly
`5ecc95c033bf3a7744bb7862bf959066e6561670`, with sole parent
`5b19d7b5aa935b65fcfad1d8bda9a210d5a5684d`. Production initially agrees locally,
in tracking, and live at `e9e26e815f1d4d890c5f5621be9e2143e7dd9ac8`.
The review branch `review/v3-f01-rev2-20260910` did not exist and was created directly
at the candidate in an isolated worktree. All four pre-existing worktrees were clean
and preserved; research remains at `03d36dd649641b28b232f4650b773413db634611`.

No applicable repository/ancestor AGENTS instruction file or repository CLAUDE/CODEX
instruction file was found; `.agents` and `.codex` are empty. Consulted the status,
handoff, convergence protocol, durable-capture principle, Operator ActiveRequest decision,
prior review and Revision-2 mission. The current Operator instruction explicitly authorizes
this review's publication, acceptance with the two pins below, freeze, closure, and normal
architecture-only production promotion; it supersedes earlier mission nonauthorization
of those actions. Phase-2 implementation and Phase 3 remain unauthorized.

Reviewed Revision-2 architecture, oracle, writer report, model and probe; reconciled the
FINAL candidate's inherited extraction, time, digest, rejection, facade and supersession
contracts and the prior independent findings. Checked the unchanged Phase-1 timeline
stage/fence/reset and registry invariants, scheduler fingerprint/digest behavior, identifier
bounds and ADR-0003. Candidate-parent and production-candidate differences contain only
engineering documentation and disposable evidence: `crates`, manifests and lockfile are
byte-identical. No Phase-2 production Rust exists in this change.

## Refusal coverage and entailment

The complete table covers every refusal of the specified one-ordinal composition under
valid typed inputs and the Phase-1 derived-index invariant, rather than arbitrary forged
private memory. Source anchors below refer to unchanged `crates/spark-core/src/timeline.rs`.

| Path | Independent disposition |
|---|---|
| Wrong profile, epoch, sequencer at stage or fence (1233–1241, 1440–1448) | P-1, P-2, P-3 check each before mutation. |
| Window arithmetic (1248–1250, 1452–1454) | P-4; independent from frontier increment. |
| Fence frontier increment (1466–1476) | P-5 prevents successful staging followed by overflow refusal. |
| Poisoned, different staged, identical staged frontier; staged tail (1277–1391) | P-6 rejects every occupied slot in the window before poison evidence insertion, poisoning, idempotent acknowledgment or unexpected finalization. |
| Finalized/staged command identity (1284–1294) | P-7 covers permanent claims; P-6 and the derived-index invariant imply staged claims are empty. |
| Finalized/staged source-sequence identity (1295–1307) | P-8, with the same empty-index argument. |
| Last finalized source sequence (1516–1533) | P-9 covers regression and equality; for one ordinal there is no additional in-fence source predecessor. |
| Out-of-window ordinal and invalid admission ticket (1252–1273) | Excluded by constructing ordinal at the frontier and minting the current ticket under the same exclusive borrow. Nonzero width is a Phase-1 constructor invariant. |
| Start not frontier, invalid end, previous fence hash (1449–1460) | Excluded by deriving start=end=frontier and the current hash without interleaving. |
| Missing/poisoned fence range (1478–1498) | A-3 necessarily newly stages the sole ordinal after P-6. |
| Ordered stream digest mismatch (1500–1507) | Excluded by the same Phase-1 digest function over exactly that ordinal and envelope hash. |

P-1 through P-9 are read-only. A-3 changes only the new slot and staged indexes; it does
not invalidate P-9. The successful positive `NewlyStaged` acknowledgment covering the
exact envelope precedes the single-ordinal fence. Fence promotion removes that slot and
moves its claims into permanent registries. Exclusive engine ownership closes interleaving.
Every ordinary refusal therefore leaves the entire timeline unchanged, including private
indexes and poison evidence, not merely finalized history. Multi-fault reason ordering is
explicitly preflight order; differential verdict equality does not falsely promise identical
Phase-1 first-error ordering.

FINAL-01 is resolved. The old S:10 then distinct S:9 recipe remains executably red;
the correction refuses before staging and a subsequent S:11 command finalizes cleanly.
I-CS is established at genesis, preserved by finalization/reset, checked at restore,
and protected by the engine facade. Unclean states are deliberately refused even where
standalone Phase-1 would accept; inherited Phase-1 semantics are unchanged.

## Decisions, lifecycle and replay

- **D-6 accepted with pin 2:** complete preflight plus entailed apply is mechanically
  specified. Cloning the ever-growing timeline is a differential reference only.
- **D-7 accepted:** clean staging and restore rejection prevent publishing staged residue.
- **D-8 accepted:** an unexpected apply result is an implementation defect and sticky
  `FINALIZATION_ENTAILMENT_VIOLATED`, never an ordinary finalization refusal. No stable
  boundary/snapshot is published; only restoration of the last committed snapshot permits
  recovery. AT-I48(j) must exercise this in the separately authorized implementation.
- **D-9 accepted:** every result binds the presented request. Dequeue requires the
  terminating result to name the actual head. Mismatch terminates nothing; head/active
  disagreement halts without popping, searching, reordering or replacing. Exact request
  bytes remain durable until completion; completion durability precedes dequeue durability.
- **D-10 accepted:** `fence.` plus decimal ordinal is valid within Phase-1 identifier bounds
  for all u64 ordinals and makes the fence identity deterministic without changing encoding.

FINAL-02 is resolved by the request-bound consumer rule and fail-closed mismatch handling.
The faulty-presentation regression preserves the genuine head and then resumes it. Reverse
durability ordering detects a lost head but is expressly not safe recovery. These are
contract/model findings, not a storage-system crash test.

FINAL-03 is resolved with pin 1: completed-boundary reconstruction requires identical
genesis stores, activated artifact/behavior epoch/caps, full timeline metadata, finalized
envelopes plus actual payloads and F. Reset records and deterministic fences are necessary.
Paused recovery instead requires the validated committed snapshot with stores, timeline,
F and ActiveRequest, plus the exact durable request. Equal history and F cannot determine
paused progress; a digest cannot reconstruct command payload. Arbitrary intervening behavior
activation is outside the fixed-artifact/epoch equivalence preconditions.

D-2 still advances F after scheduled-horizon completion when command finalization refuses;
earlier scheduled work and committed waves remain. No whole-request/cohort rollback is
introduced. Horizon expansion, cohort-local time, live least-slice full-fingerprint
comparison, atomic scheduler/obligation extraction, executable-only identity/pacing,
complete conflicted extraction/report ordering, later-wave commit retention, and the
observation/facade boundary remain intact. Phi, X, C, SH-1/SH-2 and R-8 remain withdrawn.
The five revised oracle precision items now state discriminating fixtures honestly.

## Two controlling precision pins

1. **Multiple epoch-reset records sharing one frozen frontier must replay in ascending
   `reset_index`.** This applies before the next command and to trailing resets at a
   completed boundary. Preserve recorded order and validate the resulting history/state;
   frontier alone is not an ordering key. The independent Rust fixtures reproduce full
   state in ascending order and reject the reversed order.
2. **Production P-7 through P-9 must use bounded indexed lookup, never a scan of
   ever-growing finalized history.** Prefer PX-1's three read-only accessors
   (`finalized_command_claim`, `finalized_source_sequence_claim`,
   `last_finalized_source_sequence`) or an equivalent bounded private seam. Existing
   BTreeMap lookups provide logarithmic lookup without enumerating history. This supersedes
   the candidate's permission to choose the O(history) fallback and its unresolved PX-1
   choice only to that extent. It changes no encoding, authority, identity or admission
   semantics. The pin freezes a future implementation constraint; it adds no Rust now and
   makes no constant-time or whole-engine performance claim.

## Fresh validation and limits

Evidence: `engineering/phase2/v3_f01_rev2_independent_review_evidence_2026-09-10/`.
Exact commands and exit codes are in `checks.json`, with repeatable runners and logs.

| Check | Result |
|---|---|
| Revision-2 model | 58/58 PASS; parsed JSON equals preserved writer output. |
| Disposable unchanged-Phase-1 Rust probe | Eleven refusal routes preserve full Debug state and both digests; success/reference equality and follow-on pass; old composition mutates on six routes and its negative control fails as intended. |
| Additional independent Rust corpus | 2,496 clean-state differential cases PASS; eight same-frontier reset-order fixtures PASS with reverse-order negative controls. |
| Historical regression runner | PASS: prior defects still reproduced against the prior recipe/model. |
| Workspace tests, including doctests | 232 passed, 0 failed, 0 ignored. |
| Formatting; all-target/all-feature clippy with warnings denied | PASS. |
| Core/engine strict library lint | PASS: warnings, unwrap, expect, panic, indexing and arithmetic side effects denied. |
| Metadata and whitespace | PASS. |
| Windows GNU/MSVC; Android aarch64/armv7/x86_64 static all-target compilation | All five PASS. |
| Production preservation | No changes to crates, manifests, tests or lockfile. |

The original Rust probe has no separate inactive-grant route; P-3 is covered by model and
source inspection. The model is finite, uses JSON/truncated hashes and omits poison caps;
it is not canonical encoder proof. AT-I48(j), AT-I49(b), compile-fail facade tests, independent
canonical byte vectors and the full generated production acceptance corpus remain required
implementation tests, not falsely reported as executed Phase-2 tests. Bounded indexed
lookup is not benchmarked or implemented here. Crash recovery, durable snapshots/mailboxes,
GAME integration, performance evidence and executable Windows/Android parity remain gates.

No MAJOR or BLOCKER remains for architecture acceptance with these two pins.
`V3_F01_REV2_ACCEPTABLE_FOR_OPERATOR_ACCEPTANCE_AND_FREEZE` is the independent technical
verdict. The subsequent Operator acceptance record performs the authorized closure/freeze;
this review does not release Phase-2 implementation or Phase 3. Historical artifacts remain
unchanged and are interpreted through the explicit supersession stack.
