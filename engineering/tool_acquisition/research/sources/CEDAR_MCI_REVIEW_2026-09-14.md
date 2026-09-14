# CEDAR — Policy evaluator with explicit diagnostics

Status: SCOPED CODE STUDY COMPLETE; runtime qualification not executed.
Study date: 2026-09-14. Exact pin: `2f4019fd645cc8d4a4c0c1f8bd0280c77d754e28`.
Upstream: https://github.com/cedar-policy/cedar
Language/license at inspected root: Rust / Apache-2.0. Individual dependency/reuse terms still need qualification.
Finding class: PATTERN / TEST-INVARIANT unless the disposition says idea/reference.
Primary disposition: BORROW_PATTERN.

## Finding

Cedar's authorizer evaluates each policy, records evaluation errors, and treats an errored policy as unsatisfied. Its own skip_on_error_tests demonstrates that an unconditional permit still yields Allow when a forbid errors on missing context. The public FFI carries decision and diagnostics separately.

## Traced paths

1. AuthorizationCall → parse/validate request inputs → Authorizer::is_authorized → is_authorized_core_internal → PartialResponse::concretize → decision plus diagnostic errors. Invalid FFI input is a failure response; per-policy evaluation errors are a different class and can accompany Allow.
2. Policy loop → permit/forbid result buckets → error bucket with skipped policy → final decision. This is intentional evaluator semantics, not proof of a host authorization defect.

## Evidence

- [cedar-policy-core/src/authorizer.rs](https://github.com/cedar-policy/cedar/blob/2f4019fd645cc8d4a4c0c1f8bd0280c77d754e28/cedar-policy-core/src/authorizer.rs)
- [cedar-policy/src/ffi/is_authorized.rs](https://github.com/cedar-policy/cedar/blob/2f4019fd645cc8d4a4c0c1f8bd0280c77d754e28/cedar-policy/src/ffi/is_authorized.rs)

Inline skip_on_error_tests and authorizer_sanity_check_allow/deny read in authorizer.rs. Tests inspected, not executed.

## MCI assessment

Candidate: use the public cedar-policy evaluator behind a Rust-owned adapter that checks diagnostics as well as decision, current policy revision and current identity. Do not copy internal authorizer code or adopt Cedar precedence as MCI law. A consumer policy that requires error-free admission needs an explicit diagnostics gate.

## Limits and follow-up

Policy-version ownership, schema/entity freshness, concurrency, resource bounds and host enforcement remain outside this trace. Full API/dependency qualification and consumer equivalence tests remain open.

These conclusions are research recommendations. No copied implementation, installed dependency, adopted policy or independent assurance is claimed.
