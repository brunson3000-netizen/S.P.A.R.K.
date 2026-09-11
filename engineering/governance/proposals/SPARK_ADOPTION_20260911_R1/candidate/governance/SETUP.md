# S.P.A.R.K. candidate setup and adoption

This is a non-operative review candidate. Do not copy it over an existing repository before its integration and adoption are authorized.

1. Review the package change/disposition map and exact candidate identity. Reconcile changes to the production baseline since 7e3a0aae only where relevant.
2. Preserve existing project files and entrypoints. Merge A6 entrypoints in an isolated work surface. The included source register points to controlling requirements and existing grants; this package grants no phase or cross-project expansion.
3. Obtain independent review of the actual candidate and changed tooling. Preserve findings and verify their repairs with the assigned reviewer.
4. Resolve the exact proposed adoption choices. Record an actual authorized review channel and finite timing/replacement values in `project_records/governance/RESOURCE_POLICY_REFERENCES.json`. Do not substitute example values for authority. Set the operational pointer in `project_records/missions/READINESS.json` to an actual complete mission. Existing IDs may be retained; new IDs use UUID4.
5. Update the actual discretion source and row only after the Operator's decision. Until then the row remains PROPOSED; no readiness or adopted discretion claim is valid.
6. Before adoption, compute candidate hashes with `python3 governance/tools/framework.py write-hashes`; run `check`, `index`, `check-events`, the replay suite and applicable focused checks. `ready` intentionally fails in the delivered package because actual channel/timing and ratified discretion are absent. Configuration may be adopted separately from operational readiness under A1/A5.
7. Present exact content and dispositions for explicit Operator acceptance. Only then create the real acceptance record and active pointer. Run `check` against accepted identity; preserve a trusted receipt and verify the intended destination. Keep the historical grant and status records intact.

After adoption, never repair drift by regenerating active hashes. Work on a separately identified candidate. Acceptance identifies exact new content. Operational timing and first-mission pointer changes remain outside governing identity but inside the applicable authority envelope. No local structural tool authenticates human permission or enforces runtime authority.
