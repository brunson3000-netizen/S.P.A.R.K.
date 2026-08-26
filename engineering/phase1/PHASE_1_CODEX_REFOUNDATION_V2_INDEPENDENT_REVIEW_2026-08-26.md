### 1. VERDICT

`REVISE_PHASE_1`

### 2. B-02 TRUSTED ACTIVATION

`CLOSED`.

`spark-core` contains authority vocabulary but no activation or state trust machinery. The complete ceremony is owned by `spark-engine::ActivationRegistry::activate`: it validates the manifest and definitions, computes full fingerprints rather than accepting them, validates existing identities before mutation, commits atomically, and advances deterministic registry lineage only on success. Failed activation leaves lineage unchanged. Independent identical registries produce identical manifest, activation, artifact, store, and lineage hashes; changes to authority or manifest content propagate to the activation/store/lineage bindings.

The proof-bearing activated types have private fields and no public constructors. `DefinitionDeclaration`, raw declaration conversion, built-in kind minting, `StateStore::from_activation`, and all state write paths are crate-private or absent. Default-feature external-consumer probes independently confirmed those surfaces cannot be named or called, while a positive external activation control succeeded. The dependency/feature policy and an independent default-feature consumer also confirm that no production dependency enables `test-support`.

### 3. B-03 SCHEDULER CONFLICT

`OPEN`.

The foundational first-arrival defect is repaired: `WorkKey` contains time, profile, producer, scope, occurrence, and kind; identical claims are idempotent; distinct payloads poison the key; neither payload remains executable; two-way and all six three-way permutations have identical state and drain results; duplicates into conflict are idempotent; drain/report/reschedule order is deterministic; and sets exceeding the 256-claim tracking cap converge across permutations. No first-arrival winner or executable conflicted payload remains.

However, `Scheduler::canonical_state_digest` does not commit to the full bounded internal conflict state. `ConflictEvidence` tracks the 256 smallest distinct payload hashes, but canonicalization includes only the 16 externally exposed hashes, `omitted_distinct`, and `evidence_truncated`. An independent probe built two claim sets with the same smallest 16 hashes and different additional tracked hashes, `{200,201}` versus `{202,203}`. Their scheduler digests were equal. Submitting hash `200` to both was a duplicate in the first state and a new claim in the second, producing omitted counts 2 versus 3 and divergent next digests. Thus equal canonical digests conceal states that react differently to the same next input, and the required property that different competing claim sets produce different state digests is false.

The tracking-cap refinement itself is acceptable: retaining the 256 smallest claims plus a truncation flag is bounded and order-independent, and claims forgotten beyond that cap are behaviorally collapsed. The defect is narrower: the digest hashes the 16-item presentation projection rather than all 256 items of bounded behavior-relevant state.

### 4. M-03 PANIC-FREE CANONICAL API

`CLOSED`.

Invalid runtime `CanonicalTag::try_from_static` returns an error; invalid `canonical_tag!` literals fail during const evaluation. `FixedRange` rejects reversed bounds, inherent method-call clamp is total/fallible as appropriate, checked fixed add/sub report overflow, frontier/window and occurrence arithmetic are checked, and canonical IDs, tags, categories, and text are bounded and validated. The explicit strict Clippy gate passed for both canonical crates with `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, and `arithmetic_side_effects` denied.

Fully qualified standard-library `Ord::clamp(value, min, max)` remains capable of panicking on reversed bounds; the independent caught-panic probe confirmed it. This is an acceptable documented boundary under the frozen architecture: it is a standard trait operation available to any `Ord` type, not a project-defined canonical constructor or operation, while removing `Ord` would damage the ordered deterministic collections contract. The project-owned method-call surface shadows it with safe behavior and the canonical crates contain no internal use of the panic path.

### 5. CLOSED-ITEM REGRESSION

| Item | Result | Evidence |
|---|---|---|
| B-01 | `PASS` | Semantic/admission separation, structured acknowledgement/finality, epoch handoff, and no-winner poison behavior remain intact. |
| B-04 | `PASS` | Full immutable fingerprint coverage and atomic activation validation were preserved in `spark-engine`. |
| M-01 | `PASS` | `ConfigRevision` retains unique-key validation, deterministic ordering/hashing, and deterministic multi-duplicate diagnostics. |
| M-02 | `FAIL` | Scheduler conflict state—and structurally the analogous timeline poison state—hashes only the exposed 16-claim projection, not all internally tracked behavior-relevant claims. |
| M-04 | `PASS` | Random addresses remain qualified by profile and behavior artifact/epoch inputs. |
| m-01 | `PASS` | Dependency direction is `spark-engine -> spark-core`, testkit depends downward on both, and production feature hygiene is enforced. |

### 6. FABLE CONFORMANCE

| Fable section | Classification | Ruling |
|---|---|---|
| §3 Trusted activation architecture | `IMPLEMENTED AS SPECIFIED` | One engine-owned trust door, non-public proof mints, computed fingerprints, atomic commit, lineage and artifact binding. |
| §4 Scheduler conflict architecture | `REGRESSION / CONFLICT` | Poison/no-winner semantics and the bounded tracking refinement are sound, but the canonical digest omits internally tracked claim identities and contradicts the complete-state commitment. |
| §5 Panic-free canonical API policy | `IMPLEMENTED WITH ACCEPTABLE REFINEMENT` | Project-owned APIs and strict lints satisfy the policy; fully qualified `Ord::clamp` is a documented standard-library boundary. |
| §6 Reconstruction/resume policy | `IMPLEMENTED AS SPECIFIED` | Nonzero-frontier construction is test/test-support-only; no production continuation or persistence constructor exists. |
| §7 Closed-item preservation | `REGRESSION / CONFLICT` | All listed items except M-02 are preserved; conflict/poison digest projection is incomplete. |
| §8 Module/crate map | `IMPLEMENTED AS SPECIFIED` | Canonical kernel, engine trust boundary, and testkit responsibilities match the prescribed graph. |
| §9 Public API contract | `IMPLEMENTED AS SPECIFIED` | Intended consumer surfaces are public and dangerous constructors, write paths, reconstruction hooks, and raw mints are absent from the default surface. |
| §10 Invariant/test matrix and AT-A–AT-F | `REGRESSION / CONFLICT` | The corpus is extensive, but its different-evidence tests do not distinguish claim sets sharing the exposed projection, allowing the §4/M-02 regression through. |
| §11 Reuse/replace map | `IMPLEMENTED AS SPECIFIED` | Activation/state relocation, scheduler/timeline changes, API restrictions, and deferred later-phase work follow the map. |
| §12 Implementation sequence | `IMPLEMENTED AS SPECIFIED` | Test-first baseline, crate reshape, activation, scheduler, canonical API, reconstruction, closed-item pass, and validation occurred in the prescribed order. |

Opus's 16-item exposed-evidence cap, 256-item smallest-claim tracking cap, saturation flag, and lower-bound semantics beyond the tracking cap are an acceptable order-independent bounded-memory refinement. Approval of that refinement does not approve omitting the behavior-relevant 17th–256th tracked identities from the canonical state digest.

### 7. NEW FINDINGS

- **MAJOR — incomplete canonical commitment of capped conflict/poison evidence.** Distinct scheduler states can have equal canonical state digests and then diverge under the same next claim because the digest excludes tracked hashes 17–256. `SlotPoisonEvidence::canonicalize` uses the same exposed-set/count/flag projection, so the M-02 defect is structurally duplicated in timeline poisoned state. This does not restore arrival-order authority or allow any conflicted payload to execute, so it is not equivalent in severity to the original foundational B-03 first-arrival-wins defect. Phase 1 nevertheless cannot close with a false canonical-state commitment.

### 8. TEST / TOOL RESULTS

- Branch/HEAD: `phase1-refoundation-v2` at `81c0f51419e7111d88df70aa5c0a30552b093c0f`; implementation commits `562ff20`, `33ad640`, and `d25856e` inspected.
- `cargo fmt --check`: `PASS`.
- `cargo clippy --workspace --all-targets -- -D warnings`: `PASS`.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: `PASS`.
- Explicit canonical lint gate for `spark-core` and `spark-engine`, all targets/features, with warnings plus `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, and `arithmetic_side_effects` denied: `PASS`.
- `cargo test --workspace`: `PASS`, 213/213 tests (190 unit/integration/harness tests and 23 doctests). The external harness additionally performed one positive build and 22 expected-failure consumer builds.
- `cargo metadata --format-version 1`: `PASS`; graph and feature declarations inspected.
- Installed static targets: `cargo check --workspace` and `cargo check --workspace --all-targets` both passed for `x86_64-pc-windows-gnu`, `x86_64-pc-windows-msvc`, `aarch64-linux-android`, `x86_64-linux-android`, and `armv7-linux-androideabi`. These are static compile checks only; linking, execution, and cross-platform digest parity remain later executable-portability debt.
- Independent external runtime/compile probe: activation boundary and panic-free project APIs passed; forbidden constructors/writes/resume surfaces failed to compile; invalid tag literal failed const evaluation; scheduler permutations above the tracking cap converged. The focused hidden-evidence probe reproduced the canonical digest collision and subsequent same-input divergence.
- Scope audit: no Phase-2 evaluator, effects/rules runtime, persistence, service/network transport, actor behavior, MCI/game adapter, dialogue/voice, broad catalog, scripting/plugin runtime, or runtime LLM was added.
- Final tree status: clean after committing only this review artifact; temporary probes were removed and the Downloads copy is outside the repository.

### 9. TEST-QUALITY RESULT

The test-first baseline is genuine. Replaying the v2 adversarial file against extracted pre-implementation commit `244391d` produced the recorded red state: 12 tests, 11 failures and only the already-correct poison-order test passing. The current suite has a working external compile-probe harness with a positive control, 16 compile-fail doctests, strengthened replacements for earlier insufficient tests, the original Phase-1 corpus 1–20, and AT-A through AT-F coverage.

Quality is high but not closure-sufficient. Existing two-set discrimination uses small evidence sets, while cap tests compare permutations of one set. No test compares different bounded tracked sets that share the same 16-item exposed projection and count. The independent probe filled that gap and falsified the claimed digest invariant despite 213/213 passing.

### 10. CONVERGENCE RESULT

`CONVERGED`

### 11. PHASE-1 CLOSURE

`NO`

### 12. PHASE-2 AUTHORIZATION

`NO`

The foundational trusted-activation, no-first-arrival-winner, and project-owned panic-free API defects converged, so another architecture/process escalation is not warranted. Phase 1 still requires a bounded correction that commits the full internally tracked conflict/poison state to canonical digests and adds adversarial regression coverage before closure or Phase 2 authorization.
