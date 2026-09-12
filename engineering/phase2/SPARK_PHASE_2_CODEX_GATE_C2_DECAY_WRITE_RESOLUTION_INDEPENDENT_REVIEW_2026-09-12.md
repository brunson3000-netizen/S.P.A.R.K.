# Gate C2 decay-write resolution — independent review

Reviewer: Codex, independent of the candidate writer. Date: 2026-09-12.
Candidate: `cc3dc70182b428f7cbc953ae722f85b39d407dfa`, branch
`candidate/phase2-gate-c2-decay-write-resolution-20260912`.

**Verdict: `GATE_C2_DECAY_WRITE_RESOLUTION_REQUIRES_BOUNDED_REVISION`.**
Acceptance is blocked by **C2W-01**: an additive stage followed by an identity transform
forfeits overdue decay/recovery. This violates approved behavior 1. The writer's disclosure
in report §4 does not authorize deferral. No further Operator decision is needed to establish
that this concrete additive event must settle. The current Operator instruction also
expressly requires resolution before acceptance.

Review branch: `review/phase2-gate-c2-decay-write-resolution-independent-20260912`.
Evidence E: `gate_c2_decay_write_independent_evidence_2026-09-12/` alongside this report.
The publication receipt supplies the containing full commit and synchronized remote ref.
No candidate source was repaired; no acceptance, production promotion or Phase 3 occurred.

## 1. Controlling authority and custody

The 2026-09-12 Operator resolution controls additive settlement and behaviors 2–5;
the immutable 2026-09-11 D-1–D-7/S-1–S-13 adjudication controls grids, barriers and canonical
time. Otherwise retain the controlling review `e00f248e25f6d34f1041e219f74085649714c19d`
§1 precedence: acceptance freeze/Revision-2 architecture and oracle, FINAL, retained V2,
freezes/matrices, ActiveRequest and ADRs. Withdrawn Phi, SH-1/SH-2 and architectural R-8
remain withdrawn. Prior acceptance of implementation observations does not override the
later Operator resolution.

Live GitHub heads were captured before validation in E/live-heads-before.txt. Production
and acceptance remain `7e3a0aae069a8bf840e4dfb74221cdf2687b1db5`; the preceding candidate
remains `544c5f6f99d8dabf9855f8ac68f5666286aa9741`; the controlling review remains `e00f248…`.
All are reachable ancestors of the fetched final candidate. Intermediate hashes were
verified through that live candidate's fetched history, not mistaken for branch tips:

| Commit | Verified content |
|---|---|
| `a3227eb423e75851b05b9992dfde6424a7d8932d` | Direct mission checkpoint |
| `ddee0a28f846160b958a392505c4f0076b341f72` | First child; implementation contract alone |
| `a9c15fd21ae28af8e11f3ad7c083d250676886bf` | Production change and adapted reference model |
| `b1608ddc4470d73b9c9eaf5ce26c61697000be38` | New normative tests; last crate change |
| `cc3dc70182b428f7cbc953ae722f85b39d407dfa` | Exactly 50 added documentation/evidence files under engineering/phase2 |

The final edge changes no crate, manifest, lockfile or existing file. Both crate trees are
`bba600119538312df0fc42792cf3fb53c112604b`. The contract is byte-unchanged since its first
commit. E/custody-audit.json and E/preservation.json record the independent comparisons.
Exactly four crate paths change from the mission checkpoint, with production +104/-1;
the one removed line is the raw reducer pre-value lookup. All seven inherited integration
files, 17 inline Phase-1 modules, spark-core, manifests/lockfile, protected functions
(including decay_walk/baseline_of), and all 627 pre-existing engineering files are preserved.

The provided checkout's Git metadata is read-only; its initial fetch failed without changing
it. Review used a fresh clone/main worktree at `/tmp/spark-c2-independent-review-20260912`,
detached at the exact candidate throughout validation. Existing branches/worktrees were not
reset or switched. Final publication comparisons cover every captured pre-existing remote
head and the original repository's worktree/head listing.

## 2. C2W-01 — additive composition loses overdue settlement (blocking, high priority)

Location: `crates/spark-engine/src/engine.rs:951–954,1032–1066`, and the settled lookup passed
to `reduce` at line 2159. `effects.rs:288` consults that lookup only for ADDITIVE. A body
containing a Clamp or Scale is lowered to a pre-resolved `TransformFamily::RuleBody`, so
its additive stage never sees settlement.

Concrete public-door vectors, baseline 0, rate 10/cadence 10, no evaluation before the write:

| Existing cell | Body at 15 | Required at 15 | Actual at 15 |
|---|---|---|---|
| `100@0` | `Add(+5); Clamp[-1000,1000]` | `95@15` | `105@15` |
| `-100@0` | `Add(-5); Scale(1)` | `-95@15` | `-105@15` |

Both bodies perform an additive change with an identity finishing stage; neither declares
an explicit replacement or an explicit decay stage. The presence of the identity stage
cannot reasonably convert the additive event into an exception to "An additive event must
not erase unapplied earlier grid steps." The raw 100/-100 survives instead of being settled
to 90/-90. Committing updated_at=15 permanently excludes endpoint 10 from subsequent walks.
E/own-probes.txt records cargo exit 101 and these two exact failing assertions. These are
current conformance failures, not expected historical failures.

Pure multi-Add/Subtract bodies do take the all-additive fast path and settle correctly;
the defect is the mixed-stage path. The writer's broad wording about bodies "containing
additive stages" should be narrowed accordingly. Bodies with a genuine explicit replacement
require stage-aware handling; this finding does not demand charging debt against a declared
replacement or inventing universal ordering across competing rules.

Required revision: implement behavior 1 end to end for additive compositions without an
explicit debt-consuming stage, preserving declared stage order, explicit replacements,
canonical timestamps, bounds, atomic rejection and single charging. Add normative tests for
identity and nonidentity Clamp/Scale compositions in both directions, contrasting an explicit
pre-evaluation with catch-up at the additive write. Include removed-operation and activation
barrier interactions. Publish an additive correction to the historical implementation
contract/report; do not rewrite immutable authority or historical evidence. Revalidate the
exact revised final candidate and obtain a separated independent review before acceptance.

## 3. Five approved behaviors and two boundaries

| Behavior | Disposition and independent evidence |
|---|---|
| 1 Additive settlement | **Incomplete: C2W-01.** Single/all-additive command groups pass aligned 10/20, unaligned 15/25, early 9, repeated same-time writes, both signs, saturation at baseline 11, rate 0, scheduled kicks and materialized obligations. Writer co-firing-group test also passes. The pinned 95@15/85@20 and explicit evaluation history pass. |
| 2 Explicit replacement | **PASS** for declared replacement. Own both-sign replacements at 9/10/15/20/25 remain unreduced and retain the grid. Workspace retains AT-I22′ fresh assignment 90@10 and post-shock 65@40 with unchanged rationale: fixed-grid endpoints, canonical commits, and the shock's prior steps already evaluated. |
| 3 Removal/restoration | **PASS on reviewed additive path.** Own 80@0, 3/5, removal 12, +4 at 18 gives 78@18; restoration 20 gives 78@24,75@25, both signs. Closing points 5/10 are billed; removed interval is free; restoration starts its own grid. Composition interaction remains part of C2W-01 revision. |
| 4 Same-time activations | **PASS on reviewed additive path.** Changed-and-reversed parameters at 12 give 74@15,71@17; two unchanged activations give 71@15,71@17, both signs. Restore between activations preserves order. At barrier 10, 3/5→7/7 plus +4 yields 78@10 then 71@17 from 80@0, both signs. |
| 5 Absent decay cell | **PASS.** Evaluation at 5 creates neither cell nor effect; +8 at 6 then evaluation 10 gives 5@10, mirrored below baseline. A separate initializing body stage remains allowed. |
| Standalone Scale/Clamp | **Accept retained boundary.** Half-scale at 15 yields 50@15 then 40@20; nonbinding standalone Clamp yields 100@15 then 90@20, mirrored. These are neither additive events nor declared replacements. The resolution does not require silently redefining these standalone operations. Their inclusion alongside an Add does not justify exempting that additive composition. |
| Composed body with explicit Decay | **Preserve explicit stage order and account endpoints once.** Own 20@5, body +5 then decay 3/6 at 12 gives 19@12. Writer saturation-sensitive 4@5 case gives 3@12; implicit settlement before the body is not equivalent. This specific boundary is justified; the blanket no-decay-body exemption is not. |

The double-charge argument is valid against *naively* prepending settlement: the body still
passes the original cell.updated_at to decay_walk. For 20@5, settling to 14, adding 5 and
walking the same two endpoints again gives 13, rather than 19. Nothing in the current code
marks those endpoints consumed inside the body. The writer's test commentary suggesting
that the declared stage would automatically have "nothing to do" is inaccurate. The
executed implicit-body-settlement mutant is a useful negative control, not proof that all
additive compositions may skip settlement.

## 4. Watchers, refusal, double charging and write-path accuracy

**Committed-to-committed watcher comparison is the correct retained reading.** V2 §4.2
requires complete reduction before one atomic commit; V3 §4.3 derives eligibility and parent
sets from complete predecessor groups. The settlement intermediate is neither a committed
state nor an effect. A falling watcher at 97 must see 100→95 at 15, not 90→95. Own repeated
crossing history gives alarm 1@15 then 2@25; writer effect-reapplication and replay tests pass.
No synthetic intermediate notification or backdated transition is authorized.

**Atomic refusal and no second charge pass on the settled ADDITIVE path.** plan_wave reads
settlement without mutating the store; reduce and validate_effect finish before apply_wave.
Own bounds refusal keeps bounded cell 9@0; a subsequent valid +1 at 10 settles both points
4/8 and produces 8@10. Coincident decay/shock at 20 leaves 100@0 with no committed effects;
a later +5 at 25 still settles endpoints 10/20 to 85@25. Same-time repeated additions and
barrier vectors establish that committed updated_at=now excludes previously charged endpoints.
FamilyMixture is unchanged and is not used as a settlement path.

The contract's two requested path claims are substantively correct. `effects::reduce`
consults `pre` only in ADDITIVE. Direct Host observations write only HostOwned cells;
write_path_of(HostOwned) returns None, and the evaluator rejects such targets before
emitting a decay effect. This is also covered by retained AT-I10 tests. Admission of a
rule declaration must not be confused with a successful evaluator write. Materialized
FrozenIntent::AddDelta is converted into the same canonical reduction path: the independent
scheduled materialization probe passes 95@15 and 85@20 in both directions.

One wording caveat: plan_wave eagerly *computes* settlement for every canonical target,
including replacement/transform targets, although only ADDITIVE consumes the result. Thus
"replacement never consults this" is accurate about reducer arithmetic, not about execution
of the helper. No separate reachable replacement refusal defect was established here.

## 5. Supersession and retained oracle dispositions

The original controlling own_probes.rs is byte-identical to its blob at e00f248. It runs
8 passing tests and one cargo-101 failure at `own_open_lost_prewrite_step`: actual -95@15
versus historical -105@15. The capture wrapper's exit 0 does not classify this as a pass.
The resolution-adapted file differs in exactly one literal on one line, 95→85 in the
no-pre-evaluation expected tuple table; it passes 9/9. The prior decision-adapted corpus
passes 9/9. E/custody-audit.json records the source hash and exact diff. No old lost-prewrite
assertion remains a positive requirement in crate tests. It remains historical evidence;
C2W-01 is a newly executed uncovered path, not permission to retain that old requirement.

Explicit oracle inheritance: inherit **every row in controlling review e00f248 §6**,
AT-I1–AT-I50 including I6a–d, I7a–e and I20b–d, with its exact S/P qualifications and scope.
Original unsplit I6/I7 and pre-adjudication I22/I23 remain superseded. Inheritance is sound
because all inherited tests/protected functions are byte-preserved and the entire workspace
suite was freshly rerun on cc3dc701. The changed dependencies are specifically rechecked
here: I1 effect reapplication; I7a/d and I11 reduction/refusal; I10 host boundary; I22′/I23′
grids/barriers; I25 committed crossings; I28/29/43 replay/restore/request timing; I34 suite
preservation; I36 static targets. Their prior finite support stands; **the added Operator
behavior-1 obligation is not fully supported**, so this is not an acceptance inheritance.

Retain C2-01–C2-08, C2R-01–04, C2R2-01 and D-C2-1–15 as dispositioned in e00f248, subject
to the new C2W-01 blocker on end-to-end resolution. R-1 and R-3–R-9 remain accepted (including
enqueue-canonicalization R-8, distinct from withdrawn architectural R-8). AT-I8/20b/21/28/33/
39/40 completions remain intact. C2-09 remains the history/finalization performance limit.
AT-I13 remains P: fixed-manifest support without definition migration. I39/40 are per-profile,
not shared runtime/cross-profile transaction support. I21 depth overflow uses the test seam;
I33 maximum is the configured finite bound. Replay/restore are in memory, with no durable
persistence, crash recovery, mailbox, transport, service or G.A.M.E. integration. Static
Windows/Android compilation is not runtime parity. No performance guarantee is added.

## 6. Fresh exact-final-candidate validation

All candidate checks ran while HEAD was cc3dc70182b428f7cbc953ae722f85b39d407dfa. Probes
use disposable external crates; mutants use git archives of that exact commit. E contains
commands, return codes, complete whitespace-normalized logs, scripts and source hashes.
Results: **415 workspace tests pass**; fmt, all-target/all-feature clippy, strict core/engine
library lint, metadata, all three whitespace ranges, all five static targets, preservation
and the release offline workload pass (15 required checks). Writer controls: 32 positive
baseline tests, 17 compiled mutants killed, 45 assertion kill runs and 2 specified prior-test
survivals. Reviewer controls: 11 passing baselines and 11 compiled assertion kills.
Independent probes: 8 passes (including materialization) and the 2 C2W-01 failures. Historical
own corpus: 8 passes/1 superseded failure; resolution-adapted and decision-adapted: 9/9 each.

| History | Build seconds | Indexed preflight µs | Scan control µs | Whole finalization ms |
|---|---:|---:|---:|---:|
| 1,000 | 0.907 | 6.931 | 9.605 | 1.850 |
| 4,000 | 13.943 | 2.511 | 92.463 | 6.707 |
| 16,000 | 231.999 | 5.020 | 2318.810 | 17.805 |

Measurements include concurrent build load; inherited O(history) finalization and pre-wave
store scans remain. E/validation-summary.json records checked totals. Expected historical
assertion failures and the two current blocking failures are reported separately.

Build settings bound storage: dev/test debug=0, incremental=0, two Cargo jobs; release
optimization is unchanged. Mutation archives use isolated targets in the final rerun.
An initial reviewer mutant run exposed stale shared-target baseline reuse; it was discarded
and rerun with one target directory per archive. Only the isolated-target results count.
Writer patch shapes are preserved; reviewer controls reuse useful shapes with new vectors
and add Clamp, absent-cell and refusal controls. Mutants are disposable controls, not repairs.

## 7. Exact next action

**Return cc3dc701 to the separated writer for bounded revision of C2W-01.** Implement and
test the approved additive settlement across the composed-body paths, clarify the contract
with a new record, and publish a revised candidate with fresh exact-final validation and
an independent-review mission. Then obtain separated independent review. Do not route this
back as an optional future policy question, treat disclosure as deferral authority, or
accept Gate C2 while the concrete additive-settlement failure remains. No production
promotion or Phase-3 work follows from this review.
