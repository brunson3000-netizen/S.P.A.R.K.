# S.P.A.R.K. Phase 0 Requirement-Coverage Matrix

**Project:** S.P.A.R.K. — System for Propagating Affective Responses and Consequences  
**Controlling source:** `CONTROLLING_BLUEPRINT_v0.2.md`  
**Matrix version:** 0.2  
**Status:** Phase 0 corrected writer pass / independent re-review required / implementation not started  
**Supersedes:** matrix v0.1  
**Rule:** A requirement is not covered merely because a module exists. Coverage requires an implementation location, a falsifiable verification path, and—where architectural—a gate test or independent review.

## Status Legend

- **FROZEN** — architecture/product invariant approved by the operator.
- **PROFILE_DATA** — editable profile vocabulary/content using existing frozen semantics.
- **PENDING_CALIBRATION** — mechanism exists; final numeric/content values await measurement/calibration.
- **PLANNED** — implementation home/verification defined; code does not yet exist.
- **FUTURE** — seam preserved; implementation deferred.
- **OUT** — excluded unless a later operator decision reopens scope.

## Coverage Matrix

| ID | Requirement / operator goal | Blueprint source | Architecture coverage | Planned implementation home | Required verification | Phase | Status |
|---|---|---|---|---|---|---|---|
| R-001 | Production core is Rust | §§0, 5.1, 8.1, 34.1 | Workspace/dependency ADR | `crates/spark-*` | Linux + Windows build/test; executable Android fixture path planned from inception | 0–1 | FROZEN |
| R-002 | Windows, Linux, Android compatibility from inception | §§5.1, 8.1, 31 | Platform-neutral canonical core; platform adapters outside core | workspace CI + adapters | Canonical fixture execution on Linux/Windows/Android before acceptance | 1–3 | FROZEN |
| R-003 | Standalone service and embeddable runtime share semantics | §§3.1, 5.1, 8.2, 28, 31 | One core; canonical command envelope/barriers; transport batching inert | core/service/embedded facade | Same accepted canonical command stream -> same state/output hashes despite batching/concurrency/session differences | 0–3 | FROZEN |
| R-004 | One shared versioned protocol for game, MCI, browser, CLI, inspectors | §§5.1, 9 | Protocol independent of host/core internals | `spark-protocol` + adapters | schema compatibility + handshake fixtures | 3 | FROZEN |
| R-005 | Host owns physical/economic truth | §§6.2, 7, 23.2 | Immutable authority/write class; host ingress/outcome only | core/profile/protocol | Property tests across runtime + reload/alias/migration/restore reject S.P.A.R.K. acquisition of host authority | 0–4 | FROZEN |
| R-006 | S.P.A.R.K. owns causal/social/psychological/relationship/memory/goal/advisory state | §§7.1, 11–16 | `spark_owned` StateCells + bounded stores | `spark-core` | ownership validation + replay fixtures | 1–4 | FROZEN |
| R-007 | Derived state is deterministic and not independently writable | §7 | Immutable `derived` authority/write class; evaluator only | core/profile | reject direct write and lifecycle authority conversion; recomputation equivalence | 1–3 | FROZEN |
| R-008 | BehaviorIntent is advisory, never a host command | §§7.3, 16.5 | Separate intent disposition from actual executed outcome | core/protocol | accepted-but-unexecuted produces no physical/economic feedback; reject stale/forged/cross-profile acknowledgements | 3–4 | FROZEN |
| R-009 | Frozen causal grammar with valid shortcuts | §§6.1, 11, 14.6 | Small primitives + typed graph + declared shortcut metadata | `spark-core`, `spark-profile` | profile validator + causal fixture traces | 1–2 | FROZEN |
| R-010 | Minimal primitive set stays small | §11.3 | TriggerOccurrence, StateCell, BeliefRecord, MemoryRecord, GoalInstance, BehaviorIntent, EffectBatch, PropagationRule, ProfileManifest | `spark-core`, `spark-profile` | architecture review rejects unnecessary primitive additions | all | FROZEN |
| R-011 | Vocabulary is profile-driven and replaceable | §§6.1, 12, 33 | Versioned manifests; stable IDs | `spark-profile`, `profiles/*` | add trait/trigger/religion without Rust changes | 1–10 | FROZEN |
| R-012 | Stable namespaced IDs are immutable once persisted | §§12.3, 28 | ID + immutable definition fingerprint + content-addressed behavior artifacts | profile/persistence | deprecation/migration/alias/restore fixtures; same ID cannot silently change immutable meaning | 1–3 | FROZEN |
| R-013 | Definitions declare scope, authority, type, lifecycle | §§12.1–12.2 | Common schema; authority/write class in immutable identity fingerprint | profile | invalid definition + identity-mismatch corpus | 1 | FROZEN |
| R-014 | Hot-tunable values change safely without Rust changes | §12.4 | Validated content-hashed config revision activated at canonical barrier; new behavior epoch when outcomes may change | profile/protocol | dry-run + hash + atomic activation + replay-epoch tests | 3,10 | FROZEN |
| R-015 | Structural profile changes require controlled reload/checkpoint | §12.4 | Validate/hash/stage/checkpoint/atomic barrier activation; exact source/target artifacts | profile/persistence | mid-wave reload, migration rollback, artifact-resolution fixtures | 3,10 | FROZEN |
| R-016 | Engine-semantic changes require Rust + ADR | §12.4 | Change-class governance; authority/write-class change specifically escalated | ADR/review process | independent architecture gate + authority-change negative fixtures | all | FROZEN |
| R-017 | Natural, social, political, economic, military, technical, supernatural triggers share one engine | §§0, 13, 34 | generic trigger definition + scope/effects | `spark-core`, profile data | drought + blood moon extensibility fixture | 2,6,10 | FROZEN |
| R-018 | Broad event possibility uses low-density scheduling/eligibility rather than constant polling | §§6.3, 13.2, 19.2 | Due scheduler + eligibility + cooldown + volatility mechanism | core/profiles | dormant catalog idle-cost benchmark | 1–2,10 | FROZEN |
| R-019 | Global/scoped triggers work independent of player proximity without enumerating all dormant actors/cells | §§13.4, 18.3, 27, 31 | Aggregate/inherited scope exposure + lazy actor materialization | core | distant-scope fixture + 10x dormant-population scaling test | 2,6 | FROZEN |
| R-020 | Current material bridge taxonomy is editable profile vocabulary; 12-domain list is the provisional baseline, not an engine primitive/exhaustive Rust catalog | §§14.1, 34.1 | Profile metadata labels | game-world profile | schema/profile validation; no Rust enum exhaustiveness requirement | 6+ | PROFILE_DATA |
| R-021 | Current economic/social/institutional taxonomy is editable profile vocabulary; 14-domain list is the provisional baseline | §§14.2, 34.1 | Profile taxonomy | game-world profile | profile scenario coverage; additions/reorganization within existing semantics require no Rust primitive | 6+ | PROFILE_DATA |
| R-022 | Ten actor-situation domains distinguish objective situation from interpretation | §14.3 | host observations/derived situation separate from belief/appraisal | core/profile | unit + actor scenario tests | 4,6 | FROZEN |
| R-023 | Beliefs may be false while host truth remains authoritative | §14.4 | BeliefRecord stores source/confidence/meaning | `spark-core` | false-belief behavior fixture | 4 | FROZEN |
| R-024 | Appraisals/choice-context scores are normally ephemeral; persistent interpretations may use declared StateCell | §§11.3, 14.5, 26.2 | Ephemeral working set by default; ordinary StateCell when persistence is behaviorally required | core/profile | default snapshot exclusion + explicit persistent-interpretation StateCell fixture | 4 | FROZEN |
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
| R-036 | Only host-confirmed executed outcomes feed physical/economic causal circulation | §17 | Disposition/outcome split; outcome binds outstanding intent idempotently | core/protocol | accepted-never-executed, executed, duplicate, stale, forged, cross-profile acknowledgement fixtures | 4,6 | FROZEN |
| R-037 | Feedback crosses commit/delay boundary; no zero-delay recursive cycles | §§17, 19.4–19.5 | wave commit + cycle validator | core/profile | adversarial cycle corpus | 2 | FROZEN |
| R-038 | Scheduler evaluates only due work using host monotonic integer time | §§18.1, 28 | Canonical command barriers + due-time scheduler; quota yields causally inert | core | due ordering + ten-days-once versus one-day-ten-times + quota-resume tests | 1 | FROZEN |
| R-039 | Active/warm/dormant fidelity tiers preserve world continuity | §18.2–18.4 | fidelity policy + scheduled aggregates | core/profile/host adapter | promotion/demotion + catch-up fixtures | 6 | FROZEN |
| R-040 | World size must not drive per-frame all-world evaluation | §§18.4, 27 | due/changed/sparse work model | core | scaling benchmarks | 1–10 | FROZEN |
| R-041 | RNG uses semantic random addresses, not mutable global stream | §§19.1, 28 | Root seed + exact behavior epoch/artifact context + rule/scope + persisted occurrence index | core | concurrency/batch/quota/catch-up partition invariance | 1 | FROZEN |
| R-042 | Canonical probabilities/scores use integer/fixed-point where practical | §§19.2, 28 | deterministic numeric model | core | cross-platform canonical fixture hashes | 1–4 | FROZEN |
| R-043 | Rules are constrained/declarative; no general scripting language v1 | §§19.3, 37 | fixed operation vocabulary | core/profile | reject executable/unrecognized expressions | 2 | FROZEN |
| R-044 | Deterministic waves evaluate stable snapshot -> collect -> sort -> validate -> commit within explicit canonical command barriers | §19.4 | Canonical input transaction/barrier semantics | core | same-time explicit order; concurrent/split batch equivalence; snapshot boundary/failure atomicity | 1–2 | FROZEN |
| R-045 | Dynamic dialogue is semantic/state-driven, not static-only or unrestricted LLM | §20 | expression layer consumes core state/intents | `spark-expression` | Bron food-request tree fixture | 7 | FROZEN |
| R-046 | Player response options feed relationship-sensitive outcomes | §§20.1–20.2, 30.3 | semantic response intents + host outcome | expression/core/protocol | refusal vs credible explanation fixture | 7 | FROZEN |
| R-047 | Repetition is bounded by salience/cooldowns/recent-line context | §20.5 | expression selection policy | `spark-expression` | repetition stress fixture | 7 | FROZEN |
| R-048 | Persistent deterministic voice identity; backend-neutral TTS seam | §21 | VoiceProfile separate from core; state performance modifiers | `spark-voice` | stable identity + adapter request fixture | 9 | FROZEN |
| R-049 | Quests/bounties/crises/ruler decisions are advisory hook candidates | §22 | GameplayHookCandidate; host instantiates formal content | protocol/profile | hook generation + host rejection fixture | 8 | FROZEN |
| R-050 | Game economy remains host-owned | §23.2 | derived affordability/insecurity only | game adapter/core | property test against ledger mutation | 6 | FROZEN |
| R-051 | MCI Agent Commons is a one-way fictional social sidecar | §24 | Separate trust domain/state/IDs/config/output allowlist + sanitizing bridge | MCI adapter/service/profile | cross-profile reference/subscription/config/output-routing attacks fail | 3,5 | FROZEN |
| R-052 | MCI fictional state must never be represented as real model hidden state | §§24.3, 24.7 | explicit semantic labeling in profile/protocol/expression | MCI profile/expression | output/schema review | 5 | FROZEN |
| R-053 | AI inspection/explanation is first-class and evidence-bounded | §25 | structured queries over canonical state/source refs | `spark-inspection` | “why?” fixture corpus + missing-provenance behavior | 3–10 | FROZEN |
| R-054 | AI inspectors are read/propose/stage only; no production-state bypass | §25.4 | Non-composable inspection capabilities; dry-run on isolated immutable snapshot/candidate config | protocol/service/inspection | production hash unchanged after adversarial dry-run/profiling/explanation abuse | 3 | FROZEN |
| R-055 | Persist continuity state, exact behavior-artifact references, and bounded provenance; not exhaustive Chronicle | §§26, 28 | Stable-boundary snapshot + content-addressed manifest/config/epoch references | persistence | save continuation/migration/artifact-resolution + size fixtures | 3 | FROZEN |
| R-056 | Explanations admit missing/truncated history and never invent provenance | §§25.3, 26.5 | Deterministic bounded refs + explicit complete/truncated/unknown coverage metadata | inspection/persistence | pruning/coverage fixture; summaries never treated as authoritative refs | 3 | FROZEN |
| R-057 | Engine telemetry measures scheduler, propagation, behavior, growth, latency, divergence | §26.4 | internal metrics API, no ownership of core truth | inspection/service | telemetry schema + benchmark harness | 3–10 | FROZEN |
| R-058 | Work scales with due rules/changed state/sparse edges/aggregates | §27.2 | event/due-driven scheduling + sparse stores | core | asymptotic/load benchmarks | 1–10 | FROZEN |
| R-059 | Rich named actor remains bounded in traits/goals/memories/relationships | §27.3 | profile budgets + pruning | core/profile/persistence | growth load test | 4 | CALIBRATE |
| R-060 | Deterministic replay across supported Windows/Linux/Android builds | §28 | Version/epoch/artifact/ordering/RNG constitution | core/persistence/testkit | Executable canonical fixture hash parity on Linux, Windows, and Android | 1–10 | FROZEN |
| R-061 | Service security is local-only/deny-by-default/untrusted-input/no arbitrary code | §10 | capability/security budget + validation | service/protocol/profile | malformed/oversized/auth adversarial suite | 3 | FROZEN |
| R-062 | Config mutation is explicitly scoped, validated, versioned, content-addressed, barrier-activated, and cannot alter invariants | §10.3 | Mutable-field allowlist + immutable authority identity + config revision hash | profile/protocol/service | permission/range/hash/epoch/mid-wave activation/authority-conversion tests | 3 | FROZEN |
| R-063 | MCI has no path to real work/tools/credentials/routing/budgets/governance/files/review | §10.4 | Trust-domain capability separation + output allowlist + no real-work adapter route | protocol/MCI adapter | enumerate all output families/capability composition/cross-profile routing attacks | 3,5 | FROZEN |
| R-064 | Pontafique/Lindemar drought loop is first full causal reference | §30 | end-to-end vertical fixture | `spark-testkit`, game profile/adapter | full causal trace and bounded explanation | 6 | FROZEN |
| R-065 | Curiosity can be added/tuned without Rust changes | §30.4 | data-driven trait/scoring consumers | profile | extensibility acceptance test | 4,6 | FROZEN |
| R-066 | Blood moon can be added without Rust changes | §30.4 | generic supernatural trigger + existing states/rules | profile | extensibility acceptance test | 6 | FROZEN |
| R-067 | Religion/denomination can be added without Rust changes | §§14.2, 30.4 | affiliation/profile definitions + interpretation/honor mappings | profile | distinct judgment fixture | 8 | FROZEN |
| R-068 | Trigger probabilities/cadences can be modified via validated config | §§19.2, 30.4 | hot-tune config | profile/service | dry-run/apply/replay epoch test | 3,10 | FROZEN |
| R-069 | MCI thin proof ingests harmless telemetry, emits fictional scene, has zero execution authority | §30.5 | one-way MCI vertical slice | MCI adapter/profile/expression | end-to-end negative-authority proof | 5 | FROZEN |
| R-070 | Broad dormant catalogs and global triggers do not cause proportional idle/population CPU cost | §§27, 31 | Due indices + aggregate/inherited scoped exposure + lazy materialization | core | dormant catalog and 10x dormant-population scaling benchmarks | 1–2,10 | FROZEN |
| R-071 | Invalid profiles/cycles/scopes/requests fail clearly | §§12.5, 19.5, 31 | validator + typed rejections | profile/protocol/service | invalid corpus + error contract | 1–3 | FROZEN |
| R-072 | Runtime LLM dependency or authoritative LLM NPC brain is excluded | §§5.2, 20.4, 34.4 | Deterministic structured core/expression | dependency policy | dependency/architecture audit | all | OUT |
| R-073 | No graph DB/distributed event bus/cloud/federation in v1 | §§5.2, 34.4, 37 | local deterministic runtime/service | workspace/dependency policy | dependency/architecture review | all | OUT |
| R-074 | No universal plugin platform/general scripting runtime | §§5.2, 19.3, 34.4, 37 | profile data over fixed grammar | profile/ADR governance | independent architecture review | all | OUT |
| R-075 | Maximum/Living Chronicle is excluded as an implementation dependency | §§0, 2.1, 26.5, 34 | No exhaustive world-event ledger or lifetime transcript dependency | persistence/inspection | dependency/snapshot audit | all | OUT |
| R-076 | Large research inventories remain catalogs, not persistent runtime mandates | §33 | admission rule + telemetry-driven pruning | profiles/research ingestion tooling later | definition admission review | 10 | FROZEN |
| R-077 | Operator decides meaning/authority/scope; engineer owns routine realization | §§2.4, 35 | decision-rights governance | ADR/review process | escalation audit at milestones | all | FROZEN |
| R-078 | New primitive/authority weakening/determinism weakening/paid dependency/etc. requires escalation | §35.3 | mandatory escalation rule | engineering process | milestone review | all | FROZEN |
| R-079 | Requirement coverage must be reported at every milestone | §§2.4, 37 | this matrix is living traceability record | repository docs | gate checklist diff | all | FROZEN |
| R-080 | Performance claims/world-size ceilings wait for joint profiling | §§18.4, 27.4 | benchmark-before-freeze rule | `spark-testkit` benchmarks | measured scaling curves | 1–10 | FROZEN |
| R-081 | Canonical state-changing/evaluation inputs carry effective time, logical source/sequence, unique total-order ordinal, idempotency ID, and payload hash | §§9.3, 18, 19, 28 | CanonicalCommandEnvelope | core/protocol | missing/ambiguous order reject; duplicate/id conflict tests | 1–3 | FROZEN |
| R-082 | Transport batching, concurrent request arrival, worker interleaving, and session identity cannot alter an already accepted canonical command stream | §§8.2, 9, 19, 28 | Transport packaging separated from command barriers | service/embedded/testkit | one batch vs split vs concurrent service requests produce identical hashes | 1–3 | FROZEN |
| R-083 | Authority/write class is immutable for a persisted definition ID | §§7, 12, 28, 35.3 | Definition fingerprint; new ID + explicit migration + authority ADR + operator approval for authority change | profile/persistence | reload/alias/migration/restore authority-conversion attacks | 0–3 | FROZEN |
| R-084 | Profile/config changes activate atomically only at declared canonical barriers | §§10.3, 12.4, 28 | Staged candidate + effective time/ordinal + checkpoint/rollback | profile/core/persistence | mid-wave hot tune/reload tests | 1–3 | FROZEN |
| R-085 | Old-save continuation is bound to immutable content-addressed behavior artifacts, not human version strings alone | §§12.3, 26, 28 | Manifest/config hashes + definition fingerprints + artifact resolver | persistence/profile | reused-version/different-hash and missing-artifact tests | 1–3 | FROZEN |
| R-086 | Delayed obligations preserve originating behavior semantics across reload/save | §§17–19, 26, 28 | Obligation binds creator rule fingerprint/epoch/hashes and explicit materialized-vs-re-evaluate mode | core/persistence | delayed obligation + profile replacement continuation fixture | 1–3 | FROZEN |
| R-087 | Outcome replay and mechanism replay are distinct support modes; unsupported replay fails explicitly | §28 | Separate replay contracts; snapshot is not exhaustive event log | persistence/testkit | separate fixtures + substitution-failure test | 3+ | FROZEN |
| R-088 | Global/scoped exposure remains aggregate/inherited until target activation/due work unless bounded materialization is declared | §§13.4, 18, 27 | Lazy scoped exposure | core/profile | 10x dormant population does not proportionally increase occurrence cost | 2,6 | FROZEN |
| R-089 | Availability, accessibility, and affordability remain distinct causal concepts | §14.1 | Separate profile definitions/derivations | game-world profile | alias/overwrite negative fixture + drought affordability scenario | 6 | FROZEN |
| R-090 | Trigger polarity is descriptive metadata; changing polarity alone cannot change causal effects | §13.2 | Polarity excluded from effect semantics unless an explicit rule reads another declared semantic field | profile/core | polarity-only edit leaves canonical effects unchanged | 2,10 | FROZEN |
| R-091 | Dormant-to-active promotion may not reconstruct unrecorded individual microhistory | §§17, 18.2, 26.5 | Aggregate commitments -> present state only; unknown history remains unknown | core/persistence/inspection | promotion fixture creates no invented memories/relationships/actions | 3,6 | FROZEN |
| R-092 | S.P.A.R.K. receives only permitted/sanitized player or MCI telemetry inputs | §§24.2, 26.3 | Host sanitizing bridge + input allowlist | game/MCI adapters | raw/disallowed telemetry rejection and minimization fixtures | 3,5 | FROZEN |
| R-093 | Profile isolation includes state stores, ID resolution, capabilities, config revisions, subscriptions, acknowledgements, and output routing | §§5.1, 10, 24 | Profile/trust-domain partitioning | core/profile/protocol/service | cross-profile attack matrix across every named boundary | 1–5 | FROZEN |
| R-094 | Host acceptance and actual execution/outcome are separate lifecycle stages | §§7.3, 9.6, 16.5, 17 | Intent disposition vs validated executed outcome | core/protocol | accepted-never-executed cannot create physical/economic/success feedback | 3–4 | FROZEN |
| R-095 | MCI output families are allowlisted away from real-work adapters; inspection dry-run has no production commit route | §§10.4, 24, 25.4 | Trust-domain output router + isolated staging snapshot | service/MCI/inspection | emit every shared family; production-state hash invariant under dry-run | 3,5 | FROZEN |
| R-096 | Internal queues, delayed expansion, graph size, nesting, alias chains, strings, and migration complexity are finitely bounded | §§10, 12.5, 19.5, 27, 29.5 | Versioned performance/security budget + deterministic backpressure/rejection | core/profile/protocol/service | sustained fan-out/schema-bomb/alias-cycle/migration-bomb suite | 1–3 | FROZEN |
| R-097 | Non-semantic transport/session/subscriber metadata is causally inert and canonical numeric representation is normalized | §§9, 28 | Engine-boundary canonicalization | protocol/service/core | different sessions/subscribers/JSON representations preserve accepted-command hashes or reject ambiguity | 1–3 | FROZEN |
| R-098 | Provenance pruning is deterministic and reports evidence coverage; summaries never masquerade as source evidence | §§25.3, 26.5 | Coverage status/counts/pruning policy/summary hash | persistence/inspection | repeated pruning equality + missing-source honesty fixture | 3 | FROZEN |
| R-099 | The role of domain/family taxonomies as profile vocabulary is frozen; the current exact baseline membership/counts remain editable profile data | §§6.1, 14, 34 | Grammar/vocabulary separation | profile/ADR governance | modify profile taxonomy within existing semantics without Rust primitive change | 6+ | FROZEN |
| R-100 | Exact trigger rates, probabilities, volatility multipliers, and calibration values are tunable/calibration data | §§13.2, 19.2, 33, 34.2 | Validated config/profile values, not engine law | profiles/config | rate edits through allowed config preserve schema/determinism | 10 | PENDING_CALIBRATION |

## Phase-0 Correction Result

### Primitive sufficiency

No independent-review finding requires a new runtime primitive. The reviewer explicitly found the frozen primitive set sufficient; the correction therefore changes transaction, authority, persistence, trust-domain, boundedness, and traceability contracts around those primitives rather than expanding the causal grammar.

### Corrections incorporated

- canonical input envelope, total order, command barriers, idempotency, source sequence, batch-partition invariance;
- immutable authority/write class for persisted definition identity;
- exact content-addressed manifest/config/definition artifacts;
- delayed-obligation epoch/rule binding and atomic migration;
- explicit outcome-replay versus mechanism-replay distinction;
- global-trigger lazy/aggregate exposure scaling invariant;
- missing blueprint distinctions and profile-isolation requirements;
- accepted-versus-executed host lifecycle;
- MCI output-routing and inspection staging isolation;
- internal queue/schema/graph/backpressure bounds;
- service/embedded transport metadata/numeric canonicalization;
- executable Android deterministic fixture requirement;
- deterministic provenance coverage semantics;
- taxonomy/calibration status corrections and controlling filename repair.

## Gate Rule

Phase 0 may advance to Phase 1 only when:

1. the independent re-review confirms B-01, B-02, and B-03 are closed;
2. no new blocker demonstrates a missing foundational invariant;
3. the corrected matrix has no unresolved FROZEN requirement without an implementation home and falsifiable verification path;
4. Phase 1 remains limited to the Rust core skeleton authorized by blueprint v0.2.
