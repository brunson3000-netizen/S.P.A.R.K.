# PAPERCLIP — Approval transitions and operational budget stops

Status: SCOPED CODE STUDY COMPLETE; runtime qualification not executed.
Study date: 2026-09-14. Exact pin: `13368c518303e886a5c9445fbc69afcbdf0a9228`.
Upstream: https://github.com/paperclipai/paperclip
Language/license at inspected root: TypeScript control plane / MIT. Individual dependency/reuse terms still need qualification.
Finding class: PATTERN / TEST-INVARIANT unless the disposition says idea/reference.
Primary disposition: BORROW_PATTERN.

## Finding

Approval resolution uses a conditional database update from resolvable states; a same-target retry returns applied=false. The route asserts board access and checks access to the approval before calling the service. Hire activation happens after resolution and only when applied=true.

## Traced paths

1. POST approve → assertBoard / requireApprovalAccess → resolveApproval conditional update → hire activation/reconciliation → route activity log and wakeup. The inspected sequence separates approval persistence from effects: crash after the status update but before effects is a reconciliation case, not proven atomic completion.
2. Cost event → sum recorded costEvents by scope/window → threshold incident → pause scope → cancelWorkForScope hook. heartbeat.ts supplies cancelBudgetScopeWork; it cancels active scoped runs and pending wakeups. getInvocationBlock is checked before wake admission and execution. This is real wiring, not merely a dashboard pause.

## Evidence

- [server/src/routes/approvals.ts](https://github.com/paperclipai/paperclip/blob/13368c518303e886a5c9445fbc69afcbdf0a9228/server/src/routes/approvals.ts)
- [server/src/services/approvals.ts](https://github.com/paperclipai/paperclip/blob/13368c518303e886a5c9445fbc69afcbdf0a9228/server/src/services/approvals.ts)
- [server/src/services/budgets.ts](https://github.com/paperclipai/paperclip/blob/13368c518303e886a5c9445fbc69afcbdf0a9228/server/src/services/budgets.ts)
- [server/src/services/heartbeat.ts](https://github.com/paperclipai/paperclip/blob/13368c518303e886a5c9445fbc69afcbdf0a9228/server/src/services/heartbeat.ts)
- [server/src/__tests__/approvals-service.test.ts](https://github.com/paperclipai/paperclip/blob/13368c518303e886a5c9445fbc69afcbdf0a9228/server/src/__tests__/approvals-service.test.ts)
- [server/src/__tests__/budgets-service.test.ts](https://github.com/paperclipai/paperclip/blob/13368c518303e886a5c9445fbc69afcbdf0a9228/server/src/__tests__/budgets-service.test.ts)

Approval tests cover retry no-ops and newly applied side effects. Budget tests cover hard stop, unpaused-but-over-budget admission, scoped cancel hook and embedded-Postgres cases. Assertions and relevant service code inspected; tests not run.

## MCI assessment

Borrow conditional transition/idempotency and explicit pause/admission/cancel wiring. Do not label observed-spend accounting a strict prospective mission cap: these traced paths do not establish atomic worst-case reservations before parallel calls. Borrow a mechanism, not the company/board hierarchy or full database-backed product.

## Limits and follow-up

Physical stop completion and remote provider cancellation remain adapter-specific. Crash recovery between approval and effect, approval payload/version freshness and prospective budget reservation need qualification. heartbeat.ts required blob retrieval because its size exceeds the normal contents response; an empty contents body was not treated as absence of code.

These conclusions are research recommendations. No copied implementation, installed dependency, adopted policy or independent assurance is claimed.
