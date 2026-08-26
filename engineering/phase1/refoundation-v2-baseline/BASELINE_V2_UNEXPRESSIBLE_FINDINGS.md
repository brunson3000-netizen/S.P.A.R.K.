# Re-Foundation v2 Step 1 — findings not expressible against the inherited API

**Recorded:** 2026-08-26
**Branch:** `phase1-refoundation-v2`
**Inherited HEAD:** `244391d41aa2c6f6f32af1f7529b22e934493a81`

Twelve of the mandatory v2 acceptance assertions were expressible as
runtime tests against the inherited tree from an external crate; eleven of
them fail and one (poison evidence order-independence) already holds and
is recorded as a property that must not regress. See
`BASELINE_V2_ADVERSARIAL_FAILURES.txt` for the raw output.

The requirements below could **not** be expressed as a runtime assertion
against the inherited API, because the API surface they require does not
exist at all. Their baseline is therefore "does not compile: required API
absent". Per the v2 brief §10 this is recorded rather than faked with an
unrelated malformed input, and each is encoded as a permanent test against
the re-founded API.

| Acceptance requirement | Inherited API | Baseline |
| --- | --- | --- |
| **AT-A2** `StateStore` is unconstructible without an activation artifact | `StateStore::from_activated_schema(ActivatedSchema)` is public, and `ActivatedSchema` is mintable through the public `DefinitionIdentityRegistry::activate` | does not compile as a *forbidden-surface* test: the public constructor exists and succeeds. Encoded post-implementation as external `compile_fail` probes. |
| **AT-A4** the store carries `manifest_content_hash` as well as `activation_hash` | `StateStore` has `activation_hash()` only; the manifest hash stops at `spark_profile::validate::ValidatedManifest` and is never bound into the store | does not compile: no `StateStore::manifest_content_hash` |
| **AT-A5** independent registries are byte-identical for identical manifests and visibly divergent otherwise, *observable on every downstream artifact* | partially expressible (activation hashes differ), but the manifest-content binding needed for the full claim is absent from the store | partially expressible only; the artifact-binding half does not compile |
| **AT-A8** registry lineage is append-only and deterministic | no lineage concept exists | does not compile: no `DefinitionIdentityRegistry::lineage_digest` |
| **AT-B1/B2/B4/B5** conflict *evidence* (competing payload hash set, omitted count) and `DrainOutcome::conflicted` | `schedule` returns `Result<ScheduleOutcome, ScheduleConflict>` and `drain_due` returns `Vec<DueWorkItem>`; there is no conflicted slot state, no evidence set, and no conflicted drain channel | the order-independence half is expressible and fails (recorded); the evidence/`DrainOutcome` half does not compile: types absent |
| **AT-C1** total `try_from_static` | only the panic-capable `const fn from_static` exists | does not compile: no `CanonicalTag::try_from_static`. The panic it replaces *is* expressible and is recorded as a runtime failure. |
| **AT-C2** `canonical_tag!` literal macro rejecting invalid literals at compile time | no macro exists | does not compile: no `canonical_tag!` |
| **AT-C3** `FixedRange` validated clamp domain and `checked_clamp` | only the panic-capable `FixedPoint::clamp` exists | does not compile: no `FixedRange`/`checked_clamp`. The panic it replaces *is* expressible and is recorded as a runtime failure. |
| **AT-C5** `FixedPoint::checked_add` / `checked_sub` | absent | does not compile: methods absent |
| **AT-D2** `test-support` is dev/test only | no `test-support` feature exists on any crate | does not compile / not applicable: feature absent |
| **AT-E3** dependency graph includes `spark-engine` and asserts feature hygiene | no `spark-engine` crate exists; the policy test asserts the three-crate v1 graph | not applicable: crate absent |
| **AT-F4** transcript renamed `outcome_transcript_digest` | field is `transcript_digest` | does not compile: field absent |
| **Strict canonical lint gate** (`clippy::unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `arithmetic_side_effects`) | the workspace runs `-D warnings` only; the five panic/arithmetic lints are not enabled anywhere | not a test: recorded as an absent mechanical gate. Enabling it against the inherited tree is itself the falsification, and it fails (see the v2 report §6). |

## Note on "must be impossible" claims

Per the v2 brief §10 and the Fable review §2 item 4, every "X is
impossible" property in the permanent v2 corpus is proved either by a
genuine external compile-failure or by a canonical-state-digest equality —
never by an "an error was returned" check alone, and never by feeding an
unrelated malformed input to a path that would have rejected it anyway.
The two insufficient v1 tests this replaces
(`definition_spec_cannot_self_activate`, which used an invalid spec, and
`payload_conflict_rejects_in_either_order`, which compared only error
shape and length) are restated above as
`baseline_valid_definition_spec_must_not_self_activate` and
`baseline_same_key_conflict_state_must_be_arrival_order_independent`,
both of which fail against the inherited tree.
