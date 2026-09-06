Falsification brief (public, non-secret architecture text). You are given an architecture addendum
for a deterministic simulation engine: a bounded FIFO mailbox feeds the engine through one consumer;
one request (Advance{T} or Command{effective_time e}) is active until completion; budget exhaustion
pauses the request; completion = all scheduled work with due_time <= horizon consumed (plus, for a
command, staging+finalizing+executing it atomically at the end); a scalar F = horizon of the last
completed request; a request may start only if its horizon >= F; per-cohort engine digest excludes F;
a separate stable-boundary digest includes F; no finalized-but-unexecuted command state exists.
Try to construct a concrete counterexample where (a) pacing budget changes canonical results or
request order, (b) a command evaluates at a time earlier than a committed cell timestamp,
(c) restart from (stores, F) with the head request re-presented diverges from an uninterrupted run,
or (d) two engines with equal stable-boundary digests respond differently to the same next request.
Give exact step sequences. If you cannot, say what assumption each attack needed and failed on.
