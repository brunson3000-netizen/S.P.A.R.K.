# nushell — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [nushell/nushell](https://github.com/nushell/nushell/tree/b6e6562a9a385df439ae864714c609c60314fde9). Exact pin: `b6e6562a9a385df439ae864714c609c60314fde9`. Runtime: Rust structured shell and plugin protocol. License evidence: MIT; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Shell/plugin authors need typed values and compatibility negotiation; Hello protocol/version/features produce compatibility decisions.

Native plugin execution retains OS authority. Protocol feature support is interoperability state, not a capability grant.

## Mechanism and failure semantics

ProtocolInfo::is_compatible_with parses both versions, orders them, removes prerelease markers and checks caret compatibility against the lower version. Parse failures return PluginFailedToLoad. supports_feature compares recognized features; unknown features deserialize but cannot be serialized as supported features.

## Evidence and tests

Source-level compatibility and error paths screened; native plugin spawn, process isolation and full plugin test suite not qualified.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [crates/nu-plugin-protocol/src/protocol_info.rs](https://github.com/nushell/nushell/blob/b6e6562a9a385df439ae864714c609c60314fde9/crates/nu-plugin-protocol/src/protocol_info.rs)
- [LICENSE](https://github.com/nushell/nushell/blob/b6e6562a9a385df439ae864714c609c60314fde9/LICENSE)

## MCI transfer

Borrow structured-value and negotiated-feature design for TOOLS/PROTO. Note the deliberate prerelease relaxation if exact revision compatibility matters. Keep native executable activation behind the same host policy as any other tool.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: TOOLS; cross-references: PROTO, SEC. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
