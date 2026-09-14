# Containment recovery addendum — 2026-09-14

Status: STATIC_SOURCE_FINDINGS; runtime tests NOT_RUN.
Primary disposition: EXPERIMENT_NOW.
Scope: supplements the existing containment study; no competing source study or experiment is created.

## Recovery and concurrent work

GitHub access is available for `brunson3000-netizen/S.P.A.R.K.`, branch `phase1-refoundation-v2`.
Initial repository HEAD: `795be8f2887911e91678e5a8760a991401e647fe`.
Recovery logged at `1aaac8a8e1670f5bf3e659c01dc6f7b8a213dffe`.
Before publishing this pass, HEAD advanced to `92f38ab3736b86e6a3f232362b3a0203abcaa845`; its source study and experiment were read. That work closes T-CONTAIN-01 and advances the active ledger to T-OBS-01. It is preserved. The old exported handoff is not current research state, and its obsolete harvesting script is not the current repository implementation.

This pass directly read Extism source at `d5da29759bba88645f886d9e12d3f4e4376df7b3`. Goose and Wasmtime conclusions were recovered from their prior repository studies, not independently re-reviewed. This is supplementary research, not formal independent assurance.

## Additional load-bearing findings

### 1. Manifest parsing can represent native memory

[manifest/src/lib.rs](https://github.com/extism/extism/blob/d5da29759bba88645f886d9e12d3f4e4376df7b3/manifest/src/lib.rs), `wasmdata::deserialize`, accepts a numeric ptr/len data object and uses native `slice::from_raw_parts`. [test_manifest_ptr_len](https://github.com/extism/extism/blob/d5da29759bba88645f886d9e12d3f4e4376df7b3/runtime/src/tests/runtime.rs) demonstrates the intended trusted in-process pointer case.

This means host validation must occur before Extism deserialization, not merely after obtaining a Manifest. Proposed wrapper: validate a separate bounded input schema, reject native-pointer/file/URL forms, then construct the runtime Manifest from admitted module bytes. Disabling registration features does not itself remove the pointer deserialization form.

Evidence class: CODE + TEST/INVARIANT. Confidence: high for the source path; no invalid pointer was executed and no exploit is claimed.

### 2. Call timer is not a whole-operation deadline

[runtime/src/plugin.rs](https://github.com/extism/extism/blob/d5da29759bba88645f886d9e12d3f4e4376df7b3/runtime/src/plugin.rs), `raw_call`, applies initialization fuel before reset/instantiation, refills call fuel, marshals input, then sends TimerAction::Start immediately before the export call.

The source supports initialization fuel at this pin. It does not establish that the call timeout also covers source acquisition, compilation and initialization. Arbitrary native callbacks need their own termination strategy. [timer.rs](https://github.com/extism/extism/blob/d5da29759bba88645f886d9e12d3f4e4376df7b3/runtime/src/timer.rs) increments Engine epochs; cancel returning successfully means message delivery was requested, not confirmed completion of all native work.

Evidence class: CODE + PATTERN. Confidence: high for ordering; callback-stop timing and cancellation isolation are untested.

### 3. Compare the abstraction without silently comparing different backends

[runtime/Cargo.toml](https://github.com/extism/extism/blob/d5da29759bba88645f886d9e12d3f4e4376df7b3/runtime/Cargo.toml) requests Wasmtime/WASI 48. The existing [Wasmtime study](sources/WASMTIME_WASI.md) inspected 50.0.0-dev.

For the raw-versus-Extism experiment, record exact dependency resolution and use an equivalent backend version first. Evaluate the separately pinned newer Wasmtime as another arm. Never transfer filesystem regression-test credit from the newer source to Extism's backend without checking its actual version. Do not modify source locks merely to conceal this difference.

Evidence class: CODE + TEST/INVARIANT. Confidence: high for declared versions; resolved dependency graph not built.

## Additions to the existing experiment

Apply these to [WASM_PLUGIN_BOUNDARY_EXPERIMENT.md](experiments/WASM_PLUGIN_BOUNDARY_EXPERIMENT.md), not a second experiment:

- Reject untrusted native-pointer manifest fields before the runtime parser is entered; use a loader-entry sentinel rather than executing invalid pointers.
- Instrument acquisition, compilation, initialization, input setup, exported call and native callback separately; establish deadline coverage for each.
- Exercise a stalled callback under an external child-process watchdog; distinguish runtime cancellation from watchdog termination.
- Cancel one of two instances sharing a compiled Engine and record sibling effects.
- Test vars/config/host-context isolation across reused instances.
- Record exact backend versions/features and separate adapter overhead from backend-version differences.

Use synthetic data and loopback fixtures. Runtime bounds and implementation remain to be established in the isolated experiment; this addendum does not activate it or add production dependencies.

## Resume pointer

Authoritative active queue: [ACTIVE_TRACE_LEDGER.md](ACTIVE_TRACE_LEDGER.md), T-OBS-01.
Next recorded sequence: AgentTrace, agent-observability, OpenTelemetry Collector; then observability comparison and flight-recorder experiment/schema proposal. Re-read the ledger before assignment because concurrent research is active.

Written and published as repository research only. No runtime installation, activation or consumer adoption occurred in this pass.
