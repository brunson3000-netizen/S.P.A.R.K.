# S.P.A.R.K. Phase 1 — Re-Foundation Implementation Brief v0.1

## Objective

Re-found the Phase-1 Rust core skeleton around the already-frozen Phase-0 contracts.

This is not a full project rewrite. It is a targeted replacement of the Phase-1 foundation pieces that failed to converge.

## Mandatory first action: preserve evidence

Do not rewrite or delete:

- original Phase-1 implementation commits;
- Claude writer/correction reports;
- Codex independent reviews/re-reviews;
- Phase-0 ADRs and closure evidence.

Create a new Git branch/worktree or an equivalent isolated implementation line for the re-foundation.

Recommended branch:

```text
phase1-refoundation
```

## Test-first failure corpus

Before replacing implementation, add permanent tests that encode every surviving independent-review counterexample.

### Finality/admission tests

1. Semantically identical final history must have the same canonical history/state digest even when a tail command was staged under an old overlapping admission token vs a newer token.
2. Admission token must never be part of semantic finalized identity/history hashing.
3. Structured `StageAcknowledgement` must bind at least:
   - profile ID;
   - timeline epoch;
   - ordinal;
   - command ID;
   - canonical semantic-envelope hash;
   - slot/result state.
4. Structured finalization result must identify the finalized fence/range and canonical result.
5. Distinct semantic envelope fields must remain part of canonical identity/fence history.
6. Epoch handoff remains authenticated/monotonic and preserves finalized history.

### Schema provenance / authority tests

1. External code cannot fabricate a `DefinitionSchema` and activate it into a `StateStore`.
2. `StateStore` cannot be constructed from raw public schema metadata.
3. Activated state schema must derive from a validated profile/identity-registry result or an unforgeable activation artifact/type.
4. Duplicate schema keys reject.
5. Fingerprint/profile/type/scope/bounds are carried from validated activation, not caller-provided raw fields.
6. Post-activation schema mutation is structurally unavailable.

### Scheduler tests

Use a complete semantic `WorkKey` including at least:

```text
due_time
profile_id
producer_definition_id
scope_id
occurrence_index
work_kind
```

Tests:

1. two producers may both schedule occurrence 0 at the same time without collision;
2. reversed insertion of those two items drains identically;
3. same semantic key + same payload is idempotent or deterministically rejected;
4. same semantic key + different payload is conflict;
5. ordering does not depend on call order;
6. occurrence identity remains producer/scope-local and persistence-friendly.

### Config revision tests

1. config construction rejects duplicate keys;
2. malformed/oversized values reject before hashability/activation;
3. insertion order of unique logical entries does not affect hash;
4. same duplicate-key malformed input is rejected regardless input order;
5. profile context changes config hash;
6. human label is not identity.

### Canonical construction/bounds tests

1. `DefinitionId` requires namespace according to the accepted ID contract;
2. incoherent bounds (`min > max`) reject;
3. custom definition-kind strings are bounded and validated;
4. command kind and scheduler work kind use bounded canonical identifier/value types, not unbounded raw `String`;
5. timeline window arithmetic cannot panic/wrap;
6. fence finalization is atomic if frontier advance would overflow;
7. custom scope kinds remain distinct;
8. checked fixed-point/integer construction remains passing.

## Implementation contracts

### 1. Semantic vs admission data

Separate:

```text
SemanticCommandEnvelope
AdmissionEnvelope / AdmissionTicket
```

or an equivalently explicit type boundary.

Canonical history/fence/replay hashes may include semantic command data only, plus actual finalized semantic state. Ephemeral admission tokens/transport timing must not contaminate finalized semantic identity.

### 2. Structured acknowledgements

Implement explicit result records, not unqualified enums alone.

A positive stage acknowledgement must be cryptographically/logically tied to the staged semantic envelope and slot.

Finalization returns structured evidence identifying the fence/range/result.

### 3. Trusted schema activation

Raw `DefinitionSchema` must not be a public authority declaration mechanism.

Prefer a type boundary such as:

```text
ValidatedProfile
ValidatedDefinitionSet
ActivatedSchema
StateStore::from_activated_schema(...)
```

where the constructor of the trusted activation type is inaccessible outside the validation path.

Exact names are flexible; provenance must be structural.

### 4. Complete scheduler work key

No canonical scheduler key based only on `(due_time, occurrence_index)`.

Use the complete semantic work identity so independent producers/scopes may reuse occurrence indexes safely.

### 5. Validated config

Config must have a validation/canonicalization constructor and a unique-key representation (e.g. ordered map after validation), not a freely hashable raw vector with duplicates.

### 6. Bounded canonical strings/IDs

Replace unbounded canonical `String` fields with validated bounded identifiers/types.

## Closed items that must not regress

- B-04 full immutable definition fingerprint + atomic candidate validation;
- M-04 random address includes profile + behavior artifact;
- m-01 dependency direction;
- no unsafe canonical Rust;
- no Phase-2 scope;
- Windows/Android claims remain static checks only until executable gates.

## Required validation

Before writer completion:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo metadata --format-version 1
```

Rerun installed Windows/Android static target checks.

## Completion report

Create:

```text
engineering/phase1/PHASE_1_REFOUNDATION_IMPLEMENTATION_REPORT_2026-08-26.md
```

Copy identical transfer copy to:

```text
~/Downloads/PHASE_1_REFOUNDATION_IMPLEMENTATION_REPORT_2026-08-26.md
```

Phase 2 remains unauthorized until independent review.
