# NEMOCLAW — Ambiguous policy-write outcomes and policy identity

Status: SCOPED CODE STUDY COMPLETE; runtime qualification not executed.
Study date: 2026-09-14. Exact pin: `3ea2d5f9a515d4176e0745214743a48ff4868f4c`.
Upstream: https://github.com/NVIDIA/NemoClaw
Language/license at inspected root: TypeScript/Python/shell integration / Apache-2.0. Individual dependency/reuse terms still need qualification.
Finding class: PATTERN / TEST-INVARIANT unless the disposition says idea/reference.
Primary disposition: BORROW_PATTERN.

## Finding

Policy read parsing separates YAML and applied revision. Policy-set classification checks transport-failure markers before exit success; known structured refusal maps to rejected, uncertain errors map to ambiguous. This avoids treating a timeout or unstructured diagnostic as proof of no effect.

## Traced paths

1. Base policy read → parse YAML/metadata → typed document plus appliedRevision. Policy write result → transport-failure check → applied / rejected / ambiguous. A receipt is not proof that every running process has applied a revision.
2. assertPolicyRequirementContainment compares requested sections to inspected effective policy while permitting unrelated or enriched entries. Tests explicitly accept extra network entries/device paths, and reject missing or changed requested entries.

## Evidence

- [nemoclaw/src/shared/openshell-policy-boundary.cts](https://github.com/NVIDIA/NemoClaw/blob/3ea2d5f9a515d4176e0745214743a48ff4868f4c/nemoclaw/src/shared/openshell-policy-boundary.cts)
- [nemoclaw/src/shared/openshell-policy-boundary.test.ts](https://github.com/NVIDIA/NemoClaw/blob/3ea2d5f9a515d4176e0745214743a48ff4868f4c/nemoclaw/src/shared/openshell-policy-boundary.test.ts)

Containment and metadata test bodies inspected; malformed identity, missing/drifted requirements and permitted enrichment are covered. Outcome-classifier runtime tests not executed.

## MCI assessment

Borrow three-way mutation outcomes and explicit applied revision. Requirement inclusion checks are useful for minimum requirements; they do not prove least privilege or that an effective policy has no extra authority. MCI needs a separate upper-bound/excess-capability check where required.

## Limits and follow-up

OpenShell runtime enforcement, provider-composed policy provenance and the write/readback caller's reconciliation remain further qualification. This is integration prior art, not a substitute for the core.

These conclusions are research recommendations. No copied implementation, installed dependency, adopted policy or independent assurance is claimed.
