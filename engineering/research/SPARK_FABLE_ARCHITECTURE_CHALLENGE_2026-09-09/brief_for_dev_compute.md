Falsify the attached RESEARCH design for the S.P.A.R.K. deterministic causal engine request loop.
Target claims: (1) calling the existing scheduler's drain_due with the least due time, iterated,
gives per-cohort extraction equivalent to a dedicated extraction API; (2) a paused request needs
no retained engine state because progress is time-ordered and indistinguishable from "not yet
requested"; (3) the logical clock alone suffices as the request frontier for rejecting
past-dated inputs; (4) whole-cohort transactions on an overlay map give correct retained
effects after failure. Provide concrete input sequences that break any claim, and state which
claim survives. Be terse; no restatement of the document.
