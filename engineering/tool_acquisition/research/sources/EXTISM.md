# Source Study — Extism Runtime

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: Extism
- Upstream: `https://github.com/extism/extism`
- Commit: `d5da29759bba88645f886d9e12d3f4e4376df7b3`
- Commit purpose: upgrade to Wasmtime 48 LTS and migrate WASI Preview 1 integration
- License: BSD-3-Clause
- Primary runtime: Rust SDK/runtime over Wasmtime 48 + `wasmtime-wasi` Preview 1
- Primary evidence: `runtime/src/manifest.rs`, `runtime/src/current_plugin.rs`, `runtime/src/pdk.rs`, `runtime/src/plugin.rs`, `runtime/src/plugin_builder.rs`, `runtime/src/tests/runtime.rs`, `manifest/src/lib.rs`, `runtime/Cargo.toml`.

## Problem framing

PROBLEM: Provide a portable higher-level WebAssembly plugin runtime with a stable manifest, host-function ABI, WASI filesystem mapping, HTTP controls, memory limits, timeout/cancellation and fuel without requiring every host to build raw Wasmtime plumbing.

USER: applications embedding third-party or dynamically loaded Wasm plugins.

INPUT: Wasm bytes or an Extism manifest; optional host functions; optional WASI; allowed filesystem paths/HTTP hosts; memory/output/time/fuel settings.

OUTPUT: instantiated plugin, typed calls/results, host-function interactions, errors/traps and cancellation/timeout behavior.

AUTHORITY: Extism enforces the capability envelope the embedding host gives it, but the embedding host chooses the manifest, enabled features, WASI flag and host functions. It is therefore containment/plugin infrastructure, not the authority deciding whether a plugin deserves those grants.

TRUST: Extism/Wasmtime implementation, host-selected plugin source/manifest, host functions, build features, manifest grants and configured limits.

STATE: plugin vars, Wasmtime store/memory/instances, manifest/config, optional WASI context, timer/cancel state, host context and compilation cache.

FAILURE: compilation/link/instantiation error, hash mismatch, denied filesystem/HTTP access, memory/OOM, fuel exhaustion, timeout/cancel, host-function error, trap. Several important bounds are opt-in.

## Trace A — plugin source acquisition happens before guest containment

`PluginBuilder::new(source)`
→ `CompiledPlugin::new`
→ `manifest::load`
→ resolve one or more Wasm modules
→ compile Wasmtime modules
→ only later construct `CurrentPlugin`, WASI context and guest runtime capability envelope.

Extism accepts three module source forms:

### In-memory bytes
`Wasm::Data`
→ optional SHA-256 check
→ compile bytes.

This is the strongest shape for SPARK because source acquisition can already have been authorized, provenance-checked and digested by the host.

### Host filesystem
`Wasm::File`
→ when compile feature `register-filesystem` is enabled
→ direct host `std::fs::read(path)`
→ optional SHA-256 check
→ compile.

This read occurs on the **host side** and is unrelated to guest `allowed_paths`/WASI preopens.

### Host network
`Wasm::Url`
→ when compile feature `register-http` is enabled
→ host performs the declared HTTP request, including caller-supplied headers/method
→ downloads module bytes
→ optional SHA-256 check
→ compile.

This HTTP fetch is also host-side and is unrelated to the plugin’s `allowed_hosts` runtime HTTP list.

At this pin the `extism` runtime crate’s default features include:
- `http`
- `register-http`
- `register-filesystem`
- Wasmtime default features.

### SPARK implication

**Code acquisition authority and guest runtime authority are different boundaries.**

A SPARK plugin adapter should not accept an untrusted plugin manifest as permission to read arbitrary host files or fetch arbitrary module URLs. Resolve source under the host’s acquisition/provenance policy first, require a full digest, then give the runtime admitted bytes (or an equivalent host-owned immutable artifact reference).

For qualification, build Extism with no default features and add only explicitly required runtime features.

## Trace B — guest filesystem and HTTP capability envelope

### WASI filesystem

`PluginBuilder` defaults `wasi=false`.

When WASI is enabled, `CurrentPlugin::new` constructs `WasiCtxBuilder`. Only entries in `manifest.allowed_paths` are preopened.

Each host-path key:
- starts with `ro:` → `FsPerms::ReadOnly`
- otherwise → `FsPerms::ReadWrite`.

The mapping explicitly maps host path → guest path. With no allowed paths, no filesystem preopens are added.

This is a convenient manifest wrapper around Wasmtime’s capability filesystem. It does not change the underlying lesson from raw Wasmtime: the host-selected preopen is the actual grant.

### Plugin PDK HTTP

`allowed_hosts` controls the built-in `extism:host/env::http_request` host function.

`http_request`:
→ parses URL
→ extracts initial URL hostname
→ compares hostname against configured glob patterns
→ denies if no pattern matches (`None` means deny)
→ sends request using host HTTP client
→ applies the remaining plugin wall-time timeout
→ caps response bytes (configured `max_http_response_bytes`, otherwise 50 MiB)
→ returns result through plugin memory.

This is default-deny for the built-in HTTP PDK when no host list is present.

Important granularity limitation: the manifest allowlist is hostname-oriented. It does not itself express method, scheme, port, path, resolved IP/CIDR or other network effects.

Redirect-host revalidation was **not demonstrated** in this first source trace. It must be tested before this mechanism is credited as a complete egress boundary.

### Host functions

`PluginBuilder::with_function` / `with_function_in_namespace` can link arbitrary native host functions. These run with host process authority and can expose whatever the embedder implements.

As with raw Wasmtime, host functions are the real authority edge. A manifest cannot make a powerful host callback safe.

## Trace C — resource budgets

### Memory

Manifest `MemoryOptions` provides:
- `max_pages`
- `max_http_response_bytes`
- `max_var_bytes` (default 1 MiB).

When `max_pages` is set, Extism installs a Wasmtime `ResourceLimiter` that tracks memory-growth bytes. Table growth is accepted up to the module-declared maximum when one exists.

The manifest-level abstraction is narrower than raw Wasmtime `StoreLimits`; this first trace did not find manifest knobs for total instances/tables/memories equivalent to the lower-level StoreLimits count limits. SPARK must not assume the higher-level manifest expresses every resource bound available in Wasmtime.

### Deterministic fuel — initialization versus calls

At this pin Extism deliberately separates two budgets:

- `with_fuel_limit(C)` → fresh call budget for every plugin call
- initialization budget defaults to C
- `with_initialization_fuel_limit(I)` overrides initialization only
- initialization override without a call limit is rejected
- no call fuel configured → Wasmtime fuel instrumentation disabled; initialization and calls are unmetered.

Initialization budget covers linking/instantiation/start/reactor initialization/constructors. The call budget is refreshed before normal call input/execution.

This avoids two failure classes:
1. an unbounded constructor/start function consuming arbitrary CPU before the first requested export; and
2. initialization cost unpredictably reducing only the first normal call’s budget.

Tests explicitly cover infinite reactor initialization, Wasm start functions and guest constructors exhausting fuel.

### Wall-clock timeout and cancellation

Extism enables Wasmtime epoch interruption and maintains a timer/cancel mechanism.

Manifest `timeout_ms` can stop a plugin call; `CancelHandle` can stop a running plugin from another thread. Runtime tests exercise an infinite-loop timeout and repeated cancellation.

Fuel and timeout remain different mechanisms:
- fuel = deterministic-ish Wasm computation accounting
- timeout/cancel = operational wall-clock stop.

Both are useful; neither substitutes for the other.

## Trace D — result/output surfaces

Useful built-in bounds include:
- plugin var store size
- HTTP response body size
- Wasm memory pages
- call/init fuel
- call timeout.

But output/context policy for SPARK still belongs above Extism: a plugin return may be valid yet too large/noisy for an agent. The existing SPARK evidence/reduction doctrine remains `immutable raw evidence → derived compact/search views`.

## Tests/invariants extracted

Pinned runtime tests protect:
- external cancellation of infinite plugin execution
- timeout of infinite plugin execution
- repeated per-call fuel exhaustion/refill
- initialization-fuel requires call-fuel
- infinite reactor initialization exhausts initialization fuel
- infinite module start exhausts fuel
- infinite guest constructor exhausts fuel
- fuel consumption reporting
- HTTP timeout
- repeated instantiation behavior
- read-only-path usage through `ro:` manifest preopen.

Wasmtime’s own pinned regression tests remain the stronger direct evidence for detailed `FsPerms` semantics such as readonly truncate/link boundaries.

## Boundary findings

### Artifact source ↔ host
**Separate authority plane.** `register-http` and `register-filesystem` perform host acquisition before guest sandbox configuration. The guest’s host/path allowlists do not constrain this step.

### Guest ↔ filesystem
Good narrow wrapper over Wasmtime capability preopens when the host creates the manifest/grants.

### Guest ↔ HTTP
Convenient hostname-based PDK guard, but materially less expressive than a host capability contract that includes address, port, method and destination class. Redirect/DNS/private-address behavior requires qualification.

### Guest ↔ native host functions
Every linked function is an authority-bearing escape hatch by design. Functions must be generated from host-approved capability contracts.

### Higher-level runtime ↔ lower-level substrate
Extism removes integration work but also hides some Wasmtime knobs/defaults. A SPARK adapter needs an explicit record of what Extism configures versus what Wasmtime still defaults.

## Determinization candidates

- pre-resolve/pin plugin bytes by full digest
- manifest generation from host grants rather than plugin self-request
- `allowed_paths` generation from filesystem grants
- HTTP destination policy generation
- explicit max pages/vars/response limits
- initialization + per-call fuel generation
- timeout/cancel binding
- exact host-function linker generation
- applied runtime evidence capture.

## Material extracted

IDEA: Extism is a productive higher-level wrapper for a narrow Wasm plugin contract.

PATTERN: source acquisition is distinct from guest runtime capabilities.

PATTERN: initialization compute budget and recurring call budget are distinct resources.

PATTERN: a manifest can be an execution **description**, but host policy must generate/validate it rather than treating plugin declarations as grants.

CODE: Extism is a viable Rust dependency candidate under BSD-3-Clause, but direct adoption should wait for the raw-Wasmtime-vs-Extism experiment.

TEST/INVARIANT: an untrusted manifest cannot cause host-side source acquisition or widen guest capabilities without an independent host grant.

## Primary disposition

EXPERIMENT_NOW.

### Current recommendation

Use **raw Wasmtime as the baseline** for SPARK’s first containment prototype because its authority surface is more explicit and it avoids Extism’s default host-side source registration features. Compare Extism as the higher-level alternative. Prefer Extism only if it materially reduces implementation burden while preserving the same host-generated capability/evidence contract.
