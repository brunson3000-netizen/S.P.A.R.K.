# S.P.A.R.K. constitutional candidate — local 1.1.3

Reusable governance for agent-assisted software and technical-engineering projects with one designated human Operator. It is not a universal legal constitution or a co-owner decision model. Teams need an explicit authority arrangement before using its single-Operator model; this edition does not invent voting or tie-breaking rules.

Derived from universal 1.1.2. Start with [SETUP.md](SETUP.md). Project bindings and proposed choices are populated for review. No adoption is recorded; operational readiness is intentionally incomplete.

## Layout

The seven governing files live in this folder. Root `AGENTS.md` and `CLAUDE.md` are small retrieval entrypoints. `AGENTS.md` contains one fixed governance block (Activation Manifest A6); everything else in it is editable project convention, alongside `PROJECT_CONVENTIONS.md`. `framework.py check` verifies the fixed block and the required `CLAUDE.md` imports, so the path from agents to governance cannot be edited away silently. The project's own README is untouched. Platform instructions retain their actual authority.

## Agent loading

Claude Code expands the `@` imports in `CLAUDE.md` at session start, loading the Constitution, agent rules, role overlays and conventions. Codex loads root `AGENTS.md` automatically but reads the governing files only when it follows the entrypoint instruction; this keeps Codex's shared 32 KiB instruction budget free for project conventions, at the cost of relying on that instruction. The check warns when root `AGENTS.md` passes three-quarters of that budget.

## Tools

`governance/tools/framework.py` runs `check`, `ready`, `index`, `check-events`, `new-mission`, `write-hashes` and `candidate-hashes`. `index` prints a readable table of decision rows, the provisional review queue and missions from the per-record files. New records use UUID4 identifiers plus a short human-readable title; existing source-bound identifiers remain valid, and agents cite both to the Operator (I12.9). The tooling uses Python 3.10+ and its standard library; it does not select the project's implementation language.

## Clause identifiers

The source's stable identifiers are retained. I8.1, I11.3, I14.8, OP.1 and T.2 are retired; numbering gaps and C10/C11 before C8/C9 are intentional, not missing provisions.

See [CHANGES.md](CHANGES.md) for what changed and why, and [VALIDATION.md](VALIDATION.md) for the checks actually run and their limits.
