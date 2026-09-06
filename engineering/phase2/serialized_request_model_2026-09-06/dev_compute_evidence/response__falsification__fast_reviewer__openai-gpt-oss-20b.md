# fast_reviewer / openai/gpt-oss-20b

worker_id=dcj-043a36fc012f-w1 job_id=dcj-043a36fc012f task=falsification
ok=True latency_s=52.53 error_class=None

**Attack (a) – “Pacing budget changes canonical results or request order”**
*Scenario*:
1. `Advance{T=10}` is accepted.
2. Engine processes slices with due_time ≤ 10 until budget B=1 is exhausted after the first slice.
3. Consumer retries `process(Advance{10})` (budget B=2).
4. Engine continues and completes the request.
*Expected*: Canonical state after completion is the same as if the request had run with a single call.
*Actual*: The same slices are executed in the same order; only the number of calls differs.
*Result*: **Fails** – pacing only changes call count, not canonical results or order.
*Assumption*: The engine consumes slices deterministically in ascending `(due_time, profile_id)` order and does not alter any state when pausing.

---

**Attack (b) – “A command evaluates at a time earlier than a committed cell timestamp”**
*Scenario*:
1. `Advance{T=20}` completes, setting `F=20`.
2. Producer offers `Command{effective_time=15}`.
3. Engine checks `h=15 < F=20` → rejects.
*Expected*: Command is rejected, never evaluated.
*Actual*: Command is rejected; no evaluation occurs.
*Result*: **Fails** – the `h ≥ F` rule prevents any earlier evaluation.
*Assumption*: `F` is updated only at request completion and never decremented.

---

**Attack (c) – “Restart from (stores, F) with the head request re‑presented diverges from an uninterrupted run”**
*Scenario*:
1. `Advance{T=10}` is accepted.
2. Engine processes two slices (due_time ≤ 10) and then pauses due to budget exhaustion.
3. Stores + `F=0` are snapshotted.
4. Engine is rebuilt from snapshot; consumer re‑presents the same `Advance{10}`.
5. Engine resumes and processes the remaining slice, then completes.
*Expected*: Final canonical state equals that of the uninterrupted run.
*Actual*: The same two slices are executed in the same order; the final state is identical.
*Result*: **Fails** – the snapshot includes all state changes made before the pause, so restart reproduces the same trace.
*Assumption*: Partial work that has been executed before the pause is reflected in the engine state; only the request argument is transient.

---

**Attack (d) – “Two engines with equal stable‑boundary digests respond differently to the same next request”**
*Scenario*:
1. Engine A and Engine B both finish a request with horizon `h=30`, so `F=30`.
2. Their stable‑boundary digests are `H(stable_boundary_v1 ‖ engine_state_digest ‖ 30)` and are equal.
3. Both receive `Command{effective_time=35}`.
4. Each engine checks `h=35 ≥ F=30` → accepts and executes.
*Expected*: Both engines produce the same canonical result.
*Actual*: Both execute identically; no divergence.
*Result*: **Fails** – equal digests guarantee identical stores and `F`, so the next request is handled the same.
*Assumption*: The stable‑boundary digest includes `F` and all store contents; no hidden state influences request handling.

---

### Summary
All four attempted falsifications fail under the stated assumptions of the addendum.
The only assumptions that were critical were:

1. **Deterministic slice ordering** and **no state change on pause**.
2. **`F` updated only at request completion** and never decremented.
3. **Snapshot captures all state changes** made before pause.
4. **Stable‑boundary digest includes `F` and all store contents**.

With these assumptions intact, the addendum’s claims hold.
