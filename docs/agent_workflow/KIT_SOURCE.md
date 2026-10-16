# Windows agent kit — source and identity

**Version:** 1.2.0 (2026-10-16). The version names the release; the hashes below identify the exact content.

This folder is a vendored copy of the common Windows agent kit. It is operational convention: it grants no authority, adopts no governance and changes no rule of the project that holds it. That project's own instructions govern wherever they differ. `WINDOWS_EXECUTION_PROFILE.md` records the Operator's standing tool grants with their sources; their authority is the Operator's, not the kit's.

| Question | Answer |
|---|---|
| What identifies this copy? | `WINDOWS_AGENT_KIT.sha256` in this folder lists the SHA-256 of each kit file. A file that does not match is not the kit. |
| Where is the maintained source? | The FEDERATION repository, folder `docs/windows_agent_kit/` (on this host: `C:\Users\ChroMikey\Projects\FEDERATION\docs\windows_agent_kit\`). Its `README.md` is the integration guide and its `KIT_MANIFEST.sha256` identifies the whole kit. |
| How is it updated? | Never by editing these files in place. A newer kit version is installed from the source with `tools/install_kit.py`, under this project's own rules, as one isolated change. |
| Where do project-specific adaptations go? | In the project's own instruction files (for example its `AGENTS.md`), not in this folder. |
| What does it contain? | `SHARED_AGENT_INSTRUCTIONS.md`, `ARCHITECT.md`, `CODING_AGENT.md`, `ADVERSARIAL_REVIEWER.md`, `WINDOWS_EXECUTION_PROFILE.md`, `WINDOWS_LAUNCH_BRIEFS.md`, `DOCUMENTATION_FIRST_POLICY.md` and this file. |

Using FEDERATION as the source repository imports none of FEDERATION's governance, review limits, decisions or holds.
