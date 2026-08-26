# S.P.A.R.K. Phase 1 — Final Closure Authorization

**Date:** 2026-08-26
**Operator decision:** AUTHORIZED
**Phase 2:** NOT AUTHORIZED

The operator authorizes the final Phase-1 closure implementation defined by the Fable overnight architecture mission.

Authorized scope:

1. Final B-01 admission-identity repair:
   - staged command-ID and `(source_id, source_sequence)` registries are derived indexes over positively staged commands;
   - when a staged ordinal poisons, remove the formerly staged command/source identity claims;
   - the poisoning claimant is not registered;
   - at successful fence promotion, move staged identity claims into finalized identity registries rather than retaining duplicate staged truth;
   - epoch reset behavior remains as specified by the Fable architecture;
   - preserve semantic/admission separation and all independently closed finality behavior.

2. Bundle the Fable-recommended small cleanups:
   - m-02: manifest-content hashing becomes construction-order independent for duplicate-ID invalid manifests;
   - S2: unify duplicated scheduler/timeline bounded conflict-evidence implementation where doing so preserves public/API/canonical behavior;
   - S3: simplify the proven retention-bound condition with its proof documented.

3. Implement the complete AT-H final closure test corpus from:
   `engineering/phase1/PHASE_1_FINAL_CLOSURE_TEST_MATRIX_2026-08-26.md`

4. Follow:
   `engineering/phase1/PHASE_1_OPUS_FINAL_IMPLEMENTATION_PLAN_2026-08-26.md`

Hard limits:

- no Phase-0 architecture change;
- no new causal primitive;
- no Phase-2 implementation;
- no service, persistence, actor behavior, MCI/game integration, dialogue, voice, scripting/plugin runtime, broad catalogs, or runtime LLM;
- do not weaken previously closed B-01/B-02/B-03/B-04/M-01/M-02/M-03/M-04/m-01 properties.

Independent Codex review remains mandatory before Phase 1 can close.
