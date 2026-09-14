# cosign — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [sigstore/cosign](https://github.com/sigstore/cosign/tree/633e8f4303b7db64c369bdfa315374a8bd511ac6). Exact pin: `633e8f4303b7db64c369bdfa315374a8bd511ac6`. Runtime: Go artifact verification CLI/libraries. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Artifact consumers need signature and identity checks; blob, key/certificate/bundle and verification options yield evidence validation or errors.

Configured trust roots, identities and transparency options define what is verified. A signature does not authorize executing the artifact or prove its behavior safe.

## Mechanism and failure semantics

VerifyBlob requires usable key/certificate/bundle inputs, enforces relevant exclusive options, constructs identities and CheckOpts, and invokes verification. Offline, IgnoreTlog and IgnoreSCT options change the assurance contract.

## Evidence and tests

Selected verification setup screened; no artifact signature, trust-root rotation or offline bundle test executed.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [cmd/cosign/cli/verify/verify_blob.go](https://github.com/sigstore/cosign/blob/633e8f4303b7db64c369bdfa315374a8bd511ac6/cmd/cosign/cli/verify/verify_blob.go)
- [LICENSE](https://github.com/sigstore/cosign/blob/633e8f4303b7db64c369bdfa315374a8bd511ac6/LICENSE)

## MCI transfer

Defer operational use until signed package distribution is a concrete MCI need. Retain a verification-before-activation contract and record identity, digest and verification mode. Avoid making mutable tags or a successful signature synonymous with runtime permission.

Finding classes: reference/qualification assessment. Filing home: AUDIT; cross-references: GOV, SEC. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

DEFER
