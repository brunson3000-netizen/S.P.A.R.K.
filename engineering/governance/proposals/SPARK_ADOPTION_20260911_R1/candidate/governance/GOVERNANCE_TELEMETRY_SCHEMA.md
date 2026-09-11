# Governance Telemetry Schema

Governed by [Project Agent Instructions](AGENT_RULES.md) I10.3.

**G.1** The acting agent records every actual Operator escalation, work hold, and C8 failure.

**G.2** The acting agent records event facts and marks each failure SUSPECTED or ACKNOWLEDGED; the Operator or a reviewer records the assessment.

**G.3** The reviewer assesses event sets at a milestone or weekly review rather than deriving an effectiveness claim from one entry.

**G.4** Telemetry is observational; it creates no authority, policy, recovery rule, or automatic decision.

**G.5** One issue record carries all event types for its issue in chronological order.

**G.6** A record inherits unchanged mission metadata by reference and fills only applicable fields; issue ID, event type and time, trigger, affected work, and evidence are always required.

**G.7** JSONL updates append under the same issue ID and retain the original facts and outcomes.

**G.8** A branch resolved before escalation is not counted as an Operator decision; a hold that preceded the resolution remains recorded.

**G.9** The duty holder serializes appends to its mission log; each append has a distinct event ID and retains its issue ID across updates. Workers supply event facts through their own evidence records; aggregation deduplicates event IDs and counts issues under G.5–G.8.

## JSON event schema

Each line is one UTF-8 JSON object conforming to the schema below. Timestamps include a UTC offset. A `FAILURE` event additionally includes `failure_classes`, `failure_status`, `failure_action` and `failure_consequence`. Omitted optional fields mean not recorded, never false or zero. Unchanged grant and governance metadata may be inherited from the cited mission record. A later append updates an issue's recorded state without deleting earlier facts; merge fields in append order, with later values replacing earlier values for the same field. Summaries retain distinct event-type and failure-class coverage across updates and use the latest assessment per issue.

```json
{
  "type": "object",
  "additionalProperties": false,
  "required": [
    "schema_version",
    "event_id",
    "issue_id",
    "mission_id",
    "actor",
    "event_time",
    "types",
    "trigger",
    "affected_work",
    "evidence"
  ],
  "properties": {
    "schema_version": {
      "enum": [
        "1.0"
      ]
    },
    "event_id": {
      "type": "string",
      "pattern": "^EVT-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
    },
    "issue_id": {
      "type": "string",
      "pattern": "^ISS-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
    },
    "mission_id": {
      "type": "string",
      "pattern": "^[A-Za-z0-9][A-Za-z0-9._-]*$"
    },
    "actor": {
      "type": "string",
      "minLength": 1
    },
    "event_time": {
      "type": "string",
      "format": "date-time"
    },
    "types": {
      "type": "array",
      "minItems": 1,
      "uniqueItems": true,
      "items": {
        "enum": [
          "OPERATOR_DECISION",
          "HOLD",
          "FAILURE"
        ]
      }
    },
    "trigger": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "description",
        "clause"
      ],
      "properties": {
        "description": {
          "type": "string",
          "minLength": 1
        },
        "clause": {
          "type": "string",
          "minLength": 1
        }
      }
    },
    "affected_work": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "string",
        "minLength": 1
      }
    },
    "evidence": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "string",
        "minLength": 1
      }
    },
    "grant_source": {
      "type": "string",
      "minLength": 1
    },
    "governance_identity": {
      "type": "string",
      "minLength": 1
    },
    "failure_classes": {
      "type": "array",
      "minItems": 1,
      "uniqueItems": true,
      "items": {
        "enum": [
          "AUTHORITY",
          "AUTONOMY",
          "PROGRESS",
          "COORDINATION"
        ]
      }
    },
    "failure_status": {
      "enum": [
        "SUSPECTED",
        "ACKNOWLEDGED"
      ]
    },
    "failure_action": {
      "type": "string",
      "minLength": 1
    },
    "failure_consequence": {
      "type": "string",
      "minLength": 1
    },
    "branch": {
      "enum": [
        "PROJECT_PROPERTY",
        "RESERVED_AUTHORITY",
        "NOT_APPLICABLE"
      ]
    },
    "work_continued": {
      "type": "array",
      "items": {
        "type": "string",
        "minLength": 1
      }
    },
    "dai_refs": {
      "type": "array",
      "items": {
        "type": "string",
        "minLength": 1
      }
    },
    "tdr_refs": {
      "type": "array",
      "items": {
        "type": "string",
        "minLength": 1
      }
    },
    "boundary_reached": {
      "type": "boolean"
    },
    "branch_reason": {
      "enum": [
        "MISSING_AUTHORITY",
        "UNRESOLVED_INTERPRETATION",
        "UNMET_REQUIRED_ASSURANCE",
        "NOT_APPLICABLE"
      ]
    },
    "resolver_checked": {
      "type": "boolean"
    },
    "resolver_applicable": {
      "type": "boolean"
    },
    "resolver_source": {
      "type": "string",
      "minLength": 1
    },
    "resolver_result": {
      "type": "string",
      "minLength": 1
    },
    "assurance_required": {
      "type": "boolean"
    },
    "assurance_obtained": {
      "type": "boolean"
    },
    "assurance_evidence": {
      "type": "array",
      "items": {
        "type": "string",
        "minLength": 1
      }
    },
    "consultation_refs": {
      "type": "array",
      "items": {
        "type": "string",
        "minLength": 1
      }
    },
    "dispensation_ref": {
      "type": "string",
      "minLength": 1
    },
    "operator_decision_source": {
      "type": "string",
      "minLength": 1
    },
    "captured_path": {
      "type": "string",
      "minLength": 1
    },
    "recurring_signature": {
      "type": "string",
      "minLength": 1
    },
    "assessment": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "assessor",
        "assessed_at",
        "evidence",
        "results"
      ],
      "properties": {
        "assessor": {
          "type": "string",
          "minLength": 1
        },
        "assessed_at": {
          "type": "string",
          "format": "date-time"
        },
        "evidence": {
          "type": "array",
          "minItems": 1,
          "items": {
            "type": "string",
            "minLength": 1
          }
        },
        "results": {
          "type": "array",
          "minItems": 1,
          "items": {
            "enum": [
              "ESCALATION_JUSTIFIED",
              "AGENT_COULD_HAVE_RESOLVED",
              "STEWARDSHIP_PERMISSION",
              "COORDINATION_TRANSFER",
              "ASKED_FOR_IMPLEMENTATION",
              "WRONG_BRANCH",
              "PREMATURE_BRANCH",
              "INSUFFICIENT_INVESTIGATION",
              "ANSWER_WAS_IN_RECORDS",
              "HOLD_JUSTIFIED",
              "COULD_HAVE_BEEN_DECOUPLED",
              "HOLD_TOO_BROAD",
              "INVALID_HOLD",
              "SHOULD_HAVE_ESCALATED",
              "FAILURE_CONFIRMED",
              "FAILURE_NOT_SUBSTANTIATED",
              "FAILURE_UNRESOLVED"
            ]
          }
        }
      }
    }
  }
}
```

## Assessment vocabulary

| Escalation assessment | Meaning |
|---|---|
| ESCALATION_JUSTIFIED | A governing decision or missing property required the Operator |
| AGENT_COULD_HAVE_RESOLVED | Granted discretion was transferred upward |
| STEWARDSHIP_PERMISSION | Permission was sought for already-authorized cleanup |
| COORDINATION_TRANSFER | Agent coordination work was transferred upward |
| ASKED_FOR_IMPLEMENTATION | An implementation choice was asked instead of a missing property |
| WRONG_BRANCH | The question confused authority and project property |
| PREMATURE_BRANCH | Ordinary judgment or an authorized resolver could resolve it |
| INSUFFICIENT_INVESTIGATION | Infeasibility lacked proportional evidence |
| ANSWER_WAS_IN_RECORDS | Applicable grounding would have supplied the answer |

| Hold assessment | Meaning |
|---|---|
| HOLD_JUSTIFIED | The exact action required the missing answer or assurance |
| COULD_HAVE_BEEN_DECOUPLED | Authorized work could continue through isolation |
| HOLD_TOO_BROAD | The hold exceeded the dependent work |
| INVALID_HOLD | No applicable requirement justified the hold |
| SHOULD_HAVE_ESCALATED | A terminal unresolved branch was left waiting |

| Failure assessment | Meaning |
|---|---|
| FAILURE_CONFIRMED | Evidence establishes the recorded C8 failure class |
| FAILURE_NOT_SUBSTANTIATED | Examined evidence does not establish the reported failure |
| FAILURE_UNRESOLVED | Assessment remains incomplete or inconclusive |

## Analysis fields

Summaries report counts and denominators for justified escalations, justified holds, premature branches, coordination transfers, missing required assurance, resolver checks, answers already in records, and confirmed failures by C8 class. Each issue counts once per event type and failure class; updates are not new events. Suspected, acknowledged, and unassessed failures are shown separately. Authority failures are reported alongside friction measures. Zero denominators are undefined, not success. Repeated signatures are research inputs for later authorized tool or policy work.
