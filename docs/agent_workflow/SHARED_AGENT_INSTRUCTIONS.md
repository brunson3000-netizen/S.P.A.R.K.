# Shared task cooperation

Complete the assigned narrow task through implementation, independent review and repair. Follow project instructions and existing authority. KEEP does not authorize merge, publication, installation, spending or other reserved actions.

## Roles

- Operator: the human who authorizes the task and holds final authority and reserved decisions.
- Architect: the team lead who defines assignments, clarifies scope within the Operator's delegation and receives escalations.
- Coordinator: the architect unless TASK.md explicitly names another coordinator and its delegated responsibilities.
- Coding agent: implements and validates the assigned work and handles routine local coordination.
- Reviewer: independently examines the assigned work and evidence; does not implement the fixes it reviews.

## Deterministic execution and automation

At the start of a task or problem, check the relevant known procedures, rules, scripts and tools once. Reuse an applicable verified mechanism and valid evidence. Use reasoning to select and assess mechanisms, resolve ambiguity within granted discretion, and address genuine exceptions. Automate routine work where practical; preserve recurring validated solutions when their reuse justifies upkeep and the work is authorized. Choose the simplest sufficient approach. Availability is not permission, and an automated wrapper does not change a tool's authority or spending requirements.

Immediately notify the architect when the applicable procedure and defined recovery leave an issue unresolved, or when available capabilities or authority cannot resolve it. State the issue, evidence, affected work, attempted remedies and needed decision or assistance. Use the actual escalation channel; distinguish a saved report from delivery and acknowledgement. Continue unaffected authorized work. Reserved decisions remain with the Operator. Do not silently wait, improvise repeatedly, bypass a safeguard or expand scope.

## Records and startup

Use one absolute shared folder, at the location the project's conventions name, containing:

- TASK.md: objective, scope, acceptance criteria, baseline, assignments, workspace paths and actual limits. The coder may record the architect's assignment without changing its meaning. Label its provenance: an Operator quotation, an architect's instruction under delegated authority, or an agent's recommendation.
- IMPLEMENTATION_STATUS.md: written only by the coding agent.
- REVIEW_STATUS.md: written only by the reviewer.

TASK.md names the available escalation channel to the coordinator, including Operator relay when direct contact is unavailable. A responsible owner is the person assigned to resolve a dependency; if none is known, the architect identifies the next actor without assigning unauthorized work.

Record scope rulings and escalation dispositions in a Decisions section of TASK.md. TASK.md has one named writer at a time: the architect when it has access, otherwise the actor TASK.md names, transcribing attributed instructions without changing their meaning. Record any setup-to-execution custody transfer. Each actor writes only its own status record. Send the escalation through the named available channel; a file update alone is not delivery.

Before launch, the architect gives both actors the same current assignment and exact shared TASK.md location, and names who prepares any missing scaffolding. For a simultaneous start, the reviewer may faithfully prepare the architect's supplied coordination material while the builder grounds; this authorizes no implementation, scope change or writing of another actor's acknowledgment. Label a planned path as planned until it exists.

Reuse equivalent existing files and name them in TASK.md. All agents read the same shared records, not worktree copies. Preserve prior rounds; timestamp updates and state the next actor/action. Preserve unrelated work; use separate worktrees before concurrent repository writes and isolate test outputs where they could conflict.

Prove shared access: coder posts READY with a unique token; reviewer replies with that token and its own; coder acknowledges. In its first entry each actor records the Operator's requested role, model and effort, the actual configured selection, and its observed session identity and model, or unknown; a persona name is not a model ID, and a requested model is never silently substituted. It also records its workspace (path, branch or detached HEAD, baseline) and the SHA-256 of each instruction file it read; a path mention is not proof that a file loaded. Record available wait, session-status and resume tools. Writing a file does not wake an ended session.

## Handoffs

Identify every request, acknowledgement and verdict by task, round and exact checkpoint; each task/round pair identifies one request. Include evidence and the next actor/action. Use a full commit for a clean artifact or an immutable snapshot/manifest covering all relevant changes. Where the task assigns execution units that need independent acceptance, record each unit's executor and reviewer; a reviewer who executes or repairs a unit cannot give its independent acceptance, so the changed unit goes to an eligible independent reviewer.

Allow one outstanding review request. Freeze its artifact during review. Before modifying relevant content, explicitly withdraw the outstanding request in the implementation record and notify the reviewer through the available channel. Submit the completed repair as one new checkpoint and round; do not repeatedly interrupt review with incremental edits. Any relevant change supersedes the previous request. Ignore stale or duplicate handoffs; never transfer KEEP to changed material. A REVISE keeps the repair duty open; ending a turn does not discharge it. After the next owner acknowledges a completed milestone, polling of that request ends.

Coder implements and requests review → reviewer records REVISE or KEEP → coder repairs and resubmits, or completes required final validation and authorized checkpointing, then records matching KEEP under the completion rule. A failed gate blocks only the transition that requires it. If the artifact is unchanged, record a gate-status update against the existing checkpoint and round, with evidence and the blocked transition; this is not a new review request. Preserve an existing artifact KEEP unless new evidence invalidates its findings. Never present artifact KEEP as passage of the failed gate or as task completion. A relevant artifact repair requires a new review round. It does not authorize diagnosis or repair of unrelated failures; continue unaffected authorized work.

## Evidence and validation

Validation covers the authorized changed behavior and its directly affected connections. A repository-wide run is required only when an applicable authorized task or release requirement specifically requires it; if that requirement conflicts with the Operator's scope or testing limit, report the conflict before running it.

Reuse passing evidence when the relevant implementation, test inputs and execution conditions remain applicable. A new commit identifier, records-only change, or desire for additional confidence is not by itself a reason to rerun. Repeat only the failed check after its repair or a check invalidated by a specific relevant change; a demonstrated intermittent defect can justify targeted repetition. Broad repetition follows the Operator's existing approval rule. Keep active edits out of a validation checkout. A rerun that tooling mechanically forces is disclosed as a tooling conflict, not presented as new engineering need.

Each review request names the changed behavior, the evidence it reuses and the checks that change invalidates. Correct a failed invocation or harness cause before rerunning. Coordinate expensive checks that contend for the same desktop, storage or host, and keep tested outputs and live user previews separate from changing build output.

Committing exactly the reviewed content, or adding records-only updates that do not alter governed behavior, acceptance evidence or the reviewed artifact, does not require renewed substantive review. Record the correspondence between the reviewed snapshot/manifest and final commit. Preserve accurate identity and hash bindings required by applicable tooling; an identity update is not a reason to repeat the content review or tests. If records themselves are the reviewed deliverable or affect a decision or gate, assess their actual change rather than exempting them by filename.

Integration and troubleshooting follow the documentation-first policy (`DOCUMENTATION_FIRST_POLICY.md` in this folder, or the project's own copy of it). Keep its reference map and the exact error, version, cause, remedy and open questions in the task records and handoff. Record each duty outcome as completed, cancelled, failed, interrupted or uncertain; dependent work proceeds only on evidence that its completion conditions were met.

## Findings and disagreements

A blocking finding must identify an authorized in-scope requirement and a demonstrated violation: a reproduction, a concrete code path or another verifiable demonstration. A runnable reproduction is useful but not mandatory. Reviewer requests do not expand the assignment. Agents must not add acceptance conditions beyond existing authorized requirements without new Operator direction. Distinguish optional improvements from defects; identify required checks that could not be completed rather than treating uncertainty as proof of success.

Separate a correct reviewed change from a failed external acceptance gate. For an unrelated failure, use sufficient existing evidence, preserve the limitation, and refer its disposition to the responsible owner through the architect; do not require exhaustive proof of non-causation. Further diagnosis must resolve a specific dependency of this task within its existing authority. Unrelated defects do not expand this task. If a defect prevents an acceptance criterion from being met, escalate that dependency instead of dismissing it as out of scope. A genuine regression or a required unresolved gate still blocks the affected claim; uncertainty is never relabeled success.

If a finding remains disputed after a response and reconsideration with no new evidence, record both positions and refer it to the architect. Continue unaffected work; do not repeat the same argument or repair blindly. The architect clarifies scope or arranges the needed evidence within existing authority and records the ruling in TASK.md Decisions. Its scope ruling binds both agents within the Operator's delegation; the reviewer must reassess findings against that scope and remove out-of-scope demands as blockers. The reviewer retains independent judgment on whether the in-scope requirements are met. The architect cannot compel KEEP, waive requirements or override the Operator. A dispute about the ruling's authority goes to the Operator. An unresolved in-scope blocker or authority dispute follows the blocked-work exit below; do not repeat the same review loop.

## Waiting and recovery

At startup, TASK.md names the authorized local record-read and wait mechanism, available session/resume controls, applicable deadlines and retry limits, and the escalation channel. Reuse a recorded mechanism while it remains applicable. Record capability gaps immediately; do not assume that an available tool is authorized or that a file update can wake an ended session.

After a handoff, read the peer's current record and close the read handle. Detect an update by changed bytes, such as a SHA-256, rather than a heading pattern. Start each own entry on a new line with a `## ` heading and a clock-read UTC time. Act on an actionable response. Otherwise use the named local wait for 30 seconds, then read again. A native local PowerShell session can use `Start-Sleep -Seconds 30`; this instruction does not authorize invoking a remote or metered wrapper to execute it. Keep only one wait/check sequence per actor, release file handles before waiting, and stop checking a request once its required acknowledgement or disposition is recorded. The reviewer waits for the coder's matching KEEP only while the successful-completion procedure remains applicable.

After ten unchanged checks, request peer status once through the authorized channel and inspect available session evidence. Before sending any status request or alert, re-read the current request and disposition and retire a superseded alert. Five minutes is a coordination checkpoint, not a new work deadline. Continue only while waiting is supported and authorized, there is no established blocker, and actual mission/channel limits permit it. An expected slow response alone is not failure. Do not repeatedly send the same alert or invent a deadline; recorded limits and Operator stops govern.

If the peer is confirmed stopped, use an already-authorized supported resume control with the pending action; do not create a duplicate worker. Silence alone does not prove the peer stopped. Yield to background notification only where reactivation has been observed and authorized. Never use extra sessions, hooks, timers, proxy calls or artificial activity to evade a limit.

An unavailable authorized wait, failed defined recovery, confirmed unrecoverable interruption, or capability/authority limit triggers immediate notification under Deterministic execution and automation and the Blocked-work exit below. Do not wait for ten checks when that condition is already known. Preserve the pending action; when a human interface action is unavoidable, supply one exact instruction and destination.

On restart, read TASK.md and both status records, confirm the continuing assignment and resume the recorded next action using valid evidence. A replacement needs an actual assignment and stopped or isolated predecessor writes. Conflicting next-action records go immediately to the architect. Do not repeat a failed method without a relevant correction or a defined bounded retry.

## Blocked-work exit

For a known blocker that cannot be resolved within the current authority or an unresolved dispute requiring a ruling, record BLOCKED in your own status record with the checkpoint, evidence, affected work, pending action, next actor and resume condition. Notify the coordinator through the named channel, using Operator relay if necessary. Continue unaffected authorized work; once none remains actionable, each agent records its own cleanup and ends its session. Do not wait for matching KEEP, the other agent's cleanup or a future ruling in order to exit blocked work. Never write the other agent's status or imply that its cleanup is confirmed. BLOCKED is a stopped state, not successful completion; preserve existing verdicts and evidence and do not record CLOSEOUT_COMPLETE.

This explicit exit takes precedence over the waiting and successful-completion loops. A slow or silent peer alone is not this blocker; peer waiting, polling and recovery above remain unchanged. The duty holder owns peer-status checking, request delivery, acknowledgment tracking and recovery within the grant. The Operator performs only a necessary physical/interface action agents cannot perform, or makes a reserved decision. The duty holder supplies the complete action, records its outcome and resumes coordination. Resume affected work only when the recorded blocker is resolved or authorized direction changes it; read the decision and status records and reuse valid evidence.

## Completion

For successful completion:

1. After reviewer KEEP, the coder finishes required validation using applicable existing evidence and completes authorized checkpointing before recording its own KEEP. If checkpointing is not authorized, retain the reviewed immutable snapshot/manifest and report that reserved action separately. Authorized integration ends with the approved result in the project's durable repository home (on this host `D:\Projects\<Project>\<Project>.git` where that layout is verified), checked there by exact revision and by reconstruction without the C: workspace; a GitHub push is additional, not a substitute. C: checkouts, worktrees and task folders are disposable working children, never the permanent repository.
2. If checkpointing changes the identifier without changing reviewed content, the coder records the correspondence and evidence of equivalence, such as matching relevant content hashes or a content diff. The reviewer confirms that equivalence in its own record before cleanup. This confirms identity; it is not a new substantive review round or reason to rerun tests. Preserve any exact candidate, ancestry and hash bindings required by applicable tooling; content equivalence does not waive them.
3. Both agents record KEEP for the same round and final checkpoint, or retain the original reviewer KEEP with its explicit equivalence confirmation linking it to the coder's final checkpoint. A relevant content change requires a new review round under the shared review-reuse rule.
4. Each agent stops and verifies only its own task-created processes or wakeups and records CLEAN, or CLEAN — none created. Alongside CLEAN it states the disposition of its task-created storage: unique work preserved, retained outputs with purpose and removal condition, and disposable output removed or still pending, using the project's existing preservation checks. Unknown, ignored or untracked material is not automatically disposable, and age alone never permits deletion. The Operator may delete the C: Projects folder at any time, so a workspace is called disposable only after its unique work is in the durable home or the exact exception is named; caches and build debris are removed, not copied there. Leave unrelated resources alone; late callbacks must respect completion and exit.
5. Before reporting shared completion, each agent reads the current peer records and retires its superseded requests and alerts; file placement, process existence and requested launch state do not prove receipt, active goal state or completed work. The reviewer may finish after its CLEAN. The coder uses the same waiting/recovery rules until both CLEAN records exist, then records CLOSEOUT_COMPLETE with the final checkpoint, both verdicts, any equivalence confirmation, validation, cleanup and remaining limitations or approvals. Distinguish reviewed, committed, published and installed states.
6. Before ending a completed or blocked task, the builder and adversarial reviewer must each deliver their own handoff as one ZIP staged in the task's `handoff` folder beneath the project root (the folder TASK.md names, for example `<project root>\.local\<TASK>\handoff\`); never a folder of loose files. A verified copy in the Operator's Downloads folder is authorized; one combined delivery ZIP holding both role handoffs unchanged, each with its author's verification, is preferred. Downloads is never a repository home. The ZIP holds a concise Markdown handoff, only the supporting files the recipient needs, and their SHA-256 values. Include conclusions and verdict, the exact checkpoint, a concise evidence summary, unresolved questions or blockers, and the next actor/action. State ‘None’ when there are no unresolved questions. Use a unique filename containing the task ID, agent role and timestamp. Report the absolute file path to the Operator. A shared status record alone does not satisfy this delivery requirement. Read the ZIP back and verify its members before reporting it. If delivery fails, report the failure and the preserved file’s location rather than claiming delivery.

A known blocker follows the blocked-work exit instead of this successful-completion sequence.

## Codex goal continuity — Luna

Revision: 2026-10-16. Use a task-scoped Codex goal for Luna's cooperative review when supported. This supplements Waiting and recovery; it changes no role, authority, evidence or completion rule. The Operator has reported that this workflow improves continuity.

### Activate and record

The goal is one sentence. The architect fills in the task ID and puts it in Luna's initial assignment, with the exact shared TASK.md path beside it rather than inside the sentence. Scope, waiting, review and completion duties stay in TASK.md and this file.

```text
/goal Complete the independent review and closeout of <TASK_ID> under the shared work order.
```

Luna activates the goal with a supported native goal tool when available. Receiving the text in a launch prompt or file is not activation; do not claim a CLI prompt activated `/goal`. Record the task, shared path, goal text and observed activation state in REVIEW_STATUS.md. A matching active goal needs no replacement. Missing goal support does not block otherwise authorized work and needs no routine manual paste at startup: retain the existing waiting/recovery procedure and disclose the limitation.

### Coordinate without unnecessary relay

Both agents still check and acknowledge peer records. Luna's goal cannot guarantee that Opus is active. Check available session evidence before diagnosing a stopped peer; silence alone is insufficient. Do not create overlapping pollers, artificial activity, repeated alerts or unnecessary tests to keep a session alive. The named authorized local wait followed by a fresh record check is sufficient while waiting is appropriate.

### Finish or stop honestly

An intermediate verdict is not successful reviewer completion. Apply Completion, including any equivalence confirmation, own CLEAN record and own one-ZIP handoff beneath the project root. The reviewer need not wait for the coder's later CLOSEOUT_COMPLETE after satisfying her own completion obligations.

Blocked-work exit overrides waiting and goal continuation. A failed external gate may preserve artifact KEEP while blocking the next transition. Once no unaffected authorized work remains, write the blocked handoff and exit; do not wait indefinitely for a ruling or commit that cannot currently happen. Do not mark blocked work successful to terminate a goal. If an active goal remains after a blocked exit, report the state and the exact pause action needed through the available channel; never invent a completed or paused state.

For supported Codex builds, `/goal` displays the current goal; `/goal pause`, `/goal resume` and `/goal clear` manage it. Respect user/system control of those transitions. Budget limits and interruptions are not successful completion and must not be bypassed by timers, hooks, new sessions or renewed goals. Resume only under the existing recovery and authority rules, then reread the shared records. Do not treat a goal as a cross-session wake guarantee.

Official command reference, checked 2026-10-03: https://developers.openai.com/cookbook/examples/codex/using_goals_in_codex ("Goals are available starting in Codex 0.128.0"). The evidence observed on this Windows host is recorded in the launch briefs.
