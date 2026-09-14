# in-toto — MCI source screen

Status: NARROW CODE SCREEN COMPLETE. Integration qualification: OPEN.

Study date: 2026-09-14. Upstream: [in-toto/in-toto](https://github.com/in-toto/in-toto/tree/e352b43ad7cb8915d84c36d791aa61346152a0a3). Exact pin: `e352b43ad7cb8915d84c36d791aa61346152a0a3`. Runtime: Python supply-chain layout verifier. License evidence: Apache-2.0; this is a root/declaration screen, not file-level reuse clearance. Source custody: PRIOR_PREPARATION_VERIFIED; no new claim of checkout on Fable's machine.

## Framing

Artifact pipelines need attested step and artifact checks; signed layout, keys and links produce verification outcomes and inspection results.

The layout specifies trusted keys/steps and may contain inspection commands. Verification can therefore cross into host execution, not just read evidence.

## Mechanism and failure semantics

in_toto_verify verifies layout signatures/expiry, loads and verifies link signatures, validates thresholds and artifact rules, then runs layout inspections through run_all_inspections. verify_command_alignment reports differences as warnings rather than treating every command difference as fatal.

## Evidence and tests

Source ordering and inspection path screened; no supplied layout or inspection command executed. Full cryptographic/tool suite not run.

This record supports the named mechanisms only. It does not claim a complete audit of the repository or that every relevant negative case is protected. Exact source evidence (published by the upstream project, retrieved 2026-09-14):

- [in_toto/verifylib.py](https://github.com/in-toto/in-toto/blob/e352b43ad7cb8915d84c36d791aa61346152a0a3/in_toto/verifylib.py)
- [LICENSE](https://github.com/in-toto/in-toto/blob/e352b43ad7cb8915d84c36d791aa61346152a0a3/LICENSE)

## MCI transfer

Borrow attested input/output chains and explicit verification stages for AUDIT. Keep executable inspections behind independent host admission and sandboxing. Defer the full verifier until supply-chain demand; do not feed untrusted layouts into a supposedly passive research/verifier path.

Finding classes: PATTERN; TEST/INVARIANT. Filing home: AUDIT; cross-references: GOV, TEST. Resource/platform claims beyond the inspected source are unverified; benchmark and dependency/API qualification are pending.

## Primary disposition

BORROW_PATTERN
