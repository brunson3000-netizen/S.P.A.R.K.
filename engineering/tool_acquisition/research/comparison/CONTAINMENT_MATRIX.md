# Containment / Tool-Safety Comparison Matrix

Status: FIRST CONVERGENCE PASS
Updated: 2026-09-13

Sources:
- Goose `50666ae0b9a51e260b52b7efbab2e4e020346e94`
- Wasmtime `817c58787f432bcdbbb87679011f72c5bc80dbda`
- Extism `d5da29759bba88645f886d9e12d3f4e4376df7b3`

| Dimension | Goose | Raw Wasmtime/WASI | Extism | SPARK implication |
|---|---|---|---|---|
| Layer | pre-execution tool inspection | Wasm containment substrate | higher-level plugin runtime over Wasmtime | use different layers together; do not confuse inspection with containment |
| Primary value | permission/safety findings, intended-effect extraction | explicit capability boundary and low-level limits | faster plugin integration + portable manifest/PDK | host broker stays above all three |
| Hard authority | mixed; permission baseline + stricter inspectors, some advisory/fail-open | enforces exactly configured runtime capabilities | enforces configured guest/runtime controls | authoritative grant originates in SPARK, not the source project |
| Default filesystem | N/A; observes calls | no preopens | WASI off by builder default; no preopens unless allowed paths | default deny |
| Filesystem grant | effect/policy inspection | explicit host→guest preopen + FsPerms | manifest `allowed_paths`; `ro:` → ReadOnly | generate from host grant, record applied mapping |
| Guest network | egress extraction only; no containment | TCP/UDP/name lookup default off; socket policy can check concrete use | built-in HTTP PDK default-deny by hostname list; WASI separate | actual egress must be enforced below observation/ranking |
| Network policy granularity | parses destinations/direction | potentially concrete address/operation checks | hostname glob for PDK HTTP | SPARK capability contract should be richer than host glob |
| Source acquisition | extension/tool loading outside inspector focus | embedder normally supplies selected bytes/modules | default crate features permit host file/HTTP Wasm registration before guest grants | acquisition is its own authority/provenance plane |
| Host functions | N/A for containment | arbitrary linker host functions | `with_function*` native callbacks | every import/function is an authority-bearing grant |
| Memory/resources | N/A | StoreLimits/ResourceLimiter expose memory/table/count limits; explicit config needed | max pages + var/HTTP bounds; narrower high-level surface | applied lower-level limits must be inspectable even through wrapper |
| Deterministic compute | N/A | Wasmtime fuel | per-call fuel plus separate initialization fuel | finite init + fresh per-call fuel |
| Wall stop | N/A | epoch interruption / host deadline | manifest timeout + CancelHandle + epoch interruption | use separate operational deadline/cancel |
| Safety composition | Deny/RequireApproval monotonically tighten baseline | capability set fixed by host | manifest + builder + runtime feature composition | secondary reviewers may tighten, never mint authority |
| Probabilistic checks | security classifier / adversary model; some fail-open | none required | none required | probabilistic review only defense-in-depth |
| Runtime evidence clarity | findings/telemetry | highest: linker, preopens, sockets, limits, fuel directly visible | moderate: convenient manifest but wrapper config/defaults must be expanded | record resolved contract and applied substrate state |
| Supply-chain identity | outside main trace | host selects module bytes/digest | optional per-module SHA-256; URL/file acquisition possible | require host-owned full digest/provenance before instantiate |
| Integration burden | agent-runtime specific | highest | lower | choose transparency unless wrapper produces material savings |
| Best transfer | monotonic inspectors + deterministic effect extraction | first containment prototype / likely direct Rust reuse | comparison candidate; manifest/budget patterns | combine, do not adopt wholesale |
| Current disposition | BORROW_PATTERN | EXPERIMENT_NOW + REUSE_CODE potential | EXPERIMENT_NOW | run one identical plugin-boundary experiment |

## Convergence

The three projects solve different parts of the same boundary:

1. **Goose** asks: “Does this proposed operation look allowed/risky?”
2. **SPARK broker** must answer: “Is this operation currently authorized?”
3. **Wasmtime/Extism** enforce: “What can admitted plugin code physically reach and consume?”

The resulting target shape is:

`model proposal → deterministic effect extraction/secondary inspection → host authorization → verified plugin artifact → generated runtime capabilities → contained execution → applied-authority evidence → result/evidence`

No inspection result, plugin manifest or sandbox declaration becomes authority by itself.

## Current recommendation

Use **raw Wasmtime/WASI as the experiment baseline**. Its extra integration work buys maximum visibility into capabilities and limits, which is valuable while the SPARK Tool Contract is still being established.

Run Extism against the same test vector. Promote it only if it substantially reduces code/maintenance cost without obscuring required grants, source provenance, network semantics or applied-resource evidence.

Goose is not a competing runtime; borrow its monotonic inspector/effect-extraction patterns above the broker.
