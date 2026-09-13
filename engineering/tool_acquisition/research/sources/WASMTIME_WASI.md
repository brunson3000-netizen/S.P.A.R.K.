# Source Study — Wasmtime + WASI

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: Wasmtime
- Upstream: `https://github.com/bytecodealliance/wasmtime`
- Commit: `817c58787f432bcdbbb87679011f72c5bc80dbda`
- Workspace version at pin: `50.0.0-dev`
- License: Apache-2.0 WITH LLVM-exception
- Primary evidence: `wasmtime-wasi` context/filesystem, store resource limits, interruption docs/tests, host linker APIs, WASI adversarial filesystem tests.

## Problem framing

PROBLEM: execute untrusted WebAssembly with an embedder-controlled set of host capabilities and resource bounds instead of ambient native process authority.

USER: Rust/native hosts embedding Wasm modules/components.

INPUT: Wasm module/component, Wasmtime engine/store configuration, WASI capability context, host functions and resource/interruption policy.

OUTPUT: isolated guest execution whose available filesystem/network/host APIs are only those explicitly linked/configured, plus traps/results/host interactions.

AUTHORITY: extremely significant. The embedder chooses all preopened filesystem directories, network policy and host functions. Those choices define what the guest can reach. Wasmtime is a containment substrate, not the policy authority deciding whether a given capability should be granted.

TRUST: Wasmtime/Cranelift/WASI implementation, host-selected module bytes, host functions, preopen/network configuration, resource-limit configuration and OS primitives beneath the runtime.

STATE: Engine compiled-code/cache state, Store guest memories/tables/instances/resources, WASI context, host data/resources, optional component resources and host function state.

FAILURE: guest trap, memory/table growth refusal/trap, fuel exhaustion, epoch interruption, WASI permission errors, missing imports, host-function errors. Some limits are opt-in and cover only particular resource classes.

## Trace A — host constructs default-deny WASI capability context

`WasiCtxBuilder::new()`
→ no preopened directories by default
→ no initial cwd unless set
→ TCP disabled by default
→ UDP disabled by default
→ IP-name lookup disabled by default
→ host may explicitly add environment/args/stdin/out/clock/random/network/filesystem capabilities
→ `build()` produces per-Store `WasiCtx`.

### Filesystem

`preopened_dir(host_path, guest_path, FsPerms)`:
→ opens a host directory
→ maps it into guest namespace
→ grants `ReadOnly` or `ReadWrite`
→ path resolution remains capability-relative; docs state `..` cannot traverse above the supplied directory.

Current `FsPerms::ReadOnly` contract says guest may read reachable content/metadata but cannot change/append/truncate, create/delete files/directories/symlinks/hardlinks, etc.

Current pinned tests explicitly exercise readonly truncation and hardlink-cross-permission scenarios for WASI preview variants. This is important current-source evidence that historically sensitive filesystem boundaries have targeted regression coverage at this pin.

### Network

Network use is separately controlled:
- `allow_tcp(bool)` — default false
- `allow_udp(bool)` — default false
- `allow_ip_name_lookup(bool)` — default false
- `socket_addr_check` can approve/reject each concrete socket use asynchronously
- `inherit_network()` deliberately broadens to every host-accessible address.

SPARK implication: never call `inherit_network()` for a narrow plugin runtime. Compile egress intent into an explicit host address/operation policy.

## Trace B — guest resource limits

Wasmtime `ResourceLimiter` / `StoreLimits` can bound:
- linear-memory growth
- table elements
- number of instances
- number of tables
- number of memories.

Important caveats from the API itself:
- Store limits do **not** account for 100% of memory allocated by Wasmtime/host/embedder
- shared memories are not currently passed through the memory-growth callback
- default count limits are high (10,000 instances/tables/memories)
- memory/table size are unbounded by `StoreLimits::default()` unless the embedder explicitly configures them.

Therefore a SPARK plugin profile must set explicit low limits rather than rely on Wasmtime defaults.

## Trace C — CPU/interruption limits

Wasmtime exposes two distinct mechanisms.

### Fuel

`Config::consume_fuel(true)` + per-Store `set_fuel`
→ deterministic instruction-cost budget
→ same deterministic guest/input/fuel interrupts at the same execution point, absent other nondeterminism
→ higher execution overhead.

This is excellent for deterministic qualification/bounded plugins.

### Epoch interruption

`Config::epoch_interruption(true)` + deadline/trap/yield configuration
→ host increments engine epoch based on wall-time/scheduler
→ lower overhead
→ explicitly nondeterministic interruption point because it depends on wall time.

This is useful for wall-clock protection but not deterministic replay evidence.

Strong SPARK shape may use **both**:
- deterministic fuel ceiling for computation budget
- independent host wall-clock cancellation/deadline as a final operational stop.

Neither should be mislabeled as the other.

## Trace D — host functions define the real authority edge

Wasmtime `Linker`/component linker lets the embedder add arbitrary host functions and resources. Guest code can call only imports made available, but each host callback executes native host code and can access host `Store<T>` data/resources according to how the embedder writes it.

This is the most important containment lesson:

> Wasm memory isolation does not make a powerful host function safe.

A host function that exposes filesystem/network/credential/tool access effectively grants that authority through the sandbox.

SPARK must therefore generate linker imports from the same Tool Contract/authority grant used by the broker, not from plugin self-description.

## Trace E — capability/result evidence

A robust SPARK embedder can observe and record:
- module/component digest
- exact Wasmtime/WASI version
- linked host function set
- filesystem preopens + perms
- network policy
- store limits
- fuel allocation/consumption outcome
- wall-clock deadline/interruption
- trap/error/result.

This is strong support for `declared RunSpec → applied runtime verification` discovered in ToolHive.

## Tests/invariants extracted

### Default capability surface
- filesystem absent until a preopen is configured
- TCP/UDP/name lookup disabled until enabled.

### Filesystem isolation
- path traversal is capability-relative
- readonly permissions forbid write/truncate/create/delete/link operations
- pinned repo includes readonly truncation and hard-link-cross-permission tests.

### Resources
- resource growth can return false or trap
- explicit StoreLimits available for memory/table/counts
- APIs clearly state what they do not cover.

### Interruption
- fuel deterministic by instruction accounting
- epochs lower overhead but nondeterministic/wall-time-oriented.

## Boundary findings

### Wasm guest ↔ host filesystem/network
Strong capability boundary when the embedder supplies only narrow preopens/socket rules. A broad preopen or `inherit_network()` is a deliberate capability grant, not a sandbox failure.

### Guest ↔ host functions
Potentially the broadest escape from isolation by design. Every host function is an authority-bearing API and needs independent review/grant/provenance.

### Resource limiter ↔ total host resource consumption
Limiter bounds guest resources, not every Wasmtime/host allocation. Process-level/container/cgroup/OS limits may still be required for hostile or high-risk workloads.

### Fuel ↔ time
Fuel is deterministic compute budget, not wall time. Epoch/deadline is operational time bound, not deterministic instruction budget.

### Sandbox ↔ host authority
Wasmtime enforces the capabilities it is configured to expose. It does not decide whether an Operator/project intended those grants.

## Determinization candidates

- Wasm/component digest verification
- capability-specific WASI context construction
- preopen mapping/permission generation
- destination-aware socket policy
- explicit StoreLimits
- deterministic fuel budgets
- epoch/wall-clock deadline
- host-function linker generation from Tool Contract
- trap/resource/limit evidence capture.

## Material extracted

IDEA: WebAssembly/WASI is a strong narrow plugin containment substrate for deterministic adapters whose needed capabilities can be expressed explicitly.

PATTERN: default-deny guest environment with host-generated filesystem/network/function capabilities.

PATTERN: deterministic fuel + separate wall-clock stop.

CODE: Wasmtime/wasmtime-wasi are strong direct Rust reuse candidates under their license, subject to security-update pinning and adversarial qualification.

TEST/INVARIANT: actual linked host functions/preopens/network/limits are part of applied authority evidence; sandbox substrate never substitutes for broker authorization.

## Primary disposition

EXPERIMENT_NOW, with strong REUSE_CODE potential.

A passing experiment should compare raw Wasmtime against Extism for implementation burden and authority transparency before choosing the first plugin runtime abstraction.
