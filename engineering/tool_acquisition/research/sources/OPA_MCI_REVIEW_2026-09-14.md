# OPA — Comparison evaluator with explicit error mode

Status: SCOPED CODE STUDY COMPLETE; runtime qualification not executed.
Study date: 2026-09-14. Exact pin: `961849b565d2457b80c168f254325b7d5b91461e`.
Upstream: https://github.com/open-policy-agent/opa
Language/license at inspected root: Go / Apache-2.0. Individual dependency/reuse terms still need qualification.
Finding class: PATTERN / TEST-INVARIANT unless the disposition says idea/reference.
Primary disposition: ARCHIVE_REFERENCE.

## Finding

OPA's SDK Decision executes evaluation within its transaction path and returns a generic result plus provenance. evaluate records bundle revisions, prepares/caches the query, passes request input and evaluation options, returns errors, and treats an empty result set as undefined. A non-empty value is not necessarily a boolean authorization.

## Traced paths

1. OPA.Decision → executeTransaction → evaluate → prepared Rego query Eval → generic result/provenance or error. The host must reject absent/wrong-type results and own the meaning of an affirmative result.
2. DecisionOptions.StrictBuiltinErrors → rego.StrictBuiltinErrors → evaluator. TestStrictBuiltinErrors expects division-by-zero to fail; TestBuiltinErrorList expects no top-level error while collecting a builtin error separately. The selected setting changes evidence semantics.

## Evidence

- [v1/sdk/opa.go](https://github.com/open-policy-agent/opa/blob/961849b565d2457b80c168f254325b7d5b91461e/v1/sdk/opa.go)
- [v1/sdk/options.go](https://github.com/open-policy-agent/opa/blob/961849b565d2457b80c168f254325b7d5b91461e/v1/sdk/options.go)
- [v1/rego/rego.go](https://github.com/open-policy-agent/opa/blob/961849b565d2457b80c168f254325b7d5b91461e/v1/rego/rego.go)
- [v1/rego/rego_test.go](https://github.com/open-policy-agent/opa/blob/961849b565d2457b80c168f254325b7d5b91461e/v1/rego/rego_test.go)

TestStrictBuiltinErrors, TestBuiltinErrorList and cancellation test code inspected; none executed.

## MCI assessment

Retain as a policy-semantics and versioned-bundle comparison against Cedar. Go service/library or a separately qualified Wasm route adds a boundary to the Rust core. Do not introduce that runtime just to obtain boolean decisions.

## Limits and follow-up

Query-cache behavior across per-call strictness changes, builtin external effects, bundle trust and revocation need further tests before reuse. No language engine is accepted as current MCI authority.

These conclusions are research recommendations. No copied implementation, installed dependency, adopted policy or independent assurance is claimed.
