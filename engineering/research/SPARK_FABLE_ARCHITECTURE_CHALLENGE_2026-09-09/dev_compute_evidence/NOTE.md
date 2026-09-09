# Supplementary compute note (non-authoritative)

RESEARCH / PROPOSED DESIGN — NOT ADOPTED.

One bounded headless falsification request was submitted through the documented entry point
(`tools.dev_compute.orchestrate run --task falsification --depth fast`, worker timeout 150 s,
panel deadline 200 s) against `02_FABLE_DESIGN.md` with `brief_for_dev_compute.md`.

- Attempt 1 (05:21Z) failed before submission: `cannot reserve research output: FileExistsError`
  because I passed a pre-existing `--out-dir`. Records kept in `../dev_compute_attempt1_failed/`.
- Attempt 2 (05:26Z, fresh out-dir) was accepted, validated, and then returned
  `status: blocked` in 6 ms with detail "strict reliability contract requires current economic
  and readiness evidence". No worker ran; no model response exists; `result: null`.

Per the standing rule, no further retry or troubleshooting was performed. The gateway
decided nothing: every conclusion in this archive is my own and is backed by the executable
example. Records here are credential-free (grep for key/bearer patterns: none).
