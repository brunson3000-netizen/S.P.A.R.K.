# S.P.A.R.K. Phase 2 — Operator's next authorized step after independent KEEP

**Date:** 2026-09-12. **Recorded by:** the separated Gate C2 writer (Claude Code, Opus 5,
`claude-opus-5`). The recorder adds no semantics; the step below is the Operator's.
**Status:** a forward-looking record of authorization. It is preserved as history. It
accepts no gate, promotes no production and begins no Phase-3 work.

## 1. The step

**After an independent KEEP verdict on the C2W-01 correction candidate, the next authorized
step is an adversarial test campaign against the pinned build.**

"Pinned build" means the exact candidate commit the KEEP verdict names, verified against
live GitHub — not a branch tip, not a rebuild from a moving branch, and not a local
working tree.

## 2. Who does what

| Role | May | May not |
|---|---|---|
| **Independent testers** | Attack the pinned build adversarially. Record **reproducible** bugs with the exact commit, the minimal reproduction, observed versus expected behavior, and captured evidence. Write disposable probes, harnesses, fixtures and mutants in their own workspaces. Publish findings and evidence on their own branches. | **Repair product code.** They do not edit `crates/`, do not "fix while they are in there", and do not open corrective candidates. A tester who believes they know the fix records the diagnosis as evidence, not as a change. |
| **The main engineer** | Receive the recorded bugs and evidence, triage them, and implement corrections under the ordinary writer discipline: a recorded decision or mission first, a bounded candidate, validation, evidence, and separated independent review. | Act as their own independent reviewer or tester for their own corrections. |

This preserves the separation the whole Gate C2 series has relied on. A tester who repairs
product code stops being an independent observer of it, and the resulting evidence can no
longer be trusted as independent.

## 3. What a recorded bug must contain

So that a finding is actionable without re-deriving it:

1. the exact pinned commit, verified against live GitHub;
2. a minimal, deterministic reproduction — preferably through the public door (commands,
   scheduled work, advance), in a disposable external crate;
3. observed behavior versus the behavior required, with the controlling authority cited by
   document and section (adjudication D-1 … D-7, the Operator resolution's five behaviors,
   FINAL §4, the oracle rows, and so on);
4. captured evidence: exact commands, exit codes and complete logs;
5. an honest severity and scope statement, including whether the finding blocks acceptance
   or is a disclosed limitation.

A finding that cannot be reproduced is recorded as unreproduced, not as a defect.

## 4. Ordering and limits

The campaign follows the KEEP verdict; it does not replace it, and it does not run against
an unreviewed candidate. Nothing in this record authorizes Gate C2 acceptance, production
promotion, Phase-3 work, G.A.M.E. changes, merging research runtimes, or paid external
compute. Acceptance remains the Operator's decision, taken after the independent review and
informed by whatever the campaign finds.
