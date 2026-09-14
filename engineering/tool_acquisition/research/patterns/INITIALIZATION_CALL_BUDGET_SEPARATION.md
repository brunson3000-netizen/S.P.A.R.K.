# Pattern Card — Initialization and Call Budgets Are Distinct

PATTERN: Separate Initialization Compute Budget From Per-Call Compute Budget
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- Extism @ `d5da29759bba88645f886d9e12d3f4e4376df7b3`
- Wasmtime 48 LTS migration commit and fuel design
- `runtime/src/plugin_builder.rs`
- `runtime/src/plugin.rs`
- runtime tests for reactor initialization, start functions, constructors and recurring calls
- raw Wasmtime source study establishes fuel as distinct from epoch/wall-clock interruption.

## Problem

Plugin code can execute before the requested exported function: linking, instantiation, start functions, reactor initialization and constructors. A budget that covers only normal calls leaves a pre-call denial-of-service gap. Conversely, charging initialization against only the first call makes identical calls receive different effective budgets.

## Mechanism

Maintain separate host-owned budgets:

- **initialization fuel**: one bounded allowance for linking/instantiation/start/init/constructors
- **call fuel**: a fresh bounded allowance reset for each normal invocation
- **wall deadline/cancel**: separate operational stop independent of deterministic fuel.

The host selects every value; the plugin cannot widen them.

## Benefits

- bounds pre-call guest computation
- identical recurring calls receive comparable compute allowance
- deterministic fuel evidence remains separable from wall-clock interruption
- easier failure attribution (`init_fuel`, `call_fuel`, `deadline`, `cancel`).

## Risks / failure modes

- initialization left unmetered while calls are bounded
- call budget consumed by one-time runtime setup
- “unlimited initialization” used as a convenience default
- fuel misrepresented as wall-clock time
- host functions consume significant native work outside guest fuel accounting
- different runtime/JIT settings alter fuel/setup behavior.

## Boundaries crossed

Artifact → runtime initialization.
Initialized plugin → repeated execution calls.
Guest instruction budget → host wall-clock/process budget.

## Determinization relevance

HIGH. Budget generation, resets, attribution and stop reasons belong in deterministic host runtime state.

## Likely architectural location

Plugin/adapter execution supervisor.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. Initialization has an explicit finite compute ceiling.
2. Every normal call receives a fresh explicit ceiling.
3. Wall-clock cancellation/deadline exists independently of fuel.
4. Host-function native work is not falsely claimed to be covered by Wasm fuel.
5. Result evidence distinguishes initialization failure, call-fuel exhaustion, timeout, cancellation and ordinary trap/error.
