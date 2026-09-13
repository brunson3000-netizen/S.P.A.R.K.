# ToolHive Failure Lessons

Source: `stacklok/toolhive` @ `e532cf07d45fa99f3e4e63819396a3e9c9fd763f`
Status: ACTIVE EVIDENCE

## TH-F01 — Tool visibility filtering can diverge from the executed call

FAILURE MODE: A control that filtered the original `tools/call` can cease to describe the operation after a later middleware mutates the request.

OBSERVED IN: ToolHive middleware architecture. Tool-call filtering executes before mutating webhooks. The official architecture document explicitly records that a mutating webhook can rename an allowed call into a tool excluded by `--tools` filtering.

CAUSE: presentation/routing filter and later request mutation act at different points in the chain.

PROJECT RESPONSE: documented known gap; independent authorization occurs later when configured and can still evaluate the mutated request.

DID IT WORK?: the filter itself is not sufficient authority. A later authorizer can contain the risk if correctly configured.

BOUNDARY AFFECTED: tool visibility/routing ↔ actual executed capability.

SPARK LESSON: tool search/list filtering is context/presentation policy only. Call-time authority must evaluate the **final normalized action** immediately before execution, after any mutation/translation.

TRANSFER CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN from the stronger authorization layer; REJECT filter-as-authority.

EVIDENCE: `sources/TOOLHIVE.md`; upstream `docs/middleware.md`, `pkg/mcp/tool_filter.go`, `pkg/authz/middleware.go`.

## TH-F02 — Tool-call filter layer fails open on body-read failure

FAILURE MODE: `NewToolCallMappingMiddleware` cannot inspect the request because `io.ReadAll(r.Body)` errors, so it forwards to the next handler.

CAUSE: the filtering middleware chooses compatibility/pass-through rather than hard refusal when it cannot inspect the body.

PROJECT RESPONSE: other layers can independently impose body limits, parsing and Cedar authorization; protected authz requests have stronger malformed/non-JSON refusal semantics.

DID IT WORK?: as a presentation/filtering component this can be acceptable; as a hard security boundary it is fail-open.

BOUNDARY AFFECTED: uninspectable request ↔ tool filter.

SPARK LESSON: any component designated as an authority/admission gate must deny when it cannot determine the operation it is authorizing. Convenience shapers may fail open only if a later mandatory authority gate exists and is proven to receive the request.

TRANSFER CONFIDENCE: HIGH for the pinned code path; exploitability depends on the complete configured chain and transport behavior.

DISPOSITION: ARCHIVE_REFERENCE / negative acceptance criterion.

EVIDENCE: upstream `pkg/mcp/tool_filter.go`; contrast `pkg/authz/middleware.go` fail-closed behavior.

## TH-F03 — Executable image provenance defaults to warning

FAILURE MODE: signature/provenance verification can detect an image problem but continue launching because the default image-verification mode is `warn`.

OBSERVED IN: `WorkloadService` initializes `imageVerification` to `VerifyImageWarn`; CLI run/upgrade flags also default to warn. `VerifyImageEnabled` exists for hard failure.

CAUSE: product usability default differs from high-assurance activation policy.

PROJECT RESPONSE: configurable verification mode lets operators select strict enforcement.

DID IT WORK?: reasonable product tradeoff, but a warning cannot count as passing provenance evidence for SPARK activation.

BOUNDARY AFFECTED: external executable source ↔ workload activation.

SPARK LESSON: acquisition can tolerate unverified/quarantined material for research; **activation must fail closed** when required provenance evidence is missing or invalid. Warn-only is not an accepted activation state.

TRANSFER CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN for verifier mechanics; reject warn-default for canonical activation.

EVIDENCE: upstream `pkg/api/v1/workload_service.go`, `pkg/runner/retriever/retriever.go`, CLI run/upgrade flags.

## TH-F04 — First-use signer pin can still be TOFU

FAILURE MODE: on true first use with neither existing lock state nor catalog-declared provenance, successful keyless verification records the signer identity observed from the artifact ecosystem itself.

CAUSE: there is no prior expected identity to compare against.

PROJECT RESPONSE: ToolHive supports catalog expectations and explicit external public keys; once trust is recorded, later drift is rejected.

DID IT WORK?: strong continuity after first use, but TOFU cannot prove the first signer was the intended signer.

BOUNDARY AFFECTED: first acquisition ↔ durable provenance trust.

SPARK LESSON: distinguish `VERIFIED_CHAIN` from `EXPECTED_IDENTITY_MATCH`. High-authority source activation should prefer a pre-existing catalog/operator/project expectation rather than silently promoting first observed identity to canonical trust.

TRANSFER CONFIDENCE: HIGH.

DISPOSITION: BORROW_PATTERN with stronger first-use policy.
