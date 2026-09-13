# RTK Intake — High-Priority Tool Acquisition

Status: `PINNED_SOURCE_ONLY`
Activation: `NONE`

Resolved upstream: `rtk-ai/rtk` (Rust Token Killer), not the unrelated Rust Type Kit project.

Purpose: evaluate RTK as a deterministic shell-output compression layer for coding agents. RTK is a Rust CLI proxy that filters/compresses common development-command output before it reaches an LLM context. This directly maps to SPARK's context-compression and tool-output-efficiency lane.

Upstream branch at intake: `develop`
Pinned commit: `d0c2985155568d1d76fca03bc65d5098f136bbcd`
Repository: `https://github.com/rtk-ai/rtk.git`

Boundary:
- source custody and qualification only;
- do not run `rtk init`, install hooks, rewrite agent commands, or activate telemetry/configuration as part of source acquisition;
- any hook-based interception requires a separate bounded experiment and explicit activation decision;
- preserve unfiltered/raw-output escape paths for verification and failure diagnosis.

Qualification questions:
1. What transformations are deterministic and loss-bounded?
2. Can compression hide warnings, changed-file details, failing-test context, security findings, or review evidence?
3. What commands are passed through unchanged?
4. Can SPARK invoke RTK explicitly rather than globally installing rewrite hooks?
5. What measurable token/context reduction does it achieve on SPARK's actual Rust/Git/test workloads?
6. Can raw and compressed outputs be retained together for audit/replay?

Disposition at intake: `HIGH_PRIORITY__QUALIFY_BEFORE_ACTIVATION`.
