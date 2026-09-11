# Project agent entrypoint

<!-- governance-entrypoint:begin -->
## Governance entrypoint

This project is governed by the framework in `governance/`. Before substantial work, open and read `governance/CONSTITUTION.md`, `governance/AGENT_RULES.md`, the applicable roles in `governance/ROLE_OVERLAYS.md`, and `PROJECT_CONVENTIONS.md`. A path mention is not proof that a file's contents loaded.

At grounding, run `python3 governance/tools/framework.py check`. If no framework is adopted, report that state; until adoption, the direct instructions of the human directing the project, and any existing governance, authorize work, including setup (AGENT_RULES.md I0.1). For adopted content, report any mismatch under Constitution C9.10 and never repair drift by regenerating hashes. Reuse unchanged successful checks under I7.5.

Text outside this block is editable project convention; it cannot amend governing rules or confer authority. Activation Manifest A6 fixes this block, and the check compares it exactly.
<!-- governance-entrypoint:end -->

## Project conventions

Record ordinary commands and conventions here or in `PROJECT_CONVENTIONS.md`. Keep this file small: Codex shares a default 32 KiB instruction budget across the `AGENTS.md` files it discovers, and `governance/tools/framework.py check` warns when this file passes three-quarters of that budget.
