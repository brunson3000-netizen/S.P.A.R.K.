# S.P.A.R.K. local candidate 1.1.3

NON-OPERATIVE, derived from universal 1.1.2. See package change map and unified diff. Deferred status, capture gap, operational timing/pointer and legacy IDs are proposed corrections; prior entries below are upstream history.

# Changes

## 1.1.2 — targeted drafting corrections

Applies a review of 1.1.1 that found clauses permitting unnecessary escalation or repeated work. The review scope was fixed to its six findings; the tooling, adoption checks, assurance surfaces and review independence are unchanged.

| Finding | Change |
|---|---|
| C11.2 made an agent's inability to choose between defensible courses a branch, even when the choice was delegated | C11.2 now triggers only on missing authority, an unresolved interpretation of governing sources, or unmet required assurance. Delegated choices stay with their owner under C1.3–C1.4 and C11.3. |
| Dependent of C11.2 (C9.4) | The telemetry `branch_reason` values follow the new triggers: `MISSING_AUTHORITY`, `UNRESOLVED_INTERPRETATION`, `UNMET_REQUIRED_ASSURANCE`, `NOT_APPLICABLE`. `COMPETING_COURSES` is retired. |
| I16.9 barred "resolving substantive choices" without separating delegated choices from reserved decisions | I16.9 now has the drafting agent resolve choices within delegated authority. It still identifies each necessary decision outside that authority separately for the Operator under I2. The review's wording dropped "separately"; it is kept so decisions outside the agent's authority stay visible rather than embedded in a draft. |
| I16.1 covered governing instruments only | I16 now covers solution design as well, and is renamed to match. I16.1 defines "drafting agent" for both uses. I16.5 (clause-writing conventions) and I16.12 (adoption and amendment process) are limited to governing instruments, where they apply. |
| I16.13–I16.14 set no review scope in advance and did not restrict reopening | The review scope is fixed before drafting. Corrections recheck only affected requirements, completed checks reopen only for a demonstrated defect or approved change, and work stops when approved requirements and mandatory checks are satisfied. |
| `PROJECT_CONVENTIONS.md` indexed every Operator-directed convention in the DAI, duplicating D.6 | A convention keeps its source in the conventions record and is indexed only when D.6 requires it. |
| A5.1 `NOT_CHEAPLY_REVERSIBLE` duplicated coverage | I3.1 already covers reversal requiring data migration, cross-component changes or a broken interface, so `MATERIAL` catches load-bearing libraries. The 1.1.1 rationale was mistaken, and the option is removed from the manifest, tool, setup guide and fixtures. |

No clause identifier is added or removed. The replay suite adds five cases: the retired dependency option and retired branch reason are rejected, and both new branch reasons validate.

## 1.1.1 — merge of 1.1.0 and 1.0.1

This candidate takes 1.1.0 as its base and merges the stronger parts of the parallel 1.0.1 revision. It also fixes gaps found by probing 1.1.0. It changes the reusable candidate only and amends no adopted project.

| Source | Change |
|---|---|
| 1.1.0 gap | `check` never examined the root entrypoints; removing the governance instruction from `AGENTS.md`, breaking a `CLAUDE.md` import, or deleting both files still passed. New A6 fixes an entrypoint block that `check` compares exactly, and requires the governing imports in `CLAUDE.md` when it exists. Text outside the block stays editable. |
| 1.0.1 | Codex budget warning: `check` warns when root `AGENTS.md` passes three-quarters of the default 32 KiB instruction budget. |
| 1.0.1 | Pre-adoption status (I0.1): until adoption, the direct instructions of the human directing the project authorize work, including setup, so an agent in a brand-new project does not stall on empty records. The A6 block cites it. |
| 1.0.1 | Readable references: records keep collision-free UUID4 identifiers and add a required short title (A2.5); Operator-facing references give title and identifier (I12.9). `new-mission --title` sets it. |
| 1.0.1 | Readable views: `index` renders DAI rows, the provisional review queue and missions as tables, and rejects malformed rows. It checks invalid kind or status, authoritative status without a source, SUPERSEDED without a replacement, provisional rows missing containment fields, and identifier/filename mismatch. |
| 1.0.1 | Complete grants: `mission.json` records title, actors, duration and conditions (C0.2), plus Operator wording separate from interpretation (C5.9), exclusions, role (C2.10) and delegation source (C2.7). `ready` requires the C0.2 fields. |
| 1.0.1 | Durable checkpoint: each mission folder gets `checkpoint.md` for I14.10 handoff, with the fields a successor needs. |
| 1.0.1 | Middle dependency option: A5.1 adds `NOT_CHEAPLY_REVERSIBLE` between `ALL` and `MATERIAL`. |
| 1.0.1 | Constitution C2.1 names the governing instruments without filenames, so the Constitution no longer depends on file layout. C9.10 is retained. |
| 1.1.0 gap | D.3 requires preserving ambiguous Operator input outside authoritative rows, but no location existed. Added `operator_input/` records with `INP-` identifiers and a template. |
| 1.1.0 gap | The record map had become nine rows while C9.6 speaks of three homes; A2.1 now groups every record under Governance, Design/Decision or Mission Authority. |
| 1.1.0 gap | `.gitattributes` marked everything under `governance/` as text, which would corrupt binary files such as `.pyc` under Git. It now pins LF only for governed text types. |
| 1.1.0 gap | The replay suite invoked `python` and failed where only `python3` exists. It now uses the interpreter that runs it, is written readably, checks that every 1.0.0 identifier survives, and grows from 35 to 51 cases. |
| Both | Setup gives the source and fixture timing values as reference points without making either a default. |

Retained from 1.1.0 without change: small editable root entrypoint with rules in `governance/`, explicit A5 choices with no inherited timing, separate acceptance records with pointer digest and optional trusted digest, refusal of in-place rehashing after acceptance, candidate hash lists for amendments, the enforced JSON telemetry schema, per-record files for concurrency, and the `ready` gate.

## 1.1.0 — feedback disposition

This revision changes the reusable candidate only. It does not amend an adopted project.

| Feedback | Correction |
|---|---|
| Missing record topology | A2 defines actual DAI, TDR, mission, assignment, dispensation and log homes, UUID4 IDs and separate-record concurrency. A real PROPOSED DAI row is included. G supplies exact JSON keys/types and event identity. |
| Root AGENTS is occupied by hashed rules | Moved rules to governance/AGENT_RULES.md; root AGENTS remains a short editable entrypoint. Added PROJECT_CONVENTIONS.md and I13.3 scope boundary. CLAUDE uses explicit imports. |
| Startup requires more than bindings | A5 and ready check cover dependency scope, channel-appropriate limits, conscious discretion choice, actual channel metadata and first-mission grant. No default 180/600 timing. |
| Discretion shapes first work | A5.3 requires an explicit adopt/narrow/decline source and a consistent DAI disposition; no silent adoption. |
| Hashes do not bind acceptance | C9.10/I7.6 add grounding/drift handling. The tool compares separate acceptance hashes, blocks in-place hash rewriting after acceptance, and accepts an independently trusted adoption digest. Local records remain editable, not authenticated signatures. |
| Git-only completion and broad universal claim | I6.6 uses destination/content identity; Git fields are conditional. The README states software/technical scope and the single-human-Operator limit. |
| Root clutter, dead code, numbering fossils | Supporting documents/tools moved beneath governance; no root README collision. Validator replaced and exercised with negative cases. Retired identifiers documented without renumbering. |

The broad technical review surfaces and independent artifact review remain. Dependency scope and timing now require explicit setup choices instead of silently choosing a policy for adopters. No co-owner constitution, provider integration, approval scheduler or telemetry service was added.

### Agent-loading source checks — 2026-09-11

Official Codex documentation describes a combined project-instruction budget, default 32 KiB. The previous approximately 20 KB root file fit by itself: earlier guidance can reduce its available budget, while nested files later in discovery may be truncated or omitted. A smaller entrypoint reduces discovery-budget use; reading the governing files still consumes model context and must actually occur. It does not guarantee compliance.

Official Claude documentation supports @path imports and says imported text enters startup context. Imports improve explicit loading but do not reduce that context or guarantee adherence. These imports stay inside the project; excluded memory settings or client behavior still require a receiving-project check.

Sources: https://developers.openai.com/codex/guides/agents-md and https://code.claude.com/docs/en/memory. User-cited issue reports and the third-party import guide were treated as leads, not stronger authority than the official documentation.

The previous hash validator never claimed adoption authentication, but its convenience hash writer allowed a changed file to become a self-consistent candidate. The new tool distinguishes that operation from accepted-content verification and refuses in-place rewriting once acceptance records exist. A writer able to modify the tool and every trusted record can still bypass an entirely local process.
