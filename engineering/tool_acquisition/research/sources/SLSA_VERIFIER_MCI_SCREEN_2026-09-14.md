# slsa-verifier — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [slsa-framework/slsa-verifier](https://github.com/slsa-framework/slsa-verifier/tree/30d0be3bbab553fc51557377baba2f7572dfc212). Exact pin: `30d0be3bbab553fc51557377baba2f7572dfc212`. Runtime: Go provenance verifier. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: LARGE_EXPLICIT_SELECTION; no new claim of checkout on Fable's machine.

## Framing

Artifact consumers need builder/source provenance checks; attestations, identity and expected artifact information produce matches or errors.

Trusted builder/certificate/source expectations constrain provenance, not execution behavior or operator permission.

## Mechanism and failure semantics

GHA verifier code compares supported builder/certificate/source/digest evidence and handles provenance schema variants and special cases. README explicitly says the project is no longer actively maintained at this pin.

## Evidence and tests

Source screen at the pinned remote revision; source exceeds the normal checkout cutoff. No verification command or provenance corpus executed.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [README.md](https://github.com/slsa-framework/slsa-verifier/blob/30d0be3bbab553fc51557377baba2f7572dfc212/README.md)
- [verifiers/internal/gha/verifier.go](https://github.com/slsa-framework/slsa-verifier/blob/30d0be3bbab553fc51557377baba2f7572dfc212/verifiers/internal/gha/verifier.go)
- [LICENSE](https://github.com/slsa-framework/slsa-verifier/blob/30d0be3bbab553fc51557377baba2f7572dfc212/LICENSE)

## MCI transfer

Keep verifier failure cases and provenance vocabulary as reference. Maintenance status and alternative signing ecosystem make it unsuitable as the preferred new MCI foundation without renewed evaluation.

Finding classes: reference/qualification assessment. Filing home: AUDIT; cross-references: GOV, SEC, TEST. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

ARCHIVE_REFERENCE
