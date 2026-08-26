# S.P.A.R.K. Phase 2 — Opus Architecture Freeze v3

**Date:** 2026-08-26
**Agent:** Claude Code (Opus), Phase-2 bounded architecture-correction gate (pass 3)
**Status:** ARCHITECTURE FREEZE v3. This document supersedes
`SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v2_2026-08-26.md` (retained unmodified as
history) **in exactly the sections named in §7**, after adjudicating the independent
Codex adversarial rereview
(`SPARK_PHASE_2_CODEX_ARCHITECTURE_V2_REREVIEW_2026-08-26.md`, verdict
`PHASE_2_ARCHITECTURE_V2_REVISE`). The rereview cleared every other v2 decision; those
decisions are **not reopened here**. Every v1 and v2 section not revised below remains
frozen as written. This gate authorizes **no** production Rust, **no** Phase-2
implementation writer, **no** Phase-3 work, and reopens **no** Phase-0/Phase-1 contract.
The Phase-2 writer pass, when and if the operator authorizes it, is bound to this
document and to `SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v3_2026-08-26.md`.

**Scope of this pass (bounded):** exactly two Codex blocking defects —
**B01** (pacing progress for an oversized but semantically valid earliest cohort) and
**B02** (replay-stable later-wave multi-parent emission identity) — plus the direct
consistency edits those two corrections force. Nothing else.

**Provenance correction (metadata only, 2026-08-26).** This file is the **canonical**
v3 architecture freeze. It supersedes
`SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v3_2026-08-26.md` **for provenance purposes
only**: that file recorded the producing agent as Claude Code (Fable), which is
incorrect. The v3 architecture pass was performed by **Claude Code running Opus**. The
mis-attributed file is retained on disk and in Git history, unmodified, as historical
evidence; no Git history is rewritten.

The correction is confined to the document title, the **Agent:** line, this note, and
the companion matrix's pointer to this filename. **No architecture decision, adjudication,
prescription, consistency edit, section, or acceptance-test requirement is changed,
reinterpreted, expanded, or weakened by this correction**, and the verdict in §9 remains
`PHASE_2_ARCHITECTURE_FROZEN_V3`. Substantive references elsewhere in this document to
"the Opus writer" continue to denote the *future Phase-2 implementation writer pass*,
which remains unauthorized; that usage predates and is unaffected by this correction.
This correction pass ran no architecture research, reopened no decision, and wrote no
production Rust.

---

## 1. Lineage verification (gate precondition)

- Branch: `phase1-refoundation-v2`.
- HEAD at gate entry: `d07ed38e825e4a0d58615579617e8cba34e1a21e`
  ("Record Phase 2 architecture v2 adversarial rereview"); clean worktree.
- Ancestry confirmed: the Phase-1 closure commit `20a1c66`, the v1 Phase-2 freeze
  commit `87ba1c2`, the first Codex adversarial review commit `905880e`, and the v2
  freeze commit `9ff9d9b` are all ancestors of the entry HEAD. History is additive and
  linear; nothing is rewritten by this gate.
- Phase 1 remains **CLOSED** (`PHASE_1_CLOSED`). No Phase-1 type, authority door,
  scheduler, timeline, hashing, encoding, or activation invariant is reopened. Every
  Phase-1 canonical encoding and every pinned digest value remains bit-identical under
  this revision, because this revision touches only Phase-2 constructions that have no
  implementation and therefore no pinned value.
- Prior architecture artifacts (v1 freeze, v1 matrix, v2 freeze, v2 matrix, both Codex
  reviews) are preserved byte-for-byte as historical evidence. This pass creates two new
  files and edits none.

### 1.1 Controlling record re-verified before adjudication

The Codex rereview was treated as independent adversarial evidence, not as sovereign
truth. Every controlling citation it relies on was re-read and confirmed against the
canonical record before any prescription was adopted:

| Controlling source | What it actually says | Bearing |
|---|---|---|
| ADR-0003 §15 | "quota/yield changes latency, not semantics"; **"scheduled occurrence identity derives from persisted logical occurrence indexes, never batching/worker/catch-up chunks"**; "snapshots represent complete stable boundaries" | Directly decisive for B02: a causal identity may not be a function of the drain-call/batching partition. Also decisive for B01's canonical/noncanonical split. |
| ADR-0003 §14 | command barrier = `(timeline_epoch, input_ordinal, semantic hash, covering fence hash)` after finality | Confirms the command-cohort identity retained in §4.2 is already frozen and unambiguous. |
| Perf/security budget §3.1–§3.5 | every internal queue has a finite declared admission limit; a runtime work quota does not change canonical ordering, RNG occurrence indexes, or accepted-command semantics; **"due work that is safely deferrable is resumed in the same deterministic order"**; a queue that cannot safely defer rejects atomically before partial commit | §3.3 is the requirement B01 violates: indefinite deferral is not "resumed". §3.2 is what forces pacing diagnostics out of canonical state. |
| Perf/security budget §3.7 | "Backlog age, rejection, aggregation, and **deferred counts are telemetry-visible**" | Confirms deferral reporting is a required *telemetry* obligation — i.e. exactly the noncanonical class B01 needs, not a canonical semantic report. |
| R-038 | due-work evaluation with "ten-days-once versus one-day-ten-times" catch-up equivalence; "quota yields causally inert" | Independently forces per-due-time cohort identity (§4.1) and forbids drain-partition-sensitive identity even with pacing disabled. |
| R-044 | deterministic waves; "concurrent/split batch equivalence" | Split-batch equivalence is unattainable under a drain-set-derived identity; §4.5 repairs it. |
| R-096 | internal queues and delayed expansion are finitely bounded with deterministic backpressure/rejection | Confirms semantic caps remain authoritative and terminal (v2 §6.3 upheld unchanged). |
| ADR-0006 / R-055, R-056, R-098 | bounded provenance with explicit coverage metadata and deterministic pruning identity | Unchanged; §4.3 parent sets are transient and never become a provenance or retention mechanism. |
| Phase-1 source seam (`crates/spark-core/src/scheduler.rs:118–150`) | `WorkKey` = `(due_time, profile_id, producer_definition_id, scope_id, occurrence_index, work_kind)` with derived `Ord` making `due_time` primary and `profile_id` second; `identity_digest()` = `H("work_key_identity" ‖ canonical encoding)` | Confirms the cohort-identity inputs in §4.1 already exist and that the (due_time, profile) cohort partition of §3.1 is a **contiguous** slice of the existing total order — no new ordering is invented. |
| Phase-1 source seam (`Scheduler::drain_due`, `scheduler.rs:474–490`) | removes every slot with `due_time <= now`, relying on `due_time` primacy | Confirms deferred cohorts are structurally drained before newer work with no extra retained state (v2 §6.2 upheld). |

No citation in the Codex rereview was found inaccurate. Two of its prescriptions are
adopted with substantive modification (§2), for reasons stated there.

---

## 2. Adjudication of the two Codex blocking findings

Legend, unchanged from v2: **ACCEPTED** = adopted as stated (concretized where Codex
deliberately left an encoding or boundary open); **MODIFIED** = adopted with a
substantive change, with architectural reasoning; **REJECTED** = not adopted, with
architectural reasoning.

### 2.1 B01 — oversized valid earliest cohort can starve forever

Codex's counterexample is confirmed and reproduced against the v2 text: with
`max_due_per_cycle = 4` and a six-`WorkKey` cohort at `due_time = 100` whose evaluation
stays inside every semantic cap, v2 §6.2 ("defers only whole cohorts") plus v2 §6.1
("a budget may never split a cohort") admits nothing, forever. v2 §6.3 does not rescue
it, because §6.3 fires only on *semantic* cap overflow, which this cohort does not
reach. This violates budget §3.3 (deferrable work is *resumed*) and R-038 (quota yields
are causally inert — an infinite yield is not inert, it is a permanent semantic change).

| # | Codex B01 prescription | Ruling |
|---|---|---|
| B01-P1 | validate `max_due_per_cycle >= 1` at activation | **ACCEPTED** — §3.2(a). Concretized: `max_due_per_cycle` is already an epoch-bound declared value validated at the activation door (v1 Q4, upheld), so `0` is refused there, atomically, with no partial `ActivatedRuleSet`; there is no runtime path that can reach `0`. |
| B01-P2 | normally admit the maximal ascending prefix of whole cohorts fitting the pacing budget | **ACCEPTED** — §3.2(b). Concretized: the ascending order is the cohort sequence of §3.1 (ascending `(due_time, profile_id)`), the budget is counted in `WorkKey`s (as in v1 Q4), and "maximal prefix" is the unique greedy prefix — no search, no packing, no reordering. |
| B01-P3 | if no cohort fits *solely because the earliest legal cohort itself exceeds the pacing budget*, admit exactly that earliest cohort whole | **ACCEPTED with one MODIFICATION** — §3.2(c). The modification: the word **"legal" is dropped from the selector's precondition**. Legality (whether a cohort's candidate volume stays inside `max_cohort_candidates`, `max_effects_per_wave`, `max_enqueue_per_wave`, queue admission, fan-out and depth caps) is *only knowable after evaluation*, and v2 §7 forbids any decision that depends on state or work produced later in the same transition. A selector that predicated admission on legality would have to evaluate the cohort to decide whether to admit it — circular, and a direct contradiction of the frozen preflight ordering. The corrected selector is therefore **purely structural**: it counts `WorkKey`s and nothing else. Progress is preserved in both outcomes: a structurally-admitted cohort either evaluates within every semantic cap and commits, or exceeds one and is **terminally consumed** under the already-frozen v2 §6.3 disposition. Either way the queue advances, which is the property B01 actually demands. |
| B01-P4 | record deterministic noncanonical pacing-overrun diagnostics | **ACCEPTED** — §3.4. Concretized into a typed `PacingDiagnostics` block with a frozen field list, explicitly excluded from every canonical digest and structurally unreadable by rule evaluation. Budget §3.7 already requires deferred counts to be telemetry-visible; this places that obligation in the noncanonical channel where §3.2's quota-neutrality rule permits it to live. |
| B01-P5 | defer all later cohorts untouched | **ACCEPTED** — §3.2(d), unchanged from v2 §6.2 and structurally guaranteed by `WorkKey` due-time primacy. |
| B01-X | (Codex §6.1 second contradiction) v2 §6.2 simultaneously requires deferral to be visibly reported *and* requires paced and unbudgeted runs to return identical reports | **ACCEPTED as a defect; resolved by §3.4** — the two claims are irreconcilable for a single undifferentiated report value. v2 §6.2's equivalence claim is corrected to range over the **canonical** semantic report only. Codex is right that an honest latency diagnostic must be permitted to differ; ADR-0003 §15 protects *semantics*, not telemetry. |

**Nothing in B01 is rejected.** Semantic caps remain authoritative and their disposition
(v2 §6.3, terminal consumption with a typed order-independent overload report) is
carried forward unchanged.

### 2.2 B02 — later-wave emission identity has no stable multi-parent context

Both Codex counterexamples are confirmed. (i) v2 §3.2's emission context requires a
singular "triggering `WorkKey` identity digest", but a wave-1 threshold emission caused
by the fully reduced set `{A: +10, B: +15}` has no truthful single trigger; every way of
picking one is either nondeterministic (evaluation order) or deterministic-but-arbitrary
(lexical minimum), and the arbitrary choice changes when an unrelated lower-sorting
parent is added. (ii) v2 inherits v1's heartbeat barrier identity
`H("heartbeat" ‖ logical time ‖ digest of drained WorkKeys)` and chains waves to it,
so the emission identity of otherwise identical scheduled work varies with how many
drain calls the pacing budget required.

Finding (ii) is **worse than Codex states**, and this pass records the extension as
independent confirmation rather than as new scope: the drained-`WorkKey` component
breaks R-038's catch-up equivalence *even with pacing entirely disabled*. Evaluating ten
days of due work in one drain produces a different drained-`WorkKey` digest — and
therefore different emission identities, obligation record hashes, and occurrence
mappings — than evaluating one day ten times, which is exactly the equivalence R-038
freezes. ADR-0003 §15's "scheduled occurrence identity derives from persisted logical
occurrence indexes, **never batching/worker/catch-up chunks**" is dispositive. The
heartbeat component was never admissible as causal identity.

| # | Codex B02 prescription | Ruling |
|---|---|---|
| B02-P1 | define a deterministic scheduled-cohort identity from profile, due time, count, and the ascending complete `WorkKey` identity digests | **ACCEPTED** — §4.1, adopting Codex's field list verbatim with domain-separated canonical encoding. This forces **one consistency edit** (§7 CE-1): v2 §6.1 defined the cohort as "all due work items sharing one logical `due_time`", which is not well-defined under a singular `profile` component when a boundary drains work for more than one profile. The cohort is therefore per-`(profile_id, due_time)`. This is a narrowing, not a new decision: waves are already per-profile in v2 §5.2 (`effect_batch_v2 ‖ profile ‖ behavior_epoch ‖ …`), a behavior epoch is per-profile by construction (v1 Q5), and a cause in profile P structurally cannot target a cell in profile Q (targets are `(profile_id, definition_id, scope_id)` and the ADR-0002 write doors validate the profile). The partition therefore **cannot split a co-target causal group**, and because `WorkKey`'s derived `Ord` is `due_time` then `profile_id`, each cohort is a *contiguous* slice of the already-frozen total order. |
| B02-P2 | retain the finalized command barrier identity for command cohorts | **ACCEPTED** — §4.2, unchanged from ADR-0003 §14 and v1. Command barrier identity is a function of finalized timeline position, not of any drain partition, so it needs no repair. |
| B02-P3 | derive an immediate parent-set digest from the complete sorted set of parent emission identities that made the derived operation eligible | **ACCEPTED with one MODIFICATION** — §4.3. Codex wrote "the fully reduced predecessor target group(s)"; this pass fixes the boundary Codex left open, because taken literally it is ambiguous for an operation that reads a target with **no** candidates this wave. Frozen rule: the parent set is the union, over every target group the operation read in order to become eligible, of the **canonicalized candidate emission identities in that group** — a read target with no candidates in this wave contributes nothing. Two supporting rulings make this safe: (a) a derived (wave ≥ 1) emission with an **empty** parent set is a typed evaluator defect that rejects the wave atomically, because an operation cannot newly become eligible in wave N+1 without at least one wave-N cause; and (b) unchanged committed background state is already fully committed to by the batch digest's pre-wave engine digest (v2 §5.2, unchanged), so mechanism replay stays totally discriminated without smuggling background state into causal identity — which would re-import exactly the payload/state coupling the v2 correction removed. |
| B02-P4 | scheduled wave 0: the triggering `WorkKey` may remain the leaf context | **ACCEPTED** — §4.3, encoded as a domain-separated singleton so that a wave-0 leaf can never collide with a wave-N parent set of size one. |
| B02-P5 | command wave 0: the finalized command identity/tag may remain the leaf context | **ACCEPTED** — §4.3, same domain separation. |
| B02-P6 | wave N+1 and later threshold/propagation emissions use the complete parent-set digest rather than selecting one parent | **ACCEPTED** — §4.3/§4.4. Selection of any single parent — by evaluation order, rule order, lexical minimum, or any other tiebreak — is **forbidden by construction**: no selection function exists anywhere in the frozen pipeline. |
| B02-P7 | derived emission context = cohort identity ‖ wave index ‖ parent-set digest, then the frozen rule fingerprint, operation sub-ID, producer scope, target, and behavior artifact | **ACCEPTED** — §4.4, adopting the component order verbatim. |
| B02-P8 | parent sets remain transient/recomputable from already canonicalized candidates and reduction groups; never hidden retained state | **ACCEPTED** — §4.6, folded into the v2 §12 transient-buffer row and falsified by the R1/R2 discipline the readiness blueprint makes non-negotiable. |
| B02-P9 | scheduled causal identity must not depend on heartbeat/drain-call partition or on pacing diagnostics | **ACCEPTED** — §4.5. This forces **one consistency edit** (§7 CE-2, wave-index scoping) and retires the heartbeat descriptor to noncanonical telemetry. As recorded above, this prescription is required by ADR-0003 §15 and R-038 independently of pacing, which is stronger grounds than Codex claimed. |

**Nothing in B02 is rejected.**

### 2.3 Adjudication tally

9 prescriptions ACCEPTED as stated (several with encoding/boundary concretization),
2 ACCEPTED with substantive MODIFICATION (B01-P3, B02-P3), 0 REJECTED. Both of the
rereview's supplementary observations (B01-X report contradiction; the pacing-partition
half of B02) are accepted as defects and resolved. No prescription was watered down, and
no cleared v2 decision was reopened.

---

## 3. B01 resolved — the frozen pacing selector

*Revises v2 §6.1 (cohort scoping only) and v2 §6.2 (selector and reporting). v2 §6.3 is
carried forward unchanged.*

### 3.1 The cohort (frozen; narrowed from v2 §6.1)

The atomic evaluation cohort is the **equal-`(profile_id, due_time)` slice**: all due
work items sharing one profile and one logical `due_time`, drained at one canonical
boundary, together with the candidate effects their evaluation emits at wave 0.

- The **cohort sequence** of a boundary is its cohorts in ascending `(due_time,
  profile_id)` order — a contiguous, canonical partition of the existing `WorkKey` total
  order, invented by nothing new.
- Each cohort evaluates against the committed state left by the previous cohort in that
  sequence (v2 §6.1, upheld).
- Same-cohort candidates are always reduced together. **No budget of any kind may split
  a cohort** (v2 §6.1, upheld and unweakened).
- Command-barrier rule evaluation forms its own cohort at that barrier (v2 §6.1,
  upheld); its identity is §4.2.

### 3.2 The pacing selector (frozen)

`max_due_per_cycle` is pacing, not semantics (v2 §6.2, upheld). For one drain cycle,
against the boundary's cohort sequence:

- **(a) Activation validation.** `max_due_per_cycle >= 1`. A declared value of `0` is
  refused at the activation door, atomically, with no partial activation artifact.
  Because the value is epoch-bound (v1 Q4), no runtime path can reach `0`.
- **(b) Normal admission.** Admit the **maximal ascending prefix of whole cohorts**
  whose cumulative `WorkKey` count is `<= max_due_per_cycle`. This is the unique greedy
  prefix: cohorts are taken in cohort-sequence order until the next one would not fit.
  There is no packing, no skipping, no reordering, and no lookahead.
- **(c) Oversized-earliest-cohort progress exception.** If, and only if, the prefix
  from (b) is **empty**, admit **exactly the earliest cohort, whole**, and admit no
  other cohort this cycle.
  - An empty prefix is equivalent to "no cohort has been admitted this cycle", so the
    full budget was available and the earliest cohort's `WorkKey` count strictly exceeds
    `max_due_per_cycle`. That is the only way (b) can yield nothing while due work
    exists.
  - The exception therefore fires **at most once per cycle** and **only for the earliest
    cohort in the sequence**.
  - The selector is **purely structural** — it counts `WorkKey`s. It does not, and
    cannot, predicate on semantic legality (B01-P3 modification, §2.1).
- **(d) Deferral.** Every cohort after the admitted set is left **scheduled and
  untouched** — already-canonical state, digest-checkable, not copied into any buffer.
  Because `WorkKey` orders by `due_time` first, deferred cohorts are structurally drained
  before any newer work; continuation priority needs no retained state (v2 §6.2,
  upheld).

### 3.3 Progress (the property B01 demands)

**Claim.** For any activated artifact and any non-empty due-work set, every drain cycle
admits at least one whole cohort, and every cohort is admitted after finitely many
cycles.

**Derivation.** By (a), `max_due_per_cycle >= 1`. If (b)'s prefix is non-empty, at least
one cohort is admitted. If it is empty, (c) admits the earliest cohort. So every cycle
with due work admits `>= 1` cohort. An admitted cohort leaves the queue by exactly one
of two terminal routes: it evaluates within every semantic cap and commits (its
`WorkKey`s consumed), or it exceeds a semantic cap and is **terminally consumed** under
v2 §6.3 with a typed order-independent overload report and its keys freed. Neither route
returns the cohort to the head of the queue. Cohorts strictly ahead of a given cohort C
therefore strictly decrease each cycle, so C is admitted within finitely many cycles.
**No cohort can be deferred forever, and no cycle can return a deferral report without
consuming work.** This is exactly budget §3.3's "safely deferrable work is resumed in the
same deterministic order", restored.

### 3.4 Canonical semantics versus noncanonical pacing diagnostics (frozen; resolves B01-X)

Two structurally distinct return channels; the v2 §6.2 equivalence claim ranges over the
first only.

**Canonical semantic report (participates in every equivalence claim).** Cohort identity
(§4.1/§4.2), wave index, committed effects and their resolved values, threshold
emissions, obligations created, occurrence allocations, cooldown writes, atomic
rejections and their order-independent evidence, and semantic cohort/wave cap outcomes
including v2 §6.3 terminal-overload reports. Over an identical canonical input history,
this report — together with cells, scheduler, obligation store, occurrence ledger,
cooldowns, batch digests, emission identities, and the final engine digest — is
**bit-identical between paced and unbudgeted execution**.

**Noncanonical pacing diagnostics (`PacingDiagnostics`; expected to differ).** Emitted
per drain call, with frozen fields:

```text
declared_max_due_per_cycle
admitted_cohort_count
admitted_work_key_count
admitted_cohort_identities      (ascending, cohort-sequence order)
pacing_overrun                  (bool: the §3.2(c) exception fired this cycle)
overrun_cohort_identity         (present iff pacing_overrun)
deferred_cohort_count
earliest_deferred_due_time      (present iff deferred_cohort_count > 0)
```

Binding rules:

1. `PacingDiagnostics` is **deterministic** given (canonical input history, declared
   budget, drain-call sequence). It is not nondeterministic, merely partition-dependent.
2. It participates in **no** canonical digest: not the engine digest, not
   `effect_batch_digest`, not any emission identity, not any obligation record hash, not
   occurrence allocation order, not provenance, and not any retained store.
3. It is **causally inert** (R-038): no rule operation, effect, threshold, obligation,
   cooldown, or scheduler decision may read it. This is a structural property enforced at
   the type boundary, not a convention — the evaluator is not given the value.
4. A paced run truthfully reports deferral/overrun; an unbudgeted run truthfully reports
   neither. **This difference is correct and required**, and is the honest telemetry
   budget §3.7 mandates. It is never evidence of a semantic divergence.

### 3.5 Relation to semantic caps (v2 §6.3 carried forward unchanged)

`max_cohort_candidates`, `max_effects_per_wave`, `max_wave_depth`,
`max_enqueue_per_wave`, per-rule fan-out bounds, and every queue admission cap remain
**epoch-bound declared semantics**, not pacing. A cohort whose candidate transition
exceeds one rejects atomically before any canonical mutation (v2 §7), and its `WorkKey`s
are then consumed with a typed overload report in conflicted-drain shape (v2 §6.3). The
outcome is identical on every replay of the same history under the same artifact.

The B01 correction changes nothing here. A structurally oversized cohort admitted under
§3.2(c) is **not** thereby excused from any semantic cap: it is admitted, evaluated, and
judged exactly like any other cohort. Pacing never splits a cohort, never converts legal
work into overload, and never converts overload into legal work.

---

## 4. B02 resolved — total, replay-stable causal emission identity

*Revises v2 §3.2 (emission context) and v2 §5.2 (batch identity). Every other component
of the v2 emission-identity formula is carried forward verbatim.*

### 4.1 Scheduled cohort identity (frozen; new)

```text
scheduled_cohort_identity =
  H("scheduled_cohort_v1" ‖ profile_id
                          ‖ due_time
                          ‖ work_key_count
                          ‖ WorkKey identity digests, ascending, each length-prefixed)
```

- Inputs are exactly the fields Codex prescribed, all of which already exist:
  `WorkKey::identity_digest()` and the `WorkKey` field set are frozen Phase-1 surfaces
  (`crates/spark-core/src/scheduler.rs:118–150`).
- Encoding uses the Phase-1 `CanonicalEncoder` discipline with a domain-separation tag
  and an explicit count; the count is redundant with the digest list but is retained as
  the standard concatenation-ambiguity guard.
- The identity names the **entire** equal-`(profile, due_time)` cohort. It is therefore
  a function of *which work is due*, never of which drain call admitted it, how many
  drain calls the boundary required, how large the pacing budget was, or whether pacing
  was enabled at all.

### 4.2 Command cohort identity (frozen; unchanged)

A command cohort's identity remains the **finalized command barrier identity** of
ADR-0003 §14: `(timeline_epoch, input_ordinal, semantic hash, covering fence hash)`.
It is a function of finalized timeline position only, and needs no repair. Scheduled and
command cohort identities are domain-separated and cannot collide.

### 4.3 Immediate parent-set digest (frozen; new)

Every emission carries exactly one parent context, determined by its wave index and
trigger class:

```text
wave 0, scheduled work:
  parent_context = H("parents_workkey_v1" ‖ triggering WorkKey identity digest)

wave 0, command work:
  parent_context = H("parents_command_v1" ‖ finalized command identity/tag)

wave N >= 1, derived (threshold / propagation) emissions:
  parent_context = H("parents_emission_v1" ‖ parent_count
                                           ‖ parent emission identities,
                                             ascending, each length-prefixed)
```

**The parent set (frozen).** For a derived emission at wave `N >= 1`, the parent set is
the **union, over every target reduction group the operation read in order to become
eligible, of the canonicalized candidate emission identities in that group** — the
complete set, in ascending canonical order, deduplicated by identity (an exact duplicate
parent appears exactly once, consistent with v2 §3.2 canonicalization).

Binding consequences:

1. **No parent is ever selected.** There is no tiebreak, no minimum, no first, no
   evaluation-order winner. No selection function exists in the pipeline. Adding or
   renaming a lexically lower-sorting contributing parent changes the *set* — which is
   correct, because it genuinely is a different cause set — and can never make that
   parent an arbitrary identity winner.
2. **A read target with no candidates in this wave contributes no parents.** Its value is
   unchanged committed state; it is already committed to by the pre-wave engine digest
   component of `effect_batch_digest` (v2 §5.2, unchanged), which is where background
   state belongs. Background state is deliberately kept out of causal identity, exactly
   as numeric payload is (v2 §3.2), so that identity discriminates *causes* rather than
   coincidences of value or of history.
3. **An empty parent set at wave `N >= 1` is a typed evaluator defect** that rejects the
   wave atomically with nothing mutated. An operation cannot newly become eligible in
   wave `N+1` without at least one wave-`N` cause; an empty set means the evaluator lost
   a parent, and a lost parent must never be papered over with a degenerate identity.
4. **Depth composes transitively.** A wave-2 emission's parents are wave-1 *emission
   identities*, each of which already embeds its own parent-set digest. The chain from
   any derived emission back to its wave-0 leaves is therefore complete and
   hash-committed, with no per-wave accumulator and no retained lineage store.
5. **Multi-target operations** take the union over every group read. There is no
   per-target arbitrary choice.

### 4.4 The corrected emission identity (frozen; replaces v2 §3.2's context)

```text
emission_identity = H("emission" ‖ emission context
                                 ‖ rule fingerprint (within ruleset_content_hash lineage)
                                 ‖ effect-operation qualified sub-ID
                                 ‖ producer scope_id
                                 ‖ target (definition_id, scope_id)
                                 ‖ behavior_artifact_hash)

emission context  = cohort identity (§4.1 scheduled | §4.2 command)
                    ‖ wave_index
                    ‖ parent_context (§4.3)
```

Everything after `emission context` is carried forward **verbatim** from v2 §3.2 and is
not reopened. Identity still **never** includes the numeric payload. The v2 idempotency
contract is likewise carried forward verbatim: within one wave, exact duplicates (same
identity, same payload) fold to one candidate; one identity carrying distinct payloads is
a contested emission that rejects the wave atomically; across barriers, re-emission is
structurally precluded by the Phase-1 scheduler.

**Discrimination.** Two derived emissions from the same rule operation, same producer
scope, same target, same cohort, and same wave collapse to one identity **iff** their
parent sets are identical — i.e. iff they are genuinely the same causal emission, which
is precisely when folding is correct. Different parent sets yielding the **same numeric
result** remain identity-distinct, because the parent-set digest is in the context and the
payload is not.

**Uniqueness.** Two *different* derived emissions cannot collide: they differ in at least
one of rule fingerprint, operation sub-ID, producer scope, target, wave index, cohort, or
parent set, and every one of those is a distinct hashed component under domain separation.

### 4.5 Batch identity and wave chaining (revises v2 §5.2)

```text
effect_batch_digest = H("effect_batch_v3" ‖ profile ‖ behavior_epoch
                        ‖ cohort identity            (§4.1 | §4.2 — replaces barrier identity)
                        ‖ pre-wave engine digest
                        ‖ wave_index                 (cohort-scoped, §7 CE-2)
                        ‖ canonical candidate-set digest
                        ‖ committed effect count ‖ each committed effect block)
```

The canonical candidate-set digest (v2 §5.2) is unchanged: `H` over the canonicalized
candidate multiset in ascending emission-identity order. Because emission identities are
now total (§4.4), that digest is now fully defined for every wave, which is what v2 §5.2,
§9, and §12 assumed but could not derive.

**The heartbeat descriptor `H("heartbeat" ‖ logical time ‖ digest of drained WorkKeys)`
is retired from canonical use.** It may survive only as a noncanonical drain-call label
inside `PacingDiagnostics` (§3.4), which participates in no digest. This is required by
ADR-0003 §15 and by R-038's catch-up equivalence, independently of pacing.

**Partition independence (now derivable, where v2 merely asserted it).** For any cohort
`C`:

- its cohort identity depends only on `(profile, due_time, its WorkKeys)` — §4.1;
- its pre-wave engine digest is the committed state after every cohort strictly earlier
  in the cohort sequence, and cohorts are always processed in that ascending order
  regardless of how drain calls partition them — §3.1/§3.2;
- its wave indices are cohort-scoped — §7 CE-2;
- its emission identities are functions of the above plus frozen artifact components —
  §4.4.

Therefore every canonical value produced by `C` is identical whether `C` is processed
alone, with earlier cohorts in one drain, or after any number of paced deferrals. This
simultaneously delivers R-038 catch-up equivalence, R-044 split-batch equivalence, and
v2 §6.2's paced/unbudgeted claim — all three from one construction.

### 4.6 Parent sets are transient (frozen)

Parent sets are **recomputed** within the wave from the canonicalized candidate multiset
and reduction groups that v2 §5.1 already produces. They are not stored, not persisted,
not carried across barriers, and not committed to any retained store. They fall under
the v2 §12 transient-buffer row and are falsified by the standard R1/R2 discipline:
after any committed or rejected cohort, fresh reconstruction from committed inputs is
digest-identical and next-barrier-identical.

There is likewise **no retained pacing continuation state**: deferred cohorts remain in
the scheduler as already-canonical `WorkKey`s (§3.2(d)), and a reconstruction at a stable
boundary needs nothing else to reproduce every subsequent emission identity.

### 4.7 Required invariance properties (the falsification targets)

The corrected identity is **identical** across:

| Perturbation | Why it cannot matter |
|---|---|
| `WorkKey` insertion permutations | cohort identity hashes an ascending digest set; the scheduler's order is semantic, not arrival-based (Phase-1, closed) |
| rule declaration/ID permutations | rule fingerprint is content-addressed within `ruleset_content_hash`; ordering sorts groups and reports only (v2 AT-I5, upheld) |
| candidate enumeration permutations | canonicalization is by emission identity before reduction (v2 §5.1 step 2) |
| parent enumeration permutations | the parent set is a set, hashed in ascending canonical order |
| paced versus unbudgeted execution | §4.5 partition independence |
| drain-call partition / catch-up chunking | §4.1, ADR-0003 §15, R-038 |
| fresh reconstruction at stable boundaries | §4.6 |

The corrected identity is **distinct** across: different parent sets (even with equal
numeric results), different cohorts, different wave indices, different rule fingerprints,
different operation sub-IDs, different producer scopes, and different targets.

---

## 5. Worked resolution of the two Codex counterexamples

**B01.** `max_due_per_cycle = 4`; cohort `K0..K5` at `due_time = 100` (six `WorkKey`s,
inside every semantic cap); next cohort at `due_time = 101`.
Cycle 1: §3.2(b) prefix is empty (6 > 4) → §3.2(c) admits `{K0..K5}` whole, exactly once;
the `due_time = 101` cohort is untouched; `PacingDiagnostics` reports
`pacing_overrun = true`, `admitted_work_key_count = 6`,
`declared_max_due_per_cycle = 4`, `overrun_cohort_identity = <§4.1 digest>`,
`deferred_cohort_count = 1`. Canonical semantics equal the unbudgeted run exactly. Cycle
2 proceeds to `due_time = 101`. **No starvation, no split, no livelock.**

**B02.** Pre-wave `stress = 20`; `WorkKey A → AddDelta(+10)`, `WorkKey B → AddDelta(+15)`,
both in the cohort at `due_time = 100`; fully reduced `stress = 45`; threshold `40`
crosses and emits obligation `O` in wave 1.
Wave-0 emissions: `e_A` with `parent_context = H("parents_workkey_v1" ‖ id(A))`, `e_B`
likewise with `id(B)`. Wave-1 emission `O`:

```text
cohort identity  = H("scheduled_cohort_v1" ‖ profile ‖ 100 ‖ 2 ‖ [id(A), id(B)] ascending)
wave_index       = 1
parent_context   = H("parents_emission_v1" ‖ 2 ‖ [e_A, e_B] ascending)
```

No parent is chosen. Permuting `A`/`B` insertion, rule order, candidate enumeration, or
parent enumeration yields the identical digest. A different cause set that also reduces
to `45` — say `{+20, +5}` from different emissions — produces a **different** `O`
identity, because the parent set differs and the payload never entered the hash. Adding a
lexically lower third contributor changes the set (correctly) rather than hijacking the
identity. Splitting the boundary into two drain calls changes nothing, because the
heartbeat drain set is no longer in any canonical digest.

---

## 6. Scope discipline actually observed

- **No new causal primitive.** Cohort identity, parent-set digest, and the pacing
  selector are all *derived functions* of already-frozen data: `WorkKey` identity digests,
  emission identities, canonicalized candidate/reduction groups, and declared epoch-bound
  budgets. Nothing new is retained, scheduled, or made addressable.
- **No general scripting.** The declarative operation vocabulary (blueprint §19.3) is
  untouched.
- **Phase 1 is not reopened.** No Phase-1 type, encoding, digest, or invariant changes.
  `WorkKey`, `Scheduler`, `DrainOutcome`, timeline, hashing, and the authority doors are
  consumed exactly as closed.
- **Phase 2 is not implemented.** No production Rust was written by this pass.
- **Phase 3 is not authorized** and nothing here pre-authorizes it.
- **No cleared v2 decision is reopened.** The corrected same-target effect semantics
  (v2 §§3–5), preflight (v2 §7), obligation claim sets and multi-target binding (v2 §8),
  occurrence allocation ordering (v2 §9), provenance coverage (v2 §10), the delayed-work
  boundary (v2 §11), the retained-store commitment law (v2 §12), and every v1 resolution
  upheld by v2 §13 stand as written.
- **All prior artifacts preserved** byte-for-byte as historical evidence.

---

## 7. The complete list of consistency edits (nothing outside this list changed)

| # | Edit | Forced by | Prior text |
|---|---|---|---|
| CE-1 | The cohort is the equal-**`(profile_id, due_time)`** slice; the cohort sequence is ascending `(due_time, profile_id)` | B02-P1 (cohort identity has a singular `profile` component) | v2 §6.1 "all due work items sharing one logical `due_time`" |
| CE-2 | `wave_index` and `max_wave_depth` are **cohort-scoped**, not drain- or barrier-scoped | B02-P9 (identity must not vary with drain partition) and R-038 (a drain-scoped depth budget would make ten-days-once differ from one-day-ten-times) | v1 Q3 "waves within one barrier are numbered `wave_index = 0..max_wave_depth`" |
| CE-3 | `effect_batch_digest` binds **cohort identity** in place of barrier identity, and its tag advances to `effect_batch_v3` | B02-P9 | v2 §5.2 `"effect_batch_v2" ‖ … ‖ barrier identity ‖ …` |
| CE-4 | The heartbeat descriptor is retired from canonical use and survives only as a `PacingDiagnostics` label | B02-P9 | v1 Q3 heartbeat barrier identity |
| CE-5 | The v2 §6.2 paced/unbudgeted equivalence claim ranges over the **canonical semantic report** only; pacing diagnostics are a separate noncanonical channel | B01-X | v2 §6.2 "including … reports" |
| CE-6 | v2 §9 occurrence allocation ("sorted by complete emission identity") is now **well-defined for every wave**, because §4.4 makes emission identity total | B02 (v2 §9 depended on an identity that was undefined at wave ≥ 1) | v2 §9, unchanged in substance |

No pinned digest value moves, because no Phase-2 implementation exists and no Phase-1
encoding is touched (CE-3's tag change is pre-implementation).

---

## 8. Freeze summary (delta over v2)

| Item | v3 resolution |
|---|---|
| Cohort | equal-`(profile_id, due_time)` slice; contiguous in the frozen `WorkKey` order; never split by any budget |
| Pacing selector | maximal ascending whole-cohort prefix; if that prefix is empty, admit the earliest cohort whole (structural test only); `max_due_per_cycle >= 1` at activation; later cohorts untouched |
| Pacing progress | every cycle with due work admits `>= 1` cohort; every admitted cohort leaves the queue by commit or by v2 §6.3 terminal consumption; no starvation, no livelock |
| Semantic caps | unchanged and authoritative; v2 §6.3 terminal rejection unchanged |
| Reporting | canonical semantic report (equal paced vs unbudgeted) strictly separated from noncanonical `PacingDiagnostics` (expected to differ; causally inert; in no digest) |
| Scheduled cohort identity | `H("scheduled_cohort_v1" ‖ profile ‖ due_time ‖ count ‖ ascending WorkKey identity digests)` |
| Command cohort identity | finalized command barrier identity (ADR-0003 §14), unchanged |
| Parent context | domain-separated: wave-0 scheduled `WorkKey` leaf, wave-0 command leaf, or wave-`N≥1` complete ascending parent **emission-identity set** digest |
| Parent selection | structurally impossible — no selection function exists |
| Emission context | cohort identity ‖ wave_index ‖ parent_context, then the v2 rule fingerprint / sub-ID / producer scope / target / artifact components verbatim |
| Batch digest | `effect_batch_v3`, binding cohort identity and cohort-scoped wave index |
| Drain partition | causally inert; heartbeat drain set removed from every canonical digest |
| Parent sets & pacing continuation | transient and recomputable; no new retained store |

---

## 9. Verdict

**PHASE_2_ARCHITECTURE_FROZEN_V3.**

Both Codex blocking defects are adjudicated and closed: B01 by a structural pacing
selector with a proven progress property and a strict canonical/noncanonical reporting
split; B02 by a total, parent-set-based, partition-independent causal emission identity
built entirely from already-frozen identities. Eleven prescriptions were adjudicated —
9 ACCEPTED, 2 ACCEPTED with substantive MODIFICATION (B01-P3's legality precondition,
B02-P3's parent-set boundary), 0 REJECTED — and six consistency edits are enumerated in
full. No new causal primitive, no scripting, no reopened v2 decision, no reopened Phase-1
contract, and no production Rust.

PHASE_2_IMPLEMENTATION_AUTHORIZATION: **NO** (separate operator decision; the Opus writer
is **not** authorized by this pass)
PHASE_3_AUTHORIZATION: **NO**
