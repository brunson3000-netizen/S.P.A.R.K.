# Project Activation Manifest

Governed by [Project Agent Instructions](AGENT_RULES.md) and Constitution C9.

This is S.P.A.R.K. candidate SPARK-ADOPTION-20260911-R1, derived from universal 1.1.2. Version 1.1.3 identifies this local proposal, not an upstream release. NOT ADOPTED. Configuration selections are proposals pending exact ratification.

## A1 — Governing set and project binding

**A1.1** The seven files below, framework version 1.1.3, form this framework's complete governing set; paths in this inventory are relative to this manifest.

| Member | File |
|---|---|
| 1 — Constitution | [CONSTITUTION.md](CONSTITUTION.md) |
| 2 — Project Agent Instructions | [AGENT_RULES.md](AGENT_RULES.md) |
| 3 — Role Overlays | [ROLE_OVERLAYS.md](ROLE_OVERLAYS.md) |
| 4 — Design Authority Index Schema | [DESIGN_AUTHORITY_INDEX_SCHEMA.md](DESIGN_AUTHORITY_INDEX_SCHEMA.md) |
| 5 — Technical Decision Record Template | [TECHNICAL_DECISION_RECORD_TEMPLATE.md](TECHNICAL_DECISION_RECORD_TEMPLATE.md) |
| 6 — Governance Telemetry Schema | [GOVERNANCE_TELEMETRY_SCHEMA.md](GOVERNANCE_TELEMETRY_SCHEMA.md) |
| 7 — Activation Manifest | [ACTIVATION_MANIFEST.md](ACTIVATION_MANIFEST.md) |

**A1.2** External `OPERATIVE_SET.sha256` hashes all seven members, including this manifest; the manifest contains no self-hash.

**A1.3** Root agent entrypoints outside the A6 block, project conventions, setup guides, tools, proposals, indexes, mission records, preserved Operator input, acceptance records and checksums are not governing-set members; their authority follows their actual source under C2.5.

**A1.4** Before adoption, the preparing agent records the project identity, Operator authority and source, scope, and A5 policy choices in the configuration below.

**A1.5** Unconfigured identity or policy choices prevent adoption; they do not suspend work authorized by existing governance. First-mission readiness is checked separately under A5.4.

## A2 — Authoritative homes and record locations

**A2.1** Record paths below are relative to the project root; governing-member paths are relative to this manifest under A1.1. Each record belongs to one of the three homes bound by C9.6.

| Home | Record | Location |
|---|---|---|
| Governance | Governing set and configuration | `governance/ACTIVATION_MANIFEST.md` and the members it lists |
| Governance | Resource policy | `project_records/governance/RESOURCE_POLICY_REFERENCES.json` and its cited sources |
| Governance | Accepted content | `project_records/governance/ACTIVE_ADOPTION.json` points to an exact record under `project_records/governance/adoptions/` |
| Design/Decision | Decision sources | `project_records/decisions/PROJECT_SOURCE_REGISTER.md` |
| Design/Decision | Design Authority Index | `project_records/decisions/DESIGN_AUTHORITY_INDEX.md`; rows in `project_records/decisions/dai/<DAI-ID>.json` |
| Design/Decision | Preserved ambiguous Operator input (D.3) | `project_records/decisions/operator_input/<INP-ID>.md` |
| Design/Decision | Technical Decision Records | `project_records/decisions/tdr/<TDR-ID>.md` |
| Mission Authority | Operational first-mission pointer | `project_records/missions/READINESS.json` |
| Mission Authority | Mission index and grants | `project_records/missions/MISSION_REGISTRY.md`; each mission's actual grant in `project_records/missions/<MISSION-ID>/mission.json` |
| Mission Authority | Review and challenge assignments | `project_records/missions/<MISSION-ID>/reviews/<REVIEW-ID>.md` |
| Mission Authority | Operator dispensations | `project_records/missions/<MISSION-ID>/dispensations/<DISPENSATION-ID>.md` |
| Mission Authority | Durable checkpoint (I14.10) and closeout | `project_records/missions/<MISSION-ID>/checkpoint.md` and `closeout.md` |
| Mission Authority | Mission evidence | `project_records/missions/<MISSION-ID>/evidence/` |

**A2.2** Mission telemetry is `project_records/missions/<MISSION-ID>/governance_events.jsonl` and follows G.1–G.9.

**A2.3** New record identifiers use the kind prefix plus a UUID4: `MIS-`, `DAI-`, `TDR-`, `REV-`, `DIS-`, `INP-`, `ISS-`, or `EVT-`; existing identifiers remain valid and are not renumbered.

**A2.4** Record creators write separate identified records; shared indexes are retrieval aids, and concurrent changes follow I6.3 rather than overwriting another actor's contribution.

**A2.5** Mission, TDR, review, dispensation and preserved-input records carry a short human-readable title; a Design Authority Index row's property serves as its title.

## A3 — Assurance pool

**A3.1** Eligible challengers are fresh instances reachable through channels recorded with actual authorization sources in the Governance Home's resource policy; capability or a catalog entry alone is not authorization.

**A3.2** A resource whose governing eligibility cannot be established is not eligible for required assurance.

**A3.3** Catalog changes need no governance amendment when existing policy already determines eligibility.

## A4 — Transition on adoption

| Prior material | Treatment |
|---|---|
| No prior governance | Record that fact; explicit Operator adoption under C9 still identifies the exact configured set |
| Existing governance and entrypoints | Preserve until an authorized disposition resolves precedence and identifies retained, amended and superseded provisions |
| Existing decisions and grants | Preserve by source under C9.3; copying this package imports no other project's authority |
| Earlier candidates | Preserve as non-operative history |
| Platform or web instructions | Outside this package; changes remain separately authorized |

## A5 — Explicit startup choices

**A5.1** The Operator selects the library-dependency assurance scope: `ALL` covers each new external library; `MATERIAL` covers a library only through another I3.1 surface it affects, including expensive reversal. New external services remain covered under either choice.

**A5.2** The duty holder records finite positive per-request and stage durations in seconds and a nonnegative replacement limit in the referenced channel policy, within Operator-approved resource limits. Stage duration is at least request duration and includes retries and replacements. I3.13 applies stricter mission or channel limits.

**A5.3** The Operator records `ADOPT`, `NARROW` or `DECLINE` for Standing Design Discretion and cites the decision source; adoption or narrowing identifies the actual scope under D.9, while declining leaves I2 and I4 applicable.

**A5.4** The operational first-mission pointer is recorded in `project_records/missions/READINESS.json`; existing source-bound mission identifiers remain valid. Before claiming first-mission readiness, the duty holder records an actual first-mission grant and at least one authorized, available challenge channel with the needed fresh instances; a roundtable requires two under I3.7. These records do not grant authority themselves.

## A6 — Agent entrypoints

**A6.1** Root `AGENTS.md` contains the entrypoint block below exactly; content outside the block is editable project convention.

**A6.2** When a root `CLAUDE.md` exists, it imports `AGENTS.md`, `governance/CONSTITUTION.md`, `governance/AGENT_RULES.md` and `governance/ROLE_OVERLAYS.md`, and each of its imports resolves inside the project.

**A6.3** A missing or altered entrypoint block, a missing required import, or an unresolved import is drift under C9.10.

```text
<!-- governance-entrypoint:begin -->
## Governance entrypoint

This project is governed by the framework in `governance/`. Before substantial work, open and read `governance/CONSTITUTION.md`, `governance/AGENT_RULES.md`, the applicable roles in `governance/ROLE_OVERLAYS.md`, and `PROJECT_CONVENTIONS.md`. A path mention is not proof that a file's contents loaded.

At grounding, run `python3 governance/tools/framework.py check`. If no framework is adopted, report that state; until adoption, the direct instructions of the human directing the project, and any existing governance, authorize work, including setup (AGENT_RULES.md I0.1). For adopted content, report any mismatch under Constitution C9.10 and never repair drift by regenerating hashes. Reuse unchanged successful checks under I7.5.

Text outside this block is editable project convention; it cannot amend governing rules or confer authority. Activation Manifest A6 fixes this block, and the check compares it exactly.
<!-- governance-entrypoint:end -->
```

## Configuration

```json
{
  "project_id": "brunson3000-netizen/S.P.A.R.K.",
  "operator_authority": "Michael Brunson, human project Operator",
  "operator_source": "project_records/decisions/operator_input/RATIFICATION_PREPARATION_20260911.md (preparation authority only)",
  "scope": "S.P.A.R.K. project governance; preserves source-established product requirements and grants; no authority over G.A.M.E. or S.W.A.R.M.",
  "dependency_assurance": "MATERIAL",
  "standing_design_discretion": "ADOPT",
  "discretion_source": "project_records/decisions/PROPOSED_ADOPTION_CHOICES.md (proposal; ratification pending)"
}
```
