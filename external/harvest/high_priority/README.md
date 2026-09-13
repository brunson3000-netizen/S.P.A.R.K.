# High-Priority External Source Quarantine

These repositories are pinned research/source inputs for the S.P.A.R.K. tool-acquisition program.

They are **not S.P.A.R.K. runtime dependencies** and are not activated merely because they are checked out. Do not add them to Cargo manifests, execute installer scripts, or promote copied code without the separate qualification/reuse decision recorded by the tool-acquisition mission.

S.P.A.R.K. owns this research/acquisition lane only. Nothing here grants authority over S.W.A.R.M. or any other consuming project.

After syncing S.P.A.R.K. locally:

```bash
git submodule sync --recursive
git submodule update --init --recursive
engineering/tool_acquisition/scripts/verify_high_priority_sources.sh
```

The exact expected commits are recorded in `engineering/tool_acquisition/UPSTREAM_LOCK.json`.
