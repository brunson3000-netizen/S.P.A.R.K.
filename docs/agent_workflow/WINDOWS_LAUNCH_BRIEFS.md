# Windows launch briefs (visible cooperative pair)

Operational convention. The Operator starts and stops every working agent in a visible window; no agent, script or architect launches, resumes or replaces one in the background. The architect prepares everything else, so the Operator's routine action is one double-click (Operator correction, 2026-10-16). Command forms were checked against the installed `--help` of Claude Code 2.1.301 and codex-cli 0.162.1 (2026-10-16): both accept the initial prompt as a positional argument. See `WINDOWS_EXECUTION_PROFILE.md`.

## Placeholders

| Placeholder | Meaning |
|---|---|
| `<TASK_DIR>` | Task folder beneath the project root (for example `<project root>\.local\<TASK>`) |
| `<BUILDER_WT>` | Builder's own task worktree (or project folder, for a non-Git project) |
| `<REVIEWER_WT>` | Reviewer's separate worktree, detached until an exact intake is copied |
| `<SHARED>` | Task shared home beneath the project root, holding `TASK.md` and both status records |
| `<HANDOFF>` | Task handoff folder beneath the project root for the final ZIPs |
| `<TASK>` | Task ID |

## One-step pair launch (default)

1. **Role launchers.** The architect writes `<TASK_DIR>\LAUNCH_BUILDER.cmd` and `LAUNCH_REVIEWER.cmd`. Each changes to its worktree, refuses a duplicate start with a lock folder in `<SHARED>`, runs the agent with its initial assignment as the prompt argument, and releases the lock:

   ```bat
   @echo off
   setlocal
   cd /d "<BUILDER_WT>" || exit /b 1
   mkdir "<SHARED>\BUILDER.launch-lock" 2>nul
   if errorlevel 1 (echo Existing BUILDER launch lock. Do not start a duplicate. & pause & exit /b 1)
   call claude.cmd "<BUILDER_ASSIGNMENT>" --model <MODEL> --add-dir "<SHARED>" --add-dir "<HANDOFF>"
   set "agentExit=%errorlevel%"
   rmdir "<SHARED>\BUILDER.launch-lock"
   if not "%agentExit%"=="0" pause
   exit /b %agentExit%
   ```

   The reviewer launcher is the same with `<REVIEWER_WT>`, `REVIEWER.launch-lock` and `call codex.cmd "<REVIEWER_ASSIGNMENT>" --add-dir "<SHARED>" --add-dir "<HANDOFF>"`. A project may also set TEMP and tool caches beneath the task folder. Keep `"`, `%` and `^` out of assignment text; `cmd` and `call` interpret them. Codex treats added directories as writable: add only these task paths, never the whole user profile.

2. **Pair launcher.** `<TASK_DIR>\START_PAIR.cmd` opens one task-named Windows Terminal window, builder left and reviewer right:

   ```bat
   @echo off
   wt.exe -w <TASK> new-tab --title Builder powershell.exe -NoExit -Command "& '%~dp0LAUNCH_BUILDER.cmd'" ; split-pane -V --title Reviewer powershell.exe -NoExit -Command "& '%~dp0LAUNCH_REVIEWER.cmd'"
   ```

   The ` ; ` separates Windows Terminal subcommands. Assignment text stays in the role launchers, never on this line. If Windows Terminal is unavailable, open the two role launchers in separate windows and say why.

3. **Shortcut.** A desktop shortcut named for the task targets `START_PAIR.cmd`. Tell the Operator the one action and the expected result: "Double-click '<name>'. One window opens with the builder on the left and the reviewer on the right, each already reading its assignment." Normal trust and permission prompts stay under the Operator's control; never bypass them.

4. **Verify before handing over.** Run each role launcher once from a scratch copy with stub `claude.cmd` and `codex.cmd` first on `PATH` that only record their arguments, and confirm the full assignment, model and directories arrive intact and the lock is released. This starts no inference. Report visible startup as unverified until the Operator uses the shortcut and the agents' READY entries appear.

### Initial assignment content

Builder: `You are <NAME>, builder. Read <SHARED>\TASK.md in full. Record your identity and complete READY before edits.`

Reviewer: `You are <NAME>, independent reviewer. Read <SHARED>\TASK.md in full. Activate this goal with your native goal tool if available and record the observed state: /goal Complete the independent review and closeout of <TASK> under the shared work order. Record your identity and complete READY before review.`

TASK.md carries the read list (the project's `AGENTS.md`, and `CLAUDE.md` if present; in `docs/agent_workflow/`: `SHARED_AGENT_INSTRUCTIONS.md`, the role sheet, `WINDOWS_EXECUTION_PROFILE.md` and `DOCUMENTATION_FIRST_POLICY.md`), scope, workspaces, limits and any standing grants by source. Each actor's first status entry records requested role/model/effort, the configured selection (the launcher's `--model` or the Codex configuration) and the observed identity, or unknown, plus `$PSVersionTable.PSVersion`, the interpreter and instruction SHA-256 values (`SHARED_AGENT_INSTRUCTIONS.md`, "Records and startup"). A persona name is not a model ID.

## Manual fallback

Use this only when a launcher cannot run, naming the observed limitation. Open Windows PowerShell and run, then give the initial assignment above as the first message:

```powershell
Set-Location '<BUILDER_WT>'; claude.cmd --add-dir '<SHARED>' --add-dir '<HANDOFF>'
```

```powershell
Set-Location '<REVIEWER_WT>'; codex.cmd --cd '<REVIEWER_WT>' --add-dir '<SHARED>' --add-dir '<HANDOFF>'
```

If Codex prints `Error: failed to open daemon process: Access is denied. (os error 5)`, record the error and rerun the same command with `--no-daemon` appended.

## Recovering an ended session (visible, Operator-run)

The duty holder prepares the exact command and the pending action, preferably as a one-click resume launcher like the role launchers; the Operator runs it in a visible window. Use it only for a confirmed ended session, never to start a duplicate writer or to mask a failed launch.

```powershell
Set-Location '<BUILDER_WT>'; claude.cmd --resume '<session-id>' --add-dir '<SHARED>'
```

```powershell
Set-Location '<REVIEWER_WT>'; codex.cmd resume '<session-id>' --cd '<REVIEWER_WT>' --add-dir '<SHARED>'
```

When the session ID is not recorded, do not guess it. `codex.cmd resume --cd '<REVIEWER_WT>' --add-dir '<SHARED>'` (no ID) opens Codex's own session picker, and `claude.cmd --resume` (no ID) opens Claude Code's; the Operator picks the session by its visible title and working directory. Do not use `--last` unless the Operator confirms the most recent session is the intended one.

Automatic wake of an agent whose turn has ended is not provided, with or without a goal. This visible, Operator-run resume is the supported recovery.

## Reviewer goal (Codex `/goal`)

The procedure is in `SHARED_AGENT_INSTRUCTIONS.md` ("Codex goal continuity"); this section only adds the evidence observed on this host. The one-sentence goal travels in the reviewer's initial assignment; the reviewer activates it with a native goal tool when available and records the observed state in `REVIEW_STATUS.md`. A CLI prompt does not itself activate `/goal`. For a resumed session, a matching active goal is reused, and a paused goal is not resumed without the Operator.

**Evidence on this host, with its limits:**
- **Documented:** "Goals are available starting in Codex 0.128.0"; `/goal <objective>` sets a goal, `/goal` shows it, `/goal pause`, `/goal resume` and `/goal clear` manage it (https://developers.openai.com/cookbook/examples/codex/using_goals_in_codex, read 2026-10-03).
- **Installed (2026-10-03):** codex-cli 0.160.0 listed the feature `goals` as stable and enabled (`codex.cmd features list`).
- **Observed (2026-10-16):** in FEDERATION task FED-GOV-CONSOLIDATION-20261016 the reviewer activated its assigned goal with a native goal tool and recorded status `active` (shared `REVIEW_STATUS.md`, R1). Carrying a review duty across turn endings is not yet recorded.

A goal does not wake an ended session, does not keep the builder running, and never replaces the waiting, recovery and blocked-exit rules. If goal support is unavailable, the pair works under those rules and records the limitation. One goal per reviewer session; never a second session for the same role.

## Handoff delivery

Each actor stages its handoff inside its own workspace and delivers one uniquely named ZIP in `<HANDOFF>` (for example `<PROJECT>__<TASK>__<ROLE>__<YYYYMMDDTHHMMSSZ>__HANDOFF.zip`). The ZIP holds a concise Markdown handoff, only the supporting files the recipient needs, and their SHA-256 values. The actor reads it back before reporting the absolute path. It is never a folder of loose files (Operator direction, 2026-10-03). Staging stays beneath the project root; a verified copy in the Operator's Downloads folder is authorized, preferably one combined delivery ZIP holding both role handoffs unchanged (Operator direction, 2026-10-16).
