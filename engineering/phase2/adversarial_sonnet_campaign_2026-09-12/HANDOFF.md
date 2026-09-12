# SONNET campaign handoff — 2026-09-12

- **Elapsed:** started `2026-09-12T11:30:40Z`, this handoff written `2026-09-12T11:40Z`ish,
  well inside the 30-minute window; publication (below) is closeout only.
- **Executed cases:** 30 hostile tests, all passing against the pinned target
  `67b877192cc78b75c6fbe60c69b5594dc10befe8`. Full output in `full-run.txt`.
- **Findings:** none open. See `FINDINGS_INDEX.md` and the campaign report's "Findings"
  section for six triaged non-findings (T-1 … T-6), each with its confirming source location.
- **Seeds:** none used; every vector is deterministic (fixed logical times and literal
  parameters), so `full-run.txt` reproduces exactly with `--test-threads=1`.
- **Evidence paths:**
  - `SPARK_ADVERSARIAL_SONNET_CAMPAIGN_2026-09-12.md` — campaign report
  - `FINDINGS_INDEX.md` — findings summary
  - `adversarial_probes.rs` — the 30 test cases, full source
  - `full-run.txt` — captured stdout of the final complete run (30/30 pass)
  - `harness_Cargo.toml.txt` — the disposable external crate's manifest (path deps,
    `spark-engine` with `test-support`)
  - `SHA256SUMS.txt` — hash of the probe source
- **Coverage gaps** (not findings, listed for a follow-on campaign): the bidirectional
  obligation-drop mutation needs an internal `WorkKey` not constructed in time; the
  mismatched-profile restore case did not get a genuinely distinct second profile
  (content-addressing meant the "different" one this campaign built was actually identical);
  no staged-conflicting-`WorkKey` test; no concurrent-access test; no Windows/Android runtime
  test (static-only coverage is already disclosed elsewhere).
- **Blockers:** none. Nothing prevented testing; no routine authorization was requested.
- **Product files:** unchanged. No repair, no dependency install, no other agent spawned, no
  external inference, no production or Phase-3 touch. Every pre-existing worktree, branch and
  the reviewed candidate's own worktree (`SPARK-gate-c2-w01`) were left untouched; this
  campaign used only its own detached worktrees under `/tmp/spark-adversarial-sonnet/`.
