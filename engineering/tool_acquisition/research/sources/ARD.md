# Source Study — Agentic Resource Discovery (ARD)

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: Agentic Resource Discovery Specification
- Upstream: `https://github.com/ards-project/ard-spec`
- Commit: `b76f235a8f461876ad4f1e77abd0eb0eb302b48d`
- Retrieval/study date: 2026-09-13
- License: Apache-2.0
- Form/runtime: specification in Markdown/JSON Schema; zero-dependency Python conformance tooling/examples
- Spec at pin: v0.91 Proposal, dated 2026-08-26
- Primary docs: `spec/ard.md`, ADRs, `conformance/README.md`

## Problem framing

PROBLEM: Static installation/hardcoding and eager context injection do not scale when agents need to discover thousands or millions of external capabilities.

USER: Resource publishers, discovery registries, orchestrators/agents and developer tooling.

INPUT: ARD entries published through well-known/static mechanisms or indexed registries; natural-language search text plus structured filters.

OUTPUT: Ranked discovery results carrying stable identifiers and selected metadata, optional registry referrals, or deterministic/list/introspection results.

AUTHORITY: ARD defines discovery, not execution authority. Authentication and actual invocation are delegated to the resource’s native protocol.

TRUST: Search relevance and trust are explicitly separate. Optional `trustManifest` data can bind publisher identity and feed verification/filtering, but ARD does not define the signing framework itself.

STATE: A conforming dynamic registry maintains an index. The spec itself does not prescribe registry storage or mutable runtime state.

FAILURE: Standard HTTP errors; unsupported optional endpoints may return 404/501; malformed manifests/queries fail conformance. Search remains the mandatory interoperability floor.

## Execution trace A — publish / ingest

Publisher describes one resource as an ARD entry:
- globally unique `identifier`
- `displayName`
- artifact `type`
- exactly one of `url` or inline `data`
- preferably `representativeQueries`
- optionally `capabilities`, metadata and `trustManifest`.

Publisher exposes entries through mechanisms such as `/.well-known/ard.json`, in-page JSON-LD, link relation, agentmap directive or DNS pointer.

Registry/consumer resolves entries and indexes recognized discovery terms while preserving extensible terms.

Important mechanism: artifact description is an envelope around MCP/A2A/skill/etc., not a replacement for their execution schemas.

## Execution trace B — search

Client/orchestrator
→ `POST /search`
→ `query.text` + optional structured filter
→ registry relevance/filter processing
→ bounded page (`pageSize`, default 10, max 100)
→ result entries ranked by semantic relevance
→ optional referrals to other registries.

Result invariant:
- `identifier` is mandatory in each result
- other fields may be projected/omitted by the registry
- `score` is relevance only and MUST NOT be interpreted as trust/compliance/safety.

A result may therefore be a compact selection projection rather than a full ARD entry.

## Execution trace C — discovery → invocation handoff

Orchestrator receives a selected ARD result
→ follows the artifact `url` or inline `data`
→ learns the native artifact (for example MCP server card, A2A agent card, skill)
→ invokes it using that artifact’s native protocol.

ARD explicitly delegates authentication and execution. It does not make discovery itself an authorization grant.

This is the strongest architectural distinction from progressive MCP Guardian: ARD stops at discovery/description; Guardian also performs execution through the same middleware.

## Federation trace

Search supports:
- `none` — local registry only
- `referrals` — local results plus other registries for client-controlled traversal
- `auto` — registry performs upstream federation and merges results.

This provides a useful reference for future federated capability catalogs, but no federation design is adopted by this study.

## Stable identity mechanism

ARD requires domain-anchored identifiers shaped like:
`urn:air:<publisher>:<namespace>:<agent-name>`.

The identifier is intentionally separate from:
- mutable network location (`url`)
- dynamic security principal/credential.

Optional trust-manifest identity must align with the publisher domain, giving registries a namespace-squatting defense without making the discovery identifier itself a credential.

## Tests/conformance evidence

Official conformance tooling validates:
- JSON structural integrity
- optional JSON Schema validation
- strict domain-anchored URN form
- exactly-one-of `url` / `data`
- media-type diagnostics
- `representativeQueries` discovery warnings
- well-known publisher resolution
- mandatory `POST /search` response shape
- optional `GET /agents` and `POST /explore` behavior.

The bundled basic registry example provides a concrete path from POST body validation → query extraction → catalog matching/ranking → result/referral/page envelope.

## Boundary findings

### Agent/orchestrator ↔ registry
Typed HTTP request/response shape; search page size is explicitly capped at 100. Search result projection can be compact.

### Discovery ↔ trust
Explicitly separated. Relevance score must not be treated as trust/safety. Trust metadata has a separate verification path.

### Discovery ↔ execution
Explicitly separated. ARD describes/fetches resources; native protocols own authentication and execution.

### Stable identity ↔ physical location
Explicitly separated by URN versus `url`/`data`. This is directly useful to avoid alias/name collisions and location churn.

## Important gaps / limits

1. A normative “retrieve complete ARD entry by identifier” operation is explicitly out of scope in this draft. Search results may be partial, so a `learn_capability(id)` layer needs a deterministic path to the authoritative full descriptor/artifact source.
2. The spec is still a Proposal, not a frozen standard.
3. Search implementation is intentionally flexible; informative text permits LLM interpretation/vector search, so ranking behavior is not deterministic by specification.
4. Trust verification frameworks are delegated; ARD gives a seam and binding rule, not a complete zero-trust enforcement system.

## Agent-facing surface implication

ARD itself need not be shown directly to workers. A host can compress it into a minimal primitive such as `find_capability(query, constraints)` while keeping full entry/artifact retrieval outside context.

The worker should receive only selection-relevant metadata plus a stable ID; learning and execution remain separate steps.

## Determinization candidates

- manifest/schema validation
- stable-ID parsing and collision checking
- filter evaluation
- pagination
- deterministic capability-token filters
- publisher-domain binding checks
- referral traversal policy
- descriptor/artifact retrieval

Semantic ranking may use non-deterministic methods, but admission/authority must not.

## Material extracted

IDEA: search-first capability discovery outside model context.

PATTERN: discovery/description is separate from trust and execution.

PATTERN: stable globally unique capability identity separate from location/security principal.

ALGORITHM: structured filter semantics and bounded pagination.

CODE: no direct code reuse recommended from first trace; conformance tests are more valuable than the example server.

TEST/INVARIANT: relevance must never be interpreted as authority/trust; stable ID must remain distinct from location and credential.

## Primary disposition

BORROW_PATTERN

Reason: ARD’s separation semantics and stable identity model align strongly with the emerging capability doorway. The exact wire standard remains a proposal and should be treated as interoperability prior art rather than automatically adopted.
