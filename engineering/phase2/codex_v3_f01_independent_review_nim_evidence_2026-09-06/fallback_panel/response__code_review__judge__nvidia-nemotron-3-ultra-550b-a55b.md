# judge / nvidia/nemotron-3-ultra-550b-a55b

worker_id=dcj-5bc33106384f-w3 job_id=dcj-5bc33106384f task=code_review
ok=True latency_s=283.68 error_class=None

Let me analyze this carefully. I need to review the S.P.A.R.K. V3-F01 candidate design for defects. Let me break down the key components:

1. **Scheduler facts**: WorkKey order is (due_time, profile_id, producer, scope, occurrence, kind). `drain_due(now)` atomically removes slots with due_time <= now, returning scheduled items and conflicted evidence separately. Drained keys become free and can be rescheduled.

2. **Inherited contracts**:
   - v1 Q4 budgets over stable WorkKey prefix of DrainOutcome::due
   - R-038: ten-days-once vs one-day-ten-times equivalence
   - v2 section 11: wave-created obligations have due_time >= current logical time + 1; canonical boundary drains due work at most once; AT-I21: due_time >= now + 1
   - v3: evaluation cohorts = equal-(profile_id, due_time) due-work slices; identity from complete ascending WorkKey set; later cohorts must remain scheduled untouched
   - Pre-wave engine digest includes scheduler digest

3. **Candidate API**:
   - `due_cohort_view(&self, now)` -> borrowing view of all due cohorts
   - Returns owned `DueCohortSelection` with private (due_time, profile_id, count, key-only digest)
   - `take_due_cohort(&mut self, selection)` -> Result, no `now` parameter
   - Errors: CohortAbsent, CohortMembershipChanged, CohortNotDue (when selection.due_time > boundary now)
   - View recomputed before each extraction
   - No plan/selection survives stable boundary
   - Conflicted keys included in cohort identity and pacing count
   - All-conflicted slices become reporting-only cohorts
   - Adds cohort identity and pre-wave engine/scheduler digests to canonical semantic reports
   - Wave rejection restores post-extraction snapshot and leaves keys consumed

4. **Candidate proof claim**: For any fixed canonical input history and legal drain-call partition with non-decreasing now reaching same final boundary, cohort order/membership and every pre-wave state are partition-independent. Argument: new enqueues are > boundary now and cannot enter recomputed view.

5. **Candidate oracle (tests)**:
   - AT-I39: compares catch-up call, paced calls, one-call-per-cohort, ten due times at once vs separately, split batches, same-time profile cut. All compared cohorts start pre-scheduled; one cohort enqueues later work.
   - AT-I42: tests extra-key membership change but not same-key scheduled-to-conflicted change or token replay; claims to exercise CohortNotDue.
   - AT-I29: says no selection token survives but compile probe only proves view cannot be retained.
   - CE-9 traced to extraction-error atomicity plus unchanged generic wave-rejection tests.

Now I need to attack specific questions:
1. Safe-Rust borrow feasibility
2. Token arbitrary-selection/replay/TOCTOU
3. Missing-now totality
4. Catch-up delayed-enqueue counterexample (C@100 schedules work: compare now=101 once vs now=100 then 101)
5. Command barriers and oversized/paced cases
6. Conflict identity/budget authority
7. Drain refinement
8. CE-8/CE-9
9. Oracle gaps

Let me think through each systematically.

---

**1. Safe-Rust borrow feasibility**

The API has `due_cohort_view(&self, now)` returning a "borrowing view" and `take_due_cohort(&mut self, selection)` taking `&mut self`. The view is recomputed before each extraction.

Problem: The view borrows from `&self`, but `take_due_cohort` requires `&mut self`. In safe Rust, you cannot have a live immutable borrow (the view) while calling a method that takes `&mut self`. The view would need to be dropped before calling `take_due_cohort`.

But the candidate says "view is recomputed before each extraction" - this suggests the view is short-lived, created, used to pick a selection, then dropped, then `take_due_cohort` called. That's feasible *if* the selection doesn't borrow from the view.

However, `DueCohortSelection` contains a "key-only digest" - if this digest is computed from the view's data, the selection might conceptually depend on the view's snapshot. But the selection is owned, not borrowing.

The real issue: The view is a borrowing view (`&self`), but to compute the digest for the selection, you need to read the scheduler state. If the view is just a temporary iterator/collection of references, you can't return an owned selection containing a digest computed from those references unless you compute the digest *while the view is alive* and store the computed digest (not references) in the selection.

This seems feasible: `due_cohort_view` returns a temporary struct that implements `IntoIterator` or similar, you iterate it to pick a cohort, compute the digest from the viewed data, create an owned `DueCohortSelection` with the computed digest, drop the view, then call `take_due_cohort(&mut self, selection)`.

But wait - the candidate says "any entry returns an owned DueCohortSelection". This suggests the view's entries *are* the selections, or the view has a method to produce a selection. If the view holds `&self` references, and producing a selection requires computing a digest from those references, that's fine as long as the selection owns the digest.

**However**: The digest is described as "key-only digest". If this is a cryptographic digest of the cohort's WorkKeys, computing it requires reading all keys in the cohort. The view must provide access to those keys. If the view is a borrowing iterator over cohorts, and each cohort view provides access to its keys, you could compute the digest during iteration.

But there's a deeper issue: The scheduler's `drain_due` is atomic and removes slots. The view is supposed to show *due* cohorts. If `due_cohort_view` doesn't drain, it's just a snapshot. But `take_due_cohort` must actually remove the cohort's work from the scheduler. How does it do that without a `now` parameter? It uses the selection's `due_time` and `profile_id` to identify the cohort, but the scheduler state may have changed.

The candidate says "view is recomputed before each extraction" - this implies `take_due_cohort` internally recomputes the view? Or the caller must call `due_cohort_view` again? The API shows `take_due_cohort(&mut self, selection)` with no `now`, so it must use the selection's `due_time` as the boundary.

**Critical borrow issue**: If `due_cohort_view` returns a view that borrows `&self`, and the caller holds that view while deciding which cohort to take, they cannot call `take_due_cohort(&mut self, ...)` because that requires `&mut self`. The view must be dropped first. But if the view is dropped, and time passes (or other mutations happen), the selection may be stale. The candidate acknowledges this with `CohortMembershipChanged` error.

But the selection contains a "key-only digest" - this digest is computed from the view's snapshot. When `take_due_cohort` runs, it must verify the digest matches the current state. This requires recomputing the digest from current scheduler state. That's fine.

**However**: The view is described as "borrowing view of all due cohorts". In Rust, a borrowing view typically means a struct with lifetime tied to `&self`. You cannot return such a view and also allow `&mut self` calls while it exists. The pattern must be:
```rust
let view = scheduler.due_cohort_view(now);
let selection = view.select_cohort(...); // computes digest, returns owned Selection
drop(view);
scheduler.take_due_cohort(selection)?;
```
This is feasible in safe Rust. The view doesn't survive the `&mut` call.

**But**: The candidate says "any entry returns an owned DueCohortSelection". If the view's iterator yields `DueCohortSelection` directly, each selection would be created during iteration, borrowing from the view. But the selection is owned, so it must copy/compute all needed data (due_time, profile_id, count, digest) during iteration. That's fine - the digest is computed from the borrowed keys, stored as owned bytes in the selection. The selection then has no lifetime dependency.

**Verdict**: Safe-Rust borrow feasibility is **OK** *if* the view is a temporary that is dropped before `take_due_cohort`, and selections are fully owned (no lifetimes). But the API design is fragile - it's easy to misuse by holding the view across a `&mut` call. The compiler would catch this, but it's a footgun.

Wait - there's a subtlety. The candidate says "view is recomputed before each extraction". This could mean `take_due_cohort` *internally* recomputes the view using the selection's due_time/profile_id. But the API shows `take_due_cohort(&mut self, selection)` with no `now`. If it recomputes internally, it needs a `now` to know what's "due". The selection has `due_time`, so it could use that as the boundary. But the error `CohortNotDue` triggers when `selection.due_time > boundary now` - what is "boundary now"? If `take_due_cohort` uses the selection's `due_time` as the boundary, then `selection.due_time > boundary now` would never be true (it would be equal). Unless "boundary now" is some external concept.

Ah - "canonical boundary" from v2 section 11. The canonical boundary is a specific logical time. The candidate says "no plan/selection survives a stable boundary". So `take_due_cohort` must be called with a `now` that matches the canonical boundary? But the API has no `now` parameter.

This is a **major defect**: `take_due_cohort` has no `now` parameter, but must enforce `CohortNotDue` when `selection.due_time > boundary now`. Without a `now` parameter, what is "boundary now"? It could be:
- The selection's `due_time` (but then `CohortNotDue` never triggers)
- A stored "current boundary" in the scheduler (but scheduler has no stored clock per inherited facts)
- Implicit from context (but Rust has no implicit context)

The inherited facts explicitly state: "Scheduler has no stored clock." So `take_due_cohort` cannot rely on a stored `now`. It must receive `now` as a parameter, or the selection must encode the boundary `now` it was created for.

But the selection only has `due_time`, not the `now` used to create the view. The view was created with a `now` parameter. The cohort is "due" if `due_time <= now`. The selection's `due_time` is the cohort's due_time. For `CohortNotDue` to trigger, we need `selection.due_time > boundary_now`. But `boundary_now` is the `now` at the canonical boundary when extraction happens.

If the caller calls `due_cohort_view(now1)`, gets a selection for cohort with `due_time=100`, then later calls `take_due_cohort(selection)` when the canonical boundary is `now2=99`, the cohort is not due. But `take_due_cohort` has no `now2` to check against.

**This is a critical defect**: Missing `now` parameter on `take_due_cohort` makes `CohortNotDue` unimplementable as specified, and breaks the "no stored clock" invariant.

---

**2. Token arbitrary-selection/replay/TOCTOU**

The `DueCohortSelection` is an owned token with (due_time, profile_id, count, key-only digest). The candidate claims "no plan/selection survives a stable boundary" and AT-I29 says "no selection token survives" but compile probe only proves view cannot be retained.

**Replay attack**: Since the selection is just data (due_time, profile_id, count, digest), a caller could:
- Call `due_cohort_view(now1)`, get selection for cohort A
- Call `take_due_cohort(selection)` - succeeds
- Call `take_due_cohort(selection)` again - should fail with `CohortAbsent` or `CohortMembershipChanged`

But what if between the two calls, the same cohort is re-populated with *identical* WorkKeys (same keys, same digest)? The digest would match, count would match. `take_due_cohort` would see the cohort exists with matching digest and might succeed, effectively replaying the selection.

The candidate says "drained key becomes free and may be scheduled again, typically with a later occurrence". WorkKey includes `occurrence`. If the same work is rescheduled with a *later occurrence*, the WorkKey changes (occurrence is part of key). So the digest (over WorkKeys) would differ. Good.

But what if the work is rescheduled with the *same* occurrence? The WorkKey order includes occurrence, so same (due_time, profile_id, producer, scope, occurrence, kind) would be the same key. But a drained key becomes free - can it be scheduled again with the *same* occurrence? The inherited facts don't forbid this explicitly. If the producer re-enqueues with same occurrence, the key is identical. The digest would match.

However, v2 section 11 says "wave-created obligation has due_time >= current logical time + 1". If the cohort was at due_time=100, and it's drained at now=100, any re-enqueue must have due_time >= 101. So the re-enqueued work would have a different due_time, hence different cohort (cohorts are by profile_id + due_time). So it couldn't replay the same cohort selection.

But what about *non-wave* enqueues? The inherited facts don't restrict non-wave enqueues. A producer could schedule work at due_time=100 again after it was drained. But the canonical boundary has advanced past 100 (since we drained at now=100). The candidate says "no plan/selection survives a stable boundary". If the boundary is now 101, a selection for due_time=100 shouldn't be usable.

But `take_due_cohort` has no `now` parameter to enforce this! This connects to the missing-now defect.

**Arbitrary selection**: The caller can pick any cohort from the view. The view shows all due cohorts. The caller could skip cohorts, take them out of order. The candidate proof claims "cohort order/membership ... are partition-independent" but doesn't claim the *caller* must take them in order. The API allows arbitrary selection.

Is this a defect? The contracts don't seem to require FIFO cohort processing. R-038 is about equivalence of partitioning, not about order of processing. But v3 says "later cohorts must remain scheduled untouched" - this means you can't take a later cohort before an earlier one? "Later cohorts" likely means cohorts with later due_time. If you take a cohort with due_time=101 while due_time=100 cohort exists, you're touching a later cohort while an earlier one remains. But "untouched" might mean not *drained*, not not *selected*. The API allows selecting any due cohort.

**TOCTOU**: The view is a snapshot at `now`. The selection is created from that snapshot. `take_due_cohort` verifies the digest matches current state. If state changed (cohort membership changed), `CohortMembershipChanged` error. This is standard optimistic locking. Seems fine *if* the digest covers all relevant state (including conflicted keys per candidate).

But: The digest is "key-only digest". Does it include conflicted keys? Candidate says "includes conflicted keys in cohort identity and pacing count". So yes, conflicted keys are in the digest. Good.

**Token replay across boundaries**: If a selection for due_time=100 is created at now=100, and the caller holds it until after the boundary advances to 101, then calls `take_due_cohort`, what happens? Without a `now` parameter, `take_due_cohort` can't know the boundary advanced. It would see the cohort (if still there) and might process it. But the cohort at due_time=100 should have been drained at the boundary 100. If it wasn't taken, it should still be there? No - `drain_due(now)` atomically removes all due_time <= now. At canonical boundary 100, `drain_due(100)` would have removed the cohort. So the cohort wouldn't exist anymore. `CohortAbsent` would trigger.

But the candidate API doesn't use `drain_due` directly - `take_due_cohort` presumably does the draining for that cohort. The scheduler's `drain_due` removes *all* due work. The cohort API is a higher-level abstraction.

This is confusing. Let me re-read: "drain_due(now) atomically removes every occupied slot due_time <= now". This is the low-level scheduler operation. The cohort API (`due_cohort_view`, `take_due_cohort`) is built on top. `take_due_cohort` must internally call something like `drain_due` but for a specific cohort? Or it removes just that cohort's keys?

The candidate says "wave rejection restores the post-extraction snapshot and leaves keys consumed". This suggests `take_due_cohort` extracts (drains) the cohort's keys, and if the wave is rejected, the keys are *not* restored (they remain consumed/drained). But the post-extraction *snapshot* (engine state?) is restored.

This is weird. Normally you'd want to restore keys on rejection. But candidate says "leaves keys consumed". So extraction is irreversible even on wave rejection.

**TOCTOU on conflicted keys**: Candidate says conflicted keys are included in cohort identity and pacing count. Conflicted keys are returned by `drain_due` as "conflicted evidence". They are not in `DrainOutcome::due` (per v1 Q4). But the cohort view includes them in identity.

If a cohort has some scheduled keys and some conflicted keys, the digest covers both. If between view and take, a scheduled key becomes conflicted (or vice versa), the digest changes -> `CohortMembershipChanged`. Good.

But AT-I42 "tests extra-key membership change but not same-key scheduled-to-conflicted change or token replay". This is a test gap, not necessarily a design defect. But the design should handle scheduled-to-conflicted change. Since conflicted keys are in the digest, it should be detected.

**Replay with same digest**: As analyzed, if the exact same set of WorkKeys (scheduled + conflicted) reappears, digest matches. But due_time is part of WorkKey, and re-enqueues after boundary must have later due_time (for wave-created work). For non-wave work, no such guarantee. But the canonical boundary concept implies we only process at boundaries. If the caller holds a selection across a boundary, it's a logic error by the caller. The API doesn't prevent it because `take_due_cohort` lacks `now`.

---

**3. Missing-now totality**

As identified in #1: `take_due_cohort(&mut self, selection)` has no `now` parameter, but must enforce `CohortNotDue` when `selection.due_time > boundary now`. The scheduler has no stored clock. The selection doesn't contain the `now` used to create the view.

This makes `CohortNotDue` impossible to implement correctly. The "boundary now" is the canonical boundary time at which the extraction is attempted. Without it as a parameter, the method cannot know what the current boundary is.

**Concrete scenario**:
- Scheduler has cohort at due_time=100.
- Caller calls `due_cohort_view(now=100)`, gets selection for due_time=100.
- Caller delays, canonical boundary advances to 101 (somehow - but scheduler has no clock, so boundary advance is external).
- Caller calls `take_due_cohort(selection)`.
- The cohort at due_time=100 should have been drained at boundary 100. If it wasn't (because caller didn't take it), it might still exist? But `drain_due(100)` would have removed it. Unless the cohort API bypasses `drain_due`.

The architecture is unclear on whether `take_due_cohort` uses `drain_due` internally or has its own removal logic. If it uses `drain_due(selection.due_time)`, then at boundary 101, calling `drain_due(100)` would do nothing (already drained). But the selection is for due_time=100.

The `CohortNotDue` error is specified for "when selection.due_time > boundary now". This implies the boundary now could be *less* than selection.due_time. Example: caller creates view at now=100, sees cohort due_time=100. Then calls take at a time when boundary now=99 (e.g., in a different partition/catch-up scenario). But boundaries are non-decreasing per proof claim ("non-decreasing now"). So boundary now=99 after now=100 shouldn't happen in legal partitions.

But the proof claim says "any legal drain-call partition with non-decreasing now". So boundary now only increases. If selection.due_time=100, and boundary now >= 100 (since view was created at now=100 and boundaries non-decreasing), then selection.due_time > boundary now is false. `CohortNotDue` would never trigger in legal partitions.

Unless... the view is created at now=100, but the cohort has due_time=101? No, view shows *due* cohorts, so due_time <= now. So selection.due_time <= view_now. And boundary now >= view_now (non-decreasing). So selection.due_time <= boundary now always. `CohortNotDue` is impossible in legal operation.

Then why does the error exist? And why does AT-I42 "claim to exercise CohortNotDue"?

Perhaps "boundary now" is not the drain-call `now`, but a separate concept. Or perhaps the view can show cohorts that are *not yet due*? But it's called `due_cohort_view`.

Wait: "any entry returns an owned DueCohortSelection with private (due_time, profile_id, count, key-only digest)". The view is of "all due cohorts". So due_time <= now.

But the candidate says: "It promises errors CohortAbsent, CohortMembershipChanged, CohortNotDue when selection.due_time > boundary now."

If selection.due_time <= view_now, and boundary now >= view_now, then selection.due_time <= boundary now. So CohortNotDue never triggers.

Unless the view is created with a `now` that is *not* the boundary now. The proof talks about "drain-call partition with non-decreasing now". Each drain call has a `now`. The view is created with some `now`. The extraction (take) happens at a later drain call with a larger `now`. But the selection's due_time is fixed. Since due_time <= view_now <= take_now, due_time <= take_now. So not due? No, it *is* due.

I think `CohortNotDue` is for the case where the caller creates a view at now=100, sees cohort due_time=100, but then tries to take it at a boundary where the *canonical* now is 99? But boundaries are non-decreasing.

Unless "boundary now" refers to the *cohort's* due_time boundary? No.

Another interpretation: The "canonical boundary" is a specific logical time (e.g., end of day). The `now` parameter to `due_cohort_view` is the *current* logical time, which may be *before* the canonical boundary. The view shows cohorts due *by the canonical boundary*? But the API says `due_cohort_view(&self, now)` - `now` is the parameter.

Let's look at v2 section 11: "canonical boundary drains due work at most once". And AT-I21: due_time >= now + 1 for wave-created obligations.

Perhaps the flow is:
- At canonical boundary T, the system calls `drain_due(T)` once.
- The cohort API is for *within* a wave at that boundary? No, "pre-wave engine digest includes scheduler digest".

I'm confused. But the key fact: `take_due_cohort` has no `now`, scheduler has no clock, but `CohortNotDue` depends on "boundary now". This is a **critical defect** - the parameter is missing, making the error condition unimplementable and the API unsound for the stated contract.

---

**4. Catch-up delayed-enqueue counterexample (C@100 schedules work: compare now=101 once vs now=100 then 101)**

Scenario: At logical time 100, a catch-up operation schedules work (C@100). Compare:
- Partition A: Single drain call at now=101
- Partition B: Two drain calls at now=100 then now=101

Candidate proof claim: cohort order/membership and pre-wave state are partition-independent. Argument: "new enqueues are > boundary now and cannot enter a recomputed view."

At now=100 (Partition B first call): The catch-up at C@100 schedules work. What due_time does this work get? v2 section 11: "wave-created obligation has due_time >= current logical time + 1". AT-I21: due_time >= now + 1. So at now=100, new work gets due_time >= 101.

Partition A (now=101): The catch-up at C@100 happens... when? "C@100 schedules work" - is C@100 a catch-up event at time 100? In Partition A, there's no drain call at 100. The catch-up happens at some point before the drain at 101. The scheduled work gets due_time >= 101 (since current logical time when scheduled is... what? If scheduled during the drain at 101, "current logical time" is 101, so due_time >= 102? But the candidate says "new enqueues are > boundary now". Boundary now for Partition A is 101. So new enqueues > 101, i.e., >= 102.

In Partition B: First drain at now=100. Catch-up at C@100 schedules work with due_time >= 101. This work is *not* due at now=100 (due_time >= 101 > 100), so not drained. Second drain at now=101: work with due_time=101 is drained.

In Partition A: Single drain at now=101. Catch-up schedules work with due_time >= 102 (since boundary now=101, "current logical time" during scheduling is 101? Or is it 100? The catch-up is "C@100" - does it schedule with the logical time of the catch-up event (100) or the drain call time (101)?)

This is the crux. The candidate argues "new enqueues are > boundary now". In Partition A, boundary now=101, so new enqueues > 101. In Partition B, first boundary now=100, new enqueues > 100 (i.e., >=101). Second boundary now=101, but the work was already enqueued at first step.

So in Partition A, the work gets due_time >= 102. In Partition B, the work gets due_time >= 101. Different due_times! Therefore different cohorts (cohorts keyed by due_time). Partition dependence!

The candidate's argument fails because the "current logical time" for scheduling is ambiguous. If it's the catch-up event time (100), then in both partitions due_time >= 101. But in Partition A, there's no drain at 100, so when does the scheduling happen? If it happens at the drain call at 101, the "current logical time" might be 101.

The inherited fact: "wave-created obligation has due_time >= current logical time + 1". What is "current logical time" during a catch-up that happens outside a drain call? The scheduler has no stored clock. The drain call provides `now`. If catch-up scheduling happens *during* a drain call, `now` is available. If it happens asynchronously, there's no `now`.

The counterexample: C@100 is a catch-up event at logical time 100. In Partition B, it's processed at the now=100 drain call, so work gets due_time >= 101. In Partition A, there's no drain call at 100. The catch-up event must be processed at the next drain call (now=101). If "current logical time" for scheduling is the drain call's `now` (101), work gets due_time >= 102. If it's the event's logical time (100), work gets due_time >= 101. But the scheduler has no stored clock, so it can't know the event's logical time unless passed as a parameter.

The candidate's proof argument "new enqueues are > boundary now" assumes that enqueues during a drain call at boundary `now` get due_time > `now`. But for catch-up events that *belong* to an earlier time, this breaks partition independence.

**This is a critical defect** in the proof claim and likely the design. The partition independence claim is false for catch-up scheduling that uses the drain call's `now` as "current logical time".

---

**5. Command barriers and oversized/paced cases**

The candidate mentions "pacing count" and "all-conflicted slices become reporting-only cohorts". Command barriers likely refer to the canonical boundary draining at most once (v2 section 11).

Oversized cohorts: If a cohort has many keys, the digest computation and verification could be expensive. The selection includes "count" and "key-only digest". For oversized cohorts, the digest might be a hash of many keys. Computing it on every view recomputation and every take verification could be a DoS vector.

Paced cases: "pacing count" suggests cohorts are processed in batches. The API `take_due_cohort` takes one cohort at a time (identified by profile_id + due_time). If a cohort is huge, taking it all at once might be problematic. But the candidate doesn't mention splitting cohorts.

Command barriers: The canonical boundary drains due work at most once. The cohort API allows multiple `take_due_cohort` calls per boundary? The candidate says "no plan/selection survives a stable boundary". But within a boundary, can you take multiple cohorts? Yes, `due_cohort_view` shows all due cohorts, you can take them one by one.

But v2 section 11: "canonical boundary drains due work at most once". This suggests that at a canonical boundary, you call `drain_due(now)` once, which removes *all* due work. The cohort API seems to allow incremental draining within a boundary? That would violate "drains due work at most once" if "drain" means the low-level `drain_due`.

Unless the cohort API *is* the drain mechanism for the boundary, and "at most once" means you can't call the boundary drain twice. But you can take multiple cohorts in one boundary processing.

The candidate says "wave rejection restores the post-extraction snapshot and leaves keys consumed". This implies extraction (take_due_cohort) consumes keys. If you take multiple cohorts in a boundary, and then reject the wave, all those keys are consumed (lost). That seems intentional per candidate.

**Defect**: No protection against oversized cohorts causing excessive memory/compute during digest calculation. The digest is "key-only" - if it's a cryptographic hash of all keys in the cohort, a cohort with 1M keys requires hashing 1M keys on every view recomputation and every take attempt. An attacker could create a huge cohort to DoS the system.

**Fix**: Require incremental/streaming digest, or limit cohort size, or make digest computation lazy/optional.

---

**6. Conflict identity/budget authority**

v1 Q4: "budgets over the stable WorkKey prefix of DrainOutcome::due (conflicted slots are not in due)". So budgets are computed only on *scheduled* (non-conflicted) work in the due set.

Candidate: "includes conflicted keys in cohort identity and pacing count". And "all-conflicted slices become reporting-only cohorts".

**Conflict**: Budgets exclude conflicted slots, but cohort identity includes them. This means two cohorts with identical scheduled keys but different conflicted keys have different identities (different digests), but the same budget impact. Is this intended?

"Pacing count" includes conflicted keys. If pacing limits how many cohorts/keys are processed per boundary, counting conflicted keys in pacing reduces capacity for actual work.

"All-conflicted slices become reporting-only cohorts" - these have no scheduled keys, only conflicted. They don't affect budgets (since budgets are over due/scheduled). But they consume pacing count? Candidate says "pacing count" includes conflicted keys.

**Budget authority**: Who decides the budget? v1 Q4 says budgets over stable WorkKey prefix of due. The cohort API doesn't mention budgets. The `take_due_cohort` doesn't check budgets. Budgets might be enforced at a higher layer (wave engine).

But if cohort identity includes conflicted keys, and conflicted keys can change without scheduled keys changing (e.g., a conflict resolves), the cohort identity changes, triggering `CohortMembershipChanged` on take. This could cause spurious failures.

**Defect**: Including conflicted keys in cohort identity (and thus digest) couples cohort stability to conflict state, which is orthogonal to budget/scheduling. A conflict resolution (scheduled key becomes conflicted or vice versa) changes cohort identity, forcing re-selection, even though the schedulable work is unchanged. This violates the principle that cohort identity should be based on the work to be done (scheduled keys), not on conflict metadata.

The candidate says "includes conflicted keys in cohort identity and pacing count". This is a design choice, but it conflicts with v1 Q4 budget stability (budgets ignore conflicted). It also makes `CohortMembershipChanged` fire for conflict changes, which may be frequent and not meaningful for the work.

---

**7. Drain refinement**

The low-level `drain_due(now)` removes *all* due_time <= now atomically. The cohort API `take_due_cohort` removes one cohort (one profile_id + due_time slice).

**Refinement violation**: `drain_due` is atomic for all due work. `take_due_cohort` is incremental. If the system is supposed to use `drain_due` at canonical boundaries (v2 section 11: "canonical boundary drains due work at most once"), then the cohort API must not be used at canonical boundaries, or it must be a wrapper that drains all cohorts in one atomic operation.

But the candidate describes `take_due_cohort` as the extraction mechanism, and "wave rejection restores post-extraction snapshot and leaves keys consumed". This suggests `take_due_cohort` *is* the drain operation for a cohort, and multiple calls happen per wave/boundary.

This contradicts "canonical boundary drains due work at most once" if that means one atomic drain of all due work. Unless "drains due work at most once" means the *boundary event* happens once, but the drain can be incremental within the wave.

The candidate proof claim: "for any fixed canonical input history and any legal drain-call partition with non-decreasing now reaching the same final boundary, cohort order/membership and every pre-wave state are partition-independent."

"Drain-call partition" suggests multiple `drain_due` calls (or `take_due_cohort` calls) with different `now` values. The low-level `drain_due` is the primitive. The cohort API is built on top.

If `take_due_cohort` internally calls `drain_due(selection.due_time)`, then:
- Partition A: one call `drain_due(101)` - removes all due <= 101.
- Partition B: `drain_due(100)` then `drain_due(101)` - first removes due <= 100, second removes due <= 101 (which is just due=10
