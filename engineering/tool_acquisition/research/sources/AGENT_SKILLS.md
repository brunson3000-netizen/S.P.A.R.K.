# Source Study — Agent Skills

Status: FIRST TRACE COMPLETE
Study date: 2026-09-13

## Frozen source

- Project: Agent Skills open specification + reference validator
- Upstream: `https://github.com/agentskills/agentskills`
- Commit: `69ef37e9424c0a7ea9dd2293b559e43ec8176379`
- Retrieval/study date: 2026-09-13
- License: Apache-2.0
- Form/runtime: Markdown/YAML directory format; Python `skills-ref` parser/validator/reference prompt generator; client-implementation guidance
- Primary docs: `docs/specification.mdx`, `docs/client-implementation/adding-skills-support.mdx`, skill-creation guides

## Problem framing

PROBLEM: Agents need portable specialized procedural knowledge, examples, references, scripts and templates without loading every specialization into the base context.

USER: agent harnesses/clients and models that need task-specific instructions.

INPUT: a skill directory containing required `SKILL.md` plus optional scripts/references/assets.

OUTPUT: a compact catalog entry at discovery time, full instructions at activation time, and supporting resources only when needed.

AUTHORITY: The format itself primarily describes/teaches behavior. However, skills can reference executable scripts and the experimental `allowed-tools` field can express pre-approved tool names. Actual file/tool/script execution authority is left to the client/harness.

TRUST: Not normatively solved by the format. The client guide explicitly warns that project-level skills may come from untrusted repositories and suggests trust-gating. Script/package execution inherits the client/OS trust model.

STATE: Installed skill directories; per-session activated-skill/context state in clients; no canonical registry/state service is required by the format.

FAILURE: malformed/missing metadata can cause skip/warnings; client guide encourages lenient compatibility for some cosmetic violations. Execution failures of bundled scripts/tools are outside the core format.

## Trace A — discovery / catalog construction

Client scans configured skill roots
→ find subdirectories containing `SKILL.md`
→ parse YAML frontmatter
→ require at minimum `name` + `description`
→ record name, description and location
→ apply deterministic name-collision precedence
→ optionally exclude disabled/permission-denied/model-hidden skills
→ expose compact skill catalog to the model.

Reference prompt generator `skills_ref.prompt.to_prompt` emits only:
- name
- description
- SKILL.md location.

Client guide estimates roughly 50–100 tokens per installed skill in tier-1 disclosure.

Common discovery scopes:
- project/client-specific
- project `.agents/skills/`
- user/client-specific
- user `.agents/skills/`
- optional compatibility paths.

Convention: project-level skills override user-level skills. Within the same scope, first-found or last-found is allowed if deterministic and warned.

Primary files:
- `docs/client-implementation/adding-skills-support.mdx`
- `skills-ref/src/skills_ref/parser.py`
- `skills-ref/src/skills_ref/prompt.py`

## Trace B — activation / progressive loading

Model sees tier-1 catalog
→ selects a relevant skill by description OR user explicitly activates it
→ activation either:
  - reads `SKILL.md` via ordinary file tool, or
  - calls a dedicated `activate_skill(name)` tool
→ harness returns full SKILL instructions (whole file or body-only)
→ optional structured wrapper identifies skill name, directory and bundled resource list
→ resources are listed but not eagerly read
→ model reads selected references/assets/scripts only when instructions call for them.

Specification’s three tiers:
1. metadata (`name` + `description`) at startup
2. full `SKILL.md` when activated, recommended <5000 tokens / <500 lines
3. scripts/references/assets on demand.

Dedicated activation can constrain skill name to a current enum, enforce permissions/consent, list resources and track activation.

Client guide also recommends preserving activated skill instructions through context compaction and deduplicating repeat activation.

## Trace C — optional executable resources

`SKILL.md` can instruct the model to invoke bundled `scripts/*` or one-off ecosystem tools (`uvx`, `npx`, `deno`, `go run`, etc.).

The script guide strongly encourages:
- version pinning
- non-interactive interfaces
- `--help`
- clear errors
- structured stdout / diagnostics on stderr
- idempotency
- closed input constraints
- dry-run
- meaningful exit codes
- safe defaults
- predictable/bounded output.

This is excellent agent-facing tool design prior art, but script execution remains ordinary executable authority. The skill format does not provide sandboxing, current grants, signature verification or side-effect containment.

## Validation/tests extracted

Reference parser:
- requires YAML frontmatter
- extracts metadata + trimmed body
- requires non-empty name/description for properties.

Reference validator:
- allowed frontmatter keys are closed-set
- name length <=64
- lowercase, no leading/trailing/consecutive hyphens
- name matches directory
- description <=1024
- compatibility <=500
- accepts experimental `allowed-tools`
- NFKC-normalizes names.

Tests protect those constraints, including unexpected-field rejection and `allowed-tools` acceptance.

Important interoperability detail: the client guide recommends *lenient* loading for some invalid names/directory mismatches to improve cross-client compatibility, while the reference validator is strict. That means validation compliance and practical loading behavior are intentionally not identical.

## Boundary findings

### Skill metadata ↔ model
Very compact and well suited to progressive disclosure. But description quality controls whether the model discovers the right teaching package.

### Project repository ↔ skill catalog
Potential trust boundary. A newly cloned repository can contain project skills; the guide suggests a folder trust gate but does not make it a format-level rule.

### Skill name ↔ canonical identity
Skill `name` is local/human-readable, not a globally stable provenance identity. Project-over-user shadowing is a deliberate convention. A governed multi-source catalog therefore needs a separate canonical ID/version/source identity.

### Skill instructions ↔ execution authority
The specification allows `allowed-tools` as experimental metadata and skills can instruct agents to run scripts/commands. Neither should be treated as a grant. A host must independently authorize every executable action.

### Skill directory ↔ file permissions
Client guidance suggests allowlisting the skill directory to avoid prompts when reading bundled resources. That should mean *readability of instructional resources*, not automatic execution permission for scripts.

## Determinization candidates

- directory scanning with hard bounds
- metadata parsing/validation
- canonical skill identity/provenance binding
- collision resolution
- trust filtering
- catalog generation
- activation lookup
- resource enumeration
- activation deduplication
- content/version digesting
- permission checks before resource/tool/script access.

## Material extracted

IDEA: portable procedural capability packages around a minimal required `SKILL.md`.

PATTERN: three-tier progressive skill loading — metadata → instructions → resources.

PATTERN: separate teaching package from executable authority.

ALGORITHM: deterministic discovery, parsing, collision handling and activation lookup.

CODE: the Python reference parser/validator is useful as conformance prior art; direct reuse is not preferred for a Rust core.

TEST/INVARIANT: untrusted project skills cannot silently gain tool/script authority; loading a skill does not grant execution; skill name alone is not canonical identity.

## Primary disposition

BORROW_PATTERN

Reason: the format is already cross-client prior art and maps well onto the emerging middle layer. SPARK should strongly consider compatibility with `SKILL.md` rather than inventing an incompatible teaching format, while wrapping skills in host-owned provenance, identity, trust and authority controls.
