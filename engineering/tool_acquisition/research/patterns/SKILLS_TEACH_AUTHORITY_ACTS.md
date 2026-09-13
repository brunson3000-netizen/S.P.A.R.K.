# Pattern Card — Skills Teach, Authority Acts

PATTERN: Progressive Skill Loading With Authority Separation
STATUS: STRONG_CANDIDATE
CONFIDENCE: HIGH

## Source evidence
- Agent Skills spec/reference @ `69ef37e9424c0a7ea9dd2293b559e43ec8176379`
- `docs/specification.mdx`
- `docs/client-implementation/adding-skills-support.mdx`
- `docs/skill-creation/using-scripts.mdx`
- `skills-ref/src/skills_ref/{parser,validator,prompt}.py`
- reference tests
- Cross-project convergence: ARD can describe skill artifacts; governed registries can register/search skills separately from execution.

## Problem

Agents need specialized procedures and supporting knowledge, but eagerly loading every instruction set is expensive and allowing instructional packages to grant executable permissions is unsafe.

## Mechanism

Three-tier disclosure:
1. advertise compact metadata (`name`, `description`, stable host identity/provenance)
2. load full instructions only when selected
3. expose references/assets/scripts only when needed.

Treat the skill as **instructional content**. Any requested tool/script/action passes through the same independent capability broker and call-time authority rules as if no skill were loaded.

## Dependencies

- skill/package store
- compact metadata catalog
- parser/validator
- provenance/trust identity
- activation/learning primitive
- resource reader
- independent capability/tool authority broker

## Benefits

- portable procedural specialization
- lower baseline context
- natural support for specialist workers
- existing interoperable `SKILL.md` ecosystem
- reference/resources remain outside context until needed
- skill teaching can evolve independently from tool adapters.

## Risks / failure modes

- malicious project skill instruction injection
- same-name shadowing between scopes
- stale/revoked skill remains in protected context
- experimental `allowed-tools` misread as authority
- bundled scripts or runtime package fetches introduce side effects/network/code execution
- skill description quality controls discovery
- lenient client parsing can differ from strict validation.

## Boundaries crossed

Skill source → catalog: provenance/trust gate required.
Catalog → model: compact metadata only.
Model → activation: current valid skill ID required.
Activated instructions → resource read: resource path constrained to skill package unless separately authorized.
Instructions → tool/script execution: no authority transfer; every call reauthorized.

## Agent-facing surface

Preferred host-neutral shape:
- `find_capability()` may return a skill as one resource type
- `learn_capability(stable_id)` may materialize skill instructions
- referenced resources can be fetched on demand
- executable actions still use `execute_capability()` under current grants.

## Determinization relevance

HIGH. Discovery, validation, provenance checks, collision handling, activation lookup, resource enumeration/digests and permission checks should be deterministic.

## Likely architectural location

Capability teaching layer / skill registry sitting beside, not above, executable tool authority.

## Finding class

PATTERN + TEST/INVARIANT

## Primary disposition

BORROW_PATTERN

## Compatibility recommendation

Prefer `SKILL.md` compatibility for the portable teaching package unless later evidence shows a hard requirement it cannot express. Extend outside the file with host metadata rather than forking the format prematurely.

## Required invariants

1. Skill activation never grants a tool/script permission.
2. `allowed-tools` is advisory/package metadata unless independently mapped through host policy.
3. Project skill shadowing cannot bypass trust/provenance checks.
4. Canonical skill identity includes source/provenance/version; local `name` is an alias.
5. Skill resources cannot escape the package path through traversal/symlink tricks without separate authorization.
6. Executable bundled scripts are treated as executable code, not documentation.
7. Revoked/replaced skill instructions can be invalidated even if prior content was protected from generic context compaction.
