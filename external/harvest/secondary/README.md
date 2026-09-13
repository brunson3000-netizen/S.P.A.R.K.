# Secondary External Source Quarantine

This directory contains gitlinks to exact upstream revisions selected by the SPARK External Tool Acquisition Program.

## Important

These are **source-custody references**, not SPARK runtime dependencies.

Every secondary submodule is configured with:

- `update = none` — ordinary recursive submodule synchronization skips it;
- `shallow = true` — deliberate local harvest should minimize history where Git permits it.

Do not remove those controls merely to make a blanket submodule command populate this directory.

To list or materialize a specific source:

```bash
engineering/tool_acquisition/scripts/harvest_secondary_source.sh list
engineering/tool_acquisition/scripts/harvest_secondary_source.sh <source-id>
```

Exact expected revisions and purposes are recorded in:

`engineering/tool_acquisition/SECONDARY_HARVEST_LOCK.json`

A checkout here is not an installation, activation, adoption, or grant of authority.
