# S.P.A.R.K.–G.A.M.E. contract prototype (contained, reversible)

**Status: working/non-production material.** `SPARK_GAME_CONVERGENCE_PROTOCOL_V1.md` §6
permits integration-contract requirements and test fixtures as working material before
Phase 3 is opened. This crate is exactly that and nothing more. It is **not** the
S.P.A.R.K. device, **not** a G.A.M.E. adapter, and **not** production code.

It exists so the candidate contract
`../SPARK_GAME_INTEGRATION_CONTRACT_V1_2026-09-12.md` can be reviewed against behavior
instead of prose.

## What it does and does not prove

**Does:** that the S.P.A.R.K. public surface at pinned crate tree
`7907f4d729104fd5dbfd4adad46e66cf09aa13dd` supports every logical surface Gate C3
enumerates, and that the protocol's canonical first causal sequence runs end to end across
that surface with deterministic, replayable results.

**Does not:** prove G.A.M.E. integration. `src/fake_host.rs` is a fake. Nothing here was
authored, reviewed or accepted by G.A.M.E. A real adapter, in the G.A.M.E. repository,
against a pinned **accepted** S.P.A.R.K. artifact, is Gate C4 and is not authorized.

## Isolation

- Not a workspace member. The empty `[workspace]` table in `Cargo.toml` detaches it, and
  `cargo metadata` on the repository root still reports exactly the three canonical members
  (`spark-core`, `spark-engine`, `spark-testkit`).
- Depends on `spark-core` and `spark-engine` **without** the `test-support` feature, so it
  exercises the same surface a real consumer would see.
- Changes no file under `crates/`.

## Layout

| Path | Contents |
|---|---|
| `src/lib.rs` | The contract's logical types and the prototype device façade: version/capability negotiation, the entity mapping with its injectivity and stability checks, the intent projection and batch digest, the typed rejection vocabulary, and the host's at-most-once application ledger |
| `src/fixture.rs` | The protocol §5 first-proof causal profile and rules. **Every constant is test data, not a gameplay law.** |
| `src/fake_host.rs` | A fake G.A.M.E. host: it decides legality, executes against its own world numbers, and returns confirmations only as typed observations |
| `tests/end_to_end.rs` | E-1 … E-13: negotiation, the first proof, rejection, duplicate delivery, stale input, `Busy`, message bounds, determinism, replay, restart, the re-presentation limit, intent survival across a pause, and the at-most-once key's activation component |
| `evidence/` | Captured validation output |

## Reproduce

```
cd engineering/phase3/spark_game_contract_prototype_2026-09-12
CARGO_TARGET_DIR=<a scratch dir> CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 \
  cargo test --offline -- --nocapture --test-threads=1
```

## Validation recorded in `evidence/`

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS (`proto-fmt.txt`) |
| `cargo clippy --offline --all-targets -- -D warnings` | PASS (`proto-clippy.txt`) |
| `cargo test --offline` | **14 passed, 0 failed, 0 ignored** (`proto-test.txt`) |
| `cargo check --target x86_64-pc-windows-msvc` | PASS — **COMPILE-ONLY** (`proto-cross-targets.txt`) |
| `cargo check --target x86_64-pc-windows-gnu` | PASS — **COMPILE-ONLY** |
| `cargo check --target aarch64-linux-android` | PASS — **COMPILE-ONLY** |
| `cargo check --target armv7-linux-androideabi` | PASS — **COMPILE-ONLY** |
| `cargo check --target x86_64-linux-android` | PASS — **COMPILE-ONLY** |

Linux is the only platform where anything was **executed**. The five cross-target results
are static compilation only; no Windows or Android runtime, replay or digest-parity
evidence exists, and no three-platform equivalence is claimed.

## Known limits carried by this prototype

- The engine snapshot it round-trips is an **in-memory value**. E-10 is not durable
  process-recovery evidence, and the contract's §9.4 durability gap is real and open.
- E-9's replay depends on the fixture holding its own command payloads, because a finalized
  envelope stores only the payload hash. A real adapter needs an equivalent host-side
  command log; contract §10.1 states this as a replay input.
- `BehavioralLeverage` is declared on manifest input but not republished on the activated
  definition at this tree, so the projection carries `None` rather than inventing a value.
- The outbound `max_intents_per_batch` bound is a **capacity hint**: an oversized batch is
  delivered whole with `capacity_exceeded` set, never truncated. Chunked pull of one
  boundary's results is a possible V2 surface and is not implemented.

## Correction history

Revision 2 (this revision) fixes a real defect found by the author while recovering prior
G.A.M.E. records: the device projected its batch from the completing `ProcessResult` alone,
but cohort reports are **per call**, so every intent committed before a pause in a paced
request was silently dropped. The device now accumulates cohort reports across the active
request's calls and projects once at the completed boundary. `E-12` is the regression test;
`E-6e` additionally proves an interleaved `Busy` refusal does not disturb the accumulation.
The at-most-once key also gained the session's activation identity (`E-13`).
