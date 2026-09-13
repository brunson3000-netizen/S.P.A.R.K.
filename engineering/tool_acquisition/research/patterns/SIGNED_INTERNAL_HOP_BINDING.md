# Pattern Card — Signed Internal Hop Binding

PATTERN: Signed Internal Hop Binding
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- MCP Gateway & Registry @ `7c353798fa7d4da6ca1b3f3d2c502505bfbb8f73`
- `auth_server/internal_request_token.py`
- `auth_server/server.py`
- registry-side proxied-token verification + dedicated unit tests

## Problem

After an edge component authenticates/authorizes a request, downstream processes still need identity, scope and destination information. Plain forwarding headers are forgeable and can drift from the decision that admitted the call.

## Mechanism

After validation, mint a short-lived signed assertion for exactly one internal audience/hop. Bind validated facts such as:
- subject
- validated scopes
- intended server/entity
- resolved upstream destination
- normalized method
- token use/audience
- issued/expiry time

The downstream hop verifies signature, issuer, audience, time and required claims, then treats signed claims—not caller headers—as authoritative.

Different internal uses receive distinct audiences/tokens to prevent replay across hops.

## Dependencies

- trusted signer/verifier key management
- stable hop identity/audience
- bounded TTL/leeway
- server-derived target/destination
- strict claim validation

## Benefits

- closes header-spoof boundary
- binds authorization decision to routed destination
- narrows replay window
- reduces repeated parsing of external credentials downstream
- supports process separation without turning the model into the trust carrier

## Risks / failure modes

- shared signing secret broadens blast radius
- excessive TTL creates replay window
- incomplete target claims permit sibling-resource replay
- audience reuse collapses trust boundaries
- downstream accepting unsigned headers alongside claims defeats the design

## Boundaries crossed

External caller → edge auth: external credential.
Edge auth → internal proxy: signed bounded assertion.
Internal proxy → upstream: route based on verified claim.

## Determinization relevance

VERY HIGH. This is infrastructure/security machinery and must be wholly outside model judgment.

## Likely architectural location

MCI/tool broker process boundary; sandbox/adapter handoff; federated internal service hops where applicable.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Required invariants

1. User-provided headers never override signed claims.
2. Each hop/use has a distinct audience/type.
3. Token binds exact resource/adapter and, where needed, operation/method.
4. Extra claims cannot overwrite reserved issuer/audience/subject/expiry claims.
5. TTL is short and bounded.
6. Missing/invalid token fails closed.
