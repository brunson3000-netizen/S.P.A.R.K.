# Failure Lessons — Extism

Source pin: `d5da29759bba88645f886d9e12d3f4e4376df7b3`
Status: FIRST TRACE COMPLETE

## E-001 — Guest capability policy does not constrain host-side plugin acquisition

FAILURE MODE: An untrusted manifest can name a host file or remote URL for Wasm acquisition while guest filesystem/HTTP permissions appear narrow.

CAUSE: `Wasm::File` and `Wasm::Url` are resolved by `runtime/src/manifest.rs` before `CurrentPlugin` creates WASI/PDK guest grants. Runtime default features enable both `register-filesystem` and `register-http`.

PROJECT RESPONSE: optional SHA-256 metadata can pin expected bytes; acquisition features can be disabled at compile time.

S.P.A.R.K. LESSON: source acquisition is a separate host authority plane. Resolve and digest plugin bytes before runtime instantiation; do not let plugin metadata choose arbitrary host acquisition endpoints.

TRANSFER CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN; default source-registration features should be disabled for the first SPARK experiment.

## E-002 — No fuel configuration means unmetered initialization and calls

FAILURE MODE: Runtime appears bounded because timeout/memory knobs exist, while guest instruction execution has no deterministic compute ceiling.

CAUSE: Extism enables Wasmtime fuel instrumentation only when a call fuel limit is configured. Without it, both initialization and calls are unmetered by fuel.

PROJECT RESPONSE: `with_fuel_limit` plus optional `with_initialization_fuel_limit`; new pin explicitly meters initialization/start/constructors.

S.P.A.R.K. LESSON: finite initialization and per-call fuel are mandatory profile fields for admitted plugins; absence is not an acceptable implicit default.

TRANSFER CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN.

## E-003 — Hostname allowlist is not a full network capability contract

FAILURE MODE: A hostname glob may be mistaken for complete egress confinement.

CAUSE: built-in PDK HTTP authorization checks the initial URL hostname against `allowed_hosts`. The manifest does not express HTTP method, scheme, port, path, resolved address/CIDR or destination class.

PROJECT RESPONSE: denies when hostname does not match; applies response-size and remaining-time bounds.

OPEN QUALIFICATION: Redirect-host revalidation was not demonstrated in this source trace. DNS/private-address behavior also needs adversarial tests.

S.P.A.R.K. LESSON: treat `allowed_hosts` as a convenient application-level filter, not sufficient proof of egress containment. Host/network sandbox policy must remain authoritative.

TRANSFER CONFIDENCE: HIGH for granularity limitation; UNKNOWN for redirect behavior until tested.

DISPOSITION: EXPERIMENT_NOW.

## E-004 — Higher-level manifest hides part of the lower-level resource surface

FAILURE MODE: A small manifest-level memory setting may be interpreted as complete Wasmtime resource control.

CAUSE: Extism exposes max pages, vars and HTTP response bytes, but this first trace did not find manifest-level equivalents for Wasmtime StoreLimits counts such as instances/tables/memories. Host functions and native allocations also sit outside simple guest-memory accounting.

PROJECT RESPONSE: Extism’s builder permits a Wasmtime config, while Extism overwrites several runtime options itself.

S.P.A.R.K. LESSON: qualification must enumerate **applied** Wasmtime settings under Extism, not infer them from the Extism manifest. Prefer raw Wasmtime if required controls are obscured or awkward to verify.

TRANSFER CONFIDENCE: MEDIUM-HIGH.

DISPOSITION: EXPERIMENT_NOW.

## E-005 — Runtime convenience host functions remain authority-bearing

FAILURE MODE: Wasm isolation is assumed to make arbitrary linked native functions safe.

CAUSE: `with_function*` callbacks execute native host code and receive mutable plugin/host context.

PROJECT RESPONSE: host explicitly registers functions; plugin cannot create them itself.

S.P.A.R.K. LESSON: linker/import generation must come from the same host grant that authorizes a capability. Every host function is part of the applied authority evidence.

TRANSFER CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN.
