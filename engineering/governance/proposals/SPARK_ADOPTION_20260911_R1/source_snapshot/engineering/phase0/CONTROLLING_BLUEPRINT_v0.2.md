---
title: "S.P.A.R.K. Engineering Takeover and Architecture Blueprint"
subtitle: "System for Propagating Affective Responses and Consequences"
author: "Operator engineering handoff"
date: "2026-08-25"
---

**Version:** 0.2  
**Supersedes:** `SPARK_ENGINEERING_TAKEOVER_BLUEPRINT_v0.1`  
**Status:** Architecture freeze approved; implementation handoff ready  
**Chronicle status:** OFFLINE / SHELVED / not a dependency  
**Production language:** Rust  
**Primary implementation target:** Standalone service plus embeddable deterministic runtime, sharing one versioned integration contract  
**First reference profiles:** Game-world causal/behavioral profile and MCI Agent Commons social sidecar  
**Research status:** Upstream and downstream inventories integrated as source catalogs; runtime vocabulary intentionally narrowed

[[PAGE_BREAK]]

# 0. v0.2 Supersession and Change Summary

This document supersedes v0.1 and is the controlling engineering handoff for S.P.A.R.K.

The principal revision is that the project now has a frozen causal grammar and a clearer authority boundary. S.P.A.R.K. is not merely a generic factor engine; it is a deterministic, scoped causal-circulation engine that converts triggers and host-observed conditions into pressures, actor situations, interpretations, goals, bounded choices, behavior intents, outcomes, aggregate feedback, dynamic dialogue, and AI-inspectable explanations.

The most important v0.2 decisions are:

- Rust is the committed production language.
- The Living Chronicle remains shelved and must not re-enter the dependency chain.
- The host owns physical and economic truth; S.P.A.R.K. owns its causal, social, psychological, relationship, memory, goal, and advisory behavior state.
- The causal grammar is frozen while its vocabulary remains data-driven and replaceable.
- Global and scoped triggers may be natural, supernatural, social, political, economic, military, technological, beneficial, harmful, mixed, or neutral.
- Causal propagation is a graph with valid shortcuts, not a mandatory linear pipeline.
- Detailed local actors, warm named actors, and dormant aggregate scopes use different simulation fidelity.
- Distant settlements and factions continue to evolve through cheap aggregate ticks and scheduled obligations; they do not freeze merely because the player is absent.
- The first full reference slice is the drought -> food/affordability -> household exposure -> actor choice -> hunting -> wildlife decline -> neighboring-settlement feedback loop.
- Dynamic state-driven dialogue trees, relationship-sensitive player responses, voice profiles, TTS adapters, quests, bounties, crises, opportunities, ruler decision points, and the MCI Agent Commons remain explicit product goals.
- The upstream research package supplied 244 environmental/natural candidates and 44 frequency-calibration families. The downstream package supplied 301 behavioral/social candidates, a 50-node high-leverage shortlist, 32 memory classes, 12 archetype packs, and a pseudo-depth register. These are research catalogs, not a 545-factor runtime mandate.

The engineer should implement the smallest coherent runtime that can express the frozen grammar and then populate it gradually through versioned profiles.

# 1. Executive Takeover Directive

S.P.A.R.K. is now ready to move from exploratory design into bounded implementation.

The engineer taking over this work should treat this document as the consolidated source of truth for the S.P.A.R.K. project, subject only to later operator decisions and approved architecture-decision records.

The central directive is:

> Build S.P.A.R.K. as a self-contained, deterministic, AI-inspectable causal-pressure and behavioral-consequence engine. It must accept typed host inputs, schedule and evaluate configurable triggers, propagate scoped conditions through a sparse consequence web, maintain selected actor state, and produce advisory behavioral, social, dialogue, scene, gameplay-hook, and explanation outputs through a versioned API.

The implementation must optimize for:

> **Medium-high systemic complexity, maximum gameplay leverage, rich social behavior, and bounded CPU/storage cost.**

S.P.A.R.K. is deliberately narrower than the archived Maximum Living Chronicle. It is not a universal world substrate, exhaustive history recorder, game engine, economy engine, renderer, quest engine, or runtime LLM.

The first implementation must prove that simple deterministic machinery can produce a complex circulation:

```text
Trigger
-> Condition
-> Pressure
-> Situation
-> Interpretation
-> State
-> Goal
-> Choice
-> Behavior
-> Outcome
-> Feedback
```

The engineer should preserve the rich conceptual structure while keeping the runtime primitives small, typed, versioned, testable, and data-driven.

# 2. Source of Truth, Project Boundary, and Requirement Capture

## 2.1 Project reset

The Maximum Living Chronicle is offline and shelved. It has no implementation authority over S.P.A.R.K. Selected ideas survive only where explicitly adopted here: deterministic state, bounded explanation, selective memory, AI inspection, and causal provenance sufficient to answer practical "why" questions.

## 2.2 Document precedence

Use this precedence order:

1. direct later operator decisions;
2. this blueprint and later S.P.A.R.K. blueprint versions;
3. approved S.P.A.R.K. ADRs and schema specifications;
4. analyzed upstream/downstream research catalogs;
5. related S.W.A.R.M. and game documents where they do not conflict;
6. archived Living Chronicle material as nonbinding reference only.

## 2.3 Requirement status labels

- **FROZEN:** required unless explicitly reopened.
- **PROVISIONAL:** implement behind a replaceable boundary.
- **PROFILE DATA:** editable/tunable without changing Rust when existing semantics suffice.
- **PENDING CALIBRATION:** mechanism exists; final values wait for testing or historical/scientific calibration.
- **FUTURE:** preserve a reasonable seam; do not implement now.
- **OUT OF SCOPE:** do not implement without a new operator decision.

## 2.4 Operator micromanagement boundary

The operator is not required to anticipate every factor, message field, migration edge case, or implied capability.

The engineer must:

- maintain a requirement-coverage matrix tying operator goals to schemas, modules, tests, and acceptance criteria;
- proactively identify capabilities implied by frozen goals;
- map new ideas onto the existing causal grammar before proposing new engine primitives;
- surface conflicts, security effects, cross-platform costs, and irreversible decisions;
- avoid asking the operator to choose routine implementation details that can be decided safely within this blueprint;
- escalate only decisions that materially change gameplay meaning, authority, portability, cost, privacy, or long-term compatibility.

The operator defines desired behavior, meaning, authority, and product boundaries. The engineer owns ordinary technical realization within those boundaries.

# 3. Product Definition

## 3.1 Formal definition

**S.P.A.R.K. - System for Propagating Affective Responses and Consequences** is a deterministic service and embeddable runtime that:

1. receives typed observations, signals, actors, affiliations, and time advancement from a host;
2. evaluates due triggers using configurable eligibility, cadence, frequency, severity, scope, and deterministic RNG;
3. maintains typed/scoped state for world pressures, social systems, actors, directed relationships, selected memories, beliefs, and goals;
4. propagates state changes through bounded declarative rules;
5. evaluates actor-specific situation, belief, appraisal, active goals, constraints, alternatives, power relations, and salience;
6. produces advisory behavior, speech, social-scene, screenplay, gameplay-hook, and ruler-decision intents;
7. consumes host-confirmed outcomes and feeds their consequences back into local, social, institutional, ecological, and aggregate state;
8. exposes state, configuration, health, and bounded explanations through an AI-friendly inspection API.

## 3.2 Operator mental model

The operator's shorthand remains valid:

> S.P.A.R.K. is a condition-generation and propagation engine that rains typed flags, intensities, relationships, beliefs, and pressures onto scoped targets, lets those states influence one another, and converts the resulting combinations into bounded behavioral possibilities.

The implementation must support more than Boolean flags:

- Boolean conditions;
- bounded integers/fixed-point intensities;
- quantities and ratios supplied or owned by the host;
- categorical affiliations and roles;
- timed states with decay, recovery, or expiry;
- directional relationship states;
- sparse selected memories and beliefs;
- active goals and delayed obligations;
- ephemeral appraisals, salience, choice markers, and behavior scores.

## 3.3 Meaning of "affective"

In this project, **affective** means capable of affecting state, interpretation, relationships, goals, behavior, or decision eligibility. Emotional affect is one subset. Environmental pressure, honor judgment, food unaffordability, authority, magical corruption, curiosity, trust, and faction grievance are all affective in this broader engineering sense.

# 4. Primary Use Cases

## 4.1 Game-world profile

S.P.A.R.K. should support:

- human NPCs;
- animals and social animals;
- monsters and intelligent nonhuman societies;
- households, workplaces, guilds, religions, denominations, factions, settlements, regions, kingdoms, and ecological scopes;
- rulers and offices that receive macro-level decision points;
- the player where a game rule explicitly exposes player-affect state.

Its outputs may inform:

- ambient routines and interruptions;
- approach, avoidance, curiosity, requests, cooperation, negotiation, coercion, resistance, aggression, reconciliation, building, and exploration;
- state-driven dynamic dialogue trees;
- reputation, honor, fame, status, trust, obligation, grievance, and loyalty;
- quests, bounties, rescues, investigations, crises, opportunities, rumors, faction responses, and emperor-level decisions;
- monster migration, pack behavior, territorial response, predation, resource seeking, and learned hostility;
- voice performance and TTS requests.

The host game remains authoritative for coordinates, pathfinding, collision, combat, inventories, money, employment contracts, production, prices, taxes, construction, graphics, audio, formal quests, and final action execution.

## 4.2 MCI Agent Commons profile

S.P.A.R.K. may run as a strictly non-authoritative social sidecar to S.W.A.R.M. It can consume selected sanitized telemetry and create fictional persistent social state for agent personas, including familiarity, respect, gratitude, irritation, friendship, rivalry, social energy, scene eligibility, dialogue, and a running office/town screenplay.

No S.P.A.R.K. social state may change real agent assignments, permissions, tools, budgets, governance, model routing, review gates, or execution behavior.

## 4.3 Additional clients

The same contract may support:

- command-line and test clients;
- browser visualization/control surfaces;
- AI inspection and quality-control agents;
- world editors and balancing tools;
- future experimental profiles;
- future game applications using different vocabularies and trigger catalogs.

# 5. Frozen Goals and Non-Goals

## 5.1 Frozen goals

1. Standalone and embeddable operation.
2. Rust production implementation.
3. Windows, Linux, and Android compatibility from project inception.
4. One shared versioned protocol/contract used by game, MCI, browser, CLI, and inspection adapters.
5. Explicit authority ownership for every state definition.
6. Deterministic scheduling and stochasticity within a frozen support envelope.
7. Data-driven trigger, factor, trait, appraisal, behavior, dialogue, probability, cadence, and profile vocabulary.
8. Medium-high behavioral richness from sparse, high-leverage state.
9. Broad rare-event catalogs with low/naturalistic event density.
10. Natural, beneficial, mixed, harmful, and supernatural trigger support.
11. Multi-resolution simulation: detailed local lives, reduced named-actor continuity, cheap distant outcomes.
12. AI-first real-time inspection, explanation, profiling, and quality control.
13. Dynamic state-driven dialogue trees.
14. Backend-neutral persistent voice profiles and future TTS adapters.
15. Advisory gameplay-hook candidates: quests, bounties, crises, opportunities, rumors, investigations, and ruler decisions.
16. Profile isolation between game and MCI social use.
17. Bounded performance, sparse relationships, selected memories, and no all-to-all world updates.
18. Safe schema evolution, stable IDs, and migration support.

## 5.2 Frozen non-goals

1. Maximum Living Chronicle.
2. Exhaustive world-event logging.
3. Universal ontology or arbitrary plugin platform.
4. Full scientific climate, geology, ecology, economics, or psychology.
5. Runtime LLM dependency.
6. Free-text dialogue as a v1 requirement.
7. Dense population-wide relationship or belief graphs.
8. Direct S.P.A.R.K. authority over host actions.
9. A graph database, distributed event bus, cloud service, or federation in v1.
10. Rendering, camera, animation, game physics, or TTS backend implementation inside the core.
11. Game accounting, banking, production, market clearing, or inventory authority.
12. Final trigger catalogs, factor counts, weights, cadences, probabilities, or active-window dimensions frozen before profiling.

# 6. Governing Design Principles

## 6.1 Freeze grammar; edit vocabulary

Rust hard-codes a small semantic spine. Profiles supply the named triggers, factors, traits, religions, denominations, appraisals, behaviors, templates, weights, rates, and packs.

## 6.2 Host owns reality; S.P.A.R.K. owns interpretation and behavioral state

The host owns physical/economic truth. S.P.A.R.K. owns internal causal state, belief, memory, goals, relationships, and advisory intent.

## 6.3 Broad possibility space, low event density

A large catalog of dormant rare triggers is cheap. Eligibility, cadence, susceptibility, recurrence, cooldown, severity distributions, and volatility presets keep the experienced narrative rate natural rather than chaotic.

## 6.4 Causal circulation, not a one-way chain

Global conditions can descend toward individuals. Individual behavior can aggregate sideways and upward into ecological, settlement, factional, regional, or royal consequences, which become new conditions.

## 6.5 Layers are semantic guidance, not mandatory gates

Drought normally travels through physical and resource layers. An insult may begin at perception/appraisal. An assassination may enter at institutional state. Valid shortcuts are allowed; implausible hidden shortcuts are not.

## 6.6 Moderate behavior is the default

Most actors should usually choose ordinary coping, cooperation, avoidance, negotiation, or lawful action. Extreme behavior generally requires stacked pressure, missing alternatives, low inhibition, favorable opportunity, and sufficient expected effectiveness.

## 6.7 Recovery is first-class

Every accumulating state should normally expose decay, mitigation, adaptation, reconciliation, replacement, relief, or recovery pathways.

## 6.8 Salience bounds attention

Actors do not evaluate every memory, goal, relationship, and condition equally on every cycle. Salience determines the small working set relevant to the current situation.

## 6.9 Gameplay leverage over realism trivia

A factor belongs when it changes choices, relationships, outcomes, gameplay, or AI understanding. Low-leverage detail belongs in host simulation, flavor, or nowhere.

## 6.10 Explanation is part of the product

The engine must be able to answer bounded practical questions such as "Why did this actor request aid?" and "Why did wildlife collapse in this region?" without recreating an exhaustive historical Chronicle.

# 7. Authority and State Ownership

Every definition must declare one of three authority modes:

| Authority | Meaning | Examples |
|---|---|---|
| `host_owned` | The host is canonical; S.P.A.R.K. may observe and derive pressure but may not fabricate truth. | money, inventory, position, food stock, wages, price, employment contract, combat injury, wildlife count |
| `spark_owned` | S.P.A.R.K. is canonical for its internal behavioral/social state. | fear, hope, grievance, trust, selected memory, belief, active goal, behavior pressure |
| `derived` | Deterministically computed from declared authoritative inputs; not independently writable. | food insecurity, perceived scarcity candidate, household affordability pressure, expected deterrence |

## 7.1 S.P.A.R.K.-owned concerns

- trigger evaluation where delegated by the profile;
- causal/pressure propagation;
- appraisals and choice scoring;
- internal states, directed relationships, selected memories, beliefs, and goals;
- advisory behavior, speech, scene, screenplay, and gameplay-hook intents;
- persistent voice identities;
- bounded explanation and system telemetry.

## 7.2 Host-owned concerns

- physical world and actor mechanics;
- economy and accounting;
- actual inventories, money, wages, taxes, debt, prices, employment, production, trade, and property;
- action legality and execution;
- quest authority and rewards;
- player telemetry retention;
- rendering, networking, TTS playback, and platform services;
- S.W.A.R.M. governance and real work.

## 7.3 Advisory output rule

A S.P.A.R.K. `BehaviorIntent` is not a command. The host validates feasibility and executes, rejects, defers, or translates it. Host-confirmed outcomes return to S.P.A.R.K. as inputs for feedback.

# 8. Deployment, Language, and Package Boundaries

## 8.1 Production language

Rust is frozen as the production language for the core/runtime.

Reasons include:

- Windows/Linux/Android portability;
- memory safety;
- deterministic and explicit control;
- embeddability;
- efficient headless/service operation;
- strong testing and serialization ecosystems;
- compatibility with AI-assisted engineering;
- a clean path to C ABI, JNI, C#, Python, or WebAssembly adapters where justified.

A CPython-only production core is not acceptable.

## 8.2 Required deployment forms

### Standalone service

Used by MCI/S.W.A.R.M., browser clients, test harnesses, administrative tools, and sidecar deployments.

### Embeddable runtime/library

Used by offline/local game servers, Android builds, low-latency host integration, and environments where another process is undesirable.

Both forms must expose equivalent logical semantics.

## 8.3 Recommended Rust workspace boundaries

```text
spark/
  crates/
    spark-core/            primitives, clock, scheduler, state, rules, RNG
    spark-profile/         manifests, schema loading, validation, migrations
    spark-protocol/        versioned messages and capability contract
    spark-service/         standalone service wrapper
    spark-persistence/     snapshots, local store, migrations
    spark-inspection/      queries, explanations, QC metrics
    spark-expression/      speech/dialogue/screenplay semantics
    spark-voice/           voice profiles and backend adapters
    spark-testkit/         deterministic fixtures and scenario harness
  profiles/
    game-world/
    mci-social/
  adapters/
    cli/
    mci/
    game/
    browser/
```

Exact crate names may vary, but dependency direction must remain clear. Expression, voice, service, and host adapters depend on the core; the core must not depend on them.

# 9. Shared Protocol, Handshake, and Host Adapter

## 9.1 Contract definition

The protocol should be defined independently of S.P.A.R.K.'s internal classes and independently of any host's internals.

Recommended artifacts:

- versioned message schemas;
- OpenAPI or equivalent request/query contract;
- streaming-event schema;
- compatibility policy;
- capability and permission definitions;
- generated client types where practical.

## 9.2 Handshake

The handshake confirms that both sides speak a compatible contract.

Client request should conceptually include:

```text
client_id
client_type
client_instance_id
protocol_version range
requested profile
requested capabilities
authentication credential
optional manifest/config hash
```

S.P.A.R.K. response should conceptually include:

```text
accepted protocol version
session_id
server/core version
active profile and profile version
granted capabilities
configuration manifest hash
feature flags
limits and warnings
```

## 9.3 Message envelope

Every message should include:

- message ID;
- correlation ID;
- session ID;
- profile ID;
- schema version;
- logical timestamp or host timestamp policy;
- sequence or idempotency key where required;
- sender identity;
- payload type;
- payload;
- optional trace/context metadata.

## 9.4 Initial message families

### Inputs

- `register_target`
- `update_target_context`
- `remove_target`
- `ingest_signal`
- `advance_time`
- `set_fidelity_tier`
- `acknowledge_output`

### Queries

- `describe_schema`
- `get_profile_manifest`
- `inspect_target`
- `inspect_factor`
- `inspect_relationship`
- `inspect_memory`
- `explain_current_state`
- `explain_output_candidate`
- `inspect_active_pressures`
- `inspect_system_health`

### Configuration

- `get_config`
- `validate_config_patch`
- `apply_allowed_config_patch`
- `switch_preset`

### Outputs

- `factor_state_change`
- `behavior_intent`
- `speech_intent`
- `social_interaction_candidate`
- `scene_candidate`
- `screenplay_beat`
- `decision_point_candidate`
- `gameplay_hook_candidate`
- `voice_performance_request`
- `warning_or_rejection`

## 9.5 Reference transport

The contract should be transport-neutral.

A reasonable first service transport is:

- local HTTP/JSON for commands and queries;
- WebSocket or server-sent events for pushed outputs;
- local-only binding by default;
- ephemeral authentication token.

An in-process adapter may bypass serialization while conforming to the same logical contract.

## 9.6 Host-adapter behavior

NPCs and other mobiles do not need to become independent API clients. The host adapter batches and submits relevant actor/world context on their behalf. The host may ask S.P.A.R.K. to evaluate one actor, a batch of active actors, a scoped aggregate, or due scheduled work.

The contract should support host callbacks or acknowledgements for:

- behavior accepted, rejected, deferred, or failed;
- actual resource transfer;
- actual movement/combat outcome;
- dialogue option selected;
- player refusal and supplied explanation;
- trigger candidate accepted or rejected;
- aggregate macro update committed.

# 10. Security Constitution

## 10.1 Default posture

- Local-only by default.
- Deny unrecognized clients, profiles, message types, fields, and configuration changes.
- Treat every client message as untrusted.
- No arbitrary code execution or expression evaluation from payloads.
- No direct database access for clients or AI inspectors.

## 10.2 Capability scopes

Candidate scopes include:

```text
signal:write
context:write
state:read
rules:read
explain:read
outputs:subscribe
config:read
config:patch-approved
profile:admin
```

MCI social integration should receive only the capabilities needed to submit sanitized signals, query fictional social state, and consume presentation outputs.

## 10.3 Configuration mutation

MCI or another host may change selected variables through the API only when:

- the field is explicitly mutable;
- the caller possesses the required scope;
- the value passes type, range, and compatibility validation;
- the change creates a new configuration revision;
- changes that alter deterministic outcomes are visible in the manifest;
- security or authority invariants cannot be modified through ordinary configuration.

## 10.4 Hard prohibition for MCI social mode

There must be no API path from the social profile to:

- work assignment;
- agent tool execution;
- credential access;
- model routing;
- economic authority;
- governance decisions;
- file mutation;
- task cancellation;
- review approval.

Enforce this structurally, not by prompt wording.

# 11. Frozen Causal Grammar

## 11.1 Human-readable grammar

```text
Trigger
-> Condition
-> Pressure
-> Situation
-> Interpretation
-> State
-> Goal
-> Choice
-> Behavior
-> Outcome
-> Feedback
```

## 11.2 Engineering ontology

```text
TriggerOccurrence
-> Direct World Condition
-> Resource / Material / Systemic Pressure
-> Economic / Social / Institutional Condition
-> Individual Exposure / Situation
-> Perception / Belief
-> Appraisal
-> Dynamic Internal State
-> Relational State + Selective Memory
-> GoalInstance
-> Choice Context
-> BehaviorIntent
-> EffectBatch / Outcome
-> Aggregation / Feedback
```

Not every causal path visits every class. The consequence web is a typed directed graph. The layer labels aid validation, inspection, authoring, balancing, and explanation; they are not a rigid pipeline.

## 11.3 Minimal runtime primitives

The core should remain smaller than the conceptual ontology:

| Primitive | Role |
|---|---|
| `TriggerOccurrence` | A discrete initiating occurrence or threshold crossing. |
| `StateCell` | A typed value attached to a scope; traits and relationships can specialize this primitive. |
| `BeliefRecord` | What an actor believes, with source and confidence. |
| `MemoryRecord` | Selected behaviorally relevant retained information. |
| `GoalInstance` | An active desired outcome with target, priority, urgency, and lifecycle. |
| `BehaviorIntent` | Advisory semantic action the actor is inclined to attempt. |
| `EffectBatch` | Ordered validated state changes/outcomes. |
| `PropagationRule` | Declarative relationship between inputs, conditions, scope mapping, delay, and effects. |
| `ProfileManifest` | Versioned vocabulary, rules, parameters, packs, and compatibility declarations. |

Appraisals, salience, candidate behaviors, and most choice-context scores should normally be ephemeral computations rather than persisted primitives.

# 12. Common Schema, Definitions, and Lifecycle

## 12.1 Common definition fields

Every profile definition should expose, where applicable:

```text
id                 immutable namespaced identifier
name               editable display name
kind               trigger/state/trait/appraisal/goal/behavior/etc.
domain             organizational domain
layer              semantic layer label
value_type         bool/int/fixed/categorical/reference/set/etc.
authority          host_owned/spark_owned/derived
valid_scopes       permitted target scopes
enabled            active or disabled
version            schema/content behavior version
description        operator/engineer-readable meaning
behavioral_leverage high/medium/low metadata
```

## 12.2 StateCell

A state instance should minimally identify:

```text
definition_id
scope_id
value
baseline/reference
created_at
updated_at
expiry/decay/recovery metadata
salience
source_refs (bounded)
version/behavior_epoch
```

A directed relationship is a state scoped to `(subject_actor, target_actor_or_group)`. A stable trait is a long-lived actor-scoped state. A settlement pressure is a settlement-scoped state.

## 12.3 Stable identifiers

Use immutable namespaced IDs such as:

```text
trigger.weather.drought
trigger.supernatural.blood_moon
state.resource.food_availability
trait.curiosity
relationship.trust
religion.asterian
denomination.asterian.reform
behavior.acquire.steal
```

Display names may change. Persisted IDs may be disabled, deprecated, aliased, migrated, or replaced, but not silently deleted or repurposed.

## 12.4 Definition change classes

### Hot-tunable parameter changes

May be applied through a validated configuration API where safe:

- probabilities and volatility multipliers;
- cadences;
- thresholds;
- bounded weights;
- decay/recovery rates;
- cooldowns;
- severity distributions;
- salience and repetition limits.

### Structural profile changes

Require validation and controlled reload/checkpoint:

- adding/removing/deprecating a trigger;
- adding a trait, appraisal, behavior, religion, denomination, relationship dimension, or profile pack;
- changing rule topology;
- changing scope compatibility;
- adding dialogue templates or voice mappings.

### Engine-semantic changes

Require Rust work and an ADR:

- a genuinely new primitive;
- new authority semantics;
- new deterministic arithmetic model;
- new transaction/commit semantics;
- new protocol capability that cannot be represented by existing messages.

## 12.5 Profile validation invariants

A definition is invalid if it lacks clear meaning, valid scope, ownership, bounds, update semantics, a consumer, test expectations, or a migration path. The validator should warn on duplicate/redundant concepts, dead factors, excessive fan-out, missing recovery, instant cycles, and unsupported host dependencies.

# 13. Trigger Architecture and Direct World Conditions

## 13.1 Trigger origins

A Trigger is something that happens or becomes true and initiates state change. Origins include:

- stochastic;
- threshold;
- causal/derived;
- scheduled;
- actor-caused;
- scripted;
- external host input.

Randomness is one origin mechanism, not the definition of a trigger.

## 13.2 Trigger schema

A trigger definition should support:

```text
identity and category/tags
origin mechanism
eligibility conditions
evaluation cadence
probability/frequency model
random-address namespace
scope mapping
severity model
duration model
recurrence and cooldown
direct effects
related/compound triggers
calibration/evidence metadata
volatility-preset participation
enabled/version status
```

Polarity (`beneficial`, `harmful`, `mixed`, `context-dependent`) is descriptive only. Effects determine actual consequences.

## 13.3 Trigger taxonomy slots

The trigger catalog may include:

- meteorological and climate;
- hydrological;
- geological;
- ecological and biological;
- fire;
- astronomical;
- supernatural;
- economic;
- social;
- political;
- military;
- technological;
- actor-caused and player-caused events.

Supernatural events use the same propagation engine as natural events. Examples include curses, blessings, blood moons, divine apparitions, magical storms, corruption fields, necromantic surges, sacred animal appearances, and dormant creature awakenings.

## 13.4 Scope

Triggers may target:

- actor/entity;
- cell/location;
- structure;
- household;
- workplace/organization;
- settlement;
- watershed/habitat/ecological zone;
- semantic region;
- faction/religion/denomination;
- kingdom/empire;
- continent/world;
- combinations or intersections of these.

Global events are not gated by player proximity.

## 13.5 Direct World Condition domains

Triggers may directly alter physical or supernatural conditions in these provisional domains:

1. Atmosphere and Climate
2. Water and Hydrology
3. Land and Geology
4. Ecology and Habitat
5. Biological Populations
6. Fire
7. Built Environment
8. Supernatural Environment

These are profile taxonomy slots, not hard-coded lists of all possible values.

# 14. Causal Layer Taxonomy

## 14.1 Resource / Material / Systemic bridge domains

These twelve channels preserve distinctions that produce different stories while remaining computationally cheap taxonomy labels:

| # | Domain | Purpose |
|---|---|---|
| 1 | Food & Nutrition | Food availability, dietary adequacy, crop/livestock/wild-food sufficiency. |
| 2 | Water & Sanitation | Potable water, contamination, hygiene and sanitation capacity. |
| 3 | Shelter & Habitability | Housing, warmth/cooling, exposure protection, displacement. |
| 4 | Materials & Energy | Timber, stone, ore, fuel, tools/input materials, optional magical resources. |
| 5 | Ecology & Wild Resources | Wildlife, fish, forage, habitat quality, carrying capacity. |
| 6 | Infrastructure & Utilities | Roads, bridges, wells, irrigation, mills, defenses, public works. |
| 7 | Mobility, Access & Logistics | Transport, market reach, delivery capacity, isolation, route access. |
| 8 | Labor, Skills & Production Capacity | Workforce, specialists, tools, occupational bottlenecks, productive capacity. |
| 9 | Health & Biological Burden | Disease, injury, parasites, malnutrition effects, mortality pressure. |
| 10 | Safety, Threat & Security | Crime, monsters, predation, raids, war, public safety. |
| 11 | Population, Settlement & Demand Pressure | Crowding, migration, displacement, settlement growth, resource competition. |
| 12 | Information, Communication & Coordination | Warnings, rumor, knowledge access, communication and organization capacity. |

Availability, accessibility, and affordability must remain distinct. The host may track detailed commodities; S.P.A.R.K. consumes or derives high-leverage pressures.

## 14.2 Economic / Social / Institutional domains

| # | Domain | Purpose |
|---|---|---|
| 1 | Prices & Affordability | Cost of necessities and relative ability to pay. |
| 2 | Income, Wealth & Debt | Income, savings, reserves, wealth, debt and disposable capacity. |
| 3 | Employment & Livelihood Security | Job availability, wage stability, business viability and occupational demand. |
| 4 | Trade, Markets & Exchange | Trade volume, dependency, market isolation, black markets and merchant confidence. |
| 5 | Ownership, Property & Economic Control | Land/property/resource ownership, tenancy, inheritance and dependence. |
| 6 | Taxation, Tribute & Obligations | Taxes, rent, tithes, tribute, debts, labor and military levies. |
| 7 | Governance & Administrative Capacity | Relief, services, coordination, collection and implementation ability. |
| 8 | Legitimacy, Authority & Political Trust | Perceived right to rule, fairness, succession confidence and institutional trust. |
| 9 | Law, Order & Enforcement | Legal predictability, corruption, punishment certainty, courts and property protection. |
| 10 | Factional Power & Political Competition | Faction strength, rivalry, alliances, claims, influence and military backing. |
| 11 | Social Hierarchy, Status & Mobility | Rank, prestige, class stratification, access, patronage and marginalization. |
| 12 | Community Cohesion, Mutual Aid & Social Support | Social trust, family/community support, collective efficacy and isolation. |
| 13 | Culture, Religion, Denomination & Moral Order | Faith, sect, doctrine, ritual, taboo, honor, charity, revenge/forgiveness norms. |
| 14 | Collective Mood, Unrest & Public Sentiment | Public fear, anger, hope, panic, solidarity, unrest and morale. |

Religion and denomination are first-class affiliations and belief systems. Profiles may define doctrine, devoutness, ritual obligations, religious authority, sacred/taboo rules, charity, forgiveness/revenge norms, tolerance, perceived heresy, pilgrimage, and holy-site significance.

## 14.3 Individual Exposure / Situation domains

| # | Domain | Actor-specific question |
|---|---|---|
| 1 | Basic Needs Access | Can this actor obtain food, water, shelter, warmth, medicine and necessities? |
| 2 | Economic Position | Purchasing power, savings, debt, property, job security and dependency. |
| 3 | Physical Safety & Health | Injury, illness, exposure and immediate danger. |
| 4 | Household & Dependents | Spouse, children, parents, dependents, caregiving and household vulnerability. |
| 5 | Social Position & Belonging | Status, acceptance, stigma, community and faction membership. |
| 6 | Authority & Obligation Exposure | Taxes, law, service, duties, promises, debts, contracts and commands. |
| 7 | Relationship Situation | Help, harm, trust, threat, dependence, debt and interpersonal obligations. |
| 8 | Culture, Religion & Denomination Situation | Affiliation, devoutness, doctrine, sacred duty, taboo and interfaith conditions. |
| 9 | Opportunity & Mobility | Ability to leave, hunt, trade, petition, change livelihood or join another group. |
| 10 | Knowledge & Information Position | What the actor knows, believes, suspects, misunderstands or has heard. |

The individual layer represents objective or host-authoritative circumstances, not the actor's interpretation.

## 14.4 Perception and belief

Belief records may represent cause, blame target, intent, confidence, source, expected future, believed alternatives, social interpretation, normative interpretation, and supernatural interpretation. Beliefs may be false. Actors act on their believed model, while the host retains objective truth.

## 14.5 Appraisal dimensions

Appraisals answer "What does this mean for me?" and should normally be calculated ephemerally:

| Dimension | Meaning |
|---|---|
| Threat | Danger to self or valued others. |
| Opportunity | Potential benefit or exploitable opening. |
| Controllability | Perceived ability to change the situation. |
| Predictability | Understanding of what is happening and what follows. |
| Scarcity | Perceived insufficiency of important resources. |
| Fairness / Injustice | Whether the situation violates fairness expectations. |
| Blame / Responsibility | Who or what is responsible. |
| Intent | Accidental, negligent, malicious, benevolent, natural, divine, magical. |
| Legitimacy | Whether authority, law, punishment, tax or social order is justified. |
| Social Support | Whether help and solidarity are available. |
| Status / Social Meaning | Effects on dignity, honor, shame, prestige or standing. |
| Goal Compatibility | Whether the situation supports or obstructs active goals. |

## 14.6 Valid shortcuts

- Drought normally begins at world condition and material pressure.
- An insult may begin at perception/appraisal.
- An assassination may begin at institutional condition.
- An earthquake may directly create structural damage, injury exposure, and fear.
- A supernatural apparition may directly affect belief while also changing supernatural environment state.

The validator should allow declared shortcuts and reject undeclared arbitrary mutation.

# 15. Actor Attributes, Dynamic State, Relationships, Memory, and Identity

## 15.1 Stable attributes and tendencies

Profiles may define stable actor properties such as:

- empathy;
- self-control;
- aggression;
- harm aversion;
- cooperativeness;
- patience;
- conscientiousness;
- sociability;
- curiosity;
- skepticism;
- risk tolerance;
- dominance;
- persistence;
- behavioral flexibility;
- charisma;
- intelligence;
- appearance/attractiveness properties;
- devoutness and norm adherence;
- future orientation;
- vengefulness.

Most actors should be configured so ordinary pressures produce ordinary coping rather than chaos. There is no need for one universal `reasonableness` meter; reasonableness emerges from inhibiting traits, norms, relationships, alternatives, risk, authority, and expected consequences.

Charisma should expand or strengthen social options, not guarantee affection. Intelligence may improve inference, planning, learning, deception detection, and alternative generation, but does not imply morality or domain competence. Appearance may be an actor property; its social meaning is observer-, culture-, context-, and relationship-dependent.

## 15.2 Dynamic internal state families

A compact high-leverage baseline is:

1. Stress / Arousal
2. Fear / Felt Security
3. Anger / Frustration
4. Sadness / Grief
5. Hope / Morale
6. Confidence / Self-Efficacy
7. Social Need / Loneliness
8. Moral / Self-Evaluative State (guilt, shame, pride)
9. Curiosity / Engagement / Boredom

Profiles may add states, but should prefer intensity and context over synonym proliferation.

## 15.3 Directed relationship dimensions

Provisional baseline:

- affection;
- trust;
- respect;
- fear;
- resentment/grievance;
- obligation;
- loyalty.

Sparse contextual markers may include protectiveness, rivalry, debt, betrayal, attraction, kinship, dependence, and reconciliation. Relationships are directional and sparse.

## 15.4 Reputation, honor, fame, status

Keep separate:

- **relationship:** one actor's state toward another;
- **reputation:** socially circulated assessment, potentially multidimensional;
- **honor:** conformity with a personal, cultural, religious, guild, or faction code;
- **fame/notoriety:** how widely known an actor is;
- **status/prestige:** standing within a group.

The same act may improve honor with one denomination or faction and reduce it with another.

## 15.5 Selective memory

Persist memories only when they are likely to alter future behavior. High-value memory classes include harm/help, betrayal, promise kept/broken, kin harmed, repeated cooperation/exploitation, danger/safety, resource location, successful/failed strategy, shared hardship, leader failure/protection, faction betrayal, caregiving, forgiveness, and reconciliation.

A memory should be able to retain event/meaning summary, actors, place/time, perceived cause, significance, confidence, source, decay/persistence, relationship effect, and goal effect. Do not retain a lifetime transcript.

## 15.6 Identity and affiliation

Actors may hold sparse hierarchical affiliations:

- household/kinship;
- workplace/employer/guild;
- settlement;
- faction;
- region/kingdom;
- religion and denomination;
- culture/species/archetype;
- role/office.

Global/scoped effects map to actors through these affiliations and spatial memberships rather than through player proximity.

## 15.7 Behavioral profile packs

Profiles should compose actor vocabularies rather than apply human psychology to everything. Initial reference packs include human social actor, social predator, solitary predator, herd prey, territorial herbivore, domesticated companion/work animal, scavenger/opportunist, migratory animal, hive creature, intelligent factional monster, solitary intelligent monster, and software-agent social persona.

# 16. Goals, Choice Context, and Behavior Selection

## 16.1 Goal families

Specific goals should be instances of broad reusable families:

| Goal family | Examples |
|---|---|
| Survival / Need Satisfaction | food, water, shelter, health, escape |
| Protection | self, family, settlement, faction, property |
| Acquire / Preserve Resources | money, goods, land, supplies, wealth |
| Social Connection | companionship, friendship, acceptance, relationship repair |
| Status / Recognition / Influence | prestige, reputation, authority, honor, political influence |
| Duty / Obligation | contract, authority, dependent care, religion, promise, debt |
| Justice / Retaliation / Redress | compensation, accountability, reporting, punishment, revenge |
| Security / Control / Stability | order, territory, prevention, threat removal |
| Exploration / Knowledge / Mastery | investigation, learning, exploration, skill, anomaly |
| Ideological / Moral / Religious | faith, doctrine, reform, moral code, opposition to heresy |

Actors may maintain multiple concurrent goals with priority, urgency, persistence, source, satisfaction/failure conditions, target, and status. Goal conflict is expected.

## 16.2 Choice Context families

| Choice context | Question |
|---|---|
| Physical Capability | Can the actor physically perform it? |
| Resource Availability | Does the actor possess required money, tools, equipment, contacts or materials? |
| Opportunity / Access | Can the actor reach the target or situation? |
| Knowledge / Competence | Does the actor know how? |
| Expected Effectiveness | Does the actor believe it will work? |
| Risk & Cost | What may be lost: health, money, time, reputation, relationships, retaliation? |
| Social / Legal / Moral Constraint | Law, conscience, religion, norms, punishment and duty. |
| Relationship / Role Constraint | Target identity, kinship, friendship, office and role obligations. |
| Alternative Availability | Are safer, lawful or cooperative options available? |
| Power, Authority & Deterrence | Physical, legal, political, economic, social, factional, supernatural and reputational power. |

This layer prevents desire from becoming immediate extreme action. Curiosity increases approach/investigation/social-contact candidates, but fear, norms, risk, power, relationship, context, and competing goals still moderate behavior.

## 16.3 Behavior families

| Behavior family | Illustrative methods/actions |
|---|---|
| Acquire | buy, gather, forage, hunt, borrow, request; theft is an illicit acquisition method/tag |
| Preserve / Protect | guard, hide, store, fortify, escort, defend |
| Avoid / Escape | flee, hide, avoid, withdraw, migrate, change route |
| Approach / Investigate | explore, inspect, follow, search, scout, question |
| Communicate / Request | ask, warn, inform, request aid, complain, confess, persuade, petition |
| Cooperate / Assist | help, share, heal, teach, coordinate, rescue, support |
| Exchange / Negotiate | buy, sell, barter, bargain, offer, hire, lend, repay |
| Affiliate / Socialize | visit, befriend, court, celebrate, join, attend |
| Compete / Contest | challenge, outbid, campaign, contest leadership/status |
| Dominate / Coerce | intimidate, command, extort, blackmail, force compliance |
| Resist / Defy | refuse, protest, disobey, evade, strike, rebel |
| Aggress / Harm | attack, sabotage, raid, assassinate, destroy, kidnap |
| Reconcile / Repair | apologize, forgive, compensate, mediate, make peace |
| Create / Build / Transform | craft, construct, farm, repair, invent, perform ritual |

A concrete behavior has one primary family plus tags. Example:

```text
behavior.acquire.steal_food
primary_family = acquire
tags = theft, illegal, covert, resource_transfer
```

## 16.4 Scoring and deterministic stochastic choice

Candidate scores may conceptually combine:

```text
goal satisfaction
+ trait compatibility
+ state and relationship influence
+ expected effectiveness
+ opportunity
- risk and cost
- legal/moral inhibition
- role/relationship conflict
- power/deterrence
- competing-goal harm
```

Use fixed-point/integer scoring where practical. Seeded stochastic selection may choose among competitive options so actors are varied but not incoherent. An extreme action should not win while clearly superior ordinary alternatives remain available unless the actor's state/profile legitimately makes that plausible.

## 16.5 Host execution

S.P.A.R.K. emits semantic intent such as `acquire food by hunting`. The host checks path, equipment, wildlife, legality, physical capability, combat, inventory, and actual result. The host then returns the outcome.

# 17. Outcome, Aggregation, and Feedback

Every accepted host action may produce one or more outcome families:

1. **Actor outcome:** need satisfaction, injury, fatigue, skill, goal status.
2. **Target outcome:** aid, harm, loss, repair, displacement, resource transfer.
3. **Relationship outcome:** trust, affection, fear, respect, grievance, obligation, loyalty.
4. **Material/environmental outcome:** wildlife, forest, crops, structures, roads, resources.
5. **Economic/institutional outcome:** inventory, market supply, guard workload, tax capacity, guild output.
6. **Information/reputation outcome:** rumor, fame, honor judgment, public knowledge, blame attribution.
7. **Aggregate/threshold outcome:** many local actions cross a settlement, faction, ecological, or regional threshold.

Aggregation closes the causal circulation:

```text
many hungry households choose hunting
-> regional hunting pressure rises
-> wildlife reproduction falls below harvest
-> wildlife abundance declines
-> neighboring settlement loses wild-food supply
-> new household situations and political pressures emerge
```

Local detailed actions may collapse into aggregate values. Dormant simulation may update those aggregates directly. No subsystem may claim to reconstruct unrecorded microhistory when a scope becomes active again.

Feedback loops are required, but v1 must prohibit zero-delay recursive cycles. A feedback edge must cross a scheduled/delayed boundary or a stable commit wave.

# 18. Scheduler, Heartbeat, and Multi-Resolution Simulation

## 18.1 Heartbeat model

S.P.A.R.K. has a deterministic scheduler rather than one universal polling loop. The host advances a monotonic integer simulation clock. S.P.A.R.K. evaluates only work that is due.

Profiles may define cadences such as:

- local transient evaluation: seconds/minutes;
- actor needs and recovery: minutes/hours;
- relationship/memory decay: hours/days;
- settlement/resource aggregation: days/weeks;
- technology, politics, and rare event checks: weeks/months/years.

Cadences are tunable profile data.

## 18.2 Multi-resolution tiers

### Active Simulation Envelope

Detailed actor behavior, local interactions, dialogue, movement intents, and player-visible events.

### Warm tier

Named/pinned actors, important goals, routes, relationships, obligations, and near-term plans at reduced cadence.

### Dormant tier

Households, cohorts, settlements, factions, ecosystems, technology, and routes evolve through aggregate state, scheduled milestones, and deterministic catch-up.

The final size/shape of the Active Simulation Envelope is not frozen. The host should support configurable chunk/radius policies and benchmark a meaningful local neighborhood rather than only the visible cell.

## 18.3 World continuity

Distant places do not freeze because no player is present. Global and scoped triggers apply to geographic, administrative, ecological, factional, religious, or entity scopes. The host and S.P.A.R.K. update macro state at cheap cadences. When the server process is offline, the host game may select pause or deterministic catch-up policy.

## 18.4 World size

World size should scale primarily with persisted aggregate scopes and generated world data, not with per-frame actor evaluation. Do not freeze a maximum world dimension before profiling. S.P.A.R.K. load should be driven mainly by active actors, scheduled obligations, changed factors, and aggregate scopes, not total addressable cells.

# 19. Deterministic RNG, Rule Evaluation, and Propagation Safety

## 19.1 Random-address model

Do not use one shared mutable RNG stream. Derive independent random addresses from:

```text
root seed
+ profile/behavior epoch
+ rule or trigger ID
+ scope/actor ID
+ logical occurrence/index
```

Concurrency, batching, and iteration order must not alter unrelated outcomes.

## 19.2 Probability and frequency

The profile owns rates and modifiers. S.P.A.R.K. owns deterministic evaluation machinery.

A trigger frequency may depend on:

```text
baseline rate
x regional susceptibility
x season/climate prerequisites
x current-state modifiers
x game volatility preset
```

Support presets such as `Quiet`, `Naturalistic`, `Volatile`, `Dramatic`, `Supernatural High`, and `Custom`. Scientific/historical data provide calibration references, not immutable game law.

## 19.3 Declarative rule model

The first rule language should be constrained and declarative. Support combinations of:

- thresholds and eligibility;
- add/subtract/scale/clamp;
- weighted sums;
- piecewise curves;
- probability gates;
- scope mapping;
- delays and cooldowns;
- decay and recovery;
- aggregation and threshold emission.

Do not build a general scripting language in v1.

## 19.4 Deterministic waves

Evaluate against a stable snapshot, collect effects, sort deterministically, validate, commit as a batch, and enqueue newly affected work. Bound work per cycle and defer overflow visibly.

## 19.5 Safety invariants

- no undeclared arbitrary mutation;
- no instant causal cycles;
- bounded fan-out and propagation depth;
- explicit scope mapping;
- clamped values and declared units;
- recovery semantics for accumulating pressures;
- visible truncation/defer warnings;
- deterministic replay hashes for accepted fixtures.

# 20. Dynamic Dialogue, Expression, and Player Interaction

## 20.1 State-driven dynamic dialogue trees

The primary conversational target is not a static hand-authored tree and not unrestricted free-text AI. It is a **semantic dynamic dialogue tree** generated from S.P.A.R.K. state and host capabilities.

```text
actor state/goal/knowledge/relationship
-> speech intent
-> clause selection and style
-> NPC line
-> valid player response intents
-> host UI options
-> selected response
-> new outcome/state
```

Example context:

```text
severe hunger
belief: goblins stole stores
high grievance toward goblins
moderate trust in player
goal: obtain food
```

Possible NPC expression:

> "The goblins took nearly everything. Do you have anything you can spare?"

Possible generated player options:

- Offer food.
- Ask what happened.
- Ask where the goblins are.
- Offer to hunt or confront them.
- Ask why the lord has not helped.
- Explain that the player has no food.
- Refuse without explanation.
- Leave.

## 20.2 Relationship-sensitive expectations

Relationship state affects whether an NPC asks, what they expect, and how they interpret the answer. A stranger's refusal may create mild disappointment. A close friend refusing an extreme need after a prior promise may be appraised as rejection or betrayal. If the player explains they have no food and the NPC believes them, the betrayal appraisal may be reduced or eliminated.

## 20.3 Semantic expression layer

S.P.A.R.K. should produce speech acts and clauses, not make unrestricted factual claims:

- express state;
- identify believed cause;
- request;
- warn;
- blame;
- thank;
- apologize;
- negotiate;
- threaten;
- promise;
- reconcile.

Personality, culture, denomination, role, relationship, severity, and current state modify wording and delivery. Knowledge/belief limits what may be said.

## 20.4 Free text

Future free text may be interpreted into the same structured intents, but it is not required for v1 and must not turn an LLM into the authoritative NPC brain.

## 20.5 Repetition and salience

Use cooldowns, topic salience, recent-line memory, current activity, social tendency, listener availability, urgency, and RNG to avoid constant repeated barks.

# 21. Voice Profiles and TTS

Each created actor may receive a deterministic persistent `VoiceProfile` derived from world/actor seed, profile version, species, age, body/role constraints, culture/dialect, and configured variation.

Candidate parameters include:

- voice family/speaker mapping;
- pitch and speaking rate;
- apparent size/formant;
- roughness and breathiness;
- energy and expressiveness;
- prosody;
- dialect/accent/style;
- age character;
- species/audio effects.

Identity remains stable. Current S.P.A.R.K. state modifies performance:

- anger: sharper/faster/more forceful;
- fear: variable pacing/uncertainty;
- grief: lower energy/slower/longer pauses;
- exhaustion/starvation: low energy and breathiness.

The host maps the abstract profile to the available TTS backend, synthesizes on demand, and may cache audio. The core must not depend on one voice vendor or model.

# 22. Gameplay Hooks and Decision Points

S.P.A.R.K. may emit advisory `GameplayHookCandidate` records when state creates a meaningful reason for content.

Supported families include:

- aid request;
- bounty;
- rescue/search;
- revenge/hunt;
- investigation;
- resource/opportunity discovery;
- migration/escort;
- reconciliation/mediation;
- political petition;
- crisis response;
- ruler/emperor decision point;
- faction negotiation;
- supernatural response.

Examples:

```text
repeated raids + faction grievance + authority capacity
-> bounty candidate
```

```text
food shortage + trusted player + NPC request-help behavior
-> aid interaction candidate
```

```text
provincial drought + migration + unrest + legitimacy pressure
-> ruler decision candidate
```

The host quest/political system decides whether to instantiate formal content, objectives, rewards, and completion rules.

# 23. Game-World Integration Profile

## 23.1 Host inputs

- actor/entity registration and affiliations;
- world/environment observations;
- economy observations and derived affordability/security facts;
- actual resource, population, production, technology, settlement, faction, and wildlife state;
- player actions and dialogue selections;
- combat and interaction outcomes;
- time advancement and fidelity tier;
- host-generated or delegated triggers;
- rumors, believed causes, and information availability.

## 23.2 Economy boundary

A real economy may include NPC accounts, currency, salaries, employers, rents, taxes, debt, transactions, goods, inventories, prices, production, trade, and property. That system belongs to the game or a dedicated economy module.

S.P.A.R.K. consumes meaningful economic conditions such as:

- income insufficient;
- food unaffordable;
- job lost;
- debt pressure;
- wealth/status change;
- business thriving/failing;
- tax burden;
- relief received/refused.

S.P.A.R.K. must never create money or alter ledgers directly.

## 23.3 Technology and macro evolution

Technological development belongs to the host macro simulation. Distant societies may advance through aggregate monthly/seasonal/yearly rules. S.P.A.R.K. may consume innovation pressure, specialist loss, resource access, war pressure, trade contact, and technology consequences, and may generate relevant actor/faction responses.

## 23.4 NPC evaluation

NPCs do not individually call the API. The host adapter evaluates or batches relevant active actors. S.P.A.R.K. returns semantic intent; the host performs feasibility checks and execution.

## 23.5 Graphics independence

S.P.A.R.K. outputs may feed first-person 3D, top-down 2D, text/debug, map, administrative, or other clients. The same semantic world may be represented differently without changing behavior state.

# 24. MCI Agent Commons Profile

## 24.1 Architectural boundary

```text
real S.W.A.R.M. telemetry
        -> sanitizing bridge
        -> S.P.A.R.K. MCI-social profile
        -> fictional social state
        -> scenes/screenplay/TTS/visualization
```

There is no reverse authoritative control edge.

## 24.2 Candidate safe input signals

- task completed;
- review accepted/rejected;
- assistance given;
- collaboration count;
- discovery;
- milestone;
- harmless retry/failure;
- disagreement classification;
- praise/recognition;
- time spent together on a work item.

Input payloads should avoid exposing confidential content when a coarse outcome signal is sufficient.

## 24.3 Candidate social factors

- familiarity;
- respect;
- gratitude;
- irritation;
- trust within the fiction;
- rivalry;
- affiliation;
- social energy;
- confidence;
- current conversational interest.

These are fictional presentation state, not claims about real model internals.

## 24.4 Outputs

- ambient activity suggestion;
- social interaction candidate;
- dialogue intent;
- screenplay beat;
- relationship update;
- scene queue entry;
- office/town location suggestion;
- persistent voice performance request;
- daily/weekly social recap.

## 24.5 Running screenplay

A Scene Renderer may format accepted scene candidates into screenplay form:

```text
INT. RESEARCH OFFICE - 10:42 AM

ADA looks up from the rejected patch report as MARCUS walks past.

ADA
You could have pretended to like it first.

MARCUS
Test seven was less diplomatic.
```

The screenplay is fictional interpretation. It is not evidence about real agent thought or emotion.

## 24.6 Frequency controls

The operator must be able to control:

- popup interaction frequency;
- ambient scene frequency;
- screenplay verbosity;
- work-derived signal sensitivity;
- quiet hours;
- maximum simultaneous scenes;
- relationship-change intensity.

## 24.7 Agent Commons safety invariant

Real telemetry may inspire fictional scenes, but fictional feelings, intentions, dialogue, and relationships are never evidence about an actual model's hidden internal state. The social sidecar has no reverse authority edge.

## 24.8 Software-agent social profile seed

The downstream research supports a profile containing cooperativeness, persistence, curiosity, skepticism, trust propensity, behavioral flexibility, confidence, frustration, satisfaction, disappointment, recovery, selected help/cooperation memories, and harmless social goals such as ask, verify, assist, teach, coordinate, reconcile, celebrate, and report. These remain social-fiction inputs unless an explicit separately governed experiment is authorized.

# 25. AI Inspection and Real-Time Quality Control

## 25.1 Primary objective

The API should allow an authorized S.W.A.R.M. agent to inspect S.P.A.R.K. in real time and answer:

- What pressures are active?
- Why does this actor currently have this state?
- Which rule changed it?
- What candidate behaviors are eligible and why?
- Why was this behavior/speech/scene selected?
- Which factors are unused or dominant?
- Are propagation loops being truncated?
- Is the simulation repetitive or unstable?
- Which rules consume the most CPU?
- Are relationships or memories growing without bound?

## 25.2 Initial inspection operations

```text
describe_schema
list_profiles
get_manifest
inspect_target
inspect_factor
inspect_active_pressures
inspect_relationship
inspect_memory
trace_current_sources
explain_candidate
list_recent_outputs
inspect_rule
inspect_queue
inspect_system_health
validate_dry_run
```

## 25.3 Explanation response requirements

Responses should include:

- authoritative S.P.A.R.K. state;
- profile and version;
- current value;
- immediate source factors or signals;
- rules applied;
- major score modifiers;
- random address/selection summary where relevant;
- truncation or uncertainty warnings;
- suggested drill-down operations.

## 25.4 AI authority

AI may:

- read;
- query;
- analyze;
- report;
- propose rules/config changes;
- run approved staging simulations.

AI may not:

- directly write production state;
- modify governance;
- bypass configuration validation;
- execute host actions;
- convert fictional MCI state into real agent policy.

## 25.5 Additional required questions

The initial inspection surface should be able to answer structured questions equivalent to:

- Why did this actor approach the player?
- Why was this request generated?
- Why did a refusal become betrayal rather than disappointment?
- What pressures are active on this household, settlement, faction, or region?
- Which trigger probabilities and cadences are currently active?
- What caused this wildlife decline?
- Which alternatives did the actor consider?
- Which power/authority deterrents suppressed an action?
- Which factors are never used or produce little behavioral leverage?
- What work is due on the next scheduler cycle?

# 26. Persistence, Telemetry, and the Chronicle Boundary

## 26.1 Persisted S.P.A.R.K. state

Persist only what is required for continuity and deterministic replay within the declared support envelope:

- simulation time;
- root seed and behavior epoch;
- profile/schema/content versions;
- scheduler and delayed obligations;
- S.P.A.R.K.-owned StateCells;
- directed relationships;
- selected memories and beliefs;
- active goals;
- cooldowns and occurrence counters;
- bounded source references needed for active explanations;
- voice profiles where enabled.

## 26.2 Do not persist by default

- every trigger eligibility check;
- every failed probability roll;
- every ephemeral appraisal;
- every candidate behavior score;
- every dialogue candidate;
- every low-value interaction;
- exhaustive world history;
- complete NPC lifetime transcripts.

## 26.3 Player telemetry

Comprehensive player telemetry remains a host concern and may include achievements, travel, combat, economy, social decisions, quests, exploration, progression, property, political activity, and personal history. S.P.A.R.K. may receive only permitted derived inputs and may expose its own contributions/reactions.

## 26.4 Engine telemetry

Measure at least:

- due-work volume by cadence;
- trigger eligibility and occurrence counts;
- factor updates and propagation edges;
- fan-out, depth, deferred/truncated work;
- active/warm/dormant target counts;
- candidate behavior counts and score distributions;
- memory/relationship growth and pruning;
- query/explanation latency;
- profile reload/migration events;
- CPU, memory, storage, cache hit rate;
- deterministic divergence;
- unused/dead factors and rules.

## 26.5 Bounded explanation, not Chronicle

Retain enough causal/source references to explain active state and recent meaningful transitions. Do not reintroduce a universal event ledger. When a detailed history is unavailable, explanations must say what is known rather than inventing provenance.

# 27. Performance, Complexity, and World-Scale Budget

## 27.1 Complexity target

Very complex and vibrant from the player's/operator's perspective; clean, smooth, bounded, and measurable in execution.

## 27.2 Scaling rules

- work scales with due rules, changed state, active actors, sparse edges, and aggregate scopes;
- no scan of every trigger every hour;
- no per-frame evaluation of every actor;
- broad trigger catalogs remain mostly dormant definitions;
- appraisals and choice scores are ephemeral;
- relationships and memories are sparse and policy-bounded;
- distant populations use aggregate updates;
- rule predicates/templates are compiled/cached;
- profile packs instantiate only relevant factors.

## 27.3 Provisional factor budgets

For a rich named humanoid actor, target roughly:

- 6-12 meaningful stable traits/attributes;
- 8-15 available dynamic states, with only a subset salient;
- 1-5 active goals;
- 5-25 selected memories;
- sparse relevant relationship edges, not a dense social network;
- 4-7 relationship dimensions per relevant edge, not all necessarily instantiated;
- ephemeral appraisals and behavior candidates on demand.

These are profiling targets, not hard limits.

## 27.4 Benchmark before freezing world size

Report:

- cost per trigger evaluation;
- cost per rule edge;
- cost per actor appraisal/choice cycle;
- cost per active actor batch;
- dormant settlement/faction update cost;
- catch-up cost per elapsed game year;
- persistence bytes per actor/scope;
- explanation query p50/p95;
- scaling curve for multiple active player envelopes.

Do not claim a final continent size or concurrency ceiling until the host game and S.P.A.R.K. are benchmarked together.

# 28. Determinism and Versioning Constitution

1. Same supported build, manifests, root seed, starting state, logical time, ordered host inputs, and profile versions must produce identical canonical S.P.A.R.K. outputs.
2. Use stable total ordering for equal-time work.
3. Use semantic random addresses rather than ambient/global RNG calls.
4. Use integer/fixed-point canonical probabilities and scores where practical.
5. Wall-clock time, unordered hash iteration, unversioned external data, and ambient randomness are prohibited in canonical evaluation.
6. Profile/rule changes create explicit behavior epochs.
7. Old saves retain or reference the definitions needed to continue or migrate.
8. Outcome replay and mechanism replay are distinct; unsupported replay must fail explicitly.
9. Snapshots are optimizations, not silent truth replacements.
10. Cross-platform determinism is required within declared Windows/Linux/Android support builds, not across arbitrary future compilers and versions forever.

# 29. Testing and Quality Assurance

## 29.1 Unit and schema tests

- common definition validation;
- scope/authority validation;
- fixed-point arithmetic and bounds;
- scheduler due-work behavior;
- trigger eligibility/cooldown;
- rule operations and effect batching;
- salience and pruning;
- profile migration.

## 29.2 Property-based tests

- no unauthorized host-state mutation;
- no illegal instant cycles;
- values remain in bounds;
- random-address isolation;
- deterministic ordering;
- disabled/deprecated definitions do not reappear;
- relationship directionality;
- sparse memory/relationship limits;
- same seed/input replay equivalence.

## 29.3 Behavioral tests

- moderate alternatives dominate extreme behavior under ordinary conditions;
- stacking failures can make extreme behavior plausible;
- recovery and reconciliation can terminate escalation;
- curiosity changes approach/investigation likelihood without overriding fear/risk;
- charisma changes social option effectiveness without guaranteeing liking;
- intelligence changes planning/inference without granting domain knowledge;
- power/authority/deterrence suppresses implausible aggression;
- relationship-sensitive aid/refusal produces distinct appraisals;
- false beliefs can produce coherent different behavior.

## 29.4 Integration tests

- service and embedded runtime yield equivalent semantics;
- MCI handshake and safe telemetry bridge;
- game adapter batch evaluation;
- host behavior accept/reject/outcome loop;
- macro-to-local activation and dormant catch-up;
- dynamic dialogue options and selected responses;
- TTS adapter receives stable voice/performance profile;
- configuration hot-tune and structural reload.

## 29.5 Load and adversarial tests

- broad dormant trigger catalog;
- many active actors with sparse relationships;
- worst-case fan-out and delayed feedback;
- malicious/invalid client messages;
- profile schema bombs and oversized inputs;
- explanation-query abuse;
- active/warm/dormant churn;
- long-running memory/state growth.

# 30. First Reference Implementation Slice

## 30.1 World setup

- two settlements: Pontafique and Lindemar;
- one shared forest/wildlife region;
- households with unequal wealth and food access;
- a minimal host economy stub with actual food stock, price, money, and affordability;
- a small set of named actors plus aggregate households;
- at least one ruler/authority actor;
- game time, scheduler, and deterministic seed.

## 30.2 Required causal circulation

```text
Drought trigger
-> soil moisture / water conditions decline
-> crop productivity and food availability decline
-> prices/affordability pressure rises
-> poorest Pontafique households become food insecure
-> actor appraisals/goals/choices vary by traits, relationships, alternatives and deterrence
-> actors ask for aid, hunt, migrate, steal, organize help, or petition
-> many hunting outcomes aggregate
-> wildlife abundance declines
-> Lindemar wild-food availability falls
-> Lindemar social/economic pressure rises
-> authority/ruler receives a decision-point candidate
-> chosen host response creates new consequences
```

## 30.3 Required player/NPC dialogue case

Bron requests food based on hunger, belief about goblin theft or drought cause, relationship to player, and goal. The game generates semantic response options. A friendly refusal without explanation may produce disappointment/betrayal; a credible explanation may reduce that appraisal.

## 30.4 Required extensibility tests without Rust changes

1. Add `trait.curiosity` or an equivalent new tendency through profile data and demonstrate changed approach/investigation likelihood.
2. Add `trigger.supernatural.blood_moon` through profile data and route its effects through existing direct-condition and pressure classes.
3. Add a religion/denomination definition and demonstrate distinct interpretation/honor judgments.
4. Adjust drought and blood-moon probabilities/cadences through validated configuration.

## 30.5 Required MCI proof

A minimal MCI adapter ingests harmless work telemetry, creates fictional social state, emits a scene/screenplay beat, and demonstrates zero authority over real S.W.A.R.M. execution.

# 31. Acceptance Criteria for Blueprint v0.2 Implementation

The architecture is proven only when:

1. Rust core builds/tests on Linux and Windows, with Android-compatible core constraints continuously checked.
2. Service and embedded execution produce equivalent deterministic outputs for the fixture corpus.
3. Shared protocol handshake, capability negotiation, validation, authentication, and profile selection work.
4. Host-owned state cannot be mutated by S.P.A.R.K. without host acknowledgement.
5. The complete Pontafique/Lindemar causal loop executes and is explainable.
6. Global/scoped triggers affect distant scopes independent of player presence.
7. Active, warm, and dormant updates preserve declared aggregate commitments.
8. Actor behavior differs coherently through traits, belief, relationships, goals, alternatives, power, and seeded variation.
9. Ordinary constraints prevent routine overreaction; recovery pathways work.
10. Dynamic dialogue trees are produced from semantic state and host capabilities.
11. Relationship-sensitive player responses alter trust/affection/grievance appropriately.
12. Curiosity, supernatural trigger, and religion/denomination additions pass without Rust changes.
13. Trigger probabilities, cadences, weights, and decay are editable through allowed profile/config mechanisms.
14. Bounded explanations answer why a state or behavior occurred.
15. MCI Agent Commons operates as a one-way non-authoritative sidecar.
16. Broad dormant trigger catalogs do not cause proportional idle CPU cost.
17. Deterministic replay passes across supported builds.
18. Invalid profiles, cycles, scopes, and unauthorized requests are rejected clearly.

# 32. Implementation Sequence

## Phase 0 - Freeze artifacts and ADRs

- glossary and causal grammar;
- authority matrix;
- stable identifier/version policy;
- Rust workspace and dependency rules;
- deterministic clock/RNG ADR;
- profile format and validation ADR;
- service/embedded equivalence ADR;
- initial performance and security budgets.

## Phase 1 - Rust core skeleton

- common IDs/scopes/value types;
- StateCell and ownership;
- monotonic logical clock;
- due-work scheduler;
- random-address service;
- profile manifest loader/validator;
- deterministic fixtures.

## Phase 2 - Rule and effect runtime

- declarative operations;
- stable-snapshot evaluation;
- deterministic effect batches;
- delayed obligations;
- cycle/fan-out validation;
- decay/recovery;
- aggregation.

## Phase 3 - Protocol, service, persistence, security

- handshake and capabilities;
- typed messages;
- local service wrapper;
- embedded API;
- authentication/scopes/rate limits;
- snapshots/migrations;
- inspection foundation.

## Phase 4 - Actor behavioral core

- traits and actor profiles;
- belief/memory/relationship/goal stores;
- ephemeral appraisal;
- salience;
- choice context and behavior scoring;
- host accept/reject/outcome loop.

## Phase 5 - MCI Agent Commons thin slice

- safe telemetry bridge;
- mci-social profile;
- scene/screenplay output;
- strict no-reverse-authority tests.

## Phase 6 - Game reference causal slice

- host economy/resource stub;
- drought and macro scope;
- two-settlement/wildlife loop;
- active/warm/dormant simulation;
- ruler decision candidate.

## Phase 7 - Dynamic dialogue and expression

- semantic speech acts;
- generated player response intents;
- relationship-sensitive outcomes;
- repetition/salience controls.

## Phase 8 - Reputation, honor, religion, factions, hooks

- distinct social assessments;
- denomination/culture mappings;
- quest/bounty/crisis/opportunity candidates.

## Phase 9 - Voice profile/TTS seam

- deterministic voice identity;
- performance modifiers;
- backend adapter and cache contract.

## Phase 10 - Calibration and expansion

- import selected research-catalog definitions;
- historical/scientific frequency calibration;
- supernatural trigger inventory;
- larger trigger/event ecology;
- measured factor and performance pruning.

# 33. Research Integration and Catalog Policy

## 33.1 Integrated packages

The upstream package contains:

- 244 canonical environmental/natural nodes;
- 11 major categories;
- 44 frequency-calibration families;
- 35 source groups.

The downstream package contains:

- 301 canonical psychological/social/behavioral nodes;
- 50 highest-leverage candidates;
- 32 memory classes;
- 28 middle-layer handoffs;
- 30 pseudo-depth warnings;
- 75 representative chains;
- 12 species/archetype packs.

## 33.2 Controlling interpretation

These packages are overcomplete research inventories. They are not implementation schemas and do not authorize hundreds of persistent runtime factors.

Use them to:

- identify high-connectivity candidate triggers and states;
- calibrate event families;
- populate behavior packs;
- test omissions;
- identify pseudo-depth/redundancy;
- design recovery and alternative pathways;
- create profile content and test fixtures.

## 33.3 Supernatural research

A bounded supernatural-trigger inventory should be produced later at Medium effort. It should map triggers into the existing direct-condition and bridge taxonomy, not create a parallel engine.

## 33.4 Admission rule

A candidate factor or trigger must have clear semantics, an authoritative source/derivation, a downstream consumer, bounded cost, explainability, test expectations, and a migration/retirement path. Research breadth is preserved in catalogs; runtime breadth is admitted selectively.

# 34. Frozen, Provisional, and Future Decisions

## 34.1 Frozen

- Rust production language.
- Chronicle shelved.
- service plus embeddable runtime.
- shared protocol and adapters.
- host authority boundary.
- causal grammar and minimal runtime primitives.
- deterministic scheduler and random addresses.
- profile-driven vocabulary.
- sparse state, relationships, and memory.
- no zero-delay cycles in v1.
- natural and supernatural trigger support.
- twelve bridge domains, fourteen societal domains, ten actor-situation domains, twelve appraisal dimensions, ten goal families, ten choice-context families, fourteen behavior families, seven outcome families as the provisional baseline taxonomy.
- dynamic dialogue-tree architecture.
- MCI social sidecar is non-authoritative.
- economy is host-owned.
- multi-resolution world continuity.
- AI inspection/explanation is first-class.

## 34.2 Provisional / tune by testing

- exact file/schema format;
- exact service transport;
- persistence backend details;
- factor/relationship/memory budgets;
- trigger cadences/probabilities;
- active-envelope size/shape;
- exact scoring curves and stochastic temperature;
- exact voice fields and TTS mappings;
- exact profile hot-reload boundaries.

## 34.3 Future

- free-text intent interpretation;
- advanced story/screenplay compilation;
- scientific/historical calibration automation;
- advanced counterfactual analysis;
- personality-performance experiments affecting bounded real work;
- additional game profiles;
- public mod/plugin SDK only if later justified.

## 34.4 Out of scope

- Maximum Chronicle;
- graph/distributed infrastructure;
- runtime LLM NPC brain;
- game renderer/camera/physics;
- universal economy or full civilization engine inside S.P.A.R.K.;
- ungoverned AI canon/state mutation.

# 35. Operator and Engineer Decision Rights

## 35.1 Operator approval required

- causal meaning and gameplay role of major factors;
- new authority or governance paths;
- changes to non-authoritative MCI boundary;
- data leaving the local system;
- paid/cloud dependencies;
- final profile complexity and narrative-rate goals;
- controversial social/religious/psychological abstractions;
- backward-incompatible product behavior;
- scope expansion toward Chronicle, universal plugins, or runtime AI.

## 35.2 Engineer authority

The engineer may decide within this blueprint:

- crate/module organization;
- internal Rust types and algorithms;
- serialization/storage libraries;
- testing frameworks;
- local transport details;
- indexes/caches;
- exact migration implementation;
- code-generation and schema-validation tooling;
- optimization techniques that preserve semantics.

## 35.3 Mandatory escalation

Escalate when a requested feature:

- requires a new runtime primitive;
- changes who owns canonical state;
- weakens determinism, security, privacy, or portability;
- creates a substantial paid dependency;
- invalidates old saves/profiles;
- introduces runtime LLM authority;
- turns profile data into executable untrusted code;
- materially increases CPU/storage or project duration.

# 36. Primary Risks and Mitigations

| Risk | Mitigation |
|---|---|
| Ontology sprawl | Freeze primitives; data-drive vocabulary; admission/lint rules; prune by telemetry. |
| Pseudo-depth synonyms | Intensities, scopes and tags instead of duplicate factors; research pseudo-depth register. |
| Chaotic NPC behavior | Choice Context, alternatives, inhibition, authority, salience, moderate defaults. |
| Escalation without recovery | Mandatory decay/mitigation/reconciliation pathways. |
| Trigger chaos | Large catalogs but low density through eligibility, cooldown, susceptibility and volatility. |
| Propagation explosion | Deterministic waves, fan-out/depth budgets, no instant cycles, deferred work. |
| Dense social cost | Sparse relationships, selected memories, named-actor promotion. |
| World freezing | Active/warm/dormant macro simulation and scoped global effects. |
| Host/S.P.A.R.K. authority conflict | Per-definition authority declaration and host acknowledgement. |
| Save incompatibility | Stable IDs, versioned manifests, deprecate/migrate rather than delete. |
| AI hallucinated explanations | Structured evidence, coverage warnings, no invented provenance. |
| MCI fiction contaminates work | Physically/logically one-way sidecar, no reverse capabilities. |
| Over-modularity | Hard-code grammar; no universal scripting/plugin runtime. |
| Under-modularity | Extensibility acceptance tests for trait, supernatural trigger, religion/denomination. |
| Performance uncertainty | Instrument from day one; benchmark before expanding catalogs/world claims. |

# 37. Immediate Engineer Start Instructions

1. Create a clean S.P.A.R.K. repository/workspace separate from the game and S.W.A.R.M. repositories.
2. Import this Markdown as the human-readable controlling blueprint and place the DOCX as the formatted handoff artifact.
3. Create ADRs for Rust workspace, authority model, deterministic clock/RNG, profile/schema format, service/embedded equivalence, and persistence/versioning.
4. Produce a requirement-coverage matrix before implementation.
5. Implement only the minimum common primitives and profile validator first.
6. Build deterministic test fixtures before behavior richness.
7. Establish protocol schemas and a CLI/test adapter early.
8. Prove one-way MCI capabilities and host-owned state protections.
9. Build the drought/Pontafique/Lindemar scenario as the first full causal reference.
10. Treat all large catalogs, exact rates, weights, and factor counts as data/calibration work after the runtime is observable.
11. Do not add a general scripting language, graph database, runtime LLM, distributed service, or public plugin system.
12. At every milestone, report performance, determinism, migration impact, and requirement coverage.

# 38. Engineer Handoff Summary

S.P.A.R.K. should be understood as a reusable deterministic causal-behavior engine with a small fixed grammar and editable domain vocabulary.

The host supplies reality and executes actions. S.P.A.R.K. supplies scheduled triggers, pressure propagation, actor interpretation, social/psychological state, goals, bounded choices, semantic behavior and dialogue intents, aggregate feedback, and explanations.

Its defining product characteristics are:

- complex but comprehensible causal circulation;
- broad rare-event possibility with low event density;
- natural and supernatural triggers;
- living local NPCs and cheap distant evolution;
- rich but moderate-by-default social behavior;
- state-driven dynamic dialogue and future voices;
- emergent quests, bounties, crises, and ruler decisions;
- AI-friendly real-time inspection;
- strict non-authoritative MCI social simulation;
- Rust portability across Windows, Linux, and Android;
- extensibility without a universal plugin platform.

The first implementation should prove the full loop with a small world and a small vocabulary. Expansion should be driven by measured gameplay leverage and performance, not by the size of the research catalogs.

# Appendix A. Frozen Taxonomy Quick Reference

## A.1 Direct World Condition domains

1. Atmosphere and Climate
2. Water and Hydrology
3. Land and Geology
4. Ecology and Habitat
5. Biological Populations
6. Fire
7. Built Environment
8. Supernatural Environment

## A.2 Resource/material bridge domains

1. Food & Nutrition
2. Water & Sanitation
3. Shelter & Habitability
4. Materials & Energy
5. Ecology & Wild Resources
6. Infrastructure & Utilities
7. Mobility, Access & Logistics
8. Labor, Skills & Production Capacity
9. Health & Biological Burden
10. Safety, Threat & Security
11. Population, Settlement & Demand Pressure
12. Information, Communication & Coordination

## A.3 Economic/social/institutional domains

1. Prices & Affordability
2. Income, Wealth & Debt
3. Employment & Livelihood Security
4. Trade, Markets & Exchange
5. Ownership, Property & Economic Control
6. Taxation, Tribute & Obligations
7. Governance & Administrative Capacity
8. Legitimacy, Authority & Political Trust
9. Law, Order & Enforcement
10. Factional Power & Political Competition
11. Social Hierarchy, Status & Mobility
12. Community Cohesion, Mutual Aid & Social Support
13. Culture, Religion, Denomination & Moral Order
14. Collective Mood, Unrest & Public Sentiment

## A.4 Individual situation domains

1. Basic Needs Access
2. Economic Position
3. Physical Safety & Health
4. Household & Dependents
5. Social Position & Belonging
6. Authority & Obligation Exposure
7. Relationship Situation
8. Culture, Religion & Denomination Situation
9. Opportunity & Mobility
10. Knowledge & Information Position

## A.5 Appraisal dimensions

- Threat
- Opportunity
- Controllability
- Predictability
- Scarcity
- Fairness / Injustice
- Blame / Responsibility
- Intent
- Legitimacy
- Social Support
- Status / Social Meaning
- Goal Compatibility

## A.6 Goal families

- Survival / Need Satisfaction
- Protection
- Acquire / Preserve Resources
- Social Connection
- Status / Recognition / Influence
- Duty / Obligation
- Justice / Retaliation / Redress
- Security / Control / Stability
- Exploration / Knowledge / Mastery
- Ideological / Moral / Religious

## A.7 Choice Context families

- Physical Capability
- Resource Availability
- Opportunity / Access
- Knowledge / Competence
- Expected Effectiveness
- Risk & Cost
- Social / Legal / Moral Constraint
- Relationship / Role Constraint
- Alternative Availability
- Power, Authority & Deterrence

## A.8 Behavior families

- Acquire
- Preserve / Protect
- Avoid / Escape
- Approach / Investigate
- Communicate / Request
- Cooperate / Assist
- Exchange / Negotiate
- Affiliate / Socialize
- Compete / Contest
- Dominate / Coerce
- Resist / Defy
- Aggress / Harm
- Reconcile / Repair
- Create / Build / Transform

## A.9 Outcome families

1. Actor
2. Target
3. Relationship
4. Material/environmental
5. Economic/institutional
6. Information/reputation
7. Aggregate/threshold

# Appendix B. Conceptual Schema Examples

## B.1 Trigger definition

```yaml
id: trigger.weather.drought
kind: trigger
origin: stochastic
cadence: season
eligibility:
  - region.climate.drought_susceptibility > 0
probability:
  base_per_check: 2500        # fixed-point example, not a final rate
  scale: 1000000
scope_mapping: watershed_region
severity_model: weighted_bands
duration_model: multi_season
cooldown: 4_years
direct_effects:
  - target: state.world.soil_moisture
    operation: decrease
  - target: state.world.surface_water
    operation: decrease
  - target: state.world.fire_risk
    operation: increase
```

## B.2 Actor trait definition

```yaml
id: trait.curiosity
kind: state_definition
layer: actor_attribute
authority: spark_owned
value_type: fixed_0_1000
valid_scopes: [actor]
behavioral_leverage: high
consumers:
  - appraisal.opportunity
  - behavior.approach_investigate
  - behavior.communicate_request
```

## B.3 Directed relationship state

```yaml
definition_id: relationship.trust
scope:
  subject: actor.bron
  target: player.main
value: 710
baseline: 400
updated_at: 00403-07-12T08:00
```

## B.4 Behavior intent

```yaml
intent_id: intent.9f2a
actor: actor.bron
primary_family: communicate_request
target: player.main
goal: goal.obtain_food.17
speech_act: request_aid
topic: resource.food
belief_refs:
  - belief.goblins_stole_stores
relationship_refs:
  - relationship.trust
score: 842
random_address: behavior/bron/403-07-12/17
```

# Appendix C. Conceptual Protocol Examples

## C.1 Handshake

```json
{
  "type": "hello",
  "client_id": "swarm-mci",
  "protocol_versions": ["1.0"],
  "requested_profile": "mci-social",
  "requested_capabilities": ["ingest.safe_telemetry", "query.read", "scene.read"],
  "auth": "ephemeral-local-token"
}
```

## C.2 Game batch evaluation request

```json
{
  "type": "evaluate_actors",
  "profile": "game-world",
  "logical_time": 40307120800,
  "actors": ["actor.bron", "actor.mara"],
  "context_refs": ["settlement.pontafique", "region.northwood"],
  "max_intents_per_actor": 3
}
```

## C.3 Host outcome acknowledgement

```json
{
  "type": "behavior_outcome",
  "intent_id": "intent.9f2a",
  "status": "executed",
  "actual_effects": [
    {"kind": "resource_transfer", "resource": "food", "quantity": 2},
    {"kind": "player_response", "intent": "offer_food"}
  ]
}
```

# Appendix D. Requirement Coverage Checklist

| Operator goal | Required architecture coverage |
|---|---|
| Add/remove/redesign tendencies later | Profile definitions, stable IDs, structural reload, migrations |
| Natural and supernatural events | Trigger taxonomy, Direct World Conditions, shared propagation rules |
| Editable probabilities/cadences | Hot-tunable validated configuration and volatility presets |
| Rare variety without chaos | Broad dormant catalogs, low density, eligibility, cooldown, susceptibility |
| Charisma/intelligence/appearance | Actor attributes plus observer/context-dependent appraisals |
| Reasonableness/conscience | Self-control, empathy, harm aversion, norms, alternatives, deterrence |
| Religion and denomination | First-class affiliations, doctrine, devoutness, honor/norm mappings |
| Real NPC economy | Host economy; S.P.A.R.K. consumes affordability/insecurity/obligation conditions |
| Large world continues evolving | Active/warm/dormant tiers and scoped macro updates |
| Rich NPC personalities | Traits, dynamic states, goals, selective memories, relationships, profile packs |
| Dynamic conversations | Semantic speech intents and generated player response intents |
| Persistent voices/TTS | Deterministic VoiceProfile plus host backend adapter |
| Quests/bounties/crises | Advisory GameplayHookCandidate outputs |
| MCI office/town/movie | Non-authoritative mci-social profile, scenes, screenplay, voice outputs |
| AI real-time analysis | Inspection/query/explanation/QC API |
| Security between systems | Versioned contract, handshake, capabilities, validation, rate limits |
| Cross-platform | Rust core; service and embedded adapters; Windows/Linux/Android CI |

# Appendix E. v0.1 to v0.2 Supersession Register

| v0.1 treatment | v0.2 ruling |
|---|---|
| Implementation language undecided | Rust frozen |
| Research inventories pending | Both integrated as source catalogs |
| Factor model relatively flat | Frozen causal grammar and layer taxonomy |
| Game profile active/dormant only | Active/warm/dormant plus macro continuity and scope propagation |
| Generic behavior matrix | Ten Choice Context families and fourteen behavior families |
| Trigger scheduler optional | Deterministic due-work scheduler is core; host/delegation authority explicit |
| Dialogue expression focused on NPC output | Dynamic NPC line plus generated player-response intents |
| Economy only generally separated | Explicit host-owned accounts/wages/prices/transactions boundary |
| Supernatural support implicit | First-class trigger and direct-condition taxonomy slot |
| Modularity broadly stated | Three change classes: hot tune, structural profile reload, engine semantic change |
| Chronicle excluded | Still excluded; bounded active explanation only |
