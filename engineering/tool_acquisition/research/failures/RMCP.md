# rmcp Failure / Boundary Lessons

Source: `modelcontextprotocol/rust-sdk` @ `3075dc9152d4678775f20634fcb467a7b995dbab`
Status: ACTIVE EVIDENCE

## RMCP-F01 — Protocol task “durability” is process-local

FAILURE MODE: documentation/comment wording can be read as crash durability although `TaskManager` stores canonical task state in an in-memory `Arc<Mutex<HashMap<...>>>`.

CAUSE: “durably observable” in the MCP task requirement means the task can be retrieved after the create response is returned, not that the task survives process loss.

PROJECT RESPONSE: SDK provides an in-memory reference implementation; durable application persistence is left to server implementations.

DID IT WORK?: yes for protocol behavior; no for SPARK recovery requirements.

SPARK LESSON: distinguish **protocol-observable**, **process-durable**, and **crash-durable** state explicitly. Never infer storage guarantees from the word “task.”

DISPOSITION: REUSE_CODE for task models; do not adopt TaskManager as canonical durable work registry.

## RMCP-F02 — MCP session ID is not authenticated canonical identity

FAILURE MODE: same `Mcp-Session-Id` means one logical MCP transport session, which can be mistaken for a user/agent/project identity or grant.

CAUSE: session ID exists to route stateful Streamable HTTP traffic; authentication and application authority are separate concerns.

PROJECT RESPONSE: transport/session machinery manages IDs and lets custom session stores restore them. It does not claim they replace application identity.

SPARK LESSON: MCP session IDs stay adapter-local correlation. Canonical subject/session/work identity and authenticated grants are host-owned.

DISPOSITION: BORROW_PATTERN negative invariant.

## RMCP-F03 — Default stale-on-error cache can hide current failure

FAILURE MODE: expired cached tool/resource/list response is returned as successful `Ok(...)` if re-fetch fails because `ClientCacheConfig::default().serve_stale_on_error = true`.

CAUSE: SEP-2549 permits resilience-oriented stale fallback.

PROJECT RESPONSE: callers can disable stale fallback or caching and can partition private cache by authorization context; invalidation notifications advance cache generation.

DID IT WORK?: useful client UX, unsafe if a consumer treats cached discovery/resource results as current authority/trust/health truth.

SPARK LESSON: authority-sensitive discovery and policy inputs must disable stale-on-error or carry explicit staleness state that cannot satisfy a current grant check.

DISPOSITION: REUSE_CODE with mandatory configuration/invariant.

## RMCP-F04 — Cross-process session serialization can lose request association

FAILURE MODE: the bundled legacy session manager uses a non-serialized `OriginatingRequestId` extension to route server-to-client requests onto the originating request stream. A custom distributed session store that merely serializes MCP messages loses this marker and can violate SEP-2260.

CAUSE: process-local extension state is not on the wire.

PROJECT RESPONSE: SDK documentation explicitly warns custom cross-process session managers to provide their own association mechanism.

SPARK LESSON: any adapter-local state that matters after a process/federation boundary must be serialized/bound explicitly. Do not assume cloning the protocol envelope preserves local context.

DISPOSITION: BORROW_PATTERN / acceptance test for any federated MCP adapter.

## RMCP-F05 — “Initialized” is not an authority gate

FAILURE MODE: a host might wait for or rely on the legacy `initialized` notification as proof the peer is safe/authorized.

OBSERVED IN: current server enters the main loop immediately after `InitializeResult`; early requests can be processed before `initialized`, by deliberate interoperability design.

SPARK LESSON: authorization/trust is never inferred from protocol initialization order. Host authentication/authority must be independently established.

DISPOSITION: ARCHIVE_REFERENCE / negative invariant.

## RMCP-F06 — Local tool router names can silently replace routes

FAILURE MODE: `ToolRouter::add_route` inserts into a `HashMap` by tool name; a later duplicate name replaces the earlier route.

CAUSE: local server tool namespace is name-keyed and optimized for application composition, not global provenance identity.

SPARK LESSON: canonical capability identity must include host-owned stable identity/provenance/version. Mapping to one MCP wire name is a resolver step; duplicate canonical mappings must be detected rather than silently replaced.

DISPOSITION: REUSE_CODE behind canonical identity adapter; do not use router name as SPARK ID.
