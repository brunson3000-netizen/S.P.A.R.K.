# Agent–Tool Middle-Layer Source Quarantine

These gitlinks preserve exact upstream source snapshots for S.P.A.R.K. research and qualification.

They are not production dependencies. They are not installed, activated, trusted, or adopted by being present here. Normal recursive submodule updates skip this lane because each entry uses `update = none`.

Source-of-truth pin ledger:

`engineering/tool_acquisition/MIDDLE_LAYER_HARVEST_LOCK.json`

Research synthesis:

`engineering/tool_acquisition/MIDDLE_LAYER_RESEARCH_2026-09-13.md`

Probationary architecture:

`engineering/tool_acquisition/MIDDLE_LAYER_ARCHITECTURE_HYPOTHESIS.md`

Qualification queue:

`engineering/tool_acquisition/pending_actions/MIDDLE_LAYER_QUALIFICATION.md`

To list sources:

```bash
./engineering/tool_acquisition/scripts/harvest_middle_layer_source.sh list
```

To materialize exactly one source for study:

```bash
./engineering/tool_acquisition/scripts/harvest_middle_layer_source.sh ard-spec
```

The helper verifies the exact pinned commit and performs no build, install, hook activation, gateway activation, credential configuration, or runtime adoption.
