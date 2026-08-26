# S.P.A.R.K. Phase 1 — Bounded Correction Brief v0.1

**Date:** 2026-08-26  
**Status:** AUTHORITATIVE CORRECTION MISSION / Phase 2 remains unauthorized  
**Basis:** `PHASE_1_CODEX_INDEPENDENT_REVIEW_2026-08-26.md`  
**Writer:** Claude Code  
**Independent re-review:** Codex after correction

## 1. Correction objective

Correct the Phase-1 Rust skeleton so it actually embodies the already-frozen Phase-0 contracts.

This is **not** a redesign of the S.P.A.R.K. causal grammar.

Do not add a new causal runtime primitive.

Do not begin Phase 2.

The independent review found:

```text
BLOCKERS: B-01 through B-04
MAJORS: M-01 through M-04
MINOR: m-01
PHASE-2 AUTHORIZATION: NO
```

All findings in this brief must be corrected or explicitly demonstrated false by executable evidence.

## 2. B-01 — Full canonical envelope identity and finality

### Defect to close

Timeline staging/finalization currently collapses behaviorally distinct envelopes that share ordinal, command ID, and payload hash.

### Required correction

Define one unambiguous canonical semantic envelope identity that includes every behaviorally meaningful field:

```text
profile_id
timeline_epoch
effective_time
source_id
source_sequence
input_ordinal
command_id
command_kind
canonical_payload_hash
```

The admission-window token is admission metadata and must not silently become behavior identity merely because a retry uses a newer token.

Requirements:

1. staging duplicate/collision logic compares the full semantic-envelope identity;
2. finalized records retain enough of the full envelope to reconstruct/hash canonical history;
3. ordered-stream digest commits to the full semantic envelope, not only ordinal/command/payload;
4. command IDs are unique within the appropriate profile/timeline domain:
   - exact same command identity may be idempotent;
   - same command ID with different semantic identity is conflict/rejection;
5. `(source_id, source_sequence)` is tracked deterministically:
   - same pair cannot identify two different commands;
   - canonical finalization must enforce strictly increasing source sequence within each source's finalized command order;
   - gaps may remain legal unless an accepted ADR requires contiguity;
6. envelope `profile_id` / `timeline_epoch` must match the active timeline;
7. sequencer authority is separate from `source_id`; do not incorrectly require every source to equal the sequencer because the sequencer may order multiple upstream logical sources;
8. stage/fence results must carry structured identities sufficient to prove what was staged/finalized;
9. `reset_epoch`/handoff must not be a public unauthenticated arbitrary mutation:
   - require current sequencer authority;
   - require a valid epoch transition;
   - preserve unchanged finalized frontier/history;
   - discard only old-epoch unfinalized staging;
   - record the reset/handoff deterministically;
   - reject old-epoch authority afterward.

### Required falsification tests

- same ordinal/command/payload but changed effective time -> not idempotent;
- changed source ID -> not idempotent;
- changed source sequence -> not idempotent;
- changed command kind -> not idempotent;
- same command ID reused for a different full semantic envelope -> reject/conflict;
- same source+sequence reused for different command -> reject/conflict;
- source sequence finalizes in descending order for the same source -> reject;
- different sources may legitimately interleave;
- unauthorized epoch reset/handoff rejects;
- arbitrary epoch rollback/reuse rejects;
- finalized full-history digest differs for behaviorally distinct envelopes.

## 3. B-02 — Structural authority/write-class and state-schema safety

### Defect to close

Raw state APIs allow an external caller to declare an arbitrary authority and then call host/evaluator write paths.

### Required correction

Phase 1 must expose **no public raw state mutation API** that lets an arbitrary external crate:

- declare runtime authority on demand;
- directly invoke host-ingress writes;
- directly invoke derived/evaluator commits;
- write a value that violates the validated definition's type/bounds/scope/profile.

Use a structurally constrained design.

A good acceptable shape is:

```text
validated immutable profile/state schema
-> runtime state store built against that schema
-> narrow internal/core mutation paths
-> public read-only state inspection
-> later higher-level host/evaluator facades may mediate writes
```

Exact Rust types are an engineering choice, but the following are mandatory:

1. `StateStore` cannot publicly declare or alter definition authority;
2. runtime schema is immutable after store/runtime activation;
3. schema key is profile-qualified:
   ```text
   (ProfileId, DefinitionId)
   ```
4. schema entry includes at minimum:
   - definition fingerprint;
   - authority/write class;
   - value type;
   - valid scopes;
   - declared value bounds/constraints;
5. state key is profile-qualified;
6. host-owned/evaluator/spark-owned write operations are not arbitrary public external methods;
7. writes validate profile, definition fingerprint/schema, authority, type, bounds, and allowed scope;
8. undeclared definitions reject;
9. profile A cannot write/read private state as profile B through ID collision;
10. tests may access internal paths from inside the crate; do not make unsafe runtime APIs public merely for test convenience.

### Required falsification tests

- external/public API cannot fabricate a derived definition then write it;
- undeclared ID rejects;
- wrong value type rejects;
- out-of-bounds value rejects;
- disallowed scope rejects;
- wrong profile rejects;
- host-owned through spark effect path rejects;
- derived through spark/client path rejects;
- state registry cannot be mutated after activation.

## 4. B-03 — Scheduler semantic ordering and occurrence identity

### Defect to close

Scheduler assigns occurrence identity from method call order.

### Required correction

The scheduler may not manufacture canonical occurrence identity from insertion order.

Use an explicit semantic work key supplied by the producer.

Minimum conceptual key:

```text
due_time
profile_id
producer_definition_id / rule_or_trigger_id
scope_id
occurrence_index
work_kind or equivalent domain-separated discriminator
```

Requirements:

1. `occurrence_index` is an explicit semantic/persistable input, not an auto-increment assigned by `schedule()`;
2. stable total ordering derives from semantic fields;
3. scheduling the same logical set in reversed insertion order drains identically;
4. duplicate semantic work key:
   - exact duplicate is idempotent or deterministically rejected;
   - differing payload under same key is conflict;
5. arithmetic increments/derivations are checked;
6. later recurrence logic can persist and resume occurrence indexes without relying on worker/batch count.

### Required tests

- equal-time work inserted A/B versus B/A drains identically;
- different semantic occurrence indexes order identically regardless insertion;
- conflicting duplicate work key rejects;
- occurrence overflow rejects;
- no global call-order counter remains in canonical order identity.

## 5. B-04 — Immutable full definition identity + atomic validation

### Defect to close

Definition fingerprint is computed but validation enforces only authority, mutates live identity state before validation succeeds, and has canonical encoding collisions.

### Required correction

1. immutable identity comparison uses the **full definition fingerprint**;
2. definition registry is profile-qualified:
   ```text
   (ProfileId, DefinitionId)
   ```
3. changing any immutable identity field under the same persisted ID rejects, including:
   - definition kind;
   - value type;
   - authority/write class;
   - immutable scope compatibility;
   - any other field included by the accepted fingerprint contract;
4. profile validation is pure or candidate-staged:
   - failure must leave active registry unchanged;
   - activation/registry update occurs only after complete validation succeeds;
5. canonical enum encoding is domain-separated:
   - built-in `Trigger` cannot collide with `Custom("trigger")`;
   - same rule applies to other built-in/custom kinds;
6. custom scope kinds retain/encode their actual validated custom identifier rather than collapsing all custom scopes to one tag;
7. profile partition prevents same definition ID in two profiles from contaminating each other's identity history.

### Required tests

- same ID/authority but changed ValueType rejects;
- changed valid scopes rejects where fingerprint declares scope immutable;
- failed multi-definition validation leaves live registry byte-for-byte/logically unchanged;
- corrected retry after failed validation succeeds;
- built-in Trigger fingerprint != Custom("trigger");
- profile A same DefinitionId does not conflict with independent profile B definition.

## 6. M-01 — ConfigRevision / config_revision_hash

Implement the Phase-1 requirement that was omitted.

Add a minimal deterministic logical configuration revision representation.

It does not need Phase-3 mutation APIs.

It must support:

```text
profile_id
config entries / keys / canonical values
content-addressed config_revision_hash
```

Requirements:

- order-independent canonical hashing where logical order is irrelevant;
- profile-qualified identity;
- human/display revision label is not sufficient identity;
- changed canonical config content changes hash;
- equivalent content with different insertion order hashes identically;
- use the same canonical validation/bounds discipline as other canonical values.

## 7. M-02 — Replay/state digest must prove the property

### Required correction

Do not call an incomplete projection a canonical state digest.

Either:

A. strengthen the existing digest to cover all deterministic ingress state relevant to replay, or  
B. split the concept explicitly, e.g.:

```text
canonical_history_digest
ingress_runtime_state_digest
scenario_transcript_digest
```

At minimum review evidence must distinguish:

- active profile/timeline epoch;
- sequencer/finality frontier;
- admission window width/state;
- staged/unpoisoned/poisoned slot identities;
- full finalized semantic envelopes;
- fence chain/body identities needed for deterministic continuation;
- reset/handoff records;
- operation results/errors in scenario replay where those are part of replay evidence.

`run_scenario` must not silently discard operation results.

### Required tests

- empty scenario != scenario with one successfully staged unfinalized command;
- reversed-delivery equivalent logical scenario converges to identical appropriate digest(s);
- behaviorally different finalized full envelope changes history digest;
- poisoned staging differs from clean staging;
- scenario outcome/error transcript changes when an operation result differs;
- 50+ repeated runs remain bit-identical.

## 8. M-03 — Canonical construction, bounds, overflow, IDs/scopes

Correct malformed/collision-prone canonical value entry.

### Stable IDs

- enforce a real namespaced ID contract for identifiers that require namespacing;
- maximum length must be bounded consistently with Phase-0 budget;
- do not encode directed relationship scope as a single string such as `a->b`; use structured subject/target fields.

### Scope kinds

- custom scope kind must carry and canonically encode its validated custom identifier;
- two different custom kinds must not hash/equal as the same kind.

### Fixed point / integers

- raw internal integer field should not be publicly forgeable if invariants matter;
- constructors and arithmetic use checked operations;
- overflow returns typed error; it must not debug-panic/release-wrap;
- timeline frontier/window arithmetic also uses checked operations;
- scheduler occurrence arithmetic uses checked operations.

### Definition value constraints

Add explicit declarative bounds/constraints sufficient for StateStore/schema write validation.

At minimum support appropriate constraints for:

- bool;
- integer;
- fixed-point;
- categorical/text length;
- references/sets as Phase-1 types require.

Strings/custom identifiers/categories are bounded and validated before they become canonical/hashable.

## 9. M-04 — Random-address profile/artifact context

Random address must include profile-qualified behavior context.

At minimum include:

```text
root_seed
profile_id
behavior_epoch
behavior_artifact/content identity
rule_or_trigger_id
scope
occurrence_index
```

Use exact accepted behavior-artifact context available in Phase 1, e.g. a deterministic digest representing the active manifest/config behavior context.

Tests:

- same private IDs/epoch in different profiles -> different random result;
- same profile but different behavior-artifact hash -> different random result;
- same exact semantic address -> identical result;
- call order remains irrelevant.

## 10. m-01 — Dependency-direction enforcement

- remove the unused `spark-core -> spark-testkit` dev dependency unless there is a demonstrated reason to keep it;
- make the automated dependency check inspect the dependency graph that the policy actually cares about, including relevant normal/dev/build/target-specific edges;
- a test-only tool may invoke/parse `cargo metadata` or use an equivalent robust mechanism;
- normal product dependency direction must remain unchanged.

## 11. Regression requirements

All original 45 tests must either remain valid and pass or be replaced by stricter tests that prove the same intended invariant.

Do not delete a failing architectural test simply because the corrected API makes it inconvenient.

Add the adversarial tests needed by this brief.

## 12. Commands required before writer completion

Run:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo metadata --format-version 1
```

Rerun installed Windows/Android target `cargo check` commands without asking the operator if the targets are already present.

## 13. Scope boundary

Still out of scope:

- Phase-2 propagation/effect runtime;
- service transport;
- persistence backend;
- actor behavior;
- MCI/game adapters;
- dialogue/voice;
- broad trigger/factor catalogs;
- scripting/plugin systems;
- runtime LLM.

Do not use the correction as an excuse to begin these.

## 14. Convergence rule

This is one bounded correction pass.

If you conclude that a blocker cannot be fixed without changing a frozen Phase-0 architectural contract or adding a new causal primitive, stop and report the exact conflict rather than silently redesigning.

## 15. Writer completion artifact

Create:

```text
engineering/phase1/PHASE_1_CLAUDE_CORRECTION_REPORT_2026-08-26.md
```

Include:

- exact commits;
- finding-by-finding disposition B-01..B-04, M-01..M-04, m-01;
- code/API changes;
- adversarial tests added;
- total test count/results;
- determinism/authority/finality evidence;
- dependency changes/licenses;
- Windows/Android static checks;
- any deviation/blocker;
- explicit `PHASE_2_AUTHORIZATION: NO`.

Copy the identical report to:

```text
~/Downloads/PHASE_1_CLAUDE_CORRECTION_REPORT_2026-08-26.md
```

Repository copy is canonical evidence.
