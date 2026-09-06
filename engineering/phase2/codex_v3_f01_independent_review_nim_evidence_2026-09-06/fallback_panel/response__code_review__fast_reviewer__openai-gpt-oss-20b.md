# fast_reviewer / openai/gpt-oss-20b

worker_id=dcj-5bc33106384f-w1 job_id=dcj-5bc33106384f task=code_review
ok=True latency_s=224.65 error_class=None

**1. Unsafe borrow in `due_cohort_view`**
*Severity:* **Major**
*Location:* `due_cohort_view(&self) -> BorrowedView`
*Why wrong:* The view holds immutable references to internal scheduler state while `take_due_cohort(&mut self, selection)` mutates that same state. The Rust borrow‑checker will reject this, but the candidate’s comment “view is recomputed before each extraction” does not guarantee that the view is dropped before the mutable borrow. A caller could keep the view alive across the call, causing a double‑borrow or a dangling reference.
*Fix:* Return an owned snapshot (`Vec<DueCohortSelection>`) or use interior mutability (`RefCell`) and enforce that the view is dropped before `take_due_cohort`.

---

**2. Replay/TOCTOU of selection tokens**
*Severity:* **Major**
*Location:* `DueCohortSelection` contains `(due_time, profile_id, count, key‑only digest)` but no version stamp.
*Why wrong:* A token can be stored externally and replayed after the cohort has been drained or its membership changed. The current error set (`CohortAbsent`, `CohortMembershipChanged`, `CohortNotDue`) does not detect that the token refers to a *different* set of keys (e.g., after a conflict resolution).
*Fix:* Add a monotonically increasing sequence number or a hash of the cohort’s current key set to the token and validate it on extraction.

---

**3. Missing `now` totality in `take_due_cohort`**
*Severity:* **Minor**
*Location:* `take_due_cohort(&mut self, selection)`
*Why wrong:* The method relies on an implicit “current boundary” that may be stale if the caller has not advanced it. This can lead to `CohortNotDue` being returned even though the caller intended to drain at a later time.
*Fix:* Either add an explicit `now` parameter or document that the caller must call `drain_due(now)` before invoking `take_due_cohort`.

---

**4. Catch‑up delayed‑enqueue counterexample**
*Severity:* **Major**
*Scenario:* `C@100` schedules work at time 100. The system performs a catch‑up drain at `now=101` once. Later, a new enqueue occurs at `now=100` (after the catch‑up). The new work is missed because the drain already removed all slots ≤ 101.
*Why wrong:* The candidate’s claim that “new enqueues are > boundary now” is violated; enqueues at the same time as a prior drain can be lost.
*Fix:* Treat enqueues with `due_time <= current_boundary` as *due* and include them in the next drain, or advance the boundary after each enqueue.

---

**5. Oversized/paced cohort handling**
*Severity:* **Major**
*Why wrong:* The API claims that all‑conflicted slices become reporting‑only cohorts, but it does not expose a mechanism to limit the number of keys drained per call. A cohort with > N keys will be drained in a single call, violating pacing guarantees and potentially starving other cohorts.
*Fix:* Implement a pacing limit: `take_due_cohort` should return the number of keys actually drained and leave the remainder in the selection for subsequent calls.

---

**6. Conflict identity vs. budget authority**
*Severity:* **Major**
*Why wrong:* Conflicted keys are included in the cohort’s identity and pacing count, yet budgets are applied only to *non‑conflicted* work. This can allow a user to inflate a budget by scheduling conflicted keys that are never counted.
*Fix:* Exclude conflicted keys from the pacing count and budget calculations; treat them as a separate reporting‑only slice.

---

**7. Drain refinement race**
*Severity:* **Major**
*Why wrong:* `drain_due(now)` atomically removes slots, but `due_cohort_view` recomputes the view immediately before each extraction. If a drain occurs concurrently with extraction, the view may be stale, leading to double‑drain or missed work.
*Fix:* Guard the view recomputation and extraction with a single lock or use a transactional snapshot that guarantees consistency.

---

**8. CE‑8 / CE‑9 extraction‑error atomicity**
*Severity:* **Major**
*Why wrong:* If `take_due_cohort` fails after partially consuming keys, the system may leave the scheduler in an inconsistent state (keys consumed but not reported). The candidate’s tests do not cover this scenario.
*Fix:* Make extraction all‑or‑nothing: either commit the removal of all keys in the cohort or roll back on error.

---

**9. Oracle gaps (AT‑I39, AT‑I42)**
*Severity:* **Major**
*Why wrong:* The oracle does not test:
  * same‑key scheduled‑to‑conflicted transitions
  * token replay after a drain
  * conflict resolution that changes the cohort’s identity.
These omissions leave a class of bugs unverified.
*Fix:* Extend the oracle to include:
  * a test that schedules a key, then conflicts it, and verifies that the cohort identity changes;
  * a test that replays a selection token after a drain and expects `CohortAbsent` or `CohortMembershipChanged`.

---
