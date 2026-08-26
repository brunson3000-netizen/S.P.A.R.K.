# S.P.A.R.K. Phase 0 Requirement-Coverage Matrix

**Project:** S.P.A.R.K. — System for Propagating Affective Responses and Consequences  
**Controlling source:** `SPARK_ENGINEERING_TAKEOVER_BLUEPRINT_v0.2.md`  
**Matrix version:** 0.1  
**Status:** Phase 0 baseline / implementation not started  
**Rule:** A requirement is not considered covered merely because a module exists. Coverage requires an implementation location, a verification path, and—where architectural—a gate test or independent review.

## Status Legend

- **FROZEN** — architecture/product invariant already approved by the operator.
- **PLANNED** — implementation home and verification are defined; code does not yet exist.
- **CALIBRATE** — mechanism is required, final values wait for measurement/content calibration.
- **FUTURE** — seam preserved; implementation explicitly deferred.
- **OUT** — excluded unless later operator decision reopens scope.

## Coverage Matrix

| ID | Requirement / operator goal | Blueprint source | Architecture coverage | Planned implementation home | Required verification | Phase | Status |
|---|---|---|---|---|---|---|---|
| R-001 | Production core is Rust | §§0, 5.1, 8.1, 34.1 | Workspace and dependency ADR | `crates/spark-*` | Linux + Windows build/test; Android-compatible core CI check | 0–1 | FROZEN |
| R-002 | Windows, Linux, Android compatibility from inception | §§5.1, 8.1, 31 | Platform-neutral core; adapters outside core | workspace CI + adapter boundaries | Cross-platform fixture replay | 1–3 | FROZEN |
| R-003 | Standalone service and embeddable runtime share semantics | §§3.1, 5.1, 8.2, 28, 31 | One core, two adapters, one logical contract | `spark-core`, `spark-service`, embedded API | Same fixture corpus -> identical canonical outputs | 0–3 | FROZEN |
| R-004 | One shared versioned protocol for game, MCI, browser, CLI, inspectors | §§5.1, 9 | Protocol independent of host/core internals | `spark-protocol` + adapters | schema compatibility + handshake fixtures | 3 | FROZEN |
| R-005 | Host owns physical/economic truth | §§6.2, 7, 23.2 | Per-definition authority; host acknowledgement | `spark-core` authority + protocol outcome messages | Property test: S.P.A.R.K. cannot canonically mutate host-owned state | 0–4 | FROZEN |
| R-006 | S.P.A.R.K. owns causal/social/psychological/relationship/memory/goal/advisory state | §§7.1, 11–16 | `spark_owned` StateCells + bounded stores | `spark-core` | ownership validation + replay fixtures | 1–4 | FROZEN |
| R-007 | Derived state is deterministic and not independently writable | §7 | `derived` authority mode | `spark-core` evaluator | reject direct writes; recomputation equivalence | 1–2 | FROZEN |
| R-008 | BehaviorIntent is advisory, never a host command | §§7.3, 16.5 | Intent/outcome split | `spark-core`, `spark-protocol` | host reject/defer/fail/execute integration tests | 3–4 | FROZEN |
| R-009 | Frozen causal grammar with valid shortcuts | §§6.1, 11, 14.6 | Small primitives + typed graph + declared shortcut metadata | `spark-core`, `spark-profile` | profile validator + causal fixture traces | 1–2 | FROZEN |
| R-010 | Minimal primitive set stays small | §11.3 | TriggerOccurrence, StateCell, BeliefRecord, MemoryRecord, GoalInstance, BehaviorIntent, EffectBatch, PropagationRule, ProfileManifest | `spark-core`, `spark-profile` | architecture review rejects unnecessary primitive additions | all | FROZEN |
| R-011 | Vocabulary is profile-driven and replaceable | §§6.1, 12, 33 | Versioned manifests; stable IDs | `spark-profile`, `profiles/*` | add trait/trigger/religion without Rust changes | 1–10 | FROZEN |
| R-012 | Stable namespaced IDs are immutable once persisted | §§12.3, 28 | ID/version/migration ADR | `spark-profile`, `spark-persistence` | migration/deprecation tests | 1–3 | FROZEN |
| R-013 | Definitions declare scope, authority, type, lifecycle | §§12.1–12.2 | Common schema + validator | `spark-profile` | invalid definition corpus | 1 | FROZEN |
| R-014 | Hot-tunable values change safely without Rust changes | §12.4 | validated mutable config revisions | `spark-profile`, protocol config API | dry-run + revision hash + deterministic epoch tests | 3,10 | FROZEN |
| R-015 | Structural profile changes require controlled reload/checkpoint | §12.4 | structural reload policy | `spark-profile`, `spark-persistence` | reload/migration fixtures | 3,10 | FROZEN |
| R-016 | Engine-semantic changes require Rust + ADR | §12.4 | change-class governance | repository ADR process | independent architecture gate | all | FROZEN |
| R-017 | Natural, social, political, economic, military, technical, supernatural triggers share one engine | §§0, 13, 34 | generic trigger definition + scope/effects | `spark-core`, profile data | drought + blood moon extensibility fixture | 2,6,10 | FROZEN |
| R-018 | Broad event possibility, low experienced event density | §§6.3, 13.2, 19.2 | due scheduler + eligibility + cooldown + volatility | `spark-core`, profiles | dormant catalog idle-cost benchmark + rate fixtures | 1–2,10 | FROZEN/CALIBRATE |
| R-019 | Global/scoped triggers work independent of player proximity | §§13.4, 18.3, 31 | semantic scopes + scheduler | `spark-core` | distant-scope trigger fixture | 2,6 | FROZEN |
| R-020 | Twelve material bridge domains are taxonomy, not engine primitives | §14.1 | profile metadata labels | `profiles/game-world` | schema/profile validation | 6+ | FROZEN |
| R-021 | Fourteen economic/social/institutional domains remain distinct | §14.2 | profile taxonomy | `profiles/game-world` | scenario coverage | 6+ | FROZEN |
| R-022 | Ten actor-situation domains distinguish objective situation from interpretation | §14.3 | host observations/derived situation separate from belief/appraisal | core/profile | unit + actor scenario tests | 4,6 | FROZEN |
| R-023 | Beliefs may be false while host truth remains authoritative | §14.4 | BeliefRecord stores source/confidence/meaning | `spark-core` | false-belief behavior fixture | 4 | FROZEN |
| R-024 | Appraisals are normally ephemeral | §§11.3, 14.5, 26.2 | computed working set, not canonical persistent state | `spark-core` | persistence snapshot exclusion test | 4 | FROZEN |
| R-025 | Stable traits/tendencies influence behavior without becoming guarantees | §15.1 | profile StateCells + scoring modifiers | core/profile | curiosity/charisma/intelligence behavioral tests | 4 | FROZEN |
| R-026 | Compact dynamic internal states with recovery | §§6.7, 15.2 | bounded state + decay/recovery rules | core/profile | escalation/recovery fixtures | 2,4 | FROZEN |
| R-027 | Directed relationships are sparse and directional | §§15.3, 27.3 | pair-scoped StateCells + sparse store | `spark-core`, persistence | directionality + growth/pruning property tests | 4 | FROZEN |
| R-028 | Reputation, honor, fame, and status remain separate concepts | §15.4 | distinct profile definitions/scopes | game profile | religion/faction interpretation fixtures | 8 | FROZEN |
| R-029 | Memory is selective, behaviorally relevant, and bounded | §§15.5, 26, 27 | MemoryRecord + retention/pruning policy | core/persistence | memory growth/pruning tests | 4 | FROZEN |
| R-030 | Sparse hierarchical affiliations map global/scoped effects to actors | §15.6 | target/affiliation scopes | core/profile | faction/religion/settlement propagation fixtures | 4,6,8 | FROZEN |
| R-031 | Different behavioral packs for humans, animals, monsters, software personas | §15.7 | profile packs, no new core psychology primitive | profiles | pack loading + candidate behavior coverage | 4,10 | FROZEN |
| R-032 | Actors maintain multiple active goals with conflict | §16.1 | GoalInstance lifecycle + salience | `spark-core` | conflicting-goal fixtures | 4 | FROZEN |
| R-033 | Choice Context prevents desire -> extreme action shortcuts | §16.2 | feasibility/risk/norm/relationship/alternatives/power modifiers | `spark-core` | moderate-alternative dominance tests | 4 | FROZEN |
| R-034 | Behavior families are semantic; concrete behaviors use family + tags | §16.3 | profile definitions + semantic intent | core/profile | behavior schema tests | 4 | FROZEN |
| R-035 | Deterministic stochastic choice varies actors without incoherence | §16.4 | fixed-point score + random-address tie/selection | core | seed isolation + behavior distribution fixtures | 4 | FROZEN |
| R-036 | Host-confirmed outcomes feed causal circulation | §17 | EffectBatch/outcome ingestion/aggregation | core/protocol | executed/rejected host loop fixtures | 4,6 | FROZEN |
| R-037 | Feedback crosses commit/delay boundary; no zero-delay recursive cycles | §§17, 19.4–19.5 | wave commit + cycle validator | core/profile | adversarial cycle corpus | 2 | FROZEN |
| R-038 | Scheduler evaluates only due work using host monotonic integer time | §§18.1, 28 | deterministic scheduler ADR | `spark-core` | due-work ordering/replay tests | 1 | FROZEN |
| R-039 | Active/warm/dormant fidelity tiers preserve world continuity | §18.2–18.4 | fidelity policy + scheduled aggregates | core/profile/host adapter | promotion/demotion + catch-up fixtures | 6 | FROZEN |
| R-040 | World size must not drive per-frame all-world evaluation | §§18.4, 27 | due/changed/sparse work model | core | scaling benchmarks | 1–10 | FROZEN |
| R-041 | RNG uses semantic random addresses, not a mutable global stream | §§19.1, 28 | root seed + epoch + rule + scope + occurrence | core | concurrency/batching/order invariance property tests | 1 | FROZEN |
| R-042 | Canonical probabilities/scores use integer/fixed-point where practical | §§19.2, 28 | deterministic numeric model | core | cross-platform canonical fixture hashes | 1–4 | FROZEN |
| R-043 | Rules are constrained/declarative; no general scripting language v1 | §§19.3, 37 | fixed operation vocabulary | core/profile | reject executable/unrecognized expressions | 2 | FROZEN |
| R-044 | Deterministic waves evaluate snapshot -> collect -> sort -> validate -> commit | §19.4 | transaction/wave semantics | core | equal-time ordering + failure atomicity tests | 2 | FROZEN |
| R-045 | Dynamic dialogue is semantic/state-driven, not static-only or unrestricted LLM | §20 | expression layer consumes core state/intents | `spark-expression` | Bron food-request tree fixture | 7 | FROZEN |
| R-046 | Player response options feed relationship-sensitive outcomes | §§20.1–20.2, 30.3 | semantic response intents + host outcome | expression/core/protocol | refusal vs credible explanation fixture | 7 | FROZEN |
| R-047 | Repetition is bounded by salience/cooldowns/recent-line context | §20.5 | expression selection policy | `spark-expression` | repetition stress fixture | 7 | FROZEN |
| R-048 | Persistent deterministic voice identity; backend-neutral TTS seam | §21 | VoiceProfile separate from core; state performance modifiers | `spark-voice` | stable identity + adapter request fixture | 9 | FROZEN |
| R-049 | Quests/bounties/crises/ruler decisions are advisory hook candidates | §22 | GameplayHookCandidate; host instantiates formal content | protocol/profile | hook generation + host rejection fixture | 8 | FROZEN |
| R-050 | Game economy remains host-owned | §23.2 | derived affordability/insecurity only | game adapter/core | property test against ledger mutation | 6 | FROZEN |
| R-051 | MCI Agent Commons is a one-way fictional social sidecar | §24 | separate profile + restricted capabilities + sanitizing bridge | MCI adapter/service | hard no-reverse-authority adversarial tests | 3,5 | FROZEN |
| R-052 | MCI fictional state must never be represented as real model hidden state | §§24.3, 24.7 | explicit semantic labeling in profile/protocol/expression | MCI profile/expression | output/schema review | 5 | FROZEN |
| R-053 | AI inspection/explanation is first-class and evidence-bounded | §25 | structured queries over canonical state/source refs | `spark-inspection` | “why?” fixture corpus + missing-provenance behavior | 3–10 | FROZEN |
| R-054 | AI inspectors are read/propose/stage only; no production-state bypass | §25.4 | capability scopes + config validation | protocol/service | authorization adversarial tests | 3 | FROZEN |
| R-055 | Persist continuity state, not exhaustive Chronicle | §26 | bounded snapshot model | `spark-persistence` | snapshot field whitelist + size benchmarks | 3 | FROZEN |
| R-056 | Explanations may admit missing history; never invent provenance | §§25.3, 26.5 | bounded source refs + coverage warnings | inspection | degraded-provenance fixture | 3 | FROZEN |
| R-057 | Engine telemetry measures scheduler, propagation, behavior, growth, latency, divergence | §26.4 | internal metrics API, no ownership of core truth | inspection/service | telemetry schema + benchmark harness | 3–10 | FROZEN |
| R-058 | Work scales with due rules/changed state/sparse edges/aggregates | §27.2 | event/due-driven scheduling + sparse stores | core | asymptotic/load benchmarks | 1–10 | FROZEN |
| R-059 | Rich named actor remains bounded in traits/goals/memories/relationships | §27.3 | profile budgets + pruning | core/profile/persistence | growth load test | 4 | CALIBRATE |
| R-060 | Deterministic replay across supported builds | §28 | version/epoch/ordering/RNG constitution | core/persistence/testkit | fixture hash parity Linux/Windows; Android-compatible check | 1–10 | FROZEN |
| R-061 | Service security is local-only/deny-by-default/untrusted-input/no arbitrary code | §10 | capability/security budget + validation | service/protocol/profile | malformed/oversized/auth adversarial suite | 3 | FROZEN |
| R-062 | Config mutation is explicitly scoped, validated, versioned, and cannot alter invariants | §10.3 | mutable-field allowlist + revisions | profile/protocol/service | permission/range/epoch tests | 3 | FROZEN |
| R-063 | MCI has no API path to real work assignment/tools/credentials/routing/budgets/governance/files/review | §10.4 | structural capability exclusion | protocol/MCI adapter | negative capability enumeration + adversarial tests | 3,5 | FROZEN |
| R-064 | Pontafique/Lindemar drought loop is first full causal reference | §30 | end-to-end vertical fixture | `spark-testkit`, game profile/adapter | full causal trace and bounded explanation | 6 | FROZEN |
| R-065 | Curiosity can be added/tuned without Rust changes | §30.4 | data-driven trait/scoring consumers | profile | extensibility acceptance test | 4,6 | FROZEN |
| R-066 | Blood moon can be added without Rust changes | §30.4 | generic supernatural trigger + existing states/rules | profile | extensibility acceptance test | 6 | FROZEN |
| R-067 | Religion/denomination can be added without Rust changes | §§14.2, 30.4 | affiliation/profile definitions + interpretation/honor mappings | profile | distinct judgment fixture | 8 | FROZEN |
| R-068 | Trigger probabilities/cadences can be modified via validated config | §§19.2, 30.4 | hot-tune config | profile/service | dry-run/apply/replay epoch test | 3,10 | FROZEN |
| R-069 | MCI thin proof ingests harmless telemetry, emits fictional scene, has zero execution authority | §30.5 | one-way MCI vertical slice | MCI adapter/profile/expression | end-to-end negative-authority proof | 5 | FROZEN |
| R-070 | Broad dormant catalogs must not cause proportional idle CPU cost | §§27, 31 | due scheduler/indices | core | dormant catalog benchmark | 1–2,10 | FROZEN |
| R-071 | Invalid profiles/cycles/scopes/requests fail clearly | §§12.5, 19.5, 31 | validator + typed rejections | profile/protocol/service | invalid corpus + error contract | 1–3 | FROZEN |
| R-072 | No runtime LLM dependency or authoritative LLM NPC brain | §§5.2, 20.4, 34.4 | core/expression are deterministic and structured | dependency policy | dependency audit + architecture review | all | OUT/FROZEN |
| R-073 | No graph DB/distributed event bus/cloud/federation in v1 | §§5.2, 34.4, 37 | local deterministic runtime/service | workspace/dependency policy | dependency/architecture review | all | OUT |
| R-074 | No universal plugin platform/general scripting runtime | §§5.2, 19.3, 34.4, 37 | profile data over fixed grammar | profile/ADR governance | independent architecture review | all | OUT |
| R-075 | Living Chronicle remains shelved; only bounded active explanation survives | §§0, 2.1, 26.5, 34 | persistence/explanation boundary | persistence/inspection | no exhaustive event-ledger dependency; snapshot audit | all | OUT/FROZEN |
| R-076 | Large research inventories remain catalogs, not persistent runtime mandates | §33 | admission rule + telemetry-driven pruning | profiles/research ingestion tooling later | definition admission review | 10 | FROZEN |
| R-077 | Operator decides meaning/authority/scope; engineer owns routine realization | §§2.4, 35 | decision-rights governance | ADR/review process | escalation audit at milestones | all | FROZEN |
| R-078 | New primitive/authority weakening/determinism weakening/paid dependency/etc. requires escalation | §35.3 | mandatory escalation rule | engineering process | milestone review | all | FROZEN |
| R-079 | Requirement coverage must be reported at every milestone | §§2.4, 37 | this matrix is living traceability record | repository docs | gate checklist diff | all | FROZEN |
| R-080 | Performance claims/world-size ceilings wait for joint profiling | §§18.4, 27.4 | benchmark-before-freeze rule | `spark-testkit` benchmarks | measured scaling curves | 1–10 | FROZEN |

## Phase-0 Coverage Result

### Architectural gaps discovered

**None requiring a new runtime primitive.** Every frozen v0.2 requirement currently maps to the approved primitive set and planned crate/profile boundaries.

This is not proof that the architecture is correct. It means only that the requirement set is internally mappable without an obvious missing primitive. Independent review must attempt to falsify this finding before Phase 1 implementation begins.

### Decisions intentionally not frozen here

- exact serializer/profile file format;
- exact HTTP/WebSocket/SSE implementation;
- exact persistence library/backend;
- exact actor/state/memory numeric caps;
- exact scheduler work quotas;
- exact trigger rates and score weights;
- exact Active Simulation Envelope geometry;
- exact TTS backend mapping.

These remain provisional or calibration-bound per blueprint v0.2.

## Gate Rule

Phase 0 may advance to Phase 1 only when:

1. this matrix has no unresolved FROZEN requirement without an implementation home and verification path;
2. ADR-0001 through ADR-0006 are accepted or revised;
3. performance/security budget v0.1 has no contradiction with the blueprint;
4. an independent reviewer has attempted to identify missing invariants, accidental scope expansion, authority leaks, determinism hazards, or premature implementation freezes;
5. any blocker from that review is resolved or explicitly escalated to the operator.
