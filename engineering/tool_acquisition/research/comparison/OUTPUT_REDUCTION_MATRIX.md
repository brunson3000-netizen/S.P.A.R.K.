# Output Reduction Comparison Matrix

Updated: 2026-09-13

| Dimension | RTK | context-compress | SPARK implication |
|---|---|---|---|
| Primary reduction | command-specific deterministic filtering | generic indexing + relevance search | use both as derived views |
| Executes command | usually yes | execute tool can run command; also indexes external content | canonical broker should own execution first |
| Raw exit status | preserved | executor returns status | bind status to raw evidence artifact |
| Semantic awareness | high per supported command | low/generic content awareness | command-specific reducer is valuable after execution |
| Hidden content retrieval | recall by hash/line/grep | FTS/BM25/fuzzy snippets by source/query | offer both direct raw retrieval and search |
| Byte-faithful store | yes under entry cap for stored payload; arbitrary-byte tests | no, content is normalized/chunked | raw artifact must be separate canonical layer |
| Universal raw capture | no: failures/explicit truncations; ordinary successful compression may not store raw | no | capture every load-bearing operation centrally |
| Entry cap | default 10 MiB | executor/input/search bounds vary | canonical evidence limits must be explicit and not silently destroy required bytes |
| Retention | 200 entries / 30 days default | ephemeral default / bounded sources | project evidence retention belongs to SPARK policy |
| Evidence identity | 12-hex SHA-256 prefix of command+full content | source IDs/internal rows | use full stable digest + metadata |
| Search across hidden data | grep over one recalled blob | strong full-text/fuzzy/source-scoped search | build derived index over raw artifacts |
| Truncation disclosure | explicit recall hint on integrated paths | search hint/source id | every lossy view must disclose loss/recovery path |
| Filter failure | raw fallback for correctness | fallbacks to generic/raw paths | raw fallback should stay outside prompt if oversized |
| Automatic interception | extensive hook/plugin rewrite system | explicit MCP/tool use | avoid third-party hook as canonical authority |
| Best transferable asset | filter taxonomy + command parsers + recall UX | searchable spillover/indexing | combine under host evidence custody |

## Converged doctrine

**Small context, canonical evidence.**

Preferred pipeline:

`host-authorized operation`
→ `immutable raw artifact (full digest + metadata)`
→ `RTK-like command reducer`
→ `context-compress-like search index`
→ `bounded agent view`

The raw artifact is canonical; reducer/index are disposable derived state.
