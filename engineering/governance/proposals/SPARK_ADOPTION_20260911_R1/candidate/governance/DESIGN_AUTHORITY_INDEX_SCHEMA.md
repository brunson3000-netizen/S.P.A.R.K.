# Design Authority Index Schema

Governed by [Project Agent Instructions](AGENT_RULES.md).

## D — Referential records

**D.1** An index row refers to a project decision; its authority derives only from the cited source under C2.5.

**D.2** The indexing agent records the status explicitly established by Operator action, its unambiguous effect, or a valid in-scope choice under D.10.

**D.3** Ambiguous Operator input is preserved verbatim outside authoritative rows, separately from agent proposals.

**D.4** Agent-originated rows outside D.10 use PROPOSED or PROVISIONAL-AGENT.

**D.5** A row may record a reserved technology or protected invariant only when its source establishes that reservation or invariant.

**D.6** The index contains project properties and expressly selected or reserved technologies; engineering deliberation belongs in a TDR.

**D.7** Missing rows do not revoke decisions; the agent grounds under I7 before treating a property as unspecified.

**D.8** Unspecified properties outside a valid C5.11 scope follow I2 and I4; unreserved engineering choices remain agent-owned.

**D.9** A scope of property discretion records its Operator source, applicability, permitted choices, bounds, and exclusions in the Design/Decision Home.

**D.10** An agent records a material choice within a valid scope as SETTLED, citing the scope's Operator source, the selected value, and evidence that the choice falls within the scope.

**D.11** A choice whose coverage by a scope is uncertain is outside that scope.

**D.12** Agents store decision rows and consult the index at the A2 locations, using A2 record identifiers; index omissions do not revoke source-established decisions.

## Status vocabulary

| Status | Meaning |
|---|---|
| SETTLED | Operator established the property or selection, directly or through a valid C5.11 scope |
| RESERVED | Operator expressly retained the decision |
| DEFERRED | The Operator postponed a decision or its implementation; the source states which and what, if anything, is committed |
| PROVISIONAL-AGENT | Isolated representation of an unspecified property |
| PROPOSED | Agent suggestion with no authority |
| SUPERSEDED | Operator replaced the decision; identify its replacement |

## Row fields

| Field | Content |
|---|---|
| ID | Stable DAI identifier |
| Kind | PROPERTY or TECHNOLOGY_DIRECTION |
| Property | Required property, selected technology, or reserved choice |
| Status | Value from the table above |
| Scope | Applicable system and conditions |
| Source | Exact record and clause or preserved wording; required for authoritative status |
| Indexed by / date | Recorder identity and recording date |
| Notes | Limits, dependencies, and what must not be foreclosed |
| Replacement | Source/row replacing a SUPERSEDED decision |

Provisional rows additionally record containment, reversibility, reassessment date, review priority, and hardening condition. These rows form the provisional review queue.
