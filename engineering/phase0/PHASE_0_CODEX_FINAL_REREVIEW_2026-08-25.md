### 1. VERDICT

`REVISE_PHASE_0`

The v0.3 sequencer and fence contract closes ordinal ownership, contiguous finality, collision poisoning, and finalized-history protection, but its bounded staging-overflow rule still permits arrival order to determine whether the canonical frontier can finalize.

### 2. B-01A CLOSURE

`OPEN`

ADR-0003 correctly makes staging causally inert and finalizes only a sequencer-authored, digest-matching, contiguous, hash-linked ordinal range. Within available staging capacity, this defeats the required attacks: `(n+1,n)` and `(n,n+1)` converge; opposite-order same-ordinal payload conflicts poison the slot; gaps and skipped ordinals reject; non-sequencer commands and fences reject; digest and previous-fence mismatches reject atomically; exact late duplicates are idempotent; late conflicts reject; old-epoch authority rejects after handoff; and service/embedded paths share the same logical ingress contract. Concurrent workers delivering the same in-capacity set cannot select canonical order because promotion follows the fenced ordinals.

The bounded-overflow attack still succeeds. Suppose the next unfinalized ordinal is `n` and the staging capacity is two entries:

- delivery `n+1, n+2, n` stages the first two and rejects `n` as the new overflow request; a fence through `n+1` then rejects for the missing frontier;
- delivery `n, n+1, n+2` stages `n` and `n+1`, rejects `n+2`, and the same fence through `n+1` can finalize.

Thus “overflow rejects new staging atomically” makes the resident/finalizable set depend on delivery order. In the first ordering it can also deadlock: the buffer is full, the missing frontier cannot enter, and no contiguous fence can drain the buffer. No arrival-independent admission window, reserved frontier capacity, deterministic eviction/cancellation, or equivalent recovery contract is specified. Phase 1 would have to invent that canonical ingress behavior.

### 3. B-02 REGRESSION CHECK

`CLOSED`

The v0.3 edits do not weaken ADR-0002/ADR-0004: persisted authority and implied write class remain immutable identity properties; conversion still requires a new ID, explicit migration, authority ADR, and operator approval; restore validates fingerprints; and host-owned/derived truth cannot be fabricated through ordinary lifecycle paths.

### 4. B-03 REGRESSION CHECK

`CLOSED`

The v0.3 edits do not weaken ADR-0004/ADR-0006: continuation remains bound to exact content-addressed manifest/config artifacts, definition fingerprints, behavior epochs, and explicit delayed-obligation execution modes; migration remains isolated and atomic; replay modes remain distinct; and snapshots remain stable-boundary representations.

### 5. REMAINING BLOCKERS

- **B-01A — Arrival-independent bounded staging admission is undefined.** Freeze a bounded ordinal-window/credit/reservation rule or another deterministic capacity policy that preserves room for the contiguous frontier and yields the same finalizable set for the same sequencer-authored inputs regardless of delivery order. Specify deterministic retry/eviction/cancellation behavior and add the capacity-two reversed-delivery fixture above across service, embedded, and concurrent-worker paths.

### 6. MAJORS / MINORS

- **Majors:** None beyond the remaining blocker. No prior major is reopened.
- **Minor:** A blank line before R-101 terminates the coverage matrix table, leaving R-101 through R-105 as a headerless Markdown table. Remove the blank line so the new traceability rows render under the matrix header.

### 7. PRIMITIVE-SUFFICIENCY RESULT

The frozen causal primitive set remains sufficient. `TimelineSequencerAuthority` and `TimelineFence` are correctly modeled as protocol/integration lifecycle constructs governing admission and finality, not as causal world-state primitives. Closing the capacity-policy defect requires an ingress contract correction, not a new runtime primitive.

### 8. PHASE-1 AUTHORIZATION

`NO`

The Phase-1 core skeleton must not choose an arrival-dependent staging-capacity policy or invent how a full out-of-order buffer recovers. Once that bounded admission/finality rule is frozen and covered by reversed-delivery overflow tests, the reviewed architecture has no other remaining Phase-1 blocker.
