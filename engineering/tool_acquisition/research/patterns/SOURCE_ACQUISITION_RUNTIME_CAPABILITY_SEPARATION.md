# Pattern Card — Source Acquisition Is Separate From Runtime Capability

PATTERN: Separate Code Acquisition Authority From Guest Runtime Authority
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- Extism @ `d5da29759bba88645f886d9e12d3f4e4376df7b3`
- `runtime/src/manifest.rs`
- `runtime/Cargo.toml`
- contrast: Wasmtime embedder receives host-selected module/component bytes directly
- convergence: ToolHive provenance/install research separates artifact verification from workload execution permissions.

## Problem

A sandbox can correctly restrict what loaded code may access while the loader itself performs privileged host filesystem/network work to obtain that code. If a plugin-supplied manifest controls acquisition, the plugin can influence host-side reads/fetches before its guest capability envelope even exists.

## Mechanism

Treat plugin acquisition as a separate host-owned operation:

1. resolve source under trusted acquisition policy
2. validate source/provenance
3. materialize immutable bytes/artifact
4. verify full digest
5. record origin + digest + verifier result
6. only then instantiate the runtime with admitted bytes
7. independently derive guest filesystem/network/function grants.

Guest runtime declarations never serve as acquisition authority.

## Benefits

- closes pre-sandbox host-fetch gap
- clean supply-chain provenance
- deterministic/replayable plugin identity
- allows network/file acquisition to be disabled in the runtime entirely
- makes runtime qualification independent of package-discovery mechanism.

## Risks / failure modes

- manifest URL/file fields reach runtime loader before host validation
- optional hash omitted
- short/truncated digest used as canonical identity
- redirects/mirrors change bytes after policy decision
- runtime default features silently re-enable host acquisition
- cached module bytes lose origin/provenance binding.

## Boundaries crossed

External registry/file/URL → host artifact store.
Artifact store → runtime loader.
Runtime loader → guest capability environment.

## Agent-facing surface

None. The worker refers to a stable capability/plugin ID. The host resolves that ID to a verified artifact; the model never supplies arbitrary loader paths/URLs as authority.

## Determinization relevance

CRITICAL. Acquisition, digest verification, provenance checks and artifact custody are deterministic host responsibilities.

## Likely architectural location

Capability acquisition / provenance service upstream of the sandbox/plugin runtime.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. Untrusted plugin metadata cannot trigger arbitrary host filesystem or network acquisition.
2. Runtime receives an artifact whose full digest is already known and verified.
3. Acquisition provenance and guest capability grants are separate evidence records.
4. Guest network/filesystem grants never widen source acquisition.
5. Runtime compile features that fetch/read plugin code are disabled unless explicitly required and qualified.
