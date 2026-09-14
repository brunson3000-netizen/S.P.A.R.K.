# wasmcloud — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [wasmCloud/wasmCloud](https://github.com/wasmCloud/wasmCloud/tree/1f82c28e638372fbf5a924c96214a4c3629e99f8). Exact pin: `1f82c28e638372fbf5a924c96214a4c3629e99f8`. Runtime: Rust wash runtime / component host. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Component hosts need egress controls beyond hostnames; address policy accepts resolved IP addresses and returns permit/deny decisions.

The host's network implementation applies policy; the helper itself is only an address predicate. Host configuration selects private/special-address treatment.

## Mechanism and failure semantics

EgressAddressPolicy::permits canonicalizes IPv4-mapped IPv6 and evaluates special/private ranges. Default denies special addresses but permits private addresses; permissive configuration exists. Module documentation places this after hostname policy and DNS resolution as a rebinding defense.

## Evidence and tests

Inline address-policy tests inspected, including special addresses, mapped IPv6 and configurable private ranges; not executed. Full DNS-to-connect path was not traced.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [crates/wash-runtime/src/host/egress_policy.rs](https://github.com/wasmCloud/wasmCloud/blob/1f82c28e638372fbf5a924c96214a4c3629e99f8/crates/wash-runtime/src/host/egress_policy.rs)
- [LICENSE](https://github.com/wasmCloud/wasmCloud/blob/1f82c28e638372fbf5a924c96214a4c3629e99f8/LICENSE)

## MCI transfer

Borrow the small deterministic address-classification pattern below the MCI execution contract. Default private-network access is not an MCI choice. Actual connect-time address pinning, DNS changes, proxies and host callbacks require tests. Avoid importing the wider hosting platform without a consumer need.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: SEC; cross-references: COORD, PROTO. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
